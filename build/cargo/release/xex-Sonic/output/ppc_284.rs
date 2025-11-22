pub fn sub_831928C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831928C8 size=104
    let mut pc: u32 = 0x831928C8;
    'dispatch: loop {
        match pc {
            0x831928C8 => {
    //   block [0x831928C8..0x83192930)
	// 831928C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831928CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831928D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831928D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831928D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831928DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831928E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831928E4: 4BFFD4FD  bl 0x8318fde0
	ctx.lr = 0x831928E8;
	sub_8318FDE0(ctx, base);
	// 831928E8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831928EC: 419A0018  beq cr6, 0x83192904
	if ctx.cr[6].eq {
	pc = 0x83192904; continue 'dispatch;
	}
	// 831928F0: 3C80FF03  lis r4, -0xfd
	ctx.r[4].s64 = -16580608;
	// 831928F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831928F8: 6084020C  ori r4, r4, 0x20c
	ctx.r[4].u64 = ctx.r[4].u64 | 524;
	// 831928FC: 48000A95  bl 0x83193390
	ctx.lr = 0x83192900;
	sub_83193390(ctx, base);
	// 83192900: 48000018  b 0x83192918
	pc = 0x83192918; continue 'dispatch;
	// 83192904: 389F12DC  addi r4, r31, 0x12dc
	ctx.r[4].s64 = ctx.r[31].s64 + 4828;
	// 83192908: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 8319290C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83192910: 48015C01  bl 0x831a8510
	ctx.lr = 0x83192914;
	sub_831A8510(ctx, base);
	// 83192914: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83192918: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8319291C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83192920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83192924: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83192928: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8319292C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83192930 size=92
    let mut pc: u32 = 0x83192930;
    'dispatch: loop {
        match pc {
            0x83192930 => {
    //   block [0x83192930..0x8319298C)
	// 83192930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83192934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83192938: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8319293C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83192940: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83192944: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83192948: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8319294C: 4BFFD495  bl 0x8318fde0
	ctx.lr = 0x83192950;
	sub_8318FDE0(ctx, base);
	// 83192950: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83192954: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83192958: 419A0014  beq cr6, 0x8319296c
	if ctx.cr[6].eq {
	pc = 0x8319296C; continue 'dispatch;
	}
	// 8319295C: 3C80FF03  lis r4, -0xfd
	ctx.r[4].s64 = -16580608;
	// 83192960: 6084020D  ori r4, r4, 0x20d
	ctx.r[4].u64 = ctx.r[4].u64 | 525;
	// 83192964: 48000A2D  bl 0x83193390
	ctx.lr = 0x83192968;
	sub_83193390(ctx, base);
	// 83192968: 4800000C  b 0x83192974
	pc = 0x83192974; continue 'dispatch;
	// 8319296C: 817F13C8  lwz r11, 0x13c8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5064 as u32) ) } as u64;
	// 83192970: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83192974: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83192978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319297C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83192980: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83192984: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83192988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83192990 size=156
    let mut pc: u32 = 0x83192990;
    'dispatch: loop {
        match pc {
            0x83192990 => {
    //   block [0x83192990..0x83192A2C)
	// 83192990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83192994: 480157D5  bl 0x831a8168
	ctx.lr = 0x83192998;
	sub_831A8130(ctx, base);
	// 83192998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319299C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831929A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831929A4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831929A8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 831929AC: 4BFFD435  bl 0x8318fde0
	ctx.lr = 0x831929B0;
	sub_8318FDE0(ctx, base);
	// 831929B0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831929B4: 419A001C  beq cr6, 0x831929d0
	if ctx.cr[6].eq {
	pc = 0x831929D0; continue 'dispatch;
	}
	// 831929B8: 3C80FF03  lis r4, -0xfd
	ctx.r[4].s64 = -16580608;
	// 831929BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831929C0: 6084020F  ori r4, r4, 0x20f
	ctx.r[4].u64 = ctx.r[4].u64 | 527;
	// 831929C4: 480009CD  bl 0x83193390
	ctx.lr = 0x831929C8;
	sub_83193390(ctx, base);
	// 831929C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831929CC: 480157EC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 831929D0: 817F13CC  lwz r11, 0x13cc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5068 as u32) ) } as u64;
	// 831929D4: 3D400003  lis r10, 3
	ctx.r[10].s64 = 196608;
	// 831929D8: 556B5828  slwi r11, r11, 0xb
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831929DC: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 831929E0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831929E4: 817F13DC  lwz r11, 0x13dc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5084 as u32) ) } as u64;
	// 831929E8: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831929EC: 817F13C8  lwz r11, 0x13c8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5064 as u32) ) } as u64;
	// 831929F0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 831929F4: 409A0018  bne cr6, 0x83192a0c
	if !ctx.cr[6].eq {
	pc = 0x83192A0C; continue 'dispatch;
	}
	// 831929F8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831929FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83192A00: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83192A04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83192A08: 480157B0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83192A0C: 815F13DC  lwz r10, 0x13dc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5084 as u32) ) } as u64;
	// 83192A10: 39200708  li r9, 0x708
	ctx.r[9].s64 = 1800;
	// 83192A14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83192A18: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 83192A1C: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 83192A20: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83192A24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83192A28: 48015790  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83192A30 size=84
    let mut pc: u32 = 0x83192A30;
    'dispatch: loop {
        match pc {
            0x83192A30 => {
    //   block [0x83192A30..0x83192A84)
	// 83192A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83192A34: 48015739  bl 0x831a816c
	ctx.lr = 0x83192A38;
	sub_831A8130(ctx, base);
	// 83192A38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83192A3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83192A40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83192A44: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83192A48: 4BFFD399  bl 0x8318fde0
	ctx.lr = 0x83192A4C;
	sub_8318FDE0(ctx, base);
	// 83192A4C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83192A50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83192A54: 419A0018  beq cr6, 0x83192a6c
	if ctx.cr[6].eq {
	pc = 0x83192A6C; continue 'dispatch;
	}
	// 83192A58: 3C80FF03  lis r4, -0xfd
	ctx.r[4].s64 = -16580608;
	// 83192A5C: 6084020E  ori r4, r4, 0x20e
	ctx.r[4].u64 = ctx.r[4].u64 | 526;
	// 83192A60: 48000931  bl 0x83193390
	ctx.lr = 0x83192A64;
	sub_83193390(ctx, base);
	// 83192A64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83192A68: 48015754  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83192A6C: 817F13D4  lwz r11, 0x13d4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5076 as u32) ) } as u64;
	// 83192A70: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83192A74: 817F13D8  lwz r11, 0x13d8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5080 as u32) ) } as u64;
	// 83192A78: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83192A7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83192A80: 4801573C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192A88 size=112
    let mut pc: u32 = 0x83192A88;
    'dispatch: loop {
        match pc {
            0x83192A88 => {
    //   block [0x83192A88..0x83192AF8)
	// 83192A88: 546B003A  rlwinm r11, r3, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 83192A8C: 7D2B1850  subf r9, r11, r3
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83192A90: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 83192A94: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192A98: 552B1838  slwi r11, r9, 3
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83192A9C: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 83192AA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83192AA4: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192AA8: 7D0A5830  slw r10, r8, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83192AAC: 419A0014  beq cr6, 0x83192ac0
	if ctx.cr[6].eq {
	pc = 0x83192AC0; continue 'dispatch;
	}
	// 83192AB0: 210B0020  subfic r8, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[11].s64;
	// 83192AB4: 7D284430  srw r8, r9, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[9].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 83192AB8: 7D295830  slw r9, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83192ABC: 7D0A5378  or r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 83192AC0: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192AC4: 2B0A0101  cmplwi cr6, r10, 0x101
	ctx.cr[6].compare_u32(ctx.r[10].u32, 257 as u32, &mut ctx.xer);
	// 83192AC8: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 83192ACC: 409A0260  bne cr6, 0x83192d2c
	if !ctx.cr[6].eq {
		sub_83192D2C(ctx, base);
		return;
	}
	// 83192AD0: 2F0B001B  cmpwi cr6, r11, 0x1b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 27, &mut ctx.xer);
	// 83192AD4: 41980024  blt cr6, 0x83192af8
	if ctx.cr[6].lt {
		sub_83192AF8(ctx, base);
		return;
	}
	// 83192AD8: 396BFFE5  addi r11, r11, -0x1b
	ctx.r[11].s64 = ctx.r[11].s64 + -27;
	// 83192ADC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83192AE0: 7CEA5830  slw r10, r7, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83192AE4: 409A0008  bne cr6, 0x83192aec
	if !ctx.cr[6].eq {
	pc = 0x83192AEC; continue 'dispatch;
	}
	// 83192AE8: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 83192AEC: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192AF0: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 83192AF4: 4800000C  b 0x83192b00
	sub_83192AF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192AF8 size=48
    let mut pc: u32 = 0x83192AF8;
    'dispatch: loop {
        match pc {
            0x83192AF8 => {
    //   block [0x83192AF8..0x83192B28)
	// 83192AF8: 396B0005  addi r11, r11, 5
	ctx.r[11].s64 = ctx.r[11].s64 + 5;
	// 83192AFC: 552A2834  slwi r10, r9, 5
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83192B00: 7D490034  cntlzw r9, r10
	ctx.r[9].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 83192B04: 2F0B001F  cmpwi cr6, r11, 0x1f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 31, &mut ctx.xer);
	// 83192B08: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 83192B0C: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 83192B10: 409A0018  bne cr6, 0x83192b28
	if !ctx.cr[6].eq {
		sub_83192B28(ctx, base);
		return;
	}
	// 83192B14: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 83192B18: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192B1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83192B20: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 83192B24: 4800000C  b 0x83192b30
	sub_83192B28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192B28 size=56
    let mut pc: u32 = 0x83192B28;
    'dispatch: loop {
        match pc {
            0x83192B28 => {
    //   block [0x83192B28..0x83192B60)
	// 83192B28: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83192B2C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83192B30: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83192B34: 409A01F8  bne cr6, 0x83192d2c
	if !ctx.cr[6].eq {
		sub_83192D2C(ctx, base);
		return;
	}
	// 83192B38: 7D490034  cntlzw r9, r10
	ctx.r[9].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 83192B3C: 2F0B001F  cmpwi cr6, r11, 0x1f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 31, &mut ctx.xer);
	// 83192B40: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 83192B44: 5528DFFE  rlwinm r8, r9, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 83192B48: 409A0018  bne cr6, 0x83192b60
	if !ctx.cr[6].eq {
		sub_83192B60(ctx, base);
		return;
	}
	// 83192B4C: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 83192B50: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192B54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83192B58: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 83192B5C: 4800000C  b 0x83192b68
	sub_83192B60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192B60 size=96
    let mut pc: u32 = 0x83192B60;
    'dispatch: loop {
        match pc {
            0x83192B60 => {
    //   block [0x83192B60..0x83192BC0)
	// 83192B60: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83192B64: 5549083C  slwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83192B68: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83192B6C: 419A01C0  beq cr6, 0x83192d2c
	if ctx.cr[6].eq {
		sub_83192D2C(ctx, base);
		return;
	}
	// 83192B70: 552A36BE  srwi r10, r9, 0x1a
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(26);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83192B74: 2F0B001A  cmpwi cr6, r11, 0x1a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 26, &mut ctx.xer);
	// 83192B78: 40990010  ble cr6, 0x83192b88
	if !ctx.cr[6].gt {
	pc = 0x83192B88; continue 'dispatch;
	}
	// 83192B7C: 210B003A  subfic r8, r11, 0x3a
	ctx.xer.ca = ctx.r[11].u32 <= 58 as u32;
	ctx.r[8].s64 = (58 as i64) - ctx.r[11].s64;
	// 83192B80: 7CE84430  srw r8, r7, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[7].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 83192B84: 7D0A5378  or r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 83192B88: 2B0A000B  cmplwi cr6, r10, 0xb
	ctx.cr[6].compare_u32(ctx.r[10].u32, 11 as u32, &mut ctx.xer);
	// 83192B8C: 419A003C  beq cr6, 0x83192bc8
	if ctx.cr[6].eq {
		sub_83192BC8(ctx, base);
		return;
	}
	// 83192B90: 2B0A0015  cmplwi cr6, r10, 0x15
	ctx.cr[6].compare_u32(ctx.r[10].u32, 21 as u32, &mut ctx.xer);
	// 83192B94: 40990198  ble cr6, 0x83192d2c
	if !ctx.cr[6].gt {
		sub_83192D2C(ctx, base);
		return;
	}
	// 83192B98: 2B0A0017  cmplwi cr6, r10, 0x17
	ctx.cr[6].compare_u32(ctx.r[10].u32, 23 as u32, &mut ctx.xer);
	// 83192B9C: 41990190  bgt cr6, 0x83192d2c
	if ctx.cr[6].gt {
		sub_83192D2C(ctx, base);
		return;
	}
	// 83192BA0: 396B0005  addi r11, r11, 5
	ctx.r[11].s64 = ctx.r[11].s64 + 5;
	// 83192BA4: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 83192BA8: 41980018  blt cr6, 0x83192bc0
	if ctx.cr[6].lt {
		sub_83192BC0(ctx, base);
		return;
	}
	// 83192BAC: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 83192BB0: 7CE95830  slw r9, r7, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83192BB4: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192BB8: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 83192BBC: 48000030  b 0x83192bec
	sub_83192BE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192BC0 size=8
    let mut pc: u32 = 0x83192BC0;
    'dispatch: loop {
        match pc {
            0x83192BC0 => {
    //   block [0x83192BC0..0x83192BC8)
	// 83192BC0: 55292834  slwi r9, r9, 5
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83192BC4: 48000028  b 0x83192bec
	sub_83192BE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192BC8 size=32
    let mut pc: u32 = 0x83192BC8;
    'dispatch: loop {
        match pc {
            0x83192BC8 => {
    //   block [0x83192BC8..0x83192BE8)
	// 83192BC8: 396B0006  addi r11, r11, 6
	ctx.r[11].s64 = ctx.r[11].s64 + 6;
	// 83192BCC: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 83192BD0: 41980018  blt cr6, 0x83192be8
	if ctx.cr[6].lt {
		sub_83192BE8(ctx, base);
		return;
	}
	// 83192BD4: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 83192BD8: 7CE95830  slw r9, r7, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83192BDC: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192BE0: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 83192BE4: 48000008  b 0x83192bec
	sub_83192BE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192BE8 size=72
    let mut pc: u32 = 0x83192BE8;
    'dispatch: loop {
        match pc {
            0x83192BE8 => {
    //   block [0x83192BE8..0x83192C30)
	// 83192BE8: 55293032  slwi r9, r9, 6
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(6);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83192BEC: 3905FFFF  addi r8, r5, -1
	ctx.r[8].s64 = ctx.r[5].s64 + -1;
	// 83192BF0: 552A5D7E  srwi r10, r9, 0x15
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(21);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83192BF4: 2F0B0015  cmpwi cr6, r11, 0x15
	ctx.cr[6].compare_i32(ctx.r[11].s32, 21, &mut ctx.xer);
	// 83192BF8: 40990010  ble cr6, 0x83192c08
	if !ctx.cr[6].gt {
	pc = 0x83192C08; continue 'dispatch;
	}
	// 83192BFC: 20AB0035  subfic r5, r11, 0x35
	ctx.xer.ca = ctx.r[11].u32 <= 53 as u32;
	ctx.r[5].s64 = (53 as i64) - ctx.r[11].s64;
	// 83192C00: 7CE52C30  srw r5, r7, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[5].u64 = 0;
	} else {
		ctx.r[5].u64 = ((ctx.r[7].u32) >> ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 83192C04: 7CAA5378  or r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 | ctx.r[10].u64;
	// 83192C08: 2B0A0008  cmplwi cr6, r10, 8
	ctx.cr[6].compare_u32(ctx.r[10].u32, 8 as u32, &mut ctx.xer);
	// 83192C0C: 409A0034  bne cr6, 0x83192c40
	if !ctx.cr[6].eq {
		sub_83192C30(ctx, base);
		return;
	}
	// 83192C10: 396B000B  addi r11, r11, 0xb
	ctx.r[11].s64 = ctx.r[11].s64 + 11;
	// 83192C14: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 83192C18: 41980018  blt cr6, 0x83192c30
	if ctx.cr[6].lt {
		sub_83192C30(ctx, base);
		return;
	}
	// 83192C1C: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 83192C20: 7CE95830  slw r9, r7, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83192C24: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192C28: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 83192C2C: 48000008  b 0x83192c34
	sub_83192C30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192C30 size=104
    let mut pc: u32 = 0x83192C30;
    'dispatch: loop {
        match pc {
            0x83192C30 => {
    //   block [0x83192C30..0x83192C98)
	// 83192C30: 55295828  slwi r9, r9, 0xb
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(11);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83192C34: 3908FFDF  addi r8, r8, -0x21
	ctx.r[8].s64 = ctx.r[8].s64 + -33;
	// 83192C38: 2F080021  cmpwi cr6, r8, 0x21
	ctx.cr[6].compare_i32(ctx.r[8].s32, 33, &mut ctx.xer);
	// 83192C3C: 4199FFB4  bgt cr6, 0x83192bf0
	if ctx.cr[6].gt {
		sub_83192BE8(ctx, base);
		return;
	}
	// 83192C40: 3948FFFF  addi r10, r8, -1
	ctx.r[10].s64 = ctx.r[8].s64 + -1;
	// 83192C44: 2B0A0020  cmplwi cr6, r10, 0x20
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32 as u32, &mut ctx.xer);
	// 83192C48: 419900E4  bgt cr6, 0x83192d2c
	if ctx.cr[6].gt {
		sub_83192D2C(ctx, base);
		return;
	}
	// 83192C4C: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 83192C50: 5508083C  slwi r8, r8, 1
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83192C54: 394ABC80  addi r10, r10, -0x4380
	ctx.r[10].s64 = ctx.r[10].s64 + -17280;
	// 83192C58: 7CA8522E  lhzx r5, r8, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83192C5C: 54AA063E  clrlwi r10, r5, 0x18
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 83192C60: 210A0020  subfic r8, r10, 0x20
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[10].s64;
	// 83192C64: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83192C68: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83192C6C: 4198003C  blt cr6, 0x83192ca8
	if ctx.cr[6].lt {
		sub_83192CA8(ctx, base);
		return;
	}
	// 83192C70: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 83192C74: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83192C78: 419A0020  beq cr6, 0x83192c98
	if ctx.cr[6].eq {
		sub_83192C98(ctx, base);
		return;
	}
	// 83192C7C: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83192C80: 7CEA5430  srw r10, r7, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[7].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 83192C84: 7D494B78  or r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 83192C88: 7CEA5830  slw r10, r7, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83192C8C: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192C90: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 83192C94: 48000018  b 0x83192cac
	sub_83192CA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192C98 size=16
    let mut pc: u32 = 0x83192C98;
    'dispatch: loop {
        match pc {
            0x83192C98 => {
    //   block [0x83192C98..0x83192CA8)
	// 83192C98: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 83192C9C: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192CA0: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 83192CA4: 48000008  b 0x83192cac
	sub_83192CA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192CA8 size=80
    let mut pc: u32 = 0x83192CA8;
    'dispatch: loop {
        match pc {
            0x83192CA8 => {
    //   block [0x83192CA8..0x83192CF8)
	// 83192CA8: 7D2A5030  slw r10, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 83192CAC: 7D284430  srw r8, r9, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[9].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 83192CB0: 7CA90734  extsh r9, r5
	ctx.r[9].s64 = ctx.r[5].s16 as i64;
	// 83192CB4: 5529C23E  srwi r9, r9, 8
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83192CB8: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83192CBC: 409A0070  bne cr6, 0x83192d2c
	if !ctx.cr[6].eq {
		sub_83192D2C(ctx, base);
		return;
	}
	// 83192CC0: 554A36BE  srwi r10, r10, 0x1a
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(26);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83192CC4: 2F0B001A  cmpwi cr6, r11, 0x1a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 26, &mut ctx.xer);
	// 83192CC8: 40990010  ble cr6, 0x83192cd8
	if !ctx.cr[6].gt {
	pc = 0x83192CD8; continue 'dispatch;
	}
	// 83192CCC: 212B003A  subfic r9, r11, 0x3a
	ctx.xer.ca = ctx.r[11].u32 <= 58 as u32;
	ctx.r[9].s64 = (58 as i64) - ctx.r[11].s64;
	// 83192CD0: 7CE94C30  srw r9, r7, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[7].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 83192CD4: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83192CD8: 2B0A000B  cmplwi cr6, r10, 0xb
	ctx.cr[6].compare_u32(ctx.r[10].u32, 11 as u32, &mut ctx.xer);
	// 83192CDC: 419A001C  beq cr6, 0x83192cf8
	if ctx.cr[6].eq {
		sub_83192CF8(ctx, base);
		return;
	}
	// 83192CE0: 2B0A0015  cmplwi cr6, r10, 0x15
	ctx.cr[6].compare_u32(ctx.r[10].u32, 21 as u32, &mut ctx.xer);
	// 83192CE4: 40990048  ble cr6, 0x83192d2c
	if !ctx.cr[6].gt {
		sub_83192D2C(ctx, base);
		return;
	}
	// 83192CE8: 2B0A0017  cmplwi cr6, r10, 0x17
	ctx.cr[6].compare_u32(ctx.r[10].u32, 23 as u32, &mut ctx.xer);
	// 83192CEC: 41990040  bgt cr6, 0x83192d2c
	if ctx.cr[6].gt {
		sub_83192D2C(ctx, base);
		return;
	}
	// 83192CF0: 396B0005  addi r11, r11, 5
	ctx.r[11].s64 = ctx.r[11].s64 + 5;
	// 83192CF4: 48000008  b 0x83192cfc
	sub_83192CF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192CF8 size=52
    let mut pc: u32 = 0x83192CF8;
    'dispatch: loop {
        match pc {
            0x83192CF8 => {
    //   block [0x83192CF8..0x83192D2C)
	// 83192CF8: 396B0006  addi r11, r11, 6
	ctx.r[11].s64 = ctx.r[11].s64 + 6;
	// 83192CFC: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 83192D00: 4198000C  blt cr6, 0x83192d0c
	if ctx.cr[6].lt {
	pc = 0x83192D0C; continue 'dispatch;
	}
	// 83192D04: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 83192D08: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 83192D0C: 396B0007  addi r11, r11, 7
	ctx.r[11].s64 = ctx.r[11].s64 + 7;
	// 83192D10: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 83192D14: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 83192D18: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83192D1C: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 83192D20: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 83192D24: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 83192D28: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192D2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192D2C size=8
    let mut pc: u32 = 0x83192D2C;
    'dispatch: loop {
        match pc {
            0x83192D2C => {
    //   block [0x83192D2C..0x83192D34)
	// 83192D2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83192D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83192D38 size=848
    let mut pc: u32 = 0x83192D38;
    'dispatch: loop {
        match pc {
            0x83192D38 => {
    //   block [0x83192D38..0x83193088)
	// 83192D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83192D3C: 48015431  bl 0x831a816c
	ctx.lr = 0x83192D40;
	sub_831A8130(ctx, base);
	// 83192D40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83192D44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83192D48: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83192D4C: 57CB003A  rlwinm r11, r30, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	// 83192D50: 7D2BF050  subf r9, r11, r30
	ctx.r[9].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 83192D54: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 83192D58: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192D5C: 552B1838  slwi r11, r9, 3
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83192D60: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 83192D64: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83192D68: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192D6C: 7D0A5830  slw r10, r8, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83192D70: 419A0014  beq cr6, 0x83192d84
	if ctx.cr[6].eq {
	pc = 0x83192D84; continue 'dispatch;
	}
	// 83192D74: 210B0020  subfic r8, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[11].s64;
	// 83192D78: 7D284430  srw r8, r9, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[9].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 83192D7C: 7D295830  slw r9, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83192D80: 7D0A5378  or r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 83192D84: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192D88: 2B0A0101  cmplwi cr6, r10, 0x101
	ctx.cr[6].compare_u32(ctx.r[10].u32, 257 as u32, &mut ctx.xer);
	// 83192D8C: 38860004  addi r4, r6, 4
	ctx.r[4].s64 = ctx.r[6].s64 + 4;
	// 83192D90: 409A0254  bne cr6, 0x83192fe4
	if !ctx.cr[6].eq {
	pc = 0x83192FE4; continue 'dispatch;
	}
	// 83192D94: 2F0B001B  cmpwi cr6, r11, 0x1b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 27, &mut ctx.xer);
	// 83192D98: 41980024  blt cr6, 0x83192dbc
	if ctx.cr[6].lt {
	pc = 0x83192DBC; continue 'dispatch;
	}
	// 83192D9C: 396BFFE5  addi r11, r11, -0x1b
	ctx.r[11].s64 = ctx.r[11].s64 + -27;
	// 83192DA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83192DA4: 7CEA5830  slw r10, r7, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83192DA8: 409A0008  bne cr6, 0x83192db0
	if !ctx.cr[6].eq {
	pc = 0x83192DB0; continue 'dispatch;
	}
	// 83192DAC: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 83192DB0: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192DB4: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 83192DB8: 4800000C  b 0x83192dc4
	pc = 0x83192DC4; continue 'dispatch;
	// 83192DBC: 396B0005  addi r11, r11, 5
	ctx.r[11].s64 = ctx.r[11].s64 + 5;
	// 83192DC0: 552A2834  slwi r10, r9, 5
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83192DC4: 7D490034  cntlzw r9, r10
	ctx.r[9].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 83192DC8: 2F0B001F  cmpwi cr6, r11, 0x1f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 31, &mut ctx.xer);
	// 83192DCC: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 83192DD0: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 83192DD4: 409A0018  bne cr6, 0x83192dec
	if !ctx.cr[6].eq {
	pc = 0x83192DEC; continue 'dispatch;
	}
	// 83192DD8: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 83192DDC: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83192DE4: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 83192DE8: 4800000C  b 0x83192df4
	pc = 0x83192DF4; continue 'dispatch;
	// 83192DEC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83192DF0: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83192DF4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83192DF8: 409A01EC  bne cr6, 0x83192fe4
	if !ctx.cr[6].eq {
	pc = 0x83192FE4; continue 'dispatch;
	}
	// 83192DFC: 7D490034  cntlzw r9, r10
	ctx.r[9].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 83192E00: 2F0B001F  cmpwi cr6, r11, 0x1f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 31, &mut ctx.xer);
	// 83192E04: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 83192E08: 5528DFFE  rlwinm r8, r9, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 83192E0C: 409A0018  bne cr6, 0x83192e24
	if !ctx.cr[6].eq {
	pc = 0x83192E24; continue 'dispatch;
	}
	// 83192E10: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 83192E14: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83192E1C: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 83192E20: 4800000C  b 0x83192e2c
	pc = 0x83192E2C; continue 'dispatch;
	// 83192E24: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83192E28: 5549083C  slwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83192E2C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83192E30: 419A01B4  beq cr6, 0x83192fe4
	if ctx.cr[6].eq {
	pc = 0x83192FE4; continue 'dispatch;
	}
	// 83192E34: 552A2EFE  srwi r10, r9, 0x1b
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(27);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83192E38: 2F0B001B  cmpwi cr6, r11, 0x1b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 27, &mut ctx.xer);
	// 83192E3C: 40990010  ble cr6, 0x83192e4c
	if !ctx.cr[6].gt {
	pc = 0x83192E4C; continue 'dispatch;
	}
	// 83192E40: 210B003B  subfic r8, r11, 0x3b
	ctx.xer.ca = ctx.r[11].u32 <= 59 as u32;
	ctx.r[8].s64 = (59 as i64) - ctx.r[11].s64;
	// 83192E44: 7CE84430  srw r8, r7, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[7].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 83192E48: 7D0A5378  or r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 83192E4C: 2B0A0007  cmplwi cr6, r10, 7
	ctx.cr[6].compare_u32(ctx.r[10].u32, 7 as u32, &mut ctx.xer);
	// 83192E50: 409A0194  bne cr6, 0x83192fe4
	if !ctx.cr[6].eq {
	pc = 0x83192FE4; continue 'dispatch;
	}
	// 83192E54: 396B0005  addi r11, r11, 5
	ctx.r[11].s64 = ctx.r[11].s64 + 5;
	// 83192E58: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 83192E5C: 41980018  blt cr6, 0x83192e74
	if ctx.cr[6].lt {
	pc = 0x83192E74; continue 'dispatch;
	}
	// 83192E60: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 83192E64: 7CE85830  slw r8, r7, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83192E68: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192E6C: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 83192E70: 48000008  b 0x83192e78
	pc = 0x83192E78; continue 'dispatch;
	// 83192E74: 55282834  slwi r8, r9, 5
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83192E78: 3925FFFF  addi r9, r5, -1
	ctx.r[9].s64 = ctx.r[5].s64 + -1;
	// 83192E7C: 550A5D7E  srwi r10, r8, 0x15
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shr(21);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83192E80: 2F0B0015  cmpwi cr6, r11, 0x15
	ctx.cr[6].compare_i32(ctx.r[11].s32, 21, &mut ctx.xer);
	// 83192E84: 40990010  ble cr6, 0x83192e94
	if !ctx.cr[6].gt {
	pc = 0x83192E94; continue 'dispatch;
	}
	// 83192E88: 20CB0035  subfic r6, r11, 0x35
	ctx.xer.ca = ctx.r[11].u32 <= 53 as u32;
	ctx.r[6].s64 = (53 as i64) - ctx.r[11].s64;
	// 83192E8C: 7CE63430  srw r6, r7, r6
	if (ctx.r[6].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[7].u32) >> ((ctx.r[6].u8 & 0x1F) as u32)) as u64;
	}
	// 83192E90: 7CCA5378  or r10, r6, r10
	ctx.r[10].u64 = ctx.r[6].u64 | ctx.r[10].u64;
	// 83192E94: 2B0A0008  cmplwi cr6, r10, 8
	ctx.cr[6].compare_u32(ctx.r[10].u32, 8 as u32, &mut ctx.xer);
	// 83192E98: 409A0034  bne cr6, 0x83192ecc
	if !ctx.cr[6].eq {
	pc = 0x83192ECC; continue 'dispatch;
	}
	// 83192E9C: 396B000B  addi r11, r11, 0xb
	ctx.r[11].s64 = ctx.r[11].s64 + 11;
	// 83192EA0: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 83192EA4: 41980018  blt cr6, 0x83192ebc
	if ctx.cr[6].lt {
	pc = 0x83192EBC; continue 'dispatch;
	}
	// 83192EA8: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 83192EAC: 7CE85830  slw r8, r7, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83192EB0: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192EB4: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 83192EB8: 48000008  b 0x83192ec0
	pc = 0x83192EC0; continue 'dispatch;
	// 83192EBC: 55085828  slwi r8, r8, 0xb
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(11);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83192EC0: 3929FFDF  addi r9, r9, -0x21
	ctx.r[9].s64 = ctx.r[9].s64 + -33;
	// 83192EC4: 2F090021  cmpwi cr6, r9, 0x21
	ctx.cr[6].compare_i32(ctx.r[9].s32, 33, &mut ctx.xer);
	// 83192EC8: 4199FFB4  bgt cr6, 0x83192e7c
	if ctx.cr[6].gt {
	pc = 0x83192E7C; continue 'dispatch;
	}
	// 83192ECC: 3949FFFF  addi r10, r9, -1
	ctx.r[10].s64 = ctx.r[9].s64 + -1;
	// 83192ED0: 2B0A0020  cmplwi cr6, r10, 0x20
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32 as u32, &mut ctx.xer);
	// 83192ED4: 41990110  bgt cr6, 0x83192fe4
	if ctx.cr[6].gt {
	pc = 0x83192FE4; continue 'dispatch;
	}
	// 83192ED8: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 83192EDC: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83192EE0: 394ABC80  addi r10, r10, -0x4380
	ctx.r[10].s64 = ctx.r[10].s64 + -17280;
	// 83192EE4: 7CC9522E  lhzx r6, r9, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83192EE8: 54CA063E  clrlwi r10, r6, 0x18
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 83192EEC: 212A0020  subfic r9, r10, 0x20
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[9].s64 = (32 as i64) - ctx.r[10].s64;
	// 83192EF0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83192EF4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83192EF8: 4198003C  blt cr6, 0x83192f34
	if ctx.cr[6].lt {
	pc = 0x83192F34; continue 'dispatch;
	}
	// 83192EFC: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 83192F00: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83192F04: 419A0020  beq cr6, 0x83192f24
	if ctx.cr[6].eq {
	pc = 0x83192F24; continue 'dispatch;
	}
	// 83192F08: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83192F0C: 7CEA5430  srw r10, r7, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[7].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 83192F10: 7D484378  or r8, r10, r8
	ctx.r[8].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 83192F14: 7CEA5830  slw r10, r7, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83192F18: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192F1C: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 83192F20: 48000018  b 0x83192f38
	pc = 0x83192F38; continue 'dispatch;
	// 83192F24: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 83192F28: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192F2C: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 83192F30: 48000008  b 0x83192f38
	pc = 0x83192F38; continue 'dispatch;
	// 83192F34: 7D0A5030  slw r10, r8, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 83192F38: 7D094C30  srw r9, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 83192F3C: 7CC80734  extsh r8, r6
	ctx.r[8].s64 = ctx.r[6].s16 as i64;
	// 83192F40: 5508C23E  srwi r8, r8, 8
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(8);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83192F44: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 83192F48: 409A009C  bne cr6, 0x83192fe4
	if !ctx.cr[6].eq {
	pc = 0x83192FE4; continue 'dispatch;
	}
	// 83192F4C: 554A2EFE  srwi r10, r10, 0x1b
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(27);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83192F50: 2F0B001B  cmpwi cr6, r11, 0x1b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 27, &mut ctx.xer);
	// 83192F54: 40990010  ble cr6, 0x83192f64
	if !ctx.cr[6].gt {
	pc = 0x83192F64; continue 'dispatch;
	}
	// 83192F58: 212B003B  subfic r9, r11, 0x3b
	ctx.xer.ca = ctx.r[11].u32 <= 59 as u32;
	ctx.r[9].s64 = (59 as i64) - ctx.r[11].s64;
	// 83192F5C: 7CE94C30  srw r9, r7, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[7].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 83192F60: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83192F64: 2B0A0007  cmplwi cr6, r10, 7
	ctx.cr[6].compare_u32(ctx.r[10].u32, 7 as u32, &mut ctx.xer);
	// 83192F68: 409A007C  bne cr6, 0x83192fe4
	if !ctx.cr[6].eq {
	pc = 0x83192FE4; continue 'dispatch;
	}
	// 83192F6C: 396B0005  addi r11, r11, 5
	ctx.r[11].s64 = ctx.r[11].s64 + 5;
	// 83192F70: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 83192F74: 4198000C  blt cr6, 0x83192f80
	if ctx.cr[6].lt {
	pc = 0x83192F80; continue 'dispatch;
	}
	// 83192F78: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 83192F7C: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 83192F80: 396B0007  addi r11, r11, 7
	ctx.r[11].s64 = ctx.r[11].s64 + 7;
	// 83192F84: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 83192F88: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 83192F8C: 386BFFF8  addi r3, r11, -8
	ctx.r[3].s64 = ctx.r[11].s64 + -8;
	// 83192F90: 7D7E1850  subf r11, r30, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[30].s64;
	// 83192F94: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 83192F98: 4199004C  bgt cr6, 0x83192fe4
	if ctx.cr[6].gt {
	pc = 0x83192FE4; continue 'dispatch;
	}
	// 83192F9C: 38A000CC  li r5, 0xcc
	ctx.r[5].s64 = 204;
	// 83192FA0: 7C8BE850  subf r4, r11, r29
	ctx.r[4].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 83192FA4: 4BFFF835  bl 0x831927d8
	ctx.lr = 0x83192FA8;
	sub_831927D8(ctx, base);
	// 83192FA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83192FAC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83192FB0: 419A0034  beq cr6, 0x83192fe4
	if ctx.cr[6].eq {
	pc = 0x83192FE4; continue 'dispatch;
	}
	// 83192FB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83192FB8: 4BFFF771  bl 0x83192728
	ctx.lr = 0x83192FBC;
	sub_83192728(ctx, base);
	// 83192FBC: 546B077A  rlwinm r11, r3, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 83192FC0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83192FC4: 419A002C  beq cr6, 0x83192ff0
	if ctx.cr[6].eq {
	pc = 0x83192FF0; continue 'dispatch;
	}
	// 83192FC8: 897F0005  lbz r11, 5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 83192FCC: 895F0006  lbz r10, 6(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 83192FD0: 556B0F7C  rlwinm r11, r11, 1, 0x1d, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 83192FD4: 554AC9FE  srwi r10, r10, 7
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(7);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83192FD8: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 83192FDC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83192FE0: 419A001C  beq cr6, 0x83192ffc
	if ctx.cr[6].eq {
	pc = 0x83192FFC; continue 'dispatch;
	}
	// 83192FE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83192FE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83192FEC: 480151D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83192FF0: 546B0672  rlwinm r11, r3, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 83192FF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83192FF8: 419A0030  beq cr6, 0x83193028
	if ctx.cr[6].eq {
	pc = 0x83193028; continue 'dispatch;
	}
	// 83192FFC: 7D7EF850  subf r11, r30, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 83193000: 387F0001  addi r3, r31, 1
	ctx.r[3].s64 = ctx.r[31].s64 + 1;
	// 83193004: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83193008: 38A000CC  li r5, 0xcc
	ctx.r[5].s64 = 204;
	// 8319300C: 7C8BE850  subf r4, r11, r29
	ctx.r[4].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 83193010: 4BFFF7C9  bl 0x831927d8
	ctx.lr = 0x83193014;
	sub_831927D8(ctx, base);
	// 83193014: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83193018: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8319301C: 409AFF98  bne cr6, 0x83192fb4
	if !ctx.cr[6].eq {
	pc = 0x83192FB4; continue 'dispatch;
	}
	// 83193020: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83193024: 48015198  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83193028: 546B0630  rlwinm r11, r3, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 8319302C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83193030: 419A0020  beq cr6, 0x83193050
	if ctx.cr[6].eq {
	pc = 0x83193050; continue 'dispatch;
	}
	// 83193034: 7D7EF850  subf r11, r30, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 83193038: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8319303C: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 83193040: 4099003C  ble cr6, 0x8319307c
	if !ctx.cr[6].gt {
	pc = 0x8319307C; continue 'dispatch;
	}
	// 83193044: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83193048: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8319304C: 48015170  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83193050: 546B0738  rlwinm r11, r3, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 83193054: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83193058: 419AFF8C  beq cr6, 0x83192fe4
	if ctx.cr[6].eq {
	pc = 0x83192FE4; continue 'dispatch;
	}
	// 8319305C: 897F0007  lbz r11, 7(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(7 as u32) ) } as u64;
	// 83193060: 556B0672  rlwinm r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83193064: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83193068: 419AFF7C  beq cr6, 0x83192fe4
	if ctx.cr[6].eq {
	pc = 0x83192FE4; continue 'dispatch;
	}
	// 8319306C: 7D7EF850  subf r11, r30, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 83193070: 396B0007  addi r11, r11, 7
	ctx.r[11].s64 = ctx.r[11].s64 + 7;
	// 83193074: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 83193078: 4199FF6C  bgt cr6, 0x83192fe4
	if ctx.cr[6].gt {
	pc = 0x83192FE4; continue 'dispatch;
	}
	// 8319307C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83193080: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83193084: 48015138  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193088 size=52
    let mut pc: u32 = 0x83193088;
    'dispatch: loop {
        match pc {
            0x83193088 => {
    //   block [0x83193088..0x831930BC)
	// 83193088: 54AA073E  clrlwi r10, r5, 0x1c
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x0000000Fu64;
	// 8319308C: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83193090: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83193094: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 83193098: 419A0018  beq cr6, 0x831930b0
	if ctx.cr[6].eq {
	pc = 0x831930B0; continue 'dispatch;
	}
	// 8319309C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 831930A0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 831930A4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831930A8: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 831930AC: 409AFFF0  bne cr6, 0x8319309c
	if !ctx.cr[6].eq {
	pc = 0x8319309C; continue 'dispatch;
	}
	// 831930B0: 54AAE13E  srwi r10, r5, 4
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831930B4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831930B8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831930BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831930BC size=144
    let mut pc: u32 = 0x831930BC;
    'dispatch: loop {
        match pc {
            0x831930BC => {
    //   block [0x831930BC..0x8319314C)
	// 831930BC: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 831930C0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 831930C4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831930C8: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 831930CC: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 831930D0: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 831930D4: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 831930D8: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 831930DC: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 831930E0: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 831930E4: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 831930E8: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 831930EC: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 831930F0: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 831930F4: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 831930F8: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 831930FC: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 83193100: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 83193104: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 83193108: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8319310C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 83193110: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 83193114: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 83193118: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8319311C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 83193120: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 83193124: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 83193128: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8319312C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 83193130: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 83193134: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 83193138: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8319313C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 83193140: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 83193144: 409AFF78  bne cr6, 0x831930bc
	if !ctx.cr[6].eq {
	pc = 0x831930BC; continue 'dispatch;
	}
	// 83193148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83193150 size=244
    let mut pc: u32 = 0x83193150;
    'dispatch: loop {
        match pc {
            0x83193150 => {
    //   block [0x83193150..0x83193244)
	// 83193150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83193154: 48015011  bl 0x831a8164
	ctx.lr = 0x83193158;
	sub_831A8130(ctx, base);
	// 83193158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319315C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83193160: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83193164: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83193168: 4BFFCC79  bl 0x8318fde0
	ctx.lr = 0x8319316C;
	sub_8318FDE0(ctx, base);
	// 8319316C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83193170: 419A001C  beq cr6, 0x8319318c
	if ctx.cr[6].eq {
	pc = 0x8319318C; continue 'dispatch;
	}
	// 83193174: 3C80FF03  lis r4, -0xfd
	ctx.r[4].s64 = -16580608;
	// 83193178: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8319317C: 60840209  ori r4, r4, 0x209
	ctx.r[4].u64 = ctx.r[4].u64 | 521;
	// 83193180: 48000211  bl 0x83193390
	ctx.lr = 0x83193184;
	sub_83193390(ctx, base);
	// 83193184: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83193188: 4801502C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8319318C: 817F147C  lwz r11, 0x147c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5244 as u32) ) } as u64;
	// 83193190: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83193194: 409A001C  bne cr6, 0x831931b0
	if !ctx.cr[6].eq {
	pc = 0x831931B0; continue 'dispatch;
	}
	// 83193198: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8319319C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 831931A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831931A4: 4BFDB315  bl 0x8316e4b8
	ctx.lr = 0x831931A8;
	sub_8316E4B8(ctx, base);
	// 831931A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831931AC: 48015008  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 831931B0: 387F1370  addi r3, r31, 0x1370
	ctx.r[3].s64 = ctx.r[31].s64 + 4976;
	// 831931B4: 83BF1368  lwz r29, 0x1368(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4968 as u32) ) } as u64;
	// 831931B8: 38A00044  li r5, 0x44
	ctx.r[5].s64 = 68;
	// 831931BC: 839F136C  lwz r28, 0x136c(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4972 as u32) ) } as u64;
	// 831931C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831931C4: 4801534D  bl 0x831a8510
	ctx.lr = 0x831931C8;
	sub_831A8510(ctx, base);
	// 831931C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831931CC: 48007B0D  bl 0x8319acd8
	ctx.lr = 0x831931D0;
	sub_8319ACD8(ctx, base);
	// 831931D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831931D4: 48008445  bl 0x8319b618
	ctx.lr = 0x831931D8;
	sub_8319B618(ctx, base);
	// 831931D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831931DC: 48008485  bl 0x8319b660
	ctx.lr = 0x831931E0;
	sub_8319B660(ctx, base);
	// 831931E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831931E4: 48007EDD  bl 0x8319b0c0
	ctx.lr = 0x831931E8;
	sub_8319B0C0(ctx, base);
	// 831931E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831931EC: 48008295  bl 0x8319b480
	ctx.lr = 0x831931F0;
	sub_8319B480(ctx, base);
	// 831931F0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 831931F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831931F8: 48007721  bl 0x8319a918
	ctx.lr = 0x831931FC;
	sub_8319A918(ctx, base);
	// 831931FC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83193200: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83193204: 4BF348DD  bl 0x830c7ae0
	ctx.lr = 0x83193208;
	sub_830C7AE0(ctx, base);
	// 83193208: 389F12DC  addi r4, r31, 0x12dc
	ctx.r[4].s64 = ctx.r[31].s64 + 4828;
	// 8319320C: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 83193210: 807E0034  lwz r3, 0x34(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83193214: 480152FD  bl 0x831a8510
	ctx.lr = 0x83193218;
	sub_831A8510(ctx, base);
	// 83193218: 817F1368  lwz r11, 0x1368(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4968 as u32) ) } as u64;
	// 8319321C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83193220: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 83193224: 917E0038  stw r11, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83193228: 817F136C  lwz r11, 0x136c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4972 as u32) ) } as u64;
	// 8319322C: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 83193230: 917E003C  stw r11, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 83193234: A17F13B0  lhz r11, 0x13b0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(5040 as u32) ) } as u64;
	// 83193238: B17E0040  sth r11, 0x40(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[11].u16 ) };
	// 8319323C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83193240: 48014F74  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83193248 size=176
    let mut pc: u32 = 0x83193248;
    'dispatch: loop {
        match pc {
            0x83193248 => {
    //   block [0x83193248..0x831932F8)
	// 83193248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319324C: 48014F21  bl 0x831a816c
	ctx.lr = 0x83193250;
	sub_831A8130(ctx, base);
	// 83193250: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83193254: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83193258: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8319325C: 4BFFCB85  bl 0x8318fde0
	ctx.lr = 0x83193260;
	sub_8318FDE0(ctx, base);
	// 83193260: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83193264: 419A001C  beq cr6, 0x83193280
	if ctx.cr[6].eq {
	pc = 0x83193280; continue 'dispatch;
	}
	// 83193268: 3C80FF03  lis r4, -0xfd
	ctx.r[4].s64 = -16580608;
	// 8319326C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83193270: 6084020A  ori r4, r4, 0x20a
	ctx.r[4].u64 = ctx.r[4].u64 | 522;
	// 83193274: 4800011D  bl 0x83193390
	ctx.lr = 0x83193278;
	sub_83193390(ctx, base);
	// 83193278: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8319327C: 48014F40  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83193280: 3FC0FF03  lis r30, -0xfd
	ctx.r[30].s64 = -16580608;
	// 83193284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83193288: 63DE0305  ori r30, r30, 0x305
	ctx.r[30].u64 = ctx.r[30].u64 | 773;
	// 8319328C: 4BFFDB7D  bl 0x83190e08
	ctx.lr = 0x83193290;
	sub_83190E08(ctx, base);
	// 83193290: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83193294: 419A0050  beq cr6, 0x831932e4
	if ctx.cr[6].eq {
	pc = 0x831932E4; continue 'dispatch;
	}
	// 83193298: 706B00CC  andi. r11, r3, 0xcc
	ctx.r[11].u64 = ctx.r[3].u64 & 204;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8319329C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831932A0: 409A0040  bne cr6, 0x831932e0
	if !ctx.cr[6].eq {
	pc = 0x831932E0; continue 'dispatch;
	}
	// 831932A4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 831932A8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831932AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831932B0: 4BFFDE39  bl 0x831910e8
	ctx.lr = 0x831932B4;
	sub_831910E8(ctx, base);
	// 831932B4: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 831932B8: 409A002C  bne cr6, 0x831932e4
	if !ctx.cr[6].eq {
	pc = 0x831932E4; continue 'dispatch;
	}
	// 831932BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831932C0: 4BFFDB49  bl 0x83190e08
	ctx.lr = 0x831932C4;
	sub_83190E08(ctx, base);
	// 831932C4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831932C8: 409AFFD0  bne cr6, 0x83193298
	if !ctx.cr[6].eq {
	pc = 0x83193298; continue 'dispatch;
	}
	// 831932CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831932D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831932D4: 480000BD  bl 0x83193390
	ctx.lr = 0x831932D8;
	sub_83193390(ctx, base);
	// 831932D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831932DC: 48014EE0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831932E0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831932E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831932E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831932EC: 480000A5  bl 0x83193390
	ctx.lr = 0x831932F0;
	sub_83193390(ctx, base);
	// 831932F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831932F4: 48014EC8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831932F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831932F8 size=132
    let mut pc: u32 = 0x831932F8;
    'dispatch: loop {
        match pc {
            0x831932F8 => {
    //   block [0x831932F8..0x8319337C)
	// 831932F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831932FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83193300: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83193304: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83193308: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8319330C: 4BFFCAD5  bl 0x8318fde0
	ctx.lr = 0x83193310;
	sub_8318FDE0(ctx, base);
	// 83193310: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83193314: 419A0028  beq cr6, 0x8319333c
	if ctx.cr[6].eq {
	pc = 0x8319333C; continue 'dispatch;
	}
	// 83193318: 3C80FF03  lis r4, -0xfd
	ctx.r[4].s64 = -16580608;
	// 8319331C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83193320: 60840211  ori r4, r4, 0x211
	ctx.r[4].u64 = ctx.r[4].u64 | 529;
	// 83193324: 4800006D  bl 0x83193390
	ctx.lr = 0x83193328;
	sub_83193390(ctx, base);
	// 83193328: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8319332C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83193330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83193334: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83193338: 4E800020  blr
	return;
	// 8319333C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83193340: 815F147C  lwz r10, 0x147c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5244 as u32) ) } as u64;
	// 83193344: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 83193348: 917F1528  stw r11, 0x1528(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5416 as u32), ctx.r[11].u32 ) };
	// 8319334C: 409A000C  bne cr6, 0x83193358
	if !ctx.cr[6].eq {
	pc = 0x83193358; continue 'dispatch;
	}
	// 83193350: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83193354: 4BF3478D  bl 0x830c7ae0
	ctx.lr = 0x83193358;
	sub_830C7AE0(ctx, base);
	// 83193358: 807F152C  lwz r3, 0x152c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5420 as u32) ) } as u64;
	// 8319335C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83193360: 419A0008  beq cr6, 0x83193368
	if ctx.cr[6].eq {
	pc = 0x83193368; continue 'dispatch;
	}
	// 83193364: 4800843D  bl 0x8319b7a0
	ctx.lr = 0x83193368;
	sub_8319B7A0(ctx, base);
	// 83193368: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8319336C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83193370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83193374: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83193378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193380 size=12
    let mut pc: u32 = 0x83193380;
    'dispatch: loop {
        match pc {
            0x83193380 => {
    //   block [0x83193380..0x8319338C)
	// 83193380: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83193384: 386BFE10  addi r3, r11, -0x1f0
	ctx.r[3].s64 = ctx.r[11].s64 + -496;
	// 83193388: 4BFFB618  b 0x8318e9a0
	sub_8318E9A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83193390 size=72
    let mut pc: u32 = 0x83193390;
    'dispatch: loop {
        match pc {
            0x83193390 => {
    //   block [0x83193390..0x831933D8)
	// 83193390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83193394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83193398: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8319339C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831933A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831933A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831933A8: 409A0010  bne cr6, 0x831933b8
	if !ctx.cr[6].eq {
	pc = 0x831933B8; continue 'dispatch;
	}
	// 831933AC: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 831933B0: 386BFE10  addi r3, r11, -0x1f0
	ctx.r[3].s64 = ctx.r[11].s64 + -496;
	// 831933B4: 48000008  b 0x831933bc
	pc = 0x831933BC; continue 'dispatch;
	// 831933B8: 3863135C  addi r3, r3, 0x135c
	ctx.r[3].s64 = ctx.r[3].s64 + 4956;
	// 831933BC: 4BFF8EFD  bl 0x8318c2b8
	ctx.lr = 0x831933C0;
	sub_8318C2B8(ctx, base);
	// 831933C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831933C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831933C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831933CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831933D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831933D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831933D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831933D8 size=120
    let mut pc: u32 = 0x831933D8;
    'dispatch: loop {
        match pc {
            0x831933D8 => {
    //   block [0x831933D8..0x83193450)
	// 831933D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831933DC: 48014D91  bl 0x831a816c
	ctx.lr = 0x831933E0;
	sub_831A8130(ctx, base);
	// 831933E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831933E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831933E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831933EC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831933F0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831933F4: 409A001C  bne cr6, 0x83193410
	if !ctx.cr[6].eq {
	pc = 0x83193410; continue 'dispatch;
	}
	// 831933F8: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 831933FC: 396BFE10  addi r11, r11, -0x1f0
	ctx.r[11].s64 = ctx.r[11].s64 + -496;
	// 83193400: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83193404: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 83193408: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8319340C: 48014DB0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83193410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83193414: 4BFFC9CD  bl 0x8318fde0
	ctx.lr = 0x83193418;
	sub_8318FDE0(ctx, base);
	// 83193418: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8319341C: 419A001C  beq cr6, 0x83193438
	if ctx.cr[6].eq {
	pc = 0x83193438; continue 'dispatch;
	}
	// 83193420: 3C80FF03  lis r4, -0xfd
	ctx.r[4].s64 = -16580608;
	// 83193424: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83193428: 60840203  ori r4, r4, 0x203
	ctx.r[4].u64 = ctx.r[4].u64 | 515;
	// 8319342C: 4BFFFF65  bl 0x83193390
	ctx.lr = 0x83193430;
	sub_83193390(ctx, base);
	// 83193430: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83193434: 48014D88  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83193438: 397F135C  addi r11, r31, 0x135c
	ctx.r[11].s64 = ctx.r[31].s64 + 4956;
	// 8319343C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83193440: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83193444: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 83193448: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8319344C: 48014D70  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83193450 size=204
    let mut pc: u32 = 0x83193450;
    'dispatch: loop {
        match pc {
            0x83193450 => {
    //   block [0x83193450..0x8319351C)
	// 83193450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83193454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83193458: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8319345C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83193460: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83193464: 48000875  bl 0x83193cd8
	ctx.lr = 0x83193468;
	sub_83193CD8(ctx, base);
	// 83193468: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8319346C: 409A0028  bne cr6, 0x83193494
	if !ctx.cr[6].eq {
	pc = 0x83193494; continue 'dispatch;
	}
	// 83193470: 48000891  bl 0x83193d00
	ctx.lr = 0x83193474;
	sub_83193D00(ctx, base);
	// 83193474: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83193478: F86BFE28  std r3, -0x1d8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(-472 as u32), ctx.r[3].u64 ) };
	// 8319347C: 480007CD  bl 0x83193c48
	ctx.lr = 0x83193480;
	sub_83193C48(ctx, base);
	// 83193480: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83193484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83193488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319348C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83193490: 4E800020  blr
	return;
	// 83193494: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83193498: 419A0048  beq cr6, 0x831934e0
	if ctx.cr[6].eq {
	pc = 0x831934E0; continue 'dispatch;
	}
	// 8319349C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 831934A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831934A4: 419A003C  beq cr6, 0x831934e0
	if ctx.cr[6].eq {
	pc = 0x831934E0; continue 'dispatch;
	}
	// 831934A8: 817F106C  lwz r11, 0x106c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4204 as u32) ) } as u64;
	// 831934AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831934B0: 419A0030  beq cr6, 0x831934e0
	if ctx.cr[6].eq {
	pc = 0x831934E0; continue 'dispatch;
	}
	// 831934B4: 807F1080  lwz r3, 0x1080(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4224 as u32) ) } as u64;
	// 831934B8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831934BC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 831934C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831934C4: 4E800421  bctrl
	ctx.lr = 0x831934C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831934C8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831934CC: 3D408340  lis r10, -0x7cc0
	ctx.r[10].s64 = -2092957696;
	// 831934D0: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 831934D4: F96AFE28  std r11, -0x1d8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(-472 as u32), ctx.r[11].u64 ) };
	// 831934D8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831934DC: 48000028  b 0x83193504
	pc = 0x83193504; continue 'dispatch;
	// 831934E0: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 831934E4: 3D208340  lis r9, -0x7cc0
	ctx.r[9].s64 = -2092957696;
	// 831934E8: 396BA100  addi r11, r11, -0x5f00
	ctx.r[11].s64 = ctx.r[11].s64 + -24320;
	// 831934EC: 814B01BC  lwz r10, 0x1bc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(444 as u32) ) } as u64;
	// 831934F0: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 831934F4: F949FE28  std r10, -0x1d8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(-472 as u32), ctx.r[10].u64 ) };
	// 831934F8: 814B01B0  lwz r10, 0x1b0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(432 as u32) ) } as u64;
	// 831934FC: 816B01C0  lwz r11, 0x1c0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(448 as u32) ) } as u64;
	// 83193500: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 83193504: 7D6307B4  extsw r3, r11
	ctx.r[3].s64 = ctx.r[11].s32 as i64;
	// 83193508: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8319350C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83193510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83193514: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83193518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83193520 size=84
    let mut pc: u32 = 0x83193520;
    'dispatch: loop {
        match pc {
            0x83193520 => {
    //   block [0x83193520..0x83193574)
	// 83193520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83193524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83193528: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8319352C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83193530: 3FE08340  lis r31, -0x7cc0
	ctx.r[31].s64 = -2092957696;
	// 83193534: E97FFE28  ld r11, -0x1d8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(-472 as u32) ) };
	// 83193538: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 8319353C: 409A0020  bne cr6, 0x8319355c
	if !ctx.cr[6].eq {
	pc = 0x8319355C; continue 'dispatch;
	}
	// 83193540: 4BFFFF11  bl 0x83193450
	ctx.lr = 0x83193544;
	sub_83193450(ctx, base);
	// 83193544: E87FFE28  ld r3, -0x1d8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(-472 as u32) ) };
	// 83193548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8319354C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83193550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83193554: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83193558: 4E800020  blr
	return;
	// 8319355C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83193560: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83193564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83193568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319356C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83193570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193578 size=32
    let mut pc: u32 = 0x83193578;
    'dispatch: loop {
        match pc {
            0x83193578 => {
    //   block [0x83193578..0x83193598)
	// 83193578: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8319357C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83193580: 794A0040  clrldi r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 83193584: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 83193588: F9430008  std r10, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 8319358C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 83193590: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83193594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193598 size=72
    let mut pc: u32 = 0x83193598;
    'dispatch: loop {
        match pc {
            0x83193598 => {
    //   block [0x83193598..0x831935E0)
	// 83193598: E9430000  ld r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8319359C: E9630008  ld r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	// 831935A0: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 831935A4: 7F245800  cmpd cr6, r4, r11
	ctx.cr[6].compare_i64(ctx.r[4].s64, ctx.r[11].s64, &mut ctx.xer);
	// 831935A8: F9430000  std r10, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 831935AC: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 831935B0: 41980008  blt cr6, 0x831935b8
	if ctx.cr[6].lt {
	pc = 0x831935B8; continue 'dispatch;
	}
	// 831935B4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 831935B8: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 831935BC: F9430008  std r10, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 831935C0: 7F245800  cmpd cr6, r4, r11
	ctx.cr[6].compare_i64(ctx.r[4].s64, ctx.r[11].s64, &mut ctx.xer);
	// 831935C4: 40990008  ble cr6, 0x831935cc
	if !ctx.cr[6].gt {
	pc = 0x831935CC; continue 'dispatch;
	}
	// 831935C8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 831935CC: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831935D0: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 831935D4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831935D8: 91430018  stw r10, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 831935DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831935E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831935E0 size=32
    let mut pc: u32 = 0x831935E0;
    'dispatch: loop {
        match pc {
            0x831935E0 => {
    //   block [0x831935E0..0x83193600)
	// 831935E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831935E4: 3940FFFD  li r10, -3
	ctx.r[10].s64 = -3;
	// 831935E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 831935EC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831935F0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831935F4: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 831935F8: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 831935FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83193600 size=88
    let mut pc: u32 = 0x83193600;
    'dispatch: loop {
        match pc {
            0x83193600 => {
    //   block [0x83193600..0x83193658)
	// 83193600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83193604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83193608: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8319360C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83193610: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83193614: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83193618: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8319361C: 4BFF3CC5  bl 0x831872e0
	ctx.lr = 0x83193620;
	sub_831872E0(ctx, base);
	// 83193620: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83193624: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83193628: 419A0014  beq cr6, 0x8319363c
	if ctx.cr[6].eq {
	pc = 0x8319363C; continue 'dispatch;
	}
	// 8319362C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83193630: 60840151  ori r4, r4, 0x151
	ctx.r[4].u64 = ctx.r[4].u64 | 337;
	// 83193634: 4BFF3EC5  bl 0x831874f8
	ctx.lr = 0x83193638;
	sub_831874F8(ctx, base);
	// 83193638: 48000008  b 0x83193640
	pc = 0x83193640; continue 'dispatch;
	// 8319363C: 93DF1E28  stw r30, 0x1e28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7720 as u32), ctx.r[30].u32 ) };
	// 83193640: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83193644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83193648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319364C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83193650: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83193654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193658 size=12
    let mut pc: u32 = 0x83193658;
    'dispatch: loop {
        match pc {
            0x83193658 => {
    //   block [0x83193658..0x83193664)
	// 83193658: 81631E28  lwz r11, 0x1e28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7720 as u32) ) } as u64;
	// 8319365C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83193660: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193664(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193664 size=28
    let mut pc: u32 = 0x83193664;
    'dispatch: loop {
        match pc {
            0x83193664 => {
    //   block [0x83193664..0x83193680)
	// 83193664: 814B0DB8  lwz r10, 0xdb8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3512 as u32) ) } as u64;
	// 83193668: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8319366C: 40980008  bge cr6, 0x83193674
	if !ctx.cr[6].lt {
	pc = 0x83193674; continue 'dispatch;
	}
	// 83193670: 908B0DB8  stw r4, 0xdb8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3512 as u32), ctx.r[4].u32 ) };
	// 83193674: 814B0DBC  lwz r10, 0xdbc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3516 as u32) ) } as u64;
	// 83193678: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8319367C: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193680 size=8
    let mut pc: u32 = 0x83193680;
    'dispatch: loop {
        match pc {
            0x83193680 => {
    //   block [0x83193680..0x83193688)
	// 83193680: 90AB0DBC  stw r5, 0xdbc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3516 as u32), ctx.r[5].u32 ) };
	// 83193684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83193688 size=104
    let mut pc: u32 = 0x83193688;
    'dispatch: loop {
        match pc {
            0x83193688 => {
    //   block [0x83193688..0x831936F0)
	// 83193688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319368C: 48014AE1  bl 0x831a816c
	ctx.lr = 0x83193690;
	sub_831A8130(ctx, base);
	// 83193690: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83193694: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83193698: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8319369C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831936A0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 831936A4: 4BFF87C5  bl 0x8318be68
	ctx.lr = 0x831936A8;
	sub_8318BE68(ctx, base);
	// 831936A8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831936AC: 419A0034  beq cr6, 0x831936e0
	if ctx.cr[6].eq {
	pc = 0x831936E0; continue 'dispatch;
	}
	// 831936B0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 831936B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831936B8: 4BFE5D59  bl 0x83179410
	ctx.lr = 0x831936BC;
	sub_83179410(ctx, base);
	// 831936BC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831936C0: 409A0020  bne cr6, 0x831936e0
	if !ctx.cr[6].eq {
	pc = 0x831936E0; continue 'dispatch;
	}
	// 831936C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831936C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831936CC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831936D0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 831936D4: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 831936D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831936DC: 48014AE0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831936E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831936E4: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 831936E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831936EC: 48014AD0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831936F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831936F0 size=104
    let mut pc: u32 = 0x831936F0;
    'dispatch: loop {
        match pc {
            0x831936F0 => {
    //   block [0x831936F0..0x83193758)
	// 831936F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831936F4: 48014A79  bl 0x831a816c
	ctx.lr = 0x831936F8;
	sub_831A8130(ctx, base);
	// 831936F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831936FC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83193700: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83193704: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83193708: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8319370C: 4BFF875D  bl 0x8318be68
	ctx.lr = 0x83193710;
	sub_8318BE68(ctx, base);
	// 83193710: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83193714: 419A0034  beq cr6, 0x83193748
	if ctx.cr[6].eq {
	pc = 0x83193748; continue 'dispatch;
	}
	// 83193718: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8319371C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83193720: 4BFE5CF1  bl 0x83179410
	ctx.lr = 0x83193724;
	sub_83179410(ctx, base);
	// 83193724: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83193728: 409A0020  bne cr6, 0x83193748
	if !ctx.cr[6].eq {
	pc = 0x83193748; continue 'dispatch;
	}
	// 8319372C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83193730: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83193734: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83193738: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8319373C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83193740: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83193744: 48014A78  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83193748: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8319374C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83193750: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83193754: 48014A68  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83193758 size=48
    let mut pc: u32 = 0x83193758;
    'dispatch: loop {
        match pc {
            0x83193758 => {
    //   block [0x83193758..0x83193788)
	// 83193758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319375C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83193760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83193764: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83193768: 4BFF8701  bl 0x8318be68
	ctx.lr = 0x8319376C;
	sub_8318BE68(ctx, base);
	// 8319376C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 83193770: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83193774: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 83193778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8319377C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83193780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83193784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83193788 size=100
    let mut pc: u32 = 0x83193788;
    'dispatch: loop {
        match pc {
            0x83193788 => {
    //   block [0x83193788..0x831937EC)
	// 83193788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319378C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83193790: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83193794: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83193798: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319379C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831937A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831937A4: 4BFF3B3D  bl 0x831872e0
	ctx.lr = 0x831937A8;
	sub_831872E0(ctx, base);
	// 831937A8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831937AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831937B0: 419A0014  beq cr6, 0x831937c4
	if ctx.cr[6].eq {
	pc = 0x831937C4; continue 'dispatch;
	}
	// 831937B4: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 831937B8: 6084015C  ori r4, r4, 0x15c
	ctx.r[4].u64 = ctx.r[4].u64 | 348;
	// 831937BC: 4BFF3D3D  bl 0x831874f8
	ctx.lr = 0x831937C0;
	sub_831874F8(ctx, base);
	// 831937C0: 48000014  b 0x831937d4
	pc = 0x831937D4; continue 'dispatch;
	// 831937C4: 817F1E28  lwz r11, 0x1e28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7720 as u32) ) } as u64;
	// 831937C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831937CC: 419A0008  beq cr6, 0x831937d4
	if ctx.cr[6].eq {
	pc = 0x831937D4; continue 'dispatch;
	}
	// 831937D0: 93CB0DD4  stw r30, 0xdd4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3540 as u32), ctx.r[30].u32 ) };
	// 831937D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831937D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831937DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831937E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831937E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831937E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831937F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831937F0 size=40
    let mut pc: u32 = 0x831937F0;
    'dispatch: loop {
        match pc {
            0x831937F0 => {
    //   block [0x831937F0..0x83193818)
	// 831937F0: 39631738  addi r11, r3, 0x1738
	ctx.r[11].s64 = ctx.r[3].s64 + 5944;
	// 831937F4: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 831937F8: 1D4A0074  mulli r10, r10, 0x74
	ctx.r[10].s64 = ctx.r[10].s64 * 116;
	// 831937FC: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 83193800: 814A13E8  lwz r10, 0x13e8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5096 as u32) ) } as u64;
	// 83193804: 1D4A0044  mulli r10, r10, 0x44
	ctx.r[10].s64 = ctx.r[10].s64 * 68;
	// 83193808: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8319380C: 806B0020  lwz r3, 0x20(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83193810: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83193814: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193818 size=8
    let mut pc: u32 = 0x83193818;
    'dispatch: loop {
        match pc {
            0x83193818 => {
    //   block [0x83193818..0x83193820)
	// 83193818: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8319381C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83193820 size=168
    let mut pc: u32 = 0x83193820;
    'dispatch: loop {
        match pc {
            0x83193820 => {
    //   block [0x83193820..0x831938C8)
	// 83193820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83193824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83193828: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8319382C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83193830: 83E31E28  lwz r31, 0x1e28(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7720 as u32) ) } as u64;
	// 83193834: 817F0DD0  lwz r11, 0xdd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3536 as u32) ) } as u64;
	// 83193838: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8319383C: 41990074  bgt cr6, 0x831938b0
	if ctx.cr[6].gt {
	pc = 0x831938B0; continue 'dispatch;
	}
	// 83193840: 807F0DC4  lwz r3, 0xdc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3524 as u32) ) } as u64;
	// 83193844: 80BF0DC8  lwz r5, 0xdc8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3528 as u32) ) } as u64;
	// 83193848: 809F0DCC  lwz r4, 0xdcc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3532 as u32) ) } as u64;
	// 8319384C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83193850: 40990028  ble cr6, 0x83193878
	if !ctx.cr[6].gt {
	pc = 0x83193878; continue 'dispatch;
	}
	// 83193854: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83193858: 40990020  ble cr6, 0x83193878
	if !ctx.cr[6].gt {
	pc = 0x83193878; continue 'dispatch;
	}
	// 8319385C: 4BFF7F7D  bl 0x8318b7d8
	ctx.lr = 0x83193860;
	sub_8318B7D8(ctx, base);
	// 83193860: 907F0DA8  stw r3, 0xda8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3496 as u32), ctx.r[3].u32 ) };
	// 83193864: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83193868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319386C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83193870: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83193874: 4E800020  blr
	return;
	// 83193878: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8319387C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83193880: 41990030  bgt cr6, 0x831938b0
	if ctx.cr[6].gt {
	pc = 0x831938B0; continue 'dispatch;
	}
	// 83193884: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83193888: 41990008  bgt cr6, 0x83193890
	if ctx.cr[6].gt {
	pc = 0x83193890; continue 'dispatch;
	}
	// 8319388C: 807F0DAC  lwz r3, 0xdac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3500 as u32) ) } as u64;
	// 83193890: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83193894: 4199000C  bgt cr6, 0x831938a0
	if ctx.cr[6].gt {
	pc = 0x831938A0; continue 'dispatch;
	}
	// 83193898: 80BF0DB0  lwz r5, 0xdb0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3504 as u32) ) } as u64;
	// 8319389C: 809F0DB4  lwz r4, 0xdb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3508 as u32) ) } as u64;
	// 831938A0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831938A4: 4099000C  ble cr6, 0x831938b0
	if !ctx.cr[6].gt {
	pc = 0x831938B0; continue 'dispatch;
	}
	// 831938A8: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 831938AC: 4199FFB0  bgt cr6, 0x8319385c
	if ctx.cr[6].gt {
	pc = 0x8319385C; continue 'dispatch;
	}
	// 831938B0: 917F0DA8  stw r11, 0xda8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3496 as u32), ctx.r[11].u32 ) };
	// 831938B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831938B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831938BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831938C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831938C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831938C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831938C8 size=300
    let mut pc: u32 = 0x831938C8;
    'dispatch: loop {
        match pc {
            0x831938C8 => {
    //   block [0x831938C8..0x831939F4)
	// 831938C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831938CC: 48014899  bl 0x831a8164
	ctx.lr = 0x831938D0;
	sub_831A8130(ctx, base);
	// 831938D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831938D4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 831938D8: 83FC1E28  lwz r31, 0x1e28(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(7720 as u32) ) } as u64;
	// 831938DC: 3BDF0AD0  addi r30, r31, 0xad0
	ctx.r[30].s64 = ctx.r[31].s64 + 2768;
	// 831938E0: 3BBF0D0C  addi r29, r31, 0xd0c
	ctx.r[29].s64 = ctx.r[31].s64 + 3340;
	// 831938E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831938E8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831938EC: 409A0100  bne cr6, 0x831939ec
	if !ctx.cr[6].eq {
	pc = 0x831939EC; continue 'dispatch;
	}
	// 831938F0: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 831938F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831938F8: 4BFFFD91  bl 0x83193688
	ctx.lr = 0x831938FC;
	sub_83193688(ctx, base);
	// 831938FC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83193900: 409A00EC  bne cr6, 0x831939ec
	if !ctx.cr[6].eq {
	pc = 0x831939EC; continue 'dispatch;
	}
	// 83193904: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83193908: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8319390C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83193910: 4BFFFDE1  bl 0x831936f0
	ctx.lr = 0x83193914;
	sub_831936F0(ctx, base);
	// 83193914: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83193918: 409A00D4  bne cr6, 0x831939ec
	if !ctx.cr[6].eq {
	pc = 0x831939EC; continue 'dispatch;
	}
	// 8319391C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83193920: 4BFFFE39  bl 0x83193758
	ctx.lr = 0x83193924;
	sub_83193758(ctx, base);
	// 83193924: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 83193928: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8319392C: 419A0080  beq cr6, 0x831939ac
	if ctx.cr[6].eq {
	pc = 0x831939AC; continue 'dispatch;
	}
	// 83193930: 937F08A0  stw r27, 0x8a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2208 as u32), ctx.r[27].u32 ) };
	// 83193934: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83193938: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8319393C: 419A0064  beq cr6, 0x831939a0
	if ctx.cr[6].eq {
	pc = 0x831939A0; continue 'dispatch;
	}
	// 83193940: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83193944: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83193948: 40990030  ble cr6, 0x83193978
	if !ctx.cr[6].gt {
	pc = 0x83193978; continue 'dispatch;
	}
	// 8319394C: 807F0DC4  lwz r3, 0xdc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3524 as u32) ) } as u64;
	// 83193950: 80BF0040  lwz r5, 0x40(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83193954: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83193958: 4099004C  ble cr6, 0x831939a4
	if !ctx.cr[6].gt {
	pc = 0x831939A4; continue 'dispatch;
	}
	// 8319395C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83193960: 40990044  ble cr6, 0x831939a4
	if !ctx.cr[6].gt {
	pc = 0x831939A4; continue 'dispatch;
	}
	// 83193964: 388003E8  li r4, 0x3e8
	ctx.r[4].s64 = 1000;
	// 83193968: 4BFF7E71  bl 0x8318b7d8
	ctx.lr = 0x8319396C;
	sub_8318B7D8(ctx, base);
	// 8319396C: 815F08A8  lwz r10, 0x8a8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2216 as u32) ) } as u64;
	// 83193970: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83193974: 48000064  b 0x831939d8
	pc = 0x831939D8; continue 'dispatch;
	// 83193978: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8319397C: 4BFFB8D5  bl 0x8318f250
	ctx.lr = 0x83193980;
	sub_8318F250(ctx, base);
	// 83193980: 2F03006C  cmpwi cr6, r3, 0x6c
	ctx.cr[6].compare_i32(ctx.r[3].s32, 108, &mut ctx.xer);
	// 83193984: 4098001C  bge cr6, 0x831939a0
	if !ctx.cr[6].lt {
	pc = 0x831939A0; continue 'dispatch;
	}
	// 83193988: 817F08A4  lwz r11, 0x8a4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2212 as u32) ) } as u64;
	// 8319398C: 394007E2  li r10, 0x7e2
	ctx.r[10].s64 = 2018;
	// 83193990: 556B5828  slwi r11, r11, 0xb
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83193994: 7D6B53D6  divw r11, r11, r10
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 83193998: 815F08A8  lwz r10, 0x8a8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2216 as u32) ) } as u64;
	// 8319399C: 4800003C  b 0x831939d8
	pc = 0x831939D8; continue 'dispatch;
	// 831939A0: 817F08A4  lwz r11, 0x8a4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2212 as u32) ) } as u64;
	// 831939A4: 815F08A8  lwz r10, 0x8a8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2216 as u32) ) } as u64;
	// 831939A8: 48000030  b 0x831939d8
	pc = 0x831939D8; continue 'dispatch;
	// 831939AC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831939B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831939B4: 419A0010  beq cr6, 0x831939c4
	if ctx.cr[6].eq {
	pc = 0x831939C4; continue 'dispatch;
	}
	// 831939B8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831939BC: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 831939C0: 48000018  b 0x831939d8
	pc = 0x831939D8; continue 'dispatch;
	// 831939C4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831939C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831939CC: 419A0020  beq cr6, 0x831939ec
	if ctx.cr[6].eq {
	pc = 0x831939EC; continue 'dispatch;
	}
	// 831939D0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 831939D4: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 831939D8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831939DC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831939E0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 831939E4: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 831939E8: 4BFFFE39  bl 0x83193820
	ctx.lr = 0x831939EC;
	sub_83193820(ctx, base);
	// 831939EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831939F0: 480147C4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831939F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831939F8 size=112
    let mut pc: u32 = 0x831939F8;
    'dispatch: loop {
        match pc {
            0x831939F8 => {
    //   block [0x831939F8..0x83193A68)
	// 831939F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831939FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83193A00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83193A04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83193A08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83193A0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83193A10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83193A14: 4BFF38CD  bl 0x831872e0
	ctx.lr = 0x83193A18;
	sub_831872E0(ctx, base);
	// 83193A18: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83193A1C: 419A0018  beq cr6, 0x83193a34
	if ctx.cr[6].eq {
	pc = 0x83193A34; continue 'dispatch;
	}
	// 83193A20: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83193A24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83193A28: 60840159  ori r4, r4, 0x159
	ctx.r[4].u64 = ctx.r[4].u64 | 345;
	// 83193A2C: 4BFF3ACD  bl 0x831874f8
	ctx.lr = 0x83193A30;
	sub_831874F8(ctx, base);
	// 83193A30: 48000020  b 0x83193a50
	pc = 0x83193A50; continue 'dispatch;
	// 83193A34: 817F1E28  lwz r11, 0x1e28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7720 as u32) ) } as u64;
	// 83193A38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83193A3C: 419A0010  beq cr6, 0x83193a4c
	if ctx.cr[6].eq {
	pc = 0x83193A4C; continue 'dispatch;
	}
	// 83193A40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83193A44: 93CB0DC4  stw r30, 0xdc4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3524 as u32), ctx.r[30].u32 ) };
	// 83193A48: 4BFFFDD9  bl 0x83193820
	ctx.lr = 0x83193A4C;
	sub_83193820(ctx, base);
	// 83193A4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83193A50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83193A54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83193A58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83193A5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83193A60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83193A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83193A68 size=100
    let mut pc: u32 = 0x83193A68;
    'dispatch: loop {
        match pc {
            0x83193A68 => {
    //   block [0x83193A68..0x83193ACC)
	// 83193A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83193A6C: 48014701  bl 0x831a816c
	ctx.lr = 0x83193A70;
	sub_831A8130(ctx, base);
	// 83193A70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83193A74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83193A78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83193A7C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83193A80: 4BFF3861  bl 0x831872e0
	ctx.lr = 0x83193A84;
	sub_831872E0(ctx, base);
	// 83193A84: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83193A88: 419A001C  beq cr6, 0x83193aa4
	if ctx.cr[6].eq {
	pc = 0x83193AA4; continue 'dispatch;
	}
	// 83193A8C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83193A90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83193A94: 6084015A  ori r4, r4, 0x15a
	ctx.r[4].u64 = ctx.r[4].u64 | 346;
	// 83193A98: 4BFF3A61  bl 0x831874f8
	ctx.lr = 0x83193A9C;
	sub_831874F8(ctx, base);
	// 83193A9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83193AA0: 4801471C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83193AA4: 817F1E28  lwz r11, 0x1e28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7720 as u32) ) } as u64;
	// 83193AA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83193AAC: 419A0014  beq cr6, 0x83193ac0
	if ctx.cr[6].eq {
	pc = 0x83193AC0; continue 'dispatch;
	}
	// 83193AB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83193AB4: 93CB0DC8  stw r30, 0xdc8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3528 as u32), ctx.r[30].u32 ) };
	// 83193AB8: 93AB0DCC  stw r29, 0xdcc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3532 as u32), ctx.r[29].u32 ) };
	// 83193ABC: 4BFFFD65  bl 0x83193820
	ctx.lr = 0x83193AC0;
	sub_83193820(ctx, base);
	// 83193AC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83193AC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83193AC8: 480146F4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83193AD0 size=112
    let mut pc: u32 = 0x83193AD0;
    'dispatch: loop {
        match pc {
            0x83193AD0 => {
    //   block [0x83193AD0..0x83193B40)
	// 83193AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83193AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83193AD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83193ADC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83193AE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83193AE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83193AE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83193AEC: 4BFF37F5  bl 0x831872e0
	ctx.lr = 0x83193AF0;
	sub_831872E0(ctx, base);
	// 83193AF0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83193AF4: 419A0018  beq cr6, 0x83193b0c
	if ctx.cr[6].eq {
	pc = 0x83193B0C; continue 'dispatch;
	}
	// 83193AF8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83193AFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83193B00: 6084015B  ori r4, r4, 0x15b
	ctx.r[4].u64 = ctx.r[4].u64 | 347;
	// 83193B04: 4BFF39F5  bl 0x831874f8
	ctx.lr = 0x83193B08;
	sub_831874F8(ctx, base);
	// 83193B08: 48000020  b 0x83193b28
	pc = 0x83193B28; continue 'dispatch;
	// 83193B0C: 817F1E28  lwz r11, 0x1e28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7720 as u32) ) } as u64;
	// 83193B10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83193B14: 419A0010  beq cr6, 0x83193b24
	if ctx.cr[6].eq {
	pc = 0x83193B24; continue 'dispatch;
	}
	// 83193B18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83193B1C: 93CB0DD0  stw r30, 0xdd0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3536 as u32), ctx.r[30].u32 ) };
	// 83193B20: 4BFFFD01  bl 0x83193820
	ctx.lr = 0x83193B24;
	sub_83193820(ctx, base);
	// 83193B24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83193B28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83193B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83193B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83193B34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83193B38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83193B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83193B40 size=196
    let mut pc: u32 = 0x83193B40;
    'dispatch: loop {
        match pc {
            0x83193B40 => {
    //   block [0x83193B40..0x83193C04)
	// 83193B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83193B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83193B48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83193B4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83193B50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83193B54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83193B58: 83FE1E28  lwz r31, 0x1e28(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7720 as u32) ) } as u64;
	// 83193B5C: 4BFF7E95  bl 0x8318b9f0
	ctx.lr = 0x83193B60;
	sub_8318B9F0(ctx, base);
	// 83193B60: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83193B64: 409A0088  bne cr6, 0x83193bec
	if !ctx.cr[6].eq {
	pc = 0x83193BEC; continue 'dispatch;
	}
	// 83193B68: 817F0DAC  lwz r11, 0xdac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3500 as u32) ) } as u64;
	// 83193B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83193B70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83193B74: 41990040  bgt cr6, 0x83193bb4
	if ctx.cr[6].gt {
	pc = 0x83193BB4; continue 'dispatch;
	}
	// 83193B78: 817E1E30  lwz r11, 0x1e30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7728 as u32) ) } as u64;
	// 83193B7C: 2F0BFFFD  cmpwi cr6, r11, -3
	ctx.cr[6].compare_i32(ctx.r[11].s32, -3, &mut ctx.xer);
	// 83193B80: 409A000C  bne cr6, 0x83193b8c
	if !ctx.cr[6].eq {
	pc = 0x83193B8C; continue 'dispatch;
	}
	// 83193B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83193B88: 48000010  b 0x83193b98
	pc = 0x83193B98; continue 'dispatch;
	// 83193B8C: 813F0DD4  lwz r9, 0xdd4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3540 as u32) ) } as u64;
	// 83193B90: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83193B94: 41980020  blt cr6, 0x83193bb4
	if ctx.cr[6].lt {
	pc = 0x83193BB4; continue 'dispatch;
	}
	// 83193B98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83193B9C: 4BFFFC55  bl 0x831937f0
	ctx.lr = 0x83193BA0;
	sub_831937F0(ctx, base);
	// 83193BA0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83193BA4: 419A0010  beq cr6, 0x83193bb4
	if ctx.cr[6].eq {
	pc = 0x83193BB4; continue 'dispatch;
	}
	// 83193BA8: 7D634A14  add r11, r3, r9
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[9].u64;
	// 83193BAC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83193BB0: 917F0DAC  stw r11, 0xdac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3500 as u32), ctx.r[11].u32 ) };
	// 83193BB4: 817F0DB0  lwz r11, 0xdb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3504 as u32) ) } as u64;
	// 83193BB8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83193BBC: 41990020  bgt cr6, 0x83193bdc
	if ctx.cr[6].gt {
	pc = 0x83193BDC; continue 'dispatch;
	}
	// 83193BC0: 817E0E40  lwz r11, 0xe40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(3648 as u32) ) } as u64;
	// 83193BC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83193BC8: 40990014  ble cr6, 0x83193bdc
	if !ctx.cr[6].gt {
	pc = 0x83193BDC; continue 'dispatch;
	}
	// 83193BCC: 917F0DB0  stw r11, 0xdb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3504 as u32), ctx.r[11].u32 ) };
	// 83193BD0: 817E0E44  lwz r11, 0xe44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(3652 as u32) ) } as u64;
	// 83193BD4: 917F0DB4  stw r11, 0xdb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3508 as u32), ctx.r[11].u32 ) };
	// 83193BD8: 4800000C  b 0x83193be4
	pc = 0x83193BE4; continue 'dispatch;
	// 83193BDC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83193BE0: 419A000C  beq cr6, 0x83193bec
	if ctx.cr[6].eq {
	pc = 0x83193BEC; continue 'dispatch;
	}
	// 83193BE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83193BE8: 4BFFFC39  bl 0x83193820
	ctx.lr = 0x83193BEC;
	sub_83193820(ctx, base);
	// 83193BEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83193BF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83193BF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83193BF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83193BFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83193C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83193C08 size=64
    let mut pc: u32 = 0x83193C08;
    'dispatch: loop {
        match pc {
            0x83193C08 => {
    //   block [0x83193C08..0x83193C48)
	// 83193C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83193C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83193C10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83193C14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83193C18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83193C1C: 817F1E28  lwz r11, 0x1e28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7720 as u32) ) } as u64;
	// 83193C20: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83193C24: 419A0010  beq cr6, 0x83193c34
	if ctx.cr[6].eq {
	pc = 0x83193C34; continue 'dispatch;
	}
	// 83193C28: 4BFFFCA1  bl 0x831938c8
	ctx.lr = 0x83193C2C;
	sub_831938C8(ctx, base);
	// 83193C2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83193C30: 4BFFFF11  bl 0x83193b40
	ctx.lr = 0x83193C34;
	sub_83193B40(ctx, base);
	// 83193C34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83193C38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83193C3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83193C40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83193C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83193C48 size=92
    let mut pc: u32 = 0x83193C48;
    'dispatch: loop {
        match pc {
            0x83193C48 => {
    //   block [0x83193C48..0x83193CA4)
	// 83193C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83193C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83193C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83193C54: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83193C58: 816B9944  lwz r11, -0x66bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26300 as u32) ) } as u64;
	// 83193C5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83193C60: 40990030  ble cr6, 0x83193c90
	if !ctx.cr[6].gt {
	pc = 0x83193C90; continue 'dispatch;
	}
	// 83193C64: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83193C68: 816B9940  lwz r11, -0x66c0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26304 as u32) ) } as u64;
	// 83193C6C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 83193C70: 419A0020  beq cr6, 0x83193c90
	if ctx.cr[6].eq {
	pc = 0x83193C90; continue 'dispatch;
	}
	// 83193C74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83193C78: 4BA39441  bl 0x82bcd0b8
	ctx.lr = 0x83193C7C;
	sub_82BCD0B8(ctx, base);
	// 83193C7C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83193C80: 419A0010  beq cr6, 0x83193c90
	if ctx.cr[6].eq {
	pc = 0x83193C90; continue 'dispatch;
	}
	// 83193C84: E8610050  ld r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83193C88: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 83193C8C: 409A0008  bne cr6, 0x83193c94
	if !ctx.cr[6].eq {
	pc = 0x83193C94; continue 'dispatch;
	}
	// 83193C90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83193C94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83193C98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83193C9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83193CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193CA8 size=32
    let mut pc: u32 = 0x83193CA8;
    'dispatch: loop {
        match pc {
            0x83193CA8 => {
    //   block [0x83193CA8..0x83193CC8)
	// 83193CA8: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83193CAC: 814B9944  lwz r10, -0x66bc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26300 as u32) ) } as u64;
	// 83193CB0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 83193CB4: 914B9944  stw r10, -0x66bc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26300 as u32), ctx.r[10].u32 ) };
	// 83193CB8: 814B9944  lwz r10, -0x66bc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26300 as u32) ) } as u64;
	// 83193CBC: 814B9944  lwz r10, -0x66bc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26300 as u32) ) } as u64;
	// 83193CC0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83193CC4: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193CC8 size=12
    let mut pc: u32 = 0x83193CC8;
    'dispatch: loop {
        match pc {
            0x83193CC8 => {
    //   block [0x83193CC8..0x83193CD4)
	// 83193CC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83193CCC: 914B9944  stw r10, -0x66bc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26300 as u32), ctx.r[10].u32 ) };
	// 83193CD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193CD8 size=28
    let mut pc: u32 = 0x83193CD8;
    'dispatch: loop {
        match pc {
            0x83193CD8 => {
    //   block [0x83193CD8..0x83193CF4)
	// 83193CD8: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83193CDC: E96B9938  ld r11, -0x66c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-26312 as u32) ) };
	// 83193CE0: 2F2B0001  cmpdi cr6, r11, 1
	ctx.cr[6].compare_i64(ctx.r[11].s64, 1, &mut ctx.xer);
	// 83193CE4: 419A0010  beq cr6, 0x83193cf4
	if ctx.cr[6].eq {
		sub_83193CF4(ctx, base);
		return;
	}
	// 83193CE8: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 83193CEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83193CF0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193CF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193CF4 size=8
    let mut pc: u32 = 0x83193CF4;
    'dispatch: loop {
        match pc {
            0x83193CF4 => {
    //   block [0x83193CF4..0x83193CFC)
	// 83193CF4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83193CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193D00 size=12
    let mut pc: u32 = 0x83193D00;
    'dispatch: loop {
        match pc {
            0x83193D00 => {
    //   block [0x83193D00..0x83193D0C)
	// 83193D00: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83193D04: E86B9938  ld r3, -0x66c8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-26312 as u32) ) };
	// 83193D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193D10 size=12
    let mut pc: u32 = 0x83193D10;
    'dispatch: loop {
        match pc {
            0x83193D10 => {
    //   block [0x83193D10..0x83193D1C)
	// 83193D10: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83193D14: F86B9938  std r3, -0x66c8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(-26312 as u32), ctx.r[3].u64 ) };
	// 83193D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83193D20 size=188
    let mut pc: u32 = 0x83193D20;
    'dispatch: loop {
        match pc {
            0x83193D20 => {
    //   block [0x83193D20..0x83193DDC)
	// 83193D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83193D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83193D28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83193D2C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83193D30: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83193D34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83193D38: 386BBCC8  addi r3, r11, -0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + -17208;
	// 83193D3C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83193D40: 48070669  bl 0x832043a8
	ctx.lr = 0x83193D44;
	sub_832043A8(ctx, base);
	// 83193D44: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83193D48: 409A0008  bne cr6, 0x83193d50
	if !ctx.cr[6].eq {
	pc = 0x83193D50; continue 'dispatch;
	}
	// 83193D4C: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83193D50: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83193D54: 814B9944  lwz r10, -0x66bc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26300 as u32) ) } as u64;
	// 83193D58: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83193D5C: 914B9944  stw r10, -0x66bc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26300 as u32), ctx.r[10].u32 ) };
	// 83193D60: 816B9944  lwz r11, -0x66bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26300 as u32) ) } as u64;
	// 83193D64: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83193D68: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83193D6C: 40990010  ble cr6, 0x83193d7c
	if !ctx.cr[6].gt {
	pc = 0x83193D7C; continue 'dispatch;
	}
	// 83193D70: 814B9940  lwz r10, -0x66c0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26304 as u32) ) } as u64;
	// 83193D74: 7F0AF800  cmpw cr6, r10, r31
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[31].s32, &mut ctx.xer);
	// 83193D78: 419A0050  beq cr6, 0x83193dc8
	if ctx.cr[6].eq {
	pc = 0x83193DC8; continue 'dispatch;
	}
	// 83193D7C: 93EB9940  stw r31, -0x66c0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26304 as u32), ctx.r[31].u32 ) };
	// 83193D80: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 83193D84: 419A003C  beq cr6, 0x83193dc0
	if ctx.cr[6].eq {
	pc = 0x83193DC0; continue 'dispatch;
	}
	// 83193D88: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 83193D8C: 48030845  bl 0x831c45d0
	ctx.lr = 0x83193D90;
	sub_831C45D0(ctx, base);
	// 83193D90: 3D6002FA  lis r11, 0x2fa
	ctx.r[11].s64 = 49938432;
	// 83193D94: 616AF080  ori r10, r11, 0xf080
	ctx.r[10].u64 = ctx.r[11].u64 | 61568;
	// 83193D98: E9610058  ld r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83193D9C: 7F2B5000  cmpd cr6, r11, r10
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[10].s64, &mut ctx.xer);
	// 83193DA0: 409A000C  bne cr6, 0x83193dac
	if !ctx.cr[6].eq {
	pc = 0x83193DAC; continue 'dispatch;
	}
	// 83193DA4: 3D6002F9  lis r11, 0x2f9
	ctx.r[11].s64 = 49872896;
	// 83193DA8: 616B0838  ori r11, r11, 0x838
	ctx.r[11].u64 = ctx.r[11].u64 | 2104;
	// 83193DAC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83193DB0: 419A0010  beq cr6, 0x83193dc0
	if ctx.cr[6].eq {
	pc = 0x83193DC0; continue 'dispatch;
	}
	// 83193DB4: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 83193DB8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83193DBC: 409A0008  bne cr6, 0x83193dc4
	if !ctx.cr[6].eq {
	pc = 0x83193DC4; continue 'dispatch;
	}
	// 83193DC0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83193DC4: 4BFFFF4D  bl 0x83193d10
	ctx.lr = 0x83193DC8;
	sub_83193D10(ctx, base);
	// 83193DC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83193DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83193DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83193DD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83193DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83193DE0 size=76
    let mut pc: u32 = 0x83193DE0;
    'dispatch: loop {
        match pc {
            0x83193DE0 => {
    //   block [0x83193DE0..0x83193E2C)
	// 83193DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83193DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83193DE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83193DEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83193DF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83193DF4: 38A000F0  li r5, 0xf0
	ctx.r[5].s64 = 240;
	// 83193DF8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83193DFC: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83193E00: 480143E1  bl 0x831a81e0
	ctx.lr = 0x83193E04;
	sub_831A81E0(ctx, base);
	// 83193E04: 817F01AC  lwz r11, 0x1ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(428 as u32) ) } as u64;
	// 83193E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83193E0C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83193E10: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83193E14: 917F01AC  stw r11, 0x1ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(428 as u32), ctx.r[11].u32 ) };
	// 83193E18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83193E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83193E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83193E24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83193E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193E30 size=24
    let mut pc: u32 = 0x83193E30;
    'dispatch: loop {
        match pc {
            0x83193E30 => {
    //   block [0x83193E30..0x83193E48)
	// 83193E30: E9440000  ld r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 83193E34: 39630138  addi r11, r3, 0x138
	ctx.r[11].s64 = ctx.r[3].s64 + 312;
	// 83193E38: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 83193E3C: E9440008  ld r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 83193E40: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 83193E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193E48 size=24
    let mut pc: u32 = 0x83193E48;
    'dispatch: loop {
        match pc {
            0x83193E48 => {
    //   block [0x83193E48..0x83193E60)
	// 83193E48: E9440000  ld r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 83193E4C: 39630148  addi r11, r3, 0x148
	ctx.r[11].s64 = ctx.r[3].s64 + 328;
	// 83193E50: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 83193E54: E9440008  ld r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 83193E58: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 83193E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193E60 size=24
    let mut pc: u32 = 0x83193E60;
    'dispatch: loop {
        match pc {
            0x83193E60 => {
    //   block [0x83193E60..0x83193E78)
	// 83193E60: E9440000  ld r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 83193E64: 39630158  addi r11, r3, 0x158
	ctx.r[11].s64 = ctx.r[3].s64 + 344;
	// 83193E68: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 83193E6C: E9440008  ld r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 83193E70: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 83193E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193E78 size=24
    let mut pc: u32 = 0x83193E78;
    'dispatch: loop {
        match pc {
            0x83193E78 => {
    //   block [0x83193E78..0x83193E90)
	// 83193E78: E9440000  ld r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 83193E7C: 39630168  addi r11, r3, 0x168
	ctx.r[11].s64 = ctx.r[3].s64 + 360;
	// 83193E80: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 83193E84: E9440008  ld r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 83193E88: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 83193E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193E90 size=8
    let mut pc: u32 = 0x83193E90;
    'dispatch: loop {
        match pc {
            0x83193E90 => {
    //   block [0x83193E90..0x83193E98)
	// 83193E90: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83193E94: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193E98 size=8
    let mut pc: u32 = 0x83193E98;
    'dispatch: loop {
        match pc {
            0x83193E98 => {
    //   block [0x83193E98..0x83193EA0)
	// 83193E98: 90830010  stw r4, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 83193E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193EA0 size=12
    let mut pc: u32 = 0x83193EA0;
    'dispatch: loop {
        match pc {
            0x83193EA0 => {
    //   block [0x83193EA0..0x83193EAC)
	// 83193EA0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83193EA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83193EA8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193EAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193EAC size=36
    let mut pc: u32 = 0x83193EAC;
    'dispatch: loop {
        match pc {
            0x83193EAC => {
    //   block [0x83193EAC..0x83193ED0)
	// 83193EAC: E9640000  ld r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 83193EB0: E9430130  ld r10, 0x130(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(304 as u32) ) };
	// 83193EB4: E9240008  ld r9, 8(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 83193EB8: 7D6A59D2  mulld r11, r10, r11
	ctx.r[11].s64 = ctx.r[10].s64 * ctx.r[11].s64;
	// 83193EBC: E9430128  ld r10, 0x128(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(296 as u32) ) };
	// 83193EC0: 7D6B4BD2  divd r11, r11, r9
	ctx.r[11].s64 = ctx.r[11].s64 / ctx.r[9].s64;
	// 83193EC4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83193EC8: F9630128  std r11, 0x128(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(296 as u32), ctx.r[11].u64 ) };
	// 83193ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193ED0 size=12
    let mut pc: u32 = 0x83193ED0;
    'dispatch: loop {
        match pc {
            0x83193ED0 => {
    //   block [0x83193ED0..0x83193EDC)
	// 83193ED0: 90830198  stw r4, 0x198(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(408 as u32), ctx.r[4].u32 ) };
	// 83193ED4: 90A3019C  stw r5, 0x19c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(412 as u32), ctx.r[5].u32 ) };
	// 83193ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83193EE0 size=56
    let mut pc: u32 = 0x83193EE0;
    'dispatch: loop {
        match pc {
            0x83193EE0 => {
    //   block [0x83193EE0..0x83193F18)
	// 83193EE0: 80E30010  lwz r7, 0x10(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83193EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83193EE8: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83193EEC: 40990024  ble cr6, 0x83193f10
	if !ctx.cr[6].gt {
	pc = 0x83193F10; continue 'dispatch;
	}
	// 83193EF0: 39430018  addi r10, r3, 0x18
	ctx.r[10].s64 = ctx.r[3].s64 + 24;
	// 83193EF4: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 83193EF8: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83193EFC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83193F00: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83193F04: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 83193F08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83193F0C: 409AFFEC  bne cr6, 0x83193ef8
	if !ctx.cr[6].eq {
	pc = 0x83193EF8; continue 'dispatch;
	}
	// 83193F10: 7C693BD6  divw r3, r9, r7
	ctx.r[3].s32 = ctx.r[9].s32 / ctx.r[7].s32;
	// 83193F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83193F18 size=184
    let mut pc: u32 = 0x83193F18;
    'dispatch: loop {
        match pc {
            0x83193F18 => {
    //   block [0x83193F18..0x83193FD0)
	// 83193F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83193F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83193F20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83193F24: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83193F28: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83193F2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83193F30: 388BBCD0  addi r4, r11, -0x4330
	ctx.r[4].s64 = ctx.r[11].s64 + -17200;
	// 83193F34: 38A00112  li r5, 0x112
	ctx.r[5].s64 = 274;
	// 83193F38: 480145D9  bl 0x831a8510
	ctx.lr = 0x83193F3C;
	sub_831A8510(ctx, base);
	// 83193F3C: 3FE08340  lis r31, -0x7cc0
	ctx.r[31].s64 = -2092957696;
	// 83193F40: 807FFE34  lwz r3, -0x1cc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-460 as u32) ) } as u64;
	// 83193F44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83193F48: 419A0074  beq cr6, 0x83193fbc
	if ctx.cr[6].eq {
	pc = 0x83193FBC; continue 'dispatch;
	}
	// 83193F4C: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83193F50: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83193F54: 80ABFE30  lwz r5, -0x1d0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-464 as u32) ) } as u64;
	// 83193F58: 48014289  bl 0x831a81e0
	ctx.lr = 0x83193F5C;
	sub_831A81E0(ctx, base);
	// 83193F5C: 815FFE34  lwz r10, -0x1cc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-460 as u32) ) } as u64;
	// 83193F60: 3D208345  lis r9, -0x7cbb
	ctx.r[9].s64 = -2092630016;
	// 83193F64: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 83193F68: 9149992C  stw r10, -0x66d4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26324 as u32), ctx.r[10].u32 ) };
	// 83193F6C: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83193F70: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83193F74: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83193F78: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 83193F7C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83193F80: 409AFFEC  bne cr6, 0x83193f6c
	if !ctx.cr[6].eq {
	pc = 0x83193F6C; continue 'dispatch;
	}
	// 83193F84: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 83193F88: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83193F8C: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83193F90: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83193F94: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83193F98: 409AFFF4  bne cr6, 0x83193f8c
	if !ctx.cr[6].eq {
	pc = 0x83193F8C; continue 'dispatch;
	}
	// 83193F9C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83193FA0: 8149992C  lwz r10, -0x66d4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-26324 as u32) ) } as u64;
	// 83193FA4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83193FA8: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83193FAC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83193FB0: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 83193FB4: 9169992C  stw r11, -0x66d4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26324 as u32), ctx.r[11].u32 ) };
	// 83193FB8: 916A9928  stw r11, -0x66d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26328 as u32), ctx.r[11].u32 ) };
	// 83193FBC: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 83193FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83193FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83193FC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83193FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83193FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83193FD0 size=356
    let mut pc: u32 = 0x83193FD0;
    'dispatch: loop {
        match pc {
            0x83193FD0 => {
    //   block [0x83193FD0..0x83194134)
	// 83193FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83193FD4: 4801416D  bl 0x831a8140
	ctx.lr = 0x83193FD8;
	sub_831A8130(ctx, base);
	// 83193FD8: 9421FD90  stwu r1, -0x270(r1)
	ea = ctx.r[1].u32.wrapping_add(-624 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83193FDC: 3FE08340  lis r31, -0x7cc0
	ctx.r[31].s64 = -2092957696;
	// 83193FE0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83193FE4: 817FFE34  lwz r11, -0x1cc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-460 as u32) ) } as u64;
	// 83193FE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83193FEC: 419A0140  beq cr6, 0x8319412c
	if ctx.cr[6].eq {
	pc = 0x8319412C; continue 'dispatch;
	}
	// 83193FF0: E9250190  ld r9, 0x190(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(400 as u32) ) };
	// 83193FF4: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 83193FF8: 83A501B4  lwz r29, 0x1b4(r5)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(436 as u32) ) } as u64;
	// 83193FFC: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 83194000: 7D2807B4  extsw r8, r9
	ctx.r[8].s64 = ctx.r[9].s32 as i64;
	// 83194004: 83C501B8  lwz r30, 0x1b8(r5)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(440 as u32) ) } as u64;
	// 83194008: 388ABDE8  addi r4, r10, -0x4218
	ctx.r[4].s64 = ctx.r[10].s64 + -16920;
	// 8319400C: E9450128  ld r10, 0x128(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(296 as u32) ) };
	// 83194010: 3D208345  lis r9, -0x7cbb
	ctx.r[9].s64 = -2092630016;
	// 83194014: E8C50108  ld r6, 0x108(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(264 as u32) ) };
	// 83194018: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8319401C: E9650118  ld r11, 0x118(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(280 as u32) ) };
	// 83194020: 7CC607B4  extsw r6, r6
	ctx.r[6].s64 = ctx.r[6].s32 as i64;
	// 83194024: 93A100D4  stw r29, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[29].u32 ) };
	// 83194028: 7FAA4050  subf r29, r10, r8
	ctx.r[29].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 8319402C: 93C100DC  stw r30, 0xdc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(220 as u32), ctx.r[30].u32 ) };
	// 83194030: E8E50120  ld r7, 0x120(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(288 as u32) ) };
	// 83194034: 1FCB03E8  mulli r30, r11, 0x3e8
	ctx.r[30].s64 = ctx.r[11].s64 * 1000;
	// 83194038: 8129A33C  lwz r9, -0x5cc4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-23748 as u32) ) } as u64;
	// 8319403C: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 83194040: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 83194044: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83194048: 838501A8  lwz r28, 0x1a8(r5)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(424 as u32) ) } as u64;
	// 8319404C: 912100E4  stw r9, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[9].u32 ) };
	// 83194050: 7D4A3050  subf r10, r10, r6
	ctx.r[10].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 83194054: 836501A4  lwz r27, 0x1a4(r5)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(420 as u32) ) } as u64;
	// 83194058: 834501A0  lwz r26, 0x1a0(r5)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(416 as u32) ) } as u64;
	// 8319405C: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 83194060: 832501B0  lwz r25, 0x1b0(r5)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(432 as u32) ) } as u64;
	// 83194064: 7D680676  sradi r8, r11, 0x20
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 32) - 1)) != 0);
	ctx.r[8].s64 = ctx.r[11].s64 >> 32;
	// 83194068: 830501AC  lwz r24, 0x1ac(r5)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(428 as u32) ) } as u64;
	// 8319406C: 7CDE3BD2  divd r6, r30, r7
	ctx.r[6].s64 = ctx.r[30].s64 / ctx.r[7].s64;
	// 83194070: 82E50008  lwz r23, 8(r5)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 83194074: 7D6B3BD2  divd r11, r11, r7
	ctx.r[11].s64 = ctx.r[11].s64 / ctx.r[7].s64;
	// 83194078: 82C50004  lwz r22, 4(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8319407C: 7CC707B4  extsw r7, r6
	ctx.r[7].s64 = ctx.r[6].s32 as i64;
	// 83194080: 82A501C8  lwz r21, 0x1c8(r5)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(456 as u32) ) } as u64;
	// 83194084: 5508003E  slwi r8, r8, 0
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83194088: 828501C4  lwz r20, 0x1c4(r5)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(452 as u32) ) } as u64;
	// 8319408C: 7D6607B4  extsw r6, r11
	ctx.r[6].s64 = ctx.r[11].s32 as i64;
	// 83194090: 826501C0  lwz r19, 0x1c0(r5)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(448 as u32) ) } as u64;
	// 83194094: 824501BC  lwz r18, 0x1bc(r5)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(444 as u32) ) } as u64;
	// 83194098: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 8319409C: 552A007E  clrlwi r10, r9, 1
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x7FFFFFFFu64;
	// 831940A0: 938100CC  stw r28, 0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), ctx.r[28].u32 ) };
	// 831940A4: 936100C4  stw r27, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[27].u32 ) };
	// 831940A8: 934100BC  stw r26, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[26].u32 ) };
	// 831940AC: 932100B4  stw r25, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[25].u32 ) };
	// 831940B0: 930100AC  stw r24, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[24].u32 ) };
	// 831940B4: 92E100A4  stw r23, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[23].u32 ) };
	// 831940B8: 92C1009C  stw r22, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[22].u32 ) };
	// 831940BC: 92A10094  stw r21, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[21].u32 ) };
	// 831940C0: 9281008C  stw r20, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[20].u32 ) };
	// 831940C4: 92610084  stw r19, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[19].u32 ) };
	// 831940C8: 9241007C  stw r18, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[18].u32 ) };
	// 831940CC: 93A10074  stw r29, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 831940D0: 48014A09  bl 0x831a8ad8
	ctx.lr = 0x831940D4;
	sub_831A8AD8(ctx, base);
	// 831940D4: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 831940D8: 394100F0  addi r10, r1, 0xf0
	ctx.r[10].s64 = ctx.r[1].s64 + 240;
	// 831940DC: 812B992C  lwz r9, -0x66d4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26324 as u32) ) } as u64;
	// 831940E0: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831940E4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831940E8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 831940EC: 99090000  stb r8, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 831940F0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831940F4: 409AFFEC  bne cr6, 0x831940e0
	if !ctx.cr[6].eq {
	pc = 0x831940E0; continue 'dispatch;
	}
	// 831940F8: 3D408340  lis r10, -0x7cc0
	ctx.r[10].s64 = -2092957696;
	// 831940FC: 813FFE34  lwz r9, -0x1cc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-460 as u32) ) } as u64;
	// 83194100: 814AFE30  lwz r10, -0x1d0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-464 as u32) ) } as u64;
	// 83194104: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83194108: 814B992C  lwz r10, -0x66d4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26324 as u32) ) } as u64;
	// 8319410C: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 83194110: 3929FC00  addi r9, r9, -0x400
	ctx.r[9].s64 = ctx.r[9].s64 + -1024;
	// 83194114: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83194118: 914B992C  stw r10, -0x66d4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26324 as u32), ctx.r[10].u32 ) };
	// 8319411C: 41980010  blt cr6, 0x8319412c
	if ctx.cr[6].lt {
	pc = 0x8319412C; continue 'dispatch;
	}
	// 83194120: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 83194124: 814A9928  lwz r10, -0x66d8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26328 as u32) ) } as u64;
	// 83194128: 914B992C  stw r10, -0x66d4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26324 as u32), ctx.r[10].u32 ) };
	// 8319412C: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 83194130: 48014060  b 0x831a8190
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83194138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83194138 size=240
    let mut pc: u32 = 0x83194138;
    'dispatch: loop {
        match pc {
            0x83194138 => {
    //   block [0x83194138..0x83194228)
	// 83194138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319413C: 48014031  bl 0x831a816c
	ctx.lr = 0x83194140;
	sub_831A8130(ctx, base);
	// 83194140: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83194144: 38A001D0  li r5, 0x1d0
	ctx.r[5].s64 = 464;
	// 83194148: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8319414C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83194150: 48014091  bl 0x831a81e0
	ctx.lr = 0x83194154;
	sub_831A81E0(ctx, base);
	// 83194154: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 83194158: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8319415C: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 83194160: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83194164: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83194168: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8319416C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 83194170: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 83194174: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83194178: 4BFFFC69  bl 0x83193de0
	ctx.lr = 0x8319417C;
	sub_83193DE0(ctx, base);
	// 8319417C: 3D60000F  lis r11, 0xf
	ctx.r[11].s64 = 983040;
	// 83194180: FBDF0108  std r30, 0x108(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), ctx.r[30].u64 ) };
	// 83194184: 3940412B  li r10, 0x412b
	ctx.r[10].s64 = 16683;
	// 83194188: FBBF0110  std r29, 0x110(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[29].u64 ) };
	// 8319418C: 616B4240  ori r11, r11, 0x4240
	ctx.r[11].u64 = ctx.r[11].u64 | 16960;
	// 83194190: FBDF0118  std r30, 0x118(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), ctx.r[30].u64 ) };
	// 83194194: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 83194198: FBBF0120  std r29, 0x120(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[29].u64 ) };
	// 8319419C: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 831941A0: FBDF0128  std r30, 0x128(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), ctx.r[30].u64 ) };
	// 831941A4: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 831941A8: FBBF0130  std r29, 0x130(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[29].u64 ) };
	// 831941AC: F95F0138  std r10, 0x138(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(312 as u32), ctx.r[10].u64 ) };
	// 831941B0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 831941B4: F97F0140  std r11, 0x140(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[11].u64 ) };
	// 831941B8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831941BC: 61290D40  ori r9, r9, 0xd40
	ctx.r[9].u64 = ctx.r[9].u64 | 3392;
	// 831941C0: FBDF0158  std r30, 0x158(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[30].u64 ) };
	// 831941C4: F91F0150  std r8, 0x150(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[8].u64 ) };
	// 831941C8: F8FF0160  std r7, 0x160(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(352 as u32), ctx.r[7].u64 ) };
	// 831941CC: FBDF0168  std r30, 0x168(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), ctx.r[30].u64 ) };
	// 831941D0: F95F0170  std r10, 0x170(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), ctx.r[10].u64 ) };
	// 831941D4: F93F0148  std r9, 0x148(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[9].u64 ) };
	// 831941D8: F97F0178  std r11, 0x178(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(376 as u32), ctx.r[11].u64 ) };
	// 831941DC: F97F0180  std r11, 0x180(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(384 as u32), ctx.r[11].u64 ) };
	// 831941E0: FBDF0188  std r30, 0x188(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), ctx.r[30].u64 ) };
	// 831941E4: FBDF0190  std r30, 0x190(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[30].u64 ) };
	// 831941E8: 93BF0198  stw r29, 0x198(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(408 as u32), ctx.r[29].u32 ) };
	// 831941EC: 93BF019C  stw r29, 0x19c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(412 as u32), ctx.r[29].u32 ) };
	// 831941F0: 93DF01A0  stw r30, 0x1a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(416 as u32), ctx.r[30].u32 ) };
	// 831941F4: 93DF01A4  stw r30, 0x1a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(420 as u32), ctx.r[30].u32 ) };
	// 831941F8: 93DF01A8  stw r30, 0x1a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(424 as u32), ctx.r[30].u32 ) };
	// 831941FC: 93DF01AC  stw r30, 0x1ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(428 as u32), ctx.r[30].u32 ) };
	// 83194200: 93DF01B0  stw r30, 0x1b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(432 as u32), ctx.r[30].u32 ) };
	// 83194204: 93DF01B4  stw r30, 0x1b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(436 as u32), ctx.r[30].u32 ) };
	// 83194208: 93DF01B8  stw r30, 0x1b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(440 as u32), ctx.r[30].u32 ) };
	// 8319420C: 93DF01BC  stw r30, 0x1bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(444 as u32), ctx.r[30].u32 ) };
	// 83194210: 93DF01C0  stw r30, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[30].u32 ) };
	// 83194214: 93DF01C4  stw r30, 0x1c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(452 as u32), ctx.r[30].u32 ) };
	// 83194218: 93DF01C8  stw r30, 0x1c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(456 as u32), ctx.r[30].u32 ) };
	// 8319421C: 4BFFFCFD  bl 0x83193f18
	ctx.lr = 0x83194220;
	sub_83193F18(ctx, base);
	// 83194220: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83194224: 48013F98  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83194228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83194228 size=88
    let mut pc: u32 = 0x83194228;
    'dispatch: loop {
        match pc {
            0x83194228 => {
    //   block [0x83194228..0x83194280)
	// 83194228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319422C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83194230: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83194234: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 83194238: 81660014  lwz r11, 0x14(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(20 as u32) ) } as u64;
	// 8319423C: 81460010  lwz r10, 0x10(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) } as u64;
	// 83194240: 7D2B53D6  divw r9, r11, r10
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 83194244: 7D4951D6  mullw r10, r9, r10
	ctx.r[10].s64 = (ctx.r[9].s32 as i64) * (ctx.r[10].s32 as i64);
	// 83194248: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8319424C: 396B0006  addi r11, r11, 6
	ctx.r[11].s64 = ctx.r[11].s64 + 6;
	// 83194250: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83194254: 7C8B312E  stwx r4, r11, r6
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32), ctx.r[4].u32) };
	// 83194258: 81660014  lwz r11, 0x14(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(20 as u32) ) } as u64;
	// 8319425C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83194260: 91660014  stw r11, 0x14(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83194264: 4BFFFC7D  bl 0x83193ee0
	ctx.lr = 0x83194268;
	sub_83193EE0(ctx, base);
	// 83194268: 906601B4  stw r3, 0x1b4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(436 as u32), ctx.r[3].u32 ) };
	// 8319426C: 906601B8  stw r3, 0x1b8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(440 as u32), ctx.r[3].u32 ) };
	// 83194270: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83194274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83194278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319427C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83194280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83194280 size=96
    let mut pc: u32 = 0x83194280;
    'dispatch: loop {
        match pc {
            0x83194280 => {
    //   block [0x83194280..0x831942E0)
	// 83194280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83194284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83194288: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319428C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 83194290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83194294: 81660010  lwz r11, 0x10(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) } as u64;
	// 83194298: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8319429C: 40990028  ble cr6, 0x831942c4
	if !ctx.cr[6].gt {
	pc = 0x831942C4; continue 'dispatch;
	}
	// 831942A0: 39660018  addi r11, r6, 0x18
	ctx.r[11].s64 = ctx.r[6].s64 + 24;
	// 831942A4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831942A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831942AC: 7D244850  subf r9, r4, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[4].s64;
	// 831942B0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831942B4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831942B8: 81260010  lwz r9, 0x10(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) } as u64;
	// 831942BC: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 831942C0: 4198FFE4  blt cr6, 0x831942a4
	if ctx.cr[6].lt {
	pc = 0x831942A4; continue 'dispatch;
	}
	// 831942C4: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 831942C8: 4BFFFC19  bl 0x83193ee0
	ctx.lr = 0x831942CC;
	sub_83193EE0(ctx, base);
	// 831942CC: 906601B8  stw r3, 0x1b8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(440 as u32), ctx.r[3].u32 ) };
	// 831942D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831942D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831942D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831942DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831942E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831942E0 size=1028
    let mut pc: u32 = 0x831942E0;
    'dispatch: loop {
        match pc {
            0x831942E0 => {
    //   block [0x831942E0..0x831946E4)
	// 831942E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831942E4: 48013E81  bl 0x831a8164
	ctx.lr = 0x831942E8;
	sub_831A8130(ctx, base);
	// 831942E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831942EC: E9650008  ld r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 831942F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831942F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831942F8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 831942FC: 2F2B0001  cmpdi cr6, r11, 1
	ctx.cr[6].compare_i64(ctx.r[11].s64, 1, &mut ctx.xer);
	// 83194300: 419A03CC  beq cr6, 0x831946cc
	if ctx.cr[6].eq {
	pc = 0x831946CC; continue 'dispatch;
	}
	// 83194304: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83194308: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8319430C: 419A03C0  beq cr6, 0x831946cc
	if ctx.cr[6].eq {
	pc = 0x831946CC; continue 'dispatch;
	}
	// 83194310: E97F0178  ld r11, 0x178(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(376 as u32) ) };
	// 83194314: 2F2BFFFF  cmpdi cr6, r11, -1
	ctx.cr[6].compare_i64(ctx.r[11].s64, -1, &mut ctx.xer);
	// 83194318: 409A000C  bne cr6, 0x83194324
	if !ctx.cr[6].eq {
	pc = 0x83194324; continue 'dispatch;
	}
	// 8319431C: E9650000  ld r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 83194320: F97F0178  std r11, 0x178(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(376 as u32), ctx.r[11].u64 ) };
	// 83194324: E9250000  ld r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 83194328: E91F0178  ld r8, 0x178(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(376 as u32) ) };
	// 8319432C: EB850008  ld r28, 8(r5)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 83194330: 7FA84850  subf r29, r8, r9
	ctx.r[29].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 83194334: E97F0190  ld r11, 0x190(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) };
	// 83194338: E95E0000  ld r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 8319433C: 7F2B5000  cmpd cr6, r11, r10
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[10].s64, &mut ctx.xer);
	// 83194340: FB810058  std r28, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u64 ) };
	// 83194344: FBA10050  std r29, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u64 ) };
	// 83194348: 41990008  bgt cr6, 0x83194350
	if ctx.cr[6].gt {
	pc = 0x83194350; continue 'dispatch;
	}
	// 8319434C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83194350: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 83194354: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83194358: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 8319435C: 409A0028  bne cr6, 0x83194384
	if !ctx.cr[6].eq {
	pc = 0x83194384; continue 'dispatch;
	}
	// 83194360: E97F0180  ld r11, 0x180(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(384 as u32) ) };
	// 83194364: 2F2BFFFF  cmpdi cr6, r11, -1
	ctx.cr[6].compare_i64(ctx.r[11].s64, -1, &mut ctx.xer);
	// 83194368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8319436C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83194370: 409A0090  bne cr6, 0x83194400
	if !ctx.cr[6].eq {
	pc = 0x83194400; continue 'dispatch;
	}
	// 83194374: FBBF0180  std r29, 0x180(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(384 as u32), ctx.r[29].u64 ) };
	// 83194378: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 8319437C: F97F0188  std r11, 0x188(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), ctx.r[11].u64 ) };
	// 83194380: 48000080  b 0x83194400
	pc = 0x83194400; continue 'dispatch;
	// 83194384: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83194388: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8319438C: 409A0074  bne cr6, 0x83194400
	if !ctx.cr[6].eq {
	pc = 0x83194400; continue 'dispatch;
	}
	// 83194390: E95F0190  ld r10, 0x190(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) };
	// 83194394: 7F2B5000  cmpd cr6, r11, r10
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[10].s64, &mut ctx.xer);
	// 83194398: 40990054  ble cr6, 0x831943ec
	if !ctx.cr[6].gt {
	pc = 0x831943EC; continue 'dispatch;
	}
	// 8319439C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831943A0: E95F0180  ld r10, 0x180(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(384 as u32) ) };
	// 831943A4: 2F2AFFFF  cmpdi cr6, r10, -1
	ctx.cr[6].compare_i64(ctx.r[10].s64, -1, &mut ctx.xer);
	// 831943A8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 831943AC: E95E0008  ld r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 831943B0: 409A0010  bne cr6, 0x831943c0
	if !ctx.cr[6].eq {
	pc = 0x831943C0; continue 'dispatch;
	}
	// 831943B4: E97F0158  ld r11, 0x158(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(344 as u32) ) };
	// 831943B8: E93F0160  ld r9, 0x160(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) };
	// 831943BC: 4800000C  b 0x831943c8
	pc = 0x831943C8; continue 'dispatch;
	// 831943C0: E97F0168  ld r11, 0x168(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) };
	// 831943C4: E93F0170  ld r9, 0x170(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(368 as u32) ) };
	// 831943C8: 7D6B51D2  mulld r11, r11, r10
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[10].s64;
	// 831943CC: FBBF0180  std r29, 0x180(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(384 as u32), ctx.r[29].u64 ) };
	// 831943D0: E95E0000  ld r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 831943D4: 7D6B4BD2  divd r11, r11, r9
	ctx.r[11].s64 = ctx.r[11].s64 / ctx.r[9].s64;
	// 831943D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831943DC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831943E0: F97F0188  std r11, 0x188(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), ctx.r[11].u64 ) };
	// 831943E4: 4BFFF9FD  bl 0x83193de0
	ctx.lr = 0x831943E8;
	sub_83193DE0(ctx, base);
	// 831943E8: 48000018  b 0x83194400
	pc = 0x83194400; continue 'dispatch;
	// 831943EC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831943F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831943F4: 409A000C  bne cr6, 0x83194400
	if !ctx.cr[6].eq {
	pc = 0x83194400; continue 'dispatch;
	}
	// 831943F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831943FC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83194400: E97F0190  ld r11, 0x190(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) };
	// 83194404: E95E0000  ld r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 83194408: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8319440C: 7F2B5000  cmpd cr6, r11, r10
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[10].s64, &mut ctx.xer);
	// 83194410: 41990008  bgt cr6, 0x83194418
	if ctx.cr[6].gt {
	pc = 0x83194418; continue 'dispatch;
	}
	// 83194414: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 83194418: E97F0180  ld r11, 0x180(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(384 as u32) ) };
	// 8319441C: F93F0190  std r9, 0x190(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[9].u64 ) };
	// 83194420: 2F2BFFFF  cmpdi cr6, r11, -1
	ctx.cr[6].compare_i64(ctx.r[11].s64, -1, &mut ctx.xer);
	// 83194424: 409A000C  bne cr6, 0x83194430
	if !ctx.cr[6].eq {
	pc = 0x83194430; continue 'dispatch;
	}
	// 83194428: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8319442C: 4800001C  b 0x83194448
	pc = 0x83194448; continue 'dispatch;
	// 83194430: E91E0008  ld r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 83194434: 7D6BE850  subf r11, r11, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 83194438: E95F0188  ld r10, 0x188(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(392 as u32) ) };
	// 8319443C: 7D6B41D2  mulld r11, r11, r8
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[8].s64;
	// 83194440: 7D6BE3D2  divd r11, r11, r28
	ctx.r[11].s64 = ctx.r[11].s64 / ctx.r[28].s64;
	// 83194444: 7CAB5214  add r5, r11, r10
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83194448: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8319444C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83194450: 409A0048  bne cr6, 0x83194498
	if !ctx.cr[6].eq {
	pc = 0x83194498; continue 'dispatch;
	}
	// 83194454: 7F254800  cmpd cr6, r5, r9
	ctx.cr[6].compare_i64(ctx.r[5].s64, ctx.r[9].s64, &mut ctx.xer);
	// 83194458: 40990144  ble cr6, 0x8319459c
	if !ctx.cr[6].gt {
	pc = 0x8319459C; continue 'dispatch;
	}
	// 8319445C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83194460: FBBF0180  std r29, 0x180(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(384 as u32), ctx.r[29].u64 ) };
	// 83194464: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83194468: 419A0018  beq cr6, 0x83194480
	if ctx.cr[6].eq {
	pc = 0x83194480; continue 'dispatch;
	}
	// 8319446C: 817F01A0  lwz r11, 0x1a0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(416 as u32) ) } as u64;
	// 83194470: F93F0188  std r9, 0x188(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), ctx.r[9].u64 ) };
	// 83194474: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83194478: 917F01A0  stw r11, 0x1a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(416 as u32), ctx.r[11].u32 ) };
	// 8319447C: 48000120  b 0x8319459c
	pc = 0x8319459C; continue 'dispatch;
	// 83194480: E97F0128  ld r11, 0x128(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(296 as u32) ) };
	// 83194484: F97F0188  std r11, 0x188(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), ctx.r[11].u64 ) };
	// 83194488: 817F01A0  lwz r11, 0x1a0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(416 as u32) ) } as u64;
	// 8319448C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83194490: 917F01A0  stw r11, 0x1a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(416 as u32), ctx.r[11].u32 ) };
	// 83194494: 48000108  b 0x8319459c
	pc = 0x8319459C; continue 'dispatch;
	// 83194498: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8319449C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831944A0: 409A00FC  bne cr6, 0x8319459c
	if !ctx.cr[6].eq {
	pc = 0x8319459C; continue 'dispatch;
	}
	// 831944A4: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 831944A8: 7D455850  subf r10, r5, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 831944AC: 2F2A0000  cmpdi cr6, r10, 0
	ctx.cr[6].compare_i64(ctx.r[10].s64, 0, &mut ctx.xer);
	// 831944B0: 40980008  bge cr6, 0x831944b8
	if !ctx.cr[6].lt {
	pc = 0x831944B8; continue 'dispatch;
	}
	// 831944B4: 7D4B2850  subf r10, r11, r5
	ctx.r[10].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	// 831944B8: E93F0148  ld r9, 0x148(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) };
	// 831944BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831944C0: E91E0008  ld r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 831944C4: E8FF0150  ld r7, 0x150(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) };
	// 831944C8: 7D2941D2  mulld r9, r9, r8
	ctx.r[9].s64 = ctx.r[9].s64 * ctx.r[8].s64;
	// 831944CC: 7D293BD2  divd r9, r9, r7
	ctx.r[9].s64 = ctx.r[9].s64 / ctx.r[7].s64;
	// 831944D0: 7F2A4800  cmpd cr6, r10, r9
	ctx.cr[6].compare_i64(ctx.r[10].s64, ctx.r[9].s64, &mut ctx.xer);
	// 831944D4: 40990024  ble cr6, 0x831944f8
	if !ctx.cr[6].gt {
	pc = 0x831944F8; continue 'dispatch;
	}
	// 831944D8: FBBF0180  std r29, 0x180(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(384 as u32), ctx.r[29].u64 ) };
	// 831944DC: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 831944E0: F97F0188  std r11, 0x188(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), ctx.r[11].u64 ) };
	// 831944E4: 4BFFF8FD  bl 0x83193de0
	ctx.lr = 0x831944E8;
	sub_83193DE0(ctx, base);
	// 831944E8: 817F01B0  lwz r11, 0x1b0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(432 as u32) ) } as u64;
	// 831944EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831944F0: 917F01B0  stw r11, 0x1b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(432 as u32), ctx.r[11].u32 ) };
	// 831944F4: 480000A8  b 0x8319459c
	pc = 0x8319459C; continue 'dispatch;
	// 831944F8: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 831944FC: 7CAA07B4  extsw r10, r5
	ctx.r[10].s64 = ctx.r[5].s32 as i64;
	// 83194500: 7C8A5850  subf r4, r10, r11
	ctx.r[4].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83194504: 4BFFFD25  bl 0x83194228
	ctx.lr = 0x83194508;
	sub_83194228(ctx, base);
	// 83194508: E93E0008  ld r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 8319450C: E95F0138  ld r10, 0x138(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(312 as u32) ) };
	// 83194510: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 83194514: E91F0140  ld r8, 0x140(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(320 as u32) ) };
	// 83194518: 7D4A49D2  mulld r10, r10, r9
	ctx.r[10].s64 = ctx.r[10].s64 * ctx.r[9].s64;
	// 8319451C: 7D4A43D2  divd r10, r10, r8
	ctx.r[10].s64 = ctx.r[10].s64 / ctx.r[8].s64;
	// 83194520: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 83194524: 7D2B00D0  neg r9, r11
	ctx.r[9].s64 = -ctx.r[11].s64;
	// 83194528: 41980008  blt cr6, 0x83194530
	if ctx.cr[6].lt {
	pc = 0x83194530; continue 'dispatch;
	}
	// 8319452C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 83194530: 7F295000  cmpd cr6, r9, r10
	ctx.cr[6].compare_i64(ctx.r[9].s64, ctx.r[10].s64, &mut ctx.xer);
	// 83194534: 40990068  ble cr6, 0x8319459c
	if !ctx.cr[6].gt {
	pc = 0x8319459C; continue 'dispatch;
	}
	// 83194538: 7F2B5000  cmpd cr6, r11, r10
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[10].s64, &mut ctx.xer);
	// 8319453C: 796B0FA4  sldi r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64.wrapping_shl(1);
	ctx.r[11].u32 = ctx.r[11].u64 as u32;
	// 83194540: 7D6B53D2  divd r11, r11, r10
	ctx.r[11].s64 = ctx.r[11].s64 / ctx.r[10].s64;
	// 83194544: 4099001C  ble cr6, 0x83194560
	if !ctx.cr[6].gt {
	pc = 0x83194560; continue 'dispatch;
	}
	// 83194548: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8319454C: 811F01A4  lwz r8, 0x1a4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(420 as u32) ) } as u64;
	// 83194550: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 83194554: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 83194558: 913F01A4  stw r9, 0x1a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(420 as u32), ctx.r[9].u32 ) };
	// 8319455C: 48000018  b 0x83194574
	pc = 0x83194574; continue 'dispatch;
	// 83194560: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83194564: 813F01A8  lwz r9, 0x1a8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(424 as u32) ) } as u64;
	// 83194568: 7D6807B4  extsw r8, r11
	ctx.r[8].s64 = ctx.r[11].s32 as i64;
	// 8319456C: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 83194570: 913F01A8  stw r9, 0x1a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(424 as u32), ctx.r[9].u32 ) };
	// 83194574: 7D6B51D2  mulld r11, r11, r10
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[10].s64;
	// 83194578: FBBF0180  std r29, 0x180(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(384 as u32), ctx.r[29].u64 ) };
	// 8319457C: 796A0FE0  rldicl r10, r11, 1, 0x3f
	ctx.r[10].u64 = ctx.r[11].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 83194580: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83194584: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83194588: 7D6B0E74  sradi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 1) - 1)) != 0);
	ctx.r[11].s64 = ctx.r[11].s64 >> 1;
	// 8319458C: 7D4B2A14  add r10, r11, r5
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 83194590: 7D6407B4  extsw r4, r11
	ctx.r[4].s64 = ctx.r[11].s32 as i64;
	// 83194594: F95F0188  std r10, 0x188(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), ctx.r[10].u64 ) };
	// 83194598: 4BFFFCE9  bl 0x83194280
	ctx.lr = 0x8319459C;
	sub_83194280(ctx, base);
	// 8319459C: E97F0180  ld r11, 0x180(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(384 as u32) ) };
	// 831945A0: 2F2BFFFF  cmpdi cr6, r11, -1
	ctx.cr[6].compare_i64(ctx.r[11].s64, -1, &mut ctx.xer);
	// 831945A4: 409A000C  bne cr6, 0x831945b0
	if !ctx.cr[6].eq {
	pc = 0x831945B0; continue 'dispatch;
	}
	// 831945A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831945AC: 4800001C  b 0x831945c8
	pc = 0x831945C8; continue 'dispatch;
	// 831945B0: E93E0008  ld r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 831945B4: 7D6BE850  subf r11, r11, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 831945B8: E95F0188  ld r10, 0x188(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(392 as u32) ) };
	// 831945BC: 7D6B49D2  mulld r11, r11, r9
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[9].s64;
	// 831945C0: 7D6BE3D2  divd r11, r11, r28
	ctx.r[11].s64 = ctx.r[11].s64 / ctx.r[28].s64;
	// 831945C4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831945C8: F97B0000  std r11, 0(r27)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 831945CC: E95E0008  ld r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 831945D0: F95B0008  std r10, 8(r27)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 831945D4: E95F0128  ld r10, 0x128(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(296 as u32) ) };
	// 831945D8: 7F2B5000  cmpd cr6, r11, r10
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[10].s64, &mut ctx.xer);
	// 831945DC: 40980014  bge cr6, 0x831945f0
	if !ctx.cr[6].lt {
	pc = 0x831945F0; continue 'dispatch;
	}
	// 831945E0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 831945E4: F97B0000  std r11, 0(r27)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 831945E8: E97F0130  ld r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) };
	// 831945EC: F97B0008  std r11, 8(r27)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 831945F0: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831945F4: 395F0108  addi r10, r31, 0x108
	ctx.r[10].s64 = ctx.r[31].s64 + 264;
	// 831945F8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 831945FC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83194600: E91E0000  ld r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 83194604: 393F0118  addi r9, r31, 0x118
	ctx.r[9].s64 = ctx.r[31].s64 + 280;
	// 83194608: E8EB0000  ld r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8319460C: F90A0000  std r8, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 83194610: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 83194614: E91E0008  ld r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 83194618: F8E90000  std r7, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u64 ) };
	// 8319461C: F9690008  std r11, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 83194620: F90A0008  std r8, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[8].u64 ) };
	// 83194624: E97B0000  ld r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	// 83194628: F97F0128  std r11, 0x128(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), ctx.r[11].u64 ) };
	// 8319462C: E97B0008  ld r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) };
	// 83194630: F97F0130  std r11, 0x130(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[11].u64 ) };
	// 83194634: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 83194638: E95B0000  ld r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	// 8319463C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 83194640: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 83194644: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83194648: 409A0044  bne cr6, 0x8319468c
	if !ctx.cr[6].eq {
	pc = 0x8319468C; continue 'dispatch;
	}
	// 8319464C: 813F01BC  lwz r9, 0x1bc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(444 as u32) ) } as u64;
	// 83194650: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83194654: 41990008  bgt cr6, 0x8319465c
	if ctx.cr[6].gt {
	pc = 0x8319465C; continue 'dispatch;
	}
	// 83194658: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8319465C: 815F01C0  lwz r10, 0x1c0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(448 as u32) ) } as u64;
	// 83194660: 913F01BC  stw r9, 0x1bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(444 as u32), ctx.r[9].u32 ) };
	// 83194664: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83194668: 40980008  bge cr6, 0x83194670
	if !ctx.cr[6].lt {
	pc = 0x83194670; continue 'dispatch;
	}
	// 8319466C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83194670: 917F01C0  stw r11, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[11].u32 ) };
	// 83194674: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83194678: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8319467C: 93EB9930  stw r31, -0x66d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26320 as u32), ctx.r[31].u32 ) };
	// 83194680: 4BFFF951  bl 0x83193fd0
	ctx.lr = 0x83194684;
	sub_83193FD0(ctx, base);
	// 83194684: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83194688: 48013B2C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8319468C: 813F01C4  lwz r9, 0x1c4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(452 as u32) ) } as u64;
	// 83194690: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83194694: 41990008  bgt cr6, 0x8319469c
	if ctx.cr[6].gt {
	pc = 0x8319469C; continue 'dispatch;
	}
	// 83194698: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8319469C: 815F01C8  lwz r10, 0x1c8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(456 as u32) ) } as u64;
	// 831946A0: 913F01C4  stw r9, 0x1c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(452 as u32), ctx.r[9].u32 ) };
	// 831946A4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831946A8: 40980008  bge cr6, 0x831946b0
	if !ctx.cr[6].lt {
	pc = 0x831946B0; continue 'dispatch;
	}
	// 831946AC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 831946B0: 917F01C8  stw r11, 0x1c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(456 as u32), ctx.r[11].u32 ) };
	// 831946B4: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 831946B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831946BC: 93EB9930  stw r31, -0x66d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26320 as u32), ctx.r[31].u32 ) };
	// 831946C0: 4BFFF911  bl 0x83193fd0
	ctx.lr = 0x831946C4;
	sub_83193FD0(ctx, base);
	// 831946C4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831946C8: 48013AEC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 831946CC: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 831946D0: F97B0000  std r11, 0(r27)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 831946D4: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 831946D8: F97B0008  std r11, 8(r27)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 831946DC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831946E0: 48013AD4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831946E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831946E8 size=136
    let mut pc: u32 = 0x831946E8;
    'dispatch: loop {
        match pc {
            0x831946E8 => {
    //   block [0x831946E8..0x83194770)
	// 831946E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831946EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831946F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831946F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831946F8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831946FC: 81630198  lwz r11, 0x198(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(408 as u32) ) } as u64;
	// 83194700: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83194704: 8143019C  lwz r10, 0x19c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(412 as u32) ) } as u64;
	// 83194708: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8319470C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83194710: 409A000C  bne cr6, 0x8319471c
	if !ctx.cr[6].eq {
	pc = 0x8319471C; continue 'dispatch;
	}
	// 83194714: 4BFFFBCD  bl 0x831942e0
	ctx.lr = 0x83194718;
	sub_831942E0(ctx, base);
	// 83194718: 48000040  b 0x83194758
	pc = 0x83194758; continue 'dispatch;
	// 8319471C: E93F0008  ld r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 83194720: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 83194724: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 83194728: E91F0000  ld r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 8319472C: 7D6B49D2  mulld r11, r11, r9
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[9].s64;
	// 83194730: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 83194734: 7D6B53D2  divd r11, r11, r10
	ctx.r[11].s64 = ctx.r[11].s64 / ctx.r[10].s64;
	// 83194738: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 8319473C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83194740: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 83194744: 4BFFFB9D  bl 0x831942e0
	ctx.lr = 0x83194748;
	sub_831942E0(ctx, base);
	// 83194748: E9610060  ld r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8319474C: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 83194750: E97F0008  ld r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 83194754: F97E0008  std r11, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 83194758: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8319475C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83194760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83194764: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83194768: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8319476C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83194770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83194770 size=24
    let mut pc: u32 = 0x83194770;
    'dispatch: loop {
        match pc {
            0x83194770 => {
    //   block [0x83194770..0x83194788)
	// 83194770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83194774: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83194778: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8319477C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83194780: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83194784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83194788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83194788 size=28
    let mut pc: u32 = 0x83194788;
    'dispatch: loop {
        match pc {
            0x83194788 => {
    //   block [0x83194788..0x831947A4)
	// 83194788: 54CB103A  slwi r11, r6, 2
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8319478C: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 83194790: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83194794: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83194798: 7C8A2214  add r4, r10, r4
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 8319479C: 7CAA5850  subf r5, r10, r11
	ctx.r[5].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831947A0: 48013D70  b 0x831a8510
	sub_831A8510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831947A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831947A8 size=1516
    let mut pc: u32 = 0x831947A8;
    'dispatch: loop {
        match pc {
            0x831947A8 => {
    //   block [0x831947A8..0x83194D94)
	// 831947A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831947AC: 480139BD  bl 0x831a8168
	ctx.lr = 0x831947B0;
	sub_831A8130(ctx, base);
	// 831947B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831947B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831947B8: 38A00800  li r5, 0x800
	ctx.r[5].s64 = 2048;
	// 831947BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831947C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831947C4: 48013A1D  bl 0x831a81e0
	ctx.lr = 0x831947C8;
	sub_831A81E0(ctx, base);
	// 831947C8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831947CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831947D0: 3BEBE798  addi r31, r11, -0x1868
	ctx.r[31].s64 = ctx.r[11].s64 + -6248;
	// 831947D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831947D8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831947DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831947E0: 4BFFFFA9  bl 0x83194788
	ctx.lr = 0x831947E4;
	sub_83194788(ctx, base);
	// 831947E4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 831947E8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831947EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831947F0: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 831947F4: 4BFFFF95  bl 0x83194788
	ctx.lr = 0x831947F8;
	sub_83194788(ctx, base);
	// 831947F8: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 831947FC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194800: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194804: 387D0009  addi r3, r29, 9
	ctx.r[3].s64 = ctx.r[29].s64 + 9;
	// 83194808: 4BFFFF81  bl 0x83194788
	ctx.lr = 0x8319480C;
	sub_83194788(ctx, base);
	// 8319480C: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 83194810: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194814: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194818: 387D000C  addi r3, r29, 0xc
	ctx.r[3].s64 = ctx.r[29].s64 + 12;
	// 8319481C: 4BFFFF6D  bl 0x83194788
	ctx.lr = 0x83194820;
	sub_83194788(ctx, base);
	// 83194820: 3B9D0010  addi r28, r29, 0x10
	ctx.r[28].s64 = ctx.r[29].s64 + 16;
	// 83194824: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83194828: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8319482C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194830: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83194834: 4BFFFF55  bl 0x83194788
	ctx.lr = 0x83194838;
	sub_83194788(ctx, base);
	// 83194838: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 8319483C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194840: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194844: 387C0002  addi r3, r28, 2
	ctx.r[3].s64 = ctx.r[28].s64 + 2;
	// 83194848: 4BFFFF41  bl 0x83194788
	ctx.lr = 0x8319484C;
	sub_83194788(ctx, base);
	// 8319484C: 3B9D0020  addi r28, r29, 0x20
	ctx.r[28].s64 = ctx.r[29].s64 + 32;
	// 83194850: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 83194854: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194858: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8319485C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83194860: 4BFFFF29  bl 0x83194788
	ctx.lr = 0x83194864;
	sub_83194788(ctx, base);
	// 83194864: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 83194868: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8319486C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194870: 387C0018  addi r3, r28, 0x18
	ctx.r[3].s64 = ctx.r[28].s64 + 24;
	// 83194874: 4BFFFF15  bl 0x83194788
	ctx.lr = 0x83194878;
	sub_83194788(ctx, base);
	// 83194878: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8319487C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194880: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194884: 387C0019  addi r3, r28, 0x19
	ctx.r[3].s64 = ctx.r[28].s64 + 25;
	// 83194888: 4BFFFF01  bl 0x83194788
	ctx.lr = 0x8319488C;
	sub_83194788(ctx, base);
	// 8319488C: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 83194890: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194894: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194898: 387C001A  addi r3, r28, 0x1a
	ctx.r[3].s64 = ctx.r[28].s64 + 26;
	// 8319489C: 4BFFFEED  bl 0x83194788
	ctx.lr = 0x831948A0;
	sub_83194788(ctx, base);
	// 831948A0: 3B9D0040  addi r28, r29, 0x40
	ctx.r[28].s64 = ctx.r[29].s64 + 64;
	// 831948A4: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 831948A8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831948AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831948B0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831948B4: 4BFFFED5  bl 0x83194788
	ctx.lr = 0x831948B8;
	sub_83194788(ctx, base);
	// 831948B8: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 831948BC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831948C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831948C4: 387C000C  addi r3, r28, 0xc
	ctx.r[3].s64 = ctx.r[28].s64 + 12;
	// 831948C8: 4BFFFEC1  bl 0x83194788
	ctx.lr = 0x831948CC;
	sub_83194788(ctx, base);
	// 831948CC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 831948D0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831948D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831948D8: 387C0018  addi r3, r28, 0x18
	ctx.r[3].s64 = ctx.r[28].s64 + 24;
	// 831948DC: 4BFFFEAD  bl 0x83194788
	ctx.lr = 0x831948E0;
	sub_83194788(ctx, base);
	// 831948E0: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 831948E4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831948E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831948EC: 387D0060  addi r3, r29, 0x60
	ctx.r[3].s64 = ctx.r[29].s64 + 96;
	// 831948F0: 4BFFFE99  bl 0x83194788
	ctx.lr = 0x831948F4;
	sub_83194788(ctx, base);
	// 831948F4: 3B9D0080  addi r28, r29, 0x80
	ctx.r[28].s64 = ctx.r[29].s64 + 128;
	// 831948F8: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 831948FC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194900: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194904: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83194908: 4BFFFE81  bl 0x83194788
	ctx.lr = 0x8319490C;
	sub_83194788(ctx, base);
	// 8319490C: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 83194910: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194914: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194918: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 8319491C: 4BFFFE6D  bl 0x83194788
	ctx.lr = 0x83194920;
	sub_83194788(ctx, base);
	// 83194920: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83194924: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194928: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8319492C: 387C0005  addi r3, r28, 5
	ctx.r[3].s64 = ctx.r[28].s64 + 5;
	// 83194930: 4BFFFE59  bl 0x83194788
	ctx.lr = 0x83194934;
	sub_83194788(ctx, base);
	// 83194934: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 83194938: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8319493C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194940: 387C0008  addi r3, r28, 8
	ctx.r[3].s64 = ctx.r[28].s64 + 8;
	// 83194944: 4BFFFE45  bl 0x83194788
	ctx.lr = 0x83194948;
	sub_83194788(ctx, base);
	// 83194948: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 8319494C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194950: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194954: 387C000A  addi r3, r28, 0xa
	ctx.r[3].s64 = ctx.r[28].s64 + 10;
	// 83194958: 4BFFFE31  bl 0x83194788
	ctx.lr = 0x8319495C;
	sub_83194788(ctx, base);
	// 8319495C: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 83194960: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194964: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194968: 387C000C  addi r3, r28, 0xc
	ctx.r[3].s64 = ctx.r[28].s64 + 12;
	// 8319496C: 4BFFFE1D  bl 0x83194788
	ctx.lr = 0x83194970;
	sub_83194788(ctx, base);
	// 83194970: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 83194974: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194978: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8319497C: 387C0010  addi r3, r28, 0x10
	ctx.r[3].s64 = ctx.r[28].s64 + 16;
	// 83194980: 4BFFFE09  bl 0x83194788
	ctx.lr = 0x83194984;
	sub_83194788(ctx, base);
	// 83194984: 3B9D00B0  addi r28, r29, 0xb0
	ctx.r[28].s64 = ctx.r[29].s64 + 176;
	// 83194988: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 8319498C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194990: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194994: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83194998: 4BFFFDF1  bl 0x83194788
	ctx.lr = 0x8319499C;
	sub_83194788(ctx, base);
	// 8319499C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 831949A0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831949A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831949A8: 387C0001  addi r3, r28, 1
	ctx.r[3].s64 = ctx.r[28].s64 + 1;
	// 831949AC: 4BFFFDDD  bl 0x83194788
	ctx.lr = 0x831949B0;
	sub_83194788(ctx, base);
	// 831949B0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 831949B4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831949B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831949BC: 387C0002  addi r3, r28, 2
	ctx.r[3].s64 = ctx.r[28].s64 + 2;
	// 831949C0: 4BFFFDC9  bl 0x83194788
	ctx.lr = 0x831949C4;
	sub_83194788(ctx, base);
	// 831949C4: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 831949C8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831949CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831949D0: 387C0003  addi r3, r28, 3
	ctx.r[3].s64 = ctx.r[28].s64 + 3;
	// 831949D4: 4BFFFDB5  bl 0x83194788
	ctx.lr = 0x831949D8;
	sub_83194788(ctx, base);
	// 831949D8: 38C00019  li r6, 0x19
	ctx.r[6].s64 = 25;
	// 831949DC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831949E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831949E4: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 831949E8: 4BFFFDA1  bl 0x83194788
	ctx.lr = 0x831949EC;
	sub_83194788(ctx, base);
	// 831949EC: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 831949F0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831949F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831949F8: 387C0008  addi r3, r28, 8
	ctx.r[3].s64 = ctx.r[28].s64 + 8;
	// 831949FC: 4BFFFD8D  bl 0x83194788
	ctx.lr = 0x83194A00;
	sub_83194788(ctx, base);
	// 83194A00: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 83194A04: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194A08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194A0C: 387C000C  addi r3, r28, 0xc
	ctx.r[3].s64 = ctx.r[28].s64 + 12;
	// 83194A10: 4BFFFD79  bl 0x83194788
	ctx.lr = 0x83194A14;
	sub_83194788(ctx, base);
	// 83194A14: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 83194A18: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194A1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194A20: 387C0010  addi r3, r28, 0x10
	ctx.r[3].s64 = ctx.r[28].s64 + 16;
	// 83194A24: 4BFFFD65  bl 0x83194788
	ctx.lr = 0x83194A28;
	sub_83194788(ctx, base);
	// 83194A28: 38C0001D  li r6, 0x1d
	ctx.r[6].s64 = 29;
	// 83194A2C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194A30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194A34: 387C0014  addi r3, r28, 0x14
	ctx.r[3].s64 = ctx.r[28].s64 + 20;
	// 83194A38: 4BFFFD51  bl 0x83194788
	ctx.lr = 0x83194A3C;
	sub_83194788(ctx, base);
	// 83194A3C: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 83194A40: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194A44: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194A48: 387C0018  addi r3, r28, 0x18
	ctx.r[3].s64 = ctx.r[28].s64 + 24;
	// 83194A4C: 4BFFFD3D  bl 0x83194788
	ctx.lr = 0x83194A50;
	sub_83194788(ctx, base);
	// 83194A50: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 83194A54: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194A58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194A5C: 387C001C  addi r3, r28, 0x1c
	ctx.r[3].s64 = ctx.r[28].s64 + 28;
	// 83194A60: 4BFFFD29  bl 0x83194788
	ctx.lr = 0x83194A64;
	sub_83194788(ctx, base);
	// 83194A64: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 83194A68: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194A6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194A70: 387D00E0  addi r3, r29, 0xe0
	ctx.r[3].s64 = ctx.r[29].s64 + 224;
	// 83194A74: 4BFFFD15  bl 0x83194788
	ctx.lr = 0x83194A78;
	sub_83194788(ctx, base);
	// 83194A78: 38C00021  li r6, 0x21
	ctx.r[6].s64 = 33;
	// 83194A7C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83194A80: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194A84: 387D0120  addi r3, r29, 0x120
	ctx.r[3].s64 = ctx.r[29].s64 + 288;
	// 83194A88: 4BFFFD01  bl 0x83194788
	ctx.lr = 0x83194A8C;
	sub_83194788(ctx, base);
	// 83194A8C: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 83194A90: 3BBD01A2  addi r29, r29, 0x1a2
	ctx.r[29].s64 = ctx.r[29].s64 + 418;
	// 83194A94: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83194A98: 3B80001A  li r28, 0x1a
	ctx.r[28].s64 = 26;
	// 83194A9C: 3BCBFFE0  addi r30, r11, -0x20
	ctx.r[30].s64 = ctx.r[11].s64 + -32;
	// 83194AA0: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 83194AA4: 897E0018  lbz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83194AA8: 2B0B00C0  cmplwi cr6, r11, 0xc0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 192 as u32, &mut ctx.xer);
	// 83194AAC: 4098021C  bge cr6, 0x83194cc8
	if !ctx.cr[6].lt {
	pc = 0x83194CC8; continue 'dispatch;
	}
	// 83194AB0: 2B0B00DF  cmplwi cr6, r11, 0xdf
	ctx.cr[6].compare_u32(ctx.r[11].u32, 223 as u32, &mut ctx.xer);
	// 83194AB4: 40990214  ble cr6, 0x83194cc8
	if !ctx.cr[6].gt {
	pc = 0x83194CC8; continue 'dispatch;
	}
	// 83194AB8: 2B0B00E0  cmplwi cr6, r11, 0xe0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 224 as u32, &mut ctx.xer);
	// 83194ABC: 409800D0  bge cr6, 0x83194b8c
	if !ctx.cr[6].lt {
	pc = 0x83194B8C; continue 'dispatch;
	}
	// 83194AC0: 2B0B00EF  cmplwi cr6, r11, 0xef
	ctx.cr[6].compare_u32(ctx.r[11].u32, 239 as u32, &mut ctx.xer);
	// 83194AC4: 409900C8  ble cr6, 0x83194b8c
	if !ctx.cr[6].gt {
	pc = 0x83194B8C; continue 'dispatch;
	}
	// 83194AC8: 2B0B00BD  cmplwi cr6, r11, 0xbd
	ctx.cr[6].compare_u32(ctx.r[11].u32, 189 as u32, &mut ctx.xer);
	// 83194ACC: 40980060  bge cr6, 0x83194b2c
	if !ctx.cr[6].lt {
	pc = 0x83194B2C; continue 'dispatch;
	}
	// 83194AD0: 2B0B00BF  cmplwi cr6, r11, 0xbf
	ctx.cr[6].compare_u32(ctx.r[11].u32, 191 as u32, &mut ctx.xer);
	// 83194AD4: 40990058  ble cr6, 0x83194b2c
	if !ctx.cr[6].gt {
	pc = 0x83194B2C; continue 'dispatch;
	}
	// 83194AD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83194ADC: 409A02A0  bne cr6, 0x83194d7c
	if !ctx.cr[6].eq {
	pc = 0x83194D7C; continue 'dispatch;
	}
	// 83194AE0: 38BF00B4  addi r5, r31, 0xb4
	ctx.r[5].s64 = ctx.r[31].s64 + 180;
	// 83194AE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83194AE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194AEC: 387DFFDE  addi r3, r29, -0x22
	ctx.r[3].s64 = ctx.r[29].s64 + -34;
	// 83194AF0: 4BFFFC99  bl 0x83194788
	ctx.lr = 0x83194AF4;
	sub_83194788(ctx, base);
	// 83194AF4: 38BF00B4  addi r5, r31, 0xb4
	ctx.r[5].s64 = ctx.r[31].s64 + 180;
	// 83194AF8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83194AFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194B00: 387DFFEA  addi r3, r29, -0x16
	ctx.r[3].s64 = ctx.r[29].s64 + -22;
	// 83194B04: 4BFFFC85  bl 0x83194788
	ctx.lr = 0x83194B08;
	sub_83194788(ctx, base);
	// 83194B08: 38BF00B4  addi r5, r31, 0xb4
	ctx.r[5].s64 = ctx.r[31].s64 + 180;
	// 83194B0C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83194B10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194B14: 387DFFF6  addi r3, r29, -0xa
	ctx.r[3].s64 = ctx.r[29].s64 + -10;
	// 83194B18: 4BFFFC71  bl 0x83194788
	ctx.lr = 0x83194B1C;
	sub_83194788(ctx, base);
	// 83194B1C: 38BF00B4  addi r5, r31, 0xb4
	ctx.r[5].s64 = ctx.r[31].s64 + 180;
	// 83194B20: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 83194B24: 387DFFF7  addi r3, r29, -9
	ctx.r[3].s64 = ctx.r[29].s64 + -9;
	// 83194B28: 4800024C  b 0x83194d74
	pc = 0x83194D74; continue 'dispatch;
	// 83194B2C: 38BF010C  addi r5, r31, 0x10c
	ctx.r[5].s64 = ctx.r[31].s64 + 268;
	// 83194B30: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83194B34: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194B38: 387DFFDE  addi r3, r29, -0x22
	ctx.r[3].s64 = ctx.r[29].s64 + -34;
	// 83194B3C: 4BFFFC4D  bl 0x83194788
	ctx.lr = 0x83194B40;
	sub_83194788(ctx, base);
	// 83194B40: 38BF010C  addi r5, r31, 0x10c
	ctx.r[5].s64 = ctx.r[31].s64 + 268;
	// 83194B44: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83194B48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194B4C: 387DFFEA  addi r3, r29, -0x16
	ctx.r[3].s64 = ctx.r[29].s64 + -22;
	// 83194B50: 4BFFFC39  bl 0x83194788
	ctx.lr = 0x83194B54;
	sub_83194788(ctx, base);
	// 83194B54: 38BF010C  addi r5, r31, 0x10c
	ctx.r[5].s64 = ctx.r[31].s64 + 268;
	// 83194B58: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83194B5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194B60: 387DFFF6  addi r3, r29, -0xa
	ctx.r[3].s64 = ctx.r[29].s64 + -10;
	// 83194B64: 4BFFFC25  bl 0x83194788
	ctx.lr = 0x83194B68;
	sub_83194788(ctx, base);
	// 83194B68: 38BF010C  addi r5, r31, 0x10c
	ctx.r[5].s64 = ctx.r[31].s64 + 268;
	// 83194B6C: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 83194B70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194B74: 387DFFF7  addi r3, r29, -9
	ctx.r[3].s64 = ctx.r[29].s64 + -9;
	// 83194B78: 4BFFFC11  bl 0x83194788
	ctx.lr = 0x83194B7C;
	sub_83194788(ctx, base);
	// 83194B7C: 38BF010C  addi r5, r31, 0x10c
	ctx.r[5].s64 = ctx.r[31].s64 + 268;
	// 83194B80: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83194B84: 387DFFFE  addi r3, r29, -2
	ctx.r[3].s64 = ctx.r[29].s64 + -2;
	// 83194B88: 480001EC  b 0x83194d74
	pc = 0x83194D74; continue 'dispatch;
	// 83194B8C: 38BF00C8  addi r5, r31, 0xc8
	ctx.r[5].s64 = ctx.r[31].s64 + 200;
	// 83194B90: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83194B94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194B98: 387DFFDE  addi r3, r29, -0x22
	ctx.r[3].s64 = ctx.r[29].s64 + -34;
	// 83194B9C: 4BFFFBED  bl 0x83194788
	ctx.lr = 0x83194BA0;
	sub_83194788(ctx, base);
	// 83194BA0: 38BF00C8  addi r5, r31, 0xc8
	ctx.r[5].s64 = ctx.r[31].s64 + 200;
	// 83194BA4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83194BA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194BAC: 387DFFEA  addi r3, r29, -0x16
	ctx.r[3].s64 = ctx.r[29].s64 + -22;
	// 83194BB0: 4BFFFBD9  bl 0x83194788
	ctx.lr = 0x83194BB4;
	sub_83194788(ctx, base);
	// 83194BB4: 38BF00C8  addi r5, r31, 0xc8
	ctx.r[5].s64 = ctx.r[31].s64 + 200;
	// 83194BB8: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83194BBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194BC0: 387DFFF6  addi r3, r29, -0xa
	ctx.r[3].s64 = ctx.r[29].s64 + -10;
	// 83194BC4: 4BFFFBC5  bl 0x83194788
	ctx.lr = 0x83194BC8;
	sub_83194788(ctx, base);
	// 83194BC8: 38BF00C8  addi r5, r31, 0xc8
	ctx.r[5].s64 = ctx.r[31].s64 + 200;
	// 83194BCC: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 83194BD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194BD4: 387DFFF7  addi r3, r29, -9
	ctx.r[3].s64 = ctx.r[29].s64 + -9;
	// 83194BD8: 4BFFFBB1  bl 0x83194788
	ctx.lr = 0x83194BDC;
	sub_83194788(ctx, base);
	// 83194BDC: 38BF00C8  addi r5, r31, 0xc8
	ctx.r[5].s64 = ctx.r[31].s64 + 200;
	// 83194BE0: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83194BE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194BE8: 387DFFF8  addi r3, r29, -8
	ctx.r[3].s64 = ctx.r[29].s64 + -8;
	// 83194BEC: 4BFFFB9D  bl 0x83194788
	ctx.lr = 0x83194BF0;
	sub_83194788(ctx, base);
	// 83194BF0: 38BF00C8  addi r5, r31, 0xc8
	ctx.r[5].s64 = ctx.r[31].s64 + 200;
	// 83194BF4: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 83194BF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194BFC: 387DFFFA  addi r3, r29, -6
	ctx.r[3].s64 = ctx.r[29].s64 + -6;
	// 83194C00: 4BFFFB89  bl 0x83194788
	ctx.lr = 0x83194C04;
	sub_83194788(ctx, base);
	// 83194C04: 38BF00C8  addi r5, r31, 0xc8
	ctx.r[5].s64 = ctx.r[31].s64 + 200;
	// 83194C08: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 83194C0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194C10: 387DFFFD  addi r3, r29, -3
	ctx.r[3].s64 = ctx.r[29].s64 + -3;
	// 83194C14: 4BFFFB75  bl 0x83194788
	ctx.lr = 0x83194C18;
	sub_83194788(ctx, base);
	// 83194C18: 38BF00C8  addi r5, r31, 0xc8
	ctx.r[5].s64 = ctx.r[31].s64 + 200;
	// 83194C1C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 83194C20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194C24: 387DFFFE  addi r3, r29, -2
	ctx.r[3].s64 = ctx.r[29].s64 + -2;
	// 83194C28: 4BFFFB61  bl 0x83194788
	ctx.lr = 0x83194C2C;
	sub_83194788(ctx, base);
	// 83194C2C: 38BF00C8  addi r5, r31, 0xc8
	ctx.r[5].s64 = ctx.r[31].s64 + 200;
	// 83194C30: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83194C34: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194C38: 387DFFFF  addi r3, r29, -1
	ctx.r[3].s64 = ctx.r[29].s64 + -1;
	// 83194C3C: 4BFFFB4D  bl 0x83194788
	ctx.lr = 0x83194C40;
	sub_83194788(ctx, base);
	// 83194C40: 38BF00C8  addi r5, r31, 0xc8
	ctx.r[5].s64 = ctx.r[31].s64 + 200;
	// 83194C44: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 83194C48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194C4C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83194C50: 4BFFFB39  bl 0x83194788
	ctx.lr = 0x83194C54;
	sub_83194788(ctx, base);
	// 83194C54: 38BF00C8  addi r5, r31, 0xc8
	ctx.r[5].s64 = ctx.r[31].s64 + 200;
	// 83194C58: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 83194C5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194C60: 387D0001  addi r3, r29, 1
	ctx.r[3].s64 = ctx.r[29].s64 + 1;
	// 83194C64: 4BFFFB25  bl 0x83194788
	ctx.lr = 0x83194C68;
	sub_83194788(ctx, base);
	// 83194C68: 38BF00C8  addi r5, r31, 0xc8
	ctx.r[5].s64 = ctx.r[31].s64 + 200;
	// 83194C6C: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 83194C70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194C74: 387D0002  addi r3, r29, 2
	ctx.r[3].s64 = ctx.r[29].s64 + 2;
	// 83194C78: 4BFFFB11  bl 0x83194788
	ctx.lr = 0x83194C7C;
	sub_83194788(ctx, base);
	// 83194C7C: 38BF00C8  addi r5, r31, 0xc8
	ctx.r[5].s64 = ctx.r[31].s64 + 200;
	// 83194C80: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83194C84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194C88: 387D0003  addi r3, r29, 3
	ctx.r[3].s64 = ctx.r[29].s64 + 3;
	// 83194C8C: 4BFFFAFD  bl 0x83194788
	ctx.lr = 0x83194C90;
	sub_83194788(ctx, base);
	// 83194C90: 38BF00C8  addi r5, r31, 0xc8
	ctx.r[5].s64 = ctx.r[31].s64 + 200;
	// 83194C94: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 83194C98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194C9C: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 83194CA0: 4BFFFAE9  bl 0x83194788
	ctx.lr = 0x83194CA4;
	sub_83194788(ctx, base);
	// 83194CA4: 38BF00C8  addi r5, r31, 0xc8
	ctx.r[5].s64 = ctx.r[31].s64 + 200;
	// 83194CA8: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 83194CAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194CB0: 387D0005  addi r3, r29, 5
	ctx.r[3].s64 = ctx.r[29].s64 + 5;
	// 83194CB4: 4BFFFAD5  bl 0x83194788
	ctx.lr = 0x83194CB8;
	sub_83194788(ctx, base);
	// 83194CB8: 38BF00C8  addi r5, r31, 0xc8
	ctx.r[5].s64 = ctx.r[31].s64 + 200;
	// 83194CBC: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 83194CC0: 387D0006  addi r3, r29, 6
	ctx.r[3].s64 = ctx.r[29].s64 + 6;
	// 83194CC4: 480000B0  b 0x83194d74
	pc = 0x83194D74; continue 'dispatch;
	// 83194CC8: 38BF008C  addi r5, r31, 0x8c
	ctx.r[5].s64 = ctx.r[31].s64 + 140;
	// 83194CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83194CD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194CD4: 387DFFDE  addi r3, r29, -0x22
	ctx.r[3].s64 = ctx.r[29].s64 + -34;
	// 83194CD8: 4BFFFAB1  bl 0x83194788
	ctx.lr = 0x83194CDC;
	sub_83194788(ctx, base);
	// 83194CDC: 38BF008C  addi r5, r31, 0x8c
	ctx.r[5].s64 = ctx.r[31].s64 + 140;
	// 83194CE0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83194CE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194CE8: 387DFFEA  addi r3, r29, -0x16
	ctx.r[3].s64 = ctx.r[29].s64 + -22;
	// 83194CEC: 4BFFFA9D  bl 0x83194788
	ctx.lr = 0x83194CF0;
	sub_83194788(ctx, base);
	// 83194CF0: 38BF008C  addi r5, r31, 0x8c
	ctx.r[5].s64 = ctx.r[31].s64 + 140;
	// 83194CF4: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83194CF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194CFC: 387DFFF6  addi r3, r29, -0xa
	ctx.r[3].s64 = ctx.r[29].s64 + -10;
	// 83194D00: 4BFFFA89  bl 0x83194788
	ctx.lr = 0x83194D04;
	sub_83194788(ctx, base);
	// 83194D04: 38BF008C  addi r5, r31, 0x8c
	ctx.r[5].s64 = ctx.r[31].s64 + 140;
	// 83194D08: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 83194D0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194D10: 387DFFF7  addi r3, r29, -9
	ctx.r[3].s64 = ctx.r[29].s64 + -9;
	// 83194D14: 4BFFFA75  bl 0x83194788
	ctx.lr = 0x83194D18;
	sub_83194788(ctx, base);
	// 83194D18: 38BF008C  addi r5, r31, 0x8c
	ctx.r[5].s64 = ctx.r[31].s64 + 140;
	// 83194D1C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83194D20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194D24: 387DFFF8  addi r3, r29, -8
	ctx.r[3].s64 = ctx.r[29].s64 + -8;
	// 83194D28: 4BFFFA61  bl 0x83194788
	ctx.lr = 0x83194D2C;
	sub_83194788(ctx, base);
	// 83194D2C: 38BF008C  addi r5, r31, 0x8c
	ctx.r[5].s64 = ctx.r[31].s64 + 140;
	// 83194D30: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 83194D34: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194D38: 387DFFF9  addi r3, r29, -7
	ctx.r[3].s64 = ctx.r[29].s64 + -7;
	// 83194D3C: 4BFFFA4D  bl 0x83194788
	ctx.lr = 0x83194D40;
	sub_83194788(ctx, base);
	// 83194D40: 38BF008C  addi r5, r31, 0x8c
	ctx.r[5].s64 = ctx.r[31].s64 + 140;
	// 83194D44: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 83194D48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194D4C: 387DFFFA  addi r3, r29, -6
	ctx.r[3].s64 = ctx.r[29].s64 + -6;
	// 83194D50: 4BFFFA39  bl 0x83194788
	ctx.lr = 0x83194D54;
	sub_83194788(ctx, base);
	// 83194D54: 38BF008C  addi r5, r31, 0x8c
	ctx.r[5].s64 = ctx.r[31].s64 + 140;
	// 83194D58: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 83194D5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194D60: 387DFFFE  addi r3, r29, -2
	ctx.r[3].s64 = ctx.r[29].s64 + -2;
	// 83194D64: 4BFFFA25  bl 0x83194788
	ctx.lr = 0x83194D68;
	sub_83194788(ctx, base);
	// 83194D68: 38BF008C  addi r5, r31, 0x8c
	ctx.r[5].s64 = ctx.r[31].s64 + 140;
	// 83194D6C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83194D70: 387DFFFF  addi r3, r29, -1
	ctx.r[3].s64 = ctx.r[29].s64 + -1;
	// 83194D74: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83194D78: 4BFFFA11  bl 0x83194788
	ctx.lr = 0x83194D7C;
	sub_83194788(ctx, base);
	// 83194D7C: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 83194D80: 3BBD0040  addi r29, r29, 0x40
	ctx.r[29].s64 = ctx.r[29].s64 + 64;
	// 83194D84: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83194D88: 409AFD18  bne cr6, 0x83194aa0
	if !ctx.cr[6].eq {
	pc = 0x83194AA0; continue 'dispatch;
	}
	// 83194D8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83194D90: 48013428  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83194D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83194D98 size=1244
    let mut pc: u32 = 0x83194D98;
    'dispatch: loop {
        match pc {
            0x83194D98 => {
    //   block [0x83194D98..0x83195274)
	// 83194D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83194D9C: 480133C5  bl 0x831a8160
	ctx.lr = 0x83194DA0;
	sub_831A8130(ctx, base);
	// 83194DA0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83194DA4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83194DA8: 38A00800  li r5, 0x800
	ctx.r[5].s64 = 2048;
	// 83194DAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83194DB0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83194DB4: 4801342D  bl 0x831a81e0
	ctx.lr = 0x83194DB8;
	sub_831A81E0(ctx, base);
	// 83194DB8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83194DBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83194DC0: 3BEBE9AC  addi r31, r11, -0x1654
	ctx.r[31].s64 = ctx.r[11].s64 + -5716;
	// 83194DC4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194DC8: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194DCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83194DD0: 4BFFF9B9  bl 0x83194788
	ctx.lr = 0x83194DD4;
	sub_83194788(ctx, base);
	// 83194DD4: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194DD8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83194DDC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194DE0: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 83194DE4: 4BFFF9A5  bl 0x83194788
	ctx.lr = 0x83194DE8;
	sub_83194788(ctx, base);
	// 83194DE8: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194DEC: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83194DF0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194DF4: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 83194DF8: 4BFFF991  bl 0x83194788
	ctx.lr = 0x83194DFC;
	sub_83194788(ctx, base);
	// 83194DFC: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194E00: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 83194E04: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194E08: 387E0012  addi r3, r30, 0x12
	ctx.r[3].s64 = ctx.r[30].s64 + 18;
	// 83194E0C: 4BFFF97D  bl 0x83194788
	ctx.lr = 0x83194E10;
	sub_83194788(ctx, base);
	// 83194E10: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194E14: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83194E18: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194E1C: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 83194E20: 4BFFF969  bl 0x83194788
	ctx.lr = 0x83194E24;
	sub_83194788(ctx, base);
	// 83194E24: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194E28: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 83194E2C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194E30: 387E0020  addi r3, r30, 0x20
	ctx.r[3].s64 = ctx.r[30].s64 + 32;
	// 83194E34: 4BFFF955  bl 0x83194788
	ctx.lr = 0x83194E38;
	sub_83194788(ctx, base);
	// 83194E38: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194E3C: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 83194E40: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194E44: 387E0038  addi r3, r30, 0x38
	ctx.r[3].s64 = ctx.r[30].s64 + 56;
	// 83194E48: 4BFFF941  bl 0x83194788
	ctx.lr = 0x83194E4C;
	sub_83194788(ctx, base);
	// 83194E4C: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194E50: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 83194E54: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194E58: 387E0039  addi r3, r30, 0x39
	ctx.r[3].s64 = ctx.r[30].s64 + 57;
	// 83194E5C: 4BFFF92D  bl 0x83194788
	ctx.lr = 0x83194E60;
	sub_83194788(ctx, base);
	// 83194E60: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194E64: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83194E68: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194E6C: 387E003A  addi r3, r30, 0x3a
	ctx.r[3].s64 = ctx.r[30].s64 + 58;
	// 83194E70: 4BFFF919  bl 0x83194788
	ctx.lr = 0x83194E74;
	sub_83194788(ctx, base);
	// 83194E74: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194E78: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 83194E7C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194E80: 387E003B  addi r3, r30, 0x3b
	ctx.r[3].s64 = ctx.r[30].s64 + 59;
	// 83194E84: 4BFFF905  bl 0x83194788
	ctx.lr = 0x83194E88;
	sub_83194788(ctx, base);
	// 83194E88: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194E8C: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 83194E90: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194E94: 387E003C  addi r3, r30, 0x3c
	ctx.r[3].s64 = ctx.r[30].s64 + 60;
	// 83194E98: 4BFFF8F1  bl 0x83194788
	ctx.lr = 0x83194E9C;
	sub_83194788(ctx, base);
	// 83194E9C: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194EA0: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 83194EA4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194EA8: 387E0040  addi r3, r30, 0x40
	ctx.r[3].s64 = ctx.r[30].s64 + 64;
	// 83194EAC: 4BFFF8DD  bl 0x83194788
	ctx.lr = 0x83194EB0;
	sub_83194788(ctx, base);
	// 83194EB0: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194EB4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83194EB8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194EBC: 387E0080  addi r3, r30, 0x80
	ctx.r[3].s64 = ctx.r[30].s64 + 128;
	// 83194EC0: 4BFFF8C9  bl 0x83194788
	ctx.lr = 0x83194EC4;
	sub_83194788(ctx, base);
	// 83194EC4: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194EC8: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 83194ECC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194ED0: 387E00C0  addi r3, r30, 0xc0
	ctx.r[3].s64 = ctx.r[30].s64 + 192;
	// 83194ED4: 4BFFF8B5  bl 0x83194788
	ctx.lr = 0x83194ED8;
	sub_83194788(ctx, base);
	// 83194ED8: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194EDC: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 83194EE0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194EE4: 387E00C1  addi r3, r30, 0xc1
	ctx.r[3].s64 = ctx.r[30].s64 + 193;
	// 83194EE8: 4BFFF8A1  bl 0x83194788
	ctx.lr = 0x83194EEC;
	sub_83194788(ctx, base);
	// 83194EEC: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194EF0: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 83194EF4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194EF8: 387E00C2  addi r3, r30, 0xc2
	ctx.r[3].s64 = ctx.r[30].s64 + 194;
	// 83194EFC: 4BFFF88D  bl 0x83194788
	ctx.lr = 0x83194F00;
	sub_83194788(ctx, base);
	// 83194F00: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194F04: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83194F08: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194F0C: 387E00C3  addi r3, r30, 0xc3
	ctx.r[3].s64 = ctx.r[30].s64 + 195;
	// 83194F10: 4BFFF879  bl 0x83194788
	ctx.lr = 0x83194F14;
	sub_83194788(ctx, base);
	// 83194F14: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194F18: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 83194F1C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194F20: 387E00C4  addi r3, r30, 0xc4
	ctx.r[3].s64 = ctx.r[30].s64 + 196;
	// 83194F24: 4BFFF865  bl 0x83194788
	ctx.lr = 0x83194F28;
	sub_83194788(ctx, base);
	// 83194F28: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194F2C: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 83194F30: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194F34: 387E00C8  addi r3, r30, 0xc8
	ctx.r[3].s64 = ctx.r[30].s64 + 200;
	// 83194F38: 4BFFF851  bl 0x83194788
	ctx.lr = 0x83194F3C;
	sub_83194788(ctx, base);
	// 83194F3C: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194F40: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 83194F44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194F48: 387E00CC  addi r3, r30, 0xcc
	ctx.r[3].s64 = ctx.r[30].s64 + 204;
	// 83194F4C: 4BFFF83D  bl 0x83194788
	ctx.lr = 0x83194F50;
	sub_83194788(ctx, base);
	// 83194F50: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194F54: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 83194F58: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194F5C: 387E00D0  addi r3, r30, 0xd0
	ctx.r[3].s64 = ctx.r[30].s64 + 208;
	// 83194F60: 4BFFF829  bl 0x83194788
	ctx.lr = 0x83194F64;
	sub_83194788(ctx, base);
	// 83194F64: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194F68: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 83194F6C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194F70: 387E00D4  addi r3, r30, 0xd4
	ctx.r[3].s64 = ctx.r[30].s64 + 212;
	// 83194F74: 4BFFF815  bl 0x83194788
	ctx.lr = 0x83194F78;
	sub_83194788(ctx, base);
	// 83194F78: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194F7C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 83194F80: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194F84: 387E00D8  addi r3, r30, 0xd8
	ctx.r[3].s64 = ctx.r[30].s64 + 216;
	// 83194F88: 4BFFF801  bl 0x83194788
	ctx.lr = 0x83194F8C;
	sub_83194788(ctx, base);
	// 83194F8C: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194F90: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 83194F94: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194F98: 387E00DC  addi r3, r30, 0xdc
	ctx.r[3].s64 = ctx.r[30].s64 + 220;
	// 83194F9C: 4BFFF7ED  bl 0x83194788
	ctx.lr = 0x83194FA0;
	sub_83194788(ctx, base);
	// 83194FA0: 38BFFF1C  addi r5, r31, -0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + -228;
	// 83194FA4: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 83194FA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83194FAC: 387E00E0  addi r3, r30, 0xe0
	ctx.r[3].s64 = ctx.r[30].s64 + 224;
	// 83194FB0: 4BFFF7D9  bl 0x83194788
	ctx.lr = 0x83194FB4;
	sub_83194788(ctx, base);
	// 83194FB4: 817FFF80  lwz r11, -0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-128 as u32) ) } as u64;
	// 83194FB8: 3B7E01C1  addi r27, r30, 0x1c1
	ctx.r[27].s64 = ctx.r[30].s64 + 449;
	// 83194FBC: 7F8BEA14  add r28, r11, r29
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 83194FC0: 3B400020  li r26, 0x20
	ctx.r[26].s64 = 32;
	// 83194FC4: 38BFFF94  addi r5, r31, -0x6c
	ctx.r[5].s64 = ctx.r[31].s64 + -108;
	// 83194FC8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83194FCC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83194FD0: 387BFFFF  addi r3, r27, -1
	ctx.r[3].s64 = ctx.r[27].s64 + -1;
	// 83194FD4: 4BFFF7B5  bl 0x83194788
	ctx.lr = 0x83194FD8;
	sub_83194788(ctx, base);
	// 83194FD8: 38BFFF94  addi r5, r31, -0x6c
	ctx.r[5].s64 = ctx.r[31].s64 + -108;
	// 83194FDC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83194FE0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83194FE4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83194FE8: 4BFFF7A1  bl 0x83194788
	ctx.lr = 0x83194FEC;
	sub_83194788(ctx, base);
	// 83194FEC: 38BFFF94  addi r5, r31, -0x6c
	ctx.r[5].s64 = ctx.r[31].s64 + -108;
	// 83194FF0: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83194FF4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83194FF8: 387B0001  addi r3, r27, 1
	ctx.r[3].s64 = ctx.r[27].s64 + 1;
	// 83194FFC: 4BFFF78D  bl 0x83194788
	ctx.lr = 0x83195000;
	sub_83194788(ctx, base);
	// 83195000: 38BFFF94  addi r5, r31, -0x6c
	ctx.r[5].s64 = ctx.r[31].s64 + -108;
	// 83195004: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 83195008: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8319500C: 387B0002  addi r3, r27, 2
	ctx.r[3].s64 = ctx.r[27].s64 + 2;
	// 83195010: 4BFFF779  bl 0x83194788
	ctx.lr = 0x83195014;
	sub_83194788(ctx, base);
	// 83195014: 38BFFF94  addi r5, r31, -0x6c
	ctx.r[5].s64 = ctx.r[31].s64 + -108;
	// 83195018: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8319501C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83195020: 387B0004  addi r3, r27, 4
	ctx.r[3].s64 = ctx.r[27].s64 + 4;
	// 83195024: 4BFFF765  bl 0x83194788
	ctx.lr = 0x83195028;
	sub_83194788(ctx, base);
	// 83195028: 38BFFF94  addi r5, r31, -0x6c
	ctx.r[5].s64 = ctx.r[31].s64 + -108;
	// 8319502C: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 83195030: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83195034: 387B0009  addi r3, r27, 9
	ctx.r[3].s64 = ctx.r[27].s64 + 9;
	// 83195038: 4BFFF751  bl 0x83194788
	ctx.lr = 0x8319503C;
	sub_83194788(ctx, base);
	// 8319503C: 38BFFF94  addi r5, r31, -0x6c
	ctx.r[5].s64 = ctx.r[31].s64 + -108;
	// 83195040: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 83195044: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83195048: 387B000D  addi r3, r27, 0xd
	ctx.r[3].s64 = ctx.r[27].s64 + 13;
	// 8319504C: 4BFFF73D  bl 0x83194788
	ctx.lr = 0x83195050;
	sub_83194788(ctx, base);
	// 83195050: 817FFFB0  lwz r11, -0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-80 as u32) ) } as u64;
	// 83195054: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 83195058: 7F8BE214  add r28, r11, r28
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8319505C: 3B7B0010  addi r27, r27, 0x10
	ctx.r[27].s64 = ctx.r[27].s64 + 16;
	// 83195060: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 83195064: 409AFF60  bne cr6, 0x83194fc4
	if !ctx.cr[6].eq {
	pc = 0x83194FC4; continue 'dispatch;
	}
	// 83195068: 817FFF84  lwz r11, -0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-124 as u32) ) } as u64;
	// 8319506C: 3B7E03C1  addi r27, r30, 0x3c1
	ctx.r[27].s64 = ctx.r[30].s64 + 961;
	// 83195070: 3B400010  li r26, 0x10
	ctx.r[26].s64 = 16;
	// 83195074: 7F8BEA14  add r28, r11, r29
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 83195078: 38BFFFB4  addi r5, r31, -0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + -76;
	// 8319507C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83195080: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83195084: 387BFFFF  addi r3, r27, -1
	ctx.r[3].s64 = ctx.r[27].s64 + -1;
	// 83195088: 4BFFF701  bl 0x83194788
	ctx.lr = 0x8319508C;
	sub_83194788(ctx, base);
	// 8319508C: 38BFFFB4  addi r5, r31, -0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + -76;
	// 83195090: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83195094: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83195098: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8319509C: 4BFFF6ED  bl 0x83194788
	ctx.lr = 0x831950A0;
	sub_83194788(ctx, base);
	// 831950A0: 38BFFFB4  addi r5, r31, -0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + -76;
	// 831950A4: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 831950A8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831950AC: 387B0001  addi r3, r27, 1
	ctx.r[3].s64 = ctx.r[27].s64 + 1;
	// 831950B0: 4BFFF6D9  bl 0x83194788
	ctx.lr = 0x831950B4;
	sub_83194788(ctx, base);
	// 831950B4: 38BFFFB4  addi r5, r31, -0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + -76;
	// 831950B8: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 831950BC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831950C0: 387B0003  addi r3, r27, 3
	ctx.r[3].s64 = ctx.r[27].s64 + 3;
	// 831950C4: 4BFFF6C5  bl 0x83194788
	ctx.lr = 0x831950C8;
	sub_83194788(ctx, base);
	// 831950C8: 38BFFFB4  addi r5, r31, -0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + -76;
	// 831950CC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 831950D0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831950D4: 387B0005  addi r3, r27, 5
	ctx.r[3].s64 = ctx.r[27].s64 + 5;
	// 831950D8: 4BFFF6B1  bl 0x83194788
	ctx.lr = 0x831950DC;
	sub_83194788(ctx, base);
	// 831950DC: 38BFFFB4  addi r5, r31, -0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + -76;
	// 831950E0: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 831950E4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831950E8: 387B0009  addi r3, r27, 9
	ctx.r[3].s64 = ctx.r[27].s64 + 9;
	// 831950EC: 4BFFF69D  bl 0x83194788
	ctx.lr = 0x831950F0;
	sub_83194788(ctx, base);
	// 831950F0: 38BFFFB4  addi r5, r31, -0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + -76;
	// 831950F4: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 831950F8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831950FC: 387B000B  addi r3, r27, 0xb
	ctx.r[3].s64 = ctx.r[27].s64 + 11;
	// 83195100: 4BFFF689  bl 0x83194788
	ctx.lr = 0x83195104;
	sub_83194788(ctx, base);
	// 83195104: 38BFFFB4  addi r5, r31, -0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + -76;
	// 83195108: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8319510C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83195110: 387B000F  addi r3, r27, 0xf
	ctx.r[3].s64 = ctx.r[27].s64 + 15;
	// 83195114: 4BFFF675  bl 0x83194788
	ctx.lr = 0x83195118;
	sub_83194788(ctx, base);
	// 83195118: 38BFFFB4  addi r5, r31, -0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + -76;
	// 8319511C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83195120: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83195124: 387B0013  addi r3, r27, 0x13
	ctx.r[3].s64 = ctx.r[27].s64 + 19;
	// 83195128: 4BFFF661  bl 0x83194788
	ctx.lr = 0x8319512C;
	sub_83194788(ctx, base);
	// 8319512C: 38BFFFB4  addi r5, r31, -0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + -76;
	// 83195130: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 83195134: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83195138: 387B001F  addi r3, r27, 0x1f
	ctx.r[3].s64 = ctx.r[27].s64 + 31;
	// 8319513C: 4BFFF64D  bl 0x83194788
	ctx.lr = 0x83195140;
	sub_83194788(ctx, base);
	// 83195140: 38BFFFB4  addi r5, r31, -0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + -76;
	// 83195144: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 83195148: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8319514C: 387B0020  addi r3, r27, 0x20
	ctx.r[3].s64 = ctx.r[27].s64 + 32;
	// 83195150: 4BFFF639  bl 0x83194788
	ctx.lr = 0x83195154;
	sub_83194788(ctx, base);
	// 83195154: 38BFFFB4  addi r5, r31, -0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + -76;
	// 83195158: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 8319515C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83195160: 387B0021  addi r3, r27, 0x21
	ctx.r[3].s64 = ctx.r[27].s64 + 33;
	// 83195164: 4BFFF625  bl 0x83194788
	ctx.lr = 0x83195168;
	sub_83194788(ctx, base);
	// 83195168: 38BFFFB4  addi r5, r31, -0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + -76;
	// 8319516C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 83195170: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83195174: 387B0022  addi r3, r27, 0x22
	ctx.r[3].s64 = ctx.r[27].s64 + 34;
	// 83195178: 4BFFF611  bl 0x83194788
	ctx.lr = 0x8319517C;
	sub_83194788(ctx, base);
	// 8319517C: 38BFFFB4  addi r5, r31, -0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + -76;
	// 83195180: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 83195184: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83195188: 387B0023  addi r3, r27, 0x23
	ctx.r[3].s64 = ctx.r[27].s64 + 35;
	// 8319518C: 4BFFF5FD  bl 0x83194788
	ctx.lr = 0x83195190;
	sub_83194788(ctx, base);
	// 83195190: 38BFFFB4  addi r5, r31, -0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + -76;
	// 83195194: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 83195198: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8319519C: 387B0024  addi r3, r27, 0x24
	ctx.r[3].s64 = ctx.r[27].s64 + 36;
	// 831951A0: 4BFFF5E9  bl 0x83194788
	ctx.lr = 0x831951A4;
	sub_83194788(ctx, base);
	// 831951A4: 38BFFFB4  addi r5, r31, -0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + -76;
	// 831951A8: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 831951AC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831951B0: 387B0025  addi r3, r27, 0x25
	ctx.r[3].s64 = ctx.r[27].s64 + 37;
	// 831951B4: 4BFFF5D5  bl 0x83194788
	ctx.lr = 0x831951B8;
	sub_83194788(ctx, base);
	// 831951B8: 38BFFFB4  addi r5, r31, -0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + -76;
	// 831951BC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 831951C0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831951C4: 387B0026  addi r3, r27, 0x26
	ctx.r[3].s64 = ctx.r[27].s64 + 38;
	// 831951C8: 4BFFF5C1  bl 0x83194788
	ctx.lr = 0x831951CC;
	sub_83194788(ctx, base);
	// 831951CC: 38BFFFB4  addi r5, r31, -0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + -76;
	// 831951D0: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 831951D4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831951D8: 387B0027  addi r3, r27, 0x27
	ctx.r[3].s64 = ctx.r[27].s64 + 39;
	// 831951DC: 4BFFF5AD  bl 0x83194788
	ctx.lr = 0x831951E0;
	sub_83194788(ctx, base);
	// 831951E0: 817FFFFC  lwz r11, -4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831951E4: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 831951E8: 7F8BE214  add r28, r11, r28
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 831951EC: 3B7B0040  addi r27, r27, 0x40
	ctx.r[27].s64 = ctx.r[27].s64 + 64;
	// 831951F0: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 831951F4: 409AFE84  bne cr6, 0x83195078
	if !ctx.cr[6].eq {
	pc = 0x83195078; continue 'dispatch;
	}
	// 831951F8: 817FFF88  lwz r11, -0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-120 as u32) ) } as u64;
	// 831951FC: 38BFFF10  addi r5, r31, -0xf0
	ctx.r[5].s64 = ctx.r[31].s64 + -240;
	// 83195200: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83195204: 7F8BEA14  add r28, r11, r29
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 83195208: 387E07C0  addi r3, r30, 0x7c0
	ctx.r[3].s64 = ctx.r[30].s64 + 1984;
	// 8319520C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83195210: 4BFFF579  bl 0x83194788
	ctx.lr = 0x83195214;
	sub_83194788(ctx, base);
	// 83195214: 38BFFF10  addi r5, r31, -0xf0
	ctx.r[5].s64 = ctx.r[31].s64 + -240;
	// 83195218: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8319521C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83195220: 387E07C1  addi r3, r30, 0x7c1
	ctx.r[3].s64 = ctx.r[30].s64 + 1985;
	// 83195224: 4BFFF565  bl 0x83194788
	ctx.lr = 0x83195228;
	sub_83194788(ctx, base);
	// 83195228: 817FFF8C  lwz r11, -0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-116 as u32) ) } as u64;
	// 8319522C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83195230: 7FABEA14  add r29, r11, r29
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 83195234: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83195238: 387E07E0  addi r3, r30, 0x7e0
	ctx.r[3].s64 = ctx.r[30].s64 + 2016;
	// 8319523C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83195240: 4BFFF549  bl 0x83194788
	ctx.lr = 0x83195244;
	sub_83194788(ctx, base);
	// 83195244: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83195248: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8319524C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83195250: 387E07E1  addi r3, r30, 0x7e1
	ctx.r[3].s64 = ctx.r[30].s64 + 2017;
	// 83195254: 4BFFF535  bl 0x83194788
	ctx.lr = 0x83195258;
	sub_83194788(ctx, base);
	// 83195258: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8319525C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83195260: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83195264: 387E07E2  addi r3, r30, 0x7e2
	ctx.r[3].s64 = ctx.r[30].s64 + 2018;
	// 83195268: 4BFFF521  bl 0x83194788
	ctx.lr = 0x8319526C;
	sub_83194788(ctx, base);
	// 8319526C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83195270: 48012F40  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195278 size=96
    let mut pc: u32 = 0x83195278;
    'dispatch: loop {
        match pc {
            0x83195278 => {
    //   block [0x83195278..0x831952D8)
	// 83195278: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8319527C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 83195280: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83195284: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 83195288: 394BB5EC  addi r10, r11, -0x4a14
	ctx.r[10].s64 = ctx.r[11].s64 + -18964;
	// 8319528C: 91040000  stw r8, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83195290: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83195294: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 83195298: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8319529C: 38EB0018  addi r7, r11, 0x18
	ctx.r[7].s64 = ctx.r[11].s64 + 24;
	// 831952A0: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831952A4: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831952A8: 7D064051  subf. r8, r6, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 831952AC: 40820014  bne 0x831952c0
	if !ctx.cr[0].eq {
	pc = 0x831952C0; continue 'dispatch;
	}
	// 831952B0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831952B4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831952B8: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 831952BC: 409AFFE4  bne cr6, 0x831952a0
	if !ctx.cr[6].eq {
	pc = 0x831952A0; continue 'dispatch;
	}
	// 831952C0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 831952C4: 419A0014  beq cr6, 0x831952d8
	if ctx.cr[6].eq {
		sub_831952D8(ctx, base);
		return;
	}
	// 831952C8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831952CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831952D0: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831952D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831952D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831952D8 size=16
    let mut pc: u32 = 0x831952D8;
    'dispatch: loop {
        match pc {
            0x831952D8 => {
    //   block [0x831952D8..0x831952E8)
	// 831952D8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831952DC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831952E0: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831952E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831952E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831952E8 size=160
    let mut pc: u32 = 0x831952E8;
    'dispatch: loop {
        match pc {
            0x831952E8 => {
    //   block [0x831952E8..0x83195388)
	// 831952E8: 392301C0  addi r9, r3, 0x1c0
	ctx.r[9].s64 = ctx.r[3].s64 + 448;
	// 831952EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831952F0: 5488063E  clrlwi r8, r4, 0x18
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 831952F4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 831952F8: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831952FC: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 83195300: 419A0088  beq cr6, 0x83195388
	if ctx.cr[6].eq {
		sub_83195388(ctx, base);
		return;
	}
	// 83195304: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83195308: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 8319530C: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 83195310: 4198FFE8  blt cr6, 0x831952f8
	if ctx.cr[6].lt {
	pc = 0x831952F8; continue 'dispatch;
	}
	// 83195314: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83195318: 39690200  addi r11, r9, 0x200
	ctx.r[11].s64 = ctx.r[9].s64 + 512;
	// 8319531C: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83195320: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 83195324: 419A0070  beq cr6, 0x83195394
	if ctx.cr[6].eq {
		sub_83195394(ctx, base);
		return;
	}
	// 83195328: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8319532C: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 83195330: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 83195334: 4198FFE8  blt cr6, 0x8319531c
	if ctx.cr[6].lt {
	pc = 0x8319531C; continue 'dispatch;
	}
	// 83195338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8319533C: 39690600  addi r11, r9, 0x600
	ctx.r[11].s64 = ctx.r[9].s64 + 1536;
	// 83195340: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83195344: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 83195348: 419A005C  beq cr6, 0x831953a4
	if ctx.cr[6].eq {
		sub_831953A4(ctx, base);
		return;
	}
	// 8319534C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83195350: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 83195354: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83195358: 4198FFE8  blt cr6, 0x83195340
	if ctx.cr[6].lt {
	pc = 0x83195340; continue 'dispatch;
	}
	// 8319535C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83195360: 39690620  addi r11, r9, 0x620
	ctx.r[11].s64 = ctx.r[9].s64 + 1568;
	// 83195364: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83195368: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8319536C: 419A0048  beq cr6, 0x831953b4
	if ctx.cr[6].eq {
		sub_831953B4(ctx, base);
		return;
	}
	// 83195370: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83195374: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 83195378: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 8319537C: 4198FFE8  blt cr6, 0x83195364
	if ctx.cr[6].lt {
	pc = 0x83195364; continue 'dispatch;
	}
	// 83195380: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83195384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195388 size=12
    let mut pc: u32 = 0x83195388;
    'dispatch: loop {
        match pc {
            0x83195388 => {
    //   block [0x83195388..0x83195394)
	// 83195388: 554B2036  slwi r11, r10, 4
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8319538C: 7C6B4A14  add r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 83195390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195394(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195394 size=16
    let mut pc: u32 = 0x83195394;
    'dispatch: loop {
        match pc {
            0x83195394 => {
    //   block [0x83195394..0x831953A4)
	// 83195394: 396A0008  addi r11, r10, 8
	ctx.r[11].s64 = ctx.r[10].s64 + 8;
	// 83195398: 556B3032  slwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8319539C: 7C6B4A14  add r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 831953A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831953A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831953A4 size=16
    let mut pc: u32 = 0x831953A4;
    'dispatch: loop {
        match pc {
            0x831953A4 => {
    //   block [0x831953A4..0x831953B4)
	// 831953A4: 396A0030  addi r11, r10, 0x30
	ctx.r[11].s64 = ctx.r[10].s64 + 48;
	// 831953A8: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831953AC: 7C6B4A14  add r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 831953B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831953B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831953B4 size=16
    let mut pc: u32 = 0x831953B4;
    'dispatch: loop {
        match pc {
            0x831953B4 => {
    //   block [0x831953B4..0x831953C4)
	// 831953B4: 396A0031  addi r11, r10, 0x31
	ctx.r[11].s64 = ctx.r[10].s64 + 49;
	// 831953B8: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831953BC: 7C6B4A14  add r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 831953C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831953C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831953C8 size=96
    let mut pc: u32 = 0x831953C8;
    'dispatch: loop {
        match pc {
            0x831953C8 => {
    //   block [0x831953C8..0x83195428)
	// 831953C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831953CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831953D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831953D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831953D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831953DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831953E0: 83C30004  lwz r30, 4(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831953E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831953E8: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 831953EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831953F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831953F4: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 831953F8: 48012DE9  bl 0x831a81e0
	ctx.lr = 0x831953FC;
	sub_831A81E0(ctx, base);
	// 831953FC: 389E0040  addi r4, r30, 0x40
	ctx.r[4].s64 = ctx.r[30].s64 + 64;
	// 83195400: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 83195404: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83195408: 48013109  bl 0x831a8510
	ctx.lr = 0x8319540C;
	sub_831A8510(ctx, base);
	// 8319540C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195410: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83195414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319541C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83195420: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83195424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195428 size=44
    let mut pc: u32 = 0x83195428;
    'dispatch: loop {
        match pc {
            0x83195428 => {
    //   block [0x83195428..0x83195454)
	// 83195428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8319542C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83195430: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195434: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83195438: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8319543C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83195440: 894B0038  lbz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 83195444: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83195448: 896B0039  lbz r11, 0x39(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(57 as u32) ) } as u64;
	// 8319544C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195458 size=44
    let mut pc: u32 = 0x83195458;
    'dispatch: loop {
        match pc {
            0x83195458 => {
    //   block [0x83195458..0x83195484)
	// 83195458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8319545C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83195460: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195464: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83195468: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8319546C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83195470: 894B003A  lbz r10, 0x3a(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(58 as u32) ) } as u64;
	// 83195474: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83195478: 896B003B  lbz r11, 0x3b(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(59 as u32) ) } as u64;
	// 8319547C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195488 size=48
    let mut pc: u32 = 0x83195488;
    'dispatch: loop {
        match pc {
            0x83195488 => {
    //   block [0x83195488..0x831954B8)
	// 83195488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8319548C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83195490: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195494: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83195498: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8319549C: 892B0012  lbz r9, 0x12(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 831954A0: 894B0013  lbz r10, 0x13(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(19 as u32) ) } as u64;
	// 831954A4: 552B403E  rotlwi r11, r9, 8
	ctx.r[11].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 831954A8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831954AC: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 831954B0: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831954B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831954B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831954B8 size=16
    let mut pc: u32 = 0x831954B8;
    'dispatch: loop {
        match pc {
            0x831954B8 => {
    //   block [0x831954B8..0x831954C8)
	// 831954B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831954BC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831954C0: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831954C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831954C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831954C8 size=16
    let mut pc: u32 = 0x831954C8;
    'dispatch: loop {
        match pc {
            0x831954C8 => {
    //   block [0x831954C8..0x831954D8)
	// 831954C8: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 831954CC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831954D0: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831954D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831954D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831954D8 size=20
    let mut pc: u32 = 0x831954D8;
    'dispatch: loop {
        match pc {
            0x831954D8 => {
    //   block [0x831954D8..0x831954EC)
	// 831954D8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831954DC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831954E0: 896B00C0  lbz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 831954E4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831954E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831954F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831954F0 size=20
    let mut pc: u32 = 0x831954F0;
    'dispatch: loop {
        match pc {
            0x831954F0 => {
    //   block [0x831954F0..0x83195504)
	// 831954F0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831954F4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831954F8: 896B00C1  lbz r11, 0xc1(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(193 as u32) ) } as u64;
	// 831954FC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195508 size=20
    let mut pc: u32 = 0x83195508;
    'dispatch: loop {
        match pc {
            0x83195508 => {
    //   block [0x83195508..0x8319551C)
	// 83195508: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8319550C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195510: 896B00C2  lbz r11, 0xc2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(194 as u32) ) } as u64;
	// 83195514: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195520 size=20
    let mut pc: u32 = 0x83195520;
    'dispatch: loop {
        match pc {
            0x83195520 => {
    //   block [0x83195520..0x83195534)
	// 83195520: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83195524: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195528: 896B00C3  lbz r11, 0xc3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(195 as u32) ) } as u64;
	// 8319552C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195538 size=56
    let mut pc: u32 = 0x83195538;
    'dispatch: loop {
        match pc {
            0x83195538 => {
    //   block [0x83195538..0x83195570)
	// 83195538: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8319553C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195540: 894B00C4  lbz r10, 0xc4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(196 as u32) ) } as u64;
	// 83195544: 88EB00C5  lbz r7, 0xc5(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(197 as u32) ) } as u64;
	// 83195548: 5548403E  rotlwi r8, r10, 8
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 8319554C: 892B00C6  lbz r9, 0xc6(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(198 as u32) ) } as u64;
	// 83195550: 894B00C7  lbz r10, 0xc7(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(199 as u32) ) } as u64;
	// 83195554: 7D683A14  add r11, r8, r7
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 83195558: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8319555C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 83195560: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83195564: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83195568: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8319556C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195570 size=56
    let mut pc: u32 = 0x83195570;
    'dispatch: loop {
        match pc {
            0x83195570 => {
    //   block [0x83195570..0x831955A8)
	// 83195570: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83195574: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195578: 894B00C8  lbz r10, 0xc8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 8319557C: 88EB00C9  lbz r7, 0xc9(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(201 as u32) ) } as u64;
	// 83195580: 5548403E  rotlwi r8, r10, 8
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 83195584: 892B00CA  lbz r9, 0xca(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(202 as u32) ) } as u64;
	// 83195588: 894B00CB  lbz r10, 0xcb(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(203 as u32) ) } as u64;
	// 8319558C: 7D683A14  add r11, r8, r7
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 83195590: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83195594: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 83195598: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8319559C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831955A0: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831955A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831955A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831955A8 size=56
    let mut pc: u32 = 0x831955A8;
    'dispatch: loop {
        match pc {
            0x831955A8 => {
    //   block [0x831955A8..0x831955E0)
	// 831955A8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831955AC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831955B0: 894B00CC  lbz r10, 0xcc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(204 as u32) ) } as u64;
	// 831955B4: 88EB00CD  lbz r7, 0xcd(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(205 as u32) ) } as u64;
	// 831955B8: 5548403E  rotlwi r8, r10, 8
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 831955BC: 892B00CE  lbz r9, 0xce(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(206 as u32) ) } as u64;
	// 831955C0: 894B00CF  lbz r10, 0xcf(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(207 as u32) ) } as u64;
	// 831955C4: 7D683A14  add r11, r8, r7
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 831955C8: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831955CC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 831955D0: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831955D4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831955D8: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831955DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831955E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831955E0 size=56
    let mut pc: u32 = 0x831955E0;
    'dispatch: loop {
        match pc {
            0x831955E0 => {
    //   block [0x831955E0..0x83195618)
	// 831955E0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831955E4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831955E8: 894B00D0  lbz r10, 0xd0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 831955EC: 88EB00D1  lbz r7, 0xd1(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(209 as u32) ) } as u64;
	// 831955F0: 5548403E  rotlwi r8, r10, 8
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 831955F4: 892B00D2  lbz r9, 0xd2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(210 as u32) ) } as u64;
	// 831955F8: 894B00D3  lbz r10, 0xd3(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(211 as u32) ) } as u64;
	// 831955FC: 7D683A14  add r11, r8, r7
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 83195600: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83195604: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 83195608: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8319560C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83195610: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195618 size=56
    let mut pc: u32 = 0x83195618;
    'dispatch: loop {
        match pc {
            0x83195618 => {
    //   block [0x83195618..0x83195650)
	// 83195618: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8319561C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195620: 894B00D4  lbz r10, 0xd4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(212 as u32) ) } as u64;
	// 83195624: 88EB00D5  lbz r7, 0xd5(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(213 as u32) ) } as u64;
	// 83195628: 5548403E  rotlwi r8, r10, 8
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 8319562C: 892B00D6  lbz r9, 0xd6(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(214 as u32) ) } as u64;
	// 83195630: 894B00D7  lbz r10, 0xd7(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(215 as u32) ) } as u64;
	// 83195634: 7D683A14  add r11, r8, r7
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 83195638: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8319563C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 83195640: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83195644: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83195648: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8319564C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195650 size=12
    let mut pc: u32 = 0x83195650;
    'dispatch: loop {
        match pc {
            0x83195650 => {
    //   block [0x83195650..0x8319565C)
	// 83195650: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83195654: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83195658: 4BFFFC90  b 0x831952e8
	sub_831952E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83195660 size=224
    let mut pc: u32 = 0x83195660;
    'dispatch: loop {
        match pc {
            0x83195660 => {
    //   block [0x83195660..0x831956EC)
	// 83195660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83195664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83195668: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319566C: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83195670: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 83195674: 4BFFFFDD  bl 0x83195650
	ctx.lr = 0x83195678;
	sub_83195650(ctx, base);
	// 83195678: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8319567C: 409A0014  bne cr6, 0x83195690
	if !ctx.cr[6].eq {
	pc = 0x83195690; continue 'dispatch;
	}
	// 83195680: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319568C: 4E800020  blr
	return;
	// 83195690: 81660010  lwz r11, 0x10(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) } as u64;
	// 83195694: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83195698: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 8319569C: 409A000C  bne cr6, 0x831956a8
	if !ctx.cr[6].eq {
	pc = 0x831956A8; continue 'dispatch;
	}
	// 831956A0: 556BE13E  srwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831956A4: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 831956A8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 831956AC: 396BFFDF  addi r11, r11, -0x21
	ctx.r[11].s64 = ctx.r[11].s64 + -33;
	// 831956B0: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 831956B4: 41990070  bgt cr6, 0x83195724
	if ctx.cr[6].gt {
	pc = 0x83195724; continue 'dispatch;
	}
	// 831956B8: 3D808319  lis r12, -0x7ce7
	ctx.r[12].s64 = -2095513600;
	// 831956BC: 398C56D0  addi r12, r12, 0x56d0
	ctx.r[12].s64 = ctx.r[12].s64 + 22224;
	// 831956C0: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 831956C4: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 831956C8: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831956CC: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x831956EC; continue 'dispatch;
		},
		1 => {
	pc = 0x831956F4; continue 'dispatch;
		},
		2 => {
	pc = 0x831956FC; continue 'dispatch;
		},
		3 => {
	pc = 0x83195704; continue 'dispatch;
		},
		4 => {
	pc = 0x8319570C; continue 'dispatch;
		},
		5 => {
	pc = 0x83195714; continue 'dispatch;
		},
		6 => {
	pc = 0x8319571C; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 831956D0: 831956EC  lwz r24, 0x56ec(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(22252 as u32) ) } as u64;
	// 831956D4: 831956F4  lwz r24, 0x56f4(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(22260 as u32) ) } as u64;
	// 831956D8: 831956FC  lwz r24, 0x56fc(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(22268 as u32) ) } as u64;
	// 831956DC: 83195704  lwz r24, 0x5704(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(22276 as u32) ) } as u64;
	// 831956E0: 8319570C  lwz r24, 0x570c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(22284 as u32) ) } as u64;
	// 831956E4: 83195714  lwz r24, 0x5714(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(22292 as u32) ) } as u64;
	// 831956E8: 8319571C  lwz r24, 0x571c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(22300 as u32) ) } as u64;
            }
            0x831956EC => {
    //   block [0x831956EC..0x831956F4)
	// 831956EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831956F0: 48000038  b 0x83195728
	pc = 0x83195728; continue 'dispatch;
            }
            0x831956F4 => {
    //   block [0x831956F4..0x831956FC)
	// 831956F4: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 831956F8: 48000030  b 0x83195728
	pc = 0x83195728; continue 'dispatch;
            }
            0x831956FC => {
    //   block [0x831956FC..0x83195704)
	// 831956FC: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 83195700: 48000028  b 0x83195728
	pc = 0x83195728; continue 'dispatch;
            }
            0x83195704 => {
    //   block [0x83195704..0x8319570C)
	// 83195704: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83195708: 48000020  b 0x83195728
	pc = 0x83195728; continue 'dispatch;
            }
            0x8319570C => {
    //   block [0x8319570C..0x83195714)
	// 8319570C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83195710: 48000018  b 0x83195728
	pc = 0x83195728; continue 'dispatch;
            }
            0x83195714 => {
    //   block [0x83195714..0x8319571C)
	// 83195714: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 83195718: 48000010  b 0x83195728
	pc = 0x83195728; continue 'dispatch;
            }
            0x8319571C => {
    //   block [0x8319571C..0x83195740)
	// 8319571C: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 83195720: 48000008  b 0x83195728
	pc = 0x83195728; continue 'dispatch;
	// 83195724: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83195728: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8319572C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319573C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83195740 size=152
    let mut pc: u32 = 0x83195740;
    'dispatch: loop {
        match pc {
            0x83195740 => {
    //   block [0x83195740..0x831957D8)
	// 83195740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83195744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83195748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319574C: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83195750: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 83195754: 4BFFFEFD  bl 0x83195650
	ctx.lr = 0x83195758;
	sub_83195650(ctx, base);
	// 83195758: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8319575C: 409A0018  bne cr6, 0x83195774
	if !ctx.cr[6].eq {
	pc = 0x83195774; continue 'dispatch;
	}
	// 83195760: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83195764: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319576C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195770: 4E800020  blr
	return;
	// 83195774: 81660010  lwz r11, 0x10(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) } as u64;
	// 83195778: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 8319577C: 409A0010  bne cr6, 0x8319578c
	if !ctx.cr[6].eq {
	pc = 0x8319578C; continue 'dispatch;
	}
	// 83195780: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 83195784: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 83195788: 48000034  b 0x831957bc
	pc = 0x831957BC; continue 'dispatch;
	// 8319578C: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 83195790: 2F0B0024  cmpwi cr6, r11, 0x24
	ctx.cr[6].compare_i32(ctx.r[11].s32, 36, &mut ctx.xer);
	// 83195794: 419A0024  beq cr6, 0x831957b8
	if ctx.cr[6].eq {
	pc = 0x831957B8; continue 'dispatch;
	}
	// 83195798: 2F0B0025  cmpwi cr6, r11, 0x25
	ctx.cr[6].compare_i32(ctx.r[11].s32, 37, &mut ctx.xer);
	// 8319579C: 419A0014  beq cr6, 0x831957b0
	if ctx.cr[6].eq {
	pc = 0x831957B0; continue 'dispatch;
	}
	// 831957A0: 2F0B0026  cmpwi cr6, r11, 0x26
	ctx.cr[6].compare_i32(ctx.r[11].s32, 38, &mut ctx.xer);
	// 831957A4: 409AFFBC  bne cr6, 0x83195760
	if !ctx.cr[6].eq {
	pc = 0x83195760; continue 'dispatch;
	}
	// 831957A8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 831957AC: 48000010  b 0x831957bc
	pc = 0x831957BC; continue 'dispatch;
	// 831957B0: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 831957B4: 48000008  b 0x831957bc
	pc = 0x831957BC; continue 'dispatch;
	// 831957B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831957BC: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 831957C0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831957C4: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831957C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831957CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831957D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831957D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831957D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831957D8 size=76
    let mut pc: u32 = 0x831957D8;
    'dispatch: loop {
        match pc {
            0x831957D8 => {
    //   block [0x831957D8..0x83195824)
	// 831957D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831957DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831957E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831957E4: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 831957E8: 4BFFFE69  bl 0x83195650
	ctx.lr = 0x831957EC;
	sub_83195650(ctx, base);
	// 831957EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831957F0: 409A0014  bne cr6, 0x83195804
	if !ctx.cr[6].eq {
	pc = 0x83195804; continue 'dispatch;
	}
	// 831957F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831957F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831957FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195800: 4E800020  blr
	return;
	// 83195804: 89630002  lbz r11, 2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 83195808: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8319580C: 556BE13E  srwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83195810: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195814: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319581C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83195828 size=104
    let mut pc: u32 = 0x83195828;
    'dispatch: loop {
        match pc {
            0x83195828 => {
    //   block [0x83195828..0x83195890)
	// 83195828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319582C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83195830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83195834: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83195838: 4BFFFE19  bl 0x83195650
	ctx.lr = 0x8319583C;
	sub_83195650(ctx, base);
	// 8319583C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83195840: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83195844: 409A0014  bne cr6, 0x83195858
	if !ctx.cr[6].eq {
	pc = 0x83195858; continue 'dispatch;
	}
	// 83195848: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8319584C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195854: 4E800020  blr
	return;
	// 83195858: 894B0003  lbz r10, 3(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8319585C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195860: 892B0004  lbz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83195864: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 83195868: 896B0002  lbz r11, 2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8319586C: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 83195870: 556B831E  rlwinm r11, r11, 0x10, 0xc, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 83195874: 554A033E  clrlwi r10, r10, 0xc
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000FFFFFu64;
	// 83195878: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8319587C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195880: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319588C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83195890 size=212
    let mut pc: u32 = 0x83195890;
    'dispatch: loop {
        match pc {
            0x83195890 => {
    //   block [0x83195890..0x83195920)
	// 83195890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83195894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83195898: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319589C: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 831958A0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 831958A4: 4BFFFDAD  bl 0x83195650
	ctx.lr = 0x831958A8;
	sub_83195650(ctx, base);
	// 831958A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831958AC: 409A0014  bne cr6, 0x831958c0
	if !ctx.cr[6].eq {
	pc = 0x831958C0; continue 'dispatch;
	}
	// 831958B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831958B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831958B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831958BC: 4E800020  blr
	return;
	// 831958C0: 81660010  lwz r11, 0x10(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) } as u64;
	// 831958C4: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 831958C8: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 831958CC: 409A000C  bne cr6, 0x831958d8
	if !ctx.cr[6].eq {
	pc = 0x831958D8; continue 'dispatch;
	}
	// 831958D0: 556BE13E  srwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831958D4: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 831958D8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 831958DC: 396BFFBF  addi r11, r11, -0x41
	ctx.r[11].s64 = ctx.r[11].s64 + -65;
	// 831958E0: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 831958E4: 41990064  bgt cr6, 0x83195948
	if ctx.cr[6].gt {
	pc = 0x83195948; continue 'dispatch;
	}
	// 831958E8: 3D808319  lis r12, -0x7ce7
	ctx.r[12].s64 = -2095513600;
	// 831958EC: 398C5900  addi r12, r12, 0x5900
	ctx.r[12].s64 = ctx.r[12].s64 + 22784;
	// 831958F0: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 831958F4: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 831958F8: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831958FC: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x83195920; continue 'dispatch;
		},
		1 => {
	pc = 0x83195928; continue 'dispatch;
		},
		2 => {
	pc = 0x83195930; continue 'dispatch;
		},
		3 => {
	pc = 0x83195948; continue 'dispatch;
		},
		4 => {
	pc = 0x83195948; continue 'dispatch;
		},
		5 => {
	pc = 0x83195948; continue 'dispatch;
		},
		6 => {
	pc = 0x83195938; continue 'dispatch;
		},
		7 => {
	pc = 0x83195940; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 83195900: 83195920  lwz r24, 0x5920(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(22816 as u32) ) } as u64;
	// 83195904: 83195928  lwz r24, 0x5928(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(22824 as u32) ) } as u64;
	// 83195908: 83195930  lwz r24, 0x5930(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(22832 as u32) ) } as u64;
	// 8319590C: 83195948  lwz r24, 0x5948(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(22856 as u32) ) } as u64;
	// 83195910: 83195948  lwz r24, 0x5948(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(22856 as u32) ) } as u64;
	// 83195914: 83195948  lwz r24, 0x5948(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(22856 as u32) ) } as u64;
	// 83195918: 83195938  lwz r24, 0x5938(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(22840 as u32) ) } as u64;
	// 8319591C: 83195940  lwz r24, 0x5940(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(22848 as u32) ) } as u64;
            }
            0x83195920 => {
    //   block [0x83195920..0x83195928)
	// 83195920: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83195924: 48000028  b 0x8319594c
	pc = 0x8319594C; continue 'dispatch;
            }
            0x83195928 => {
    //   block [0x83195928..0x83195930)
	// 83195928: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8319592C: 48000020  b 0x8319594c
	pc = 0x8319594C; continue 'dispatch;
            }
            0x83195930 => {
    //   block [0x83195930..0x83195938)
	// 83195930: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83195934: 48000018  b 0x8319594c
	pc = 0x8319594C; continue 'dispatch;
            }
            0x83195938 => {
    //   block [0x83195938..0x83195940)
	// 83195938: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8319593C: 48000010  b 0x8319594c
	pc = 0x8319594C; continue 'dispatch;
            }
            0x83195940 => {
    //   block [0x83195940..0x83195948)
	// 83195940: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 83195944: 48000008  b 0x8319594c
	pc = 0x8319594C; continue 'dispatch;
            }
            0x83195948 => {
    //   block [0x83195948..0x83195964)
	// 83195948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8319594C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195950: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195954: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319595C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83195968 size=120
    let mut pc: u32 = 0x83195968;
    'dispatch: loop {
        match pc {
            0x83195968 => {
    //   block [0x83195968..0x831959E0)
	// 83195968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319596C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83195970: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83195974: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83195978: 4BFFFCD9  bl 0x83195650
	ctx.lr = 0x8319597C;
	sub_83195650(ctx, base);
	// 8319597C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83195980: 409A0014  bne cr6, 0x83195994
	if !ctx.cr[6].eq {
	pc = 0x83195994; continue 'dispatch;
	}
	// 83195984: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319598C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195990: 4E800020  blr
	return;
	// 83195994: 8963000C  lbz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83195998: 8903000D  lbz r8, 0xd(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 8319599C: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 831959A0: 8923000E  lbz r9, 0xe(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 831959A4: 8943000F  lbz r10, 0xf(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(15 as u32) ) } as u64;
	// 831959A8: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 831959AC: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831959B0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 831959B4: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831959B8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831959BC: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 831959C0: 409A0008  bne cr6, 0x831959c8
	if !ctx.cr[6].eq {
	pc = 0x831959C8; continue 'dispatch;
	}
	// 831959C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831959C8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831959CC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831959D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831959D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831959D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831959DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831959E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831959E0 size=116
    let mut pc: u32 = 0x831959E0;
    'dispatch: loop {
        match pc {
            0x831959E0 => {
    //   block [0x831959E0..0x83195A54)
	// 831959E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831959E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831959E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831959EC: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 831959F0: 4BFFFC61  bl 0x83195650
	ctx.lr = 0x831959F4;
	sub_83195650(ctx, base);
	// 831959F4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831959F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831959FC: 409A0014  bne cr6, 0x83195a10
	if !ctx.cr[6].eq {
	pc = 0x83195A10; continue 'dispatch;
	}
	// 83195A00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195A04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195A08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195A0C: 4E800020  blr
	return;
	// 83195A10: 894B0004  lbz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83195A14: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195A18: 890B0005  lbz r8, 5(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 83195A1C: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 83195A20: 88EB0002  lbz r7, 2(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83195A24: 892B0003  lbz r9, 3(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83195A28: 7D6A4214  add r11, r10, r8
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 83195A2C: 54EA403E  rotlwi r10, r7, 8
	ctx.r[10].u64 = ((ctx.r[7].u32).rotate_left(8)) as u64;
	// 83195A30: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 83195A34: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83195A38: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 83195A3C: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83195A40: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195A44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83195A58 size=92
    let mut pc: u32 = 0x83195A58;
    'dispatch: loop {
        match pc {
            0x83195A58 => {
    //   block [0x83195A58..0x83195AB4)
	// 83195A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83195A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83195A60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83195A64: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83195A68: 4BFFFBE9  bl 0x83195650
	ctx.lr = 0x83195A6C;
	sub_83195650(ctx, base);
	// 83195A6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83195A70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83195A74: 409A0014  bne cr6, 0x83195a88
	if !ctx.cr[6].eq {
	pc = 0x83195A88; continue 'dispatch;
	}
	// 83195A78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195A84: 4E800020  blr
	return;
	// 83195A88: 892B000A  lbz r9, 0xa(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 83195A8C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195A90: 894B000B  lbz r10, 0xb(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11 as u32) ) } as u64;
	// 83195A94: 552B403E  rotlwi r11, r9, 8
	ctx.r[11].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 83195A98: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83195A9C: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 83195AA0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195AA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195AA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195AAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83195AB8 size=72
    let mut pc: u32 = 0x83195AB8;
    'dispatch: loop {
        match pc {
            0x83195AB8 => {
    //   block [0x83195AB8..0x83195B00)
	// 83195AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83195ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83195AC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83195AC4: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83195AC8: 4BFFFB89  bl 0x83195650
	ctx.lr = 0x83195ACC;
	sub_83195650(ctx, base);
	// 83195ACC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83195AD0: 409A0014  bne cr6, 0x83195ae4
	if !ctx.cr[6].eq {
	pc = 0x83195AE4; continue 'dispatch;
	}
	// 83195AD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195AE0: 4E800020  blr
	return;
	// 83195AE4: 89630021  lbz r11, 0x21(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(33 as u32) ) } as u64;
	// 83195AE8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195AEC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195AF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83195B00 size=72
    let mut pc: u32 = 0x83195B00;
    'dispatch: loop {
        match pc {
            0x83195B00 => {
    //   block [0x83195B00..0x83195B48)
	// 83195B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83195B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83195B08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83195B0C: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83195B10: 4BFFFB41  bl 0x83195650
	ctx.lr = 0x83195B14;
	sub_83195650(ctx, base);
	// 83195B14: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83195B18: 409A0014  bne cr6, 0x83195b2c
	if !ctx.cr[6].eq {
	pc = 0x83195B2C; continue 'dispatch;
	}
	// 83195B1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195B28: 4E800020  blr
	return;
	// 83195B2C: 89630022  lbz r11, 0x22(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(34 as u32) ) } as u64;
	// 83195B30: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195B34: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195B38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83195B48 size=76
    let mut pc: u32 = 0x83195B48;
    'dispatch: loop {
        match pc {
            0x83195B48 => {
    //   block [0x83195B48..0x83195B94)
	// 83195B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83195B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83195B50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83195B54: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83195B58: 4BFFFAF9  bl 0x83195650
	ctx.lr = 0x83195B5C;
	sub_83195650(ctx, base);
	// 83195B5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83195B60: 409A0014  bne cr6, 0x83195b74
	if !ctx.cr[6].eq {
	pc = 0x83195B74; continue 'dispatch;
	}
	// 83195B64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195B68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195B6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195B70: 4E800020  blr
	return;
	// 83195B74: 89630023  lbz r11, 0x23(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(35 as u32) ) } as u64;
	// 83195B78: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195B7C: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 83195B80: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195B84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83195B98 size=76
    let mut pc: u32 = 0x83195B98;
    'dispatch: loop {
        match pc {
            0x83195B98 => {
    //   block [0x83195B98..0x83195BE4)
	// 83195B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83195B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83195BA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83195BA4: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83195BA8: 4BFFFAA9  bl 0x83195650
	ctx.lr = 0x83195BAC;
	sub_83195650(ctx, base);
	// 83195BAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83195BB0: 409A0014  bne cr6, 0x83195bc4
	if !ctx.cr[6].eq {
	pc = 0x83195BC4; continue 'dispatch;
	}
	// 83195BB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195BC0: 4E800020  blr
	return;
	// 83195BC4: 89630023  lbz r11, 0x23(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(35 as u32) ) } as u64;
	// 83195BC8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195BCC: 556BE7FE  rlwinm r11, r11, 0x1c, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 83195BD0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195BD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83195BE8 size=72
    let mut pc: u32 = 0x83195BE8;
    'dispatch: loop {
        match pc {
            0x83195BE8 => {
    //   block [0x83195BE8..0x83195C30)
	// 83195BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83195BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83195BF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83195BF4: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83195BF8: 4BFFFA59  bl 0x83195650
	ctx.lr = 0x83195BFC;
	sub_83195650(ctx, base);
	// 83195BFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83195C00: 409A0014  bne cr6, 0x83195c14
	if !ctx.cr[6].eq {
	pc = 0x83195C14; continue 'dispatch;
	}
	// 83195C04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195C10: 4E800020  blr
	return;
	// 83195C14: 89630024  lbz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 83195C18: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195C1C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195C20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83195C30 size=88
    let mut pc: u32 = 0x83195C30;
    'dispatch: loop {
        match pc {
            0x83195C30 => {
    //   block [0x83195C30..0x83195C88)
	// 83195C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83195C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83195C38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83195C3C: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83195C40: 4BFFFA11  bl 0x83195650
	ctx.lr = 0x83195C44;
	sub_83195650(ctx, base);
	// 83195C44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83195C48: 409A0014  bne cr6, 0x83195c5c
	if !ctx.cr[6].eq {
	pc = 0x83195C5C; continue 'dispatch;
	}
	// 83195C4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195C58: 4E800020  blr
	return;
	// 83195C5C: 89630025  lbz r11, 0x25(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(37 as u32) ) } as u64;
	// 83195C60: 2F0B003F  cmpwi cr6, r11, 0x3f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 63, &mut ctx.xer);
	// 83195C64: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195C68: 4099000C  ble cr6, 0x83195c74
	if !ctx.cr[6].gt {
	pc = 0x83195C74; continue 'dispatch;
	}
	// 83195C6C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83195C70: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195C74: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195C78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83195C88 size=88
    let mut pc: u32 = 0x83195C88;
    'dispatch: loop {
        match pc {
            0x83195C88 => {
    //   block [0x83195C88..0x83195CE0)
	// 83195C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83195C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83195C90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83195C94: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83195C98: 4BFFF9B9  bl 0x83195650
	ctx.lr = 0x83195C9C;
	sub_83195650(ctx, base);
	// 83195C9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83195CA0: 409A0014  bne cr6, 0x83195cb4
	if !ctx.cr[6].eq {
	pc = 0x83195CB4; continue 'dispatch;
	}
	// 83195CA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195CB0: 4E800020  blr
	return;
	// 83195CB4: 89630026  lbz r11, 0x26(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(38 as u32) ) } as u64;
	// 83195CB8: 2F0B003F  cmpwi cr6, r11, 0x3f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 63, &mut ctx.xer);
	// 83195CBC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195CC0: 4099000C  ble cr6, 0x83195ccc
	if !ctx.cr[6].gt {
	pc = 0x83195CCC; continue 'dispatch;
	}
	// 83195CC4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83195CC8: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195CCC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195CD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83195CE0 size=72
    let mut pc: u32 = 0x83195CE0;
    'dispatch: loop {
        match pc {
            0x83195CE0 => {
    //   block [0x83195CE0..0x83195D28)
	// 83195CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83195CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83195CE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83195CEC: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83195CF0: 4BFFF961  bl 0x83195650
	ctx.lr = 0x83195CF4;
	sub_83195650(ctx, base);
	// 83195CF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83195CF8: 409A0014  bne cr6, 0x83195d0c
	if !ctx.cr[6].eq {
	pc = 0x83195D0C; continue 'dispatch;
	}
	// 83195CFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195D00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195D04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195D08: 4E800020  blr
	return;
	// 83195D0C: 89630027  lbz r11, 0x27(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(39 as u32) ) } as u64;
	// 83195D10: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195D14: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195D28 size=20
    let mut pc: u32 = 0x83195D28;
    'dispatch: loop {
        match pc {
            0x83195D28 => {
    //   block [0x83195D28..0x83195D3C)
	// 83195D28: 89630020  lbz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83195D2C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83195D30: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83195D34: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 83195D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195D40 size=20
    let mut pc: u32 = 0x83195D40;
    'dispatch: loop {
        match pc {
            0x83195D40 => {
    //   block [0x83195D40..0x83195D54)
	// 83195D40: 3963FF40  addi r11, r3, -0xc0
	ctx.r[11].s64 = ctx.r[3].s64 + -192;
	// 83195D44: 2B0B001F  cmplwi cr6, r11, 0x1f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 31 as u32, &mut ctx.xer);
	// 83195D48: 4199000C  bgt cr6, 0x83195d54
	if ctx.cr[6].gt {
		sub_83195D54(ctx, base);
		return;
	}
	// 83195D4C: 386000C0  li r3, 0xc0
	ctx.r[3].s64 = 192;
	// 83195D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195D54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195D54 size=20
    let mut pc: u32 = 0x83195D54;
    'dispatch: loop {
        match pc {
            0x83195D54 => {
    //   block [0x83195D54..0x83195D68)
	// 83195D54: 3963FF20  addi r11, r3, -0xe0
	ctx.r[11].s64 = ctx.r[3].s64 + -224;
	// 83195D58: 2B0B000F  cmplwi cr6, r11, 0xf
	ctx.cr[6].compare_u32(ctx.r[11].u32, 15 as u32, &mut ctx.xer);
	// 83195D5C: 4199000C  bgt cr6, 0x83195d68
	if ctx.cr[6].gt {
		sub_83195D68(ctx, base);
		return;
	}
	// 83195D60: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 83195D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195D68 size=20
    let mut pc: u32 = 0x83195D68;
    'dispatch: loop {
        match pc {
            0x83195D68 => {
    //   block [0x83195D68..0x83195D7C)
	// 83195D68: 2F0300BD  cmpwi cr6, r3, 0xbd
	ctx.cr[6].compare_i32(ctx.r[3].s32, 189, &mut ctx.xer);
	// 83195D6C: 419A0010  beq cr6, 0x83195d7c
	if ctx.cr[6].eq {
		sub_83195D7C(ctx, base);
		return;
	}
	// 83195D70: 2F0300BF  cmpwi cr6, r3, 0xbf
	ctx.cr[6].compare_i32(ctx.r[3].s32, 191, &mut ctx.xer);
	// 83195D74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83195D78: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195D7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195D7C size=8
    let mut pc: u32 = 0x83195D7C;
    'dispatch: loop {
        match pc {
            0x83195D7C => {
    //   block [0x83195D7C..0x83195D84)
	// 83195D7C: 386000BD  li r3, 0xbd
	ctx.r[3].s64 = 189;
	// 83195D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83195D88 size=64
    let mut pc: u32 = 0x83195D88;
    'dispatch: loop {
        match pc {
            0x83195D88 => {
    //   block [0x83195D88..0x83195DC8)
	// 83195D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83195D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83195D90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83195D94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83195D98: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195D9C: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83195DA0: 4BFFF549  bl 0x831952e8
	ctx.lr = 0x83195DA4;
	sub_831952E8(ctx, base);
	// 83195DA4: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 83195DA8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83195DAC: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83195DB0: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83195DB4: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195DB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83195DC8 size=100
    let mut pc: u32 = 0x83195DC8;
    'dispatch: loop {
        match pc {
            0x83195DC8 => {
    //   block [0x83195DC8..0x83195E2C)
	// 83195DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83195DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83195DD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83195DD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83195DD8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83195DDC: 5483063E  clrlwi r3, r4, 0x18
	ctx.r[3].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83195DE0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195DE4: 4BFFFF5D  bl 0x83195d40
	ctx.lr = 0x83195DE8;
	sub_83195D40(ctx, base);
	// 83195DE8: 2F0300E0  cmpwi cr6, r3, 0xe0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 224, &mut ctx.xer);
	// 83195DEC: 419A0018  beq cr6, 0x83195e04
	if ctx.cr[6].eq {
	pc = 0x83195E04; continue 'dispatch;
	}
	// 83195DF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83195DF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195DF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195DFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195E00: 4E800020  blr
	return;
	// 83195E04: 806A0004  lwz r3, 4(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83195E08: 4BFFF4E1  bl 0x831952e8
	ctx.lr = 0x83195E0C;
	sub_831952E8(ctx, base);
	// 83195E0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83195E10: 419AFFE0  beq cr6, 0x83195df0
	if ctx.cr[6].eq {
	pc = 0x83195DF0; continue 'dispatch;
	}
	// 83195E14: 4BFFFF15  bl 0x83195d28
	ctx.lr = 0x83195E18;
	sub_83195D28(ctx, base);
	// 83195E18: 90650000  stw r3, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83195E1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83195E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83195E30 size=132
    let mut pc: u32 = 0x83195E30;
    'dispatch: loop {
        match pc {
            0x83195E30 => {
    //   block [0x83195E30..0x83195EB4)
	// 83195E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83195E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83195E38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83195E3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83195E40: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83195E44: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83195E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83195E4C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83195E50: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 83195E54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195E58: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195E5C: 4BFFF56D  bl 0x831953c8
	ctx.lr = 0x83195E60;
	sub_831953C8(ctx, base);
	// 83195E60: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83195E64: 409A000C  bne cr6, 0x83195e70
	if !ctx.cr[6].eq {
	pc = 0x83195E70; continue 'dispatch;
	}
	// 83195E68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83195E6C: 48000030  b 0x83195e9c
	pc = 0x83195E9C; continue 'dispatch;
	// 83195E70: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83195E74: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83195E78: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83195E7C: 4800060D  bl 0x83196488
	ctx.lr = 0x83195E80;
	sub_83196488(ctx, base);
	// 83195E80: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83195E84: 419AFFE4  beq cr6, 0x83195e68
	if ctx.cr[6].eq {
	pc = 0x83195E68; continue 'dispatch;
	}
	// 83195E88: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83195E8C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195E90: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83195E94: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195E98: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83195E9C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83195EA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195EA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195EA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83195EAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83195EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195EB8 size=96
    let mut pc: u32 = 0x83195EB8;
    'dispatch: loop {
        match pc {
            0x83195EB8 => {
    //   block [0x83195EB8..0x83195F18)
	// 83195EB8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83195EBC: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 83195EC0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83195EC4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 83195EC8: 394BB608  addi r10, r11, -0x49f8
	ctx.r[10].s64 = ctx.r[11].s64 + -18936;
	// 83195ECC: 91040000  stw r8, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83195ED0: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83195ED4: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 83195ED8: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 83195EDC: 38EB0018  addi r7, r11, 0x18
	ctx.r[7].s64 = ctx.r[11].s64 + 24;
	// 83195EE0: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83195EE4: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83195EE8: 7D064051  subf. r8, r6, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83195EEC: 40820014  bne 0x83195f00
	if !ctx.cr[0].eq {
	pc = 0x83195F00; continue 'dispatch;
	}
	// 83195EF0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83195EF4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83195EF8: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 83195EFC: 409AFFE4  bne cr6, 0x83195ee0
	if !ctx.cr[6].eq {
	pc = 0x83195EE0; continue 'dispatch;
	}
	// 83195F00: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83195F04: 419A0014  beq cr6, 0x83195f18
	if ctx.cr[6].eq {
		sub_83195F18(ctx, base);
		return;
	}
	// 83195F08: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83195F0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83195F10: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195F18 size=16
    let mut pc: u32 = 0x83195F18;
    'dispatch: loop {
        match pc {
            0x83195F18 => {
    //   block [0x83195F18..0x83195F28)
	// 83195F18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83195F1C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195F20: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195F28 size=48
    let mut pc: u32 = 0x83195F28;
    'dispatch: loop {
        match pc {
            0x83195F28 => {
    //   block [0x83195F28..0x83195F58)
	// 83195F28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83195F2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83195F30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83195F34: 396B0180  addi r11, r11, 0x180
	ctx.r[11].s64 = ctx.r[11].s64 + 384;
	// 83195F38: 892B0018  lbz r9, 0x18(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83195F3C: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83195F40: 419A0018  beq cr6, 0x83195f58
	if ctx.cr[6].eq {
		sub_83195F58(ctx, base);
		return;
	}
	// 83195F44: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83195F48: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 83195F4C: 2F0A001A  cmpwi cr6, r10, 0x1a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 26, &mut ctx.xer);
	// 83195F50: 4198FFE8  blt cr6, 0x83195f38
	if ctx.cr[6].lt {
	pc = 0x83195F38; continue 'dispatch;
	}
	// 83195F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195F58 size=8
    let mut pc: u32 = 0x83195F58;
    'dispatch: loop {
        match pc {
            0x83195F58 => {
    //   block [0x83195F58..0x83195F60)
	// 83195F58: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83195F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83195F60 size=104
    let mut pc: u32 = 0x83195F60;
    'dispatch: loop {
        match pc {
            0x83195F60 => {
    //   block [0x83195F60..0x83195FC8)
	// 83195F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83195F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83195F68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83195F6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83195F70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83195F74: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83195F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83195F7C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83195F80: 38A00041  li r5, 0x41
	ctx.r[5].s64 = 65;
	// 83195F84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83195F88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83195F8C: 995F0000  stb r10, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83195F90: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83195F94: 3BCB0060  addi r30, r11, 0x60
	ctx.r[30].s64 = ctx.r[11].s64 + 96;
	// 83195F98: 48012249  bl 0x831a81e0
	ctx.lr = 0x83195F9C;
	sub_831A81E0(ctx, base);
	// 83195F9C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 83195FA0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83195FA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83195FA8: 48012569  bl 0x831a8510
	ctx.lr = 0x83195FAC;
	sub_831A8510(ctx, base);
	// 83195FAC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195FB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83195FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83195FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83195FBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83195FC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83195FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195FC8 size=24
    let mut pc: u32 = 0x83195FC8;
    'dispatch: loop {
        match pc {
            0x83195FC8 => {
    //   block [0x83195FC8..0x83195FE0)
	// 83195FC8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83195FCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83195FD0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83195FD4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195FD8: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83195FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195FE0 size=20
    let mut pc: u32 = 0x83195FE0;
    'dispatch: loop {
        match pc {
            0x83195FE0 => {
    //   block [0x83195FE0..0x83195FF4)
	// 83195FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83195FE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83195FE8: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195FEC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83195FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83195FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83195FF8 size=76
    let mut pc: u32 = 0x83195FF8;
    'dispatch: loop {
        match pc {
            0x83195FF8 => {
    //   block [0x83195FF8..0x83196044)
	// 83195FF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83195FFC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83196000: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196004: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83196008: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8319600C: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 83196010: 7D694670  srawi r9, r11, 8
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 8) as i64;
	// 83196014: 556A063E  clrlwi r10, r11, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83196018: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 8319601C: 7D688670  srawi r8, r11, 0x10
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 16) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 16) as i64;
	// 83196020: 5149442E  rlwimi r9, r10, 8, 0x10, 0x17
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[9].u64 & 0xFFFFFFFFFFFF00FF);
	// 83196024: 7D6BC670  srawi r11, r11, 0x18
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 24) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 24) as i64;
	// 83196028: 550A063E  clrlwi r10, r8, 0x18
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 8319602C: 5529043E  clrlwi r9, r9, 0x10
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 83196030: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83196034: 512A402E  rlwimi r10, r9, 8, 0, 0x17
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[10].u64 & 0xFFFFFFFF000000FF);
	// 83196038: 514B402E  rlwimi r11, r10, 8, 0, 0x17
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[11].u64 & 0xFFFFFFFF000000FF);
	// 8319603C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196048 size=32
    let mut pc: u32 = 0x83196048;
    'dispatch: loop {
        match pc {
            0x83196048 => {
    //   block [0x83196048..0x83196068)
	// 83196048: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8319604C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83196050: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196054: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83196058: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8319605C: 896B0084  lbz r11, 0x84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 83196060: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196068 size=56
    let mut pc: u32 = 0x83196068;
    'dispatch: loop {
        match pc {
            0x83196068 => {
    //   block [0x83196068..0x831960A0)
	// 83196068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8319606C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83196070: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196074: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83196078: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8319607C: A16B0088  lhz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 83196080: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83196084: 556BC23E  srwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83196088: 554A442E  rlwinm r10, r10, 8, 0x10, 0x17
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 8319608C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83196090: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 83196094: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 83196098: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8319609C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831960A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831960A0 size=76
    let mut pc: u32 = 0x831960A0;
    'dispatch: loop {
        match pc {
            0x831960A0 => {
    //   block [0x831960A0..0x831960EC)
	// 831960A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831960A4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831960A8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831960AC: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831960B0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831960B4: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 831960B8: 7D694670  srawi r9, r11, 8
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 8) as i64;
	// 831960BC: 556A063E  clrlwi r10, r11, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 831960C0: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 831960C4: 7D688670  srawi r8, r11, 0x10
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 16) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 16) as i64;
	// 831960C8: 5149442E  rlwimi r9, r10, 8, 0x10, 0x17
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[9].u64 & 0xFFFFFFFFFFFF00FF);
	// 831960CC: 7D6BC670  srawi r11, r11, 0x18
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 24) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 24) as i64;
	// 831960D0: 550A063E  clrlwi r10, r8, 0x18
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 831960D4: 5529043E  clrlwi r9, r9, 0x10
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 831960D8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 831960DC: 512A402E  rlwimi r10, r9, 8, 0, 0x17
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[10].u64 & 0xFFFFFFFF000000FF);
	// 831960E0: 514B402E  rlwimi r11, r10, 8, 0, 0x17
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[11].u64 & 0xFFFFFFFF000000FF);
	// 831960E4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831960E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831960F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831960F0 size=96
    let mut pc: u32 = 0x831960F0;
    'dispatch: loop {
        match pc {
            0x831960F0 => {
    //   block [0x831960F0..0x83196150)
	// 831960F0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831960F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831960F8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 831960FC: 2F0B002E  cmpwi cr6, r11, 0x2e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 46, &mut ctx.xer);
	// 83196100: 419A0048  beq cr6, 0x83196148
	if ctx.cr[6].eq {
	pc = 0x83196148; continue 'dispatch;
	}
	// 83196104: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 83196108: 419A0040  beq cr6, 0x83196148
	if ctx.cr[6].eq {
	pc = 0x83196148; continue 'dispatch;
	}
	// 8319610C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83196110: 419A0038  beq cr6, 0x83196148
	if ctx.cr[6].eq {
	pc = 0x83196148; continue 'dispatch;
	}
	// 83196114: 392BFFD0  addi r9, r11, -0x30
	ctx.r[9].s64 = ctx.r[11].s64 + -48;
	// 83196118: 2B090009  cmplwi cr6, r9, 9
	ctx.cr[6].compare_u32(ctx.r[9].u32, 9 as u32, &mut ctx.xer);
	// 8319611C: 4199002C  bgt cr6, 0x83196148
	if ctx.cr[6].gt {
	pc = 0x83196148; continue 'dispatch;
	}
	// 83196120: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83196124: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 83196128: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8319612C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83196130: 89030000  lbz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83196134: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83196138: 7D0B0774  extsb r11, r8
	ctx.r[11].s64 = ctx.r[8].s8 as i64;
	// 8319613C: 394AFFD0  addi r10, r10, -0x30
	ctx.r[10].s64 = ctx.r[10].s64 + -48;
	// 83196140: 2F0B002E  cmpwi cr6, r11, 0x2e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 46, &mut ctx.xer);
	// 83196144: 409AFFC0  bne cr6, 0x83196104
	if !ctx.cr[6].eq {
	pc = 0x83196104; continue 'dispatch;
	}
	// 83196148: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8319614C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196150 size=20
    let mut pc: u32 = 0x83196150;
    'dispatch: loop {
        match pc {
            0x83196150 => {
    //   block [0x83196150..0x83196164)
	// 83196150: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83196154: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196158: 896B00B0  lbz r11, 0xb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 8319615C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196168 size=20
    let mut pc: u32 = 0x83196168;
    'dispatch: loop {
        match pc {
            0x83196168 => {
    //   block [0x83196168..0x8319617C)
	// 83196168: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8319616C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196170: 896B00B1  lbz r11, 0xb1(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(177 as u32) ) } as u64;
	// 83196174: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196180 size=20
    let mut pc: u32 = 0x83196180;
    'dispatch: loop {
        match pc {
            0x83196180 => {
    //   block [0x83196180..0x83196194)
	// 83196180: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83196184: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196188: 896B00B2  lbz r11, 0xb2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(178 as u32) ) } as u64;
	// 8319618C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196198 size=20
    let mut pc: u32 = 0x83196198;
    'dispatch: loop {
        match pc {
            0x83196198 => {
    //   block [0x83196198..0x831961AC)
	// 83196198: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8319619C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831961A0: 896B00B3  lbz r11, 0xb3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(179 as u32) ) } as u64;
	// 831961A4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831961A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831961B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831961B0 size=20
    let mut pc: u32 = 0x831961B0;
    'dispatch: loop {
        match pc {
            0x831961B0 => {
    //   block [0x831961B0..0x831961C4)
	// 831961B0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 831961B4: 2F0B006E  cmpwi cr6, r11, 0x6e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 110, &mut ctx.xer);
	// 831961B8: 4098000C  bge cr6, 0x831961c4
	if !ctx.cr[6].lt {
		sub_831961C4(ctx, base);
		return;
	}
	// 831961BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831961C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831961C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831961C4 size=64
    let mut pc: u32 = 0x831961C4;
    'dispatch: loop {
        match pc {
            0x831961C4 => {
    //   block [0x831961C4..0x83196204)
	// 831961C4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831961C8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831961CC: 816B00B4  lwz r11, 0xb4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(180 as u32) ) } as u64;
	// 831961D0: 7D694670  srawi r9, r11, 8
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 8) as i64;
	// 831961D4: 556A063E  clrlwi r10, r11, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 831961D8: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 831961DC: 7D688670  srawi r8, r11, 0x10
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 16) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 16) as i64;
	// 831961E0: 5149442E  rlwimi r9, r10, 8, 0x10, 0x17
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[9].u64 & 0xFFFFFFFFFFFF00FF);
	// 831961E4: 7D6BC670  srawi r11, r11, 0x18
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 24) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 24) as i64;
	// 831961E8: 550A063E  clrlwi r10, r8, 0x18
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 831961EC: 5529043E  clrlwi r9, r9, 0x10
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 831961F0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 831961F4: 512A402E  rlwimi r10, r9, 8, 0, 0x17
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[10].u64 & 0xFFFFFFFF000000FF);
	// 831961F8: 514B402E  rlwimi r11, r10, 8, 0, 0x17
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[11].u64 & 0xFFFFFFFF000000FF);
	// 831961FC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196208 size=64
    let mut pc: u32 = 0x83196208;
    'dispatch: loop {
        match pc {
            0x83196208 => {
    //   block [0x83196208..0x83196248)
	// 83196208: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8319620C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196210: 816B00B8  lwz r11, 0xb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(184 as u32) ) } as u64;
	// 83196214: 7D694670  srawi r9, r11, 8
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 8) as i64;
	// 83196218: 556A063E  clrlwi r10, r11, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8319621C: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 83196220: 7D688670  srawi r8, r11, 0x10
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 16) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 16) as i64;
	// 83196224: 5149442E  rlwimi r9, r10, 8, 0x10, 0x17
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[9].u64 & 0xFFFFFFFFFFFF00FF);
	// 83196228: 7D6BC670  srawi r11, r11, 0x18
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 24) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 24) as i64;
	// 8319622C: 550A063E  clrlwi r10, r8, 0x18
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 83196230: 5529043E  clrlwi r9, r9, 0x10
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 83196234: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83196238: 512A402E  rlwimi r10, r9, 8, 0, 0x17
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[10].u64 & 0xFFFFFFFF000000FF);
	// 8319623C: 514B402E  rlwimi r11, r10, 8, 0, 0x17
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[11].u64 & 0xFFFFFFFF000000FF);
	// 83196240: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196248 size=64
    let mut pc: u32 = 0x83196248;
    'dispatch: loop {
        match pc {
            0x83196248 => {
    //   block [0x83196248..0x83196288)
	// 83196248: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8319624C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196250: 816B00BC  lwz r11, 0xbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(188 as u32) ) } as u64;
	// 83196254: 7D694670  srawi r9, r11, 8
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 8) as i64;
	// 83196258: 556A063E  clrlwi r10, r11, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8319625C: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 83196260: 7D688670  srawi r8, r11, 0x10
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 16) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 16) as i64;
	// 83196264: 5149442E  rlwimi r9, r10, 8, 0x10, 0x17
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[9].u64 & 0xFFFFFFFFFFFF00FF);
	// 83196268: 7D6BC670  srawi r11, r11, 0x18
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 24) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 24) as i64;
	// 8319626C: 550A063E  clrlwi r10, r8, 0x18
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 83196270: 5529043E  clrlwi r9, r9, 0x10
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 83196274: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83196278: 512A402E  rlwimi r10, r9, 8, 0, 0x17
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[10].u64 & 0xFFFFFFFF000000FF);
	// 8319627C: 514B402E  rlwimi r11, r10, 8, 0, 0x17
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[11].u64 & 0xFFFFFFFF000000FF);
	// 83196280: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196288 size=64
    let mut pc: u32 = 0x83196288;
    'dispatch: loop {
        match pc {
            0x83196288 => {
    //   block [0x83196288..0x831962C8)
	// 83196288: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8319628C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196290: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 83196294: 7D694670  srawi r9, r11, 8
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 8) as i64;
	// 83196298: 556A063E  clrlwi r10, r11, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8319629C: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 831962A0: 7D688670  srawi r8, r11, 0x10
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 16) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 16) as i64;
	// 831962A4: 5149442E  rlwimi r9, r10, 8, 0x10, 0x17
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[9].u64 & 0xFFFFFFFFFFFF00FF);
	// 831962A8: 7D6BC670  srawi r11, r11, 0x18
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 24) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 24) as i64;
	// 831962AC: 550A063E  clrlwi r10, r8, 0x18
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 831962B0: 5529043E  clrlwi r9, r9, 0x10
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 831962B4: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 831962B8: 512A402E  rlwimi r10, r9, 8, 0, 0x17
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[10].u64 & 0xFFFFFFFF000000FF);
	// 831962BC: 514B402E  rlwimi r11, r10, 8, 0, 0x17
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[11].u64 & 0xFFFFFFFF000000FF);
	// 831962C0: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831962C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831962C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831962C8 size=20
    let mut pc: u32 = 0x831962C8;
    'dispatch: loop {
        match pc {
            0x831962C8 => {
    //   block [0x831962C8..0x831962DC)
	// 831962C8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 831962CC: 2F0B00E1  cmpwi cr6, r11, 0xe1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 225, &mut ctx.xer);
	// 831962D0: 4098000C  bge cr6, 0x831962dc
	if !ctx.cr[6].lt {
		sub_831962DC(ctx, base);
		return;
	}
	// 831962D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831962D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831962DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831962DC size=64
    let mut pc: u32 = 0x831962DC;
    'dispatch: loop {
        match pc {
            0x831962DC => {
    //   block [0x831962DC..0x8319631C)
	// 831962DC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831962E0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831962E4: 816B00C4  lwz r11, 0xc4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(196 as u32) ) } as u64;
	// 831962E8: 7D694670  srawi r9, r11, 8
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 8) as i64;
	// 831962EC: 556A063E  clrlwi r10, r11, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 831962F0: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 831962F4: 7D688670  srawi r8, r11, 0x10
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 16) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 16) as i64;
	// 831962F8: 5149442E  rlwimi r9, r10, 8, 0x10, 0x17
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[9].u64 & 0xFFFFFFFFFFFF00FF);
	// 831962FC: 7D6BC670  srawi r11, r11, 0x18
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 24) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 24) as i64;
	// 83196300: 550A063E  clrlwi r10, r8, 0x18
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 83196304: 5529043E  clrlwi r9, r9, 0x10
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 83196308: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8319630C: 512A402E  rlwimi r10, r9, 8, 0, 0x17
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[10].u64 & 0xFFFFFFFF000000FF);
	// 83196310: 514B402E  rlwimi r11, r10, 8, 0, 0x17
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[11].u64 & 0xFFFFFFFF000000FF);
	// 83196314: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196320 size=20
    let mut pc: u32 = 0x83196320;
    'dispatch: loop {
        match pc {
            0x83196320 => {
    //   block [0x83196320..0x83196334)
	// 83196320: 89630020  lbz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83196324: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83196328: 4099000C  ble cr6, 0x83196334
	if !ctx.cr[6].gt {
		sub_83196334(ctx, base);
		return;
	}
	// 8319632C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83196330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196334(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196334 size=16
    let mut pc: u32 = 0x83196334;
    'dispatch: loop {
        match pc {
            0x83196334 => {
    //   block [0x83196334..0x83196344)
	// 83196334: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83196338: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8319633C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 83196340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196348 size=20
    let mut pc: u32 = 0x83196348;
    'dispatch: loop {
        match pc {
            0x83196348 => {
    //   block [0x83196348..0x8319635C)
	// 83196348: 3963FF40  addi r11, r3, -0xc0
	ctx.r[11].s64 = ctx.r[3].s64 + -192;
	// 8319634C: 2B0B001F  cmplwi cr6, r11, 0x1f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 31 as u32, &mut ctx.xer);
	// 83196350: 4199000C  bgt cr6, 0x8319635c
	if ctx.cr[6].gt {
		sub_8319635C(ctx, base);
		return;
	}
	// 83196354: 386000C0  li r3, 0xc0
	ctx.r[3].s64 = 192;
	// 83196358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8319635C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8319635C size=20
    let mut pc: u32 = 0x8319635C;
    'dispatch: loop {
        match pc {
            0x8319635C => {
    //   block [0x8319635C..0x83196370)
	// 8319635C: 3963FF20  addi r11, r3, -0xe0
	ctx.r[11].s64 = ctx.r[3].s64 + -224;
	// 83196360: 2B0B000F  cmplwi cr6, r11, 0xf
	ctx.cr[6].compare_u32(ctx.r[11].u32, 15 as u32, &mut ctx.xer);
	// 83196364: 4199000C  bgt cr6, 0x83196370
	if ctx.cr[6].gt {
		sub_83196370(ctx, base);
		return;
	}
	// 83196368: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 8319636C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196370 size=20
    let mut pc: u32 = 0x83196370;
    'dispatch: loop {
        match pc {
            0x83196370 => {
    //   block [0x83196370..0x83196384)
	// 83196370: 2B0300BD  cmplwi cr6, r3, 0xbd
	ctx.cr[6].compare_u32(ctx.r[3].u32, 189 as u32, &mut ctx.xer);
	// 83196374: 419A0010  beq cr6, 0x83196384
	if ctx.cr[6].eq {
		sub_83196384(ctx, base);
		return;
	}
	// 83196378: 2B0300BF  cmplwi cr6, r3, 0xbf
	ctx.cr[6].compare_u32(ctx.r[3].u32, 191 as u32, &mut ctx.xer);
	// 8319637C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83196380: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196384(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196384 size=8
    let mut pc: u32 = 0x83196384;
    'dispatch: loop {
        match pc {
            0x83196384 => {
    //   block [0x83196384..0x8319638C)
	// 83196384: 386000BD  li r3, 0xbd
	ctx.r[3].s64 = 189;
	// 83196388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83196390 size=24
    let mut pc: u32 = 0x83196390;
    'dispatch: loop {
        match pc {
            0x83196390 => {
    //   block [0x83196390..0x831963A8)
	// 83196390: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83196394: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83196398: 216B0002  subfic r11, r11, 2
	ctx.xer.ca = ctx.r[11].u32 <= 2 as u32;
	ctx.r[11].s64 = (2 as i64) - ctx.r[11].s64;
	// 8319639C: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 831963A0: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 831963A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831963A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831963A8 size=36
    let mut pc: u32 = 0x831963A8;
    'dispatch: loop {
        match pc {
            0x831963A8 => {
    //   block [0x831963A8..0x831963CC)
	// 831963A8: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 831963AC: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 831963B0: 41990088  bgt cr6, 0x83196438
	if ctx.cr[6].gt {
		sub_83196438(ctx, base);
		return;
	}
	// 831963B4: 3D808319  lis r12, -0x7ce7
	ctx.r[12].s64 = -2095513600;
	// 831963B8: 398C63CC  addi r12, r12, 0x63cc
	ctx.r[12].s64 = ctx.r[12].s64 + 25548;
	// 831963BC: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 831963C0: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 831963C4: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831963C8: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
			// ERROR: 0x831963EC
			return;
		},
		1 => {
			// ERROR: 0x831963F4
			return;
		},
		2 => {
			// ERROR: 0x831963FC
			return;
		},
		3 => {
			// ERROR: 0x83196404
			return;
		},
		4 => {
			// ERROR: 0x8319640C
			return;
		},
		5 => {
			// ERROR: 0x83196414
			return;
		},
		6 => {
			// ERROR: 0x83196420
			return;
		},
		7 => {
			// ERROR: 0x8319642C
			return;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831963CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831963CC size=40
    let mut pc: u32 = 0x831963CC;
    'dispatch: loop {
        match pc {
            0x831963CC => {
    //   block [0x831963CC..0x831963F4)
	// 831963CC: 831963EC  lwz r24, 0x63ec(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(25580 as u32) ) } as u64;
	// 831963D0: 831963F4  lwz r24, 0x63f4(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(25588 as u32) ) } as u64;
	// 831963D4: 831963FC  lwz r24, 0x63fc(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(25596 as u32) ) } as u64;
	// 831963D8: 83196404  lwz r24, 0x6404(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(25604 as u32) ) } as u64;
	// 831963DC: 8319640C  lwz r24, 0x640c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(25612 as u32) ) } as u64;
	// 831963E0: 83196414  lwz r24, 0x6414(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(25620 as u32) ) } as u64;
	// 831963E4: 83196420  lwz r24, 0x6420(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(25632 as u32) ) } as u64;
	// 831963E8: 8319642C  lwz r24, 0x642c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(25644 as u32) ) } as u64;
	// 831963EC: 38605DA8  li r3, 0x5da8
	ctx.r[3].s64 = 23976;
	// 831963F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831963F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831963F4 size=8
    let mut pc: u32 = 0x831963F4;
    'dispatch: loop {
        match pc {
            0x831963F4 => {
    //   block [0x831963F4..0x831963FC)
	// 831963F4: 38605DC0  li r3, 0x5dc0
	ctx.r[3].s64 = 24000;
	// 831963F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831963FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831963FC size=8
    let mut pc: u32 = 0x831963FC;
    'dispatch: loop {
        match pc {
            0x831963FC => {
    //   block [0x831963FC..0x83196404)
	// 831963FC: 386061A8  li r3, 0x61a8
	ctx.r[3].s64 = 25000;
	// 83196400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196404(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196404 size=8
    let mut pc: u32 = 0x83196404;
    'dispatch: loop {
        match pc {
            0x83196404 => {
    //   block [0x83196404..0x8319640C)
	// 83196404: 38607512  li r3, 0x7512
	ctx.r[3].s64 = 29970;
	// 83196408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8319640C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8319640C size=8
    let mut pc: u32 = 0x8319640C;
    'dispatch: loop {
        match pc {
            0x8319640C => {
    //   block [0x8319640C..0x83196414)
	// 8319640C: 38607530  li r3, 0x7530
	ctx.r[3].s64 = 30000;
	// 83196410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196414(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196414 size=12
    let mut pc: u32 = 0x83196414;
    'dispatch: loop {
        match pc {
            0x83196414 => {
    //   block [0x83196414..0x83196420)
	// 83196414: 3C600000  lis r3, 0
	ctx.r[3].s64 = 0;
	// 83196418: 6063C350  ori r3, r3, 0xc350
	ctx.r[3].u64 = ctx.r[3].u64 | 50000;
	// 8319641C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196420 size=12
    let mut pc: u32 = 0x83196420;
    'dispatch: loop {
        match pc {
            0x83196420 => {
    //   block [0x83196420..0x8319642C)
	// 83196420: 3C600000  lis r3, 0
	ctx.r[3].s64 = 0;
	// 83196424: 6063EA24  ori r3, r3, 0xea24
	ctx.r[3].u64 = ctx.r[3].u64 | 59940;
	// 83196428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8319642C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8319642C size=12
    let mut pc: u32 = 0x8319642C;
    'dispatch: loop {
        match pc {
            0x8319642C => {
    //   block [0x8319642C..0x83196438)
	// 8319642C: 3C600000  lis r3, 0
	ctx.r[3].s64 = 0;
	// 83196430: 6063EA60  ori r3, r3, 0xea60
	ctx.r[3].u64 = ctx.r[3].u64 | 60000;
	// 83196434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196438 size=8
    let mut pc: u32 = 0x83196438;
    'dispatch: loop {
        match pc {
            0x83196438 => {
    //   block [0x83196438..0x83196440)
	// 83196438: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8319643C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83196440 size=68
    let mut pc: u32 = 0x83196440;
    'dispatch: loop {
        match pc {
            0x83196440 => {
    //   block [0x83196440..0x83196484)
	// 83196440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83196444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83196448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319644C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83196450: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83196454: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196458: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8319645C: 4BFFFACD  bl 0x83195f28
	ctx.lr = 0x83196460;
	sub_83195F28(ctx, base);
	// 83196460: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 83196464: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83196468: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8319646C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83196470: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196474: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319647C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83196488 size=100
    let mut pc: u32 = 0x83196488;
    'dispatch: loop {
        match pc {
            0x83196488 => {
    //   block [0x83196488..0x831964EC)
	// 83196488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319648C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83196490: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83196494: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83196498: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319649C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831964A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831964A4: 388BBE78  addi r4, r11, -0x4188
	ctx.r[4].s64 = ctx.r[11].s64 + -16776;
	// 831964A8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 831964AC: 48016435  bl 0x831ac8e0
	ctx.lr = 0x831964B0;
	sub_831AC8E0(ctx, base);
	// 831964B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831964B4: 419A0020  beq cr6, 0x831964d4
	if ctx.cr[6].eq {
	pc = 0x831964D4; continue 'dispatch;
	}
	// 831964B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831964BC: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 831964C0: 4BFFFC31  bl 0x831960f0
	ctx.lr = 0x831964C4;
	sub_831960F0(ctx, base);
	// 831964C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831964C8: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 831964CC: 4BFFFC25  bl 0x831960f0
	ctx.lr = 0x831964D0;
	sub_831960F0(ctx, base);
	// 831964D0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831964D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831964D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831964DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831964E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831964E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831964E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831964F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831964F0 size=84
    let mut pc: u32 = 0x831964F0;
    'dispatch: loop {
        match pc {
            0x831964F0 => {
    //   block [0x831964F0..0x83196544)
	// 831964F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831964F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831964F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831964FC: 4BFFFE4D  bl 0x83196348
	ctx.lr = 0x83196500;
	sub_83196348(ctx, base);
	// 83196500: 2B0300C0  cmplwi cr6, r3, 0xc0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 192 as u32, &mut ctx.xer);
	// 83196504: 419A0018  beq cr6, 0x8319651c
	if ctx.cr[6].eq {
	pc = 0x8319651C; continue 'dispatch;
	}
	// 83196508: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8319650C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196518: 4E800020  blr
	return;
	// 8319651C: 89640020  lbz r11, 0x20(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 83196520: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83196524: 4199FFE4  bgt cr6, 0x83196508
	if ctx.cr[6].gt {
	pc = 0x83196508; continue 'dispatch;
	}
	// 83196528: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8319652C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83196530: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 83196534: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319653C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83196548 size=88
    let mut pc: u32 = 0x83196548;
    'dispatch: loop {
        match pc {
            0x83196548 => {
    //   block [0x83196548..0x831965A0)
	// 83196548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319654C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83196550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83196554: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83196558: 4BFFFE39  bl 0x83196390
	ctx.lr = 0x8319655C;
	sub_83196390(ctx, base);
	// 8319655C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83196560: 409A0014  bne cr6, 0x83196574
	if !ctx.cr[6].eq {
	pc = 0x83196574; continue 'dispatch;
	}
	// 83196564: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319656C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196570: 4E800020  blr
	return;
	// 83196574: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 83196578: 2F0B006B  cmpwi cr6, r11, 0x6b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 107, &mut ctx.xer);
	// 8319657C: 419A0010  beq cr6, 0x8319658c
	if ctx.cr[6].eq {
	pc = 0x8319658C; continue 'dispatch;
	}
	// 83196580: 2F0B006E  cmpwi cr6, r11, 0x6e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 110, &mut ctx.xer);
	// 83196584: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83196588: 41980008  blt cr6, 0x83196590
	if ctx.cr[6].lt {
	pc = 0x83196590; continue 'dispatch;
	}
	// 8319658C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319659C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831965A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831965A0 size=168
    let mut pc: u32 = 0x831965A0;
    'dispatch: loop {
        match pc {
            0x831965A0 => {
    //   block [0x831965A0..0x83196648)
	// 831965A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831965A4: 48011BC5  bl 0x831a8168
	ctx.lr = 0x831965A8;
	sub_831A8130(ctx, base);
	// 831965A8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831965AC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831965B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831965B4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 831965B8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 831965BC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831965C0: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831965C4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831965C8: 8BEB0038  lbz r31, 0x38(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 831965CC: 8BCB0039  lbz r30, 0x39(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(57 as u32) ) } as u64;
	// 831965D0: 4BFFF991  bl 0x83195f60
	ctx.lr = 0x831965D4;
	sub_83195F60(ctx, base);
	// 831965D4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831965D8: 409A0010  bne cr6, 0x831965e8
	if !ctx.cr[6].eq {
	pc = 0x831965E8; continue 'dispatch;
	}
	// 831965DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831965E0: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 831965E4: 48011BD4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 831965E8: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 831965EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831965F0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 831965F4: 4BFFFE95  bl 0x83196488
	ctx.lr = 0x831965F8;
	sub_83196488(ctx, base);
	// 831965F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831965FC: 419AFFE0  beq cr6, 0x831965dc
	if ctx.cr[6].eq {
	pc = 0x831965DC; continue 'dispatch;
	}
	// 83196600: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83196604: 1D7F0064  mulli r11, r31, 0x64
	ctx.r[11].s64 = ctx.r[31].s64 * 100;
	// 83196608: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8319660C: 1D490064  mulli r10, r9, 0x64
	ctx.r[10].s64 = ctx.r[9].s64 * 100;
	// 83196610: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 83196614: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83196618: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8319661C: 41980018  blt cr6, 0x83196634
	if ctx.cr[6].lt {
	pc = 0x83196634; continue 'dispatch;
	}
	// 83196620: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83196624: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196628: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8319662C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 83196630: 48011B88  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83196634: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83196638: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8319663C: 911C0000  stw r8, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83196640: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 83196644: 48011B74  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83196648 size=68
    let mut pc: u32 = 0x83196648;
    'dispatch: loop {
        match pc {
            0x83196648 => {
    //   block [0x83196648..0x8319668C)
	// 83196648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319664C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83196650: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83196654: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 83196658: 4BFFFEF1  bl 0x83196548
	ctx.lr = 0x8319665C;
	sub_83196548(ctx, base);
	// 8319665C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83196660: 409A0014  bne cr6, 0x83196674
	if !ctx.cr[6].eq {
	pc = 0x83196674; continue 'dispatch;
	}
	// 83196664: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319666C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196670: 4E800020  blr
	return;
	// 83196674: 80690004  lwz r3, 4(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83196678: 4BFFF8B1  bl 0x83195f28
	ctx.lr = 0x8319667C;
	sub_83195F28(ctx, base);
	// 8319667C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83196690 size=168
    let mut pc: u32 = 0x83196690;
    'dispatch: loop {
        match pc {
            0x83196690 => {
    //   block [0x83196690..0x831966F4)
	// 83196690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83196694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83196698: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319669C: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 831966A0: 4BFFFFA9  bl 0x83196648
	ctx.lr = 0x831966A4;
	sub_83196648(ctx, base);
	// 831966A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831966A8: 409A0014  bne cr6, 0x831966bc
	if !ctx.cr[6].eq {
	pc = 0x831966BC; continue 'dispatch;
	}
	// 831966AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831966B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831966B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831966B8: 4E800020  blr
	return;
	// 831966BC: 89630019  lbz r11, 0x19(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(25 as u32) ) } as u64;
	// 831966C0: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 831966C4: 41990058  bgt cr6, 0x8319671c
	if ctx.cr[6].gt {
	pc = 0x8319671C; continue 'dispatch;
	}
	// 831966C8: 3D808319  lis r12, -0x7ce7
	ctx.r[12].s64 = -2095513600;
	// 831966CC: 398C66E0  addi r12, r12, 0x66e0
	ctx.r[12].s64 = ctx.r[12].s64 + 26336;
	// 831966D0: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 831966D4: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 831966D8: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831966DC: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x831966F4; continue 'dispatch;
		},
		1 => {
	pc = 0x831966FC; continue 'dispatch;
		},
		2 => {
	pc = 0x83196704; continue 'dispatch;
		},
		3 => {
	pc = 0x8319670C; continue 'dispatch;
		},
		4 => {
	pc = 0x83196714; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 831966E0: 831966F4  lwz r24, 0x66f4(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(26356 as u32) ) } as u64;
	// 831966E4: 831966FC  lwz r24, 0x66fc(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(26364 as u32) ) } as u64;
	// 831966E8: 83196704  lwz r24, 0x6704(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(26372 as u32) ) } as u64;
	// 831966EC: 8319670C  lwz r24, 0x670c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(26380 as u32) ) } as u64;
	// 831966F0: 83196714  lwz r24, 0x6714(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(26388 as u32) ) } as u64;
            }
            0x831966F4 => {
    //   block [0x831966F4..0x831966FC)
	// 831966F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831966F8: 48000028  b 0x83196720
	pc = 0x83196720; continue 'dispatch;
            }
            0x831966FC => {
    //   block [0x831966FC..0x83196704)
	// 831966FC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83196700: 48000020  b 0x83196720
	pc = 0x83196720; continue 'dispatch;
            }
            0x83196704 => {
    //   block [0x83196704..0x8319670C)
	// 83196704: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 83196708: 48000018  b 0x83196720
	pc = 0x83196720; continue 'dispatch;
            }
            0x8319670C => {
    //   block [0x8319670C..0x83196714)
	// 8319670C: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 83196710: 48000010  b 0x83196720
	pc = 0x83196720; continue 'dispatch;
            }
            0x83196714 => {
    //   block [0x83196714..0x83196738)
	// 83196714: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 83196718: 48000008  b 0x83196720
	pc = 0x83196720; continue 'dispatch;
	// 8319671C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83196720: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196724: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196728: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8319672C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83196738 size=88
    let mut pc: u32 = 0x83196738;
    'dispatch: loop {
        match pc {
            0x83196738 => {
    //   block [0x83196738..0x83196790)
	// 83196738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319673C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83196740: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83196744: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83196748: 4BFFFF01  bl 0x83196648
	ctx.lr = 0x8319674C;
	sub_83196648(ctx, base);
	// 8319674C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83196750: 409A0018  bne cr6, 0x83196768
	if !ctx.cr[6].eq {
	pc = 0x83196768; continue 'dispatch;
	}
	// 83196754: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83196758: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8319675C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196764: 4E800020  blr
	return;
	// 83196768: 89630019  lbz r11, 0x19(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(25 as u32) ) } as u64;
	// 8319676C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 83196770: 409AFFE4  bne cr6, 0x83196754
	if !ctx.cr[6].eq {
	pc = 0x83196754; continue 'dispatch;
	}
	// 83196774: 8963001A  lbz r11, 0x1a(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(26 as u32) ) } as u64;
	// 83196778: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8319677C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196780: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319678C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83196790 size=72
    let mut pc: u32 = 0x83196790;
    'dispatch: loop {
        match pc {
            0x83196790 => {
    //   block [0x83196790..0x831967D8)
	// 83196790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83196794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83196798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319679C: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 831967A0: 4BFFFEA9  bl 0x83196648
	ctx.lr = 0x831967A4;
	sub_83196648(ctx, base);
	// 831967A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831967A8: 409A0014  bne cr6, 0x831967bc
	if !ctx.cr[6].eq {
	pc = 0x831967BC; continue 'dispatch;
	}
	// 831967AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831967B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831967B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831967B8: 4E800020  blr
	return;
	// 831967BC: 8963001B  lbz r11, 0x1b(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(27 as u32) ) } as u64;
	// 831967C0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831967C4: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831967C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831967CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831967D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831967D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831967D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831967D8 size=116
    let mut pc: u32 = 0x831967D8;
    'dispatch: loop {
        match pc {
            0x831967D8 => {
    //   block [0x831967D8..0x8319684C)
	// 831967D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831967DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831967E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831967E4: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 831967E8: 4BFFFE61  bl 0x83196648
	ctx.lr = 0x831967EC;
	sub_83196648(ctx, base);
	// 831967EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831967F0: 409A0014  bne cr6, 0x83196804
	if !ctx.cr[6].eq {
	pc = 0x83196804; continue 'dispatch;
	}
	// 831967F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831967F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831967FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196800: 4E800020  blr
	return;
	// 83196804: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 83196808: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8319680C: 7D694670  srawi r9, r11, 8
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 8) as i64;
	// 83196810: 556A063E  clrlwi r10, r11, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83196814: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 83196818: 7D688670  srawi r8, r11, 0x10
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 16) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 16) as i64;
	// 8319681C: 5149442E  rlwimi r9, r10, 8, 0x10, 0x17
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[9].u64 & 0xFFFFFFFFFFFF00FF);
	// 83196820: 7D6BC670  srawi r11, r11, 0x18
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 24) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 24) as i64;
	// 83196824: 550A063E  clrlwi r10, r8, 0x18
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 83196828: 5529043E  clrlwi r9, r9, 0x10
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 8319682C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83196830: 512A402E  rlwimi r10, r9, 8, 0, 0x17
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[10].u64 & 0xFFFFFFFF000000FF);
	// 83196834: 514B402E  rlwimi r11, r10, 8, 0, 0x17
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[11].u64 & 0xFFFFFFFF000000FF);
	// 83196838: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8319683C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83196850 size=172
    let mut pc: u32 = 0x83196850;
    'dispatch: loop {
        match pc {
            0x83196850 => {
    //   block [0x83196850..0x831968B8)
	// 83196850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83196854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83196858: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319685C: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83196860: 4BFFFDE9  bl 0x83196648
	ctx.lr = 0x83196864;
	sub_83196648(ctx, base);
	// 83196864: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83196868: 409A0014  bne cr6, 0x8319687c
	if !ctx.cr[6].eq {
	pc = 0x8319687C; continue 'dispatch;
	}
	// 8319686C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196878: 4E800020  blr
	return;
	// 8319687C: 89630019  lbz r11, 0x19(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(25 as u32) ) } as u64;
	// 83196880: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 83196884: 4199005C  bgt cr6, 0x831968e0
	if ctx.cr[6].gt {
	pc = 0x831968E0; continue 'dispatch;
	}
	// 83196888: 3D808319  lis r12, -0x7ce7
	ctx.r[12].s64 = -2095513600;
	// 8319688C: 398C68A0  addi r12, r12, 0x68a0
	ctx.r[12].s64 = ctx.r[12].s64 + 26784;
	// 83196890: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 83196894: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83196898: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8319689C: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x831968B8; continue 'dispatch;
		},
		1 => {
	pc = 0x831968C0; continue 'dispatch;
		},
		2 => {
	pc = 0x831968C0; continue 'dispatch;
		},
		3 => {
	pc = 0x831968C8; continue 'dispatch;
		},
		4 => {
	pc = 0x831968D0; continue 'dispatch;
		},
		5 => {
	pc = 0x831968D8; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 831968A0: 831968B8  lwz r24, 0x68b8(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(26808 as u32) ) } as u64;
	// 831968A4: 831968C0  lwz r24, 0x68c0(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(26816 as u32) ) } as u64;
	// 831968A8: 831968C0  lwz r24, 0x68c0(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(26816 as u32) ) } as u64;
	// 831968AC: 831968C8  lwz r24, 0x68c8(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(26824 as u32) ) } as u64;
	// 831968B0: 831968D0  lwz r24, 0x68d0(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(26832 as u32) ) } as u64;
	// 831968B4: 831968D8  lwz r24, 0x68d8(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(26840 as u32) ) } as u64;
            }
            0x831968B8 => {
    //   block [0x831968B8..0x831968C0)
	// 831968B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831968BC: 48000028  b 0x831968e4
	pc = 0x831968E4; continue 'dispatch;
            }
            0x831968C0 => {
    //   block [0x831968C0..0x831968C8)
	// 831968C0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 831968C4: 48000020  b 0x831968e4
	pc = 0x831968E4; continue 'dispatch;
            }
            0x831968C8 => {
    //   block [0x831968C8..0x831968D0)
	// 831968C8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 831968CC: 48000018  b 0x831968e4
	pc = 0x831968E4; continue 'dispatch;
            }
            0x831968D0 => {
    //   block [0x831968D0..0x831968D8)
	// 831968D0: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 831968D4: 48000010  b 0x831968e4
	pc = 0x831968E4; continue 'dispatch;
            }
            0x831968D8 => {
    //   block [0x831968D8..0x831968FC)
	// 831968D8: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 831968DC: 48000008  b 0x831968e4
	pc = 0x831968E4; continue 'dispatch;
	// 831968E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831968E4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831968E8: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831968EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831968F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831968F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831968F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83196900 size=108
    let mut pc: u32 = 0x83196900;
    'dispatch: loop {
        match pc {
            0x83196900 => {
    //   block [0x83196900..0x8319696C)
	// 83196900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83196904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83196908: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319690C: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83196910: 4BFFFD39  bl 0x83196648
	ctx.lr = 0x83196914;
	sub_83196648(ctx, base);
	// 83196914: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83196918: 409A0014  bne cr6, 0x8319692c
	if !ctx.cr[6].eq {
	pc = 0x8319692C; continue 'dispatch;
	}
	// 8319691C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196928: 4E800020  blr
	return;
	// 8319692C: A163001A  lhz r11, 0x1a(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(26 as u32) ) } as u64;
	// 83196930: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83196934: 556BC23E  srwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83196938: 554A442E  rlwinm r10, r10, 8, 0x10, 0x17
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 8319693C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83196940: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 83196944: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 83196948: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 8319694C: 409A0008  bne cr6, 0x83196954
	if !ctx.cr[6].eq {
	pc = 0x83196954; continue 'dispatch;
	}
	// 83196950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83196954: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196958: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8319695C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83196970 size=128
    let mut pc: u32 = 0x83196970;
    'dispatch: loop {
        match pc {
            0x83196970 => {
    //   block [0x83196970..0x831969F0)
	// 83196970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83196974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83196978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319697C: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83196980: 4BFFFCC9  bl 0x83196648
	ctx.lr = 0x83196984;
	sub_83196648(ctx, base);
	// 83196984: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83196988: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8319698C: 409A0014  bne cr6, 0x831969a0
	if !ctx.cr[6].eq {
	pc = 0x831969A0; continue 'dispatch;
	}
	// 83196990: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319699C: 4E800020  blr
	return;
	// 831969A0: 894B001C  lbz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831969A4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831969A8: 5549203E  rotlwi r9, r10, 4
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(4)) as u64;
	// 831969AC: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831969B0: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 831969B4: 554AE13E  srwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831969B8: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 831969BC: 554A053E  clrlwi r10, r10, 0x14
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000FFFu64;
	// 831969C0: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831969C4: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 831969C8: 5549403E  rotlwi r9, r10, 8
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 831969CC: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831969D0: 896B001E  lbz r11, 0x1e(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(30 as u32) ) } as u64;
	// 831969D4: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 831969D8: 556B053E  clrlwi r11, r11, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 831969DC: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831969E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831969E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831969E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831969EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831969F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831969F0 size=80
    let mut pc: u32 = 0x831969F0;
    'dispatch: loop {
        match pc {
            0x831969F0 => {
    //   block [0x831969F0..0x83196A40)
	// 831969F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831969F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831969F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831969FC: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83196A00: 4BFFFC49  bl 0x83196648
	ctx.lr = 0x83196A04;
	sub_83196648(ctx, base);
	// 83196A04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83196A08: 409A0014  bne cr6, 0x83196a1c
	if !ctx.cr[6].eq {
	pc = 0x83196A1C; continue 'dispatch;
	}
	// 83196A0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196A18: 4E800020  blr
	return;
	// 83196A1C: 8863001F  lbz r3, 0x1f(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(31 as u32) ) } as u64;
	// 83196A20: 4BFFF989  bl 0x831963a8
	ctx.lr = 0x83196A24;
	sub_831963A8(ctx, base);
	// 83196A24: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83196A28: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196A2C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83196A40 size=72
    let mut pc: u32 = 0x83196A40;
    'dispatch: loop {
        match pc {
            0x83196A40 => {
    //   block [0x83196A40..0x83196A88)
	// 83196A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83196A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83196A48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83196A4C: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83196A50: 4BFFFBF9  bl 0x83196648
	ctx.lr = 0x83196A54;
	sub_83196648(ctx, base);
	// 83196A54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83196A58: 409A0014  bne cr6, 0x83196a6c
	if !ctx.cr[6].eq {
	pc = 0x83196A6C; continue 'dispatch;
	}
	// 83196A5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196A68: 4E800020  blr
	return;
	// 83196A6C: 89630021  lbz r11, 0x21(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(33 as u32) ) } as u64;
	// 83196A70: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196A74: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196A78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83196A88 size=72
    let mut pc: u32 = 0x83196A88;
    'dispatch: loop {
        match pc {
            0x83196A88 => {
    //   block [0x83196A88..0x83196AD0)
	// 83196A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83196A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83196A90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83196A94: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83196A98: 4BFFFBB1  bl 0x83196648
	ctx.lr = 0x83196A9C;
	sub_83196648(ctx, base);
	// 83196A9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83196AA0: 409A0014  bne cr6, 0x83196ab4
	if !ctx.cr[6].eq {
	pc = 0x83196AB4; continue 'dispatch;
	}
	// 83196AA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196AA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196AAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196AB0: 4E800020  blr
	return;
	// 83196AB4: 89630022  lbz r11, 0x22(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(34 as u32) ) } as u64;
	// 83196AB8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196ABC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196AC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196AC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196AC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196ACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83196AD0 size=76
    let mut pc: u32 = 0x83196AD0;
    'dispatch: loop {
        match pc {
            0x83196AD0 => {
    //   block [0x83196AD0..0x83196B1C)
	// 83196AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83196AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83196AD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83196ADC: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83196AE0: 4BFFFB69  bl 0x83196648
	ctx.lr = 0x83196AE4;
	sub_83196648(ctx, base);
	// 83196AE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83196AE8: 409A0014  bne cr6, 0x83196afc
	if !ctx.cr[6].eq {
	pc = 0x83196AFC; continue 'dispatch;
	}
	// 83196AEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196AF8: 4E800020  blr
	return;
	// 83196AFC: 89630023  lbz r11, 0x23(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(35 as u32) ) } as u64;
	// 83196B00: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196B04: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 83196B08: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196B0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196B10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196B14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83196B20 size=76
    let mut pc: u32 = 0x83196B20;
    'dispatch: loop {
        match pc {
            0x83196B20 => {
    //   block [0x83196B20..0x83196B6C)
	// 83196B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83196B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83196B28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83196B2C: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83196B30: 4BFFFB19  bl 0x83196648
	ctx.lr = 0x83196B34;
	sub_83196648(ctx, base);
	// 83196B34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83196B38: 409A0014  bne cr6, 0x83196b4c
	if !ctx.cr[6].eq {
	pc = 0x83196B4C; continue 'dispatch;
	}
	// 83196B3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196B48: 4E800020  blr
	return;
	// 83196B4C: 89630023  lbz r11, 0x23(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(35 as u32) ) } as u64;
	// 83196B50: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196B54: 556BE7FE  rlwinm r11, r11, 0x1c, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 83196B58: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196B5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196B60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196B64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196B68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83196B70 size=72
    let mut pc: u32 = 0x83196B70;
    'dispatch: loop {
        match pc {
            0x83196B70 => {
    //   block [0x83196B70..0x83196BB8)
	// 83196B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83196B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83196B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83196B7C: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83196B80: 4BFFFAC9  bl 0x83196648
	ctx.lr = 0x83196B84;
	sub_83196648(ctx, base);
	// 83196B84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83196B88: 409A0014  bne cr6, 0x83196b9c
	if !ctx.cr[6].eq {
	pc = 0x83196B9C; continue 'dispatch;
	}
	// 83196B8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196B98: 4E800020  blr
	return;
	// 83196B9C: 89630024  lbz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 83196BA0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196BA4: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196BA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83196BB8 size=88
    let mut pc: u32 = 0x83196BB8;
    'dispatch: loop {
        match pc {
            0x83196BB8 => {
    //   block [0x83196BB8..0x83196C10)
	// 83196BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83196BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83196BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83196BC4: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83196BC8: 4BFFFA81  bl 0x83196648
	ctx.lr = 0x83196BCC;
	sub_83196648(ctx, base);
	// 83196BCC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83196BD0: 409A0014  bne cr6, 0x83196be4
	if !ctx.cr[6].eq {
	pc = 0x83196BE4; continue 'dispatch;
	}
	// 83196BD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196BE0: 4E800020  blr
	return;
	// 83196BE4: 89630025  lbz r11, 0x25(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(37 as u32) ) } as u64;
	// 83196BE8: 2F0B003F  cmpwi cr6, r11, 0x3f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 63, &mut ctx.xer);
	// 83196BEC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196BF0: 4099000C  ble cr6, 0x83196bfc
	if !ctx.cr[6].gt {
	pc = 0x83196BFC; continue 'dispatch;
	}
	// 83196BF4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83196BF8: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196BFC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83196C10 size=88
    let mut pc: u32 = 0x83196C10;
    'dispatch: loop {
        match pc {
            0x83196C10 => {
    //   block [0x83196C10..0x83196C68)
	// 83196C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83196C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83196C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83196C1C: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83196C20: 4BFFFA29  bl 0x83196648
	ctx.lr = 0x83196C24;
	sub_83196648(ctx, base);
	// 83196C24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83196C28: 409A0014  bne cr6, 0x83196c3c
	if !ctx.cr[6].eq {
	pc = 0x83196C3C; continue 'dispatch;
	}
	// 83196C2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196C38: 4E800020  blr
	return;
	// 83196C3C: 89630026  lbz r11, 0x26(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(38 as u32) ) } as u64;
	// 83196C40: 2F0B003F  cmpwi cr6, r11, 0x3f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 63, &mut ctx.xer);
	// 83196C44: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196C48: 4099000C  ble cr6, 0x83196c54
	if !ctx.cr[6].gt {
	pc = 0x83196C54; continue 'dispatch;
	}
	// 83196C4C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83196C50: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196C54: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196C58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83196C68 size=92
    let mut pc: u32 = 0x83196C68;
    'dispatch: loop {
        match pc {
            0x83196C68 => {
    //   block [0x83196C68..0x83196CC4)
	// 83196C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83196C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83196C70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83196C74: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83196C78: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 83196C7C: 4BFFF9CD  bl 0x83196648
	ctx.lr = 0x83196C80;
	sub_83196648(ctx, base);
	// 83196C80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83196C84: 409A0018  bne cr6, 0x83196c9c
	if !ctx.cr[6].eq {
	pc = 0x83196C9C; continue 'dispatch;
	}
	// 83196C88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83196C8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196C90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196C94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196C98: 4E800020  blr
	return;
	// 83196C9C: 8168000C  lwz r11, 0xc(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 83196CA0: 2F0B00D2  cmpwi cr6, r11, 0xd2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 210, &mut ctx.xer);
	// 83196CA4: 4198FFE4  blt cr6, 0x83196c88
	if ctx.cr[6].lt {
	pc = 0x83196C88; continue 'dispatch;
	}
	// 83196CA8: 89630027  lbz r11, 0x27(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(39 as u32) ) } as u64;
	// 83196CAC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196CB0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83196CB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83196CC8 size=160
    let mut pc: u32 = 0x83196CC8;
    'dispatch: loop {
        match pc {
            0x83196CC8 => {
    //   block [0x83196CC8..0x83196D68)
	// 83196CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83196CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83196CD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83196CD4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83196CD8: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 83196CDC: 2F0B006E  cmpwi cr6, r11, 0x6e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 110, &mut ctx.xer);
	// 83196CE0: 40980018  bge cr6, 0x83196cf8
	if !ctx.cr[6].lt {
	pc = 0x83196CF8; continue 'dispatch;
	}
	// 83196CE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83196CE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196CF4: 4E800020  blr
	return;
	// 83196CF8: 5488063E  clrlwi r8, r4, 0x18
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83196CFC: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 83196D00: 4BFFF649  bl 0x83196348
	ctx.lr = 0x83196D04;
	sub_83196348(ctx, base);
	// 83196D04: 2B0300C0  cmplwi cr6, r3, 0xc0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 192 as u32, &mut ctx.xer);
	// 83196D08: 419A0028  beq cr6, 0x83196d30
	if ctx.cr[6].eq {
	pc = 0x83196D30; continue 'dispatch;
	}
	// 83196D0C: 2B0300E0  cmplwi cr6, r3, 0xe0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 224 as u32, &mut ctx.xer);
	// 83196D10: 409AFFD4  bne cr6, 0x83196ce4
	if !ctx.cr[6].eq {
	pc = 0x83196CE4; continue 'dispatch;
	}
	// 83196D14: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 83196D18: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 83196D1C: 4BFFF92D  bl 0x83196648
	ctx.lr = 0x83196D20;
	sub_83196648(ctx, base);
	// 83196D20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83196D24: 419AFFC0  beq cr6, 0x83196ce4
	if ctx.cr[6].eq {
	pc = 0x83196CE4; continue 'dispatch;
	}
	// 83196D28: 4BFFF5F9  bl 0x83196320
	ctx.lr = 0x83196D2C;
	sub_83196320(ctx, base);
	// 83196D2C: 48000024  b 0x83196d50
	pc = 0x83196D50; continue 'dispatch;
	// 83196D30: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 83196D34: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 83196D38: 4BFFF911  bl 0x83196648
	ctx.lr = 0x83196D3C;
	sub_83196648(ctx, base);
	// 83196D3C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83196D40: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83196D44: 419AFFA0  beq cr6, 0x83196ce4
	if ctx.cr[6].eq {
	pc = 0x83196CE4; continue 'dispatch;
	}
	// 83196D48: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 83196D4C: 4BFFF7A5  bl 0x831964f0
	ctx.lr = 0x83196D50;
	sub_831964F0(ctx, base);
	// 83196D50: 90650000  stw r3, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83196D54: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83196D58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83196D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83196D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83196D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196D68 size=48
    let mut pc: u32 = 0x83196D68;
    'dispatch: loop {
        match pc {
            0x83196D68 => {
    //   block [0x83196D68..0x83196D98)
	// 83196D68: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83196D6C: 80E40004  lwz r7, 4(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 83196D70: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 83196D74: 81240008  lwz r9, 8(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 83196D78: 8104000C  lwz r8, 0xc(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 83196D7C: 54E5103A  slwi r5, r7, 2
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83196D80: 80C3000C  lwz r6, 0xc(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83196D84: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83196D88: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83196D8C: 7CA54050  subf r5, r5, r8
	ctx.r[5].s64 = ctx.r[8].s64 - ctx.r[5].s64;
	// 83196D90: 7CC73050  subf r6, r7, r6
	ctx.r[6].s64 = ctx.r[6].s64 - ctx.r[7].s64;
	// 83196D94: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196D98 size=68
    let mut pc: u32 = 0x83196D98;
    'dispatch: loop {
        match pc {
            0x83196D98 => {
    //   block [0x83196D98..0x83196DDC)
	// 83196D98: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 83196D9C: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83196DA0: 40990024  ble cr6, 0x83196dc4
	if !ctx.cr[6].gt {
	pc = 0x83196DC4; continue 'dispatch;
	}
	// 83196DA4: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 83196DA8: 888A0000  lbz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83196DAC: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 83196DB0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83196DB4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83196DB8: 988B0000  stb r4, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 83196DBC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83196DC0: 409AFFE8  bne cr6, 0x83196da8
	if !ctx.cr[6].eq {
	pc = 0x83196DA8; continue 'dispatch;
	}
	// 83196DC4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83196DC8: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 83196DCC: 7D4A3214  add r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 83196DD0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83196DD4: 409AFFC8  bne cr6, 0x83196d9c
	if !ctx.cr[6].eq {
	pc = 0x83196D9C; continue 'dispatch;
	}
	// 83196DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196DE0 size=128
    let mut pc: u32 = 0x83196DE0;
    'dispatch: loop {
        match pc {
            0x83196DE0 => {
    //   block [0x83196DE0..0x83196E60)
	// 83196DE0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 83196DE4: 81040004  lwz r8, 4(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 83196DE8: 81240008  lwz r9, 8(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 83196DEC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83196DF0: 80E4000C  lwz r7, 0xc(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 83196DF4: 5504103A  slwi r4, r8, 2
	ctx.r[4].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83196DF8: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83196DFC: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 83196E00: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83196E04: 7C843850  subf r4, r4, r7
	ctx.r[4].s64 = ctx.r[7].s64 - ctx.r[4].s64;
	// 83196E08: 7CC8F850  subf r6, r8, r31
	ctx.r[6].s64 = ctx.r[31].s64 - ctx.r[8].s64;
	// 83196E0C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83196E10: 40990048  ble cr6, 0x83196e58
	if !ctx.cr[6].gt {
	pc = 0x83196E58; continue 'dispatch;
	}
	// 83196E14: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 83196E18: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83196E1C: 40990028  ble cr6, 0x83196e44
	if !ctx.cr[6].gt {
	pc = 0x83196E44; continue 'dispatch;
	}
	// 83196E20: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 83196E24: 886A0000  lbz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83196E28: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 83196E2C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83196E30: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83196E34: 7C6328AE  lbzx r3, r3, r5
	ctx.r[3].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 83196E38: 986B0000  stb r3, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u8 ) };
	// 83196E3C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83196E40: 409AFFE4  bne cr6, 0x83196e24
	if !ctx.cr[6].eq {
	pc = 0x83196E24; continue 'dispatch;
	}
	// 83196E44: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 83196E48: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 83196E4C: 7D4A3214  add r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 83196E50: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 83196E54: 409AFFC4  bne cr6, 0x83196e18
	if !ctx.cr[6].eq {
	pc = 0x83196E18; continue 'dispatch;
	}
	// 83196E58: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 83196E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83196E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83196E60 size=600
    let mut pc: u32 = 0x83196E60;
    'dispatch: loop {
        match pc {
            0x83196E60 => {
    //   block [0x83196E60..0x831970B8)
	// 83196E60: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83196E64: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 83196E68: 396B0ED8  addi r11, r11, 0xed8
	ctx.r[11].s64 = ctx.r[11].s64 + 3800;
	// 83196E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83196E70: 394BF800  addi r10, r11, -0x800
	ctx.r[10].s64 = ctx.r[11].s64 + -2048;
	// 83196E74: C808D760  lfd f0, -0x28a0(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(-10400 as u32) ) };
	// 83196E78: 7D2707B4  extsw r7, r9
	ctx.r[7].s64 = ctx.r[9].s32 as i64;
	// 83196E7C: 390BF800  addi r8, r11, -0x800
	ctx.r[8].s64 = ctx.r[11].s64 + -2048;
	// 83196E80: 39290003  addi r9, r9, 3
	ctx.r[9].s64 = ctx.r[9].s64 + 3;
	// 83196E84: 39080080  addi r8, r8, 0x80
	ctx.r[8].s64 = ctx.r[8].s64 + 128;
	// 83196E88: F8E1FFF0  std r7, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[7].u64 ) };
	// 83196E8C: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83196E90: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 83196E94: FDAD0032  fmul f13, f13, f0
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 83196E98: D9AA0000  stfd f13, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.f[13].u64 ) };
	// 83196E9C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 83196EA0: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83196EA4: 4198FFD4  blt cr6, 0x83196e78
	if ctx.cr[6].lt {
	pc = 0x83196E78; continue 'dispatch;
	}
	// 83196EA8: 3CE0821A  lis r7, -0x7de6
	ctx.r[7].s64 = -2112225280;
	// 83196EAC: 394BF800  addi r10, r11, -0x800
	ctx.r[10].s64 = ctx.r[11].s64 + -2048;
	// 83196EB0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83196EB4: 390000B0  li r8, 0xb0
	ctx.r[8].s64 = 176;
	// 83196EB8: 394A0080  addi r10, r10, 0x80
	ctx.r[10].s64 = ctx.r[10].s64 + 128;
	// 83196EBC: C9A7BEA0  lfd f13, -0x4160(r7)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(-16736 as u32) ) };
	// 83196EC0: 7D2607B4  extsw r6, r9
	ctx.r[6].s64 = ctx.r[9].s32 as i64;
	// 83196EC4: 38EBF800  addi r7, r11, -0x800
	ctx.r[7].s64 = ctx.r[11].s64 + -2048;
	// 83196EC8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83196ECC: 38E70580  addi r7, r7, 0x580
	ctx.r[7].s64 = ctx.r[7].s64 + 1408;
	// 83196ED0: F8C1FFF0  std r6, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[6].u64 ) };
	// 83196ED4: C981FFF0  lfd f12, -0x10(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83196ED8: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 83196EDC: FD8C683A  fmadd f12, f12, f0, f13
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[13].f64;
	// 83196EE0: D98A0000  stfd f12, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.f[12].u64 ) };
	// 83196EE4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 83196EE8: 7F0A3800  cmpw cr6, r10, r7
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[7].s32, &mut ctx.xer);
	// 83196EEC: 4198FFD4  blt cr6, 0x83196ec0
	if ctx.cr[6].lt {
	pc = 0x83196EC0; continue 'dispatch;
	}
	// 83196EF0: 2F0800C0  cmpwi cr6, r8, 0xc0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 192, &mut ctx.xer);
	// 83196EF4: 40980050  bge cr6, 0x83196f44
	if !ctx.cr[6].lt {
	pc = 0x83196F44; continue 'dispatch;
	}
	// 83196EF8: 38EBF800  addi r7, r11, -0x800
	ctx.r[7].s64 = ctx.r[11].s64 + -2048;
	// 83196EFC: 550A1838  slwi r10, r8, 3
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83196F00: 3928FF50  addi r9, r8, -0xb0
	ctx.r[9].s64 = ctx.r[8].s64 + -176;
	// 83196F04: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 83196F08: 3CE0821A  lis r7, -0x7de6
	ctx.r[7].s64 = -2112225280;
	// 83196F0C: 390000C0  li r8, 0xc0
	ctx.r[8].s64 = 192;
	// 83196F10: C807BE98  lfd f0, -0x4168(r7)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(-16744 as u32) ) };
	// 83196F14: 7D2607B4  extsw r6, r9
	ctx.r[6].s64 = ctx.r[9].s32 as i64;
	// 83196F18: 38EBF800  addi r7, r11, -0x800
	ctx.r[7].s64 = ctx.r[11].s64 + -2048;
	// 83196F1C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83196F20: 38E70600  addi r7, r7, 0x600
	ctx.r[7].s64 = ctx.r[7].s64 + 1536;
	// 83196F24: F8C1FFF0  std r6, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[6].u64 ) };
	// 83196F28: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83196F2C: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 83196F30: FDAD002A  fadd f13, f13, f0
	ctx.f[13].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	// 83196F34: D9AA0000  stfd f13, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.f[13].u64 ) };
	// 83196F38: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 83196F3C: 7F0A3800  cmpw cr6, r10, r7
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[7].s32, &mut ctx.xer);
	// 83196F40: 4198FFD4  blt cr6, 0x83196f14
	if ctx.cr[6].lt {
	pc = 0x83196F14; continue 'dispatch;
	}
	// 83196F44: 2F080100  cmpwi cr6, r8, 0x100
	ctx.cr[6].compare_i32(ctx.r[8].s32, 256, &mut ctx.xer);
	// 83196F48: 40980068  bge cr6, 0x83196fb0
	if !ctx.cr[6].lt {
	pc = 0x83196FB0; continue 'dispatch;
	}
	// 83196F4C: 38EBF800  addi r7, r11, -0x800
	ctx.r[7].s64 = ctx.r[11].s64 + -2048;
	// 83196F50: 550A1838  slwi r10, r8, 3
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83196F54: 3928FF40  addi r9, r8, -0xc0
	ctx.r[9].s64 = ctx.r[8].s64 + -192;
	// 83196F58: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 83196F5C: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 83196F60: 3CE0821A  lis r7, -0x7de6
	ctx.r[7].s64 = -2112225280;
	// 83196F64: 3D008205  lis r8, -0x7dfb
	ctx.r[8].s64 = -2113601536;
	// 83196F68: C9A6C500  lfd f13, -0x3b00(r6)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(-15104 as u32) ) };
	// 83196F6C: C967BE90  lfd f11, -0x4170(r7)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(-16752 as u32) ) };
	// 83196F70: C988AA10  lfd f12, -0x55f0(r8)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(-22000 as u32) ) };
	// 83196F74: 7D2807B4  extsw r8, r9
	ctx.r[8].s64 = ctx.r[9].s32 as i64;
	// 83196F78: F901FFF0  std r8, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[8].u64 ) };
	// 83196F7C: C801FFF0  lfd f0, -0x10(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83196F80: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83196F84: FC005B3A  fmadd f0, f0, f12, f11
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[12].f64 + ctx.f[11].f64;
	// 83196F88: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83196F8C: 40990008  ble cr6, 0x83196f94
	if !ctx.cr[6].gt {
	pc = 0x83196F94; continue 'dispatch;
	}
	// 83196F90: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 83196F94: 390BF800  addi r8, r11, -0x800
	ctx.r[8].s64 = ctx.r[11].s64 + -2048;
	// 83196F98: D80A0000  stfd f0, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.f[0].u64 ) };
	// 83196F9C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 83196FA0: 39080800  addi r8, r8, 0x800
	ctx.r[8].s64 = ctx.r[8].s64 + 2048;
	// 83196FA4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83196FA8: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83196FAC: 4198FFC8  blt cr6, 0x83196f74
	if ctx.cr[6].lt {
	pc = 0x83196F74; continue 'dispatch;
	}
	// 83196FB0: 3CC0821A  lis r6, -0x7de6
	ctx.r[6].s64 = -2112225280;
	// 83196FB4: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 83196FB8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83196FBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83196FC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83196FC4: C806BE88  lfd f0, -0x4178(r6)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(-16760 as u32) ) };
	// 83196FC8: C9672450  lfd f11, 0x2450(r7)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(9296 as u32) ) };
	// 83196FCC: 7D2507B4  extsw r5, r9
	ctx.r[5].s64 = ctx.r[9].s32 as i64;
	// 83196FD0: 38CBEF70  addi r6, r11, -0x1090
	ctx.r[6].s64 = ctx.r[11].s64 + -4240;
	// 83196FD4: 38EBEF70  addi r7, r11, -0x1090
	ctx.r[7].s64 = ctx.r[11].s64 + -4240;
	// 83196FD8: 38C60400  addi r6, r6, 0x400
	ctx.r[6].s64 = ctx.r[6].s64 + 1024;
	// 83196FDC: 38E70400  addi r7, r7, 0x400
	ctx.r[7].s64 = ctx.r[7].s64 + 1024;
	// 83196FE0: F8A1FFF0  std r5, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[5].u64 ) };
	// 83196FE4: 38AB0400  addi r5, r11, 0x400
	ctx.r[5].s64 = ctx.r[11].s64 + 1024;
	// 83196FE8: 388B0400  addi r4, r11, 0x400
	ctx.r[4].s64 = ctx.r[11].s64 + 1024;
	// 83196FEC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83196FF0: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83196FF4: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 83196FF8: FDAD02F2  fmul f13, f13, f11
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[11].f64;
	// 83196FFC: FD8D002A  fadd f12, f13, f0
	ctx.f[12].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	// 83197000: 7D8835AE  stfdx f12, r8, r6
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32), ctx.f[12].u64) };
	// 83197004: 7D882DAE  stfdx f12, r8, r5
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[5].u32), ctx.f[12].u64) };
	// 83197008: FDA06828  fsub f13, f0, f13
	ctx.f[13].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 8319700C: 7DAA3DAE  stfdx f13, r10, r7
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32), ctx.f[13].u64) };
	// 83197010: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 83197014: 7DAA25AE  stfdx f13, r10, r4
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32), ctx.f[13].u64) };
	// 83197018: 394AFFF8  addi r10, r10, -8
	ctx.r[10].s64 = ctx.r[10].s64 + -8;
	// 8319701C: 2F0AFF40  cmpwi cr6, r10, -0xc0
	ctx.cr[6].compare_i32(ctx.r[10].s32, -192, &mut ctx.xer);
	// 83197020: 4199FFAC  bgt cr6, 0x83196fcc
	if ctx.cr[6].gt {
	pc = 0x83196FCC; continue 'dispatch;
	}
	// 83197024: 2F090080  cmpwi cr6, r9, 0x80
	ctx.cr[6].compare_i32(ctx.r[9].s32, 128, &mut ctx.xer);
	// 83197028: 4098007C  bge cr6, 0x831970a4
	if !ctx.cr[6].lt {
	pc = 0x831970A4; continue 'dispatch;
	}
	// 8319702C: 7CE900D0  neg r7, r9
	ctx.r[7].s64 = -ctx.r[9].s64;
	// 83197030: 3909FFE8  addi r8, r9, -0x18
	ctx.r[8].s64 = ctx.r[9].s64 + -24;
	// 83197034: 552A1838  slwi r10, r9, 3
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83197038: 54E91838  slwi r9, r7, 3
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8319703C: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 83197040: 3CE0821A  lis r7, -0x7de6
	ctx.r[7].s64 = -2112225280;
	// 83197044: C946C260  lfd f10, -0x3da0(r6)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(-15776 as u32) ) };
	// 83197048: C967BE80  lfd f11, -0x4180(r7)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(-16768 as u32) ) };
	// 8319704C: 7D0607B4  extsw r6, r8
	ctx.r[6].s64 = ctx.r[8].s32 as i64;
	// 83197050: 38EBEF70  addi r7, r11, -0x1090
	ctx.r[7].s64 = ctx.r[11].s64 + -4240;
	// 83197054: 38AB0400  addi r5, r11, 0x400
	ctx.r[5].s64 = ctx.r[11].s64 + 1024;
	// 83197058: 388B0400  addi r4, r11, 0x400
	ctx.r[4].s64 = ctx.r[11].s64 + 1024;
	// 8319705C: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 83197060: F8C1FFF0  std r6, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[6].u64 ) };
	// 83197064: 38C70400  addi r6, r7, 0x400
	ctx.r[6].s64 = ctx.r[7].s64 + 1024;
	// 83197068: 38EBEF70  addi r7, r11, -0x1090
	ctx.r[7].s64 = ctx.r[11].s64 + -4240;
	// 8319706C: 38E70400  addi r7, r7, 0x400
	ctx.r[7].s64 = ctx.r[7].s64 + 1024;
	// 83197070: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83197074: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 83197078: FDAD52FA  fmadd f13, f13, f11, f10
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[11].f64 + ctx.f[10].f64;
	// 8319707C: FD8D002A  fadd f12, f13, f0
	ctx.f[12].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	// 83197080: 7D8A35AE  stfdx f12, r10, r6
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[6].u32), ctx.f[12].u64) };
	// 83197084: 7D8A2DAE  stfdx f12, r10, r5
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[5].u32), ctx.f[12].u64) };
	// 83197088: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8319708C: FDA06828  fsub f13, f0, f13
	ctx.f[13].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 83197090: 7DA93DAE  stfdx f13, r9, r7
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32), ctx.f[13].u64) };
	// 83197094: 7DA925AE  stfdx f13, r9, r4
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[4].u32), ctx.f[13].u64) };
	// 83197098: 2F0A0400  cmpwi cr6, r10, 0x400
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1024, &mut ctx.xer);
	// 8319709C: 3929FFF8  addi r9, r9, -8
	ctx.r[9].s64 = ctx.r[9].s64 + -8;
	// 831970A0: 4198FFAC  blt cr6, 0x8319704c
	if ctx.cr[6].lt {
	pc = 0x8319704C; continue 'dispatch;
	}
	// 831970A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 831970A8: C80AD228  lfd f0, -0x2dd8(r10)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-11736 as u32) ) };
	// 831970AC: D80B0000  stfd f0, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.f[0].u64 ) };
	// 831970B0: D80BEF70  stfd f0, -0x1090(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(-4240 as u32), ctx.f[0].u64 ) };
	// 831970B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831970B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831970B8 size=320
    let mut pc: u32 = 0x831970B8;
    'dispatch: loop {
        match pc {
            0x831970B8 => {
    //   block [0x831970B8..0x831971F8)
	// 831970B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831970BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831970C0: 3981FFF8  addi r12, r1, -8
	ctx.r[12].s64 = ctx.r[1].s64 + -8;
	// 831970C4: 480119B1  bl 0x831a8a74
	ctx.lr = 0x831970C8;
	sub_831A8A40(ctx, base);
	// 831970C8: C9830010  lfd f12, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 831970CC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 831970D0: C8E30038  lfd f7, 0x38(r3)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	// 831970D4: C9030030  lfd f8, 0x30(r3)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	// 831970D8: FFC70332  fmul f30, f7, f12
	ctx.f[30].f64 = ctx.f[7].f64 * ctx.f[12].f64;
	// 831970DC: C9630018  lfd f11, 0x18(r3)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 831970E0: FF880332  fmul f28, f8, f12
	ctx.f[28].f64 = ctx.f[8].f64 * ctx.f[12].f64;
	// 831970E4: C9230028  lfd f9, 0x28(r3)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	// 831970E8: FF6B0332  fmul f27, f11, f12
	ctx.f[27].f64 = ctx.f[11].f64 * ctx.f[12].f64;
	// 831970EC: C9430020  lfd f10, 0x20(r3)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 831970F0: FCA80272  fmul f5, f8, f9
	ctx.f[5].f64 = ctx.f[8].f64 * ctx.f[9].f64;
	// 831970F4: C8030008  lfd f0, 8(r3)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	// 831970F8: FFE802B2  fmul f31, f8, f10
	ctx.f[31].f64 = ctx.f[8].f64 * ctx.f[10].f64;
	// 831970FC: FFAA0332  fmul f29, f10, f12
	ctx.f[29].f64 = ctx.f[10].f64 * ctx.f[12].f64;
	// 83197100: C8C30040  lfd f6, 0x40(r3)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) };
	// 83197104: FD080032  fmul f8, f8, f0
	ctx.f[8].f64 = ctx.f[8].f64 * ctx.f[0].f64;
	// 83197108: C9A30000  lfd f13, 0(r3)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8319710C: FC4602F2  fmul f2, f6, f11
	ctx.f[2].f64 = ctx.f[6].f64 * ctx.f[11].f64;
	// 83197110: FC6602B2  fmul f3, f6, f10
	ctx.f[3].f64 = ctx.f[6].f64 * ctx.f[10].f64;
	// 83197114: FC2702F2  fmul f1, f7, f11
	ctx.f[1].f64 = ctx.f[7].f64 * ctx.f[11].f64;
	// 83197118: FFC6F038  fmsub f30, f6, f0, f30
	ctx.f[30].f64 = ctx.f[6].f64 * ctx.f[0].f64 - ctx.f[30].f64;
	// 8319711C: FCC6E378  fmsub f6, f6, f13, f28
	ctx.f[6].f64 = ctx.f[6].f64 * ctx.f[13].f64 - ctx.f[28].f64;
	// 83197120: FF8B0032  fmul f28, f11, f0
	ctx.f[28].f64 = ctx.f[11].f64 * ctx.f[0].f64;
	// 83197124: FC870272  fmul f4, f7, f9
	ctx.f[4].f64 = ctx.f[7].f64 * ctx.f[9].f64;
	// 83197128: FD69DB78  fmsub f11, f9, f13, f27
	ctx.f[11].f64 = ctx.f[9].f64 * ctx.f[13].f64 - ctx.f[27].f64;
	// 8319712C: FFA9E838  fmsub f29, f9, f0, f29
	ctx.f[29].f64 = ctx.f[9].f64 * ctx.f[0].f64 - ctx.f[29].f64;
	// 83197130: FD274378  fmsub f9, f7, f13, f8
	ctx.f[9].f64 = ctx.f[7].f64 * ctx.f[13].f64 - ctx.f[8].f64;
	// 83197134: FD050032  fmul f8, f5, f0
	ctx.f[8].f64 = ctx.f[5].f64 * ctx.f[0].f64;
	// 83197138: FC020032  fmul f0, f2, f0
	ctx.f[0].f64 = ctx.f[2].f64 * ctx.f[0].f64;
	// 8319713C: FCA22828  fsub f5, f2, f5
	ctx.f[5].f64 = ctx.f[2].f64 - ctx.f[5].f64;
	// 83197140: FC41F828  fsub f2, f1, f31
	ctx.f[2].f64 = ctx.f[1].f64 - ctx.f[31].f64;
	// 83197144: FCE32028  fsub f7, f3, f4
	ctx.f[7].f64 = ctx.f[3].f64 - ctx.f[4].f64;
	// 83197148: FD03437A  fmadd f8, f3, f13, f8
	ctx.f[8].f64 = ctx.f[3].f64 * ctx.f[13].f64 + ctx.f[8].f64;
	// 8319714C: FC04037A  fmadd f0, f4, f13, f0
	ctx.f[0].f64 = ctx.f[4].f64 * ctx.f[13].f64 + ctx.f[0].f64;
	// 83197150: FDAAE378  fmsub f13, f10, f13, f28
	ctx.f[13].f64 = ctx.f[10].f64 * ctx.f[13].f64 - ctx.f[28].f64;
	// 83197154: FD01433A  fmadd f8, f1, f12, f8
	ctx.f[8].f64 = ctx.f[1].f64 * ctx.f[12].f64 + ctx.f[8].f64;
	// 83197158: FC1F033A  fmadd f0, f31, f12, f0
	ctx.f[0].f64 = ctx.f[31].f64 * ctx.f[12].f64 + ctx.f[0].f64;
	// 8319715C: C98BE3A0  lfd f12, -0x1c60(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-7264 as u32) ) };
	// 83197160: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83197164: FC080028  fsub f0, f8, f0
	ctx.f[0].f64 = ctx.f[8].f64 - ctx.f[0].f64;
	// 83197168: FC0C0024  fdiv f0, f12, f0
	ctx.f[0].f64 = ctx.f[12].f64 / ctx.f[0].f64;
	// 8319716C: C98BBEB0  lfd f12, -0x4150(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-16720 as u32) ) };
	// 83197170: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83197174: FD070032  fmul f8, f7, f0
	ctx.f[8].f64 = ctx.f[7].f64 * ctx.f[0].f64;
	// 83197178: FCFE0032  fmul f7, f30, f0
	ctx.f[7].f64 = ctx.f[30].f64 * ctx.f[0].f64;
	// 8319717C: FC9D0032  fmul f4, f29, f0
	ctx.f[4].f64 = ctx.f[29].f64 * ctx.f[0].f64;
	// 83197180: FCA50032  fmul f5, f5, f0
	ctx.f[5].f64 = ctx.f[5].f64 * ctx.f[0].f64;
	// 83197184: FC6B0032  fmul f3, f11, f0
	ctx.f[3].f64 = ctx.f[11].f64 * ctx.f[0].f64;
	// 83197188: FCC60032  fmul f6, f6, f0
	ctx.f[6].f64 = ctx.f[6].f64 * ctx.f[0].f64;
	// 8319718C: FC420032  fmul f2, f2, f0
	ctx.f[2].f64 = ctx.f[2].f64 * ctx.f[0].f64;
	// 83197190: FD290032  fmul f9, f9, f0
	ctx.f[9].f64 = ctx.f[9].f64 * ctx.f[0].f64;
	// 83197194: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 83197198: FD680332  fmul f11, f8, f12
	ctx.f[11].f64 = ctx.f[8].f64 * ctx.f[12].f64;
	// 8319719C: D9640000  stfd f11, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.f[11].u64 ) };
	// 831971A0: C96BBEA8  lfd f11, -0x4158(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-16728 as u32) ) };
	// 831971A4: FD0702F2  fmul f8, f7, f11
	ctx.f[8].f64 = ctx.f[7].f64 * ctx.f[11].f64;
	// 831971A8: D9040008  stfd f8, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.f[8].u64 ) };
	// 831971AC: FD040332  fmul f8, f4, f12
	ctx.f[8].f64 = ctx.f[4].f64 * ctx.f[12].f64;
	// 831971B0: D9040010  stfd f8, 0x10(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.f[8].u64 ) };
	// 831971B4: FD0502F2  fmul f8, f5, f11
	ctx.f[8].f64 = ctx.f[5].f64 * ctx.f[11].f64;
	// 831971B8: D9040018  stfd f8, 0x18(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), ctx.f[8].u64 ) };
	// 831971BC: FD060332  fmul f8, f6, f12
	ctx.f[8].f64 = ctx.f[6].f64 * ctx.f[12].f64;
	// 831971C0: D9040020  stfd f8, 0x20(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(32 as u32), ctx.f[8].u64 ) };
	// 831971C4: FD0302F2  fmul f8, f3, f11
	ctx.f[8].f64 = ctx.f[3].f64 * ctx.f[11].f64;
	// 831971C8: D9040028  stfd f8, 0x28(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(40 as u32), ctx.f[8].u64 ) };
	// 831971CC: FD020332  fmul f8, f2, f12
	ctx.f[8].f64 = ctx.f[2].f64 * ctx.f[12].f64;
	// 831971D0: D9040030  stfd f8, 0x30(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(48 as u32), ctx.f[8].u64 ) };
	// 831971D4: FD6902F2  fmul f11, f9, f11
	ctx.f[11].f64 = ctx.f[9].f64 * ctx.f[11].f64;
	// 831971D8: D9640038  stfd f11, 0x38(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(56 as u32), ctx.f[11].u64 ) };
	// 831971DC: FC000332  fmul f0, f0, f12
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[12].f64;
	// 831971E0: D8040040  stfd f0, 0x40(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(64 as u32), ctx.f[0].u64 ) };
	// 831971E4: 3981FFF8  addi r12, r1, -8
	ctx.r[12].s64 = ctx.r[1].s64 + -8;
	// 831971E8: 480118D9  bl 0x831a8ac0
	ctx.lr = 0x831971EC;
	sub_831A8A8C(ctx, base);
	// 831971EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831971F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831971F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831971F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831971F8 size=244
    let mut pc: u32 = 0x831971F8;
    'dispatch: loop {
        match pc {
            0x831971F8 => {
    //   block [0x831971F8..0x831972EC)
	// 831971F8: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 831971FC: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 83197200: 7CC32050  subf r6, r3, r4
	ctx.r[6].s64 = ctx.r[4].s64 - ctx.r[3].s64;
	// 83197204: 39440006  addi r10, r4, 6
	ctx.r[10].s64 = ctx.r[4].s64 + 6;
	// 83197208: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 8319720C: 3FC0821A  lis r30, -0x7de6
	ctx.r[30].s64 = -2112225280;
	// 83197210: 3FE0821A  lis r31, -0x7de6
	ctx.r[31].s64 = -2112225280;
	// 83197214: 3C60821A  lis r3, -0x7de6
	ctx.r[3].s64 = -2112225280;
	// 83197218: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 8319721C: 3CA0821A  lis r5, -0x7de6
	ctx.r[5].s64 = -2112225280;
	// 83197220: C11EBEC4  lfs f8, -0x413c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16700 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 83197224: 38E0E000  li r7, -0x2000
	ctx.r[7].s64 = -8192;
	// 83197228: C13FBEC0  lfs f9, -0x4140(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-16704 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8319722C: 39202000  li r9, 0x2000
	ctx.r[9].s64 = 8192;
	// 83197230: C143BEBC  lfs f10, -0x4144(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-16708 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 83197234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83197238: C0049450  lfs f0, -0x6bb0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8319723C: C165BEB8  lfs f11, -0x4148(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-16712 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 83197240: 7CE507B4  extsw r5, r7
	ctx.r[5].s64 = ctx.r[7].s32 as i64;
	// 83197244: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83197248: 7D2407B4  extsw r4, r9
	ctx.r[4].s64 = ctx.r[9].s32 as i64;
	// 8319724C: B10B0002  sth r8, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 83197250: 3929FFC0  addi r9, r9, -0x40
	ctx.r[9].s64 = ctx.r[9].s64 + -64;
	// 83197254: 38E70040  addi r7, r7, 0x40
	ctx.r[7].s64 = ctx.r[7].s64 + 64;
	// 83197258: 2F09E000  cmpwi cr6, r9, -0x2000
	ctx.cr[6].compare_i32(ctx.r[9].s32, -8192, &mut ctx.xer);
	// 8319725C: F8A1FFC0  std r5, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[5].u64 ) };
	// 83197260: F881FFC8  std r4, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.r[4].u64 ) };
	// 83197264: C9A1FFC0  lfd f13, -0x40(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 83197268: C981FFC8  lfd f12, -0x38(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8319726C: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 83197270: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 83197274: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 83197278: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8319727C: ECED02FA  fmadds f7, f13, f11, f0
	ctx.f[7].f64 = (((ctx.f[13].f64 * ctx.f[11].f64 + ctx.f[0].f64) as f32) as f64);
	// 83197280: ECCC02BA  fmadds f6, f12, f10, f0
	ctx.f[6].f64 = (((ctx.f[12].f64 * ctx.f[10].f64 + ctx.f[0].f64) as f32) as f64);
	// 83197284: ED8C027A  fmadds f12, f12, f9, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[9].f64 + ctx.f[0].f64) as f32) as f64);
	// 83197288: EDAD023A  fmadds f13, f13, f8, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[8].f64 + ctx.f[0].f64) as f32) as f64);
	// 8319728C: FCE0381E  fctiwz f7, f7
	ctx.f[7].s64 = if ctx.f[7].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[7].f64.trunc() as i32 as i64 };
	// 83197290: D8E1FFD0  stfd f7, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[7].u64 ) };
	// 83197294: FCE0301E  fctiwz f7, f6
	ctx.f[7].s64 = if ctx.f[6].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[6].f64.trunc() as i32 as i64 };
	// 83197298: D8E1FFD8  stfd f7, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[7].u64 ) };
	// 8319729C: FD80601E  fctiwz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 831972A0: D981FFE0  stfd f12, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[12].u64 ) };
	// 831972A4: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 831972A8: D9A1FFE8  stfd f13, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[13].u64 ) };
	// 831972AC: A0A1FFD6  lhz r5, -0x2a(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-42 as u32) ) } as u64;
	// 831972B0: A081FFDE  lhz r4, -0x22(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-34 as u32) ) } as u64;
	// 831972B4: A061FFE6  lhz r3, -0x1a(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-26 as u32) ) } as u64;
	// 831972B8: A3E1FFEE  lhz r31, -0x12(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-18 as u32) ) } as u64;
	// 831972BC: B0ABFFFC  sth r5, -4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[5].u16 ) };
	// 831972C0: B08BFFFE  sth r4, -2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[4].u16 ) };
	// 831972C4: B10AFFFA  sth r8, -6(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(-6 as u32), ctx.r[8].u16 ) };
	// 831972C8: B06AFFFC  sth r3, -4(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[3].u16 ) };
	// 831972CC: 7FE65B2E  sthx r31, r6, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32), ctx.r[31].u16) };
	// 831972D0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831972D4: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831972D8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 831972DC: 4199FF64  bgt cr6, 0x83197240
	if ctx.cr[6].gt {
	pc = 0x83197240; continue 'dispatch;
	}
	// 831972E0: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831972E4: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 831972E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831972F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831972F0 size=160
    let mut pc: u32 = 0x831972F0;
    'dispatch: loop {
        match pc {
            0x831972F0 => {
    //   block [0x831972F0..0x83197390)
	// 831972F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831972F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831972F8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831972FC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83197300: 80A50000  lwz r5, 0(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 83197304: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83197308: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8319730C: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83197310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83197314: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 83197318: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 8319731C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83197320: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 83197324: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83197328: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8319732C: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 83197330: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83197334: 91610084  stw r11, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 83197338: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8319733C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83197340: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 83197344: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83197348: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 8319734C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83197350: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 83197354: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83197358: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8319735C: 409A001C  bne cr6, 0x83197378
	if !ctx.cr[6].eq {
	pc = 0x83197378; continue 'dispatch;
	}
	// 83197360: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 83197364: 4BFFFA05  bl 0x83196d68
	ctx.lr = 0x83197368;
	sub_83196D68(ctx, base);
	// 83197368: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8319736C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83197370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83197374: 4E800020  blr
	return;
	// 83197378: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 8319737C: 4BFFFA65  bl 0x83196de0
	ctx.lr = 0x83197380;
	sub_83196DE0(ctx, base);
	// 83197380: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83197384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83197388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319738C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83197390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83197390 size=392
    let mut pc: u32 = 0x83197390;
    'dispatch: loop {
        match pc {
            0x83197390 => {
    //   block [0x83197390..0x83197518)
	// 83197390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83197394: 48010DD5  bl 0x831a8168
	ctx.lr = 0x83197398;
	sub_831A8130(ctx, base);
	// 83197398: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319739C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 831973A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831973A4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831973A8: 389F1000  addi r4, r31, 0x1000
	ctx.r[4].s64 = ctx.r[31].s64 + 4096;
	// 831973AC: 387F0800  addi r3, r31, 0x800
	ctx.r[3].s64 = ctx.r[31].s64 + 2048;
	// 831973B0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 831973B4: 4BFFFE45  bl 0x831971f8
	ctx.lr = 0x831973B8;
	sub_831971F8(ctx, base);
	// 831973B8: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 831973BC: 3D20821A  lis r9, -0x7de6
	ctx.r[9].s64 = -2112225280;
	// 831973C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831973C4: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 831973C8: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 831973CC: C9A8D760  lfd f13, -0x28a0(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(-10400 as u32) ) };
	// 831973D0: C809BEC8  lfd f0, -0x4138(r9)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(-16696 as u32) ) };
	// 831973D4: 392AFFF0  addi r9, r10, -0x10
	ctx.r[9].s64 = ctx.r[10].s64 + -16;
	// 831973D8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831973DC: 7D2907B4  extsw r9, r9
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 831973E0: 2F0A0100  cmpwi cr6, r10, 0x100
	ctx.cr[6].compare_i32(ctx.r[10].s32, 256, &mut ctx.xer);
	// 831973E4: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 831973E8: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831973EC: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 831973F0: FD8C683A  fmadd f12, f12, f0, f13
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[13].f64;
	// 831973F4: FD80601E  fctiwz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 831973F8: D9810058  stfd f12, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[12].u64 ) };
	// 831973FC: A121005E  lhz r9, 0x5e(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(94 as u32) ) } as u64;
	// 83197400: B12BFFFC  sth r9, -4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u16 ) };
	// 83197404: B12BFFFE  sth r9, -2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[9].u16 ) };
	// 83197408: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 8319740C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83197410: 4198FFC4  blt cr6, 0x831973d4
	if ctx.cr[6].lt {
	pc = 0x831973D4; continue 'dispatch;
	}
	// 83197414: 7D3DE050  subf r9, r29, r28
	ctx.r[9].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 83197418: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 8319741C: 7D2B07B4  extsw r11, r9
	ctx.r[11].s64 = ctx.r[9].s32 as i64;
	// 83197420: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 83197424: 39003FC0  li r8, 0x3fc0
	ctx.r[8].s64 = 16320;
	// 83197428: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8319742C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83197430: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83197434: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 83197438: C80BC500  lfd f0, -0x3b00(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-15104 as u32) ) };
	// 8319743C: 397F0006  addi r11, r31, 6
	ctx.r[11].s64 = ctx.r[31].s64 + 6;
	// 83197440: FC006824  fdiv f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[13].f64;
	// 83197444: 409A006C  bne cr6, 0x831974b0
	if !ctx.cr[6].eq {
	pc = 0x831974B0; continue 'dispatch;
	}
	// 83197448: 7D29EA14  add r9, r9, r29
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[29].u64;
	// 8319744C: 7F0AE800  cmpw cr6, r10, r29
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[29].s32, &mut ctx.xer);
	// 83197450: 4098000C  bge cr6, 0x8319745c
	if !ctx.cr[6].lt {
	pc = 0x8319745C; continue 'dispatch;
	}
	// 83197454: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83197458: 4800003C  b 0x83197494
	pc = 0x83197494; continue 'dispatch;
	// 8319745C: 7F0AE000  cmpw cr6, r10, r28
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83197460: 4099000C  ble cr6, 0x8319746c
	if !ctx.cr[6].gt {
	pc = 0x8319746C; continue 'dispatch;
	}
	// 83197464: B0EB0000  sth r7, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 83197468: 4800002C  b 0x83197494
	pc = 0x83197494; continue 'dispatch;
	// 8319746C: 7D2607B4  extsw r6, r9
	ctx.r[6].s64 = ctx.r[9].s32 as i64;
	// 83197470: F8C10058  std r6, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u64 ) };
	// 83197474: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83197478: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8319747C: FDAD0032  fmul f13, f13, f0
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 83197480: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 83197484: D9A10050  stfd f13, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[13].u64 ) };
	// 83197488: A0C10056  lhz r6, 0x56(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 8319748C: 54C6303E  rotlwi r6, r6, 6
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(6)) as u64;
	// 83197490: B0CB0000  sth r6, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 83197494: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83197498: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8319749C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831974A0: 2F0A0100  cmpwi cr6, r10, 0x100
	ctx.cr[6].compare_i32(ctx.r[10].s32, 256, &mut ctx.xer);
	// 831974A4: 4198FFA8  blt cr6, 0x8319744c
	if ctx.cr[6].lt {
	pc = 0x8319744C; continue 'dispatch;
	}
	// 831974A8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831974AC: 48010D0C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 831974B0: 7D3D00D0  neg r9, r29
	ctx.r[9].s64 = -ctx.r[29].s64;
	// 831974B4: 7F0AE800  cmpw cr6, r10, r29
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[29].s32, &mut ctx.xer);
	// 831974B8: 4098000C  bge cr6, 0x831974c4
	if !ctx.cr[6].lt {
	pc = 0x831974C4; continue 'dispatch;
	}
	// 831974BC: B0EB0000  sth r7, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 831974C0: 4800003C  b 0x831974fc
	pc = 0x831974FC; continue 'dispatch;
	// 831974C4: 7F0AE000  cmpw cr6, r10, r28
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[28].s32, &mut ctx.xer);
	// 831974C8: 4099000C  ble cr6, 0x831974d4
	if !ctx.cr[6].gt {
	pc = 0x831974D4; continue 'dispatch;
	}
	// 831974CC: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831974D0: 4800002C  b 0x831974fc
	pc = 0x831974FC; continue 'dispatch;
	// 831974D4: 7D2607B4  extsw r6, r9
	ctx.r[6].s64 = ctx.r[9].s32 as i64;
	// 831974D8: F8C10058  std r6, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u64 ) };
	// 831974DC: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831974E0: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 831974E4: FDAD0032  fmul f13, f13, f0
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 831974E8: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 831974EC: D9A10050  stfd f13, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[13].u64 ) };
	// 831974F0: A0C10056  lhz r6, 0x56(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 831974F4: 54C6303E  rotlwi r6, r6, 6
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(6)) as u64;
	// 831974F8: B0CB0000  sth r6, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 831974FC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83197500: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83197504: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83197508: 2F0A0100  cmpwi cr6, r10, 0x100
	ctx.cr[6].compare_i32(ctx.r[10].s32, 256, &mut ctx.xer);
	// 8319750C: 4198FFA8  blt cr6, 0x831974b4
	if ctx.cr[6].lt {
	pc = 0x831974B4; continue 'dispatch;
	}
	// 83197510: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83197514: 48010CA4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83197518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83197518 size=360
    let mut pc: u32 = 0x83197518;
    'dispatch: loop {
        match pc {
            0x83197518 => {
    //   block [0x83197518..0x83197680)
	// 83197518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319751C: 48010C4D  bl 0x831a8168
	ctx.lr = 0x83197520;
	sub_831A8130(ctx, base);
	// 83197520: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83197524: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83197528: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8319752C: 389F1000  addi r4, r31, 0x1000
	ctx.r[4].s64 = ctx.r[31].s64 + 4096;
	// 83197530: 387F0800  addi r3, r31, 0x800
	ctx.r[3].s64 = ctx.r[31].s64 + 2048;
	// 83197534: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83197538: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8319753C: 4BFFFCBD  bl 0x831971f8
	ctx.lr = 0x83197540;
	sub_831971F8(ctx, base);
	// 83197540: 57C8063E  clrlwi r8, r30, 0x18
	ctx.r[8].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 83197544: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 83197548: 39400009  li r10, 9
	ctx.r[10].s64 = 9;
	// 8319754C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83197550: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 83197554: B12BFFFC  sth r9, -4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u16 ) };
	// 83197558: B12BFFFE  sth r9, -2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[9].u16 ) };
	// 8319755C: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83197560: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83197564: B10B0002  sth r8, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 83197568: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8319756C: 409AFFE4  bne cr6, 0x83197550
	if !ctx.cr[6].eq {
	pc = 0x83197550; continue 'dispatch;
	}
	// 83197570: 3CA0820D  lis r5, -0x7df3
	ctx.r[5].s64 = -2113077248;
	// 83197574: 3CC0821A  lis r6, -0x7de6
	ctx.r[6].s64 = -2112225280;
	// 83197578: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8319757C: 3D008202  lis r8, -0x7dfe
	ctx.r[8].s64 = -2113798144;
	// 83197580: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 83197584: C925D760  lfd f9, -0x28a0(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(-10400 as u32) ) };
	// 83197588: 57A434B2  rlwinm r4, r29, 6, 0x12, 0x19
	ctx.r[4].u64 = ctx.r[29].u32 as u64 & 0x03FFFFFFu64;
	// 8319758C: C946BED8  lfd f10, -0x4128(r6)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(-16680 as u32) ) };
	// 83197590: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 83197594: C967C500  lfd f11, -0x3b00(r7)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(-15104 as u32) ) };
	// 83197598: 397F004C  addi r11, r31, 0x4c
	ctx.r[11].s64 = ctx.r[31].s64 + 76;
	// 8319759C: C988D228  lfd f12, -0x2dd8(r8)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(-11736 as u32) ) };
	// 831975A0: C9AAE5A0  lfd f13, -0x1a60(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-6752 as u32) ) };
	// 831975A4: 7D2A07B4  extsw r10, r9
	ctx.r[10].s64 = ctx.r[9].s32 as i64;
	// 831975A8: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 831975AC: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831975B0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 831975B4: FC006828  fsub f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 831975B8: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 831975BC: 4098000C  bge cr6, 0x831975c8
	if !ctx.cr[6].lt {
	pc = 0x831975C8; continue 'dispatch;
	}
	// 831975C0: FC006090  fmr f0, f12
	ctx.f[0].f64 = ctx.f[12].f64;
	// 831975C4: 48000010  b 0x831975d4
	pc = 0x831975D4; continue 'dispatch;
	// 831975C8: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 831975CC: 40990008  ble cr6, 0x831975d4
	if !ctx.cr[6].gt {
	pc = 0x831975D4; continue 'dispatch;
	}
	// 831975D0: FC005890  fmr f0, f11
	ctx.f[0].f64 = ctx.f[11].f64;
	// 831975D4: FC004ABA  fmadd f0, f0, f10, f9
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[10].f64 + ctx.f[9].f64;
	// 831975D8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831975DC: B08B0002  sth r4, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[4].u16 ) };
	// 831975E0: 2F090086  cmpwi cr6, r9, 0x86
	ctx.cr[6].compare_i32(ctx.r[9].s32, 134, &mut ctx.xer);
	// 831975E4: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 831975E8: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 831975EC: A141005E  lhz r10, 0x5e(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(94 as u32) ) } as u64;
	// 831975F0: B14BFFFC  sth r10, -4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[10].u16 ) };
	// 831975F4: B14BFFFE  sth r10, -2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[10].u16 ) };
	// 831975F8: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 831975FC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83197600: 4198FFA4  blt cr6, 0x831975a4
	if ctx.cr[6].lt {
	pc = 0x831975A4; continue 'dispatch;
	}
	// 83197604: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 83197608: 578834B2  rlwinm r8, r28, 6, 0x12, 0x19
	ctx.r[8].u64 = ctx.r[28].u32 as u64 & 0x03FFFFFFu64;
	// 8319760C: 39200086  li r9, 0x86
	ctx.r[9].s64 = 134;
	// 83197610: 397F0434  addi r11, r31, 0x434
	ctx.r[11].s64 = ctx.r[31].s64 + 1076;
	// 83197614: C9AABED0  lfd f13, -0x4130(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-16688 as u32) ) };
	// 83197618: 7D2A07B4  extsw r10, r9
	ctx.r[10].s64 = ctx.r[9].s32 as i64;
	// 8319761C: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 83197620: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83197624: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83197628: FC0D0028  fsub f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 - ctx.f[0].f64;
	// 8319762C: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 83197630: 4098000C  bge cr6, 0x8319763c
	if !ctx.cr[6].lt {
	pc = 0x8319763C; continue 'dispatch;
	}
	// 83197634: FC006090  fmr f0, f12
	ctx.f[0].f64 = ctx.f[12].f64;
	// 83197638: 48000010  b 0x83197648
	pc = 0x83197648; continue 'dispatch;
	// 8319763C: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 83197640: 40990008  ble cr6, 0x83197648
	if !ctx.cr[6].gt {
	pc = 0x83197648; continue 'dispatch;
	}
	// 83197644: FC005890  fmr f0, f11
	ctx.f[0].f64 = ctx.f[11].f64;
	// 83197648: FC004ABA  fmadd f0, f0, f10, f9
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[10].f64 + ctx.f[9].f64;
	// 8319764C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83197650: B10B0002  sth r8, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 83197654: 2F090100  cmpwi cr6, r9, 0x100
	ctx.cr[6].compare_i32(ctx.r[9].s32, 256, &mut ctx.xer);
	// 83197658: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8319765C: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 83197660: A1410056  lhz r10, 0x56(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 83197664: B14BFFFC  sth r10, -4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[10].u16 ) };
	// 83197668: B14BFFFE  sth r10, -2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[10].u16 ) };
	// 8319766C: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83197670: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83197674: 4198FFA4  blt cr6, 0x83197618
	if ctx.cr[6].lt {
	pc = 0x83197618; continue 'dispatch;
	}
	// 83197678: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8319767C: 48010B3C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83197680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83197680 size=440
    let mut pc: u32 = 0x83197680;
    'dispatch: loop {
        match pc {
            0x83197680 => {
    //   block [0x83197680..0x83197838)
	// 83197680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83197684: 48010AE5  bl 0x831a8168
	ctx.lr = 0x83197688;
	sub_831A8130(ctx, base);
	// 83197688: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319768C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83197690: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83197694: 389E1000  addi r4, r30, 0x1000
	ctx.r[4].s64 = ctx.r[30].s64 + 4096;
	// 83197698: 387E0800  addi r3, r30, 0x800
	ctx.r[3].s64 = ctx.r[30].s64 + 2048;
	// 8319769C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831976A0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 831976A4: 4BFFFB55  bl 0x831971f8
	ctx.lr = 0x831976A8;
	sub_831971F8(ctx, base);
	// 831976A8: 57E9063E  clrlwi r9, r31, 0x18
	ctx.r[9].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 831976AC: 395E0006  addi r10, r30, 6
	ctx.r[10].s64 = ctx.r[30].s64 + 6;
	// 831976B0: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 831976B4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831976B8: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 831976BC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 831976C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831976C4: 409AFFF0  bne cr6, 0x831976b4
	if !ctx.cr[6].eq {
	pc = 0x831976B4; continue 'dispatch;
	}
	// 831976C8: 57A934B2  rlwinm r9, r29, 6, 0x12, 0x19
	ctx.r[9].u64 = ctx.r[29].u32 as u64 & 0x03FFFFFFu64;
	// 831976CC: 395E0186  addi r10, r30, 0x186
	ctx.r[10].s64 = ctx.r[30].s64 + 390;
	// 831976D0: 39600052  li r11, 0x52
	ctx.r[11].s64 = 82;
	// 831976D4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831976D8: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 831976DC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 831976E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831976E4: 409AFFF0  bne cr6, 0x831976d4
	if !ctx.cr[6].eq {
	pc = 0x831976D4; continue 'dispatch;
	}
	// 831976E8: 578934B2  rlwinm r9, r28, 6, 0x12, 0x19
	ctx.r[9].u64 = ctx.r[28].u32 as u64 & 0x03FFFFFFu64;
	// 831976EC: 395E0416  addi r10, r30, 0x416
	ctx.r[10].s64 = ctx.r[30].s64 + 1046;
	// 831976F0: 3960007E  li r11, 0x7e
	ctx.r[11].s64 = 126;
	// 831976F4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831976F8: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 831976FC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 83197700: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83197704: 409AFFF0  bne cr6, 0x831976f4
	if !ctx.cr[6].eq {
	pc = 0x831976F4; continue 'dispatch;
	}
	// 83197708: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 8319770C: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 83197710: 3920FB78  li r9, -0x488
	ctx.r[9].s64 = -1160;
	// 83197714: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 83197718: B12BFFFC  sth r9, -4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u16 ) };
	// 8319771C: B12BFFFE  sth r9, -2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[9].u16 ) };
	// 83197720: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83197724: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83197728: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8319772C: 409AFFE8  bne cr6, 0x83197714
	if !ctx.cr[6].eq {
	pc = 0x83197714; continue 'dispatch;
	}
	// 83197730: 3CA0820D  lis r5, -0x7df3
	ctx.r[5].s64 = -2113077248;
	// 83197734: 3CC0821A  lis r6, -0x7de6
	ctx.r[6].s64 = -2112225280;
	// 83197738: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8319773C: 3D008202  lis r8, -0x7dfe
	ctx.r[8].s64 = -2113798144;
	// 83197740: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 83197744: C925D760  lfd f9, -0x28a0(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(-10400 as u32) ) };
	// 83197748: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 8319774C: C986BEF8  lfd f12, -0x4108(r6)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(-16648 as u32) ) };
	// 83197750: 397E0204  addi r11, r30, 0x204
	ctx.r[11].s64 = ctx.r[30].s64 + 516;
	// 83197754: C947C500  lfd f10, -0x3b00(r7)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(-15104 as u32) ) };
	// 83197758: C968D228  lfd f11, -0x2dd8(r8)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(-11736 as u32) ) };
	// 8319775C: C9AABEF0  lfd f13, -0x4110(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-16656 as u32) ) };
	// 83197760: 7D2A07B4  extsw r10, r9
	ctx.r[10].s64 = ctx.r[9].s32 as i64;
	// 83197764: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 83197768: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8319776C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83197770: FC006828  fsub f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 83197774: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 83197778: 4098000C  bge cr6, 0x83197784
	if !ctx.cr[6].lt {
	pc = 0x83197784; continue 'dispatch;
	}
	// 8319777C: FC005890  fmr f0, f11
	ctx.f[0].f64 = ctx.f[11].f64;
	// 83197780: 48000010  b 0x83197790
	pc = 0x83197790; continue 'dispatch;
	// 83197784: FF005000  fcmpu cr6, f0, f10
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[10].f64);
	// 83197788: 40990008  ble cr6, 0x83197790
	if !ctx.cr[6].gt {
	pc = 0x83197790; continue 'dispatch;
	}
	// 8319778C: FC005090  fmr f0, f10
	ctx.f[0].f64 = ctx.f[10].f64;
	// 83197790: FC004B3A  fmadd f0, f0, f12, f9
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[12].f64 + ctx.f[9].f64;
	// 83197794: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83197798: 2F090080  cmpwi cr6, r9, 0x80
	ctx.cr[6].compare_i32(ctx.r[9].s32, 128, &mut ctx.xer);
	// 8319779C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 831977A0: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 831977A4: A141005E  lhz r10, 0x5e(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(94 as u32) ) } as u64;
	// 831977A8: B14BFFFC  sth r10, -4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[10].u16 ) };
	// 831977AC: B14BFFFE  sth r10, -2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[10].u16 ) };
	// 831977B0: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 831977B4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831977B8: 4198FFA8  blt cr6, 0x83197760
	if ctx.cr[6].lt {
	pc = 0x83197760; continue 'dispatch;
	}
	// 831977BC: 3D00821A  lis r8, -0x7de6
	ctx.r[8].s64 = -2112225280;
	// 831977C0: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 831977C4: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
	// 831977C8: 397E0404  addi r11, r30, 0x404
	ctx.r[11].s64 = ctx.r[30].s64 + 1028;
	// 831977CC: C988BEE8  lfd f12, -0x4118(r8)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(-16664 as u32) ) };
	// 831977D0: C9AABEE0  lfd f13, -0x4120(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-16672 as u32) ) };
	// 831977D4: 7D2A07B4  extsw r10, r9
	ctx.r[10].s64 = ctx.r[9].s32 as i64;
	// 831977D8: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 831977DC: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831977E0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 831977E4: FC0D0028  fsub f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 - ctx.f[0].f64;
	// 831977E8: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 831977EC: 4098000C  bge cr6, 0x831977f8
	if !ctx.cr[6].lt {
	pc = 0x831977F8; continue 'dispatch;
	}
	// 831977F0: FC005890  fmr f0, f11
	ctx.f[0].f64 = ctx.f[11].f64;
	// 831977F4: 48000010  b 0x83197804
	pc = 0x83197804; continue 'dispatch;
	// 831977F8: FF005000  fcmpu cr6, f0, f10
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[10].f64);
	// 831977FC: 40990008  ble cr6, 0x83197804
	if !ctx.cr[6].gt {
	pc = 0x83197804; continue 'dispatch;
	}
	// 83197800: FC005090  fmr f0, f10
	ctx.f[0].f64 = ctx.f[10].f64;
	// 83197804: FC004B3A  fmadd f0, f0, f12, f9
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[12].f64 + ctx.f[9].f64;
	// 83197808: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8319780C: 2F090100  cmpwi cr6, r9, 0x100
	ctx.cr[6].compare_i32(ctx.r[9].s32, 256, &mut ctx.xer);
	// 83197810: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 83197814: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 83197818: A1410056  lhz r10, 0x56(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 8319781C: B14BFFFC  sth r10, -4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[10].u16 ) };
	// 83197820: B14BFFFE  sth r10, -2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[10].u16 ) };
	// 83197824: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83197828: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8319782C: 4198FFA8  blt cr6, 0x831977d4
	if ctx.cr[6].lt {
	pc = 0x831977D4; continue 'dispatch;
	}
	// 83197830: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83197834: 48010984  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83197838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83197838 size=428
    let mut pc: u32 = 0x83197838;
    'dispatch: loop {
        match pc {
            0x83197838 => {
    //   block [0x83197838..0x831979E4)
	// 83197838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319783C: 48010925  bl 0x831a8160
	ctx.lr = 0x83197840;
	sub_831A8130(ctx, base);
	// 83197840: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83197844: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83197848: 394BFE38  addi r10, r11, -0x1c8
	ctx.r[10].s64 = ctx.r[11].s64 + -456;
	// 8319784C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83197850: 388A0858  addi r4, r10, 0x858
	ctx.r[4].s64 = ctx.r[10].s64 + 2136;
	// 83197854: 386BE9C0  addi r3, r11, -0x1640
	ctx.r[3].s64 = ctx.r[11].s64 + -5696;
	// 83197858: 4BFFF861  bl 0x831970b8
	ctx.lr = 0x8319785C;
	sub_831970B8(ctx, base);
	// 8319785C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83197860: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83197864: 3CC0821A  lis r6, -0x7de6
	ctx.r[6].s64 = -2112225280;
	// 83197868: 808A0008  lwz r4, 8(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8319786C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83197870: 7FC75850  subf r30, r7, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 83197874: 39270002  addi r9, r7, 2
	ctx.r[9].s64 = ctx.r[7].s64 + 2;
	// 83197878: 7FA72050  subf r29, r7, r4
	ctx.r[29].s64 = ctx.r[4].s64 - ctx.r[7].s64;
	// 8319787C: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 83197880: C9A6BE88  lfd f13, -0x4178(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(-16760 as u32) ) };
	// 83197884: 390B0006  addi r8, r11, 6
	ctx.r[8].s64 = ctx.r[11].s64 + 6;
	// 83197888: 7F8B2050  subf r28, r11, r4
	ctx.r[28].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 8319788C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 83197890: 3B603FC0  li r27, 0x3fc0
	ctx.r[27].s64 = 16320;
	// 83197894: C807D760  lfd f0, -0x28a0(r7)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(-10400 as u32) ) };
	// 83197898: 38AA08A0  addi r5, r10, 0x8a0
	ctx.r[5].s64 = ctx.r[10].s64 + 2208;
	// 8319789C: C98A0858  lfd f12, 0x858(r10)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(2136 as u32) ) };
	// 831978A0: 38CA0010  addi r6, r10, 0x10
	ctx.r[6].s64 = ctx.r[10].s64 + 16;
	// 831978A4: 38EA10A0  addi r7, r10, 0x10a0
	ctx.r[7].s64 = ctx.r[10].s64 + 4256;
	// 831978A8: 7C6B2214  add r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 831978AC: 7D6B2CAE  lfdx f11, r11, r5
	ctx.f[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) };
	// 831978B0: FD8B033A  fmadd f12, f11, f12, f0
	ctx.f[12].f64 = ctx.f[11].f64 * ctx.f[12].f64 + ctx.f[0].f64;
	// 831978B4: FD80601E  fctiwz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 831978B8: D9810050  stfd f12, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[12].u64 ) };
	// 831978BC: A3410056  lhz r26, 0x56(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 831978C0: B3490002  sth r26, 2(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[26].u16 ) };
	// 831978C4: 7D6B2CAE  lfdx f11, r11, r5
	ctx.f[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) };
	// 831978C8: C98A0870  lfd f12, 0x870(r10)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(2160 as u32) ) };
	// 831978CC: FD8B033A  fmadd f12, f11, f12, f0
	ctx.f[12].f64 = ctx.f[11].f64 * ctx.f[12].f64 + ctx.f[0].f64;
	// 831978D0: FD80601E  fctiwz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 831978D4: D9810050  stfd f12, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[12].u64 ) };
	// 831978D8: A3410056  lhz r26, 0x56(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 831978DC: B3490000  sth r26, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 831978E0: 7D6B2CAE  lfdx f11, r11, r5
	ctx.f[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) };
	// 831978E4: C98A0888  lfd f12, 0x888(r10)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(2184 as u32) ) };
	// 831978E8: B3690004  sth r27, 4(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[27].u16 ) };
	// 831978EC: FD8B033A  fmadd f12, f11, f12, f0
	ctx.f[12].f64 = ctx.f[11].f64 * ctx.f[12].f64 + ctx.f[0].f64;
	// 831978F0: FD80601E  fctiwz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 831978F4: D9810050  stfd f12, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[12].u64 ) };
	// 831978F8: A0A10056  lhz r5, 0x56(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 831978FC: B0A9FFFE  sth r5, -2(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(-2 as u32), ctx.r[5].u16 ) };
	// 83197900: 7D6B34AE  lfdx f11, r11, r6
	ctx.f[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) };
	// 83197904: FD6B6828  fsub f11, f11, f13
	ctx.f[11].f64 = ctx.f[11].f64 - ctx.f[13].f64;
	// 83197908: C98A0860  lfd f12, 0x860(r10)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(2144 as u32) ) };
	// 8319790C: FD8B033A  fmadd f12, f11, f12, f0
	ctx.f[12].f64 = ctx.f[11].f64 * ctx.f[12].f64 + ctx.f[0].f64;
	// 83197910: FD80601E  fctiwz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 83197914: D9810050  stfd f12, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[12].u64 ) };
	// 83197918: A0A10056  lhz r5, 0x56(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 8319791C: B0A8FFFE  sth r5, -2(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(-2 as u32), ctx.r[5].u16 ) };
	// 83197920: 7D6B34AE  lfdx f11, r11, r6
	ctx.f[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) };
	// 83197924: FD6B6828  fsub f11, f11, f13
	ctx.f[11].f64 = ctx.f[11].f64 - ctx.f[13].f64;
	// 83197928: C98A0878  lfd f12, 0x878(r10)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(2168 as u32) ) };
	// 8319792C: FD8B033A  fmadd f12, f11, f12, f0
	ctx.f[12].f64 = ctx.f[11].f64 * ctx.f[12].f64 + ctx.f[0].f64;
	// 83197930: FD80601E  fctiwz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 83197934: D9810050  stfd f12, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[12].u64 ) };
	// 83197938: A0A10056  lhz r5, 0x56(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 8319793C: 7CBE4B2E  sthx r5, r30, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[9].u32), ctx.r[5].u16) };
	// 83197940: 7D6B34AE  lfdx f11, r11, r6
	ctx.f[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) };
	// 83197944: FD6B6828  fsub f11, f11, f13
	ctx.f[11].f64 = ctx.f[11].f64 - ctx.f[13].f64;
	// 83197948: C98A0890  lfd f12, 0x890(r10)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(2192 as u32) ) };
	// 8319794C: B3E80000  sth r31, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[31].u16 ) };
	// 83197950: FD8B033A  fmadd f12, f11, f12, f0
	ctx.f[12].f64 = ctx.f[11].f64 * ctx.f[12].f64 + ctx.f[0].f64;
	// 83197954: FD80601E  fctiwz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 83197958: D9810050  stfd f12, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[12].u64 ) };
	// 8319795C: A0C10056  lhz r6, 0x56(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 83197960: B0C8FFFA  sth r6, -6(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(-6 as u32), ctx.r[6].u16 ) };
	// 83197964: 7D6B3CAE  lfdx f11, r11, r7
	ctx.f[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) };
	// 83197968: FD6B6828  fsub f11, f11, f13
	ctx.f[11].f64 = ctx.f[11].f64 - ctx.f[13].f64;
	// 8319796C: C98A0868  lfd f12, 0x868(r10)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(2152 as u32) ) };
	// 83197970: FD8B033A  fmadd f12, f11, f12, f0
	ctx.f[12].f64 = ctx.f[11].f64 * ctx.f[12].f64 + ctx.f[0].f64;
	// 83197974: FD80601E  fctiwz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 83197978: D9810050  stfd f12, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[12].u64 ) };
	// 8319797C: A0C10056  lhz r6, 0x56(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 83197980: B0C30004  sth r6, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u16 ) };
	// 83197984: 7D6B3CAE  lfdx f11, r11, r7
	ctx.f[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) };
	// 83197988: FD6B6828  fsub f11, f11, f13
	ctx.f[11].f64 = ctx.f[11].f64 - ctx.f[13].f64;
	// 8319798C: C98A0880  lfd f12, 0x880(r10)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(2176 as u32) ) };
	// 83197990: FD8B033A  fmadd f12, f11, f12, f0
	ctx.f[12].f64 = ctx.f[11].f64 * ctx.f[12].f64 + ctx.f[0].f64;
	// 83197994: FD80601E  fctiwz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 83197998: D9810050  stfd f12, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[12].u64 ) };
	// 8319799C: A0C10056  lhz r6, 0x56(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 831979A0: 7CDD4B2E  sthx r6, r29, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[9].u32), ctx.r[6].u16) };
	// 831979A4: 7D8B3CAE  lfdx f12, r11, r7
	ctx.f[12].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) };
	// 831979A8: FD6C6828  fsub f11, f12, f13
	ctx.f[11].f64 = ctx.f[12].f64 - ctx.f[13].f64;
	// 831979AC: C98A0898  lfd f12, 0x898(r10)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(2200 as u32) ) };
	// 831979B0: 7FFC432E  sthx r31, r28, r8
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[8].u32), ctx.r[31].u16) };
	// 831979B4: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 831979B8: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 831979BC: FD8B033A  fmadd f12, f11, f12, f0
	ctx.f[12].f64 = ctx.f[11].f64 * ctx.f[12].f64 + ctx.f[0].f64;
	// 831979C0: FD80601E  fctiwz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 831979C4: D9810050  stfd f12, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[12].u64 ) };
	// 831979C8: A0E10056  lhz r7, 0x56(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 831979CC: 7CEB232E  sthx r7, r11, r4
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), ctx.r[7].u16) };
	// 831979D0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831979D4: 2F0B0800  cmpwi cr6, r11, 0x800
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2048, &mut ctx.xer);
	// 831979D8: 4198FEC0  blt cr6, 0x83197898
	if ctx.cr[6].lt {
	pc = 0x83197898; continue 'dispatch;
	}
	// 831979DC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831979E0: 480107D0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831979E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831979E8 size=64
    let mut pc: u32 = 0x831979E8;
    'dispatch: loop {
        match pc {
            0x831979E8 => {
    //   block [0x831979E8..0x83197A28)
	// 831979E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831979EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831979F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831979F4: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 831979F8: 39430800  addi r10, r3, 0x800
	ctx.r[10].s64 = ctx.r[3].s64 + 2048;
	// 831979FC: 396BFE38  addi r11, r11, -0x1c8
	ctx.r[11].s64 = ctx.r[11].s64 + -456;
	// 83197A00: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83197A04: 39431000  addi r10, r3, 0x1000
	ctx.r[10].s64 = ctx.r[3].s64 + 4096;
	// 83197A08: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83197A0C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83197A10: 4BFFF451  bl 0x83196e60
	ctx.lr = 0x83197A14;
	sub_83196E60(ctx, base);
	// 83197A14: 4BFFFE25  bl 0x83197838
	ctx.lr = 0x83197A18;
	sub_83197838(ctx, base);
	// 83197A18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83197A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83197A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83197A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83197A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83197A28 size=332
    let mut pc: u32 = 0x83197A28;
    'dispatch: loop {
        match pc {
            0x83197A28 => {
    //   block [0x83197A28..0x83197B74)
	// 83197A28: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83197A2C: 3CC0821A  lis r6, -0x7de6
	ctx.r[6].s64 = -2112225280;
	// 83197A30: 3CE0821A  lis r7, -0x7de6
	ctx.r[7].s64 = -2112225280;
	// 83197A34: 3D00821A  lis r8, -0x7de6
	ctx.r[8].s64 = -2112225280;
	// 83197A38: 394B16D8  addi r10, r11, 0x16d8
	ctx.r[10].s64 = ctx.r[11].s64 + 5848;
	// 83197A3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83197A40: C9A6BF30  lfd f13, -0x40d0(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(-16592 as u32) ) };
	// 83197A44: 396A0C00  addi r11, r10, 0xc00
	ctx.r[11].s64 = ctx.r[10].s64 + 3072;
	// 83197A48: C987BF28  lfd f12, -0x40d8(r7)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(-16600 as u32) ) };
	// 83197A4C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 83197A50: C808BF20  lfd f0, -0x40e0(r8)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(-16608 as u32) ) };
	// 83197A54: 38E9FFF0  addi r7, r9, -0x10
	ctx.r[7].s64 = ctx.r[9].s64 + -16;
	// 83197A58: 390A0C00  addi r8, r10, 0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + 3072;
	// 83197A5C: 7CE707B4  extsw r7, r7
	ctx.r[7].s64 = ctx.r[7].s32 as i64;
	// 83197A60: 39080400  addi r8, r8, 0x400
	ctx.r[8].s64 = ctx.r[8].s64 + 1024;
	// 83197A64: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83197A68: F8E1FFF0  std r7, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[7].u64 ) };
	// 83197A6C: C961FFF0  lfd f11, -0x10(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83197A70: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 83197A74: FD6B603A  fmadd f11, f11, f0, f12
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 83197A78: FD6B0372  fmul f11, f11, f13
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	// 83197A7C: FD60581E  fctiwz f11, f11
	ctx.f[11].s64 = if ctx.f[11].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[11].f64.trunc() as i32 as i64 };
	// 83197A80: 7D605FAE  stfiwx f11, 0, r11
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 83197A84: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83197A88: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83197A8C: 4198FFC8  blt cr6, 0x83197a54
	if ctx.cr[6].lt {
	pc = 0x83197A54; continue 'dispatch;
	}
	// 83197A90: 3C80821A  lis r4, -0x7de6
	ctx.r[4].s64 = -2112225280;
	// 83197A94: 3CA0821A  lis r5, -0x7de6
	ctx.r[5].s64 = -2112225280;
	// 83197A98: 3CC0821A  lis r6, -0x7de6
	ctx.r[6].s64 = -2112225280;
	// 83197A9C: 3CE0821A  lis r7, -0x7de6
	ctx.r[7].s64 = -2112225280;
	// 83197AA0: 3900FF80  li r8, -0x80
	ctx.r[8].s64 = -128;
	// 83197AA4: C924BF18  lfd f9, -0x40e8(r4)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(-16616 as u32) ) };
	// 83197AA8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83197AAC: C945BF10  lfd f10, -0x40f0(r5)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(-16624 as u32) ) };
	// 83197AB0: 39200100  li r9, 0x100
	ctx.r[9].s64 = 256;
	// 83197AB4: C966BF08  lfd f11, -0x40f8(r6)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(-16632 as u32) ) };
	// 83197AB8: C987BF00  lfd f12, -0x4100(r7)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(-16640 as u32) ) };
	// 83197ABC: 7D0707B4  extsw r7, r8
	ctx.r[7].s64 = ctx.r[8].s32 as i64;
	// 83197AC0: 38AA1000  addi r5, r10, 0x1000
	ctx.r[5].s64 = ctx.r[10].s64 + 4096;
	// 83197AC4: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83197AC8: 38CA0800  addi r6, r10, 0x800
	ctx.r[6].s64 = ctx.r[10].s64 + 2048;
	// 83197ACC: 7CAB2A14  add r5, r11, r5
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 83197AD0: F8E1FFF0  std r7, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[7].u64 ) };
	// 83197AD4: 38EA0400  addi r7, r10, 0x400
	ctx.r[7].s64 = ctx.r[10].s64 + 1024;
	// 83197AD8: 7CCB3214  add r6, r11, r6
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 83197ADC: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 83197AE0: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 83197AE4: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 83197AE8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83197AEC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83197AF0: C801FFF0  lfd f0, -0x10(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83197AF4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83197AF8: FD000332  fmul f8, f0, f12
	ctx.f[8].f64 = ctx.f[0].f64 * ctx.f[12].f64;
	// 83197AFC: FCE002F2  fmul f7, f0, f11
	ctx.f[7].f64 = ctx.f[0].f64 * ctx.f[11].f64;
	// 83197B00: FCC002B2  fmul f6, f0, f10
	ctx.f[6].f64 = ctx.f[0].f64 * ctx.f[10].f64;
	// 83197B04: FC000272  fmul f0, f0, f9
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[9].f64;
	// 83197B08: FD080372  fmul f8, f8, f13
	ctx.f[8].f64 = ctx.f[8].f64 * ctx.f[13].f64;
	// 83197B0C: FCE70372  fmul f7, f7, f13
	ctx.f[7].f64 = ctx.f[7].f64 * ctx.f[13].f64;
	// 83197B10: FCC60372  fmul f6, f6, f13
	ctx.f[6].f64 = ctx.f[6].f64 * ctx.f[13].f64;
	// 83197B14: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 83197B18: FD00401E  fctiwz f8, f8
	ctx.f[8].s64 = if ctx.f[8].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[8].f64.trunc() as i32 as i64 };
	// 83197B1C: 7D0027AE  stfiwx f8, 0, r4
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32, tmp.u32) };
	// 83197B20: FD00381E  fctiwz f8, f7
	ctx.f[8].s64 = if ctx.f[7].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[7].f64.trunc() as i32 as i64 };
	// 83197B24: 7D002FAE  stfiwx f8, 0, r5
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32, tmp.u32) };
	// 83197B28: FD00301E  fctiwz f8, f6
	ctx.f[8].s64 = if ctx.f[6].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[6].f64.trunc() as i32 as i64 };
	// 83197B2C: 7D0037AE  stfiwx f8, 0, r6
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32, tmp.u32) };
	// 83197B30: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 83197B34: 7C003FAE  stfiwx f0, 0, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32, tmp.u32) };
	// 83197B38: 409AFF84  bne cr6, 0x83197abc
	if !ctx.cr[6].eq {
	pc = 0x83197ABC; continue 'dispatch;
	}
	// 83197B3C: 396A1400  addi r11, r10, 0x1400
	ctx.r[11].s64 = ctx.r[10].s64 + 5120;
	// 83197B40: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 83197B44: 396B0400  addi r11, r11, 0x400
	ctx.r[11].s64 = ctx.r[11].s64 + 1024;
	// 83197B48: 38E000FF  li r7, 0xff
	ctx.r[7].s64 = 255;
	// 83197B4C: 390A1400  addi r8, r10, 0x1400
	ctx.r[8].s64 = ctx.r[10].s64 + 5120;
	// 83197B50: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83197B54: 906BFC00  stw r3, -0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-1024 as u32), ctx.r[3].u32 ) };
	// 83197B58: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83197B5C: 90EB0400  stw r7, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[7].u32 ) };
	// 83197B60: 39080800  addi r8, r8, 0x800
	ctx.r[8].s64 = ctx.r[8].s64 + 2048;
	// 83197B64: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83197B68: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83197B6C: 4198FFE0  blt cr6, 0x83197b4c
	if ctx.cr[6].lt {
	pc = 0x83197B4C; continue 'dispatch;
	}
	// 83197B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83197B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83197B78 size=2184
    let mut pc: u32 = 0x83197B78;
    'dispatch: loop {
        match pc {
            0x83197B78 => {
    //   block [0x83197B78..0x83198400)
	// 83197B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83197B7C: 480105B5  bl 0x831a8130
	ctx.lr = 0x83197B80;
	sub_831A8130(ctx, base);
	// 83197B80: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 83197B84: 81240008  lwz r9, 8(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 83197B88: 7D671E70  srawi r7, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 83197B8C: 8104000C  lwz r8, 0xc(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 83197B90: 83A40000  lwz r29, 0(r4)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83197B94: 5579003E  slwi r25, r11, 0
	ctx.r[25].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 83197B98: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83197B9C: 5508F0BE  srwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83197BA0: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83197BA4: 83630014  lwz r27, 0x14(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83197BA8: 5545083C  slwi r5, r10, 1
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83197BAC: 83430010  lwz r26, 0x10(r3)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83197BB0: 5546003A  rlwinm r6, r10, 0, 0, 0x1d
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 83197BB4: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83197BB8: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83197BBC: 7C670194  addze r3, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[3].s64 = tmp.s64;
	// 83197BC0: 7D290E70  srawi r9, r9, 1
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 1) as i64;
	// 83197BC4: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 83197BC8: 7D670E70  srawi r7, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83197BCC: 7D6B2850  subf r11, r11, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	// 83197BD0: 9061FF50  stw r3, -0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), ctx.r[3].u32 ) };
	// 83197BD4: 7D470194  addze r10, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[10].s64 = tmp.s64;
	// 83197BD8: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83197BDC: 5508083C  slwi r8, r8, 1
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83197BE0: 7F87EA14  add r28, r7, r29
	ctx.r[28].u64 = ctx.r[7].u64 + ctx.r[29].u64;
	// 83197BE4: 7CA6F214  add r5, r6, r30
	ctx.r[5].u64 = ctx.r[6].u64 + ctx.r[30].u64;
	// 83197BE8: 7CF94050  subf r7, r25, r8
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[25].s64;
	// 83197BEC: 7CCAD050  subf r6, r10, r26
	ctx.r[6].s64 = ctx.r[26].s64 - ctx.r[10].s64;
	// 83197BF0: 7D0AD850  subf r8, r10, r27
	ctx.r[8].s64 = ctx.r[27].s64 - ctx.r[10].s64;
	// 83197BF4: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83197BF8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83197BFC: 550AF0BE  srwi r10, r8, 2
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83197C00: 54C6F0BE  srwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shr(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 83197C04: 419A07F8  beq cr6, 0x831983fc
	if ctx.cr[6].eq {
	pc = 0x831983FC; continue 'dispatch;
	}
	// 83197C08: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83197C0C: 9121FF44  stw r9, -0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-188 as u32), ctx.r[9].u32 ) };
	// 83197C10: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83197C14: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83197C18: 3D40FFFF  lis r10, -1
	ctx.r[10].s64 = -65536;
	// 83197C1C: 9101FF54  stw r8, -0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-172 as u32), ctx.r[8].u32 ) };
	// 83197C20: 9161FF5C  stw r11, -0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-164 as u32), ctx.r[11].u32 ) };
	// 83197C24: 54CB103A  slwi r11, r6, 2
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83197C28: 90E1FF58  stw r7, -0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.r[7].u32 ) };
	// 83197C2C: 9161FF60  stw r11, -0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.r[11].u32 ) };
	// 83197C30: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83197C34: 396B16D8  addi r11, r11, 0x16d8
	ctx.r[11].s64 = ctx.r[11].s64 + 5848;
	// 83197C38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83197C3C: 419A0790  beq cr6, 0x831983cc
	if ctx.cr[6].eq {
	pc = 0x831983CC; continue 'dispatch;
	}
	// 83197C40: 9061FF38  stw r3, -0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-200 as u32), ctx.r[3].u32 ) };
	// 83197C44: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83197C48: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 83197C4C: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83197C50: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83197C54: 3B6B0C00  addi r27, r11, 0xc00
	ctx.r[27].s64 = ctx.r[11].s64 + 3072;
	// 83197C58: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83197C5C: 80C50000  lwz r6, 0(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 83197C60: 3B4B1000  addi r26, r11, 0x1000
	ctx.r[26].s64 = ctx.r[11].s64 + 4096;
	// 83197C64: 3B2B0800  addi r25, r11, 0x800
	ctx.r[25].s64 = ctx.r[11].s64 + 2048;
	// 83197C68: 9081FF40  stw r4, -0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-192 as u32), ctx.r[4].u32 ) };
	// 83197C6C: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83197C70: 9061FF3C  stw r3, -0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-196 as u32), ctx.r[3].u32 ) };
	// 83197C74: 550355BA  rlwinm r3, r8, 0xa, 0x16, 0x1d
	ctx.r[3].u64 = ctx.r[8].u32 as u64 & 0x003FFFFFu64;
	// 83197C78: 38A50004  addi r5, r5, 4
	ctx.r[5].s64 = ctx.r[5].s64 + 4;
	// 83197C7C: 3B0B0400  addi r24, r11, 0x400
	ctx.r[24].s64 = ctx.r[11].s64 + 1024;
	// 83197C80: 3AEB1400  addi r23, r11, 0x1400
	ctx.r[23].s64 = ctx.r[11].s64 + 5120;
	// 83197C84: 9081FF4C  stw r4, -0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-180 as u32), ctx.r[4].u32 ) };
	// 83197C88: 54E455BA  rlwinm r4, r7, 0xa, 0x16, 0x1d
	ctx.r[4].u64 = ctx.r[7].u32 as u64 & 0x003FFFFFu64;
	// 83197C8C: 7FC3D02E  lwzx r30, r3, r26
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83197C90: 3ACB1400  addi r22, r11, 0x1400
	ctx.r[22].s64 = ctx.r[11].s64 + 5120;
	// 83197C94: 90A1FF30  stw r5, -0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-208 as u32), ctx.r[5].u32 ) };
	// 83197C98: 7D555378  mr r21, r10
	ctx.r[21].u64 = ctx.r[10].u64;
	// 83197C9C: 3A8B1400  addi r20, r11, 0x1400
	ctx.r[20].s64 = ctx.r[11].s64 + 5120;
	// 83197CA0: 3A6B0C00  addi r19, r11, 0xc00
	ctx.r[19].s64 = ctx.r[11].s64 + 3072;
	// 83197CA4: 7FE4D82E  lwzx r31, r4, r27
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 83197CA8: 552455BA  rlwinm r4, r9, 0xa, 0x16, 0x1d
	ctx.r[4].u64 = ctx.r[9].u32 as u64 & 0x003FFFFFu64;
	// 83197CAC: 54EE95BA  rlwinm r14, r7, 0x12, 0x16, 0x1d
	ctx.r[14].u64 = ctx.r[7].u32 as u64 & 0x00003FFFu64;
	// 83197CB0: 3A4B1400  addi r18, r11, 0x1400
	ctx.r[18].s64 = ctx.r[11].s64 + 5120;
	// 83197CB4: 3A2B1400  addi r17, r11, 0x1400
	ctx.r[17].s64 = ctx.r[11].s64 + 5120;
	// 83197CB8: 7D505378  mr r16, r10
	ctx.r[16].u64 = ctx.r[10].u64;
	// 83197CBC: 7F64C82E  lwzx r27, r4, r25
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 83197CC0: 39EB1400  addi r15, r11, 0x1400
	ctx.r[15].s64 = ctx.r[11].s64 + 5120;
	// 83197CC4: 7CA4582E  lwzx r5, r4, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83197CC8: 54DA55BA  rlwinm r26, r6, 0xa, 0x16, 0x1d
	ctx.r[26].u64 = ctx.r[6].u32 as u64 & 0x003FFFFFu64;
	// 83197CCC: 7C83C02E  lwzx r4, r3, r24
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 83197CD0: 7C7EDA14  add r3, r30, r27
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[27].u64;
	// 83197CD4: 7FDF2A14  add r30, r31, r5
	ctx.r[30].u64 = ctx.r[31].u64 + ctx.r[5].u64;
	// 83197CD8: 7F7F2214  add r27, r31, r4
	ctx.r[27].u64 = ctx.r[31].u64 + ctx.r[4].u64;
	// 83197CDC: 7FE3F850  subf r31, r3, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 83197CE0: 7FDEA670  srawi r30, r30, 0x14
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[30].s32 >> 20) as i64;
	// 83197CE4: 7FFFA670  srawi r31, r31, 0x14
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 20) as i64;
	// 83197CE8: 57DE103A  slwi r30, r30, 2
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 83197CEC: 57FF103A  slwi r31, r31, 2
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 83197CF0: 7F7BA670  srawi r27, r27, 0x14
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[27].s32 >> 20) as i64;
	// 83197CF4: 3B2B1400  addi r25, r11, 0x1400
	ctx.r[25].s64 = ctx.r[11].s64 + 5120;
	// 83197CF8: 577B103A  slwi r27, r27, 2
	ctx.r[27].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 83197CFC: 7FDEB82E  lwzx r30, r30, r23
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 83197D00: 3B0B1400  addi r24, r11, 0x1400
	ctx.r[24].s64 = ctx.r[11].s64 + 5120;
	// 83197D04: 7FFFB02E  lwzx r31, r31, r22
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 83197D08: 7D575378  mr r23, r10
	ctx.r[23].u64 = ctx.r[10].u64;
	// 83197D0C: 53D5442E  rlwimi r21, r30, 8, 0x10, 0x17
	ctx.r[21].u64 = (((ctx.r[30].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[21].u64 & 0xFFFFFFFFFFFF00FF);
	// 83197D10: 54D695BA  rlwinm r22, r6, 0x12, 0x16, 0x1d
	ctx.r[22].u64 = ctx.r[6].u32 as u64 & 0x00003FFFu64;
	// 83197D14: 7EBFFB78  or r31, r21, r31
	ctx.r[31].u64 = ctx.r[21].u64 | ctx.r[31].u64;
	// 83197D18: 7FDBA02E  lwzx r30, r27, r20
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 83197D1C: 3AAB1400  addi r21, r11, 0x1400
	ctx.r[21].s64 = ctx.r[11].s64 + 5120;
	// 83197D20: 57FF402E  slwi r31, r31, 8
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(8);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 83197D24: 7FFFF378  or r31, r31, r30
	ctx.r[31].u64 = ctx.r[31].u64 | ctx.r[30].u64;
	// 83197D28: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 83197D2C: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83197D30: 7FEE982E  lwzx r31, r14, r19
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 83197D34: 7FBF2A14  add r29, r31, r5
	ctx.r[29].u64 = ctx.r[31].u64 + ctx.r[5].u64;
	// 83197D38: 7F63F850  subf r27, r3, r31
	ctx.r[27].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 83197D3C: 7FBDA670  srawi r29, r29, 0x14
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[29].s32 >> 20) as i64;
	// 83197D40: 7F7BA670  srawi r27, r27, 0x14
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[27].s32 >> 20) as i64;
	// 83197D44: 57BD103A  slwi r29, r29, 2
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 83197D48: 7FFF2214  add r31, r31, r4
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[4].u64;
	// 83197D4C: 577B103A  slwi r27, r27, 2
	ctx.r[27].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 83197D50: 7FFFA670  srawi r31, r31, 0x14
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 20) as i64;
	// 83197D54: 7FBD902E  lwzx r29, r29, r18
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 83197D58: 57FF103A  slwi r31, r31, 2
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 83197D5C: 7F7B882E  lwzx r27, r27, r17
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 83197D60: 53B0442E  rlwimi r16, r29, 8, 0x10, 0x17
	ctx.r[16].u64 = (((ctx.r[29].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[16].u64 & 0xFFFFFFFFFFFF00FF);
	// 83197D64: 7E1DDB78  or r29, r16, r27
	ctx.r[29].u64 = ctx.r[16].u64 | ctx.r[27].u64;
	// 83197D68: 7FFF782E  lwzx r31, r31, r15
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[15].u32)) } as u64;
	// 83197D6C: 3B6B0C00  addi r27, r11, 0xc00
	ctx.r[27].s64 = ctx.r[11].s64 + 3072;
	// 83197D70: 57BD402E  slwi r29, r29, 8
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(8);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 83197D74: 7FBFFB78  or r31, r29, r31
	ctx.r[31].u64 = ctx.r[29].u64 | ctx.r[31].u64;
	// 83197D78: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83197D7C: 7FFAD82E  lwzx r31, r26, r27
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 83197D80: 3B6B1400  addi r27, r11, 0x1400
	ctx.r[27].s64 = ctx.r[11].s64 + 5120;
	// 83197D84: 3B4B0C00  addi r26, r11, 0xc00
	ctx.r[26].s64 = ctx.r[11].s64 + 3072;
	// 83197D88: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83197D8C: 7DC3F850  subf r14, r3, r31
	ctx.r[14].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 83197D90: 3A8B1400  addi r20, r11, 0x1400
	ctx.r[20].s64 = ctx.r[11].s64 + 5120;
	// 83197D94: 7D535378  mr r19, r10
	ctx.r[19].u64 = ctx.r[10].u64;
	// 83197D98: 3A4B1400  addi r18, r11, 0x1400
	ctx.r[18].s64 = ctx.r[11].s64 + 5120;
	// 83197D9C: 93C1FF34  stw r30, -0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-204 as u32), ctx.r[30].u32 ) };
	// 83197DA0: 7FDF2A14  add r30, r31, r5
	ctx.r[30].u64 = ctx.r[31].u64 + ctx.r[5].u64;
	// 83197DA4: 7FFF2214  add r31, r31, r4
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[4].u64;
	// 83197DA8: 7FDEA670  srawi r30, r30, 0x14
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[30].s32 >> 20) as i64;
	// 83197DAC: 7DCEA670  srawi r14, r14, 0x14
	ctx.xer.ca = (ctx.r[14].s32 < 0) && ((ctx.r[14].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[14].s64 = (ctx.r[14].s32 >> 20) as i64;
	// 83197DB0: 57DE103A  slwi r30, r30, 2
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 83197DB4: 7FFFA670  srawi r31, r31, 0x14
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 20) as i64;
	// 83197DB8: 55CE103A  slwi r14, r14, 2
	ctx.r[14].u32 = ctx.r[14].u32.wrapping_shl(2);
	ctx.r[14].u64 = ctx.r[14].u32 as u64;
	// 83197DBC: 57FF103A  slwi r31, r31, 2
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 83197DC0: 3A2B1000  addi r17, r11, 0x1000
	ctx.r[17].s64 = ctx.r[11].s64 + 4096;
	// 83197DC4: 93C1FF48  stw r30, -0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-184 as u32), ctx.r[30].u32 ) };
	// 83197DC8: 553E95BA  rlwinm r30, r9, 0x12, 0x16, 0x1d
	ctx.r[30].u64 = ctx.r[9].u32 as u64 & 0x00003FFFu64;
	// 83197DCC: 3A0B0800  addi r16, r11, 0x800
	ctx.r[16].s64 = ctx.r[11].s64 + 2048;
	// 83197DD0: 39EB0400  addi r15, r11, 0x400
	ctx.r[15].s64 = ctx.r[11].s64 + 1024;
	// 83197DD4: 83A1FF48  lwz r29, -0xb8(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-184 as u32) ) } as u64;
	// 83197DD8: 93E1FF48  stw r31, -0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-184 as u32), ctx.r[31].u32 ) };
	// 83197DDC: 7FEEC02E  lwzx r31, r14, r24
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 83197DE0: 54F815BA  rlwinm r24, r7, 2, 0x16, 0x1d
	ctx.r[24].u64 = ctx.r[7].u32 as u64 & 0x3FFFFFFFu64;
	// 83197DE4: 7F3DC82E  lwzx r25, r29, r25
	ctx.r[25].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 83197DE8: 551D95BA  rlwinm r29, r8, 0x12, 0x16, 0x1d
	ctx.r[29].u64 = ctx.r[8].u32 as u64 & 0x00003FFFu64;
	// 83197DEC: 5337442E  rlwimi r23, r25, 8, 0x10, 0x17
	ctx.r[23].u64 = (((ctx.r[25].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[23].u64 & 0xFFFFFFFFFFFF00FF);
	// 83197DF0: 7EFFFB78  or r31, r23, r31
	ctx.r[31].u64 = ctx.r[23].u64 | ctx.r[31].u64;
	// 83197DF4: 3AEB1400  addi r23, r11, 0x1400
	ctx.r[23].s64 = ctx.r[11].s64 + 5120;
	// 83197DF8: 57FF402E  slwi r31, r31, 8
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(8);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 83197DFC: 8321FF48  lwz r25, -0xb8(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-184 as u32) ) } as u64;
	// 83197E00: 7F79D82E  lwzx r27, r25, r27
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 83197E04: 3B2B0C00  addi r25, r11, 0xc00
	ctx.r[25].s64 = ctx.r[11].s64 + 3072;
	// 83197E08: 7FFFDB78  or r31, r31, r27
	ctx.r[31].u64 = ctx.r[31].u64 | ctx.r[27].u64;
	// 83197E0C: 3B6B1400  addi r27, r11, 0x1400
	ctx.r[27].s64 = ctx.r[11].s64 + 5120;
	// 83197E10: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83197E14: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 83197E18: 7FF6D02E  lwzx r31, r22, r26
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83197E1C: 7D5A5378  mr r26, r10
	ctx.r[26].u64 = ctx.r[10].u64;
	// 83197E20: 3ACB1400  addi r22, r11, 0x1400
	ctx.r[22].s64 = ctx.r[11].s64 + 5120;
	// 83197E24: 7CBF2A14  add r5, r31, r5
	ctx.r[5].u64 = ctx.r[31].u64 + ctx.r[5].u64;
	// 83197E28: 7C63F850  subf r3, r3, r31
	ctx.r[3].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 83197E2C: 7CA5A670  srawi r5, r5, 0x14
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[5].s32 >> 20) as i64;
	// 83197E30: 7C63A670  srawi r3, r3, 0x14
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[3].s32 >> 20) as i64;
	// 83197E34: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83197E38: 7C9F2214  add r4, r31, r4
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[4].u64;
	// 83197E3C: 5463103A  slwi r3, r3, 2
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83197E40: 7C84A670  srawi r4, r4, 0x14
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[4].s32 >> 20) as i64;
	// 83197E44: 7CA5A82E  lwzx r5, r5, r21
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 83197E48: 5484103A  slwi r4, r4, 2
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83197E4C: 7D555378  mr r21, r10
	ctx.r[21].u64 = ctx.r[10].u64;
	// 83197E50: 50B3442E  rlwimi r19, r5, 8, 0x10, 0x17
	ctx.r[19].u64 = (((ctx.r[5].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[19].u64 & 0xFFFFFFFFFFFF00FF);
	// 83197E54: 7CA3A02E  lwzx r5, r3, r20
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 83197E58: 3A8B1400  addi r20, r11, 0x1400
	ctx.r[20].s64 = ctx.r[11].s64 + 5120;
	// 83197E5C: 7E652B78  or r5, r19, r5
	ctx.r[5].u64 = ctx.r[19].u64 | ctx.r[5].u64;
	// 83197E60: 7C84902E  lwzx r4, r4, r18
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 83197E64: 3A6B0C00  addi r19, r11, 0xc00
	ctx.r[19].s64 = ctx.r[11].s64 + 3072;
	// 83197E68: 54A5402E  slwi r5, r5, 8
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83197E6C: 54D2D5BA  rlwinm r18, r6, 0x1a, 0x16, 0x1d
	ctx.r[18].u64 = ctx.r[6].u32 as u64 & 0x0000003Fu64;
	// 83197E70: 7CA52378  or r5, r5, r4
	ctx.r[5].u64 = ctx.r[5].u64 | ctx.r[4].u64;
	// 83197E74: 90BC0000  stw r5, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 83197E78: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 83197E7C: 7FFE802E  lwzx r31, r30, r16
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[16].u32)) } as u64;
	// 83197E80: 3A0B1400  addi r16, r11, 0x1400
	ctx.r[16].s64 = ctx.r[11].s64 + 5120;
	// 83197E84: 7C7D882E  lwzx r3, r29, r17
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 83197E88: 3A2B1400  addi r17, r11, 0x1400
	ctx.r[17].s64 = ctx.r[11].s64 + 5120;
	// 83197E8C: 7CBE582E  lwzx r5, r30, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83197E90: 54FED5BA  rlwinm r30, r7, 0x1a, 0x16, 0x1d
	ctx.r[30].u64 = ctx.r[7].u32 as u64 & 0x0000003Fu64;
	// 83197E94: 7C63FA14  add r3, r3, r31
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 83197E98: 7C9D782E  lwzx r4, r29, r15
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[15].u32)) } as u64;
	// 83197E9C: 3BEB0C00  addi r31, r11, 0xc00
	ctx.r[31].s64 = ctx.r[11].s64 + 3072;
	// 83197EA0: 3BAB1400  addi r29, r11, 0x1400
	ctx.r[29].s64 = ctx.r[11].s64 + 5120;
	// 83197EA4: 7D4F5378  mr r15, r10
	ctx.r[15].u64 = ctx.r[10].u64;
	// 83197EA8: 7FFEF82E  lwzx r31, r30, r31
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83197EAC: 3BCB1400  addi r30, r11, 0x1400
	ctx.r[30].s64 = ctx.r[11].s64 + 5120;
	// 83197EB0: 7CFF2A14  add r7, r31, r5
	ctx.r[7].u64 = ctx.r[31].u64 + ctx.r[5].u64;
	// 83197EB4: 7DC3F850  subf r14, r3, r31
	ctx.r[14].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 83197EB8: 7CE7A670  srawi r7, r7, 0x14
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 20) as i64;
	// 83197EBC: 7DCEA670  srawi r14, r14, 0x14
	ctx.xer.ca = (ctx.r[14].s32 < 0) && ((ctx.r[14].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[14].s64 = (ctx.r[14].s32 >> 20) as i64;
	// 83197EC0: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83197EC4: 7FFF2214  add r31, r31, r4
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[4].u64;
	// 83197EC8: 55CE103A  slwi r14, r14, 2
	ctx.r[14].u32 = ctx.r[14].u32.wrapping_shl(2);
	ctx.r[14].u64 = ctx.r[14].u32 as u64;
	// 83197ECC: 7CE7E82E  lwzx r7, r7, r29
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 83197ED0: 7FFFA670  srawi r31, r31, 0x14
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 20) as i64;
	// 83197ED4: 50FA442E  rlwimi r26, r7, 8, 0x10, 0x17
	ctx.r[26].u64 = (((ctx.r[7].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[26].u64 & 0xFFFFFFFFFFFF00FF);
	// 83197ED8: 7CEED82E  lwzx r7, r14, r27
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 83197EDC: 57FF103A  slwi r31, r31, 2
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 83197EE0: 7F473B78  or r7, r26, r7
	ctx.r[7].u64 = ctx.r[26].u64 | ctx.r[7].u64;
	// 83197EE4: 3B4B0C00  addi r26, r11, 0xc00
	ctx.r[26].s64 = ctx.r[11].s64 + 3072;
	// 83197EE8: 54E7402E  slwi r7, r7, 8
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(8);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83197EEC: 7FFFF02E  lwzx r31, r31, r30
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 83197EF0: 7CFFFB78  or r31, r7, r31
	ctx.r[31].u64 = ctx.r[7].u64 | ctx.r[31].u64;
	// 83197EF4: 80E1FF34  lwz r7, -0xcc(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-204 as u32) ) } as u64;
	// 83197EF8: 93E70000  stw r31, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83197EFC: 3BE70004  addi r31, r7, 4
	ctx.r[31].s64 = ctx.r[7].s64 + 4;
	// 83197F00: 7CF8C82E  lwzx r7, r24, r25
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 83197F04: 54D915BA  rlwinm r25, r6, 2, 0x16, 0x1d
	ctx.r[25].u64 = ctx.r[6].u32 as u64 & 0x3FFFFFFFu64;
	// 83197F08: 3B0B1400  addi r24, r11, 0x1400
	ctx.r[24].s64 = ctx.r[11].s64 + 5120;
	// 83197F0C: 7FC72A14  add r30, r7, r5
	ctx.r[30].u64 = ctx.r[7].u64 + ctx.r[5].u64;
	// 83197F10: 7FA33850  subf r29, r3, r7
	ctx.r[29].s64 = ctx.r[7].s64 - ctx.r[3].s64;
	// 83197F14: 7FDEA670  srawi r30, r30, 0x14
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[30].s32 >> 20) as i64;
	// 83197F18: 7FBDA670  srawi r29, r29, 0x14
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[29].s32 >> 20) as i64;
	// 83197F1C: 57DE103A  slwi r30, r30, 2
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 83197F20: 7CE72214  add r7, r7, r4
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[4].u64;
	// 83197F24: 57BD103A  slwi r29, r29, 2
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 83197F28: 7CE7A670  srawi r7, r7, 0x14
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 20) as i64;
	// 83197F2C: 38DC0004  addi r6, r28, 4
	ctx.r[6].s64 = ctx.r[28].s64 + 4;
	// 83197F30: 7FDEB82E  lwzx r30, r30, r23
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 83197F34: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83197F38: 7D575378  mr r23, r10
	ctx.r[23].u64 = ctx.r[10].u64;
	// 83197F3C: 53D5442E  rlwimi r21, r30, 8, 0x10, 0x17
	ctx.r[21].u64 = (((ctx.r[30].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[21].u64 & 0xFFFFFFFFFFFF00FF);
	// 83197F40: 7FDDB02E  lwzx r30, r29, r22
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 83197F44: 3BBF0004  addi r29, r31, 4
	ctx.r[29].s64 = ctx.r[31].s64 + 4;
	// 83197F48: 7EBEF378  or r30, r21, r30
	ctx.r[30].u64 = ctx.r[21].u64 | ctx.r[30].u64;
	// 83197F4C: 7CE7A02E  lwzx r7, r7, r20
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 83197F50: 3ACB1000  addi r22, r11, 0x1000
	ctx.r[22].s64 = ctx.r[11].s64 + 4096;
	// 83197F54: 57DE402E  slwi r30, r30, 8
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(8);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 83197F58: 3AAB0800  addi r21, r11, 0x800
	ctx.r[21].s64 = ctx.r[11].s64 + 2048;
	// 83197F5C: 7FC73B78  or r7, r30, r7
	ctx.r[7].u64 = ctx.r[30].u64 | ctx.r[7].u64;
	// 83197F60: 3A8B1400  addi r20, r11, 0x1400
	ctx.r[20].s64 = ctx.r[11].s64 + 5120;
	// 83197F64: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 83197F68: 7CF2982E  lwzx r7, r18, r19
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[18].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 83197F6C: 3A6B1400  addi r19, r11, 0x1400
	ctx.r[19].s64 = ctx.r[11].s64 + 5120;
	// 83197F70: 7D525378  mr r18, r10
	ctx.r[18].u64 = ctx.r[10].u64;
	// 83197F74: 7FE72A14  add r31, r7, r5
	ctx.r[31].u64 = ctx.r[7].u64 + ctx.r[5].u64;
	// 83197F78: 7FC33850  subf r30, r3, r7
	ctx.r[30].s64 = ctx.r[7].s64 - ctx.r[3].s64;
	// 83197F7C: 7FFFA670  srawi r31, r31, 0x14
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 20) as i64;
	// 83197F80: 7FDEA670  srawi r30, r30, 0x14
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[30].s32 >> 20) as i64;
	// 83197F84: 57FF103A  slwi r31, r31, 2
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 83197F88: 57DE103A  slwi r30, r30, 2
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 83197F8C: 7CE72214  add r7, r7, r4
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[4].u64;
	// 83197F90: 7CE7A670  srawi r7, r7, 0x14
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 20) as i64;
	// 83197F94: 7FFF882E  lwzx r31, r31, r17
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 83197F98: 3A2B0400  addi r17, r11, 0x400
	ctx.r[17].s64 = ctx.r[11].s64 + 1024;
	// 83197F9C: 7FDE802E  lwzx r30, r30, r16
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[16].u32)) } as u64;
	// 83197FA0: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83197FA4: 53EF442E  rlwimi r15, r31, 8, 0x10, 0x17
	ctx.r[15].u64 = (((ctx.r[31].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[15].u64 & 0xFFFFFFFFFFFF00FF);
	// 83197FA8: 7DFFF378  or r31, r15, r30
	ctx.r[31].u64 = ctx.r[15].u64 | ctx.r[30].u64;
	// 83197FAC: 3BCB1400  addi r30, r11, 0x1400
	ctx.r[30].s64 = ctx.r[11].s64 + 5120;
	// 83197FB0: 57FB402E  slwi r27, r31, 8
	ctx.r[27].u32 = ctx.r[31].u32.wrapping_shl(8);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 83197FB4: 553FD5BA  rlwinm r31, r9, 0x1a, 0x16, 0x1d
	ctx.r[31].u64 = ctx.r[9].u32 as u64 & 0x0000003Fu64;
	// 83197FB8: 7CE7F02E  lwzx r7, r7, r30
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 83197FBC: 7F673B78  or r7, r27, r7
	ctx.r[7].u64 = ctx.r[27].u64 | ctx.r[7].u64;
	// 83197FC0: 3B6B1400  addi r27, r11, 0x1400
	ctx.r[27].s64 = ctx.r[11].s64 + 5120;
	// 83197FC4: 90FC0000  stw r7, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 83197FC8: 551CD5BA  rlwinm r28, r8, 0x1a, 0x16, 0x1d
	ctx.r[28].u64 = ctx.r[8].u32 as u64 & 0x0000003Fu64;
	// 83197FCC: 7CF9D02E  lwzx r7, r25, r26
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83197FD0: 3B4B1400  addi r26, r11, 0x1400
	ctx.r[26].s64 = ctx.r[11].s64 + 5120;
	// 83197FD4: 3B2B0C00  addi r25, r11, 0xc00
	ctx.r[25].s64 = ctx.r[11].s64 + 3072;
	// 83197FD8: 7CA72A14  add r5, r7, r5
	ctx.r[5].u64 = ctx.r[7].u64 + ctx.r[5].u64;
	// 83197FDC: 7C633850  subf r3, r3, r7
	ctx.r[3].s64 = ctx.r[7].s64 - ctx.r[3].s64;
	// 83197FE0: 7CA5A670  srawi r5, r5, 0x14
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[5].s32 >> 20) as i64;
	// 83197FE4: 7CE72214  add r7, r7, r4
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[4].u64;
	// 83197FE8: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83197FEC: 7C63A670  srawi r3, r3, 0x14
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[3].s32 >> 20) as i64;
	// 83197FF0: 7CE7A670  srawi r7, r7, 0x14
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 20) as i64;
	// 83197FF4: 5464103A  slwi r4, r3, 2
	ctx.r[4].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83197FF8: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83197FFC: 7CA5C02E  lwzx r5, r5, r24
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 83198000: 50B7442E  rlwimi r23, r5, 8, 0x10, 0x17
	ctx.r[23].u64 = (((ctx.r[5].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[23].u64 & 0xFFFFFFFFFFFF00FF);
	// 83198004: 7CA4D82E  lwzx r5, r4, r27
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 83198008: 7CE7D02E  lwzx r7, r7, r26
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 8319800C: 7EE52B78  or r5, r23, r5
	ctx.r[5].u64 = ctx.r[23].u64 | ctx.r[5].u64;
	// 83198010: 54A5402E  slwi r5, r5, 8
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83198014: 3B4B0C00  addi r26, r11, 0xc00
	ctx.r[26].s64 = ctx.r[11].s64 + 3072;
	// 83198018: 7CA73B78  or r7, r5, r7
	ctx.r[7].u64 = ctx.r[5].u64 | ctx.r[7].u64;
	// 8319801C: 80A1FF30  lwz r5, -0xd0(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-208 as u32) ) } as u64;
	// 83198020: 3B0B1400  addi r24, r11, 0x1400
	ctx.r[24].s64 = ctx.r[11].s64 + 5120;
	// 83198024: 3AEB1400  addi r23, r11, 0x1400
	ctx.r[23].s64 = ctx.r[11].s64 + 5120;
	// 83198028: 3A0B1400  addi r16, r11, 0x1400
	ctx.r[16].s64 = ctx.r[11].s64 + 5120;
	// 8319802C: 7D4F5378  mr r15, r10
	ctx.r[15].u64 = ctx.r[10].u64;
	// 83198030: 90E60000  stw r7, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 83198034: 38E60004  addi r7, r6, 4
	ctx.r[7].s64 = ctx.r[6].s64 + 4;
	// 83198038: 80C1FF4C  lwz r6, -0xb4(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-180 as u32) ) } as u64;
	// 8319803C: 7C7CB02E  lwzx r3, r28, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 83198040: 3ACB0C00  addi r22, r11, 0xc00
	ctx.r[22].s64 = ctx.r[11].s64 + 3072;
	// 83198044: 3BC60004  addi r30, r6, 4
	ctx.r[30].s64 = ctx.r[6].s64 + 4;
	// 83198048: 7C9C882E  lwzx r4, r28, r17
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 8319804C: 90E1FF48  stw r7, -0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-184 as u32), ctx.r[7].u32 ) };
	// 83198050: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 83198054: 80C50000  lwz r6, 0(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 83198058: 38A50004  addi r5, r5, 4
	ctx.r[5].s64 = ctx.r[5].s64 + 4;
	// 8319805C: 54D195BA  rlwinm r17, r6, 0x12, 0x16, 0x1d
	ctx.r[17].u64 = ctx.r[6].u32 as u64 & 0x00003FFFu64;
	// 83198060: 90A1FF30  stw r5, -0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-208 as u32), ctx.r[5].u32 ) };
	// 83198064: 7CBF582E  lwzx r5, r31, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83198068: 7FFFA82E  lwzx r31, r31, r21
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 8319806C: 3AAB1400  addi r21, r11, 0x1400
	ctx.r[21].s64 = ctx.r[11].s64 + 5120;
	// 83198070: 7C63FA14  add r3, r3, r31
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 83198074: 54FF55BA  rlwinm r31, r7, 0xa, 0x16, 0x1d
	ctx.r[31].u64 = ctx.r[7].u32 as u64 & 0x003FFFFFu64;
	// 83198078: 7FFFC82E  lwzx r31, r31, r25
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 8319807C: 54F995BA  rlwinm r25, r7, 0x12, 0x16, 0x1d
	ctx.r[25].u64 = ctx.r[7].u32 as u64 & 0x00003FFFu64;
	// 83198080: 7F9F2A14  add r28, r31, r5
	ctx.r[28].u64 = ctx.r[31].u64 + ctx.r[5].u64;
	// 83198084: 7F63F850  subf r27, r3, r31
	ctx.r[27].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 83198088: 7F9CA670  srawi r28, r28, 0x14
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[28].s32 >> 20) as i64;
	// 8319808C: 7F7BA670  srawi r27, r27, 0x14
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[27].s32 >> 20) as i64;
	// 83198090: 579C103A  slwi r28, r28, 2
	ctx.r[28].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 83198094: 577B103A  slwi r27, r27, 2
	ctx.r[27].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 83198098: 7FFF2214  add r31, r31, r4
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[4].u64;
	// 8319809C: 7FFFA670  srawi r31, r31, 0x14
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 20) as i64;
	// 831980A0: 7F9CA02E  lwzx r28, r28, r20
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 831980A4: 7D545378  mr r20, r10
	ctx.r[20].u64 = ctx.r[10].u64;
	// 831980A8: 7F7B982E  lwzx r27, r27, r19
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 831980AC: 57FF103A  slwi r31, r31, 2
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 831980B0: 5392442E  rlwimi r18, r28, 8, 0x10, 0x17
	ctx.r[18].u64 = (((ctx.r[28].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[18].u64 & 0xFFFFFFFFFFFF00FF);
	// 831980B4: 3A6B1400  addi r19, r11, 0x1400
	ctx.r[19].s64 = ctx.r[11].s64 + 5120;
	// 831980B8: 7E5CDB78  or r28, r18, r27
	ctx.r[28].u64 = ctx.r[18].u64 | ctx.r[27].u64;
	// 831980BC: 3B6B1400  addi r27, r11, 0x1400
	ctx.r[27].s64 = ctx.r[11].s64 + 5120;
	// 831980C0: 579C402E  slwi r28, r28, 8
	ctx.r[28].u32 = ctx.r[28].u32.wrapping_shl(8);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 831980C4: 3A4B0C00  addi r18, r11, 0xc00
	ctx.r[18].s64 = ctx.r[11].s64 + 3072;
	// 831980C8: 7FFFD82E  lwzx r31, r31, r27
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 831980CC: 3B6B1400  addi r27, r11, 0x1400
	ctx.r[27].s64 = ctx.r[11].s64 + 5120;
	// 831980D0: 7F9FFB78  or r31, r28, r31
	ctx.r[31].u64 = ctx.r[28].u64 | ctx.r[31].u64;
	// 831980D4: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 831980D8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 831980DC: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 831980E0: 93E1FF34  stw r31, -0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-204 as u32), ctx.r[31].u32 ) };
	// 831980E4: 7FF9D02E  lwzx r31, r25, r26
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 831980E8: 54DA55BA  rlwinm r26, r6, 0xa, 0x16, 0x1d
	ctx.r[26].u64 = ctx.r[6].u32 as u64 & 0x003FFFFFu64;
	// 831980EC: 3B2B1400  addi r25, r11, 0x1400
	ctx.r[25].s64 = ctx.r[11].s64 + 5120;
	// 831980F0: 7FBF2A14  add r29, r31, r5
	ctx.r[29].u64 = ctx.r[31].u64 + ctx.r[5].u64;
	// 831980F4: 7DC3F850  subf r14, r3, r31
	ctx.r[14].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 831980F8: 7FBDA670  srawi r29, r29, 0x14
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[29].s32 >> 20) as i64;
	// 831980FC: 7DCEA670  srawi r14, r14, 0x14
	ctx.xer.ca = (ctx.r[14].s32 < 0) && ((ctx.r[14].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[14].s64 = (ctx.r[14].s32 >> 20) as i64;
	// 83198100: 57BD103A  slwi r29, r29, 2
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 83198104: 7FFF2214  add r31, r31, r4
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[4].u64;
	// 83198108: 55CE103A  slwi r14, r14, 2
	ctx.r[14].u32 = ctx.r[14].u32.wrapping_shl(2);
	ctx.r[14].u64 = ctx.r[14].u32 as u64;
	// 8319810C: 7FFFA670  srawi r31, r31, 0x14
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 20) as i64;
	// 83198110: 7FBDC02E  lwzx r29, r29, r24
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 83198114: 57FF103A  slwi r31, r31, 2
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 83198118: 53BC442E  rlwimi r28, r29, 8, 0x10, 0x17
	ctx.r[28].u64 = (((ctx.r[29].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[28].u64 & 0xFFFFFFFFFFFF00FF);
	// 8319811C: 7FAED82E  lwzx r29, r14, r27
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 83198120: 7F9DEB78  or r29, r28, r29
	ctx.r[29].u64 = ctx.r[28].u64 | ctx.r[29].u64;
	// 83198124: 7FFFB82E  lwzx r31, r31, r23
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 83198128: 57BD402E  slwi r29, r29, 8
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(8);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8319812C: 7FBDFB78  or r29, r29, r31
	ctx.r[29].u64 = ctx.r[29].u64 | ctx.r[31].u64;
	// 83198130: 83E1FF34  lwz r31, -0xcc(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-204 as u32) ) } as u64;
	// 83198134: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83198138: 3BBF0004  addi r29, r31, 4
	ctx.r[29].s64 = ctx.r[31].s64 + 4;
	// 8319813C: 7FFAB02E  lwzx r31, r26, r22
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 83198140: 7F9F2A14  add r28, r31, r5
	ctx.r[28].u64 = ctx.r[31].u64 + ctx.r[5].u64;
	// 83198144: 7F63F850  subf r27, r3, r31
	ctx.r[27].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 83198148: 7F9CA670  srawi r28, r28, 0x14
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[28].s32 >> 20) as i64;
	// 8319814C: 7F7BA670  srawi r27, r27, 0x14
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[27].s32 >> 20) as i64;
	// 83198150: 579C103A  slwi r28, r28, 2
	ctx.r[28].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 83198154: 7FFF2214  add r31, r31, r4
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[4].u64;
	// 83198158: 7F9CC82E  lwzx r28, r28, r25
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 8319815C: 577B103A  slwi r27, r27, 2
	ctx.r[27].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 83198160: 7FFFA670  srawi r31, r31, 0x14
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 20) as i64;
	// 83198164: 5394442E  rlwimi r20, r28, 8, 0x10, 0x17
	ctx.r[20].u64 = (((ctx.r[28].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[20].u64 & 0xFFFFFFFFFFFF00FF);
	// 83198168: 57FF103A  slwi r31, r31, 2
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 8319816C: 3B4B0400  addi r26, r11, 0x400
	ctx.r[26].s64 = ctx.r[11].s64 + 1024;
	// 83198170: 7F9BA82E  lwzx r28, r27, r21
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 83198174: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 83198178: 3B2B1400  addi r25, r11, 0x1400
	ctx.r[25].s64 = ctx.r[11].s64 + 5120;
	// 8319817C: 7E9CE378  or r28, r20, r28
	ctx.r[28].u64 = ctx.r[20].u64 | ctx.r[28].u64;
	// 83198180: 7FFF982E  lwzx r31, r31, r19
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 83198184: 54F415BA  rlwinm r20, r7, 2, 0x16, 0x1d
	ctx.r[20].u64 = ctx.r[7].u32 as u64 & 0x3FFFFFFFu64;
	// 83198188: 579C402E  slwi r28, r28, 8
	ctx.r[28].u32 = ctx.r[28].u32.wrapping_shl(8);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 8319818C: 3B0B1400  addi r24, r11, 0x1400
	ctx.r[24].s64 = ctx.r[11].s64 + 5120;
	// 83198190: 7F9CFB78  or r28, r28, r31
	ctx.r[28].u64 = ctx.r[28].u64 | ctx.r[31].u64;
	// 83198194: 83E1FF48  lwz r31, -0xb8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-184 as u32) ) } as u64;
	// 83198198: 7D575378  mr r23, r10
	ctx.r[23].u64 = ctx.r[10].u64;
	// 8319819C: 3ACB1400  addi r22, r11, 0x1400
	ctx.r[22].s64 = ctx.r[11].s64 + 5120;
	// 831981A0: 3AAB0C00  addi r21, r11, 0xc00
	ctx.r[21].s64 = ctx.r[11].s64 + 3072;
	// 831981A4: 3A6B1400  addi r19, r11, 0x1400
	ctx.r[19].s64 = ctx.r[11].s64 + 5120;
	// 831981A8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 831981AC: 3B9F0004  addi r28, r31, 4
	ctx.r[28].s64 = ctx.r[31].s64 + 4;
	// 831981B0: 7FF1902E  lwzx r31, r17, r18
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[17].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 831981B4: 7D525378  mr r18, r10
	ctx.r[18].u64 = ctx.r[10].u64;
	// 831981B8: 7CBF2A14  add r5, r31, r5
	ctx.r[5].u64 = ctx.r[31].u64 + ctx.r[5].u64;
	// 831981BC: 7D03F850  subf r8, r3, r31
	ctx.r[8].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 831981C0: 7CA5A670  srawi r5, r5, 0x14
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[5].s32 >> 20) as i64;
	// 831981C4: 7D08A670  srawi r8, r8, 0x14
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 20) as i64;
	// 831981C8: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831981CC: 7C9F2214  add r4, r31, r4
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[4].u64;
	// 831981D0: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831981D4: 7C84A670  srawi r4, r4, 0x14
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[4].s32 >> 20) as i64;
	// 831981D8: 386B1400  addi r3, r11, 0x1400
	ctx.r[3].s64 = ctx.r[11].s64 + 5120;
	// 831981DC: 7CA5802E  lwzx r5, r5, r16
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[16].u32)) } as u64;
	// 831981E0: 5484103A  slwi r4, r4, 2
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 831981E4: 3BEB0800  addi r31, r11, 0x800
	ctx.r[31].s64 = ctx.r[11].s64 + 2048;
	// 831981E8: 50AF442E  rlwimi r15, r5, 8, 0x10, 0x17
	ctx.r[15].u64 = (((ctx.r[5].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[15].u64 & 0xFFFFFFFFFFFF00FF);
	// 831981EC: 38AB1400  addi r5, r11, 0x1400
	ctx.r[5].s64 = ctx.r[11].s64 + 5120;
	// 831981F0: 7C84182E  lwzx r4, r4, r3
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 831981F4: 386B1000  addi r3, r11, 0x1000
	ctx.r[3].s64 = ctx.r[11].s64 + 4096;
	// 831981F8: 7D08282E  lwzx r8, r8, r5
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 831981FC: 38AB0C00  addi r5, r11, 0xc00
	ctx.r[5].s64 = ctx.r[11].s64 + 3072;
	// 83198200: 7DE84378  or r8, r15, r8
	ctx.r[8].u64 = ctx.r[15].u64 | ctx.r[8].u64;
	// 83198204: 5508402E  slwi r8, r8, 8
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(8);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83198208: 7D082378  or r8, r8, r4
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[4].u64;
	// 8319820C: 54E4D5BA  rlwinm r4, r7, 0x1a, 0x16, 0x1d
	ctx.r[4].u64 = ctx.r[7].u32 as u64 & 0x0000003Fu64;
	// 83198210: 576715BA  rlwinm r7, r27, 2, 0x16, 0x1d
	ctx.r[7].u64 = ctx.r[27].u32 as u64 & 0x3FFFFFFFu64;
	// 83198214: 911C0000  stw r8, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83198218: 552815BA  rlwinm r8, r9, 2, 0x16, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x3FFFFFFFu64;
	// 8319821C: 7CA4282E  lwzx r5, r4, r5
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 83198220: 389C0004  addi r4, r28, 4
	ctx.r[4].s64 = ctx.r[28].s64 + 4;
	// 83198224: 7C67182E  lwzx r3, r7, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 83198228: 7FE8F82E  lwzx r31, r8, r31
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8319822C: 7D28582E  lwzx r9, r8, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83198230: 7D07D02E  lwzx r8, r7, r26
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83198234: 7CE3FA14  add r7, r3, r31
	ctx.r[7].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 83198238: 7C654A14  add r3, r5, r9
	ctx.r[3].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 8319823C: 7FE54214  add r31, r5, r8
	ctx.r[31].u64 = ctx.r[5].u64 + ctx.r[8].u64;
	// 83198240: 7CA72850  subf r5, r7, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[7].s64;
	// 83198244: 7C63A670  srawi r3, r3, 0x14
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[3].s32 >> 20) as i64;
	// 83198248: 7CA5A670  srawi r5, r5, 0x14
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[5].s32 >> 20) as i64;
	// 8319824C: 5463103A  slwi r3, r3, 2
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83198250: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83198254: 7FFFA670  srawi r31, r31, 0x14
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 20) as i64;
	// 83198258: 57FF103A  slwi r31, r31, 2
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 8319825C: 7C63C82E  lwzx r3, r3, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 83198260: 7CA5C02E  lwzx r5, r5, r24
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 83198264: 5077442E  rlwimi r23, r3, 8, 0x10, 0x17
	ctx.r[23].u64 = (((ctx.r[3].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[23].u64 & 0xFFFFFFFFFFFF00FF);
	// 83198268: 7EE52B78  or r5, r23, r5
	ctx.r[5].u64 = ctx.r[23].u64 | ctx.r[5].u64;
	// 8319826C: 7C7FB02E  lwzx r3, r31, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 83198270: 54A5402E  slwi r5, r5, 8
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83198274: 7CA51B78  or r5, r5, r3
	ctx.r[5].u64 = ctx.r[5].u64 | ctx.r[3].u64;
	// 83198278: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 8319827C: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 83198280: 7CB4A82E  lwzx r5, r20, r21
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 83198284: 7FE54A14  add r31, r5, r9
	ctx.r[31].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 83198288: 7FA72850  subf r29, r7, r5
	ctx.r[29].s64 = ctx.r[5].s64 - ctx.r[7].s64;
	// 8319828C: 7FFFA670  srawi r31, r31, 0x14
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 20) as i64;
	// 83198290: 7FBDA670  srawi r29, r29, 0x14
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[29].s32 >> 20) as i64;
	// 83198294: 57FF103A  slwi r31, r31, 2
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 83198298: 7FFF982E  lwzx r31, r31, r19
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 8319829C: 7CA54214  add r5, r5, r8
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[8].u64;
	// 831982A0: 57BD103A  slwi r29, r29, 2
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 831982A4: 53F2442E  rlwimi r18, r31, 8, 0x10, 0x17
	ctx.r[18].u64 = (((ctx.r[31].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[18].u64 & 0xFFFFFFFFFFFF00FF);
	// 831982A8: 3BEB1400  addi r31, r11, 0x1400
	ctx.r[31].s64 = ctx.r[11].s64 + 5120;
	// 831982AC: 7CA5A670  srawi r5, r5, 0x14
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[5].s32 >> 20) as i64;
	// 831982B0: 3B8B1400  addi r28, r11, 0x1400
	ctx.r[28].s64 = ctx.r[11].s64 + 5120;
	// 831982B4: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831982B8: 54DBD5BA  rlwinm r27, r6, 0x1a, 0x16, 0x1d
	ctx.r[27].u64 = ctx.r[6].u32 as u64 & 0x0000003Fu64;
	// 831982BC: 7FFDF82E  lwzx r31, r29, r31
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 831982C0: 3BAB0C00  addi r29, r11, 0xc00
	ctx.r[29].s64 = ctx.r[11].s64 + 3072;
	// 831982C4: 54D815BA  rlwinm r24, r6, 2, 0x16, 0x1d
	ctx.r[24].u64 = ctx.r[6].u32 as u64 & 0x3FFFFFFFu64;
	// 831982C8: 80C1FF38  lwz r6, -0xc8(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-200 as u32) ) } as u64;
	// 831982CC: 7E5FFB78  or r31, r18, r31
	ctx.r[31].u64 = ctx.r[18].u64 | ctx.r[31].u64;
	// 831982D0: 7CA5E02E  lwzx r5, r5, r28
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 831982D4: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 831982D8: 57FF402E  slwi r31, r31, 8
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(8);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 831982DC: 3B8B1400  addi r28, r11, 0x1400
	ctx.r[28].s64 = ctx.r[11].s64 + 5120;
	// 831982E0: 7FE52B78  or r5, r31, r5
	ctx.r[5].u64 = ctx.r[31].u64 | ctx.r[5].u64;
	// 831982E4: 3BEB1400  addi r31, r11, 0x1400
	ctx.r[31].s64 = ctx.r[11].s64 + 5120;
	// 831982E8: 90C1FF38  stw r6, -0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-200 as u32), ctx.r[6].u32 ) };
	// 831982EC: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 831982F0: 7D5A5378  mr r26, r10
	ctx.r[26].u64 = ctx.r[10].u64;
	// 831982F4: 3B2B1400  addi r25, r11, 0x1400
	ctx.r[25].s64 = ctx.r[11].s64 + 5120;
	// 831982F8: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 831982FC: 3AEB1400  addi r23, r11, 0x1400
	ctx.r[23].s64 = ctx.r[11].s64 + 5120;
	// 83198300: 7CBBE82E  lwzx r5, r27, r29
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 83198304: 3BA30004  addi r29, r3, 4
	ctx.r[29].s64 = ctx.r[3].s64 + 4;
	// 83198308: 3B6B0C00  addi r27, r11, 0xc00
	ctx.r[27].s64 = ctx.r[11].s64 + 3072;
	// 8319830C: 7C654A14  add r3, r5, r9
	ctx.r[3].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 83198310: 7E672850  subf r19, r7, r5
	ctx.r[19].s64 = ctx.r[5].s64 - ctx.r[7].s64;
	// 83198314: 7C63A670  srawi r3, r3, 0x14
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[3].s32 >> 20) as i64;
	// 83198318: 7E73A670  srawi r19, r19, 0x14
	ctx.xer.ca = (ctx.r[19].s32 < 0) && ((ctx.r[19].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[19].s64 = (ctx.r[19].s32 >> 20) as i64;
	// 8319831C: 5463103A  slwi r3, r3, 2
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83198320: 7CA54214  add r5, r5, r8
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[8].u64;
	// 83198324: 5673103A  slwi r19, r19, 2
	ctx.r[19].u32 = ctx.r[19].u32.wrapping_shl(2);
	ctx.r[19].u64 = ctx.r[19].u32 as u64;
	// 83198328: 7CA5A670  srawi r5, r5, 0x14
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[5].s32 >> 20) as i64;
	// 8319832C: 3ACB1400  addi r22, r11, 0x1400
	ctx.r[22].s64 = ctx.r[11].s64 + 5120;
	// 83198330: 7CC3E02E  lwzx r6, r3, r28
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 83198334: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83198338: 7D555378  mr r21, r10
	ctx.r[21].u64 = ctx.r[10].u64;
	// 8319833C: 50DA442E  rlwimi r26, r6, 8, 0x10, 0x17
	ctx.r[26].u64 = (((ctx.r[6].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[26].u64 & 0xFFFFFFFFFFFF00FF);
	// 83198340: 7CD3F82E  lwzx r6, r19, r31
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[19].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83198344: 3A8B1400  addi r20, r11, 0x1400
	ctx.r[20].s64 = ctx.r[11].s64 + 5120;
	// 83198348: 83E1FF3C  lwz r31, -0xc4(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-196 as u32) ) } as u64;
	// 8319834C: 7F463378  or r6, r26, r6
	ctx.r[6].u64 = ctx.r[26].u64 | ctx.r[6].u64;
	// 83198350: 7CA5C82E  lwzx r5, r5, r25
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 83198354: 54C6402E  slwi r6, r6, 8
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(8);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 83198358: 7CC62B78  or r6, r6, r5
	ctx.r[6].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 8319835C: 38A40004  addi r5, r4, 4
	ctx.r[5].s64 = ctx.r[4].s64 + 4;
	// 83198360: 3B850004  addi r28, r5, 4
	ctx.r[28].s64 = ctx.r[5].s64 + 4;
	// 83198364: 90C40000  stw r6, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 83198368: 7CD8D82E  lwzx r6, r24, r27
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 8319836C: 8081FF40  lwz r4, -0xc0(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-192 as u32) ) } as u64;
	// 83198370: 7D264A14  add r9, r6, r9
	ctx.r[9].u64 = ctx.r[6].u64 + ctx.r[9].u64;
	// 83198374: 7CE73050  subf r7, r7, r6
	ctx.r[7].s64 = ctx.r[6].s64 - ctx.r[7].s64;
	// 83198378: 7D29A670  srawi r9, r9, 0x14
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 20) as i64;
	// 8319837C: 7CE7A670  srawi r7, r7, 0x14
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 20) as i64;
	// 83198380: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83198384: 7D064214  add r8, r6, r8
	ctx.r[8].u64 = ctx.r[6].u64 + ctx.r[8].u64;
	// 83198388: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8319838C: 7D08A670  srawi r8, r8, 0x14
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 20) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 20) as i64;
	// 83198390: 7D29B82E  lwzx r9, r9, r23
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 83198394: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83198398: 7CE7B02E  lwzx r7, r7, r22
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 8319839C: 5135442E  rlwimi r21, r9, 8, 0x10, 0x17
	ctx.r[21].u64 = (((ctx.r[9].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[21].u64 & 0xFFFFFFFFFFFF00FF);
	// 831983A0: 7EA93B78  or r9, r21, r7
	ctx.r[9].u64 = ctx.r[21].u64 | ctx.r[7].u64;
	// 831983A4: 7D08A02E  lwzx r8, r8, r20
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 831983A8: 5529402E  slwi r9, r9, 8
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831983AC: 7D294378  or r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 831983B0: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831983B4: 80A1FF30  lwz r5, -0xd0(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-208 as u32) ) } as u64;
	// 831983B8: 409AF88C  bne cr6, 0x83197c44
	if !ctx.cr[6].eq {
	pc = 0x83197C44; continue 'dispatch;
	}
	// 831983BC: 8061FF50  lwz r3, -0xb0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-176 as u32) ) } as u64;
	// 831983C0: 8121FF44  lwz r9, -0xbc(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-188 as u32) ) } as u64;
	// 831983C4: 8101FF54  lwz r8, -0xac(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-172 as u32) ) } as u64;
	// 831983C8: 80E1FF58  lwz r7, -0xa8(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-168 as u32) ) } as u64;
	// 831983CC: 80C1FF5C  lwz r6, -0xa4(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-164 as u32) ) } as u64;
	// 831983D0: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 831983D4: 7FC8F214  add r30, r8, r30
	ctx.r[30].u64 = ctx.r[8].u64 + ctx.r[30].u64;
	// 831983D8: 7C862214  add r4, r6, r4
	ctx.r[4].u64 = ctx.r[6].u64 + ctx.r[4].u64;
	// 831983DC: 80C1FF60  lwz r6, -0xa0(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-160 as u32) ) } as u64;
	// 831983E0: 7CA82A14  add r5, r8, r5
	ctx.r[5].u64 = ctx.r[8].u64 + ctx.r[5].u64;
	// 831983E4: 7FE6FA14  add r31, r6, r31
	ctx.r[31].u64 = ctx.r[6].u64 + ctx.r[31].u64;
	// 831983E8: 9121FF44  stw r9, -0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-188 as u32), ctx.r[9].u32 ) };
	// 831983EC: 7FA7EA14  add r29, r7, r29
	ctx.r[29].u64 = ctx.r[7].u64 + ctx.r[29].u64;
	// 831983F0: 7F87E214  add r28, r7, r28
	ctx.r[28].u64 = ctx.r[7].u64 + ctx.r[28].u64;
	// 831983F4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 831983F8: 409AF840  bne cr6, 0x83197c38
	if !ctx.cr[6].eq {
	pc = 0x83197C38; continue 'dispatch;
	}
	// 831983FC: 4800FD84  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83198400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83198400 size=132
    let mut pc: u32 = 0x83198400;
    'dispatch: loop {
        match pc {
            0x83198400 => {
    //   block [0x83198400..0x83198484)
	// 83198400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83198404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83198408: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319840C: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 83198410: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83198414: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83198418: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 8319841C: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 83198420: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 83198424: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83198428: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8319842C: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 83198430: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83198434: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 83198438: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8319843C: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 83198440: 91610084  stw r11, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 83198444: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 83198448: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8319844C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 83198450: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83198454: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 83198458: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8319845C: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 83198460: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83198464: 409A0010  bne cr6, 0x83198474
	if !ctx.cr[6].eq {
	pc = 0x83198474; continue 'dispatch;
	}
	// 83198468: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8319846C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83198470: 4BFFF709  bl 0x83197b78
	ctx.lr = 0x83198474;
	sub_83197B78(ctx, base);
	// 83198474: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83198478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319847C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83198480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83198488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83198488 size=68
    let mut pc: u32 = 0x83198488;
    'dispatch: loop {
        match pc {
            0x83198488 => {
    //   block [0x83198488..0x831984CC)
	// 83198488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319848C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83198490: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83198494: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83198498: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319849C: 38A00128  li r5, 0x128
	ctx.r[5].s64 = 296;
	// 831984A0: 3BEB9800  addi r31, r11, -0x6800
	ctx.r[31].s64 = ctx.r[11].s64 + -26624;
	// 831984A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831984A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831984AC: 4800FD35  bl 0x831a81e0
	ctx.lr = 0x831984B0;
	sub_831A81E0(ctx, base);
	// 831984B0: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 831984B4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831984B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831984BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831984C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831984C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831984C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831984D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831984D0 size=40
    let mut pc: u32 = 0x831984D0;
    'dispatch: loop {
        match pc {
            0x831984D0 => {
    //   block [0x831984D0..0x831984F8)
	// 831984D0: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 831984D4: 394B9800  addi r10, r11, -0x6800
	ctx.r[10].s64 = ctx.r[11].s64 + -26624;
	// 831984D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831984DC: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 831984E0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831984E4: 40990024  ble cr6, 0x83198508
	if !ctx.cr[6].gt {
		sub_831984F8(ctx, base);
		return;
	}
	// 831984E8: 386A0008  addi r3, r10, 8
	ctx.r[3].s64 = ctx.r[10].s64 + 8;
	// 831984EC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831984F0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831984F4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831984F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831984F8 size=24
    let mut pc: u32 = 0x831984F8;
    'dispatch: loop {
        match pc {
            0x831984F8 => {
    //   block [0x831984F8..0x83198510)
	// 831984F8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831984FC: 38630024  addi r3, r3, 0x24
	ctx.r[3].s64 = ctx.r[3].s64 + 36;
	// 83198500: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83198504: 4198FFE8  blt cr6, 0x831984ec
	if ctx.cr[6].lt {
		sub_831984D0(ctx, base);
		return;
	}
	// 83198508: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8319850C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83198510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83198510 size=56
    let mut pc: u32 = 0x83198510;
    'dispatch: loop {
        match pc {
            0x83198510 => {
    //   block [0x83198510..0x83198548)
	// 83198510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83198514: 3940001F  li r10, 0x1f
	ctx.r[10].s64 = 31;
	// 83198518: 39200064  li r9, 0x64
	ctx.r[9].s64 = 100;
	// 8319851C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83198520: 38E0007F  li r7, 0x7f
	ctx.r[7].s64 = 127;
	// 83198524: 38C000FF  li r6, 0xff
	ctx.r[6].s64 = 255;
	// 83198528: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8319852C: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83198530: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 83198534: 91030004  stw r8, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 83198538: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 8319853C: 98E30015  stb r7, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[7].u8 ) };
	// 83198540: 98C30016  stb r6, 0x16(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(22 as u32), ctx.r[6].u8 ) };
	// 83198544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83198548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83198548 size=8
    let mut pc: u32 = 0x83198548;
    'dispatch: loop {
        match pc {
            0x83198548 => {
    //   block [0x83198548..0x83198550)
	// 83198548: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8319854C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83198550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83198550 size=28
    let mut pc: u32 = 0x83198550;
    'dispatch: loop {
        match pc {
            0x83198550 => {
    //   block [0x83198550..0x8319856C)
	// 83198550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83198554: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83198558: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319855C: 814B9800  lwz r10, -0x6800(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26624 as u32) ) } as u64;
	// 83198560: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 83198564: 914B9800  stw r10, -0x6800(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26624 as u32), ctx.r[10].u32 ) };
	// 83198568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83198570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83198570 size=84
    let mut pc: u32 = 0x83198570;
    'dispatch: loop {
        match pc {
            0x83198570 => {
    //   block [0x83198570..0x831985C4)
	// 83198570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83198574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83198578: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8319857C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83198580: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83198584: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 83198588: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8319858C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83198590: 419A0018  beq cr6, 0x831985a8
	if ctx.cr[6].eq {
	pc = 0x831985A8; continue 'dispatch;
	}
	// 83198594: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83198598: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8319859C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831985A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831985A4: 4E800421  bctrl
	ctx.lr = 0x831985A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831985A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831985AC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831985B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831985B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831985B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831985BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831985C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831985C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831985C8 size=20
    let mut pc: u32 = 0x831985C8;
    'dispatch: loop {
        match pc {
            0x831985C8 => {
    //   block [0x831985C8..0x831985DC)
	// 831985C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831985CC: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 831985D0: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831985D4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831985D8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831985DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831985DC size=20
    let mut pc: u32 = 0x831985DC;
    'dispatch: loop {
        match pc {
            0x831985DC => {
    //   block [0x831985DC..0x831985F0)
	// 831985DC: 88CB0016  lbz r6, 0x16(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(22 as u32) ) } as u64;
	// 831985E0: 88AB0015  lbz r5, 0x15(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 831985E4: 888B0014  lbz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 831985E8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831985EC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831985F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831985F0 size=4
    let mut pc: u32 = 0x831985F0;
    'dispatch: loop {
        match pc {
            0x831985F0 => {
    //   block [0x831985F0..0x831985F4)
	// 831985F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831985F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831985F8 size=20
    let mut pc: u32 = 0x831985F8;
    'dispatch: loop {
        match pc {
            0x831985F8 => {
    //   block [0x831985F8..0x8319860C)
	// 831985F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831985FC: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 83198600: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83198604: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83198608: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8319860C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8319860C size=20
    let mut pc: u32 = 0x8319860C;
    'dispatch: loop {
        match pc {
            0x8319860C => {
    //   block [0x8319860C..0x83198620)
	// 8319860C: 88CB0016  lbz r6, 0x16(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(22 as u32) ) } as u64;
	// 83198610: 88AB0015  lbz r5, 0x15(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 83198614: 888B0014  lbz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83198618: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8319861C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83198620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83198620 size=4
    let mut pc: u32 = 0x83198620;
    'dispatch: loop {
        match pc {
            0x83198620 => {
    //   block [0x83198620..0x83198624)
	// 83198620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83198628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83198628 size=4
    let mut pc: u32 = 0x83198628;
    'dispatch: loop {
        match pc {
            0x83198628 => {
    //   block [0x83198628..0x8319862C)
	// 83198628: 4BFFFE60  b 0x83198488
	sub_83198488(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83198630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83198630 size=84
    let mut pc: u32 = 0x83198630;
    'dispatch: loop {
        match pc {
            0x83198630 => {
    //   block [0x83198630..0x83198684)
	// 83198630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83198634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83198638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319863C: 4BFFFE95  bl 0x831984d0
	ctx.lr = 0x83198640;
	sub_831984D0(ctx, base);
	// 83198640: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83198644: 409A0014  bne cr6, 0x83198658
	if !ctx.cr[6].eq {
	pc = 0x83198658; continue 'dispatch;
	}
	// 83198648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8319864C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83198650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83198654: 4E800020  blr
	return;
	// 83198658: 4BFFFEB9  bl 0x83198510
	ctx.lr = 0x8319865C;
	sub_83198510(ctx, base);
	// 8319865C: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83198660: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83198664: 814B9800  lwz r10, -0x6800(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26624 as u32) ) } as u64;
	// 83198668: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8319866C: 914B9800  stw r10, -0x6800(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26624 as u32), ctx.r[10].u32 ) };
	// 83198670: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83198674: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83198678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319867C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83198680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83198688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83198688 size=8
    let mut pc: u32 = 0x83198688;
    'dispatch: loop {
        match pc {
            0x83198688 => {
    //   block [0x83198688..0x83198690)
	// 83198688: 38600011  li r3, 0x11
	ctx.r[3].s64 = 17;
	// 8319868C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83198690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83198690 size=20
    let mut pc: u32 = 0x83198690;
    'dispatch: loop {
        match pc {
            0x83198690 => {
    //   block [0x83198690..0x831986A4)
	// 83198690: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83198694: 3D408340  lis r10, -0x7cc0
	ctx.r[10].s64 = -2092957696;
	// 83198698: 396BBF78  addi r11, r11, -0x4088
	ctx.r[11].s64 = ctx.r[11].s64 + -16520;
	// 8319869C: 916A36D8  stw r11, 0x36d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(14040 as u32), ctx.r[11].u32 ) };
	// 831986A0: 4BFFF388  b 0x83197a28
	sub_83197A28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831986A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831986A8 size=4
    let mut pc: u32 = 0x831986A8;
    'dispatch: loop {
        match pc {
            0x831986A8 => {
    //   block [0x831986A8..0x831986AC)
	// 831986A8: 4BFF2668  b 0x8318ad10
	sub_8318AD10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831986B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831986B0 size=4
    let mut pc: u32 = 0x831986B0;
    'dispatch: loop {
        match pc {
            0x831986B0 => {
    //   block [0x831986B0..0x831986B4)
	// 831986B0: 4BFF2570  b 0x8318ac20
	sub_8318AC20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831986B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831986B8 size=64
    let mut pc: u32 = 0x831986B8;
    'dispatch: loop {
        match pc {
            0x831986B8 => {
    //   block [0x831986B8..0x831986F8)
	// 831986B8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831986BC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 831986C0: 396BBB50  addi r11, r11, -0x44b0
	ctx.r[11].s64 = ctx.r[11].s64 + -17584;
	// 831986C4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831986C8: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831986CC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831986D0: 7D084850  subf r8, r8, r9
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 831986D4: 419A0014  beq cr6, 0x831986e8
	if ctx.cr[6].eq {
	pc = 0x831986E8; continue 'dispatch;
	}
	// 831986D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831986DC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831986E0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 831986E4: 419AFFE0  beq cr6, 0x831986c4
	if ctx.cr[6].eq {
	pc = 0x831986C4; continue 'dispatch;
	}
	// 831986E8: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 831986EC: 419A000C  beq cr6, 0x831986f8
	if ctx.cr[6].eq {
		sub_831986F8(ctx, base);
		return;
	}
	// 831986F0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831986F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831986F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831986F8 size=28
    let mut pc: u32 = 0x831986F8;
    'dispatch: loop {
        match pc {
            0x831986F8 => {
    //   block [0x831986F8..0x83198714)
	// 831986F8: 2B041530  cmplwi cr6, r4, 0x1530
	ctx.cr[6].compare_u32(ctx.r[4].u32, 5424 as u32, &mut ctx.xer);
	// 831986FC: 409AFFF4  bne cr6, 0x831986f0
	if !ctx.cr[6].eq {
		sub_831986B8(ctx, base);
		return;
	}
	// 83198700: 3965FF80  addi r11, r5, -0x80
	ctx.r[11].s64 = ctx.r[5].s64 + -128;
	// 83198704: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83198708: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8319870C: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 83198710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83198718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83198718 size=712
    let mut pc: u32 = 0x83198718;
    'dispatch: loop {
        match pc {
            0x83198718 => {
    //   block [0x83198718..0x831989E0)
	// 83198718: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8319871C: 38800240  li r4, 0x240
	ctx.r[4].s64 = 576;
	// 83198720: 38AB43A0  addi r5, r11, 0x43a0
	ctx.r[5].s64 = ctx.r[11].s64 + 17312;
	// 83198724: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 83198728: 39650640  addi r11, r5, 0x640
	ctx.r[11].s64 = ctx.r[5].s64 + 1600;
	// 8319872C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 83198730: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83198734: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83198738: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8319873C: 4200FFF8  bdnz 0x83198734
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83198734; continue 'dispatch;
	}
	// 83198740: 39650640  addi r11, r5, 0x640
	ctx.r[11].s64 = ctx.r[5].s64 + 1600;
	// 83198744: 3900023B  li r8, 0x23b
	ctx.r[8].s64 = 571;
	// 83198748: 39450640  addi r10, r5, 0x640
	ctx.r[10].s64 = ctx.r[5].s64 + 1600;
	// 8319874C: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 83198750: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 83198754: 394A0024  addi r10, r10, 0x24
	ctx.r[10].s64 = ctx.r[10].s64 + 36;
	// 83198758: B10B0020  sth r8, 0x20(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[8].u16 ) };
	// 8319875C: B10B0022  sth r8, 0x22(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(34 as u32), ctx.r[8].u16 ) };
	// 83198760: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83198764: B0EA0000  sth r7, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 83198768: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8319876C: 4200FFF8  bdnz 0x83198764
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83198764; continue 'dispatch;
	}
	// 83198770: 39650640  addi r11, r5, 0x640
	ctx.r[11].s64 = ctx.r[5].s64 + 1600;
	// 83198774: 3900022B  li r8, 0x22b
	ctx.r[8].s64 = 555;
	// 83198778: 39450640  addi r10, r5, 0x640
	ctx.r[10].s64 = ctx.r[5].s64 + 1600;
	// 8319877C: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 83198780: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83198784: 394A0040  addi r10, r10, 0x40
	ctx.r[10].s64 = ctx.r[10].s64 + 64;
	// 83198788: B10B003C  sth r8, 0x3c(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[8].u16 ) };
	// 8319878C: B10B003E  sth r8, 0x3e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(62 as u32), ctx.r[8].u16 ) };
	// 83198790: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83198794: B0EA0000  sth r7, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 83198798: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8319879C: 4200FFF8  bdnz 0x83198794
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83198794; continue 'dispatch;
	}
	// 831987A0: 39650640  addi r11, r5, 0x640
	ctx.r[11].s64 = ctx.r[5].s64 + 1600;
	// 831987A4: 39400021  li r10, 0x21
	ctx.r[10].s64 = 33;
	// 831987A8: 396B0060  addi r11, r11, 0x60
	ctx.r[11].s64 = ctx.r[11].s64 + 96;
	// 831987AC: 55492036  slwi r9, r10, 4
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831987B0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 831987B4: 6128440D  ori r8, r9, 0x440d
	ctx.r[8].u64 = ctx.r[9].u64 | 17421;
	// 831987B8: 6129040C  ori r9, r9, 0x40c
	ctx.r[9].u64 = ctx.r[9].u64 | 1036;
	// 831987BC: 2F0A0016  cmpwi cr6, r10, 0x16
	ctx.cr[6].compare_i32(ctx.r[10].s32, 22, &mut ctx.xer);
	// 831987C0: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831987C4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831987C8: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 831987CC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831987D0: 4098FFDC  bge cr6, 0x831987ac
	if !ctx.cr[6].lt {
	pc = 0x831987AC; continue 'dispatch;
	}
	// 831987D4: 39400015  li r10, 0x15
	ctx.r[10].s64 = 21;
	// 831987D8: 55492036  slwi r9, r10, 4
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831987DC: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 831987E0: 6127440C  ori r7, r9, 0x440c
	ctx.r[7].u64 = ctx.r[9].u64 | 17420;
	// 831987E4: 6126040B  ori r6, r9, 0x40b
	ctx.r[6].u64 = ctx.r[9].u64 | 1035;
	// 831987E8: 7CE90734  extsh r9, r7
	ctx.r[9].s64 = ctx.r[7].s16 as i64;
	// 831987EC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831987F0: 7CC70734  extsh r7, r6
	ctx.r[7].s64 = ctx.r[6].s16 as i64;
	// 831987F4: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 831987F8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 831987FC: B1280000  sth r9, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83198800: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83198804: B1280002  sth r9, 2(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 83198808: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 8319880C: B0E60000  sth r7, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 83198810: B0E60002  sth r7, 2(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(2 as u32), ctx.r[7].u16 ) };
	// 83198814: 4098FFC4  bge cr6, 0x831987d8
	if !ctx.cr[6].lt {
	pc = 0x831987D8; continue 'dispatch;
	}
	// 83198818: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 8319881C: 54C92036  slwi r9, r6, 4
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83198820: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83198824: 6127440A  ori r7, r9, 0x440a
	ctx.r[7].u64 = ctx.r[9].u64 | 17418;
	// 83198828: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8319882C: 7CE70734  extsh r7, r7
	ctx.r[7].s64 = ctx.r[7].s16 as i64;
	// 83198830: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83198834: B0EA0000  sth r7, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 83198838: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8319883C: 4200FFF8  bdnz 0x83198834
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83198834; continue 'dispatch;
	}
	// 83198840: 612A0409  ori r10, r9, 0x409
	ctx.r[10].u64 = ctx.r[9].u64 | 1033;
	// 83198844: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 83198848: 7D480734  extsh r8, r10
	ctx.r[8].s64 = ctx.r[10].s16 as i64;
	// 8319884C: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 83198850: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 83198854: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83198858: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8319885C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83198860: 4200FFF8  bdnz 0x83198858
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83198858; continue 'dispatch;
	}
	// 83198864: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 83198868: 39690010  addi r11, r9, 0x10
	ctx.r[11].s64 = ctx.r[9].s64 + 16;
	// 8319886C: 2F06000A  cmpwi cr6, r6, 0xa
	ctx.cr[6].compare_i32(ctx.r[6].s32, 10, &mut ctx.xer);
	// 83198870: 4098FFAC  bge cr6, 0x8319881c
	if !ctx.cr[6].lt {
	pc = 0x8319881C; continue 'dispatch;
	}
	// 83198874: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 83198878: 54C92036  slwi r9, r6, 4
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8319887C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83198880: 61274409  ori r7, r9, 0x4409
	ctx.r[7].u64 = ctx.r[9].u64 | 17417;
	// 83198884: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 83198888: 7CE70734  extsh r7, r7
	ctx.r[7].s64 = ctx.r[7].s16 as i64;
	// 8319888C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83198890: B0EA0000  sth r7, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 83198894: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198898: 4200FFF8  bdnz 0x83198890
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83198890; continue 'dispatch;
	}
	// 8319889C: 612A0408  ori r10, r9, 0x408
	ctx.r[10].u64 = ctx.r[9].u64 | 1032;
	// 831988A0: 392B0020  addi r9, r11, 0x20
	ctx.r[9].s64 = ctx.r[11].s64 + 32;
	// 831988A4: 7D480734  extsh r8, r10
	ctx.r[8].s64 = ctx.r[10].s16 as i64;
	// 831988A8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 831988AC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 831988B0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831988B4: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831988B8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831988BC: 4200FFF8  bdnz 0x831988b4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831988B4; continue 'dispatch;
	}
	// 831988C0: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 831988C4: 39690020  addi r11, r9, 0x20
	ctx.r[11].s64 = ctx.r[9].s64 + 32;
	// 831988C8: 2F060008  cmpwi cr6, r6, 8
	ctx.cr[6].compare_i32(ctx.r[6].s32, 8, &mut ctx.xer);
	// 831988CC: 4098FFAC  bge cr6, 0x83198878
	if !ctx.cr[6].lt {
	pc = 0x83198878; continue 'dispatch;
	}
	// 831988D0: B0850000  sth r4, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[4].u16 ) };
	// 831988D4: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 831988D8: B0850002  sth r4, 2(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(2 as u32), ctx.r[4].u16 ) };
	// 831988DC: 39650008  addi r11, r5, 8
	ctx.r[11].s64 = ctx.r[5].s64 + 8;
	// 831988E0: B0850004  sth r4, 4(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[4].u16 ) };
	// 831988E4: B0850006  sth r4, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[4].u16 ) };
	// 831988E8: 55492036  slwi r9, r10, 4
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831988EC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 831988F0: 61284407  ori r8, r9, 0x4407
	ctx.r[8].u64 = ctx.r[9].u64 | 17415;
	// 831988F4: 61290406  ori r9, r9, 0x406
	ctx.r[9].u64 = ctx.r[9].u64 | 1030;
	// 831988F8: 2F0A0006  cmpwi cr6, r10, 6
	ctx.cr[6].compare_i32(ctx.r[10].s32, 6, &mut ctx.xer);
	// 831988FC: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83198900: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83198904: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83198908: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8319890C: 4098FFDC  bge cr6, 0x831988e8
	if !ctx.cr[6].lt {
	pc = 0x831988E8; continue 'dispatch;
	}
	// 83198910: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 83198914: 55492036  slwi r9, r10, 4
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83198918: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8319891C: 61274406  ori r7, r9, 0x4406
	ctx.r[7].u64 = ctx.r[9].u64 | 17414;
	// 83198920: 61260405  ori r6, r9, 0x405
	ctx.r[6].u64 = ctx.r[9].u64 | 1029;
	// 83198924: 7CE90734  extsh r9, r7
	ctx.r[9].s64 = ctx.r[7].s16 as i64;
	// 83198928: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8319892C: 7CC70734  extsh r7, r6
	ctx.r[7].s64 = ctx.r[6].s16 as i64;
	// 83198930: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 83198934: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 83198938: B1280000  sth r9, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 8319893C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83198940: B1280002  sth r9, 2(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 83198944: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 83198948: B0E60000  sth r7, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 8319894C: B0E60002  sth r7, 2(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(2 as u32), ctx.r[7].u16 ) };
	// 83198950: 4098FFC4  bge cr6, 0x83198914
	if !ctx.cr[6].lt {
	pc = 0x83198914; continue 'dispatch;
	}
	// 83198954: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 83198958: 54C92036  slwi r9, r6, 4
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8319895C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83198960: 38EB0008  addi r7, r11, 8
	ctx.r[7].s64 = ctx.r[11].s64 + 8;
	// 83198964: 612B4405  ori r11, r9, 0x4405
	ctx.r[11].u64 = ctx.r[9].u64 | 17413;
	// 83198968: 61280404  ori r8, r9, 0x404
	ctx.r[8].u64 = ctx.r[9].u64 | 1028;
	// 8319896C: 7D690734  extsh r9, r11
	ctx.r[9].s64 = ctx.r[11].s16 as i64;
	// 83198970: 7D080734  extsh r8, r8
	ctx.r[8].s64 = ctx.r[8].s16 as i64;
	// 83198974: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 83198978: 39670008  addi r11, r7, 8
	ctx.r[11].s64 = ctx.r[7].s64 + 8;
	// 8319897C: 2F060002  cmpwi cr6, r6, 2
	ctx.cr[6].compare_i32(ctx.r[6].s32, 2, &mut ctx.xer);
	// 83198980: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83198984: B12A0002  sth r9, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 83198988: B12A0004  sth r9, 4(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 8319898C: B12A0006  sth r9, 6(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 83198990: B1070000  sth r8, 0(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83198994: B1070002  sth r8, 2(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 83198998: B1070004  sth r8, 4(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 8319899C: B1070006  sth r8, 6(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 831989A0: 4098FFB8  bge cr6, 0x83198958
	if !ctx.cr[6].lt {
	pc = 0x83198958; continue 'dispatch;
	}
	// 831989A4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 831989A8: 39004413  li r8, 0x4413
	ctx.r[8].s64 = 17427;
	// 831989AC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 831989B0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831989B4: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831989B8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 831989BC: 4200FFF8  bdnz 0x831989b4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831989B4; continue 'dispatch;
	}
	// 831989C0: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 831989C4: 39200412  li r9, 0x412
	ctx.r[9].s64 = 1042;
	// 831989C8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 831989CC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831989D0: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 831989D4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831989D8: 4200FFF8  bdnz 0x831989d0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831989D0; continue 'dispatch;
	}
	// 831989DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831989E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831989E0 size=820
    let mut pc: u32 = 0x831989E0;
    'dispatch: loop {
        match pc {
            0x831989E0 => {
    //   block [0x831989E0..0x83198D14)
	// 831989E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831989E4: 4800F77D  bl 0x831a8160
	ctx.lr = 0x831989E8;
	sub_831A8130(ctx, base);
	// 831989E8: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 831989EC: 3BC00240  li r30, 0x240
	ctx.r[30].s64 = 576;
	// 831989F0: 396B4360  addi r11, r11, 0x4360
	ctx.r[11].s64 = ctx.r[11].s64 + 17248;
	// 831989F4: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 831989F8: 394B0880  addi r10, r11, 0x880
	ctx.r[10].s64 = ctx.r[11].s64 + 2176;
	// 831989FC: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 83198A00: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83198A04: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83198A08: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198A0C: 4200FFF8  bdnz 0x83198a04
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83198A04; continue 'dispatch;
	}
	// 83198A10: 3940023B  li r10, 0x23b
	ctx.r[10].s64 = 571;
	// 83198A14: 392B0880  addi r9, r11, 0x880
	ctx.r[9].s64 = ctx.r[11].s64 + 2176;
	// 83198A18: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 83198A1C: B14B0890  sth r10, 0x890(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2192 as u32), ctx.r[10].u16 ) };
	// 83198A20: 39490012  addi r10, r9, 0x12
	ctx.r[10].s64 = ctx.r[9].s64 + 18;
	// 83198A24: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 83198A28: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83198A2C: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83198A30: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198A34: 4200FFF8  bdnz 0x83198a2c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83198A2C; continue 'dispatch;
	}
	// 83198A38: 3940022B  li r10, 0x22b
	ctx.r[10].s64 = 555;
	// 83198A3C: 392B0880  addi r9, r11, 0x880
	ctx.r[9].s64 = ctx.r[11].s64 + 2176;
	// 83198A40: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 83198A44: B14B089E  sth r10, 0x89e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2206 as u32), ctx.r[10].u16 ) };
	// 83198A48: 39490020  addi r10, r9, 0x20
	ctx.r[10].s64 = ctx.r[9].s64 + 32;
	// 83198A4C: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 83198A50: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83198A54: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83198A58: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198A5C: 4200FFF8  bdnz 0x83198a54
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83198A54; continue 'dispatch;
	}
	// 83198A60: 394B0880  addi r10, r11, 0x880
	ctx.r[10].s64 = ctx.r[11].s64 + 2176;
	// 83198A64: 39200021  li r9, 0x21
	ctx.r[9].s64 = 33;
	// 83198A68: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	// 83198A6C: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 83198A70: 51282036  rlwimi r8, r9, 4, 0, 0x1b
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(4) as u64) & 0x00000000FFFFFFF0) | (ctx.r[8].u64 & 0xFFFFFFFF0000000F);
	// 83198A74: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 83198A78: 2F090016  cmpwi cr6, r9, 0x16
	ctx.cr[6].compare_i32(ctx.r[9].s32, 22, &mut ctx.xer);
	// 83198A7C: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83198A80: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198A84: 4098FFE8  bge cr6, 0x83198a6c
	if !ctx.cr[6].lt {
	pc = 0x83198A6C; continue 'dispatch;
	}
	// 83198A88: 3920015A  li r9, 0x15a
	ctx.r[9].s64 = 346;
	// 83198A8C: 3900A95B  li r8, -0x56a5
	ctx.r[8].s64 = -22181;
	// 83198A90: 38E0014A  li r7, 0x14a
	ctx.r[7].s64 = 330;
	// 83198A94: 38C0A94B  li r6, -0x56b5
	ctx.r[6].s64 = -22197;
	// 83198A98: 38A0013A  li r5, 0x13a
	ctx.r[5].s64 = 314;
	// 83198A9C: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83198AA0: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198AA4: 3880A93B  li r4, -0x56c5
	ctx.r[4].s64 = -22213;
	// 83198AA8: 3860012A  li r3, 0x12a
	ctx.r[3].s64 = 298;
	// 83198AAC: 3BE0A92B  li r31, -0x56d5
	ctx.r[31].s64 = -22229;
	// 83198AB0: 3BA0011A  li r29, 0x11a
	ctx.r[29].s64 = 282;
	// 83198AB4: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83198AB8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198ABC: 3B80A91B  li r28, -0x56e5
	ctx.r[28].s64 = -22245;
	// 83198AC0: 3B60010A  li r27, 0x10a
	ctx.r[27].s64 = 266;
	// 83198AC4: 3B40A90B  li r26, -0x56f5
	ctx.r[26].s64 = -22261;
	// 83198AC8: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 83198ACC: B0EA0000  sth r7, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 83198AD0: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198AD4: B0CA0000  sth r6, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 83198AD8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198ADC: 38C0A809  li r6, -0x57f7
	ctx.r[6].s64 = -22519;
	// 83198AE0: B0AA0000  sth r5, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 83198AE4: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198AE8: 38A0880A  li r5, -0x77f6
	ctx.r[5].s64 = -30710;
	// 83198AEC: B08A0000  sth r4, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u16 ) };
	// 83198AF0: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198AF4: 3880A00B  li r4, -0x5ff5
	ctx.r[4].s64 = -24565;
	// 83198AF8: B06A0000  sth r3, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 83198AFC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198B00: B3EA0000  sth r31, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[31].u16 ) };
	// 83198B04: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198B08: B3AA0000  sth r29, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 83198B0C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198B10: B38A0000  sth r28, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[28].u16 ) };
	// 83198B14: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198B18: B36A0000  sth r27, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[27].u16 ) };
	// 83198B1C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198B20: 390A0002  addi r8, r10, 2
	ctx.r[8].s64 = ctx.r[10].s64 + 2;
	// 83198B24: B34A0000  sth r26, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 83198B28: 552A2036  slwi r10, r9, 4
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83198B2C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 83198B30: 615F0008  ori r31, r10, 8
	ctx.r[31].u64 = ctx.r[10].u64 | 8;
	// 83198B34: 7D432378  or r3, r10, r4
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[4].u64;
	// 83198B38: 7D472B78  or r7, r10, r5
	ctx.r[7].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 83198B3C: 7D4A3378  or r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[6].u64;
	// 83198B40: 7CE70734  extsh r7, r7
	ctx.r[7].s64 = ctx.r[7].s16 as i64;
	// 83198B44: B3E80000  sth r31, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[31].u16 ) };
	// 83198B48: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198B4C: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 83198B50: 2F09000A  cmpwi cr6, r9, 0xa
	ctx.cr[6].compare_i32(ctx.r[9].s32, 10, &mut ctx.xer);
	// 83198B54: B0680000  sth r3, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 83198B58: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198B5C: B0E80000  sth r7, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 83198B60: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198B64: B0E80000  sth r7, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 83198B68: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198B6C: B1480000  sth r10, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83198B70: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198B74: B1480000  sth r10, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83198B78: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198B7C: B1480000  sth r10, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83198B80: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198B84: B1480000  sth r10, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83198B88: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198B8C: 4098FF9C  bge cr6, 0x83198b28
	if !ctx.cr[6].lt {
	pc = 0x83198B28; continue 'dispatch;
	}
	// 83198B90: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 83198B94: 3BE0A00A  li r31, -0x5ff6
	ctx.r[31].s64 = -24566;
	// 83198B98: 38608809  li r3, -0x77f7
	ctx.r[3].s64 = -30711;
	// 83198B9C: 3880A808  li r4, -0x57f8
	ctx.r[4].s64 = -22520;
	// 83198BA0: 54EA2036  slwi r10, r7, 4
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83198BA4: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 83198BA8: 61490007  ori r9, r10, 7
	ctx.r[9].u64 = ctx.r[10].u64 | 7;
	// 83198BAC: 7D45FB78  or r5, r10, r31
	ctx.r[5].u64 = ctx.r[10].u64 | ctx.r[31].u64;
	// 83198BB0: 7D260734  extsh r6, r9
	ctx.r[6].s64 = ctx.r[9].s16 as i64;
	// 83198BB4: 7CA50734  extsh r5, r5
	ctx.r[5].s64 = ctx.r[5].s16 as i64;
	// 83198BB8: 7D491B78  or r9, r10, r3
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[3].u64;
	// 83198BBC: 7D4A2378  or r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[4].u64;
	// 83198BC0: 7D290734  extsh r9, r9
	ctx.r[9].s64 = ctx.r[9].s16 as i64;
	// 83198BC4: B0C80000  sth r6, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 83198BC8: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198BCC: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 83198BD0: 2F070008  cmpwi cr6, r7, 8
	ctx.cr[6].compare_i32(ctx.r[7].s32, 8, &mut ctx.xer);
	// 83198BD4: B0C80000  sth r6, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 83198BD8: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198BDC: B0A80000  sth r5, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 83198BE0: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198BE4: B0A80000  sth r5, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 83198BE8: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198BEC: B1280000  sth r9, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83198BF0: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198BF4: B1280000  sth r9, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83198BF8: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198BFC: B1280000  sth r9, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83198C00: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198C04: B1280000  sth r9, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83198C08: 39280002  addi r9, r8, 2
	ctx.r[9].s64 = ctx.r[8].s64 + 2;
	// 83198C0C: B1490000  sth r10, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83198C10: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 83198C14: B1490000  sth r10, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83198C18: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 83198C1C: B1490000  sth r10, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83198C20: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 83198C24: B1490000  sth r10, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83198C28: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 83198C2C: B1490000  sth r10, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83198C30: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 83198C34: B1490000  sth r10, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83198C38: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 83198C3C: B1490000  sth r10, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83198C40: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 83198C44: 39090002  addi r8, r9, 2
	ctx.r[8].s64 = ctx.r[9].s64 + 2;
	// 83198C48: B1490000  sth r10, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83198C4C: 4098FF54  bge cr6, 0x83198ba0
	if !ctx.cr[6].lt {
	pc = 0x83198BA0; continue 'dispatch;
	}
	// 83198C50: 39400075  li r10, 0x75
	ctx.r[10].s64 = 117;
	// 83198C54: B3CB0000  sth r30, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 83198C58: B3CB0002  sth r30, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[30].u16 ) };
	// 83198C5C: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 83198C60: 39400065  li r10, 0x65
	ctx.r[10].s64 = 101;
	// 83198C64: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 83198C68: 39400054  li r10, 0x54
	ctx.r[10].s64 = 84;
	// 83198C6C: B14B0008  sth r10, 8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u16 ) };
	// 83198C70: 3940A855  li r10, -0x57ab
	ctx.r[10].s64 = -22443;
	// 83198C74: B14B000A  sth r10, 0xa(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(10 as u32), ctx.r[10].u16 ) };
	// 83198C78: 39400044  li r10, 0x44
	ctx.r[10].s64 = 68;
	// 83198C7C: B14B000C  sth r10, 0xc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u16 ) };
	// 83198C80: 3940A845  li r10, -0x57bb
	ctx.r[10].s64 = -22459;
	// 83198C84: B14B000E  sth r10, 0xe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[10].u16 ) };
	// 83198C88: 39400033  li r10, 0x33
	ctx.r[10].s64 = 51;
	// 83198C8C: B14B0010  sth r10, 0x10(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u16 ) };
	// 83198C90: 39408835  li r10, -0x77cb
	ctx.r[10].s64 = -30667;
	// 83198C94: B14B0012  sth r10, 0x12(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(18 as u32), ctx.r[10].u16 ) };
	// 83198C98: 3940A834  li r10, -0x57cc
	ctx.r[10].s64 = -22476;
	// 83198C9C: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 83198CA0: B14B0016  sth r10, 0x16(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(22 as u32), ctx.r[10].u16 ) };
	// 83198CA4: 39400023  li r10, 0x23
	ctx.r[10].s64 = 35;
	// 83198CA8: B14B0018  sth r10, 0x18(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u16 ) };
	// 83198CAC: 39408825  li r10, -0x77db
	ctx.r[10].s64 = -30683;
	// 83198CB0: B14B001A  sth r10, 0x1a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(26 as u32), ctx.r[10].u16 ) };
	// 83198CB4: 3940A824  li r10, -0x57dc
	ctx.r[10].s64 = -22492;
	// 83198CB8: B14B001C  sth r10, 0x1c(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u16 ) };
	// 83198CBC: B14B001E  sth r10, 0x1e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(30 as u32), ctx.r[10].u16 ) };
	// 83198CC0: 39400011  li r10, 0x11
	ctx.r[10].s64 = 17;
	// 83198CC4: B14B0020  sth r10, 0x20(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u16 ) };
	// 83198CC8: B14B0022  sth r10, 0x22(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(34 as u32), ctx.r[10].u16 ) };
	// 83198CCC: 3940A014  li r10, -0x5fec
	ctx.r[10].s64 = -24556;
	// 83198CD0: B14B0024  sth r10, 0x24(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[10].u16 ) };
	// 83198CD4: B14B0026  sth r10, 0x26(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(38 as u32), ctx.r[10].u16 ) };
	// 83198CD8: 39408813  li r10, -0x77ed
	ctx.r[10].s64 = -30701;
	// 83198CDC: B14B0028  sth r10, 0x28(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[10].u16 ) };
	// 83198CE0: B14B002A  sth r10, 0x2a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(42 as u32), ctx.r[10].u16 ) };
	// 83198CE4: B14B002C  sth r10, 0x2c(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[10].u16 ) };
	// 83198CE8: B14B002E  sth r10, 0x2e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(46 as u32), ctx.r[10].u16 ) };
	// 83198CEC: 3940A812  li r10, -0x57ee
	ctx.r[10].s64 = -22510;
	// 83198CF0: B14B0030  sth r10, 0x30(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u16 ) };
	// 83198CF4: B14B0032  sth r10, 0x32(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(50 as u32), ctx.r[10].u16 ) };
	// 83198CF8: B14B0034  sth r10, 0x34(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[10].u16 ) };
	// 83198CFC: B14B0036  sth r10, 0x36(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(54 as u32), ctx.r[10].u16 ) };
	// 83198D00: B14B0038  sth r10, 0x38(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[10].u16 ) };
	// 83198D04: B14B003A  sth r10, 0x3a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(58 as u32), ctx.r[10].u16 ) };
	// 83198D08: B14B003C  sth r10, 0x3c(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[10].u16 ) };
	// 83198D0C: B14B003E  sth r10, 0x3e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(62 as u32), ctx.r[10].u16 ) };
	// 83198D10: 4800F4A0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83198D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83198D18 size=840
    let mut pc: u32 = 0x83198D18;
    'dispatch: loop {
        match pc {
            0x83198D18 => {
    //   block [0x83198D18..0x83199060)
	// 83198D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83198D1C: 4800F439  bl 0x831a8154
	ctx.lr = 0x83198D20;
	sub_831A8130(ctx, base);
	// 83198D20: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83198D24: 3B400240  li r26, 0x240
	ctx.r[26].s64 = 576;
	// 83198D28: 396B4520  addi r11, r11, 0x4520
	ctx.r[11].s64 = ctx.r[11].s64 + 17696;
	// 83198D2C: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 83198D30: 394BFBC0  addi r10, r11, -0x440
	ctx.r[10].s64 = ctx.r[11].s64 + -1088;
	// 83198D34: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 83198D38: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83198D3C: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83198D40: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198D44: 4200FFF8  bdnz 0x83198d3c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83198D3C; continue 'dispatch;
	}
	// 83198D48: 3940023B  li r10, 0x23b
	ctx.r[10].s64 = 571;
	// 83198D4C: 392BFBC0  addi r9, r11, -0x440
	ctx.r[9].s64 = ctx.r[11].s64 + -1088;
	// 83198D50: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 83198D54: B14BFBD0  sth r10, -0x430(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-1072 as u32), ctx.r[10].u16 ) };
	// 83198D58: 39490012  addi r10, r9, 0x12
	ctx.r[10].s64 = ctx.r[9].s64 + 18;
	// 83198D5C: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 83198D60: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83198D64: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83198D68: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198D6C: 4200FFF8  bdnz 0x83198d64
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83198D64; continue 'dispatch;
	}
	// 83198D70: 3940022B  li r10, 0x22b
	ctx.r[10].s64 = 555;
	// 83198D74: 392BFBC0  addi r9, r11, -0x440
	ctx.r[9].s64 = ctx.r[11].s64 + -1088;
	// 83198D78: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 83198D7C: B14BFBDE  sth r10, -0x422(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-1058 as u32), ctx.r[10].u16 ) };
	// 83198D80: 39490020  addi r10, r9, 0x20
	ctx.r[10].s64 = ctx.r[9].s64 + 32;
	// 83198D84: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 83198D88: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83198D8C: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83198D90: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198D94: 4200FFF8  bdnz 0x83198d8c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83198D8C; continue 'dispatch;
	}
	// 83198D98: 394BFBC0  addi r10, r11, -0x440
	ctx.r[10].s64 = ctx.r[11].s64 + -1088;
	// 83198D9C: 39200021  li r9, 0x21
	ctx.r[9].s64 = 33;
	// 83198DA0: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	// 83198DA4: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 83198DA8: 51282036  rlwimi r8, r9, 4, 0, 0x1b
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(4) as u64) & 0x00000000FFFFFFF0) | (ctx.r[8].u64 & 0xFFFFFFFF0000000F);
	// 83198DAC: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 83198DB0: 2F090016  cmpwi cr6, r9, 0x16
	ctx.cr[6].compare_i32(ctx.r[9].s32, 22, &mut ctx.xer);
	// 83198DB4: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83198DB8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198DBC: 4098FFE8  bge cr6, 0x83198da4
	if !ctx.cr[6].lt {
	pc = 0x83198DA4; continue 'dispatch;
	}
	// 83198DC0: 3860015A  li r3, 0x15a
	ctx.r[3].s64 = 346;
	// 83198DC4: 3880014A  li r4, 0x14a
	ctx.r[4].s64 = 330;
	// 83198DC8: 38A0013A  li r5, 0x13a
	ctx.r[5].s64 = 314;
	// 83198DCC: 38C0012A  li r6, 0x12a
	ctx.r[6].s64 = 298;
	// 83198DD0: 38E0011A  li r7, 0x11a
	ctx.r[7].s64 = 282;
	// 83198DD4: B06A0000  sth r3, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 83198DD8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198DDC: 3900010A  li r8, 0x10a
	ctx.r[8].s64 = 266;
	// 83198DE0: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 83198DE4: 3BE0900B  li r31, -0x6ff5
	ctx.r[31].s64 = -28661;
	// 83198DE8: B06A0000  sth r3, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 83198DEC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198DF0: 3860980B  li r3, -0x67f5
	ctx.r[3].s64 = -26613;
	// 83198DF4: B08A0000  sth r4, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u16 ) };
	// 83198DF8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198DFC: B08A0000  sth r4, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u16 ) };
	// 83198E00: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198E04: 3880B00A  li r4, -0x4ff6
	ctx.r[4].s64 = -20470;
	// 83198E08: B0AA0000  sth r5, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 83198E0C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198E10: B0AA0000  sth r5, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 83198E14: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198E18: 38A0B80A  li r5, -0x47f6
	ctx.r[5].s64 = -18422;
	// 83198E1C: B0CA0000  sth r6, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 83198E20: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198E24: B0CA0000  sth r6, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 83198E28: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198E2C: B0EA0000  sth r7, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 83198E30: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198E34: B0EA0000  sth r7, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 83198E38: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198E3C: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83198E40: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83198E44: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83198E48: 390A0002  addi r8, r10, 2
	ctx.r[8].s64 = ctx.r[10].s64 + 2;
	// 83198E4C: 552A2036  slwi r10, r9, 4
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83198E50: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 83198E54: 61470008  ori r7, r10, 8
	ctx.r[7].u64 = ctx.r[10].u64 | 8;
	// 83198E58: 7D5EFB78  or r30, r10, r31
	ctx.r[30].u64 = ctx.r[10].u64 | ctx.r[31].u64;
	// 83198E5C: 7CE70734  extsh r7, r7
	ctx.r[7].s64 = ctx.r[7].s16 as i64;
	// 83198E60: 7D5C1B78  or r28, r10, r3
	ctx.r[28].u64 = ctx.r[10].u64 | ctx.r[3].u64;
	// 83198E64: 7D5D2378  or r29, r10, r4
	ctx.r[29].u64 = ctx.r[10].u64 | ctx.r[4].u64;
	// 83198E68: 7D4A2B78  or r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 83198E6C: 7FA60734  extsh r6, r29
	ctx.r[6].s64 = ctx.r[29].s16 as i64;
	// 83198E70: B0E80000  sth r7, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 83198E74: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198E78: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 83198E7C: 2F09000A  cmpwi cr6, r9, 0xa
	ctx.cr[6].compare_i32(ctx.r[9].s32, 10, &mut ctx.xer);
	// 83198E80: B0E80000  sth r7, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 83198E84: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198E88: B3C80000  sth r30, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 83198E8C: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198E90: B3880000  sth r28, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[28].u16 ) };
	// 83198E94: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198E98: B0C80000  sth r6, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 83198E9C: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198EA0: B0C80000  sth r6, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 83198EA4: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198EA8: B1480000  sth r10, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83198EAC: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198EB0: B1480000  sth r10, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83198EB4: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198EB8: 4098FF94  bge cr6, 0x83198e4c
	if !ctx.cr[6].lt {
	pc = 0x83198E4C; continue 'dispatch;
	}
	// 83198EBC: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 83198EC0: 3B60A00B  li r27, -0x5ff5
	ctx.r[27].s64 = -24565;
	// 83198EC4: 3B80A80B  li r28, -0x57f5
	ctx.r[28].s64 = -22517;
	// 83198EC8: 3BA0900A  li r29, -0x6ff6
	ctx.r[29].s64 = -28662;
	// 83198ECC: 3BC0980A  li r30, -0x67f6
	ctx.r[30].s64 = -26614;
	// 83198ED0: 3BE0B009  li r31, -0x4ff7
	ctx.r[31].s64 = -20471;
	// 83198ED4: 3860B809  li r3, -0x47f7
	ctx.r[3].s64 = -18423;
	// 83198ED8: 54EA2036  slwi r10, r7, 4
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83198EDC: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 83198EE0: 61490007  ori r9, r10, 7
	ctx.r[9].u64 = ctx.r[10].u64 | 7;
	// 83198EE4: 7D59DB78  or r25, r10, r27
	ctx.r[25].u64 = ctx.r[10].u64 | ctx.r[27].u64;
	// 83198EE8: 7D260734  extsh r6, r9
	ctx.r[6].s64 = ctx.r[9].s16 as i64;
	// 83198EEC: 7D58E378  or r24, r10, r28
	ctx.r[24].u64 = ctx.r[10].u64 | ctx.r[28].u64;
	// 83198EF0: 7D45EB78  or r5, r10, r29
	ctx.r[5].u64 = ctx.r[10].u64 | ctx.r[29].u64;
	// 83198EF4: 7D49F378  or r9, r10, r30
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[30].u64;
	// 83198EF8: 7CA50734  extsh r5, r5
	ctx.r[5].s64 = ctx.r[5].s16 as i64;
	// 83198EFC: B0C80000  sth r6, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 83198F00: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198F04: 7D240734  extsh r4, r9
	ctx.r[4].s64 = ctx.r[9].s16 as i64;
	// 83198F08: 7D57FB78  or r23, r10, r31
	ctx.r[23].u64 = ctx.r[10].u64 | ctx.r[31].u64;
	// 83198F0C: 7D4A1B78  or r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[3].u64;
	// 83198F10: 7EE90734  extsh r9, r23
	ctx.r[9].s64 = ctx.r[23].s16 as i64;
	// 83198F14: B0C80000  sth r6, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 83198F18: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198F1C: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 83198F20: 2F070008  cmpwi cr6, r7, 8
	ctx.cr[6].compare_i32(ctx.r[7].s32, 8, &mut ctx.xer);
	// 83198F24: B3280000  sth r25, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[25].u16 ) };
	// 83198F28: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198F2C: B3080000  sth r24, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[24].u16 ) };
	// 83198F30: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198F34: B0A80000  sth r5, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 83198F38: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198F3C: B0A80000  sth r5, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 83198F40: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198F44: B0880000  sth r4, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[4].u16 ) };
	// 83198F48: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198F4C: B0880000  sth r4, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[4].u16 ) };
	// 83198F50: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198F54: B1280000  sth r9, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83198F58: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198F5C: B1280000  sth r9, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83198F60: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198F64: B1280000  sth r9, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83198F68: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83198F6C: B1280000  sth r9, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83198F70: 39280002  addi r9, r8, 2
	ctx.r[9].s64 = ctx.r[8].s64 + 2;
	// 83198F74: B1490000  sth r10, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83198F78: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 83198F7C: B1490000  sth r10, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83198F80: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 83198F84: B1490000  sth r10, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83198F88: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 83198F8C: 39090002  addi r8, r9, 2
	ctx.r[8].s64 = ctx.r[9].s64 + 2;
	// 83198F90: B1490000  sth r10, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83198F94: 4098FF44  bge cr6, 0x83198ed8
	if !ctx.cr[6].lt {
	pc = 0x83198ED8; continue 'dispatch;
	}
	// 83198F98: 39400075  li r10, 0x75
	ctx.r[10].s64 = 117;
	// 83198F9C: B34B0000  sth r26, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 83198FA0: B34B0002  sth r26, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[26].u16 ) };
	// 83198FA4: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 83198FA8: 39400065  li r10, 0x65
	ctx.r[10].s64 = 101;
	// 83198FAC: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 83198FB0: 39400054  li r10, 0x54
	ctx.r[10].s64 = 84;
	// 83198FB4: B14B0008  sth r10, 8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u16 ) };
	// 83198FB8: B14B000A  sth r10, 0xa(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(10 as u32), ctx.r[10].u16 ) };
	// 83198FBC: 39400044  li r10, 0x44
	ctx.r[10].s64 = 68;
	// 83198FC0: B14B000C  sth r10, 0xc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u16 ) };
	// 83198FC4: B14B000E  sth r10, 0xe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[10].u16 ) };
	// 83198FC8: 39400033  li r10, 0x33
	ctx.r[10].s64 = 51;
	// 83198FCC: B14B0010  sth r10, 0x10(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u16 ) };
	// 83198FD0: B14B0012  sth r10, 0x12(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(18 as u32), ctx.r[10].u16 ) };
	// 83198FD4: 3940B035  li r10, -0x4fcb
	ctx.r[10].s64 = -20427;
	// 83198FD8: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 83198FDC: 3940B835  li r10, -0x47cb
	ctx.r[10].s64 = -18379;
	// 83198FE0: B14B0016  sth r10, 0x16(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(22 as u32), ctx.r[10].u16 ) };
	// 83198FE4: 39400023  li r10, 0x23
	ctx.r[10].s64 = 35;
	// 83198FE8: B14B0018  sth r10, 0x18(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u16 ) };
	// 83198FEC: B14B001A  sth r10, 0x1a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(26 as u32), ctx.r[10].u16 ) };
	// 83198FF0: 3940B025  li r10, -0x4fdb
	ctx.r[10].s64 = -20443;
	// 83198FF4: B14B001C  sth r10, 0x1c(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u16 ) };
	// 83198FF8: 3940B825  li r10, -0x47db
	ctx.r[10].s64 = -18395;
	// 83198FFC: B14B001E  sth r10, 0x1e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(30 as u32), ctx.r[10].u16 ) };
	// 83199000: 39400011  li r10, 0x11
	ctx.r[10].s64 = 17;
	// 83199004: B14B0020  sth r10, 0x20(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u16 ) };
	// 83199008: B14B0022  sth r10, 0x22(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(34 as u32), ctx.r[10].u16 ) };
	// 8319900C: 3940A015  li r10, -0x5feb
	ctx.r[10].s64 = -24555;
	// 83199010: B14B0024  sth r10, 0x24(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[10].u16 ) };
	// 83199014: 3940A815  li r10, -0x57eb
	ctx.r[10].s64 = -22507;
	// 83199018: B14B0026  sth r10, 0x26(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(38 as u32), ctx.r[10].u16 ) };
	// 8319901C: 39409014  li r10, -0x6fec
	ctx.r[10].s64 = -28652;
	// 83199020: B14B0028  sth r10, 0x28(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[10].u16 ) };
	// 83199024: B14B002A  sth r10, 0x2a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(42 as u32), ctx.r[10].u16 ) };
	// 83199028: 39409814  li r10, -0x67ec
	ctx.r[10].s64 = -26604;
	// 8319902C: B14B002C  sth r10, 0x2c(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[10].u16 ) };
	// 83199030: B14B002E  sth r10, 0x2e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(46 as u32), ctx.r[10].u16 ) };
	// 83199034: 3940B013  li r10, -0x4fed
	ctx.r[10].s64 = -20461;
	// 83199038: B14B0030  sth r10, 0x30(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u16 ) };
	// 8319903C: B14B0032  sth r10, 0x32(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(50 as u32), ctx.r[10].u16 ) };
	// 83199040: B14B0034  sth r10, 0x34(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[10].u16 ) };
	// 83199044: B14B0036  sth r10, 0x36(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(54 as u32), ctx.r[10].u16 ) };
	// 83199048: 3940B813  li r10, -0x47ed
	ctx.r[10].s64 = -18413;
	// 8319904C: B14B0038  sth r10, 0x38(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[10].u16 ) };
	// 83199050: B14B003A  sth r10, 0x3a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(58 as u32), ctx.r[10].u16 ) };
	// 83199054: B14B003C  sth r10, 0x3c(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[10].u16 ) };
	// 83199058: B14B003E  sth r10, 0x3e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(62 as u32), ctx.r[10].u16 ) };
	// 8319905C: 4800F148  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83199060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83199060 size=120
    let mut pc: u32 = 0x83199060;
    'dispatch: loop {
        match pc {
            0x83199060 => {
    //   block [0x83199060..0x831990D8)
	// 83199060: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83199064: 39201106  li r9, 0x1106
	ctx.r[9].s64 = 4358;
	// 83199068: 396B4460  addi r11, r11, 0x4460
	ctx.r[11].s64 = ctx.r[11].s64 + 17504;
	// 8319906C: 39400803  li r10, 0x803
	ctx.r[10].s64 = 2051;
	// 83199070: 38E00202  li r7, 0x202
	ctx.r[7].s64 = 514;
	// 83199074: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 83199078: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 8319907C: 39201205  li r9, 0x1205
	ctx.r[9].s64 = 4613;
	// 83199080: B12B0002  sth r9, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 83199084: 39201A05  li r9, 0x1a05
	ctx.r[9].s64 = 6661;
	// 83199088: B12B0004  sth r9, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 8319908C: 39200105  li r9, 0x105
	ctx.r[9].s64 = 261;
	// 83199090: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 83199094: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 83199098: B14B0008  sth r10, 8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u16 ) };
	// 8319909C: B14B000A  sth r10, 0xa(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(10 as u32), ctx.r[10].u16 ) };
	// 831990A0: B14B000C  sth r10, 0xc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u16 ) };
	// 831990A4: B14B000E  sth r10, 0xe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[10].u16 ) };
	// 831990A8: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 831990AC: B0E90000  sth r7, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 831990B0: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 831990B4: 4200FFF8  bdnz 0x831990ac
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831990AC; continue 'dispatch;
	}
	// 831990B8: 39400A01  li r10, 0xa01
	ctx.r[10].s64 = 2561;
	// 831990BC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 831990C0: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 831990C4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831990C8: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 831990CC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831990D0: 4200FFF8  bdnz 0x831990c8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831990C8; continue 'dispatch;
	}
	// 831990D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831990D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831990D8 size=220
    let mut pc: u32 = 0x831990D8;
    'dispatch: loop {
        match pc {
            0x831990D8 => {
    //   block [0x831990D8..0x831991B4)
	// 831990D8: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 831990DC: 39401F00  li r10, 0x1f00
	ctx.r[10].s64 = 7936;
	// 831990E0: 396B41E0  addi r11, r11, 0x41e0
	ctx.r[11].s64 = ctx.r[11].s64 + 16864;
	// 831990E4: 38C01E05  li r6, 0x1e05
	ctx.r[6].s64 = 7685;
	// 831990E8: 38E00105  li r7, 0x105
	ctx.r[7].s64 = 261;
	// 831990EC: 39200804  li r9, 0x804
	ctx.r[9].s64 = 2052;
	// 831990F0: 38800403  li r4, 0x403
	ctx.r[4].s64 = 1027;
	// 831990F4: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 831990F8: 39401106  li r10, 0x1106
	ctx.r[10].s64 = 4358;
	// 831990FC: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 83199100: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 83199104: B14B0002  sth r10, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 83199108: 39401606  li r10, 0x1606
	ctx.r[10].s64 = 5638;
	// 8319910C: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 83199110: 39401A06  li r10, 0x1a06
	ctx.r[10].s64 = 6662;
	// 83199114: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 83199118: 39400A04  li r10, 0xa04
	ctx.r[10].s64 = 2564;
	// 8319911C: B0CB0008  sth r6, 8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[6].u16 ) };
	// 83199120: B0CB000A  sth r6, 0xa(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(10 as u32), ctx.r[6].u16 ) };
	// 83199124: B0EB000C  sth r7, 0xc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[7].u16 ) };
	// 83199128: B0EB000E  sth r7, 0xe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[7].u16 ) };
	// 8319912C: B12B0010  sth r9, 0x10(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[9].u16 ) };
	// 83199130: B12B0012  sth r9, 0x12(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(18 as u32), ctx.r[9].u16 ) };
	// 83199134: B12B0014  sth r9, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[9].u16 ) };
	// 83199138: B12B0016  sth r9, 0x16(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(22 as u32), ctx.r[9].u16 ) };
	// 8319913C: B14B0018  sth r10, 0x18(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u16 ) };
	// 83199140: B14B001A  sth r10, 0x1a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(26 as u32), ctx.r[10].u16 ) };
	// 83199144: B14B001C  sth r10, 0x1c(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u16 ) };
	// 83199148: B14B001E  sth r10, 0x1e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(30 as u32), ctx.r[10].u16 ) };
	// 8319914C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83199150: B0850000  sth r4, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[4].u16 ) };
	// 83199154: 38A50002  addi r5, r5, 2
	ctx.r[5].s64 = ctx.r[5].s64 + 2;
	// 83199158: 4200FFF8  bdnz 0x83199150
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199150; continue 'dispatch;
	}
	// 8319915C: 39000603  li r8, 0x603
	ctx.r[8].s64 = 1539;
	// 83199160: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 83199164: 394B0030  addi r10, r11, 0x30
	ctx.r[10].s64 = ctx.r[11].s64 + 48;
	// 83199168: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8319916C: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83199170: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83199174: 4200FFF8  bdnz 0x8319916c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8319916C; continue 'dispatch;
	}
	// 83199178: 39000C02  li r8, 0xc02
	ctx.r[8].s64 = 3074;
	// 8319917C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83199180: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	// 83199184: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199188: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8319918C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83199190: 4200FFF8  bdnz 0x83199188
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199188; continue 'dispatch;
	}
	// 83199194: 39200E02  li r9, 0xe02
	ctx.r[9].s64 = 3586;
	// 83199198: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8319919C: 396B0060  addi r11, r11, 0x60
	ctx.r[11].s64 = ctx.r[11].s64 + 96;
	// 831991A0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831991A4: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 831991A8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831991AC: 4200FFF8  bdnz 0x831991a4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831991A4; continue 'dispatch;
	}
	// 831991B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831991B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831991B8 size=460
    let mut pc: u32 = 0x831991B8;
    'dispatch: loop {
        match pc {
            0x831991B8 => {
    //   block [0x831991B8..0x83199384)
	// 831991B8: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 831991BC: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 831991C0: 3BE0007F  li r31, 0x7f
	ctx.r[31].s64 = 127;
	// 831991C4: 396B4420  addi r11, r11, 0x4420
	ctx.r[11].s64 = ctx.r[11].s64 + 17440;
	// 831991C8: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 831991CC: 394BFE40  addi r10, r11, -0x1c0
	ctx.r[10].s64 = ctx.r[11].s64 + -448;
	// 831991D0: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 831991D4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831991D8: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831991DC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 831991E0: 4200FFF8  bdnz 0x831991d8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831991D8; continue 'dispatch;
	}
	// 831991E4: 394BFE40  addi r10, r11, -0x1c0
	ctx.r[10].s64 = ctx.r[11].s64 + -448;
	// 831991E8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 831991EC: 3920FFF0  li r9, -0x10
	ctx.r[9].s64 = -16;
	// 831991F0: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	// 831991F4: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 831991F8: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 831991FC: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 83199200: 50E6402E  rlwimi r6, r7, 8, 0, 0x17
	ctx.r[6].u64 = (((ctx.r[7].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[6].u64 & 0xFFFFFFFF000000FF);
	// 83199204: 50E5402E  rlwimi r5, r7, 8, 0, 0x17
	ctx.r[5].u64 = (((ctx.r[7].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[5].u64 & 0xFFFFFFFF000000FF);
	// 83199208: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8319920C: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83199210: 2F09FFF5  cmpwi cr6, r9, -0xb
	ctx.cr[6].compare_i32(ctx.r[9].s32, -11, &mut ctx.xer);
	// 83199214: B0CA0000  sth r6, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 83199218: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8319921C: B0AA0000  sth r5, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 83199220: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83199224: 4099FFD4  ble cr6, 0x831991f8
	if !ctx.cr[6].gt {
	pc = 0x831991F8; continue 'dispatch;
	}
	// 83199228: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8319922C: 3920FFF6  li r9, -0xa
	ctx.r[9].s64 = -10;
	// 83199230: 5507063E  clrlwi r7, r8, 0x18
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 83199234: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 83199238: 5526063E  clrlwi r6, r9, 0x18
	ctx.r[6].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 8319923C: 60E70A00  ori r7, r7, 0xa00
	ctx.r[7].u64 = ctx.r[7].u64 | 2560;
	// 83199240: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83199244: 60C60A00  ori r6, r6, 0xa00
	ctx.r[6].u64 = ctx.r[6].u64 | 2560;
	// 83199248: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 8319924C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83199250: B0E50000  sth r7, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 83199254: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83199258: B0E50002  sth r7, 2(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(2 as u32), ctx.r[7].u16 ) };
	// 8319925C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83199260: B0C40000  sth r6, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 83199264: 2F09FFF8  cmpwi cr6, r9, -8
	ctx.cr[6].compare_i32(ctx.r[9].s32, -8, &mut ctx.xer);
	// 83199268: B0C40002  sth r6, 2(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(2 as u32), ctx.r[6].u16 ) };
	// 8319926C: 4099FFC4  ble cr6, 0x83199230
	if !ctx.cr[6].gt {
	pc = 0x83199230; continue 'dispatch;
	}
	// 83199270: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 83199274: 38C0FFF9  li r6, -7
	ctx.r[6].s64 = -7;
	// 83199278: 54A8063E  clrlwi r8, r5, 0x18
	ctx.r[8].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 8319927C: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 83199280: 61070800  ori r7, r8, 0x800
	ctx.r[7].u64 = ctx.r[8].u64 | 2048;
	// 83199284: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 83199288: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8319928C: B0E90000  sth r7, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 83199290: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 83199294: 4200FFF8  bdnz 0x8319928c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8319928C; continue 'dispatch;
	}
	// 83199298: 54C8063E  clrlwi r8, r6, 0x18
	ctx.r[8].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 8319929C: 392A0010  addi r9, r10, 0x10
	ctx.r[9].s64 = ctx.r[10].s64 + 16;
	// 831992A0: 61070800  ori r7, r8, 0x800
	ctx.r[7].u64 = ctx.r[8].u64 | 2048;
	// 831992A4: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 831992A8: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 831992AC: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 831992B0: B0EA0000  sth r7, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 831992B4: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 831992B8: 4200FFF8  bdnz 0x831992b0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831992B0; continue 'dispatch;
	}
	// 831992BC: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 831992C0: 39490010  addi r10, r9, 0x10
	ctx.r[10].s64 = ctx.r[9].s64 + 16;
	// 831992C4: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 831992C8: 2F06FFFB  cmpwi cr6, r6, -5
	ctx.cr[6].compare_i32(ctx.r[6].s32, -5, &mut ctx.xer);
	// 831992CC: 4099FFAC  ble cr6, 0x83199278
	if !ctx.cr[6].gt {
	pc = 0x83199278; continue 'dispatch;
	}
	// 831992D0: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 831992D4: 38E00704  li r7, 0x704
	ctx.r[7].s64 = 1796;
	// 831992D8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 831992DC: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 831992E0: B0E90000  sth r7, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 831992E4: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 831992E8: 4200FFF8  bdnz 0x831992e0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831992E0; continue 'dispatch;
	}
	// 831992EC: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 831992F0: 390007FC  li r8, 0x7fc
	ctx.r[8].s64 = 2044;
	// 831992F4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 831992F8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831992FC: B10A0000  sth r8, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83199300: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83199304: 4200FFF8  bdnz 0x831992fc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831992FC; continue 'dispatch;
	}
	// 83199308: 38C00503  li r6, 0x503
	ctx.r[6].s64 = 1283;
	// 8319930C: B3EB0000  sth r31, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u16 ) };
	// 83199310: B3EB0002  sth r31, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[31].u16 ) };
	// 83199314: 38800402  li r4, 0x402
	ctx.r[4].s64 = 1026;
	// 83199318: 38A004FE  li r5, 0x4fe
	ctx.r[5].s64 = 1278;
	// 8319931C: 39200301  li r9, 0x301
	ctx.r[9].s64 = 769;
	// 83199320: B0CB0004  sth r6, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u16 ) };
	// 83199324: 38C005FD  li r6, 0x5fd
	ctx.r[6].s64 = 1533;
	// 83199328: 394003FF  li r10, 0x3ff
	ctx.r[10].s64 = 1023;
	// 8319932C: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 83199330: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 83199334: 390B0020  addi r8, r11, 0x20
	ctx.r[8].s64 = ctx.r[11].s64 + 32;
	// 83199338: B0CB0006  sth r6, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 8319933C: B08B0008  sth r4, 8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[4].u16 ) };
	// 83199340: B08B000A  sth r4, 0xa(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(10 as u32), ctx.r[4].u16 ) };
	// 83199344: B0AB000C  sth r5, 0xc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[5].u16 ) };
	// 83199348: B0AB000E  sth r5, 0xe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[5].u16 ) };
	// 8319934C: B12B0010  sth r9, 0x10(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[9].u16 ) };
	// 83199350: B12B0012  sth r9, 0x12(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(18 as u32), ctx.r[9].u16 ) };
	// 83199354: B12B0014  sth r9, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[9].u16 ) };
	// 83199358: B12B0016  sth r9, 0x16(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(22 as u32), ctx.r[9].u16 ) };
	// 8319935C: B14B0018  sth r10, 0x18(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u16 ) };
	// 83199360: B14B001A  sth r10, 0x1a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(26 as u32), ctx.r[10].u16 ) };
	// 83199364: B14B001C  sth r10, 0x1c(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u16 ) };
	// 83199368: B14B001E  sth r10, 0x1e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(30 as u32), ctx.r[10].u16 ) };
	// 8319936C: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 83199370: B0680000  sth r3, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 83199374: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83199378: 4200FFF8  bdnz 0x83199370
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199370; continue 'dispatch;
	}
	// 8319937C: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 83199380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83199388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83199388 size=628
    let mut pc: u32 = 0x83199388;
    'dispatch: loop {
        match pc {
            0x83199388 => {
    //   block [0x83199388..0x831995FC)
	// 83199388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319938C: 4800EDD5  bl 0x831a8160
	ctx.lr = 0x83199390;
	sub_831A8130(ctx, base);
	// 83199390: 3940DB09  li r10, -0x24f7
	ctx.r[10].s64 = -9463;
	// 83199394: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83199398: 3B60E709  li r27, -0x18f7
	ctx.r[27].s64 = -6391;
	// 8319939C: 3B40DF09  li r26, -0x20f7
	ctx.r[26].s64 = -8439;
	// 831993A0: 3B80BA08  li r28, -0x45f8
	ctx.r[28].s64 = -17912;
	// 831993A4: B141FFB0  sth r10, -0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.r[10].u16 ) };
	// 831993A8: 3940FB09  li r10, -0x4f7
	ctx.r[10].s64 = -1271;
	// 831993AC: B1630000  sth r11, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 831993B0: 3BA0B608  li r29, -0x49f8
	ctx.r[29].s64 = -18936;
	// 831993B4: B1630002  sth r11, 2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 831993B8: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 831993BC: 3BC0AE08  li r30, -0x51f8
	ctx.r[30].s64 = -20984;
	// 831993C0: 3BE09E08  li r31, -0x61f8
	ctx.r[31].s64 = -25080;
	// 831993C4: B141FFB2  sth r10, -0x4e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-78 as u32), ctx.r[10].u16 ) };
	// 831993C8: 3940F709  li r10, -0x8f7
	ctx.r[10].s64 = -2295;
	// 831993CC: 38607908  li r3, 0x7908
	ctx.r[3].s64 = 30984;
	// 831993D0: B36B0000  sth r27, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u16 ) };
	// 831993D4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831993D8: 38807508  li r4, 0x7508
	ctx.r[4].s64 = 29960;
	// 831993DC: 38A06D08  li r5, 0x6d08
	ctx.r[5].s64 = 27912;
	// 831993E0: B141FFB4  sth r10, -0x4c(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-76 as u32), ctx.r[10].u16 ) };
	// 831993E4: 3940EF09  li r10, -0x10f7
	ctx.r[10].s64 = -4343;
	// 831993E8: 38C05D08  li r6, 0x5d08
	ctx.r[6].s64 = 23816;
	// 831993EC: 38E0A608  li r7, -0x59f8
	ctx.r[7].s64 = -23032;
	// 831993F0: 39009A08  li r8, -0x65f8
	ctx.r[8].s64 = -26104;
	// 831993F4: 39206508  li r9, 0x6508
	ctx.r[9].s64 = 25864;
	// 831993F8: B141FFB6  sth r10, -0x4a(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-74 as u32), ctx.r[10].u16 ) };
	// 831993FC: 39405908  li r10, 0x5908
	ctx.r[10].s64 = 22792;
	// 83199400: A361FFB0  lhz r27, -0x50(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) } as u64;
	// 83199404: B36B0000  sth r27, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u16 ) };
	// 83199408: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8319940C: A361FFB2  lhz r27, -0x4e(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-78 as u32) ) } as u64;
	// 83199410: B36B0000  sth r27, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u16 ) };
	// 83199414: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83199418: A361FFB4  lhz r27, -0x4c(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-76 as u32) ) } as u64;
	// 8319941C: B36B0000  sth r27, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u16 ) };
	// 83199420: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83199424: A361FFB6  lhz r27, -0x4a(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-74 as u32) ) } as u64;
	// 83199428: B36B0000  sth r27, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u16 ) };
	// 8319942C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83199430: B34B0000  sth r26, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 83199434: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83199438: 3B40EB08  li r26, -0x14f8
	ctx.r[26].s64 = -5368;
	// 8319943C: B38B0000  sth r28, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u16 ) };
	// 83199440: B38B0002  sth r28, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[28].u16 ) };
	// 83199444: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83199448: B341FFB6  sth r26, -0x4a(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-74 as u32), ctx.r[26].u16 ) };
	// 8319944C: B3AB0000  sth r29, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 83199450: B3AB0002  sth r29, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[29].u16 ) };
	// 83199454: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83199458: B3CB0000  sth r30, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 8319945C: B3CB0002  sth r30, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[30].u16 ) };
	// 83199460: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83199464: B3EB0000  sth r31, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u16 ) };
	// 83199468: B3EB0002  sth r31, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[31].u16 ) };
	// 8319946C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83199470: B06B0000  sth r3, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 83199474: B06B0002  sth r3, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[3].u16 ) };
	// 83199478: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8319947C: B08B0000  sth r4, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u16 ) };
	// 83199480: B08B0002  sth r4, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[4].u16 ) };
	// 83199484: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83199488: B0AB0000  sth r5, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 8319948C: B0AB0002  sth r5, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[5].u16 ) };
	// 83199490: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83199494: B0CB0000  sth r6, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 83199498: B0CB0002  sth r6, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[6].u16 ) };
	// 8319949C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831994A0: B0EB0000  sth r7, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 831994A4: B0EB0002  sth r7, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[7].u16 ) };
	// 831994A8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831994AC: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831994B0: B10B0002  sth r8, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 831994B4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831994B8: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 831994BC: B12B0002  sth r9, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 831994C0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831994C4: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 831994C8: B14B0002  sth r10, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 831994CC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831994D0: 3940D708  li r10, -0x28f8
	ctx.r[10].s64 = -10488;
	// 831994D4: B141FFB4  sth r10, -0x4c(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-76 as u32), ctx.r[10].u16 ) };
	// 831994D8: 3B60CF08  li r27, -0x30f8
	ctx.r[27].s64 = -12536;
	// 831994DC: B141FFB2  sth r10, -0x4e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-78 as u32), ctx.r[10].u16 ) };
	// 831994E0: 3940F308  li r10, -0xcf8
	ctx.r[10].s64 = -3320;
	// 831994E4: B34B0000  sth r26, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 831994E8: 575A043E  clrlwi r26, r26, 0x10
	ctx.r[26].u64 = ctx.r[26].u32 as u64 & 0x0000FFFFu64;
	// 831994EC: 3B80AA08  li r28, -0x55f8
	ctx.r[28].s64 = -22008;
	// 831994F0: 3BA09608  li r29, -0x69f8
	ctx.r[29].s64 = -27128;
	// 831994F4: 3BC0B208  li r30, -0x4df8
	ctx.r[30].s64 = -19960;
	// 831994F8: B361FFBA  sth r27, -0x46(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-70 as u32), ctx.r[27].u16 ) };
	// 831994FC: B141FFB0  sth r10, -0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.r[10].u16 ) };
	// 83199500: 3BE08E08  li r31, -0x71f8
	ctx.r[31].s64 = -29176;
	// 83199504: B141FFB8  sth r10, -0x48(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.r[10].u16 ) };
	// 83199508: 38606908  li r3, 0x6908
	ctx.r[3].s64 = 26888;
	// 8319950C: B34B0002  sth r26, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[26].u16 ) };
	// 83199510: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83199514: 38805508  li r4, 0x5508
	ctx.r[4].s64 = 21768;
	// 83199518: 38A07108  li r5, 0x7108
	ctx.r[5].s64 = 28936;
	// 8319951C: 38C04D08  li r6, 0x4d08
	ctx.r[6].s64 = 19720;
	// 83199520: 38E0E308  li r7, -0x1cf8
	ctx.r[7].s64 = -7416;
	// 83199524: 3900D308  li r8, -0x2cf8
	ctx.r[8].s64 = -11512;
	// 83199528: 3920CB08  li r9, -0x34f8
	ctx.r[9].s64 = -13560;
	// 8319952C: 3940C708  li r10, -0x38f8
	ctx.r[10].s64 = -14584;
	// 83199530: A341FFB4  lhz r26, -0x4c(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-76 as u32) ) } as u64;
	// 83199534: B34B0000  sth r26, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 83199538: A341FFB2  lhz r26, -0x4e(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-78 as u32) ) } as u64;
	// 8319953C: B34B0002  sth r26, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[26].u16 ) };
	// 83199540: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83199544: A341FFB0  lhz r26, -0x50(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) } as u64;
	// 83199548: B34B0000  sth r26, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 8319954C: A341FFB8  lhz r26, -0x48(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) } as u64;
	// 83199550: B34B0002  sth r26, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[26].u16 ) };
	// 83199554: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83199558: 577A043E  clrlwi r26, r27, 0x10
	ctx.r[26].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	// 8319955C: B36B0002  sth r27, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[27].u16 ) };
	// 83199560: B34B0000  sth r26, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 83199564: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83199568: B38B0000  sth r28, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u16 ) };
	// 8319956C: B38B0002  sth r28, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[28].u16 ) };
	// 83199570: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83199574: B3AB0000  sth r29, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 83199578: B3AB0002  sth r29, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[29].u16 ) };
	// 8319957C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83199580: B3CB0000  sth r30, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 83199584: B3CB0002  sth r30, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[30].u16 ) };
	// 83199588: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8319958C: B3EB0000  sth r31, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u16 ) };
	// 83199590: B3EB0002  sth r31, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[31].u16 ) };
	// 83199594: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83199598: B06B0000  sth r3, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 8319959C: B06B0002  sth r3, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[3].u16 ) };
	// 831995A0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831995A4: B08B0000  sth r4, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u16 ) };
	// 831995A8: B08B0002  sth r4, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[4].u16 ) };
	// 831995AC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831995B0: B0AB0000  sth r5, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 831995B4: B0AB0002  sth r5, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[5].u16 ) };
	// 831995B8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831995BC: B0CB0000  sth r6, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 831995C0: B0CB0002  sth r6, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[6].u16 ) };
	// 831995C4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831995C8: B0EB0000  sth r7, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 831995CC: B0EB0002  sth r7, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[7].u16 ) };
	// 831995D0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831995D4: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831995D8: B10B0002  sth r8, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 831995DC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831995E0: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 831995E4: B12B0002  sth r9, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 831995E8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831995EC: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 831995F0: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 831995F4: B14B0002  sth r10, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 831995F8: 4800EBB8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83199600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83199600 size=956
    let mut pc: u32 = 0x83199600;
    'dispatch: loop {
        match pc {
            0x83199600 => {
    //   block [0x83199600..0x831999BC)
	// 83199600: 38A09207  li r5, -0x6df9
	ctx.r[5].s64 = -28153;
	// 83199604: 39408A07  li r10, -0x75f9
	ctx.r[10].s64 = -30201;
	// 83199608: 3960A207  li r11, -0x5df9
	ctx.r[11].s64 = -24057;
	// 8319960C: 38C08607  li r6, -0x79f9
	ctx.r[6].s64 = -31225;
	// 83199610: 38E06107  li r7, 0x6107
	ctx.r[7].s64 = 24839;
	// 83199614: B0A1FFF2  sth r5, -0xe(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-14 as u32), ctx.r[5].u16 ) };
	// 83199618: 39005107  li r8, 0x5107
	ctx.r[8].s64 = 20743;
	// 8319961C: B0A1FFF4  sth r5, -0xc(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[5].u16 ) };
	// 83199620: 39204907  li r9, 0x4907
	ctx.r[9].s64 = 18695;
	// 83199624: B141FFF6  sth r10, -0xa(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-10 as u32), ctx.r[10].u16 ) };
	// 83199628: B141FFF8  sth r10, -8(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[10].u16 ) };
	// 8319962C: B141FFFA  sth r10, -6(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-6 as u32), ctx.r[10].u16 ) };
	// 83199630: B1630000  sth r11, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83199634: B1630002  sth r11, 2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 83199638: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8319963C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 83199640: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 83199644: B141FFFC  sth r10, -4(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), ctx.r[10].u16 ) };
	// 83199648: 39404507  li r10, 0x4507
	ctx.r[10].s64 = 17671;
	// 8319964C: B0A1FFF0  sth r5, -0x10(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[5].u16 ) };
	// 83199650: B0AB0000  sth r5, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 83199654: 54A5043E  clrlwi r5, r5, 0x10
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0x0000FFFFu64;
	// 83199658: B0AB0002  sth r5, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[5].u16 ) };
	// 8319965C: A0A1FFF2  lhz r5, -0xe(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-14 as u32) ) } as u64;
	// 83199660: B0AB0004  sth r5, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u16 ) };
	// 83199664: A0A1FFF4  lhz r5, -0xc(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 83199668: B0AB0006  sth r5, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 8319966C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83199670: A0A1FFF6  lhz r5, -0xa(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-10 as u32) ) } as u64;
	// 83199674: B0AB0000  sth r5, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 83199678: A0A1FFF8  lhz r5, -8(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319967C: B0AB0002  sth r5, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[5].u16 ) };
	// 83199680: A0A1FFFA  lhz r5, -6(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-6 as u32) ) } as u64;
	// 83199684: B0AB0004  sth r5, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u16 ) };
	// 83199688: A0A1FFFC  lhz r5, -4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8319968C: B0AB0006  sth r5, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 83199690: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83199694: B0CB0000  sth r6, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 83199698: B0CB0002  sth r6, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[6].u16 ) };
	// 8319969C: B0CB0004  sth r6, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u16 ) };
	// 831996A0: B0CB0006  sth r6, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 831996A4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831996A8: 38C0BE05  li r6, -0x41fb
	ctx.r[6].s64 = -16891;
	// 831996AC: B0EB0000  sth r7, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 831996B0: B0EB0002  sth r7, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[7].u16 ) };
	// 831996B4: B0EB0004  sth r7, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u16 ) };
	// 831996B8: B0EB0006  sth r7, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 831996BC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831996C0: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 831996C4: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831996C8: B10B0002  sth r8, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 831996CC: B10B0004  sth r8, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 831996D0: B10B0006  sth r8, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 831996D4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831996D8: 3900C306  li r8, -0x3cfa
	ctx.r[8].s64 = -15610;
	// 831996DC: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 831996E0: B12B0002  sth r9, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 831996E4: B12B0004  sth r9, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 831996E8: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 831996EC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831996F0: 39202406  li r9, 0x2406
	ctx.r[9].s64 = 9222;
	// 831996F4: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 831996F8: B14B0002  sth r10, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 831996FC: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 83199700: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 83199704: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83199708: 3940FF06  li r10, -0xfa
	ctx.r[10].s64 = -250;
	// 8319970C: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83199710: B14B0002  sth r10, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 83199714: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 83199718: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8319971C: B14B0008  sth r10, 8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u16 ) };
	// 83199720: B14B000A  sth r10, 0xa(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(10 as u32), ctx.r[10].u16 ) };
	// 83199724: B14B000C  sth r10, 0xc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u16 ) };
	// 83199728: B14B000E  sth r10, 0xe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[10].u16 ) };
	// 8319972C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 83199730: 39401806  li r10, 0x1806
	ctx.r[10].s64 = 6150;
	// 83199734: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83199738: B10B0002  sth r8, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 8319973C: B10B0004  sth r8, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 83199740: B10B0006  sth r8, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 83199744: B10B0008  sth r8, 8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[8].u16 ) };
	// 83199748: B10B000A  sth r8, 0xa(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(10 as u32), ctx.r[8].u16 ) };
	// 8319974C: B10B000C  sth r8, 0xc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u16 ) };
	// 83199750: B10B000E  sth r8, 0xe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[8].u16 ) };
	// 83199754: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 83199758: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 8319975C: B12B0002  sth r9, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 83199760: B12B0004  sth r9, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 83199764: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 83199768: B12B0008  sth r9, 8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u16 ) };
	// 8319976C: B12B000A  sth r9, 0xa(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(10 as u32), ctx.r[9].u16 ) };
	// 83199770: B12B000C  sth r9, 0xc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u16 ) };
	// 83199774: B12B000E  sth r9, 0xe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[9].u16 ) };
	// 83199778: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 8319977C: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83199780: B14B0002  sth r10, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 83199784: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 83199788: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8319978C: B14B0008  sth r10, 8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u16 ) };
	// 83199790: B14B000A  sth r10, 0xa(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(10 as u32), ctx.r[10].u16 ) };
	// 83199794: B14B000C  sth r10, 0xc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u16 ) };
	// 83199798: B14B000E  sth r10, 0xe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[10].u16 ) };
	// 8319979C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 831997A0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 831997A4: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 831997A8: B0CA0000  sth r6, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 831997AC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 831997B0: 4200FFF8  bdnz 0x831997a8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831997A8; continue 'dispatch;
	}
	// 831997B4: 394B0020  addi r10, r11, 0x20
	ctx.r[10].s64 = ctx.r[11].s64 + 32;
	// 831997B8: 39008205  li r8, -0x7dfb
	ctx.r[8].s64 = -32251;
	// 831997BC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 831997C0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 831997C4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831997C8: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831997CC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831997D0: 4200FFF8  bdnz 0x831997c8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831997C8; continue 'dispatch;
	}
	// 831997D4: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 831997D8: 39007D05  li r8, 0x7d05
	ctx.r[8].s64 = 32005;
	// 831997DC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 831997E0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 831997E4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831997E8: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831997EC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831997F0: 4200FFF8  bdnz 0x831997e8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831997E8; continue 'dispatch;
	}
	// 831997F4: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 831997F8: 39004105  li r8, 0x4105
	ctx.r[8].s64 = 16645;
	// 831997FC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83199800: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83199804: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199808: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8319980C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83199810: 4200FFF8  bdnz 0x83199808
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199808; continue 'dispatch;
	}
	// 83199814: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 83199818: 39003805  li r8, 0x3805
	ctx.r[8].s64 = 14341;
	// 8319981C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83199820: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83199824: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199828: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8319982C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83199830: 4200FFF8  bdnz 0x83199828
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199828; continue 'dispatch;
	}
	// 83199834: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 83199838: 39003405  li r8, 0x3405
	ctx.r[8].s64 = 13317;
	// 8319983C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83199840: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83199844: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199848: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8319984C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83199850: 4200FFF8  bdnz 0x83199848
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199848; continue 'dispatch;
	}
	// 83199854: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 83199858: 39002C05  li r8, 0x2c05
	ctx.r[8].s64 = 11269;
	// 8319985C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83199860: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83199864: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199868: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8319986C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83199870: 4200FFF8  bdnz 0x83199868
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199868; continue 'dispatch;
	}
	// 83199874: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 83199878: 39001C05  li r8, 0x1c05
	ctx.r[8].s64 = 7173;
	// 8319987C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83199880: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83199884: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199888: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8319988C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83199890: 4200FFF8  bdnz 0x83199888
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199888; continue 'dispatch;
	}
	// 83199894: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 83199898: 39002805  li r8, 0x2805
	ctx.r[8].s64 = 10245;
	// 8319989C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 831998A0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 831998A4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831998A8: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831998AC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831998B0: 4200FFF8  bdnz 0x831998a8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831998A8; continue 'dispatch;
	}
	// 831998B4: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 831998B8: 39001405  li r8, 0x1405
	ctx.r[8].s64 = 5125;
	// 831998BC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 831998C0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 831998C4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831998C8: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831998CC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831998D0: 4200FFF8  bdnz 0x831998c8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831998C8; continue 'dispatch;
	}
	// 831998D4: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 831998D8: 39003005  li r8, 0x3005
	ctx.r[8].s64 = 12293;
	// 831998DC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 831998E0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 831998E4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831998E8: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831998EC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831998F0: 4200FFF8  bdnz 0x831998e8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831998E8; continue 'dispatch;
	}
	// 831998F4: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 831998F8: 39000C05  li r8, 0xc05
	ctx.r[8].s64 = 3077;
	// 831998FC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83199900: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83199904: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199908: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8319990C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83199910: 4200FFF8  bdnz 0x83199908
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199908; continue 'dispatch;
	}
	// 83199914: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 83199918: 39002004  li r8, 0x2004
	ctx.r[8].s64 = 8196;
	// 8319991C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 83199920: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83199924: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199928: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8319992C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83199930: 4200FFF8  bdnz 0x83199928
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199928; continue 'dispatch;
	}
	// 83199934: 394A0040  addi r10, r10, 0x40
	ctx.r[10].s64 = ctx.r[10].s64 + 64;
	// 83199938: 39001004  li r8, 0x1004
	ctx.r[8].s64 = 4100;
	// 8319993C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 83199940: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83199944: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199948: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8319994C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83199950: 4200FFF8  bdnz 0x83199948
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199948; continue 'dispatch;
	}
	// 83199954: 394A0040  addi r10, r10, 0x40
	ctx.r[10].s64 = ctx.r[10].s64 + 64;
	// 83199958: 39000804  li r8, 0x804
	ctx.r[8].s64 = 2052;
	// 8319995C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 83199960: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83199964: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199968: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8319996C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83199970: 4200FFF8  bdnz 0x83199968
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199968; continue 'dispatch;
	}
	// 83199974: 394A0040  addi r10, r10, 0x40
	ctx.r[10].s64 = ctx.r[10].s64 + 64;
	// 83199978: 39000404  li r8, 0x404
	ctx.r[8].s64 = 1028;
	// 8319997C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 83199980: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83199984: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199988: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8319998C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83199990: 4200FFF8  bdnz 0x83199988
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199988; continue 'dispatch;
	}
	// 83199994: 390A0040  addi r8, r10, 0x40
	ctx.r[8].s64 = ctx.r[10].s64 + 64;
	// 83199998: 39203C03  li r9, 0x3c03
	ctx.r[9].s64 = 15363;
	// 8319999C: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 831999A0: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 831999A4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831999A8: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 831999AC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831999B0: 4200FFF8  bdnz 0x831999a8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831999A8; continue 'dispatch;
	}
	// 831999B4: 38680080  addi r3, r8, 0x80
	ctx.r[3].s64 = ctx.r[8].s64 + 128;
	// 831999B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831999C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831999C0 size=232
    let mut pc: u32 = 0x831999C0;
    'dispatch: loop {
        match pc {
            0x831999C0 => {
    //   block [0x831999C0..0x83199AA8)
	// 831999C0: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 831999C4: 39000012  li r8, 0x12
	ctx.r[8].s64 = 18;
	// 831999C8: 396B44A0  addi r11, r11, 0x44a0
	ctx.r[11].s64 = ctx.r[11].s64 + 17568;
	// 831999CC: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 831999D0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 831999D4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831999D8: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 831999DC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831999E0: 4200FFF8  bdnz 0x831999d8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831999D8; continue 'dispatch;
	}
	// 831999E4: 39000022  li r8, 0x22
	ctx.r[8].s64 = 34;
	// 831999E8: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 831999EC: 394B0020  addi r10, r11, 0x20
	ctx.r[10].s64 = ctx.r[11].s64 + 32;
	// 831999F0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831999F4: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 831999F8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831999FC: 4200FFF8  bdnz 0x831999f4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831999F4; continue 'dispatch;
	}
	// 83199A00: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 83199A04: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83199A08: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	// 83199A0C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199A10: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 83199A14: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199A18: 4200FFF8  bdnz 0x83199a10
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199A10; continue 'dispatch;
	}
	// 83199A1C: 39000033  li r8, 0x33
	ctx.r[8].s64 = 51;
	// 83199A20: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83199A24: 394B0050  addi r10, r11, 0x50
	ctx.r[10].s64 = ctx.r[11].s64 + 80;
	// 83199A28: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199A2C: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 83199A30: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199A34: 4200FFF8  bdnz 0x83199a2c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199A2C; continue 'dispatch;
	}
	// 83199A38: 39000043  li r8, 0x43
	ctx.r[8].s64 = 67;
	// 83199A3C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83199A40: 394B0060  addi r10, r11, 0x60
	ctx.r[10].s64 = ctx.r[11].s64 + 96;
	// 83199A44: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199A48: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 83199A4C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199A50: 4200FFF8  bdnz 0x83199a48
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199A48; continue 'dispatch;
	}
	// 83199A54: 39400054  li r10, 0x54
	ctx.r[10].s64 = 84;
	// 83199A58: 994B0070  stb r10, 0x70(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[10].u8 ) };
	// 83199A5C: 994B0071  stb r10, 0x71(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(113 as u32), ctx.r[10].u8 ) };
	// 83199A60: 994B0072  stb r10, 0x72(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(114 as u32), ctx.r[10].u8 ) };
	// 83199A64: 994B0073  stb r10, 0x73(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(115 as u32), ctx.r[10].u8 ) };
	// 83199A68: 994B0074  stb r10, 0x74(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), ctx.r[10].u8 ) };
	// 83199A6C: 994B0075  stb r10, 0x75(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(117 as u32), ctx.r[10].u8 ) };
	// 83199A70: 994B0076  stb r10, 0x76(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(118 as u32), ctx.r[10].u8 ) };
	// 83199A74: 994B0077  stb r10, 0x77(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(119 as u32), ctx.r[10].u8 ) };
	// 83199A78: 39400065  li r10, 0x65
	ctx.r[10].s64 = 101;
	// 83199A7C: 994B0078  stb r10, 0x78(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(120 as u32), ctx.r[10].u8 ) };
	// 83199A80: 994B0079  stb r10, 0x79(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(121 as u32), ctx.r[10].u8 ) };
	// 83199A84: 994B007A  stb r10, 0x7a(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(122 as u32), ctx.r[10].u8 ) };
	// 83199A88: 994B007B  stb r10, 0x7b(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(123 as u32), ctx.r[10].u8 ) };
	// 83199A8C: 39400076  li r10, 0x76
	ctx.r[10].s64 = 118;
	// 83199A90: 994B007C  stb r10, 0x7c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u8 ) };
	// 83199A94: 994B007D  stb r10, 0x7d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(125 as u32), ctx.r[10].u8 ) };
	// 83199A98: 3940FF87  li r10, -0x79
	ctx.r[10].s64 = -121;
	// 83199A9C: 994B007E  stb r10, 0x7e(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(126 as u32), ctx.r[10].u8 ) };
	// 83199AA0: 994B007F  stb r10, 0x7f(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(127 as u32), ctx.r[10].u8 ) };
	// 83199AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83199AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83199AA8 size=208
    let mut pc: u32 = 0x83199AA8;
    'dispatch: loop {
        match pc {
            0x83199AA8 => {
    //   block [0x83199AA8..0x83199B78)
	// 83199AA8: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83199AAC: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 83199AB0: 396B4560  addi r11, r11, 0x4560
	ctx.r[11].s64 = ctx.r[11].s64 + 17760;
	// 83199AB4: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 83199AB8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83199ABC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199AC0: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 83199AC4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199AC8: 4200FFF8  bdnz 0x83199ac0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199AC0; continue 'dispatch;
	}
	// 83199ACC: 39000012  li r8, 0x12
	ctx.r[8].s64 = 18;
	// 83199AD0: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 83199AD4: 394B0020  addi r10, r11, 0x20
	ctx.r[10].s64 = ctx.r[11].s64 + 32;
	// 83199AD8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199ADC: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 83199AE0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199AE4: 4200FFF8  bdnz 0x83199adc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199ADC; continue 'dispatch;
	}
	// 83199AE8: 39000022  li r8, 0x22
	ctx.r[8].s64 = 34;
	// 83199AEC: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 83199AF0: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	// 83199AF4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199AF8: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 83199AFC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199B00: 4200FFF8  bdnz 0x83199af8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199AF8; continue 'dispatch;
	}
	// 83199B04: 39200033  li r9, 0x33
	ctx.r[9].s64 = 51;
	// 83199B08: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 83199B0C: 394B0060  addi r10, r11, 0x60
	ctx.r[10].s64 = ctx.r[11].s64 + 96;
	// 83199B10: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83199B14: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83199B18: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199B1C: 4200FFF8  bdnz 0x83199b14
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199B14; continue 'dispatch;
	}
	// 83199B20: 39400044  li r10, 0x44
	ctx.r[10].s64 = 68;
	// 83199B24: 994B0070  stb r10, 0x70(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[10].u8 ) };
	// 83199B28: 994B0071  stb r10, 0x71(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(113 as u32), ctx.r[10].u8 ) };
	// 83199B2C: 994B0072  stb r10, 0x72(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(114 as u32), ctx.r[10].u8 ) };
	// 83199B30: 994B0073  stb r10, 0x73(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(115 as u32), ctx.r[10].u8 ) };
	// 83199B34: 994B0074  stb r10, 0x74(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), ctx.r[10].u8 ) };
	// 83199B38: 994B0075  stb r10, 0x75(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(117 as u32), ctx.r[10].u8 ) };
	// 83199B3C: 994B0076  stb r10, 0x76(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(118 as u32), ctx.r[10].u8 ) };
	// 83199B40: 994B0077  stb r10, 0x77(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(119 as u32), ctx.r[10].u8 ) };
	// 83199B44: 39400055  li r10, 0x55
	ctx.r[10].s64 = 85;
	// 83199B48: 994B0078  stb r10, 0x78(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(120 as u32), ctx.r[10].u8 ) };
	// 83199B4C: 994B0079  stb r10, 0x79(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(121 as u32), ctx.r[10].u8 ) };
	// 83199B50: 994B007A  stb r10, 0x7a(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(122 as u32), ctx.r[10].u8 ) };
	// 83199B54: 994B007B  stb r10, 0x7b(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(123 as u32), ctx.r[10].u8 ) };
	// 83199B58: 39400066  li r10, 0x66
	ctx.r[10].s64 = 102;
	// 83199B5C: 994B007C  stb r10, 0x7c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u8 ) };
	// 83199B60: 994B007D  stb r10, 0x7d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(125 as u32), ctx.r[10].u8 ) };
	// 83199B64: 39400077  li r10, 0x77
	ctx.r[10].s64 = 119;
	// 83199B68: 994B007E  stb r10, 0x7e(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(126 as u32), ctx.r[10].u8 ) };
	// 83199B6C: 3940FF88  li r10, -0x78
	ctx.r[10].s64 = -120;
	// 83199B70: 994B007F  stb r10, 0x7f(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(127 as u32), ctx.r[10].u8 ) };
	// 83199B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83199B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83199B78 size=316
    let mut pc: u32 = 0x83199B78;
    'dispatch: loop {
        match pc {
            0x83199B78 => {
    //   block [0x83199B78..0x83199CB4)
	// 83199B78: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83199B7C: 39000012  li r8, 0x12
	ctx.r[8].s64 = 18;
	// 83199B80: 396B45E0  addi r11, r11, 0x45e0
	ctx.r[11].s64 = ctx.r[11].s64 + 17888;
	// 83199B84: 39200100  li r9, 0x100
	ctx.r[9].s64 = 256;
	// 83199B88: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83199B8C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199B90: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 83199B94: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199B98: 4200FFF8  bdnz 0x83199b90
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199B90; continue 'dispatch;
	}
	// 83199B9C: 39000022  li r8, 0x22
	ctx.r[8].s64 = 34;
	// 83199BA0: 39200100  li r9, 0x100
	ctx.r[9].s64 = 256;
	// 83199BA4: 394B0100  addi r10, r11, 0x100
	ctx.r[10].s64 = ctx.r[11].s64 + 256;
	// 83199BA8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199BAC: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 83199BB0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199BB4: 4200FFF8  bdnz 0x83199bac
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199BAC; continue 'dispatch;
	}
	// 83199BB8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 83199BBC: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
	// 83199BC0: 394B0200  addi r10, r11, 0x200
	ctx.r[10].s64 = ctx.r[11].s64 + 512;
	// 83199BC4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199BC8: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 83199BCC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199BD0: 4200FFF8  bdnz 0x83199bc8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199BC8; continue 'dispatch;
	}
	// 83199BD4: 39000033  li r8, 0x33
	ctx.r[8].s64 = 51;
	// 83199BD8: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
	// 83199BDC: 394B0280  addi r10, r11, 0x280
	ctx.r[10].s64 = ctx.r[11].s64 + 640;
	// 83199BE0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199BE4: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 83199BE8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199BEC: 4200FFF8  bdnz 0x83199be4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199BE4; continue 'dispatch;
	}
	// 83199BF0: 39000043  li r8, 0x43
	ctx.r[8].s64 = 67;
	// 83199BF4: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
	// 83199BF8: 394B0300  addi r10, r11, 0x300
	ctx.r[10].s64 = ctx.r[11].s64 + 768;
	// 83199BFC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199C00: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 83199C04: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199C08: 4200FFF8  bdnz 0x83199c00
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199C00; continue 'dispatch;
	}
	// 83199C0C: 39200054  li r9, 0x54
	ctx.r[9].s64 = 84;
	// 83199C10: 39000040  li r8, 0x40
	ctx.r[8].s64 = 64;
	// 83199C14: 394B0380  addi r10, r11, 0x380
	ctx.r[10].s64 = ctx.r[11].s64 + 896;
	// 83199C18: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83199C1C: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83199C20: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199C24: 4200FFF8  bdnz 0x83199c1c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199C1C; continue 'dispatch;
	}
	// 83199C28: 39200065  li r9, 0x65
	ctx.r[9].s64 = 101;
	// 83199C2C: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 83199C30: 394B03C0  addi r10, r11, 0x3c0
	ctx.r[10].s64 = ctx.r[11].s64 + 960;
	// 83199C34: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83199C38: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83199C3C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199C40: 4200FFF8  bdnz 0x83199c38
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199C38; continue 'dispatch;
	}
	// 83199C44: 39200076  li r9, 0x76
	ctx.r[9].s64 = 118;
	// 83199C48: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 83199C4C: 394B03E0  addi r10, r11, 0x3e0
	ctx.r[10].s64 = ctx.r[11].s64 + 992;
	// 83199C50: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83199C54: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83199C58: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199C5C: 4200FFF8  bdnz 0x83199c54
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199C54; continue 'dispatch;
	}
	// 83199C60: 3940FF87  li r10, -0x79
	ctx.r[10].s64 = -121;
	// 83199C64: 994B03F0  stb r10, 0x3f0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1008 as u32), ctx.r[10].u8 ) };
	// 83199C68: 994B03F1  stb r10, 0x3f1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1009 as u32), ctx.r[10].u8 ) };
	// 83199C6C: 994B03F2  stb r10, 0x3f2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1010 as u32), ctx.r[10].u8 ) };
	// 83199C70: 994B03F3  stb r10, 0x3f3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1011 as u32), ctx.r[10].u8 ) };
	// 83199C74: 994B03F4  stb r10, 0x3f4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1012 as u32), ctx.r[10].u8 ) };
	// 83199C78: 994B03F5  stb r10, 0x3f5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1013 as u32), ctx.r[10].u8 ) };
	// 83199C7C: 994B03F6  stb r10, 0x3f6(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1014 as u32), ctx.r[10].u8 ) };
	// 83199C80: 994B03F7  stb r10, 0x3f7(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1015 as u32), ctx.r[10].u8 ) };
	// 83199C84: 3940FF98  li r10, -0x68
	ctx.r[10].s64 = -104;
	// 83199C88: 994B03F8  stb r10, 0x3f8(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1016 as u32), ctx.r[10].u8 ) };
	// 83199C8C: 994B03F9  stb r10, 0x3f9(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1017 as u32), ctx.r[10].u8 ) };
	// 83199C90: 994B03FA  stb r10, 0x3fa(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1018 as u32), ctx.r[10].u8 ) };
	// 83199C94: 994B03FB  stb r10, 0x3fb(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1019 as u32), ctx.r[10].u8 ) };
	// 83199C98: 3940FFA9  li r10, -0x57
	ctx.r[10].s64 = -87;
	// 83199C9C: 994B03FC  stb r10, 0x3fc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1020 as u32), ctx.r[10].u8 ) };
	// 83199CA0: 994B03FD  stb r10, 0x3fd(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1021 as u32), ctx.r[10].u8 ) };
	// 83199CA4: 3940FFB9  li r10, -0x47
	ctx.r[10].s64 = -71;
	// 83199CA8: 994B03FE  stb r10, 0x3fe(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1022 as u32), ctx.r[10].u8 ) };
	// 83199CAC: 994B03FF  stb r10, 0x3ff(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1023 as u32), ctx.r[10].u8 ) };
	// 83199CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83199CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83199CB8 size=292
    let mut pc: u32 = 0x83199CB8;
    'dispatch: loop {
        match pc {
            0x83199CB8 => {
    //   block [0x83199CB8..0x83199DDC)
	// 83199CB8: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83199CBC: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 83199CC0: 396B36E0  addi r11, r11, 0x36e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14048;
	// 83199CC4: 39200100  li r9, 0x100
	ctx.r[9].s64 = 256;
	// 83199CC8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83199CCC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199CD0: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 83199CD4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199CD8: 4200FFF8  bdnz 0x83199cd0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199CD0; continue 'dispatch;
	}
	// 83199CDC: 39000012  li r8, 0x12
	ctx.r[8].s64 = 18;
	// 83199CE0: 39200100  li r9, 0x100
	ctx.r[9].s64 = 256;
	// 83199CE4: 394B0100  addi r10, r11, 0x100
	ctx.r[10].s64 = ctx.r[11].s64 + 256;
	// 83199CE8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199CEC: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 83199CF0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199CF4: 4200FFF8  bdnz 0x83199cec
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199CEC; continue 'dispatch;
	}
	// 83199CF8: 39000022  li r8, 0x22
	ctx.r[8].s64 = 34;
	// 83199CFC: 39200100  li r9, 0x100
	ctx.r[9].s64 = 256;
	// 83199D00: 394B0200  addi r10, r11, 0x200
	ctx.r[10].s64 = ctx.r[11].s64 + 512;
	// 83199D04: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199D08: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 83199D0C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199D10: 4200FFF8  bdnz 0x83199d08
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199D08; continue 'dispatch;
	}
	// 83199D14: 39200033  li r9, 0x33
	ctx.r[9].s64 = 51;
	// 83199D18: 39000080  li r8, 0x80
	ctx.r[8].s64 = 128;
	// 83199D1C: 394B0300  addi r10, r11, 0x300
	ctx.r[10].s64 = ctx.r[11].s64 + 768;
	// 83199D20: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83199D24: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83199D28: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199D2C: 4200FFF8  bdnz 0x83199d24
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199D24; continue 'dispatch;
	}
	// 83199D30: 39200044  li r9, 0x44
	ctx.r[9].s64 = 68;
	// 83199D34: 39000040  li r8, 0x40
	ctx.r[8].s64 = 64;
	// 83199D38: 394B0380  addi r10, r11, 0x380
	ctx.r[10].s64 = ctx.r[11].s64 + 896;
	// 83199D3C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83199D40: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83199D44: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199D48: 4200FFF8  bdnz 0x83199d40
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199D40; continue 'dispatch;
	}
	// 83199D4C: 39200055  li r9, 0x55
	ctx.r[9].s64 = 85;
	// 83199D50: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 83199D54: 394B03C0  addi r10, r11, 0x3c0
	ctx.r[10].s64 = ctx.r[11].s64 + 960;
	// 83199D58: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83199D5C: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83199D60: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199D64: 4200FFF8  bdnz 0x83199d5c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199D5C; continue 'dispatch;
	}
	// 83199D68: 39200066  li r9, 0x66
	ctx.r[9].s64 = 102;
	// 83199D6C: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 83199D70: 394B03E0  addi r10, r11, 0x3e0
	ctx.r[10].s64 = ctx.r[11].s64 + 992;
	// 83199D74: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83199D78: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83199D7C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83199D80: 4200FFF8  bdnz 0x83199d78
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199D78; continue 'dispatch;
	}
	// 83199D84: 39400077  li r10, 0x77
	ctx.r[10].s64 = 119;
	// 83199D88: 994B03F0  stb r10, 0x3f0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1008 as u32), ctx.r[10].u8 ) };
	// 83199D8C: 994B03F1  stb r10, 0x3f1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1009 as u32), ctx.r[10].u8 ) };
	// 83199D90: 994B03F2  stb r10, 0x3f2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1010 as u32), ctx.r[10].u8 ) };
	// 83199D94: 994B03F3  stb r10, 0x3f3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1011 as u32), ctx.r[10].u8 ) };
	// 83199D98: 994B03F4  stb r10, 0x3f4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1012 as u32), ctx.r[10].u8 ) };
	// 83199D9C: 994B03F5  stb r10, 0x3f5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1013 as u32), ctx.r[10].u8 ) };
	// 83199DA0: 994B03F6  stb r10, 0x3f6(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1014 as u32), ctx.r[10].u8 ) };
	// 83199DA4: 994B03F7  stb r10, 0x3f7(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1015 as u32), ctx.r[10].u8 ) };
	// 83199DA8: 3940FF88  li r10, -0x78
	ctx.r[10].s64 = -120;
	// 83199DAC: 994B03F8  stb r10, 0x3f8(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1016 as u32), ctx.r[10].u8 ) };
	// 83199DB0: 994B03F9  stb r10, 0x3f9(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1017 as u32), ctx.r[10].u8 ) };
	// 83199DB4: 994B03FA  stb r10, 0x3fa(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1018 as u32), ctx.r[10].u8 ) };
	// 83199DB8: 994B03FB  stb r10, 0x3fb(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1019 as u32), ctx.r[10].u8 ) };
	// 83199DBC: 3940FF99  li r10, -0x67
	ctx.r[10].s64 = -103;
	// 83199DC0: 994B03FC  stb r10, 0x3fc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1020 as u32), ctx.r[10].u8 ) };
	// 83199DC4: 994B03FD  stb r10, 0x3fd(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1021 as u32), ctx.r[10].u8 ) };
	// 83199DC8: 3940FFAA  li r10, -0x56
	ctx.r[10].s64 = -86;
	// 83199DCC: 994B03FE  stb r10, 0x3fe(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1022 as u32), ctx.r[10].u8 ) };
	// 83199DD0: 3940FFBA  li r10, -0x46
	ctx.r[10].s64 = -70;
	// 83199DD4: 994B03FF  stb r10, 0x3ff(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1023 as u32), ctx.r[10].u8 ) };
	// 83199DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83199DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83199DE0 size=504
    let mut pc: u32 = 0x83199DE0;
    'dispatch: loop {
        match pc {
            0x83199DE0 => {
    //   block [0x83199DE0..0x83199FD8)
	// 83199DE0: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83199DE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83199DE8: 396B3EE0  addi r11, r11, 0x3ee0
	ctx.r[11].s64 = ctx.r[11].s64 + 16096;
	// 83199DEC: 3D400006  lis r10, 6
	ctx.r[10].s64 = 393216;
	// 83199DF0: 3CC00008  lis r6, 8
	ctx.r[6].s64 = 524288;
	// 83199DF4: 614A4040  ori r10, r10, 0x4040
	ctx.r[10].u64 = ctx.r[10].u64 | 16448;
	// 83199DF8: 60C60202  ori r6, r6, 0x202
	ctx.r[6].u64 = ctx.r[6].u64 | 514;
	// 83199DFC: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 83199E00: 3CA00008  lis r5, 8
	ctx.r[5].s64 = 524288;
	// 83199E04: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 83199E08: 3C800008  lis r4, 8
	ctx.r[4].s64 = 524288;
	// 83199E0C: 90EB0008  stw r7, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 83199E10: 60A50109  ori r5, r5, 0x109
	ctx.r[5].u64 = ctx.r[5].u64 | 265;
	// 83199E14: 90EB000C  stw r7, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 83199E18: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83199E1C: 60840400  ori r4, r4, 0x400
	ctx.r[4].u64 = ctx.r[4].u64 | 1024;
	// 83199E20: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83199E24: 3C600008  lis r3, 8
	ctx.r[3].s64 = 524288;
	// 83199E28: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 83199E2C: 3D200007  lis r9, 7
	ctx.r[9].s64 = 458752;
	// 83199E30: 914B001C  stw r10, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 83199E34: 90CB0020  stw r6, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 83199E38: 60630108  ori r3, r3, 0x108
	ctx.r[3].u64 = ctx.r[3].u64 | 264;
	// 83199E3C: 90CB0024  stw r6, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[6].u32 ) };
	// 83199E40: 90AB0028  stw r5, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[5].u32 ) };
	// 83199E44: 61290107  ori r9, r9, 0x107
	ctx.r[9].u64 = ctx.r[9].u64 | 263;
	// 83199E48: 90AB002C  stw r5, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[5].u32 ) };
	// 83199E4C: 908B0030  stw r4, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[4].u32 ) };
	// 83199E50: 3D000007  lis r8, 7
	ctx.r[8].s64 = 458752;
	// 83199E54: 908B0034  stw r4, 0x34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[4].u32 ) };
	// 83199E58: 906B0038  stw r3, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 83199E5C: 61080106  ori r8, r8, 0x106
	ctx.r[8].u64 = ctx.r[8].u64 | 262;
	// 83199E60: 906B003C  stw r3, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[3].u32 ) };
	// 83199E64: 912B0040  stw r9, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[9].u32 ) };
	// 83199E68: 3D400007  lis r10, 7
	ctx.r[10].s64 = 458752;
	// 83199E6C: 912B0044  stw r9, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 83199E70: 3CC00006  lis r6, 6
	ctx.r[6].s64 = 393216;
	// 83199E74: 912B0048  stw r9, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[9].u32 ) };
	// 83199E78: 614A0201  ori r10, r10, 0x201
	ctx.r[10].u64 = ctx.r[10].u64 | 513;
	// 83199E7C: 912B004C  stw r9, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[9].u32 ) };
	// 83199E80: 910B0050  stw r8, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 83199E84: 3D200007  lis r9, 7
	ctx.r[9].s64 = 458752;
	// 83199E88: 910B0054  stw r8, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 83199E8C: 60C60300  ori r6, r6, 0x300
	ctx.r[6].u64 = ctx.r[6].u64 | 768;
	// 83199E90: 910B0058  stw r8, 0x58(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 83199E94: 61290105  ori r9, r9, 0x105
	ctx.r[9].u64 = ctx.r[9].u64 | 261;
	// 83199E98: 910B005C  stw r8, 0x5c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83199E9C: 914B0060  stw r10, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 83199EA0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 83199EA4: 914B0064  stw r10, 0x64(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 83199EA8: 390B00A0  addi r8, r11, 0xa0
	ctx.r[8].s64 = ctx.r[11].s64 + 160;
	// 83199EAC: 914B0068  stw r10, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 83199EB0: 914B006C  stw r10, 0x6c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 83199EB4: 3D400009  lis r10, 9
	ctx.r[10].s64 = 589824;
	// 83199EB8: 912B0070  stw r9, 0x70(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 83199EBC: 614A010D  ori r10, r10, 0x10d
	ctx.r[10].u64 = ctx.r[10].u64 | 269;
	// 83199EC0: 912B0074  stw r9, 0x74(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 83199EC4: 912B0078  stw r9, 0x78(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(120 as u32), ctx.r[9].u32 ) };
	// 83199EC8: 912B007C  stw r9, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[9].u32 ) };
	// 83199ECC: 914B0080  stw r10, 0x80(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 83199ED0: 3D400009  lis r10, 9
	ctx.r[10].s64 = 589824;
	// 83199ED4: 614A0600  ori r10, r10, 0x600
	ctx.r[10].u64 = ctx.r[10].u64 | 1536;
	// 83199ED8: 914B0084  stw r10, 0x84(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 83199EDC: 3D400009  lis r10, 9
	ctx.r[10].s64 = 589824;
	// 83199EE0: 614A010C  ori r10, r10, 0x10c
	ctx.r[10].u64 = ctx.r[10].u64 | 268;
	// 83199EE4: 914B0088  stw r10, 0x88(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 83199EE8: 3D400009  lis r10, 9
	ctx.r[10].s64 = 589824;
	// 83199EEC: 614A010B  ori r10, r10, 0x10b
	ctx.r[10].u64 = ctx.r[10].u64 | 267;
	// 83199EF0: 914B008C  stw r10, 0x8c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 83199EF4: 3D400009  lis r10, 9
	ctx.r[10].s64 = 589824;
	// 83199EF8: 614A0203  ori r10, r10, 0x203
	ctx.r[10].u64 = ctx.r[10].u64 | 515;
	// 83199EFC: 914B0090  stw r10, 0x90(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 83199F00: 3D400009  lis r10, 9
	ctx.r[10].s64 = 589824;
	// 83199F04: 614A0301  ori r10, r10, 0x301
	ctx.r[10].u64 = ctx.r[10].u64 | 769;
	// 83199F08: 914B0094  stw r10, 0x94(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 83199F0C: 3D400009  lis r10, 9
	ctx.r[10].s64 = 589824;
	// 83199F10: 614A0500  ori r10, r10, 0x500
	ctx.r[10].u64 = ctx.r[10].u64 | 1280;
	// 83199F14: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 83199F18: 3D400009  lis r10, 9
	ctx.r[10].s64 = 589824;
	// 83199F1C: 614A010A  ori r10, r10, 0x10a
	ctx.r[10].u64 = ctx.r[10].u64 | 266;
	// 83199F20: 914B009C  stw r10, 0x9c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 83199F24: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 83199F28: 90C80000  stw r6, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 83199F2C: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 83199F30: 4200FFF8  bdnz 0x83199f28
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199F28; continue 'dispatch;
	}
	// 83199F34: 3D000006  lis r8, 6
	ctx.r[8].s64 = 393216;
	// 83199F38: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 83199F3C: 61080104  ori r8, r8, 0x104
	ctx.r[8].u64 = ctx.r[8].u64 | 260;
	// 83199F40: 394B00C0  addi r10, r11, 0xc0
	ctx.r[10].s64 = ctx.r[11].s64 + 192;
	// 83199F44: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199F48: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83199F4C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83199F50: 4200FFF8  bdnz 0x83199f48
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199F48; continue 'dispatch;
	}
	// 83199F54: 3D000006  lis r8, 6
	ctx.r[8].s64 = 393216;
	// 83199F58: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 83199F5C: 61080103  ori r8, r8, 0x103
	ctx.r[8].u64 = ctx.r[8].u64 | 259;
	// 83199F60: 394B00E0  addi r10, r11, 0xe0
	ctx.r[10].s64 = ctx.r[11].s64 + 224;
	// 83199F64: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199F68: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83199F6C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83199F70: 4200FFF8  bdnz 0x83199f68
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199F68; continue 'dispatch;
	}
	// 83199F74: 3D000005  lis r8, 5
	ctx.r[8].s64 = 327680;
	// 83199F78: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83199F7C: 61080200  ori r8, r8, 0x200
	ctx.r[8].u64 = ctx.r[8].u64 | 512;
	// 83199F80: 394B0100  addi r10, r11, 0x100
	ctx.r[10].s64 = ctx.r[11].s64 + 256;
	// 83199F84: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199F88: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83199F8C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83199F90: 4200FFF8  bdnz 0x83199f88
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199F88; continue 'dispatch;
	}
	// 83199F94: 3D000005  lis r8, 5
	ctx.r[8].s64 = 327680;
	// 83199F98: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83199F9C: 61080102  ori r8, r8, 0x102
	ctx.r[8].u64 = ctx.r[8].u64 | 258;
	// 83199FA0: 394B0140  addi r10, r11, 0x140
	ctx.r[10].s64 = ctx.r[11].s64 + 320;
	// 83199FA4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199FA8: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83199FAC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83199FB0: 4200FFF8  bdnz 0x83199fa8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199FA8; continue 'dispatch;
	}
	// 83199FB4: 3D400004  lis r10, 4
	ctx.r[10].s64 = 262144;
	// 83199FB8: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 83199FBC: 614A0101  ori r10, r10, 0x101
	ctx.r[10].u64 = ctx.r[10].u64 | 257;
	// 83199FC0: 396B0180  addi r11, r11, 0x180
	ctx.r[11].s64 = ctx.r[11].s64 + 384;
	// 83199FC4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83199FC8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83199FCC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83199FD0: 4200FFF8  bdnz 0x83199fc8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83199FC8; continue 'dispatch;
	}
	// 83199FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83199FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83199FD8 size=4
    let mut pc: u32 = 0x83199FD8;
    'dispatch: loop {
        match pc {
            0x83199FD8 => {
    //   block [0x83199FD8..0x83199FDC)
	// 83199FD8: 4BFFFE08  b 0x83199de0
	sub_83199DE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83199FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83199FE0 size=284
    let mut pc: u32 = 0x83199FE0;
    'dispatch: loop {
        match pc {
            0x83199FE0 => {
    //   block [0x83199FE0..0x8319A0FC)
	// 83199FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83199FE4: 4800E15D  bl 0x831a8140
	ctx.lr = 0x83199FE8;
	sub_831A8130(ctx, base);
	// 83199FE8: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83199FEC: 3E408345  lis r18, -0x7cbb
	ctx.r[18].s64 = -2092630016;
	// 83199FF0: 396B36E0  addi r11, r11, 0x36e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14048;
	// 83199FF4: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 83199FF8: 392B1300  addi r9, r11, 0x1300
	ctx.r[9].s64 = ctx.r[11].s64 + 4864;
	// 83199FFC: 390B0CC0  addi r8, r11, 0xcc0
	ctx.r[8].s64 = ctx.r[11].s64 + 3264;
	// 8319A000: 38EB1500  addi r7, r11, 0x1500
	ctx.r[7].s64 = ctx.r[11].s64 + 5376;
	// 8319A004: 38CB0C80  addi r6, r11, 0xc80
	ctx.r[6].s64 = ctx.r[11].s64 + 3200;
	// 8319A008: 38AB0A00  addi r5, r11, 0xa00
	ctx.r[5].s64 = ctx.r[11].s64 + 2560;
	// 8319A00C: 913297D0  stw r9, -0x6830(r18)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[18].u32.wrapping_add(-26672 as u32), ctx.r[9].u32 ) };
	// 8319A010: 3D208345  lis r9, -0x7cbb
	ctx.r[9].s64 = -2092630016;
	// 8319A014: 388B0E40  addi r4, r11, 0xe40
	ctx.r[4].s64 = ctx.r[11].s64 + 3648;
	// 8319A018: 386B0D80  addi r3, r11, 0xd80
	ctx.r[3].s64 = ctx.r[11].s64 + 3456;
	// 8319A01C: 3BEB0B00  addi r31, r11, 0xb00
	ctx.r[31].s64 = ctx.r[11].s64 + 2816;
	// 8319A020: 3BCB0B80  addi r30, r11, 0xb80
	ctx.r[30].s64 = ctx.r[11].s64 + 2944;
	// 8319A024: 910997B0  stw r8, -0x6850(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26704 as u32), ctx.r[8].u32 ) };
	// 8319A028: 3D208345  lis r9, -0x7cbb
	ctx.r[9].s64 = -2092630016;
	// 8319A02C: 3BAB0D40  addi r29, r11, 0xd40
	ctx.r[29].s64 = ctx.r[11].s64 + 3392;
	// 8319A030: 3B8B0400  addi r28, r11, 0x400
	ctx.r[28].s64 = ctx.r[11].s64 + 1024;
	// 8319A034: 3B6B0DC0  addi r27, r11, 0xdc0
	ctx.r[27].s64 = ctx.r[11].s64 + 3520;
	// 8319A038: 3B4B0E80  addi r26, r11, 0xe80
	ctx.r[26].s64 = ctx.r[11].s64 + 3712;
	// 8319A03C: 90E997B8  stw r7, -0x6848(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26696 as u32), ctx.r[7].u32 ) };
	// 8319A040: 3D208345  lis r9, -0x7cbb
	ctx.r[9].s64 = -2092630016;
	// 8319A044: 3B2B0F00  addi r25, r11, 0xf00
	ctx.r[25].s64 = ctx.r[11].s64 + 3840;
	// 8319A048: 3A6B0800  addi r19, r11, 0x800
	ctx.r[19].s64 = ctx.r[11].s64 + 2048;
	// 8319A04C: 394ABFC8  addi r10, r10, -0x4038
	ctx.r[10].s64 = ctx.r[10].s64 + -16440;
	// 8319A050: 90C997B4  stw r6, -0x684c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26700 as u32), ctx.r[6].u32 ) };
	// 8319A054: 3D208345  lis r9, -0x7cbb
	ctx.r[9].s64 = -2092630016;
	// 8319A058: 3B0A0020  addi r24, r10, 0x20
	ctx.r[24].s64 = ctx.r[10].s64 + 32;
	// 8319A05C: 3AEA0040  addi r23, r10, 0x40
	ctx.r[23].s64 = ctx.r[10].s64 + 64;
	// 8319A060: 3ACA0060  addi r22, r10, 0x60
	ctx.r[22].s64 = ctx.r[10].s64 + 96;
	// 8319A064: 3AAA0080  addi r21, r10, 0x80
	ctx.r[21].s64 = ctx.r[10].s64 + 128;
	// 8319A068: 90A997A8  stw r5, -0x6858(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26712 as u32), ctx.r[5].u32 ) };
	// 8319A06C: 3D208345  lis r9, -0x7cbb
	ctx.r[9].s64 = -2092630016;
	// 8319A070: 3A8A00A0  addi r20, r10, 0xa0
	ctx.r[20].s64 = ctx.r[10].s64 + 160;
	// 8319A074: 908997A4  stw r4, -0x685c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26716 as u32), ctx.r[4].u32 ) };
	// 8319A078: 3D208345  lis r9, -0x7cbb
	ctx.r[9].s64 = -2092630016;
	// 8319A07C: 906997A0  stw r3, -0x6860(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26720 as u32), ctx.r[3].u32 ) };
	// 8319A080: 3D208345  lis r9, -0x7cbb
	ctx.r[9].s64 = -2092630016;
	// 8319A084: 93E997BC  stw r31, -0x6844(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26692 as u32), ctx.r[31].u32 ) };
	// 8319A088: 3D208345  lis r9, -0x7cbb
	ctx.r[9].s64 = -2092630016;
	// 8319A08C: 93C997D4  stw r30, -0x682c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26668 as u32), ctx.r[30].u32 ) };
	// 8319A090: 3D208345  lis r9, -0x7cbb
	ctx.r[9].s64 = -2092630016;
	// 8319A094: 93A9979C  stw r29, -0x6864(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26724 as u32), ctx.r[29].u32 ) };
	// 8319A098: 3D208345  lis r9, -0x7cbb
	ctx.r[9].s64 = -2092630016;
	// 8319A09C: 938997C8  stw r28, -0x6838(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26680 as u32), ctx.r[28].u32 ) };
	// 8319A0A0: 3D208345  lis r9, -0x7cbb
	ctx.r[9].s64 = -2092630016;
	// 8319A0A4: 936997AC  stw r27, -0x6854(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26708 as u32), ctx.r[27].u32 ) };
	// 8319A0A8: 3D208345  lis r9, -0x7cbb
	ctx.r[9].s64 = -2092630016;
	// 8319A0AC: 934997CC  stw r26, -0x6834(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26676 as u32), ctx.r[26].u32 ) };
	// 8319A0B0: 3D208345  lis r9, -0x7cbb
	ctx.r[9].s64 = -2092630016;
	// 8319A0B4: 932997DC  stw r25, -0x6824(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26660 as u32), ctx.r[25].u32 ) };
	// 8319A0B8: 3D208345  lis r9, -0x7cbb
	ctx.r[9].s64 = -2092630016;
	// 8319A0BC: 916997C4  stw r11, -0x683c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26684 as u32), ctx.r[11].u32 ) };
	// 8319A0C0: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A0C4: 914B97E0  stw r10, -0x6820(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26656 as u32), ctx.r[10].u32 ) };
	// 8319A0C8: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A0CC: 930B9798  stw r24, -0x6868(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26728 as u32), ctx.r[24].u32 ) };
	// 8319A0D0: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A0D4: 92EB9794  stw r23, -0x686c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26732 as u32), ctx.r[23].u32 ) };
	// 8319A0D8: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A0DC: 92CB97E4  stw r22, -0x681c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26652 as u32), ctx.r[22].u32 ) };
	// 8319A0E0: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A0E4: 92AB97C0  stw r21, -0x6840(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26688 as u32), ctx.r[21].u32 ) };
	// 8319A0E8: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A0EC: 928B97D8  stw r20, -0x6828(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26664 as u32), ctx.r[20].u32 ) };
	// 8319A0F0: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A0F4: 926B97E8  stw r19, -0x6818(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26648 as u32), ctx.r[19].u32 ) };
	// 8319A0F8: 4800E098  b 0x831a8190
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8319A100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8319A100 size=256
    let mut pc: u32 = 0x8319A100;
    'dispatch: loop {
        match pc {
            0x8319A100 => {
    //   block [0x8319A100..0x8319A200)
	// 8319A100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319A104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8319A108: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8319A10C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8319A110: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319A114: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8319A118: 3BE3FE00  addi r31, r3, -0x200
	ctx.r[31].s64 = ctx.r[3].s64 + -512;
	// 8319A11C: 388B3EE0  addi r4, r11, 0x3ee0
	ctx.r[4].s64 = ctx.r[11].s64 + 16096;
	// 8319A120: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A124: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 8319A128: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8319A12C: 93EB97E8  stw r31, -0x6818(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26648 as u32), ctx.r[31].u32 ) };
	// 8319A130: 48000359  bl 0x8319a488
	ctx.lr = 0x8319A134;
	sub_8319A488(ctx, base);
	// 8319A134: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8319A138: 3BDFFFF0  addi r30, r31, -0x10
	ctx.r[30].s64 = ctx.r[31].s64 + -16;
	// 8319A13C: 3BEBBFC8  addi r31, r11, -0x4038
	ctx.r[31].s64 = ctx.r[11].s64 + -16440;
	// 8319A140: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A144: 389F00A0  addi r4, r31, 0xa0
	ctx.r[4].s64 = ctx.r[31].s64 + 160;
	// 8319A148: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8319A14C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8319A150: 93CB97D8  stw r30, -0x6828(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26664 as u32), ctx.r[30].u32 ) };
	// 8319A154: 48000335  bl 0x8319a488
	ctx.lr = 0x8319A158;
	sub_8319A488(ctx, base);
	// 8319A158: 3BDEFFE0  addi r30, r30, -0x20
	ctx.r[30].s64 = ctx.r[30].s64 + -32;
	// 8319A15C: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A160: 389F0080  addi r4, r31, 0x80
	ctx.r[4].s64 = ctx.r[31].s64 + 128;
	// 8319A164: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 8319A168: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8319A16C: 93CB97C0  stw r30, -0x6840(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26688 as u32), ctx.r[30].u32 ) };
	// 8319A170: 48000319  bl 0x8319a488
	ctx.lr = 0x8319A174;
	sub_8319A488(ctx, base);
	// 8319A174: 3BDEFFE0  addi r30, r30, -0x20
	ctx.r[30].s64 = ctx.r[30].s64 + -32;
	// 8319A178: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A17C: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 8319A180: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 8319A184: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8319A188: 93CB97E4  stw r30, -0x681c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26652 as u32), ctx.r[30].u32 ) };
	// 8319A18C: 480002FD  bl 0x8319a488
	ctx.lr = 0x8319A190;
	sub_8319A488(ctx, base);
	// 8319A190: 3BDEFFE0  addi r30, r30, -0x20
	ctx.r[30].s64 = ctx.r[30].s64 + -32;
	// 8319A194: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A198: 389F0040  addi r4, r31, 0x40
	ctx.r[4].s64 = ctx.r[31].s64 + 64;
	// 8319A19C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 8319A1A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8319A1A4: 93CB9794  stw r30, -0x686c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26732 as u32), ctx.r[30].u32 ) };
	// 8319A1A8: 480002E1  bl 0x8319a488
	ctx.lr = 0x8319A1AC;
	sub_8319A488(ctx, base);
	// 8319A1AC: 3BDEFFE0  addi r30, r30, -0x20
	ctx.r[30].s64 = ctx.r[30].s64 + -32;
	// 8319A1B0: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A1B4: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 8319A1B8: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 8319A1BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8319A1C0: 93CB9798  stw r30, -0x6868(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26728 as u32), ctx.r[30].u32 ) };
	// 8319A1C4: 480002C5  bl 0x8319a488
	ctx.lr = 0x8319A1C8;
	sub_8319A488(ctx, base);
	// 8319A1C8: 3BDEFFE0  addi r30, r30, -0x20
	ctx.r[30].s64 = ctx.r[30].s64 + -32;
	// 8319A1CC: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A1D0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 8319A1D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8319A1D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8319A1DC: 93CB97E0  stw r30, -0x6820(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26656 as u32), ctx.r[30].u32 ) };
	// 8319A1E0: 480002A9  bl 0x8319a488
	ctx.lr = 0x8319A1E4;
	sub_8319A488(ctx, base);
	// 8319A1E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8319A1E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8319A1EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319A1F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319A1F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8319A1F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8319A1FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8319A200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8319A200 size=112
    let mut pc: u32 = 0x8319A200;
    'dispatch: loop {
        match pc {
            0x8319A200 => {
    //   block [0x8319A200..0x8319A270)
	// 8319A200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319A204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8319A208: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8319A20C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8319A210: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319A214: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8319A218: 3BE3FF80  addi r31, r3, -0x80
	ctx.r[31].s64 = ctx.r[3].s64 + -128;
	// 8319A21C: 3BCB4560  addi r30, r11, 0x4560
	ctx.r[30].s64 = ctx.r[11].s64 + 17760;
	// 8319A220: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A224: 389EFF40  addi r4, r30, -0xc0
	ctx.r[4].s64 = ctx.r[30].s64 + -192;
	// 8319A228: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8319A22C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8319A230: 93EB97AC  stw r31, -0x6854(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26708 as u32), ctx.r[31].u32 ) };
	// 8319A234: 48000255  bl 0x8319a488
	ctx.lr = 0x8319A238;
	sub_8319A488(ctx, base);
	// 8319A238: 3BFFFF80  addi r31, r31, -0x80
	ctx.r[31].s64 = ctx.r[31].s64 + -128;
	// 8319A23C: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A240: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8319A244: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8319A248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8319A24C: 93EB97CC  stw r31, -0x6834(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26676 as u32), ctx.r[31].u32 ) };
	// 8319A250: 48000239  bl 0x8319a488
	ctx.lr = 0x8319A254;
	sub_8319A488(ctx, base);
	// 8319A254: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8319A258: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8319A25C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319A260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319A264: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8319A268: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8319A26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8319A270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8319A270 size=112
    let mut pc: u32 = 0x8319A270;
    'dispatch: loop {
        match pc {
            0x8319A270 => {
    //   block [0x8319A270..0x8319A2E0)
	// 8319A270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319A274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8319A278: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8319A27C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8319A280: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319A284: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8319A288: 3BE3FF00  addi r31, r3, -0x100
	ctx.r[31].s64 = ctx.r[3].s64 + -256;
	// 8319A28C: 3BCB4420  addi r30, r11, 0x4420
	ctx.r[30].s64 = ctx.r[11].s64 + 17440;
	// 8319A290: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A294: 389EFE40  addi r4, r30, -0x1c0
	ctx.r[4].s64 = ctx.r[30].s64 + -448;
	// 8319A298: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8319A29C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8319A2A0: 93EB97D4  stw r31, -0x682c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26668 as u32), ctx.r[31].u32 ) };
	// 8319A2A4: 480001E5  bl 0x8319a488
	ctx.lr = 0x8319A2A8;
	sub_8319A488(ctx, base);
	// 8319A2A8: 3BFFFFC0  addi r31, r31, -0x40
	ctx.r[31].s64 = ctx.r[31].s64 + -64;
	// 8319A2AC: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A2B0: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 8319A2B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8319A2B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8319A2BC: 93EB979C  stw r31, -0x6864(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26724 as u32), ctx.r[31].u32 ) };
	// 8319A2C0: 480001C9  bl 0x8319a488
	ctx.lr = 0x8319A2C4;
	sub_8319A488(ctx, base);
	// 8319A2C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8319A2C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8319A2CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319A2D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319A2D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8319A2D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8319A2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8319A2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8319A2E0 size=112
    let mut pc: u32 = 0x8319A2E0;
    'dispatch: loop {
        match pc {
            0x8319A2E0 => {
    //   block [0x8319A2E0..0x8319A350)
	// 8319A2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319A2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8319A2E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8319A2EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8319A2F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319A2F4: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8319A2F8: 3BE3FFC0  addi r31, r3, -0x40
	ctx.r[31].s64 = ctx.r[3].s64 + -64;
	// 8319A2FC: 3BCB41E0  addi r30, r11, 0x41e0
	ctx.r[30].s64 = ctx.r[11].s64 + 16864;
	// 8319A300: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A304: 389E0280  addi r4, r30, 0x280
	ctx.r[4].s64 = ctx.r[30].s64 + 640;
	// 8319A308: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 8319A30C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8319A310: 93EB97A0  stw r31, -0x6860(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26720 as u32), ctx.r[31].u32 ) };
	// 8319A314: 48000175  bl 0x8319a488
	ctx.lr = 0x8319A318;
	sub_8319A488(ctx, base);
	// 8319A318: 3BFFFF80  addi r31, r31, -0x80
	ctx.r[31].s64 = ctx.r[31].s64 + -128;
	// 8319A31C: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319A320: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8319A324: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8319A328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8319A32C: 93EB97BC  stw r31, -0x6844(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26692 as u32), ctx.r[31].u32 ) };
	// 8319A330: 48000159  bl 0x8319a488
	ctx.lr = 0x8319A334;
	sub_8319A488(ctx, base);
	// 8319A334: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8319A338: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8319A33C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319A340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319A344: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8319A348: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8319A34C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8319A350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8319A350 size=40
    let mut pc: u32 = 0x8319A350;
    'dispatch: loop {
        match pc {
            0x8319A350 => {
    //   block [0x8319A350..0x8319A378)
	// 8319A350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319A354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8319A358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319A35C: 4BFFE3BD  bl 0x83198718
	ctx.lr = 0x8319A360;
	sub_83198718(ctx, base);
	// 8319A360: 4BFFE681  bl 0x831989e0
	ctx.lr = 0x8319A364;
	sub_831989E0(ctx, base);
	// 8319A364: 4BFFE9B5  bl 0x83198d18
	ctx.lr = 0x8319A368;
	sub_83198D18(ctx, base);
	// 8319A368: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8319A36C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319A370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319A374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8319A378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8319A378 size=36
    let mut pc: u32 = 0x8319A378;
    'dispatch: loop {
        match pc {
            0x8319A378 => {
    //   block [0x8319A378..0x8319A39C)
	// 8319A378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319A37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8319A380: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319A384: 4BFFECDD  bl 0x83199060
	ctx.lr = 0x8319A388;
	sub_83199060(ctx, base);
	// 8319A388: 4BFFED51  bl 0x831990d8
	ctx.lr = 0x8319A38C;
	sub_831990D8(ctx, base);
	// 8319A38C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8319A390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319A394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319A398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8319A3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8319A3A0 size=44
    let mut pc: u32 = 0x8319A3A0;
    'dispatch: loop {
        match pc {
            0x8319A3A0 => {
    //   block [0x8319A3A0..0x8319A3CC)
	// 8319A3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319A3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8319A3A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319A3AC: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8319A3B0: 386B3AE0  addi r3, r11, 0x3ae0
	ctx.r[3].s64 = ctx.r[11].s64 + 15072;
	// 8319A3B4: 4BFFEFD5  bl 0x83199388
	ctx.lr = 0x8319A3B8;
	sub_83199388(ctx, base);
	// 8319A3B8: 4BFFF249  bl 0x83199600
	ctx.lr = 0x8319A3BC;
	sub_83199600(ctx, base);
	// 8319A3BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8319A3C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319A3C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319A3C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8319A3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8319A3D0 size=44
    let mut pc: u32 = 0x8319A3D0;
    'dispatch: loop {
        match pc {
            0x8319A3D0 => {
    //   block [0x8319A3D0..0x8319A3FC)
	// 8319A3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319A3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8319A3D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319A3DC: 4BFFF5E5  bl 0x831999c0
	ctx.lr = 0x8319A3E0;
	sub_831999C0(ctx, base);
	// 8319A3E0: 4BFFF6C9  bl 0x83199aa8
	ctx.lr = 0x8319A3E4;
	sub_83199AA8(ctx, base);
	// 8319A3E4: 4BFFF795  bl 0x83199b78
	ctx.lr = 0x8319A3E8;
	sub_83199B78(ctx, base);
	// 8319A3E8: 4BFFF8D1  bl 0x83199cb8
	ctx.lr = 0x8319A3EC;
	sub_83199CB8(ctx, base);
	// 8319A3EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8319A3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319A3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319A3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8319A400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8319A400 size=48
    let mut pc: u32 = 0x8319A400;
    'dispatch: loop {
        match pc {
            0x8319A400 => {
    //   block [0x8319A400..0x8319A430)
	// 8319A400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319A404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8319A408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319A40C: 386305B0  addi r3, r3, 0x5b0
	ctx.r[3].s64 = ctx.r[3].s64 + 1456;
	// 8319A410: 4BFFFCF1  bl 0x8319a100
	ctx.lr = 0x8319A414;
	sub_8319A100(ctx, base);
	// 8319A414: 4BFFFDED  bl 0x8319a200
	ctx.lr = 0x8319A418;
	sub_8319A200(ctx, base);
	// 8319A418: 4BFFFE59  bl 0x8319a270
	ctx.lr = 0x8319A41C;
	sub_8319A270(ctx, base);
	// 8319A41C: 4BFFFEC5  bl 0x8319a2e0
	ctx.lr = 0x8319A420;
	sub_8319A2E0(ctx, base);
	// 8319A420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8319A424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319A428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319A42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8319A430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8319A430 size=84
    let mut pc: u32 = 0x8319A430;
    'dispatch: loop {
        match pc {
            0x8319A430 => {
    //   block [0x8319A430..0x8319A484)
	// 8319A430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319A434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8319A438: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8319A43C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319A440: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8319A444: 4BFFFF0D  bl 0x8319a350
	ctx.lr = 0x8319A448;
	sub_8319A350(ctx, base);
	// 8319A448: 4BFFFF31  bl 0x8319a378
	ctx.lr = 0x8319A44C;
	sub_8319A378(ctx, base);
	// 8319A44C: 4BFFED6D  bl 0x831991b8
	ctx.lr = 0x8319A450;
	sub_831991B8(ctx, base);
	// 8319A450: 4BFFFF51  bl 0x8319a3a0
	ctx.lr = 0x8319A454;
	sub_8319A3A0(ctx, base);
	// 8319A454: 4BFFFF7D  bl 0x8319a3d0
	ctx.lr = 0x8319A458;
	sub_8319A3D0(ctx, base);
	// 8319A458: 4BFFFB81  bl 0x83199fd8
	ctx.lr = 0x8319A45C;
	sub_83199FD8(ctx, base);
	// 8319A45C: 4BFFFB85  bl 0x83199fe0
	ctx.lr = 0x8319A460;
	sub_83199FE0(ctx, base);
	// 8319A460: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8319A464: 419A000C  beq cr6, 0x8319a470
	if ctx.cr[6].eq {
	pc = 0x8319A470; continue 'dispatch;
	}
	// 8319A468: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8319A46C: 4BFFFF95  bl 0x8319a400
	ctx.lr = 0x8319A470;
	sub_8319A400(ctx, base);
	// 8319A470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8319A474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319A478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319A47C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8319A480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8319A488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8319A488 size=60
    let mut pc: u32 = 0x8319A488;
    'dispatch: loop {
        match pc {
            0x8319A488 => {
    //   block [0x8319A488..0x8319A4C4)
	// 8319A488: 54A9073E  clrlwi r9, r5, 0x1c
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0x0000000Fu64;
	// 8319A48C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8319A490: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8319A494: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8319A498: 419A0020  beq cr6, 0x8319a4b8
	if ctx.cr[6].eq {
	pc = 0x8319A4B8; continue 'dispatch;
	}
	// 8319A49C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319A4A0: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8319A4A4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8319A4A8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8319A4AC: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8319A4B0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8319A4B4: 409AFFE8  bne cr6, 0x8319a49c
	if !ctx.cr[6].eq {
	pc = 0x8319A49C; continue 'dispatch;
	}
	// 8319A4B8: 54A9E13E  srwi r9, r5, 4
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shr(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8319A4BC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8319A4C0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8319A4C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8319A4C4 size=212
    let mut pc: u32 = 0x8319A4C4;
    'dispatch: loop {
        match pc {
            0x8319A4C4 => {
    //   block [0x8319A4C4..0x8319A598)
	// 8319A4C4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319A4C8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8319A4CC: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8319A4D0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8319A4D4: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319A4D8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8319A4DC: 80CA0000  lwz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319A4E0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8319A4E4: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319A4E8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8319A4EC: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8319A4F0: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 8319A4F4: 90CB0008  stw r6, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 8319A4F8: 90AB000C  stw r5, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 8319A4FC: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319A500: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8319A504: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319A508: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8319A50C: 80CA0000  lwz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319A510: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8319A514: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319A518: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8319A51C: 910B0010  stw r8, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 8319A520: 90EB0014  stw r7, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 8319A524: 90CB0018  stw r6, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[6].u32 ) };
	// 8319A528: 90AB001C  stw r5, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 8319A52C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319A530: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8319A534: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319A538: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8319A53C: 80CA0000  lwz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319A540: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8319A544: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319A548: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8319A54C: 910B0020  stw r8, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 8319A550: 90EB0024  stw r7, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[7].u32 ) };
	// 8319A554: 90CB0028  stw r6, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[6].u32 ) };
	// 8319A558: 90AB002C  stw r5, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[5].u32 ) };
	// 8319A55C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319A560: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8319A564: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319A568: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8319A56C: 80CA0000  lwz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319A570: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8319A574: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319A578: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8319A57C: 910B0030  stw r8, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[8].u32 ) };
	// 8319A580: 90EB0034  stw r7, 0x34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[7].u32 ) };
	// 8319A584: 90CB0038  stw r6, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[6].u32 ) };
	// 8319A588: 90AB003C  stw r5, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[5].u32 ) };
	// 8319A58C: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 8319A590: 409AFF34  bne cr6, 0x8319a4c4
	if !ctx.cr[6].eq {
	pc = 0x8319A4C4; continue 'dispatch;
	}
	// 8319A594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


