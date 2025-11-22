pub fn sub_824A5600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A5600 size=228
    let mut pc: u32 = 0x824A5600;
    'dispatch: loop {
        match pc {
            0x824A5600 => {
    //   block [0x824A5600..0x824A56E4)
	// 824A5600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A5604: 4808FAA1  bl 0x825350a4
	ctx.lr = 0x824A5608;
	sub_82535080(ctx, base);
	// 824A5608: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A560C: 3B830180  addi r28, r3, 0x180
	ctx.r[28].s64 = ctx.r[3].s64 + 384;
	// 824A5610: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824A5614: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 824A5618: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A561C: 3B2BFFFF  addi r25, r11, -1
	ctx.r[25].s64 = ctx.r[11].s64 + -1;
	// 824A5620: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 824A5624: 419800B0  blt cr6, 0x824a56d4
	if ctx.cr[6].lt {
	pc = 0x824A56D4; continue 'dispatch;
	}
	// 824A5628: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824A562C: 573D103A  slwi r29, r25, 2
	ctx.r[29].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 824A5630: 3B0B74DC  addi r24, r11, 0x74dc
	ctx.r[24].s64 = ctx.r[11].s64 + 29916;
	// 824A5634: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824A5638: 3AEB06BC  addi r23, r11, 0x6bc
	ctx.r[23].s64 = ctx.r[11].s64 + 1724;
	// 824A563C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A5640: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824A5644: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A5648: 419A007C  beq cr6, 0x824a56c4
	if ctx.cr[6].eq {
	pc = 0x824A56C4; continue 'dispatch;
	}
	// 824A564C: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A5650: 3BE00014  li r31, 0x14
	ctx.r[31].s64 = 20;
	// 824A5654: 7D5FF02E  lwzx r10, r31, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824A5658: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A565C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5660: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A5664: 40980018  bge cr6, 0x824a567c
	if !ctx.cr[6].lt {
	pc = 0x824A567C; continue 'dispatch;
	}
	// 824A5668: 92EB0000  stw r23, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 824A566C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A5670: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A5674: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A5678: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A567C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A5680: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 824A5684: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824A5688: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824A568C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A5690: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5694: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A5698: 4E800421  bctrl
	ctx.lr = 0x824A569C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A569C: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824A56A0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A56A4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A56A8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A56AC: 40980018  bge cr6, 0x824a56c4
	if !ctx.cr[6].lt {
	pc = 0x824A56C4; continue 'dispatch;
	}
	// 824A56B0: 930A0000  stw r24, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 824A56B4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A56B8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 824A56BC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A56C0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A56C4: 3B39FFFF  addi r25, r25, -1
	ctx.r[25].s64 = ctx.r[25].s64 + -1;
	// 824A56C8: 3BBDFFFC  addi r29, r29, -4
	ctx.r[29].s64 = ctx.r[29].s64 + -4;
	// 824A56CC: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 824A56D0: 4098FF6C  bge cr6, 0x824a563c
	if !ctx.cr[6].lt {
	pc = 0x824A563C; continue 'dispatch;
	}
	// 824A56D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824A56D8: 4BFFE821  bl 0x824a3ef8
	ctx.lr = 0x824A56DC;
	sub_824A3EF8(ctx, base);
	// 824A56DC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824A56E0: 4808FA14  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A56E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A56E8 size=220
    let mut pc: u32 = 0x824A56E8;
    'dispatch: loop {
        match pc {
            0x824A56E8 => {
    //   block [0x824A56E8..0x824A57C4)
	// 824A56E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A56EC: 4808F9BD  bl 0x825350a8
	ctx.lr = 0x824A56F0;
	sub_82535080(ctx, base);
	// 824A56F0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A56F4: 3B830138  addi r28, r3, 0x138
	ctx.r[28].s64 = ctx.r[3].s64 + 312;
	// 824A56F8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824A56FC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5700: 3B4BFFFF  addi r26, r11, -1
	ctx.r[26].s64 = ctx.r[11].s64 + -1;
	// 824A5704: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 824A5708: 419800AC  blt cr6, 0x824a57b4
	if ctx.cr[6].lt {
	pc = 0x824A57B4; continue 'dispatch;
	}
	// 824A570C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824A5710: 575D103A  slwi r29, r26, 2
	ctx.r[29].u32 = ctx.r[26].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 824A5714: 3B2B74DC  addi r25, r11, 0x74dc
	ctx.r[25].s64 = ctx.r[11].s64 + 29916;
	// 824A5718: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824A571C: 3B0B06CC  addi r24, r11, 0x6cc
	ctx.r[24].s64 = ctx.r[11].s64 + 1740;
	// 824A5720: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A5724: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824A5728: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A572C: 419A0078  beq cr6, 0x824a57a4
	if ctx.cr[6].eq {
	pc = 0x824A57A4; continue 'dispatch;
	}
	// 824A5730: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A5734: 3BE00014  li r31, 0x14
	ctx.r[31].s64 = 20;
	// 824A5738: 7D5FF02E  lwzx r10, r31, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824A573C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5740: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5744: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A5748: 40980018  bge cr6, 0x824a5760
	if !ctx.cr[6].lt {
	pc = 0x824A5760; continue 'dispatch;
	}
	// 824A574C: 930B0000  stw r24, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 824A5750: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A5754: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A5758: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A575C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A5760: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A5764: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824A5768: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824A576C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A5770: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A5774: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A5778: 4E800421  bctrl
	ctx.lr = 0x824A577C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A577C: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824A5780: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5784: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5788: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A578C: 40980018  bge cr6, 0x824a57a4
	if !ctx.cr[6].lt {
	pc = 0x824A57A4; continue 'dispatch;
	}
	// 824A5790: 932A0000  stw r25, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 824A5794: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A5798: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 824A579C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A57A0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A57A4: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 824A57A8: 3BBDFFFC  addi r29, r29, -4
	ctx.r[29].s64 = ctx.r[29].s64 + -4;
	// 824A57AC: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 824A57B0: 4098FF70  bge cr6, 0x824a5720
	if !ctx.cr[6].lt {
	pc = 0x824A5720; continue 'dispatch;
	}
	// 824A57B4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824A57B8: 4BFFE741  bl 0x824a3ef8
	ctx.lr = 0x824A57BC;
	sub_824A3EF8(ctx, base);
	// 824A57BC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824A57C0: 4808F938  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A57C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A57C8 size=92
    let mut pc: u32 = 0x824A57C8;
    'dispatch: loop {
        match pc {
            0x824A57C8 => {
    //   block [0x824A57C8..0x824A5824)
	// 824A57C8: 89630127  lbz r11, 0x127(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(295 as u32) ) } as u64;
	// 824A57CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824A57D0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 824A57D4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 824A57D8: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 824A57DC: 99630127  stb r11, 0x127(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(295 as u32), ctx.r[11].u8 ) };
	// 824A57E0: 554A077E  clrlwi r10, r10, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 824A57E4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824A57E8: 409A0010  bne cr6, 0x824a57f8
	if !ctx.cr[6].eq {
	pc = 0x824A57F8; continue 'dispatch;
	}
	// 824A57EC: 89630125  lbz r11, 0x125(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(293 as u32) ) } as u64;
	// 824A57F0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 824A57F4: 99630125  stb r11, 0x125(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(293 as u32), ctx.r[11].u8 ) };
	// 824A57F8: 89630127  lbz r11, 0x127(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(295 as u32) ) } as u64;
	// 824A57FC: 556B077E  clrlwi r11, r11, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 824A5800: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A5804: 409A0010  bne cr6, 0x824a5814
	if !ctx.cr[6].eq {
	pc = 0x824A5814; continue 'dispatch;
	}
	// 824A5808: 89630125  lbz r11, 0x125(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(293 as u32) ) } as u64;
	// 824A580C: 696B0002  xori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 ^ 2;
	// 824A5810: 99630125  stb r11, 0x125(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(293 as u32), ctx.r[11].u8 ) };
	// 824A5814: 89630127  lbz r11, 0x127(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(295 as u32) ) } as u64;
	// 824A5818: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 824A581C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A5820: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A5824(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A5824 size=24
    let mut pc: u32 = 0x824A5824;
    'dispatch: loop {
        match pc {
            0x824A5824 => {
    //   block [0x824A5824..0x824A583C)
	// 824A5824: 89630126  lbz r11, 0x126(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(294 as u32) ) } as u64;
	// 824A5828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824A582C: 216B0001  subfic r11, r11, 1
	ctx.xer.ca = ctx.r[11].u32 <= 1 as u32;
	ctx.r[11].s64 = (1 as i64) - ctx.r[11].s64;
	// 824A5830: 99430127  stb r10, 0x127(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(295 as u32), ctx.r[10].u8 ) };
	// 824A5834: 99630126  stb r11, 0x126(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(294 as u32), ctx.r[11].u8 ) };
	// 824A5838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A5840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A5840 size=16
    let mut pc: u32 = 0x824A5840;
    'dispatch: loop {
        match pc {
            0x824A5840 => {
    //   block [0x824A5840..0x824A5850)
	// 824A5840: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824A5844: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824A5848: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824A584C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A5850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824A5850 size=16
    let mut pc: u32 = 0x824A5850;
    'dispatch: loop {
        match pc {
            0x824A5850 => {
    //   block [0x824A5850..0x824A5860)
	// 824A5850: C0030014  lfs f0, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A5854: EC00082A  fadds f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 824A5858: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824A585C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A5860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824A5860 size=32
    let mut pc: u32 = 0x824A5860;
    'dispatch: loop {
        match pc {
            0x824A5860 => {
    //   block [0x824A5860..0x824A5880)
	// 824A5860: C0030014  lfs f0, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A5864: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824A5868: C1A30020  lfs f13, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A586C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 824A5870: 419A0008  beq cr6, 0x824a5878
	if ctx.cr[6].eq {
	pc = 0x824A5878; continue 'dispatch;
	}
	// 824A5874: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A5878: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 824A587C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A5880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824A5880 size=32
    let mut pc: u32 = 0x824A5880;
    'dispatch: loop {
        match pc {
            0x824A5880 => {
    //   block [0x824A5880..0x824A58A0)
	// 824A5880: C0030018  lfs f0, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A5884: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824A5888: C1A30014  lfs f13, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A588C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 824A5890: 419A0008  beq cr6, 0x824a5898
	if ctx.cr[6].eq {
	pc = 0x824A5898; continue 'dispatch;
	}
	// 824A5894: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A5898: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 824A589C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A58A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824A58A0 size=68
    let mut pc: u32 = 0x824A58A0;
    'dispatch: loop {
        match pc {
            0x824A58A0 => {
    //   block [0x824A58A0..0x824A58E4)
	// 824A58A0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824A58A4: C0030020  lfs f0, 0x20(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A58A8: C18B2074  lfs f12, 0x2074(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8308 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824A58AC: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 824A58B0: 419A0020  beq cr6, 0x824a58d0
	if ctx.cr[6].eq {
	pc = 0x824A58D0; continue 'dispatch;
	}
	// 824A58B4: C1A30018  lfs f13, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A58B8: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 824A58BC: C1630024  lfs f11, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824A58C0: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 824A58C4: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 824A58C8: 40980008  bge cr6, 0x824a58d0
	if !ctx.cr[6].lt {
	pc = 0x824A58D0; continue 'dispatch;
	}
	// 824A58CC: D1A30020  stfs f13, 0x20(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824A58D0: C0030020  lfs f0, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A58D4: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 824A58D8: 409A000C  bne cr6, 0x824a58e4
	if !ctx.cr[6].eq {
		sub_824A58E4(ctx, base);
		return;
	}
	// 824A58DC: C0230018  lfs f1, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824A58E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A58E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824A58E4 size=16
    let mut pc: u32 = 0x824A58E4;
    'dispatch: loop {
        match pc {
            0x824A58E4 => {
    //   block [0x824A58E4..0x824A58F4)
	// 824A58E4: C1A30018  lfs f13, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A58E8: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 824A58EC: FC2C682E  fsel f1, f12, f0, f13
	ctx.f[1].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 824A58F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A58F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A58F8 size=164
    let mut pc: u32 = 0x824A58F8;
    'dispatch: loop {
        match pc {
            0x824A58F8 => {
    //   block [0x824A58F8..0x824A599C)
	// 824A58F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A58FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824A5900: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824A5904: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A5908: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824A590C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824A5910: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A5914: 419A000C  beq cr6, 0x824a5920
	if ctx.cr[6].eq {
	pc = 0x824A5920; continue 'dispatch;
	}
	// 824A5918: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 824A591C: 409A0018  bne cr6, 0x824a5934
	if !ctx.cr[6].eq {
	pc = 0x824A5934; continue 'dispatch;
	}
	// 824A5920: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A5924: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824A5928: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824A592C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A5930: 4E800421  bctrl
	ctx.lr = 0x824A5934;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A5934: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824A5938: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A593C: 419A000C  beq cr6, 0x824a5948
	if ctx.cr[6].eq {
	pc = 0x824A5948; continue 'dispatch;
	}
	// 824A5940: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 824A5944: 409A0018  bne cr6, 0x824a595c
	if !ctx.cr[6].eq {
	pc = 0x824A595C; continue 'dispatch;
	}
	// 824A5948: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A594C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824A5950: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824A5954: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A5958: 4E800421  bctrl
	ctx.lr = 0x824A595C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A595C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824A5960: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A5964: 419A000C  beq cr6, 0x824a5970
	if ctx.cr[6].eq {
	pc = 0x824A5970; continue 'dispatch;
	}
	// 824A5968: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 824A596C: 409A0018  bne cr6, 0x824a5984
	if !ctx.cr[6].eq {
	pc = 0x824A5984; continue 'dispatch;
	}
	// 824A5970: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A5974: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824A5978: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824A597C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A5980: 4E800421  bctrl
	ctx.lr = 0x824A5984;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A5984: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824A5988: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824A598C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824A5990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824A5994: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824A5998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A59A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A59A0 size=12
    let mut pc: u32 = 0x824A59A0;
    'dispatch: loop {
        match pc {
            0x824A59A0 => {
    //   block [0x824A59A0..0x824A59AC)
	// 824A59A0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 824A59A4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 824A59A8: 4802ED50  b 0x824d46f8
	sub_824D46F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A59B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824A59B0 size=84
    let mut pc: u32 = 0x824A59B0;
    'dispatch: loop {
        match pc {
            0x824A59B0 => {
    //   block [0x824A59B0..0x824A5A04)
	// 824A59B0: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824A59B4: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 824A59B8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824A59BC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824A59C0: 396BE39C  addi r11, r11, -0x1c64
	ctx.r[11].s64 = ctx.r[11].s64 + -7268;
	// 824A59C4: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A59C8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824A59CC: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824A59D0: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824A59D4: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 824A59D8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824A59DC: 99230010  stb r9, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u8 ) };
	// 824A59E0: C1AA2074  lfs f13, 0x2074(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8308 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A59E4: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824A59E8: D1A30020  stfs f13, 0x20(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824A59EC: C18A20AC  lfs f12, 0x20ac(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8364 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824A59F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824A59F4: D1830024  stfs f12, 0x24(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 824A59F8: 91430028  stw r10, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 824A59FC: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824A5A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A5A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A5A08 size=4
    let mut pc: u32 = 0x824A5A08;
    'dispatch: loop {
        match pc {
            0x824A5A08 => {
    //   block [0x824A5A08..0x824A5A0C)
	// 824A5A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A5A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824A5A10 size=456
    let mut pc: u32 = 0x824A5A10;
    'dispatch: loop {
        match pc {
            0x824A5A10 => {
    //   block [0x824A5A10..0x824A5BD8)
	// 824A5A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A5A14: 4808F6A9  bl 0x825350bc
	ctx.lr = 0x824A5A18;
	sub_82535080(ctx, base);
	// 824A5A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A5A1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824A5A20: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824A5A24: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824A5A28: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5A2C: 4BFF31F5  bl 0x82498c20
	ctx.lr = 0x824A5A30;
	sub_82498C20(ctx, base);
	// 824A5A30: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824A5A34: C01F0018  lfs f0, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A5A38: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 824A5A3C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824A5A40: 409A0038  bne cr6, 0x824a5a78
	if !ctx.cr[6].eq {
	pc = 0x824A5A78; continue 'dispatch;
	}
	// 824A5A44: C1BF0014  lfs f13, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A5A48: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 824A5A4C: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 824A5A50: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 824A5A54: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A5A58: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 824A5A5C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 824A5A60: 419A0010  beq cr6, 0x824a5a70
	if ctx.cr[6].eq {
	pc = 0x824A5A70; continue 'dispatch;
	}
	// 824A5A64: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824A5A68: C1AB1850  lfs f13, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A5A6C: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 824A5A70: D1A1005C  stfs f13, 0x5c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 824A5A74: 48000038  b 0x824a5aac
	pc = 0x824A5AAC; continue 'dispatch;
	// 824A5A78: C1BF001C  lfs f13, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A5A7C: EDAD002A  fadds f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 824A5A80: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 824A5A84: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 824A5A88: EDAD0028  fsubs f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 824A5A8C: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A5A90: D1A10058  stfs f13, 0x58(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 824A5A94: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824A5A98: 419A0010  beq cr6, 0x824a5aa8
	if ctx.cr[6].eq {
	pc = 0x824A5AA8; continue 'dispatch;
	}
	// 824A5A9C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824A5AA0: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A5AA4: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 824A5AA8: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 824A5AAC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A5AB0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824A5AB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824A5AB8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 824A5ABC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A5AC0: 4E800421  bctrl
	ctx.lr = 0x824A5AC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A5AC4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824A5AC8: 816B9004  lwz r11, -0x6ffc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28668 as u32) ) } as u64;
	// 824A5ACC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5AD0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824A5AD4: 409A0048  bne cr6, 0x824a5b1c
	if !ctx.cr[6].eq {
	pc = 0x824A5B1C; continue 'dispatch;
	}
	// 824A5AD8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A5ADC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 824A5AE0: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824A5AE4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5AE8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5AEC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A5AF0: 40980020  bge cr6, 0x824a5b10
	if !ctx.cr[6].lt {
	pc = 0x824A5B10; continue 'dispatch;
	}
	// 824A5AF4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824A5AF8: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 824A5AFC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A5B00: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A5B04: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A5B08: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A5B0C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A5B10: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824A5B14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824A5B18: 4808F5F4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 824A5B1C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824A5B20: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 824A5B24: 409A00A0  bne cr6, 0x824a5bc4
	if !ctx.cr[6].eq {
	pc = 0x824A5BC4; continue 'dispatch;
	}
	// 824A5B28: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5B2C: C01F0018  lfs f0, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A5B30: C1BF001C  lfs f13, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A5B34: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 824A5B38: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824A5B3C: 816B0154  lwz r11, 0x154(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(340 as u32) ) } as u64;
	// 824A5B40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A5B44: 419A0078  beq cr6, 0x824a5bbc
	if ctx.cr[6].eq {
	pc = 0x824A5BBC; continue 'dispatch;
	}
	// 824A5B48: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A5B4C: 3BC00014  li r30, 0x14
	ctx.r[30].s64 = 20;
	// 824A5B50: 7D5EE82E  lwzx r10, r30, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824A5B54: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5B58: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5B5C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A5B60: 40980020  bge cr6, 0x824a5b80
	if !ctx.cr[6].lt {
	pc = 0x824A5B80; continue 'dispatch;
	}
	// 824A5B64: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824A5B68: 392906F0  addi r9, r9, 0x6f0
	ctx.r[9].s64 = ctx.r[9].s64 + 1776;
	// 824A5B6C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A5B70: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A5B74: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A5B78: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A5B7C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A5B80: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824A5B84: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5B88: 4BFFF7C1  bl 0x824a5348
	ctx.lr = 0x824A5B8C;
	sub_824A5348(ctx, base);
	// 824A5B8C: 7D5EE82E  lwzx r10, r30, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824A5B90: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5B94: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5B98: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A5B9C: 40980020  bge cr6, 0x824a5bbc
	if !ctx.cr[6].lt {
	pc = 0x824A5BBC; continue 'dispatch;
	}
	// 824A5BA0: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824A5BA4: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 824A5BA8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A5BAC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A5BB0: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A5BB4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A5BB8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A5BBC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824A5BC0: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 824A5BC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A5BC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824A5BCC: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 824A5BD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824A5BD4: 4808F538  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A5BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824A5BD8 size=248
    let mut pc: u32 = 0x824A5BD8;
    'dispatch: loop {
        match pc {
            0x824A5BD8 => {
    //   block [0x824A5BD8..0x824A5CD0)
	// 824A5BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A5BDC: 4808F4E1  bl 0x825350bc
	ctx.lr = 0x824A5BE0;
	sub_82535080(ctx, base);
	// 824A5BE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A5BE4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824A5BE8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824A5BEC: C01D0020  lfs f0, 0x20(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A5BF0: C18B2074  lfs f12, 0x2074(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8308 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824A5BF4: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 824A5BF8: 419A0020  beq cr6, 0x824a5c18
	if ctx.cr[6].eq {
	pc = 0x824A5C18; continue 'dispatch;
	}
	// 824A5BFC: C1BD0018  lfs f13, 0x18(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A5C00: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 824A5C04: C17D0024  lfs f11, 0x24(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824A5C08: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 824A5C0C: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 824A5C10: 40980008  bge cr6, 0x824a5c18
	if !ctx.cr[6].lt {
	pc = 0x824A5C18; continue 'dispatch;
	}
	// 824A5C14: D1BD0020  stfs f13, 0x20(r29)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824A5C18: C01D0020  lfs f0, 0x20(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A5C1C: C1BD0018  lfs f13, 0x18(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A5C20: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 824A5C24: 419A000C  beq cr6, 0x824a5c30
	if ctx.cr[6].eq {
	pc = 0x824A5C30; continue 'dispatch;
	}
	// 824A5C28: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 824A5C2C: FDAC682E  fsel f13, f12, f0, f13
	ctx.f[13].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 824A5C30: D1BD0014  stfs f13, 0x14(r29)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824A5C34: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824A5C38: 41980084  blt cr6, 0x824a5cbc
	if ctx.cr[6].lt {
	pc = 0x824A5CBC; continue 'dispatch;
	}
	// 824A5C3C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5C40: 816B013C  lwz r11, 0x13c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(316 as u32) ) } as u64;
	// 824A5C44: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A5C48: 419A0074  beq cr6, 0x824a5cbc
	if ctx.cr[6].eq {
	pc = 0x824A5CBC; continue 'dispatch;
	}
	// 824A5C4C: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A5C50: 3BE00014  li r31, 0x14
	ctx.r[31].s64 = 20;
	// 824A5C54: 7D5FF02E  lwzx r10, r31, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824A5C58: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5C5C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5C60: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A5C64: 40980020  bge cr6, 0x824a5c84
	if !ctx.cr[6].lt {
	pc = 0x824A5C84; continue 'dispatch;
	}
	// 824A5C68: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824A5C6C: 39290700  addi r9, r9, 0x700
	ctx.r[9].s64 = ctx.r[9].s64 + 1792;
	// 824A5C70: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A5C74: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A5C78: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A5C7C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A5C80: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A5C84: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5C88: 4BFFF4E1  bl 0x824a5168
	ctx.lr = 0x824A5C8C;
	sub_824A5168(ctx, base);
	// 824A5C8C: 7D5FF02E  lwzx r10, r31, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824A5C90: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5C94: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5C98: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A5C9C: 40980020  bge cr6, 0x824a5cbc
	if !ctx.cr[6].lt {
	pc = 0x824A5CBC; continue 'dispatch;
	}
	// 824A5CA0: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824A5CA4: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 824A5CA8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A5CAC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A5CB0: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A5CB4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A5CB8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A5CBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A5CC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824A5CC4: 917D0028  stw r11, 0x28(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 824A5CC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824A5CCC: 4808F440  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A5CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A5CD0 size=156
    let mut pc: u32 = 0x824A5CD0;
    'dispatch: loop {
        match pc {
            0x824A5CD0 => {
    //   block [0x824A5CD0..0x824A5D6C)
	// 824A5CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A5CD4: 4808F3D9  bl 0x825350ac
	ctx.lr = 0x824A5CD8;
	sub_82535080(ctx, base);
	// 824A5CD8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A5CDC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824A5CE0: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 824A5CE4: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5CE8: 3B4B0028  addi r26, r11, 0x28
	ctx.r[26].s64 = ctx.r[11].s64 + 40;
	// 824A5CEC: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5CF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A5CF4: 40990070  ble cr6, 0x824a5d64
	if !ctx.cr[6].gt {
	pc = 0x824A5D64; continue 'dispatch;
	}
	// 824A5CF8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 824A5CFC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A5D00: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824A5D04: 7D6BD82E  lwzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 824A5D08: 3BAB0038  addi r29, r11, 0x38
	ctx.r[29].s64 = ctx.r[11].s64 + 56;
	// 824A5D0C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5D10: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A5D14: 4099003C  ble cr6, 0x824a5d50
	if !ctx.cr[6].gt {
	pc = 0x824A5D50; continue 'dispatch;
	}
	// 824A5D18: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824A5D1C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A5D20: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5D24: 388B0190  addi r4, r11, 0x190
	ctx.r[4].s64 = ctx.r[11].s64 + 400;
	// 824A5D28: 7C6AF02E  lwzx r3, r10, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824A5D2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A5D30: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5D34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A5D38: 4E800421  bctrl
	ctx.lr = 0x824A5D3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A5D3C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5D40: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 824A5D44: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 824A5D48: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824A5D4C: 4198FFD0  blt cr6, 0x824a5d1c
	if ctx.cr[6].lt {
	pc = 0x824A5D1C; continue 'dispatch;
	}
	// 824A5D50: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5D54: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 824A5D58: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 824A5D5C: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824A5D60: 4198FF9C  blt cr6, 0x824a5cfc
	if ctx.cr[6].lt {
	pc = 0x824A5CFC; continue 'dispatch;
	}
	// 824A5D64: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824A5D68: 4808F394  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A5D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A5D70 size=132
    let mut pc: u32 = 0x824A5D70;
    'dispatch: loop {
        match pc {
            0x824A5D70 => {
    //   block [0x824A5D70..0x824A5DF4)
	// 824A5D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A5D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824A5D78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824A5D7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824A5D80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A5D84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824A5D88: 3BFE00E0  addi r31, r30, 0xe0
	ctx.r[31].s64 = ctx.r[30].s64 + 224;
	// 824A5D8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824A5D90: 48105871  bl 0x825ab600
	ctx.lr = 0x824A5D94;
	sub_825AB600(ctx, base);
	// 824A5D94: 397E0160  addi r11, r30, 0x160
	ctx.r[11].s64 = ctx.r[30].s64 + 352;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A5DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824A5DF8 size=528
    let mut pc: u32 = 0x824A5DF8;
    'dispatch: loop {
        match pc {
            0x824A5DF8 => {
    //   block [0x824A5DF8..0x824A6008)
	// 824A5DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A5DFC: 4808F2BD  bl 0x825350b8
	ctx.lr = 0x824A5E00;
	sub_82535080(ctx, base);
	// 824A5E00: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A5E04: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A5E08: 3BC00014  li r30, 0x14
	ctx.r[30].s64 = 20;
	// 824A5E0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824A5E10: 7D5EE82E  lwzx r10, r30, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824A5E14: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5E18: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5E1C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A5E20: 40980020  bge cr6, 0x824a5e40
	if !ctx.cr[6].lt {
	pc = 0x824A5E40; continue 'dispatch;
	}
	// 824A5E24: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824A5E28: 39290728  addi r9, r9, 0x728
	ctx.r[9].s64 = ctx.r[9].s64 + 1832;
	// 824A5E2C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A5E30: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A5E34: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A5E38: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A5E3C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A5E40: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824A5E44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A5E48: 419A0044  beq cr6, 0x824a5e8c
	if ctx.cr[6].eq {
	pc = 0x824A5E8C; continue 'dispatch;
	}
	// 824A5E4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824A5E50: 4BFFFBC1  bl 0x824a5a10
	ctx.lr = 0x824A5E54;
	sub_824A5A10(ctx, base);
	// 824A5E54: 7D5EE82E  lwzx r10, r30, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824A5E58: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5E5C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5E60: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A5E64: 4098019C  bge cr6, 0x824a6000
	if !ctx.cr[6].lt {
	pc = 0x824A6000; continue 'dispatch;
	}
	// 824A5E68: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824A5E6C: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 824A5E70: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A5E74: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A5E78: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A5E7C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A5E80: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A5E84: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824A5E88: 4808F280  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 824A5E8C: C01F0018  lfs f0, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A5E90: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824A5E94: C1BF001C  lfs f13, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A5E98: EDA0682A  fadds f13, f0, f13
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 824A5E9C: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 824A5EA0: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 824A5EA4: EDAD0028  fsubs f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 824A5EA8: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A5EAC: D1A10058  stfs f13, 0x58(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 824A5EB0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824A5EB4: 419A0010  beq cr6, 0x824a5ec4
	if ctx.cr[6].eq {
	pc = 0x824A5EC4; continue 'dispatch;
	}
	// 824A5EB8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824A5EBC: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A5EC0: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 824A5EC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A5EC8: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 824A5ECC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824A5ED0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824A5ED4: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 824A5ED8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A5EDC: 4E800421  bctrl
	ctx.lr = 0x824A5EE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A5EE0: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824A5EE4: 816B9004  lwz r11, -0x6ffc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28668 as u32) ) } as u64;
	// 824A5EE8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5EEC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824A5EF0: 409A0048  bne cr6, 0x824a5f38
	if !ctx.cr[6].eq {
	pc = 0x824A5F38; continue 'dispatch;
	}
	// 824A5EF4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 824A5EF8: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 824A5EFC: 7D5EE82E  lwzx r10, r30, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824A5F00: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5F04: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5F08: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A5F0C: 40980020  bge cr6, 0x824a5f2c
	if !ctx.cr[6].lt {
	pc = 0x824A5F2C; continue 'dispatch;
	}
	// 824A5F10: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824A5F14: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 824A5F18: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A5F1C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A5F20: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A5F24: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A5F28: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A5F2C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 824A5F30: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824A5F34: 4808F1D4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 824A5F38: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5F3C: C01F0018  lfs f0, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A5F40: C1BF001C  lfs f13, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A5F44: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 824A5F48: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824A5F4C: 816B0154  lwz r11, 0x154(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(340 as u32) ) } as u64;
	// 824A5F50: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A5F54: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824A5F58: 3B8B74DC  addi r28, r11, 0x74dc
	ctx.r[28].s64 = ctx.r[11].s64 + 29916;
	// 824A5F5C: 419A0068  beq cr6, 0x824a5fc4
	if ctx.cr[6].eq {
	pc = 0x824A5FC4; continue 'dispatch;
	}
	// 824A5F60: 7D5EE82E  lwzx r10, r30, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824A5F64: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5F68: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5F6C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A5F70: 40980020  bge cr6, 0x824a5f90
	if !ctx.cr[6].lt {
	pc = 0x824A5F90; continue 'dispatch;
	}
	// 824A5F74: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824A5F78: 392906F0  addi r9, r9, 0x6f0
	ctx.r[9].s64 = ctx.r[9].s64 + 1776;
	// 824A5F7C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A5F80: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A5F84: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A5F88: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A5F8C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A5F90: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824A5F94: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5F98: 4BFFF3B1  bl 0x824a5348
	ctx.lr = 0x824A5F9C;
	sub_824A5348(ctx, base);
	// 824A5F9C: 7D5EE82E  lwzx r10, r30, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824A5FA0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5FA4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5FA8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A5FAC: 40980018  bge cr6, 0x824a5fc4
	if !ctx.cr[6].lt {
	pc = 0x824A5FC4; continue 'dispatch;
	}
	// 824A5FB0: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 824A5FB4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A5FB8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A5FBC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A5FC0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A5FC4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824A5FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824A5FCC: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 824A5FD0: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 824A5FD4: 7D5EE82E  lwzx r10, r30, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824A5FD8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A5FDC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A5FE0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A5FE4: 40980018  bge cr6, 0x824a5ffc
	if !ctx.cr[6].lt {
	pc = 0x824A5FFC; continue 'dispatch;
	}
	// 824A5FE8: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 824A5FEC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A5FF0: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A5FF4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A5FF8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A5FFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824A6000: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824A6004: 4808F104  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A6008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824A6008 size=1212
    let mut pc: u32 = 0x824A6008;
    'dispatch: loop {
        match pc {
            0x824A6008 => {
    //   block [0x824A6008..0x824A64C4)
	// 824A6008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A600C: 4808F08D  bl 0x82535098
	ctx.lr = 0x824A6010;
	sub_82535080(ctx, base);
	// 824A6010: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A6014: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A6018: 3B600014  li r27, 0x14
	ctx.r[27].s64 = 20;
	// 824A601C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824A6020: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824A6024: 7D5BD02E  lwzx r10, r27, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 824A6028: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A602C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A6030: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A6034: 4098002C  bge cr6, 0x824a6060
	if !ctx.cr[6].lt {
	pc = 0x824A6060; continue 'dispatch;
	}
	// 824A6038: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824A603C: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824A6040: 39290784  addi r9, r9, 0x784
	ctx.r[9].s64 = ctx.r[9].s64 + 1924;
	// 824A6044: 3908077C  addi r8, r8, 0x77c
	ctx.r[8].s64 = ctx.r[8].s64 + 1916;
	// 824A6048: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A604C: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824A6050: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A6054: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 824A6058: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A605C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A6060: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A6064: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824A6068: E93C0000  ld r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	// 824A606C: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 824A6070: 396B0190  addi r11, r11, 0x190
	ctx.r[11].s64 = ctx.r[11].s64 + 400;
	// 824A6074: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 824A6078: F92B0000  std r9, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 824A607C: E93C0008  ld r9, 8(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) };
	// 824A6080: F92B0008  std r9, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u64 ) };
	// 824A6084: C01C0008  lfs f0, 8(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A6088: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A608C: 396B01A0  addi r11, r11, 0x1a0
	ctx.r[11].s64 = ctx.r[11].s64 + 416;
	// 824A6090: 392B010C  addi r9, r11, 0x10c
	ctx.r[9].s64 = ctx.r[11].s64 + 268;
	// 824A6094: 80CB0114  lwz r6, 0x114(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(276 as u32) ) } as u64;
	// 824A6098: C1AB0120  lfs f13, 0x120(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(288 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A609C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 824A60A0: D00B010C  stfs f0, 0x10c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(268 as u32), tmp.u32 ) };
	// 824A60A4: 7CC607B4  extsw r6, r6
	ctx.r[6].s64 = ctx.r[6].s32 as i64;
	// 824A60A8: C01C000C  lfs f0, 0xc(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A60AC: F8C10050  std r6, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u64 ) };
	// 824A60B0: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 824A60B4: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 824A60B8: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 824A60BC: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 824A60C0: D00B0110  stfs f0, 0x110(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), tmp.u32 ) };
	// 824A60C4: 80DE000C  lwz r6, 0xc(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A64C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824A64C8 size=236
    let mut pc: u32 = 0x824A64C8;
    'dispatch: loop {
        match pc {
            0x824A64C8 => {
    //   block [0x824A64C8..0x824A65B4)
	// 824A64C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A64CC: 4808EBE9  bl 0x825350b4
	ctx.lr = 0x824A64D0;
	sub_82535080(ctx, base);
	// 824A64D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A64D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824A64D8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 824A64DC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824A64E0: 3B7F0010  addi r27, r31, 0x10
	ctx.r[27].s64 = ctx.r[31].s64 + 16;
	// 824A64E4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824A64E8: C00B8CB4  lfs f0, -0x734c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A64EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A64F0: D01F3030  stfs f0, 0x3030(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12336 as u32), tmp.u32 ) };
	// 824A64F4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824A64F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824A64FC: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 824A6500: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824A6504: 917F3080  stw r11, 0x3080(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12416 as u32), ctx.r[11].u32 ) };
	// 824A6508: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A650C: 394B1C60  addi r10, r11, 0x1c60
	ctx.r[10].s64 = ctx.r[11].s64 + 7264;
	// 824A6510: 915E0060  stw r10, 0x60(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 824A6514: 895D000C  lbz r10, 0xc(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A6518: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 824A651C: 554A3032  slwi r10, r10, 6
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824A6520: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824A6524: 816B1C30  lwz r11, 0x1c30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7216 as u32) ) } as u64;
	// 824A6528: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824A652C: 80DD0008  lwz r6, 8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A6530: 4802F3C9  bl 0x824d58f8
	ctx.lr = 0x824A6534;
	sub_824D58F8(ctx, base);
	// 824A6534: 3F808293  lis r28, -0x7d6d
	ctx.r[28].s64 = -2104295424;
	// 824A6538: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824A653C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824A6540: 809C9004  lwz r4, -0x6ffc(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-28668 as u32) ) } as u64;
	// 824A6544: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A6548: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 824A654C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A6550: 4E800421  bctrl
	ctx.lr = 0x824A6554;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A6554: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A6558: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A655C: 409A0010  bne cr6, 0x824a656c
	if !ctx.cr[6].eq {
	pc = 0x824A656C; continue 'dispatch;
	}
	// 824A6560: 817C9004  lwz r11, -0x6ffc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-28668 as u32) ) } as u64;
	// 824A6564: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824A6568: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824A656C: 817C9004  lwz r11, -0x6ffc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-28668 as u32) ) } as u64;
	// 824A6570: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A6574: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824A6578: 419A0034  beq cr6, 0x824a65ac
	if ctx.cr[6].eq {
	pc = 0x824A65AC; continue 'dispatch;
	}
	// 824A657C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A6580: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 824A6584: 419A0028  beq cr6, 0x824a65ac
	if ctx.cr[6].eq {
	pc = 0x824A65AC; continue 'dispatch;
	}
	// 824A6588: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A658C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 824A6590: 80BD0014  lwz r5, 0x14(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 824A6594: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824A6598: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 824A659C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A65A0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824A65A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A65A8: 4E800421  bctrl
	ctx.lr = 0x824A65AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A65AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824A65B0: 4808EB54  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A65B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824A65B8 size=468
    let mut pc: u32 = 0x824A65B8;
    'dispatch: loop {
        match pc {
            0x824A65B8 => {
    //   block [0x824A65B8..0x824A678C)
	// 824A65B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A65BC: 4808EAFD  bl 0x825350b8
	ctx.lr = 0x824A65C0;
	sub_82535080(ctx, base);
	// 824A65C0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A65C4: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A65C8: 3B800014  li r28, 0x14
	ctx.r[28].s64 = 20;
	// 824A65CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824A65D0: 7D5EE02E  lwzx r10, r30, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 824A65D4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A65D8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A65DC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A65E0: 40980020  bge cr6, 0x824a6600
	if !ctx.cr[6].lt {
	pc = 0x824A6600; continue 'dispatch;
	}
	// 824A65E4: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824A65E8: 39290728  addi r9, r9, 0x728
	ctx.r[9].s64 = ctx.r[9].s64 + 1832;
	// 824A65EC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A65F0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A65F4: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A65F8: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A65FC: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A6600: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A6604: D03F001C  stfs f1, 0x1c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 824A6608: 4BFEE341  bl 0x82494948
	ctx.lr = 0x824A660C;
	sub_82494948(ctx, base);
	// 824A660C: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824A6610: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824A6614: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824A6618: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 824A661C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824A6620: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 824A6624: 396BFFF0  addi r11, r11, -0x10
	ctx.r[11].s64 = ctx.r[11].s64 + -16;
	// 824A6628: 7F055800  cmpw cr6, r5, r11
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824A662C: 40990078  ble cr6, 0x824a66a4
	if !ctx.cr[6].gt {
	pc = 0x824A66A4; continue 'dispatch;
	}
	// 824A6630: 3FA08293  lis r29, -0x7d6d
	ctx.r[29].s64 = -2104295424;
	// 824A6634: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824A6638: 809D9004  lwz r4, -0x6ffc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-28668 as u32) ) } as u64;
	// 824A663C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A6640: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 824A6644: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A6648: 4E800421  bctrl
	ctx.lr = 0x824A664C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A664C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A6650: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A6654: 409A0050  bne cr6, 0x824a66a4
	if !ctx.cr[6].eq {
	pc = 0x824A66A4; continue 'dispatch;
	}
	// 824A6658: 817D9004  lwz r11, -0x6ffc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-28668 as u32) ) } as u64;
	// 824A665C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824A6660: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824A6664: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 824A6668: 7D7EE02E  lwzx r11, r30, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 824A666C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A6670: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A6674: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A6678: 40980020  bge cr6, 0x824a6698
	if !ctx.cr[6].lt {
	pc = 0x824A6698; continue 'dispatch;
	}
	// 824A667C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824A6680: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 824A6684: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A6688: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A668C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 824A6690: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A6694: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A6698: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824A669C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824A66A0: 4808EA68  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 824A66A4: C01F0018  lfs f0, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A66A8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824A66AC: C1BF001C  lfs f13, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A66B0: EDA0682A  fadds f13, f0, f13
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 824A66B4: D1A10064  stfs f13, 0x64(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 824A66B8: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 824A66BC: EDAD0028  fsubs f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 824A66C0: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A66C4: D1A10068  stfs f13, 0x68(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 824A66C8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824A66CC: 419A0010  beq cr6, 0x824a66dc
	if ctx.cr[6].eq {
	pc = 0x824A66DC; continue 'dispatch;
	}
	// 824A66D0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824A66D4: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A66D8: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 824A66DC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 824A66E0: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A66E4: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 824A66E8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 824A66EC: 39290190  addi r9, r9, 0x190
	ctx.r[9].s64 = ctx.r[9].s64 + 400;
	// 824A66F0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 824A66F4: E90B0000  ld r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 824A66F8: F9090000  std r8, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 824A66FC: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 824A6700: F9690008  std r11, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 824A6704: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A6708: E92A0000  ld r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 824A670C: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 824A6710: 396B0050  addi r11, r11, 0x50
	ctx.r[11].s64 = ctx.r[11].s64 + 80;
	// 824A6714: F92B0000  std r9, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 824A6718: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 824A671C: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 824A6720: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A6724: 8064004C  lwz r3, 0x4c(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 824A6728: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A672C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824A6730: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A6734: 4E800421  bctrl
	ctx.lr = 0x824A6738;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A6738: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 824A673C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824A6740: 4BFFF8C9  bl 0x824a6008
	ctx.lr = 0x824A6744;
	sub_824A6008(ctx, base);
	// 824A6744: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A6748: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 824A674C: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 824A6750: 7D7EE02E  lwzx r11, r30, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 824A6754: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A6758: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A675C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A6760: 40980020  bge cr6, 0x824a6780
	if !ctx.cr[6].lt {
	pc = 0x824A6780; continue 'dispatch;
	}
	// 824A6764: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824A6768: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 824A676C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A6770: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A6774: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 824A6778: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A677C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A6780: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824A6784: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824A6788: 4808E980  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A6790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824A6790 size=1856
    let mut pc: u32 = 0x824A6790;
    'dispatch: loop {
        match pc {
            0x824A6790 => {
    //   block [0x824A6790..0x824A6ED0)
	// 824A6790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A6794: 4808E8ED  bl 0x82535080
	ctx.lr = 0x824A6798;
	sub_82535080(ctx, base);
	// 824A6798: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 824A679C: 4808F82D  bl 0x82535fc8
	ctx.lr = 0x824A67A0;
	sub_82535FB0(ctx, base);
	// 824A67A0: 3981FF00  addi r12, r1, -0x100
	ctx.r[12].s64 = ctx.r[1].s64 + -256;
	// 824A67A4: 48092C01  bl 0x825393a4
	ctx.lr = 0x824A67A8;
	sub_82539130(ctx, base);
	// 824A67A8: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 824A67AC: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 824A67B0: E981D000  ld r12, -0x3000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-12288 as u32) ) };
	// 824A67B4: 9421CC70  stwu r1, -0x3390(r1)
	ea = ctx.r[1].u32.wrapping_add(-13200 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A67B8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824A67BC: 910133CC  stw r8, 0x33cc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(13260 as u32), ctx.r[8].u32 ) };
	// 824A67C0: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 824A67C4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824A67C8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824A67CC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824A67D0: 93A133B4  stw r29, 0x33b4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(13236 as u32), ctx.r[29].u32 ) };
	// 824A67D4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 824A67D8: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 824A67DC: 4BFC6AA5  bl 0x8246d280
	ctx.lr = 0x824A67E0;
	sub_8246D280(ctx, base);
	// 824A67E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824A67E4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824A67E8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824A67EC: C00B8CB4  lfs f0, -0x734c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A67F0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824A67F4: D00131F0  stfs f0, 0x31f0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(12784 as u32), tmp.u32 ) };
	// 824A67F8: 938101C4  stw r28, 0x1c4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(452 as u32), ctx.r[28].u32 ) };
	// 824A67FC: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 824A6800: C2EB1FF8  lfs f23, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 824A6804: D2E13210  stfs f23, 0x3210(r1)
	tmp.f32 = (ctx.f[23].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(12816 as u32), tmp.u32 ) };
	// 824A6808: D2E13214  stfs f23, 0x3214(r1)
	tmp.f32 = (ctx.f[23].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(12820 as u32), tmp.u32 ) };
	// 824A680C: 409906A4  ble cr6, 0x824a6eb0
	if !ctx.cr[6].gt {
	pc = 0x824A6EB0; continue 'dispatch;
	}
	// 824A6810: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824A6814: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 824A6818: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 824A681C: 396B9004  addi r11, r11, -0x6ffc
	ctx.r[11].s64 = ctx.r[11].s64 + -28668;
	// 824A6820: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 824A6824: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 824A6828: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 824A682C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 824A6830: C3062954  lfs f24, 0x2954(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(10580 as u32) ) };
	ctx.f[24].f64 = (tmp.f32 as f64);
	// 824A6834: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 824A6838: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824A683C: C3272938  lfs f25, 0x2938(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(10552 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 824A6840: 3AA00010  li r21, 0x10
	ctx.r[21].s64 = 16;
	// 824A6844: 396BFA60  addi r11, r11, -0x5a0
	ctx.r[11].s64 = ctx.r[11].s64 + -1440;
	// 824A6848: C2A82968  lfs f21, 0x2968(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(10600 as u32) ) };
	ctx.f[21].f64 = (tmp.f32 as f64);
	// 824A684C: C2C92298  lfs f22, 0x2298(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8856 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 824A6850: 3AC00020  li r22, 0x20
	ctx.r[22].s64 = 32;
	// 824A6854: C3AA1850  lfs f29, 0x1850(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6224 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 824A6858: 3AE00030  li r23, 0x30
	ctx.r[23].s64 = 48;
	// 824A685C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 824A6860: 835F0000  lwz r26, 0(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A6864: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824A6868: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824A686C: 3A9A0010  addi r20, r26, 0x10
	ctx.r[20].s64 = ctx.r[26].s64 + 16;
	// 824A6870: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 824A6874: 4BFC6B8D  bl 0x8246d400
	ctx.lr = 0x824A6878;
	sub_8246D400(ctx, base);
	// 824A6878: 81740050  lwz r11, 0x50(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(80 as u32) ) } as u64;
	// 824A687C: 3A200000  li r17, 0
	ctx.r[17].s64 = 0;
	// 824A6880: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A6884: 40990614  ble cr6, 0x824a6e98
	if !ctx.cr[6].gt {
	pc = 0x824A6E98; continue 'dispatch;
	}
	// 824A6888: 3A54004C  addi r18, r20, 0x4c
	ctx.r[18].s64 = ctx.r[20].s64 + 76;
	// 824A688C: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 824A6890: 81720000  lwz r11, 0(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A6894: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824A6898: 7F6B9A14  add r27, r11, r19
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[19].u64;
	// 824A689C: 809B0004  lwz r4, 4(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A68A0: 4BFC6C39  bl 0x8246d4d8
	ctx.lr = 0x824A68A4;
	sub_8246D4D8(ctx, base);
	// 824A68A4: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 824A68A8: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824A68AC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824A68B0: 40990008  ble cr6, 0x824a68b8
	if !ctx.cr[6].gt {
	pc = 0x824A68B8; continue 'dispatch;
	}
	// 824A68B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A68B8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824A68BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A68C0: 409A05B4  bne cr6, 0x824a6e74
	if !ctx.cr[6].eq {
	pc = 0x824A6E74; continue 'dispatch;
	}
	// 824A68C4: 83BB0000  lwz r29, 0(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A68C8: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 824A68CC: 815D0014  lwz r10, 0x14(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 824A68D0: 892B0010  lbz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824A68D4: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 824A68D8: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824A68DC: 892B00D8  lbz r9, 0xd8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(216 as u32) ) } as u64;
	// 824A68E0: 2B090007  cmplwi cr6, r9, 7
	ctx.cr[6].compare_u32(ctx.r[9].u32, 7 as u32, &mut ctx.xer);
	// 824A68E4: 409A0010  bne cr6, 0x824a68f4
	if !ctx.cr[6].eq {
	pc = 0x824A68F4; continue 'dispatch;
	}
	// 824A68E8: 896A0010  lbz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 824A68EC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824A68F0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824A68F4: 816B00B8  lwz r11, 0xb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(184 as u32) ) } as u64;
	// 824A68F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824A68FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824A6900: 916101C4  stw r11, 0x1c4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(452 as u32), ctx.r[11].u32 ) };
	// 824A6904: 4802DDF5  bl 0x824d46f8
	ctx.lr = 0x824A6908;
	sub_824D46F8(ctx, base);
	// 824A6908: 38C101C0  addi r6, r1, 0x1c0
	ctx.r[6].s64 = ctx.r[1].s64 + 448;
	// 824A690C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824A6910: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824A6914: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 824A6918: 7F0903A6  mtctr r24
	ctx.ctr.u64 = ctx.r[24].u64;
	// 824A691C: 4E800421  bctrl
	ctx.lr = 0x824A6920;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A6920: 816133CC  lwz r11, 0x33cc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(13260 as u32) ) } as u64;
	// 824A6924: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824A6928: 409A0538  bne cr6, 0x824a6e60
	if !ctx.cr[6].eq {
	pc = 0x824A6E60; continue 'dispatch;
	}
	// 824A692C: 814101C0  lwz r10, 0x1c0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(448 as u32) ) } as u64;
	// 824A6930: 396101D0  addi r11, r1, 0x1d0
	ctx.r[11].s64 = ctx.r[1].s64 + 464;
	// 824A6934: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824A6938: 419A0528  beq cr6, 0x824a6e60
	if ctx.cr[6].eq {
	pc = 0x824A6E60; continue 'dispatch;
	}
	// 824A693C: 3B9A0150  addi r28, r26, 0x150
	ctx.r[28].s64 = ctx.r[26].s64 + 336;
	// 824A6940: 38E101E0  addi r7, r1, 0x1e0
	ctx.r[7].s64 = ctx.r[1].s64 + 480;
	// 824A6944: 38C10150  addi r6, r1, 0x150
	ctx.r[6].s64 = ctx.r[1].s64 + 336;
	// 824A6948: 38A100B0  addi r5, r1, 0xb0
	ctx.r[5].s64 = ctx.r[1].s64 + 176;
	// 824A694C: 3BFA00E0  addi r31, r26, 0xe0
	ctx.r[31].s64 = ctx.r[26].s64 + 224;
	// 824A6950: E87C0000  ld r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	// 824A6954: 39610160  addi r11, r1, 0x160
	ctx.r[11].s64 = ctx.r[1].s64 + 352;
	// 824A6958: EA1C0008  ld r16, 8(r28)
	ctx.r[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) };
	// 824A695C: 39410100  addi r10, r1, 0x100
	ctx.r[10].s64 = ctx.r[1].s64 + 256;
	// 824A6960: E8870000  ld r4, 0(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	// 824A6964: 39210170  addi r9, r1, 0x170
	ctx.r[9].s64 = ctx.r[1].s64 + 368;
	// 824A6968: E8E70008  ld r7, 8(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) };
	// 824A696C: 39010120  addi r8, r1, 0x120
	ctx.r[8].s64 = ctx.r[1].s64 + 288;
	// 824A6970: E9DF0008  ld r14, 8(r31)
	ctx.r[14].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 824A6974: F8660000  std r3, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 824A6978: FA060008  std r16, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[16].u64 ) };
	// 824A697C: F8850000  std r4, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[4].u64 ) };
	// 824A6980: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 824A6984: E8DF0000  ld r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 824A6988: F8E50008  std r7, 8(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[7].u64 ) };
	// 824A698C: 38FF0010  addi r7, r31, 0x10
	ctx.r[7].s64 = ctx.r[31].s64 + 16;
	// 824A6990: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 824A6994: E9E40000  ld r15, 0(r4)
	ctx.r[15].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 824A6998: F8CB0000  std r6, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 824A699C: E8670000  ld r3, 0(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	// 824A69A0: F9CB0008  std r14, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[14].u64 ) };
	// 824A69A4: 38C10160  addi r6, r1, 0x160
	ctx.r[6].s64 = ctx.r[1].s64 + 352;
	// 824A69A8: E8E70008  ld r7, 8(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) };
	// 824A69AC: 39610180  addi r11, r1, 0x180
	ctx.r[11].s64 = ctx.r[1].s64 + 384;
	// 824A69B0: EA050000  ld r16, 0(r5)
	ctx.r[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 824A69B4: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 824A69B8: F86A0000  std r3, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 824A69BC: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A6ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A6ED0 size=1308
    let mut pc: u32 = 0x824A6ED0;
    'dispatch: loop {
        match pc {
            0x824A6ED0 => {
    //   block [0x824A6ED0..0x824A73EC)
	// 824A6ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A6ED4: 4808E1BD  bl 0x82535090
	ctx.lr = 0x824A6ED8;
	sub_82535080(ctx, base);
	// 824A6ED8: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A6EDC: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A6EE0: 3A400014  li r18, 0x14
	ctx.r[18].s64 = 20;
	// 824A6EE4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824A6EE8: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 824A6EEC: 7CB42B78  mr r20, r5
	ctx.r[20].u64 = ctx.r[5].u64;
	// 824A6EF0: 7D5A902E  lwzx r10, r26, r18
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 824A6EF4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A6EF8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A6EFC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A6F00: 4098002C  bge cr6, 0x824a6f2c
	if !ctx.cr[6].lt {
	pc = 0x824A6F2C; continue 'dispatch;
	}
	// 824A6F04: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824A6F08: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824A6F0C: 392907DC  addi r9, r9, 0x7dc
	ctx.r[9].s64 = ctx.r[9].s64 + 2012;
	// 824A6F10: 390807D0  addi r8, r8, 0x7d0
	ctx.r[8].s64 = ctx.r[8].s64 + 2000;
	// 824A6F14: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A6F18: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824A6F1C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A6F20: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 824A6F24: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A6F28: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A6F2C: 3AC00010  li r22, 0x10
	ctx.r[22].s64 = 16;
	// 824A6F30: 83B402FC  lwz r29, 0x2fc(r20)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(764 as u32) ) } as u64;
	// 824A6F34: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 824A6F38: 3FE08000  lis r31, -0x8000
	ctx.r[31].s64 = -2147483648;
	// 824A6F3C: 395D0002  addi r10, r29, 2
	ctx.r[10].s64 = ctx.r[29].s64 + 2;
	// 824A6F40: 7D7AB02E  lwzx r11, r26, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 824A6F44: 55441836  rlwinm r4, r10, 3, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x1FFFFFFFu64;
	// 824A6F48: 92610060  stw r19, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[19].u32 ) };
	// 824A6F4C: 92610064  stw r19, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[19].u32 ) };
	// 824A6F50: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 824A6F54: 806B0020  lwz r3, 0x20(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824A6F58: 812B002C  lwz r9, 0x2c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 824A6F5C: 7D432214  add r10, r3, r4
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 824A6F60: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A6F64: 4199000C  bgt cr6, 0x824a6f70
	if ctx.cr[6].gt {
	pc = 0x824A6F70; continue 'dispatch;
	}
	// 824A6F68: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 824A6F6C: 48000018  b 0x824a6f84
	pc = 0x824A6F84; continue 'dispatch;
	// 824A6F70: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A6F74: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 824A6F78: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 824A6F7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A6F80: 4E800421  bctrl
	ctx.lr = 0x824A6F84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A6F84: 7FAAFB78  or r10, r29, r31
	ctx.r[10].u64 = ctx.r[29].u64 | ctx.r[31].u64;
	// 824A6F88: 7D7AB02E  lwzx r11, r26, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 824A6F8C: 83D402FC  lwz r30, 0x2fc(r20)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(764 as u32) ) } as u64;
	// 824A6F90: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 824A6F94: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 824A6F98: 92610070  stw r19, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[19].u32 ) };
	// 824A6F9C: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 824A6FA0: 395E0002  addi r10, r30, 2
	ctx.r[10].s64 = ctx.r[30].s64 + 2;
	// 824A6FA4: 92610074  stw r19, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[19].u32 ) };
	// 824A6FA8: 93E10078  stw r31, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[31].u32 ) };
	// 824A6FAC: 55441836  rlwinm r4, r10, 3, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x1FFFFFFFu64;
	// 824A6FB0: 806B0020  lwz r3, 0x20(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824A6FB4: 812B002C  lwz r9, 0x2c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 824A6FB8: 7D432214  add r10, r3, r4
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 824A6FBC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A6FC0: 4199000C  bgt cr6, 0x824a6fcc
	if ctx.cr[6].gt {
	pc = 0x824A6FCC; continue 'dispatch;
	}
	// 824A6FC4: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 824A6FC8: 48000018  b 0x824a6fe0
	pc = 0x824A6FE0; continue 'dispatch;
	// 824A6FCC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A6FD0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 824A6FD4: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 824A6FD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A6FDC: 4E800421  bctrl
	ctx.lr = 0x824A6FE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A6FE0: 7FC9FB78  or r9, r30, r31
	ctx.r[9].u64 = ctx.r[30].u64 | ctx.r[31].u64;
	// 824A6FE4: 7D7AB02E  lwzx r11, r26, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 824A6FE8: 56AA2834  slwi r10, r21, 5
	ctx.r[10].u32 = ctx.r[21].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824A6FEC: 90610070  stw r3, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 824A6FF0: 9061007C  stw r3, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[3].u32 ) };
	// 824A6FF4: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 824A6FF8: 91210078  stw r9, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[9].u32 ) };
	// 824A6FFC: 55440036  rlwinm r4, r10, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 824A7000: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824A7004: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 824A7008: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 824A700C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 824A7010: 41990010  bgt cr6, 0x824a7020
	if ctx.cr[6].gt {
	pc = 0x824A7020; continue 'dispatch;
	}
	// 824A7014: 7D575378  mr r23, r10
	ctx.r[23].u64 = ctx.r[10].u64;
	// 824A7018: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 824A701C: 4800001C  b 0x824a7038
	pc = 0x824A7038; continue 'dispatch;
	// 824A7020: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A7024: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 824A7028: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 824A702C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A7030: 4E800421  bctrl
	ctx.lr = 0x824A7034;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A7034: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 824A7038: 7C7AB02E  lwzx r3, r26, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 824A703C: 39750004  addi r11, r21, 4
	ctx.r[11].s64 = ctx.r[21].s64 + 4;
	// 824A7040: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824A7044: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 824A7048: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 824A704C: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 824A7050: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A7054: 41990010  bgt cr6, 0x824a7064
	if ctx.cr[6].gt {
	pc = 0x824A7064; continue 'dispatch;
	}
	// 824A7058: 7D785B78  mr r24, r11
	ctx.r[24].u64 = ctx.r[11].u64;
	// 824A705C: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 824A7060: 48000018  b 0x824a7078
	pc = 0x824A7078; continue 'dispatch;
	// 824A7064: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A7068: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824A706C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A7070: 4E800421  bctrl
	ctx.lr = 0x824A7074;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A7074: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 824A7078: 7D5A902E  lwzx r10, r26, r18
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 824A707C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A7080: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A7084: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A7088: 40980020  bge cr6, 0x824a70a8
	if !ctx.cr[6].lt {
	pc = 0x824A70A8; continue 'dispatch;
	}
	// 824A708C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824A7090: 392907C4  addi r9, r9, 0x7c4
	ctx.r[9].s64 = ctx.r[9].s64 + 1988;
	// 824A7094: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A7098: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A709C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A70A0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A70A4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A70A8: 3B75FFFF  addi r27, r21, -1
	ctx.r[27].s64 = ctx.r[21].s64 + -1;
	// 824A70AC: 7EFDBB78  mr r29, r23
	ctx.r[29].u64 = ctx.r[23].u64;
	// 824A70B0: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 824A70B4: 41980098  blt cr6, 0x824a714c
	if ctx.cr[6].lt {
	pc = 0x824A714C; continue 'dispatch;
	}
	// 824A70B8: 7F3CC050  subf r25, r28, r24
	ctx.r[25].s64 = ctx.r[24].s64 - ctx.r[28].s64;
	// 824A70BC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A70C0: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 824A70C4: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 824A70C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824A70CC: 397F0014  addi r11, r31, 0x14
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	// 824A70D0: 7D79E12E  stwx r11, r25, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[25].u32.wrapping_add(ctx.r[28].u32), ctx.r[11].u32) };
	// 824A70D4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A70D8: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 824A70DC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824A70E0: 40990014  ble cr6, 0x824a70f4
	if !ctx.cr[6].gt {
	pc = 0x824A70F4; continue 'dispatch;
	}
	// 824A70E4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824A70E8: 8074006C  lwz r3, 0x6c(r20)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(108 as u32) ) } as u64;
	// 824A70EC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 824A70F0: 4BFF51F1  bl 0x8249c2e0
	ctx.lr = 0x824A70F4;
	sub_8249C2E0(ctx, base);
	// 824A70F4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 824A70F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A70FC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 824A7100: 7D5F5850  subf r10, r31, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 824A7104: 7D4AEA14  add r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 824A7108: 394AFFE8  addi r10, r10, -0x18
	ctx.r[10].s64 = ctx.r[10].s64 + -24;
	// 824A710C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A7110: 912AFFF8  stw r9, -8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), ctx.r[9].u32 ) };
	// 824A7114: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A7118: 912AFFFC  stw r9, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[9].u32 ) };
	// 824A711C: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A7120: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A7124: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A7128: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 824A712C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A7130: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 824A7134: 4200FFD8  bdnz 0x824a710c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824A710C; continue 'dispatch;
	}
	// 824A7138: 3B7BFFFF  addi r27, r27, -1
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	// 824A713C: 3BBD0020  addi r29, r29, 0x20
	ctx.r[29].s64 = ctx.r[29].s64 + 32;
	// 824A7140: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 824A7144: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 824A7148: 4098FF74  bge cr6, 0x824a70bc
	if !ctx.cr[6].lt {
	pc = 0x824A70BC; continue 'dispatch;
	}
	// 824A714C: 7D5A902E  lwzx r10, r26, r18
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 824A7150: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A7154: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A7158: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A715C: 40980020  bge cr6, 0x824a717c
	if !ctx.cr[6].lt {
	pc = 0x824A717C; continue 'dispatch;
	}
	// 824A7160: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824A7164: 392907B4  addi r9, r9, 0x7b4
	ctx.r[9].s64 = ctx.r[9].s64 + 1972;
	// 824A7168: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A716C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A7170: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A7174: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A7178: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A717C: 80740054  lwz r3, 0x54(r20)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(84 as u32) ) } as u64;
	// 824A7180: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 824A7184: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 824A7188: 7EA6AB78  mr r6, r21
	ctx.r[6].u64 = ctx.r[21].u64;
	// 824A718C: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 824A7190: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 824A7194: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A7198: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 824A719C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A71A0: 4E800421  bctrl
	ctx.lr = 0x824A71A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A71A4: 7C7AB02E  lwzx r3, r26, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 824A71A8: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 824A71AC: 93030020  stw r24, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[24].u32 ) };
	// 824A71B0: 7F185840  cmplw cr6, r24, r11
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824A71B4: 409A0018  bne cr6, 0x824a71cc
	if !ctx.cr[6].eq {
	pc = 0x824A71CC; continue 'dispatch;
	}
	// 824A71B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A71BC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 824A71C0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824A71C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A71C8: 4E800421  bctrl
	ctx.lr = 0x824A71CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A71CC: 7C7AB02E  lwzx r3, r26, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 824A71D0: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 824A71D4: 92E30020  stw r23, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[23].u32 ) };
	// 824A71D8: 7F175840  cmplw cr6, r23, r11
	ctx.cr[6].compare_u32(ctx.r[23].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824A71DC: 409A0018  bne cr6, 0x824a71f4
	if !ctx.cr[6].eq {
	pc = 0x824A71F4; continue 'dispatch;
	}
	// 824A71E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A71E4: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 824A71E8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824A71EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A71F0: 4E800421  bctrl
	ctx.lr = 0x824A71F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A71F4: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 824A71F8: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 824A71FC: 7D6B5215  add. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A7200: 4081011C  ble 0x824a731c
	if !ctx.cr[0].gt {
	pc = 0x824A731C; continue 'dispatch;
	}
	// 824A7204: 7D5A902E  lwzx r10, r26, r18
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 824A7208: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A720C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A7210: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A7214: 40980020  bge cr6, 0x824a7234
	if !ctx.cr[6].lt {
	pc = 0x824A7234; continue 'dispatch;
	}
	// 824A7218: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824A721C: 392907A8  addi r9, r9, 0x7a8
	ctx.r[9].s64 = ctx.r[9].s64 + 1960;
	// 824A7220: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A7224: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A7228: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A722C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A7230: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A7234: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 824A7238: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 824A723C: 4805705D  bl 0x824fe298
	ctx.lr = 0x824A7240;
	sub_824FE298(ctx, base);
	// 824A7240: 7D5A902E  lwzx r10, r26, r18
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 824A7244: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A7248: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A724C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A7250: 40980020  bge cr6, 0x824a7270
	if !ctx.cr[6].lt {
	pc = 0x824A7270; continue 'dispatch;
	}
	// 824A7254: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824A7258: 3929079C  addi r9, r9, 0x79c
	ctx.r[9].s64 = ctx.r[9].s64 + 1948;
	// 824A725C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A7260: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A7264: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A7268: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A726C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A7270: 80A10074  lwz r5, 0x74(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 824A7274: 80740058  lwz r3, 0x58(r20)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(88 as u32) ) } as u64;
	// 824A7278: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 824A727C: 48056FA5  bl 0x824fe220
	ctx.lr = 0x824A7280;
	sub_824FE220(ctx, base);
	// 824A7280: 3FE08293  lis r31, -0x7d6d
	ctx.r[31].s64 = -2104295424;
	// 824A7284: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 824A7288: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824A728C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824A7290: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824A7294: 809F9004  lwz r4, -0x6ffc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28668 as u32) ) } as u64;
	// 824A7298: 55653830  slwi r5, r11, 7
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824A729C: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A72A0: 81490060  lwz r10, 0x60(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(96 as u32) ) } as u64;
	// 824A72A4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 824A72A8: 4E800421  bctrl
	ctx.lr = 0x824A72AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A72AC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A72B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A72B4: 409A0014  bne cr6, 0x824a72c8
	if !ctx.cr[6].eq {
	pc = 0x824A72C8; continue 'dispatch;
	}
	// 824A72B8: 817F9004  lwz r11, -0x6ffc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28668 as u32) ) } as u64;
	// 824A72BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824A72C0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824A72C4: 48000058  b 0x824a731c
	pc = 0x824A731C; continue 'dispatch;
	// 824A72C8: 7D5A902E  lwzx r10, r26, r18
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 824A72CC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A72D0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A72D4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A72D8: 40980020  bge cr6, 0x824a72f8
	if !ctx.cr[6].lt {
	pc = 0x824A72F8; continue 'dispatch;
	}
	// 824A72DC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824A72E0: 39290790  addi r9, r9, 0x790
	ctx.r[9].s64 = ctx.r[9].s64 + 1936;
	// 824A72E4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A72E8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A72EC: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A72F0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A72F4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A72F8: 81740070  lwz r11, 0x70(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(112 as u32) ) } as u64;
	// 824A72FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A7300: 38CB0008  addi r6, r11, 8
	ctx.r[6].s64 = ctx.r[11].s64 + 8;
	// 824A7304: 409A0008  bne cr6, 0x824a730c
	if !ctx.cr[6].eq {
	pc = 0x824A730C; continue 'dispatch;
	}
	// 824A7308: 7E669B78  mr r6, r19
	ctx.r[6].u64 = ctx.r[19].u64;
	// 824A730C: 80740058  lwz r3, 0x58(r20)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(88 as u32) ) } as u64;
	// 824A7310: 80A10064  lwz r5, 0x64(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 824A7314: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824A7318: 48056E49  bl 0x824fe160
	ctx.lr = 0x824A731C;
	sub_824FE160(ctx, base);
	// 824A731C: 7D5A902E  lwzx r10, r26, r18
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 824A7320: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A7324: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A7328: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A732C: 40980020  bge cr6, 0x824a734c
	if !ctx.cr[6].lt {
	pc = 0x824A734C; continue 'dispatch;
	}
	// 824A7330: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824A7334: 3929FE14  addi r9, r9, -0x1ec
	ctx.r[9].s64 = ctx.r[9].s64 + -492;
	// 824A7338: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A733C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A7340: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A7344: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A7348: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A734C: 7C7AB02E  lwzx r3, r26, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 824A7350: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 824A7354: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 824A7358: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 824A735C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824A7360: 409A0014  bne cr6, 0x824a7374
	if !ctx.cr[6].eq {
	pc = 0x824A7374; continue 'dispatch;
	}
	// 824A7364: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A7368: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824A736C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A7370: 4E800421  bctrl
	ctx.lr = 0x824A7374;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A7374: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 824A7378: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824A737C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824A7380: 409A0018  bne cr6, 0x824a7398
	if !ctx.cr[6].eq {
	pc = 0x824A7398; continue 'dispatch;
	}
	// 824A7384: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824A7388: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 824A738C: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824A7390: 7C7AB02E  lwzx r3, r26, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 824A7394: 4BFBCD25  bl 0x824640b8
	ctx.lr = 0x824A7398;
	sub_824640B8(ctx, base);
	// 824A7398: 7C7AB02E  lwzx r3, r26, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 824A739C: 8081006C  lwz r4, 0x6c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 824A73A0: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 824A73A4: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 824A73A8: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824A73AC: 409A0014  bne cr6, 0x824a73c0
	if !ctx.cr[6].eq {
	pc = 0x824A73C0; continue 'dispatch;
	}
	// 824A73B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A73B4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824A73B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A73BC: 4E800421  bctrl
	ctx.lr = 0x824A73C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A73C0: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 824A73C4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824A73C8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824A73CC: 409A0018  bne cr6, 0x824a73e4
	if !ctx.cr[6].eq {
	pc = 0x824A73E4; continue 'dispatch;
	}
	// 824A73D0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824A73D4: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824A73D8: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824A73DC: 7C7AB02E  lwzx r3, r26, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 824A73E0: 4BFBCCD9  bl 0x824640b8
	ctx.lr = 0x824A73E4;
	sub_824640B8(ctx, base);
	// 824A73E4: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 824A73E8: 4808DCF8  b 0x825350e0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A73F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824A73F0 size=864
    let mut pc: u32 = 0x824A73F0;
    'dispatch: loop {
        match pc {
            0x824A73F0 => {
    //   block [0x824A73F0..0x824A7750)
	// 824A73F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A73F4: 4808DCA1  bl 0x82535094
	ctx.lr = 0x824A73F8;
	sub_82535080(ctx, base);
	// 824A73F8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A73FC: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A7400: 3B400014  li r26, 0x14
	ctx.r[26].s64 = 20;
	// 824A7404: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824A7408: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824A740C: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824A7410: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A7414: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A7418: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A741C: 40980020  bge cr6, 0x824a743c
	if !ctx.cr[6].lt {
	pc = 0x824A743C; continue 'dispatch;
	}
	// 824A7420: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824A7424: 39290804  addi r9, r9, 0x804
	ctx.r[9].s64 = ctx.r[9].s64 + 2052;
	// 824A7428: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A742C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A7430: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A7434: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A7438: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A743C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A7440: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 824A7444: E95B0000  ld r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	// 824A7448: 3E808293  lis r20, -0x7d6d
	ctx.r[20].s64 = -2104295424;
	// 824A744C: 396B0190  addi r11, r11, 0x190
	ctx.r[11].s64 = ctx.r[11].s64 + 400;
	// 824A7450: 7E7C9B78  mr r28, r19
	ctx.r[28].u64 = ctx.r[19].u64;
	// 824A7454: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 824A7458: E95B0008  ld r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) };
	// 824A745C: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 824A7460: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A7464: E95B0000  ld r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	// 824A7468: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 824A746C: 396B0050  addi r11, r11, 0x50
	ctx.r[11].s64 = ctx.r[11].s64 + 80;
	// 824A7470: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 824A7474: E95B0008  ld r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) };
	// 824A7478: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 824A747C: C01B0008  lfs f0, 8(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A7480: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A7484: 814B02B4  lwz r10, 0x2b4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(692 as u32) ) } as u64;
	// 824A7488: C1AB02C0  lfs f13, 0x2c0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(704 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A748C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 824A7490: D00B02AC  stfs f0, 0x2ac(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(684 as u32), tmp.u32 ) };
	// 824A7494: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 824A7498: C01B000C  lfs f0, 0xc(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A749C: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 824A74A0: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 824A74A4: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 824A74A8: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 824A74AC: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 824A74B0: D00B02B0  stfs f0, 0x2b0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(688 as u32), tmp.u32 ) };
	// 824A74B4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A74B8: 814B0084  lwz r10, 0x84(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 824A74BC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824A74C0: 914B0084  stw r10, 0x84(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 824A74C4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A74C8: 3BCB0028  addi r30, r11, 0x28
	ctx.r[30].s64 = ctx.r[11].s64 + 40;
	// 824A74CC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A74D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A74D4: 40990044  ble cr6, 0x824a7518
	if !ctx.cr[6].gt {
	pc = 0x824A7518; continue 'dispatch;
	}
	// 824A74D8: 7E7D9B78  mr r29, r19
	ctx.r[29].u64 = ctx.r[19].u64;
	// 824A74DC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A74E0: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A74E4: 7D7D582E  lwzx r11, r29, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824A74E8: 808B004C  lwz r4, 0x4c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 824A74EC: 806B0048  lwz r3, 0x48(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 824A74F0: 4BFFF9E1  bl 0x824a6ed0
	ctx.lr = 0x824A74F4;
	sub_824A6ED0(ctx, base);
	// 824A74F4: 81749004  lwz r11, -0x6ffc(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-28668 as u32) ) } as u64;
	// 824A74F8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A74FC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824A7500: 419A01FC  beq cr6, 0x824a76fc
	if ctx.cr[6].eq {
	pc = 0x824A76FC; continue 'dispatch;
	}
	// 824A7504: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A7508: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 824A750C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 824A7510: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824A7514: 4198FFC8  blt cr6, 0x824a74dc
	if ctx.cr[6].lt {
	pc = 0x824A74DC; continue 'dispatch;
	}
	// 824A7518: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A751C: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 824A7520: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A7524: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824A7528: 40820020  bne 0x824a7548
	if !ctx.cr[0].eq {
	pc = 0x824A7548; continue 'dispatch;
	}
	// 824A752C: 8963008C  lbz r11, 0x8c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 824A7530: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A7534: 409A0014  bne cr6, 0x824a7548
	if !ctx.cr[6].eq {
	pc = 0x824A7548; continue 'dispatch;
	}
	// 824A7538: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 824A753C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A7540: 419A0008  beq cr6, 0x824a7548
	if ctx.cr[6].eq {
	pc = 0x824A7548; continue 'dispatch;
	}
	// 824A7544: 4BFEBC7D  bl 0x824931c0
	ctx.lr = 0x824A7548;
	sub_824931C0(ctx, base);
	// 824A7548: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A754C: 7E789B78  mr r24, r19
	ctx.r[24].u64 = ctx.r[19].u64;
	// 824A7550: 814B0084  lwz r10, 0x84(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 824A7554: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824A7558: 914B0084  stw r10, 0x84(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 824A755C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A7560: 3BAB0028  addi r29, r11, 0x28
	ctx.r[29].s64 = ctx.r[11].s64 + 40;
	// 824A7564: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A7568: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A756C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824A7570: 3AAB74DC  addi r21, r11, 0x74dc
	ctx.r[21].s64 = ctx.r[11].s64 + 29916;
	// 824A7574: 40990128  ble cr6, 0x824a769c
	if !ctx.cr[6].gt {
	pc = 0x824A769C; continue 'dispatch;
	}
	// 824A7578: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824A757C: 7E7C9B78  mr r28, r19
	ctx.r[28].u64 = ctx.r[19].u64;
	// 824A7580: 3AEB070C  addi r23, r11, 0x70c
	ctx.r[23].s64 = ctx.r[11].s64 + 1804;
	// 824A7584: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824A7588: 3ACB07EC  addi r22, r11, 0x7ec
	ctx.r[22].s64 = ctx.r[11].s64 + 2028;
	// 824A758C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A7590: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A7594: 7D7AC82E  lwzx r11, r26, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824A7598: 7FCAE02E  lwzx r30, r10, r28
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 824A759C: 80A9006C  lwz r5, 0x6c(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(108 as u32) ) } as u64;
	// 824A75A0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A75A4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A75A8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A75AC: 40980018  bge cr6, 0x824a75c4
	if !ctx.cr[6].lt {
	pc = 0x824A75C4; continue 'dispatch;
	}
	// 824A75B0: 92EA0000  stw r23, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 824A75B4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A75B8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 824A75BC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A75C0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A75C4: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A75C8: 389E0058  addi r4, r30, 0x58
	ctx.r[4].s64 = ctx.r[30].s64 + 88;
	// 824A75CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824A75D0: 92650010  stw r19, 0x10(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[19].u32 ) };
	// 824A75D4: 396B1C60  addi r11, r11, 0x1c60
	ctx.r[11].s64 = ctx.r[11].s64 + 7264;
	// 824A75D8: 91650060  stw r11, 0x60(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824A75DC: 4802E70D  bl 0x824d5ce8
	ctx.lr = 0x824A75E0;
	sub_824D5CE8(ctx, base);
	// 824A75E0: 7D7AC82E  lwzx r11, r26, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824A75E4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A75E8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A75EC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A75F0: 40980018  bge cr6, 0x824a7608
	if !ctx.cr[6].lt {
	pc = 0x824A7608; continue 'dispatch;
	}
	// 824A75F4: 92AA0000  stw r21, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[21].u32 ) };
	// 824A75F8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A75FC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 824A7600: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A7604: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A7608: 81749004  lwz r11, -0x6ffc(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-28668 as u32) ) } as u64;
	// 824A760C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A7610: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824A7614: 419A0088  beq cr6, 0x824a769c
	if ctx.cr[6].eq {
	pc = 0x824A769C; continue 'dispatch;
	}
	// 824A7618: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A761C: 816B016C  lwz r11, 0x16c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(364 as u32) ) } as u64;
	// 824A7620: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A7624: 419A0064  beq cr6, 0x824a7688
	if ctx.cr[6].eq {
	pc = 0x824A7688; continue 'dispatch;
	}
	// 824A7628: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824A762C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A7630: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A7634: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A7638: 40980018  bge cr6, 0x824a7650
	if !ctx.cr[6].lt {
	pc = 0x824A7650; continue 'dispatch;
	}
	// 824A763C: 92CB0000  stw r22, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[22].u32 ) };
	// 824A7640: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A7644: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A7648: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A764C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A7650: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 824A7654: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A7658: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824A765C: 4BFFDEBD  bl 0x824a5518
	ctx.lr = 0x824A7660;
	sub_824A5518(ctx, base);
	// 824A7660: 7D7AC82E  lwzx r11, r26, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824A7664: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A7668: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A766C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A7670: 40980018  bge cr6, 0x824a7688
	if !ctx.cr[6].lt {
	pc = 0x824A7688; continue 'dispatch;
	}
	// 824A7674: 92AA0000  stw r21, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[21].u32 ) };
	// 824A7678: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A767C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 824A7680: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A7684: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A7688: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A768C: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 824A7690: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 824A7694: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824A7698: 4198FEF4  blt cr6, 0x824a758c
	if ctx.cr[6].lt {
	pc = 0x824A758C; continue 'dispatch;
	}
	// 824A769C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A76A0: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 824A76A4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A76A8: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824A76AC: 40820020  bne 0x824a76cc
	if !ctx.cr[0].eq {
	pc = 0x824A76CC; continue 'dispatch;
	}
	// 824A76B0: 8963008C  lbz r11, 0x8c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 824A76B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A76B8: 409A0014  bne cr6, 0x824a76cc
	if !ctx.cr[6].eq {
	pc = 0x824A76CC; continue 'dispatch;
	}
	// 824A76BC: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 824A76C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A76C4: 419A0008  beq cr6, 0x824a76cc
	if ctx.cr[6].eq {
	pc = 0x824A76CC; continue 'dispatch;
	}
	// 824A76C8: 4BFEBAF9  bl 0x824931c0
	ctx.lr = 0x824A76CC;
	sub_824931C0(ctx, base);
	// 824A76CC: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824A76D0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A76D4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A76D8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A76DC: 40980018  bge cr6, 0x824a76f4
	if !ctx.cr[6].lt {
	pc = 0x824A76F4; continue 'dispatch;
	}
	// 824A76E0: 92AB0000  stw r21, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[21].u32 ) };
	// 824A76E4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824A76E8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824A76EC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824A76F0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A76F4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824A76F8: 4808D9EC  b 0x825350e4
	sub_825350D0(ctx, base);
	return;
	// 824A76FC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A7700: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 824A7704: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A7708: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824A770C: 40820020  bne 0x824a772c
	if !ctx.cr[0].eq {
	pc = 0x824A772C; continue 'dispatch;
	}
	// 824A7710: 8963008C  lbz r11, 0x8c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 824A7714: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A7718: 409A0014  bne cr6, 0x824a772c
	if !ctx.cr[6].eq {
	pc = 0x824A772C; continue 'dispatch;
	}
	// 824A771C: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 824A7720: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A7724: 419A0008  beq cr6, 0x824a772c
	if ctx.cr[6].eq {
	pc = 0x824A772C; continue 'dispatch;
	}
	// 824A7728: 4BFEBA99  bl 0x824931c0
	ctx.lr = 0x824A772C;
	sub_824931C0(ctx, base);
	// 824A772C: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824A7730: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A7734: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A7738: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A773C: 4098FFB8  bge cr6, 0x824a76f4
	if !ctx.cr[6].lt {
	pc = 0x824A76F4; continue 'dispatch;
	}
	// 824A7740: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824A7744: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 824A7748: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A774C: 4BFFFF98  b 0x824a76e4
	pc = 0x824A76E4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A7750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A7750 size=16
    let mut pc: u32 = 0x824A7750;
    'dispatch: loop {
        match pc {
            0x824A7750 => {
    //   block [0x824A7750..0x824A7760)
	// 824A7750: 3D60824A  lis r11, -0x7db6
	ctx.r[11].s64 = -2109079552;
	// 824A7754: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 824A7758: 38EB64C8  addi r7, r11, 0x64c8
	ctx.r[7].s64 = ctx.r[11].s64 + 25800;
	// 824A775C: 4BFFF034  b 0x824a6790
	sub_824A6790(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A7760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A7760 size=96
    let mut pc: u32 = 0x824A7760;
    'dispatch: loop {
        match pc {
            0x824A7760 => {
    //   block [0x824A7760..0x824A77C0)
	// 824A7760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A7764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824A7768: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824A776C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824A7770: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A7774: 7CEB0774  extsb r11, r7
	ctx.r[11].s64 = ctx.r[7].s8 as i64;
	// 824A7778: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824A777C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824A7780: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A7784: 409A0018  bne cr6, 0x824a779c
	if !ctx.cr[6].eq {
	pc = 0x824A779C; continue 'dispatch;
	}
	// 824A7788: 3D60824A  lis r11, -0x7db6
	ctx.r[11].s64 = -2109079552;
	// 824A778C: 80C6006C  lwz r6, 0x6c(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(108 as u32) ) } as u64;
	// 824A7790: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 824A7794: 38EB59A0  addi r7, r11, 0x59a0
	ctx.r[7].s64 = ctx.r[11].s64 + 22944;
	// 824A7798: 4BFFEFF9  bl 0x824a6790
	ctx.lr = 0x824A779C;
	sub_824A6790(ctx, base);
	// 824A779C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824A77A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824A77A4: 4BFF4B05  bl 0x8249c2a8
	ctx.lr = 0x824A77A8;
	sub_8249C2A8(ctx, base);
	// 824A77A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824A77AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824A77B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824A77B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824A77B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824A77BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A77C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824A77C0 size=424
    let mut pc: u32 = 0x824A77C0;
    'dispatch: loop {
        match pc {
            0x824A77C0 => {
    //   block [0x824A77C0..0x824A7968)
	// 824A77C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A77C4: 4808D8E5  bl 0x825350a8
	ctx.lr = 0x824A77C8;
	sub_82535080(ctx, base);
	// 824A77C8: DBE1FFB0  stfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 824A77CC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A77D0: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 824A77D4: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 824A77D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824A77DC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 824A77E0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 824A77E4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 824A77E8: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 824A77EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824A77F0: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824A77F4: 4BFEB6E5  bl 0x82492ed8
	ctx.lr = 0x824A77F8;
	sub_82492ED8(ctx, base);
	// 824A77F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824A77FC: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 824A7800: 4BFEB6C9  bl 0x82492ec8
	ctx.lr = 0x824A7804;
	sub_82492EC8(ctx, base);
	// 824A7804: EC1F0828  fsubs f0, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[1].f64) as f32) as f64);
	// 824A7808: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824A780C: D3E10054  stfs f31, 0x54(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 824A7810: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 824A7814: D0210050  stfs f1, 0x50(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 824A7818: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A781C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 824A7820: 419A0010  beq cr6, 0x824a7830
	if ctx.cr[6].eq {
	pc = 0x824A7830; continue 'dispatch;
	}
	// 824A7824: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824A7828: C1AB1850  lfs f13, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A782C: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 824A7830: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824A7834: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 824A7838: 572907FE  clrlwi r9, r25, 0x1f
	ctx.r[9].u64 = ctx.r[25].u32 as u64 & 0x00000001u64;
	// 824A783C: D1A1005C  stfs f13, 0x5c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 824A7840: 394A0050  addi r10, r10, 0x50
	ctx.r[10].s64 = ctx.r[10].s64 + 80;
	// 824A7844: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824A7848: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 824A784C: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 824A7850: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 824A7854: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 824A7858: 419A0060  beq cr6, 0x824a78b8
	if ctx.cr[6].eq {
	pc = 0x824A78B8; continue 'dispatch;
	}
	// 824A785C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 824A7860: 40990034  ble cr6, 0x824a7894
	if !ctx.cr[6].gt {
	pc = 0x824A7894; continue 'dispatch;
	}
	// 824A7864: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 824A7868: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 824A786C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A7870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824A7874: 3B8B00E0  addi r28, r11, 0xe0
	ctx.r[28].s64 = ctx.r[11].s64 + 224;
	// 824A7878: 4BFEB651  bl 0x82492ec8
	ctx.lr = 0x824A787C;
	sub_82492EC8(ctx, base);
	// 824A787C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824A7880: 48102061  bl 0x825a98e0
	ctx.lr = 0x824A7884;
	sub_825A98E0(ctx, base);
	// 824A7884: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 824A7888: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 824A788C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824A7890: 409AFFDC  bne cr6, 0x824a786c
	if !ctx.cr[6].eq {
	pc = 0x824A786C; continue 'dispatch;
	}
	// 824A7894: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 824A7898: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 824A789C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 824A78A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824A78A4: 48014785  bl 0x824bc028
	ctx.lr = 0x824A78A8;
	sub_824BC028(ctx, base);
	// 824A78A8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 824A78AC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 824A78B0: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 824A78B4: 4BFF4A2D  bl 0x8249c2e0
	ctx.lr = 0x824A78B8;
	sub_8249C2E0(ctx, base);
	// 824A78B8: 572B07BC  rlwinm r11, r25, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0xFFFFFFFFu64;
	// 824A78BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A78C0: 419A0014  beq cr6, 0x824a78d4
	if ctx.cr[6].eq {
	pc = 0x824A78D4; continue 'dispatch;
	}
	// 824A78C4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824A78C8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824A78CC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824A78D0: 4BFFF601  bl 0x824a6ed0
	ctx.lr = 0x824A78D4;
	sub_824A6ED0(ctx, base);
	// 824A78D4: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 824A78D8: 40990028  ble cr6, 0x824a7900
	if !ctx.cr[6].gt {
	pc = 0x824A7900; continue 'dispatch;
	}
	// 824A78DC: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 824A78E0: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 824A78E4: 809F006C  lwz r4, 0x6c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 824A78E8: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A78EC: 48010595  bl 0x824b7e80
	ctx.lr = 0x824A78F0;
	sub_824B7E80(ctx, base);
	// 824A78F0: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 824A78F4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 824A78F8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824A78FC: 409AFFE8  bne cr6, 0x824a78e4
	if !ctx.cr[6].eq {
	pc = 0x824A78E4; continue 'dispatch;
	}
	// 824A7900: 572B077A  rlwinm r11, r25, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0xFFFFFFFFu64;
	// 824A7904: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A7908: 419A0024  beq cr6, 0x824a792c
	if ctx.cr[6].eq {
	pc = 0x824A792C; continue 'dispatch;
	}
	// 824A790C: 3D60824A  lis r11, -0x7db6
	ctx.r[11].s64 = -2109079552;
	// 824A7910: 80DF006C  lwz r6, 0x6c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 824A7914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 824A7918: 38EB64C8  addi r7, r11, 0x64c8
	ctx.r[7].s64 = ctx.r[11].s64 + 25800;
	// 824A791C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 824A7920: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 824A7924: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 824A7928: 4BFFEE69  bl 0x824a6790
	ctx.lr = 0x824A792C;
	sub_824A6790(ctx, base);
	// 824A792C: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 824A7930: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A7934: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824A7938: 40820024  bne 0x824a795c
	if !ctx.cr[0].eq {
	pc = 0x824A795C; continue 'dispatch;
	}
	// 824A793C: 897F008C  lbz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 824A7940: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A7944: 409A0018  bne cr6, 0x824a795c
	if !ctx.cr[6].eq {
	pc = 0x824A795C; continue 'dispatch;
	}
	// 824A7948: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 824A794C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A7950: 419A000C  beq cr6, 0x824a795c
	if ctx.cr[6].eq {
	pc = 0x824A795C; continue 'dispatch;
	}
	// 824A7954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824A7958: 4BFEB869  bl 0x824931c0
	ctx.lr = 0x824A795C;
	sub_824931C0(ctx, base);
	// 824A795C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 824A7960: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 824A7964: 4808D794  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A7968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A7968 size=208
    let mut pc: u32 = 0x824A7968;
    'dispatch: loop {
        match pc {
            0x824A7968 => {
    //   block [0x824A7968..0x824A7A38)
	// 824A7968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A796C: 4808D745  bl 0x825350b0
	ctx.lr = 0x824A7970;
	sub_82535080(ctx, base);
	// 824A7970: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A7974: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 824A7978: E9470000  ld r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	// 824A797C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824A7980: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 824A7984: E9070008  ld r8, 8(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) };
	// 824A7988: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 824A798C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824A7990: 83DF006C  lwz r30, 0x6c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 824A7994: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 824A7998: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824A799C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824A79A0: F92B0000  std r9, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 824A79A4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824A79A8: F90B0008  std r8, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[8].u64 ) };
	// 824A79AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824A79B0: F95E0050  std r10, 0x50(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 824A79B4: E9670008  ld r11, 8(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) };
	// 824A79B8: F97E0058  std r11, 0x58(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 824A79BC: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 824A79C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824A79C4: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824A79C8: 4BFFF509  bl 0x824a6ed0
	ctx.lr = 0x824A79CC;
	sub_824A6ED0(ctx, base);
	// 824A79CC: 3D60824A  lis r11, -0x7db6
	ctx.r[11].s64 = -2109079552;
	// 824A79D0: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 824A79D4: 38EB64C8  addi r7, r11, 0x64c8
	ctx.r[7].s64 = ctx.r[11].s64 + 25800;
	// 824A79D8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824A79DC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824A79E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824A79E4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824A79E8: 4BFFEDA9  bl 0x824a6790
	ctx.lr = 0x824A79EC;
	sub_824A6790(ctx, base);
	// 824A79EC: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 824A79F0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A79F4: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824A79F8: 40820024  bne 0x824a7a1c
	if !ctx.cr[0].eq {
	pc = 0x824A7A1C; continue 'dispatch;
	}
	// 824A79FC: 897F008C  lbz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 824A7A00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A7A04: 409A0018  bne cr6, 0x824a7a1c
	if !ctx.cr[6].eq {
	pc = 0x824A7A1C; continue 'dispatch;
	}
	// 824A7A08: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 824A7A0C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A7A10: 419A000C  beq cr6, 0x824a7a1c
	if ctx.cr[6].eq {
	pc = 0x824A7A1C; continue 'dispatch;
	}
	// 824A7A14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824A7A18: 4BFEB7A9  bl 0x824931c0
	ctx.lr = 0x824A7A1C;
	sub_824931C0(ctx, base);
	// 824A7A1C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824A7A20: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 824A7A24: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 824A7A28: F95E0050  std r10, 0x50(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 824A7A2C: F97E0058  std r11, 0x58(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 824A7A30: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824A7A34: 4808D6CC  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A7A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A7A38 size=396
    let mut pc: u32 = 0x824A7A38;
    'dispatch: loop {
        match pc {
            0x824A7A38 => {
    //   block [0x824A7A38..0x824A7BC4)
	// 824A7A38: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 824A7A3C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 824A7A40: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A7BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A7BC8 size=16
    let mut pc: u32 = 0x824A7BC8;
    'dispatch: loop {
        match pc {
            0x824A7BC8 => {
    //   block [0x824A7BC8..0x824A7BD8)
	// 824A7BC8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 824A7BCC: 38800094  li r4, 0x94
	ctx.r[4].s64 = 148;
	// 824A7BD0: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 824A7BD4: 4BFFADDC  b 0x824a29b0
	sub_824A29B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A7BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A7BD8 size=32
    let mut pc: u32 = 0x824A7BD8;
    'dispatch: loop {
        match pc {
            0x824A7BD8 => {
    //   block [0x824A7BD8..0x824A7BF8)
	// 824A7BD8: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 824A7BDC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A7BE0: 419A0018  beq cr6, 0x824a7bf8
	if ctx.cr[6].eq {
		sub_824A7BF8(ctx, base);
		return;
	}
	// 824A7BE4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824A7BE8: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 824A7BEC: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824A7BF0: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824A7BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A7BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A7BF8 size=16
    let mut pc: u32 = 0x824A7BF8;
    'dispatch: loop {
        match pc {
            0x824A7BF8 => {
    //   block [0x824A7BF8..0x824A7C08)
	// 824A7BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A7BFC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824A7C00: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824A7C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A7C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824A7C08 size=140
    let mut pc: u32 = 0x824A7C08;
    'dispatch: loop {
        match pc {
            0x824A7C08 => {
    //   block [0x824A7C08..0x824A7C94)
	// 824A7C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A7C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824A7C10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824A7C14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824A7C18: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 824A7C1C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A7C20: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824A7C24: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824A7C28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824A7C2C: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 824A7C30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824A7C34: C3EB2238  lfs f31, 0x2238(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8760 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824A7C38: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824A7C3C: 48103B25  bl 0x825ab760
	ctx.lr = 0x824A7C40;
	sub_825AB760(ctx, base);
	// 824A7C40: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A7C44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A7C48: 419A0024  beq cr6, 0x824a7c6c
	if ctx.cr[6].eq {
	pc = 0x824A7C6C; continue 'dispatch;
	}
	// 824A7C4C: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 824A7C50: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824A7C54: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 824A7C58: 48103B09  bl 0x825ab760
	ctx.lr = 0x824A7C5C;
	sub_825AB760(ctx, base);
	// 824A7C5C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A7C60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A7C64: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824A7C68: 409A0008  bne cr6, 0x824a7c70
	if !ctx.cr[6].eq {
	pc = 0x824A7C70; continue 'dispatch;
	}
	// 824A7C6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A7C70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824A7C74: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824A7C78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824A7C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824A7C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824A7C84: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 824A7C88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824A7C8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824A7C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A7C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A7C98 size=8
    let mut pc: u32 = 0x824A7C98;
    'dispatch: loop {
        match pc {
            0x824A7C98 => {
    //   block [0x824A7C98..0x824A7CA0)
	// 824A7C98: 3860000E  li r3, 0xe
	ctx.r[3].s64 = 14;
	// 824A7C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A7CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A7CA0 size=464
    let mut pc: u32 = 0x824A7CA0;
    'dispatch: loop {
        match pc {
            0x824A7CA0 => {
    //   block [0x824A7CA0..0x824A7E70)
	// 824A7CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A7CA4: 4808D415  bl 0x825350b8
	ctx.lr = 0x824A7CA8;
	sub_82535080(ctx, base);
	// 824A7CA8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A7CAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824A7CB0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 824A7CB4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824A7CB8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824A7CBC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 824A7CC0: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 824A7CC4: 48103E8D  bl 0x825abb50
	ctx.lr = 0x824A7CC8;
	sub_825ABB50(ctx, base);
	// 824A7CC8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824A7CCC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824A7CD0: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 824A7CD4: 48103E7D  bl 0x825abb50
	ctx.lr = 0x824A7CD8;
	sub_825ABB50(ctx, base);
	// 824A7CD8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824A7CDC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 824A7CE0: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 824A7CE4: 3D005555  lis r8, 0x5555
	ctx.r[8].s64 = 1431633920;
	// 824A7CE8: 393F0060  addi r9, r31, 0x60
	ctx.r[9].s64 = ctx.r[31].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A7E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A7E70 size=188
    let mut pc: u32 = 0x824A7E70;
    'dispatch: loop {
        match pc {
            0x824A7E70 => {
    //   block [0x824A7E70..0x824A7F2C)
	// 824A7E70: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 824A7E74: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
	// 824A7E78: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 824A7E7C: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 824A7E80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824A7E84: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A7F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A7F30 size=356
    let mut pc: u32 = 0x824A7F30;
    'dispatch: loop {
        match pc {
            0x824A7F30 => {
    //   block [0x824A7F30..0x824A8094)
	// 824A7F30: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 824A7F34: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A8098 size=16
    let mut pc: u32 = 0x824A8098;
    'dispatch: loop {
        match pc {
            0x824A8098 => {
    //   block [0x824A8098..0x824A80A8)
	// 824A8098: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 824A809C: 38800084  li r4, 0x84
	ctx.r[4].s64 = 132;
	// 824A80A0: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 824A80A4: 4BFFA90C  b 0x824a29b0
	sub_824A29B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A80A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A80A8 size=20
    let mut pc: u32 = 0x824A80A8;
    'dispatch: loop {
        match pc {
            0x824A80A8 => {
    //   block [0x824A80A8..0x824A80BC)
	// 824A80A8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 824A80AC: 3940001C  li r10, 0x1c
	ctx.r[10].s64 = 28;
	// 824A80B0: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824A80B4: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824A80B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A80C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824A80C0 size=140
    let mut pc: u32 = 0x824A80C0;
    'dispatch: loop {
        match pc {
            0x824A80C0 => {
    //   block [0x824A80C0..0x824A814C)
	// 824A80C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A80C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824A80C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824A80CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824A80D0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 824A80D4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A80D8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824A80DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824A80E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824A80E4: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 824A80E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824A80EC: C3EB2238  lfs f31, 0x2238(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8760 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824A80F0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824A80F4: 4810366D  bl 0x825ab760
	ctx.lr = 0x824A80F8;
	sub_825AB760(ctx, base);
	// 824A80F8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A80FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A8100: 419A0024  beq cr6, 0x824a8124
	if ctx.cr[6].eq {
	pc = 0x824A8124; continue 'dispatch;
	}
	// 824A8104: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 824A8108: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824A810C: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 824A8110: 48103651  bl 0x825ab760
	ctx.lr = 0x824A8114;
	sub_825AB760(ctx, base);
	// 824A8114: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A8118: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A811C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824A8120: 409A0008  bne cr6, 0x824a8128
	if !ctx.cr[6].eq {
	pc = 0x824A8128; continue 'dispatch;
	}
	// 824A8124: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A8128: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824A812C: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824A8130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824A8134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824A8138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824A813C: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 824A8140: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824A8144: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824A8148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A8150 size=8
    let mut pc: u32 = 0x824A8150;
    'dispatch: loop {
        match pc {
            0x824A8150 => {
    //   block [0x824A8150..0x824A8158)
	// 824A8150: 38600012  li r3, 0x12
	ctx.r[3].s64 = 18;
	// 824A8154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A8158 size=204
    let mut pc: u32 = 0x824A8158;
    'dispatch: loop {
        match pc {
            0x824A8158 => {
    //   block [0x824A8158..0x824A8224)
	// 824A8158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A815C: 4808CF61  bl 0x825350bc
	ctx.lr = 0x824A8160;
	sub_82535080(ctx, base);
	// 824A8160: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824A8164: 3961FFD0  addi r11, r1, -0x30
	ctx.r[11].s64 = ctx.r[1].s64 + -48;
	// 824A8168: 38E8E1B4  addi r7, r8, -0x1e4c
	ctx.r[7].s64 = ctx.r[8].s64 + -7756;
	// 824A816C: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 824A8170: 3941FFD0  addi r10, r1, -0x30
	ctx.r[10].s64 = ctx.r[1].s64 + -48;
	// 824A8174: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824A8178: 39230010  addi r9, r3, 0x10
	ctx.r[9].s64 = ctx.r[3].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824A8228 size=544
    let mut pc: u32 = 0x824A8228;
    'dispatch: loop {
        match pc {
            0x824A8228 => {
    //   block [0x824A8228..0x824A8448)
	// 824A8228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A822C: 4808CE89  bl 0x825350b4
	ctx.lr = 0x824A8230;
	sub_82535080(ctx, base);
	// 824A8230: C0060000  lfs f0, 0(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A8234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824A8238: C1A60004  lfs f13, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A823C: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 824A8240: FDA06A10  fabs f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 824A8244: C1860008  lfs f12, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824A8248: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824A824C: FD806210  fabs f12, f12
	ctx.f[12].u64 = ctx.f[12].u64 & !0x8000_0000_0000_0000u64;
	// 824A8250: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 824A8254: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824A8258: 40980010  bge cr6, 0x824a8268
	if !ctx.cr[6].lt {
	pc = 0x824A8268; continue 'dispatch;
	}
	// 824A825C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A8260: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 824A8264: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824A8268: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 824A826C: 40980008  bge cr6, 0x824a8274
	if !ctx.cr[6].lt {
	pc = 0x824A8274; continue 'dispatch;
	}
	// 824A8270: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 824A8274: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A8448 size=328
    let mut pc: u32 = 0x824A8448;
    'dispatch: loop {
        match pc {
            0x824A8448 => {
    //   block [0x824A8448..0x824A8590)
	// 824A8448: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A8590 size=16
    let mut pc: u32 = 0x824A8590;
    'dispatch: loop {
        match pc {
            0x824A8590 => {
    //   block [0x824A8590..0x824A85A0)
	// 824A8590: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 824A8594: 388000AC  li r4, 0xac
	ctx.r[4].s64 = 172;
	// 824A8598: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 824A859C: 4BFFA414  b 0x824a29b0
	sub_824A29B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A85A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A85A0 size=20
    let mut pc: u32 = 0x824A85A0;
    'dispatch: loop {
        match pc {
            0x824A85A0 => {
    //   block [0x824A85A0..0x824A85B4)
	// 824A85A0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 824A85A4: 3940001C  li r10, 0x1c
	ctx.r[10].s64 = 28;
	// 824A85A8: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824A85AC: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824A85B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A85B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824A85B8 size=384
    let mut pc: u32 = 0x824A85B8;
    'dispatch: loop {
        match pc {
            0x824A85B8 => {
    //   block [0x824A85B8..0x824A8738)
	// 824A85B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A85BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824A85C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824A85C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824A85C8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 824A85CC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A85D0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824A85D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824A85D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824A85DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824A85E0: 389E0020  addi r4, r30, 0x20
	ctx.r[4].s64 = ctx.r[30].s64 + 32;
	// 824A85E4: C3EB2238  lfs f31, 0x2238(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8760 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824A85E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824A85EC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824A85F0: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824A85F4: 4810316D  bl 0x825ab760
	ctx.lr = 0x824A85F8;
	sub_825AB760(ctx, base);
	// 824A85F8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A85FC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824A8600: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824A8604: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824A8608: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 824A860C: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 824A8610: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824A8614: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824A8618: 419A0024  beq cr6, 0x824a863c
	if ctx.cr[6].eq {
	pc = 0x824A863C; continue 'dispatch;
	}
	// 824A861C: 389E0050  addi r4, r30, 0x50
	ctx.r[4].s64 = ctx.r[30].s64 + 80;
	// 824A8620: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824A8624: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824A8628: 48103139  bl 0x825ab760
	ctx.lr = 0x824A862C;
	sub_825AB760(ctx, base);
	// 824A862C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A8630: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A8634: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824A8638: 409A0008  bne cr6, 0x824a8640
	if !ctx.cr[6].eq {
	pc = 0x824A8640; continue 'dispatch;
	}
	// 824A863C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A8640: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 824A8644: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824A8648: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824A864C: 419A001C  beq cr6, 0x824a8668
	if ctx.cr[6].eq {
	pc = 0x824A8668; continue 'dispatch;
	}
	// 824A8650: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824A8654: C1BE009C  lfs f13, 0x9c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A8658: C00B233C  lfs f0, 0x233c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9020 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A865C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824A8660: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824A8664: 419A0008  beq cr6, 0x824a866c
	if ctx.cr[6].eq {
	pc = 0x824A866C; continue 'dispatch;
	}
	// 824A8668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A866C: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 824A8670: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824A8674: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824A8678: 419A001C  beq cr6, 0x824a8694
	if ctx.cr[6].eq {
	pc = 0x824A8694; continue 'dispatch;
	}
	// 824A867C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824A8680: C1BE00A0  lfs f13, 0xa0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A8684: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A8688: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824A868C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824A8690: 40980008  bge cr6, 0x824a8698
	if !ctx.cr[6].lt {
	pc = 0x824A8698; continue 'dispatch;
	}
	// 824A8694: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A8698: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 824A869C: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824A86A0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824A86A4: 419A001C  beq cr6, 0x824a86c0
	if ctx.cr[6].eq {
	pc = 0x824A86C0; continue 'dispatch;
	}
	// 824A86A8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824A86AC: C1BE00A0  lfs f13, 0xa0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A86B0: C00B2930  lfs f0, 0x2930(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10544 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A86B4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824A86B8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824A86BC: 41980008  blt cr6, 0x824a86c4
	if ctx.cr[6].lt {
	pc = 0x824A86C4; continue 'dispatch;
	}
	// 824A86C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A86C4: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 824A86C8: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824A86CC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824A86D0: 419A0018  beq cr6, 0x824a86e8
	if ctx.cr[6].eq {
	pc = 0x824A86E8; continue 'dispatch;
	}
	// 824A86D4: C01E00B0  lfs f0, 0xb0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A86D8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824A86DC: C1BE00B4  lfs f13, 0xb4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A86E0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 824A86E4: 40990008  ble cr6, 0x824a86ec
	if !ctx.cr[6].gt {
	pc = 0x824A86EC; continue 'dispatch;
	}
	// 824A86E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A86EC: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 824A86F0: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824A86F4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824A86F8: 419A0018  beq cr6, 0x824a8710
	if ctx.cr[6].eq {
	pc = 0x824A8710; continue 'dispatch;
	}
	// 824A86FC: C01E0088  lfs f0, 0x88(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A8700: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824A8704: C1BE008C  lfs f13, 0x8c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A8708: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 824A870C: 40990008  ble cr6, 0x824a8714
	if !ctx.cr[6].gt {
	pc = 0x824A8714; continue 'dispatch;
	}
	// 824A8710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A8714: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824A8718: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824A871C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824A8720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824A8724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824A8728: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 824A872C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824A8730: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824A8734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A8738 size=28
    let mut pc: u32 = 0x824A8738;
    'dispatch: loop {
        match pc {
            0x824A8738 => {
    //   block [0x824A8738..0x824A8754)
	// 824A8738: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 824A873C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824A8740: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824A8744: 556B2636  rlwinm r11, r11, 4, 0x18, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0FFFFFFFu64;
	// 824A8748: 696B0010  xori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 ^ 16;
	// 824A874C: 9963009A  stb r11, 0x9a(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(154 as u32), ctx.r[11].u8 ) };
	// 824A8750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A8758 size=40
    let mut pc: u32 = 0x824A8758;
    'dispatch: loop {
        match pc {
            0x824A8758 => {
    //   block [0x824A8758..0x824A8780)
	// 824A8758: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 824A875C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824A8760: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A8780 size=40
    let mut pc: u32 = 0x824A8780;
    'dispatch: loop {
        match pc {
            0x824A8780 => {
    //   block [0x824A8780..0x824A87A8)
	// 824A8780: 39630050  addi r11, r3, 0x50
	ctx.r[11].s64 = ctx.r[3].s64 + 80;
	// 824A8784: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824A8788: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A87A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A87A8 size=8
    let mut pc: u32 = 0x824A87A8;
    'dispatch: loop {
        match pc {
            0x824A87A8 => {
    //   block [0x824A87A8..0x824A87B0)
	// 824A87A8: 38600013  li r3, 0x13
	ctx.r[3].s64 = 19;
	// 824A87AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A87B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A87B0 size=312
    let mut pc: u32 = 0x824A87B0;
    'dispatch: loop {
        match pc {
            0x824A87B0 => {
    //   block [0x824A87B0..0x824A88E8)
	// 824A87B0: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 824A87B4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 824A87B8: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 824A87BC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 824A87C0: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 824A87C4: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 824A87C8: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A88E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A88E8 size=336
    let mut pc: u32 = 0x824A88E8;
    'dispatch: loop {
        match pc {
            0x824A88E8 => {
    //   block [0x824A88E8..0x824A8A38)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824A8A38 size=80
    let mut pc: u32 = 0x824A8A38;
    'dispatch: loop {
        match pc {
            0x824A8A38 => {
    //   block [0x824A8A38..0x824A8A88)
	// 824A8A38: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824A8A3C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824A8A40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824A8A44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 824A8A48: 396BE320  addi r11, r11, -0x1ce0
	ctx.r[11].s64 = ctx.r[11].s64 + -7392;
	// 824A8A4C: C1AA0858  lfs f13, 0x858(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2136 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824A8A50: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824A8A54: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 824A8A58: 99030008  stb r8, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u8 ) };
	// 824A8A5C: D1A3000C  stfs f13, 0xc(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824A8A60: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824A8A64: C18A24B0  lfs f12, 0x24b0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(9392 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824A8A68: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824A8A6C: D1830010  stfs f12, 0x10(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824A8A70: C00A1FF8  lfs f0, 0x1ff8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A8A74: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 824A8A78: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824A8A7C: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824A8A80: 99430008  stb r10, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 824A8A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824A8A88 size=60
    let mut pc: u32 = 0x824A8A88;
    'dispatch: loop {
        match pc {
            0x824A8A88 => {
    //   block [0x824A8A88..0x824A8AC4)
	// 824A8A88: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824A8A8C: FC001850  fneg f0, f3
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = ctx.f[3].u64 ^ 0x8000_0000_0000_0000u64;
	// 824A8A90: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824A8A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824A8A98: 396BE320  addi r11, r11, -0x1ce0
	ctx.r[11].s64 = ctx.r[11].s64 + -7392;
	// 824A8A9C: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 824A8AA0: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824A8AA4: 99230008  stb r9, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u8 ) };
	// 824A8AA8: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824A8AAC: D0630010  stfs f3, 0x10(r3)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824A8AB0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824A8AB4: D0230014  stfs f1, 0x14(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824A8AB8: 99030008  stb r8, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u8 ) };
	// 824A8ABC: D0430018  stfs f2, 0x18(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824A8AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824A8AC8 size=156
    let mut pc: u32 = 0x824A8AC8;
    'dispatch: loop {
        match pc {
            0x824A8AC8 => {
    //   block [0x824A8AC8..0x824A8B64)
	// 824A8AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A8ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824A8AD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824A8AD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A8AD8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A8ADC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824A8AE0: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 824A8AE4: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 824A8AE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824A8AEC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824A8AF0: 4BFBB549  bl 0x82464038
	ctx.lr = 0x824A8AF4;
	sub_82464038(ctx, base);
	// 824A8AF4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824A8AF8: 3900001C  li r8, 0x1c
	ctx.r[8].s64 = 28;
	// 824A8AFC: 396BE2F8  addi r11, r11, -0x1d08
	ctx.r[11].s64 = ctx.r[11].s64 + -7432;
	// 824A8B00: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 824A8B04: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824A8B08: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824A8B0C: B1030004  sth r8, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 824A8B10: 394AE30C  addi r10, r10, -0x1cf4
	ctx.r[10].s64 = ctx.r[10].s64 + -7412;
	// 824A8B14: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824A8B18: 3929E320  addi r9, r9, -0x1ce0
	ctx.r[9].s64 = ctx.r[9].s64 + -7392;
	// 824A8B1C: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 824A8B20: 897F0008  lbz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A8B24: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824A8B28: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 824A8B2C: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A8B30: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824A8B34: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A8B38: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A8B3C: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824A8B40: C01F0014  lfs f0, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A8B44: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824A8B48: C01F0018  lfs f0, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824A8B4C: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824A8B50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824A8B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824A8B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824A8B5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824A8B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A8B68 size=156
    let mut pc: u32 = 0x824A8B68;
    'dispatch: loop {
        match pc {
            0x824A8B68 => {
    //   block [0x824A8B68..0x824A8C04)
	// 824A8B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A8B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824A8B70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824A8B74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824A8B78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A8B7C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824A8B80: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824A8B84: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824A8B88: 419A001C  beq cr6, 0x824a8ba4
	if ctx.cr[6].eq {
	pc = 0x824A8BA4; continue 'dispatch;
	}
	// 824A8B8C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A8B90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A8B94: 419A0010  beq cr6, 0x824a8ba4
	if ctx.cr[6].eq {
	pc = 0x824A8BA4; continue 'dispatch;
	}
	// 824A8B98: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 824A8B9C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824A8BA0: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824A8BA4: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 824A8BA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824A8BAC: 419A003C  beq cr6, 0x824a8be8
	if ctx.cr[6].eq {
	pc = 0x824A8BE8; continue 'dispatch;
	}
	// 824A8BB0: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A8BB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A8BB8: 419A0030  beq cr6, 0x824a8be8
	if ctx.cr[6].eq {
	pc = 0x824A8BE8; continue 'dispatch;
	}
	// 824A8BBC: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824A8BC0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824A8BC4: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824A8BC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A8BCC: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824A8BD0: 409A0018  bne cr6, 0x824a8be8
	if !ctx.cr[6].eq {
	pc = 0x824A8BE8; continue 'dispatch;
	}
	// 824A8BD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A8BD8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824A8BDC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A8BE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A8BE4: 4E800421  bctrl
	ctx.lr = 0x824A8BE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A8BE8: 93FE0018  stw r31, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 824A8BEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824A8BF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824A8BF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824A8BF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824A8BFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824A8C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A8C08 size=20
    let mut pc: u32 = 0x824A8C08;
    'dispatch: loop {
        match pc {
            0x824A8C08 => {
    //   block [0x824A8C08..0x824A8C1C)
	// 824A8C08: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 824A8C0C: 3940003C  li r10, 0x3c
	ctx.r[10].s64 = 60;
	// 824A8C10: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824A8C14: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824A8C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A8C20 size=16
    let mut pc: u32 = 0x824A8C20;
    'dispatch: loop {
        match pc {
            0x824A8C20 => {
    //   block [0x824A8C20..0x824A8C30)
	// 824A8C20: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824A8C24: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 824A8C28: 38AB0060  addi r5, r11, 0x60
	ctx.r[5].s64 = ctx.r[11].s64 + 96;
	// 824A8C2C: 48102EDC  b 0x825abb08
	sub_825ABB08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A8C30 size=8
    let mut pc: u32 = 0x824A8C30;
    'dispatch: loop {
        match pc {
            0x824A8C30 => {
    //   block [0x824A8C30..0x824A8C38)
	// 824A8C30: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 824A8C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A8C38 size=292
    let mut pc: u32 = 0x824A8C38;
    'dispatch: loop {
        match pc {
            0x824A8C38 => {
    //   block [0x824A8C38..0x824A8D5C)
	// 824A8C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A8C3C: 4808C47D  bl 0x825350b8
	ctx.lr = 0x824A8C40;
	sub_82535080(ctx, base);
	// 824A8C40: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A8C44: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 824A8C48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824A8C4C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824A8C50: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 824A8C54: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824A8C58: 419A001C  beq cr6, 0x824a8c74
	if ctx.cr[6].eq {
	pc = 0x824A8C74; continue 'dispatch;
	}
	// 824A8C5C: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A8C60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A8C64: 419A0010  beq cr6, 0x824a8c74
	if ctx.cr[6].eq {
	pc = 0x824A8C74; continue 'dispatch;
	}
	// 824A8C68: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 824A8C6C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824A8C70: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824A8C74: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824A8C78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824A8C7C: 419A003C  beq cr6, 0x824a8cb8
	if ctx.cr[6].eq {
	pc = 0x824A8CB8; continue 'dispatch;
	}
	// 824A8C80: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A8C84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A8C88: 419A0030  beq cr6, 0x824a8cb8
	if ctx.cr[6].eq {
	pc = 0x824A8CB8; continue 'dispatch;
	}
	// 824A8C8C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824A8C90: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824A8C94: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824A8C98: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A8C9C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824A8CA0: 409A0018  bne cr6, 0x824a8cb8
	if !ctx.cr[6].eq {
	pc = 0x824A8CB8; continue 'dispatch;
	}
	// 824A8CA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A8CA8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824A8CAC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A8CB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A8CB4: 4E800421  bctrl
	ctx.lr = 0x824A8CB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A8CB8: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 824A8CBC: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 824A8CC0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 824A8CC4: 397F0030  addi r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 + 48;
	// 824A8CC8: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 824A8CCC: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A8D60 size=236
    let mut pc: u32 = 0x824A8D60;
    'dispatch: loop {
        match pc {
            0x824A8D60 => {
    //   block [0x824A8D60..0x824A8E4C)
	// 824A8D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A8D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824A8D68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824A8D6C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A8D70: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824A8D74: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 824A8D78: 3909E418  addi r8, r9, -0x1be8
	ctx.r[8].s64 = ctx.r[9].s64 + -7144;
	// 824A8D7C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 824A8D80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824A8D84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A8E50 size=132
    let mut pc: u32 = 0x824A8E50;
    'dispatch: loop {
        match pc {
            0x824A8E50 => {
    //   block [0x824A8E50..0x824A8ED4)
	// 824A8E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A8E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824A8E58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824A8E5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A8E60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824A8E64: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824A8E68: 396BE418  addi r11, r11, -0x1be8
	ctx.r[11].s64 = ctx.r[11].s64 + -7144;
	// 824A8E6C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824A8E70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824A8E74: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824A8E78: 419A003C  beq cr6, 0x824a8eb4
	if ctx.cr[6].eq {
	pc = 0x824A8EB4; continue 'dispatch;
	}
	// 824A8E7C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A8E80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A8E84: 419A0030  beq cr6, 0x824a8eb4
	if ctx.cr[6].eq {
	pc = 0x824A8EB4; continue 'dispatch;
	}
	// 824A8E88: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824A8E8C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824A8E90: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824A8E94: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A8E98: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824A8E9C: 409A0018  bne cr6, 0x824a8eb4
	if !ctx.cr[6].eq {
	pc = 0x824A8EB4; continue 'dispatch;
	}
	// 824A8EA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A8EA4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824A8EA8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A8EAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A8EB0: 4E800421  bctrl
	ctx.lr = 0x824A8EB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A8EB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824A8EB8: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824A8EBC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824A8EC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824A8EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824A8EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824A8ECC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824A8ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A8ED8 size=52
    let mut pc: u32 = 0x824A8ED8;
    'dispatch: loop {
        match pc {
            0x824A8ED8 => {
    //   block [0x824A8ED8..0x824A8F0C)
	// 824A8ED8: 3963000C  addi r11, r3, 0xc
	ctx.r[11].s64 = ctx.r[3].s64 + 12;
	// 824A8EDC: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 824A8EE0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824A8EE4: 39000180  li r8, 0x180
	ctx.r[8].s64 = 384;
	// 824A8EE8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 824A8EEC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 824A8EF0: 91640010  stw r11, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824A8EF4: 91440014  stw r10, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 824A8EF8: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824A8EFC: 91040004  stw r8, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824A8F00: 90E40008  stw r7, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 824A8F04: 90C4000C  stw r6, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 824A8F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A8F10 size=40
    let mut pc: u32 = 0x824A8F10;
    'dispatch: loop {
        match pc {
            0x824A8F10 => {
    //   block [0x824A8F10..0x824A8F38)
	// 824A8F10: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 824A8F14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A8F18: 419A0014  beq cr6, 0x824a8f2c
	if ctx.cr[6].eq {
	pc = 0x824A8F2C; continue 'dispatch;
	}
	// 824A8F1C: 89640020  lbz r11, 0x20(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 824A8F20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A8F24: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824A8F28: 409A0008  bne cr6, 0x824a8f30
	if !ctx.cr[6].eq {
	pc = 0x824A8F30; continue 'dispatch;
	}
	// 824A8F2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A8F30: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824A8F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A8F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A8F38 size=512
    let mut pc: u32 = 0x824A8F38;
    'dispatch: loop {
        match pc {
            0x824A8F38 => {
    //   block [0x824A8F38..0x824A9138)
	// 824A8F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A8F3C: 4808C16D  bl 0x825350a8
	ctx.lr = 0x824A8F40;
	sub_82535080(ctx, base);
	// 824A8F40: DBE1FFB0  stfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 824A8F44: 9421FDF0  stwu r1, -0x210(r1)
	ea = ctx.r[1].u32.wrapping_add(-528 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A8F48: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 824A8F4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824A8F50: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 824A8F54: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 824A8F58: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 824A8F5C: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 824A8F60: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 824A8F64: 419A001C  beq cr6, 0x824a8f80
	if ctx.cr[6].eq {
	pc = 0x824A8F80; continue 'dispatch;
	}
	// 824A8F68: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A8F6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A8F70: 419A0010  beq cr6, 0x824a8f80
	if ctx.cr[6].eq {
	pc = 0x824A8F80; continue 'dispatch;
	}
	// 824A8F74: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 824A8F78: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824A8F7C: B17D0006  sth r11, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824A8F80: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824A8F84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824A8F88: 419A003C  beq cr6, 0x824a8fc4
	if ctx.cr[6].eq {
	pc = 0x824A8FC4; continue 'dispatch;
	}
	// 824A8F8C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A8F90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A8F94: 419A0030  beq cr6, 0x824a8fc4
	if ctx.cr[6].eq {
	pc = 0x824A8FC4; continue 'dispatch;
	}
	// 824A8F98: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824A8F9C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824A8FA0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824A8FA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A8FA8: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824A8FAC: 409A0018  bne cr6, 0x824a8fc4
	if !ctx.cr[6].eq {
	pc = 0x824A8FC4; continue 'dispatch;
	}
	// 824A8FB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A8FB4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824A8FB8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A8FBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A8FC0: 4E800421  bctrl
	ctx.lr = 0x824A8FC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A8FC4: 397F0070  addi r11, r31, 0x70
	ctx.r[11].s64 = ctx.r[31].s64 + 112;
	// 824A8FC8: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 824A8FCC: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A9138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A9138 size=1140
    let mut pc: u32 = 0x824A9138;
    'dispatch: loop {
        match pc {
            0x824A9138 => {
    //   block [0x824A9138..0x824A95AC)
	// 824A9138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A913C: 4808BF79  bl 0x825350b4
	ctx.lr = 0x824A9140;
	sub_82535080(ctx, base);
	// 824A9140: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A95B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A95B0 size=16
    let mut pc: u32 = 0x824A95B0;
    'dispatch: loop {
        match pc {
            0x824A95B0 => {
    //   block [0x824A95B0..0x824A95C0)
	// 824A95B0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A95B4: 390BFFFF  addi r8, r11, -1
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	// 824A95B8: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 824A95BC: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A95C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A95C0 size=104
    let mut pc: u32 = 0x824A95C0;
    'dispatch: loop {
        match pc {
            0x824A95C0 => {
    //   block [0x824A95C0..0x824A9628)
	// 824A95C0: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824A95C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A95C8: 7D67582E  lwzx r11, r7, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824A95CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A95D0: 409A0044  bne cr6, 0x824a9614
	if !ctx.cr[6].eq {
	pc = 0x824A9614; continue 'dispatch;
	}
	// 824A95D4: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A95D8: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 824A95DC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824A95E0: 7F085000  cmpw cr6, r8, r10
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824A95E4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824A95E8: 4098002C  bge cr6, 0x824a9614
	if !ctx.cr[6].lt {
	pc = 0x824A9614; continue 'dispatch;
	}
	// 824A95EC: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 824A95F0: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A95F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824A95F8: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824A95FC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824A9600: 80C90004  lwz r6, 4(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A9604: 90C90000  stw r6, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 824A9608: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A960C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824A9610: 4198FFE0  blt cr6, 0x824a95f0
	if ctx.cr[6].lt {
	pc = 0x824A95F0; continue 'dispatch;
	}
	// 824A9614: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 824A9618: 38E7FFFC  addi r7, r7, -4
	ctx.r[7].s64 = ctx.r[7].s64 + -4;
	// 824A961C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 824A9620: 4098FFA4  bge cr6, 0x824a95c4
	if !ctx.cr[6].lt {
	pc = 0x824A95C4; continue 'dispatch;
	}
	// 824A9624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A9628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A9628 size=16
    let mut pc: u32 = 0x824A9628;
    'dispatch: loop {
        match pc {
            0x824A9628 => {
    //   block [0x824A9628..0x824A9638)
	// 824A9628: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A962C: 390BFFFF  addi r8, r11, -1
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	// 824A9630: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 824A9634: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A9638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A9638 size=104
    let mut pc: u32 = 0x824A9638;
    'dispatch: loop {
        match pc {
            0x824A9638 => {
    //   block [0x824A9638..0x824A96A0)
	// 824A9638: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824A963C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9640: 7D67582E  lwzx r11, r7, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824A9644: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A9648: 409A0044  bne cr6, 0x824a968c
	if !ctx.cr[6].eq {
	pc = 0x824A968C; continue 'dispatch;
	}
	// 824A964C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A9650: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 824A9654: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824A9658: 7F085000  cmpw cr6, r8, r10
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824A965C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824A9660: 4098002C  bge cr6, 0x824a968c
	if !ctx.cr[6].lt {
	pc = 0x824A968C; continue 'dispatch;
	}
	// 824A9664: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 824A9668: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A966C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824A9670: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824A9674: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824A9678: 80C90004  lwz r6, 4(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A967C: 90C90000  stw r6, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 824A9680: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A9684: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824A9688: 4198FFE0  blt cr6, 0x824a9668
	if ctx.cr[6].lt {
	pc = 0x824A9668; continue 'dispatch;
	}
	// 824A968C: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 824A9690: 38E7FFFC  addi r7, r7, -4
	ctx.r[7].s64 = ctx.r[7].s64 + -4;
	// 824A9694: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 824A9698: 4098FFA4  bge cr6, 0x824a963c
	if !ctx.cr[6].lt {
	pc = 0x824A963C; continue 'dispatch;
	}
	// 824A969C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A96A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A96A0 size=100
    let mut pc: u32 = 0x824A96A0;
    'dispatch: loop {
        match pc {
            0x824A96A0 => {
    //   block [0x824A96A0..0x824A9704)
	// 824A96A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A96A4: 4808BA19  bl 0x825350bc
	ctx.lr = 0x824A96A8;
	sub_82535080(ctx, base);
	// 824A96A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A96AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824A96B0: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 824A96B4: 3BABFFFF  addi r29, r11, -1
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	// 824A96B8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824A96BC: 41980040  blt cr6, 0x824a96fc
	if ctx.cr[6].lt {
	pc = 0x824A96FC; continue 'dispatch;
	}
	// 824A96C0: 57BF103A  slwi r31, r29, 2
	ctx.r[31].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 824A96C4: 817E008C  lwz r11, 0x8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 824A96C8: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824A96CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824A96D0: 419A001C  beq cr6, 0x824a96ec
	if ctx.cr[6].eq {
	pc = 0x824A96EC; continue 'dispatch;
	}
	// 824A96D4: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824A96D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824A96DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A96E0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824A96E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A96E8: 4E800421  bctrl
	ctx.lr = 0x824A96EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A96EC: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 824A96F0: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 824A96F4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824A96F8: 4098FFCC  bge cr6, 0x824a96c4
	if !ctx.cr[6].lt {
	pc = 0x824A96C4; continue 'dispatch;
	}
	// 824A96FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824A9700: 4808BA0C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A9708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A9708 size=108
    let mut pc: u32 = 0x824A9708;
    'dispatch: loop {
        match pc {
            0x824A9708 => {
    //   block [0x824A9708..0x824A9774)
	// 824A9708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A970C: 4808B9B1  bl 0x825350bc
	ctx.lr = 0x824A9710;
	sub_82535080(ctx, base);
	// 824A9710: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A9714: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824A9718: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 824A971C: 3BABFFFF  addi r29, r11, -1
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	// 824A9720: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824A9724: 41980040  blt cr6, 0x824a9764
	if ctx.cr[6].lt {
	pc = 0x824A9764; continue 'dispatch;
	}
	// 824A9728: 57BF103A  slwi r31, r29, 2
	ctx.r[31].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 824A972C: 817E008C  lwz r11, 0x8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 824A9730: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824A9734: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824A9738: 419A001C  beq cr6, 0x824a9754
	if ctx.cr[6].eq {
	pc = 0x824A9754; continue 'dispatch;
	}
	// 824A973C: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824A9740: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824A9744: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9748: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A974C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A9750: 4E800421  bctrl
	ctx.lr = 0x824A9754;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A9754: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 824A9758: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 824A975C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824A9760: 4098FFCC  bge cr6, 0x824a972c
	if !ctx.cr[6].lt {
	pc = 0x824A972C; continue 'dispatch;
	}
	// 824A9764: 387E008C  addi r3, r30, 0x8c
	ctx.r[3].s64 = ctx.r[30].s64 + 140;
	// 824A9768: 4BFFFE49  bl 0x824a95b0
	ctx.lr = 0x824A976C;
	sub_824A95B0(ctx, base);
	// 824A976C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824A9770: 4808B99C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A9778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A9778 size=108
    let mut pc: u32 = 0x824A9778;
    'dispatch: loop {
        match pc {
            0x824A9778 => {
    //   block [0x824A9778..0x824A97E4)
	// 824A9778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A977C: 4808B941  bl 0x825350bc
	ctx.lr = 0x824A9780;
	sub_82535080(ctx, base);
	// 824A9780: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A9784: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824A9788: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 824A978C: 3BABFFFF  addi r29, r11, -1
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	// 824A9790: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824A9794: 41980040  blt cr6, 0x824a97d4
	if ctx.cr[6].lt {
	pc = 0x824A97D4; continue 'dispatch;
	}
	// 824A9798: 57BF103A  slwi r31, r29, 2
	ctx.r[31].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 824A979C: 817E008C  lwz r11, 0x8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 824A97A0: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824A97A4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824A97A8: 419A001C  beq cr6, 0x824a97c4
	if ctx.cr[6].eq {
	pc = 0x824A97C4; continue 'dispatch;
	}
	// 824A97AC: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824A97B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824A97B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A97B8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A97BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A97C0: 4E800421  bctrl
	ctx.lr = 0x824A97C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A97C4: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 824A97C8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 824A97CC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824A97D0: 4098FFCC  bge cr6, 0x824a979c
	if !ctx.cr[6].lt {
	pc = 0x824A979C; continue 'dispatch;
	}
	// 824A97D4: 387E008C  addi r3, r30, 0x8c
	ctx.r[3].s64 = ctx.r[30].s64 + 140;
	// 824A97D8: 4BFFFDD9  bl 0x824a95b0
	ctx.lr = 0x824A97DC;
	sub_824A95B0(ctx, base);
	// 824A97DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824A97E0: 4808B92C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A97E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A97E8 size=108
    let mut pc: u32 = 0x824A97E8;
    'dispatch: loop {
        match pc {
            0x824A97E8 => {
    //   block [0x824A97E8..0x824A9854)
	// 824A97E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A97EC: 4808B8D1  bl 0x825350bc
	ctx.lr = 0x824A97F0;
	sub_82535080(ctx, base);
	// 824A97F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A97F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824A97F8: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 824A97FC: 3BABFFFF  addi r29, r11, -1
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	// 824A9800: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824A9804: 41980040  blt cr6, 0x824a9844
	if ctx.cr[6].lt {
	pc = 0x824A9844; continue 'dispatch;
	}
	// 824A9808: 57BF103A  slwi r31, r29, 2
	ctx.r[31].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 824A980C: 817E008C  lwz r11, 0x8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 824A9810: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824A9814: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824A9818: 419A001C  beq cr6, 0x824a9834
	if ctx.cr[6].eq {
	pc = 0x824A9834; continue 'dispatch;
	}
	// 824A981C: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824A9820: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824A9824: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9828: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824A982C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A9830: 4E800421  bctrl
	ctx.lr = 0x824A9834;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A9834: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 824A9838: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 824A983C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824A9840: 4098FFCC  bge cr6, 0x824a980c
	if !ctx.cr[6].lt {
	pc = 0x824A980C; continue 'dispatch;
	}
	// 824A9844: 387E008C  addi r3, r30, 0x8c
	ctx.r[3].s64 = ctx.r[30].s64 + 140;
	// 824A9848: 4BFFFD69  bl 0x824a95b0
	ctx.lr = 0x824A984C;
	sub_824A95B0(ctx, base);
	// 824A984C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824A9850: 4808B8BC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A9858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A9858 size=76
    let mut pc: u32 = 0x824A9858;
    'dispatch: loop {
        match pc {
            0x824A9858 => {
    //   block [0x824A9858..0x824A98A4)
	// 824A9858: 3903008C  addi r8, r3, 0x8c
	ctx.r[8].s64 = ctx.r[3].s64 + 140;
	// 824A985C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A9860: 81280004  lwz r9, 4(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A9864: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824A9868: 40990024  ble cr6, 0x824a988c
	if !ctx.cr[6].gt {
	pc = 0x824A988C; continue 'dispatch;
	}
	// 824A986C: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9870: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9874: 7F072040  cmplw cr6, r7, r4
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824A9878: 419A0018  beq cr6, 0x824a9890
	if ctx.cr[6].eq {
	pc = 0x824A9890; continue 'dispatch;
	}
	// 824A987C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824A9880: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824A9884: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824A9888: 4198FFE8  blt cr6, 0x824a9870
	if ctx.cr[6].lt {
	pc = 0x824A9870; continue 'dispatch;
	}
	// 824A988C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 824A9890: 81480004  lwz r10, 4(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A9894: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824A9898: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824A989C: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824A98A0: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A98A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A98A4 size=44
    let mut pc: u32 = 0x824A98A4;
    'dispatch: loop {
        match pc {
            0x824A98A4 => {
    //   block [0x824A98A4..0x824A98D0)
	// 824A98A4: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824A98A8: 81280000  lwz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A98AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824A98B0: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 824A98B4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824A98B8: 80E90004  lwz r7, 4(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A98BC: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 824A98C0: 81280004  lwz r9, 4(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A98C4: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824A98C8: 4198FFE0  blt cr6, 0x824a98a8
	if ctx.cr[6].lt {
	pc = 0x824A98A8; continue 'dispatch;
	}
	// 824A98CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A98D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A98D0 size=76
    let mut pc: u32 = 0x824A98D0;
    'dispatch: loop {
        match pc {
            0x824A98D0 => {
    //   block [0x824A98D0..0x824A991C)
	// 824A98D0: 39030080  addi r8, r3, 0x80
	ctx.r[8].s64 = ctx.r[3].s64 + 128;
	// 824A98D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824A98D8: 81280004  lwz r9, 4(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A98DC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824A98E0: 40990024  ble cr6, 0x824a9904
	if !ctx.cr[6].gt {
	pc = 0x824A9904; continue 'dispatch;
	}
	// 824A98E4: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A98E8: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A98EC: 7F072040  cmplw cr6, r7, r4
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824A98F0: 419A0018  beq cr6, 0x824a9908
	if ctx.cr[6].eq {
	pc = 0x824A9908; continue 'dispatch;
	}
	// 824A98F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824A98F8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824A98FC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824A9900: 4198FFE8  blt cr6, 0x824a98e8
	if ctx.cr[6].lt {
	pc = 0x824A98E8; continue 'dispatch;
	}
	// 824A9904: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 824A9908: 81480004  lwz r10, 4(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A990C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824A9910: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824A9914: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824A9918: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A991C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A991C size=44
    let mut pc: u32 = 0x824A991C;
    'dispatch: loop {
        match pc {
            0x824A991C => {
    //   block [0x824A991C..0x824A9948)
	// 824A991C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824A9920: 81280000  lwz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9924: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824A9928: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 824A992C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824A9930: 80E90004  lwz r7, 4(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A9934: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 824A9938: 81280004  lwz r9, 4(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A993C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824A9940: 4198FFE0  blt cr6, 0x824a9920
	if ctx.cr[6].lt {
	pc = 0x824A9920; continue 'dispatch;
	}
	// 824A9944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A9948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A9948 size=112
    let mut pc: u32 = 0x824A9948;
    'dispatch: loop {
        match pc {
            0x824A9948 => {
    //   block [0x824A9948..0x824A99B8)
	// 824A9948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A994C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824A9950: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824A9954: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824A9958: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A995C: 3BE3008C  addi r31, r3, 0x8c
	ctx.r[31].s64 = ctx.r[3].s64 + 140;
	// 824A9960: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824A9964: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9968: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A996C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824A9970: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824A9974: 409A0010  bne cr6, 0x824a9984
	if !ctx.cr[6].eq {
	pc = 0x824A9984; continue 'dispatch;
	}
	// 824A9978: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824A997C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824A9980: 4BFC49D1  bl 0x8246e350
	ctx.lr = 0x824A9984;
	sub_8246E350(ctx, base);
	// 824A9984: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A9988: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A998C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824A9990: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 824A9994: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A9998: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824A999C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824A99A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824A99A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824A99A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824A99AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824A99B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824A99B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A99B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A99B8 size=112
    let mut pc: u32 = 0x824A99B8;
    'dispatch: loop {
        match pc {
            0x824A99B8 => {
    //   block [0x824A99B8..0x824A9A28)
	// 824A99B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A99BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824A99C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824A99C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824A99C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A99CC: 3BE30080  addi r31, r3, 0x80
	ctx.r[31].s64 = ctx.r[3].s64 + 128;
	// 824A99D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824A99D4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A99D8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A99DC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824A99E0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824A99E4: 409A0010  bne cr6, 0x824a99f4
	if !ctx.cr[6].eq {
	pc = 0x824A99F4; continue 'dispatch;
	}
	// 824A99E8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824A99EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824A99F0: 4BFC4961  bl 0x8246e350
	ctx.lr = 0x824A99F4;
	sub_8246E350(ctx, base);
	// 824A99F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A99F8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A99FC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824A9A00: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 824A9A04: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A9A08: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824A9A0C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824A9A10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824A9A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824A9A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824A9A1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824A9A20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824A9A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A9A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A9A28 size=184
    let mut pc: u32 = 0x824A9A28;
    'dispatch: loop {
        match pc {
            0x824A9A28 => {
    //   block [0x824A9A28..0x824A9AE0)
	// 824A9A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A9A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824A9A30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824A9A34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824A9A38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A9A3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824A9A40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824A9A44: 4BFF231D  bl 0x8249bd60
	ctx.lr = 0x824A9A48;
	sub_8249BD60(ctx, base);
	// 824A9A48: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 824A9A4C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824A9A50: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824A9A54: 409A0034  bne cr6, 0x824a9a88
	if !ctx.cr[6].eq {
	pc = 0x824A9A88; continue 'dispatch;
	}
	// 824A9A58: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9A5C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824A9A60: 80FF0084  lwz r7, 0x84(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 824A9A64: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824A9A68: 80DF0080  lwz r6, 0x80(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 824A9A6C: 388A0880  addi r4, r10, 0x880
	ctx.r[4].s64 = ctx.r[10].s64 + 2176;
	// 824A9A70: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824A9A74: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 824A9A78: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9A7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824A9A80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A9A84: 4E800421  bctrl
	ctx.lr = 0x824A9A88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A9A88: 815F0094  lwz r10, 0x94(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 824A9A8C: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 824A9A90: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A9A94: 409A0034  bne cr6, 0x824a9ac8
	if !ctx.cr[6].eq {
	pc = 0x824A9AC8; continue 'dispatch;
	}
	// 824A9A98: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9A9C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824A9AA0: 80FF0090  lwz r7, 0x90(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 824A9AA4: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824A9AA8: 388B0870  addi r4, r11, 0x870
	ctx.r[4].s64 = ctx.r[11].s64 + 2160;
	// 824A9AAC: 80DF008C  lwz r6, 0x8c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 824A9AB0: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824A9AB4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 824A9AB8: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9ABC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824A9AC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A9AC4: 4E800421  bctrl
	ctx.lr = 0x824A9AC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A9AC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824A9ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824A9AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824A9AD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824A9AD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824A9ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A9AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A9AE0 size=192
    let mut pc: u32 = 0x824A9AE0;
    'dispatch: loop {
        match pc {
            0x824A9AE0 => {
    //   block [0x824A9AE0..0x824A9BA0)
	// 824A9AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A9AE4: 4808B5D5  bl 0x825350b8
	ctx.lr = 0x824A9AE8;
	sub_82535080(ctx, base);
	// 824A9AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A9AEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824A9AF0: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 824A9AF4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824A9AF8: 817E0084  lwz r11, 0x84(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 824A9AFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A9B00: 409A0048  bne cr6, 0x824a9b48
	if !ctx.cr[6].eq {
	pc = 0x824A9B48; continue 'dispatch;
	}
	// 824A9B04: 3BFE0080  addi r31, r30, 0x80
	ctx.r[31].s64 = ctx.r[30].s64 + 128;
	// 824A9B08: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9B0C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824A9B10: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824A9B14: 409A0020  bne cr6, 0x824a9b34
	if !ctx.cr[6].eq {
	pc = 0x824A9B34; continue 'dispatch;
	}
	// 824A9B18: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9B1C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824A9B20: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824A9B24: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9B28: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824A9B2C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824A9B30: 4BFBA589  bl 0x824640b8
	ctx.lr = 0x824A9B34;
	sub_824640B8(ctx, base);
	// 824A9B34: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9B38: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 824A9B3C: 538BF880  rlwimi r11, r28, 0x1f, 2, 0
	ctx.r[11].u64 = (((ctx.r[28].u32).rotate_left(31) as u64) & 0xFFFFFFFFBFFFFFFF) | (ctx.r[11].u64 & 0x0000000040000000);
	// 824A9B40: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 824A9B44: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824A9B48: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 824A9B4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A9B50: 409A0048  bne cr6, 0x824a9b98
	if !ctx.cr[6].eq {
	pc = 0x824A9B98; continue 'dispatch;
	}
	// 824A9B54: 3BFE008C  addi r31, r30, 0x8c
	ctx.r[31].s64 = ctx.r[30].s64 + 140;
	// 824A9B58: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9B5C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824A9B60: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824A9B64: 409A0020  bne cr6, 0x824a9b84
	if !ctx.cr[6].eq {
	pc = 0x824A9B84; continue 'dispatch;
	}
	// 824A9B68: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9B6C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824A9B70: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824A9B74: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9B78: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824A9B7C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824A9B80: 4BFBA539  bl 0x824640b8
	ctx.lr = 0x824A9B84;
	sub_824640B8(ctx, base);
	// 824A9B84: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9B88: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 824A9B8C: 538BF880  rlwimi r11, r28, 0x1f, 2, 0
	ctx.r[11].u64 = (((ctx.r[28].u32).rotate_left(31) as u64) & 0xFFFFFFFFBFFFFFFF) | (ctx.r[11].u64 & 0x0000000040000000);
	// 824A9B90: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 824A9B94: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824A9B98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824A9B9C: 4808B56C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A9BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A9BA0 size=152
    let mut pc: u32 = 0x824A9BA0;
    'dispatch: loop {
        match pc {
            0x824A9BA0 => {
    //   block [0x824A9BA0..0x824A9C38)
	// 824A9BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A9BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824A9BA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824A9BAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A9BB0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824A9BB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824A9BB8: 396BE47C  addi r11, r11, -0x1b84
	ctx.r[11].s64 = ctx.r[11].s64 + -7044;
	// 824A9BBC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824A9BC0: 4BFFFAE1  bl 0x824a96a0
	ctx.lr = 0x824A9BC4;
	sub_824A96A0(ctx, base);
	// 824A9BC4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 824A9BC8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824A9BCC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824A9BD0: 409A0020  bne cr6, 0x824a9bf0
	if !ctx.cr[6].eq {
	pc = 0x824A9BF0; continue 'dispatch;
	}
	// 824A9BD4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9BD8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824A9BDC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824A9BE0: 809F008C  lwz r4, 0x8c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 824A9BE4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824A9BE8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824A9BEC: 4BFBA4CD  bl 0x824640b8
	ctx.lr = 0x824A9BF0;
	sub_824640B8(ctx, base);
	// 824A9BF0: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 824A9BF4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824A9BF8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824A9BFC: 409A0020  bne cr6, 0x824a9c1c
	if !ctx.cr[6].eq {
	pc = 0x824A9C1C; continue 'dispatch;
	}
	// 824A9C00: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9C04: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824A9C08: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824A9C0C: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 824A9C10: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824A9C14: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824A9C18: 4BFBA4A1  bl 0x824640b8
	ctx.lr = 0x824A9C1C;
	sub_824640B8(ctx, base);
	// 824A9C1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824A9C20: 4BFE5841  bl 0x8248f460
	ctx.lr = 0x824A9C24;
	sub_8248F460(ctx, base);
	// 824A9C24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824A9C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824A9C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824A9C30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824A9C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A9C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A9C38 size=720
    let mut pc: u32 = 0x824A9C38;
    'dispatch: loop {
        match pc {
            0x824A9C38 => {
    //   block [0x824A9C38..0x824A9F08)
	// 824A9C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A9C3C: 4808B471  bl 0x825350ac
	ctx.lr = 0x824A9C40;
	sub_82535080(ctx, base);
	// 824A9C40: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A9C44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824A9C48: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 824A9C4C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9C50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824A9C54: 419A02AC  beq cr6, 0x824a9f00
	if ctx.cr[6].eq {
	pc = 0x824A9F00; continue 'dispatch;
	}
	// 824A9C58: 81630088  lwz r11, 0x88(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(136 as u32) ) } as u64;
	// 824A9C5C: 81430084  lwz r10, 0x84(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 824A9C60: 7D6B5215  add. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A9C64: 41820024  beq 0x824a9c88
	if ctx.cr[0].eq {
	pc = 0x824A9C88; continue 'dispatch;
	}
	// 824A9C68: 39600011  li r11, 0x11
	ctx.r[11].s64 = 17;
	// 824A9C6C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 824A9C70: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 824A9C74: 93210060  stw r25, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[25].u32 ) };
	// 824A9C78: 99610058  stb r11, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 824A9C7C: 4BFE955D  bl 0x824931d8
	ctx.lr = 0x824A9C80;
	sub_824931D8(ctx, base);
	// 824A9C80: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824A9C84: 4808B478  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 824A9C88: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9C8C: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 824A9C90: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9C94: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824A9C98: 3FA08000  lis r29, -0x8000
	ctx.r[29].s64 = -2147483648;
	// 824A9C9C: 814B0084  lwz r10, 0x84(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 824A9CA0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824A9CA4: 914B0084  stw r10, 0x84(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 824A9CA8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9CAC: 7D7BD02E  lwzx r11, r27, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 824A9CB0: 83CA02FC  lwz r30, 0x2fc(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(764 as u32) ) } as u64;
	// 824A9CB4: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 824A9CB8: 395E0002  addi r10, r30, 2
	ctx.r[10].s64 = ctx.r[30].s64 + 2;
	// 824A9CBC: 93810084  stw r28, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[28].u32 ) };
	// 824A9CC0: 93A10088  stw r29, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[29].u32 ) };
	// 824A9CC4: 806B0020  lwz r3, 0x20(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824A9CC8: 55441836  rlwinm r4, r10, 3, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x1FFFFFFFu64;
	// 824A9CCC: 812B002C  lwz r9, 0x2c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 824A9CD0: 7D432214  add r10, r3, r4
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 824A9CD4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824A9CD8: 4199000C  bgt cr6, 0x824a9ce4
	if ctx.cr[6].gt {
	pc = 0x824A9CE4; continue 'dispatch;
	}
	// 824A9CDC: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 824A9CE0: 48000018  b 0x824a9cf8
	pc = 0x824A9CF8; continue 'dispatch;
	// 824A9CE4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9CE8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 824A9CEC: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 824A9CF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A9CF4: 4E800421  bctrl
	ctx.lr = 0x824A9CF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A9CF8: 7FC9EB78  or r9, r30, r29
	ctx.r[9].u64 = ctx.r[30].u64 | ctx.r[29].u64;
	// 824A9CFC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9D00: 90610080  stw r3, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[3].u32 ) };
	// 824A9D04: 9061008C  stw r3, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[3].u32 ) };
	// 824A9D08: 7D7BD02E  lwzx r11, r27, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 824A9D0C: 91210088  stw r9, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[9].u32 ) };
	// 824A9D10: 83CA02FC  lwz r30, 0x2fc(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(764 as u32) ) } as u64;
	// 824A9D14: 93810070  stw r28, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[28].u32 ) };
	// 824A9D18: 393E0002  addi r9, r30, 2
	ctx.r[9].s64 = ctx.r[30].s64 + 2;
	// 824A9D1C: 93810074  stw r28, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[28].u32 ) };
	// 824A9D20: 93A10078  stw r29, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 824A9D24: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824A9D28: 55241836  rlwinm r4, r9, 3, 0, 0x1b
	ctx.r[4].u64 = ctx.r[9].u32 as u64 & 0x1FFFFFFFu64;
	// 824A9D2C: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 824A9D30: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 824A9D34: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 824A9D38: 4199000C  bgt cr6, 0x824a9d44
	if ctx.cr[6].gt {
	pc = 0x824A9D44; continue 'dispatch;
	}
	// 824A9D3C: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 824A9D40: 4800001C  b 0x824a9d5c
	pc = 0x824A9D5C; continue 'dispatch;
	// 824A9D44: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9D48: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 824A9D4C: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 824A9D50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A9D54: 4E800421  bctrl
	ctx.lr = 0x824A9D58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A9D58: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 824A9D5C: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 824A9D60: 7FC9EB78  or r9, r30, r29
	ctx.r[9].u64 = ctx.r[30].u64 | ctx.r[29].u64;
	// 824A9D64: 9141007C  stw r10, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 824A9D68: 395F0024  addi r10, r31, 0x24
	ctx.r[10].s64 = ctx.r[31].s64 + 36;
	// 824A9D6C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9D70: 91210078  stw r9, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[9].u32 ) };
	// 824A9D74: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 824A9D78: 806B0054  lwz r3, 0x54(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 824A9D7C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824A9D80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A9D84: 419A0008  beq cr6, 0x824a9d8c
	if ctx.cr[6].eq {
	pc = 0x824A9D8C; continue 'dispatch;
	}
	// 824A9D88: 4802C559  bl 0x824d62e0
	ctx.lr = 0x824A9D8C;
	sub_824D62E0(ctx, base);
	// 824A9D8C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9D90: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 824A9D94: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 824A9D98: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 824A9D9C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 824A9DA0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824A9DA4: 806B0054  lwz r3, 0x54(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 824A9DA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9DAC: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 824A9DB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A9DB4: 4E800421  bctrl
	ctx.lr = 0x824A9DB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A9DB8: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 824A9DBC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A9DC0: 409A0010  bne cr6, 0x824a9dd0
	if !ctx.cr[6].eq {
	pc = 0x824A9DD0; continue 'dispatch;
	}
	// 824A9DC4: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 824A9DC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A9DCC: 419A0054  beq cr6, 0x824a9e20
	if ctx.cr[6].eq {
	pc = 0x824A9E20; continue 'dispatch;
	}
	// 824A9DD0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 824A9DD4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 824A9DD8: 480544C1  bl 0x824fe298
	ctx.lr = 0x824A9DDC;
	sub_824FE298(ctx, base);
	// 824A9DDC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9DE0: 80A10074  lwz r5, 0x74(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 824A9DE4: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 824A9DE8: 806B0058  lwz r3, 0x58(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 824A9DEC: 48054435  bl 0x824fe220
	ctx.lr = 0x824A9DF0;
	sub_824FE220(ctx, base);
	// 824A9DF0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9DF4: 816A0070  lwz r11, 0x70(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(112 as u32) ) } as u64;
	// 824A9DF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A9DFC: 38CB0008  addi r6, r11, 8
	ctx.r[6].s64 = ctx.r[11].s64 + 8;
	// 824A9E00: 409A0008  bne cr6, 0x824a9e08
	if !ctx.cr[6].eq {
	pc = 0x824A9E08; continue 'dispatch;
	}
	// 824A9E04: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 824A9E08: 806A0058  lwz r3, 0x58(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 824A9E0C: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 824A9E10: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 824A9E14: 4805434D  bl 0x824fe160
	ctx.lr = 0x824A9E18;
	sub_824FE160(ctx, base);
	// 824A9E18: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 824A9E1C: 4BFFF80D  bl 0x824a9628
	ctx.lr = 0x824A9E20;
	sub_824A9628(ctx, base);
	// 824A9E20: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9E24: 806B0054  lwz r3, 0x54(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 824A9E28: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824A9E2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A9E30: 419A0008  beq cr6, 0x824a9e38
	if ctx.cr[6].eq {
	pc = 0x824A9E38; continue 'dispatch;
	}
	// 824A9E34: 4802C49D  bl 0x824d62d0
	ctx.lr = 0x824A9E38;
	sub_824D62D0(ctx, base);
	// 824A9E38: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9E3C: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 824A9E40: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A9E44: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824A9E48: 40820020  bne 0x824a9e68
	if !ctx.cr[0].eq {
	pc = 0x824A9E68; continue 'dispatch;
	}
	// 824A9E4C: 8963008C  lbz r11, 0x8c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 824A9E50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A9E54: 409A0014  bne cr6, 0x824a9e68
	if !ctx.cr[6].eq {
	pc = 0x824A9E68; continue 'dispatch;
	}
	// 824A9E58: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 824A9E5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A9E60: 419A0008  beq cr6, 0x824a9e68
	if ctx.cr[6].eq {
	pc = 0x824A9E68; continue 'dispatch;
	}
	// 824A9E64: 4BFE935D  bl 0x824931c0
	ctx.lr = 0x824A9E68;
	sub_824931C0(ctx, base);
	// 824A9E68: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 824A9E6C: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 824A9E70: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 824A9E74: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 824A9E78: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824A9E7C: 409A0014  bne cr6, 0x824a9e90
	if !ctx.cr[6].eq {
	pc = 0x824A9E90; continue 'dispatch;
	}
	// 824A9E80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9E84: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824A9E88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A9E8C: 4E800421  bctrl
	ctx.lr = 0x824A9E90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A9E90: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 824A9E94: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824A9E98: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824A9E9C: 409A0018  bne cr6, 0x824a9eb4
	if !ctx.cr[6].eq {
	pc = 0x824A9EB4; continue 'dispatch;
	}
	// 824A9EA0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824A9EA4: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 824A9EA8: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824A9EAC: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 824A9EB0: 4BFBA209  bl 0x824640b8
	ctx.lr = 0x824A9EB4;
	sub_824640B8(ctx, base);
	// 824A9EB4: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 824A9EB8: 8081008C  lwz r4, 0x8c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 824A9EBC: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 824A9EC0: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 824A9EC4: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824A9EC8: 409A0014  bne cr6, 0x824a9edc
	if !ctx.cr[6].eq {
	pc = 0x824A9EDC; continue 'dispatch;
	}
	// 824A9ECC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9ED0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824A9ED4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A9ED8: 4E800421  bctrl
	ctx.lr = 0x824A9EDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A9EDC: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 824A9EE0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824A9EE4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824A9EE8: 409A0018  bne cr6, 0x824a9f00
	if !ctx.cr[6].eq {
	pc = 0x824A9F00; continue 'dispatch;
	}
	// 824A9EEC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824A9EF0: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 824A9EF4: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824A9EF8: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 824A9EFC: 4BFBA1BD  bl 0x824640b8
	ctx.lr = 0x824A9F00;
	sub_824640B8(ctx, base);
	// 824A9F00: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824A9F04: 4808B1F8  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A9F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A9F08 size=8
    let mut pc: u32 = 0x824A9F08;
    'dispatch: loop {
        match pc {
            0x824A9F08 => {
    //   block [0x824A9F08..0x824A9F10)
	// 824A9F08: 386300A0  addi r3, r3, 0xa0
	ctx.r[3].s64 = ctx.r[3].s64 + 160;
	// 824A9F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A9F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824A9F10 size=4
    let mut pc: u32 = 0x824A9F10;
    'dispatch: loop {
        match pc {
            0x824A9F10 => {
    //   block [0x824A9F10..0x824A9F14)
	// 824A9F10: 4BFFFBD0  b 0x824a9ae0
	sub_824A9AE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824A9F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824A9F18 size=336
    let mut pc: u32 = 0x824A9F18;
    'dispatch: loop {
        match pc {
            0x824A9F18 => {
    //   block [0x824A9F18..0x824AA068)
	// 824A9F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824A9F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824A9F20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824A9F24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824A9F28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824A9F2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824A9F30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824A9F34: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9F38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824A9F3C: 419A0050  beq cr6, 0x824a9f8c
	if ctx.cr[6].eq {
	pc = 0x824A9F8C; continue 'dispatch;
	}
	// 824A9F40: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 824A9F44: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A9F48: 419A0024  beq cr6, 0x824a9f6c
	if ctx.cr[6].eq {
	pc = 0x824A9F6C; continue 'dispatch;
	}
	// 824A9F4C: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 824A9F50: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 824A9F54: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824A9F58: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 824A9F5C: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 824A9F60: 4BFE9279  bl 0x824931d8
	ctx.lr = 0x824A9F64;
	sub_824931D8(ctx, base);
	// 824A9F64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824A9F68: 480000E8  b 0x824aa050
	pc = 0x824AA050; continue 'dispatch;
	// 824A9F6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824A9F70: 419A001C  beq cr6, 0x824a9f8c
	if ctx.cr[6].eq {
	pc = 0x824A9F8C; continue 'dispatch;
	}
	// 824A9F74: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 824A9F78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824A9F7C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824A9F80: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824A9F84: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9F88: 4BFF6789  bl 0x824a0710
	ctx.lr = 0x824A9F8C;
	sub_824A0710(ctx, base);
	// 824A9F8C: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A9F90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A9F94: 419A0010  beq cr6, 0x824a9fa4
	if ctx.cr[6].eq {
	pc = 0x824A9FA4; continue 'dispatch;
	}
	// 824A9F98: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 824A9F9C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824A9FA0: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824A9FA4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824A9FA8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824A9FAC: 419A003C  beq cr6, 0x824a9fe8
	if ctx.cr[6].eq {
	pc = 0x824A9FE8; continue 'dispatch;
	}
	// 824A9FB0: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824A9FB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824A9FB8: 419A0030  beq cr6, 0x824a9fe8
	if ctx.cr[6].eq {
	pc = 0x824A9FE8; continue 'dispatch;
	}
	// 824A9FBC: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824A9FC0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824A9FC4: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824A9FC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824A9FCC: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824A9FD0: 409A0018  bne cr6, 0x824a9fe8
	if !ctx.cr[6].eq {
	pc = 0x824A9FE8; continue 'dispatch;
	}
	// 824A9FD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9FD8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824A9FDC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824A9FE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824A9FE4: 4E800421  bctrl
	ctx.lr = 0x824A9FE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824A9FE8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824A9FEC: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 824A9FF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824A9FF4: 419A000C  beq cr6, 0x824aa000
	if ctx.cr[6].eq {
	pc = 0x824AA000; continue 'dispatch;
	}
	// 824A9FF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824A9FFC: 4BFFA61D  bl 0x824a4618
	ctx.lr = 0x824AA000;
	sub_824A4618(ctx, base);
	// 824AA000: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AA004: 4BFFF7E5  bl 0x824a97e8
	ctx.lr = 0x824AA008;
	sub_824A97E8(ctx, base);
	// 824AA008: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AA00C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824AA010: 419A003C  beq cr6, 0x824aa04c
	if ctx.cr[6].eq {
	pc = 0x824AA04C; continue 'dispatch;
	}
	// 824AA014: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824AA018: 4BFF6469  bl 0x824a0480
	ctx.lr = 0x824AA01C;
	sub_824A0480(ctx, base);
	// 824AA01C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AA020: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 824AA024: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AA028: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824AA02C: 40820020  bne 0x824aa04c
	if !ctx.cr[0].eq {
	pc = 0x824AA04C; continue 'dispatch;
	}
	// 824AA030: 8963008C  lbz r11, 0x8c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 824AA034: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AA038: 409A0014  bne cr6, 0x824aa04c
	if !ctx.cr[6].eq {
	pc = 0x824AA04C; continue 'dispatch;
	}
	// 824AA03C: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 824AA040: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AA044: 419A0008  beq cr6, 0x824aa04c
	if ctx.cr[6].eq {
	pc = 0x824AA04C; continue 'dispatch;
	}
	// 824AA048: 4BFE9179  bl 0x824931c0
	ctx.lr = 0x824AA04C;
	sub_824931C0(ctx, base);
	// 824AA04C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824AA050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824AA054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AA058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AA05C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AA060: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AA064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AA068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AA068 size=156
    let mut pc: u32 = 0x824AA068;
    'dispatch: loop {
        match pc {
            0x824AA068 => {
    //   block [0x824AA068..0x824AA104)
	// 824AA068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AA06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AA070: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AA074: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AA078: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AA108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AA108 size=124
    let mut pc: u32 = 0x824AA108;
    'dispatch: loop {
        match pc {
            0x824AA108 => {
    //   block [0x824AA108..0x824AA184)
	// 824AA108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AA10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AA110: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AA114: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AA118: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AA188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824AA188 size=56
    let mut pc: u32 = 0x824AA188;
    'dispatch: loop {
        match pc {
            0x824AA188 => {
    //   block [0x824AA188..0x824AA1C0)
	// 824AA188: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824AA18C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 824AA190: 388B00A0  addi r4, r11, 0xa0
	ctx.r[4].s64 = ctx.r[11].s64 + 160;
	// 824AA194: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AA198: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824AA19C: 816A006C  lwz r11, 0x6c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(108 as u32) ) } as u64;
	// 824AA1A0: C1AB0004  lfs f13, 4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824AA1A4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824AA1A8: C00BBFFC  lfs f0, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AA1AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA1B0: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 824AA1B4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 824AA1B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AA1BC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AA1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AA1C0 size=132
    let mut pc: u32 = 0x824AA1C0;
    'dispatch: loop {
        match pc {
            0x824AA1C0 => {
    //   block [0x824AA1C0..0x824AA244)
	// 824AA1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AA1C4: 4808AEF9  bl 0x825350bc
	ctx.lr = 0x824AA1C8;
	sub_82535080(ctx, base);
	// 824AA1C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AA1CC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824AA1D0: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 824AA1D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AA1D8: 4BFF1D21  bl 0x8249bef8
	ctx.lr = 0x824AA1DC;
	sub_8249BEF8(ctx, base);
	// 824AA1DC: 395F0010  addi r10, r31, 0x10
	ctx.r[10].s64 = ctx.r[31].s64 + 16;
	// 824AA1E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AA1E4: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 824AA1E8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824AA1EC: 7CEAF850  subf r7, r10, r31
	ctx.r[7].s64 = ctx.r[31].s64 - ctx.r[10].s64;
	// 824AA1F0: 3929E4B4  addi r9, r9, -0x1b4c
	ctx.r[9].s64 = ctx.r[9].s64 + -6988;
	// 824AA1F4: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 824AA1F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824AA1FC: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824AA200: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824AA204: 911F0088  stw r8, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[8].u32 ) };
	// 824AA208: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 824AA20C: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 824AA210: 911F0094  stw r8, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[8].u32 ) };
	// 824AA214: 98EA0010  stb r7, 0x10(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[7].u8 ) };
	// 824AA218: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824AA21C: 480FE535  bl 0x825a8750
	ctx.lr = 0x824AA220;
	sub_825A8750(ctx, base);
	// 824AA220: 3BBF00A0  addi r29, r31, 0xa0
	ctx.r[29].s64 = ctx.r[31].s64 + 160;
	// 824AA224: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824AA228: 389E0030  addi r4, r30, 0x30
	ctx.r[4].s64 = ctx.r[30].s64 + 48;
	// 824AA22C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824AA230: 48100549  bl 0x825aa778
	ctx.lr = 0x824AA234;
	sub_825AA778(ctx, base);
	// 824AA234: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AA238: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 824AA23C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824AA240: 4808AECC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AA248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AA248 size=8
    let mut pc: u32 = 0x824AA248;
    'dispatch: loop {
        match pc {
            0x824AA248 => {
    //   block [0x824AA248..0x824AA250)
	// 824AA248: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 824AA24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AA250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AA250 size=132
    let mut pc: u32 = 0x824AA250;
    'dispatch: loop {
        match pc {
            0x824AA250 => {
    //   block [0x824AA250..0x824AA2D4)
	// 824AA250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AA254: 4808AE5D  bl 0x825350b0
	ctx.lr = 0x824AA258;
	sub_82535080(ctx, base);
	// 824AA258: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AA25C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 824AA260: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824AA264: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 824AA268: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 824AA26C: 409A000C  bne cr6, 0x824aa278
	if !ctx.cr[6].eq {
	pc = 0x824AA278; continue 'dispatch;
	}
	// 824AA270: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AA274: 836B006C  lwz r27, 0x6c(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 824AA278: 817D0154  lwz r11, 0x154(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(340 as u32) ) } as u64;
	// 824AA27C: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 824AA280: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824AA284: 41980048  blt cr6, 0x824aa2cc
	if ctx.cr[6].lt {
	pc = 0x824AA2CC; continue 'dispatch;
	}
	// 824AA288: 3B9D0010  addi r28, r29, 0x10
	ctx.r[28].s64 = ctx.r[29].s64 + 16;
	// 824AA28C: 57FE1838  slwi r30, r31, 3
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 824AA290: 817D0150  lwz r11, 0x150(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(336 as u32) ) } as u64;
	// 824AA294: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 824AA298: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 824AA29C: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 824AA2A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824AA2A4: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA2A8: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AA2AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA2B0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824AA2B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AA2B8: 4E800421  bctrl
	ctx.lr = 0x824AA2BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AA2BC: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 824AA2C0: 3BDEFFF8  addi r30, r30, -8
	ctx.r[30].s64 = ctx.r[30].s64 + -8;
	// 824AA2C4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824AA2C8: 4098FFC8  bge cr6, 0x824aa290
	if !ctx.cr[6].lt {
	pc = 0x824AA290; continue 'dispatch;
	}
	// 824AA2CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824AA2D0: 4808AE30  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AA2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AA2D8 size=144
    let mut pc: u32 = 0x824AA2D8;
    'dispatch: loop {
        match pc {
            0x824AA2D8 => {
    //   block [0x824AA2D8..0x824AA368)
	// 824AA2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AA2DC: 4808ADD5  bl 0x825350b0
	ctx.lr = 0x824AA2E0;
	sub_82535080(ctx, base);
	// 824AA2E0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AA2E4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 824AA2E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824AA2EC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824AA2F0: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 824AA2F4: 409A000C  bne cr6, 0x824aa300
	if !ctx.cr[6].eq {
	pc = 0x824AA300; continue 'dispatch;
	}
	// 824AA2F8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AA2FC: 836B006C  lwz r27, 0x6c(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 824AA300: 817E0154  lwz r11, 0x154(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(340 as u32) ) } as u64;
	// 824AA304: 3B4BFFFF  addi r26, r11, -1
	ctx.r[26].s64 = ctx.r[11].s64 + -1;
	// 824AA308: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 824AA30C: 41980054  blt cr6, 0x824aa360
	if ctx.cr[6].lt {
	pc = 0x824AA360; continue 'dispatch;
	}
	// 824AA310: 3B9E0010  addi r28, r30, 0x10
	ctx.r[28].s64 = ctx.r[30].s64 + 16;
	// 824AA314: 575F1838  slwi r31, r26, 3
	ctx.r[31].u32 = ctx.r[26].u32.wrapping_shl(3);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 824AA318: 817E0150  lwz r11, 0x150(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(336 as u32) ) } as u64;
	// 824AA31C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 824AA320: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 824AA324: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 824AA328: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824AA32C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA330: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AA334: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA338: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AA33C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AA340: 4E800421  bctrl
	ctx.lr = 0x824AA344;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AA344: 897D0004  lbz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AA348: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AA34C: 409A0014  bne cr6, 0x824aa360
	if !ctx.cr[6].eq {
	pc = 0x824AA360; continue 'dispatch;
	}
	// 824AA350: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 824AA354: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 824AA358: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 824AA35C: 4098FFBC  bge cr6, 0x824aa318
	if !ctx.cr[6].lt {
	pc = 0x824AA318; continue 'dispatch;
	}
	// 824AA360: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824AA364: 4808AD9C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AA368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AA368 size=64
    let mut pc: u32 = 0x824AA368;
    'dispatch: loop {
        match pc {
            0x824AA368 => {
    //   block [0x824AA368..0x824AA3A8)
	// 824AA368: 81240154  lwz r9, 0x154(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(340 as u32) ) } as u64;
	// 824AA36C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824AA370: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824AA374: 40990028  ble cr6, 0x824aa39c
	if !ctx.cr[6].gt {
	pc = 0x824AA39C; continue 'dispatch;
	}
	// 824AA378: 81640150  lwz r11, 0x150(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(336 as u32) ) } as u64;
	// 824AA37C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824AA380: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA384: 7F082840  cmplw cr6, r8, r5
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[5].u32, &mut ctx.xer);
	// 824AA388: 419A0020  beq cr6, 0x824aa3a8
	if ctx.cr[6].eq {
		sub_824AA3A8(ctx, base);
		return;
	}
	// 824AA38C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824AA390: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824AA394: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824AA398: 4198FFE8  blt cr6, 0x824aa380
	if ctx.cr[6].lt {
	pc = 0x824AA380; continue 'dispatch;
	}
	// 824AA39C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AA3A0: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824AA3A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AA3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AA3A8 size=12
    let mut pc: u32 = 0x824AA3A8;
    'dispatch: loop {
        match pc {
            0x824AA3A8 => {
    //   block [0x824AA3A8..0x824AA3B4)
	// 824AA3A8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824AA3AC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824AA3B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AA3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AA3B8 size=272
    let mut pc: u32 = 0x824AA3B8;
    'dispatch: loop {
        match pc {
            0x824AA3B8 => {
    //   block [0x824AA3B8..0x824AA4C8)
	// 824AA3B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AA3BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AA3C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AA3C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AA3C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AA3CC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AA3D0: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA3D4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824AA3D8: 396B0894  addi r11, r11, 0x894
	ctx.r[11].s64 = ctx.r[11].s64 + 2196;
	// 824AA3DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AA3E0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AA3E4: B1210066  sth r9, 0x66(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(102 as u32), ctx.r[9].u16 ) };
	// 824AA3E8: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824AA3EC: 419A0058  beq cr6, 0x824aa444
	if ctx.cr[6].eq {
	pc = 0x824AA444; continue 'dispatch;
	}
	// 824AA3F0: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 824AA3F4: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 824AA3F8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824AA3FC: 41980030  blt cr6, 0x824aa42c
	if ctx.cr[6].lt {
	pc = 0x824AA42C; continue 'dispatch;
	}
	// 824AA400: 815F0150  lwz r10, 0x150(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 824AA404: 57CB1838  slwi r11, r30, 3
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AA408: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824AA40C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824AA410: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA414: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824AA418: 419A0044  beq cr6, 0x824aa45c
	if ctx.cr[6].eq {
	pc = 0x824AA45C; continue 'dispatch;
	}
	// 824AA41C: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 824AA420: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 824AA424: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824AA428: 4098FFE8  bge cr6, 0x824aa410
	if !ctx.cr[6].lt {
	pc = 0x824AA410; continue 'dispatch;
	}
	// 824AA42C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824AA430: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824AA434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AA438: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 824AA43C: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA440: 48000901  bl 0x824aad40
	ctx.lr = 0x824AA444;
	sub_824AAD40(ctx, base);
	// 824AA444: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824AA448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AA44C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AA450: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AA454: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AA458: 4E800020  blr
	return;
	// 824AA45C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824AA460: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 824AA464: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AA468: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA46C: 480008D5  bl 0x824aad40
	ctx.lr = 0x824AA470;
	sub_824AAD40(ctx, base);
	// 824AA470: 817F0150  lwz r11, 0x150(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 824AA474: 57DE1838  slwi r30, r30, 3
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 824AA478: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AA47C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824AA480: 419A0018  beq cr6, 0x824aa498
	if ctx.cr[6].eq {
	pc = 0x824AA498; continue 'dispatch;
	}
	// 824AA484: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA488: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 824AA48C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 824AA490: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AA494: 4E800421  bctrl
	ctx.lr = 0x824AA498;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AA498: 815F0154  lwz r10, 0x154(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 824AA49C: 817F0150  lwz r11, 0x150(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 824AA4A0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824AA4A4: 7D0BF214  add r8, r11, r30
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824AA4A8: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824AA4AC: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824AA4B0: 915F0154  stw r10, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[10].u32 ) };
	// 824AA4B4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA4B8: 91480000  stw r10, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824AA4BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AA4C0: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824AA4C4: 4BFFFF80  b 0x824aa444
	pc = 0x824AA444; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AA4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AA4C8 size=172
    let mut pc: u32 = 0x824AA4C8;
    'dispatch: loop {
        match pc {
            0x824AA4C8 => {
    //   block [0x824AA4C8..0x824AA574)
	// 824AA4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AA4CC: 4808ABED  bl 0x825350b8
	ctx.lr = 0x824AA4D0;
	sub_82535080(ctx, base);
	// 824AA4D0: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AA4D4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824AA4D8: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 824AA4DC: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 824AA4E0: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AA4E4: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 824AA4E8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 824AA4EC: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 824AA4F0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824AA4F4: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 824AA4F8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824AA4FC: 4200FFF0  bdnz 0x824aa4ec
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824AA4EC; continue 'dispatch;
	}
	// 824AA500: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AA504: 815D0154  lwz r10, 0x154(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(340 as u32) ) } as u64;
	// 824AA508: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824AA50C: 396B0894  addi r11, r11, 0x894
	ctx.r[11].s64 = ctx.r[11].s64 + 2196;
	// 824AA510: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824AA514: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AA518: B1210056  sth r9, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[9].u16 ) };
	// 824AA51C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 824AA520: 4099004C  ble cr6, 0x824aa56c
	if !ctx.cr[6].gt {
	pc = 0x824AA56C; continue 'dispatch;
	}
	// 824AA524: 3B9D0010  addi r28, r29, 0x10
	ctx.r[28].s64 = ctx.r[29].s64 + 16;
	// 824AA528: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824AA52C: 817D0150  lwz r11, 0x150(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(336 as u32) ) } as u64;
	// 824AA530: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 824AA534: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 824AA538: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 824AA53C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824AA540: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA544: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AA548: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA54C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824AA550: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AA554: 4E800421  bctrl
	ctx.lr = 0x824AA558;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AA558: 817D0154  lwz r11, 0x154(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(340 as u32) ) } as u64;
	// 824AA55C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 824AA560: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 824AA564: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824AA568: 4198FFC4  blt cr6, 0x824aa52c
	if ctx.cr[6].lt {
	pc = 0x824AA52C; continue 'dispatch;
	}
	// 824AA56C: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 824AA570: 4808AB98  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AA578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AA578 size=96
    let mut pc: u32 = 0x824AA578;
    'dispatch: loop {
        match pc {
            0x824AA578 => {
    //   block [0x824AA578..0x824AA5D8)
	// 824AA578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AA57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AA580: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AA584: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AA588: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AA58C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AA590: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 824AA594: 4BFFFC2D  bl 0x824aa1c0
	ctx.lr = 0x824AA598;
	sub_824AA1C0(ctx, base);
	// 824AA598: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AA59C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824AA5A0: 396BE4FC  addi r11, r11, -0x1b04
	ctx.r[11].s64 = ctx.r[11].s64 + -6916;
	// 824AA5A4: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 824AA5A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AA5AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AA5B0: 915F0150  stw r10, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[10].u32 ) };
	// 824AA5B4: 915F0154  stw r10, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[10].u32 ) };
	// 824AA5B8: 913F0158  stw r9, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[9].u32 ) };
	// 824AA5BC: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 824AA5C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AA5C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AA5C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AA5CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AA5D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AA5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AA5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AA5D8 size=192
    let mut pc: u32 = 0x824AA5D8;
    'dispatch: loop {
        match pc {
            0x824AA5D8 => {
    //   block [0x824AA5D8..0x824AA698)
	// 824AA5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AA5DC: 4808AAE1  bl 0x825350bc
	ctx.lr = 0x824AA5E0;
	sub_82535080(ctx, base);
	// 824AA5E0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AA5E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824AA5E8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824AA5EC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824AA5F0: 394AE4FC  addi r10, r10, -0x1b04
	ctx.r[10].s64 = ctx.r[10].s64 + -6916;
	// 824AA5F4: 39290894  addi r9, r9, 0x894
	ctx.r[9].s64 = ctx.r[9].s64 + 2196;
	// 824AA5F8: 817E0154  lwz r11, 0x154(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(340 as u32) ) } as u64;
	// 824AA5FC: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 824AA600: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824AA604: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824AA608: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 824AA60C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824AA610: B1610056  sth r11, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[11].u16 ) };
	// 824AA614: 41980034  blt cr6, 0x824aa648
	if ctx.cr[6].lt {
	pc = 0x824AA648; continue 'dispatch;
	}
	// 824AA618: 57FD1838  slwi r29, r31, 3
	ctx.r[29].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 824AA61C: 817E0150  lwz r11, 0x150(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(336 as u32) ) } as u64;
	// 824AA620: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824AA624: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824AA628: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA62C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 824AA630: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AA634: 4E800421  bctrl
	ctx.lr = 0x824AA638;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AA638: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 824AA63C: 3BBDFFF8  addi r29, r29, -8
	ctx.r[29].s64 = ctx.r[29].s64 + -8;
	// 824AA640: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824AA644: 4098FFD8  bge cr6, 0x824aa61c
	if !ctx.cr[6].lt {
	pc = 0x824AA61C; continue 'dispatch;
	}
	// 824AA648: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824AA64C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824AA650: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824AA654: 915E0154  stw r10, 0x154(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(340 as u32), ctx.r[10].u32 ) };
	// 824AA658: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 824AA65C: 817E0158  lwz r11, 0x158(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(344 as u32) ) } as u64;
	// 824AA660: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AA664: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AA668: 409A0020  bne cr6, 0x824aa688
	if !ctx.cr[6].eq {
	pc = 0x824AA688; continue 'dispatch;
	}
	// 824AA66C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA670: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824AA674: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AA678: 809E0150  lwz r4, 0x150(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(336 as u32) ) } as u64;
	// 824AA67C: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AA680: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824AA684: 4BFB9A35  bl 0x824640b8
	ctx.lr = 0x824AA688;
	sub_824640B8(ctx, base);
	// 824AA688: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AA68C: 4BFFF515  bl 0x824a9ba0
	ctx.lr = 0x824AA690;
	sub_824A9BA0(ctx, base);
	// 824AA690: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824AA694: 4808AA78  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AA698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AA698 size=440
    let mut pc: u32 = 0x824AA698;
    'dispatch: loop {
        match pc {
            0x824AA698 => {
    //   block [0x824AA698..0x824AA850)
	// 824AA698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AA69C: 4808AA11  bl 0x825350ac
	ctx.lr = 0x824AA6A0;
	sub_82535080(ctx, base);
	// 824AA6A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AA6A4: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA6A8: 3B400010  li r26, 0x10
	ctx.r[26].s64 = 16;
	// 824AA6AC: 38A00031  li r5, 0x31
	ctx.r[5].s64 = 49;
	// 824AA6B0: 38800160  li r4, 0x160
	ctx.r[4].s64 = 352;
	// 824AA6B4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824AA6B8: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824AA6BC: 4BFB997D  bl 0x82464038
	ctx.lr = 0x824AA6C0;
	sub_82464038(ctx, base);
	// 824AA6C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AA6C4: 39600160  li r11, 0x160
	ctx.r[11].s64 = 352;
	// 824AA6C8: 38BB00A0  addi r5, r27, 0xa0
	ctx.r[5].s64 = ctx.r[27].s64 + 160;
	// 824AA6CC: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 824AA6D0: 839B002C  lwz r28, 0x2c(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(44 as u32) ) } as u64;
	// 824AA6D4: 809B0010  lwz r4, 0x10(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 824AA6D8: 4BFFFAE9  bl 0x824aa1c0
	ctx.lr = 0x824AA6DC;
	sub_824AA1C0(ctx, base);
	// 824AA6DC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AA6E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824AA6E4: 396BE4FC  addi r11, r11, -0x1b04
	ctx.r[11].s64 = ctx.r[11].s64 + -6916;
	// 824AA6E8: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 824AA6EC: 3BDF0080  addi r30, r31, 0x80
	ctx.r[30].s64 = ctx.r[31].s64 + 128;
	// 824AA6F0: 3BBB0080  addi r29, r27, 0x80
	ctx.r[29].s64 = ctx.r[27].s64 + 128;
	// 824AA6F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AA6F8: 915F0150  stw r10, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[10].u32 ) };
	// 824AA6FC: 915F0154  stw r10, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[10].u32 ) };
	// 824AA700: 913F0158  stw r9, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[9].u32 ) };
	// 824AA704: 939F002C  stw r28, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 824AA708: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AA70C: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AA710: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 824AA714: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824AA718: 40980050  bge cr6, 0x824aa768
	if !ctx.cr[6].lt {
	pc = 0x824AA768; continue 'dispatch;
	}
	// 824AA71C: 554A0000  rlwinm r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 824AA720: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AA724: 409A0018  bne cr6, 0x824aa73c
	if !ctx.cr[6].eq {
	pc = 0x824AA73C; continue 'dispatch;
	}
	// 824AA728: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AA72C: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA730: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AA734: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824AA738: 4BFB9981  bl 0x824640b8
	ctx.lr = 0x824AA73C;
	sub_824640B8(ctx, base);
	// 824AA73C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AA740: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 824AA744: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824AA748: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824AA74C: 4BFB98ED  bl 0x82464038
	ctx.lr = 0x824AA750;
	sub_82464038(ctx, base);
	// 824AA750: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AA754: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824AA758: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AA75C: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AA760: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 824AA764: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824AA768: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AA76C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA770: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AA774: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824AA778: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA77C: 40990020  ble cr6, 0x824aa79c
	if !ctx.cr[6].gt {
	pc = 0x824AA79C; continue 'dispatch;
	}
	// 824AA780: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 824AA784: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AA788: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824AA78C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824AA790: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824AA794: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824AA798: 409AFFEC  bne cr6, 0x824aa784
	if !ctx.cr[6].eq {
	pc = 0x824AA784; continue 'dispatch;
	}
	// 824AA79C: 3BDF008C  addi r30, r31, 0x8c
	ctx.r[30].s64 = ctx.r[31].s64 + 140;
	// 824AA7A0: 3BBB008C  addi r29, r27, 0x8c
	ctx.r[29].s64 = ctx.r[27].s64 + 140;
	// 824AA7A4: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AA7A8: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AA7AC: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 824AA7B0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824AA7B4: 40980050  bge cr6, 0x824aa804
	if !ctx.cr[6].lt {
	pc = 0x824AA804; continue 'dispatch;
	}
	// 824AA7B8: 554A0000  rlwinm r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 824AA7BC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AA7C0: 409A0018  bne cr6, 0x824aa7d8
	if !ctx.cr[6].eq {
	pc = 0x824AA7D8; continue 'dispatch;
	}
	// 824AA7C4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AA7C8: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA7CC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AA7D0: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824AA7D4: 4BFB98E5  bl 0x824640b8
	ctx.lr = 0x824AA7D8;
	sub_824640B8(ctx, base);
	// 824AA7D8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AA7DC: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 824AA7E0: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824AA7E4: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824AA7E8: 4BFB9851  bl 0x82464038
	ctx.lr = 0x824AA7EC;
	sub_82464038(ctx, base);
	// 824AA7EC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AA7F0: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824AA7F4: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AA7F8: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AA7FC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 824AA800: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824AA804: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AA808: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA80C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AA810: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824AA814: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA818: 40990020  ble cr6, 0x824aa838
	if !ctx.cr[6].gt {
	pc = 0x824AA838; continue 'dispatch;
	}
	// 824AA81C: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 824AA820: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AA824: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824AA828: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824AA82C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824AA830: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824AA834: 409AFFEC  bne cr6, 0x824aa820
	if !ctx.cr[6].eq {
	pc = 0x824AA820; continue 'dispatch;
	}
	// 824AA838: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824AA83C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AA840: 4BFE67B1  bl 0x82490ff0
	ctx.lr = 0x824AA844;
	sub_82490FF0(ctx, base);
	// 824AA844: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AA848: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824AA84C: 4808A8B0  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AA850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AA850 size=256
    let mut pc: u32 = 0x824AA850;
    'dispatch: loop {
        match pc {
            0x824AA850 => {
    //   block [0x824AA850..0x824AA950)
	// 824AA850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AA854: 4808A865  bl 0x825350b8
	ctx.lr = 0x824AA858;
	sub_82535080(ctx, base);
	// 824AA858: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AA85C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824AA860: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824AA864: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA868: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AA86C: 419A00DC  beq cr6, 0x824aa948
	if ctx.cr[6].eq {
	pc = 0x824AA948; continue 'dispatch;
	}
	// 824AA870: 48000459  bl 0x824aacc8
	ctx.lr = 0x824AA874;
	sub_824AACC8(ctx, base);
	// 824AA874: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824AA878: 409A00D0  bne cr6, 0x824aa948
	if !ctx.cr[6].eq {
	pc = 0x824AA948; continue 'dispatch;
	}
	// 824AA87C: 3BFC0150  addi r31, r28, 0x150
	ctx.r[31].s64 = ctx.r[28].s64 + 336;
	// 824AA880: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AA884: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AA888: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824AA88C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824AA890: 409A0010  bne cr6, 0x824aa8a0
	if !ctx.cr[6].eq {
	pc = 0x824AA8A0; continue 'dispatch;
	}
	// 824AA894: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 824AA898: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AA89C: 4BFC3AB5  bl 0x8246e350
	ctx.lr = 0x824AA8A0;
	sub_8246E350(ctx, base);
	// 824AA8A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AA8A4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 824AA8A8: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA8AC: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 824AA8B0: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 824AA8B4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AA8B8: 7FCB4214  add r30, r11, r8
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 824AA8BC: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 824AA8C0: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AA8C4: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 824AA8C8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 824AA8CC: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 824AA8D0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824AA8D4: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 824AA8D8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824AA8DC: 4200FFF0  bdnz 0x824aa8cc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824AA8CC; continue 'dispatch;
	}
	// 824AA8E0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824AA8E4: 387C0010  addi r3, r28, 0x10
	ctx.r[3].s64 = ctx.r[28].s64 + 16;
	// 824AA8E8: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA8EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824AA8F0: 394B1C60  addi r10, r11, 0x1c60
	ctx.r[10].s64 = ctx.r[11].s64 + 7264;
	// 824AA8F4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824AA8F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824AA8FC: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA900: 914100B0  stw r10, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 824AA904: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824AA908: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 824AA90C: 394B01A0  addi r10, r11, 0x1a0
	ctx.r[10].s64 = ctx.r[11].s64 + 416;
	// 824AA910: 8108000C  lwz r8, 0xc(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AA914: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AA918: 55082834  slwi r8, r8, 5
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(5);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824AA91C: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 824AA920: 7D4A48AE  lbzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 824AA924: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 824AA928: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824AA92C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824AA930: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824AA934: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 824AA938: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AA93C: 4E800421  bctrl
	ctx.lr = 0x824AA940;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AA940: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824AA944: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 824AA948: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 824AA94C: 4808A7BC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AA950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AA950 size=244
    let mut pc: u32 = 0x824AA950;
    'dispatch: loop {
        match pc {
            0x824AA950 => {
    //   block [0x824AA950..0x824AAA44)
	// 824AA950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AA954: 4808A761  bl 0x825350b4
	ctx.lr = 0x824AA958;
	sub_82535080(ctx, base);
	// 824AA958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AA95C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824AA960: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AA964: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AA968: 388B08C4  addi r4, r11, 0x8c4
	ctx.r[4].s64 = ctx.r[11].s64 + 2244;
	// 824AA96C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 824AA970: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA974: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 824AA978: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AA97C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AA980: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AA984: 4E800421  bctrl
	ctx.lr = 0x824AA988;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AA988: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824AA98C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AA990: 4BFFF099  bl 0x824a9a28
	ctx.lr = 0x824AA994;
	sub_824A9A28(ctx, base);
	// 824AA994: 817F0158  lwz r11, 0x158(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(344 as u32) ) } as u64;
	// 824AA998: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AA99C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AA9A0: 409A0034  bne cr6, 0x824aa9d4
	if !ctx.cr[6].eq {
	pc = 0x824AA9D4; continue 'dispatch;
	}
	// 824AA9A4: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA9A8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824AA9AC: 80FF0154  lwz r7, 0x154(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 824AA9B0: 55681838  slwi r8, r11, 3
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824AA9B4: 80DF0150  lwz r6, 0x150(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 824AA9B8: 388A08B8  addi r4, r10, 0x8b8
	ctx.r[4].s64 = ctx.r[10].s64 + 2232;
	// 824AA9BC: 54E71838  slwi r7, r7, 3
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824AA9C0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824AA9C4: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AA9C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AA9CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AA9D0: 4E800421  bctrl
	ctx.lr = 0x824AA9D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AA9D4: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 824AA9D8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824AA9DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AA9E0: 40990048  ble cr6, 0x824aaa28
	if !ctx.cr[6].gt {
	pc = 0x824AAA28; continue 'dispatch;
	}
	// 824AA9E4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AA9E8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824AA9EC: 3B6B08B0  addi r27, r11, 0x8b0
	ctx.r[27].s64 = ctx.r[11].s64 + 2224;
	// 824AA9F0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AA9F4: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824AA9F8: 815F0150  lwz r10, 0x150(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 824AA9FC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824AAA00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AAA04: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AAA08: 7CCAE82E  lwzx r6, r10, r29
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824AAA0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AAA10: 4E800421  bctrl
	ctx.lr = 0x824AAA14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AAA14: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 824AAA18: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 824AAA1C: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 824AAA20: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824AAA24: 4198FFCC  blt cr6, 0x824aa9f0
	if ctx.cr[6].lt {
	pc = 0x824AA9F0; continue 'dispatch;
	}
	// 824AAA28: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AAA2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AAA30: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824AAA34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AAA38: 4E800421  bctrl
	ctx.lr = 0x824AAA3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AAA3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824AAA40: 4808A6C4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AAA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AAA48 size=144
    let mut pc: u32 = 0x824AAA48;
    'dispatch: loop {
        match pc {
            0x824AAA48 => {
    //   block [0x824AAA48..0x824AAAD8)
	// 824AAA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AAA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AAA50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AAA54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AAA58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AAA5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824AAA60: 817E0154  lwz r11, 0x154(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(340 as u32) ) } as u64;
	// 824AAA64: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AAA68: 409A0050  bne cr6, 0x824aaab8
	if !ctx.cr[6].eq {
	pc = 0x824AAAB8; continue 'dispatch;
	}
	// 824AAA6C: 3BFE0150  addi r31, r30, 0x150
	ctx.r[31].s64 = ctx.r[30].s64 + 336;
	// 824AAA70: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AAA74: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AAA78: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AAA7C: 409A0020  bne cr6, 0x824aaa9c
	if !ctx.cr[6].eq {
	pc = 0x824AAA9C; continue 'dispatch;
	}
	// 824AAA80: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AAA84: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824AAA88: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AAA8C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AAA90: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AAA94: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824AAA98: 4BFB9621  bl 0x824640b8
	ctx.lr = 0x824AAA9C;
	sub_824640B8(ctx, base);
	// 824AAA9C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AAAA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824AAAA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AAAA8: 512AF880  rlwimi r10, r9, 0x1f, 2, 0
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(31) as u64) & 0xFFFFFFFFBFFFFFFF) | (ctx.r[10].u64 & 0x0000000040000000);
	// 824AAAAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AAAB0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824AAAB4: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824AAAB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AAABC: 4BFFF455  bl 0x824a9f10
	ctx.lr = 0x824AAAC0;
	sub_824A9F10(ctx, base);
	// 824AAAC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AAAC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AAAC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AAACC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AAAD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AAAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AAAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AAAD8 size=492
    let mut pc: u32 = 0x824AAAD8;
    'dispatch: loop {
        match pc {
            0x824AAAD8 => {
    //   block [0x824AAAD8..0x824AACC4)
	// 824AAAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AAADC: 4808A5D1  bl 0x825350ac
	ctx.lr = 0x824AAAE0;
	sub_82535080(ctx, base);
	// 824AAAE0: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AACC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AACC8 size=120
    let mut pc: u32 = 0x824AACC8;
    'dispatch: loop {
        match pc {
            0x824AACC8 => {
    //   block [0x824AACC8..0x824AAD40)
	// 824AACC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AACCC: 4808A3F1  bl 0x825350bc
	ctx.lr = 0x824AACD0;
	sub_82535080(ctx, base);
	// 824AACD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AACD4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824AACD8: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 824AACDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824AACE0: 817D0084  lwz r11, 0x84(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 824AACE4: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 824AACE8: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 824AACEC: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 824AACF0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824AACF4: 41980044  blt cr6, 0x824aad38
	if ctx.cr[6].lt {
	pc = 0x824AAD38; continue 'dispatch;
	}
	// 824AACF8: 57DF103A  slwi r31, r30, 2
	ctx.r[31].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 824AACFC: 817D0080  lwz r11, 0x80(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(128 as u32) ) } as u64;
	// 824AAD00: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824AAD04: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824AAD08: 419A001C  beq cr6, 0x824aad24
	if ctx.cr[6].eq {
	pc = 0x824AAD24; continue 'dispatch;
	}
	// 824AAD0C: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824AAD10: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824AAD14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AAD18: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AAD1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AAD20: 4E800421  bctrl
	ctx.lr = 0x824AAD24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AAD24: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 824AAD28: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 824AAD2C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824AAD30: 4098FFCC  bge cr6, 0x824aacfc
	if !ctx.cr[6].lt {
	pc = 0x824AACFC; continue 'dispatch;
	}
	// 824AAD34: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824AAD38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824AAD3C: 4808A3D0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AAD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AAD40 size=112
    let mut pc: u32 = 0x824AAD40;
    'dispatch: loop {
        match pc {
            0x824AAD40 => {
    //   block [0x824AAD40..0x824AADB0)
	// 824AAD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AAD44: 4808A379  bl 0x825350bc
	ctx.lr = 0x824AAD48;
	sub_82535080(ctx, base);
	// 824AAD48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AAD4C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824AAD50: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 824AAD54: 98A10058  stb r5, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u8 ) };
	// 824AAD58: 817D0084  lwz r11, 0x84(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 824AAD5C: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 824AAD60: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 824AAD64: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824AAD68: 41980040  blt cr6, 0x824aada8
	if ctx.cr[6].lt {
	pc = 0x824AADA8; continue 'dispatch;
	}
	// 824AAD6C: 57DF103A  slwi r31, r30, 2
	ctx.r[31].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 824AAD70: 817D0080  lwz r11, 0x80(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(128 as u32) ) } as u64;
	// 824AAD74: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824AAD78: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824AAD7C: 419A001C  beq cr6, 0x824aad98
	if ctx.cr[6].eq {
	pc = 0x824AAD98; continue 'dispatch;
	}
	// 824AAD80: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824AAD84: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824AAD88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AAD8C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AAD90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AAD94: 4E800421  bctrl
	ctx.lr = 0x824AAD98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AAD98: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 824AAD9C: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 824AADA0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824AADA4: 4098FFCC  bge cr6, 0x824aad70
	if !ctx.cr[6].lt {
	pc = 0x824AAD70; continue 'dispatch;
	}
	// 824AADA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824AADAC: 4808A360  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AADB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824AADB0 size=100
    let mut pc: u32 = 0x824AADB0;
    'dispatch: loop {
        match pc {
            0x824AADB0 => {
    //   block [0x824AADB0..0x824AAE14)
	// 824AADB0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824AADB4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AADB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824AADBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 824AADC0: 396BE914  addi r11, r11, -0x16ec
	ctx.r[11].s64 = ctx.r[11].s64 + -5868;
	// 824AADC4: C1AA0858  lfs f13, 0x858(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2136 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824AADC8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824AADCC: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 824AADD0: 99030008  stb r8, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u8 ) };
	// 824AADD4: D1A3000C  stfs f13, 0xc(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824AADD8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AADDC: C18A24B0  lfs f12, 0x24b0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(9392 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824AADE0: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824AADE4: D1830010  stfs f12, 0x10(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824AADE8: 99230008  stb r9, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u8 ) };
	// 824AADEC: C16A207C  lfs f11, 0x207c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8316 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824AADF0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 824AADF4: D1630014  stfs f11, 0x14(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824AADF8: C00A1850  lfs f0, 0x1850(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AADFC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824AAE00: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824AAE04: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824AAE08: C14A2068  lfs f10, 0x2068(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8296 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 824AAE0C: D143001C  stfs f10, 0x1c(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 824AAE10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AAE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824AAE18 size=172
    let mut pc: u32 = 0x824AAE18;
    'dispatch: loop {
        match pc {
            0x824AAE18 => {
    //   block [0x824AAE18..0x824AAEC4)
	// 824AAE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AAE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AAE20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AAE24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AAE28: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AAE2C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824AAE30: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 824AAE34: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 824AAE38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AAE3C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AAE40: 4BFB91F9  bl 0x82464038
	ctx.lr = 0x824AAE44;
	sub_82464038(ctx, base);
	// 824AAE44: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AAE48: 39000024  li r8, 0x24
	ctx.r[8].s64 = 36;
	// 824AAE4C: 396BE2F8  addi r11, r11, -0x1d08
	ctx.r[11].s64 = ctx.r[11].s64 + -7432;
	// 824AAE50: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 824AAE54: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824AAE58: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824AAE5C: B1030004  sth r8, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 824AAE60: 394AE30C  addi r10, r10, -0x1cf4
	ctx.r[10].s64 = ctx.r[10].s64 + -7412;
	// 824AAE64: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AAE68: 3929E914  addi r9, r9, -0x16ec
	ctx.r[9].s64 = ctx.r[9].s64 + -5868;
	// 824AAE6C: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 824AAE70: 897F0008  lbz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AAE74: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824AAE78: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 824AAE7C: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AAE80: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824AAE84: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AAE88: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824AAE8C: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824AAE90: C01F0014  lfs f0, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AAE94: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824AAE98: C01F0018  lfs f0, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AAE9C: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824AAEA0: C01F001C  lfs f0, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AAEA4: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 824AAEA8: C01F0020  lfs f0, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AAEAC: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824AAEB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824AAEB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AAEB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AAEBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AAEC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AAEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AAEC8 size=32
    let mut pc: u32 = 0x824AAEC8;
    'dispatch: loop {
        match pc {
            0x824AAEC8 => {
    //   block [0x824AAEC8..0x824AAEE8)
	// 824AAEC8: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 824AAECC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AAED0: 419A0018  beq cr6, 0x824aaee8
	if ctx.cr[6].eq {
		sub_824AAEE8(ctx, base);
		return;
	}
	// 824AAED4: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 824AAED8: 39400028  li r10, 0x28
	ctx.r[10].s64 = 40;
	// 824AAEDC: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824AAEE0: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824AAEE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AAEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AAEE8 size=16
    let mut pc: u32 = 0x824AAEE8;
    'dispatch: loop {
        match pc {
            0x824AAEE8 => {
    //   block [0x824AAEE8..0x824AAEF8)
	// 824AAEE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AAEEC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AAEF0: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824AAEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AAEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AAEF8 size=16
    let mut pc: u32 = 0x824AAEF8;
    'dispatch: loop {
        match pc {
            0x824AAEF8 => {
    //   block [0x824AAEF8..0x824AAF08)
	// 824AAEF8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 824AAEFC: 38800096  li r4, 0x96
	ctx.r[4].s64 = 150;
	// 824AAF00: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 824AAF04: 4BFF7AAC  b 0x824a29b0
	sub_824A29B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AAF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824AAF08 size=140
    let mut pc: u32 = 0x824AAF08;
    'dispatch: loop {
        match pc {
            0x824AAF08 => {
    //   block [0x824AAF08..0x824AAF94)
	// 824AAF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AAF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AAF10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AAF14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AAF18: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 824AAF1C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AAF20: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824AAF24: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824AAF28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824AAF2C: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 824AAF30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824AAF34: C3EB2238  lfs f31, 0x2238(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8760 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824AAF38: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824AAF3C: 48100825  bl 0x825ab760
	ctx.lr = 0x824AAF40;
	sub_825AB760(ctx, base);
	// 824AAF40: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AAF44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AAF48: 419A0024  beq cr6, 0x824aaf6c
	if ctx.cr[6].eq {
	pc = 0x824AAF6C; continue 'dispatch;
	}
	// 824AAF4C: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 824AAF50: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824AAF54: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 824AAF58: 48100809  bl 0x825ab760
	ctx.lr = 0x824AAF5C;
	sub_825AB760(ctx, base);
	// 824AAF5C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AAF60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AAF64: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824AAF68: 409A0008  bne cr6, 0x824aaf70
	if !ctx.cr[6].eq {
	pc = 0x824AAF70; continue 'dispatch;
	}
	// 824AAF6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AAF70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AAF74: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824AAF78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824AAF7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AAF80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AAF84: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 824AAF88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AAF8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AAF90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AAF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AAF98 size=8
    let mut pc: u32 = 0x824AAF98;
    'dispatch: loop {
        match pc {
            0x824AAF98 => {
    //   block [0x824AAF98..0x824AAFA0)
	// 824AAF98: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824AAF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AAFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AAFA0 size=628
    let mut pc: u32 = 0x824AAFA0;
    'dispatch: loop {
        match pc {
            0x824AAFA0 => {
    //   block [0x824AAFA0..0x824AB214)
	// 824AAFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AAFA4: 4808A10D  bl 0x825350b0
	ctx.lr = 0x824AAFA8;
	sub_82535080(ctx, base);
	// 824AAFA8: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AB218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AB218 size=568
    let mut pc: u32 = 0x824AB218;
    'dispatch: loop {
        match pc {
            0x824AB218 => {
    //   block [0x824AB218..0x824AB450)
	// 824AB218: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 824AB21C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 824AB220: 39400050  li r10, 0x50
	ctx.r[10].s64 = 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AB450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AB450 size=196
    let mut pc: u32 = 0x824AB450;
    'dispatch: loop {
        match pc {
            0x824AB450 => {
    //   block [0x824AB450..0x824AB514)
	// 824AB450: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 824AB454: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
	// 824AB458: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 824AB45C: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 824AB460: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 824AB464: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AB518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824AB518 size=20
    let mut pc: u32 = 0x824AB518;
    'dispatch: loop {
        match pc {
            0x824AB518 => {
    //   block [0x824AB518..0x824AB52C)
	// 824AB518: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824AB51C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 824AB520: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AB524: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 824AB528: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AB52C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AB52C size=8
    let mut pc: u32 = 0x824AB52C;
    'dispatch: loop {
        match pc {
            0x824AB52C => {
    //   block [0x824AB52C..0x824AB534)
	// 824AB52C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824AB530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AB538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AB538 size=8
    let mut pc: u32 = 0x824AB538;
    'dispatch: loop {
        match pc {
            0x824AB538 => {
    //   block [0x824AB538..0x824AB540)
	// 824AB538: 38600066  li r3, 0x66
	ctx.r[3].s64 = 102;
	// 824AB53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AB540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AB540 size=132
    let mut pc: u32 = 0x824AB540;
    'dispatch: loop {
        match pc {
            0x824AB540 => {
    //   block [0x824AB540..0x824AB5C4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AB5C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AB5C4 size=28
    let mut pc: u32 = 0x824AB5C4;
    'dispatch: loop {
        match pc {
            0x824AB5C4 => {
    //   block [0x824AB5C4..0x824AB5E0)
	// 824AB5C4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AB5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AB5E0 size=88
    let mut pc: u32 = 0x824AB5E0;
    'dispatch: loop {
        match pc {
            0x824AB5E0 => {
    //   block [0x824AB5E0..0x824AB638)
	// 824AB5E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AB5E4: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 824AB5E8: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 824AB5EC: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 824AB5F0: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824AB5F4: 91440010  stw r10, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824AB5F8: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824AB5FC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AB600: 91040004  stw r8, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824AB604: 91240014  stw r9, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 824AB608: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 824AB60C: 1D4B0534  mulli r10, r11, 0x534
	ctx.r[10].s64 = ctx.r[11].s64 * 1332;
	// 824AB610: 394A0053  addi r10, r10, 0x53
	ctx.r[10].s64 = ctx.r[10].s64 + 83;
	// 824AB614: 554A0036  rlwinm r10, r10, 0, 0, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 824AB618: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 824AB61C: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824AB620: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824AB624: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824AB628: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AB62C: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824AB630: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824AB634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AB638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AB638 size=52
    let mut pc: u32 = 0x824AB638;
    'dispatch: loop {
        match pc {
            0x824AB638 => {
    //   block [0x824AB638..0x824AB66C)
	// 824AB638: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 824AB63C: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824AB640: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824AB644: 392B0003  addi r9, r11, 3
	ctx.r[9].s64 = ctx.r[11].s64 + 3;
	// 824AB648: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 824AB64C: 5529003A  rlwinm r9, r9, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 824AB650: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AB654: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824AB658: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824AB65C: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824AB660: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824AB664: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824AB668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AB670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AB670 size=72
    let mut pc: u32 = 0x824AB670;
    'dispatch: loop {
        match pc {
            0x824AB670 => {
    //   block [0x824AB670..0x824AB6B8)
	// 824AB670: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AB674: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AB678: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 824AB67C: 81230028  lwz r9, 0x28(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 824AB680: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 824AB684: 814A001C  lwz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 824AB688: 8129001C  lwz r9, 0x1c(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 824AB68C: 7D085830  slw r8, r8, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 824AB690: 7CE75830  slw r7, r7, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 824AB694: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AB698: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824AB69C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AB6A0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824AB6A4: 7D4B20AE  lbzx r10, r11, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 824AB6A8: 7D4A4078  andc r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 & !ctx.r[8].u64;
	// 824AB6AC: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 824AB6B0: 7D4B21AE  stbx r10, r11, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), ctx.r[10].u8) };
	// 824AB6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AB6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AB6B8 size=72
    let mut pc: u32 = 0x824AB6B8;
    'dispatch: loop {
        match pc {
            0x824AB6B8 => {
    //   block [0x824AB6B8..0x824AB700)
	// 824AB6B8: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AB6BC: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AB6C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 824AB6C4: 81230028  lwz r9, 0x28(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 824AB6C8: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 824AB6CC: 814A001C  lwz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 824AB6D0: 8129001C  lwz r9, 0x1c(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 824AB6D4: 7D085830  slw r8, r8, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 824AB6D8: 7CE75830  slw r7, r7, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 824AB6DC: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AB6E0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824AB6E4: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AB6E8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824AB6EC: 7D4B20AE  lbzx r10, r11, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 824AB6F0: 7D4A4078  andc r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 & !ctx.r[8].u64;
	// 824AB6F4: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 824AB6F8: 7D4B21AE  stbx r10, r11, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), ctx.r[10].u8) };
	// 824AB6FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AB700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824AB700 size=164
    let mut pc: u32 = 0x824AB700;
    'dispatch: loop {
        match pc {
            0x824AB700 => {
    //   block [0x824AB700..0x824AB7A4)
	// 824AB700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AB704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AB708: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AB70C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AB710: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824AB714: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824AB718: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AB71C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 824AB720: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 824AB724: C18B2984  lfs f12, 0x2984(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10628 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824AB728: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824AB72C: C1698E30  lfs f11, -0x71d0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-29136 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824AB730: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 824AB734: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824AB738: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 824AB73C: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AB740: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AB744: 394B0908  addi r10, r11, 0x908
	ctx.r[10].s64 = ctx.r[11].s64 + 2312;
	// 824AB748: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AB74C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824AB750: C1AB0900  lfs f13, 0x900(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2304 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824AB754: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AB758: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824AB75C: B11F000C  sth r8, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u16 ) };
	// 824AB760: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824AB764: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 824AB768: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 824AB76C: D19F0024  stfs f12, 0x24(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 824AB770: D01F0028  stfs f0, 0x28(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 824AB774: D1BF002C  stfs f13, 0x2c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824AB778: D01F0030  stfs f0, 0x30(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 824AB77C: D1BF0034  stfs f13, 0x34(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 824AB780: D01F0038  stfs f0, 0x38(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 824AB784: D17F003C  stfs f11, 0x3c(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 824AB788: 4BFF7731  bl 0x824a2eb8
	ctx.lr = 0x824AB78C;
	sub_824A2EB8(ctx, base);
	// 824AB78C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AB790: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824AB794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AB798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AB79C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AB7A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AB7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AB7A8 size=104
    let mut pc: u32 = 0x824AB7A8;
    'dispatch: loop {
        match pc {
            0x824AB7A8 => {
    //   block [0x824AB7A8..0x824AB810)
	// 824AB7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AB7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AB7B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AB7B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AB7B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AB7BC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AB7C0: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 824AB7C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AB7C8: 396B0908  addi r11, r11, 0x908
	ctx.r[11].s64 = ctx.r[11].s64 + 2312;
	// 824AB7CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824AB7D0: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 824AB7D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AB7D8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AB7DC: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824AB7E0: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AB7E4: 4BFF76D5  bl 0x824a2eb8
	ctx.lr = 0x824AB7E8;
	sub_824A2EB8(ctx, base);
	// 824AB7E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824AB7EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AB7F0: 4BFF76C9  bl 0x824a2eb8
	ctx.lr = 0x824AB7F4;
	sub_824A2EB8(ctx, base);
	// 824AB7F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AB7F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AB7FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AB800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AB804: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AB808: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AB80C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AB810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AB810 size=244
    let mut pc: u32 = 0x824AB810;
    'dispatch: loop {
        match pc {
            0x824AB810 => {
    //   block [0x824AB810..0x824AB904)
	// 824AB810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AB814: 480898A1  bl 0x825350b4
	ctx.lr = 0x824AB818;
	sub_82535080(ctx, base);
	// 824AB818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AB81C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824AB820: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AB824: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 824AB828: 396B0908  addi r11, r11, 0x908
	ctx.r[11].s64 = ctx.r[11].s64 + 2312;
	// 824AB82C: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 824AB830: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AB834: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AB838: 40990080  ble cr6, 0x824ab8b8
	if !ctx.cr[6].gt {
	pc = 0x824AB8B8; continue 'dispatch;
	}
	// 824AB83C: 3B800040  li r28, 0x40
	ctx.r[28].s64 = 64;
	// 824AB840: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 824AB844: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	// 824AB848: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 824AB84C: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824AB850: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824AB854: 419A0040  beq cr6, 0x824ab894
	if ctx.cr[6].eq {
	pc = 0x824AB894; continue 'dispatch;
	}
	// 824AB858: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824AB85C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AB860: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AB864: 419A0030  beq cr6, 0x824ab894
	if ctx.cr[6].eq {
	pc = 0x824AB894; continue 'dispatch;
	}
	// 824AB868: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824AB86C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824AB870: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824AB874: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AB878: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824AB87C: 409A0018  bne cr6, 0x824ab894
	if !ctx.cr[6].eq {
	pc = 0x824AB894; continue 'dispatch;
	}
	// 824AB880: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AB884: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824AB888: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AB88C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AB890: 4E800421  bctrl
	ctx.lr = 0x824AB894;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AB894: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 824AB898: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824AB89C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 824AB8A0: 409AFFA8  bne cr6, 0x824ab848
	if !ctx.cr[6].eq {
	pc = 0x824AB848; continue 'dispatch;
	}
	// 824AB8A4: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 824AB8A8: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 824AB8AC: 3B9C0050  addi r28, r28, 0x50
	ctx.r[28].s64 = ctx.r[28].s64 + 80;
	// 824AB8B0: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824AB8B4: 4198FF8C  blt cr6, 0x824ab840
	if ctx.cr[6].lt {
	pc = 0x824AB840; continue 'dispatch;
	}
	// 824AB8B8: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 824AB8BC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AB8C0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AB8C4: 409A002C  bne cr6, 0x824ab8f0
	if !ctx.cr[6].eq {
	pc = 0x824AB8F0; continue 'dispatch;
	}
	// 824AB8C8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AB8CC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824AB8D0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824AB8D4: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 824AB8D8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AB8DC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824AB8E0: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824AB8E4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824AB8E8: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AB8EC: 4BFB87CD  bl 0x824640b8
	ctx.lr = 0x824AB8F0;
	sub_824640B8(ctx, base);
	// 824AB8F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824AB8F4: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824AB8F8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AB8FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824AB900: 48089804  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AB908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AB908 size=256
    let mut pc: u32 = 0x824AB908;
    'dispatch: loop {
        match pc {
            0x824AB908 => {
    //   block [0x824AB908..0x824ABA08)
	// 824AB908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AB90C: 480897A1  bl 0x825350ac
	ctx.lr = 0x824AB910;
	sub_82535080(ctx, base);
	// 824AB910: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AB914: 3BE30018  addi r31, r3, 0x18
	ctx.r[31].s64 = ctx.r[3].s64 + 24;
	// 824AB918: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824AB91C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 824AB920: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 824AB924: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 824AB928: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AB92C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 824AB930: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AB934: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 824AB938: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824AB93C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824AB940: 409A0010  bne cr6, 0x824ab950
	if !ctx.cr[6].eq {
	pc = 0x824AB950; continue 'dispatch;
	}
	// 824AB944: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 824AB948: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AB94C: 4BFC2A05  bl 0x8246e350
	ctx.lr = 0x824AB950;
	sub_8246E350(ctx, base);
	// 824AB950: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AB954: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 824AB958: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AB95C: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 824AB960: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824AB964: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 824AB968: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824AB96C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AB970: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824AB974: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ABA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824ABA08 size=76
    let mut pc: u32 = 0x824ABA08;
    'dispatch: loop {
        match pc {
            0x824ABA08 => {
    //   block [0x824ABA08..0x824ABA54)
	// 824ABA08: C1A40004  lfs f13, 4(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824ABA0C: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824ABA10: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 824ABA14: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824ABA18: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824ABA1C: C003000C  lfs f0, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824ABA20: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 824ABA24: D0060004  stfs f0, 4(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824ABA28: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824ABA2C: D0060008  stfs f0, 8(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824ABA30: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824ABA34: D006000C  stfs f0, 0xc(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824ABA38: C0030014  lfs f0, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824ABA3C: D0060010  stfs f0, 0x10(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824ABA40: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824ABA44: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824ABA48: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 824ABA4C: D005001C  stfs f0, 0x1c(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 824ABA50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ABA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824ABA58 size=1948
    let mut pc: u32 = 0x824ABA58;
    'dispatch: loop {
        match pc {
            0x824ABA58 => {
    //   block [0x824ABA58..0x824AC1F4)
	// 824ABA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824ABA5C: 48089625  bl 0x82535080
	ctx.lr = 0x824ABA60;
	sub_82535080(ctx, base);
	// 824ABA60: DBE1FF60  stfd f31, -0xa0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 824ABA64: 9421F490  stwu r1, -0xb70(r1)
	ea = ctx.r[1].u32.wrapping_add(-2928 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824ABA68: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824ABA6C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 824ABA70: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 824ABA74: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 824ABA78: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824ABA7C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824ABA80: 80BB004C  lwz r5, 0x4c(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(76 as u32) ) } as u64;
	// 824ABA84: 93610B8C  stw r27, 0xb8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(2956 as u32), ctx.r[27].u32 ) };
	// 824ABA88: 93810B94  stw r28, 0xb94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(2964 as u32), ctx.r[28].u32 ) };
	// 824ABA8C: 480461E5  bl 0x824f1c70
	ctx.lr = 0x824ABA90;
	sub_824F1C70(ctx, base);
	// 824ABA90: 39410140  addi r10, r1, 0x140
	ctx.r[10].s64 = ctx.r[1].s64 + 320;
	// 824ABA94: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 824ABA98: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 824ABA9C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 824ABAA0: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 824ABAA4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824ABAA8: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 824ABAAC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824ABAB0: 4200FFF0  bdnz 0x824abaa0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824ABAA0; continue 'dispatch;
	}
	// 824ABAB4: 396101AC  addi r11, r1, 0x1ac
	ctx.r[11].s64 = ctx.r[1].s64 + 428;
	// 824ABAB8: 83FB0048  lwz r31, 0x48(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 824ABABC: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 824ABAC0: 815B0030  lwz r10, 0x30(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(48 as u32) ) } as u64;
	// 824ABAC4: 3BBF002C  addi r29, r31, 0x2c
	ctx.r[29].s64 = ctx.r[31].s64 + 44;
	// 824ABAC8: 916101A0  stw r11, 0x1a0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(416 as u32), ctx.r[11].u32 ) };
	// 824ABACC: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 824ABAD0: 93210188  stw r25, 0x188(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(392 as u32), ctx.r[25].u32 ) };
	// 824ABAD4: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 824ABAD8: 9321018C  stw r25, 0x18c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(396 as u32), ctx.r[25].u32 ) };
	// 824ABADC: 932101A4  stw r25, 0x1a4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(420 as u32), ctx.r[25].u32 ) };
	// 824ABAE0: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 824ABAE4: 916101A8  stw r11, 0x1a8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(424 as u32), ctx.r[11].u32 ) };
	// 824ABAE8: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824ABAEC: 81290090  lwz r9, 0x90(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(144 as u32) ) } as u64;
	// 824ABAF0: 7FC95050  subf r30, r9, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 824ABAF4: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824ABAF8: 392102CC  addi r9, r1, 0x2cc
	ctx.r[9].s64 = ctx.r[1].s64 + 716;
	// 824ABAFC: 932102C4  stw r25, 0x2c4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(708 as u32), ctx.r[25].u32 ) };
	// 824ABB00: 39EAFFFF  addi r15, r10, -1
	ctx.r[15].s64 = ctx.r[10].s64 + -1;
	// 824ABB04: 916102C8  stw r11, 0x2c8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(712 as u32), ctx.r[11].u32 ) };
	// 824ABB08: 2F0F0020  cmpwi cr6, r15, 0x20
	ctx.cr[6].compare_i32(ctx.r[15].s32, 32, &mut ctx.xer);
	// 824ABB0C: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 824ABB10: 912102C0  stw r9, 0x2c0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(704 as u32), ctx.r[9].u32 ) };
	// 824ABB14: 40990020  ble cr6, 0x824abb34
	if !ctx.cr[6].gt {
	pc = 0x824ABB34; continue 'dispatch;
	}
	// 824ABB18: 2F0F0040  cmpwi cr6, r15, 0x40
	ctx.cr[6].compare_i32(ctx.r[15].s32, 64, &mut ctx.xer);
	// 824ABB1C: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 824ABB20: 41980008  blt cr6, 0x824abb28
	if ctx.cr[6].lt {
	pc = 0x824ABB28; continue 'dispatch;
	}
	// 824ABB24: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 824ABB28: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 824ABB2C: 386102C0  addi r3, r1, 0x2c0
	ctx.r[3].s64 = ctx.r[1].s64 + 704;
	// 824ABB30: 4BFC2799  bl 0x8246e2c8
	ctx.lr = 0x824ABB34;
	sub_8246E2C8(ctx, base);
	// 824ABB34: 91E102C4  stw r15, 0x2c4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(708 as u32), ctx.r[15].u32 ) };
	// 824ABB38: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ABB3C: 814101A4  lwz r10, 0x1a4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(420 as u32) ) } as u64;
	// 824ABB40: 810101A0  lwz r8, 0x1a0(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(416 as u32) ) } as u64;
	// 824ABB44: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824ABB48: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ABB4C: 814B0090  lwz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 824ABB50: 396B00E0  addi r11, r11, 0xe0
	ctx.r[11].s64 = ctx.r[11].s64 + 224;
	// 824ABB54: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 824ABB58: 91410174  stw r10, 0x174(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(372 as u32), ctx.r[10].u32 ) };
	// 824ABB5C: 7D5E5050  subf r10, r30, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[30].s64;
	// 824ABB60: 7D49412E  stwx r10, r9, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u32) };
	// 824ABB64: 9161017C  stw r11, 0x17c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(380 as u32), ctx.r[11].u32 ) };
	// 824ABB68: 81610148  lwz r11, 0x148(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(328 as u32) ) } as u64;
	// 824ABB6C: 814101A4  lwz r10, 0x1a4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(420 as u32) ) } as u64;
	// 824ABB70: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ABB74: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824ABB78: 9161015C  stw r11, 0x15c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(348 as u32), ctx.r[11].u32 ) };
	// 824ABB7C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824ABB80: 914101A4  stw r10, 0x1a4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(420 as u32), ctx.r[10].u32 ) };
	// 824ABB84: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824ABB88: D0010160  stfs f0, 0x160(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(352 as u32), tmp.u32 ) };
	// 824ABB8C: 4804A3D5  bl 0x824f5f60
	ctx.lr = 0x824ABB90;
	sub_824F5F60(ctx, base);
	// 824ABB90: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ABB94: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 824ABB98: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 824ABB9C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 824ABBA0: 4804A3C9  bl 0x824f5f68
	ctx.lr = 0x824ABBA4;
	sub_824F5F68(ctx, base);
	// 824ABBA4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824ABBA8: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 824ABBAC: 7F35CB78  mr r21, r25
	ctx.r[21].u64 = ctx.r[25].u64;
	// 824ABBB0: 2F0F0000  cmpwi cr6, r15, 0
	ctx.cr[6].compare_i32(ctx.r[15].s32, 0, &mut ctx.xer);
	// 824ABBB4: C3EB1FF8  lfs f31, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824ABBB8: 4099052C  ble cr6, 0x824ac0e4
	if !ctx.cr[6].gt {
	pc = 0x824AC0E4; continue 'dispatch;
	}
	// 824ABBBC: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 824ABBC0: 93210050  stw r25, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[25].u32 ) };
	// 824ABBC4: 39D6001C  addi r14, r22, 0x1c
	ctx.r[14].s64 = ctx.r[22].s64 + 28;
	// 824ABBC8: 3A7F0028  addi r19, r31, 0x28
	ctx.r[19].s64 = ctx.r[31].s64 + 40;
	// 824ABBCC: 3B560018  addi r26, r22, 0x18
	ctx.r[26].s64 = ctx.r[22].s64 + 24;
	// 824ABBD0: 7F38CB78  mr r24, r25
	ctx.r[24].u64 = ctx.r[25].u64;
	// 824ABBD4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824ABBD8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824ABBDC: 3A000010  li r16, 0x10
	ctx.r[16].s64 = 16;
	// 824ABBE0: 396B08F0  addi r11, r11, 0x8f0
	ctx.r[11].s64 = ctx.r[11].s64 + 2288;
	// 824ABBE4: 3A200020  li r17, 0x20
	ctx.r[17].s64 = 32;
	// 824ABBE8: 3A400030  li r18, 0x30
	ctx.r[18].s64 = 48;
	// 824ABBEC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 824ABBF0: 81610174  lwz r11, 0x174(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 824ABBF4: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824ABBF8: 814101A8  lwz r10, 0x1a8(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(424 as u32) ) } as u64;
	// 824ABBFC: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 824ABC00: 91610170  stw r11, 0x170(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(368 as u32), ctx.r[11].u32 ) };
	// 824ABC04: 8161017C  lwz r11, 0x17c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(380 as u32) ) } as u64;
	// 824ABC08: 91610178  stw r11, 0x178(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(376 as u32), ctx.r[11].u32 ) };
	// 824ABC0C: 397F002C  addi r11, r31, 0x2c
	ctx.r[11].s64 = ctx.r[31].s64 + 44;
	// 824ABC10: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ABC14: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824ABC18: 812101A4  lwz r9, 0x1a4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(420 as u32) ) } as u64;
	// 824ABC1C: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824ABC20: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824ABC24: 3BEB00D0  addi r31, r11, 0xd0
	ctx.r[31].s64 = ctx.r[11].s64 + 208;
	// 824ABC28: 814B0090  lwz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 824ABC2C: 7D6AF214  add r11, r10, r30
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 824ABC30: 7FDE5850  subf r30, r30, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 824ABC34: 91610174  stw r11, 0x174(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(372 as u32), ctx.r[11].u32 ) };
	// 824ABC38: 409A0010  bne cr6, 0x824abc48
	if !ctx.cr[6].eq {
	pc = 0x824ABC48; continue 'dispatch;
	}
	// 824ABC3C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824ABC40: 386101A0  addi r3, r1, 0x1a0
	ctx.r[3].s64 = ctx.r[1].s64 + 416;
	// 824ABC44: 4BFC270D  bl 0x8246e350
	ctx.lr = 0x824ABC48;
	sub_8246E350(ctx, base);
	// 824ABC48: 816101A4  lwz r11, 0x1a4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(420 as u32) ) } as u64;
	// 824ABC4C: 38610140  addi r3, r1, 0x140
	ctx.r[3].s64 = ctx.r[1].s64 + 320;
	// 824ABC50: 814101A0  lwz r10, 0x1a0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(416 as u32) ) } as u64;
	// 824ABC54: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824ABC58: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 824ABC5C: 816101A4  lwz r11, 0x1a4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(420 as u32) ) } as u64;
	// 824ABC60: 813A0000  lwz r9, 0(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ABC64: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824ABC68: 81410178  lwz r10, 0x178(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(376 as u32) ) } as u64;
	// 824ABC6C: 7D384A14  add r9, r24, r9
	ctx.r[9].u64 = ctx.r[24].u64 + ctx.r[9].u64;
	// 824ABC70: 916101A4  stw r11, 0x1a4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(420 as u32), ctx.r[11].u32 ) };
	// 824ABC74: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 824ABC78: 9161017C  stw r11, 0x17c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(380 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AC1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AC1F8 size=100
    let mut pc: u32 = 0x824AC1F8;
    'dispatch: loop {
        match pc {
            0x824AC1F8 => {
    //   block [0x824AC1F8..0x824AC25C)
	// 824AC1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AC1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AC200: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AC204: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AC208: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AC20C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AC210: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824AC214: 4BFFF5FD  bl 0x824ab810
	ctx.lr = 0x824AC218;
	sub_824AB810(ctx, base);
	// 824AC218: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824AC21C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AC220: 419A0020  beq cr6, 0x824ac240
	if ctx.cr[6].eq {
	pc = 0x824AC240; continue 'dispatch;
	}
	// 824AC224: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC228: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824AC22C: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 824AC230: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AC234: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824AC238: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AC23C: 4BFB7E7D  bl 0x824640b8
	ctx.lr = 0x824AC240;
	sub_824640B8(ctx, base);
	// 824AC240: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AC244: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AC248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AC24C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AC250: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AC254: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AC258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AC260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AC260 size=8
    let mut pc: u32 = 0x824AC260;
    'dispatch: loop {
        match pc {
            0x824AC260 => {
    //   block [0x824AC260..0x824AC268)
	// 824AC260: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824AC264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AC268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AC268 size=60
    let mut pc: u32 = 0x824AC268;
    'dispatch: loop {
        match pc {
            0x824AC268 => {
    //   block [0x824AC268..0x824AC2A4)
	// 824AC268: 81240154  lwz r9, 0x154(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(340 as u32) ) } as u64;
	// 824AC26C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AC270: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824AC274: 40990024  ble cr6, 0x824ac298
	if !ctx.cr[6].gt {
	pc = 0x824AC298; continue 'dispatch;
	}
	// 824AC278: 81440150  lwz r10, 0x150(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(336 as u32) ) } as u64;
	// 824AC27C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC280: 7F082840  cmplw cr6, r8, r5
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[5].u32, &mut ctx.xer);
	// 824AC284: 419A0020  beq cr6, 0x824ac2a4
	if ctx.cr[6].eq {
		sub_824AC2A4(ctx, base);
		return;
	}
	// 824AC288: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824AC28C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824AC290: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824AC294: 4198FFE8  blt cr6, 0x824ac27c
	if ctx.cr[6].lt {
	pc = 0x824AC27C; continue 'dispatch;
	}
	// 824AC298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AC29C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824AC2A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AC2A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AC2A4 size=12
    let mut pc: u32 = 0x824AC2A4;
    'dispatch: loop {
        match pc {
            0x824AC2A4 => {
    //   block [0x824AC2A4..0x824AC2B0)
	// 824AC2A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824AC2A8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824AC2AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AC2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AC2B0 size=200
    let mut pc: u32 = 0x824AC2B0;
    'dispatch: loop {
        match pc {
            0x824AC2B0 => {
    //   block [0x824AC2B0..0x824AC378)
	// 824AC2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AC2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AC2B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AC2BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AC2C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AC2C4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC2C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AC2CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AC2D0: 419A0054  beq cr6, 0x824ac324
	if ctx.cr[6].eq {
	pc = 0x824AC324; continue 'dispatch;
	}
	// 824AC2D4: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 824AC2D8: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 824AC2DC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824AC2E0: 4198002C  blt cr6, 0x824ac30c
	if ctx.cr[6].lt {
	pc = 0x824AC30C; continue 'dispatch;
	}
	// 824AC2E4: 815F0150  lwz r10, 0x150(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 824AC2E8: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AC2EC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824AC2F0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC2F4: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824AC2F8: 419A0044  beq cr6, 0x824ac33c
	if ctx.cr[6].eq {
	pc = 0x824AC33C; continue 'dispatch;
	}
	// 824AC2FC: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 824AC300: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 824AC304: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824AC308: 4098FFE8  bge cr6, 0x824ac2f0
	if !ctx.cr[6].lt {
	pc = 0x824AC2F0; continue 'dispatch;
	}
	// 824AC30C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824AC310: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824AC314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AC318: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 824AC31C: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC320: 4BFFEA21  bl 0x824aad40
	ctx.lr = 0x824AC324;
	sub_824AAD40(ctx, base);
	// 824AC324: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AC328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AC32C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AC330: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AC334: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AC338: 4E800020  blr
	return;
	// 824AC33C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824AC340: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824AC344: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AC348: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 824AC34C: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC350: 4BFFE9F1  bl 0x824aad40
	ctx.lr = 0x824AC354;
	sub_824AAD40(ctx, base);
	// 824AC354: 815F0154  lwz r10, 0x154(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 824AC358: 57C9103A  slwi r9, r30, 2
	ctx.r[9].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824AC35C: 817F0150  lwz r11, 0x150(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 824AC360: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824AC364: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824AC368: 915F0154  stw r10, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[10].u32 ) };
	// 824AC36C: 7D48582E  lwzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AC370: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 824AC374: 4BFFFFB0  b 0x824ac324
	pc = 0x824AC324; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AC378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AC378 size=96
    let mut pc: u32 = 0x824AC378;
    'dispatch: loop {
        match pc {
            0x824AC378 => {
    //   block [0x824AC378..0x824AC3D8)
	// 824AC378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AC37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AC380: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AC384: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AC388: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AC38C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AC390: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 824AC394: 4BFFDE2D  bl 0x824aa1c0
	ctx.lr = 0x824AC398;
	sub_824AA1C0(ctx, base);
	// 824AC398: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AC39C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824AC3A0: 396BEDCC  addi r11, r11, -0x1234
	ctx.r[11].s64 = ctx.r[11].s64 + -4660;
	// 824AC3A4: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 824AC3A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AC3AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AC3B0: 915F0150  stw r10, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[10].u32 ) };
	// 824AC3B4: 915F0154  stw r10, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[10].u32 ) };
	// 824AC3B8: 913F0158  stw r9, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[9].u32 ) };
	// 824AC3BC: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 824AC3C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AC3C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AC3C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AC3CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AC3D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AC3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AC3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AC3D8 size=104
    let mut pc: u32 = 0x824AC3D8;
    'dispatch: loop {
        match pc {
            0x824AC3D8 => {
    //   block [0x824AC3D8..0x824AC440)
	// 824AC3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AC3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AC3E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AC3E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AC3E8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AC3EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AC3F0: 396BEDCC  addi r11, r11, -0x1234
	ctx.r[11].s64 = ctx.r[11].s64 + -4660;
	// 824AC3F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AC3F8: 817F0158  lwz r11, 0x158(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(344 as u32) ) } as u64;
	// 824AC3FC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AC400: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AC404: 409A0020  bne cr6, 0x824ac424
	if !ctx.cr[6].eq {
	pc = 0x824AC424; continue 'dispatch;
	}
	// 824AC408: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC40C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824AC410: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AC414: 809F0150  lwz r4, 0x150(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 824AC418: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AC41C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824AC420: 4BFB7C99  bl 0x824640b8
	ctx.lr = 0x824AC424;
	sub_824640B8(ctx, base);
	// 824AC424: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AC428: 4BFFD779  bl 0x824a9ba0
	ctx.lr = 0x824AC42C;
	sub_824A9BA0(ctx, base);
	// 824AC42C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824AC430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AC434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AC438: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AC43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AC440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AC440 size=440
    let mut pc: u32 = 0x824AC440;
    'dispatch: loop {
        match pc {
            0x824AC440 => {
    //   block [0x824AC440..0x824AC5F8)
	// 824AC440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AC444: 48088C69  bl 0x825350ac
	ctx.lr = 0x824AC448;
	sub_82535080(ctx, base);
	// 824AC448: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AC44C: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC450: 3B400010  li r26, 0x10
	ctx.r[26].s64 = 16;
	// 824AC454: 38A00031  li r5, 0x31
	ctx.r[5].s64 = 49;
	// 824AC458: 38800160  li r4, 0x160
	ctx.r[4].s64 = 352;
	// 824AC45C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824AC460: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824AC464: 4BFB7BD5  bl 0x82464038
	ctx.lr = 0x824AC468;
	sub_82464038(ctx, base);
	// 824AC468: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AC46C: 39600160  li r11, 0x160
	ctx.r[11].s64 = 352;
	// 824AC470: 38BB00A0  addi r5, r27, 0xa0
	ctx.r[5].s64 = ctx.r[27].s64 + 160;
	// 824AC474: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 824AC478: 839B002C  lwz r28, 0x2c(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(44 as u32) ) } as u64;
	// 824AC47C: 809B0010  lwz r4, 0x10(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 824AC480: 4BFFDD41  bl 0x824aa1c0
	ctx.lr = 0x824AC484;
	sub_824AA1C0(ctx, base);
	// 824AC484: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AC488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824AC48C: 396BEDCC  addi r11, r11, -0x1234
	ctx.r[11].s64 = ctx.r[11].s64 + -4660;
	// 824AC490: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 824AC494: 3BDF0080  addi r30, r31, 0x80
	ctx.r[30].s64 = ctx.r[31].s64 + 128;
	// 824AC498: 3BBB0080  addi r29, r27, 0x80
	ctx.r[29].s64 = ctx.r[27].s64 + 128;
	// 824AC49C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AC4A0: 915F0150  stw r10, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[10].u32 ) };
	// 824AC4A4: 915F0154  stw r10, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[10].u32 ) };
	// 824AC4A8: 913F0158  stw r9, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[9].u32 ) };
	// 824AC4AC: 939F002C  stw r28, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 824AC4B0: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AC4B4: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AC4B8: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 824AC4BC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824AC4C0: 40980050  bge cr6, 0x824ac510
	if !ctx.cr[6].lt {
	pc = 0x824AC510; continue 'dispatch;
	}
	// 824AC4C4: 554A0000  rlwinm r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 824AC4C8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AC4CC: 409A0018  bne cr6, 0x824ac4e4
	if !ctx.cr[6].eq {
	pc = 0x824AC4E4; continue 'dispatch;
	}
	// 824AC4D0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AC4D4: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC4D8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AC4DC: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824AC4E0: 4BFB7BD9  bl 0x824640b8
	ctx.lr = 0x824AC4E4;
	sub_824640B8(ctx, base);
	// 824AC4E4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AC4E8: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 824AC4EC: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824AC4F0: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824AC4F4: 4BFB7B45  bl 0x82464038
	ctx.lr = 0x824AC4F8;
	sub_82464038(ctx, base);
	// 824AC4F8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AC4FC: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824AC500: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AC504: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AC508: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 824AC50C: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824AC510: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AC514: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC518: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AC51C: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824AC520: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC524: 40990020  ble cr6, 0x824ac544
	if !ctx.cr[6].gt {
	pc = 0x824AC544; continue 'dispatch;
	}
	// 824AC528: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 824AC52C: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AC530: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824AC534: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824AC538: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824AC53C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824AC540: 409AFFEC  bne cr6, 0x824ac52c
	if !ctx.cr[6].eq {
	pc = 0x824AC52C; continue 'dispatch;
	}
	// 824AC544: 3BDF008C  addi r30, r31, 0x8c
	ctx.r[30].s64 = ctx.r[31].s64 + 140;
	// 824AC548: 3BBB008C  addi r29, r27, 0x8c
	ctx.r[29].s64 = ctx.r[27].s64 + 140;
	// 824AC54C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AC550: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AC554: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 824AC558: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824AC55C: 40980050  bge cr6, 0x824ac5ac
	if !ctx.cr[6].lt {
	pc = 0x824AC5AC; continue 'dispatch;
	}
	// 824AC560: 554A0000  rlwinm r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 824AC564: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AC568: 409A0018  bne cr6, 0x824ac580
	if !ctx.cr[6].eq {
	pc = 0x824AC580; continue 'dispatch;
	}
	// 824AC56C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AC570: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC574: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AC578: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824AC57C: 4BFB7B3D  bl 0x824640b8
	ctx.lr = 0x824AC580;
	sub_824640B8(ctx, base);
	// 824AC580: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AC584: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 824AC588: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824AC58C: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824AC590: 4BFB7AA9  bl 0x82464038
	ctx.lr = 0x824AC594;
	sub_82464038(ctx, base);
	// 824AC594: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AC598: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824AC59C: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AC5A0: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AC5A4: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 824AC5A8: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824AC5AC: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AC5B0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC5B4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AC5B8: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824AC5BC: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC5C0: 40990020  ble cr6, 0x824ac5e0
	if !ctx.cr[6].gt {
	pc = 0x824AC5E0; continue 'dispatch;
	}
	// 824AC5C4: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 824AC5C8: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AC5CC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824AC5D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824AC5D4: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824AC5D8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824AC5DC: 409AFFEC  bne cr6, 0x824ac5c8
	if !ctx.cr[6].eq {
	pc = 0x824AC5C8; continue 'dispatch;
	}
	// 824AC5E0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824AC5E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AC5E8: 4BFE4A09  bl 0x82490ff0
	ctx.lr = 0x824AC5EC;
	sub_82490FF0(ctx, base);
	// 824AC5EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AC5F0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824AC5F4: 48088B08  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AC5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AC5F8 size=172
    let mut pc: u32 = 0x824AC5F8;
    'dispatch: loop {
        match pc {
            0x824AC5F8 => {
    //   block [0x824AC5F8..0x824AC6A4)
	// 824AC5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AC5FC: 48088AB1  bl 0x825350ac
	ctx.lr = 0x824AC600;
	sub_82535080(ctx, base);
	// 824AC600: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AC604: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 824AC608: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824AC60C: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 824AC610: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 824AC614: 409A000C  bne cr6, 0x824ac620
	if !ctx.cr[6].eq {
	pc = 0x824AC620; continue 'dispatch;
	}
	// 824AC618: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AC61C: 834B006C  lwz r26, 0x6c(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 824AC620: 817B0154  lwz r11, 0x154(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(340 as u32) ) } as u64;
	// 824AC624: 839A0000  lwz r28, 0(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC628: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 824AC62C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824AC630: 4198006C  blt cr6, 0x824ac69c
	if ctx.cr[6].lt {
	pc = 0x824AC69C; continue 'dispatch;
	}
	// 824AC634: 3BBB0010  addi r29, r27, 0x10
	ctx.r[29].s64 = ctx.r[27].s64 + 16;
	// 824AC638: 57FE103A  slwi r30, r31, 2
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 824AC63C: 815B0150  lwz r10, 0x150(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(336 as u32) ) } as u64;
	// 824AC640: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 824AC644: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC648: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 824AC64C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824AC650: 7C8AF02E  lwzx r4, r10, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824AC654: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AC658: 396B000D  addi r11, r11, 0xd
	ctx.r[11].s64 = ctx.r[11].s64 + 13;
	// 824AC65C: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC660: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AC664: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AC668: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824AC66C: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 824AC670: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 824AC674: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824AC678: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AC67C: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 824AC680: 816B09A8  lwz r11, 0x9a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2472 as u32) ) } as u64;
	// 824AC684: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AC688: 4E800421  bctrl
	ctx.lr = 0x824AC68C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AC68C: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 824AC690: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 824AC694: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824AC698: 4098FFA4  bge cr6, 0x824ac63c
	if !ctx.cr[6].lt {
	pc = 0x824AC63C; continue 'dispatch;
	}
	// 824AC69C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824AC6A0: 48088A5C  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AC6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AC6A8 size=184
    let mut pc: u32 = 0x824AC6A8;
    'dispatch: loop {
        match pc {
            0x824AC6A8 => {
    //   block [0x824AC6A8..0x824AC760)
	// 824AC6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AC6AC: 48088A01  bl 0x825350ac
	ctx.lr = 0x824AC6B0;
	sub_82535080(ctx, base);
	// 824AC6B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AC6B4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 824AC6B8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824AC6BC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824AC6C0: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 824AC6C4: 409A000C  bne cr6, 0x824ac6d0
	if !ctx.cr[6].eq {
	pc = 0x824AC6D0; continue 'dispatch;
	}
	// 824AC6C8: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AC6CC: 834B006C  lwz r26, 0x6c(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 824AC6D0: 817B0154  lwz r11, 0x154(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(340 as u32) ) } as u64;
	// 824AC6D4: 83DA0000  lwz r30, 0(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC6D8: 3B2BFFFF  addi r25, r11, -1
	ctx.r[25].s64 = ctx.r[11].s64 + -1;
	// 824AC6DC: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 824AC6E0: 41980078  blt cr6, 0x824ac758
	if ctx.cr[6].lt {
	pc = 0x824AC758; continue 'dispatch;
	}
	// 824AC6E4: 3BFB0010  addi r31, r27, 0x10
	ctx.r[31].s64 = ctx.r[27].s64 + 16;
	// 824AC6E8: 573C103A  slwi r28, r25, 2
	ctx.r[28].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 824AC6EC: 815B0150  lwz r10, 0x150(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(336 as u32) ) } as u64;
	// 824AC6F0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 824AC6F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC6F8: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 824AC6FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AC700: 7C8AE02E  lwzx r4, r10, r28
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 824AC704: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AC708: 396B000D  addi r11, r11, 0xd
	ctx.r[11].s64 = ctx.r[11].s64 + 13;
	// 824AC70C: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC710: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AC714: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AC718: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824AC71C: 7D6BF0AE  lbzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824AC720: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 824AC724: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824AC728: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AC72C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824AC730: 816B09A4  lwz r11, 0x9a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2468 as u32) ) } as u64;
	// 824AC734: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AC738: 4E800421  bctrl
	ctx.lr = 0x824AC73C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AC73C: 897D0004  lbz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AC740: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AC744: 409A0014  bne cr6, 0x824ac758
	if !ctx.cr[6].eq {
	pc = 0x824AC758; continue 'dispatch;
	}
	// 824AC748: 3B39FFFF  addi r25, r25, -1
	ctx.r[25].s64 = ctx.r[25].s64 + -1;
	// 824AC74C: 3B9CFFFC  addi r28, r28, -4
	ctx.r[28].s64 = ctx.r[28].s64 + -4;
	// 824AC750: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 824AC754: 4098FF98  bge cr6, 0x824ac6ec
	if !ctx.cr[6].lt {
	pc = 0x824AC6EC; continue 'dispatch;
	}
	// 824AC758: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824AC75C: 480889A0  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AC760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AC760 size=136
    let mut pc: u32 = 0x824AC760;
    'dispatch: loop {
        match pc {
            0x824AC760 => {
    //   block [0x824AC760..0x824AC7E8)
	// 824AC760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AC764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AC768: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AC76C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AC770: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AC774: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824AC778: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AC77C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC780: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AC784: 419A004C  beq cr6, 0x824ac7d0
	if ctx.cr[6].eq {
	pc = 0x824AC7D0; continue 'dispatch;
	}
	// 824AC788: 4BFFE541  bl 0x824aacc8
	ctx.lr = 0x824AC78C;
	sub_824AACC8(ctx, base);
	// 824AC78C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824AC790: 409A0040  bne cr6, 0x824ac7d0
	if !ctx.cr[6].eq {
	pc = 0x824AC7D0; continue 'dispatch;
	}
	// 824AC794: 3BFF0150  addi r31, r31, 0x150
	ctx.r[31].s64 = ctx.r[31].s64 + 336;
	// 824AC798: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AC79C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AC7A0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824AC7A4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824AC7A8: 409A0010  bne cr6, 0x824ac7b8
	if !ctx.cr[6].eq {
	pc = 0x824AC7B8; continue 'dispatch;
	}
	// 824AC7AC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824AC7B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AC7B4: 4BFC1B9D  bl 0x8246e350
	ctx.lr = 0x824AC7B8;
	sub_8246E350(ctx, base);
	// 824AC7B8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AC7BC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC7C0: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824AC7C4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824AC7C8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824AC7CC: 7FCA492E  stwx r30, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u32) };
	// 824AC7D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AC7D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AC7D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AC7DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AC7E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AC7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AC7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AC7E8 size=184
    let mut pc: u32 = 0x824AC7E8;
    'dispatch: loop {
        match pc {
            0x824AC7E8 => {
    //   block [0x824AC7E8..0x824AC8A0)
	// 824AC7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AC7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AC7F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AC7F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AC7F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AC7FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824AC800: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AC804: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824AC808: 388B0948  addi r4, r11, 0x948
	ctx.r[4].s64 = ctx.r[11].s64 + 2376;
	// 824AC80C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824AC810: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC814: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 824AC818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AC81C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AC820: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AC824: 4E800421  bctrl
	ctx.lr = 0x824AC828;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AC828: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824AC82C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AC830: 4BFFD1F9  bl 0x824a9a28
	ctx.lr = 0x824AC834;
	sub_824A9A28(ctx, base);
	// 824AC834: 817E0158  lwz r11, 0x158(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(344 as u32) ) } as u64;
	// 824AC838: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AC83C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AC840: 409A0034  bne cr6, 0x824ac874
	if !ctx.cr[6].eq {
	pc = 0x824AC874; continue 'dispatch;
	}
	// 824AC844: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC848: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824AC84C: 80FE0154  lwz r7, 0x154(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(340 as u32) ) } as u64;
	// 824AC850: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824AC854: 80DE0150  lwz r6, 0x150(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(336 as u32) ) } as u64;
	// 824AC858: 388A093C  addi r4, r10, 0x93c
	ctx.r[4].s64 = ctx.r[10].s64 + 2364;
	// 824AC85C: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824AC860: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824AC864: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AC868: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AC86C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AC870: 4E800421  bctrl
	ctx.lr = 0x824AC874;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AC874: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC878: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AC87C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824AC880: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AC884: 4E800421  bctrl
	ctx.lr = 0x824AC888;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AC888: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AC88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AC890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AC894: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AC898: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AC89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AC8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AC8A0 size=144
    let mut pc: u32 = 0x824AC8A0;
    'dispatch: loop {
        match pc {
            0x824AC8A0 => {
    //   block [0x824AC8A0..0x824AC930)
	// 824AC8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AC8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AC8A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AC8AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AC8B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AC8B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824AC8B8: 817E0154  lwz r11, 0x154(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(340 as u32) ) } as u64;
	// 824AC8BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AC8C0: 409A0050  bne cr6, 0x824ac910
	if !ctx.cr[6].eq {
	pc = 0x824AC910; continue 'dispatch;
	}
	// 824AC8C4: 3BFE0150  addi r31, r30, 0x150
	ctx.r[31].s64 = ctx.r[30].s64 + 336;
	// 824AC8C8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AC8CC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AC8D0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AC8D4: 409A0020  bne cr6, 0x824ac8f4
	if !ctx.cr[6].eq {
	pc = 0x824AC8F4; continue 'dispatch;
	}
	// 824AC8D8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC8DC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824AC8E0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AC8E4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AC8E8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AC8EC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824AC8F0: 4BFB77C9  bl 0x824640b8
	ctx.lr = 0x824AC8F4;
	sub_824640B8(ctx, base);
	// 824AC8F4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AC8F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824AC8FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AC900: 512AF880  rlwimi r10, r9, 0x1f, 2, 0
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(31) as u64) & 0xFFFFFFFFBFFFFFFF) | (ctx.r[10].u64 & 0x0000000040000000);
	// 824AC904: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AC908: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824AC90C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824AC910: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AC914: 4BFFD5FD  bl 0x824a9f10
	ctx.lr = 0x824AC918;
	sub_824A9F10(ctx, base);
	// 824AC918: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AC91C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AC920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AC924: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AC928: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AC92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AC930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AC930 size=532
    let mut pc: u32 = 0x824AC930;
    'dispatch: loop {
        match pc {
            0x824AC930 => {
    //   block [0x824AC930..0x824ACB44)
	// 824AC930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AC934: 48088779  bl 0x825350ac
	ctx.lr = 0x824AC938;
	sub_82535080(ctx, base);
	// 824AC938: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ACB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824ACB48 size=8
    let mut pc: u32 = 0x824ACB48;
    'dispatch: loop {
        match pc {
            0x824ACB48 => {
    //   block [0x824ACB48..0x824ACB50)
	// 824ACB48: 38600065  li r3, 0x65
	ctx.r[3].s64 = 101;
	// 824ACB4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ACB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824ACB50 size=84
    let mut pc: u32 = 0x824ACB50;
    'dispatch: loop {
        match pc {
            0x824ACB50 => {
    //   block [0x824ACB50..0x824ACBA4)
	// 824ACB50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824ACB54: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 824ACB58: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 824ACB5C: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 824ACB60: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824ACB64: 91440010  stw r10, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824ACB68: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824ACB6C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824ACB70: 91040004  stw r8, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824ACB74: 91240014  stw r9, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 824ACB78: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 824ACB7C: 1D4B0134  mulli r10, r11, 0x134
	ctx.r[10].s64 = ctx.r[11].s64 * 308;
	// 824ACB80: 394A0033  addi r10, r10, 0x33
	ctx.r[10].s64 = ctx.r[10].s64 + 51;
	// 824ACB84: 554A0036  rlwinm r10, r10, 0, 0, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 824ACB88: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 824ACB8C: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824ACB90: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824ACB94: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824ACB98: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824ACB9C: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824ACBA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ACBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824ACBA8 size=40
    let mut pc: u32 = 0x824ACBA8;
    'dispatch: loop {
        match pc {
            0x824ACBA8 => {
    //   block [0x824ACBA8..0x824ACBD0)
	// 824ACBA8: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 824ACBAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824ACBB0: 419A0020  beq cr6, 0x824acbd0
	if ctx.cr[6].eq {
		sub_824ACBD0(ctx, base);
		return;
	}
	// 824ACBB4: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 824ACBB8: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824ACBBC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824ACBC0: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824ACBC4: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824ACBC8: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824ACBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ACBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824ACBD0 size=16
    let mut pc: u32 = 0x824ACBD0;
    'dispatch: loop {
        match pc {
            0x824ACBD0 => {
    //   block [0x824ACBD0..0x824ACBE0)
	// 824ACBD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824ACBD4: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824ACBD8: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824ACBDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ACBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824ACBE0 size=152
    let mut pc: u32 = 0x824ACBE0;
    'dispatch: loop {
        match pc {
            0x824ACBE0 => {
    //   block [0x824ACBE0..0x824ACC78)
	// 824ACBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824ACBE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824ACBE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824ACBEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824ACBF0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824ACBF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824ACBF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824ACBFC: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 824ACC00: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 824ACC04: C00B2984  lfs f0, 0x2984(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10628 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824ACC08: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824ACC0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824ACC10: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 824ACC14: C1AB1850  lfs f13, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824ACC18: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824ACC1C: 394B095C  addi r10, r11, 0x95c
	ctx.r[10].s64 = ctx.r[11].s64 + 2396;
	// 824ACC20: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824ACC24: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824ACC28: C18B0900  lfs f12, 0x900(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2304 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824ACC2C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824ACC30: C16B8E30  lfs f11, -0x71d0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29136 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824ACC34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824ACC38: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824ACC3C: B13F000C  sth r9, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u16 ) };
	// 824ACC40: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824ACC44: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 824ACC48: 911F0020  stw r8, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 824ACC4C: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 824ACC50: D1BF0028  stfs f13, 0x28(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 824ACC54: D19F002C  stfs f12, 0x2c(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824ACC58: D17F0030  stfs f11, 0x30(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 824ACC5C: 4BFF625D  bl 0x824a2eb8
	ctx.lr = 0x824ACC60;
	sub_824A2EB8(ctx, base);
	// 824ACC60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824ACC64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824ACC68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824ACC6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824ACC70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824ACC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ACC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824ACC78 size=104
    let mut pc: u32 = 0x824ACC78;
    'dispatch: loop {
        match pc {
            0x824ACC78 => {
    //   block [0x824ACC78..0x824ACCE0)
	// 824ACC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824ACC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824ACC80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824ACC84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824ACC88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824ACC8C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824ACC90: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 824ACC94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824ACC98: 396B095C  addi r11, r11, 0x95c
	ctx.r[11].s64 = ctx.r[11].s64 + 2396;
	// 824ACC9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824ACCA0: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 824ACCA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824ACCA8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824ACCAC: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824ACCB0: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824ACCB4: 4BFF6205  bl 0x824a2eb8
	ctx.lr = 0x824ACCB8;
	sub_824A2EB8(ctx, base);
	// 824ACCB8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824ACCBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824ACCC0: 4BFF61F9  bl 0x824a2eb8
	ctx.lr = 0x824ACCC4;
	sub_824A2EB8(ctx, base);
	// 824ACCC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824ACCC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824ACCCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824ACCD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824ACCD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824ACCD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824ACCDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ACCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824ACCE0 size=108
    let mut pc: u32 = 0x824ACCE0;
    'dispatch: loop {
        match pc {
            0x824ACCE0 => {
    //   block [0x824ACCE0..0x824ACD4C)
	// 824ACCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824ACCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824ACCE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824ACCEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824ACCF0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824ACCF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824ACCF8: 396B095C  addi r11, r11, 0x95c
	ctx.r[11].s64 = ctx.r[11].s64 + 2396;
	// 824ACCFC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824ACD00: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824ACD04: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824ACD08: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824ACD0C: 409A0020  bne cr6, 0x824acd2c
	if !ctx.cr[6].eq {
	pc = 0x824ACD2C; continue 'dispatch;
	}
	// 824ACD10: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ACD14: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824ACD18: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824ACD1C: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824ACD20: 55652834  slwi r5, r11, 5
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824ACD24: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824ACD28: 4BFB7391  bl 0x824640b8
	ctx.lr = 0x824ACD2C;
	sub_824640B8(ctx, base);
	// 824ACD2C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824ACD30: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824ACD34: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824ACD38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824ACD3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824ACD40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824ACD44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824ACD48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ACD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824ACD50 size=108
    let mut pc: u32 = 0x824ACD50;
    'dispatch: loop {
        match pc {
            0x824ACD50 => {
    //   block [0x824ACD50..0x824ACDBC)
	// 824ACD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824ACD54: 48088369  bl 0x825350bc
	ctx.lr = 0x824ACD58;
	sub_82535080(ctx, base);
	// 824ACD58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824ACD5C: 3BE30018  addi r31, r3, 0x18
	ctx.r[31].s64 = ctx.r[3].s64 + 24;
	// 824ACD60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824ACD64: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824ACD68: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824ACD6C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824ACD70: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824ACD74: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824ACD78: 409A0010  bne cr6, 0x824acd88
	if !ctx.cr[6].eq {
	pc = 0x824ACD88; continue 'dispatch;
	}
	// 824ACD7C: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 824ACD80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824ACD84: 4BFC15CD  bl 0x8246e350
	ctx.lr = 0x824ACD88;
	sub_8246E350(ctx, base);
	// 824ACD88: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824ACD8C: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 824ACD90: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ACD94: 556A2834  slwi r10, r11, 5
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824ACD98: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 824ACD9C: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824ACDA0: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ACDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824ACDC0 size=640
    let mut pc: u32 = 0x824ACDC0;
    'dispatch: loop {
        match pc {
            0x824ACDC0 => {
    //   block [0x824ACDC0..0x824AD040)
	// 824ACDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824ACDC4: 480882C9  bl 0x8253508c
	ctx.lr = 0x824ACDC8;
	sub_82535080(ctx, base);
	// 824ACDC8: 9421FE10  stwu r1, -0x1f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-496 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824ACDCC: 7C932378  mr r19, r4
	ctx.r[19].u64 = ctx.r[4].u64;
	// 824ACDD0: 7CB12B78  mr r17, r5
	ctx.r[17].u64 = ctx.r[5].u64;
	// 824ACDD4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 824ACDD8: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 824ACDDC: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 824ACDE0: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 824ACDE4: 80B3004C  lwz r5, 0x4c(r19)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(76 as u32) ) } as u64;
	// 824ACDE8: 48044E89  bl 0x824f1c70
	ctx.lr = 0x824ACDEC;
	sub_824F1C70(ctx, base);
	// 824ACDEC: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 824ACDF0: 7E6B9B78  mr r11, r19
	ctx.r[11].u64 = ctx.r[19].u64;
	// 824ACDF4: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 824ACDF8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 824ACDFC: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 824ACE00: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824ACE04: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 824ACE08: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824ACE0C: 4200FFF0  bdnz 0x824acdfc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824ACDFC; continue 'dispatch;
	}
	// 824ACE10: 394100EC  addi r10, r1, 0xec
	ctx.r[10].s64 = ctx.r[1].s64 + 236;
	// 824ACE14: 82930048  lwz r20, 0x48(r19)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(72 as u32) ) } as u64;
	// 824ACE18: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824ACE1C: 81730030  lwz r11, 0x30(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(48 as u32) ) } as u64;
	// 824ACE20: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824ACE24: 80710000  lwz r3, 0(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ACE28: 914100E0  stw r10, 0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[10].u32 ) };
	// 824ACE2C: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 824ACE30: 93C100E4  stw r30, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[30].u32 ) };
	// 824ACE34: 614A0020  ori r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 | 32;
	// 824ACE38: 93C100A8  stw r30, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 824ACE3C: 93C100AC  stw r30, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[30].u32 ) };
	// 824ACE40: 914100E8  stw r10, 0xe8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[10].u32 ) };
	// 824ACE44: 81540014  lwz r10, 0x14(r20)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(20 as u32) ) } as u64;
	// 824ACE48: 814A0090  lwz r10, 0x90(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(144 as u32) ) } as u64;
	// 824ACE4C: 7EAA5850  subf r21, r10, r11
	ctx.r[21].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 824ACE50: 8154002C  lwz r10, 0x2c(r20)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(44 as u32) ) } as u64;
	// 824ACE54: 81740030  lwz r11, 0x30(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(48 as u32) ) } as u64;
	// 824ACE58: 3A4BFFFF  addi r18, r11, -1
	ctx.r[18].s64 = ctx.r[11].s64 + -1;
	// 824ACE5C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ACE60: 814B0090  lwz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 824ACE64: 396B00E0  addi r11, r11, 0xe0
	ctx.r[11].s64 = ctx.r[11].s64 + 224;
	// 824ACE68: 912100E4  stw r9, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[9].u32 ) };
	// 824ACE6C: 9161009C  stw r11, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 824ACE70: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824ACE74: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824ACE78: 7D6AAA14  add r11, r10, r21
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[21].u64;
	// 824ACE7C: D0010080  stfs f0, 0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 824ACE80: 91610094  stw r11, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 824ACE84: 7D755850  subf r11, r21, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[21].s64;
	// 824ACE88: 916100EC  stw r11, 0xec(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), ctx.r[11].u32 ) };
	// 824ACE8C: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 824ACE90: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 824ACE94: 480490C5  bl 0x824f5f58
	ctx.lr = 0x824ACE98;
	sub_824F5F58(ctx, base);
	// 824ACE98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824ACE9C: 2F120000  cmpwi cr6, r18, 0
	ctx.cr[6].compare_i32(ctx.r[18].s32, 0, &mut ctx.xer);
	// 824ACEA0: 40990144  ble cr6, 0x824acfe4
	if !ctx.cr[6].gt {
	pc = 0x824ACFE4; continue 'dispatch;
	}
	// 824ACEA4: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 824ACEA8: 7E5D9378  mr r29, r18
	ctx.r[29].u64 = ctx.r[18].u64;
	// 824ACEAC: 3AC00010  li r22, 0x10
	ctx.r[22].s64 = 16;
	// 824ACEB0: 3AE00020  li r23, 0x20
	ctx.r[23].s64 = 32;
	// 824ACEB4: 3B000030  li r24, 0x30
	ctx.r[24].s64 = 48;
	// 824ACEB8: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 824ACEBC: 814100E8  lwz r10, 0xe8(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(232 as u32) ) } as u64;
	// 824ACEC0: 812100E4  lwz r9, 0xe4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(228 as u32) ) } as u64;
	// 824ACEC4: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 824ACEC8: 91610090  stw r11, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 824ACECC: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824ACED0: 8161009C  lwz r11, 0x9c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 824ACED4: 91610098  stw r11, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 824ACED8: 8174002C  lwz r11, 0x2c(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(44 as u32) ) } as u64;
	// 824ACEDC: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 824ACEE0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824ACEE4: 3B6B00D0  addi r27, r11, 0xd0
	ctx.r[27].s64 = ctx.r[11].s64 + 208;
	// 824ACEE8: 814B0090  lwz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 824ACEEC: 7D6AAA14  add r11, r10, r21
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[21].u64;
	// 824ACEF0: 7F555850  subf r26, r21, r11
	ctx.r[26].s64 = ctx.r[11].s64 - ctx.r[21].s64;
	// 824ACEF4: 91610094  stw r11, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 824ACEF8: 409A0010  bne cr6, 0x824acf08
	if !ctx.cr[6].eq {
	pc = 0x824ACF08; continue 'dispatch;
	}
	// 824ACEFC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824ACF00: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 824ACF04: 4BFC144D  bl 0x8246e350
	ctx.lr = 0x824ACF08;
	sub_8246E350(ctx, base);
	// 824ACF08: 816100E4  lwz r11, 0xe4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(228 as u32) ) } as u64;
	// 824ACF0C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 824ACF10: 814100E0  lwz r10, 0xe0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(224 as u32) ) } as u64;
	// 824ACF14: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824ACF18: 7F4B512E  stwx r26, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u32) };
	// 824ACF1C: 816100E4  lwz r11, 0xe4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(228 as u32) ) } as u64;
	// 824ACF20: 81390018  lwz r9, 0x18(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 824ACF24: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824ACF28: 81410098  lwz r10, 0x98(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 824ACF2C: 7D3C4A14  add r9, r28, r9
	ctx.r[9].u64 = ctx.r[28].u64 + ctx.r[9].u64;
	// 824ACF30: 916100E4  stw r11, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 824ACF34: 397B0010  addi r11, r27, 0x10
	ctx.r[11].s64 = ctx.r[27].s64 + 16;
	// 824ACF38: 9161009C  stw r11, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AD040 size=164
    let mut pc: u32 = 0x824AD040;
    'dispatch: loop {
        match pc {
            0x824AD040 => {
    //   block [0x824AD040..0x824AD0E4)
	// 824AD040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AD044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AD048: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AD04C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AD050: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AD054: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AD058: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AD05C: 396B095C  addi r11, r11, 0x95c
	ctx.r[11].s64 = ctx.r[11].s64 + 2396;
	// 824AD060: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824AD064: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AD068: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824AD06C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AD070: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AD074: 409A0020  bne cr6, 0x824ad094
	if !ctx.cr[6].eq {
	pc = 0x824AD094; continue 'dispatch;
	}
	// 824AD078: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AD07C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824AD080: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AD084: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824AD088: 55652834  slwi r5, r11, 5
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AD08C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824AD090: 4BFB7029  bl 0x824640b8
	ctx.lr = 0x824AD094;
	sub_824640B8(ctx, base);
	// 824AD094: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824AD098: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824AD09C: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824AD0A0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824AD0A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AD0A8: 419A0020  beq cr6, 0x824ad0c8
	if ctx.cr[6].eq {
	pc = 0x824AD0C8; continue 'dispatch;
	}
	// 824AD0AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AD0B0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824AD0B4: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 824AD0B8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AD0BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824AD0C0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AD0C4: 4BFB6FF5  bl 0x824640b8
	ctx.lr = 0x824AD0C8;
	sub_824640B8(ctx, base);
	// 824AD0C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AD0CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AD0D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AD0D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AD0D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AD0DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AD0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AD0E8 size=8
    let mut pc: u32 = 0x824AD0E8;
    'dispatch: loop {
        match pc {
            0x824AD0E8 => {
    //   block [0x824AD0E8..0x824AD0F0)
	// 824AD0E8: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 824AD0EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AD0F0 size=84
    let mut pc: u32 = 0x824AD0F0;
    'dispatch: loop {
        match pc {
            0x824AD0F0 => {
    //   block [0x824AD0F0..0x824AD144)
	// 824AD0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AD0F4: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 824AD0F8: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 824AD0FC: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 824AD100: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824AD104: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824AD108: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AD10C: 91440010  stw r10, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824AD110: 91040004  stw r8, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824AD114: 91240014  stw r9, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 824AD118: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 824AD11C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824AD120: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824AD124: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824AD128: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824AD12C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AD130: 396B0033  addi r11, r11, 0x33
	ctx.r[11].s64 = ctx.r[11].s64 + 51;
	// 824AD134: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AD138: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 824AD13C: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824AD140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AD148 size=32
    let mut pc: u32 = 0x824AD148;
    'dispatch: loop {
        match pc {
            0x824AD148 => {
    //   block [0x824AD148..0x824AD168)
	// 824AD148: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 824AD14C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AD150: 419A0018  beq cr6, 0x824ad168
	if ctx.cr[6].eq {
		sub_824AD168(ctx, base);
		return;
	}
	// 824AD154: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 824AD158: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824AD15C: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824AD160: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824AD164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AD168 size=16
    let mut pc: u32 = 0x824AD168;
    'dispatch: loop {
        match pc {
            0x824AD168 => {
    //   block [0x824AD168..0x824AD178)
	// 824AD168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AD16C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AD170: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824AD174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824AD178 size=140
    let mut pc: u32 = 0x824AD178;
    'dispatch: loop {
        match pc {
            0x824AD178 => {
    //   block [0x824AD178..0x824AD204)
	// 824AD178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AD17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AD180: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AD184: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AD188: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824AD18C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AD190: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824AD194: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 824AD198: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 824AD19C: C00B2984  lfs f0, 0x2984(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10628 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AD1A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824AD1A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824AD1A8: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 824AD1AC: C1AB1850  lfs f13, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824AD1B0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AD1B4: 394B0994  addi r10, r11, 0x994
	ctx.r[10].s64 = ctx.r[11].s64 + 2452;
	// 824AD1B8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AD1BC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824AD1C0: C18B0900  lfs f12, 0x900(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2304 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824AD1C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AD1C8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824AD1CC: B13F000C  sth r9, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u16 ) };
	// 824AD1D0: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824AD1D4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 824AD1D8: 911F0020  stw r8, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 824AD1DC: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 824AD1E0: D1BF0028  stfs f13, 0x28(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 824AD1E4: D19F002C  stfs f12, 0x2c(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824AD1E8: 4BFF5CD1  bl 0x824a2eb8
	ctx.lr = 0x824AD1EC;
	sub_824A2EB8(ctx, base);
	// 824AD1EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AD1F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824AD1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AD1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AD1FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AD200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AD208 size=124
    let mut pc: u32 = 0x824AD208;
    'dispatch: loop {
        match pc {
            0x824AD208 => {
    //   block [0x824AD208..0x824AD284)
	// 824AD208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AD20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AD210: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AD214: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AD218: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AD21C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AD220: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 824AD224: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AD228: 396B0994  addi r11, r11, 0x994
	ctx.r[11].s64 = ctx.r[11].s64 + 2452;
	// 824AD22C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824AD230: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 824AD234: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AD238: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AD23C: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824AD240: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AD244: 4BFF5C75  bl 0x824a2eb8
	ctx.lr = 0x824AD248;
	sub_824A2EB8(ctx, base);
	// 824AD248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AD24C: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 824AD250: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824AD254: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AD258: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824AD25C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 824AD260: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 824AD264: 4BFF5C55  bl 0x824a2eb8
	ctx.lr = 0x824AD268;
	sub_824A2EB8(ctx, base);
	// 824AD268: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AD26C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AD270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AD274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AD278: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AD27C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AD280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AD288 size=120
    let mut pc: u32 = 0x824AD288;
    'dispatch: loop {
        match pc {
            0x824AD288 => {
    //   block [0x824AD288..0x824AD300)
	// 824AD288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AD28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AD290: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AD294: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AD298: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AD29C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AD2A0: 396B0994  addi r11, r11, 0x994
	ctx.r[11].s64 = ctx.r[11].s64 + 2452;
	// 824AD2A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AD2A8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824AD2AC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AD2B0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AD2B4: 409A002C  bne cr6, 0x824ad2e0
	if !ctx.cr[6].eq {
	pc = 0x824AD2E0; continue 'dispatch;
	}
	// 824AD2B8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AD2BC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824AD2C0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824AD2C4: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824AD2C8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AD2CC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824AD2D0: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824AD2D4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824AD2D8: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AD2DC: 4BFB6DDD  bl 0x824640b8
	ctx.lr = 0x824AD2E0;
	sub_824640B8(ctx, base);
	// 824AD2E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824AD2E4: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824AD2E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AD2EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824AD2F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AD2F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AD2F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AD2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AD300 size=132
    let mut pc: u32 = 0x824AD300;
    'dispatch: loop {
        match pc {
            0x824AD300 => {
    //   block [0x824AD300..0x824AD384)
	// 824AD300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AD304: 48087DB9  bl 0x825350bc
	ctx.lr = 0x824AD308;
	sub_82535080(ctx, base);
	// 824AD308: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 824AD30C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AD310: 3BE30018  addi r31, r3, 0x18
	ctx.r[31].s64 = ctx.r[3].s64 + 24;
	// 824AD314: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 824AD318: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824AD31C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824AD320: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AD324: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AD328: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824AD32C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824AD330: 409A0010  bne cr6, 0x824ad340
	if !ctx.cr[6].eq {
	pc = 0x824AD340; continue 'dispatch;
	}
	// 824AD334: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 824AD338: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AD33C: 4BFC1015  bl 0x8246e350
	ctx.lr = 0x824AD340;
	sub_8246E350(ctx, base);
	// 824AD340: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AD344: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 824AD348: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AD34C: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824AD350: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 824AD354: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824AD358: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AD35C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824AD360: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824AD388 size=720
    let mut pc: u32 = 0x824AD388;
    'dispatch: loop {
        match pc {
            0x824AD388 => {
    //   block [0x824AD388..0x824AD658)
	// 824AD388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AD38C: 48087D05  bl 0x82535090
	ctx.lr = 0x824AD390;
	sub_82535080(ctx, base);
	// 824AD390: DBA1FF70  stfd f29, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.f[29].u64 ) };
	// 824AD394: DBC1FF78  stfd f30, -0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[30].u64 ) };
	// 824AD398: DBE1FF80  stfd f31, -0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.f[31].u64 ) };
	// 824AD39C: 9421FDC0  stwu r1, -0x240(r1)
	ea = ctx.r[1].u32.wrapping_add(-576 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AD3A0: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 824AD3A4: 7CB42B78  mr r20, r5
	ctx.r[20].u64 = ctx.r[5].u64;
	// 824AD3A8: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 824AD3AC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 824AD3B0: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 824AD3B4: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 824AD3B8: 80B6004C  lwz r5, 0x4c(r22)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(76 as u32) ) } as u64;
	// 824AD3BC: 480448B5  bl 0x824f1c70
	ctx.lr = 0x824AD3C0;
	sub_824F1C70(ctx, base);
	// 824AD3C0: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 824AD3C4: 7ECBB378  mr r11, r22
	ctx.r[11].u64 = ctx.r[22].u64;
	// 824AD3C8: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 824AD3CC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 824AD3D0: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 824AD3D4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824AD3D8: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 824AD3DC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824AD3E0: 4200FFF0  bdnz 0x824ad3d0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824AD3D0; continue 'dispatch;
	}
	// 824AD3E4: 3921012C  addi r9, r1, 0x12c
	ctx.r[9].s64 = ctx.r[1].s64 + 300;
	// 824AD3E8: 81760048  lwz r11, 0x48(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(72 as u32) ) } as u64;
	// 824AD3EC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824AD3F0: 81560030  lwz r10, 0x30(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(48 as u32) ) } as u64;
	// 824AD3F4: 3B2B002C  addi r25, r11, 0x2c
	ctx.r[25].s64 = ctx.r[11].s64 + 44;
	// 824AD3F8: 80740000  lwz r3, 0(r20)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AD3FC: 91210120  stw r9, 0x120(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(288 as u32), ctx.r[9].u32 ) };
	// 824AD400: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 824AD404: 93E10124  stw r31, 0x124(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(292 as u32), ctx.r[31].u32 ) };
	// 824AD408: 61290020  ori r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 | 32;
	// 824AD40C: 93E100C8  stw r31, 0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[31].u32 ) };
	// 824AD410: 93E100CC  stw r31, 0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), ctx.r[31].u32 ) };
	// 824AD414: 91210128  stw r9, 0x128(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(296 as u32), ctx.r[9].u32 ) };
	// 824AD418: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824AD41C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824AD420: 816B0090  lwz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 824AD424: 7FCB5050  subf r30, r11, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 824AD428: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AD42C: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AD430: 3AABFFFF  addi r21, r11, -1
	ctx.r[21].s64 = ctx.r[11].s64 + -1;
	// 824AD434: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AD438: 814B0090  lwz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 824AD43C: 396B00E0  addi r11, r11, 0xe0
	ctx.r[11].s64 = ctx.r[11].s64 + 224;
	// 824AD440: 91210124  stw r9, 0x124(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(292 as u32), ctx.r[9].u32 ) };
	// 824AD444: 916100BC  stw r11, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[11].u32 ) };
	// 824AD448: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824AD44C: C3AB1850  lfs f29, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 824AD450: 7D6AF214  add r11, r10, r30
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 824AD454: D3A100A0  stfs f29, 0xa0(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 824AD458: 916100B4  stw r11, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 824AD45C: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 824AD460: 9161012C  stw r11, 0x12c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(300 as u32), ctx.r[11].u32 ) };
	// 824AD464: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 824AD468: 9161009C  stw r11, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 824AD46C: 48048AE5  bl 0x824f5f50
	ctx.lr = 0x824AD470;
	sub_824F5F50(ctx, base);
	// 824AD470: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 824AD474: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 824AD478: 40990178  ble cr6, 0x824ad5f0
	if !ctx.cr[6].gt {
	pc = 0x824AD5F0; continue 'dispatch;
	}
	// 824AD47C: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824AD480: 3D608283  lis r11, -0x7d7d
	ctx.r[11].s64 = -2105344000;
	// 824AD484: 3B730018  addi r27, r19, 0x18
	ctx.r[27].s64 = ctx.r[19].s64 + 24;
	// 824AD488: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 824AD48C: 7EB7AB78  mr r23, r21
	ctx.r[23].u64 = ctx.r[21].u64;
	// 824AD490: C3CA1FF8  lfs f30, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 824AD494: 3A4B2690  addi r18, r11, 0x2690
	ctx.r[18].s64 = ctx.r[11].s64 + 9872;
	// 824AD498: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 824AD49C: 81410128  lwz r10, 0x128(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(296 as u32) ) } as u64;
	// 824AD4A0: 81210124  lwz r9, 0x124(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 824AD4A4: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 824AD4A8: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 824AD4AC: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824AD4B0: 816100BC  lwz r11, 0xbc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 824AD4B4: 916100B8  stw r11, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 824AD4B8: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AD4BC: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 824AD4C0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AD4C4: 3BAB00D0  addi r29, r11, 0xd0
	ctx.r[29].s64 = ctx.r[11].s64 + 208;
	// 824AD4C8: 814B0090  lwz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 824AD4CC: 7D6AF214  add r11, r10, r30
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 824AD4D0: 7F9E5850  subf r28, r30, r11
	ctx.r[28].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 824AD4D4: 916100B4  stw r11, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 824AD4D8: 409A0010  bne cr6, 0x824ad4e8
	if !ctx.cr[6].eq {
	pc = 0x824AD4E8; continue 'dispatch;
	}
	// 824AD4DC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824AD4E0: 38610120  addi r3, r1, 0x120
	ctx.r[3].s64 = ctx.r[1].s64 + 288;
	// 824AD4E4: 4BFC0E6D  bl 0x8246e350
	ctx.lr = 0x824AD4E8;
	sub_8246E350(ctx, base);
	// 824AD4E8: 81610124  lwz r11, 0x124(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 824AD4EC: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 824AD4F0: 81410120  lwz r10, 0x120(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(288 as u32) ) } as u64;
	// 824AD4F4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AD4F8: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 824AD4FC: 81410124  lwz r10, 0x124(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 824AD500: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AD504: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824AD508: 808100B8  lwz r4, 0xb8(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) } as u64;
	// 824AD50C: 7CBF5A14  add r5, r31, r11
	ctx.r[5].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 824AD510: 91410124  stw r10, 0x124(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(292 as u32), ctx.r[10].u32 ) };
	// 824AD514: 395D0010  addi r10, r29, 0x10
	ctx.r[10].s64 = ctx.r[29].s64 + 16;
	// 824AD518: 914100BC  stw r10, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[10].u32 ) };
	// 824AD51C: 480FE5ED  bl 0x825abb08
	ctx.lr = 0x824AD520;
	sub_825ABB08(ctx, base);
	// 824AD520: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AD524: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 824AD528: 808100BC  lwz r4, 0xbc(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 824AD52C: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 824AD530: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 824AD534: 480FE5D5  bl 0x825abb08
	ctx.lr = 0x824AD538;
	sub_825ABB08(ctx, base);
	// 824AD538: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
	// 824AD53C: 39410110  addi r10, r1, 0x110
	ctx.r[10].s64 = ctx.r[1].s64 + 272;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AD658 size=100
    let mut pc: u32 = 0x824AD658;
    'dispatch: loop {
        match pc {
            0x824AD658 => {
    //   block [0x824AD658..0x824AD6BC)
	// 824AD658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AD65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AD660: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AD664: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AD668: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AD66C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AD670: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824AD674: 4BFFFC15  bl 0x824ad288
	ctx.lr = 0x824AD678;
	sub_824AD288(ctx, base);
	// 824AD678: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824AD67C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AD680: 419A0020  beq cr6, 0x824ad6a0
	if ctx.cr[6].eq {
	pc = 0x824AD6A0; continue 'dispatch;
	}
	// 824AD684: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AD688: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824AD68C: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 824AD690: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AD694: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824AD698: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AD69C: 4BFB6A1D  bl 0x824640b8
	ctx.lr = 0x824AD6A0;
	sub_824640B8(ctx, base);
	// 824AD6A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AD6A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AD6A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AD6AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AD6B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AD6B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AD6B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824AD6C0 size=164
    let mut pc: u32 = 0x824AD6C0;
    'dispatch: loop {
        match pc {
            0x824AD6C0 => {
    //   block [0x824AD6C0..0x824AD764)
	// 824AD6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AD6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AD6C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AD6CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AD6D0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AD6D4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824AD6D8: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 824AD6DC: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 824AD6E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AD6E4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AD6E8: 4BFB6951  bl 0x82464038
	ctx.lr = 0x824AD6EC;
	sub_82464038(ctx, base);
	// 824AD6EC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AD6F0: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 824AD6F4: 396BE2F8  addi r11, r11, -0x1d08
	ctx.r[11].s64 = ctx.r[11].s64 + -7432;
	// 824AD6F8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 824AD6FC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824AD700: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824AD704: B1030004  sth r8, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 824AD708: 394AE30C  addi r10, r10, -0x1cf4
	ctx.r[10].s64 = ctx.r[10].s64 + -7412;
	// 824AD70C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AD710: 3929F0E4  addi r9, r9, -0xf1c
	ctx.r[9].s64 = ctx.r[9].s64 + -3868;
	// 824AD714: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 824AD718: 897F0008  lbz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AD71C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824AD720: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 824AD724: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AD728: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824AD72C: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AD730: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824AD734: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824AD738: C01F0014  lfs f0, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AD73C: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824AD740: C01F0018  lfs f0, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AD744: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824AD748: 897F001C  lbz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824AD74C: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 824AD750: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824AD754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AD758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AD75C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AD760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AD768 size=88
    let mut pc: u32 = 0x824AD768;
    'dispatch: loop {
        match pc {
            0x824AD768 => {
    //   block [0x824AD768..0x824AD7C0)
	// 824AD768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AD76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AD770: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AD774: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AD778: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AD77C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824AD780: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824AD784: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AD788: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AD78C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 824AD790: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AD794: 4E800421  bctrl
	ctx.lr = 0x824AD798;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AD798: 397E0010  addi r11, r30, 0x10
	ctx.r[11].s64 = ctx.r[30].s64 + 16;
	// 824AD79C: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 824AD7A0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824AD7A4: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 824AD7A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AD7AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AD7B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AD7B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AD7B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AD7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AD7C0 size=20
    let mut pc: u32 = 0x824AD7C0;
    'dispatch: loop {
        match pc {
            0x824AD7C0 => {
    //   block [0x824AD7C0..0x824AD7D4)
	// 824AD7C0: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AD7C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AD7C8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824AD7CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AD7D0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824AD7D8 size=8
    let mut pc: u32 = 0x824AD7D8;
    'dispatch: loop {
        match pc {
            0x824AD7D8 => {
    //   block [0x824AD7D8..0x824AD7E0)
	// 824AD7D8: D023001C  stfs f1, 0x1c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 824AD7DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824AD7E0 size=8
    let mut pc: u32 = 0x824AD7E0;
    'dispatch: loop {
        match pc {
            0x824AD7E0 => {
    //   block [0x824AD7E0..0x824AD7E8)
	// 824AD7E0: C023001C  lfs f1, 0x1c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824AD7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AD7E8 size=64
    let mut pc: u32 = 0x824AD7E8;
    'dispatch: loop {
        match pc {
            0x824AD7E8 => {
    //   block [0x824AD7E8..0x824AD828)
	// 824AD7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AD7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AD7F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AD7F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AD7F8: 8084000C  lwz r4, 0xc(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AD7FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AD800: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AD804: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AD808: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AD80C: 4E800421  bctrl
	ctx.lr = 0x824AD810;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AD810: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AD814: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824AD818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AD81C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AD820: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AD824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AD828 size=8
    let mut pc: u32 = 0x824AD828;
    'dispatch: loop {
        match pc {
            0x824AD828 => {
    //   block [0x824AD828..0x824AD830)
	// 824AD828: 3860000D  li r3, 0xd
	ctx.r[3].s64 = 13;
	// 824AD82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AD830 size=8
    let mut pc: u32 = 0x824AD830;
    'dispatch: loop {
        match pc {
            0x824AD830 => {
    //   block [0x824AD830..0x824AD838)
	// 824AD830: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AD834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AD838 size=8
    let mut pc: u32 = 0x824AD838;
    'dispatch: loop {
        match pc {
            0x824AD838 => {
    //   block [0x824AD838..0x824AD840)
	// 824AD838: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AD83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824AD840 size=132
    let mut pc: u32 = 0x824AD840;
    'dispatch: loop {
        match pc {
            0x824AD840 => {
    //   block [0x824AD840..0x824AD8C4)
	// 824AD840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AD844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AD848: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AD84C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AD850: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824AD854: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AD858: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AD85C: 396B09CC  addi r11, r11, 0x9cc
	ctx.r[11].s64 = ctx.r[11].s64 + 2508;
	// 824AD860: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824AD864: C00A20B0  lfs f0, 0x20b0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8368 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AD868: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824AD86C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 824AD870: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 824AD874: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AD878: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824AD87C: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824AD880: B15F0010  sth r10, 0x10(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u16 ) };
	// 824AD884: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 824AD888: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AD88C: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AD890: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824AD894: 419A0010  beq cr6, 0x824ad8a4
	if ctx.cr[6].eq {
	pc = 0x824AD8A4; continue 'dispatch;
	}
	// 824AD898: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 824AD89C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824AD8A0: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824AD8A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824AD8A8: 4BFF5611  bl 0x824a2eb8
	ctx.lr = 0x824AD8AC;
	sub_824A2EB8(ctx, base);
	// 824AD8AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AD8B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824AD8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AD8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AD8BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AD8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AD8C8 size=104
    let mut pc: u32 = 0x824AD8C8;
    'dispatch: loop {
        match pc {
            0x824AD8C8 => {
    //   block [0x824AD8C8..0x824AD930)
	// 824AD8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AD8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AD8D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AD8D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AD8D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AD8DC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AD8E0: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 824AD8E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AD8E8: 396B09CC  addi r11, r11, 0x9cc
	ctx.r[11].s64 = ctx.r[11].s64 + 2508;
	// 824AD8EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824AD8F0: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 824AD8F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AD8F8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AD8FC: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824AD900: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AD904: 4BFF55B5  bl 0x824a2eb8
	ctx.lr = 0x824AD908;
	sub_824A2EB8(ctx, base);
	// 824AD908: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824AD90C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AD910: 4BFF55A9  bl 0x824a2eb8
	ctx.lr = 0x824AD914;
	sub_824A2EB8(ctx, base);
	// 824AD914: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AD918: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AD91C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AD920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AD924: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AD928: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AD92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AD930 size=124
    let mut pc: u32 = 0x824AD930;
    'dispatch: loop {
        match pc {
            0x824AD930 => {
    //   block [0x824AD930..0x824AD9AC)
	// 824AD930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AD934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AD938: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AD93C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AD940: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AD944: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AD948: 396B09CC  addi r11, r11, 0x9cc
	ctx.r[11].s64 = ctx.r[11].s64 + 2508;
	// 824AD94C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AD950: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AD954: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AD958: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AD95C: 419A0030  beq cr6, 0x824ad98c
	if ctx.cr[6].eq {
	pc = 0x824AD98C; continue 'dispatch;
	}
	// 824AD960: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824AD964: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824AD968: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824AD96C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AD970: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824AD974: 409A0018  bne cr6, 0x824ad98c
	if !ctx.cr[6].eq {
	pc = 0x824AD98C; continue 'dispatch;
	}
	// 824AD978: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AD97C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824AD980: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AD984: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AD988: 4E800421  bctrl
	ctx.lr = 0x824AD98C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AD98C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824AD990: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824AD994: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AD998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824AD99C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AD9A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AD9A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AD9A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AD9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824AD9B0 size=184
    let mut pc: u32 = 0x824AD9B0;
    'dispatch: loop {
        match pc {
            0x824AD9B0 => {
    //   block [0x824AD9B0..0x824ADA68)
	// 824AD9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AD9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AD9B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AD9BC: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AD9C0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 824AD9C4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 824AD9C8: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 824AD9CC: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 824AD9D0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 824AD9D4: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 824AD9D8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824AD9DC: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 824AD9E0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824AD9E4: 4200FFF0  bdnz 0x824ad9d4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824AD9D4; continue 'dispatch;
	}
	// 824AD9E8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AD9EC: C003001C  lfs f0, 0x1c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AD9F0: C1A10090  lfs f13, 0x90(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824AD9F4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824AD9F8: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 824AD9FC: D0010090  stfs f0, 0x90(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 824ADA00: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 824ADA04: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ADA08: 816A002C  lwz r11, 0x2c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 824ADA0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824ADA10: 4E800421  bctrl
	ctx.lr = 0x824ADA14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824ADA14: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824ADA18: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 824ADA1C: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ADA20: 2B0B0016  cmplwi cr6, r11, 0x16
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22 as u32, &mut ctx.xer);
	// 824ADA24: 409A0024  bne cr6, 0x824ada48
	if !ctx.cr[6].eq {
	pc = 0x824ADA48; continue 'dispatch;
	}
	// 824ADA28: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824ADA2C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 824ADA30: 480491B1  bl 0x824f6be0
	ctx.lr = 0x824ADA34;
	sub_824F6BE0(ctx, base);
	// 824ADA34: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 824ADA38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824ADA3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824ADA40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824ADA44: 4E800020  blr
	return;
	// 824ADA48: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 824ADA4C: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 824ADA50: 480443C9  bl 0x824f1e18
	ctx.lr = 0x824ADA54;
	sub_824F1E18(ctx, base);
	// 824ADA54: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 824ADA58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824ADA5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824ADA60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824ADA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ADA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824ADA68 size=100
    let mut pc: u32 = 0x824ADA68;
    'dispatch: loop {
        match pc {
            0x824ADA68 => {
    //   block [0x824ADA68..0x824ADACC)
	// 824ADA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824ADA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824ADA70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824ADA74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824ADA78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824ADA7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824ADA80: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824ADA84: 4BFFFEAD  bl 0x824ad930
	ctx.lr = 0x824ADA88;
	sub_824AD930(ctx, base);
	// 824ADA88: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824ADA8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824ADA90: 419A0020  beq cr6, 0x824adab0
	if ctx.cr[6].eq {
	pc = 0x824ADAB0; continue 'dispatch;
	}
	// 824ADA94: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ADA98: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824ADA9C: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 824ADAA0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824ADAA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824ADAA8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824ADAAC: 4BFB660D  bl 0x824640b8
	ctx.lr = 0x824ADAB0;
	sub_824640B8(ctx, base);
	// 824ADAB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824ADAB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824ADAB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824ADABC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824ADAC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824ADAC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824ADAC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ADAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824ADAD0 size=72
    let mut pc: u32 = 0x824ADAD0;
    'dispatch: loop {
        match pc {
            0x824ADAD0 => {
    //   block [0x824ADAD0..0x824ADB18)
	// 824ADAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824ADAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824ADAD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824ADADC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824ADAE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824ADAE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824ADAE8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824ADAEC: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 824ADAF0: 480FDB11  bl 0x825ab600
	ctx.lr = 0x824ADAF4;
	sub_825AB600(ctx, base);
	// 824ADAF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824ADAF8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 824ADAFC: 480FDB05  bl 0x825ab600
	ctx.lr = 0x824ADB00;
	sub_825AB600(ctx, base);
	// 824ADB00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824ADB04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824ADB08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824ADB0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824ADB10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824ADB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ADB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824ADB18 size=16
    let mut pc: u32 = 0x824ADB18;
    'dispatch: loop {
        match pc {
            0x824ADB18 => {
    //   block [0x824ADB18..0x824ADB28)
	// 824ADB18: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 824ADB1C: 38800074  li r4, 0x74
	ctx.r[4].s64 = 116;
	// 824ADB20: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 824ADB24: 4BFF4E8C  b 0x824a29b0
	sub_824A29B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ADB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824ADB28 size=32
    let mut pc: u32 = 0x824ADB28;
    'dispatch: loop {
        match pc {
            0x824ADB28 => {
    //   block [0x824ADB28..0x824ADB48)
	// 824ADB28: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 824ADB2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824ADB30: 419A0018  beq cr6, 0x824adb48
	if ctx.cr[6].eq {
		sub_824ADB48(ctx, base);
		return;
	}
	// 824ADB34: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 824ADB38: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 824ADB3C: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824ADB40: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824ADB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ADB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824ADB48 size=16
    let mut pc: u32 = 0x824ADB48;
    'dispatch: loop {
        match pc {
            0x824ADB48 => {
    //   block [0x824ADB48..0x824ADB58)
	// 824ADB48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824ADB4C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824ADB50: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824ADB54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ADB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824ADB58 size=140
    let mut pc: u32 = 0x824ADB58;
    'dispatch: loop {
        match pc {
            0x824ADB58 => {
    //   block [0x824ADB58..0x824ADBE4)
	// 824ADB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824ADB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824ADB60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824ADB64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824ADB68: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 824ADB6C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824ADB70: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824ADB74: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824ADB78: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824ADB7C: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 824ADB80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824ADB84: C3EB2238  lfs f31, 0x2238(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8760 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824ADB88: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824ADB8C: 480FDBD5  bl 0x825ab760
	ctx.lr = 0x824ADB90;
	sub_825AB760(ctx, base);
	// 824ADB90: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ADB94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824ADB98: 419A0024  beq cr6, 0x824adbbc
	if ctx.cr[6].eq {
	pc = 0x824ADBBC; continue 'dispatch;
	}
	// 824ADB9C: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 824ADBA0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824ADBA4: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 824ADBA8: 480FDBB9  bl 0x825ab760
	ctx.lr = 0x824ADBAC;
	sub_825AB760(ctx, base);
	// 824ADBAC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ADBB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824ADBB4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824ADBB8: 409A0008  bne cr6, 0x824adbc0
	if !ctx.cr[6].eq {
	pc = 0x824ADBC0; continue 'dispatch;
	}
	// 824ADBBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824ADBC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824ADBC4: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824ADBC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824ADBCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824ADBD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824ADBD4: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 824ADBD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824ADBDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824ADBE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ADBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824ADBE8 size=8
    let mut pc: u32 = 0x824ADBE8;
    'dispatch: loop {
        match pc {
            0x824ADBE8 => {
    //   block [0x824ADBE8..0x824ADBF0)
	// 824ADBE8: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 824ADBEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ADBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824ADBF0 size=164
    let mut pc: u32 = 0x824ADBF0;
    'dispatch: loop {
        match pc {
            0x824ADBF0 => {
    //   block [0x824ADBF0..0x824ADC94)
	// 824ADBF0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 824ADBF4: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
	// 824ADBF8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 824ADBFC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824ADC00: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 824ADC04: 3929F1A4  addi r9, r9, -0xe5c
	ctx.r[9].s64 = ctx.r[9].s64 + -3676;
	// 824ADC08: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ADC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824ADC98 size=92
    let mut pc: u32 = 0x824ADC98;
    'dispatch: loop {
        match pc {
            0x824ADC98 => {
    //   block [0x824ADC98..0x824ADCF4)
	// 824ADC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824ADC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824ADCA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824ADCA4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824ADCA8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824ADCAC: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 824ADCB0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ADCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824ADCF8 size=72
    let mut pc: u32 = 0x824ADCF8;
    'dispatch: loop {
        match pc {
            0x824ADCF8 => {
    //   block [0x824ADCF8..0x824ADD40)
	// 824ADCF8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824ADCFC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824ADD00: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824ADD04: 396BF214  addi r11, r11, -0xdec
	ctx.r[11].s64 = ctx.r[11].s64 + -3564;
	// 824ADD08: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 824ADD0C: C00A0858  lfs f0, 0x858(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824ADD10: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824ADD14: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 824ADD18: C1AA24B0  lfs f13, 0x24b0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(9392 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824ADD1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824ADD20: 99430008  stb r10, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 824ADD24: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824ADD28: 90830018  stw r4, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 824ADD2C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824ADD30: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824ADD34: 99030008  stb r8, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u8 ) };
	// 824ADD38: 90A30014  stw r5, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 824ADD3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ADD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824ADD40 size=180
    let mut pc: u32 = 0x824ADD40;
    'dispatch: loop {
        match pc {
            0x824ADD40 => {
    //   block [0x824ADD40..0x824ADDF4)
	// 824ADD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824ADD44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824ADD48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824ADD4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824ADD50: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ADD54: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824ADD58: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 824ADD5C: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 824ADD60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824ADD64: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824ADD68: 4BFB62D1  bl 0x82464038
	ctx.lr = 0x824ADD6C;
	sub_82464038(ctx, base);
	// 824ADD6C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824ADD70: 39000028  li r8, 0x28
	ctx.r[8].s64 = 40;
	// 824ADD74: 396BE2F8  addi r11, r11, -0x1d08
	ctx.r[11].s64 = ctx.r[11].s64 + -7432;
	// 824ADD78: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 824ADD7C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824ADD80: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824ADD84: B1030004  sth r8, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 824ADD88: 394AE30C  addi r10, r10, -0x1cf4
	ctx.r[10].s64 = ctx.r[10].s64 + -7412;
	// 824ADD8C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824ADD90: 3929F214  addi r9, r9, -0xdec
	ctx.r[9].s64 = ctx.r[9].s64 + -3564;
	// 824ADD94: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 824ADD98: 897F0008  lbz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824ADD9C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824ADDA0: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 824ADDA4: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824ADDA8: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824ADDAC: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824ADDB0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824ADDB4: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824ADDB8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824ADDBC: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 824ADDC0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824ADDC4: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824ADDC8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824ADDCC: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 824ADDD0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824ADDD4: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 824ADDD8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 824ADDDC: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 824ADDE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824ADDE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824ADDE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824ADDEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824ADDF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ADDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824ADDF8 size=92
    let mut pc: u32 = 0x824ADDF8;
    'dispatch: loop {
        match pc {
            0x824ADDF8 => {
    //   block [0x824ADDF8..0x824ADE54)
	// 824ADDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824ADDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824ADE00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824ADE04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824ADE08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824ADE0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824ADE10: 549E103A  slwi r30, r4, 2
	ctx.r[30].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 824ADE14: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824ADE18: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824ADE1C: 4BFEDC35  bl 0x8249ba50
	ctx.lr = 0x824ADE20;
	sub_8249BA50(ctx, base);
	// 824ADE20: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824ADE24: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824ADE28: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824ADE2C: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824ADE30: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 824ADE34: 7D49582E  lwzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824ADE38: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 824ADE3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824ADE40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824ADE44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824ADE48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824ADE4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824ADE50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ADE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824ADE58 size=92
    let mut pc: u32 = 0x824ADE58;
    'dispatch: loop {
        match pc {
            0x824ADE58 => {
    //   block [0x824ADE58..0x824ADEB4)
	// 824ADE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824ADE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824ADE60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824ADE64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824ADE68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824ADE6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824ADE70: 549E103A  slwi r30, r4, 2
	ctx.r[30].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 824ADE74: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 824ADE78: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824ADE7C: 4BFEDBD5  bl 0x8249ba50
	ctx.lr = 0x824ADE80;
	sub_8249BA50(ctx, base);
	// 824ADE80: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 824ADE84: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 824ADE88: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824ADE8C: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824ADE90: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 824ADE94: 7D49582E  lwzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824ADE98: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 824ADE9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824ADEA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824ADEA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824ADEA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824ADEAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824ADEB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ADEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824ADEB8 size=144
    let mut pc: u32 = 0x824ADEB8;
    'dispatch: loop {
        match pc {
            0x824ADEB8 => {
    //   block [0x824ADEB8..0x824ADF48)
	// 824ADEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824ADEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824ADEC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824ADEC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824ADEC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824ADECC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824ADED0: 549E103A  slwi r30, r4, 2
	ctx.r[30].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 824ADED4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824ADED8: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824ADEDC: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824ADEE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824ADEE4: 419A0030  beq cr6, 0x824adf14
	if ctx.cr[6].eq {
	pc = 0x824ADF14; continue 'dispatch;
	}
	// 824ADEE8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824ADEEC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824ADEF0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824ADEF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824ADEF8: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824ADEFC: 409A0018  bne cr6, 0x824adf14
	if !ctx.cr[6].eq {
	pc = 0x824ADF14; continue 'dispatch;
	}
	// 824ADF00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ADF04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824ADF08: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ADF0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824ADF10: 4E800421  bctrl
	ctx.lr = 0x824ADF14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824ADF14: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824ADF18: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824ADF1C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824ADF20: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824ADF24: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 824ADF28: 7D49582E  lwzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824ADF2C: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 824ADF30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824ADF34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824ADF38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824ADF3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824ADF40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824ADF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ADF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824ADF48 size=144
    let mut pc: u32 = 0x824ADF48;
    'dispatch: loop {
        match pc {
            0x824ADF48 => {
    //   block [0x824ADF48..0x824ADFD8)
	// 824ADF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824ADF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824ADF50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824ADF54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824ADF58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824ADF5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824ADF60: 549E103A  slwi r30, r4, 2
	ctx.r[30].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 824ADF64: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824ADF68: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824ADF6C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824ADF70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824ADF74: 419A0030  beq cr6, 0x824adfa4
	if ctx.cr[6].eq {
	pc = 0x824ADFA4; continue 'dispatch;
	}
	// 824ADF78: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824ADF7C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824ADF80: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824ADF84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824ADF88: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824ADF8C: 409A0018  bne cr6, 0x824adfa4
	if !ctx.cr[6].eq {
	pc = 0x824ADFA4; continue 'dispatch;
	}
	// 824ADF90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ADF94: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824ADF98: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824ADF9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824ADFA0: 4E800421  bctrl
	ctx.lr = 0x824ADFA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824ADFA4: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 824ADFA8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824ADFAC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824ADFB0: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824ADFB4: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 824ADFB8: 7D49582E  lwzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824ADFBC: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 824ADFC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824ADFC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824ADFC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824ADFCC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824ADFD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824ADFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824ADFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824ADFD8 size=92
    let mut pc: u32 = 0x824ADFD8;
    'dispatch: loop {
        match pc {
            0x824ADFD8 => {
    //   block [0x824ADFD8..0x824AE034)
	// 824ADFD8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824ADFDC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 824ADFE0: 392BF2F4  addi r9, r11, -0xd0c
	ctx.r[9].s64 = ctx.r[11].s64 + -3340;
	// 824ADFE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824ADFE8: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 824ADFEC: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 824ADFF0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824ADFF4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824ADFF8: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824ADFFC: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824AE000: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 824AE004: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824AE008: 9143001C  stw r10, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 824AE00C: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 824AE010: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 824AE014: 91430028  stw r10, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 824AE018: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 824AE01C: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 824AE020: 91430034  stw r10, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 824AE024: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 824AE028: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 824AE02C: 99030040  stb r8, 0x40(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[8].u8 ) };
	// 824AE030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AE038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AE038 size=568
    let mut pc: u32 = 0x824AE038;
    'dispatch: loop {
        match pc {
            0x824AE038 => {
    //   block [0x824AE038..0x824AE270)
	// 824AE038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AE03C: 48087081  bl 0x825350bc
	ctx.lr = 0x824AE040;
	sub_82535080(ctx, base);
	// 824AE040: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AE044: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824AE048: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824AE04C: 394AF2F4  addi r10, r10, -0xd0c
	ctx.r[10].s64 = ctx.r[10].s64 + -3340;
	// 824AE050: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AE054: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AE058: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824AE05C: 40990034  ble cr6, 0x824ae090
	if !ctx.cr[6].gt {
	pc = 0x824AE090; continue 'dispatch;
	}
	// 824AE060: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824AE064: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 824AE068: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE06C: 7D5F582E  lwzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AE070: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824AE074: 419A000C  beq cr6, 0x824ae080
	if ctx.cr[6].eq {
	pc = 0x824AE080; continue 'dispatch;
	}
	// 824AE078: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824AE07C: 4BFED9D5  bl 0x8249ba50
	ctx.lr = 0x824AE080;
	sub_8249BA50(ctx, base);
	// 824AE080: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 824AE084: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824AE088: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824AE08C: 409AFFDC  bne cr6, 0x824ae068
	if !ctx.cr[6].eq {
	pc = 0x824AE068; continue 'dispatch;
	}
	// 824AE090: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 824AE094: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AE098: 40990034  ble cr6, 0x824ae0cc
	if !ctx.cr[6].gt {
	pc = 0x824AE0CC; continue 'dispatch;
	}
	// 824AE09C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824AE0A0: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 824AE0A4: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 824AE0A8: 7D5F582E  lwzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AE0AC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824AE0B0: 419A000C  beq cr6, 0x824ae0bc
	if ctx.cr[6].eq {
	pc = 0x824AE0BC; continue 'dispatch;
	}
	// 824AE0B4: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824AE0B8: 4BFED999  bl 0x8249ba50
	ctx.lr = 0x824AE0BC;
	sub_8249BA50(ctx, base);
	// 824AE0BC: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 824AE0C0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824AE0C4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824AE0C8: 409AFFDC  bne cr6, 0x824ae0a4
	if !ctx.cr[6].eq {
	pc = 0x824AE0A4; continue 'dispatch;
	}
	// 824AE0CC: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 824AE0D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AE0D4: 40990068  ble cr6, 0x824ae13c
	if !ctx.cr[6].gt {
	pc = 0x824AE13C; continue 'dispatch;
	}
	// 824AE0D8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824AE0DC: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 824AE0E0: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 824AE0E4: 7D5F582E  lwzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AE0E8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824AE0EC: 419A0040  beq cr6, 0x824ae12c
	if ctx.cr[6].eq {
	pc = 0x824AE12C; continue 'dispatch;
	}
	// 824AE0F0: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824AE0F4: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE0F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AE0FC: 419A0030  beq cr6, 0x824ae12c
	if ctx.cr[6].eq {
	pc = 0x824AE12C; continue 'dispatch;
	}
	// 824AE100: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824AE104: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824AE108: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824AE10C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AE110: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824AE114: 409A0018  bne cr6, 0x824ae12c
	if !ctx.cr[6].eq {
	pc = 0x824AE12C; continue 'dispatch;
	}
	// 824AE118: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE11C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824AE120: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE124: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AE128: 4E800421  bctrl
	ctx.lr = 0x824AE12C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AE12C: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 824AE130: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824AE134: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824AE138: 409AFFA8  bne cr6, 0x824ae0e0
	if !ctx.cr[6].eq {
	pc = 0x824AE0E0; continue 'dispatch;
	}
	// 824AE13C: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 824AE140: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AE144: 40990068  ble cr6, 0x824ae1ac
	if !ctx.cr[6].gt {
	pc = 0x824AE1AC; continue 'dispatch;
	}
	// 824AE148: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824AE14C: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 824AE150: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 824AE154: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824AE158: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824AE15C: 419A0040  beq cr6, 0x824ae19c
	if ctx.cr[6].eq {
	pc = 0x824AE19C; continue 'dispatch;
	}
	// 824AE160: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824AE164: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE168: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AE16C: 419A0030  beq cr6, 0x824ae19c
	if ctx.cr[6].eq {
	pc = 0x824AE19C; continue 'dispatch;
	}
	// 824AE170: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824AE174: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824AE178: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824AE17C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AE180: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824AE184: 409A0018  bne cr6, 0x824ae19c
	if !ctx.cr[6].eq {
	pc = 0x824AE19C; continue 'dispatch;
	}
	// 824AE188: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE18C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824AE190: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE194: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AE198: 4E800421  bctrl
	ctx.lr = 0x824AE19C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AE19C: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 824AE1A0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824AE1A4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824AE1A8: 409AFFA8  bne cr6, 0x824ae150
	if !ctx.cr[6].eq {
	pc = 0x824AE150; continue 'dispatch;
	}
	// 824AE1AC: 817D0034  lwz r11, 0x34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 824AE1B0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AE1B4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AE1B8: 409A0020  bne cr6, 0x824ae1d8
	if !ctx.cr[6].eq {
	pc = 0x824AE1D8; continue 'dispatch;
	}
	// 824AE1BC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE1C0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824AE1C4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AE1C8: 809D002C  lwz r4, 0x2c(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 824AE1CC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AE1D0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824AE1D4: 4BFB5EE5  bl 0x824640b8
	ctx.lr = 0x824AE1D8;
	sub_824640B8(ctx, base);
	// 824AE1D8: 817D0028  lwz r11, 0x28(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 824AE1DC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AE1E0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AE1E4: 409A0020  bne cr6, 0x824ae204
	if !ctx.cr[6].eq {
	pc = 0x824AE204; continue 'dispatch;
	}
	// 824AE1E8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE1EC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824AE1F0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AE1F4: 809D0020  lwz r4, 0x20(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 824AE1F8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AE1FC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824AE200: 4BFB5EB9  bl 0x824640b8
	ctx.lr = 0x824AE204;
	sub_824640B8(ctx, base);
	// 824AE204: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 824AE208: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AE20C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AE210: 409A0020  bne cr6, 0x824ae230
	if !ctx.cr[6].eq {
	pc = 0x824AE230; continue 'dispatch;
	}
	// 824AE214: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE218: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824AE21C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AE220: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 824AE224: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AE228: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824AE22C: 4BFB5E8D  bl 0x824640b8
	ctx.lr = 0x824AE230;
	sub_824640B8(ctx, base);
	// 824AE230: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 824AE234: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AE238: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AE23C: 409A0020  bne cr6, 0x824ae25c
	if !ctx.cr[6].eq {
	pc = 0x824AE25C; continue 'dispatch;
	}
	// 824AE240: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE244: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824AE248: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AE24C: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE250: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AE254: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824AE258: 4BFB5E61  bl 0x824640b8
	ctx.lr = 0x824AE25C;
	sub_824640B8(ctx, base);
	// 824AE25C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824AE260: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824AE264: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AE268: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AE26C: 48086EA0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AE270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AE270 size=740
    let mut pc: u32 = 0x824AE270;
    'dispatch: loop {
        match pc {
            0x824AE270 => {
    //   block [0x824AE270..0x824AE554)
	// 824AE270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AE274: 48086E45  bl 0x825350b8
	ctx.lr = 0x824AE278;
	sub_82535080(ctx, base);
	// 824AE278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AE27C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824AE280: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824AE284: 3BFD0008  addi r31, r29, 8
	ctx.r[31].s64 = ctx.r[29].s64 + 8;
	// 824AE288: 3BDC0008  addi r30, r28, 8
	ctx.r[30].s64 = ctx.r[28].s64 + 8;
	// 824AE28C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE290: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE294: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824AE298: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824AE29C: 40980060  bge cr6, 0x824ae2fc
	if !ctx.cr[6].lt {
	pc = 0x824AE2FC; continue 'dispatch;
	}
	// 824AE2A0: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AE2A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AE2A8: 409A0020  bne cr6, 0x824ae2c8
	if !ctx.cr[6].eq {
	pc = 0x824AE2C8; continue 'dispatch;
	}
	// 824AE2AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE2B0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824AE2B4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AE2B8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE2BC: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AE2C0: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AE2C4: 4BFB5DF5  bl 0x824640b8
	ctx.lr = 0x824AE2C8;
	sub_824640B8(ctx, base);
	// 824AE2C8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE2CC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824AE2D0: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE2D4: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 824AE2D8: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824AE2DC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AE2E0: 4BFB5D59  bl 0x82464038
	ctx.lr = 0x824AE2E4;
	sub_82464038(ctx, base);
	// 824AE2E4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE2E8: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824AE2EC: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE2F0: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AE2F4: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 824AE2F8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824AE2FC: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE300: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE304: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AE308: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824AE30C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE310: 40990020  ble cr6, 0x824ae330
	if !ctx.cr[6].gt {
	pc = 0x824AE330; continue 'dispatch;
	}
	// 824AE314: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 824AE318: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AE31C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824AE320: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824AE324: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824AE328: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824AE32C: 409AFFEC  bne cr6, 0x824ae318
	if !ctx.cr[6].eq {
	pc = 0x824AE318; continue 'dispatch;
	}
	// 824AE330: 3BFD002C  addi r31, r29, 0x2c
	ctx.r[31].s64 = ctx.r[29].s64 + 44;
	// 824AE334: 3BDC002C  addi r30, r28, 0x2c
	ctx.r[30].s64 = ctx.r[28].s64 + 44;
	// 824AE338: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE33C: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE340: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824AE344: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824AE348: 40980060  bge cr6, 0x824ae3a8
	if !ctx.cr[6].lt {
	pc = 0x824AE3A8; continue 'dispatch;
	}
	// 824AE34C: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AE350: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AE354: 409A0020  bne cr6, 0x824ae374
	if !ctx.cr[6].eq {
	pc = 0x824AE374; continue 'dispatch;
	}
	// 824AE358: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE35C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824AE360: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AE364: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE368: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AE36C: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AE370: 4BFB5D49  bl 0x824640b8
	ctx.lr = 0x824AE374;
	sub_824640B8(ctx, base);
	// 824AE374: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE378: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824AE37C: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE380: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 824AE384: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824AE388: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AE38C: 4BFB5CAD  bl 0x82464038
	ctx.lr = 0x824AE390;
	sub_82464038(ctx, base);
	// 824AE390: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE394: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824AE398: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE39C: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AE3A0: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 824AE3A4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824AE3A8: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE3AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE3B0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AE3B4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824AE3B8: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE3BC: 40990020  ble cr6, 0x824ae3dc
	if !ctx.cr[6].gt {
	pc = 0x824AE3DC; continue 'dispatch;
	}
	// 824AE3C0: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 824AE3C4: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AE3C8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824AE3CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824AE3D0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824AE3D4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824AE3D8: 409AFFEC  bne cr6, 0x824ae3c4
	if !ctx.cr[6].eq {
	pc = 0x824AE3C4; continue 'dispatch;
	}
	// 824AE3DC: 3BFD0014  addi r31, r29, 0x14
	ctx.r[31].s64 = ctx.r[29].s64 + 20;
	// 824AE3E0: 3BDC0014  addi r30, r28, 0x14
	ctx.r[30].s64 = ctx.r[28].s64 + 20;
	// 824AE3E4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE3E8: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE3EC: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824AE3F0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824AE3F4: 40980060  bge cr6, 0x824ae454
	if !ctx.cr[6].lt {
	pc = 0x824AE454; continue 'dispatch;
	}
	// 824AE3F8: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AE3FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AE400: 409A0020  bne cr6, 0x824ae420
	if !ctx.cr[6].eq {
	pc = 0x824AE420; continue 'dispatch;
	}
	// 824AE404: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE408: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824AE40C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AE410: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE414: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AE418: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AE41C: 4BFB5C9D  bl 0x824640b8
	ctx.lr = 0x824AE420;
	sub_824640B8(ctx, base);
	// 824AE420: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE424: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824AE428: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE42C: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 824AE430: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824AE434: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AE438: 4BFB5C01  bl 0x82464038
	ctx.lr = 0x824AE43C;
	sub_82464038(ctx, base);
	// 824AE43C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE440: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824AE444: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE448: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AE44C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 824AE450: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824AE454: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE458: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE45C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AE460: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824AE464: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE468: 40990020  ble cr6, 0x824ae488
	if !ctx.cr[6].gt {
	pc = 0x824AE488; continue 'dispatch;
	}
	// 824AE46C: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 824AE470: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AE474: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824AE478: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824AE47C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824AE480: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824AE484: 409AFFEC  bne cr6, 0x824ae470
	if !ctx.cr[6].eq {
	pc = 0x824AE470; continue 'dispatch;
	}
	// 824AE488: 3BFD0020  addi r31, r29, 0x20
	ctx.r[31].s64 = ctx.r[29].s64 + 32;
	// 824AE48C: 3BDC0020  addi r30, r28, 0x20
	ctx.r[30].s64 = ctx.r[28].s64 + 32;
	// 824AE490: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE494: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE498: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824AE49C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824AE4A0: 40980060  bge cr6, 0x824ae500
	if !ctx.cr[6].lt {
	pc = 0x824AE500; continue 'dispatch;
	}
	// 824AE4A4: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AE4A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AE4AC: 409A0020  bne cr6, 0x824ae4cc
	if !ctx.cr[6].eq {
	pc = 0x824AE4CC; continue 'dispatch;
	}
	// 824AE4B0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE4B4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824AE4B8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AE4BC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE4C0: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AE4C4: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AE4C8: 4BFB5BF1  bl 0x824640b8
	ctx.lr = 0x824AE4CC;
	sub_824640B8(ctx, base);
	// 824AE4CC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE4D0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824AE4D4: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE4D8: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 824AE4DC: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824AE4E0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AE4E4: 4BFB5B55  bl 0x82464038
	ctx.lr = 0x824AE4E8;
	sub_82464038(ctx, base);
	// 824AE4E8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE4EC: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824AE4F0: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE4F4: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AE4F8: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 824AE4FC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824AE500: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE504: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE508: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AE50C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824AE510: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE514: 40990020  ble cr6, 0x824ae534
	if !ctx.cr[6].gt {
	pc = 0x824AE534; continue 'dispatch;
	}
	// 824AE518: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 824AE51C: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AE520: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824AE524: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824AE528: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824AE52C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824AE530: 409AFFEC  bne cr6, 0x824ae51c
	if !ctx.cr[6].eq {
	pc = 0x824AE51C; continue 'dispatch;
	}
	// 824AE534: 817C0038  lwz r11, 0x38(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(56 as u32) ) } as u64;
	// 824AE538: 917D0038  stw r11, 0x38(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 824AE53C: 817C003C  lwz r11, 0x3c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(60 as u32) ) } as u64;
	// 824AE540: 917D003C  stw r11, 0x3c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 824AE544: 897C0040  lbz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 824AE548: 997D0040  stb r11, 0x40(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 824AE54C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824AE550: 48086BB8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AE558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AE558 size=132
    let mut pc: u32 = 0x824AE558;
    'dispatch: loop {
        match pc {
            0x824AE558 => {
    //   block [0x824AE558..0x824AE5DC)
	// 824AE558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AE55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AE560: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AE564: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AE568: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AE56C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824AE570: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AE574: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824AE578: 419A004C  beq cr6, 0x824ae5c4
	if ctx.cr[6].eq {
	pc = 0x824AE5C4; continue 'dispatch;
	}
	// 824AE57C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AE580: 4BFED4B1  bl 0x8249ba30
	ctx.lr = 0x824AE584;
	sub_8249BA30(ctx, base);
	// 824AE584: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 824AE588: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE58C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE590: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824AE594: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824AE598: 409A0010  bne cr6, 0x824ae5a8
	if !ctx.cr[6].eq {
	pc = 0x824AE5A8; continue 'dispatch;
	}
	// 824AE59C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824AE5A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AE5A4: 4BFBFDAD  bl 0x8246e350
	ctx.lr = 0x824AE5A8;
	sub_8246E350(ctx, base);
	// 824AE5A8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE5AC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE5B0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AE5B4: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 824AE5B8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE5BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824AE5C0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824AE5C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AE5C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AE5CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AE5D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AE5D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AE5D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AE5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AE5E0 size=132
    let mut pc: u32 = 0x824AE5E0;
    'dispatch: loop {
        match pc {
            0x824AE5E0 => {
    //   block [0x824AE5E0..0x824AE664)
	// 824AE5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AE5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AE5E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AE5EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AE5F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AE5F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824AE5F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AE5FC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824AE600: 419A004C  beq cr6, 0x824ae64c
	if ctx.cr[6].eq {
	pc = 0x824AE64C; continue 'dispatch;
	}
	// 824AE604: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AE608: 4BFED429  bl 0x8249ba30
	ctx.lr = 0x824AE60C;
	sub_8249BA30(ctx, base);
	// 824AE60C: 3BFF002C  addi r31, r31, 0x2c
	ctx.r[31].s64 = ctx.r[31].s64 + 44;
	// 824AE610: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE614: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE618: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824AE61C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824AE620: 409A0010  bne cr6, 0x824ae630
	if !ctx.cr[6].eq {
	pc = 0x824AE630; continue 'dispatch;
	}
	// 824AE624: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824AE628: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AE62C: 4BFBFD25  bl 0x8246e350
	ctx.lr = 0x824AE630;
	sub_8246E350(ctx, base);
	// 824AE630: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE634: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE638: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AE63C: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 824AE640: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE644: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824AE648: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824AE64C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AE650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AE654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AE658: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AE65C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AE660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AE668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AE668 size=144
    let mut pc: u32 = 0x824AE668;
    'dispatch: loop {
        match pc {
            0x824AE668 => {
    //   block [0x824AE668..0x824AE6F8)
	// 824AE668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AE66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AE670: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AE674: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AE678: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AE67C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824AE680: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824AE684: 419A005C  beq cr6, 0x824ae6e0
	if ctx.cr[6].eq {
	pc = 0x824AE6E0; continue 'dispatch;
	}
	// 824AE688: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE68C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AE690: 419A0010  beq cr6, 0x824ae6a0
	if ctx.cr[6].eq {
	pc = 0x824AE6A0; continue 'dispatch;
	}
	// 824AE694: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 824AE698: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824AE69C: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824AE6A0: 3BE30014  addi r31, r3, 0x14
	ctx.r[31].s64 = ctx.r[3].s64 + 20;
	// 824AE6A4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE6A8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE6AC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824AE6B0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824AE6B4: 409A0010  bne cr6, 0x824ae6c4
	if !ctx.cr[6].eq {
	pc = 0x824AE6C4; continue 'dispatch;
	}
	// 824AE6B8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824AE6BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AE6C0: 4BFBFC91  bl 0x8246e350
	ctx.lr = 0x824AE6C4;
	sub_8246E350(ctx, base);
	// 824AE6C4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE6C8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE6CC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AE6D0: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 824AE6D4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE6D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824AE6DC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824AE6E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AE6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AE6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AE6EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AE6F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AE6F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AE6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AE6F8 size=144
    let mut pc: u32 = 0x824AE6F8;
    'dispatch: loop {
        match pc {
            0x824AE6F8 => {
    //   block [0x824AE6F8..0x824AE788)
	// 824AE6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AE6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AE700: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AE704: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AE708: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AE70C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824AE710: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824AE714: 419A005C  beq cr6, 0x824ae770
	if ctx.cr[6].eq {
	pc = 0x824AE770; continue 'dispatch;
	}
	// 824AE718: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE71C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AE720: 419A0010  beq cr6, 0x824ae730
	if ctx.cr[6].eq {
	pc = 0x824AE730; continue 'dispatch;
	}
	// 824AE724: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 824AE728: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824AE72C: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824AE730: 3BE30020  addi r31, r3, 0x20
	ctx.r[31].s64 = ctx.r[3].s64 + 32;
	// 824AE734: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE738: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE73C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824AE740: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824AE744: 409A0010  bne cr6, 0x824ae754
	if !ctx.cr[6].eq {
	pc = 0x824AE754; continue 'dispatch;
	}
	// 824AE748: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824AE74C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824AE750: 4BFBFC01  bl 0x8246e350
	ctx.lr = 0x824AE754;
	sub_8246E350(ctx, base);
	// 824AE754: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE758: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE75C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AE760: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 824AE764: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AE768: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824AE76C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824AE770: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AE774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AE778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AE77C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AE780: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AE784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AE788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AE788 size=1440
    let mut pc: u32 = 0x824AE788;
    'dispatch: loop {
        match pc {
            0x824AE788 => {
    //   block [0x824AE788..0x824AED28)
	// 824AE788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AE78C: 48086909  bl 0x82535094
	ctx.lr = 0x824AE790;
	sub_82535080(ctx, base);
	// 824AE790: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AE794: 826D0000  lwz r19, 0(r13)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE798: 3A800010  li r20, 0x10
	ctx.r[20].s64 = 16;
	// 824AE79C: 38A0002F  li r5, 0x2f
	ctx.r[5].s64 = 47;
	// 824AE7A0: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 824AE7A4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824AE7A8: 7C74982E  lwzx r3, r20, r19
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 824AE7AC: 4BFB588D  bl 0x82464038
	ctx.lr = 0x824AE7B0;
	sub_82464038(ctx, base);
	// 824AE7B0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AE7B4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 824AE7B8: 396BF2F4  addi r11, r11, -0xd0c
	ctx.r[11].s64 = ctx.r[11].s64 + -3340;
	// 824AE7BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824AE7C0: 39200044  li r9, 0x44
	ctx.r[9].s64 = 68;
	// 824AE7C4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 824AE7C8: 3EC08000  lis r22, -0x8000
	ctx.r[22].s64 = -2147483648;
	// 824AE7CC: 3B190008  addi r24, r25, 8
	ctx.r[24].s64 = ctx.r[25].s64 + 8;
	// 824AE7D0: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AE7D4: 3B990014  addi r28, r25, 0x14
	ctx.r[28].s64 = ctx.r[25].s64 + 20;
	// 824AE7D8: B1590006  sth r10, 6(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824AE7DC: 3AB90020  addi r21, r25, 0x20
	ctx.r[21].s64 = ctx.r[25].s64 + 32;
	// 824AE7E0: B1390004  sth r9, 4(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 824AE7E4: 3AF9002C  addi r23, r25, 0x2c
	ctx.r[23].s64 = ctx.r[25].s64 + 44;
	// 824AE7E8: 93580000  stw r26, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 824AE7EC: 93580004  stw r26, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 824AE7F0: 92D80008  stw r22, 8(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 824AE7F4: 935C0000  stw r26, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 824AE7F8: 935C0004  stw r26, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 824AE7FC: 92DC0008  stw r22, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 824AE800: 93550000  stw r26, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 824AE804: 93550004  stw r26, 4(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 824AE808: 92D50008  stw r22, 8(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 824AE80C: 93570000  stw r26, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 824AE810: 93570004  stw r26, 4(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 824AE814: 92D70008  stw r22, 8(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 824AE818: 93590038  stw r26, 0x38(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(56 as u32), ctx.r[26].u32 ) };
	// 824AE81C: 9359003C  stw r26, 0x3c(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(60 as u32), ctx.r[26].u32 ) };
	// 824AE820: 99590040  stb r10, 0x40(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(64 as u32), ctx.r[10].u8 ) };
	// 824AE824: 817B0038  lwz r11, 0x38(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(56 as u32) ) } as u64;
	// 824AE828: 91790038  stw r11, 0x38(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 824AE82C: 817B003C  lwz r11, 0x3c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(60 as u32) ) } as u64;
	// 824AE830: 9179003C  stw r11, 0x3c(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 824AE834: 897B0040  lbz r11, 0x40(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(64 as u32) ) } as u64;
	// 824AE838: 99790040  stb r11, 0x40(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 824AE83C: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE840: 83FB000C  lwz r31, 0xc(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AE844: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824AE848: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 824AE84C: 40980024  bge cr6, 0x824ae870
	if !ctx.cr[6].lt {
	pc = 0x824AE870; continue 'dispatch;
	}
	// 824AE850: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AE854: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824AE858: 41980008  blt cr6, 0x824ae860
	if ctx.cr[6].lt {
	pc = 0x824AE860; continue 'dispatch;
	}
	// 824AE85C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 824AE860: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 824AE864: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824AE868: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 824AE86C: 4BFBFA5D  bl 0x8246e2c8
	ctx.lr = 0x824AE870;
	sub_8246E2C8(ctx, base);
	// 824AE870: 93F80004  stw r31, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 824AE874: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE878: 83FB0030  lwz r31, 0x30(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(48 as u32) ) } as u64;
	// 824AE87C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824AE880: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 824AE884: 40980024  bge cr6, 0x824ae8a8
	if !ctx.cr[6].lt {
	pc = 0x824AE8A8; continue 'dispatch;
	}
	// 824AE888: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AE88C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824AE890: 41980008  blt cr6, 0x824ae898
	if ctx.cr[6].lt {
	pc = 0x824AE898; continue 'dispatch;
	}
	// 824AE894: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 824AE898: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 824AE89C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824AE8A0: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 824AE8A4: 4BFBFA25  bl 0x8246e2c8
	ctx.lr = 0x824AE8A8;
	sub_8246E2C8(ctx, base);
	// 824AE8A8: 93F70004  stw r31, 4(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 824AE8AC: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE8B0: 83FB0018  lwz r31, 0x18(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 824AE8B4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824AE8B8: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 824AE8BC: 40980024  bge cr6, 0x824ae8e0
	if !ctx.cr[6].lt {
	pc = 0x824AE8E0; continue 'dispatch;
	}
	// 824AE8C0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AE8C4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824AE8C8: 41980008  blt cr6, 0x824ae8d0
	if ctx.cr[6].lt {
	pc = 0x824AE8D0; continue 'dispatch;
	}
	// 824AE8CC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 824AE8D0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 824AE8D4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824AE8D8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824AE8DC: 4BFBF9ED  bl 0x8246e2c8
	ctx.lr = 0x824AE8E0;
	sub_8246E2C8(ctx, base);
	// 824AE8E0: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 824AE8E4: 81750008  lwz r11, 8(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE8E8: 83FB0024  lwz r31, 0x24(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 824AE8EC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824AE8F0: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 824AE8F4: 40980024  bge cr6, 0x824ae918
	if !ctx.cr[6].lt {
	pc = 0x824AE918; continue 'dispatch;
	}
	// 824AE8F8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AE8FC: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824AE900: 41980008  blt cr6, 0x824ae908
	if ctx.cr[6].lt {
	pc = 0x824AE908; continue 'dispatch;
	}
	// 824AE904: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 824AE908: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 824AE90C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824AE910: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 824AE914: 4BFBF9B5  bl 0x8246e2c8
	ctx.lr = 0x824AE918;
	sub_8246E2C8(ctx, base);
	// 824AE918: 93F50004  stw r31, 4(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 824AE91C: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 824AE920: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AE924: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AE928: 40990040  ble cr6, 0x824ae968
	if !ctx.cr[6].gt {
	pc = 0x824AE968; continue 'dispatch;
	}
	// 824AE92C: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 824AE930: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE934: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE938: 7FABFA14  add r29, r11, r31
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 824AE93C: 7C6AF82E  lwzx r3, r10, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824AE940: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE944: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824AE948: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AE94C: 4E800421  bctrl
	ctx.lr = 0x824AE950;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AE950: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824AE954: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AE958: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 824AE95C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824AE960: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824AE964: 4198FFCC  blt cr6, 0x824ae930
	if ctx.cr[6].lt {
	pc = 0x824AE930; continue 'dispatch;
	}
	// 824AE968: 817B0030  lwz r11, 0x30(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(48 as u32) ) } as u64;
	// 824AE96C: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 824AE970: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AE974: 40990040  ble cr6, 0x824ae9b4
	if !ctx.cr[6].gt {
	pc = 0x824AE9B4; continue 'dispatch;
	}
	// 824AE978: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 824AE97C: 815B002C  lwz r10, 0x2c(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(44 as u32) ) } as u64;
	// 824AE980: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE984: 7FBF5A14  add r29, r31, r11
	ctx.r[29].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 824AE988: 7C6AF82E  lwzx r3, r10, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824AE98C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE990: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 824AE994: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AE998: 4E800421  bctrl
	ctx.lr = 0x824AE99C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AE99C: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824AE9A0: 817B0030  lwz r11, 0x30(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(48 as u32) ) } as u64;
	// 824AE9A4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 824AE9A8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824AE9AC: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824AE9B0: 4198FFCC  blt cr6, 0x824ae97c
	if ctx.cr[6].lt {
	pc = 0x824AE97C; continue 'dispatch;
	}
	// 824AE9B4: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 824AE9B8: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 824AE9BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AE9C0: 409900BC  ble cr6, 0x824aea7c
	if !ctx.cr[6].gt {
	pc = 0x824AEA7C; continue 'dispatch;
	}
	// 824AE9C4: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 824AE9C8: 815B0014  lwz r10, 0x14(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 824AE9CC: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 824AE9D0: 813B000C  lwz r9, 0xc(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AE9D4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824AE9D8: 7C7E502E  lwzx r3, r30, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824AE9DC: 81030014  lwz r8, 0x14(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 824AE9E0: 40990024  ble cr6, 0x824aea04
	if !ctx.cr[6].gt {
	pc = 0x824AEA04; continue 'dispatch;
	}
	// 824AE9E4: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AE9E8: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AE9EC: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 824AE9F0: 419A00BC  beq cr6, 0x824aeaac
	if ctx.cr[6].eq {
	pc = 0x824AEAAC; continue 'dispatch;
	}
	// 824AE9F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824AE9F8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824AE9FC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824AEA00: 4198FFE8  blt cr6, 0x824ae9e8
	if ctx.cr[6].lt {
	pc = 0x824AE9E8; continue 'dispatch;
	}
	// 824AEA04: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 824AEA08: 813B000C  lwz r9, 0xc(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AEA0C: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 824AEA10: 81030018  lwz r8, 0x18(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 824AEA14: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824AEA18: 40990024  ble cr6, 0x824aea3c
	if !ctx.cr[6].gt {
	pc = 0x824AEA3C; continue 'dispatch;
	}
	// 824AEA1C: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AEA20: 80CA0000  lwz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AEA24: 7F064040  cmplw cr6, r6, r8
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[8].u32, &mut ctx.xer);
	// 824AEA28: 419A008C  beq cr6, 0x824aeab4
	if ctx.cr[6].eq {
	pc = 0x824AEAB4; continue 'dispatch;
	}
	// 824AEA2C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824AEA30: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824AEA34: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824AEA38: 4198FFE8  blt cr6, 0x824aea20
	if ctx.cr[6].lt {
	pc = 0x824AEA20; continue 'dispatch;
	}
	// 824AEA3C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 824AEA40: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AEA44: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824AEA48: 54E9103A  slwi r9, r7, 2
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824AEA4C: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AEA50: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824AEA54: 7FEAF214  add r31, r10, r30
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 824AEA58: 7CA8582E  lwzx r5, r8, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AEA5C: 7C89582E  lwzx r4, r9, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AEA60: 4BFE3781  bl 0x824921e0
	ctx.lr = 0x824AEA64;
	sub_824921E0(ctx, base);
	// 824AEA64: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824AEA68: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824AEA6C: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 824AEA70: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 824AEA74: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824AEA78: 4198FF50  blt cr6, 0x824ae9c8
	if ctx.cr[6].lt {
	pc = 0x824AE9C8; continue 'dispatch;
	}
	// 824AEA7C: 817B0024  lwz r11, 0x24(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 824AEA80: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 824AEA84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AEA88: 40990294  ble cr6, 0x824aed1c
	if !ctx.cr[6].gt {
	pc = 0x824AED1C; continue 'dispatch;
	}
	// 824AEA8C: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 824AEA90: 817B0020  lwz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 824AEA94: 7FDD582E  lwzx r30, r29, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AEA98: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824AEA9C: 409A0020  bne cr6, 0x824aeabc
	if !ctx.cr[6].eq {
	pc = 0x824AEABC; continue 'dispatch;
	}
	// 824AEAA0: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AEAA4: 7F5D592E  stwx r26, r29, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32), ctx.r[26].u32) };
	// 824AEAA8: 48000260  b 0x824aed08
	pc = 0x824AED08; continue 'dispatch;
	// 824AEAAC: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 824AEAB0: 4BFFFF58  b 0x824aea08
	pc = 0x824AEA08; continue 'dispatch;
	// 824AEAB4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 824AEAB8: 4BFFFF88  b 0x824aea40
	pc = 0x824AEA40; continue 'dispatch;
	// 824AEABC: 93410080  stw r26, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[26].u32 ) };
	// 824AEAC0: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 824AEAC4: 93410084  stw r26, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[26].u32 ) };
	// 824AEAC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AEACC: 92C10088  stw r22, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[22].u32 ) };
	// 824AEAD0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AEAD4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824AEAD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AEADC: 4E800421  bctrl
	ctx.lr = 0x824AEAE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AEAE0: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 824AEAE4: 93410060  stw r26, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[26].u32 ) };
	// 824AEAE8: 93410064  stw r26, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[26].u32 ) };
	// 824AEAEC: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 824AEAF0: 92C10068  stw r22, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[22].u32 ) };
	// 824AEAF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AEAF8: 4099001C  ble cr6, 0x824aeb14
	if !ctx.cr[6].gt {
	pc = 0x824AEB14; continue 'dispatch;
	}
	// 824AEAFC: 40980008  bge cr6, 0x824aeb04
	if !ctx.cr[6].lt {
	pc = 0x824AEB04; continue 'dispatch;
	}
	// 824AEB00: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 824AEB04: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 824AEB08: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824AEB0C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 824AEB10: 4BFBF7B9  bl 0x8246e2c8
	ctx.lr = 0x824AEB14;
	sub_8246E2C8(ctx, base);
	// 824AEB14: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 824AEB18: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824AEB1C: 4099006C  ble cr6, 0x824aeb88
	if !ctx.cr[6].gt {
	pc = 0x824AEB88; continue 'dispatch;
	}
	// 824AEB20: 38DB0008  addi r6, r27, 8
	ctx.r[6].s64 = ctx.r[27].s64 + 8;
	// 824AEB24: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 824AEB28: 81410080  lwz r10, 0x80(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 824AEB2C: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 824AEB30: 81260004  lwz r9, 4(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AEB34: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824AEB38: 7CE8502E  lwzx r7, r8, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824AEB3C: 40990024  ble cr6, 0x824aeb60
	if !ctx.cr[6].gt {
	pc = 0x824AEB60; continue 'dispatch;
	}
	// 824AEB40: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AEB44: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AEB48: 7F053840  cmplw cr6, r5, r7
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[7].u32, &mut ctx.xer);
	// 824AEB4C: 419A0018  beq cr6, 0x824aeb64
	if ctx.cr[6].eq {
	pc = 0x824AEB64; continue 'dispatch;
	}
	// 824AEB50: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824AEB54: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824AEB58: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824AEB5C: 4198FFE8  blt cr6, 0x824aeb44
	if ctx.cr[6].lt {
	pc = 0x824AEB44; continue 'dispatch;
	}
	// 824AEB60: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 824AEB64: 81580000  lwz r10, 0(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AEB68: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AEB6C: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 824AEB70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824AEB74: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824AEB78: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824AEB7C: 7D68512E  stwx r11, r8, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 824AEB80: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 824AEB84: 409AFFA4  bne cr6, 0x824aeb28
	if !ctx.cr[6].eq {
	pc = 0x824AEB28; continue 'dispatch;
	}
	// 824AEB88: 93410070  stw r26, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[26].u32 ) };
	// 824AEB8C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 824AEB90: 93410074  stw r26, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[26].u32 ) };
	// 824AEB94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AEB98: 92C10078  stw r22, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[22].u32 ) };
	// 824AEB9C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AEBA0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824AEBA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AEBA8: 4E800421  bctrl
	ctx.lr = 0x824AEBAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AEBAC: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 824AEBB0: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 824AEBB4: 93410054  stw r26, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u32 ) };
	// 824AEBB8: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 824AEBBC: 92C10058  stw r22, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[22].u32 ) };
	// 824AEBC0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AEBC4: 4099001C  ble cr6, 0x824aebe0
	if !ctx.cr[6].gt {
	pc = 0x824AEBE0; continue 'dispatch;
	}
	// 824AEBC8: 40980008  bge cr6, 0x824aebd0
	if !ctx.cr[6].lt {
	pc = 0x824AEBD0; continue 'dispatch;
	}
	// 824AEBCC: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 824AEBD0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 824AEBD4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824AEBD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824AEBDC: 4BFBF6ED  bl 0x8246e2c8
	ctx.lr = 0x824AEBE0;
	sub_8246E2C8(ctx, base);
	// 824AEBE0: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 824AEBE4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824AEBE8: 4099006C  ble cr6, 0x824aec54
	if !ctx.cr[6].gt {
	pc = 0x824AEC54; continue 'dispatch;
	}
	// 824AEBEC: 38DB002C  addi r6, r27, 0x2c
	ctx.r[6].s64 = ctx.r[27].s64 + 44;
	// 824AEBF0: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 824AEBF4: 81410070  lwz r10, 0x70(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 824AEBF8: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 824AEBFC: 81260004  lwz r9, 4(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AEC00: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824AEC04: 7CE8502E  lwzx r7, r8, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824AEC08: 40990024  ble cr6, 0x824aec2c
	if !ctx.cr[6].gt {
	pc = 0x824AEC2C; continue 'dispatch;
	}
	// 824AEC0C: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AEC10: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AEC14: 7F053840  cmplw cr6, r5, r7
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[7].u32, &mut ctx.xer);
	// 824AEC18: 419A0018  beq cr6, 0x824aec30
	if ctx.cr[6].eq {
	pc = 0x824AEC30; continue 'dispatch;
	}
	// 824AEC1C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824AEC20: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824AEC24: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824AEC28: 4198FFE8  blt cr6, 0x824aec10
	if ctx.cr[6].lt {
	pc = 0x824AEC10; continue 'dispatch;
	}
	// 824AEC2C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 824AEC30: 81570000  lwz r10, 0(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AEC34: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824AEC38: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 824AEC3C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824AEC40: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824AEC44: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824AEC48: 7D68512E  stwx r11, r8, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 824AEC4C: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 824AEC50: 409AFFA4  bne cr6, 0x824aebf4
	if !ctx.cr[6].eq {
	pc = 0x824AEBF4; continue 'dispatch;
	}
	// 824AEC54: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AEC58: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824AEC5C: 83F50000  lwz r31, 0(r21)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AEC60: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 824AEC64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AEC68: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 824AEC6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AEC70: 4E800421  bctrl
	ctx.lr = 0x824AEC74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AEC74: 7C7DF92E  stwx r3, r29, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32), ctx.r[3].u32) };
	// 824AEC78: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824AEC7C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AEC80: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AEC84: 409A0018  bne cr6, 0x824aec9c
	if !ctx.cr[6].eq {
	pc = 0x824AEC9C; continue 'dispatch;
	}
	// 824AEC88: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AEC8C: 7C74982E  lwzx r3, r20, r19
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 824AEC90: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AEC94: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824AEC98: 4BFB5421  bl 0x824640b8
	ctx.lr = 0x824AEC9C;
	sub_824640B8(ctx, base);
	// 824AEC9C: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 824AECA0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AECA4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AECA8: 409A0018  bne cr6, 0x824aecc0
	if !ctx.cr[6].eq {
	pc = 0x824AECC0; continue 'dispatch;
	}
	// 824AECAC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AECB0: 7C74982E  lwzx r3, r20, r19
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 824AECB4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AECB8: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 824AECBC: 4BFB53FD  bl 0x824640b8
	ctx.lr = 0x824AECC0;
	sub_824640B8(ctx, base);
	// 824AECC0: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 824AECC4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AECC8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AECCC: 409A0018  bne cr6, 0x824aece4
	if !ctx.cr[6].eq {
	pc = 0x824AECE4; continue 'dispatch;
	}
	// 824AECD0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AECD4: 7C74982E  lwzx r3, r20, r19
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 824AECD8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AECDC: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824AECE0: 4BFB53D9  bl 0x824640b8
	ctx.lr = 0x824AECE4;
	sub_824640B8(ctx, base);
	// 824AECE4: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 824AECE8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824AECEC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AECF0: 409A0018  bne cr6, 0x824aed08
	if !ctx.cr[6].eq {
	pc = 0x824AED08; continue 'dispatch;
	}
	// 824AECF4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824AECF8: 7C74982E  lwzx r3, r20, r19
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 824AECFC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824AED00: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 824AED04: 4BFB53B5  bl 0x824640b8
	ctx.lr = 0x824AED08;
	sub_824640B8(ctx, base);
	// 824AED08: 817B0024  lwz r11, 0x24(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 824AED0C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 824AED10: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 824AED14: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824AED18: 4198FD78  blt cr6, 0x824aea90
	if ctx.cr[6].lt {
	pc = 0x824AEA90; continue 'dispatch;
	}
	// 824AED1C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 824AED20: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 824AED24: 480863C0  b 0x825350e4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AED28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AED28 size=16
    let mut pc: u32 = 0x824AED28;
    'dispatch: loop {
        match pc {
            0x824AED28 => {
    //   block [0x824AED28..0x824AED38)
	// 824AED28: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 824AED2C: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 824AED30: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 824AED34: 4BFF3C7C  b 0x824a29b0
	sub_824A29B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AED38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AED38 size=32
    let mut pc: u32 = 0x824AED38;
    'dispatch: loop {
        match pc {
            0x824AED38 => {
    //   block [0x824AED38..0x824AED58)
	// 824AED38: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 824AED3C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AED40: 419A0018  beq cr6, 0x824aed58
	if ctx.cr[6].eq {
		sub_824AED58(ctx, base);
		return;
	}
	// 824AED44: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824AED48: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 824AED4C: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824AED50: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824AED54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AED58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AED58 size=16
    let mut pc: u32 = 0x824AED58;
    'dispatch: loop {
        match pc {
            0x824AED58 => {
    //   block [0x824AED58..0x824AED68)
	// 824AED58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AED5C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AED60: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824AED64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AED68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824AED68 size=36
    let mut pc: u32 = 0x824AED68;
    'dispatch: loop {
        match pc {
            0x824AED68 => {
    //   block [0x824AED68..0x824AED8C)
	// 824AED68: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824AED6C: C1A40044  lfs f13, 0x44(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824AED70: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AED74: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824AED78: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824AED7C: 41990008  bgt cr6, 0x824aed84
	if ctx.cr[6].gt {
	pc = 0x824AED84; continue 'dispatch;
	}
	// 824AED80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AED84: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824AED88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AED90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AED90 size=8
    let mut pc: u32 = 0x824AED90;
    'dispatch: loop {
        match pc {
            0x824AED90 => {
    //   block [0x824AED90..0x824AED98)
	// 824AED90: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 824AED94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AED98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824AED98 size=92
    let mut pc: u32 = 0x824AED98;
    'dispatch: loop {
        match pc {
            0x824AED98 => {
    //   block [0x824AED98..0x824AEDF4)
	// 824AED98: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824AED9C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824AEDA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 824AEDA4: 392AF384  addi r9, r10, -0xc7c
	ctx.r[9].s64 = ctx.r[10].s64 + -3196;
	// 824AEDA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824AEDAC: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 824AEDB0: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AEDB4: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 824AEDB8: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 824AEDBC: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 824AEDC0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824AEDC4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824AEDC8: 39430030  addi r10, r3, 0x30
	ctx.r[10].s64 = ctx.r[3].s64 + 48;
	// 824AEDCC: B0E30010  sth r7, 0x10(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u16 ) };
	// 824AEDD0: B0C30040  sth r6, 0x40(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[6].u16 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AEDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AEDF8 size=8
    let mut pc: u32 = 0x824AEDF8;
    'dispatch: loop {
        match pc {
            0x824AEDF8 => {
    //   block [0x824AEDF8..0x824AEE00)
	// 824AEDF8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824AEDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AEE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AEE00 size=12
    let mut pc: u32 = 0x824AEE00;
    'dispatch: loop {
        match pc {
            0x824AEE00 => {
    //   block [0x824AEE00..0x824AEE0C)
	// 824AEE00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AEE04: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824AEE08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AEE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AEE10 size=12
    let mut pc: u32 = 0x824AEE10;
    'dispatch: loop {
        match pc {
            0x824AEE10 => {
    //   block [0x824AEE10..0x824AEE1C)
	// 824AEE10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AEE14: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824AEE18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AEE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AEE20 size=252
    let mut pc: u32 = 0x824AEE20;
    'dispatch: loop {
        match pc {
            0x824AEE20 => {
    //   block [0x824AEE20..0x824AEF1C)
	// 824AEE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AEE24: 48086299  bl 0x825350bc
	ctx.lr = 0x824AEE28;
	sub_82535080(ctx, base);
	// 824AEE28: A0E40000  lhz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AEE2C: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
	// 824AEE30: 39430010  addi r10, r3, 0x10
	ctx.r[10].s64 = ctx.r[3].s64 + 16;
	// 824AEE34: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 824AEE38: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 824AEE3C: 39240050  addi r9, r4, 0x50
	ctx.r[9].s64 = ctx.r[4].s64 + 80;
	// 824AEE40: B0E30000  sth r7, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AEF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AEF20 size=288
    let mut pc: u32 = 0x824AEF20;
    'dispatch: loop {
        match pc {
            0x824AEF20 => {
    //   block [0x824AEF20..0x824AF040)
	// 824AEF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AEF24: 48086179  bl 0x8253509c
	ctx.lr = 0x824AEF28;
	sub_82535080(ctx, base);
	// 824AEF28: A0E40000  lhz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AEF2C: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
	// 824AEF30: 39430010  addi r10, r3, 0x10
	ctx.r[10].s64 = ctx.r[3].s64 + 16;
	// 824AEF34: 38C40090  addi r6, r4, 0x90
	ctx.r[6].s64 = ctx.r[4].s64 + 144;
	// 824AEF38: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 824AEF3C: 38A30090  addi r5, r3, 0x90
	ctx.r[5].s64 = ctx.r[3].s64 + 144;
	// 824AEF40: B0E30000  sth r7, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 824AEF44: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 824AEF48: 3BE00020  li r31, 0x20
	ctx.r[31].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AF040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AF040 size=356
    let mut pc: u32 = 0x824AF040;
    'dispatch: loop {
        match pc {
            0x824AF040 => {
    //   block [0x824AF040..0x824AF1A4)
	// 824AF040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AF044: 48086061  bl 0x825350a4
	ctx.lr = 0x824AF048;
	sub_82535080(ctx, base);
	// 824AF048: A1240000  lhz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AF04C: 38E40010  addi r7, r4, 0x10
	ctx.r[7].s64 = ctx.r[4].s64 + 16;
	// 824AF050: 38C30010  addi r6, r3, 0x10
	ctx.r[6].s64 = ctx.r[3].s64 + 16;
	// 824AF054: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 824AF058: 3B800030  li r28, 0x30
	ctx.r[28].s64 = 48;
	// 824AF05C: 38A40050  addi r5, r4, 0x50
	ctx.r[5].s64 = ctx.r[4].s64 + 80;
	// 824AF060: B1230000  sth r9, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 824AF064: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AF1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AF1A8 size=408
    let mut pc: u32 = 0x824AF1A8;
    'dispatch: loop {
        match pc {
            0x824AF1A8 => {
    //   block [0x824AF1A8..0x824AF340)
	// 824AF1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AF1AC: 48085F11  bl 0x825350bc
	ctx.lr = 0x824AF1B0;
	sub_82535080(ctx, base);
	// 824AF1B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AF1B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AF1B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AF1BC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824AF1C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AF1C4: 4E800421  bctrl
	ctx.lr = 0x824AF1C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AF1C8: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 824AF1CC: 419A010C  beq cr6, 0x824af2d8
	if ctx.cr[6].eq {
	pc = 0x824AF2D8; continue 'dispatch;
	}
	// 824AF1D0: 2F030006  cmpwi cr6, r3, 6
	ctx.cr[6].compare_i32(ctx.r[3].s32, 6, &mut ctx.xer);
	// 824AF1D4: 419A009C  beq cr6, 0x824af270
	if ctx.cr[6].eq {
	pc = 0x824AF270; continue 'dispatch;
	}
	// 824AF1D8: 2F030007  cmpwi cr6, r3, 7
	ctx.cr[6].compare_i32(ctx.r[3].s32, 7, &mut ctx.xer);
	// 824AF1DC: 419A0010  beq cr6, 0x824af1ec
	if ctx.cr[6].eq {
	pc = 0x824AF1EC; continue 'dispatch;
	}
	// 824AF1E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824AF1E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AF1E8: 48085F24  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 824AF1EC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AF1F0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824AF1F4: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 824AF1F8: 38800140  li r4, 0x140
	ctx.r[4].s64 = 320;
	// 824AF1FC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AF200: 4BFB4E39  bl 0x82464038
	ctx.lr = 0x824AF204;
	sub_82464038(ctx, base);
	// 824AF204: 39600140  li r11, 0x140
	ctx.r[11].s64 = 320;
	// 824AF208: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 824AF20C: 480013AD  bl 0x824b05b8
	ctx.lr = 0x824AF210;
	sub_824B05B8(ctx, base);
	// 824AF210: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824AF214: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 824AF218: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 824AF21C: 4BFFFE25  bl 0x824af040
	ctx.lr = 0x824AF220;
	sub_824AF040(ctx, base);
	// 824AF220: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AF224: 397E00E0  addi r11, r30, 0xe0
	ctx.r[11].s64 = ctx.r[30].s64 + 224;
	// 824AF228: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	// 824AF22C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 824AF230: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824AF234: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AF238: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824AF23C: 419A0018  beq cr6, 0x824af254
	if ctx.cr[6].eq {
	pc = 0x824AF254; continue 'dispatch;
	}
	// 824AF240: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AF244: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AF248: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AF24C: 4E800421  bctrl
	ctx.lr = 0x824AF250;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AF250: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824AF254: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 824AF258: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824AF25C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 824AF260: 409AFFD4  bne cr6, 0x824af234
	if !ctx.cr[6].eq {
	pc = 0x824AF234; continue 'dispatch;
	}
	// 824AF264: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AF268: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AF26C: 48085EA0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 824AF270: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AF274: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824AF278: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 824AF27C: 388000D0  li r4, 0xd0
	ctx.r[4].s64 = 208;
	// 824AF280: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AF284: 4BFB4DB5  bl 0x82464038
	ctx.lr = 0x824AF288;
	sub_82464038(ctx, base);
	// 824AF288: 396000D0  li r11, 0xd0
	ctx.r[11].s64 = 208;
	// 824AF28C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 824AF290: 4800A851  bl 0x824b9ae0
	ctx.lr = 0x824AF294;
	sub_824B9AE0(ctx, base);
	// 824AF294: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824AF298: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 824AF29C: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 824AF2A0: 4BFFFC81  bl 0x824aef20
	ctx.lr = 0x824AF2A4;
	sub_824AEF20(ctx, base);
	// 824AF2A4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AF2A8: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824AF2AC: 807E00AC  lwz r3, 0xac(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 824AF2B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824AF2B4: 419A0080  beq cr6, 0x824af334
	if ctx.cr[6].eq {
	pc = 0x824AF334; continue 'dispatch;
	}
	// 824AF2B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AF2BC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AF2C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AF2C4: 4E800421  bctrl
	ctx.lr = 0x824AF2C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AF2C8: 907E00AC  stw r3, 0xac(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(172 as u32), ctx.r[3].u32 ) };
	// 824AF2CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AF2D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AF2D4: 48085E38  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 824AF2D8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AF2DC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824AF2E0: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 824AF2E4: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 824AF2E8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824AF2EC: 4BFB4D4D  bl 0x82464038
	ctx.lr = 0x824AF2F0;
	sub_82464038(ctx, base);
	// 824AF2F0: 396000E0  li r11, 0xe0
	ctx.r[11].s64 = 224;
	// 824AF2F4: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 824AF2F8: 480008E1  bl 0x824afbd8
	ctx.lr = 0x824AF2FC;
	sub_824AFBD8(ctx, base);
	// 824AF2FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824AF300: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 824AF304: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 824AF308: 4BFFFB19  bl 0x824aee20
	ctx.lr = 0x824AF30C;
	sub_824AEE20(ctx, base);
	// 824AF30C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824AF310: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824AF314: 807E00B0  lwz r3, 0xb0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) } as u64;
	// 824AF318: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824AF31C: 419A0018  beq cr6, 0x824af334
	if ctx.cr[6].eq {
	pc = 0x824AF334; continue 'dispatch;
	}
	// 824AF320: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AF324: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824AF328: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AF32C: 4E800421  bctrl
	ctx.lr = 0x824AF330;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AF330: 907E00B0  stw r3, 0xb0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(176 as u32), ctx.r[3].u32 ) };
	// 824AF334: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AF338: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AF33C: 48085DD0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AF340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AF340 size=8
    let mut pc: u32 = 0x824AF340;
    'dispatch: loop {
        match pc {
            0x824AF340 => {
    //   block [0x824AF340..0x824AF348)
	// 824AF340: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 824AF344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AF348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824AF348 size=164
    let mut pc: u32 = 0x824AF348;
    'dispatch: loop {
        match pc {
            0x824AF348 => {
    //   block [0x824AF348..0x824AF3EC)
	// 824AF348: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 824AF34C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 824AF350: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824AF354: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 824AF358: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 824AF35C: C1882090  lfs f12, 0x2090(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8336 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824AF360: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 824AF364: C1A920B0  lfs f13, 0x20b0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8368 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824AF368: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824AF36C: C00A2074  lfs f0, 0x2074(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AF370: 39430030  addi r10, r3, 0x30
	ctx.r[10].s64 = ctx.r[3].s64 + 48;
	// 824AF374: 3929F4B4  addi r9, r9, -0xb4c
	ctx.r[9].s64 = ctx.r[9].s64 + -2892;
	// 824AF378: D0030050  stfs f0, 0x50(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 824AF37C: D1A30054  stfs f13, 0x54(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 824AF380: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 824AF384: C1688E30  lfs f11, -0x71d0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29136 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824AF388: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 824AF38C: D1830058  stfs f12, 0x58(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 824AF390: D163005C  stfs f11, 0x5c(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 824AF394: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824AF398: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 824AF39C: C148D6C8  lfs f10, -0x2938(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-10552 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 824AF3A0: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 824AF3A4: D1430060  stfs f10, 0x60(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 824AF3A8: C0088CB4  lfs f0, -0x734c(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AF3AC: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 824AF3B0: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AF3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AF3F0 size=32
    let mut pc: u32 = 0x824AF3F0;
    'dispatch: loop {
        match pc {
            0x824AF3F0 => {
    //   block [0x824AF3F0..0x824AF410)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AF410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824AF410 size=136
    let mut pc: u32 = 0x824AF410;
    'dispatch: loop {
        match pc {
            0x824AF410 => {
    //   block [0x824AF410..0x824AF498)
	// 824AF410: 396500E0  addi r11, r5, 0xe0
	ctx.r[11].s64 = ctx.r[5].s64 + 224;
	// 824AF414: 39240010  addi r9, r4, 0x10
	ctx.r[9].s64 = ctx.r[4].s64 + 16;
	// 824AF418: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	// 824AF41C: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 824AF420: 38EA0010  addi r7, r10, 0x10
	ctx.r[7].s64 = ctx.r[10].s64 + 16;
	// 824AF424: C1AB00A0  lfs f13, 0xa0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824AF428: 39040020  addi r8, r4, 0x20
	ctx.r[8].s64 = ctx.r[4].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AF498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AF498 size=24
    let mut pc: u32 = 0x824AF498;
    'dispatch: loop {
        match pc {
            0x824AF498 => {
    //   block [0x824AF498..0x824AF4B0)
	// 824AF498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AF4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824AF4B0 size=136
    let mut pc: u32 = 0x824AF4B0;
    'dispatch: loop {
        match pc {
            0x824AF4B0 => {
    //   block [0x824AF4B0..0x824AF538)
	// 824AF4B0: 396500E0  addi r11, r5, 0xe0
	ctx.r[11].s64 = ctx.r[5].s64 + 224;
	// 824AF4B4: 39240030  addi r9, r4, 0x30
	ctx.r[9].s64 = ctx.r[4].s64 + 48;
	// 824AF4B8: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	// 824AF4BC: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 824AF4C0: 38EA0010  addi r7, r10, 0x10
	ctx.r[7].s64 = ctx.r[10].s64 + 16;
	// 824AF4C4: C1AB00A0  lfs f13, 0xa0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824AF4C8: 39040040  addi r8, r4, 0x40
	ctx.r[8].s64 = ctx.r[4].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AF538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AF538 size=24
    let mut pc: u32 = 0x824AF538;
    'dispatch: loop {
        match pc {
            0x824AF538 => {
    //   block [0x824AF538..0x824AF550)
	// 824AF538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AF550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AF550 size=356
    let mut pc: u32 = 0x824AF550;
    'dispatch: loop {
        match pc {
            0x824AF550 => {
    //   block [0x824AF550..0x824AF6B4)
	// 824AF550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AF554: 48085B69  bl 0x825350bc
	ctx.lr = 0x824AF558;
	sub_82535080(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AF6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AF6B8 size=156
    let mut pc: u32 = 0x824AF6B8;
    'dispatch: loop {
        match pc {
            0x824AF6B8 => {
    //   block [0x824AF6B8..0x824AF754)
	// 824AF6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AF6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AF6C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AF6C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AF6C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AF6CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824AF6D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824AF6D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824AF6D8: 419A001C  beq cr6, 0x824af6f4
	if ctx.cr[6].eq {
	pc = 0x824AF6F4; continue 'dispatch;
	}
	// 824AF6DC: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AF6E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AF6E4: 419A0010  beq cr6, 0x824af6f4
	if ctx.cr[6].eq {
	pc = 0x824AF6F4; continue 'dispatch;
	}
	// 824AF6E8: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 824AF6EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824AF6F0: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824AF6F4: 807E00B0  lwz r3, 0xb0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) } as u64;
	// 824AF6F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824AF6FC: 419A003C  beq cr6, 0x824af738
	if ctx.cr[6].eq {
	pc = 0x824AF738; continue 'dispatch;
	}
	// 824AF700: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AF704: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AF708: 419A0030  beq cr6, 0x824af738
	if ctx.cr[6].eq {
	pc = 0x824AF738; continue 'dispatch;
	}
	// 824AF70C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824AF710: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824AF714: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824AF718: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AF71C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824AF720: 409A0018  bne cr6, 0x824af738
	if !ctx.cr[6].eq {
	pc = 0x824AF738; continue 'dispatch;
	}
	// 824AF724: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AF728: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824AF72C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AF730: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AF734: 4E800421  bctrl
	ctx.lr = 0x824AF738;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AF738: 93FE00B0  stw r31, 0xb0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 824AF73C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824AF740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AF744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AF748: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AF74C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AF750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AF758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AF758 size=16
    let mut pc: u32 = 0x824AF758;
    'dispatch: loop {
        match pc {
            0x824AF758 => {
    //   block [0x824AF758..0x824AF768)
	// 824AF758: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 824AF75C: 388000C6  li r4, 0xc6
	ctx.r[4].s64 = 198;
	// 824AF760: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 824AF764: 4BFF324C  b 0x824a29b0
	sub_824A29B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AF768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AF768 size=20
    let mut pc: u32 = 0x824AF768;
    'dispatch: loop {
        match pc {
            0x824AF768 => {
    //   block [0x824AF768..0x824AF77C)
	// 824AF768: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 824AF76C: 39400050  li r10, 0x50
	ctx.r[10].s64 = 80;
	// 824AF770: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824AF774: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824AF778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AF780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824AF780 size=168
    let mut pc: u32 = 0x824AF780;
    'dispatch: loop {
        match pc {
            0x824AF780 => {
    //   block [0x824AF780..0x824AF828)
	// 824AF780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AF784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AF788: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AF78C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AF790: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 824AF794: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AF798: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824AF79C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824AF7A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824AF7A4: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 824AF7A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824AF7AC: C3EB2238  lfs f31, 0x2238(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8760 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824AF7B0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824AF7B4: 480FBFAD  bl 0x825ab760
	ctx.lr = 0x824AF7B8;
	sub_825AB760(ctx, base);
	// 824AF7B8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AF7BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AF7C0: 419A0040  beq cr6, 0x824af800
	if ctx.cr[6].eq {
	pc = 0x824AF800; continue 'dispatch;
	}
	// 824AF7C4: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 824AF7C8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824AF7CC: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 824AF7D0: 480FBF91  bl 0x825ab760
	ctx.lr = 0x824AF7D4;
	sub_825AB760(ctx, base);
	// 824AF7D4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AF7D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AF7DC: 419A0024  beq cr6, 0x824af800
	if ctx.cr[6].eq {
	pc = 0x824AF800; continue 'dispatch;
	}
	// 824AF7E0: 897F00A2  lbz r11, 0xa2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(162 as u32) ) } as u64;
	// 824AF7E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AF7E8: 419A0010  beq cr6, 0x824af7f8
	if ctx.cr[6].eq {
	pc = 0x824AF7F8; continue 'dispatch;
	}
	// 824AF7EC: 897F00C2  lbz r11, 0xc2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(194 as u32) ) } as u64;
	// 824AF7F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AF7F4: 419A000C  beq cr6, 0x824af800
	if ctx.cr[6].eq {
	pc = 0x824AF800; continue 'dispatch;
	}
	// 824AF7F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824AF7FC: 48000008  b 0x824af804
	pc = 0x824AF804; continue 'dispatch;
	// 824AF800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AF804: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824AF808: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824AF80C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824AF810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AF814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AF818: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 824AF81C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824AF820: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AF824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AF828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AF828 size=8
    let mut pc: u32 = 0x824AF828;
    'dispatch: loop {
        match pc {
            0x824AF828 => {
    //   block [0x824AF828..0x824AF830)
	// 824AF828: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 824AF82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AF830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824AF830 size=132
    let mut pc: u32 = 0x824AF830;
    'dispatch: loop {
        match pc {
            0x824AF830 => {
    //   block [0x824AF830..0x824AF8B4)
	// 824AF830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AF834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AF838: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AF83C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AF840: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AF844: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824AF848: 396BF5AC  addi r11, r11, -0xa54
	ctx.r[11].s64 = ctx.r[11].s64 + -2644;
	// 824AF84C: 807F00B0  lwz r3, 0xb0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 824AF850: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824AF854: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AF858: 419A003C  beq cr6, 0x824af894
	if ctx.cr[6].eq {
	pc = 0x824AF894; continue 'dispatch;
	}
	// 824AF85C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824AF860: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AF864: 419A0030  beq cr6, 0x824af894
	if ctx.cr[6].eq {
	pc = 0x824AF894; continue 'dispatch;
	}
	// 824AF868: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824AF86C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824AF870: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824AF874: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AF878: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824AF87C: 409A0018  bne cr6, 0x824af894
	if !ctx.cr[6].eq {
	pc = 0x824AF894; continue 'dispatch;
	}
	// 824AF880: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AF884: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824AF888: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AF88C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824AF890: 4E800421  bctrl
	ctx.lr = 0x824AF894;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824AF894: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824AF898: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824AF89C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824AF8A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824AF8A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824AF8A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824AF8AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824AF8B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AF8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AF8B8 size=720
    let mut pc: u32 = 0x824AF8B8;
    'dispatch: loop {
        match pc {
            0x824AF8B8 => {
    //   block [0x824AF8B8..0x824AFB88)
	// 824AF8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AF8BC: 480857F5  bl 0x825350b0
	ctx.lr = 0x824AF8C0;
	sub_82535080(ctx, base);
	// 824AF8C0: 3980FF90  li r12, -0x70
	ctx.r[12].s64 = -112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AFB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AFB88 size=28
    let mut pc: u32 = 0x824AFB88;
    'dispatch: loop {
        match pc {
            0x824AFB88 => {
    //   block [0x824AFB88..0x824AFBA4)
	// 824AFB88: 7CAB0774  extsb r11, r5
	ctx.r[11].s64 = ctx.r[5].s8 as i64;
	// 824AFB8C: 98A300A2  stb r5, 0xa2(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(162 as u32), ctx.r[5].u8 ) };
	// 824AFB90: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824AFB94: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824AFB98: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824AFB9C: 996300B6  stb r11, 0xb6(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(182 as u32), ctx.r[11].u8 ) };
	// 824AFBA0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AFBA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AFBA4 size=12
    let mut pc: u32 = 0x824AFBA4;
    'dispatch: loop {
        match pc {
            0x824AFBA4 => {
    //   block [0x824AFBA4..0x824AFBB0)
	// 824AFBA4: 81640028  lwz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 824AFBA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AFBAC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AFBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AFBB0 size=12
    let mut pc: u32 = 0x824AFBB0;
    'dispatch: loop {
        match pc {
            0x824AFBB0 => {
    //   block [0x824AFBB0..0x824AFBBC)
	// 824AFBB0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 824AFBB4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824AFBB8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AFBBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824AFBBC size=28
    let mut pc: u32 = 0x824AFBBC;
    'dispatch: loop {
        match pc {
            0x824AFBBC => {
    //   block [0x824AFBBC..0x824AFBD8)
	// 824AFBBC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824AFBC0: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AFBC4: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824AFBC8: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824AFBCC: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824AFBD0: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824AFBD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AFBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AFBD8 size=336
    let mut pc: u32 = 0x824AFBD8;
    'dispatch: loop {
        match pc {
            0x824AFBD8 => {
    //   block [0x824AFBD8..0x824AFD28)
	// 824AFBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AFBDC: 480854D5  bl 0x825350b0
	ctx.lr = 0x824AFBE0;
	sub_82535080(ctx, base);
	// 824AFBE0: 3941FFB0  addi r10, r1, -0x50
	ctx.r[10].s64 = ctx.r[1].s64 + -80;
	// 824AFBE4: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 824AFBE8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 824AFBEC: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 824AFBF0: 3BC00012  li r30, 0x12
	ctx.r[30].s64 = 18;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AFD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AFD28 size=388
    let mut pc: u32 = 0x824AFD28;
    'dispatch: loop {
        match pc {
            0x824AFD28 => {
    //   block [0x824AFD28..0x824AFEAC)
	// 824AFD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AFD2C: 4808538D  bl 0x825350b8
	ctx.lr = 0x824AFD30;
	sub_82535080(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AFEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AFEB0 size=16
    let mut pc: u32 = 0x824AFEB0;
    'dispatch: loop {
        match pc {
            0x824AFEB0 => {
    //   block [0x824AFEB0..0x824AFEC0)
	// 824AFEB0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 824AFEB4: 3880012A  li r4, 0x12a
	ctx.r[4].s64 = 298;
	// 824AFEB8: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 824AFEBC: 4BFF2AF4  b 0x824a29b0
	sub_824A29B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AFEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AFEC0 size=20
    let mut pc: u32 = 0x824AFEC0;
    'dispatch: loop {
        match pc {
            0x824AFEC0 => {
    //   block [0x824AFEC0..0x824AFED4)
	// 824AFEC0: 3960000C  li r11, 0xc
	ctx.r[11].s64 = 12;
	// 824AFEC4: 39400080  li r10, 0x80
	ctx.r[10].s64 = 128;
	// 824AFEC8: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824AFECC: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824AFED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AFED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824AFED8 size=8
    let mut pc: u32 = 0x824AFED8;
    'dispatch: loop {
        match pc {
            0x824AFED8 => {
    //   block [0x824AFED8..0x824AFEE0)
	// 824AFED8: D02300F8  stfs f1, 0xf8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 824AFEDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AFEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824AFEE0 size=28
    let mut pc: u32 = 0x824AFEE0;
    'dispatch: loop {
        match pc {
            0x824AFEE0 => {
    //   block [0x824AFEE0..0x824AFEFC)
	// 824AFEE0: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 824AFEE4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824AFEE8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824AFEEC: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 824AFEF0: 1D6B0038  mulli r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 * 56;
	// 824AFEF4: 99630116  stb r11, 0x116(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(278 as u32), ctx.r[11].u8 ) };
	// 824AFEF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824AFF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824AFF00 size=384
    let mut pc: u32 = 0x824AFF00;
    'dispatch: loop {
        match pc {
            0x824AFF00 => {
    //   block [0x824AFF00..0x824B0080)
	// 824AFF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824AFF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824AFF08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824AFF0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824AFF10: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 824AFF14: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824AFF18: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824AFF1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824AFF20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824AFF24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824AFF28: 389E0020  addi r4, r30, 0x20
	ctx.r[4].s64 = ctx.r[30].s64 + 32;
	// 824AFF2C: C3EB2238  lfs f31, 0x2238(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8760 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824AFF30: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824AFF34: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824AFF38: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824AFF3C: 480FB825  bl 0x825ab760
	ctx.lr = 0x824AFF40;
	sub_825AB760(ctx, base);
	// 824AFF40: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AFF44: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824AFF48: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824AFF4C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824AFF50: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 824AFF54: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 824AFF58: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AFF5C: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824AFF60: 419A0024  beq cr6, 0x824aff84
	if ctx.cr[6].eq {
	pc = 0x824AFF84; continue 'dispatch;
	}
	// 824AFF64: 389E0060  addi r4, r30, 0x60
	ctx.r[4].s64 = ctx.r[30].s64 + 96;
	// 824AFF68: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824AFF6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824AFF70: 480FB7F1  bl 0x825ab760
	ctx.lr = 0x824AFF74;
	sub_825AB760(ctx, base);
	// 824AFF74: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824AFF78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824AFF7C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824AFF80: 409A0008  bne cr6, 0x824aff88
	if !ctx.cr[6].eq {
	pc = 0x824AFF88; continue 'dispatch;
	}
	// 824AFF84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AFF88: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 824AFF8C: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824AFF90: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AFF94: 419A001C  beq cr6, 0x824affb0
	if ctx.cr[6].eq {
	pc = 0x824AFFB0; continue 'dispatch;
	}
	// 824AFF98: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824AFF9C: C1BE0118  lfs f13, 0x118(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(280 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824AFFA0: C00B233C  lfs f0, 0x233c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9020 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AFFA4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824AFFA8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824AFFAC: 419A0008  beq cr6, 0x824affb4
	if ctx.cr[6].eq {
	pc = 0x824AFFB4; continue 'dispatch;
	}
	// 824AFFB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AFFB4: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 824AFFB8: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824AFFBC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AFFC0: 419A001C  beq cr6, 0x824affdc
	if ctx.cr[6].eq {
	pc = 0x824AFFDC; continue 'dispatch;
	}
	// 824AFFC4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824AFFC8: C1BE011C  lfs f13, 0x11c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(284 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824AFFCC: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AFFD0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824AFFD4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824AFFD8: 40980008  bge cr6, 0x824affe0
	if !ctx.cr[6].lt {
	pc = 0x824AFFE0; continue 'dispatch;
	}
	// 824AFFDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824AFFE0: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 824AFFE4: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824AFFE8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824AFFEC: 419A001C  beq cr6, 0x824b0008
	if ctx.cr[6].eq {
	pc = 0x824B0008; continue 'dispatch;
	}
	// 824AFFF0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824AFFF4: C1BE011C  lfs f13, 0x11c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(284 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824AFFF8: C00B2930  lfs f0, 0x2930(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10544 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824AFFFC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824B0000: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824B0004: 41980008  blt cr6, 0x824b000c
	if ctx.cr[6].lt {
	pc = 0x824B000C; continue 'dispatch;
	}
	// 824B0008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824B000C: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 824B0010: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824B0014: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824B0018: 419A0018  beq cr6, 0x824b0030
	if ctx.cr[6].eq {
	pc = 0x824B0030; continue 'dispatch;
	}
	// 824B001C: C01E012C  lfs f0, 0x12c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(300 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824B0020: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824B0024: C1BE0130  lfs f13, 0x130(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(304 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824B0028: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 824B002C: 40990008  ble cr6, 0x824b0034
	if !ctx.cr[6].gt {
	pc = 0x824B0034; continue 'dispatch;
	}
	// 824B0030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824B0034: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 824B0038: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824B003C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824B0040: 419A0018  beq cr6, 0x824b0058
	if ctx.cr[6].eq {
	pc = 0x824B0058; continue 'dispatch;
	}
	// 824B0044: C01E0104  lfs f0, 0x104(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(260 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824B0048: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824B004C: C1BE0108  lfs f13, 0x108(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(264 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824B0050: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 824B0054: 40990008  ble cr6, 0x824b005c
	if !ctx.cr[6].gt {
	pc = 0x824B005C; continue 'dispatch;
	}
	// 824B0058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824B005C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824B0060: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824B0064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824B0068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824B006C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824B0070: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 824B0074: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824B0078: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824B007C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824B0080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824B0080 size=40
    let mut pc: u32 = 0x824B0080;
    'dispatch: loop {
        match pc {
            0x824B0080 => {
    //   block [0x824B0080..0x824B00A8)
	// 824B0080: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 824B0084: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824B0088: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824B00A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824B00A8 size=40
    let mut pc: u32 = 0x824B00A8;
    'dispatch: loop {
        match pc {
            0x824B00A8 => {
    //   block [0x824B00A8..0x824B00D0)
	// 824B00A8: 39630060  addi r11, r3, 0x60
	ctx.r[11].s64 = ctx.r[3].s64 + 96;
	// 824B00AC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824B00B0: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824B00D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824B00D0 size=40
    let mut pc: u32 = 0x824B00D0;
    'dispatch: loop {
        match pc {
            0x824B00D0 => {
    //   block [0x824B00D0..0x824B00F8)
	// 824B00D0: 396300B0  addi r11, r3, 0xb0
	ctx.r[11].s64 = ctx.r[3].s64 + 176;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824B00F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824B00F8 size=12
    let mut pc: u32 = 0x824B00F8;
    'dispatch: loop {
        match pc {
            0x824B00F8 => {
    //   block [0x824B00F8..0x824B0104)
	// 824B00F8: 38A30020  addi r5, r3, 0x20
	ctx.r[5].s64 = ctx.r[3].s64 + 32;
	// 824B00FC: 386300B0  addi r3, r3, 0xb0
	ctx.r[3].s64 = ctx.r[3].s64 + 176;
	// 824B0100: 480F7120  b 0x825a7220
	sub_825A7220(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824B0108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824B0108 size=40
    let mut pc: u32 = 0x824B0108;
    'dispatch: loop {
        match pc {
            0x824B0108 => {
    //   block [0x824B0108..0x824B0130)
	// 824B0108: 396300B0  addi r11, r3, 0xb0
	ctx.r[11].s64 = ctx.r[3].s64 + 176;
	// 824B010C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824B0110: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824B0130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824B0130 size=8
    let mut pc: u32 = 0x824B0130;
    'dispatch: loop {
        match pc {
            0x824B0130 => {
    //   block [0x824B0130..0x824B0138)
	// 824B0130: 806300E0  lwz r3, 0xe0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(224 as u32) ) } as u64;
	// 824B0134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824B0138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824B0138 size=140
    let mut pc: u32 = 0x824B0138;
    'dispatch: loop {
        match pc {
            0x824B0138 => {
    //   block [0x824B0138..0x824B01C4)
	// 824B0138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824B013C: 48084F81  bl 0x825350bc
	ctx.lr = 0x824B0140;
	sub_82535080(ctx, base);
	// 824B0140: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824B0144: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824B0148: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824B014C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824B0150: 419A001C  beq cr6, 0x824b016c
	if ctx.cr[6].eq {
	pc = 0x824B016C; continue 'dispatch;
	}
	// 824B0154: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824B0158: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824B015C: 419A0010  beq cr6, 0x824b016c
	if ctx.cr[6].eq {
	pc = 0x824B016C; continue 'dispatch;
	}
	// 824B0160: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 824B0164: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824B0168: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824B016C: 39640038  addi r11, r4, 0x38
	ctx.r[11].s64 = ctx.r[4].s64 + 56;
	// 824B0170: 557F103A  slwi r31, r11, 2
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 824B0174: 7C7FE82E  lwzx r3, r31, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824B0178: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824B017C: 419A003C  beq cr6, 0x824b01b8
	if ctx.cr[6].eq {
	pc = 0x824B01B8; continue 'dispatch;
	}
	// 824B0180: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824B0184: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824B0188: 419A0030  beq cr6, 0x824b01b8
	if ctx.cr[6].eq {
	pc = 0x824B01B8; continue 'dispatch;
	}
	// 824B018C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824B0190: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824B0194: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824B0198: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824B019C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824B01A0: 409A0018  bne cr6, 0x824b01b8
	if !ctx.cr[6].eq {
	pc = 0x824B01B8; continue 'dispatch;
	}
	// 824B01A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824B01A8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824B01AC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824B01B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824B01B4: 4E800421  bctrl
	ctx.lr = 0x824B01B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824B01B8: 7FDFE92E  stwx r30, r31, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32), ctx.r[30].u32) };
	// 824B01BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824B01C0: 48084F4C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


