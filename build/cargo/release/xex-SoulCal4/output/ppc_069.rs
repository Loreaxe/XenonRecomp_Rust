pub fn sub_824D3650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D3650 size=832
    let mut pc: u32 = 0x824D3650;
    'dispatch: loop {
        match pc {
            0x824D3650 => {
    //   block [0x824D3650..0x824D3990)
	// 824D3650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D3654: 48061A5D  bl 0x825350b0
	ctx.lr = 0x824D3658;
	sub_82535080(ctx, base);
	// 824D3658: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D365C: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D3660: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824D3664: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 824D3668: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824D366C: 395D0024  addi r10, r29, 0x24
	ctx.r[10].s64 = ctx.r[29].s64 + 36;
	// 824D3670: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 824D3674: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D3678: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 824D367C: 419A0014  beq cr6, 0x824d3690
	if ctx.cr[6].eq {
	pc = 0x824D3690; continue 'dispatch;
	}
	// 824D3680: 7D684B78  or r8, r11, r9
	ctx.r[8].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 824D3684: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824D3688: 935D0004  stw r26, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 824D368C: 911D0008  stw r8, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 824D3690: 3BDD000C  addi r30, r29, 0xc
	ctx.r[30].s64 = ctx.r[29].s64 + 12;
	// 824D3694: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824D3698: 419A001C  beq cr6, 0x824d36b4
	if ctx.cr[6].eq {
	pc = 0x824D36B4; continue 'dispatch;
	}
	// 824D369C: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824D36A0: 935E0004  stw r26, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 824D36A4: 7D674B78  or r7, r11, r9
	ctx.r[7].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 824D36A8: 7D085214  add r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 824D36AC: 90FE0008  stw r7, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 824D36B0: 911E0000  stw r8, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824D36B4: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 824D36B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824D36BC: 419A001C  beq cr6, 0x824d36d8
	if ctx.cr[6].eq {
	pc = 0x824D36D8; continue 'dispatch;
	}
	// 824D36C0: 55681838  slwi r8, r11, 3
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824D36C4: 935F0004  stw r26, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 824D36C8: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 824D36CC: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 824D36D0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824D36D4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824D36D8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D36DC: B3410054  sth r26, 0x54(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u16 ) };
	// 824D36E0: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D36E4: B3410056  sth r26, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[26].u16 ) };
	// 824D36E8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D36EC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824D36F0: 7D69512E  stwx r11, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 824D36F4: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D36F8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824D36FC: 915D0004  stw r10, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824D3700: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D3704: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3708: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824D370C: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 824D3710: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D3714: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824D3718: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824D371C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D3720: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3724: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824D3728: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 824D372C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D3730: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824D3734: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824D3738: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D373C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3740: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824D3744: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824D3748: 41980194  blt cr6, 0x824d38dc
	if ctx.cr[6].lt {
	pc = 0x824D38DC; continue 'dispatch;
	}
	// 824D374C: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3750: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824D3754: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D3758: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D375C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824D3760: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D3764: 80E300AC  lwz r7, 0xac(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 824D3768: A0C60008  lhz r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D376C: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 824D3770: 54C8103E  rotlwi r8, r6, 2
	ctx.r[8].u64 = ((ctx.r[6].u32).rotate_left(2)) as u64;
	// 824D3774: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 824D3778: A0E80000  lhz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D377C: B0E90000  sth r7, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 824D3780: A1080002  lhz r8, 2(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(2 as u32) ) } as u64;
	// 824D3784: B1090002  sth r8, 2(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 824D3788: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D378C: 80FD0000  lwz r7, 0(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3790: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824D3794: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824D3798: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 824D379C: 913D0004  stw r9, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824D37A0: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 824D37A4: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D37A8: 80E300AC  lwz r7, 0xac(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 824D37AC: A108000A  lhz r8, 0xa(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(10 as u32) ) } as u64;
	// 824D37B0: 5508103E  rotlwi r8, r8, 2
	ctx.r[8].u64 = ((ctx.r[8].u32).rotate_left(2)) as u64;
	// 824D37B4: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 824D37B8: A0E80000  lhz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D37BC: B0E90000  sth r7, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 824D37C0: A1080002  lhz r8, 2(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(2 as u32) ) } as u64;
	// 824D37C4: B1090002  sth r8, 2(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 824D37C8: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D37CC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824D37D0: 913D0004  stw r9, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824D37D4: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D37D8: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D37DC: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D37E0: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D37E4: 80E300B8  lwz r7, 0xb8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 824D37E8: A0C60000  lhz r6, 0(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D37EC: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 824D37F0: 54C8103E  rotlwi r8, r6, 2
	ctx.r[8].u64 = ((ctx.r[6].u32).rotate_left(2)) as u64;
	// 824D37F4: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 824D37F8: A0E80000  lhz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D37FC: B0E90000  sth r7, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 824D3800: A1080002  lhz r8, 2(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(2 as u32) ) } as u64;
	// 824D3804: B1090002  sth r8, 2(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 824D3808: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D380C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824D3810: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824D3814: 5529003E  slwi r9, r9, 0
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D3818: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D381C: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3820: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D3824: 80E300B8  lwz r7, 0xb8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 824D3828: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 824D382C: A0C60004  lhz r6, 4(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D3830: 54C8103E  rotlwi r8, r6, 2
	ctx.r[8].u64 = ((ctx.r[6].u32).rotate_left(2)) as u64;
	// 824D3834: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 824D3838: A0E80000  lhz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D383C: B0E90000  sth r7, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 824D3840: A1080002  lhz r8, 2(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(2 as u32) ) } as u64;
	// 824D3844: B1090002  sth r8, 2(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 824D3848: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D384C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824D3850: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824D3854: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3858: 810300C4  lwz r8, 0xc4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 824D385C: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3860: A1290002  lhz r9, 2(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(2 as u32) ) } as u64;
	// 824D3864: 5529103E  rotlwi r9, r9, 2
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(2)) as u64;
	// 824D3868: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 824D386C: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D3870: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824D3874: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 824D3878: A0E90000  lhz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D387C: B0E80000  sth r7, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 824D3880: A1290002  lhz r9, 2(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(2 as u32) ) } as u64;
	// 824D3884: B1280002  sth r9, 2(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 824D3888: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D388C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824D3890: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824D3894: 5529003E  slwi r9, r9, 0
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D3898: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D389C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824D38A0: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D38A4: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D38A8: 80E300C4  lwz r7, 0xc4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 824D38AC: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 824D38B0: A0C60006  lhz r6, 6(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(6 as u32) ) } as u64;
	// 824D38B4: 54C8103E  rotlwi r8, r6, 2
	ctx.r[8].u64 = ((ctx.r[6].u32).rotate_left(2)) as u64;
	// 824D38B8: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 824D38BC: A0E80000  lhz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D38C0: B0E90000  sth r7, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 824D38C4: A1080002  lhz r8, 2(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(2 as u32) ) } as u64;
	// 824D38C8: B1090002  sth r8, 2(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 824D38CC: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D38D0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824D38D4: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824D38D8: 4098FE74  bge cr6, 0x824d374c
	if !ctx.cr[6].lt {
	pc = 0x824D374C; continue 'dispatch;
	}
	// 824D38DC: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 824D38E0: 3B600003  li r27, 3
	ctx.r[27].s64 = 3;
	// 824D38E4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 824D38E8: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D38EC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824D38F0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824D38F4: 9B4A0000  stb r26, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[26].u8 ) };
	// 824D38F8: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D38FC: 40990018  ble cr6, 0x824d3914
	if !ctx.cr[6].gt {
	pc = 0x824D3914; continue 'dispatch;
	}
	// 824D3900: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3904: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 824D3908: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D390C: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 824D3910: 48000B91  bl 0x824d44a0
	ctx.lr = 0x824D3914;
	sub_824D44A0(ctx, base);
	// 824D3914: 3B7BFFFF  addi r27, r27, -1
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	// 824D3918: 3B9C000C  addi r28, r28, 0xc
	ctx.r[28].s64 = ctx.r[28].s64 + 12;
	// 824D391C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 824D3920: 409AFFC4  bne cr6, 0x824d38e4
	if !ctx.cr[6].eq {
	pc = 0x824D38E4; continue 'dispatch;
	}
	// 824D3924: 3940FFFC  li r10, -4
	ctx.r[10].s64 = -4;
	// 824D3928: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D392C: B3410056  sth r26, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[26].u16 ) };
	// 824D3930: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D3934: B1410054  sth r10, 0x54(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u16 ) };
	// 824D3938: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D393C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824D3940: 7D69512E  stwx r11, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 824D3944: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D3948: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824D394C: 915D0004  stw r10, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824D3950: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D3954: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3958: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824D395C: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 824D3960: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D3964: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824D3968: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824D396C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D3970: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3974: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824D3978: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 824D397C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D3980: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824D3984: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824D3988: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824D398C: 48061774  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D3990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D3990 size=928
    let mut pc: u32 = 0x824D3990;
    'dispatch: loop {
        match pc {
            0x824D3990 => {
    //   block [0x824D3990..0x824D3D30)
	// 824D3990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D3994: 48061719  bl 0x825350ac
	ctx.lr = 0x824D3998;
	sub_82535080(ctx, base);
	// 824D3998: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D399C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D39A0: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D39A4: 3B400010  li r26, 0x10
	ctx.r[26].s64 = 16;
	// 824D39A8: 83BF00A4  lwz r29, 0xa4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 824D39AC: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824D39B0: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 824D39B4: 55642036  slwi r4, r11, 4
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824D39B8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 824D39BC: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 824D39C0: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 824D39C4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824D39C8: 41990010  bgt cr6, 0x824d39d8
	if ctx.cr[6].gt {
	pc = 0x824D39D8; continue 'dispatch;
	}
	// 824D39CC: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 824D39D0: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 824D39D4: 48000018  b 0x824d39ec
	pc = 0x824D39EC; continue 'dispatch;
	// 824D39D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D39DC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D39E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D39E4: 4E800421  bctrl
	ctx.lr = 0x824D39E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D39E8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824D39EC: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824D39F0: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 824D39F4: 557E1036  rlwinm r30, r11, 2, 0, 0x1b
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824D39F8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 824D39FC: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 824D3A00: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824D3A04: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824D3A08: 41990010  bgt cr6, 0x824d3a18
	if ctx.cr[6].gt {
	pc = 0x824D3A18; continue 'dispatch;
	}
	// 824D3A0C: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 824D3A10: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 824D3A14: 4800001C  b 0x824d3a30
	pc = 0x824D3A30; continue 'dispatch;
	// 824D3A18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3A1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824D3A20: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D3A24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D3A28: 4E800421  bctrl
	ctx.lr = 0x824D3A2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D3A2C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824D3A30: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824D3A34: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 824D3A38: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 824D3A3C: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824D3A40: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824D3A44: 41990010  bgt cr6, 0x824d3a54
	if ctx.cr[6].gt {
	pc = 0x824D3A54; continue 'dispatch;
	}
	// 824D3A48: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 824D3A4C: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 824D3A50: 4800001C  b 0x824d3a6c
	pc = 0x824D3A6C; continue 'dispatch;
	// 824D3A54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3A58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824D3A5C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D3A60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D3A64: 4E800421  bctrl
	ctx.lr = 0x824D3A68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D3A68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824D3A6C: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 824D3A70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 824D3A74: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D3A78: 4099007C  ble cr6, 0x824d3af4
	if !ctx.cr[6].gt {
	pc = 0x824D3AF4; continue 'dispatch;
	}
	// 824D3A7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824D3A80: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 824D3A84: 815F00A0  lwz r10, 0xa0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 824D3A88: 7D2BE214  add r9, r11, r28
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 824D3A8C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 824D3A90: 7CAB5214  add r5, r11, r10
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824D3A94: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 824D3A98: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 824D3A9C: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 824D3AA0: 80A90000  lwz r5, 0(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3AA4: 90AA0000  stw r5, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 824D3AA8: 80A90004  lwz r5, 4(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D3AAC: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 824D3AB0: 80A90008  lwz r5, 8(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D3AB4: 90AA0008  stw r5, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 824D3AB8: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 824D3ABC: 912A000C  stw r9, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824D3AC0: 815F00A0  lwz r10, 0xa0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 824D3AC4: 813F00AC  lwz r9, 0xac(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 824D3AC8: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824D3ACC: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 824D3AD0: A14A0008  lhz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D3AD4: 554A103E  rotlwi r10, r10, 2
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 824D3AD8: 7D4A4A2E  lhzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 824D3ADC: B0C80002  sth r6, 2(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[6].u16 ) };
	// 824D3AE0: B1480000  sth r10, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 824D3AE4: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 824D3AE8: 815F00A4  lwz r10, 0xa4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 824D3AEC: 7F075000  cmpw cr6, r7, r10
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824D3AF0: 4198FF94  blt cr6, 0x824d3a84
	if ctx.cr[6].lt {
	pc = 0x824D3A84; continue 'dispatch;
	}
	// 824D3AF4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 824D3AF8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824D3AFC: 397DFFFF  addi r11, r29, -1
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	// 824D3B00: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824D3B04: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 824D3B08: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D3B0C: 40990014  ble cr6, 0x824d3b20
	if !ctx.cr[6].gt {
	pc = 0x824D3B20; continue 'dispatch;
	}
	// 824D3B10: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 824D3B14: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D3B18: 387B0004  addi r3, r27, 4
	ctx.r[3].s64 = ctx.r[27].s64 + 4;
	// 824D3B1C: 48000985  bl 0x824d44a0
	ctx.lr = 0x824D3B20;
	sub_824D44A0(ctx, base);
	// 824D3B20: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 824D3B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 824D3B28: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D3B2C: 4099006C  ble cr6, 0x824d3b98
	if !ctx.cr[6].gt {
	pc = 0x824D3B98; continue 'dispatch;
	}
	// 824D3B30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 824D3B34: 393B0002  addi r9, r27, 2
	ctx.r[9].s64 = ctx.r[27].s64 + 2;
	// 824D3B38: A1690000  lhz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3B3C: 556B103E  rotlwi r11, r11, 2
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 824D3B40: 7D0BF12E  stwx r8, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[8].u32) };
	// 824D3B44: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 824D3B48: A1690000  lhz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3B4C: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 824D3B50: 815F00A0  lwz r10, 0xa0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 824D3B54: 556B203E  rotlwi r11, r11, 4
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(4)) as u64;
	// 824D3B58: 7D475214  add r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 824D3B5C: 7CCBE214  add r6, r11, r28
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 824D3B60: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 824D3B64: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 824D3B68: 38E70010  addi r7, r7, 0x10
	ctx.r[7].s64 = ctx.r[7].s64 + 16;
	// 824D3B6C: 80CA0000  lwz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3B70: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 824D3B74: 80CA0004  lwz r6, 4(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D3B78: 90CB0004  stw r6, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 824D3B7C: 80CA0008  lwz r6, 8(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D3B80: 90CB0008  stw r6, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 824D3B84: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824D3B88: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 824D3B8C: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 824D3B90: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824D3B94: 4198FFA4  blt cr6, 0x824d3b38
	if ctx.cr[6].lt {
	pc = 0x824D3B38; continue 'dispatch;
	}
	// 824D3B98: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 824D3B9C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824D3BA0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824D3BA4: 4099004C  ble cr6, 0x824d3bf0
	if !ctx.cr[6].gt {
	pc = 0x824D3BF0; continue 'dispatch;
	}
	// 824D3BA8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 824D3BAC: 817F00A0  lwz r11, 0xa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 824D3BB0: 7D485A14  add r10, r8, r11
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 824D3BB4: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824D3BB8: 556707FE  clrlwi r7, r11, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 824D3BBC: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 824D3BC0: 409A0010  bne cr6, 0x824d3bd0
	if !ctx.cr[6].eq {
	pc = 0x824D3BD0; continue 'dispatch;
	}
	// 824D3BC4: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D3BC8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824D3BCC: 48000010  b 0x824d3bdc
	pc = 0x824D3BDC; continue 'dispatch;
	// 824D3BD0: 815F00D8  lwz r10, 0xd8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 824D3BD4: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824D3BD8: 7D2B532E  sthx r9, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u16) };
	// 824D3BDC: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 824D3BE0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824D3BE4: 39080010  addi r8, r8, 0x10
	ctx.r[8].s64 = ctx.r[8].s64 + 16;
	// 824D3BE8: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824D3BEC: 4198FFC0  blt cr6, 0x824d3bac
	if ctx.cr[6].lt {
	pc = 0x824D3BAC; continue 'dispatch;
	}
	// 824D3BF0: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 824D3BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824D3BF8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D3BFC: 40990060  ble cr6, 0x824d3c5c
	if !ctx.cr[6].gt {
	pc = 0x824D3C5C; continue 'dispatch;
	}
	// 824D3C00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 824D3C04: 817F00D8  lwz r11, 0xd8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 824D3C08: 7D075A14  add r8, r7, r11
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 824D3C0C: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D3C10: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824D3C14: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D3C18: 41980030  blt cr6, 0x824d3c48
	if ctx.cr[6].lt {
	pc = 0x824D3C48; continue 'dispatch;
	}
	// 824D3C1C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824D3C20: 81280004  lwz r9, 4(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D3C24: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824D3C28: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824D3C2C: 394AFFFE  addi r10, r10, -2
	ctx.r[10].s64 = ctx.r[10].s64 + -2;
	// 824D3C30: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D3C34: A0A90000  lhz r5, 0(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3C38: 54A5103E  rotlwi r5, r5, 2
	ctx.r[5].u64 = ((ctx.r[5].u32).rotate_left(2)) as u64;
	// 824D3C3C: 7CA5F02E  lwzx r5, r5, r30
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824D3C40: B0A90000  sth r5, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 824D3C44: 4098FFDC  bge cr6, 0x824d3c20
	if !ctx.cr[6].lt {
	pc = 0x824D3C20; continue 'dispatch;
	}
	// 824D3C48: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 824D3C4C: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 824D3C50: 38E70010  addi r7, r7, 0x10
	ctx.r[7].s64 = ctx.r[7].s64 + 16;
	// 824D3C54: 7F065800  cmpw cr6, r6, r11
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824D3C58: 4198FFAC  blt cr6, 0x824d3c04
	if ctx.cr[6].lt {
	pc = 0x824D3C04; continue 'dispatch;
	}
	// 824D3C5C: 393F00B0  addi r9, r31, 0xb0
	ctx.r[9].s64 = ctx.r[31].s64 + 176;
	// 824D3C60: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 824D3C64: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824D3C6C: 8169FFFC  lwz r11, -4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-4 as u32) ) } as u64;
	// 824D3C70: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 824D3C74: 4099002C  ble cr6, 0x824d3ca0
	if !ctx.cr[6].gt {
	pc = 0x824D3CA0; continue 'dispatch;
	}
	// 824D3C78: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 824D3C7C: A0EB0000  lhz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3C80: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824D3C84: 54E7103E  rotlwi r7, r7, 2
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(2)) as u64;
	// 824D3C88: 7CE7F02E  lwzx r7, r7, r30
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824D3C8C: B0EB0000  sth r7, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 824D3C90: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824D3C94: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3C98: 7F0A3800  cmpw cr6, r10, r7
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[7].s32, &mut ctx.xer);
	// 824D3C9C: 4198FFE0  blt cr6, 0x824d3c7c
	if ctx.cr[6].lt {
	pc = 0x824D3C7C; continue 'dispatch;
	}
	// 824D3CA0: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 824D3CA4: 3929000C  addi r9, r9, 0xc
	ctx.r[9].s64 = ctx.r[9].s64 + 12;
	// 824D3CA8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 824D3CAC: 409AFFB8  bne cr6, 0x824d3c64
	if !ctx.cr[6].eq {
	pc = 0x824D3C64; continue 'dispatch;
	}
	// 824D3CB0: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824D3CB4: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 824D3CB8: 93C30020  stw r30, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 824D3CBC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824D3CC0: 409A0018  bne cr6, 0x824d3cd8
	if !ctx.cr[6].eq {
	pc = 0x824D3CD8; continue 'dispatch;
	}
	// 824D3CC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3CC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824D3CCC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824D3CD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D3CD4: 4E800421  bctrl
	ctx.lr = 0x824D3CD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D3CD8: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824D3CDC: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 824D3CE0: 93630020  stw r27, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[27].u32 ) };
	// 824D3CE4: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824D3CE8: 409A0018  bne cr6, 0x824d3d00
	if !ctx.cr[6].eq {
	pc = 0x824D3D00; continue 'dispatch;
	}
	// 824D3CEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3CF0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824D3CF4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824D3CF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D3CFC: 4E800421  bctrl
	ctx.lr = 0x824D3D00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D3D00: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824D3D04: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 824D3D08: 93830020  stw r28, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[28].u32 ) };
	// 824D3D0C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824D3D10: 409A0018  bne cr6, 0x824d3d28
	if !ctx.cr[6].eq {
	pc = 0x824D3D28; continue 'dispatch;
	}
	// 824D3D14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3D18: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824D3D1C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824D3D20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D3D24: 4E800421  bctrl
	ctx.lr = 0x824D3D28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D3D28: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824D3D2C: 480613D0  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D3D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D3D30 size=1604
    let mut pc: u32 = 0x824D3D30;
    'dispatch: loop {
        match pc {
            0x824D3D30 => {
    //   block [0x824D3D30..0x824D4374)
	// 824D3D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D3D34: 48061365  bl 0x82535098
	ctx.lr = 0x824D3D38;
	sub_82535080(ctx, base);
	// 824D3D38: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D3D3C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824D3D40: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3D44: 3AE00010  li r23, 0x10
	ctx.r[23].s64 = 16;
	// 824D3D48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824D3D4C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 824D3D50: 7CB42B78  mr r20, r5
	ctx.r[20].u64 = ctx.r[5].u64;
	// 824D3D54: 83FC00A4  lwz r31, 0xa4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(164 as u32) ) } as u64;
	// 824D3D58: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 824D3D5C: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 824D3D60: 93210064  stw r25, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[25].u32 ) };
	// 824D3D64: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824D3D68: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 824D3D6C: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 824D3D70: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 824D3D74: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824D3D78: 41990010  bgt cr6, 0x824d3d88
	if ctx.cr[6].gt {
	pc = 0x824D3D88; continue 'dispatch;
	}
	// 824D3D7C: 7D785B78  mr r24, r11
	ctx.r[24].u64 = ctx.r[11].u64;
	// 824D3D80: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 824D3D84: 48000018  b 0x824d3d9c
	pc = 0x824D3D9C; continue 'dispatch;
	// 824D3D88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3D8C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D3D90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D3D94: 4E800421  bctrl
	ctx.lr = 0x824D3D98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D3D98: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 824D3D9C: 83BC00A4  lwz r29, 0xa4(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(164 as u32) ) } as u64;
	// 824D3DA0: 67F58000  oris r21, r31, 0x8000
	ctx.r[21].u64 = ctx.r[31].u64 | 2147483648;
	// 824D3DA4: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 824D3DA8: 7FAA2E70  srawi r10, r29, 5
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[29].s32 >> 5) as i64;
	// 824D3DAC: 93010060  stw r24, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[24].u32 ) };
	// 824D3DB0: 9301006C  stw r24, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[24].u32 ) };
	// 824D3DB4: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 824D3DB8: 92A10068  stw r21, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[21].u32 ) };
	// 824D3DBC: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 824D3DC0: 55441036  rlwinm r4, r10, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 824D3DC4: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 824D3DC8: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 824D3DCC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824D3DD0: 41990010  bgt cr6, 0x824d3de0
	if ctx.cr[6].gt {
	pc = 0x824D3DE0; continue 'dispatch;
	}
	// 824D3DD4: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 824D3DD8: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 824D3DDC: 48000018  b 0x824d3df4
	pc = 0x824D3DF4; continue 'dispatch;
	// 824D3DE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D3DE4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D3DE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D3DEC: 4E800421  bctrl
	ctx.lr = 0x824D3DF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D3DF0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824D3DF4: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 824D3DF8: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D4378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D4378 size=296
    let mut pc: u32 = 0x824D4378;
    'dispatch: loop {
        match pc {
            0x824D4378 => {
    //   block [0x824D4378..0x824D44A0)
	// 824D4378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D437C: 48060D2D  bl 0x825350a8
	ctx.lr = 0x824D4380;
	sub_82535080(ctx, base);
	// 824D4380: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D4384: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4388: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824D438C: 3B400010  li r26, 0x10
	ctx.r[26].s64 = 16;
	// 824D4390: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824D4394: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824D4398: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 824D439C: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D43A0: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824D43A4: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 824D43A8: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 824D43AC: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824D43B0: 83C30020  lwz r30, 0x20(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 824D43B4: 8143002C  lwz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 824D43B8: 7D7E2214  add r11, r30, r4
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[4].u64;
	// 824D43BC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824D43C0: 4199000C  bgt cr6, 0x824d43cc
	if ctx.cr[6].gt {
	pc = 0x824D43CC; continue 'dispatch;
	}
	// 824D43C4: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 824D43C8: 48000018  b 0x824d43e0
	pc = 0x824D43E0; continue 'dispatch;
	// 824D43CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D43D0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D43D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D43D8: 4E800421  bctrl
	ctx.lr = 0x824D43DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D43DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824D43E0: 679C8000  oris r28, r28, 0x8000
	ctx.r[28].u64 = ctx.r[28].u64 | 2147483648;
	// 824D43E4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D43E8: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 824D43EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824D43F0: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 824D43F4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824D43F8: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 824D43FC: 40990044  ble cr6, 0x824d4440
	if !ctx.cr[6].gt {
	pc = 0x824D4440; continue 'dispatch;
	}
	// 824D4400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824D4404: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4408: 57A7103A  slwi r7, r29, 2
	ctx.r[7].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824D440C: 811B00A0  lwz r8, 0xa0(r27)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(160 as u32) ) } as u64;
	// 824D4410: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824D4414: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824D4418: 7D29502E  lwzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824D441C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824D4420: 81290014  lwz r9, 0x14(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D4424: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D4428: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 824D442C: 7D27F12E  stwx r9, r7, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[30].u32), ctx.r[9].u32) };
	// 824D4430: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4434: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824D4438: 4198FFCC  blt cr6, 0x824d4404
	if ctx.cr[6].lt {
	pc = 0x824D4404; continue 'dispatch;
	}
	// 824D443C: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 824D4440: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 824D4444: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824D4448: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824D444C: 4BFFF205  bl 0x824d3650
	ctx.lr = 0x824D4450;
	sub_824D3650(ctx, base);
	// 824D4450: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824D4454: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 824D4458: 93C30020  stw r30, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 824D445C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824D4460: 409A0018  bne cr6, 0x824d4478
	if !ctx.cr[6].eq {
	pc = 0x824D4478; continue 'dispatch;
	}
	// 824D4464: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4468: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824D446C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824D4470: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D4474: 4E800421  bctrl
	ctx.lr = 0x824D4478;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D4478: 578B0000  rlwinm r11, r28, 0, 0, 0
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	// 824D447C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D4480: 409A0018  bne cr6, 0x824d4498
	if !ctx.cr[6].eq {
	pc = 0x824D4498; continue 'dispatch;
	}
	// 824D4484: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824D4488: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824D448C: 5785103A  slwi r5, r28, 2
	ctx.r[5].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824D4490: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824D4494: 4BF8FC25  bl 0x824640b8
	ctx.lr = 0x824D4498;
	sub_824640B8(ctx, base);
	// 824D4498: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 824D449C: 48060C5C  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D44A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824D44A0 size=256
    let mut pc: u32 = 0x824D44A0;
    'dispatch: loop {
        match pc {
            0x824D44A0 => {
    //   block [0x824D44A0..0x824D45A0)
	// 824D44A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D44A4: 48060C15  bl 0x825350b8
	ctx.lr = 0x824D44A8;
	sub_82535080(ctx, base);
	// 824D44A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D44AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824D44B0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824D44B4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 824D44B8: 7D64EA14  add r11, r4, r29
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[29].u64;
	// 824D44BC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824D44C0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 824D44C4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824D44C8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D44CC: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824D44D0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 824D44D4: A1010050  lhz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D44D8: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D44DC: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824D44E0: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D44E4: 7D485010  subfc r10, r8, r10
	ctx.xer.ca = ctx.r[10].u32 >= ctx.r[8].u32;
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 824D44E8: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 824D44EC: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 824D44F0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824D44F4: 419A0010  beq cr6, 0x824d4504
	if ctx.cr[6].eq {
	pc = 0x824D4504; continue 'dispatch;
	}
	// 824D44F8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 824D44FC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824D4500: 4BFFFFE0  b 0x824d44e0
	pc = 0x824D44E0; continue 'dispatch;
	// 824D4504: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D4508: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824D450C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4510: 7D4A4010  subfc r10, r10, r8
	ctx.xer.ca = ctx.r[8].u32 >= ctx.r[10].u32;
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 824D4514: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 824D4518: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 824D451C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824D4520: 419A0010  beq cr6, 0x824d4530
	if ctx.cr[6].eq {
	pc = 0x824D4530; continue 'dispatch;
	}
	// 824D4524: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 824D4528: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 824D452C: 4BFFFFE0  b 0x824d450c
	pc = 0x824D450C; continue 'dispatch;
	// 824D4530: 7F05F800  cmpw cr6, r5, r31
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[31].s32, &mut ctx.xer);
	// 824D4534: 41980040  blt cr6, 0x824d4574
	if ctx.cr[6].lt {
	pc = 0x824D4574; continue 'dispatch;
	}
	// 824D4538: 419A002C  beq cr6, 0x824d4564
	if ctx.cr[6].eq {
	pc = 0x824D4564; continue 'dispatch;
	}
	// 824D453C: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D4540: 54AA103A  slwi r10, r5, 2
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824D4544: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824D4548: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 824D454C: A0EB0000  lhz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4550: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4554: B0EA0000  sth r7, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 824D4558: A0EB0002  lhz r7, 2(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 824D455C: B0EA0002  sth r7, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[7].u16 ) };
	// 824D4560: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824D4564: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 824D4568: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 824D456C: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 824D4570: 4099FF68  ble cr6, 0x824d44d8
	if !ctx.cr[6].gt {
	pc = 0x824D44D8; continue 'dispatch;
	}
	// 824D4574: 7F042800  cmpw cr6, r4, r5
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[5].s32, &mut ctx.xer);
	// 824D4578: 40980010  bge cr6, 0x824d4588
	if !ctx.cr[6].lt {
	pc = 0x824D4588; continue 'dispatch;
	}
	// 824D457C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 824D4580: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824D4584: 4BFFFF1D  bl 0x824d44a0
	ctx.lr = 0x824D4588;
	sub_824D44A0(ctx, base);
	// 824D4588: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 824D458C: 4098000C  bge cr6, 0x824d4598
	if !ctx.cr[6].lt {
	pc = 0x824D4598; continue 'dispatch;
	}
	// 824D4590: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824D4594: 4BFFFF24  b 0x824d44b8
	pc = 0x824D44B8; continue 'dispatch;
	// 824D4598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824D459C: 48060B6C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D45A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D45A0 size=28
    let mut pc: u32 = 0x824D45A0;
    'dispatch: loop {
        match pc {
            0x824D45A0 => {
    //   block [0x824D45A0..0x824D45BC)
	// 824D45A0: 39600070  li r11, 0x70
	ctx.r[11].s64 = 112;
	// 824D45A4: 39400080  li r10, 0x80
	ctx.r[10].s64 = 128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D45C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D45C0 size=100
    let mut pc: u32 = 0x824D45C0;
    'dispatch: loop {
        match pc {
            0x824D45C0 => {
    //   block [0x824D45C0..0x824D4624)
	// 824D45C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D45C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D45C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824D45CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D45D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D45D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D45D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824D45DC: 4BFFE5FD  bl 0x824d2bd8
	ctx.lr = 0x824D45E0;
	sub_824D2BD8(ctx, base);
	// 824D45E0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824D45E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D45E8: 419A0020  beq cr6, 0x824d4608
	if ctx.cr[6].eq {
	pc = 0x824D4608; continue 'dispatch;
	}
	// 824D45EC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D45F0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824D45F4: 38C00021  li r6, 0x21
	ctx.r[6].s64 = 33;
	// 824D45F8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D45FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824D4600: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824D4604: 4BF8FAB5  bl 0x824640b8
	ctx.lr = 0x824D4608;
	sub_824640B8(ctx, base);
	// 824D4608: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D460C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824D4610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D4614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D4618: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824D461C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D4620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D4628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D4628 size=4
    let mut pc: u32 = 0x824D4628;
    'dispatch: loop {
        match pc {
            0x824D4628 => {
    //   block [0x824D4628..0x824D462C)
	// 824D4628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D4630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D4630 size=36
    let mut pc: u32 = 0x824D4630;
    'dispatch: loop {
        match pc {
            0x824D4630 => {
    //   block [0x824D4630..0x824D4654)
	// 824D4630: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D4634: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4638: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D463C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4640: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824D4644: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 824D4648: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824D464C: 386BFF80  addi r3, r11, -0x80
	ctx.r[3].s64 = ctx.r[11].s64 + -128;
	// 824D4650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D4658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D4658 size=44
    let mut pc: u32 = 0x824D4658;
    'dispatch: loop {
        match pc {
            0x824D4658 => {
    //   block [0x824D4658..0x824D4684)
	// 824D4658: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D465C: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4660: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824D4664: 5529183E  rotlwi r9, r9, 3
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(3)) as u64;
	// 824D4668: 556B183E  rotlwi r11, r11, 3
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(3)) as u64;
	// 824D466C: 814A004C  lwz r10, 0x4c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 824D4670: 7C69512E  stwx r3, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[3].u32) };
	// 824D4674: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D4678: 814A004C  lwz r10, 0x4c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 824D467C: 7C6A592E  stwx r3, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 824D4680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D4688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D4688 size=40
    let mut pc: u32 = 0x824D4688;
    'dispatch: loop {
        match pc {
            0x824D4688 => {
    //   block [0x824D4688..0x824D46B0)
	// 824D4688: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D468C: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 824D4690: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 824D4694: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824D4698: 419A0020  beq cr6, 0x824d46b8
	if ctx.cr[6].eq {
		sub_824D46B8(ctx, base);
		return;
	}
	// 824D469C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 824D46A0: 419A0010  beq cr6, 0x824d46b0
	if ctx.cr[6].eq {
		sub_824D46B0(ctx, base);
		return;
	}
	// 824D46A4: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 824D46A8: 419A0010  beq cr6, 0x824d46b8
	if ctx.cr[6].eq {
		sub_824D46B8(ctx, base);
		return;
	}
	// 824D46AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D46B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D46B0 size=8
    let mut pc: u32 = 0x824D46B0;
    'dispatch: loop {
        match pc {
            0x824D46B0 => {
    //   block [0x824D46B0..0x824D46B8)
	// 824D46B0: 38830030  addi r4, r3, 0x30
	ctx.r[4].s64 = ctx.r[3].s64 + 48;
	// 824D46B4: 48000008  b 0x824d46bc
	sub_824D46B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D46B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D46B8 size=40
    let mut pc: u32 = 0x824D46B8;
    'dispatch: loop {
        match pc {
            0x824D46B8 => {
    //   block [0x824D46B8..0x824D46E0)
	// 824D46B8: 38830020  addi r4, r3, 0x20
	ctx.r[4].s64 = ctx.r[3].s64 + 32;
	// 824D46BC: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 824D46C0: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D46C4: 5568103E  rotlwi r8, r11, 2
	ctx.r[8].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 824D46C8: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 824D46CC: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D46D0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824D46D4: 816B16C8  lwz r11, 0x16c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5832 as u32) ) } as u64;
	// 824D46D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D46DC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D46E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D46E0 size=20
    let mut pc: u32 = 0x824D46E0;
    'dispatch: loop {
        match pc {
            0x824D46E0 => {
    //   block [0x824D46E0..0x824D46F4)
	// 824D46E0: 81030008  lwz r8, 8(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D46E4: 80C30014  lwz r6, 0x14(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D46E8: 80A30010  lwz r5, 0x10(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D46EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D46F0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D46F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D46F8 size=36
    let mut pc: u32 = 0x824D46F8;
    'dispatch: loop {
        match pc {
            0x824D46F8 => {
    //   block [0x824D46F8..0x824D471C)
	// 824D46F8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D46FC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 824D4700: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824D4704: 419A0064  beq cr6, 0x824d4768
	if ctx.cr[6].eq {
		sub_824D4768(ctx, base);
		return;
	}
	// 824D4708: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 824D470C: 419A0010  beq cr6, 0x824d471c
	if ctx.cr[6].eq {
		sub_824D471C(ctx, base);
		return;
	}
	// 824D4710: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 824D4714: 419A0054  beq cr6, 0x824d4768
	if ctx.cr[6].eq {
		sub_824D4768(ctx, base);
		return;
	}
	// 824D4718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D471C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824D471C size=64
    let mut pc: u32 = 0x824D471C;
    'dispatch: loop {
        match pc {
            0x824D471C => {
    //   block [0x824D471C..0x824D475C)
	// 824D471C: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824D4720: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 824D4724: C00A2074  lfs f0, 0x2074(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D4728: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D475C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D475C size=12
    let mut pc: u32 = 0x824D475C;
    'dispatch: loop {
        match pc {
            0x824D475C => {
    //   block [0x824D475C..0x824D4768)
	// 824D475C: 38830030  addi r4, r3, 0x30
	ctx.r[4].s64 = ctx.r[3].s64 + 48;
	// 824D4760: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D4764: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D4768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D4768 size=36
    let mut pc: u32 = 0x824D4768;
    'dispatch: loop {
        match pc {
            0x824D4768 => {
    //   block [0x824D4768..0x824D478C)
	// 824D4768: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 824D476C: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4770: 396B0049  addi r11, r11, 0x49
	ctx.r[11].s64 = ctx.r[11].s64 + 73;
	// 824D4774: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824D4778: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824D477C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D4780: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 824D4784: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D4788: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D478C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D478C size=12
    let mut pc: u32 = 0x824D478C;
    'dispatch: loop {
        match pc {
            0x824D478C => {
    //   block [0x824D478C..0x824D4798)
	// 824D478C: 38830020  addi r4, r3, 0x20
	ctx.r[4].s64 = ctx.r[3].s64 + 32;
	// 824D4790: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D4794: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D4798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D4798 size=36
    let mut pc: u32 = 0x824D4798;
    'dispatch: loop {
        match pc {
            0x824D4798 => {
    //   block [0x824D4798..0x824D47BC)
	// 824D4798: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D479C: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 824D47A0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824D47A4: 419A0068  beq cr6, 0x824d480c
	if ctx.cr[6].eq {
		sub_824D480C(ctx, base);
		return;
	}
	// 824D47A8: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 824D47AC: 419A0010  beq cr6, 0x824d47bc
	if ctx.cr[6].eq {
		sub_824D47BC(ctx, base);
		return;
	}
	// 824D47B0: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 824D47B4: 419A0058  beq cr6, 0x824d480c
	if ctx.cr[6].eq {
		sub_824D480C(ctx, base);
		return;
	}
	// 824D47B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D47BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824D47BC size=24
    let mut pc: u32 = 0x824D47BC;
    'dispatch: loop {
        match pc {
            0x824D47BC => {
    //   block [0x824D47BC..0x824D47D4)
	// 824D47BC: C0030018  lfs f0, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D47C0: 38830030  addi r4, r3, 0x30
	ctx.r[4].s64 = ctx.r[3].s64 + 48;
	// 824D47C4: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 824D47C8: 409A000C  bne cr6, 0x824d47d4
	if !ctx.cr[6].eq {
		sub_824D47D4(ctx, base);
		return;
	}
	// 824D47CC: D0430018  stfs f2, 0x18(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824D47D0: 48000010  b 0x824d47e0
	sub_824D47D4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D47D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824D47D4 size=48
    let mut pc: u32 = 0x824D47D4;
    'dispatch: loop {
        match pc {
            0x824D47D4 => {
    //   block [0x824D47D4..0x824D4804)
	// 824D47D4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D47D8: C00B2074  lfs f0, 0x2074(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D47DC: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824D47E0: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 824D47E4: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D47E8: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 824D47EC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824D47F0: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D47F4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824D47F8: 816B16D4  lwz r11, 0x16d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5844 as u32) ) } as u64;
	// 824D47FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D4800: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D4804(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D4804 size=8
    let mut pc: u32 = 0x824D4804;
    'dispatch: loop {
        match pc {
            0x824D4804 => {
    //   block [0x824D4804..0x824D480C)
	// 824D4804: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D4808: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D480C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D480C size=36
    let mut pc: u32 = 0x824D480C;
    'dispatch: loop {
        match pc {
            0x824D480C => {
    //   block [0x824D480C..0x824D4830)
	// 824D480C: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 824D4810: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4814: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 824D4818: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824D481C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D4820: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824D4824: 816B16D4  lwz r11, 0x16d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5844 as u32) ) } as u64;
	// 824D4828: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D482C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D4830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D4830 size=12
    let mut pc: u32 = 0x824D4830;
    'dispatch: loop {
        match pc {
            0x824D4830 => {
    //   block [0x824D4830..0x824D483C)
	// 824D4830: 38830020  addi r4, r3, 0x20
	ctx.r[4].s64 = ctx.r[3].s64 + 32;
	// 824D4834: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D4838: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D4840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D4840 size=24
    let mut pc: u32 = 0x824D4840;
    'dispatch: loop {
        match pc {
            0x824D4840 => {
    //   block [0x824D4840..0x824D4858)
	// 824D4840: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D4844: 81440050  lwz r10, 0x50(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D4848: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824D484C: 4199000C  bgt cr6, 0x824d4858
	if ctx.cr[6].gt {
		sub_824D4858(ctx, base);
		return;
	}
	// 824D4850: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824D4854: 4800000C  b 0x824d4860
	sub_824D4858(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D4858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D4858 size=68
    let mut pc: u32 = 0x824D4858;
    'dispatch: loop {
        match pc {
            0x824D4858 => {
    //   block [0x824D4858..0x824D489C)
	// 824D4858: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 824D485C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824D4860: 812B0050  lwz r9, 0x50(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D4864: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824D4868: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824D486C: 40990028  ble cr6, 0x824d4894
	if !ctx.cr[6].gt {
	pc = 0x824D4894; continue 'dispatch;
	}
	// 824D4870: 810B004C  lwz r8, 0x4c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 824D4874: 39680004  addi r11, r8, 4
	ctx.r[11].s64 = ctx.r[8].s64 + 4;
	// 824D4878: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D487C: 7F072040  cmplw cr6, r7, r4
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824D4880: 419A001C  beq cr6, 0x824d489c
	if ctx.cr[6].eq {
		sub_824D489C(ctx, base);
		return;
	}
	// 824D4884: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824D4888: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824D488C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824D4890: 4198FFE8  blt cr6, 0x824d4878
	if ctx.cr[6].lt {
	pc = 0x824D4878; continue 'dispatch;
	}
	// 824D4894: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824D4898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D489C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D489C size=12
    let mut pc: u32 = 0x824D489C;
    'dispatch: loop {
        match pc {
            0x824D489C => {
    //   block [0x824D489C..0x824D48A8)
	// 824D489C: 554B1838  slwi r11, r10, 3
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D48A0: 7C6B402E  lwzx r3, r11, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 824D48A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D48A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D48A8 size=16
    let mut pc: u32 = 0x824D48A8;
    'dispatch: loop {
        match pc {
            0x824D48A8 => {
    //   block [0x824D48A8..0x824D48B8)
	// 824D48A8: 80A30008  lwz r5, 8(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D48AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 824D48B0: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 824D48B4: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D48B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D48B8 size=40
    let mut pc: u32 = 0x824D48B8;
    'dispatch: loop {
        match pc {
            0x824D48B8 => {
    //   block [0x824D48B8..0x824D48E0)
	// 824D48B8: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D48BC: 54A6003E  slwi r6, r5, 0
	ctx.r[6].u32 = ctx.r[5].u32.wrapping_shl(0);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824D48C0: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 824D48C4: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D48C8: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 824D48CC: 7F083000  cmpw cr6, r8, r6
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[6].s32, &mut ctx.xer);
	// 824D48D0: 409A0010  bne cr6, 0x824d48e0
	if !ctx.cr[6].eq {
		sub_824D48E0(ctx, base);
		return;
	}
	// 824D48D4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D48D8: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824D48DC: 48000008  b 0x824d48e4
	sub_824D48E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D48E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D48E0 size=40
    let mut pc: u32 = 0x824D48E0;
    'dispatch: loop {
        match pc {
            0x824D48E0 => {
    //   block [0x824D48E0..0x824D4908)
	// 824D48E0: 394B0200  addi r10, r11, 0x200
	ctx.r[10].s64 = ctx.r[11].s64 + 512;
	// 824D48E4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824D48E8: 40980014  bge cr6, 0x824d48fc
	if !ctx.cr[6].lt {
	pc = 0x824D48FC; continue 'dispatch;
	}
	// 824D48EC: 892B0003  lbz r9, 3(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 824D48F0: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824D48F4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824D48F8: 4198FFF4  blt cr6, 0x824d48ec
	if ctx.cr[6].lt {
	pc = 0x824D48EC; continue 'dispatch;
	}
	// 824D48FC: 7F082800  cmpw cr6, r8, r5
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[5].s32, &mut ctx.xer);
	// 824D4900: 4198FFC0  blt cr6, 0x824d48c0
	if ctx.cr[6].lt {
		sub_824D48B8(ctx, base);
		return;
	}
	// 824D4904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D4908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824D4908 size=212
    let mut pc: u32 = 0x824D4908;
    'dispatch: loop {
        match pc {
            0x824D4908 => {
    //   block [0x824D4908..0x824D49DC)
	// 824D4908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D490C: 480607A9  bl 0x825350b4
	ctx.lr = 0x824D4910;
	sub_82535080(ctx, base);
	// 824D4910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D4914: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D4918: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824D491C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824D4920: 3B9E0020  addi r28, r30, 0x20
	ctx.r[28].s64 = ctx.r[30].s64 + 32;
	// 824D4924: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4928: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824D492C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D4930: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D4934: C02A0050  lfs f1, 0x50(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824D4938: 386B0040  addi r3, r11, 0x40
	ctx.r[3].s64 = ctx.r[11].s64 + 64;
	// 824D493C: 480D4A6D  bl 0x825a93a8
	ctx.lr = 0x824D4940;
	sub_825A93A8(ctx, base);
	// 824D4940: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4944: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D4948: 3B7E0060  addi r27, r30, 0x60
	ctx.r[27].s64 = ctx.r[30].s64 + 96;
	// 824D494C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 824D4950: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D4954: C02A0050  lfs f1, 0x50(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824D4958: 386B0040  addi r3, r11, 0x40
	ctx.r[3].s64 = ctx.r[11].s64 + 64;
	// 824D495C: 480D4A4D  bl 0x825a93a8
	ctx.lr = 0x824D4960;
	sub_825A93A8(ctx, base);
	// 824D4960: 397E0010  addi r11, r30, 0x10
	ctx.r[11].s64 = ctx.r[30].s64 + 16;
	// 824D4964: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 824D4968: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D496C: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824D4970: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824D4974: 915D000C  stw r10, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 824D4978: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D497C: 915D0008  stw r10, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824D4980: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4984: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4988: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D498C: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824D4990: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824D4994: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4998: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D499C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D49A0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824D49A4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824D49A8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D49AC: 939E0008  stw r28, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 824D49B0: 915E000C  stw r10, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 824D49B4: 419A0010  beq cr6, 0x824d49c4
	if ctx.cr[6].eq {
	pc = 0x824D49C4; continue 'dispatch;
	}
	// 824D49B8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D49BC: 936B0008  stw r27, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 824D49C0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 824D49C4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 824D49C8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824D49CC: 387D0010  addi r3, r29, 0x10
	ctx.r[3].s64 = ctx.r[29].s64 + 16;
	// 824D49D0: 480D4789  bl 0x825a9158
	ctx.lr = 0x824D49D4;
	sub_825A9158(ctx, base);
	// 824D49D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824D49D8: 4806072C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D49E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D49E0 size=180
    let mut pc: u32 = 0x824D49E0;
    'dispatch: loop {
        match pc {
            0x824D49E0 => {
    //   block [0x824D49E0..0x824D4A94)
	// 824D49E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D49E4: 480606D9  bl 0x825350bc
	ctx.lr = 0x824D49E8;
	sub_82535080(ctx, base);
	// 824D49E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D49EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824D49F0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D49F4: 2B0B0200  cmplwi cr6, r11, 0x200
	ctx.cr[6].compare_u32(ctx.r[11].u32, 512 as u32, &mut ctx.xer);
	// 824D49F8: 4098002C  bge cr6, 0x824d4a24
	if !ctx.cr[6].lt {
	pc = 0x824D4A24; continue 'dispatch;
	}
	// 824D49FC: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D4A00: 390B0080  addi r8, r11, 0x80
	ctx.r[8].s64 = ctx.r[11].s64 + 128;
	// 824D4A04: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4A08: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824D4A0C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824D4A10: 814AFFFC  lwz r10, -4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) } as u64;
	// 824D4A14: 911E0000  stw r8, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824D4A18: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824D4A1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824D4A20: 480606EC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 824D4A24: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4A28: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824D4A2C: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 824D4A30: 38800200  li r4, 0x200
	ctx.r[4].s64 = 512;
	// 824D4A34: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824D4A38: 4BF8F601  bl 0x82464038
	ctx.lr = 0x824D4A3C;
	sub_82464038(ctx, base);
	// 824D4A3C: 3BFE0004  addi r31, r30, 4
	ctx.r[31].s64 = ctx.r[30].s64 + 4;
	// 824D4A40: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824D4A44: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D4A48: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4A4C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824D4A50: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824D4A54: 409A0010  bne cr6, 0x824d4a64
	if !ctx.cr[6].eq {
	pc = 0x824D4A64; continue 'dispatch;
	}
	// 824D4A58: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824D4A5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D4A60: 4BF998F1  bl 0x8246e350
	ctx.lr = 0x824D4A64;
	sub_8246E350(ctx, base);
	// 824D4A64: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4A68: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
	// 824D4A6C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4A70: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824D4A74: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D4A78: 7FAB512E  stwx r29, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 824D4A7C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4A80: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824D4A84: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824D4A88: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824D4A8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824D4A90: 4806067C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D4A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D4A98 size=236
    let mut pc: u32 = 0x824D4A98;
    'dispatch: loop {
        match pc {
            0x824D4A98 => {
    //   block [0x824D4A98..0x824D4B84)
	// 824D4A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D4A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D4AA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D4AA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D4AA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D4AAC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4AB0: 356BFF80  addic. r11, r11, -0x80
	ctx.xer.ca = (ctx.r[11].u32 > (!(-128 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -128;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D4AB4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D4AB8: 40820044  bne 0x824d4afc
	if !ctx.cr[0].eq {
	pc = 0x824D4AFC; continue 'dispatch;
	}
	// 824D4ABC: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 824D4AC0: 80ED0000  lwz r7, 0(r13)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4AC4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 824D4AC8: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824D4ACC: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 824D4AD0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4AD4: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4AD8: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D4ADC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824D4AE0: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 824D4AE4: 8089FFFC  lwz r4, -4(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-4 as u32) ) } as u64;
	// 824D4AE8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824D4AEC: 7C63382E  lwzx r3, r3, r7
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 824D4AF0: 4BF8F5C9  bl 0x824640b8
	ctx.lr = 0x824D4AF4;
	sub_824640B8(ctx, base);
	// 824D4AF4: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 824D4AF8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D4AFC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 824D4B00: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D4B04: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 824D4B08: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D4B0C: 409A0064  bne cr6, 0x824d4b70
	if !ctx.cr[6].eq {
	pc = 0x824D4B70; continue 'dispatch;
	}
	// 824D4B10: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4B14: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 824D4B18: 5569843E  srwi r9, r11, 0x10
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D4B1C: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D4B20: 5569C23E  srwi r9, r11, 8
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D4B24: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D4B28: 5569E13E  srwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D4B2C: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D4B30: 5569F0BE  srwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D4B34: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D4B38: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D4B3C: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D4B40: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 824D4B44: 2F060001  cmpwi cr6, r6, 1
	ctx.cr[6].compare_i32(ctx.r[6].s32, 1, &mut ctx.xer);
	// 824D4B48: 40980010  bge cr6, 0x824d4b58
	if !ctx.cr[6].lt {
	pc = 0x824D4B58; continue 'dispatch;
	}
	// 824D4B4C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 824D4B50: 38A3000C  addi r5, r3, 0xc
	ctx.r[5].s64 = ctx.r[3].s64 + 12;
	// 824D4B54: 48000014  b 0x824d4b68
	pc = 0x824D4B68; continue 'dispatch;
	// 824D4B58: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 824D4B5C: 7F065800  cmpw cr6, r6, r11
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824D4B60: 40980010  bge cr6, 0x824d4b70
	if !ctx.cr[6].lt {
	pc = 0x824D4B70; continue 'dispatch;
	}
	// 824D4B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824D4B68: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824D4B6C: 4BF9987D  bl 0x8246e3e8
	ctx.lr = 0x824D4B70;
	sub_8246E3E8(ctx, base);
	// 824D4B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824D4B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D4B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D4B7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D4B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D4B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D4B88 size=52
    let mut pc: u32 = 0x824D4B88;
    'dispatch: loop {
        match pc {
            0x824D4B88 => {
    //   block [0x824D4B88..0x824D4BBC)
	// 824D4B88: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4B8C: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4B90: 81050010  lwz r8, 0x10(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D4B94: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4B98: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 824D4B9C: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824D4BA0: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 824D4BA4: 55482834  slwi r8, r10, 5
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824D4BA8: 7D084A14  add r8, r8, r9
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 824D4BAC: 7D085A14  add r8, r8, r11
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 824D4BB0: 419A000C  beq cr6, 0x824d4bbc
	if ctx.cr[6].eq {
		sub_824D4BBC(ctx, base);
		return;
	}
	// 824D4BB4: 888812B0  lbz r4, 0x12b0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(4784 as u32) ) } as u64;
	// 824D4BB8: 48000008  b 0x824d4bc0
	sub_824D4BBC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D4BBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D4BBC size=144
    let mut pc: u32 = 0x824D4BBC;
    'dispatch: loop {
        match pc {
            0x824D4BBC => {
    //   block [0x824D4BBC..0x824D4C4C)
	// 824D4BBC: 88880EB0  lbz r4, 0xeb0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(3760 as u32) ) } as u64;
	// 824D4BC0: 5488103A  slwi r8, r4, 2
	ctx.r[8].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824D4BC4: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 824D4BC8: 7D044214  add r8, r4, r8
	ctx.r[8].u64 = ctx.r[4].u64 + ctx.r[8].u64;
	// 824D4BCC: 55082036  slwi r8, r8, 4
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824D4BD0: 7D085A14  add r8, r8, r11
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 824D4BD4: 810816F0  lwz r8, 0x16f0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(5872 as u32) ) } as u64;
	// 824D4BD8: 3908FFFE  addi r8, r8, -2
	ctx.r[8].s64 = ctx.r[8].s64 + -2;
	// 824D4BDC: 7D080034  cntlzw r8, r8
	ctx.r[8].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 824D4BE0: 5508DFFE  rlwinm r8, r8, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 824D4BE4: 409A0038  bne cr6, 0x824d4c1c
	if !ctx.cr[6].eq {
	pc = 0x824D4C1C; continue 'dispatch;
	}
	// 824D4BE8: 390A000D  addi r8, r10, 0xd
	ctx.r[8].s64 = ctx.r[10].s64 + 13;
	// 824D4BEC: 55082834  slwi r8, r8, 5
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(5);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824D4BF0: 7D084A14  add r8, r8, r9
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 824D4BF4: 7D0858AE  lbzx r8, r8, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824D4BF8: 3908007C  addi r8, r8, 0x7c
	ctx.r[8].s64 = ctx.r[8].s64 + 124;
	// 824D4BFC: 5503103A  slwi r3, r8, 2
	ctx.r[3].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824D4C00: 7D081A14  add r8, r8, r3
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[3].u64;
	// 824D4C04: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824D4C08: 7D0858AE  lbzx r8, r8, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824D4C0C: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 824D4C10: 7D080034  cntlzw r8, r8
	ctx.r[8].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 824D4C14: 5508DFFE  rlwinm r8, r8, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 824D4C18: 69080001  xori r8, r8, 1
	ctx.r[8].u64 = ctx.r[8].u64 ^ 1;
	// 824D4C1C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 824D4C20: 91070000  stw r8, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824D4C24: 419A0034  beq cr6, 0x824d4c58
	if ctx.cr[6].eq {
		sub_824D4C58(ctx, base);
		return;
	}
	// 824D4C28: 55292834  slwi r9, r9, 5
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D4C2C: 81050010  lwz r8, 0x10(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D4C30: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 824D4C34: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 824D4C38: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824D4C3C: 419A0010  beq cr6, 0x824d4c4c
	if ctx.cr[6].eq {
		sub_824D4C4C(ctx, base);
		return;
	}
	// 824D4C40: 896B12B0  lbz r11, 0x12b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4784 as u32) ) } as u64;
	// 824D4C44: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D4C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D4C4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D4C4C size=12
    let mut pc: u32 = 0x824D4C4C;
    'dispatch: loop {
        match pc {
            0x824D4C4C => {
    //   block [0x824D4C4C..0x824D4C58)
	// 824D4C4C: 896B0EB0  lbz r11, 0xeb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3760 as u32) ) } as u64;
	// 824D4C50: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D4C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D4C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D4C58 size=8
    let mut pc: u32 = 0x824D4C58;
    'dispatch: loop {
        match pc {
            0x824D4C58 => {
    //   block [0x824D4C58..0x824D4C60)
	// 824D4C58: 90860000  stw r4, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 824D4C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D4C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824D4C60 size=460
    let mut pc: u32 = 0x824D4C60;
    'dispatch: loop {
        match pc {
            0x824D4C60 => {
    //   block [0x824D4C60..0x824D4E2C)
	// 824D4C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D4C64: 48060441  bl 0x825350a4
	ctx.lr = 0x824D4C68;
	sub_82535080(ctx, base);
	// 824D4C68: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D4C6C: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 824D4C70: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824D4C74: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D4C78: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824D4C7C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 824D4C80: 82FC0000  lwz r23, 0(r28)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4C84: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 824D4C88: 4BFFFD59  bl 0x824d49e0
	ctx.lr = 0x824D4C8C;
	sub_824D49E0(ctx, base);
	// 824D4C8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D4C90: 3B1D004C  addi r24, r29, 0x4c
	ctx.r[24].s64 = ctx.r[29].s64 + 76;
	// 824D4C94: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 824D4C98: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 824D4C9C: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 824D4CA0: 817D0050  lwz r11, 0x50(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D4CA4: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 824D4CA8: 817E0050  lwz r11, 0x50(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D4CAC: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824D4CB0: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D4CB4: 81580004  lwz r10, 4(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4CB8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824D4CBC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824D4CC0: 409A0010  bne cr6, 0x824d4cd0
	if !ctx.cr[6].eq {
	pc = 0x824D4CD0; continue 'dispatch;
	}
	// 824D4CC4: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 824D4CC8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 824D4CCC: 4BF99685  bl 0x8246e350
	ctx.lr = 0x824D4CD0;
	sub_8246E350(ctx, base);
	// 824D4CD0: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4CD4: 3B7E004C  addi r27, r30, 0x4c
	ctx.r[27].s64 = ctx.r[30].s64 + 76;
	// 824D4CD8: 81580000  lwz r10, 0(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4CDC: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 824D4CE0: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D4CE4: 7F2B5214  add r25, r11, r10
	ctx.r[25].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824D4CE8: 91380004  stw r9, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824D4CEC: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D4CF0: 815B0004  lwz r10, 4(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4CF4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824D4CF8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824D4CFC: 409A0010  bne cr6, 0x824d4d0c
	if !ctx.cr[6].eq {
	pc = 0x824D4D0C; continue 'dispatch;
	}
	// 824D4D00: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 824D4D04: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824D4D08: 4BF99649  bl 0x8246e350
	ctx.lr = 0x824D4D0C;
	sub_8246E350(ctx, base);
	// 824D4D0C: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4D10: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 824D4D14: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4D18: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824D4D1C: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 824D4D20: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824D4D24: 911B0004  stw r8, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824D4D28: 93F90000  stw r31, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 824D4D2C: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 824D4D30: 93D90004  stw r30, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 824D4D34: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 824D4D38: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 824D4D3C: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 824D4D40: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D4D44: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D4D48: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D4D4C: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 824D4D50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 824D4D54: 480D4405  bl 0x825a9158
	ctx.lr = 0x824D4D58;
	sub_825A9158(ctx, base);
	// 824D4D58: A17D001A  lhz r11, 0x1a(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(26 as u32) ) } as u64;
	// 824D4D5C: A15E001A  lhz r10, 0x1a(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(26 as u32) ) } as u64;
	// 824D4D60: 5568103E  rotlwi r8, r11, 2
	ctx.r[8].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 824D4D64: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4D68: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 824D4D6C: 5748103A  slwi r8, r26, 2
	ctx.r[8].u32 = ctx.r[26].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824D4D70: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D4D74: 7D1A4214  add r8, r26, r8
	ctx.r[8].u64 = ctx.r[26].u64 + ctx.r[8].u64;
	// 824D4D78: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824D4D7C: 550B2036  slwi r11, r8, 4
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D4D80: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824D4D84: 7D2BBA14  add r9, r11, r23
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 824D4D88: 896A1BB0  lbz r11, 0x1bb0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(7088 as u32) ) } as u64;
	// 824D4D8C: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 824D4D90: 554A3032  slwi r10, r10, 6
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824D4D94: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 824D4D98: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4D9C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824D4DA0: 396B1C20  addi r11, r11, 0x1c20
	ctx.r[11].s64 = ctx.r[11].s64 + 7200;
	// 824D4DA4: 917C0060  stw r11, 0x60(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824D4DA8: 9B5F0001  stb r26, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[26].u8 ) };
	// 824D4DAC: 816916D8  lwz r11, 0x16d8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(5848 as u32) ) } as u64;
	// 824D4DB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D4DB4: 419A0030  beq cr6, 0x824d4de4
	if ctx.cr[6].eq {
	pc = 0x824D4DE4; continue 'dispatch;
	}
	// 824D4DB8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824D4DBC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 824D4DC0: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 824D4DC4: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 824D4DC8: C00A2074  lfs f0, 0x2074(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D4DCC: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824D4DD0: 993F0000  stb r9, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D4E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D4E30 size=580
    let mut pc: u32 = 0x824D4E30;
    'dispatch: loop {
        match pc {
            0x824D4E30 => {
    //   block [0x824D4E30..0x824D5074)
	// 824D4E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D4E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D4E38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824D4E3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D4E40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D4E44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824D4E48: 3BFE0004  addi r31, r30, 4
	ctx.r[31].s64 = ctx.r[30].s64 + 4;
	// 824D4E4C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4E50: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4E54: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4E58: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D4E5C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824D4E60: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 824D4E64: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824D4E68: 396BFF80  addi r11, r11, -0x80
	ctx.r[11].s64 = ctx.r[11].s64 + -128;
	// 824D4E6C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824D4E70: 419A012C  beq cr6, 0x824d4f9c
	if ctx.cr[6].eq {
	pc = 0x824D4F9C; continue 'dispatch;
	}
	// 824D4E74: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4E78: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824D4E7C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4E80: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824D4E84: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D4E88: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824D4E8C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824D4E90: 9144000C  stw r10, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 824D4E94: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D4E98: 91440010  stw r10, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824D4E9C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D4EA0: 91440014  stw r10, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 824D4EA4: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824D4EA8: 91440018  stw r10, 0x18(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 824D4EAC: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 824D4EB0: 9144001C  stw r10, 0x1c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 824D4EB4: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824D4EB8: 91440020  stw r10, 0x20(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 824D4EBC: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 824D4EC0: 91440024  stw r10, 0x24(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 824D4EC4: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 824D4EC8: 91440028  stw r10, 0x28(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 824D4ECC: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 824D4ED0: 9144002C  stw r10, 0x2c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 824D4ED4: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 824D4ED8: 91440030  stw r10, 0x30(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 824D4EDC: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 824D4EE0: 91440034  stw r10, 0x34(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 824D4EE4: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 824D4EE8: 91440038  stw r10, 0x38(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 824D4EEC: 814B003C  lwz r10, 0x3c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 824D4EF0: 9144003C  stw r10, 0x3c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 824D4EF4: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 824D4EF8: 91440040  stw r10, 0x40(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 824D4EFC: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 824D4F00: 91440044  stw r10, 0x44(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 824D4F04: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 824D4F08: 91440048  stw r10, 0x48(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 824D4F0C: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 824D4F10: 9144004C  stw r10, 0x4c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 824D4F14: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D4F18: 91440050  stw r10, 0x50(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 824D4F1C: 814B0054  lwz r10, 0x54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 824D4F20: 91440054  stw r10, 0x54(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 824D4F24: 814B0058  lwz r10, 0x58(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 824D4F28: 91440058  stw r10, 0x58(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 824D4F2C: 814B005C  lwz r10, 0x5c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 824D4F30: 9144005C  stw r10, 0x5c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 824D4F34: 814B0060  lwz r10, 0x60(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D4F38: 91440060  stw r10, 0x60(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 824D4F3C: 814B0064  lwz r10, 0x64(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 824D4F40: 91440064  stw r10, 0x64(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 824D4F44: 814B0068  lwz r10, 0x68(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 824D4F48: 91440068  stw r10, 0x68(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 824D4F4C: 814B006C  lwz r10, 0x6c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 824D4F50: 9144006C  stw r10, 0x6c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 824D4F54: 814B0070  lwz r10, 0x70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 824D4F58: 91440070  stw r10, 0x70(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 824D4F5C: 814B0074  lwz r10, 0x74(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 824D4F60: 91440074  stw r10, 0x74(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 824D4F64: 814B0078  lwz r10, 0x78(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 824D4F68: 91440078  stw r10, 0x78(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 824D4F6C: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 824D4F70: 9164007C  stw r11, 0x7c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 824D4F74: 81440010  lwz r10, 0x10(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D4F78: A1240004  lhz r9, 4(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4F7C: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 824D4F80: 5529183E  rotlwi r9, r9, 3
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(3)) as u64;
	// 824D4F84: 556B183E  rotlwi r11, r11, 3
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(3)) as u64;
	// 824D4F88: 814A004C  lwz r10, 0x4c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 824D4F8C: 7C89512E  stwx r4, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[4].u32) };
	// 824D4F90: 81440014  lwz r10, 0x14(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D4F94: 814A004C  lwz r10, 0x4c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 824D4F98: 7C8B512E  stwx r4, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[4].u32) };
	// 824D4F9C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4FA0: 356BFF80  addic. r11, r11, -0x80
	ctx.xer.ca = (ctx.r[11].u32 > (!(-128 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -128;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D4FA4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D4FA8: 40820040  bne 0x824d4fe8
	if !ctx.cr[0].eq {
	pc = 0x824D4FE8; continue 'dispatch;
	}
	// 824D4FAC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4FB0: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 824D4FB4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4FB8: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824D4FBC: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824D4FC0: 810D0000  lwz r8, 0(r13)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D4FC4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824D4FC8: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824D4FCC: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 824D4FD0: 808AFFFC  lwz r4, -4(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) } as u64;
	// 824D4FD4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824D4FD8: 7C67402E  lwzx r3, r7, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 824D4FDC: 4BF8F0DD  bl 0x824640b8
	ctx.lr = 0x824D4FE0;
	sub_824640B8(ctx, base);
	// 824D4FE0: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 824D4FE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D4FE8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D4FEC: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 824D4FF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D4FF4: 409A0068  bne cr6, 0x824d505c
	if !ctx.cr[6].eq {
	pc = 0x824D505C; continue 'dispatch;
	}
	// 824D4FF8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D4FFC: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 824D5000: 5569843E  srwi r9, r11, 0x10
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D5004: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D5008: 5569C23E  srwi r9, r11, 8
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D500C: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D5010: 5569E13E  srwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D5014: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D5018: 5569F0BE  srwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D501C: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D5020: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D5024: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D5028: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 824D502C: 2F060001  cmpwi cr6, r6, 1
	ctx.cr[6].compare_i32(ctx.r[6].s32, 1, &mut ctx.xer);
	// 824D5030: 40980010  bge cr6, 0x824d5040
	if !ctx.cr[6].lt {
	pc = 0x824D5040; continue 'dispatch;
	}
	// 824D5034: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 824D5038: 38BF000C  addi r5, r31, 0xc
	ctx.r[5].s64 = ctx.r[31].s64 + 12;
	// 824D503C: 48000014  b 0x824d5050
	pc = 0x824D5050; continue 'dispatch;
	// 824D5040: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 824D5044: 7F065800  cmpw cr6, r6, r11
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824D5048: 40980014  bge cr6, 0x824d505c
	if !ctx.cr[6].lt {
	pc = 0x824D505C; continue 'dispatch;
	}
	// 824D504C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824D5050: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824D5054: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D5058: 4BF99391  bl 0x8246e3e8
	ctx.lr = 0x824D505C;
	sub_8246E3E8(ctx, base);
	// 824D505C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824D5060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D5064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D5068: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824D506C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D5070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D5078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D5078 size=344
    let mut pc: u32 = 0x824D5078;
    'dispatch: loop {
        match pc {
            0x824D5078 => {
    //   block [0x824D5078..0x824D51D0)
	// 824D5078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D507C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D5080: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D5084: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D5088: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824D508C: 889F000D  lbz r4, 0xd(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(13 as u32) ) } as u64;
	// 824D5090: 4BFFF951  bl 0x824d49e0
	ctx.lr = 0x824D5094;
	sub_824D49E0(ctx, base);
	// 824D5094: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D5098: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D509C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D50A0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824D50A4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D50A8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824D50AC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824D50B0: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824D50B4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D50B8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824D50BC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D50C0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 824D50C4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824D50C8: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824D50CC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824D50D0: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 824D50D4: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824D50D8: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 824D50DC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 824D50E0: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 824D50E4: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824D50E8: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 824D50EC: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 824D50F0: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 824D50F4: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 824D50F8: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 824D50FC: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 824D5100: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 824D5104: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 824D5108: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 824D510C: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 824D5110: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 824D5114: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 824D5118: 91630040  stw r11, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 824D511C: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 824D5120: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 824D5124: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824D5128: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 824D512C: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 824D5130: 9163004C  stw r11, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 824D5134: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D5138: 91630050  stw r11, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 824D513C: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 824D5140: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824D5144: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 824D5148: 91630058  stw r11, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 824D514C: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 824D5150: 9163005C  stw r11, 0x5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 824D5154: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D5158: 91630060  stw r11, 0x60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824D515C: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 824D5160: 91630064  stw r11, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 824D5164: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 824D5168: 91630068  stw r11, 0x68(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 824D516C: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 824D5170: 9163006C  stw r11, 0x6c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 824D5174: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 824D5178: 91630070  stw r11, 0x70(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 824D517C: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 824D5180: 91630074  stw r11, 0x74(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 824D5184: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 824D5188: 91630078  stw r11, 0x78(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 824D518C: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 824D5190: 9163007C  stw r11, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 824D5194: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D5198: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D519C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824D51A0: 5529183E  rotlwi r9, r9, 3
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(3)) as u64;
	// 824D51A4: 556B183E  rotlwi r11, r11, 3
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(3)) as u64;
	// 824D51A8: 814A004C  lwz r10, 0x4c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 824D51AC: 7C69512E  stwx r3, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[3].u32) };
	// 824D51B0: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D51B4: 814A004C  lwz r10, 0x4c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 824D51B8: 7C6B512E  stwx r3, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[3].u32) };
	// 824D51BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824D51C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D51C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D51C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D51CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D51D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824D51D0 size=1016
    let mut pc: u32 = 0x824D51D0;
    'dispatch: loop {
        match pc {
            0x824D51D0 => {
    //   block [0x824D51D0..0x824D55C8)
	// 824D51D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D51D4: 4805FED5  bl 0x825350a8
	ctx.lr = 0x824D51D8;
	sub_82535080(ctx, base);
	// 824D51D8: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D51DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D51E0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824D51E4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824D51E8: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 824D51EC: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 824D51F0: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D51F4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824D51F8: 419A02D4  beq cr6, 0x824d54cc
	if ctx.cr[6].eq {
	pc = 0x824D54CC; continue 'dispatch;
	}
	// 824D51FC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 824D5200: 419A0034  beq cr6, 0x824d5234
	if ctx.cr[6].eq {
	pc = 0x824D5234; continue 'dispatch;
	}
	// 824D5204: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 824D5208: 409A03B8  bne cr6, 0x824d55c0
	if !ctx.cr[6].eq {
	pc = 0x824D55C0; continue 'dispatch;
	}
	// 824D520C: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824D5210: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824D5214: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D5218: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D521C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D5220: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824D5224: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D5228: 4E800421  bctrl
	ctx.lr = 0x824D522C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D522C: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 824D5230: 4805FEC8  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 824D5234: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D5238: C1BF0018  lfs f13, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D523C: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D5240: C19E0050  lfs f12, 0x50(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D5244: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D5248: 390B0008  addi r8, r11, 8
	ctx.r[8].s64 = ctx.r[11].s64 + 8;
	// 824D524C: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 824D5250: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 824D5254: 93A100B4  stw r29, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[29].u32 ) };
	// 824D5258: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 824D525C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824D5260: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 824D5264: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 824D5268: 3B7F0030  addi r27, r31, 0x30
	ctx.r[27].s64 = ctx.r[31].s64 + 48;
	// 824D526C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 824D5270: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824D5274: 83AB0008  lwz r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D5278: C01E0058  lfs f0, 0x58(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D527C: 838A0008  lwz r28, 8(r10)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D5280: 397D0040  addi r11, r29, 0x40
	ctx.r[11].s64 = ctx.r[29].s64 + 64;
	// 824D5284: 395C0040  addi r10, r28, 0x40
	ctx.r[10].s64 = ctx.r[28].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D55C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D55C8 size=352
    let mut pc: u32 = 0x824D55C8;
    'dispatch: loop {
        match pc {
            0x824D55C8 => {
    //   block [0x824D55C8..0x824D5728)
	// 824D55C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D55CC: 4805FAE9  bl 0x825350b4
	ctx.lr = 0x824D55D0;
	sub_82535080(ctx, base);
	// 824D55D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D55D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824D55D8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824D55DC: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 824D55E0: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D55E4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824D55E8: 419A0014  beq cr6, 0x824d55fc
	if ctx.cr[6].eq {
	pc = 0x824D55FC; continue 'dispatch;
	}
	// 824D55EC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 824D55F0: 419A0130  beq cr6, 0x824d5720
	if ctx.cr[6].eq {
	pc = 0x824D5720; continue 'dispatch;
	}
	// 824D55F4: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 824D55F8: 409A0030  bne cr6, 0x824d5628
	if !ctx.cr[6].eq {
	pc = 0x824D5628; continue 'dispatch;
	}
	// 824D55FC: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 824D5600: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 824D5604: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D5608: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D560C: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 824D5610: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824D5614: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D5618: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 824D561C: 816B16B4  lwz r11, 0x16b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5812 as u32) ) } as u64;
	// 824D5620: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D5624: 4E800421  bctrl
	ctx.lr = 0x824D5628;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D5628: 3BBF0004  addi r29, r31, 4
	ctx.r[29].s64 = ctx.r[31].s64 + 4;
	// 824D562C: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 824D5630: 3B800002  li r28, 2
	ctx.r[28].s64 = 2;
	// 824D5634: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D5638: A15D0000  lhz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D563C: 386B004C  addi r3, r11, 0x4c
	ctx.r[3].s64 = ctx.r[11].s64 + 76;
	// 824D5640: 5547183E  rotlwi r7, r10, 3
	ctx.r[7].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 824D5644: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D5648: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D564C: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 824D5650: 7CA74A14  add r5, r7, r9
	ctx.r[5].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 824D5654: 55061838  slwi r6, r8, 3
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824D5658: 7D264A14  add r9, r6, r9
	ctx.r[9].u64 = ctx.r[6].u64 + ctx.r[9].u64;
	// 824D565C: 91030004  stw r8, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824D5660: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D5664: 91050000  stw r8, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824D5668: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D566C: 91250004  stw r9, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824D5670: 812B0050  lwz r9, 0x50(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D5674: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824D5678: 40980030  bge cr6, 0x824d56a8
	if !ctx.cr[6].lt {
	pc = 0x824D56A8; continue 'dispatch;
	}
	// 824D567C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D5680: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 824D5684: 7D47482E  lwzx r10, r7, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 824D5688: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D568C: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 824D5690: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824D5694: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824D5698: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 824D569C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 824D56A0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D56A4: 7D0B532E  sthx r8, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u16) };
	// 824D56A8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D56AC: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D56B0: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 824D56B4: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 824D56B8: 5569843E  srwi r9, r11, 0x10
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D56BC: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D56C0: 5569C23E  srwi r9, r11, 8
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D56C4: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D56C8: 5569E13E  srwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D56CC: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D56D0: 5569F0BE  srwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D56D4: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D56D8: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D56DC: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D56E0: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 824D56E4: 7F065000  cmpw cr6, r6, r10
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824D56E8: 41990010  bgt cr6, 0x824d56f8
	if ctx.cr[6].gt {
	pc = 0x824D56F8; continue 'dispatch;
	}
	// 824D56EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824D56F0: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 824D56F4: 4BF98CF5  bl 0x8246e3e8
	ctx.lr = 0x824D56F8;
	sub_8246E3E8(ctx, base);
	// 824D56F8: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 824D56FC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 824D5700: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 824D5704: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 824D5708: 409AFF2C  bne cr6, 0x824d5634
	if !ctx.cr[6].eq {
	pc = 0x824D5634; continue 'dispatch;
	}
	// 824D570C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824D5710: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824D5714: 4BFFF71D  bl 0x824d4e30
	ctx.lr = 0x824D5718;
	sub_824D4E30(ctx, base);
	// 824D5718: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824D571C: 4805F9E8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 824D5720: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 824D5724: 4BFFFEDC  b 0x824d5600
	pc = 0x824D5600; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D5728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D5728 size=348
    let mut pc: u32 = 0x824D5728;
    'dispatch: loop {
        match pc {
            0x824D5728 => {
    //   block [0x824D5728..0x824D5884)
	// 824D5728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D572C: 4805F98D  bl 0x825350b8
	ctx.lr = 0x824D5730;
	sub_82535080(ctx, base);
	// 824D5730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D5734: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824D5738: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824D573C: 3B800200  li r28, 0x200
	ctx.r[28].s64 = 512;
	// 824D5740: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D5744: 2B0B0200  cmplwi cr6, r11, 0x200
	ctx.cr[6].compare_u32(ctx.r[11].u32, 512 as u32, &mut ctx.xer);
	// 824D5748: 419A0104  beq cr6, 0x824d584c
	if ctx.cr[6].eq {
	pc = 0x824D584C; continue 'dispatch;
	}
	// 824D574C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D5750: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D5754: 419A00F8  beq cr6, 0x824d584c
	if ctx.cr[6].eq {
	pc = 0x824D584C; continue 'dispatch;
	}
	// 824D5758: 3BFE0004  addi r31, r30, 4
	ctx.r[31].s64 = ctx.r[30].s64 + 4;
	// 824D575C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D5760: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824D5764: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D5768: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D576C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D5770: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824D5774: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 824D5778: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824D577C: 388BFF80  addi r4, r11, -0x80
	ctx.r[4].s64 = ctx.r[11].s64 + -128;
	// 824D5780: 4BFFF8F9  bl 0x824d5078
	ctx.lr = 0x824D5784;
	sub_824D5078(ctx, base);
	// 824D5784: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D5788: 356BFF80  addic. r11, r11, -0x80
	ctx.xer.ca = (ctx.r[11].u32 > (!(-128 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -128;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D578C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D5790: 4082003C  bne 0x824d57cc
	if !ctx.cr[0].eq {
	pc = 0x824D57CC; continue 'dispatch;
	}
	// 824D5794: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D5798: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 824D579C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D57A0: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824D57A4: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824D57A8: 810D0000  lwz r8, 0(r13)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D57AC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824D57B0: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824D57B4: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 824D57B8: 808AFFFC  lwz r4, -4(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) } as u64;
	// 824D57BC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824D57C0: 7C67402E  lwzx r3, r7, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 824D57C4: 4BF8E8F5  bl 0x824640b8
	ctx.lr = 0x824D57C8;
	sub_824640B8(ctx, base);
	// 824D57C8: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 824D57CC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D57D0: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 824D57D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D57D8: 409A0068  bne cr6, 0x824d5840
	if !ctx.cr[6].eq {
	pc = 0x824D5840; continue 'dispatch;
	}
	// 824D57DC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D57E0: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 824D57E4: 5569843E  srwi r9, r11, 0x10
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D57E8: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D57EC: 5569C23E  srwi r9, r11, 8
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D57F0: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D57F4: 5569E13E  srwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D57F8: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D57FC: 5569F0BE  srwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D5800: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D5804: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D5808: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 824D580C: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 824D5810: 2F060001  cmpwi cr6, r6, 1
	ctx.cr[6].compare_i32(ctx.r[6].s32, 1, &mut ctx.xer);
	// 824D5814: 40980010  bge cr6, 0x824d5824
	if !ctx.cr[6].lt {
	pc = 0x824D5824; continue 'dispatch;
	}
	// 824D5818: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 824D581C: 38BF000C  addi r5, r31, 0xc
	ctx.r[5].s64 = ctx.r[31].s64 + 12;
	// 824D5820: 48000014  b 0x824d5834
	pc = 0x824D5834; continue 'dispatch;
	// 824D5824: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 824D5828: 7F065800  cmpw cr6, r6, r11
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824D582C: 40980014  bge cr6, 0x824d5840
	if !ctx.cr[6].lt {
	pc = 0x824D5840; continue 'dispatch;
	}
	// 824D5830: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824D5834: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824D5838: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D583C: 4BF98BAD  bl 0x8246e3e8
	ctx.lr = 0x824D5840;
	sub_8246E3E8(ctx, base);
	// 824D5840: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D5844: 2B0B0200  cmplwi cr6, r11, 0x200
	ctx.cr[6].compare_u32(ctx.r[11].u32, 512 as u32, &mut ctx.xer);
	// 824D5848: 409AFF04  bne cr6, 0x824d574c
	if !ctx.cr[6].eq {
	pc = 0x824D574C; continue 'dispatch;
	}
	// 824D584C: 80DE0008  lwz r6, 8(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D5850: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 824D5854: 419A0028  beq cr6, 0x824d587c
	if ctx.cr[6].eq {
	pc = 0x824D587C; continue 'dispatch;
	}
	// 824D5858: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 824D585C: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D5860: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D5864: 4BFF62B5  bl 0x824cbb18
	ctx.lr = 0x824D5868;
	sub_824CBB18(ctx, base);
	// 824D5868: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D586C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824D5870: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D5874: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824D5878: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 824D587C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824D5880: 4805F888  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D5888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D5888 size=108
    let mut pc: u32 = 0x824D5888;
    'dispatch: loop {
        match pc {
            0x824D5888 => {
    //   block [0x824D5888..0x824D58F4)
	// 824D5888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D588C: 4805F82D  bl 0x825350b8
	ctx.lr = 0x824D5890;
	sub_82535080(ctx, base);
	// 824D5890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D5894: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D5898: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824D589C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 824D58A0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D58A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D58A8: 419A0044  beq cr6, 0x824d58ec
	if ctx.cr[6].eq {
	pc = 0x824D58EC; continue 'dispatch;
	}
	// 824D58AC: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 824D58B0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D58B4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 824D58B8: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D58BC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824D58C0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D58C4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D58C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D58CC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824D58D0: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 824D58D4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824D58D8: 388BFF80  addi r4, r11, -0x80
	ctx.r[4].s64 = ctx.r[11].s64 + -128;
	// 824D58DC: 4BFFFCED  bl 0x824d55c8
	ctx.lr = 0x824D58E0;
	sub_824D55C8(ctx, base);
	// 824D58E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D58E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D58E8: 409AFFC8  bne cr6, 0x824d58b0
	if !ctx.cr[6].eq {
	pc = 0x824D58B0; continue 'dispatch;
	}
	// 824D58EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824D58F0: 4805F818  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D58F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824D58F8 size=1008
    let mut pc: u32 = 0x824D58F8;
    'dispatch: loop {
        match pc {
            0x824D58F8 => {
    //   block [0x824D58F8..0x824D5CE8)
	// 824D58F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D58FC: 4805F7B1  bl 0x825350ac
	ctx.lr = 0x824D5900;
	sub_82535080(ctx, base);
	// 824D5900: 9421FD90  stwu r1, -0x270(r1)
	ea = ctx.r[1].u32.wrapping_add(-624 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D5904: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D5908: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824D590C: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 824D5910: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D5914: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824D5918: 419A02D0  beq cr6, 0x824d5be8
	if ctx.cr[6].eq {
	pc = 0x824D5BE8; continue 'dispatch;
	}
	// 824D591C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 824D5920: 419A0038  beq cr6, 0x824d5958
	if ctx.cr[6].eq {
	pc = 0x824D5958; continue 'dispatch;
	}
	// 824D5924: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 824D5928: 409A03B8  bne cr6, 0x824d5ce0
	if !ctx.cr[6].eq {
	pc = 0x824D5CE0; continue 'dispatch;
	}
	// 824D592C: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824D5930: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824D5934: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D5938: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 824D593C: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D5940: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D5944: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824D5948: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D594C: 4E800421  bctrl
	ctx.lr = 0x824D5950;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D5950: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 824D5954: 4805F7A8  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 824D5958: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824D595C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D5960: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D5964: C1BF0018  lfs f13, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D5968: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 824D596C: 390B0008  addi r8, r11, 8
	ctx.r[8].s64 = ctx.r[11].s64 + 8;
	// 824D5970: C19E0050  lfs f12, 0x50(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D5974: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 824D5978: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 824D597C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 824D5980: 912100B4  stw r9, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[9].u32 ) };
	// 824D5984: 3BBF0030  addi r29, r31, 0x30
	ctx.r[29].s64 = ctx.r[31].s64 + 48;
	// 824D5988: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D598C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824D5990: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 824D5994: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 824D5998: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824D599C: 838B0008  lwz r28, 8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D59A0: C01E0058  lfs f0, 0x58(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D59A4: 836A0008  lwz r27, 8(r10)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D59A8: 397C0040  addi r11, r28, 0x40
	ctx.r[11].s64 = ctx.r[28].s64 + 64;
	// 824D59AC: 395B0040  addi r10, r27, 0x40
	ctx.r[10].s64 = ctx.r[27].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D5CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824D5CE8 size=1412
    let mut pc: u32 = 0x824D5CE8;
    'dispatch: loop {
        match pc {
            0x824D5CE8 => {
    //   block [0x824D5CE8..0x824D626C)
	// 824D5CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D5CEC: 4805F395  bl 0x82535080
	ctx.lr = 0x824D5CF0;
	sub_82535080(ctx, base);
	// 824D5CF0: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 824D5CF4: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 824D5CF8: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 824D5CFC: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 824D5D00: E981D000  ld r12, -0x3000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-12288 as u32) ) };
	// 824D5D04: 9421CC80  stwu r1, -0x3380(r1)
	ea = ctx.r[1].u32.wrapping_add(-13184 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D5D08: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824D5D0C: 90610244  stw r3, 0x244(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(580 as u32), ctx.r[3].u32 ) };
	// 824D5D10: 7C902378  mr r16, r4
	ctx.r[16].u64 = ctx.r[4].u64;
	// 824D5D14: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 824D5D18: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824D5D1C: 7EF2BB78  mr r18, r23
	ctx.r[18].u64 = ctx.r[23].u64;
	// 824D5D20: C3CB8CB4  lfs f30, -0x734c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 824D5D24: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D5D28: 81500008  lwz r10, 8(r16)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D5D2C: D3C13270  stfs f30, 0x3270(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(12912 as u32), tmp.u32 ) };
	// 824D5D30: 92E1005C  stw r23, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[23].u32 ) };
	// 824D5D34: 39F00008  addi r15, r16, 8
	ctx.r[15].s64 = ctx.r[16].s64 + 8;
	// 824D5D38: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824D5D3C: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D5D40: D0013290  stfs f0, 0x3290(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(12944 as u32), tmp.u32 ) };
	// 824D5D44: D0013294  stfs f0, 0x3294(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(12948 as u32), tmp.u32 ) };
	// 824D5D48: 409904C8  ble cr6, 0x824d6210
	if !ctx.cr[6].gt {
	pc = 0x824D6210; continue 'dispatch;
	}
	// 824D5D4C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824D5D50: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824D5D54: 7EF1BB78  mr r17, r23
	ctx.r[17].u64 = ctx.r[23].u64;
	// 824D5D58: 3A6B9004  addi r19, r11, -0x6ffc
	ctx.r[19].s64 = ctx.r[11].s64 + -28668;
	// 824D5D5C: 3B000010  li r24, 0x10
	ctx.r[24].s64 = 16;
	// 824D5D60: C3EA0DA0  lfs f31, 0xda0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3488 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824D5D64: 39C00001  li r14, 1
	ctx.r[14].s64 = 1;
	// 824D5D68: 39700004  addi r11, r16, 4
	ctx.r[11].s64 = ctx.r[16].s64 + 4;
	// 824D5D6C: 814F0000  lwz r10, 0(r15)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D5D70: 3A520001  addi r18, r18, 1
	ctx.r[18].s64 = ctx.r[18].s64 + 1;
	// 824D5D74: 7F125000  cmpw cr6, r18, r10
	ctx.cr[6].compare_i32(ctx.r[18].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824D5D78: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D5D7C: 7FEB882E  lwzx r31, r11, r17
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 824D5D80: 3A310004  addi r17, r17, 4
	ctx.r[17].s64 = ctx.r[17].s64 + 4;
	// 824D5D84: 409A0010  bne cr6, 0x824d5d94
	if !ctx.cr[6].eq {
	pc = 0x824D5D94; continue 'dispatch;
	}
	// 824D5D88: 81700000  lwz r11, 0(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D5D8C: 7E8BFA14  add r20, r11, r31
	ctx.r[20].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 824D5D90: 48000008  b 0x824d5d98
	pc = 0x824D5D98; continue 'dispatch;
	// 824D5D94: 3A9F0200  addi r20, r31, 0x200
	ctx.r[20].s64 = ctx.r[31].s64 + 512;
	// 824D5D98: 7F1FA040  cmplw cr6, r31, r20
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[20].u32, &mut ctx.xer);
	// 824D5D9C: 40980468  bge cr6, 0x824d6204
	if !ctx.cr[6].lt {
	pc = 0x824D6204; continue 'dispatch;
	}
	// 824D5DA0: 39610250  addi r11, r1, 0x250
	ctx.r[11].s64 = ctx.r[1].s64 + 592;
	// 824D5DA4: 82DF0010  lwz r22, 0x10(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D5DA8: 82BF0014  lwz r21, 0x14(r31)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D5DAC: D3C13270  stfs f30, 0x3270(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(12912 as u32), tmp.u32 ) };
	// 824D5DB0: 92E132C0  stw r23, 0x32c0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(12992 as u32), ctx.r[23].u32 ) };
	// 824D5DB4: 3B3F0008  addi r25, r31, 8
	ctx.r[25].s64 = ctx.r[31].s64 + 8;
	// 824D5DB8: 91610240  stw r11, 0x240(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(576 as u32), ctx.r[11].u32 ) };
	// 824D5DBC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D5DC0: 7C005A2C  dcbt 0, r11
	// 824D5DC4: 397F0080  addi r11, r31, 0x80
	ctx.r[11].s64 = ctx.r[31].s64 + 128;
	// 824D5DC8: 7C005A2C  dcbt 0, r11
	// 824D5DCC: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D5DD0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824D5DD4: 419A02BC  beq cr6, 0x824d6090
	if ctx.cr[6].eq {
	pc = 0x824D6090; continue 'dispatch;
	}
	// 824D5DD8: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 824D5DDC: 419A0028  beq cr6, 0x824d5e04
	if ctx.cr[6].eq {
	pc = 0x824D5E04; continue 'dispatch;
	}
	// 824D5DE0: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 824D5DE4: 409A039C  bne cr6, 0x824d6180
	if !ctx.cr[6].eq {
	pc = 0x824D6180; continue 'dispatch;
	}
	// 824D5DE8: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824D5DEC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824D5DF0: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D5DF4: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D5DF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D5DFC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824D5E00: 48000374  b 0x824d6174
	pc = 0x824D6174; continue 'dispatch;
	// 824D5E04: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D5E08: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 824D5E0C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 824D5E10: C19E0050  lfs f12, 0x50(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D5E14: 3B7F0030  addi r27, r31, 0x30
	ctx.r[27].s64 = ctx.r[31].s64 + 48;
	// 824D5E18: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 824D5E1C: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D5E20: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 824D5E24: 81390000  lwz r9, 0(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D5E28: 93C10078  stw r30, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 824D5E2C: 92E100C4  stw r23, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[23].u32 ) };
	// 824D5E30: 9121007C  stw r9, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[9].u32 ) };
	// 824D5E34: 392B0008  addi r9, r11, 8
	ctx.r[9].s64 = ctx.r[11].s64 + 8;
	// 824D5E38: 83AB0008  lwz r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D5E3C: C01E0058  lfs f0, 0x58(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D5E40: 838A0008  lwz r28, 8(r10)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D5E44: 397D0040  addi r11, r29, 0x40
	ctx.r[11].s64 = ctx.r[29].s64 + 64;
	// 824D5E48: 395C0040  addi r10, r28, 0x40
	ctx.r[10].s64 = ctx.r[28].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D6270 size=92
    let mut pc: u32 = 0x824D6270;
    'dispatch: loop {
        match pc {
            0x824D6270 => {
    //   block [0x824D6270..0x824D62CC)
	// 824D6270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D6274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D6278: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D627C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D6280: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D6284: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D6288: 396B14A4  addi r11, r11, 0x14a4
	ctx.r[11].s64 = ctx.r[11].s64 + 5284;
	// 824D628C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824D6290: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 824D6294: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824D6298: 3900FFD1  li r8, -0x2f
	ctx.r[8].s64 = -47;
	// 824D629C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D62A0: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 824D62A4: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 824D62A8: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824D62AC: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824D62B0: 480D3E49  bl 0x825aa0f8
	ctx.lr = 0x824D62B4;
	sub_825AA0F8(ctx, base);
	// 824D62B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D62B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824D62BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D62C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D62C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D62C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D62D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D62D0 size=16
    let mut pc: u32 = 0x824D62D0;
    'dispatch: loop {
        match pc {
            0x824D62D0 => {
    //   block [0x824D62D0..0x824D62E0)
	// 824D62D0: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D62D4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 824D62D8: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 824D62DC: 48236F90  b 0x8270d26c
	sub_8270D26C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D62E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D62E0 size=8
    let mut pc: u32 = 0x824D62E0;
    'dispatch: loop {
        match pc {
            0x824D62E0 => {
    //   block [0x824D62E0..0x824D62E8)
	// 824D62E0: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D62E4: 4BF8FE94  b 0x82466178
	sub_82466178(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D62E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D62E8 size=172
    let mut pc: u32 = 0x824D62E8;
    'dispatch: loop {
        match pc {
            0x824D62E8 => {
    //   block [0x824D62E8..0x824D6394)
	// 824D62E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D62EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D62F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824D62F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D62F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D62FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824D6300: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824D6304: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D6308: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D630C: 409A0070  bne cr6, 0x824d637c
	if !ctx.cr[6].eq {
	pc = 0x824D637C; continue 'dispatch;
	}
	// 824D6310: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6314: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824D6318: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824D631C: 814B0060  lwz r10, 0x60(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D6320: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824D6324: 419A0020  beq cr6, 0x824d6344
	if ctx.cr[6].eq {
	pc = 0x824D6344; continue 'dispatch;
	}
	// 824D6328: 812B0064  lwz r9, 0x64(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 824D632C: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 824D6330: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 824D6334: 912B0064  stw r9, 0x64(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 824D6338: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D633C: 914B0060  stw r10, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 824D6340: 48000010  b 0x824d6350
	pc = 0x824D6350; continue 'dispatch;
	// 824D6344: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 824D6348: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 824D634C: 4BF8DAF5  bl 0x82463e40
	ctx.lr = 0x824D6350;
	sub_82463E40(ctx, base);
	// 824D6350: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824D6354: 419A0018  beq cr6, 0x824d636c
	if ctx.cr[6].eq {
	pc = 0x824D636C; continue 'dispatch;
	}
	// 824D6358: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824D635C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824D6360: 4BF90041  bl 0x824663a0
	ctx.lr = 0x824D6364;
	sub_824663A0(ctx, base);
	// 824D6364: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824D6368: 48000008  b 0x824d6370
	pc = 0x824D6370; continue 'dispatch;
	// 824D636C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824D6370: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 824D6374: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824D6378: 480D3D91  bl 0x825aa108
	ctx.lr = 0x824D637C;
	sub_825AA108(ctx, base);
	// 824D637C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824D6380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D6384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D6388: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824D638C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D6390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D6398 size=92
    let mut pc: u32 = 0x824D6398;
    'dispatch: loop {
        match pc {
            0x824D6398 => {
    //   block [0x824D6398..0x824D63F4)
	// 824D6398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D639C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D63A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D63A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D63A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D63AC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D63B0: 396B14A4  addi r11, r11, 0x14a4
	ctx.r[11].s64 = ctx.r[11].s64 + 5284;
	// 824D63B4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D63B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824D63BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D63C0: 419A0014  beq cr6, 0x824d63d4
	if ctx.cr[6].eq {
	pc = 0x824D63D4; continue 'dispatch;
	}
	// 824D63C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824D63C8: 4BFC49C9  bl 0x8249ad90
	ctx.lr = 0x824D63CC;
	sub_8249AD90(ctx, base);
	// 824D63CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824D63D0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824D63D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824D63D8: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824D63DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D63E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824D63E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D63E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D63EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D63F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D63F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D63F8 size=148
    let mut pc: u32 = 0x824D63F8;
    'dispatch: loop {
        match pc {
            0x824D63F8 => {
    //   block [0x824D63F8..0x824D648C)
	// 824D63F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D63FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D6400: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824D6404: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D6408: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D640C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D6410: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D6414: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824D6418: 396B14A4  addi r11, r11, 0x14a4
	ctx.r[11].s64 = ctx.r[11].s64 + 5284;
	// 824D641C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D6420: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824D6424: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D6428: 419A0014  beq cr6, 0x824d643c
	if ctx.cr[6].eq {
	pc = 0x824D643C; continue 'dispatch;
	}
	// 824D642C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824D6430: 4BFC4961  bl 0x8249ad90
	ctx.lr = 0x824D6434;
	sub_8249AD90(ctx, base);
	// 824D6434: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824D6438: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824D643C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824D6440: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824D6444: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824D6448: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824D644C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D6450: 419A0020  beq cr6, 0x824d6470
	if ctx.cr[6].eq {
	pc = 0x824D6470; continue 'dispatch;
	}
	// 824D6454: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6458: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824D645C: 38C00021  li r6, 0x21
	ctx.r[6].s64 = 33;
	// 824D6460: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D6464: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824D6468: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824D646C: 4BF8DC4D  bl 0x824640b8
	ctx.lr = 0x824D6470;
	sub_824640B8(ctx, base);
	// 824D6470: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D6474: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824D6478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D647C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D6480: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824D6484: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D6488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D6490 size=384
    let mut pc: u32 = 0x824D6490;
    'dispatch: loop {
        match pc {
            0x824D6490 => {
    //   block [0x824D6490..0x824D6610)
	// 824D6490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D6494: 4805EC15  bl 0x825350a8
	ctx.lr = 0x824D6498;
	sub_82535080(ctx, base);
	// 824D6498: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D649C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824D64A0: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 824D64A4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 824D64A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824D64AC: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D64B0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824D64B4: 40990068  ble cr6, 0x824d651c
	if !ctx.cr[6].gt {
	pc = 0x824D651C; continue 'dispatch;
	}
	// 824D64B8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824D64BC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824D64C0: 3BAA151C  addi r29, r10, 0x151c
	ctx.r[29].s64 = ctx.r[10].s64 + 5404;
	// 824D64C4: 3BCB0001  addi r30, r11, 1
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	// 824D64C8: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D64CC: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D64D0: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824D64D4: 7CDF582E  lwzx r6, r31, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824D64D8: 409A000C  bne cr6, 0x824d64e4
	if !ctx.cr[6].eq {
	pc = 0x824D64E4; continue 'dispatch;
	}
	// 824D64DC: 80FC0000  lwz r7, 0(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D64E0: 48000008  b 0x824d64e8
	pc = 0x824D64E8; continue 'dispatch;
	// 824D64E4: 38E00200  li r7, 0x200
	ctx.r[7].s64 = 512;
	// 824D64E8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D64EC: 39000200  li r8, 0x200
	ctx.r[8].s64 = 512;
	// 824D64F0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824D64F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824D64F8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824D64FC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D6500: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D6504: 4E800421  bctrl
	ctx.lr = 0x824D6508;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D6508: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D650C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 824D6510: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824D6514: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824D6518: 4198FFAC  blt cr6, 0x824d64c4
	if ctx.cr[6].lt {
	pc = 0x824D64C4; continue 'dispatch;
	}
	// 824D651C: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D6520: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824D6524: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D6528: 409900D8  ble cr6, 0x824d6600
	if !ctx.cr[6].gt {
	pc = 0x824D6600; continue 'dispatch;
	}
	// 824D652C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D6530: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 824D6534: 3B0B1510  addi r24, r11, 0x1510
	ctx.r[24].s64 = ctx.r[11].s64 + 5392;
	// 824D6538: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D653C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824D6540: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D6544: 7F1D5000  cmpw cr6, r29, r10
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824D6548: 7FF9582E  lwzx r31, r25, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824D654C: 3B390004  addi r25, r25, 4
	ctx.r[25].s64 = ctx.r[25].s64 + 4;
	// 824D6550: 409A0010  bne cr6, 0x824d6560
	if !ctx.cr[6].eq {
	pc = 0x824D6560; continue 'dispatch;
	}
	// 824D6554: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6558: 7FCBFA14  add r30, r11, r31
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 824D655C: 48000008  b 0x824d6564
	pc = 0x824D6564; continue 'dispatch;
	// 824D6560: 3BDF0200  addi r30, r31, 0x200
	ctx.r[30].s64 = ctx.r[31].s64 + 512;
	// 824D6564: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 824D6568: 4098008C  bge cr6, 0x824d65f4
	if !ctx.cr[6].lt {
	pc = 0x824D65F4; continue 'dispatch;
	}
	// 824D656C: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6570: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824D6574: 419A0014  beq cr6, 0x824d6588
	if ctx.cr[6].eq {
	pc = 0x824D6588; continue 'dispatch;
	}
	// 824D6578: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 824D657C: 419A008C  beq cr6, 0x824d6608
	if ctx.cr[6].eq {
	pc = 0x824D6608; continue 'dispatch;
	}
	// 824D6580: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 824D6584: 409A0060  bne cr6, 0x824d65e4
	if !ctx.cr[6].eq {
	pc = 0x824D65E4; continue 'dispatch;
	}
	// 824D6588: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 824D658C: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 824D6590: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6594: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 824D6598: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824D659C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D65A0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824D65A4: 816B16CC  lwz r11, 0x16cc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5836 as u32) ) } as u64;
	// 824D65A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D65AC: 419A0018  beq cr6, 0x824d65c4
	if ctx.cr[6].eq {
	pc = 0x824D65C4; continue 'dispatch;
	}
	// 824D65B0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 824D65B4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 824D65B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D65BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D65C0: 4E800421  bctrl
	ctx.lr = 0x824D65C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D65C4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D65C8: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824D65CC: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D65D0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 824D65D4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824D65D8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824D65DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D65E0: 4E800421  bctrl
	ctx.lr = 0x824D65E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D65E4: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 824D65E8: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 824D65EC: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 824D65F0: 4198FF7C  blt cr6, 0x824d656c
	if ctx.cr[6].lt {
	pc = 0x824D656C; continue 'dispatch;
	}
	// 824D65F4: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D65F8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824D65FC: 4198FF3C  blt cr6, 0x824d6538
	if ctx.cr[6].lt {
	pc = 0x824D6538; continue 'dispatch;
	}
	// 824D6600: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824D6604: 4805EAF4  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 824D6608: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 824D660C: 4BFFFF80  b 0x824d658c
	pc = 0x824D658C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D6610 size=324
    let mut pc: u32 = 0x824D6610;
    'dispatch: loop {
        match pc {
            0x824D6610 => {
    //   block [0x824D6610..0x824D66F8)
	// 824D6610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D6614: 4805EA99  bl 0x825350ac
	ctx.lr = 0x824D6618;
	sub_82535080(ctx, base);
	// 824D6618: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D661C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824D6620: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824D6624: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824D6628: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D662C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824D6630: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824D6634: 409A0034  bne cr6, 0x824d6668
	if !ctx.cr[6].eq {
	pc = 0x824D6668; continue 'dispatch;
	}
	// 824D6638: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D663C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824D6640: 80FA0004  lwz r7, 4(r26)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D6644: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824D6648: 80DA0000  lwz r6, 0(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D664C: 388A1524  addi r4, r10, 0x1524
	ctx.r[4].s64 = ctx.r[10].s64 + 5412;
	// 824D6650: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824D6654: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824D6658: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D665C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824D6660: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D6664: 4E800421  bctrl
	ctx.lr = 0x824D6668;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D6668: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D666C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824D6670: 3B2B151C  addi r25, r11, 0x151c
	ctx.r[25].s64 = ctx.r[11].s64 + 5404;
	// 824D6674: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6678: 39000200  li r8, 0x200
	ctx.r[8].s64 = 512;
	// 824D667C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6680: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824D6684: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 824D6688: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824D668C: 7CCBE02E  lwzx r6, r11, r28
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 824D6690: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D6694: 3BE60010  addi r31, r6, 0x10
	ctx.r[31].s64 = ctx.r[6].s64 + 16;
	// 824D6698: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D669C: 7D4B3214  add r10, r11, r6
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 824D66A0: 38EB0010  addi r7, r11, 0x10
	ctx.r[7].s64 = ctx.r[11].s64 + 16;
	// 824D66A4: 3B6A0010  addi r27, r10, 0x10
	ctx.r[27].s64 = ctx.r[10].s64 + 16;
	// 824D66A8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 824D66AC: 4E800421  bctrl
	ctx.lr = 0x824D66B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D66B0: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 824D66B4: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D66B8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 824D66BC: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 824D66C0: 41990080  bgt cr6, 0x824d6740
	if ctx.cr[6].gt {
	pc = 0x824D6740; continue 'dispatch;
	}
	// 824D66C4: 3D80824D  lis r12, -0x7db3
	ctx.r[12].s64 = -2108882944;
	// 824D66C8: 398C66DC  addi r12, r12, 0x66dc
	ctx.r[12].s64 = ctx.r[12].s64 + 26332;
	// 824D66CC: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 824D66D0: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 824D66D4: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 824D66D8: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x824D6738; continue 'dispatch;
		},
		1 => {
	pc = 0x824D674C; continue 'dispatch;
		},
		2 => {
	pc = 0x824D66FC; continue 'dispatch;
		},
		3 => {
	pc = 0x824D66FC; continue 'dispatch;
		},
		4 => {
	pc = 0x824D66F8; continue 'dispatch;
		},
		5 => {
	pc = 0x824D66F8; continue 'dispatch;
		},
		6 => {
	pc = 0x824D66FC; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 824D66DC: 824D6738  lwz r18, 0x6738(r13)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(26424 as u32) ) } as u64;
	// 824D66E0: 824D674C  lwz r18, 0x674c(r13)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(26444 as u32) ) } as u64;
	// 824D66E4: 824D66FC  lwz r18, 0x66fc(r13)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(26364 as u32) ) } as u64;
	// 824D66E8: 824D66FC  lwz r18, 0x66fc(r13)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(26364 as u32) ) } as u64;
	// 824D66EC: 824D66F8  lwz r18, 0x66f8(r13)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(26360 as u32) ) } as u64;
	// 824D66F0: 824D66F8  lwz r18, 0x66f8(r13)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(26360 as u32) ) } as u64;
	// 824D66F4: 824D66FC  lwz r18, 0x66fc(r13)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(26364 as u32) ) } as u64;
            }
            0x824D66F8 => {
    //   block [0x824D66F8..0x824D66FC)
	// 824D66F8: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	pc = 0x824D66FC; continue 'dispatch;
            }
            0x824D66FC => {
    //   block [0x824D66FC..0x824D6738)
	// 824D66FC: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 824D6700: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6704: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 824D6708: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824D670C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D6710: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824D6714: 816B16CC  lwz r11, 0x16cc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5836 as u32) ) } as u64;
	// 824D6718: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D671C: 419A001C  beq cr6, 0x824d6738
	if ctx.cr[6].eq {
	pc = 0x824D6738; continue 'dispatch;
	}
	// 824D6720: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824D6724: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824D6728: 7C9F4214  add r4, r31, r8
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[8].u64;
	// 824D672C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D6730: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D6734: 4E800421  bctrl
	ctx.lr = 0x824D6738;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824D6738 => {
    //   block [0x824D6738..0x824D674C)
	// 824D6738: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 824D673C: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 824D6740: 7F1FD840  cmplw cr6, r31, r27
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[27].u32, &mut ctx.xer);
	// 824D6744: 4198FF70  blt cr6, 0x824d66b4
	if ctx.cr[6].lt {
	pc = 0x824D66B4; continue 'dispatch;
	}
	// 824D6748: 4BFFFF2C  b 0x824d6674
	pc = 0x824D6674; continue 'dispatch;
            }
            0x824D674C => {
    //   block [0x824D674C..0x824D6754)
	// 824D674C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824D6750: 4805E9AC  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D6758 size=260
    let mut pc: u32 = 0x824D6758;
    'dispatch: loop {
        match pc {
            0x824D6758 => {
    //   block [0x824D6758..0x824D685C)
	// 824D6758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D675C: 4805E951  bl 0x825350ac
	ctx.lr = 0x824D6760;
	sub_82535080(ctx, base);
	// 824D6760: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D6764: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824D6768: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824D676C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 824D6770: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824D6774: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824D6778: 409A0034  bne cr6, 0x824d67ac
	if !ctx.cr[6].eq {
	pc = 0x824D67AC; continue 'dispatch;
	}
	// 824D677C: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6780: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824D6784: 80FD0008  lwz r7, 8(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D6788: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824D678C: 80DD0004  lwz r6, 4(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D6790: 388A1530  addi r4, r10, 0x1530
	ctx.r[4].s64 = ctx.r[10].s64 + 5424;
	// 824D6794: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824D6798: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824D679C: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D67A0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824D67A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D67A8: 4E800421  bctrl
	ctx.lr = 0x824D67AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D67AC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D67B0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824D67B4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D67B8: 4099009C  ble cr6, 0x824d6854
	if !ctx.cr[6].gt {
	pc = 0x824D6854; continue 'dispatch;
	}
	// 824D67BC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D67C0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 824D67C4: 3B2B1510  addi r25, r11, 0x1510
	ctx.r[25].s64 = ctx.r[11].s64 + 5392;
	// 824D67C8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D67CC: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 824D67D0: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D67D4: 7F1C5000  cmpw cr6, r28, r10
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824D67D8: 7FFA582E  lwzx r31, r26, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824D67DC: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 824D67E0: 409A0010  bne cr6, 0x824d67f0
	if !ctx.cr[6].eq {
	pc = 0x824D67F0; continue 'dispatch;
	}
	// 824D67E4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D67E8: 7FCBFA14  add r30, r11, r31
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 824D67EC: 48000008  b 0x824d67f4
	pc = 0x824D67F4; continue 'dispatch;
	// 824D67F0: 3BDF0200  addi r30, r31, 0x200
	ctx.r[30].s64 = ctx.r[31].s64 + 512;
	// 824D67F4: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 824D67F8: 40980050  bge cr6, 0x824d6848
	if !ctx.cr[6].lt {
	pc = 0x824D6848; continue 'dispatch;
	}
	// 824D67FC: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6800: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824D6804: 419A0014  beq cr6, 0x824d6818
	if ctx.cr[6].eq {
	pc = 0x824D6818; continue 'dispatch;
	}
	// 824D6808: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 824D680C: 419A000C  beq cr6, 0x824d6818
	if ctx.cr[6].eq {
	pc = 0x824D6818; continue 'dispatch;
	}
	// 824D6810: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 824D6814: 409A0024  bne cr6, 0x824d6838
	if !ctx.cr[6].eq {
	pc = 0x824D6838; continue 'dispatch;
	}
	// 824D6818: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D681C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824D6820: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D6824: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 824D6828: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824D682C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824D6830: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D6834: 4E800421  bctrl
	ctx.lr = 0x824D6838;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D6838: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 824D683C: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 824D6840: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 824D6844: 4198FFB8  blt cr6, 0x824d67fc
	if ctx.cr[6].lt {
	pc = 0x824D67FC; continue 'dispatch;
	}
	// 824D6848: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D684C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824D6850: 4198FF78  blt cr6, 0x824d67c8
	if ctx.cr[6].lt {
	pc = 0x824D67C8; continue 'dispatch;
	}
	// 824D6854: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824D6858: 4805E8A4  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D6860 size=168
    let mut pc: u32 = 0x824D6860;
    'dispatch: loop {
        match pc {
            0x824D6860 => {
    //   block [0x824D6860..0x824D6908)
	// 824D6860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D6864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D6868: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824D686C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D6870: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D6874: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824D6878: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824D687C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824D6880: 388BC7F8  addi r4, r11, -0x3808
	ctx.r[4].s64 = ctx.r[11].s64 + -14344;
	// 824D6884: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824D6888: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D688C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824D6890: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D6894: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D6898: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D689C: 4E800421  bctrl
	ctx.lr = 0x824D68A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D68A0: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 824D68A4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824D68A8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824D68AC: 409A0030  bne cr6, 0x824d68dc
	if !ctx.cr[6].eq {
	pc = 0x824D68DC; continue 'dispatch;
	}
	// 824D68B0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D68B4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824D68B8: 556800BE  clrlwi r8, r11, 2
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824D68BC: 80FE0024  lwz r7, 0x24(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 824D68C0: 80DE0020  lwz r6, 0x20(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 824D68C4: 388A1538  addi r4, r10, 0x1538
	ctx.r[4].s64 = ctx.r[10].s64 + 5432;
	// 824D68C8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824D68CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D68D0: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D68D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D68D8: 4E800421  bctrl
	ctx.lr = 0x824D68DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D68DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D68E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D68E4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824D68E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D68EC: 4E800421  bctrl
	ctx.lr = 0x824D68F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D68F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824D68F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D68F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D68FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824D6900: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D6904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6908 size=16
    let mut pc: u32 = 0x824D6908;
    'dispatch: loop {
        match pc {
            0x824D6908 => {
    //   block [0x824D6908..0x824D6918)
	// 824D6908: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824D690C: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824D6910: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D6914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6918 size=12
    let mut pc: u32 = 0x824D6918;
    'dispatch: loop {
        match pc {
            0x824D6918 => {
    //   block [0x824D6918..0x824D6924)
	// 824D6918: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824D691C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824D6920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D6928 size=96
    let mut pc: u32 = 0x824D6928;
    'dispatch: loop {
        match pc {
            0x824D6928 => {
    //   block [0x824D6928..0x824D6988)
	// 824D6928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D692C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D6930: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D6934: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D6938: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824D693C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D6940: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 824D6944: 388B1540  addi r4, r11, 0x1540
	ctx.r[4].s64 = ctx.r[11].s64 + 5440;
	// 824D6948: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 824D694C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6950: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D6954: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D6958: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D695C: 4E800421  bctrl
	ctx.lr = 0x824D6960;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D6960: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6964: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D6968: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824D696C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D6970: 4E800421  bctrl
	ctx.lr = 0x824D6974;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D6974: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824D6978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D697C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D6980: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D6984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6988 size=8
    let mut pc: u32 = 0x824D6988;
    'dispatch: loop {
        match pc {
            0x824D6988 => {
    //   block [0x824D6988..0x824D6990)
	// 824D6988: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824D698C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6990 size=8
    let mut pc: u32 = 0x824D6990;
    'dispatch: loop {
        match pc {
            0x824D6990 => {
    //   block [0x824D6990..0x824D6998)
	// 824D6990: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824D6994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6998 size=32
    let mut pc: u32 = 0x824D6998;
    'dispatch: loop {
        match pc {
            0x824D6998 => {
    //   block [0x824D6998..0x824D69B8)
	// 824D6998: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D699C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824D69A0: 396B1560  addi r11, r11, 0x1560
	ctx.r[11].s64 = ctx.r[11].s64 + 5472;
	// 824D69A4: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 824D69A8: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824D69AC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D69B0: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824D69B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D69B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D69B8 size=168
    let mut pc: u32 = 0x824D69B8;
    'dispatch: loop {
        match pc {
            0x824D69B8 => {
    //   block [0x824D69B8..0x824D6A60)
	// 824D69B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D69BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D69C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D69C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D69C8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D69CC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 824D69D0: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D69D4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824D69D8: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 824D69DC: 55040036  rlwinm r4, r8, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 824D69E0: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 824D69E4: 7D6A482E  lwzx r11, r10, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 824D69E8: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824D69EC: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 824D69F0: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 824D69F4: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 824D69F8: 4199000C  bgt cr6, 0x824d6a04
	if ctx.cr[6].gt {
	pc = 0x824D6A04; continue 'dispatch;
	}
	// 824D69FC: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 824D6A00: 4800001C  b 0x824d6a1c
	pc = 0x824D6A1C; continue 'dispatch;
	// 824D6A04: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6A08: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 824D6A0C: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D6A10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D6A14: 4E800421  bctrl
	ctx.lr = 0x824D6A18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D6A18: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 824D6A1C: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 824D6A20: 396003E8  li r11, 0x3e8
	ctx.r[11].s64 = 1000;
	// 824D6A24: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 824D6A28: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 824D6A2C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 824D6A30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824D6A34: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824D6A38: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 824D6A3C: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 824D6A40: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824D6A44: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824D6A48: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824D6A4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824D6A50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D6A54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D6A58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D6A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6A60 size=32
    let mut pc: u32 = 0x824D6A60;
    'dispatch: loop {
        match pc {
            0x824D6A60 => {
    //   block [0x824D6A60..0x824D6A80)
	// 824D6A60: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6A64: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824D6A68: 80860018  lwz r4, 0x18(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(24 as u32) ) } as u64;
	// 824D6A6C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824D6A70: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 824D6A74: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 824D6A78: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824D6A7C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6A80 size=16
    let mut pc: u32 = 0x824D6A80;
    'dispatch: loop {
        match pc {
            0x824D6A80 => {
    //   block [0x824D6A80..0x824D6A90)
	// 824D6A80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6A84: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824D6A88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D6A8C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6A90 size=4
    let mut pc: u32 = 0x824D6A90;
    'dispatch: loop {
        match pc {
            0x824D6A90 => {
    //   block [0x824D6A90..0x824D6A94)
	// 824D6A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6A98 size=8
    let mut pc: u32 = 0x824D6A98;
    'dispatch: loop {
        match pc {
            0x824D6A98 => {
    //   block [0x824D6A98..0x824D6AA0)
	// 824D6A98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824D6A9C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6AA0 size=24
    let mut pc: u32 = 0x824D6AA0;
    'dispatch: loop {
        match pc {
            0x824D6AA0 => {
    //   block [0x824D6AA0..0x824D6AB8)
	// 824D6AA0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D6AA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824D6AA8: 396B15F0  addi r11, r11, 0x15f0
	ctx.r[11].s64 = ctx.r[11].s64 + 5616;
	// 824D6AAC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824D6AB0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D6AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6AB8 size=20
    let mut pc: u32 = 0x824D6AB8;
    'dispatch: loop {
        match pc {
            0x824D6AB8 => {
    //   block [0x824D6AB8..0x824D6ACC)
	// 824D6AB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6ABC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D6AC0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6AC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D6AC8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6AD0 size=12
    let mut pc: u32 = 0x824D6AD0;
    'dispatch: loop {
        match pc {
            0x824D6AD0 => {
    //   block [0x824D6AD0..0x824D6ADC)
	// 824D6AD0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D6AD4: 386B15F0  addi r3, r11, 0x15f0
	ctx.r[3].s64 = ctx.r[11].s64 + 5616;
	// 824D6AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6AE0 size=20
    let mut pc: u32 = 0x824D6AE0;
    'dispatch: loop {
        match pc {
            0x824D6AE0 => {
    //   block [0x824D6AE0..0x824D6AF4)
	// 824D6AE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6AE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D6AE8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6AEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D6AF0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6AF8 size=4
    let mut pc: u32 = 0x824D6AF8;
    'dispatch: loop {
        match pc {
            0x824D6AF8 => {
    //   block [0x824D6AF8..0x824D6AFC)
	// 824D6AF8: 48006448  b 0x824dcf40
	sub_824DCF40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6B00 size=4
    let mut pc: u32 = 0x824D6B00;
    'dispatch: loop {
        match pc {
            0x824D6B00 => {
    //   block [0x824D6B00..0x824D6B04)
	// 824D6B00: 48006440  b 0x824dcf40
	sub_824DCF40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6B08 size=4
    let mut pc: u32 = 0x824D6B08;
    'dispatch: loop {
        match pc {
            0x824D6B08 => {
    //   block [0x824D6B08..0x824D6B0C)
	// 824D6B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6B10 size=4
    let mut pc: u32 = 0x824D6B10;
    'dispatch: loop {
        match pc {
            0x824D6B10 => {
    //   block [0x824D6B10..0x824D6B14)
	// 824D6B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6B18 size=12
    let mut pc: u32 = 0x824D6B18;
    'dispatch: loop {
        match pc {
            0x824D6B18 => {
    //   block [0x824D6B18..0x824D6B24)
	// 824D6B18: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824D6B1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D6B20: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6B24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6B24 size=32
    let mut pc: u32 = 0x824D6B24;
    'dispatch: loop {
        match pc {
            0x824D6B24 => {
    //   block [0x824D6B24..0x824D6B44)
	// 824D6B24: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824D6B28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824D6B2C: 394A16D0  addi r10, r10, 0x16d0
	ctx.r[10].s64 = ctx.r[10].s64 + 5840;
	// 824D6B30: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824D6B34: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 824D6B38: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 824D6B3C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824D6B40: 4BFCC518  b 0x824a3058
	sub_824A3058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6B44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6B44 size=4
    let mut pc: u32 = 0x824D6B44;
    'dispatch: loop {
        match pc {
            0x824D6B44 => {
    //   block [0x824D6B44..0x824D6B48)
	// 824D6B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D6B48 size=64
    let mut pc: u32 = 0x824D6B48;
    'dispatch: loop {
        match pc {
            0x824D6B48 => {
    //   block [0x824D6B48..0x824D6B88)
	// 824D6B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D6B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D6B50: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D6B54: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D6B58: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824D6B5C: 396B16D0  addi r11, r11, 0x16d0
	ctx.r[11].s64 = ctx.r[11].s64 + 5840;
	// 824D6B60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D6B64: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824D6B68: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 824D6B6C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 824D6B70: 4BFCC4E9  bl 0x824a3058
	ctx.lr = 0x824D6B74;
	sub_824A3058(ctx, base);
	// 824D6B74: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D6B78: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 824D6B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D6B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D6B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D6B88 size=100
    let mut pc: u32 = 0x824D6B88;
    'dispatch: loop {
        match pc {
            0x824D6B88 => {
    //   block [0x824D6B88..0x824D6BEC)
	// 824D6B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D6B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D6B90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824D6B94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D6B98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D6B9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D6BA0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824D6BA4: 4800651D  bl 0x824dd0c0
	ctx.lr = 0x824D6BA8;
	sub_824DD0C0(ctx, base);
	// 824D6BA8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824D6BAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D6BB0: 419A0020  beq cr6, 0x824d6bd0
	if ctx.cr[6].eq {
	pc = 0x824D6BD0; continue 'dispatch;
	}
	// 824D6BB4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6BB8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824D6BBC: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 824D6BC0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D6BC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824D6BC8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824D6BCC: 4BF8D4ED  bl 0x824640b8
	ctx.lr = 0x824D6BD0;
	sub_824640B8(ctx, base);
	// 824D6BD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D6BD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824D6BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D6BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D6BE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824D6BE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D6BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6BF0 size=4
    let mut pc: u32 = 0x824D6BF0;
    'dispatch: loop {
        match pc {
            0x824D6BF0 => {
    //   block [0x824D6BF0..0x824D6BF4)
	// 824D6BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6BF8 size=4
    let mut pc: u32 = 0x824D6BF8;
    'dispatch: loop {
        match pc {
            0x824D6BF8 => {
    //   block [0x824D6BF8..0x824D6BFC)
	// 824D6BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6C00 size=4
    let mut pc: u32 = 0x824D6C00;
    'dispatch: loop {
        match pc {
            0x824D6C00 => {
    //   block [0x824D6C00..0x824D6C04)
	// 824D6C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6C08 size=8
    let mut pc: u32 = 0x824D6C08;
    'dispatch: loop {
        match pc {
            0x824D6C08 => {
    //   block [0x824D6C08..0x824D6C10)
	// 824D6C08: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D6C0C: 4800008C  b 0x824d6c98
	sub_824D6C98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6C10 size=4
    let mut pc: u32 = 0x824D6C10;
    'dispatch: loop {
        match pc {
            0x824D6C10 => {
    //   block [0x824D6C10..0x824D6C14)
	// 824D6C10: 48000008  b 0x824d6c18
	sub_824D6C18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D6C18 size=128
    let mut pc: u32 = 0x824D6C18;
    'dispatch: loop {
        match pc {
            0x824D6C18 => {
    //   block [0x824D6C18..0x824D6C98)
	// 824D6C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D6C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D6C20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D6C24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D6C28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D6C2C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824D6C30: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824D6C34: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824D6C38: 409A0020  bne cr6, 0x824d6c58
	if !ctx.cr[6].eq {
	pc = 0x824D6C58; continue 'dispatch;
	}
	// 824D6C3C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6C40: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824D6C44: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824D6C48: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824D6C4C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824D6C50: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824D6C54: 4BF8D465  bl 0x824640b8
	ctx.lr = 0x824D6C58;
	sub_824640B8(ctx, base);
	// 824D6C58: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D6C5C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824D6C60: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824D6C64: 409A0020  bne cr6, 0x824d6c84
	if !ctx.cr[6].eq {
	pc = 0x824D6C84; continue 'dispatch;
	}
	// 824D6C68: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6C6C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824D6C70: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824D6C74: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6C78: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824D6C7C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824D6C80: 4BF8D439  bl 0x824640b8
	ctx.lr = 0x824D6C84;
	sub_824640B8(ctx, base);
	// 824D6C84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824D6C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D6C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D6C90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D6C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D6C98 size=184
    let mut pc: u32 = 0x824D6C98;
    'dispatch: loop {
        match pc {
            0x824D6C98 => {
    //   block [0x824D6C98..0x824D6D50)
	// 824D6C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D6C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D6CA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824D6CA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D6CA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D6CAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D6CB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824D6CB4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D6CB8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824D6CBC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824D6CC0: 409A0020  bne cr6, 0x824d6ce0
	if !ctx.cr[6].eq {
	pc = 0x824D6CE0; continue 'dispatch;
	}
	// 824D6CC4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6CC8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824D6CCC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824D6CD0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6CD4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824D6CD8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824D6CDC: 4BF8D3DD  bl 0x824640b8
	ctx.lr = 0x824D6CE0;
	sub_824640B8(ctx, base);
	// 824D6CE0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824D6CE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D6CE8: 419A004C  beq cr6, 0x824d6d34
	if ctx.cr[6].eq {
	pc = 0x824D6D34; continue 'dispatch;
	}
	// 824D6CEC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6CF0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824D6CF4: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824D6CF8: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 824D6CFC: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 824D6D00: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824D6D04: 41980018  blt cr6, 0x824d6d1c
	if ctx.cr[6].lt {
	pc = 0x824D6D1C; continue 'dispatch;
	}
	// 824D6D08: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824D6D0C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 824D6D10: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 824D6D14: 4BF8D205  bl 0x82463f18
	ctx.lr = 0x824D6D18;
	sub_82463F18(ctx, base);
	// 824D6D18: 4800001C  b 0x824d6d34
	pc = 0x824D6D34; continue 'dispatch;
	// 824D6D1C: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 824D6D20: 812B0048  lwz r9, 0x48(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 824D6D24: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824D6D28: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 824D6D2C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824D6D30: 93EB0048  stw r31, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 824D6D34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D6D38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824D6D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D6D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D6D44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824D6D48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D6D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6D50 size=4
    let mut pc: u32 = 0x824D6D50;
    'dispatch: loop {
        match pc {
            0x824D6D50 => {
    //   block [0x824D6D50..0x824D6D54)
	// 824D6D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6D58 size=4
    let mut pc: u32 = 0x824D6D58;
    'dispatch: loop {
        match pc {
            0x824D6D58 => {
    //   block [0x824D6D58..0x824D6D5C)
	// 824D6D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6D60 size=20
    let mut pc: u32 = 0x824D6D60;
    'dispatch: loop {
        match pc {
            0x824D6D60 => {
    //   block [0x824D6D60..0x824D6D74)
	// 824D6D60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6D64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D6D68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6D6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D6D70: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6D78 size=8
    let mut pc: u32 = 0x824D6D78;
    'dispatch: loop {
        match pc {
            0x824D6D78 => {
    //   block [0x824D6D78..0x824D6D80)
	// 824D6D78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824D6D7C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6D80 size=24
    let mut pc: u32 = 0x824D6D80;
    'dispatch: loop {
        match pc {
            0x824D6D80 => {
    //   block [0x824D6D80..0x824D6D98)
	// 824D6D80: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D6D84: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824D6D88: 396B184C  addi r11, r11, 0x184c
	ctx.r[11].s64 = ctx.r[11].s64 + 6220;
	// 824D6D8C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824D6D90: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D6D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6D98 size=12
    let mut pc: u32 = 0x824D6D98;
    'dispatch: loop {
        match pc {
            0x824D6D98 => {
    //   block [0x824D6D98..0x824D6DA4)
	// 824D6D98: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D6D9C: 386B184C  addi r3, r11, 0x184c
	ctx.r[3].s64 = ctx.r[11].s64 + 6220;
	// 824D6DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D6DA8 size=100
    let mut pc: u32 = 0x824D6DA8;
    'dispatch: loop {
        match pc {
            0x824D6DA8 => {
    //   block [0x824D6DA8..0x824D6E0C)
	// 824D6DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D6DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D6DB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824D6DB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D6DB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D6DBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D6DC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824D6DC4: 480071FD  bl 0x824ddfc0
	ctx.lr = 0x824D6DC8;
	sub_824DDFC0(ctx, base);
	// 824D6DC8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824D6DCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D6DD0: 419A0020  beq cr6, 0x824d6df0
	if ctx.cr[6].eq {
	pc = 0x824D6DF0; continue 'dispatch;
	}
	// 824D6DD4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6DD8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824D6DDC: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 824D6DE0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D6DE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824D6DE8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824D6DEC: 4BF8D2CD  bl 0x824640b8
	ctx.lr = 0x824D6DF0;
	sub_824640B8(ctx, base);
	// 824D6DF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D6DF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824D6DF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D6DFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D6E00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824D6E04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D6E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6E10 size=20
    let mut pc: u32 = 0x824D6E10;
    'dispatch: loop {
        match pc {
            0x824D6E10 => {
    //   block [0x824D6E10..0x824D6E24)
	// 824D6E10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6E14: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D6E18: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6E1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D6E20: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6E28 size=12
    let mut pc: u32 = 0x824D6E28;
    'dispatch: loop {
        match pc {
            0x824D6E28 => {
    //   block [0x824D6E28..0x824D6E34)
	// 824D6E28: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824D6E2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824D6E30: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6E34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6E34 size=8
    let mut pc: u32 = 0x824D6E34;
    'dispatch: loop {
        match pc {
            0x824D6E34 => {
    //   block [0x824D6E34..0x824D6E3C)
	// 824D6E34: 4800003C  b 0x824d6e70
	sub_824D6E70(ctx, base);
	return;
	// 824D6E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D6E40 size=44
    let mut pc: u32 = 0x824D6E40;
    'dispatch: loop {
        match pc {
            0x824D6E40 => {
    //   block [0x824D6E40..0x824D6E6C)
	// 824D6E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D6E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D6E48: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D6E4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D6E50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824D6E54: 4800001D  bl 0x824d6e70
	ctx.lr = 0x824D6E58;
	sub_824D6E70(ctx, base);
	// 824D6E58: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D6E5C: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 824D6E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D6E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D6E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D6E70 size=108
    let mut pc: u32 = 0x824D6E70;
    'dispatch: loop {
        match pc {
            0x824D6E70 => {
    //   block [0x824D6E70..0x824D6EDC)
	// 824D6E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D6E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D6E78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D6E7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D6E80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D6E84: 4802793D  bl 0x824fe7c0
	ctx.lr = 0x824D6E88;
	sub_824FE7C0(ctx, base);
	// 824D6E88: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D6E8C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824D6E90: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824D6E94: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824D6E98: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 824D6E9C: 396B18C4  addi r11, r11, 0x18c4
	ctx.r[11].s64 = ctx.r[11].s64 + 6340;
	// 824D6EA0: 394A18B8  addi r10, r10, 0x18b8
	ctx.r[10].s64 = ctx.r[10].s64 + 6328;
	// 824D6EA4: 392918A4  addi r9, r9, 0x18a4
	ctx.r[9].s64 = ctx.r[9].s64 + 6308;
	// 824D6EA8: 39081898  addi r8, r8, 0x1898
	ctx.r[8].s64 = ctx.r[8].s64 + 6296;
	// 824D6EAC: 38E7188C  addi r7, r7, 0x188c
	ctx.r[7].s64 = ctx.r[7].s64 + 6284;
	// 824D6EB0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D6EB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D6EB8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824D6EBC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824D6EC0: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 824D6EC4: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 824D6EC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824D6ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D6ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D6ED4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D6ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6EE0 size=8
    let mut pc: u32 = 0x824D6EE0;
    'dispatch: loop {
        match pc {
            0x824D6EE0 => {
    //   block [0x824D6EE0..0x824D6EE8)
	// 824D6EE0: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 824D6EE4: 4802796C  b 0x824fe850
	sub_824FE850(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6EE8 size=8
    let mut pc: u32 = 0x824D6EE8;
    'dispatch: loop {
        match pc {
            0x824D6EE8 => {
    //   block [0x824D6EE8..0x824D6EF0)
	// 824D6EE8: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 824D6EEC: 48027964  b 0x824fe850
	sub_824FE850(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D6EF0 size=104
    let mut pc: u32 = 0x824D6EF0;
    'dispatch: loop {
        match pc {
            0x824D6EF0 => {
    //   block [0x824D6EF0..0x824D6F58)
	// 824D6EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D6EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D6EF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824D6EFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D6F00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D6F04: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824D6F08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D6F0C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D6F10: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D6F14: 419A002C  beq cr6, 0x824d6f40
	if ctx.cr[6].eq {
	pc = 0x824D6F40; continue 'dispatch;
	}
	// 824D6F18: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6F1C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D6F20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D6F24: 4E800421  bctrl
	ctx.lr = 0x824D6F28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D6F28: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6F2C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824D6F30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D6F34: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D6F38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D6F3C: 4E800421  bctrl
	ctx.lr = 0x824D6F40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D6F40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824D6F44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D6F48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D6F4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824D6F50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D6F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D6F58 size=72
    let mut pc: u32 = 0x824D6F58;
    'dispatch: loop {
        match pc {
            0x824D6F58 => {
    //   block [0x824D6F58..0x824D6FA0)
	// 824D6F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D6F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D6F60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D6F64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D6F68: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D6F6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D6F70: 396B191C  addi r11, r11, 0x191c
	ctx.r[11].s64 = ctx.r[11].s64 + 6428;
	// 824D6F74: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 824D6F78: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824D6F7C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D6F80: 419A000C  beq cr6, 0x824d6f8c
	if ctx.cr[6].eq {
	pc = 0x824D6F8C; continue 'dispatch;
	}
	// 824D6F84: 4805BC35  bl 0x82532bb8
	ctx.lr = 0x824D6F88;
	sub_82532BB8(ctx, base);
	// 824D6F88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D6F8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824D6F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D6F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D6F98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D6F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6FA0 size=4
    let mut pc: u32 = 0x824D6FA0;
    'dispatch: loop {
        match pc {
            0x824D6FA0 => {
    //   block [0x824D6FA0..0x824D6FA4)
	// 824D6FA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6FA8 size=20
    let mut pc: u32 = 0x824D6FA8;
    'dispatch: loop {
        match pc {
            0x824D6FA8 => {
    //   block [0x824D6FA8..0x824D6FBC)
	// 824D6FA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6FAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D6FB0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D6FB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D6FB8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6FC0 size=12
    let mut pc: u32 = 0x824D6FC0;
    'dispatch: loop {
        match pc {
            0x824D6FC0 => {
    //   block [0x824D6FC0..0x824D6FCC)
	// 824D6FC0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824D6FC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824D6FC8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6FCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D6FCC size=8
    let mut pc: u32 = 0x824D6FCC;
    'dispatch: loop {
        match pc {
            0x824D6FCC => {
    //   block [0x824D6FCC..0x824D6FD4)
	// 824D6FCC: 4800003C  b 0x824d7008
	sub_824D7008(ctx, base);
	return;
	// 824D6FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D6FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D6FD8 size=44
    let mut pc: u32 = 0x824D6FD8;
    'dispatch: loop {
        match pc {
            0x824D6FD8 => {
    //   block [0x824D6FD8..0x824D7004)
	// 824D6FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D6FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D6FE0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D6FE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D6FE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824D6FEC: 4800001D  bl 0x824d7008
	ctx.lr = 0x824D6FF0;
	sub_824D7008(ctx, base);
	// 824D6FF0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D6FF4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824D6FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D6FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D7000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D7008 size=152
    let mut pc: u32 = 0x824D7008;
    'dispatch: loop {
        match pc {
            0x824D7008 => {
    //   block [0x824D7008..0x824D70A0)
	// 824D7008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D700C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D7010: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D7014: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D7018: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D701C: 480277A5  bl 0x824fe7c0
	ctx.lr = 0x824D7020;
	sub_824FE7C0(ctx, base);
	// 824D7020: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D7024: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824D7028: 396B191C  addi r11, r11, 0x191c
	ctx.r[11].s64 = ctx.r[11].s64 + 6428;
	// 824D702C: 394A1984  addi r10, r10, 0x1984
	ctx.r[10].s64 = ctx.r[10].s64 + 6532;
	// 824D7030: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824D7034: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824D7038: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 824D703C: 3CC08206  lis r6, -0x7dfa
	ctx.r[6].s64 = -2113536000;
	// 824D7040: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 824D7044: 3CA08206  lis r5, -0x7dfa
	ctx.r[5].s64 = -2113536000;
	// 824D7048: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824D704C: 39291978  addi r9, r9, 0x1978
	ctx.r[9].s64 = ctx.r[9].s64 + 6520;
	// 824D7050: 39081964  addi r8, r8, 0x1964
	ctx.r[8].s64 = ctx.r[8].s64 + 6500;
	// 824D7054: 38E71958  addi r7, r7, 0x1958
	ctx.r[7].s64 = ctx.r[7].s64 + 6488;
	// 824D7058: 38C6194C  addi r6, r6, 0x194c
	ctx.r[6].s64 = ctx.r[6].s64 + 6476;
	// 824D705C: 38A51934  addi r5, r5, 0x1934
	ctx.r[5].s64 = ctx.r[5].s64 + 6452;
	// 824D7060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824D7064: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824D7068: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 824D706C: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824D7070: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 824D7074: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D7078: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 824D707C: 90BF0030  stw r5, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[5].u32 ) };
	// 824D7080: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 824D7084: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 824D7088: 915F003C  stw r10, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 824D708C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824D7090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D7094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D7098: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D709C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D70A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D70A0 size=100
    let mut pc: u32 = 0x824D70A0;
    'dispatch: loop {
        match pc {
            0x824D70A0 => {
    //   block [0x824D70A0..0x824D7104)
	// 824D70A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D70A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D70A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824D70AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D70B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D70B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D70B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824D70BC: 48007A45  bl 0x824deb00
	ctx.lr = 0x824D70C0;
	sub_824DEB00(ctx, base);
	// 824D70C0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824D70C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D70C8: 419A0020  beq cr6, 0x824d70e8
	if ctx.cr[6].eq {
	pc = 0x824D70E8; continue 'dispatch;
	}
	// 824D70CC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D70D0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824D70D4: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824D70D8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D70DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824D70E0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824D70E4: 4BF8CFD5  bl 0x824640b8
	ctx.lr = 0x824D70E8;
	sub_824640B8(ctx, base);
	// 824D70E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D70EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824D70F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D70F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D70F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824D70FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D7100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D7108 size=8
    let mut pc: u32 = 0x824D7108;
    'dispatch: loop {
        match pc {
            0x824D7108 => {
    //   block [0x824D7108..0x824D7110)
	// 824D7108: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 824D710C: 4BFFFF94  b 0x824d70a0
	sub_824D70A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D7110 size=8
    let mut pc: u32 = 0x824D7110;
    'dispatch: loop {
        match pc {
            0x824D7110 => {
    //   block [0x824D7110..0x824D7118)
	// 824D7110: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 824D7114: 4BFFFF8C  b 0x824d70a0
	sub_824D70A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D7118 size=8
    let mut pc: u32 = 0x824D7118;
    'dispatch: loop {
        match pc {
            0x824D7118 => {
    //   block [0x824D7118..0x824D7120)
	// 824D7118: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 824D711C: 4BFFFF84  b 0x824d70a0
	sub_824D70A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D7120 size=8
    let mut pc: u32 = 0x824D7120;
    'dispatch: loop {
        match pc {
            0x824D7120 => {
    //   block [0x824D7120..0x824D7128)
	// 824D7120: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 824D7124: 4BFFFF7C  b 0x824d70a0
	sub_824D70A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D7128 size=8
    let mut pc: u32 = 0x824D7128;
    'dispatch: loop {
        match pc {
            0x824D7128 => {
    //   block [0x824D7128..0x824D7130)
	// 824D7128: 3863FFD0  addi r3, r3, -0x30
	ctx.r[3].s64 = ctx.r[3].s64 + -48;
	// 824D712C: 4BFFFF74  b 0x824d70a0
	sub_824D70A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D7130 size=20
    let mut pc: u32 = 0x824D7130;
    'dispatch: loop {
        match pc {
            0x824D7130 => {
    //   block [0x824D7130..0x824D7144)
	// 824D7130: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D7134: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D7138: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D713C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D7140: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D7148 size=12
    let mut pc: u32 = 0x824D7148;
    'dispatch: loop {
        match pc {
            0x824D7148 => {
    //   block [0x824D7148..0x824D7154)
	// 824D7148: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824D714C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824D7150: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7154(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D7154 size=8
    let mut pc: u32 = 0x824D7154;
    'dispatch: loop {
        match pc {
            0x824D7154 => {
    //   block [0x824D7154..0x824D715C)
	// 824D7154: 4800003C  b 0x824d7190
	sub_824D7190(ctx, base);
	return;
	// 824D7158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D7160 size=44
    let mut pc: u32 = 0x824D7160;
    'dispatch: loop {
        match pc {
            0x824D7160 => {
    //   block [0x824D7160..0x824D718C)
	// 824D7160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D7164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D7168: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D716C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D7170: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824D7174: 4800001D  bl 0x824d7190
	ctx.lr = 0x824D7178;
	sub_824D7190(ctx, base);
	// 824D7178: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D717C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824D7180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D7184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D7188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D7190 size=152
    let mut pc: u32 = 0x824D7190;
    'dispatch: loop {
        match pc {
            0x824D7190 => {
    //   block [0x824D7190..0x824D7228)
	// 824D7190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D7194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D7198: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D719C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D71A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D71A4: 4802761D  bl 0x824fe7c0
	ctx.lr = 0x824D71A8;
	sub_824FE7C0(ctx, base);
	// 824D71A8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D71AC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824D71B0: 396B191C  addi r11, r11, 0x191c
	ctx.r[11].s64 = ctx.r[11].s64 + 6428;
	// 824D71B4: 394A1A00  addi r10, r10, 0x1a00
	ctx.r[10].s64 = ctx.r[10].s64 + 6656;
	// 824D71B8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824D71BC: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824D71C0: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 824D71C4: 3CC08206  lis r6, -0x7dfa
	ctx.r[6].s64 = -2113536000;
	// 824D71C8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 824D71CC: 3CA08206  lis r5, -0x7dfa
	ctx.r[5].s64 = -2113536000;
	// 824D71D0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824D71D4: 392919F4  addi r9, r9, 0x19f4
	ctx.r[9].s64 = ctx.r[9].s64 + 6644;
	// 824D71D8: 390819E0  addi r8, r8, 0x19e0
	ctx.r[8].s64 = ctx.r[8].s64 + 6624;
	// 824D71DC: 38E719D4  addi r7, r7, 0x19d4
	ctx.r[7].s64 = ctx.r[7].s64 + 6612;
	// 824D71E0: 38C619C8  addi r6, r6, 0x19c8
	ctx.r[6].s64 = ctx.r[6].s64 + 6600;
	// 824D71E4: 38A519B0  addi r5, r5, 0x19b0
	ctx.r[5].s64 = ctx.r[5].s64 + 6576;
	// 824D71E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824D71EC: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824D71F0: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 824D71F4: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824D71F8: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 824D71FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D7200: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 824D7204: 90BF0030  stw r5, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[5].u32 ) };
	// 824D7208: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 824D720C: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 824D7210: 915F003C  stw r10, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 824D7214: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824D7218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D721C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D7220: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D7224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D7228 size=100
    let mut pc: u32 = 0x824D7228;
    'dispatch: loop {
        match pc {
            0x824D7228 => {
    //   block [0x824D7228..0x824D728C)
	// 824D7228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D722C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D7230: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824D7234: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D7238: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D723C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D7240: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824D7244: 48008085  bl 0x824df2c8
	ctx.lr = 0x824D7248;
	sub_824DF2C8(ctx, base);
	// 824D7248: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824D724C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D7250: 419A0020  beq cr6, 0x824d7270
	if ctx.cr[6].eq {
	pc = 0x824D7270; continue 'dispatch;
	}
	// 824D7254: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D7258: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824D725C: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824D7260: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D7264: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824D7268: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824D726C: 4BF8CE4D  bl 0x824640b8
	ctx.lr = 0x824D7270;
	sub_824640B8(ctx, base);
	// 824D7270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D7274: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824D7278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D727C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D7280: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824D7284: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D7288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D7290 size=8
    let mut pc: u32 = 0x824D7290;
    'dispatch: loop {
        match pc {
            0x824D7290 => {
    //   block [0x824D7290..0x824D7298)
	// 824D7290: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 824D7294: 4BFFFF94  b 0x824d7228
	sub_824D7228(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D7298 size=8
    let mut pc: u32 = 0x824D7298;
    'dispatch: loop {
        match pc {
            0x824D7298 => {
    //   block [0x824D7298..0x824D72A0)
	// 824D7298: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 824D729C: 4BFFFF8C  b 0x824d7228
	sub_824D7228(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D72A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D72A0 size=8
    let mut pc: u32 = 0x824D72A0;
    'dispatch: loop {
        match pc {
            0x824D72A0 => {
    //   block [0x824D72A0..0x824D72A8)
	// 824D72A0: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 824D72A4: 4BFFFF84  b 0x824d7228
	sub_824D7228(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D72A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D72A8 size=8
    let mut pc: u32 = 0x824D72A8;
    'dispatch: loop {
        match pc {
            0x824D72A8 => {
    //   block [0x824D72A8..0x824D72B0)
	// 824D72A8: 3863FFD0  addi r3, r3, -0x30
	ctx.r[3].s64 = ctx.r[3].s64 + -48;
	// 824D72AC: 4BFFFF7C  b 0x824d7228
	sub_824D7228(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D72B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D72B0 size=8
    let mut pc: u32 = 0x824D72B0;
    'dispatch: loop {
        match pc {
            0x824D72B0 => {
    //   block [0x824D72B0..0x824D72B8)
	// 824D72B0: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 824D72B4: 4BFFFF74  b 0x824d7228
	sub_824D7228(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D72B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D72B8 size=8
    let mut pc: u32 = 0x824D72B8;
    'dispatch: loop {
        match pc {
            0x824D72B8 => {
    //   block [0x824D72B8..0x824D72C0)
	// 824D72B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824D72BC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D72C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D72C0 size=24
    let mut pc: u32 = 0x824D72C0;
    'dispatch: loop {
        match pc {
            0x824D72C0 => {
    //   block [0x824D72C0..0x824D72D8)
	// 824D72C0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D72C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824D72C8: 396B1A98  addi r11, r11, 0x1a98
	ctx.r[11].s64 = ctx.r[11].s64 + 6808;
	// 824D72CC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824D72D0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D72D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D72D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D72D8 size=20
    let mut pc: u32 = 0x824D72D8;
    'dispatch: loop {
        match pc {
            0x824D72D8 => {
    //   block [0x824D72D8..0x824D72EC)
	// 824D72D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D72DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D72E0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D72E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D72E8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D72F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D72F0 size=12
    let mut pc: u32 = 0x824D72F0;
    'dispatch: loop {
        match pc {
            0x824D72F0 => {
    //   block [0x824D72F0..0x824D72FC)
	// 824D72F0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D72F4: 386B1A98  addi r3, r11, 0x1a98
	ctx.r[3].s64 = ctx.r[11].s64 + 6808;
	// 824D72F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D7300 size=100
    let mut pc: u32 = 0x824D7300;
    'dispatch: loop {
        match pc {
            0x824D7300 => {
    //   block [0x824D7300..0x824D7364)
	// 824D7300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D7304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D7308: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824D730C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D7310: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D7314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D7318: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824D731C: 480D774D  bl 0x825aea68
	ctx.lr = 0x824D7320;
	sub_825AEA68(ctx, base);
	// 824D7320: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824D7324: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D7328: 419A0020  beq cr6, 0x824d7348
	if ctx.cr[6].eq {
	pc = 0x824D7348; continue 'dispatch;
	}
	// 824D732C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D7330: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824D7334: 38C00029  li r6, 0x29
	ctx.r[6].s64 = 41;
	// 824D7338: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D733C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824D7340: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824D7344: 4BF8CD75  bl 0x824640b8
	ctx.lr = 0x824D7348;
	sub_824640B8(ctx, base);
	// 824D7348: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D734C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824D7350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D7354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D7358: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824D735C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D7360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D7368 size=20
    let mut pc: u32 = 0x824D7368;
    'dispatch: loop {
        match pc {
            0x824D7368 => {
    //   block [0x824D7368..0x824D737C)
	// 824D7368: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D736C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D7370: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D7374: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D7378: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D7380 size=8
    let mut pc: u32 = 0x824D7380;
    'dispatch: loop {
        match pc {
            0x824D7380 => {
    //   block [0x824D7380..0x824D7388)
	// 824D7380: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824D7384: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D7388 size=24
    let mut pc: u32 = 0x824D7388;
    'dispatch: loop {
        match pc {
            0x824D7388 => {
    //   block [0x824D7388..0x824D73A0)
	// 824D7388: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D738C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824D7390: 396B1AD4  addi r11, r11, 0x1ad4
	ctx.r[11].s64 = ctx.r[11].s64 + 6868;
	// 824D7394: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824D7398: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D739C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D73A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D73A0 size=12
    let mut pc: u32 = 0x824D73A0;
    'dispatch: loop {
        match pc {
            0x824D73A0 => {
    //   block [0x824D73A0..0x824D73AC)
	// 824D73A0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D73A4: 386B1AD4  addi r3, r11, 0x1ad4
	ctx.r[3].s64 = ctx.r[11].s64 + 6868;
	// 824D73A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D73B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D73B0 size=196
    let mut pc: u32 = 0x824D73B0;
    'dispatch: loop {
        match pc {
            0x824D73B0 => {
    //   block [0x824D73B0..0x824D7474)
	// 824D73B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D73B4: 4805DD09  bl 0x825350bc
	ctx.lr = 0x824D73B8;
	sub_82535080(ctx, base);
	// 824D73B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D73BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D73C0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D73C4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824D73C8: 396B1AD4  addi r11, r11, 0x1ad4
	ctx.r[11].s64 = ctx.r[11].s64 + 6868;
	// 824D73CC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824D73D0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824D73D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D73D8: 4099005C  ble cr6, 0x824d7434
	if !ctx.cr[6].gt {
	pc = 0x824D7434; continue 'dispatch;
	}
	// 824D73DC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824D73E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D73E4: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824D73E8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D73EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D73F0: 419A0030  beq cr6, 0x824d7420
	if ctx.cr[6].eq {
	pc = 0x824D7420; continue 'dispatch;
	}
	// 824D73F4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824D73F8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824D73FC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824D7400: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D7404: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824D7408: 409A0018  bne cr6, 0x824d7420
	if !ctx.cr[6].eq {
	pc = 0x824D7420; continue 'dispatch;
	}
	// 824D740C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D7410: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824D7414: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D7418: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D741C: 4E800421  bctrl
	ctx.lr = 0x824D7420;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824D7420: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824D7424: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824D7428: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 824D742C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824D7430: 4198FFB0  blt cr6, 0x824d73e0
	if ctx.cr[6].lt {
	pc = 0x824D73E0; continue 'dispatch;
	}
	// 824D7434: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D7438: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824D743C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824D7440: 409A0020  bne cr6, 0x824d7460
	if !ctx.cr[6].eq {
	pc = 0x824D7460; continue 'dispatch;
	}
	// 824D7444: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D7448: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824D744C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824D7450: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D7454: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824D7458: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824D745C: 4BF8CC5D  bl 0x824640b8
	ctx.lr = 0x824D7460;
	sub_824640B8(ctx, base);
	// 824D7460: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824D7464: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824D7468: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D746C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824D7470: 4805DC9C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D7478 size=100
    let mut pc: u32 = 0x824D7478;
    'dispatch: loop {
        match pc {
            0x824D7478 => {
    //   block [0x824D7478..0x824D74DC)
	// 824D7478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D747C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D7480: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824D7484: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D7488: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D748C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D7490: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824D7494: 4BFFFF1D  bl 0x824d73b0
	ctx.lr = 0x824D7498;
	sub_824D73B0(ctx, base);
	// 824D7498: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824D749C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D74A0: 419A0020  beq cr6, 0x824d74c0
	if ctx.cr[6].eq {
	pc = 0x824D74C0; continue 'dispatch;
	}
	// 824D74A4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D74A8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824D74AC: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 824D74B0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D74B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824D74B8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824D74BC: 4BF8CBFD  bl 0x824640b8
	ctx.lr = 0x824D74C0;
	sub_824640B8(ctx, base);
	// 824D74C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D74C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824D74C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D74CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D74D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824D74D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D74D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D74E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D74E0 size=4
    let mut pc: u32 = 0x824D74E0;
    'dispatch: loop {
        match pc {
            0x824D74E0 => {
    //   block [0x824D74E0..0x824D74E4)
	// 824D74E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D74E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D74E8 size=20
    let mut pc: u32 = 0x824D74E8;
    'dispatch: loop {
        match pc {
            0x824D74E8 => {
    //   block [0x824D74E8..0x824D74FC)
	// 824D74E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D74EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D74F0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D74F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D74F8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D7500 size=8
    let mut pc: u32 = 0x824D7500;
    'dispatch: loop {
        match pc {
            0x824D7500 => {
    //   block [0x824D7500..0x824D7508)
	// 824D7500: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824D7504: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D7508 size=24
    let mut pc: u32 = 0x824D7508;
    'dispatch: loop {
        match pc {
            0x824D7508 => {
    //   block [0x824D7508..0x824D7520)
	// 824D7508: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D750C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824D7510: 396B1B2C  addi r11, r11, 0x1b2c
	ctx.r[11].s64 = ctx.r[11].s64 + 6956;
	// 824D7514: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824D7518: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D751C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D7520 size=12
    let mut pc: u32 = 0x824D7520;
    'dispatch: loop {
        match pc {
            0x824D7520 => {
    //   block [0x824D7520..0x824D752C)
	// 824D7520: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D7524: 386B1B2C  addi r3, r11, 0x1b2c
	ctx.r[3].s64 = ctx.r[11].s64 + 6956;
	// 824D7528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D7530 size=100
    let mut pc: u32 = 0x824D7530;
    'dispatch: loop {
        match pc {
            0x824D7530 => {
    //   block [0x824D7530..0x824D7594)
	// 824D7530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D7534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D7538: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824D753C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D7540: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D7544: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D7548: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824D754C: 48008265  bl 0x824df7b0
	ctx.lr = 0x824D7550;
	sub_824DF7B0(ctx, base);
	// 824D7550: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824D7554: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824D7558: 419A0020  beq cr6, 0x824d7578
	if ctx.cr[6].eq {
	pc = 0x824D7578; continue 'dispatch;
	}
	// 824D755C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D7560: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824D7564: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 824D7568: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D756C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824D7570: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824D7574: 4BF8CB45  bl 0x824640b8
	ctx.lr = 0x824D7578;
	sub_824640B8(ctx, base);
	// 824D7578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D757C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824D7580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D7584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D7588: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824D758C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D7590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D7598 size=8
    let mut pc: u32 = 0x824D7598;
    'dispatch: loop {
        match pc {
            0x824D7598 => {
    //   block [0x824D7598..0x824D75A0)
	// 824D7598: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824D759C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D75A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D75A0 size=24
    let mut pc: u32 = 0x824D75A0;
    'dispatch: loop {
        match pc {
            0x824D75A0 => {
    //   block [0x824D75A0..0x824D75B8)
	// 824D75A0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D75A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824D75A8: 396B1B9C  addi r11, r11, 0x1b9c
	ctx.r[11].s64 = ctx.r[11].s64 + 7068;
	// 824D75AC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824D75B0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D75B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D75B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D75B8 size=20
    let mut pc: u32 = 0x824D75B8;
    'dispatch: loop {
        match pc {
            0x824D75B8 => {
    //   block [0x824D75B8..0x824D75CC)
	// 824D75B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D75BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D75C0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D75C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D75C8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D75D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D75D0 size=12
    let mut pc: u32 = 0x824D75D0;
    'dispatch: loop {
        match pc {
            0x824D75D0 => {
    //   block [0x824D75D0..0x824D75DC)
	// 824D75D0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D75D4: 386B1B9C  addi r3, r11, 0x1b9c
	ctx.r[3].s64 = ctx.r[11].s64 + 7068;
	// 824D75D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D75E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D75E0 size=20
    let mut pc: u32 = 0x824D75E0;
    'dispatch: loop {
        match pc {
            0x824D75E0 => {
    //   block [0x824D75E0..0x824D75F4)
	// 824D75E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D75E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D75E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D75EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824D75F0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D75F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D75F8 size=8
    let mut pc: u32 = 0x824D75F8;
    'dispatch: loop {
        match pc {
            0x824D75F8 => {
    //   block [0x824D75F8..0x824D7600)
	// 824D75F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824D75FC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D7600 size=24
    let mut pc: u32 = 0x824D7600;
    'dispatch: loop {
        match pc {
            0x824D7600 => {
    //   block [0x824D7600..0x824D7618)
	// 824D7600: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D7604: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824D7608: 396B1BD8  addi r11, r11, 0x1bd8
	ctx.r[11].s64 = ctx.r[11].s64 + 7128;
	// 824D760C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824D7610: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D7614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D7618 size=12
    let mut pc: u32 = 0x824D7618;
    'dispatch: loop {
        match pc {
            0x824D7618 => {
    //   block [0x824D7618..0x824D7624)
	// 824D7618: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824D761C: 386B1BD8  addi r3, r11, 0x1bd8
	ctx.r[3].s64 = ctx.r[11].s64 + 7128;
	// 824D7620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824D7628 size=96
    let mut pc: u32 = 0x824D7628;
    'dispatch: loop {
        match pc {
            0x824D7628 => {
    //   block [0x824D7628..0x824D7688)
	// 824D7628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D762C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D7630: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D7634: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D7638: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824D763C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D7640: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824D7644: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 824D7648: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824D764C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824D7650: 419A0020  beq cr6, 0x824d7670
	if ctx.cr[6].eq {
	pc = 0x824D7670; continue 'dispatch;
	}
	// 824D7654: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D7658: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824D765C: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 824D7660: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D7664: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824D7668: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824D766C: 4BF8CA4D  bl 0x824640b8
	ctx.lr = 0x824D7670;
	sub_824640B8(ctx, base);
	// 824D7670: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824D7674: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824D7678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D767C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D7680: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D7684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824D7688 size=24
    let mut pc: u32 = 0x824D7688;
    'dispatch: loop {
        match pc {
            0x824D7688 => {
    //   block [0x824D7688..0x824D76A0)
	// 824D7688: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D768C: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D7690: FF020000  fcmpu cr6, f2, f0
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[0].f64);
	// 824D7694: 4199000C  bgt cr6, 0x824d76a0
	if ctx.cr[6].gt {
		sub_824D76A0(ctx, base);
		return;
	}
	// 824D7698: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824D769C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D76A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824D76A0 size=144
    let mut pc: u32 = 0x824D76A0;
    'dispatch: loop {
        match pc {
            0x824D76A0 => {
    //   block [0x824D76A0..0x824D7730)
	// 824D76A0: FF010000  fcmpu cr6, f1, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 824D76A4: 4099FFF4  ble cr6, 0x824d7698
	if !ctx.cr[6].gt {
		sub_824D7688(ctx, base);
		return;
	}
	// 824D76A8: EDA10072  fmuls f13, f1, f1
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[1].f64) as f32) as f64);
	// 824D76AC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 824D76B0: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 824D76B4: 39650020  addi r11, r5, 0x20
	ctx.r[11].s64 = ctx.r[5].s64 + 32;
	// 824D76B8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 824D76BC: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 824D76C0: C0091850  lfs f0, 0x1850(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D76C4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824D7730 size=24
    let mut pc: u32 = 0x824D7730;
    'dispatch: loop {
        match pc {
            0x824D7730 => {
    //   block [0x824D7730..0x824D7748)
	// 824D7730: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D7734: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D7738: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 824D773C: 4199000C  bgt cr6, 0x824d7748
	if ctx.cr[6].gt {
		sub_824D7748(ctx, base);
		return;
	}
	// 824D7740: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824D7744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824D7748 size=180
    let mut pc: u32 = 0x824D7748;
    'dispatch: loop {
        match pc {
            0x824D7748 => {
    //   block [0x824D7748..0x824D77FC)
	// 824D7748: C1A30004  lfs f13, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D774C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 824D7750: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7754: ED0D0372  fmuls f8, f13, f13
	ctx.f[8].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7758: EDAC0372  fmuls f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D775C: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 824D7760: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D7764: ECEC0332  fmuls f7, f12, f12
	ctx.f[7].f64 = (((ctx.f[12].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7768: ED200032  fmuls f9, f0, f0
	ctx.f[9].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D776C: 39650020  addi r11, r5, 0x20
	ctx.r[11].s64 = ctx.r[5].s64 + 32;
	// 824D7770: C1491850  lfs f10, 0x1850(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(6224 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 824D7774: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 824D7778: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824D7800 size=64
    let mut pc: u32 = 0x824D7800;
    'dispatch: loop {
        match pc {
            0x824D7800 => {
    //   block [0x824D7800..0x824D7840)
	// 824D7800: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824D7804: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D7808: 38E00080  li r7, 0x80
	ctx.r[7].s64 = 128;
	// 824D780C: D0040090  stfs f0, 0x90(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 824D7810: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 824D7814: 39440050  addi r10, r4, 0x50
	ctx.r[10].s64 = ctx.r[4].s64 + 80;
	// 824D7818: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824D7840 size=128
    let mut pc: u32 = 0x824D7840;
    'dispatch: loop {
        match pc {
            0x824D7840 => {
    //   block [0x824D7840..0x824D78C0)
	// 824D7840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D7844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D7848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D784C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824D7850: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D7854: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 824D7858: EC010024  fdivs f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 / ctx.f[0].f64) as f32) as f64;
	// 824D785C: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 824D7860: D0250090  stfs f1, 0x90(r5)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 824D7864: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 824D7868: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 824D786C: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D78C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824D78C0 size=12
    let mut pc: u32 = 0x824D78C0;
    'dispatch: loop {
        match pc {
            0x824D78C0 => {
    //   block [0x824D78C0..0x824D78CC)
	// 824D78C0: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D78C4: EC200072  fmuls f1, f0, f1
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 824D78C8: 4BFFFF78  b 0x824d7840
	sub_824D7840(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D78D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824D78D0 size=72
    let mut pc: u32 = 0x824D78D0;
    'dispatch: loop {
        match pc {
            0x824D78D0 => {
    //   block [0x824D78D0..0x824D7918)
	// 824D78D0: C1A40050  lfs f13, 0x50(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D78D4: C1840064  lfs f12, 0x64(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(100 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D78D8: EC0D6028  fsubs f0, f13, f12
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 824D78DC: C1640078  lfs f11, 0x78(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(120 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824D78E0: FC00636E  fsel f0, f0, f13, f12
	ctx.f[0].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[12].f64 };
	// 824D78E4: ED405828  fsubs f10, f0, f11
	ctx.f[10].f64 = (((ctx.f[0].f64 - ctx.f[11].f64) as f32) as f64);
	// 824D78E8: FC0A582E  fsel f0, f10, f0, f11
	ctx.f[0].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[11].f64 };
	// 824D78EC: EC000824  fdivs f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[1].f64) as f32) as f64;
	// 824D78F0: ED4D0028  fsubs f10, f13, f0
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 824D78F4: ED2C0028  fsubs f9, f12, f0
	ctx.f[9].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 824D78F8: ED0B0028  fsubs f8, f11, f0
	ctx.f[8].f64 = (((ctx.f[11].f64 - ctx.f[0].f64) as f32) as f64);
	// 824D78FC: FDAA036E  fsel f13, f10, f13, f0
	ctx.f[13].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 824D7900: D1A40050  stfs f13, 0x50(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 824D7904: FDA9032E  fsel f13, f9, f12, f0
	ctx.f[13].f64 = if ctx.f[9].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[0].f64 };
	// 824D7908: D1A40064  stfs f13, 0x64(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 824D790C: FC0802EE  fsel f0, f8, f11, f0
	ctx.f[0].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[11].f64 } else { ctx.f[0].f64 };
	// 824D7910: D0040078  stfs f0, 0x78(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 824D7914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824D7918 size=172
    let mut pc: u32 = 0x824D7918;
    'dispatch: loop {
        match pc {
            0x824D7918 => {
    //   block [0x824D7918..0x824D79C4)
	// 824D7918: C1A30008  lfs f13, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D791C: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7920: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D7924: C1850000  lfs f12, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7928: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 824D792C: EC00607C  fnmsubs f0, f0, f1, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[1].f64 - ctx.f[12].f64) as f32) as f64);
	// 824D7930: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824D7934: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D7938: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D793C: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D7940: C1850014  lfs f12, 0x14(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7944: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 824D7948: EC00607C  fnmsubs f0, f0, f1, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[1].f64 - ctx.f[12].f64) as f32) as f64);
	// 824D794C: D0050014  stfs f0, 0x14(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824D7950: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D7954: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7958: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D795C: C1850028  lfs f12, 0x28(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7960: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 824D7964: EC00607C  fnmsubs f0, f0, f1, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[1].f64 - ctx.f[12].f64) as f32) as f64);
	// 824D7968: D0050028  stfs f0, 0x28(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 824D796C: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D7970: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 824D7974: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D7978: C1850004  lfs f12, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D797C: EC00637A  fmadds f0, f0, f13, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64);
	// 824D7980: D0050004  stfs f0, 4(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824D7984: D0050010  stfs f0, 0x10(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824D7988: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D798C: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D7990: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D7994: C1850018  lfs f12, 0x18(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7998: EC00637A  fmadds f0, f0, f13, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64);
	// 824D799C: D0050018  stfs f0, 0x18(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824D79A0: D0050024  stfs f0, 0x24(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 824D79A4: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D79A8: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 824D79AC: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D79B0: C1850020  lfs f12, 0x20(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D79B4: EC00637A  fmadds f0, f0, f13, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64);
	// 824D79B8: D0050020  stfs f0, 0x20(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824D79BC: D0050008  stfs f0, 8(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824D79C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D79C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824D79C8 size=172
    let mut pc: u32 = 0x824D79C8;
    'dispatch: loop {
        match pc {
            0x824D79C8 => {
    //   block [0x824D79C8..0x824D7A74)
	// 824D79C8: C1A30008  lfs f13, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D79CC: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D79D0: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D79D4: C1850000  lfs f12, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D79D8: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 824D79DC: EC00607A  fmadds f0, f0, f1, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[12].f64) as f32) as f64);
	// 824D79E0: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824D79E4: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D79E8: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D79EC: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D79F0: C1850014  lfs f12, 0x14(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D79F4: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 824D79F8: EC00607A  fmadds f0, f0, f1, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[12].f64) as f32) as f64);
	// 824D79FC: D0050014  stfs f0, 0x14(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824D7A00: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D7A04: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7A08: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D7A0C: C1850028  lfs f12, 0x28(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7A10: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 824D7A14: EC00607A  fmadds f0, f0, f1, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[12].f64) as f32) as f64);
	// 824D7A18: D0050028  stfs f0, 0x28(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 824D7A1C: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D7A20: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 824D7A24: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D7A28: C1850004  lfs f12, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7A2C: EC00637C  fnmsubs f0, f0, f13, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 824D7A30: D0050004  stfs f0, 4(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824D7A34: D0050010  stfs f0, 0x10(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824D7A38: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D7A3C: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D7A40: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D7A44: C1850018  lfs f12, 0x18(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7A48: EC00637C  fnmsubs f0, f0, f13, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 824D7A4C: D0050018  stfs f0, 0x18(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824D7A50: D0050024  stfs f0, 0x24(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 824D7A54: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D7A58: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 824D7A5C: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D7A60: C1850020  lfs f12, 0x20(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7A64: EC00637C  fnmsubs f0, f0, f13, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 824D7A68: D0050020  stfs f0, 0x20(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824D7A6C: D0050008  stfs f0, 8(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824D7A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824D7A78 size=80
    let mut pc: u32 = 0x824D7A78;
    'dispatch: loop {
        match pc {
            0x824D7A78 => {
    //   block [0x824D7A78..0x824D7AC8)
	// 824D7A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D7A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D7A80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D7A84: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 824D7A88: D0210084  stfs f1, 0x84(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 824D7A8C: 39410084  addi r10, r1, 0x84
	ctx.r[10].s64 = ctx.r[1].s64 + 132;
	// 824D7A90: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 824D7A94: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824D7A98: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824D7AC8 size=76
    let mut pc: u32 = 0x824D7AC8;
    'dispatch: loop {
        match pc {
            0x824D7AC8 => {
    //   block [0x824D7AC8..0x824D7B14)
	// 824D7AC8: C1A30000  lfs f13, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D7ACC: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 824D7AD0: C0030014  lfs f0, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D7AD4: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 824D7AD8: ED6D0028  fsubs f11, f13, f0
	ctx.f[11].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 824D7ADC: C1830028  lfs f12, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824D7B18 size=688
    let mut pc: u32 = 0x824D7B18;
    'dispatch: loop {
        match pc {
            0x824D7B18 => {
    //   block [0x824D7B18..0x824D7DC8)
	// 824D7B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D7B1C: 4805D5A1  bl 0x825350bc
	ctx.lr = 0x824D7B20;
	sub_82535080(ctx, base);
	// 824D7B20: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 824D7B24: 4805E491  bl 0x82535fb4
	ctx.lr = 0x824D7B28;
	sub_82535FB0(ctx, base);
	// 824D7B28: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D7B2C: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D7B30: 83C30004  lwz r30, 4(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D7B34: 3FA08200  lis r29, -0x7e00
	ctx.r[29].s64 = -2113929216;
	// 824D7B38: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 824D7B3C: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 824D7B40: 57E9103A  slwi r9, r31, 2
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D7B44: 57C8103A  slwi r8, r30, 2
	ctx.r[8].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824D7B48: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D7B4C: 3CA05555  lis r5, 0x5555
	ctx.r[5].s64 = 1431633920;
	// 824D7B50: C0BD294C  lfs f5, 0x294c(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(10572 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 824D7B54: C0C62940  lfs f6, 0x2940(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(10560 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 824D7B58: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824D7B5C: C0272068  lfs f1, 0x2068(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8296 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824D7B60: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 824D7B64: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 824D7B68: 7D292214  add r9, r9, r4
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 824D7B6C: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824D7B70: 7D082214  add r8, r8, r4
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 824D7B74: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 824D7B78: 60A55556  ori r5, r5, 0x5556
	ctx.r[5].u64 = ctx.r[5].u64 | 21846;
	// 824D7B7C: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 824D7B80: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824D7B84: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 824D7B88: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824D7B8C: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824D7B90: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824D7B94: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824D7B98: 7CEB2896  mulhw r7, r11, r5
	ctx.r[7].s64 = ((ctx.r[11].s32 as i64 * ctx.r[5].s32 as i64) >> 32);
	// 824D7B9C: C1890000  lfs f12, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7BA0: 54E60FFE  srwi r6, r7, 0x1f
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shr(31);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824D7BA4: C1680000  lfs f11, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824D7BA8: ED2C0332  fmuls f9, f12, f12
	ctx.f[9].f64 = (((ctx.f[12].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7BAC: C323000C  lfs f25, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 824D7BB0: C2E30010  lfs f23, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 824D7BB4: EC8B02F2  fmuls f4, f11, f11
	ctx.f[4].f64 = (((ctx.f[11].f64 * ctx.f[11].f64) as f32) as f64);
	// 824D7BB8: C2A30024  lfs f21, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[21].f64 = (tmp.f32 as f64);
	// 824D7BBC: C2C30018  lfs f22, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 824D7BC0: C263001C  lfs f19, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[19].f64 = (tmp.f32 as f64);
	// 824D7BC4: C2430028  lfs f18, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[18].f64 = (tmp.f32 as f64);
	// 824D7BC8: C2830030  lfs f20, 0x30(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[20].f64 = (tmp.f32 as f64);
	// 824D7BCC: C223002C  lfs f17, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[17].f64 = (tmp.f32 as f64);
	// 824D7BD0: EC690332  fmuls f3, f9, f12
	ctx.f[3].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7BD4: ED0402F2  fmuls f8, f4, f11
	ctx.f[8].f64 = (((ctx.f[4].f64 * ctx.f[11].f64) as f32) as f64);
	// 824D7BD8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824D7BDC: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 824D7BE0: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 824D7BE4: 54E6083C  slwi r6, r7, 1
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824D7BE8: 39080010  addi r8, r8, 0x10
	ctx.r[8].s64 = ctx.r[8].s64 + 16;
	// 824D7BEC: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 824D7BF0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824D7BF4: 7CE75850  subf r7, r7, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 824D7BF8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824D7BFC: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824D7C00: 7CC7FA14  add r6, r7, r31
	ctx.r[6].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 824D7C04: 7CE7F214  add r7, r7, r30
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[30].u64;
	// 824D7C08: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824D7C0C: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824D7C10: 7C06242E  lfsx f0, r6, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D7C14: EF800332  fmuls f28, f0, f12
	ctx.f[28].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7C18: 7DA7242E  lfsx f13, r7, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D7C1C: ED4D5828  fsubs f10, f13, f11
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[11].f64) as f32) as f64);
	// 824D7C20: EFE0602A  fadds f31, f0, f12
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 824D7C24: EC400032  fmuls f2, f0, f0
	ctx.f[2].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D7C28: EFAD582A  fadds f29, f13, f11
	ctx.f[29].f64 = ((ctx.f[13].f64 + ctx.f[11].f64) as f32) as f64;
	// 824D7C2C: EFCD0372  fmuls f30, f13, f13
	ctx.f[30].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7C30: ECE06028  fsubs f7, f0, f12
	ctx.f[7].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 824D7C34: EF9C0072  fmuls f28, f28, f1
	ctx.f[28].f64 = (((ctx.f[28].f64 * ctx.f[1].f64) as f32) as f64);
	// 824D7C38: EF1FCABA  fmadds f24, f31, f10, f25
	ctx.f[24].f64 = (((ctx.f[31].f64 * ctx.f[10].f64 + ctx.f[25].f64) as f32) as f64);
	// 824D7C3C: D303000C  stfs f24, 0xc(r3)
	tmp.f32 = (ctx.f[24].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824D7C40: EF5F483A  fmadds f26, f31, f0, f9
	ctx.f[26].f64 = (((ctx.f[31].f64 * ctx.f[0].f64 + ctx.f[9].f64) as f32) as f64);
	// 824D7C44: EFBD237A  fmadds f29, f29, f13, f4
	ctx.f[29].f64 = (((ctx.f[29].f64 * ctx.f[13].f64 + ctx.f[4].f64) as f32) as f64);
	// 824D7C48: EF7E0372  fmuls f27, f30, f13
	ctx.f[27].f64 = (((ctx.f[30].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7C4C: EFDE02F2  fmuls f30, f30, f11
	ctx.f[30].f64 = (((ctx.f[30].f64 * ctx.f[11].f64) as f32) as f64);
	// 824D7C50: EC840372  fmuls f4, f4, f13
	ctx.f[4].f64 = (((ctx.f[4].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7C54: EF22E1BA  fmadds f25, f2, f6, f28
	ctx.f[25].f64 = (((ctx.f[2].f64 * ctx.f[6].f64 + ctx.f[28].f64) as f32) as f64);
	// 824D7C58: EF89E1BA  fmadds f28, f9, f6, f28
	ctx.f[28].f64 = (((ctx.f[9].f64 * ctx.f[6].f64 + ctx.f[28].f64) as f32) as f64);
	// 824D7C5C: EFFD437A  fmadds f31, f29, f13, f8
	ctx.f[31].f64 = (((ctx.f[29].f64 * ctx.f[13].f64 + ctx.f[8].f64) as f32) as f64);
	// 824D7C60: ED39482A  fadds f9, f25, f9
	ctx.f[9].f64 = ((ctx.f[25].f64 + ctx.f[9].f64) as f32) as f64;
	// 824D7C64: EF3A183A  fmadds f25, f26, f0, f3
	ctx.f[25].f64 = (((ctx.f[26].f64 * ctx.f[0].f64 + ctx.f[3].f64) as f32) as f64);
	// 824D7C68: EF5ABABA  fmadds f26, f26, f10, f23
	ctx.f[26].f64 = (((ctx.f[26].f64 * ctx.f[10].f64 + ctx.f[23].f64) as f32) as f64);
	// 824D7C6C: D3430010  stfs f26, 0x10(r3)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824D7C70: EF9C102A  fadds f28, f28, f2
	ctx.f[28].f64 = ((ctx.f[28].f64 + ctx.f[2].f64) as f32) as f64;
	// 824D7C74: EC420032  fmuls f2, f2, f0
	ctx.f[2].f64 = (((ctx.f[2].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D7C78: EDFF0372  fmuls f15, f31, f13
	ctx.f[15].f64 = (((ctx.f[31].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7C7C: EE090332  fmuls f16, f9, f12
	ctx.f[16].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7C80: EEF90032  fmuls f23, f25, f0
	ctx.f[23].f64 = (((ctx.f[25].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D7C84: EF39B2BA  fmadds f25, f25, f10, f22
	ctx.f[25].f64 = (((ctx.f[25].f64 * ctx.f[10].f64 + ctx.f[22].f64) as f32) as f64);
	// 824D7C88: C2C30014  lfs f22, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 824D7C8C: D3230018  stfs f25, 0x18(r3)
	tmp.f32 = (ctx.f[25].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824D7C90: EC42817A  fmadds f2, f2, f5, f16
	ctx.f[2].f64 = (((ctx.f[2].f64 * ctx.f[5].f64 + ctx.f[16].f64) as f32) as f64);
	// 824D7C94: EEE3BB3A  fmadds f23, f3, f12, f23
	ctx.f[23].f64 = (((ctx.f[3].f64 * ctx.f[12].f64 + ctx.f[23].f64) as f32) as f64);
	// 824D7C98: EC630172  fmuls f3, f3, f5
	ctx.f[3].f64 = (((ctx.f[3].f64 * ctx.f[5].f64) as f32) as f64);
	// 824D7C9C: EE087AFA  fmadds f16, f8, f11, f15
	ctx.f[16].f64 = (((ctx.f[8].f64 * ctx.f[11].f64 + ctx.f[15].f64) as f32) as f64);
	// 824D7CA0: EEF7AABA  fmadds f23, f23, f10, f21
	ctx.f[23].f64 = (((ctx.f[23].f64 * ctx.f[10].f64 + ctx.f[21].f64) as f32) as f64);
	// 824D7CA4: C2A30020  lfs f21, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[21].f64 = (tmp.f32 as f64);
	// 824D7CA8: EC7C183A  fmadds f3, f28, f0, f3
	ctx.f[3].f64 = (((ctx.f[28].f64 * ctx.f[0].f64 + ctx.f[3].f64) as f32) as f64);
	// 824D7CAC: D2E30024  stfs f23, 0x24(r3)
	tmp.f32 = (ctx.f[23].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 824D7CB0: EF9C02F2  fmuls f28, f28, f11
	ctx.f[28].f64 = (((ctx.f[28].f64 * ctx.f[11].f64) as f32) as f64);
	// 824D7CB4: EC6302F2  fmuls f3, f3, f11
	ctx.f[3].f64 = (((ctx.f[3].f64 * ctx.f[11].f64) as f32) as f64);
	// 824D7CB8: EF89E37A  fmadds f28, f9, f13, f28
	ctx.f[28].f64 = (((ctx.f[9].f64 * ctx.f[13].f64 + ctx.f[28].f64) as f32) as f64);
	// 824D7CBC: ED3FA9FA  fmadds f9, f31, f7, f21
	ctx.f[9].f64 = (((ctx.f[31].f64 * ctx.f[7].f64 + ctx.f[21].f64) as f32) as f64);
	// 824D7CC0: ED7DB1FA  fmadds f11, f29, f7, f22
	ctx.f[11].f64 = (((ctx.f[29].f64 * ctx.f[7].f64 + ctx.f[22].f64) as f32) as f64);
	// 824D7CC4: EFFE0072  fmuls f31, f30, f1
	ctx.f[31].f64 = (((ctx.f[30].f64 * ctx.f[1].f64) as f32) as f64);
	// 824D7CC8: EC421B7A  fmadds f2, f2, f13, f3
	ctx.f[2].f64 = (((ctx.f[2].f64 * ctx.f[13].f64 + ctx.f[3].f64) as f32) as f64);
	// 824D7CCC: EC7C9ABA  fmadds f3, f28, f10, f19
	ctx.f[3].f64 = (((ctx.f[28].f64 * ctx.f[10].f64 + ctx.f[19].f64) as f32) as f64);
	// 824D7CD0: EDB0A1FA  fmadds f13, f16, f7, f20
	ctx.f[13].f64 = (((ctx.f[16].f64 * ctx.f[7].f64 + ctx.f[20].f64) as f32) as f64);
	// 824D7CD4: ED4292BA  fmadds f10, f2, f10, f18
	ctx.f[10].f64 = (((ctx.f[2].f64 * ctx.f[10].f64 + ctx.f[18].f64) as f32) as f64);
	// 824D7CD8: EC5E01B2  fmuls f2, f30, f6
	ctx.f[2].f64 = (((ctx.f[30].f64 * ctx.f[6].f64) as f32) as f64);
	// 824D7CDC: EC44107A  fmadds f2, f4, f1, f2
	ctx.f[2].f64 = (((ctx.f[4].f64 * ctx.f[1].f64 + ctx.f[2].f64) as f32) as f64);
	// 824D7CE0: D1630014  stfs f11, 0x14(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824D7CE4: EC84F9BA  fmadds f4, f4, f6, f31
	ctx.f[4].f64 = (((ctx.f[4].f64 * ctx.f[6].f64 + ctx.f[31].f64) as f32) as f64);
	// 824D7CE8: D1230020  stfs f9, 0x20(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824D7CEC: D1A30030  stfs f13, 0x30(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 824D7CF0: D063001C  stfs f3, 0x1c(r3)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 824D7CF4: D1430028  stfs f10, 0x28(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 824D7CF8: EC5B117A  fmadds f2, f27, f5, f2
	ctx.f[2].f64 = (((ctx.f[27].f64 * ctx.f[5].f64 + ctx.f[2].f64) as f32) as f64);
	// 824D7CFC: EC88217A  fmadds f4, f8, f5, f4
	ctx.f[4].f64 = (((ctx.f[8].f64 * ctx.f[5].f64 + ctx.f[4].f64) as f32) as f64);
	// 824D7D00: ED02402A  fadds f8, f2, f8
	ctx.f[8].f64 = ((ctx.f[2].f64 + ctx.f[8].f64) as f32) as f64;
	// 824D7D04: EC84D82A  fadds f4, f4, f27
	ctx.f[4].f64 = ((ctx.f[4].f64 + ctx.f[27].f64) as f32) as f64;
	// 824D7D08: EC080032  fmuls f0, f8, f0
	ctx.f[0].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D7D0C: EC04033A  fmadds f0, f4, f12, f0
	ctx.f[0].f64 = (((ctx.f[4].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 824D7D10: EC0089FA  fmadds f0, f0, f7, f17
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[7].f64 + ctx.f[17].f64) as f32) as f64);
	// 824D7D14: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824D7D18: 409AFE80  bne cr6, 0x824d7b98
	if !ctx.cr[6].eq {
	pc = 0x824D7B98; continue 'dispatch;
	}
	// 824D7D1C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D7D20: C18BBFFC  lfs f12, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7D24: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D7D28: ED180332  fmuls f8, f24, f12
	ctx.f[8].f64 = (((ctx.f[24].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7D2C: D103000C  stfs f8, 0xc(r3)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824D7D30: C18B2120  lfs f12, 0x2120(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8480 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7D34: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D7D38: ECFA0332  fmuls f7, f26, f12
	ctx.f[7].f64 = (((ctx.f[26].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7D3C: D0E30010  stfs f7, 0x10(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824D7D40: C18B20A8  lfs f12, 0x20a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8360 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7D44: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824D7D48: ED190332  fmuls f8, f25, f12
	ctx.f[8].f64 = (((ctx.f[25].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7D4C: D1030018  stfs f8, 0x18(r3)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824D7D50: C18B2954  lfs f12, 0x2954(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10580 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7D54: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D7D58: ECF70332  fmuls f7, f23, f12
	ctx.f[7].f64 = (((ctx.f[23].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7D5C: D0E30024  stfs f7, 0x24(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 824D7D60: C18B280C  lfs f12, 0x280c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10252 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7D64: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D7D68: ED6B0332  fmuls f11, f11, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7D6C: D1630014  stfs f11, 0x14(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824D7D70: C18B294C  lfs f12, 0x294c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10572 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7D74: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D7D78: ED290332  fmuls f9, f9, f12
	ctx.f[9].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7D7C: D1230020  stfs f9, 0x20(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824D7D80: C18B2520  lfs f12, 0x2520(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9504 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7D84: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D7D88: ED8D0332  fmuls f12, f13, f12
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7D8C: D1830030  stfs f12, 0x30(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 824D7D90: C1AB2450  lfs f13, 0x2450(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9296 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D7D94: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D7D98: ED630372  fmuls f11, f3, f13
	ctx.f[11].f64 = (((ctx.f[3].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7D9C: D163001C  stfs f11, 0x1c(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 824D7DA0: C1AB2198  lfs f13, 0x2198(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8600 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D7DA4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D7DA8: ED8A0372  fmuls f12, f10, f13
	ctx.f[12].f64 = (((ctx.f[10].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7DAC: D1830028  stfs f12, 0x28(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 824D7DB0: C1AB22C8  lfs f13, 0x22c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8904 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D7DB4: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7DB8: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824D7DBC: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 824D7DC0: 4805E241  bl 0x82536000
	ctx.lr = 0x824D7DC4;
	sub_82535FFC(ctx, base);
	// 824D7DC4: 4805D348  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D7DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824D7DC8 size=584
    let mut pc: u32 = 0x824D7DC8;
    'dispatch: loop {
        match pc {
            0x824D7DC8 => {
    //   block [0x824D7DC8..0x824D8010)
	// 824D7DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D7DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D7DD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D7DD4: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 824D7DD8: 4805E1F5  bl 0x82535fcc
	ctx.lr = 0x824D7DDC;
	sub_82535FB0(ctx, base);
	// 824D7DDC: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D7DE0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 824D7DE4: 4BFFFD35  bl 0x824d7b18
	ctx.lr = 0x824D7DE8;
	sub_824D7B18(ctx, base);
	// 824D7DE8: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D7DEC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824D7DF0: C1BF0000  lfs f13, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D7DF4: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D7DF8: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7DFC: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D7E00: C1840004  lfs f12, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7E04: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D7E08: C17F0004  lfs f11, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824D7E0C: C0A40008  lfs f5, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 824D7E10: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824D7E14: C03F0008  lfs f1, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824D7E18: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D7E1C: 7D29FC2E  lfsx f9, r9, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 824D7E20: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 824D7E24: C1430010  lfs f10, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 824D7E28: C1030014  lfs f8, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 824D7E2C: C3E3000C  lfs f31, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824D7E30: C0E30018  lfs f7, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 824D7E34: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 824D7E38: C0091850  lfs f0, 0x1850(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D7E3C: EC004824  fdivs f0, f0, f9
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[9].f64) as f32) as f64;
	// 824D7E40: C0C30020  lfs f6, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 824D7E44: C0630024  lfs f3, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 824D7E48: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 824D7E4C: C083001C  lfs f4, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 824D7E50: C0430030  lfs f2, 0x30(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 824D7E54: C1292068  lfs f9, 0x2068(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8296 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 824D7E58: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 824D7E5C: EDAC6AFC  fnmsubs f13, f12, f11, f13
	ctx.f[13].f64 = -(((ctx.f[12].f64 * ctx.f[11].f64 - ctx.f[13].f64) as f32) as f64);
	// 824D7E60: ED870032  fmuls f12, f7, f0
	ctx.f[12].f64 = (((ctx.f[7].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D7E64: EFC60032  fmuls f30, f6, f0
	ctx.f[30].f64 = (((ctx.f[6].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D7E68: EFA30032  fmuls f29, f3, f0
	ctx.f[29].f64 = (((ctx.f[3].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D7E6C: EF820032  fmuls f28, f2, f0
	ctx.f[28].f64 = (((ctx.f[2].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D7E70: ED65687C  fnmsubs f11, f5, f1, f13
	ctx.f[11].f64 = -(((ctx.f[5].f64 * ctx.f[1].f64 - ctx.f[13].f64) as f32) as f64);
	// 824D7E74: EDAA0032  fmuls f13, f10, f0
	ctx.f[13].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D7E78: D1A30034  stfs f13, 0x34(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 824D7E7C: EDA80032  fmuls f13, f8, f0
	ctx.f[13].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D7E80: D1A30038  stfs f13, 0x38(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 824D7E84: 7DAAFC2E  lfsx f13, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D7E88: EC200032  fmuls f1, f0, f0
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D7E8C: EDA80372  fmuls f13, f8, f13
	ctx.f[13].f64 = (((ctx.f[8].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7E90: 7CABFC2E  lfsx f5, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 824D7E94: D3C30044  stfs f30, 0x44(r3)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 824D7E98: D1830040  stfs f12, 0x40(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 824D7E9C: EFFF02F2  fmuls f31, f31, f11
	ctx.f[31].f64 = (((ctx.f[31].f64 * ctx.f[11].f64) as f32) as f64);
	// 824D7EA0: EFC10032  fmuls f30, f1, f0
	ctx.f[30].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D7EA4: EDA56ABA  fmadds f13, f5, f10, f13
	ctx.f[13].f64 = (((ctx.f[5].f64 * ctx.f[10].f64 + ctx.f[13].f64) as f32) as f64);
	// 824D7EA8: EF1E0032  fmuls f24, f30, f0
	ctx.f[24].f64 = (((ctx.f[30].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D7EAC: EDADF82A  fadds f13, f13, f31
	ctx.f[13].f64 = ((ctx.f[13].f64 + ctx.f[31].f64) as f32) as f64;
	// 824D7EB0: EDAD0072  fmuls f13, f13, f1
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[1].f64) as f32) as f64);
	// 824D7EB4: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 824D7EB8: D1A3003C  stfs f13, 0x3c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 824D7EBC: 7D8BFC2E  lfsx f12, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7EC0: 7DAAFC2E  lfsx f13, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D7EC4: ECAC02B2  fmuls f5, f12, f10
	ctx.f[5].f64 = (((ctx.f[12].f64 * ctx.f[10].f64) as f32) as f64);
	// 824D7EC8: D3A3004C  stfs f29, 0x4c(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 824D7ECC: EFA40332  fmuls f29, f4, f12
	ctx.f[29].f64 = (((ctx.f[4].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7ED0: D3830050  stfs f28, 0x50(r3)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 824D7ED4: EF870332  fmuls f28, f7, f12
	ctx.f[28].f64 = (((ctx.f[7].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7ED8: ECAD2A3A  fmadds f5, f13, f8, f5
	ctx.f[5].f64 = (((ctx.f[13].f64 * ctx.f[8].f64 + ctx.f[5].f64) as f32) as f64);
	// 824D7EDC: EFBD0372  fmuls f29, f29, f13
	ctx.f[29].f64 = (((ctx.f[29].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7EE0: ECA5FA7A  fmadds f5, f5, f9, f31
	ctx.f[5].f64 = (((ctx.f[5].f64 * ctx.f[9].f64 + ctx.f[31].f64) as f32) as f64);
	// 824D7EE4: EFBD0272  fmuls f29, f29, f9
	ctx.f[29].f64 = (((ctx.f[29].f64 * ctx.f[9].f64) as f32) as f64);
	// 824D7EE8: ECA5EAFA  fmadds f5, f5, f11, f29
	ctx.f[5].f64 = (((ctx.f[5].f64 * ctx.f[11].f64 + ctx.f[29].f64) as f32) as f64);
	// 824D7EEC: EFA60372  fmuls f29, f6, f13
	ctx.f[29].f64 = (((ctx.f[6].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7EF0: EDBD2B7A  fmadds f13, f29, f13, f5
	ctx.f[13].f64 = (((ctx.f[29].f64 * ctx.f[13].f64 + ctx.f[5].f64) as f32) as f64);
	// 824D7EF4: C3A3002C  lfs f29, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 824D7EF8: C0A92940  lfs f5, 0x2940(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(10560 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 824D7EFC: EDBC6B3A  fmadds f13, f28, f12, f13
	ctx.f[13].f64 = (((ctx.f[28].f64 * ctx.f[12].f64 + ctx.f[13].f64) as f32) as f64);
	// 824D7F00: C3830028  lfs f28, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 824D7F04: EDAD07B2  fmuls f13, f13, f30
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[30].f64) as f32) as f64);
	// 824D7F08: D1A30048  stfs f13, 0x48(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 824D7F0C: 7DABFC2E  lfsx f13, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D7F10: EF4D02B2  fmuls f26, f13, f10
	ctx.f[26].f64 = (((ctx.f[13].f64 * ctx.f[10].f64) as f32) as f64);
	// 824D7F14: 7D8AFC2E  lfsx f12, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D7F18: EF640372  fmuls f27, f4, f13
	ctx.f[27].f64 = (((ctx.f[4].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7F1C: EF230372  fmuls f25, f3, f13
	ctx.f[25].f64 = (((ctx.f[3].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7F20: ED4A02F2  fmuls f10, f10, f11
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[11].f64) as f32) as f64);
	// 824D7F24: EC1C0032  fmuls f0, f28, f0
	ctx.f[0].f64 = (((ctx.f[28].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D7F28: D0030058  stfs f0, 0x58(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 824D7F2C: EC0CD23A  fmadds f0, f12, f8, f26
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[8].f64 + ctx.f[26].f64) as f32) as f64);
	// 824D7F30: EF7B0332  fmuls f27, f27, f12
	ctx.f[27].f64 = (((ctx.f[27].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7F34: EEFD0372  fmuls f23, f29, f13
	ctx.f[23].f64 = (((ctx.f[29].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7F38: EEC70372  fmuls f22, f7, f13
	ctx.f[22].f64 = (((ctx.f[7].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7F3C: EEBC0332  fmuls f21, f28, f12
	ctx.f[21].f64 = (((ctx.f[28].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7F40: EF390372  fmuls f25, f25, f13
	ctx.f[25].f64 = (((ctx.f[25].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7F44: EC00F97A  fmadds f0, f0, f5, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[5].f64 + ctx.f[31].f64) as f32) as f64);
	// 824D7F48: EFE60332  fmuls f31, f6, f12
	ctx.f[31].f64 = (((ctx.f[6].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7F4C: EF570332  fmuls f26, f23, f12
	ctx.f[26].f64 = (((ctx.f[23].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7F50: EEE20332  fmuls f23, f2, f12
	ctx.f[23].f64 = (((ctx.f[2].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7F54: ED150372  fmuls f8, f21, f13
	ctx.f[8].f64 = (((ctx.f[21].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7F58: EC0002F2  fmuls f0, f0, f11
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[11].f64) as f32) as f64);
	// 824D7F5C: EFFF0332  fmuls f31, f31, f12
	ctx.f[31].f64 = (((ctx.f[31].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7F60: EF5A0332  fmuls f26, f26, f12
	ctx.f[26].f64 = (((ctx.f[26].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7F64: EEF70332  fmuls f23, f23, f12
	ctx.f[23].f64 = (((ctx.f[23].f64 * ctx.f[12].f64) as f32) as f64);
	// 824D7F68: ED080372  fmuls f8, f8, f13
	ctx.f[8].f64 = (((ctx.f[8].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7F6C: EFFBFA7A  fmadds f31, f27, f9, f31
	ctx.f[31].f64 = (((ctx.f[27].f64 * ctx.f[9].f64 + ctx.f[31].f64) as f32) as f64);
	// 824D7F70: EF7A0172  fmuls f27, f26, f5
	ctx.f[27].f64 = (((ctx.f[26].f64 * ctx.f[5].f64) as f32) as f64);
	// 824D7F74: EFF6FB7A  fmadds f31, f22, f13, f31
	ctx.f[31].f64 = (((ctx.f[22].f64 * ctx.f[13].f64 + ctx.f[31].f64) as f32) as f64);
	// 824D7F78: EC1F017A  fmadds f0, f31, f5, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[5].f64 + ctx.f[0].f64) as f32) as f64);
	// 824D7F7C: EC00DAFA  fmadds f0, f0, f11, f27
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[11].f64 + ctx.f[27].f64) as f32) as f64);
	// 824D7F80: EC08017A  fmadds f0, f8, f5, f0
	ctx.f[0].f64 = (((ctx.f[8].f64 * ctx.f[5].f64 + ctx.f[0].f64) as f32) as f64);
	// 824D7F84: EC17033A  fmadds f0, f23, f12, f0
	ctx.f[0].f64 = (((ctx.f[23].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 824D7F88: EC19037A  fmadds f0, f25, f13, f0
	ctx.f[0].f64 = (((ctx.f[25].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 824D7F8C: EC000632  fmuls f0, f0, f24
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[24].f64) as f32) as f64);
	// 824D7F90: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 824D7F94: D0030054  stfs f0, 0x54(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 824D7F98: 7C0AFC2E  lfsx f0, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D7F9C: EC020032  fmuls f0, f2, f0
	ctx.f[0].f64 = (((ctx.f[2].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D7FA0: 7DABFC2E  lfsx f13, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D7FA4: EC0D077A  fmadds f0, f13, f29, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[29].f64 + ctx.f[0].f64) as f32) as f64);
	// 824D7FA8: EC0602FA  fmadds f0, f6, f11, f0
	ctx.f[0].f64 = (((ctx.f[6].f64 * ctx.f[11].f64 + ctx.f[0].f64) as f32) as f64);
	// 824D7FAC: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 824D7FB0: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 824D7FB4: D003005C  stfs f0, 0x5c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 824D7FB8: 7DABFC2E  lfsx f13, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D7FBC: ED870372  fmuls f12, f7, f13
	ctx.f[12].f64 = (((ctx.f[7].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7FC0: 7C0AFC2E  lfsx f0, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D7FC4: ED1C0372  fmuls f8, f28, f13
	ctx.f[8].f64 = (((ctx.f[28].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7FC8: ED84603A  fmadds f12, f4, f0, f12
	ctx.f[12].f64 = (((ctx.f[4].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 824D7FCC: ED080032  fmuls f8, f8, f0
	ctx.f[8].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D7FD0: ED8C527A  fmadds f12, f12, f9, f10
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[9].f64 + ctx.f[10].f64) as f32) as f64);
	// 824D7FD4: ED480272  fmuls f10, f8, f9
	ctx.f[10].f64 = (((ctx.f[8].f64 * ctx.f[9].f64) as f32) as f64);
	// 824D7FD8: ED8C52FA  fmadds f12, f12, f11, f10
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64 + ctx.f[10].f64) as f32) as f64);
	// 824D7FDC: ED7D0032  fmuls f11, f29, f0
	ctx.f[11].f64 = (((ctx.f[29].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D7FE0: ED430372  fmuls f10, f3, f13
	ctx.f[10].f64 = (((ctx.f[3].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D7FE4: EC0B603A  fmadds f0, f11, f0, f12
	ctx.f[0].f64 = (((ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 824D7FE8: EC0A037A  fmadds f0, f10, f13, f0
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 824D7FEC: EC0007B2  fmuls f0, f0, f30
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64) as f32) as f64);
	// 824D7FF0: D0030060  stfs f0, 0x60(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 824D7FF4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 824D7FF8: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 824D7FFC: 4805E01D  bl 0x82536018
	ctx.lr = 0x824D8000;
	sub_82535FFC(ctx, base);
	// 824D8000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824D8004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824D8008: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824D800C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D8010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824D8010 size=340
    let mut pc: u32 = 0x824D8010;
    'dispatch: loop {
        match pc {
            0x824D8010 => {
    //   block [0x824D8010..0x824D8164)
	// 824D8010: C1A30064  lfs f13, 0x64(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D8014: C0030068  lfs f0, 0x68(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D8018: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 824D801C: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824D8020: C1A30064  lfs f13, 0x64(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D8024: C003006C  lfs f0, 0x6c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D8028: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 824D802C: D0060004  stfs f0, 4(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824D8030: C1A30064  lfs f13, 0x64(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D8034: C0030070  lfs f0, 0x70(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D8038: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 824D803C: D0060008  stfs f0, 8(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824D8040: C1A3007C  lfs f13, 0x7c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D8044: C0030078  lfs f0, 0x78(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D8048: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 824D804C: EC0000B2  fmuls f0, f0, f2
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 824D8050: D0070000  stfs f0, 0(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824D8054: C1A3007C  lfs f13, 0x7c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D8058: C0030074  lfs f0, 0x74(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D805C: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 824D8060: EC0000B2  fmuls f0, f0, f2
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 824D8064: D0070014  stfs f0, 0x14(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824D8068: C1A30078  lfs f13, 0x78(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D806C: C0030074  lfs f0, 0x74(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D8070: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 824D8074: EC0000B2  fmuls f0, f0, f2
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 824D8078: D0070028  stfs f0, 0x28(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 824D807C: C0030080  lfs f0, 0x80(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D8080: EC0000B2  fmuls f0, f0, f2
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 824D8084: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 824D8088: D0070004  stfs f0, 4(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824D808C: D0070010  stfs f0, 0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824D8090: C0030084  lfs f0, 0x84(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D8094: EC0000B2  fmuls f0, f0, f2
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 824D8098: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 824D809C: D0070018  stfs f0, 0x18(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824D80A0: D0070024  stfs f0, 0x24(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 824D80A4: C0030088  lfs f0, 0x88(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D80A8: EC0000B2  fmuls f0, f0, f2
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 824D80AC: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 824D80B0: D0070020  stfs f0, 0x20(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824D80B4: D0070008  stfs f0, 8(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824D80B8: C0060008  lfs f0, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D80BC: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 824D80C0: C1A60004  lfs f13, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D80C4: C1870000  lfs f12, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D80C8: EC0D037A  fmadds f0, f13, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 824D80CC: EC00607C  fnmsubs f0, f0, f1, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[1].f64 - ctx.f[12].f64) as f32) as f64);
	// 824D80D0: D0070000  stfs f0, 0(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824D80D4: C1A60008  lfs f13, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D80D8: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D80DC: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D80E0: C1870014  lfs f12, 0x14(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D80E4: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 824D80E8: EC00607C  fnmsubs f0, f0, f1, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[1].f64 - ctx.f[12].f64) as f32) as f64);
	// 824D80EC: D0070014  stfs f0, 0x14(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824D80F0: C1A60004  lfs f13, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D80F4: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D80F8: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D80FC: C1870028  lfs f12, 0x28(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824D8100: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 824D8104: EC00607C  fnmsubs f0, f0, f1, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[1].f64 - ctx.f[12].f64) as f32) as f64);
	// 824D8108: D0070028  stfs f0, 0x28(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 824D810C: C1A60004  lfs f13, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D8110: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D8114: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D8118: C1A70004  lfs f13, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D811C: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 824D8120: D0070004  stfs f0, 4(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824D8124: D0070010  stfs f0, 0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824D8128: C1A60004  lfs f13, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D812C: C0060008  lfs f0, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D8130: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D8134: C1A70018  lfs f13, 0x18(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D8138: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 824D813C: D0070018  stfs f0, 0x18(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824D8140: D0070024  stfs f0, 0x24(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 824D8144: C1A60000  lfs f13, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D8148: C0060008  lfs f0, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D814C: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 824D8150: C1A70020  lfs f13, 0x20(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D8154: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 824D8158: D0070020  stfs f0, 0x20(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824D815C: D0070008  stfs f0, 8(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824D8160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D8168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D8168 size=116
    let mut pc: u32 = 0x824D8168;
    'dispatch: loop {
        match pc {
            0x824D8168 => {
    //   block [0x824D8168..0x824D81DC)
	// 824D8168: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D81DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824D81DC size=68
    let mut pc: u32 = 0x824D81DC;
    'dispatch: loop {
        match pc {
            0x824D81DC => {
    //   block [0x824D81DC..0x824D8220)
	// 824D81DC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D81E0: 114D004A  vsubfp v10, v13, v0
	ctx.fpscr.enable_flush_mode_unconditional();
	for i in 0..4 {
		ctx.v[10].f32[i] = ctx.v[13].f32[i] - ctx.v[0].f32[i];
	}
	// 824D81E4: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 824D81E8: C00BBFFC  lfs f0, -0x4004(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D81EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 824D81F0: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D8220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D8220 size=760
    let mut pc: u32 = 0x824D8220;
    'dispatch: loop {
        match pc {
            0x824D8220 => {
    //   block [0x824D8220..0x824D8518)
	// 824D8220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D8224: 4805CE81  bl 0x825350a4
	ctx.lr = 0x824D8228;
	sub_82535080(ctx, base);
	// 824D8228: 3981FFB0  addi r12, r1, -0x50
	ctx.r[12].s64 = ctx.r[1].s64 + -80;
	// 824D822C: 4805DDB1  bl 0x82535fdc
	ctx.lr = 0x824D8230;
	sub_82535FB0(ctx, base);
	// 824D8230: 3980FF60  li r12, -0xa0
	ctx.r[12].s64 = -160;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D8518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824D8518 size=80
    let mut pc: u32 = 0x824D8518;
    'dispatch: loop {
        match pc {
            0x824D8518 => {
    //   block [0x824D8518..0x824D8568)
	// 824D8518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D851C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D8520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D8524: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824D8528: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 824D852C: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 824D8530: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 824D8534: C00B0004  lfs f0, 4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D8538: EC010024  fdivs f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 / ctx.f[0].f64) as f32) as f64;
	// 824D853C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D8568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824D8568 size=88
    let mut pc: u32 = 0x824D8568;
    'dispatch: loop {
        match pc {
            0x824D8568 => {
    //   block [0x824D8568..0x824D85C0)
	// 824D8568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D856C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D8570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D8574: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824D8578: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 824D857C: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 824D8580: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 824D8584: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D8588: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 824D858C: C1AB0004  lfs f13, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D8590: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824D8594: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 824D8598: EDA06824  fdivs f13, f0, f13
	ctx.f[13].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 824D859C: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D85C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824D85C0 size=424
    let mut pc: u32 = 0x824D85C0;
    'dispatch: loop {
        match pc {
            0x824D85C0 => {
    //   block [0x824D85C0..0x824D8768)
	// 824D85C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D85C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D85C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D85CC: DBA1FFD8  stfd f29, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[29].u64 ) };
	// 824D85D0: DBC1FFE0  stfd f30, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 824D85D4: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 824D85D8: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D85DC: FFA01090  fmr f29, f2
	ctx.f[29].f64 = ctx.f[2].f64;
	// 824D85E0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D85E4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 824D85E8: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D85EC: FF1D0000  fcmpu cr6, f29, f0
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[0].f64);
	// 824D85F0: 4199000C  bgt cr6, 0x824d85fc
	if ctx.cr[6].gt {
	pc = 0x824D85FC; continue 'dispatch;
	}
	// 824D85F4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824D85F8: 48000150  b 0x824d8748
	pc = 0x824D8748; continue 'dispatch;
	// 824D85FC: FF011800  fcmpu cr6, f1, f3
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[3].f64);
	// 824D8600: 4099FFF4  ble cr6, 0x824d85f4
	if !ctx.cr[6].gt {
	pc = 0x824D85F4; continue 'dispatch;
	}
	// 824D8604: FF030000  fcmpu cr6, f3, f0
	ctx.cr[6].compare_f64(ctx.f[3].f64, ctx.f[0].f64);
	// 824D8608: 4099FFEC  ble cr6, 0x824d85f4
	if !ctx.cr[6].gt {
	pc = 0x824D85F4; continue 'dispatch;
	}
	// 824D860C: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
	// 824D8610: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 824D8614: D00100A0  stfs f0, 0xa0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 824D8618: EDA10072  fmuls f13, f1, f1
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[1].f64) as f32) as f64);
	// 824D861C: D00100A4  stfs f0, 0xa4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 824D8620: ED411828  fsubs f10, f1, f3
	ctx.f[10].f64 = (((ctx.f[1].f64 - ctx.f[3].f64) as f32) as f64);
	// 824D8624: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D8768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824D8768 size=516
    let mut pc: u32 = 0x824D8768;
    'dispatch: loop {
        match pc {
            0x824D8768 => {
    //   block [0x824D8768..0x824D896C)
	// 824D8768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D876C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D8770: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D8774: DBA1FFD8  stfd f29, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[29].u64 ) };
	// 824D8778: DBC1FFE0  stfd f30, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 824D877C: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 824D8780: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D8784: FFA00890  fmr f29, f1
	ctx.f[29].f64 = ctx.f[1].f64;
	// 824D8788: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D878C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 824D8790: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D8794: FF1D0000  fcmpu cr6, f29, f0
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[0].f64);
	// 824D8798: 409901B0  ble cr6, 0x824d8948
	if !ctx.cr[6].gt {
	pc = 0x824D8948; continue 'dispatch;
	}
	// 824D879C: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D87A0: FF0D1000  fcmpu cr6, f13, f2
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[2].f64);
	// 824D87A4: 409901A4  ble cr6, 0x824d8948
	if !ctx.cr[6].gt {
	pc = 0x824D8948; continue 'dispatch;
	}
	// 824D87A8: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D87AC: FF0D1000  fcmpu cr6, f13, f2
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[2].f64);
	// 824D87B0: 40990198  ble cr6, 0x824d8948
	if !ctx.cr[6].gt {
	pc = 0x824D8948; continue 'dispatch;
	}
	// 824D87B4: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D87B8: FF0D1000  fcmpu cr6, f13, f2
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[2].f64);
	// 824D87BC: 4099018C  ble cr6, 0x824d8948
	if !ctx.cr[6].gt {
	pc = 0x824D8948; continue 'dispatch;
	}
	// 824D87C0: FF020000  fcmpu cr6, f2, f0
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[0].f64);
	// 824D87C4: 40990184  ble cr6, 0x824d8948
	if !ctx.cr[6].gt {
	pc = 0x824D8948; continue 'dispatch;
	}
	// 824D87C8: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 824D87CC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824D87D0: D00100B0  stfs f0, 0xb0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), tmp.u32 ) };
	// 824D87D4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 824D87D8: D00100B4  stfs f0, 0xb4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 824D87DC: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824D87E0: C1830004  lfs f12, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D8970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824D8970 size=608
    let mut pc: u32 = 0x824D8970;
    'dispatch: loop {
        match pc {
            0x824D8970 => {
    //   block [0x824D8970..0x824D8BD0)
	// 824D8970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D8974: 4805C739  bl 0x825350ac
	ctx.lr = 0x824D8978;
	sub_82535080(ctx, base);
	// 824D8978: DBA1FFA8  stfd f29, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[29].u64 ) };
	// 824D897C: DBC1FFB0  stfd f30, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[30].u64 ) };
	// 824D8980: DBE1FFB8  stfd f31, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 824D8984: 3980FF90  li r12, -0x70
	ctx.r[12].s64 = -112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D8BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824D8BD0 size=1104
    let mut pc: u32 = 0x824D8BD0;
    'dispatch: loop {
        match pc {
            0x824D8BD0 => {
    //   block [0x824D8BD0..0x824D9020)
	// 824D8BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D8BD4: 4805C4AD  bl 0x82535080
	ctx.lr = 0x824D8BD8;
	sub_82535080(ctx, base);
	// 824D8BD8: DBA1FF50  stfd f29, -0xb0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), ctx.f[29].u64 ) };
	// 824D8BDC: DBC1FF58  stfd f30, -0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 824D8BE0: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 824D8BE4: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D8BE8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824D8BEC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824D8BF0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 824D8BF4: 3B230088  addi r25, r3, 0x88
	ctx.r[25].s64 = ctx.r[3].s64 + 136;
	// 824D8BF8: 3B030084  addi r24, r3, 0x84
	ctx.r[24].s64 = ctx.r[3].s64 + 132;
	// 824D8BFC: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 824D8C00: 3AE30080  addi r23, r3, 0x80
	ctx.r[23].s64 = ctx.r[3].s64 + 128;
	// 824D8C04: C3AA1FF8  lfs f29, 0x1ff8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 824D8C08: 3AC3007C  addi r22, r3, 0x7c
	ctx.r[22].s64 = ctx.r[3].s64 + 124;
	// 824D8C0C: D3A30088  stfs f29, 0x88(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 824D8C10: 3AA30078  addi r21, r3, 0x78
	ctx.r[21].s64 = ctx.r[3].s64 + 120;
	// 824D8C14: D3A30084  stfs f29, 0x84(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 824D8C18: 3A830074  addi r20, r3, 0x74
	ctx.r[20].s64 = ctx.r[3].s64 + 116;
	// 824D8C1C: D3A30080  stfs f29, 0x80(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 824D8C20: 3A630070  addi r19, r3, 0x70
	ctx.r[19].s64 = ctx.r[3].s64 + 112;
	// 824D8C24: D3A3007C  stfs f29, 0x7c(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 824D8C28: 3A43006C  addi r18, r3, 0x6c
	ctx.r[18].s64 = ctx.r[3].s64 + 108;
	// 824D8C2C: D3A30078  stfs f29, 0x78(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 824D8C30: 3A230068  addi r17, r3, 0x68
	ctx.r[17].s64 = ctx.r[3].s64 + 104;
	// 824D8C34: D3A30074  stfs f29, 0x74(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 824D8C38: 3BC30064  addi r30, r3, 0x64
	ctx.r[30].s64 = ctx.r[3].s64 + 100;
	// 824D8C3C: D3A30070  stfs f29, 0x70(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 824D8C40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824D8C44: D3A3006C  stfs f29, 0x6c(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 824D8C48: D3A30068  stfs f29, 0x68(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 824D8C4C: D3A30064  stfs f29, 0x64(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 824D8C50: 40990340  ble cr6, 0x824d8f90
	if !ctx.cr[6].gt {
	pc = 0x824D8F90; continue 'dispatch;
	}
	// 824D8C54: 7D705B78  mr r16, r11
	ctx.r[16].u64 = ctx.r[11].u64;
	// 824D8C58: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824D8C5C: 39E00000  li r15, 0
	ctx.r[15].s64 = 0;
	// 824D8C60: 3D405555  lis r10, 0x5555
	ctx.r[10].s64 = 1431633920;
	// 824D8C64: 3B5D000C  addi r26, r29, 0xc
	ctx.r[26].s64 = ctx.r[29].s64 + 12;
	// 824D8C68: 7DFC7B78  mr r28, r15
	ctx.r[28].u64 = ctx.r[15].u64;
	// 824D8C6C: C3CB1850  lfs f30, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 824D8C70: 615F5556  ori r31, r10, 0x5556
	ctx.r[31].u64 = ctx.r[10].u64 | 21846;
	// 824D8C74: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D9020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824D9020 size=564
    let mut pc: u32 = 0x824D9020;
    'dispatch: loop {
        match pc {
            0x824D9020 => {
    //   block [0x824D9020..0x824D9254)
	// 824D9020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D9024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D9028: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824D902C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D9030: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 824D9034: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 824D9038: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D903C: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 824D9040: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D9044: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824D9048: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 824D904C: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824D9050: FF1EF800  fcmpu cr6, f30, f31
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 824D9054: 4199000C  bgt cr6, 0x824d9060
	if ctx.cr[6].gt {
	pc = 0x824D9060; continue 'dispatch;
	}
	// 824D9058: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824D905C: C3CB1850  lfs f30, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 824D9060: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 824D9064: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D9068: 38A100B0  addi r5, r1, 0xb0
	ctx.r[5].s64 = ctx.r[1].s64 + 176;
	// 824D906C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D9070: 4BFFF0F9  bl 0x824d8168
	ctx.lr = 0x824D9074;
	sub_824D8168(ctx, base);
	// 824D9074: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824D9078: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 824D907C: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 824D9080: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D9258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824D9258 size=496
    let mut pc: u32 = 0x824D9258;
    'dispatch: loop {
        match pc {
            0x824D9258 => {
    //   block [0x824D9258..0x824D9448)
	// 824D9258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D925C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824D9260: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824D9264: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824D9268: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 824D926C: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 824D9270: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D9274: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 824D9278: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D927C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824D9280: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 824D9284: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824D9288: FF1EF800  fcmpu cr6, f30, f31
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 824D928C: 4199000C  bgt cr6, 0x824d9298
	if ctx.cr[6].gt {
	pc = 0x824D9298; continue 'dispatch;
	}
	// 824D9290: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824D9294: 48000194  b 0x824d9428
	pc = 0x824D9428; continue 'dispatch;
	// 824D9298: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 824D929C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824D92A0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 824D92A4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D92A8: 4BFFEEC1  bl 0x824d8168
	ctx.lr = 0x824D92AC;
	sub_824D8168(ctx, base);
	// 824D92AC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824D92B0: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 824D92B4: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 824D92B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D9448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824D9448 size=1664
    let mut pc: u32 = 0x824D9448;
    'dispatch: loop {
        match pc {
            0x824D9448 => {
    //   block [0x824D9448..0x824D9AC8)
	// 824D9448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D944C: 4805BC6D  bl 0x825350b8
	ctx.lr = 0x824D9450;
	sub_82535080(ctx, base);
	// 824D9450: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 824D9454: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D9458: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 824D945C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 824D9460: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824D9464: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824D9468: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 824D946C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824D9470: 41990014  bgt cr6, 0x824d9484
	if ctx.cr[6].gt {
	pc = 0x824D9484; continue 'dispatch;
	}
	// 824D9474: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824D9478: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824D947C: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 824D9480: 4805BC88  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 824D9484: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824D9488: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824D948C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824D9490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 824D9494: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 824D9498: 90810060  stw r4, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 824D949C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 824D94A0: 40990018  ble cr6, 0x824d94b8
	if !ctx.cr[6].gt {
	pc = 0x824D94B8; continue 'dispatch;
	}
	// 824D94A4: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 824D94A8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824D94AC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 824D94B0: 4BF94E19  bl 0x8246e2c8
	ctx.lr = 0x824D94B4;
	sub_8246E2C8(ctx, base);
	// 824D94B4: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D94B8: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 824D94BC: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 824D94C0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824D94C4: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 824D94C8: 419800F4  blt cr6, 0x824d95bc
	if ctx.cr[6].lt {
	pc = 0x824D95BC; continue 'dispatch;
	}
	// 824D94CC: 393CFFFC  addi r9, r28, -4
	ctx.r[9].s64 = ctx.r[28].s64 + -4;
	// 824D94D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824D94D4: 5529F0BE  srwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D94D8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824D94DC: 5526103A  slwi r6, r9, 2
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824D94E0: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D94E4: 390B0030  addi r8, r11, 0x30
	ctx.r[8].s64 = ctx.r[11].s64 + 48;
	// 824D94E8: 7C0B252E  stfsx f0, r11, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 824D94EC: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D94F0: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D94F4: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 824D94F8: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 824D94FC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824D9500: D0070004  stfs f0, 4(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824D9504: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D9508: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D950C: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 824D9510: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 824D9514: D0070008  stfs f0, 8(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824D9518: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D951C: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9520: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 824D9524: D0070010  stfs f0, 0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824D9528: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D952C: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9530: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 824D9534: D0070014  stfs f0, 0x14(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824D9538: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D953C: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9540: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 824D9544: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 824D9548: D0070018  stfs f0, 0x18(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824D954C: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D9550: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9554: 7CE83A14  add r7, r8, r7
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 824D9558: D007FFF0  stfs f0, -0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 824D955C: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D9560: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9564: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 824D9568: D0070024  stfs f0, 0x24(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 824D956C: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D9570: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9574: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 824D9578: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 824D957C: D0070028  stfs f0, 0x28(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 824D9580: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D9584: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9588: 7C083D2E  stfsx f0, r8, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 824D958C: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D9590: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9594: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 824D9598: D0080034  stfs f0, 0x34(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 824D959C: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D95A0: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D95A4: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 824D95A8: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 824D95AC: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 824D95B0: D0080038  stfs f0, 0x38(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 824D95B4: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D95B8: 409AFF28  bne cr6, 0x824d94e0
	if !ctx.cr[6].eq {
	pc = 0x824D94E0; continue 'dispatch;
	}
	// 824D95BC: 7F06E000  cmpw cr6, r6, r28
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[28].s32, &mut ctx.xer);
	// 824D95C0: 4098004C  bge cr6, 0x824d960c
	if !ctx.cr[6].lt {
	pc = 0x824D960C; continue 'dispatch;
	}
	// 824D95C4: 54CB2036  slwi r11, r6, 4
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D95C8: 7D26E050  subf r9, r6, r28
	ctx.r[9].s64 = ctx.r[28].s64 - ctx.r[6].s64;
	// 824D95CC: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D95D0: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 824D95D4: 7C0B252E  stfsx f0, r11, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 824D95D8: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D95DC: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D95E0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824D95E4: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 824D95E8: D0080004  stfs f0, 4(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824D95EC: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D95F0: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D95F4: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 824D95F8: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 824D95FC: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 824D9600: D0080008  stfs f0, 8(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824D9604: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D9608: 409AFFC4  bne cr6, 0x824d95cc
	if !ctx.cr[6].eq {
	pc = 0x824D95CC; continue 'dispatch;
	}
	// 824D960C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 824D9610: D3FF0004  stfs f31, 4(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824D9614: 395F0020  addi r10, r31, 0x20
	ctx.r[10].s64 = ctx.r[31].s64 + 32;
	// 824D9618: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 824D961C: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 824D9620: 393F0010  addi r9, r31, 0x10
	ctx.r[9].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D9AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824D9AC8 size=668
    let mut pc: u32 = 0x824D9AC8;
    'dispatch: loop {
        match pc {
            0x824D9AC8 => {
    //   block [0x824D9AC8..0x824D9D64)
	// 824D9AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D9ACC: 4805B5E5  bl 0x825350b0
	ctx.lr = 0x824D9AD0;
	sub_82535080(ctx, base);
	// 824D9AD0: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 824D9AD4: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D9AD8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 824D9ADC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D9AE0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824D9AE4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824D9AE8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824D9AEC: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 824D9AF0: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9AF4: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 824D9AF8: 41990014  bgt cr6, 0x824d9b0c
	if ctx.cr[6].gt {
	pc = 0x824D9B0C; continue 'dispatch;
	}
	// 824D9AFC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824D9B00: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 824D9B04: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 824D9B08: 4805B5F8  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 824D9B0C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824D9B10: 3F608000  lis r27, -0x8000
	ctx.r[27].s64 = -2147483648;
	// 824D9B14: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824D9B18: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 824D9B1C: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 824D9B20: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 824D9B24: 4099001C  ble cr6, 0x824d9b40
	if !ctx.cr[6].gt {
	pc = 0x824D9B40; continue 'dispatch;
	}
	// 824D9B28: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824D9B2C: 41980008  blt cr6, 0x824d9b34
	if ctx.cr[6].lt {
	pc = 0x824D9B34; continue 'dispatch;
	}
	// 824D9B30: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824D9B34: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 824D9B38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824D9B3C: 4BF9478D  bl 0x8246e2c8
	ctx.lr = 0x824D9B40;
	sub_8246E2C8(ctx, base);
	// 824D9B40: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 824D9B44: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 824D9B48: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 824D9B4C: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 824D9B50: 419800F4  blt cr6, 0x824d9c44
	if ctx.cr[6].lt {
	pc = 0x824D9C44; continue 'dispatch;
	}
	// 824D9B54: 393DFFFC  addi r9, r29, -4
	ctx.r[9].s64 = ctx.r[29].s64 + -4;
	// 824D9B58: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 824D9B5C: 5529F0BE  srwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824D9B60: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824D9B64: 5526103A  slwi r6, r9, 2
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824D9B68: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D9B6C: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9B70: 390B0030  addi r8, r11, 0x30
	ctx.r[8].s64 = ctx.r[11].s64 + 48;
	// 824D9B74: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 824D9B78: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824D9B7C: 7C0B3D2E  stfsx f0, r11, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 824D9B80: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D9B84: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9B88: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 824D9B8C: D0070004  stfs f0, 4(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824D9B90: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D9B94: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9B98: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 824D9B9C: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 824D9BA0: D0070008  stfs f0, 8(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824D9BA4: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D9BA8: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9BAC: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 824D9BB0: D0070010  stfs f0, 0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824D9BB4: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D9BB8: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9BBC: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 824D9BC0: D0070014  stfs f0, 0x14(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824D9BC4: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D9BC8: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9BCC: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 824D9BD0: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 824D9BD4: D0070018  stfs f0, 0x18(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824D9BD8: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D9BDC: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9BE0: 7CE83A14  add r7, r8, r7
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 824D9BE4: D007FFF0  stfs f0, -0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 824D9BE8: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D9BEC: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9BF0: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 824D9BF4: D0070024  stfs f0, 0x24(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 824D9BF8: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D9BFC: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9C00: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 824D9C04: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 824D9C08: D0070028  stfs f0, 0x28(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 824D9C0C: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D9C10: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9C14: 7C083D2E  stfsx f0, r8, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 824D9C18: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D9C1C: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9C20: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 824D9C24: D0080034  stfs f0, 0x34(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 824D9C28: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D9C2C: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9C30: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 824D9C34: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 824D9C38: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 824D9C3C: D0080038  stfs f0, 0x38(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 824D9C40: 409AFF28  bne cr6, 0x824d9b68
	if !ctx.cr[6].eq {
	pc = 0x824D9B68; continue 'dispatch;
	}
	// 824D9C44: 7F06E800  cmpw cr6, r6, r29
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[29].s32, &mut ctx.xer);
	// 824D9C48: 4098004C  bge cr6, 0x824d9c94
	if !ctx.cr[6].lt {
	pc = 0x824D9C94; continue 'dispatch;
	}
	// 824D9C4C: 54CB2036  slwi r11, r6, 4
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824D9C50: 7D26E850  subf r9, r6, r29
	ctx.r[9].s64 = ctx.r[29].s64 - ctx.r[6].s64;
	// 824D9C54: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D9C58: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9C5C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 824D9C60: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824D9C64: 7C0B452E  stfsx f0, r11, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 824D9C68: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D9C6C: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9C70: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 824D9C74: D0080004  stfs f0, 4(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824D9C78: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D9C7C: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824D9C80: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 824D9C84: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 824D9C88: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 824D9C8C: D0080008  stfs f0, 8(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824D9C90: 409AFFC4  bne cr6, 0x824d9c54
	if !ctx.cr[6].eq {
	pc = 0x824D9C54; continue 'dispatch;
	}
	// 824D9C94: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824D9C98: 93A10074  stw r29, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 824D9C9C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 824D9CA0: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 824D9CA4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 824D9CA8: 93810084  stw r28, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[28].u32 ) };
	// 824D9CAC: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 824D9CB0: 93610088  stw r27, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[27].u32 ) };
	// 824D9CB4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824D9CB8: 9381008C  stw r28, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[28].u32 ) };
	// 824D9CBC: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 824D9CC0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D9CC4: 93810090  stw r28, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[28].u32 ) };
	// 824D9CC8: 93610094  stw r27, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[27].u32 ) };
	// 824D9CCC: 93810060  stw r28, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 824D9CD0: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 824D9CD4: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 824D9CD8: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 824D9CDC: 480D760D  bl 0x825b12e8
	ctx.lr = 0x824D9CE0;
	sub_825B12E8(ctx, base);
	// 824D9CE0: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 824D9CE4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824D9CE8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 824D9CEC: 4BFFF335  bl 0x824d9020
	ctx.lr = 0x824D9CF0;
	sub_824D9020(ctx, base);
	// 824D9CF0: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 824D9CF4: D3FA0004  stfs f31, 4(r26)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824D9CF8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824D9CFC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824D9D00: 409A0020  bne cr6, 0x824d9d20
	if !ctx.cr[6].eq {
	pc = 0x824D9D20; continue 'dispatch;
	}
	// 824D9D04: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D9D08: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824D9D0C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824D9D10: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824D9D14: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824D9D18: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824D9D1C: 4BF8A39D  bl 0x824640b8
	ctx.lr = 0x824D9D20;
	sub_824640B8(ctx, base);
	// 824D9D20: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 824D9D24: 480028D5  bl 0x824dc5f8
	ctx.lr = 0x824D9D28;
	sub_824DC5F8(ctx, base);
	// 824D9D28: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824D9D2C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824D9D30: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824D9D34: 409A0020  bne cr6, 0x824d9d54
	if !ctx.cr[6].eq {
	pc = 0x824D9D54; continue 'dispatch;
	}
	// 824D9D38: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824D9D3C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824D9D40: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824D9D44: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824D9D48: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824D9D4C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824D9D50: 4BF8A369  bl 0x824640b8
	ctx.lr = 0x824D9D54;
	sub_824640B8(ctx, base);
	// 824D9D54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824D9D58: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 824D9D5C: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 824D9D60: 4805B3A0  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824D9D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824D9D68 size=2688
    let mut pc: u32 = 0x824D9D68;
    'dispatch: loop {
        match pc {
            0x824D9D68 => {
    //   block [0x824D9D68..0x824DA7E8)
	// 824D9D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824D9D6C: 4805B34D  bl 0x825350b8
	ctx.lr = 0x824D9D70;
	sub_82535080(ctx, base);
	// 824D9D70: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 824D9D74: 4805C25D  bl 0x82535fd0
	ctx.lr = 0x824D9D78;
	sub_82535FB0(ctx, base);
	// 824D9D78: 9421FAA0  stwu r1, -0x560(r1)
	ea = ctx.r[1].u32.wrapping_add(-1376 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824D9D7C: FEC01090  fmr f22, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[22].f64 = ctx.f[2].f64;
	// 824D9D80: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824D9D84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824D9D88: FFA00890  fmr f29, f1
	ctx.f[29].f64 = ctx.f[1].f64;
	// 824D9D8C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824D9D90: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 824D9D94: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824D9D98: FF16F800  fcmpu cr6, f22, f31
	ctx.cr[6].compare_f64(ctx.f[22].f64, ctx.f[31].f64);
	// 824D9D9C: 41990018  bgt cr6, 0x824d9db4
	if ctx.cr[6].gt {
	pc = 0x824D9DB4; continue 'dispatch;
	}
	// 824D9DA0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824D9DA4: 38210560  addi r1, r1, 0x560
	ctx.r[1].s64 = ctx.r[1].s64 + 1376;
	// 824D9DA8: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 824D9DAC: 4805C271  bl 0x8253601c
	ctx.lr = 0x824D9DB0;
	sub_82535FFC(ctx, base);
	// 824D9DB0: 4805B358  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 824D9DB4: FF1DF800  fcmpu cr6, f29, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[31].f64);
	// 824D9DB8: 4099FFE8  ble cr6, 0x824d9da0
	if !ctx.cr[6].gt {
	pc = 0x824D9DA0; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DA7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824DA7E8 size=864
    let mut pc: u32 = 0x824DA7E8;
    'dispatch: loop {
        match pc {
            0x824DA7E8 => {
    //   block [0x824DA7E8..0x824DAB48)
	// 824DA7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DA7EC: 4805A8D1  bl 0x825350bc
	ctx.lr = 0x824DA7F0;
	sub_82535080(ctx, base);
	// 824DA7F0: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 824DA7F4: 4805B7F5  bl 0x82535fe8
	ctx.lr = 0x824DA7F8;
	sub_82535FB0(ctx, base);
	// 824DA7F8: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DA7FC: FF801090  fmr f28, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[2].f64;
	// 824DA800: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824DA804: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824DA808: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 824DA80C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824DA810: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 824DA814: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824DA818: FF1CF800  fcmpu cr6, f28, f31
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[31].f64);
	// 824DA81C: 40990318  ble cr6, 0x824dab34
	if !ctx.cr[6].gt {
	pc = 0x824DAB34; continue 'dispatch;
	}
	// 824DA820: FF1EF800  fcmpu cr6, f30, f31
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 824DA824: 40990310  ble cr6, 0x824dab34
	if !ctx.cr[6].gt {
	pc = 0x824DAB34; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DAB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824DAB48 size=644
    let mut pc: u32 = 0x824DAB48;
    'dispatch: loop {
        match pc {
            0x824DAB48 => {
    //   block [0x824DAB48..0x824DADCC)
	// 824DAB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DAB4C: 4805A569  bl 0x825350b4
	ctx.lr = 0x824DAB50;
	sub_82535080(ctx, base);
	// 824DAB50: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 824DAB54: 4805B495  bl 0x82535fe8
	ctx.lr = 0x824DAB58;
	sub_82535FB0(ctx, base);
	// 824DAB58: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DAB5C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824DAB60: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 824DAB64: 3941008C  addi r10, r1, 0x8c
	ctx.r[10].s64 = ctx.r[1].s64 + 140;
	// 824DAB68: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824DAB6C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824DAB70: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DAB74: 91410080  stw r10, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 824DAB78: 3B8BFFFF  addi r28, r11, -1
	ctx.r[28].s64 = ctx.r[11].s64 + -1;
	// 824DAB7C: 91210084  stw r9, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 824DAB80: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 824DAB84: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824DAB88: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 824DAB8C: 91610088  stw r11, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 824DAB90: 4198009C  blt cr6, 0x824dac2c
	if ctx.cr[6].lt {
	pc = 0x824DAC2C; continue 'dispatch;
	}
	// 824DAB94: 579E103A  slwi r30, r28, 2
	ctx.r[30].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 824DAB98: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DAB9C: 7FFE502E  lwzx r31, r30, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824DABA0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824DABA4: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 824DABA8: 409A000C  bne cr6, 0x824dabb4
	if !ctx.cr[6].eq {
	pc = 0x824DABB4; continue 'dispatch;
	}
	// 824DABAC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824DABB0: 48000010  b 0x824dabc0
	pc = 0x824DABC0; continue 'dispatch;
	// 824DABB4: 811F0018  lwz r8, 0x18(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824DABB8: 7F08D840  cmplw cr6, r8, r27
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[27].u32, &mut ctx.xer);
	// 824DABBC: 409A0060  bne cr6, 0x824dac1c
	if !ctx.cr[6].eq {
	pc = 0x824DAC1C; continue 'dispatch;
	}
	// 824DABC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DABC4: 419A0058  beq cr6, 0x824dac1c
	if ctx.cr[6].eq {
	pc = 0x824DAC1C; continue 'dispatch;
	}
	// 824DABC8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DABCC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824DABD0: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824DABD4: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824DABD8: 7D69502E  lwzx r11, r9, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824DABDC: 7D7E512E  stwx r11, r30, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 824DABE0: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 824DABE4: 81410084  lwz r10, 0x84(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 824DABE8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824DABEC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824DABF0: 409A0010  bne cr6, 0x824dac00
	if !ctx.cr[6].eq {
	pc = 0x824DAC00; continue 'dispatch;
	}
	// 824DABF4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824DABF8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 824DABFC: 4BF93755  bl 0x8246e350
	ctx.lr = 0x824DAC00;
	sub_8246E350(ctx, base);
	// 824DAC00: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 824DAC04: 81410080  lwz r10, 0x80(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 824DAC08: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DAC0C: 7FEB512E  stwx r31, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 824DAC10: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 824DAC14: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 824DAC18: 91210084  stw r9, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 824DAC1C: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 824DAC20: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 824DAC24: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824DAC28: 4098FF70  bge cr6, 0x824dab98
	if !ctx.cr[6].lt {
	pc = 0x824DAB98; continue 'dispatch;
	}
	// 824DAC2C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824DAC30: 3BC9FFFF  addi r30, r9, -1
	ctx.r[30].s64 = ctx.r[9].s64 + -1;
	// 824DAC34: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824DAC38: C38B1FF8  lfs f28, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 824DAC3C: FFA0E090  fmr f29, f28
	ctx.f[29].f64 = ctx.f[28].f64;
	// 824DAC40: FFE0E090  fmr f31, f28
	ctx.f[31].f64 = ctx.f[28].f64;
	// 824DAC44: 4198004C  blt cr6, 0x824dac90
	if ctx.cr[6].lt {
	pc = 0x824DAC90; continue 'dispatch;
	}
	// 824DAC48: 57DF103A  slwi r31, r30, 2
	ctx.r[31].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 824DAC4C: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 824DAC50: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824DAC54: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824DAC58: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 824DAC5C: 409A0008  bne cr6, 0x824dac64
	if !ctx.cr[6].eq {
	pc = 0x824DAC64; continue 'dispatch;
	}
	// 824DAC60: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824DAC64: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 824DAC68: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 824DAC6C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824DAC70: 4BFFFED9  bl 0x824dab48
	ctx.lr = 0x824DAC74;
	sub_824DAB48(ctx, base);
	// 824DAC74: EC01F828  fsubs f0, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[1].f64 - ctx.f[31].f64) as f32) as f64);
	// 824DAC78: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 824DAC7C: EFA1E82A  fadds f29, f1, f29
	ctx.f[29].f64 = ((ctx.f[1].f64 + ctx.f[29].f64) as f32) as f64;
	// 824DAC80: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 824DAC84: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824DAC88: FFE0F86E  fsel f31, f0, f1, f31
	ctx.f[31].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[1].f64 } else { ctx.f[31].f64 };
	// 824DAC8C: 4098FFC0  bge cr6, 0x824dac4c
	if !ctx.cr[6].lt {
	pc = 0x824DAC4C; continue 'dispatch;
	}
	// 824DAC90: 897B00D8  lbz r11, 0xd8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(216 as u32) ) } as u64;
	// 824DAC94: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 824DAC98: 419A0010  beq cr6, 0x824daca8
	if ctx.cr[6].eq {
	pc = 0x824DACA8; continue 'dispatch;
	}
	// 824DAC9C: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 824DACA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824DACA4: 409A0008  bne cr6, 0x824dacac
	if !ctx.cr[6].eq {
	pc = 0x824DACAC; continue 'dispatch;
	}
	// 824DACA8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824DACAC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824DACB0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DACB4: 419A0044  beq cr6, 0x824dacf8
	if ctx.cr[6].eq {
	pc = 0x824DACF8; continue 'dispatch;
	}
	// 824DACB8: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 824DACBC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DACC0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DACC4: 409A0020  bne cr6, 0x824dace4
	if !ctx.cr[6].eq {
	pc = 0x824DACE4; continue 'dispatch;
	}
	// 824DACC8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DACCC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824DACD0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DACD4: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 824DACD8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DACDC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824DACE0: 4BF893D9  bl 0x824640b8
	ctx.lr = 0x824DACE4;
	sub_824640B8(ctx, base);
	// 824DACE4: FC20E090  fmr f1, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[28].f64;
	// 824DACE8: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 824DACEC: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 824DACF0: 4805B345  bl 0x82536034
	ctx.lr = 0x824DACF4;
	sub_82535FFC(ctx, base);
	// 824DACF4: 4805A410  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 824DACF8: 817B00D0  lwz r11, 0xd0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(208 as u32) ) } as u64;
	// 824DACFC: 387B00D0  addi r3, r27, 0xd0
	ctx.r[3].s64 = ctx.r[27].s64 + 208;
	// 824DAD00: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824DAD04: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824DAD08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DAD0C: 4E800421  bctrl
	ctx.lr = 0x824DAD10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DAD10: C0010064  lfs f0, 0x64(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824DAD14: C1A10078  lfs f13, 0x78(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824DAD18: FF1DE000  fcmpu cr6, f29, f28
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[28].f64);
	// 824DAD1C: ED806828  fsubs f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 824DAD20: FD6C682E  fsel f11, f12, f0, f13
	ctx.f[11].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 824DAD24: C1810050  lfs f12, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824DAD28: ED4C5828  fsubs f10, f12, f11
	ctx.f[10].f64 = (((ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 824DAD2C: FF8A5B2E  fsel f28, f10, f12, f11
	ctx.f[28].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[11].f64 };
	// 824DAD30: 419AFF88  beq cr6, 0x824dacb8
	if ctx.cr[6].eq {
	pc = 0x824DACB8; continue 'dispatch;
	}
	// 824DAD34: EFDC07B2  fmuls f30, f28, f30
	ctx.f[30].f64 = (((ctx.f[28].f64 * ctx.f[30].f64) as f32) as f64);
	// 824DAD38: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824DAD3C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824DAD40: ED7DF028  fsubs f11, f29, f30
	ctx.f[11].f64 = (((ctx.f[29].f64 - ctx.f[30].f64) as f32) as f64);
	// 824DAD44: FD6BEFAE  fsel f11, f11, f30, f29
	ctx.f[11].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[30].f64 } else { ctx.f[29].f64 };
	// 824DAD48: ED4BF828  fsubs f10, f11, f31
	ctx.f[10].f64 = (((ctx.f[11].f64 - ctx.f[31].f64) as f32) as f64);
	// 824DAD4C: FFEAFAEE  fsel f31, f10, f11, f31
	ctx.f[31].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[11].f64 } else { ctx.f[31].f64 };
	// 824DAD50: ED40F828  fsubs f10, f0, f31
	ctx.f[10].f64 = (((ctx.f[0].f64 - ctx.f[31].f64) as f32) as f64);
	// 824DAD54: ED6CF828  fsubs f11, f12, f31
	ctx.f[11].f64 = (((ctx.f[12].f64 - ctx.f[31].f64) as f32) as f64);
	// 824DAD58: ED2DF828  fsubs f9, f13, f31
	ctx.f[9].f64 = (((ctx.f[13].f64 - ctx.f[31].f64) as f32) as f64);
	// 824DAD5C: FC0AF82E  fsel f0, f10, f0, f31
	ctx.f[0].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[31].f64 };
	// 824DAD60: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 824DAD64: FD8BFB2E  fsel f12, f11, f12, f31
	ctx.f[12].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[31].f64 };
	// 824DAD68: D1810050  stfs f12, 0x50(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 824DAD6C: FC09FB6E  fsel f0, f9, f13, f31
	ctx.f[0].f64 = if ctx.f[9].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[31].f64 };
	// 824DAD70: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 824DAD74: 4BFB4EFD  bl 0x8248fc70
	ctx.lr = 0x824DAD78;
	sub_8248FC70(ctx, base);
	// 824DAD78: EC1CE82A  fadds f0, f28, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[28].f64 + ctx.f[29].f64) as f32) as f64;
	// 824DAD7C: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 824DAD80: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DAD84: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DAD88: EDBE0028  fsubs f13, f30, f0
	ctx.f[13].f64 = (((ctx.f[30].f64 - ctx.f[0].f64) as f32) as f64);
	// 824DAD8C: FC0DF02E  fsel f0, f13, f0, f30
	ctx.f[0].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[30].f64 };
	// 824DAD90: EDA0F828  fsubs f13, f0, f31
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[31].f64) as f32) as f64);
	// 824DAD94: FFEDF82E  fsel f31, f13, f0, f31
	ctx.f[31].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[31].f64 };
	// 824DAD98: 409A0020  bne cr6, 0x824dadb8
	if !ctx.cr[6].eq {
	pc = 0x824DADB8; continue 'dispatch;
	}
	// 824DAD9C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DADA0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824DADA4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DADA8: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 824DADAC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DADB0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824DADB4: 4BF89305  bl 0x824640b8
	ctx.lr = 0x824DADB8;
	sub_824640B8(ctx, base);
	// 824DADB8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824DADBC: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 824DADC0: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 824DADC4: 4805B271  bl 0x82536034
	ctx.lr = 0x824DADC8;
	sub_82535FFC(ctx, base);
	// 824DADC8: 4805A33C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DADD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DADD0 size=308
    let mut pc: u32 = 0x824DADD0;
    'dispatch: loop {
        match pc {
            0x824DADD0 => {
    //   block [0x824DADD0..0x824DAF04)
	// 824DADD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DADD4: 4805A2DD  bl 0x825350b0
	ctx.lr = 0x824DADD8;
	sub_82535080(ctx, base);
	// 824DADD8: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 824DADDC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DADE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824DADE4: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DADE8: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 824DADEC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 824DADF0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824DADF4: 3FC08000  lis r30, -0x8000
	ctx.r[30].s64 = -2147483648;
	// 824DADF8: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 824DADFC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 824DAE00: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824DAE04: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824DAE08: 55441036  rlwinm r4, r10, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 824DAE0C: 7D7CD82E  lwzx r11, r28, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 824DAE10: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 824DAE14: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 824DAE18: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824DAE1C: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 824DAE20: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 824DAE24: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 824DAE28: 4199000C  bgt cr6, 0x824dae34
	if ctx.cr[6].gt {
	pc = 0x824DAE34; continue 'dispatch;
	}
	// 824DAE2C: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 824DAE30: 4800001C  b 0x824dae4c
	pc = 0x824DAE4C; continue 'dispatch;
	// 824DAE34: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DAE38: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 824DAE3C: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 824DAE40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DAE44: 4E800421  bctrl
	ctx.lr = 0x824DAE48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DAE48: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 824DAE4C: 7FEBF378  or r11, r31, r30
	ctx.r[11].u64 = ctx.r[31].u64 | ctx.r[30].u64;
	// 824DAE50: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 824DAE54: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 824DAE58: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824DAE5C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 824DAE60: 4099003C  ble cr6, 0x824dae9c
	if !ctx.cr[6].gt {
	pc = 0x824DAE9C; continue 'dispatch;
	}
	// 824DAE64: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824DAE68: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 824DAE6C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 824DAE70: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DAE74: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824DAE78: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824DAE7C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824DAE80: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824DAE84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DAE88: 7D09392E  stwx r8, r9, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32), ctx.r[8].u32) };
	// 824DAE8C: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824DAE90: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824DAE94: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 824DAE98: 409AFFD8  bne cr6, 0x824dae70
	if !ctx.cr[6].eq {
	pc = 0x824DAE70; continue 'dispatch;
	}
	// 824DAE9C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 824DAEA0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824DAEA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824DAEA8: 4BFFFCA1  bl 0x824dab48
	ctx.lr = 0x824DAEAC;
	sub_824DAB48(ctx, base);
	// 824DAEAC: 7C7CD82E  lwzx r3, r28, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 824DAEB0: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 824DAEB4: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 824DAEB8: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 824DAEBC: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824DAEC0: 409A0014  bne cr6, 0x824daed4
	if !ctx.cr[6].eq {
	pc = 0x824DAED4; continue 'dispatch;
	}
	// 824DAEC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DAEC8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824DAECC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DAED0: 4E800421  bctrl
	ctx.lr = 0x824DAED4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DAED4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824DAED8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DAEDC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DAEE0: 409A0018  bne cr6, 0x824daef8
	if !ctx.cr[6].eq {
	pc = 0x824DAEF8; continue 'dispatch;
	}
	// 824DAEE4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DAEE8: 7C7CD82E  lwzx r3, r28, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 824DAEEC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DAEF0: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824DAEF4: 4BF891C5  bl 0x824640b8
	ctx.lr = 0x824DAEF8;
	sub_824640B8(ctx, base);
	// 824DAEF8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824DAEFC: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 824DAF00: 4805A200  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DAF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DAF08 size=2232
    let mut pc: u32 = 0x824DAF08;
    'dispatch: loop {
        match pc {
            0x824DAF08 => {
    //   block [0x824DAF08..0x824DB7C0)
	// 824DAF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DAF0C: 4805A191  bl 0x8253509c
	ctx.lr = 0x824DAF10;
	sub_82535080(ctx, base);
	// 824DAF10: 3981FFA0  addi r12, r1, -0x60
	ctx.r[12].s64 = ctx.r[1].s64 + -96;
	// 824DAF14: 4805B0B9  bl 0x82535fcc
	ctx.lr = 0x824DAF18;
	sub_82535FB0(ctx, base);
	// 824DAF18: 3980FF30  li r12, -0xd0
	ctx.r[12].s64 = -208;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DB7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DB7C0 size=1228
    let mut pc: u32 = 0x824DB7C0;
    'dispatch: loop {
        match pc {
            0x824DB7C0 => {
    //   block [0x824DB7C0..0x824DBC8C)
	// 824DB7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DB7C4: 480598E1  bl 0x825350a4
	ctx.lr = 0x824DB7C8;
	sub_82535080(ctx, base);
	// 824DB7C8: 3981FFB0  addi r12, r1, -0x50
	ctx.r[12].s64 = ctx.r[1].s64 + -80;
	// 824DB7CC: 4805A815  bl 0x82535fe0
	ctx.lr = 0x824DB7D0;
	sub_82535FB0(ctx, base);
	// 824DB7D0: 3980FF60  li r12, -0xa0
	ctx.r[12].s64 = -160;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DBC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824DBC90 size=1692
    //   switch @ 0x824DBD2C: r11 with 27 label(s)
    //       case  0 → 0x824DBD9C
    //       case  1 → 0x824DC0F0
    //       case  2 → 0x824DBE90
    //       case  3 → 0x824DBDB4
    //       case  4 → 0x824DC0D0
    //       case  5 → 0x824DBDD8
    //       case  6 → 0x824DC044
    //       case  7 → 0x824DC044
    //       case  8 → 0x824DC044
    //       case  9 → 0x824DC044
    //       case 10 → 0x824DBEF0
    //       case 11 → 0x824DBF5C
    //       case 12 → 0x824DC314
    //       case 13 → 0x824DC044
    //       case 14 → 0x824DC314
    //       case 15 → 0x824DC314
    //       case 16 → 0x824DC314
    //       case 17 → 0x824DC314
    //       case 18 → 0x824DBFA4
    //       case 19 → 0x824DC044
    //       case 20 → 0x824DC044
    //       case 21 → 0x824DC314
    //       case 22 → 0x824DC314
    //       case 23 → 0x824DC314
    //       case 24 → 0x824DBEDC
    //       case 25 → 0x824DC314
    //       case 26 → 0x824DBF80
    let mut pc: u32 = 0x824DBC90;
    'dispatch: loop {
        match pc {
            0x824DBC90 => {
    //   block [0x824DBC90..0x824DBD9C)
	// 824DBC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DBC94: 48059411  bl 0x825350a4
	ctx.lr = 0x824DBC98;
	sub_82535080(ctx, base);
	// 824DBC98: DBC1FFA0  stfd f30, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[30].u64 ) };
	// 824DBC9C: DBE1FFA8  stfd f31, -0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 824DBCA0: 3981FFA0  addi r12, r1, -0x60
	ctx.r[12].s64 = ctx.r[1].s64 + -96;
	// 824DBCA4: 4805D6E1  bl 0x82539384
	ctx.lr = 0x824DBCA8;
	sub_82539130(ctx, base);
	// 824DBCA8: 9421FA50  stwu r1, -0x5b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-1456 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DBCAC: 396100C0  addi r11, r1, 0xc0
	ctx.r[11].s64 = ctx.r[1].s64 + 192;
	// 824DBCB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824DBCB4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824DBCB8: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	pc = 0x824DBD9C; continue 'dispatch;
            }
            0x824DBD9C => {
    //   block [0x824DBD9C..0x824DBDB4)
	// 824DBD9C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824DBDA0: C03F0010  lfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824DBDA4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 824DBDA8: C04B1850  lfs f2, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 824DBDAC: 4BFFB8DD  bl 0x824d7688
	ctx.lr = 0x824DBDB0;
	sub_824D7688(ctx, base);
	// 824DBDB0: 48000368  b 0x824dc118
	pc = 0x824DC118; continue 'dispatch;
            }
            0x824DBDB4 => {
    //   block [0x824DBDB4..0x824DBDD8)
	// 824DBDB4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	pc = 0x824DBDD8; continue 'dispatch;
            }
            0x824DBDD8 => {
    //   block [0x824DBDD8..0x824DBE90)
	// 824DBDD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DBDDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824DBDE0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 824DBDE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DBDE8: 4E800421  bctrl
	ctx.lr = 0x824DBDEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DBDEC: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DBDF0: 3B400010  li r26, 0x10
	ctx.r[26].s64 = 16;
	// 824DBDF4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824DBDF8: 397B0001  addi r11, r27, 1
	ctx.r[11].s64 = ctx.r[27].s64 + 1;
	// 824DBDFC: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824DBE00: 55642036  slwi r4, r11, 4
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824DBE04: 83A30020  lwz r29, 0x20(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 824DBE08: 8143002C  lwz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 824DBE0C: 7D7D2214  add r11, r29, r4
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[4].u64;
	// 824DBE10: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824DBE14: 4199000C  bgt cr6, 0x824dbe20
	if ctx.cr[6].gt {
	pc = 0x824DBE20; continue 'dispatch;
	}
	// 824DBE18: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 824DBE1C: 48000018  b 0x824dbe34
	pc = 0x824DBE34; continue 'dispatch;
	// 824DBE20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DBE24: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824DBE28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DBE2C: 4E800421  bctrl
	ctx.lr = 0x824DBE30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DBE30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824DBE34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DBE38: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824DBE3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824DBE40: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 824DBE44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DBE48: 4E800421  bctrl
	ctx.lr = 0x824DBE4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DBE4C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824DBE50: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 824DBE54: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 824DBE58: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 824DBE5C: C02B1850  lfs f1, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824DBE60: 4BFFDC69  bl 0x824d9ac8
	ctx.lr = 0x824DBE64;
	sub_824D9AC8(ctx, base);
	// 824DBE64: 7C7AC82E  lwzx r3, r26, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824DBE68: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 824DBE6C: 93A30020  stw r29, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 824DBE70: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824DBE74: 409A02A4  bne cr6, 0x824dc118
	if !ctx.cr[6].eq {
	pc = 0x824DC118; continue 'dispatch;
	}
	// 824DBE78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DBE7C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824DBE80: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824DBE84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DBE88: 4E800421  bctrl
	ctx.lr = 0x824DBE8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DBE8C: 4800028C  b 0x824dc118
	pc = 0x824DC118; continue 'dispatch;
            }
            0x824DBE90 => {
    //   block [0x824DBE90..0x824DBEDC)
	// 824DBE90: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	pc = 0x824DBEDC; continue 'dispatch;
            }
            0x824DBEDC => {
    //   block [0x824DBEDC..0x824DBEF0)
	// 824DBEDC: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 824DBEE0: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824DBEE4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824DBEE8: 4BFFFDA9  bl 0x824dbc90
	ctx.lr = 0x824DBEEC;
	sub_824DBC90(ctx, base);
	// 824DBEEC: 48000428  b 0x824dc314
	pc = 0x824DC314; continue 'dispatch;
            }
            0x824DBEF0 => {
    //   block [0x824DBEF0..0x824DBF5C)
	// 824DBEF0: 396100C0  addi r11, r1, 0xc0
	ctx.r[11].s64 = ctx.r[1].s64 + 192;
	// 824DBEF4: 38A100E0  addi r5, r1, 0xe0
	ctx.r[5].s64 = ctx.r[1].s64 + 224;
	// 824DBEF8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824DBEFC: 38610250  addi r3, r1, 0x250
	ctx.r[3].s64 = ctx.r[1].s64 + 592;
	pc = 0x824DBF5C; continue 'dispatch;
            }
            0x824DBF5C => {
    //   block [0x824DBF5C..0x824DBF80)
	// 824DBF5C: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 824DBF60: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824DBF64: 38610290  addi r3, r1, 0x290
	ctx.r[3].s64 = ctx.r[1].s64 + 656;
	// 824DBF68: 480CD131  bl 0x825a9098
	ctx.lr = 0x824DBF6C;
	sub_825A9098(ctx, base);
	// 824DBF6C: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 824DBF70: 38810290  addi r4, r1, 0x290
	ctx.r[4].s64 = ctx.r[1].s64 + 656;
	// 824DBF74: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824DBF78: 4BFFFD19  bl 0x824dbc90
	ctx.lr = 0x824DBF7C;
	sub_824DBC90(ctx, base);
	// 824DBF7C: 48000398  b 0x824dc314
	pc = 0x824DC314; continue 'dispatch;
            }
            0x824DBF80 => {
    //   block [0x824DBF80..0x824DBFA4)
	// 824DBF80: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 824DBF84: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824DBF88: 38610210  addi r3, r1, 0x210
	ctx.r[3].s64 = ctx.r[1].s64 + 528;
	// 824DBF8C: 480CD10D  bl 0x825a9098
	ctx.lr = 0x824DBF90;
	sub_825A9098(ctx, base);
	// 824DBF90: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 824DBF94: 38810210  addi r4, r1, 0x210
	ctx.r[4].s64 = ctx.r[1].s64 + 528;
	// 824DBF98: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824DBF9C: 4BFFFCF5  bl 0x824dbc90
	ctx.lr = 0x824DBFA0;
	sub_824DBC90(ctx, base);
	// 824DBFA0: 48000374  b 0x824dc314
	pc = 0x824DC314; continue 'dispatch;
            }
            0x824DBFA4 => {
    //   block [0x824DBFA4..0x824DC044)
	// 824DBFA4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824DBFA8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824DBFAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DBFB0: 40990364  ble cr6, 0x824dc314
	if !ctx.cr[6].gt {
	pc = 0x824DC314; continue 'dispatch;
	}
	// 824DBFB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824DBFB8: 3B7C0010  addi r27, r28, 0x10
	ctx.r[27].s64 = ctx.r[28].s64 + 16;
	// 824DBFBC: 3B5C0020  addi r26, r28, 0x20
	ctx.r[26].s64 = ctx.r[28].s64 + 32;
	// 824DBFC0: 3B3C0030  addi r25, r28, 0x30
	ctx.r[25].s64 = ctx.r[28].s64 + 48;
	// 824DBFC4: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 824DBFC8: 3B0B6DD0  addi r24, r11, 0x6dd0
	ctx.r[24].s64 = ctx.r[11].s64 + 28112;
	// 824DBFCC: 39610120  addi r11, r1, 0x120
	ctx.r[11].s64 = ctx.r[1].s64 + 288;
	pc = 0x824DC044; continue 'dispatch;
            }
            0x824DC044 => {
    //   block [0x824DC044..0x824DC0D0)
	// 824DC044: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC048: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824DC04C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824DC050: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DC054: 4E800421  bctrl
	ctx.lr = 0x824DC058;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DC058: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824DC05C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC060: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824DC064: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DC068: 4E800421  bctrl
	ctx.lr = 0x824DC06C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DC06C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824DC070: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 824DC074: 419A02A0  beq cr6, 0x824dc314
	if ctx.cr[6].eq {
	pc = 0x824DC314; continue 'dispatch;
	}
	// 824DC078: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC07C: 38A102D0  addi r5, r1, 0x2d0
	ctx.r[5].s64 = ctx.r[1].s64 + 720;
	// 824DC080: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824DC084: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824DC088: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824DC08C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DC090: 4E800421  bctrl
	ctx.lr = 0x824DC094;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DC094: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824DC098: 419A0010  beq cr6, 0x824dc0a8
	if ctx.cr[6].eq {
	pc = 0x824DC0A8; continue 'dispatch;
	}
	// 824DC09C: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 824DC0A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824DC0A4: 4BFFFBED  bl 0x824dbc90
	ctx.lr = 0x824DC0A8;
	sub_824DBC90(ctx, base);
	// 824DC0A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC0AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824DC0B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824DC0B4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824DC0B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DC0BC: 4E800421  bctrl
	ctx.lr = 0x824DC0C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DC0C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824DC0C4: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 824DC0C8: 409AFFB0  bne cr6, 0x824dc078
	if !ctx.cr[6].eq {
	pc = 0x824DC078; continue 'dispatch;
	}
	// 824DC0CC: 48000248  b 0x824dc314
	pc = 0x824DC314; continue 'dispatch;
            }
            0x824DC0D0 => {
    //   block [0x824DC0D0..0x824DC0F0)
	// 824DC0D0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824DC0D4: C03F0010  lfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824DC0D8: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 824DC0DC: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 824DC0E0: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 824DC0E4: C04B1850  lfs f2, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 824DC0E8: 4BFFDC81  bl 0x824d9d68
	ctx.lr = 0x824DC0EC;
	sub_824D9D68(ctx, base);
	// 824DC0EC: 4800002C  b 0x824dc118
	pc = 0x824DC118; continue 'dispatch;
            }
            0x824DC0F0 => {
    //   block [0x824DC0F0..0x824DC314)
	// 824DC0F0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824DC0F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824DC0F8: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 824DC0FC: C3EB1850  lfs f31, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824DC100: 480252E1  bl 0x825013e0
	ctx.lr = 0x824DC104;
	sub_825013E0(ctx, base);
	// 824DC104: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 824DC108: FC40F890  fmr f2, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[31].f64;
	// 824DC10C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 824DC110: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 824DC114: 4BFFE6D5  bl 0x824da7e8
	ctx.lr = 0x824DC118;
	sub_824DA7E8(ctx, base);
	// 824DC118: C0210060  lfs f1, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824DC11C: FF01F000  fcmpu cr6, f1, f30
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[30].f64);
	// 824DC120: 419A01F4  beq cr6, 0x824dc314
	if ctx.cr[6].eq {
	pc = 0x824DC314; continue 'dispatch;
	}
	// 824DC124: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 824DC128: 4BFFB951  bl 0x824d7a78
	ctx.lr = 0x824DC12C;
	sub_824D7A78(ctx, base);
	// 824DC12C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824DC130: 38610180  addi r3, r1, 0x180
	ctx.r[3].s64 = ctx.r[1].s64 + 384;
	// 824DC134: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 824DC138: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824DC13C: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 824DC140: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 824DC144: 4800042D  bl 0x824dc570
	ctx.lr = 0x824DC148;
	sub_824DC570(ctx, base);
	// 824DC148: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824DC14C: 3BE00010  li r31, 0x10
	ctx.r[31].s64 = 16;
	// 824DC150: C3F70000  lfs f31, 0(r23)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824DC154: 39570020  addi r10, r23, 0x20
	ctx.r[10].s64 = ctx.r[23].s64 + 32;
	// 824DC158: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824DC15C: C3D70004  lfs f30, 4(r23)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 824DC160: 39610190  addi r11, r1, 0x190
	ctx.r[11].s64 = ctx.r[1].s64 + 400;
	// 824DC164: D3E10180  stfs f31, 0x180(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(384 as u32), tmp.u32 ) };
	// 824DC168: D3C10184  stfs f30, 0x184(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(388 as u32), tmp.u32 ) };
	pc = 0x824DC314; continue 'dispatch;
            }
            0x824DC314 => {
    //   block [0x824DC314..0x824DC32C)
	// 824DC314: 382105B0  addi r1, r1, 0x5b0
	ctx.r[1].s64 = ctx.r[1].s64 + 1456;
	// 824DC318: 3981FFA0  addi r12, r1, -0x60
	ctx.r[12].s64 = ctx.r[1].s64 + -96;
	// 824DC31C: 4805D301  bl 0x8253961c
	ctx.lr = 0x824DC320;
	sub_825393C8(ctx, base);
	// 824DC320: CBC1FFA0  lfd f30, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-96 as u32) ) };
	// 824DC324: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 824DC328: 48058DCC  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DC330 size=384
    let mut pc: u32 = 0x824DC330;
    'dispatch: loop {
        match pc {
            0x824DC330 => {
    //   block [0x824DC330..0x824DC4B0)
	// 824DC330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DC334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824DC338: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824DC33C: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 824DC340: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 824DC344: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DC348: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 824DC34C: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 824DC350: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 824DC354: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 824DC358: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DC4B0 size=192
    let mut pc: u32 = 0x824DC4B0;
    'dispatch: loop {
        match pc {
            0x824DC4B0 => {
    //   block [0x824DC4B0..0x824DC570)
	// 824DC4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DC4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824DC4B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824DC4BC: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DC4C0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824DC4C4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 824DC4C8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824DC570 size=136
    let mut pc: u32 = 0x824DC570;
    'dispatch: loop {
        match pc {
            0x824DC570 => {
    //   block [0x824DC570..0x824DC5F8)
	// 824DC570: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824DC574: 39230010  addi r9, r3, 0x10
	ctx.r[9].s64 = ctx.r[3].s64 + 16;
	// 824DC578: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 824DC57C: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 824DC580: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 824DC584: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824DC588: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 824DC58C: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824DC590: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 824DC594: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DC5F8 size=140
    let mut pc: u32 = 0x824DC5F8;
    'dispatch: loop {
        match pc {
            0x824DC5F8 => {
    //   block [0x824DC5F8..0x824DC684)
	// 824DC5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DC5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824DC600: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824DC604: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DC608: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824DC60C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824DC610: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DC614: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DC618: 409A002C  bne cr6, 0x824dc644
	if !ctx.cr[6].eq {
	pc = 0x824DC644; continue 'dispatch;
	}
	// 824DC61C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC620: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824DC624: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824DC628: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824DC62C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DC630: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824DC634: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824DC638: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824DC63C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DC640: 4BF87A79  bl 0x824640b8
	ctx.lr = 0x824DC644;
	sub_824640B8(ctx, base);
	// 824DC644: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824DC648: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DC64C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DC650: 409A0020  bne cr6, 0x824dc670
	if !ctx.cr[6].eq {
	pc = 0x824DC670; continue 'dispatch;
	}
	// 824DC654: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC658: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824DC65C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DC660: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC664: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DC668: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824DC66C: 4BF87A4D  bl 0x824640b8
	ctx.lr = 0x824DC670;
	sub_824640B8(ctx, base);
	// 824DC670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824DC674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824DC678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824DC67C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824DC680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DC688 size=72
    let mut pc: u32 = 0x824DC688;
    'dispatch: loop {
        match pc {
            0x824DC688 => {
    //   block [0x824DC688..0x824DC6D0)
	// 824DC688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DC68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824DC690: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824DC694: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DC698: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DC69C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824DC6A0: 396B1C0C  addi r11, r11, 0x1c0c
	ctx.r[11].s64 = ctx.r[11].s64 + 7180;
	// 824DC6A4: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 824DC6A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824DC6AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824DC6B0: 419A000C  beq cr6, 0x824dc6bc
	if ctx.cr[6].eq {
	pc = 0x824DC6BC; continue 'dispatch;
	}
	// 824DC6B4: 48056505  bl 0x82532bb8
	ctx.lr = 0x824DC6B8;
	sub_82532BB8(ctx, base);
	// 824DC6B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824DC6BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824DC6C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824DC6C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824DC6C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824DC6CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC6D0 size=20
    let mut pc: u32 = 0x824DC6D0;
    'dispatch: loop {
        match pc {
            0x824DC6D0 => {
    //   block [0x824DC6D0..0x824DC6E4)
	// 824DC6D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC6D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824DC6D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC6DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DC6E0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC6E8 size=12
    let mut pc: u32 = 0x824DC6E8;
    'dispatch: loop {
        match pc {
            0x824DC6E8 => {
    //   block [0x824DC6E8..0x824DC6F4)
	// 824DC6E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824DC6EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824DC6F0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC6F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC6F4 size=8
    let mut pc: u32 = 0x824DC6F4;
    'dispatch: loop {
        match pc {
            0x824DC6F4 => {
    //   block [0x824DC6F4..0x824DC6FC)
	// 824DC6F4: 4800003C  b 0x824dc730
	sub_824DC730(ctx, base);
	return;
	// 824DC6F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DC700 size=44
    let mut pc: u32 = 0x824DC700;
    'dispatch: loop {
        match pc {
            0x824DC700 => {
    //   block [0x824DC700..0x824DC72C)
	// 824DC700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DC704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824DC708: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DC70C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824DC710: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824DC714: 4800001D  bl 0x824dc730
	ctx.lr = 0x824DC718;
	sub_824DC730(ctx, base);
	// 824DC718: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824DC71C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824DC720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824DC724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824DC728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DC730 size=132
    let mut pc: u32 = 0x824DC730;
    'dispatch: loop {
        match pc {
            0x824DC730 => {
    //   block [0x824DC730..0x824DC7B4)
	// 824DC730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DC734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824DC738: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824DC73C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DC740: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824DC744: 4802207D  bl 0x824fe7c0
	ctx.lr = 0x824DC748;
	sub_824FE7C0(ctx, base);
	// 824DC748: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DC74C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824DC750: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824DC754: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824DC758: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 824DC75C: 3CC08206  lis r6, -0x7dfa
	ctx.r[6].s64 = -2113536000;
	// 824DC760: 3CA08206  lis r5, -0x7dfa
	ctx.r[5].s64 = -2113536000;
	// 824DC764: 396B1C0C  addi r11, r11, 0x1c0c
	ctx.r[11].s64 = ctx.r[11].s64 + 7180;
	// 824DC768: 394A1C64  addi r10, r10, 0x1c64
	ctx.r[10].s64 = ctx.r[10].s64 + 7268;
	// 824DC76C: 39291C58  addi r9, r9, 0x1c58
	ctx.r[9].s64 = ctx.r[9].s64 + 7256;
	// 824DC770: 39081C44  addi r8, r8, 0x1c44
	ctx.r[8].s64 = ctx.r[8].s64 + 7236;
	// 824DC774: 38E71C38  addi r7, r7, 0x1c38
	ctx.r[7].s64 = ctx.r[7].s64 + 7224;
	// 824DC778: 38C61C2C  addi r6, r6, 0x1c2c
	ctx.r[6].s64 = ctx.r[6].s64 + 7212;
	// 824DC77C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 824DC780: 38A51C1C  addi r5, r5, 0x1c1c
	ctx.r[5].s64 = ctx.r[5].s64 + 7196;
	// 824DC784: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824DC788: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824DC78C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824DC790: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824DC794: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 824DC798: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 824DC79C: 90BF0030  stw r5, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[5].u32 ) };
	// 824DC7A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824DC7A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824DC7A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824DC7AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824DC7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DC7B8 size=100
    let mut pc: u32 = 0x824DC7B8;
    'dispatch: loop {
        match pc {
            0x824DC7B8 => {
    //   block [0x824DC7B8..0x824DC81C)
	// 824DC7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DC7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824DC7C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824DC7C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824DC7C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DC7CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824DC7D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824DC7D4: 480033FD  bl 0x824dfbd0
	ctx.lr = 0x824DC7D8;
	sub_824DFBD0(ctx, base);
	// 824DC7D8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824DC7DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DC7E0: 419A0020  beq cr6, 0x824dc800
	if ctx.cr[6].eq {
	pc = 0x824DC800; continue 'dispatch;
	}
	// 824DC7E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC7E8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824DC7EC: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824DC7F0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DC7F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824DC7F8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824DC7FC: 4BF878BD  bl 0x824640b8
	ctx.lr = 0x824DC800;
	sub_824640B8(ctx, base);
	// 824DC800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824DC804: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824DC808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824DC80C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824DC810: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824DC814: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824DC818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC820 size=8
    let mut pc: u32 = 0x824DC820;
    'dispatch: loop {
        match pc {
            0x824DC820 => {
    //   block [0x824DC820..0x824DC828)
	// 824DC820: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 824DC824: 4BFFFF94  b 0x824dc7b8
	sub_824DC7B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC828 size=8
    let mut pc: u32 = 0x824DC828;
    'dispatch: loop {
        match pc {
            0x824DC828 => {
    //   block [0x824DC828..0x824DC830)
	// 824DC828: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 824DC82C: 4BFFFF8C  b 0x824dc7b8
	sub_824DC7B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC830 size=8
    let mut pc: u32 = 0x824DC830;
    'dispatch: loop {
        match pc {
            0x824DC830 => {
    //   block [0x824DC830..0x824DC838)
	// 824DC830: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 824DC834: 4BFFFF84  b 0x824dc7b8
	sub_824DC7B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC838 size=8
    let mut pc: u32 = 0x824DC838;
    'dispatch: loop {
        match pc {
            0x824DC838 => {
    //   block [0x824DC838..0x824DC840)
	// 824DC838: 3863FFD0  addi r3, r3, -0x30
	ctx.r[3].s64 = ctx.r[3].s64 + -48;
	// 824DC83C: 4BFFFF7C  b 0x824dc7b8
	sub_824DC7B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC840 size=8
    let mut pc: u32 = 0x824DC840;
    'dispatch: loop {
        match pc {
            0x824DC840 => {
    //   block [0x824DC840..0x824DC848)
	// 824DC840: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 824DC844: 4BFFFF74  b 0x824dc7b8
	sub_824DC7B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC848 size=8
    let mut pc: u32 = 0x824DC848;
    'dispatch: loop {
        match pc {
            0x824DC848 => {
    //   block [0x824DC848..0x824DC850)
	// 824DC848: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824DC84C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC850 size=24
    let mut pc: u32 = 0x824DC850;
    'dispatch: loop {
        match pc {
            0x824DC850 => {
    //   block [0x824DC850..0x824DC868)
	// 824DC850: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DC854: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824DC858: 396B1CD4  addi r11, r11, 0x1cd4
	ctx.r[11].s64 = ctx.r[11].s64 + 7380;
	// 824DC85C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824DC860: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824DC864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC868 size=20
    let mut pc: u32 = 0x824DC868;
    'dispatch: loop {
        match pc {
            0x824DC868 => {
    //   block [0x824DC868..0x824DC87C)
	// 824DC868: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC86C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824DC870: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC874: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DC878: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC880 size=12
    let mut pc: u32 = 0x824DC880;
    'dispatch: loop {
        match pc {
            0x824DC880 => {
    //   block [0x824DC880..0x824DC88C)
	// 824DC880: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DC884: 386B1CD4  addi r3, r11, 0x1cd4
	ctx.r[3].s64 = ctx.r[11].s64 + 7380;
	// 824DC888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC890 size=20
    let mut pc: u32 = 0x824DC890;
    'dispatch: loop {
        match pc {
            0x824DC890 => {
    //   block [0x824DC890..0x824DC8A4)
	// 824DC890: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC894: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824DC898: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC89C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DC8A0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC8A8 size=8
    let mut pc: u32 = 0x824DC8A8;
    'dispatch: loop {
        match pc {
            0x824DC8A8 => {
    //   block [0x824DC8A8..0x824DC8B0)
	// 824DC8A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824DC8AC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC8B0 size=24
    let mut pc: u32 = 0x824DC8B0;
    'dispatch: loop {
        match pc {
            0x824DC8B0 => {
    //   block [0x824DC8B0..0x824DC8C8)
	// 824DC8B0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DC8B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824DC8B8: 396B1D28  addi r11, r11, 0x1d28
	ctx.r[11].s64 = ctx.r[11].s64 + 7464;
	// 824DC8BC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824DC8C0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824DC8C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC8C8 size=12
    let mut pc: u32 = 0x824DC8C8;
    'dispatch: loop {
        match pc {
            0x824DC8C8 => {
    //   block [0x824DC8C8..0x824DC8D4)
	// 824DC8C8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DC8CC: 386B1D28  addi r3, r11, 0x1d28
	ctx.r[3].s64 = ctx.r[11].s64 + 7464;
	// 824DC8D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DC8D8 size=100
    let mut pc: u32 = 0x824DC8D8;
    'dispatch: loop {
        match pc {
            0x824DC8D8 => {
    //   block [0x824DC8D8..0x824DC93C)
	// 824DC8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DC8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824DC8E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824DC8E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824DC8E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DC8EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824DC8F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824DC8F4: 48003F85  bl 0x824e0878
	ctx.lr = 0x824DC8F8;
	sub_824E0878(ctx, base);
	// 824DC8F8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824DC8FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DC900: 419A0020  beq cr6, 0x824dc920
	if ctx.cr[6].eq {
	pc = 0x824DC920; continue 'dispatch;
	}
	// 824DC904: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC908: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824DC90C: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 824DC910: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DC914: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824DC918: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824DC91C: 4BF8779D  bl 0x824640b8
	ctx.lr = 0x824DC920;
	sub_824640B8(ctx, base);
	// 824DC920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824DC924: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824DC928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824DC92C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824DC930: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824DC934: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824DC938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC940 size=8
    let mut pc: u32 = 0x824DC940;
    'dispatch: loop {
        match pc {
            0x824DC940 => {
    //   block [0x824DC940..0x824DC948)
	// 824DC940: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824DC944: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC948 size=24
    let mut pc: u32 = 0x824DC948;
    'dispatch: loop {
        match pc {
            0x824DC948 => {
    //   block [0x824DC948..0x824DC960)
	// 824DC948: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DC94C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824DC950: 396B1DFC  addi r11, r11, 0x1dfc
	ctx.r[11].s64 = ctx.r[11].s64 + 7676;
	// 824DC954: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824DC958: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824DC95C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC960 size=20
    let mut pc: u32 = 0x824DC960;
    'dispatch: loop {
        match pc {
            0x824DC960 => {
    //   block [0x824DC960..0x824DC974)
	// 824DC960: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC964: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824DC968: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC96C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DC970: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC978 size=12
    let mut pc: u32 = 0x824DC978;
    'dispatch: loop {
        match pc {
            0x824DC978 => {
    //   block [0x824DC978..0x824DC984)
	// 824DC978: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DC97C: 386B1DFC  addi r3, r11, 0x1dfc
	ctx.r[3].s64 = ctx.r[11].s64 + 7676;
	// 824DC980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DC988 size=100
    let mut pc: u32 = 0x824DC988;
    'dispatch: loop {
        match pc {
            0x824DC988 => {
    //   block [0x824DC988..0x824DC9EC)
	// 824DC988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DC98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824DC990: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824DC994: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824DC998: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DC99C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824DC9A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824DC9A4: 480D1E95  bl 0x825ae838
	ctx.lr = 0x824DC9A8;
	sub_825AE838(ctx, base);
	// 824DC9A8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824DC9AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DC9B0: 419A0020  beq cr6, 0x824dc9d0
	if ctx.cr[6].eq {
	pc = 0x824DC9D0; continue 'dispatch;
	}
	// 824DC9B4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC9B8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824DC9BC: 38C00029  li r6, 0x29
	ctx.r[6].s64 = 41;
	// 824DC9C0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DC9C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824DC9C8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824DC9CC: 4BF876ED  bl 0x824640b8
	ctx.lr = 0x824DC9D0;
	sub_824640B8(ctx, base);
	// 824DC9D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824DC9D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824DC9D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824DC9DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824DC9E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824DC9E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824DC9E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DC9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DC9F0 size=20
    let mut pc: u32 = 0x824DC9F0;
    'dispatch: loop {
        match pc {
            0x824DC9F0 => {
    //   block [0x824DC9F0..0x824DCA04)
	// 824DC9F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC9F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824DC9F8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DC9FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DCA00: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DCA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DCA08 size=92
    let mut pc: u32 = 0x824DCA08;
    'dispatch: loop {
        match pc {
            0x824DCA08 => {
    //   block [0x824DCA08..0x824DCA64)
	// 824DCA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DCA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824DCA10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824DCA14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DCA18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824DCA1C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824DCA20: 419A0030  beq cr6, 0x824dca50
	if ctx.cr[6].eq {
	pc = 0x824DCA50; continue 'dispatch;
	}
	// 824DCA24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824DCA28: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824DCA2C: 480D1FD5  bl 0x825aea00
	ctx.lr = 0x824DCA30;
	sub_825AEA00(ctx, base);
	// 824DCA30: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DCA34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824DCA38: 396B1EE4  addi r11, r11, 0x1ee4
	ctx.r[11].s64 = ctx.r[11].s64 + 7908;
	// 824DCA3C: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 824DCA40: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824DCA44: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 824DCA48: 915F0058  stw r10, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 824DCA4C: 913F005C  stw r9, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 824DCA50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824DCA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824DCA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824DCA5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824DCA60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DCA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DCA68 size=52
    let mut pc: u32 = 0x824DCA68;
    'dispatch: loop {
        match pc {
            0x824DCA68 => {
    //   block [0x824DCA68..0x824DCA9C)
	// 824DCA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DCA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824DCA70: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DCA74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824DCA78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824DCA7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824DCA80: 480D1F81  bl 0x825aea00
	ctx.lr = 0x824DCA84;
	sub_825AEA00(ctx, base);
	// 824DCA84: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DCA88: 386B1EE4  addi r3, r11, 0x1ee4
	ctx.r[3].s64 = ctx.r[11].s64 + 7908;
	// 824DCA8C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 824DCA90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824DCA94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824DCA98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DCAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DCAA0 size=148
    let mut pc: u32 = 0x824DCAA0;
    'dispatch: loop {
        match pc {
            0x824DCAA0 => {
    //   block [0x824DCAA0..0x824DCB34)
	// 824DCAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DCAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824DCAA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824DCAAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824DCAB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DCAB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824DCAB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824DCABC: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 824DCAC0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DCAC4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DCAC8: 409A0020  bne cr6, 0x824dcae8
	if !ctx.cr[6].eq {
	pc = 0x824DCAE8; continue 'dispatch;
	}
	// 824DCACC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DCAD0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824DCAD4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DCAD8: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 824DCADC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DCAE0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824DCAE4: 4BF875D5  bl 0x824640b8
	ctx.lr = 0x824DCAE8;
	sub_824640B8(ctx, base);
	// 824DCAE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824DCAEC: 480D1F7D  bl 0x825aea68
	ctx.lr = 0x824DCAF0;
	sub_825AEA68(ctx, base);
	// 824DCAF0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824DCAF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DCAF8: 419A0020  beq cr6, 0x824dcb18
	if ctx.cr[6].eq {
	pc = 0x824DCB18; continue 'dispatch;
	}
	// 824DCAFC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DCB00: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824DCB04: 38C00029  li r6, 0x29
	ctx.r[6].s64 = 41;
	// 824DCB08: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DCB0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824DCB10: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824DCB14: 4BF875A5  bl 0x824640b8
	ctx.lr = 0x824DCB18;
	sub_824640B8(ctx, base);
	// 824DCB18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824DCB1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824DCB20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824DCB24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824DCB28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824DCB2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824DCB30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DCB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DCB38 size=20
    let mut pc: u32 = 0x824DCB38;
    'dispatch: loop {
        match pc {
            0x824DCB38 => {
    //   block [0x824DCB38..0x824DCB4C)
	// 824DCB38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DCB3C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824DCB40: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DCB44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DCB48: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DCB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DCB50 size=8
    let mut pc: u32 = 0x824DCB50;
    'dispatch: loop {
        match pc {
            0x824DCB50 => {
    //   block [0x824DCB50..0x824DCB58)
	// 824DCB50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824DCB54: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DCB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DCB58 size=24
    let mut pc: u32 = 0x824DCB58;
    'dispatch: loop {
        match pc {
            0x824DCB58 => {
    //   block [0x824DCB58..0x824DCB70)
	// 824DCB58: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DCB5C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824DCB60: 396B1F20  addi r11, r11, 0x1f20
	ctx.r[11].s64 = ctx.r[11].s64 + 7968;
	// 824DCB64: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824DCB68: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824DCB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DCB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DCB70 size=12
    let mut pc: u32 = 0x824DCB70;
    'dispatch: loop {
        match pc {
            0x824DCB70 => {
    //   block [0x824DCB70..0x824DCB7C)
	// 824DCB70: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DCB74: 386B1F20  addi r3, r11, 0x1f20
	ctx.r[3].s64 = ctx.r[11].s64 + 7968;
	// 824DCB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DCB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DCB80 size=100
    let mut pc: u32 = 0x824DCB80;
    'dispatch: loop {
        match pc {
            0x824DCB80 => {
    //   block [0x824DCB80..0x824DCBE4)
	// 824DCB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DCB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824DCB88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824DCB8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824DCB90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DCB94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824DCB98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824DCB9C: 48005505  bl 0x824e20a0
	ctx.lr = 0x824DCBA0;
	sub_824E20A0(ctx, base);
	// 824DCBA0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824DCBA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DCBA8: 419A0020  beq cr6, 0x824dcbc8
	if ctx.cr[6].eq {
	pc = 0x824DCBC8; continue 'dispatch;
	}
	// 824DCBAC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DCBB0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824DCBB4: 38C0002F  li r6, 0x2f
	ctx.r[6].s64 = 47;
	// 824DCBB8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DCBBC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824DCBC0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824DCBC4: 4BF874F5  bl 0x824640b8
	ctx.lr = 0x824DCBC8;
	sub_824640B8(ctx, base);
	// 824DCBC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824DCBCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824DCBD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824DCBD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824DCBD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824DCBDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824DCBE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DCBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DCBE8 size=4
    let mut pc: u32 = 0x824DCBE8;
    'dispatch: loop {
        match pc {
            0x824DCBE8 => {
    //   block [0x824DCBE8..0x824DCBEC)
	// 824DCBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DCBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824DCBF0 size=104
    let mut pc: u32 = 0x824DCBF0;
    'dispatch: loop {
        match pc {
            0x824DCBF0 => {
    //   block [0x824DCBF0..0x824DCC58)
	// 824DCBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DCBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824DCBF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824DCBFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DCC00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824DCC04: 480D1BA5  bl 0x825ae7a8
	ctx.lr = 0x824DCC08;
	sub_825AE7A8(ctx, base);
	// 824DCC08: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DCC0C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824DCC10: 396B15F0  addi r11, r11, 0x15f0
	ctx.r[11].s64 = ctx.r[11].s64 + 5616;
	// 824DCC14: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 824DCC18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824DCC1C: C00A8E30  lfs f0, -0x71d0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824DCC20: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824DCC24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824DCC28: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 824DCC2C: D01F0030  stfs f0, 0x30(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 824DCC30: 396B9FA0  addi r11, r11, -0x6060
	ctx.r[11].s64 = ctx.r[11].s64 + -24672;
	// 824DCC34: C1AA20B0  lfs f13, 0x20b0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8368 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824DCC38: D1BF0034  stfs f13, 0x34(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DCC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DCC58 size=164
    let mut pc: u32 = 0x824DCC58;
    'dispatch: loop {
        match pc {
            0x824DCC58 => {
    //   block [0x824DCC58..0x824DCCFC)
	// 824DCC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DCC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824DCC60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824DCC64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824DCC68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DCC6C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824DCC70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824DCC74: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DCC78: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824DCC7C: 419A000C  beq cr6, 0x824dcc88
	if ctx.cr[6].eq {
	pc = 0x824DCC88; continue 'dispatch;
	}
	// 824DCC80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824DCC84: 48000060  b 0x824dcce4
	pc = 0x824DCCE4; continue 'dispatch;
	// 824DCC88: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DCC8C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DCC90: 409AFFF0  bne cr6, 0x824dcc80
	if !ctx.cr[6].eq {
	pc = 0x824DCC80; continue 'dispatch;
	}
	// 824DCC94: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DCC98: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824DCC9C: 38A00029  li r5, 0x29
	ctx.r[5].s64 = 41;
	// 824DCCA0: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 824DCCA4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824DCCA8: 4BF87391  bl 0x82464038
	ctx.lr = 0x824DCCAC;
	sub_82464038(ctx, base);
	// 824DCCAC: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 824DCCB0: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 824DCCB4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DCCB8: 80DF0010  lwz r6, 0x10(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824DCCBC: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DCCC0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DCCC4: 4BFFFF2D  bl 0x824dcbf0
	ctx.lr = 0x824DCCC8;
	sub_824DCBF0(ctx, base);
	// 824DCCC8: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DCD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DCD00 size=576
    let mut pc: u32 = 0x824DCD00;
    'dispatch: loop {
        match pc {
            0x824DCD00 => {
    //   block [0x824DCD00..0x824DCF40)
	// 824DCD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DCD04: 480583B9  bl 0x825350bc
	ctx.lr = 0x824DCD08;
	sub_82535080(ctx, base);
	// 824DCD08: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 824DCD0C: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DCF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DCF40 size=380
    let mut pc: u32 = 0x824DCF40;
    'dispatch: loop {
        match pc {
            0x824DCF40 => {
    //   block [0x824DCF40..0x824DD0BC)
	// 824DCF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DCF44: 48058175  bl 0x825350b8
	ctx.lr = 0x824DCF48;
	sub_82535080(ctx, base);
	// 824DCF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DCF4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824DCF50: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824DCF54: 556B17FE  rlwinm r11, r11, 2, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824DCF58: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DCF5C: 409A0100  bne cr6, 0x824dd05c
	if !ctx.cr[6].eq {
	pc = 0x824DD05C; continue 'dispatch;
	}
	// 824DCF60: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DCF64: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824DCF68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DCF6C: 4099006C  ble cr6, 0x824dcfd8
	if !ctx.cr[6].gt {
	pc = 0x824DCFD8; continue 'dispatch;
	}
	// 824DCF70: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824DCF74: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DCF78: 7CBF582E  lwzx r5, r31, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824DCF7C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 824DCF80: 419A0044  beq cr6, 0x824dcfc4
	if ctx.cr[6].eq {
	pc = 0x824DCFC4; continue 'dispatch;
	}
	// 824DCF84: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DCF88: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824DCF8C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824DCF90: 8163009C  lwz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 824DCF94: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 824DCF98: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824DCF9C: 41980010  blt cr6, 0x824dcfac
	if ctx.cr[6].lt {
	pc = 0x824DCFAC; continue 'dispatch;
	}
	// 824DCFA0: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 824DCFA4: 4BF86F75  bl 0x82463f18
	ctx.lr = 0x824DCFA8;
	sub_82463F18(ctx, base);
	// 824DCFA8: 4800001C  b 0x824dcfc4
	pc = 0x824DCFC4; continue 'dispatch;
	// 824DCFAC: 8163009C  lwz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 824DCFB0: 81430098  lwz r10, 0x98(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 824DCFB4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824DCFB8: 9163009C  stw r11, 0x9c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 824DCFBC: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824DCFC0: 90A30098  stw r5, 0x98(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[5].u32 ) };
	// 824DCFC4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DCFC8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824DCFCC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824DCFD0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824DCFD4: 4198FFA0  blt cr6, 0x824dcf74
	if ctx.cr[6].lt {
	pc = 0x824DCF74; continue 'dispatch;
	}
	// 824DCFD8: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 824DCFDC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824DCFE0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DCFE4: 40990078  ble cr6, 0x824dd05c
	if !ctx.cr[6].gt {
	pc = 0x824DD05C; continue 'dispatch;
	}
	// 824DCFE8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824DCFEC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 824DCFF0: 7FFD582E  lwzx r31, r29, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824DCFF4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824DCFF8: 419A0050  beq cr6, 0x824dd048
	if ctx.cr[6].eq {
	pc = 0x824DD048; continue 'dispatch;
	}
	// 824DCFFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824DD000: 4BFFFF41  bl 0x824dcf40
	ctx.lr = 0x824DD004;
	sub_824DCF40(ctx, base);
	// 824DD004: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD008: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824DD00C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824DD010: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 824DD014: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 824DD018: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824DD01C: 41980014  blt cr6, 0x824dd030
	if ctx.cr[6].lt {
	pc = 0x824DD030; continue 'dispatch;
	}
	// 824DD020: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824DD024: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 824DD028: 4BF86EF1  bl 0x82463f18
	ctx.lr = 0x824DD02C;
	sub_82463F18(ctx, base);
	// 824DD02C: 4800001C  b 0x824dd048
	pc = 0x824DD048; continue 'dispatch;
	// 824DD030: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 824DD034: 81430050  lwz r10, 0x50(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 824DD038: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824DD03C: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824DD040: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824DD044: 93E30050  stw r31, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 824DD048: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 824DD04C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 824DD050: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 824DD054: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824DD058: 4198FF94  blt cr6, 0x824dcfec
	if ctx.cr[6].lt {
	pc = 0x824DCFEC; continue 'dispatch;
	}
	// 824DD05C: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 824DD060: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DD064: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DD068: 409A0020  bne cr6, 0x824dd088
	if !ctx.cr[6].eq {
	pc = 0x824DD088; continue 'dispatch;
	}
	// 824DD06C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD070: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824DD074: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DD078: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 824DD07C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DD080: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824DD084: 4BF87035  bl 0x824640b8
	ctx.lr = 0x824DD088;
	sub_824640B8(ctx, base);
	// 824DD088: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824DD08C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DD090: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DD094: 409A0020  bne cr6, 0x824dd0b4
	if !ctx.cr[6].eq {
	pc = 0x824DD0B4; continue 'dispatch;
	}
	// 824DD098: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD09C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824DD0A0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DD0A4: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD0A8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DD0AC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824DD0B0: 4BF87009  bl 0x824640b8
	ctx.lr = 0x824DD0B4;
	sub_824640B8(ctx, base);
	// 824DD0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824DD0B8: 48058050  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DD0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DD0C0 size=236
    let mut pc: u32 = 0x824DD0C0;
    'dispatch: loop {
        match pc {
            0x824DD0C0 => {
    //   block [0x824DD0C0..0x824DD1AC)
	// 824DD0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DD0C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824DD0C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824DD0CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DD0D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824DD0D4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DD0D8: 396B16D0  addi r11, r11, 0x16d0
	ctx.r[11].s64 = ctx.r[11].s64 + 5840;
	// 824DD0DC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824DD0E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824DD0E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824DD0E8: 419A0008  beq cr6, 0x824dd0f0
	if ctx.cr[6].eq {
	pc = 0x824DD0F0; continue 'dispatch;
	}
	// 824DD0EC: 4BFBE965  bl 0x8249ba50
	ctx.lr = 0x824DD0F0;
	sub_8249BA50(ctx, base);
	// 824DD0F0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824DD0F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824DD0F8: 419A0008  beq cr6, 0x824dd100
	if ctx.cr[6].eq {
	pc = 0x824DD100; continue 'dispatch;
	}
	// 824DD0FC: 4BFBE955  bl 0x8249ba50
	ctx.lr = 0x824DD100;
	sub_8249BA50(ctx, base);
	// 824DD100: 387F00F4  addi r3, r31, 0xf4
	ctx.r[3].s64 = ctx.r[31].s64 + 244;
	// 824DD104: 4BFFFE3D  bl 0x824dcf40
	ctx.lr = 0x824DD108;
	sub_824DCF40(ctx, base);
	// 824DD108: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 824DD10C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DD110: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DD114: 409A0020  bne cr6, 0x824dd134
	if !ctx.cr[6].eq {
	pc = 0x824DD134; continue 'dispatch;
	}
	// 824DD118: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD11C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824DD120: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DD124: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 824DD128: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824DD12C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824DD130: 4BF86F89  bl 0x824640b8
	ctx.lr = 0x824DD134;
	sub_824640B8(ctx, base);
	// 824DD134: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 824DD138: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DD13C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DD140: 409A0020  bne cr6, 0x824dd160
	if !ctx.cr[6].eq {
	pc = 0x824DD160; continue 'dispatch;
	}
	// 824DD144: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD148: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824DD14C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DD150: 809F005C  lwz r4, 0x5c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 824DD154: 55652834  slwi r5, r11, 5
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DD158: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824DD15C: 4BF86F5D  bl 0x824640b8
	ctx.lr = 0x824DD160;
	sub_824640B8(ctx, base);
	// 824DD160: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 824DD164: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DD168: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DD16C: 409A0020  bne cr6, 0x824dd18c
	if !ctx.cr[6].eq {
	pc = 0x824DD18C; continue 'dispatch;
	}
	// 824DD170: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD174: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824DD178: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DD17C: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 824DD180: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824DD184: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824DD188: 4BF86F31  bl 0x824640b8
	ctx.lr = 0x824DD18C;
	sub_824640B8(ctx, base);
	// 824DD18C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824DD190: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824DD194: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824DD198: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824DD19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824DD1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824DD1A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824DD1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DD1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DD1B0 size=276
    let mut pc: u32 = 0x824DD1B0;
    'dispatch: loop {
        match pc {
            0x824DD1B0 => {
    //   block [0x824DD1B0..0x824DD2C4)
	// 824DD1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DD1B4: 48057EF9  bl 0x825350ac
	ctx.lr = 0x824DD1B8;
	sub_82535080(ctx, base);
	// 824DD1B8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DD1BC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824DD1C0: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD1C4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824DD1C8: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 824DD1CC: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 824DD1D0: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 824DD1D4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD1D8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 824DD1DC: 7F8B5214  add r28, r11, r10
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824DD1E0: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD1E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DD1E8: 409900D4  ble cr6, 0x824dd2bc
	if !ctx.cr[6].gt {
	pc = 0x824DD2BC; continue 'dispatch;
	}
	// 824DD1EC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD1F0: 815A0014  lwz r10, 0x14(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 824DD1F4: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 824DD1F8: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD1FC: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824DD200: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD204: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD208: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824DD20C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824DD210: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD214: 813E0018  lwz r9, 0x18(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 824DD218: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 824DD21C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 824DD220: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD224: 7C6B482E  lwzx r3, r11, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 824DD228: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824DD22C: 419A003C  beq cr6, 0x824dd268
	if ctx.cr[6].eq {
	pc = 0x824DD268; continue 'dispatch;
	}
	// 824DD230: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD234: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DD238: 419A0030  beq cr6, 0x824dd268
	if ctx.cr[6].eq {
	pc = 0x824DD268; continue 'dispatch;
	}
	// 824DD23C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824DD240: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824DD244: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824DD248: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DD24C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824DD250: 409A0018  bne cr6, 0x824dd268
	if !ctx.cr[6].eq {
	pc = 0x824DD268; continue 'dispatch;
	}
	// 824DD254: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD258: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824DD25C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD260: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DD264: 4E800421  bctrl
	ctx.lr = 0x824DD268;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DD268: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD26C: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 824DD270: 813E0018  lwz r9, 0x18(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 824DD274: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824DD278: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824DD27C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD280: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 824DD284: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 824DD288: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD28C: 7F2B492E  stwx r25, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[25].u32) };
	// 824DD290: 419A001C  beq cr6, 0x824dd2ac
	if ctx.cr[6].eq {
	pc = 0x824DD2AC; continue 'dispatch;
	}
	// 824DD294: A1790004  lhz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD298: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DD29C: 419A0010  beq cr6, 0x824dd2ac
	if ctx.cr[6].eq {
	pc = 0x824DD2AC; continue 'dispatch;
	}
	// 824DD2A0: A1790006  lhz r11, 6(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(6 as u32) ) } as u64;
	// 824DD2A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824DD2A8: B1790006  sth r11, 6(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824DD2AC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD2B0: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 824DD2B4: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824DD2B8: 4198FF34  blt cr6, 0x824dd1ec
	if ctx.cr[6].lt {
	pc = 0x824DD1EC; continue 'dispatch;
	}
	// 824DD2BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824DD2C0: 48057E3C  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DD2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DD2C8 size=36
    let mut pc: u32 = 0x824DD2C8;
    'dispatch: loop {
        match pc {
            0x824DD2C8 => {
    //   block [0x824DD2C8..0x824DD2EC)
	// 824DD2C8: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824DD2CC: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824DD2D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824DD2D4: 7D445214  add r10, r4, r10
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[10].u64;
	// 824DD2D8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824DD2DC: 7D0A4A14  add r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824DD2E0: 81480004  lwz r10, 4(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD2E4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DD2E8: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DD2EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DD2EC size=80
    let mut pc: u32 = 0x824DD2EC;
    'dispatch: loop {
        match pc {
            0x824DD2EC => {
    //   block [0x824DD2EC..0x824DD33C)
	// 824DD2EC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 824DD2F0: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DD340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DD340 size=212
    let mut pc: u32 = 0x824DD340;
    'dispatch: loop {
        match pc {
            0x824DD340 => {
    //   block [0x824DD340..0x824DD414)
	// 824DD340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DD344: 48057D6D  bl 0x825350b0
	ctx.lr = 0x824DD348;
	sub_82535080(ctx, base);
	// 824DD348: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DD34C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824DD350: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD354: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 824DD358: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 824DD35C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 824DD360: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 824DD364: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD368: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824DD36C: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824DD370: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD374: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DD378: 40990094  ble cr6, 0x824dd40c
	if !ctx.cr[6].gt {
	pc = 0x824DD40C; continue 'dispatch;
	}
	// 824DD37C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD380: 815C0014  lwz r10, 0x14(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 824DD384: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 824DD388: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD38C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824DD390: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD394: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD398: 812B0018  lwz r9, 0x18(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824DD39C: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD3A0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824DD3A4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD3A8: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 824DD3AC: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 824DD3B0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD3B4: 7FEB482E  lwzx r31, r11, r9
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 824DD3B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824DD3BC: 419A0040  beq cr6, 0x824dd3fc
	if ctx.cr[6].eq {
	pc = 0x824DD3FC; continue 'dispatch;
	}
	// 824DD3C0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 824DD3C4: 815A0004  lwz r10, 4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD3C8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824DD3CC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824DD3D0: 409A0010  bne cr6, 0x824dd3e0
	if !ctx.cr[6].eq {
	pc = 0x824DD3E0; continue 'dispatch;
	}
	// 824DD3D4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824DD3D8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824DD3DC: 4BF90F75  bl 0x8246e350
	ctx.lr = 0x824DD3E0;
	sub_8246E350(ctx, base);
	// 824DD3E0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD3E4: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD3E8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD3EC: 7FEB512E  stwx r31, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 824DD3F0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD3F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824DD3F8: 917A0004  stw r11, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824DD3FC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD400: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824DD404: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824DD408: 4198FF74  blt cr6, 0x824dd37c
	if ctx.cr[6].lt {
	pc = 0x824DD37C; continue 'dispatch;
	}
	// 824DD40C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824DD410: 48057CF0  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DD418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DD418 size=2984
    let mut pc: u32 = 0x824DD418;
    'dispatch: loop {
        match pc {
            0x824DD418 => {
    //   block [0x824DD418..0x824DDFC0)
	// 824DD418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DD41C: 48057C65  bl 0x82535080
	ctx.lr = 0x824DD420;
	sub_82535080(ctx, base);
	// 824DD420: 9421FC70  stwu r1, -0x390(r1)
	ea = ctx.r[1].u32.wrapping_add(-912 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DD424: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD428: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 824DD42C: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824DD430: B06103A6  sth r3, 0x3a6(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(934 as u32), ctx.r[3].u16 ) };
	// 824DD434: 7C8E2378  mr r14, r4
	ctx.r[14].u64 = ctx.r[4].u64;
	// 824DD438: 7CB02B78  mr r16, r5
	ctx.r[16].u64 = ctx.r[5].u64;
	// 824DD43C: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 824DD440: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 824DD444: 934103BC  stw r26, 0x3bc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(956 as u32), ctx.r[26].u32 ) };
	// 824DD448: 7C6BC02E  lwzx r3, r11, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 824DD44C: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 824DD450: 4BF86BE9  bl 0x82464038
	ctx.lr = 0x824DD454;
	sub_82464038(ctx, base);
	// 824DD454: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 824DD458: 3940002C  li r10, 0x2c
	ctx.r[10].s64 = 44;
	// 824DD45C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DD460: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824DD464: 396B184C  addi r11, r11, 0x184c
	ctx.r[11].s64 = ctx.r[11].s64 + 6220;
	// 824DD468: 3EC08000  lis r22, -0x8000
	ctx.r[22].s64 = -2147483648;
	// 824DD46C: B1550004  sth r10, 4(r21)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[21].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 824DD470: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824DD474: 3B350008  addi r25, r21, 8
	ctx.r[25].s64 = ctx.r[21].s64 + 8;
	// 824DD478: 3A550014  addi r18, r21, 0x14
	ctx.r[18].s64 = ctx.r[21].s64 + 20;
	// 824DD47C: 3AF50020  addi r23, r21, 0x20
	ctx.r[23].s64 = ctx.r[21].s64 + 32;
	// 824DD480: 91750000  stw r11, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824DD484: B1550006  sth r10, 6(r21)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[21].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824DD488: 93B90000  stw r29, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 824DD48C: 93B90004  stw r29, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 824DD490: 92D90008  stw r22, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 824DD494: 93B20000  stw r29, 0(r18)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[18].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 824DD498: 93B20004  stw r29, 4(r18)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[18].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 824DD49C: 92D20008  stw r22, 8(r18)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[18].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 824DD4A0: 93B70000  stw r29, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 824DD4A4: 93B70004  stw r29, 4(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 824DD4A8: 92D70008  stw r22, 8(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 824DD4AC: 83EE0004  lwz r31, 4(r14)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD4B0: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 824DD4B4: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 824DD4B8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824DD4BC: 92C10060  stw r22, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[22].u32 ) };
	// 824DD4C0: 93A10078  stw r29, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 824DD4C4: 93A1007C  stw r29, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[29].u32 ) };
	// 824DD4C8: 92C10080  stw r22, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[22].u32 ) };
	// 824DD4CC: 4099001C  ble cr6, 0x824dd4e8
	if !ctx.cr[6].gt {
	pc = 0x824DD4E8; continue 'dispatch;
	}
	// 824DD4D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824DD4D4: 41980008  blt cr6, 0x824dd4dc
	if ctx.cr[6].lt {
	pc = 0x824DD4DC; continue 'dispatch;
	}
	// 824DD4D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824DD4DC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 824DD4E0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 824DD4E4: 4BF90DE5  bl 0x8246e2c8
	ctx.lr = 0x824DD4E8;
	sub_8246E2C8(ctx, base);
	// 824DD4E8: 57E5103A  slwi r5, r31, 2
	ctx.r[5].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DD4EC: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 824DD4F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824DD4F4: 93E1007C  stw r31, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[31].u32 ) };
	// 824DD4F8: 4BF8CE41  bl 0x8246a338
	ctx.lr = 0x824DD4FC;
	sub_8246A338(ctx, base);
	// 824DD4FC: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 824DD500: 4BF8FE41  bl 0x8246d340
	ctx.lr = 0x824DD504;
	sub_8246D340(ctx, base);
	// 824DD504: 816E0004  lwz r11, 4(r14)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD508: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 824DD50C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DD510: 40990030  ble cr6, 0x824dd540
	if !ctx.cr[6].gt {
	pc = 0x824DD540; continue 'dispatch;
	}
	// 824DD514: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 824DD518: 816E0000  lwz r11, 0(r14)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD51C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824DD520: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 824DD524: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824DD528: 4BF8FED9  bl 0x8246d400
	ctx.lr = 0x824DD52C;
	sub_8246D400(ctx, base);
	// 824DD52C: 816E0004  lwz r11, 4(r14)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD530: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 824DD534: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 824DD538: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824DD53C: 4198FFDC  blt cr6, 0x824dd518
	if ctx.cr[6].lt {
	pc = 0x824DD518; continue 'dispatch;
	}
	// 824DD540: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 824DD544: 83EE0004  lwz r31, 4(r14)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD548: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824DD54C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 824DD550: 40980024  bge cr6, 0x824dd574
	if !ctx.cr[6].lt {
	pc = 0x824DD574; continue 'dispatch;
	}
	// 824DD554: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD558: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824DD55C: 41980008  blt cr6, 0x824dd564
	if ctx.cr[6].lt {
	pc = 0x824DD564; continue 'dispatch;
	}
	// 824DD560: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 824DD564: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 824DD568: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824DD56C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 824DD570: 4BF90D59  bl 0x8246e2c8
	ctx.lr = 0x824DD574;
	sub_8246E2C8(ctx, base);
	// 824DD574: 93F90004  stw r31, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 824DD578: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824DD57C: 8175000C  lwz r11, 0xc(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(12 as u32) ) } as u64;
	// 824DD580: 80790000  lwz r3, 0(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD584: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824DD588: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824DD58C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DD590: 4BF8CDA9  bl 0x8246a338
	ctx.lr = 0x824DD594;
	sub_8246A338(ctx, base);
	// 824DD594: 81700004  lwz r11, 4(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD598: 7FB1EB78  mr r17, r29
	ctx.r[17].u64 = ctx.r[29].u64;
	// 824DD59C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DD5A0: 40990240  ble cr6, 0x824dd7e0
	if !ctx.cr[6].gt {
	pc = 0x824DD7E0; continue 'dispatch;
	}
	// 824DD5A4: 89E103A7  lbz r15, 0x3a7(r1)
	ctx.r[15].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(935 as u32) ) } as u64;
	// 824DD5A8: 7FB4EB78  mr r20, r29
	ctx.r[20].u64 = ctx.r[29].u64;
	// 824DD5AC: 8A6103A6  lbz r19, 0x3a6(r1)
	ctx.r[19].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(934 as u32) ) } as u64;
	// 824DD5B0: 81700000  lwz r11, 0(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD5B4: 38E10068  addi r7, r1, 0x68
	ctx.r[7].s64 = ctx.r[1].s64 + 104;
	// 824DD5B8: 38C10088  addi r6, r1, 0x88
	ctx.r[6].s64 = ctx.r[1].s64 + 136;
	// 824DD5BC: 93A10088  stw r29, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[29].u32 ) };
	// 824DD5C0: 7D745A14  add r11, r20, r11
	ctx.r[11].u64 = ctx.r[20].u64 + ctx.r[11].u64;
	// 824DD5C4: 93A1008C  stw r29, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[29].u32 ) };
	// 824DD5C8: 7DC37378  mr r3, r14
	ctx.r[3].u64 = ctx.r[14].u64;
	// 824DD5CC: 92C10090  stw r22, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[22].u32 ) };
	// 824DD5D0: 93A10068  stw r29, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 824DD5D4: 93A1006C  stw r29, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 824DD5D8: 92C10070  stw r22, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[22].u32 ) };
	// 824DD5DC: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD5E0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD5E4: 4800607D  bl 0x824e3660
	ctx.lr = 0x824DD5E8;
	sub_824E3660(ctx, base);
	// 824DD5E8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824DD5EC: 419A03F4  beq cr6, 0x824dd9e0
	if ctx.cr[6].eq {
	pc = 0x824DD9E0; continue 'dispatch;
	}
	// 824DD5F0: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 824DD5F4: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 824DD5F8: 48005811  bl 0x824e2e08
	ctx.lr = 0x824DD5FC;
	sub_824E2E08(ctx, base);
	// 824DD5FC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824DD600: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 824DD604: 419A0534  beq cr6, 0x824ddb38
	if ctx.cr[6].eq {
	pc = 0x824DDB38; continue 'dispatch;
	}
	// 824DD608: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 824DD60C: 81570004  lwz r10, 4(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD610: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824DD614: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824DD618: 409A0010  bne cr6, 0x824dd628
	if !ctx.cr[6].eq {
	pc = 0x824DD628; continue 'dispatch;
	}
	// 824DD61C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824DD620: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 824DD624: 4BF90D2D  bl 0x8246e350
	ctx.lr = 0x824DD628;
	sub_8246E350(ctx, base);
	// 824DD628: 81770004  lwz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD62C: 7FB8EB78  mr r24, r29
	ctx.r[24].u64 = ctx.r[29].u64;
	// 824DD630: 81570000  lwz r10, 0(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD634: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD638: 7F6B512E  stwx r27, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u32) };
	// 824DD63C: 81770004  lwz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD640: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824DD644: 91770004  stw r11, 4(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824DD648: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 824DD64C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DD650: 40990120  ble cr6, 0x824dd770
	if !ctx.cr[6].gt {
	pc = 0x824DD770; continue 'dispatch;
	}
	// 824DD654: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 824DD658: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 824DD65C: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 824DD660: 7C9C582E  lwzx r4, r28, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824DD664: 4BF8FE75  bl 0x8246d4d8
	ctx.lr = 0x824DD668;
	sub_8246D4D8(ctx, base);
	// 824DD668: 816100A0  lwz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 824DD66C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824DD670: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824DD674: 40990008  ble cr6, 0x824dd67c
	if !ctx.cr[6].gt {
	pc = 0x824DD67C; continue 'dispatch;
	}
	// 824DD678: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 824DD67C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 824DD680: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DD684: 419A060C  beq cr6, 0x824ddc90
	if ctx.cr[6].eq {
	pc = 0x824DDC90; continue 'dispatch;
	}
	// 824DD688: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824DD68C: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824DD690: 81390000  lwz r9, 0(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD694: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 824DD698: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 824DD69C: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 824DD6A0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824DD6A4: 550A103A  slwi r10, r8, 2
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824DD6A8: 81010098  lwz r8, 0x98(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 824DD6AC: 7FEA402E  lwzx r31, r10, r8
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 824DD6B0: 57EA083C  slwi r10, r31, 1
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824DD6B4: 7D5F5214  add r10, r31, r10
	ctx.r[10].u64 = ctx.r[31].u64 + ctx.r[10].u64;
	// 824DD6B8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824DD6BC: 7F4A4A14  add r26, r10, r9
	ctx.r[26].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824DD6C0: 409A0014  bne cr6, 0x824dd6d4
	if !ctx.cr[6].eq {
	pc = 0x824DD6D4; continue 'dispatch;
	}
	// 824DD6C4: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 824DD6C8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 824DD6CC: 4BF90C85  bl 0x8246e350
	ctx.lr = 0x824DD6D0;
	sub_8246E350(ctx, base);
	// 824DD6D0: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 824DD6D4: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 824DD6D8: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824DD6DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 824DD6E0: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824DD6E4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824DD6E8: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824DD6EC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD6F0: 7FCB4A14  add r30, r11, r9
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824DD6F4: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 824DD6F8: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 824DD6FC: 7D2A582E  lwzx r9, r10, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824DD700: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824DD704: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 824DD708: 807B000C  lwz r3, 0xc(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 824DD70C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD710: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824DD714: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DD718: 4E800421  bctrl
	ctx.lr = 0x824DD71C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DD71C: 2F030066  cmpwi cr6, r3, 0x66
	ctx.cr[6].compare_i32(ctx.r[3].s32, 102, &mut ctx.xer);
	// 824DD720: 409A0708  bne cr6, 0x824dde28
	if !ctx.cr[6].eq {
	pc = 0x824DDE28; continue 'dispatch;
	}
	// 824DD724: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 824DD728: 7E6A0774  extsb r10, r19
	ctx.r[10].s64 = ctx.r[19].s8 as i64;
	// 824DD72C: 931E0004  stw r24, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[24].u32 ) };
	// 824DD730: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DD734: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824DD738: 419A0020  beq cr6, 0x824dd758
	if ctx.cr[6].eq {
	pc = 0x824DD758; continue 'dispatch;
	}
	// 824DD73C: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 824DD740: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DD744: 409A0014  bne cr6, 0x824dd758
	if !ctx.cr[6].eq {
	pc = 0x824DD758; continue 'dispatch;
	}
	// 824DD748: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 824DD74C: 7C7C582E  lwzx r3, r28, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824DD750: 48004B89  bl 0x824e22d8
	ctx.lr = 0x824DD754;
	sub_824E22D8(ctx, base);
	// 824DD754: 907A0008  stw r3, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 824DD758: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 824DD75C: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 824DD760: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 824DD764: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824DD768: 4198FEF0  blt cr6, 0x824dd658
	if ctx.cr[6].lt {
	pc = 0x824DD658; continue 'dispatch;
	}
	// 824DD76C: 834103BC  lwz r26, 0x3bc(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(956 as u32) ) } as u64;
	// 824DD770: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 824DD774: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DD778: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DD77C: 409A0020  bne cr6, 0x824dd79c
	if !ctx.cr[6].eq {
	pc = 0x824DD79C; continue 'dispatch;
	}
	// 824DD780: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DD784: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824DD788: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824DD78C: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 824DD790: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DD794: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824DD798: 4BF86921  bl 0x824640b8
	ctx.lr = 0x824DD79C;
	sub_824640B8(ctx, base);
	// 824DD79C: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 824DD7A0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DD7A4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DD7A8: 409A0020  bne cr6, 0x824dd7c8
	if !ctx.cr[6].eq {
	pc = 0x824DD7C8; continue 'dispatch;
	}
	// 824DD7AC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DD7B0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824DD7B4: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824DD7B8: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 824DD7BC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DD7C0: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824DD7C4: 4BF868F5  bl 0x824640b8
	ctx.lr = 0x824DD7C8;
	sub_824640B8(ctx, base);
	// 824DD7C8: 81700004  lwz r11, 4(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD7CC: 3A310001  addi r17, r17, 1
	ctx.r[17].s64 = ctx.r[17].s64 + 1;
	// 824DD7D0: 83010050  lwz r24, 0x50(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824DD7D4: 3A940008  addi r20, r20, 8
	ctx.r[20].s64 = ctx.r[20].s64 + 8;
	// 824DD7D8: 7F115800  cmpw cr6, r17, r11
	ctx.cr[6].compare_i32(ctx.r[17].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824DD7DC: 4198FDD4  blt cr6, 0x824dd5b0
	if ctx.cr[6].lt {
	pc = 0x824DD5B0; continue 'dispatch;
	}
	// 824DD7E0: 8175000C  lwz r11, 0xc(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(12 as u32) ) } as u64;
	// 824DD7E4: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 824DD7E8: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 824DD7EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DD7F0: 40990044  ble cr6, 0x824dd834
	if !ctx.cr[6].gt {
	pc = 0x824DD834; continue 'dispatch;
	}
	// 824DD7F4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 824DD7F8: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 824DD7FC: 81190000  lwz r8, 0(r25)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD800: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824DD804: 7FEB412E  stwx r31, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[31].u32) };
	// 824DD808: 80F90000  lwz r7, 0(r25)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD80C: 81010078  lwz r8, 0x78(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 824DD810: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 824DD814: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 824DD818: 7D0A402E  lwzx r8, r10, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 824DD81C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824DD820: 93A70004  stw r29, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 824DD824: 7FE8FA14  add r31, r8, r31
	ctx.r[31].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 824DD828: 8115000C  lwz r8, 0xc(r21)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(12 as u32) ) } as u64;
	// 824DD82C: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 824DD830: 4198FFCC  blt cr6, 0x824dd7fc
	if ctx.cr[6].lt {
	pc = 0x824DD7FC; continue 'dispatch;
	}
	// 824DD834: 81720008  lwz r11, 8(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(8 as u32) ) } as u64;
	// 824DD838: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824DD83C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 824DD840: 40980024  bge cr6, 0x824dd864
	if !ctx.cr[6].lt {
	pc = 0x824DD864; continue 'dispatch;
	}
	// 824DD844: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD848: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824DD84C: 41980008  blt cr6, 0x824dd854
	if ctx.cr[6].lt {
	pc = 0x824DD854; continue 'dispatch;
	}
	// 824DD850: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 824DD854: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824DD858: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824DD85C: 7E439378  mr r3, r18
	ctx.r[3].u64 = ctx.r[18].u64;
	// 824DD860: 4BF90A69  bl 0x8246e2c8
	ctx.lr = 0x824DD864;
	sub_8246E2C8(ctx, base);
	// 824DD864: 93F20004  stw r31, 4(r18)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[18].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 824DD868: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 824DD86C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 824DD870: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DD874: 40990070  ble cr6, 0x824dd8e4
	if !ctx.cr[6].gt {
	pc = 0x824DD8E4; continue 'dispatch;
	}
	// 824DD878: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 824DD87C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824DD880: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824DD884: 80F90000  lwz r7, 0(r25)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD888: 7D485A14  add r10, r8, r11
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 824DD88C: 80D20000  lwz r6, 0(r18)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD890: 3908000C  addi r8, r8, 0xc
	ctx.r[8].s64 = ctx.r[8].s64 + 12;
	// 824DD894: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 824DD898: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD89C: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DD8A0: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 824DD8A4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD8A8: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 824DD8AC: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD8B0: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD8B4: 7CE72A14  add r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[5].u64;
	// 824DD8B8: 54E71838  slwi r7, r7, 3
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824DD8BC: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 824DD8C0: 90870000  stw r4, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 824DD8C4: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD8C8: 91470004  stw r10, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824DD8CC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD8D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824DD8D4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824DD8D8: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 824DD8DC: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824DD8E0: 4198FF9C  blt cr6, 0x824dd87c
	if ctx.cr[6].lt {
	pc = 0x824DD87C; continue 'dispatch;
	}
	// 824DD8E4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 824DD8E8: 419A0088  beq cr6, 0x824dd970
	if ctx.cr[6].eq {
	pc = 0x824DD970; continue 'dispatch;
	}
	// 824DD8EC: 8175000C  lwz r11, 0xc(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(12 as u32) ) } as u64;
	// 824DD8F0: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 824DD8F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DD8F8: 40990078  ble cr6, 0x824dd970
	if !ctx.cr[6].gt {
	pc = 0x824DD970; continue 'dispatch;
	}
	// 824DD8FC: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 824DD900: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD904: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 824DD908: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD90C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DD910: 409A0048  bne cr6, 0x824dd958
	if !ctx.cr[6].eq {
	pc = 0x824DD958; continue 'dispatch;
	}
	// 824DD914: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 824DD918: 815A0004  lwz r10, 4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD91C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824DD920: 83CE0000  lwz r30, 0(r14)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD924: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824DD928: 409A0010  bne cr6, 0x824dd938
	if !ctx.cr[6].eq {
	pc = 0x824DD938; continue 'dispatch;
	}
	// 824DD92C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824DD930: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824DD934: 4BF90A1D  bl 0x8246e350
	ctx.lr = 0x824DD938;
	sub_8246E350(ctx, base);
	// 824DD938: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD93C: 7D5FF02E  lwzx r10, r31, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824DD940: 813A0000  lwz r9, 0(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DD944: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824DD948: 7D4B492E  stwx r10, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u32) };
	// 824DD94C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DD950: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824DD954: 917A0004  stw r11, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824DD958: 8175000C  lwz r11, 0xc(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(12 as u32) ) } as u64;
	// 824DD95C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 824DD960: 3BBD000C  addi r29, r29, 0xc
	ctx.r[29].s64 = ctx.r[29].s64 + 12;
	// 824DD964: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824DD968: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824DD96C: 4198FF94  blt cr6, 0x824dd900
	if ctx.cr[6].lt {
	pc = 0x824DD900; continue 'dispatch;
	}
	// 824DD970: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 824DD974: 4BF8FA55  bl 0x8246d3c8
	ctx.lr = 0x824DD978;
	sub_8246D3C8(ctx, base);
	// 824DD978: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 824DD97C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DD980: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DD984: 409A001C  bne cr6, 0x824dd9a0
	if !ctx.cr[6].eq {
	pc = 0x824DD9A0; continue 'dispatch;
	}
	// 824DD988: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DD98C: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 824DD990: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824DD994: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DD998: 7C6BC02E  lwzx r3, r11, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 824DD99C: 4BF8671D  bl 0x824640b8
	ctx.lr = 0x824DD9A0;
	sub_824640B8(ctx, base);
	// 824DD9A0: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824DD9A4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DD9A8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DD9AC: 409A0028  bne cr6, 0x824dd9d4
	if !ctx.cr[6].eq {
	pc = 0x824DD9D4; continue 'dispatch;
	}
	// 824DD9B0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824DD9B4: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824DD9B8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824DD9BC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DD9C0: 7C6AC02E  lwzx r3, r10, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 824DD9C4: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824DD9C8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824DD9CC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DD9D0: 4BF866E9  bl 0x824640b8
	ctx.lr = 0x824DD9D4;
	sub_824640B8(ctx, base);
	// 824DD9D4: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 824DD9D8: 38210390  addi r1, r1, 0x390
	ctx.r[1].s64 = ctx.r[1].s64 + 912;
	// 824DD9DC: 480576F4  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
	// 824DD9E0: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 824DD9E4: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 824DD9E8: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 824DD9EC: 4BF8B40D  bl 0x82468df8
	ctx.lr = 0x824DD9F0;
	sub_82468DF8(ctx, base);
	// 824DD9F0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DD9F4: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 824DD9F8: 388B21D8  addi r4, r11, 0x21d8
	ctx.r[4].s64 = ctx.r[11].s64 + 8664;
	// 824DD9FC: 4BF8A8A5  bl 0x824682a0
	ctx.lr = 0x824DDA00;
	sub_824682A0(ctx, base);
	// 824DDA00: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824DDA04: 3CA0ABBA  lis r5, -0x5446
	ctx.r[5].s64 = -1413873664;
	// 824DDA08: 39000044  li r8, 0x44
	ctx.r[8].s64 = 68;
	// 824DDA0C: 38C100F0  addi r6, r1, 0xf0
	ctx.r[6].s64 = ctx.r[1].s64 + 240;
	// 824DDA10: 60A5AA88  ori r5, r5, 0xaa88
	ctx.r[5].u64 = ctx.r[5].u64 | 43656;
	// 824DDA14: 806B9190  lwz r3, -0x6e70(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 824DDA18: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DDA1C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824DDA20: 38EB21A4  addi r7, r11, 0x21a4
	ctx.r[7].s64 = ctx.r[11].s64 + 8612;
	// 824DDA24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DDA28: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824DDA2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DDA30: 4E800421  bctrl
	ctx.lr = 0x824DDA34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DDA34: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 824DDA38: 4BF8AEA9  bl 0x824688e0
	ctx.lr = 0x824DDA3C;
	sub_824688E0(ctx, base);
	// 824DDA3C: A1750004  lhz r11, 4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DDA40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DDA44: 419A0034  beq cr6, 0x824dda78
	if ctx.cr[6].eq {
	pc = 0x824DDA78; continue 'dispatch;
	}
	// 824DDA48: A1750006  lhz r11, 6(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[21].u32.wrapping_add(6 as u32) ) } as u64;
	// 824DDA4C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824DDA50: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824DDA54: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DDA58: B1750006  sth r11, 6(r21)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[21].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824DDA5C: 409A001C  bne cr6, 0x824dda78
	if !ctx.cr[6].eq {
	pc = 0x824DDA78; continue 'dispatch;
	}
	// 824DDA60: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DDA64: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824DDA68: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 824DDA6C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DDA70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DDA74: 4E800421  bctrl
	ctx.lr = 0x824DDA78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DDA78: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 824DDA7C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DDA80: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DDA84: 409A001C  bne cr6, 0x824ddaa0
	if !ctx.cr[6].eq {
	pc = 0x824DDAA0; continue 'dispatch;
	}
	// 824DDA88: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DDA8C: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 824DDA90: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824DDA94: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DDA98: 7C6BC02E  lwzx r3, r11, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 824DDA9C: 4BF8661D  bl 0x824640b8
	ctx.lr = 0x824DDAA0;
	sub_824640B8(ctx, base);
	// 824DDAA0: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 824DDAA4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DDAA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DDAAC: 409A001C  bne cr6, 0x824ddac8
	if !ctx.cr[6].eq {
	pc = 0x824DDAC8; continue 'dispatch;
	}
	// 824DDAB0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DDAB4: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 824DDAB8: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824DDABC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DDAC0: 7C6BC02E  lwzx r3, r11, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 824DDAC4: 4BF865F5  bl 0x824640b8
	ctx.lr = 0x824DDAC8;
	sub_824640B8(ctx, base);
	// 824DDAC8: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 824DDACC: 4BF8F8FD  bl 0x8246d3c8
	ctx.lr = 0x824DDAD0;
	sub_8246D3C8(ctx, base);
	// 824DDAD0: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 824DDAD4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DDAD8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DDADC: 409A001C  bne cr6, 0x824ddaf8
	if !ctx.cr[6].eq {
	pc = 0x824DDAF8; continue 'dispatch;
	}
	// 824DDAE0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DDAE4: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 824DDAE8: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824DDAEC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DDAF0: 7C6BC02E  lwzx r3, r11, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 824DDAF4: 4BF865C5  bl 0x824640b8
	ctx.lr = 0x824DDAF8;
	sub_824640B8(ctx, base);
	// 824DDAF8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824DDAFC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DDB00: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DDB04: 409A0028  bne cr6, 0x824ddb2c
	if !ctx.cr[6].eq {
	pc = 0x824DDB2C; continue 'dispatch;
	}
	// 824DDB08: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824DDB0C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824DDB10: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824DDB14: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DDB18: 7C6AC02E  lwzx r3, r10, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 824DDB1C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824DDB20: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824DDB24: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DDB28: 4BF86591  bl 0x824640b8
	ctx.lr = 0x824DDB2C;
	sub_824640B8(ctx, base);
	// 824DDB2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824DDB30: 38210390  addi r1, r1, 0x390
	ctx.r[1].s64 = ctx.r[1].s64 + 912;
	// 824DDB34: 4805759C  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
	// 824DDB38: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 824DDB3C: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 824DDB40: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 824DDB44: 4BF8B2B5  bl 0x82468df8
	ctx.lr = 0x824DDB48;
	sub_82468DF8(ctx, base);
	// 824DDB48: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DDB4C: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 824DDB50: 388B2188  addi r4, r11, 0x2188
	ctx.r[4].s64 = ctx.r[11].s64 + 8584;
	// 824DDB54: 4BF8A74D  bl 0x824682a0
	ctx.lr = 0x824DDB58;
	sub_824682A0(ctx, base);
	// 824DDB58: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824DDB5C: 3CA0ABBA  lis r5, -0x5446
	ctx.r[5].s64 = -1413873664;
	// 824DDB60: 3900004E  li r8, 0x4e
	ctx.r[8].s64 = 78;
	// 824DDB64: 38C100F0  addi r6, r1, 0xf0
	ctx.r[6].s64 = ctx.r[1].s64 + 240;
	// 824DDB68: 60A5DDAA  ori r5, r5, 0xddaa
	ctx.r[5].u64 = ctx.r[5].u64 | 56746;
	// 824DDB6C: 806B9190  lwz r3, -0x6e70(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 824DDB70: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DDB74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824DDB78: 38EB21A4  addi r7, r11, 0x21a4
	ctx.r[7].s64 = ctx.r[11].s64 + 8612;
	// 824DDB7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DDB80: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824DDB84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DDB88: 4E800421  bctrl
	ctx.lr = 0x824DDB8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DDB8C: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 824DDB90: 4BF8AD51  bl 0x824688e0
	ctx.lr = 0x824DDB94;
	sub_824688E0(ctx, base);
	// 824DDB94: A1750004  lhz r11, 4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DDB98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DDB9C: 419A0034  beq cr6, 0x824ddbd0
	if ctx.cr[6].eq {
	pc = 0x824DDBD0; continue 'dispatch;
	}
	// 824DDBA0: A1750006  lhz r11, 6(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[21].u32.wrapping_add(6 as u32) ) } as u64;
	// 824DDBA4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824DDBA8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824DDBAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DDBB0: B1750006  sth r11, 6(r21)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[21].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824DDBB4: 409A001C  bne cr6, 0x824ddbd0
	if !ctx.cr[6].eq {
	pc = 0x824DDBD0; continue 'dispatch;
	}
	// 824DDBB8: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DDBBC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824DDBC0: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 824DDBC4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DDBC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DDBCC: 4E800421  bctrl
	ctx.lr = 0x824DDBD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DDBD0: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 824DDBD4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DDBD8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DDBDC: 409A001C  bne cr6, 0x824ddbf8
	if !ctx.cr[6].eq {
	pc = 0x824DDBF8; continue 'dispatch;
	}
	// 824DDBE0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DDBE4: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 824DDBE8: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824DDBEC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DDBF0: 7C6BC02E  lwzx r3, r11, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 824DDBF4: 4BF864C5  bl 0x824640b8
	ctx.lr = 0x824DDBF8;
	sub_824640B8(ctx, base);
	// 824DDBF8: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 824DDBFC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DDC00: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DDC04: 409A001C  bne cr6, 0x824ddc20
	if !ctx.cr[6].eq {
	pc = 0x824DDC20; continue 'dispatch;
	}
	// 824DDC08: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DDC0C: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 824DDC10: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824DDC14: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DDC18: 7C6BC02E  lwzx r3, r11, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 824DDC1C: 4BF8649D  bl 0x824640b8
	ctx.lr = 0x824DDC20;
	sub_824640B8(ctx, base);
	// 824DDC20: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 824DDC24: 4BF8F7A5  bl 0x8246d3c8
	ctx.lr = 0x824DDC28;
	sub_8246D3C8(ctx, base);
	// 824DDC28: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 824DDC2C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DDC30: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DDC34: 409A001C  bne cr6, 0x824ddc50
	if !ctx.cr[6].eq {
	pc = 0x824DDC50; continue 'dispatch;
	}
	// 824DDC38: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DDC3C: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 824DDC40: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824DDC44: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DDC48: 7C6BC02E  lwzx r3, r11, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 824DDC4C: 4BF8646D  bl 0x824640b8
	ctx.lr = 0x824DDC50;
	sub_824640B8(ctx, base);
	// 824DDC50: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824DDC54: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DDC58: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DDC5C: 409AFED0  bne cr6, 0x824ddb2c
	if !ctx.cr[6].eq {
	pc = 0x824DDB2C; continue 'dispatch;
	}
	// 824DDC60: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824DDC64: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824DDC68: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824DDC6C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DDC70: 7C6AC02E  lwzx r3, r10, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 824DDC74: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824DDC78: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824DDC7C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DDC80: 4BF86439  bl 0x824640b8
	ctx.lr = 0x824DDC84;
	sub_824640B8(ctx, base);
	// 824DDC84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824DDC88: 38210390  addi r1, r1, 0x390
	ctx.r[1].s64 = ctx.r[1].s64 + 912;
	// 824DDC8C: 48057444  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
	// 824DDC90: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 824DDC94: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 824DDC98: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 824DDC9C: 4BF8B15D  bl 0x82468df8
	ctx.lr = 0x824DDCA0;
	sub_82468DF8(ctx, base);
	// 824DDCA0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DDCA4: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 824DDCA8: 388B2178  addi r4, r11, 0x2178
	ctx.r[4].s64 = ctx.r[11].s64 + 8568;
	// 824DDCAC: 4BF8A5F5  bl 0x824682a0
	ctx.lr = 0x824DDCB0;
	sub_824682A0(ctx, base);
	// 824DDCB0: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824DDCB4: 3CA0ABBA  lis r5, -0x5446
	ctx.r[5].s64 = -1413873664;
	// 824DDCB8: 3900005D  li r8, 0x5d
	ctx.r[8].s64 = 93;
	// 824DDCBC: 38C100F0  addi r6, r1, 0xf0
	ctx.r[6].s64 = ctx.r[1].s64 + 240;
	// 824DDCC0: 60A599DD  ori r5, r5, 0x99dd
	ctx.r[5].u64 = ctx.r[5].u64 | 39389;
	// 824DDCC4: 806B9190  lwz r3, -0x6e70(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 824DDCC8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DDCCC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824DDCD0: 38EB21A4  addi r7, r11, 0x21a4
	ctx.r[7].s64 = ctx.r[11].s64 + 8612;
	// 824DDCD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DDCD8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824DDCDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DDCE0: 4E800421  bctrl
	ctx.lr = 0x824DDCE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DDCE4: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 824DDCE8: 4BF8ABF9  bl 0x824688e0
	ctx.lr = 0x824DDCEC;
	sub_824688E0(ctx, base);
	// 824DDCEC: A17B0004  lhz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DDCF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DDCF4: 419A0034  beq cr6, 0x824ddd28
	if ctx.cr[6].eq {
	pc = 0x824DDD28; continue 'dispatch;
	}
	// 824DDCF8: A17B0006  lhz r11, 6(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(6 as u32) ) } as u64;
	// 824DDCFC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824DDD00: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824DDD04: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DDD08: B17B0006  sth r11, 6(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824DDD0C: 409A001C  bne cr6, 0x824ddd28
	if !ctx.cr[6].eq {
	pc = 0x824DDD28; continue 'dispatch;
	}
	// 824DDD10: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DDD14: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824DDD18: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824DDD1C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DDD20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DDD24: 4E800421  bctrl
	ctx.lr = 0x824DDD28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DDD28: A1750004  lhz r11, 4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DDD2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DDD30: 419A0034  beq cr6, 0x824ddd64
	if ctx.cr[6].eq {
	pc = 0x824DDD64; continue 'dispatch;
	}
	// 824DDD34: A1750006  lhz r11, 6(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[21].u32.wrapping_add(6 as u32) ) } as u64;
	// 824DDD38: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824DDD3C: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824DDD40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DDD44: B1750006  sth r11, 6(r21)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[21].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824DDD48: 409A001C  bne cr6, 0x824ddd64
	if !ctx.cr[6].eq {
	pc = 0x824DDD64; continue 'dispatch;
	}
	// 824DDD4C: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DDD50: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824DDD54: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 824DDD58: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DDD5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DDD60: 4E800421  bctrl
	ctx.lr = 0x824DDD64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DDD64: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 824DDD68: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824DDD6C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DDD70: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DDD74: 409A001C  bne cr6, 0x824ddd90
	if !ctx.cr[6].eq {
	pc = 0x824DDD90; continue 'dispatch;
	}
	// 824DDD78: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DDD7C: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 824DDD80: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824DDD84: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DDD88: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824DDD8C: 4BF8632D  bl 0x824640b8
	ctx.lr = 0x824DDD90;
	sub_824640B8(ctx, base);
	// 824DDD90: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 824DDD94: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DDD98: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DDD9C: 409A001C  bne cr6, 0x824dddb8
	if !ctx.cr[6].eq {
	pc = 0x824DDDB8; continue 'dispatch;
	}
	// 824DDDA0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DDDA4: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 824DDDA8: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824DDDAC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DDDB0: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824DDDB4: 4BF86305  bl 0x824640b8
	ctx.lr = 0x824DDDB8;
	sub_824640B8(ctx, base);
	// 824DDDB8: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 824DDDBC: 4BF8F60D  bl 0x8246d3c8
	ctx.lr = 0x824DDDC0;
	sub_8246D3C8(ctx, base);
	// 824DDDC0: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 824DDDC4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DDDC8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DDDCC: 409A001C  bne cr6, 0x824ddde8
	if !ctx.cr[6].eq {
	pc = 0x824DDDE8; continue 'dispatch;
	}
	// 824DDDD0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DDDD4: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 824DDDD8: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824DDDDC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DDDE0: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824DDDE4: 4BF862D5  bl 0x824640b8
	ctx.lr = 0x824DDDE8;
	sub_824640B8(ctx, base);
	// 824DDDE8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824DDDEC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DDDF0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DDDF4: 409A0028  bne cr6, 0x824dde1c
	if !ctx.cr[6].eq {
	pc = 0x824DDE1C; continue 'dispatch;
	}
	// 824DDDF8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824DDDFC: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824DDE00: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824DDE04: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DDE08: 7C6AF82E  lwzx r3, r10, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824DDE0C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824DDE10: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824DDE14: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DDE18: 4BF862A1  bl 0x824640b8
	ctx.lr = 0x824DDE1C;
	sub_824640B8(ctx, base);
	// 824DDE1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824DDE20: 38210390  addi r1, r1, 0x390
	ctx.r[1].s64 = ctx.r[1].s64 + 912;
	// 824DDE24: 480572AC  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
	// 824DDE28: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 824DDE2C: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 824DDE30: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 824DDE34: 4BF8AFC5  bl 0x82468df8
	ctx.lr = 0x824DDE38;
	sub_82468DF8(ctx, base);
	// 824DDE38: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DDE3C: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 824DDE40: 388B2154  addi r4, r11, 0x2154
	ctx.r[4].s64 = ctx.r[11].s64 + 8532;
	// 824DDE44: 4BF8A45D  bl 0x824682a0
	ctx.lr = 0x824DDE48;
	sub_824682A0(ctx, base);
	// 824DDE48: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824DDE4C: 3CA0ABBA  lis r5, -0x5446
	ctx.r[5].s64 = -1413873664;
	// 824DDE50: 3900006C  li r8, 0x6c
	ctx.r[8].s64 = 108;
	// 824DDE54: 38C100F0  addi r6, r1, 0xf0
	ctx.r[6].s64 = ctx.r[1].s64 + 240;
	// 824DDE58: 60A59D6D  ori r5, r5, 0x9d6d
	ctx.r[5].u64 = ctx.r[5].u64 | 40301;
	// 824DDE5C: 806B9190  lwz r3, -0x6e70(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 824DDE60: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DDE64: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824DDE68: 38EB21A4  addi r7, r11, 0x21a4
	ctx.r[7].s64 = ctx.r[11].s64 + 8612;
	// 824DDE6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DDE70: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824DDE74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DDE78: 4E800421  bctrl
	ctx.lr = 0x824DDE7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DDE7C: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 824DDE80: 4BF8AA61  bl 0x824688e0
	ctx.lr = 0x824DDE84;
	sub_824688E0(ctx, base);
	// 824DDE84: A17B0004  lhz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DDE88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DDE8C: 419A0034  beq cr6, 0x824ddec0
	if ctx.cr[6].eq {
	pc = 0x824DDEC0; continue 'dispatch;
	}
	// 824DDE90: A17B0006  lhz r11, 6(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(6 as u32) ) } as u64;
	// 824DDE94: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824DDE98: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824DDE9C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DDEA0: B17B0006  sth r11, 6(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824DDEA4: 409A001C  bne cr6, 0x824ddec0
	if !ctx.cr[6].eq {
	pc = 0x824DDEC0; continue 'dispatch;
	}
	// 824DDEA8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DDEAC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824DDEB0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824DDEB4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DDEB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DDEBC: 4E800421  bctrl
	ctx.lr = 0x824DDEC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DDEC0: A1750004  lhz r11, 4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DDEC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DDEC8: 419A0034  beq cr6, 0x824ddefc
	if ctx.cr[6].eq {
	pc = 0x824DDEFC; continue 'dispatch;
	}
	// 824DDECC: A1750006  lhz r11, 6(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[21].u32.wrapping_add(6 as u32) ) } as u64;
	// 824DDED0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824DDED4: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824DDED8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DDEDC: B1750006  sth r11, 6(r21)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[21].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824DDEE0: 409A001C  bne cr6, 0x824ddefc
	if !ctx.cr[6].eq {
	pc = 0x824DDEFC; continue 'dispatch;
	}
	// 824DDEE4: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DDEE8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824DDEEC: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 824DDEF0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DDEF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DDEF8: 4E800421  bctrl
	ctx.lr = 0x824DDEFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DDEFC: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 824DDF00: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824DDF04: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DDF08: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DDF0C: 409A001C  bne cr6, 0x824ddf28
	if !ctx.cr[6].eq {
	pc = 0x824DDF28; continue 'dispatch;
	}
	// 824DDF10: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DDF14: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 824DDF18: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824DDF1C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DDF20: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824DDF24: 4BF86195  bl 0x824640b8
	ctx.lr = 0x824DDF28;
	sub_824640B8(ctx, base);
	// 824DDF28: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 824DDF2C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DDF30: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DDF34: 409A001C  bne cr6, 0x824ddf50
	if !ctx.cr[6].eq {
	pc = 0x824DDF50; continue 'dispatch;
	}
	// 824DDF38: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DDF3C: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 824DDF40: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824DDF44: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DDF48: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824DDF4C: 4BF8616D  bl 0x824640b8
	ctx.lr = 0x824DDF50;
	sub_824640B8(ctx, base);
	// 824DDF50: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 824DDF54: 4BF8F475  bl 0x8246d3c8
	ctx.lr = 0x824DDF58;
	sub_8246D3C8(ctx, base);
	// 824DDF58: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 824DDF5C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DDF60: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DDF64: 409A001C  bne cr6, 0x824ddf80
	if !ctx.cr[6].eq {
	pc = 0x824DDF80; continue 'dispatch;
	}
	// 824DDF68: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DDF6C: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 824DDF70: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824DDF74: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DDF78: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824DDF7C: 4BF8613D  bl 0x824640b8
	ctx.lr = 0x824DDF80;
	sub_824640B8(ctx, base);
	// 824DDF80: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824DDF84: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DDF88: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DDF8C: 409AFE90  bne cr6, 0x824dde1c
	if !ctx.cr[6].eq {
	pc = 0x824DDE1C; continue 'dispatch;
	}
	// 824DDF90: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824DDF94: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824DDF98: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824DDF9C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DDFA0: 7C6AF82E  lwzx r3, r10, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824DDFA4: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824DDFA8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824DDFAC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DDFB0: 4BF86109  bl 0x824640b8
	ctx.lr = 0x824DDFB4;
	sub_824640B8(ctx, base);
	// 824DDFB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824DDFB8: 38210390  addi r1, r1, 0x390
	ctx.r[1].s64 = ctx.r[1].s64 + 912;
	// 824DDFBC: 48057114  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DDFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DDFC0 size=412
    let mut pc: u32 = 0x824DDFC0;
    'dispatch: loop {
        match pc {
            0x824DDFC0 => {
    //   block [0x824DDFC0..0x824DE15C)
	// 824DDFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DDFC4: 480570F9  bl 0x825350bc
	ctx.lr = 0x824DDFC8;
	sub_82535080(ctx, base);
	// 824DDFC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DDFCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824DDFD0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824DDFD4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824DDFD8: 396B184C  addi r11, r11, 0x184c
	ctx.r[11].s64 = ctx.r[11].s64 + 6220;
	// 824DDFDC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824DDFE0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DDFE4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824DDFE8: 40990068  ble cr6, 0x824de050
	if !ctx.cr[6].gt {
	pc = 0x824DE050; continue 'dispatch;
	}
	// 824DDFEC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824DDFF0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824DDFF4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824DDFF8: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824DDFFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824DE000: 419A003C  beq cr6, 0x824de03c
	if ctx.cr[6].eq {
	pc = 0x824DE03C; continue 'dispatch;
	}
	// 824DE004: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DE008: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DE00C: 419A0030  beq cr6, 0x824de03c
	if ctx.cr[6].eq {
	pc = 0x824DE03C; continue 'dispatch;
	}
	// 824DE010: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824DE014: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824DE018: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824DE01C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DE020: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824DE024: 409A0018  bne cr6, 0x824de03c
	if !ctx.cr[6].eq {
	pc = 0x824DE03C; continue 'dispatch;
	}
	// 824DE028: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DE02C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824DE030: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DE034: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DE038: 4E800421  bctrl
	ctx.lr = 0x824DE03C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DE03C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824DE040: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824DE044: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 824DE048: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824DE04C: 4198FFA4  blt cr6, 0x824ddff0
	if ctx.cr[6].lt {
	pc = 0x824DDFF0; continue 'dispatch;
	}
	// 824DE050: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 824DE054: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824DE058: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DE05C: 4099005C  ble cr6, 0x824de0b8
	if !ctx.cr[6].gt {
	pc = 0x824DE0B8; continue 'dispatch;
	}
	// 824DE060: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824DE064: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824DE068: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824DE06C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824DE070: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DE074: 419A0030  beq cr6, 0x824de0a4
	if ctx.cr[6].eq {
	pc = 0x824DE0A4; continue 'dispatch;
	}
	// 824DE078: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824DE07C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824DE080: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824DE084: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824DE088: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824DE08C: 409A0018  bne cr6, 0x824de0a4
	if !ctx.cr[6].eq {
	pc = 0x824DE0A4; continue 'dispatch;
	}
	// 824DE090: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DE094: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824DE098: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DE09C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824DE0A0: 4E800421  bctrl
	ctx.lr = 0x824DE0A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824DE0A4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 824DE0A8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824DE0AC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 824DE0B0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824DE0B4: 4198FFB0  blt cr6, 0x824de064
	if ctx.cr[6].lt {
	pc = 0x824DE064; continue 'dispatch;
	}
	// 824DE0B8: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824DE0BC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DE0C0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DE0C4: 409A0020  bne cr6, 0x824de0e4
	if !ctx.cr[6].eq {
	pc = 0x824DE0E4; continue 'dispatch;
	}
	// 824DE0C8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DE0CC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824DE0D0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DE0D4: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824DE0D8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DE0DC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824DE0E0: 4BF85FD9  bl 0x824640b8
	ctx.lr = 0x824DE0E4;
	sub_824640B8(ctx, base);
	// 824DE0E4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824DE0E8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DE0EC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DE0F0: 409A0020  bne cr6, 0x824de110
	if !ctx.cr[6].eq {
	pc = 0x824DE110; continue 'dispatch;
	}
	// 824DE0F4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DE0F8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824DE0FC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DE100: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824DE104: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DE108: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824DE10C: 4BF85FAD  bl 0x824640b8
	ctx.lr = 0x824DE110;
	sub_824640B8(ctx, base);
	// 824DE110: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824DE114: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DE118: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DE11C: 409A002C  bne cr6, 0x824de148
	if !ctx.cr[6].eq {
	pc = 0x824DE148; continue 'dispatch;
	}
	// 824DE120: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DE124: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824DE128: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824DE12C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824DE130: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DE134: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824DE138: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824DE13C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824DE140: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DE144: 4BF85F75  bl 0x824640b8
	ctx.lr = 0x824DE148;
	sub_824640B8(ctx, base);
	// 824DE148: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824DE14C: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824DE150: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824DE154: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824DE158: 48056FB4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DE160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824DE160 size=236
    let mut pc: u32 = 0x824DE160;
    'dispatch: loop {
        match pc {
            0x824DE160 => {
    //   block [0x824DE160..0x824DE24C)
	// 824DE160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DE164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824DE168: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824DE16C: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 824DE170: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 824DE174: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DE178: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824DE17C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 824DE180: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 824DE184: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 824DE188: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 824DE18C: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 824DE190: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 824DE194: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824DE198: 4BFFF1A9  bl 0x824dd340
	ctx.lr = 0x824DE19C;
	sub_824DD340(ctx, base);
	// 824DE19C: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 824DE1A0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DE1A4: 419A0060  beq cr6, 0x824de204
	if ctx.cr[6].eq {
	pc = 0x824DE204; continue 'dispatch;
	}
	// 824DE1A8: 7D4907B4  extsw r9, r10
	ctx.r[9].s64 = ctx.r[10].s32 as i64;
	// 824DE1AC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DE1B0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 824DE1B4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 824DE1B8: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 824DE1BC: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 824DE1C0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 824DE1C4: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 824DE1C8: C00A1850  lfs f0, 0x1850(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824DE1CC: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 824DE1D0: EDA007F2  fmuls f13, f0, f31
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 824DE1D4: EC0007B2  fmuls f0, f0, f30
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64) as f32) as f64);
	// 824DE1D8: 4099002C  ble cr6, 0x824de204
	if !ctx.cr[6].gt {
	pc = 0x824DE204; continue 'dispatch;
	}
	// 824DE1DC: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 824DE1E0: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824DE1E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824DE1E8: 7D2A482E  lwzx r9, r10, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 824DE1EC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824DE1F0: D1A9000C  stfs f13, 0xc(r9)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824DE1F4: D0090010  stfs f0, 0x10(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824DE1F8: 8121005C  lwz r9, 0x5c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 824DE1FC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824DE200: 4198FFE0  blt cr6, 0x824de1e0
	if ctx.cr[6].lt {
	pc = 0x824DE1E0; continue 'dispatch;
	}
	// 824DE204: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824DE208: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824DE20C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824DE210: 409A0020  bne cr6, 0x824de230
	if !ctx.cr[6].eq {
	pc = 0x824DE230; continue 'dispatch;
	}
	// 824DE214: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824DE218: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824DE21C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824DE220: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824DE224: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824DE228: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824DE22C: 4BF85E8D  bl 0x824640b8
	ctx.lr = 0x824DE230;
	sub_824640B8(ctx, base);
	// 824DE230: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824DE234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824DE238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824DE23C: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 824DE240: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824DE244: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824DE248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DE250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824DE250 size=144
    let mut pc: u32 = 0x824DE250;
    'dispatch: loop {
        match pc {
            0x824DE250 => {
    //   block [0x824DE250..0x824DE2E0)
	// 824DE250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824DE254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824DE258: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824DE25C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824DE260: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824DE264: 4802055D  bl 0x824fe7c0
	ctx.lr = 0x824DE268;
	sub_824FE7C0(ctx, base);
	// 824DE268: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824DE26C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824DE270: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824DE274: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 824DE278: 3CC08206  lis r6, -0x7dfa
	ctx.r[6].s64 = -2113536000;
	// 824DE27C: 394A18C4  addi r10, r10, 0x18c4
	ctx.r[10].s64 = ctx.r[10].s64 + 6340;
	// 824DE280: 392918B8  addi r9, r9, 0x18b8
	ctx.r[9].s64 = ctx.r[9].s64 + 6328;
	// 824DE284: 390818A4  addi r8, r8, 0x18a4
	ctx.r[8].s64 = ctx.r[8].s64 + 6308;
	// 824DE288: 38E71898  addi r7, r7, 0x1898
	ctx.r[7].s64 = ctx.r[7].s64 + 6296;
	// 824DE28C: 38C6188C  addi r6, r6, 0x188c
	ctx.r[6].s64 = ctx.r[6].s64 + 6284;
	// 824DE290: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824DE294: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824DE298: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824DE29C: 397F0034  addi r11, r31, 0x34
	ctx.r[11].s64 = ctx.r[31].s64 + 52;
	// 824DE2A0: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824DE2A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824DE2A8: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 824DE2AC: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 824DE2B0: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 824DE2B4: 98BF0030  stb r5, 0x30(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[5].u8 ) };
	// 824DE2B8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 824DE2BC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824DE2C0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824DE2C4: 4200FFF8  bdnz 0x824de2bc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824DE2BC; continue 'dispatch;
	}
	// 824DE2C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824DE2CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824DE2D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824DE2D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824DE2D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824DE2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DE2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DE2E0 size=52
    let mut pc: u32 = 0x824DE2E0;
    'dispatch: loop {
        match pc {
            0x824DE2E0 => {
    //   block [0x824DE2E0..0x824DE314)
	// 824DE2E0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 824DE2E4: 419A03D4  beq cr6, 0x824de6b8
	if ctx.cr[6].eq {
		sub_824DE6B8(ctx, base);
		return;
	}
	// 824DE2E8: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824DE2EC: 419A03CC  beq cr6, 0x824de6b8
	if ctx.cr[6].eq {
		sub_824DE6B8(ctx, base);
		return;
	}
	// 824DE2F0: 54AA063E  clrlwi r10, r5, 0x18
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 824DE2F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824DE2F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824DE2FC: 419A00E0  beq cr6, 0x824de3dc
	if ctx.cr[6].eq {
		sub_824DE3D4(ctx, base);
		return;
	}
	// 824DE300: 54AB07FE  clrlwi r11, r5, 0x1f
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x00000001u64;
	// 824DE304: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DE308: 419A000C  beq cr6, 0x824de314
	if ctx.cr[6].eq {
		sub_824DE314(ctx, base);
		return;
	}
	// 824DE30C: 81440034  lwz r10, 0x34(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 824DE310: 48000008  b 0x824de318
	sub_824DE314(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DE314(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DE314 size=24
    let mut pc: u32 = 0x824DE314;
    'dispatch: loop {
        match pc {
            0x824DE314 => {
    //   block [0x824DE314..0x824DE32C)
	// 824DE314: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824DE318: 54AB07BC  rlwinm r11, r5, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 824DE31C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824DE320: 419A000C  beq cr6, 0x824de32c
	if ctx.cr[6].eq {
		sub_824DE32C(ctx, base);
		return;
	}
	// 824DE324: 81640038  lwz r11, 0x38(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) } as u64;
	// 824DE328: 48000008  b 0x824de330
	sub_824DE32C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DE32C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DE32C size=28
    let mut pc: u32 = 0x824DE32C;
    'dispatch: loop {
        match pc {
            0x824DE32C => {
    //   block [0x824DE32C..0x824DE348)
	// 824DE32C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824DE330: 54A9077A  rlwinm r9, r5, 0, 0x1d, 0x1d
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 824DE334: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 824DE338: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824DE33C: 419A000C  beq cr6, 0x824de348
	if ctx.cr[6].eq {
		sub_824DE348(ctx, base);
		return;
	}
	// 824DE340: 8164003C  lwz r11, 0x3c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 824DE344: 48000008  b 0x824de34c
	sub_824DE348(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DE348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DE348 size=28
    let mut pc: u32 = 0x824DE348;
    'dispatch: loop {
        match pc {
            0x824DE348 => {
    //   block [0x824DE348..0x824DE364)
	// 824DE348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824DE34C: 54A90738  rlwinm r9, r5, 0, 0x1c, 0x1c
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 824DE350: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 824DE354: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824DE358: 419A000C  beq cr6, 0x824de364
	if ctx.cr[6].eq {
		sub_824DE364(ctx, base);
		return;
	}
	// 824DE35C: 81640040  lwz r11, 0x40(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) } as u64;
	// 824DE360: 48000008  b 0x824de368
	sub_824DE364(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DE364(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DE364 size=28
    let mut pc: u32 = 0x824DE364;
    'dispatch: loop {
        match pc {
            0x824DE364 => {
    //   block [0x824DE364..0x824DE380)
	// 824DE364: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824DE368: 54A906F6  rlwinm r9, r5, 0, 0x1b, 0x1b
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 824DE36C: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 824DE370: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824DE374: 419A000C  beq cr6, 0x824de380
	if ctx.cr[6].eq {
		sub_824DE380(ctx, base);
		return;
	}
	// 824DE378: 81640044  lwz r11, 0x44(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 824DE37C: 48000008  b 0x824de384
	sub_824DE380(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DE380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DE380 size=28
    let mut pc: u32 = 0x824DE380;
    'dispatch: loop {
        match pc {
            0x824DE380 => {
    //   block [0x824DE380..0x824DE39C)
	// 824DE380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824DE384: 54A906B4  rlwinm r9, r5, 0, 0x1a, 0x1a
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 824DE388: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 824DE38C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824DE390: 419A000C  beq cr6, 0x824de39c
	if ctx.cr[6].eq {
		sub_824DE39C(ctx, base);
		return;
	}
	// 824DE394: 81640048  lwz r11, 0x48(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 824DE398: 48000008  b 0x824de3a0
	sub_824DE39C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DE39C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DE39C size=28
    let mut pc: u32 = 0x824DE39C;
    'dispatch: loop {
        match pc {
            0x824DE39C => {
    //   block [0x824DE39C..0x824DE3B8)
	// 824DE39C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824DE3A0: 54A90672  rlwinm r9, r5, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 824DE3A4: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 824DE3A8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824DE3AC: 419A000C  beq cr6, 0x824de3b8
	if ctx.cr[6].eq {
		sub_824DE3B8(ctx, base);
		return;
	}
	// 824DE3B0: 8164004C  lwz r11, 0x4c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 824DE3B4: 48000008  b 0x824de3bc
	sub_824DE3B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DE3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DE3B8 size=28
    let mut pc: u32 = 0x824DE3B8;
    'dispatch: loop {
        match pc {
            0x824DE3B8 => {
    //   block [0x824DE3B8..0x824DE3D4)
	// 824DE3B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824DE3BC: 54A90630  rlwinm r9, r5, 0, 0x18, 0x18
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 824DE3C0: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 824DE3C4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824DE3C8: 419A000C  beq cr6, 0x824de3d4
	if ctx.cr[6].eq {
		sub_824DE3D4(ctx, base);
		return;
	}
	// 824DE3CC: 81640050  lwz r11, 0x50(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(80 as u32) ) } as u64;
	// 824DE3D0: 48000008  b 0x824de3d8
	sub_824DE3D4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824DE3D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824DE3D4 size=40
    let mut pc: u32 = 0x824DE3D4;
    'dispatch: loop {
        match pc {
            0x824DE3D4 => {
    //   block [0x824DE3D4..0x824DE3FC)
	// 824DE3D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824DE3D8: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 824DE3DC: 54AA042E  rlwinm r10, r5, 0, 0x10, 0x17
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 824DE3E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824DE3E4: 419A00E4  beq cr6, 0x824de4c8
	if ctx.cr[6].eq {
		sub_824DE4C0(ctx, base);
		return;
	}
	// 824DE3E8: 54AA05EE  rlwinm r10, r5, 0, 0x17, 0x17
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 824DE3EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824DE3F0: 419A000C  beq cr6, 0x824de3fc
	if ctx.cr[6].eq {
		sub_824DE3FC(ctx, base);
		return;
	}
	// 824DE3F4: 81440054  lwz r10, 0x54(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(84 as u32) ) } as u64;
	// 824DE3F8: 48000008  b 0x824de400
	sub_824DE3FC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


