pub fn sub_83180BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83180BC0 size=344
    let mut pc: u32 = 0x83180BC0;
    'dispatch: loop {
        match pc {
            0x83180BC0 => {
    //   block [0x83180BC0..0x83180D18)
	// 83180BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83180BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83180BC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83180BCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83180BD0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83180BD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83180BD8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83180BDC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 83180BE0: 809E17D0  lwz r4, 0x17d0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(6096 as u32) ) } as u64;
	// 83180BE4: 4800DA9D  bl 0x8318e680
	ctx.lr = 0x83180BE8;
	sub_8318E680(ctx, base);
	// 83180BE8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83180BEC: 419A000C  beq cr6, 0x83180bf8
	if ctx.cr[6].eq {
	pc = 0x83180BF8; continue 'dispatch;
	}
	// 83180BF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83180BF4: 4800010C  b 0x83180d00
	pc = 0x83180D00; continue 'dispatch;
	// 83180BF8: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83180BFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83180C00: 419AFFF0  beq cr6, 0x83180bf0
	if ctx.cr[6].eq {
	pc = 0x83180BF0; continue 'dispatch;
	}
	// 83180C04: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83180C08: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83180C0C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83180C10: 4BFFDB71  bl 0x8317e780
	ctx.lr = 0x83180C14;
	sub_8317E780(ctx, base);
	// 83180C14: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83180C18: 409A0078  bne cr6, 0x83180c90
	if !ctx.cr[6].eq {
	pc = 0x83180C90; continue 'dispatch;
	}
	// 83180C1C: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 83180C20: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83180C24: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83180C28: 37EBFFFD  addic. r31, r11, -3
	ctx.xer.ca = (ctx.r[11].u32 > (!(-3 as u32)));
	ctx.r[31].s64 = ctx.r[11].s64 + -3;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83180C2C: 41810008  bgt 0x83180c34
	if ctx.cr[0].gt {
	pc = 0x83180C34; continue 'dispatch;
	}
	// 83180C30: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83180C34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83180C38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83180C3C: 4BFFEE3D  bl 0x8317fa78
	ctx.lr = 0x83180C40;
	sub_8317FA78(ctx, base);
	// 83180C40: 80E10064  lwz r7, 0x64(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83180C44: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83180C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83180C4C: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83180C50: 7D275050  subf r9, r7, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 83180C54: 2F1F0003  cmpwi cr6, r31, 3
	ctx.cr[6].compare_i32(ctx.r[31].s32, 3, &mut ctx.xer);
	// 83180C58: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 83180C5C: 41980008  blt cr6, 0x83180c64
	if ctx.cr[6].lt {
	pc = 0x83180C64; continue 'dispatch;
	}
	// 83180C60: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 83180C64: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83180C68: 40980084  bge cr6, 0x83180cec
	if !ctx.cr[6].lt {
	pc = 0x83180CEC; continue 'dispatch;
	}
	// 83180C6C: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 83180C70: 7D4B4214  add r10, r11, r8
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 83180C74: 41980008  blt cr6, 0x83180c7c
	if ctx.cr[6].lt {
	pc = 0x83180C7C; continue 'dispatch;
	}
	// 83180C78: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 83180C7C: 894A0000  lbz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83180C80: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83180C84: 409A0058  bne cr6, 0x83180cdc
	if !ctx.cr[6].eq {
	pc = 0x83180CDC; continue 'dispatch;
	}
	// 83180C88: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83180C8C: 4BFFFFC8  b 0x83180c54
	pc = 0x83180C54; continue 'dispatch;
	// 83180C90: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83180C94: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83180C98: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83180C9C: 41990018  bgt cr6, 0x83180cb4
	if ctx.cr[6].gt {
	pc = 0x83180CB4; continue 'dispatch;
	}
	// 83180CA0: 7D4B4A14  add r10, r11, r9
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 83180CA4: 7F035040  cmplw cr6, r3, r10
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83180CA8: 4098000C  bge cr6, 0x83180cb4
	if !ctx.cr[6].lt {
	pc = 0x83180CB4; continue 'dispatch;
	}
	// 83180CAC: 7FEB1850  subf r31, r11, r3
	ctx.r[31].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83180CB0: 4BFFFF84  b 0x83180c34
	pc = 0x83180C34; continue 'dispatch;
	// 83180CB4: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83180CB8: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83180CBC: 4199FF74  bgt cr6, 0x83180c30
	if ctx.cr[6].gt {
	pc = 0x83180C30; continue 'dispatch;
	}
	// 83180CC0: 8141006C  lwz r10, 0x6c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 83180CC4: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83180CC8: 7F035040  cmplw cr6, r3, r10
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83180CCC: 4098FF64  bge cr6, 0x83180c30
	if !ctx.cr[6].lt {
	pc = 0x83180C30; continue 'dispatch;
	}
	// 83180CD0: 7D6B1850  subf r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83180CD4: 7FEB4A14  add r31, r11, r9
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 83180CD8: 4BFFFF5C  b 0x83180c34
	pc = 0x83180C34; continue 'dispatch;
	// 83180CDC: E95E09B0  ld r10, 0x9b0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(2480 as u32) ) };
	// 83180CE0: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 83180CE4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83180CE8: F97E09B0  std r11, 0x9b0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(2480 as u32), ctx.r[11].u64 ) };
	// 83180CEC: E95E09A8  ld r10, 0x9a8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(2472 as u32) ) };
	// 83180CF0: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 83180CF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83180CF8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83180CFC: F97E09A8  std r11, 0x9a8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(2472 as u32), ctx.r[11].u64 ) };
	// 83180D00: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83180D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83180D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83180D0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83180D10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83180D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83180D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83180D18 size=184
    let mut pc: u32 = 0x83180D18;
    'dispatch: loop {
        match pc {
            0x83180D18 => {
    //   block [0x83180D18..0x83180DD0)
	// 83180D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83180D1C: 48027441  bl 0x831a815c
	ctx.lr = 0x83180D20;
	sub_831A8130(ctx, base);
	// 83180D20: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83180D24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83180D28: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83180D2C: 4099009C  ble cr6, 0x83180dc8
	if !ctx.cr[6].gt {
	pc = 0x83180DC8; continue 'dispatch;
	}
	// 83180D30: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83180D34: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83180D38: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83180D3C: 3B200005  li r25, 5
	ctx.r[25].s64 = 5;
	// 83180D40: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 83180D44: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 83180D48: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83180D4C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83180D50: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83180D54: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 83180D58: B3DF000C  sth r30, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u16 ) };
	// 83180D5C: B3DF000E  sth r30, 0xe(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[30].u16 ) };
	// 83180D60: B3DF0010  sth r30, 0x10(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u16 ) };
	// 83180D64: B3DF0012  sth r30, 0x12(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(18 as u32), ctx.r[30].u16 ) };
	// 83180D68: 933F0014  stw r25, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[25].u32 ) };
	// 83180D6C: 4BFFB79D  bl 0x8317c508
	ctx.lr = 0x83180D70;
	sub_8317C508(ctx, base);
	// 83180D70: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83180D74: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 83180D78: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 83180D7C: 935F0048  stw r26, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[26].u32 ) };
	// 83180D80: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 83180D84: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83180D88: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83180D8C: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 83180D90: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 83180D94: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 83180D98: 937F0064  stw r27, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 83180D9C: 4BFFED4D  bl 0x8317fae8
	ctx.lr = 0x83180DA0;
	sub_8317FAE8(ctx, base);
	// 83180DA0: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 83180DA4: FB7F00E8  std r27, 0xe8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[27].u64 ) };
	// 83180DA8: 93DF00F0  stw r30, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[30].u32 ) };
	// 83180DAC: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 83180DB0: 93DF00F4  stw r30, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[30].u32 ) };
	// 83180DB4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83180DB8: 93DF00F8  stw r30, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[30].u32 ) };
	// 83180DBC: B3DF00FC  sth r30, 0xfc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[30].u16 ) };
	// 83180DC0: 3BFF0100  addi r31, r31, 0x100
	ctx.r[31].s64 = ctx.r[31].s64 + 256;
	// 83180DC4: 409AFF84  bne cr6, 0x83180d48
	if !ctx.cr[6].eq {
	pc = 0x83180D48; continue 'dispatch;
	}
	// 83180DC8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83180DCC: 480273E0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83180DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83180DD0 size=96
    let mut pc: u32 = 0x83180DD0;
    'dispatch: loop {
        match pc {
            0x83180DD0 => {
    //   block [0x83180DD0..0x83180E30)
	// 83180DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83180DD4: 48027399  bl 0x831a816c
	ctx.lr = 0x83180DD8;
	sub_831A8130(ctx, base);
	// 83180DD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83180DDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83180DE0: 83BF17C8  lwz r29, 0x17c8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6088 as u32) ) } as u64;
	// 83180DE4: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83180DE8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83180DEC: 419A0038  beq cr6, 0x83180e24
	if ctx.cr[6].eq {
	pc = 0x83180E24; continue 'dispatch;
	}
	// 83180DF0: 48006D91  bl 0x83187b80
	ctx.lr = 0x83180DF4;
	sub_83187B80(ctx, base);
	// 83180DF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83180DF8: 4BFFED21  bl 0x8317fb18
	ctx.lr = 0x83180DFC;
	sub_8317FB18(ctx, base);
	// 83180DFC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83180E00: 419A001C  beq cr6, 0x83180e1c
	if ctx.cr[6].eq {
	pc = 0x83180E1C; continue 'dispatch;
	}
	// 83180E04: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83180E08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83180E0C: 60840F0C  ori r4, r4, 0xf0c
	ctx.r[4].u64 = ctx.r[4].u64 | 3852;
	// 83180E10: 480066E9  bl 0x831874f8
	ctx.lr = 0x83180E14;
	sub_831874F8(ctx, base);
	// 83180E14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83180E18: 480273A4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83180E1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83180E20: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83180E24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83180E28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83180E2C: 48027390  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83180E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83180E30 size=12
    let mut pc: u32 = 0x83180E30;
    'dispatch: loop {
        match pc {
            0x83180E30 => {
    //   block [0x83180E30..0x83180E3C)
	// 83180E30: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 83180E34: 38CBFB68  addi r6, r11, -0x498
	ctx.r[6].s64 = ctx.r[11].s64 + -1176;
	// 83180E38: 48007680  b 0x831884b8
	sub_831884B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83180E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83180E40 size=164
    let mut pc: u32 = 0x83180E40;
    'dispatch: loop {
        match pc {
            0x83180E40 => {
    //   block [0x83180E40..0x83180EE4)
	// 83180E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83180E44: 48027325  bl 0x831a8168
	ctx.lr = 0x83180E48;
	sub_831A8130(ctx, base);
	// 83180E48: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83180E4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83180E50: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83180E54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83180E58: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83180E5C: 4BFFEE75  bl 0x8317fcd0
	ctx.lr = 0x83180E60;
	sub_8317FCD0(ctx, base);
	// 83180E60: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83180E64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83180E68: 419A0070  beq cr6, 0x83180ed8
	if ctx.cr[6].eq {
	pc = 0x83180ED8; continue 'dispatch;
	}
	// 83180E6C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83180E70: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83180E74: 419A0064  beq cr6, 0x83180ed8
	if ctx.cr[6].eq {
	pc = 0x83180ED8; continue 'dispatch;
	}
	// 83180E78: 394B000C  addi r10, r11, 0xc
	ctx.r[10].s64 = ctx.r[11].s64 + 12;
	// 83180E7C: 83A40000  lwz r29, 0(r4)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83180E80: 387E0DC4  addi r3, r30, 0xdc4
	ctx.r[3].s64 = ctx.r[30].s64 + 3524;
	// 83180E84: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 83180E88: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 83180E8C: 3BEB0038  addi r31, r11, 0x38
	ctx.r[31].s64 = ctx.r[11].s64 + 56;
	// 83180E90: 48027681  bl 0x831a8510
	ctx.lr = 0x83180E94;
	sub_831A8510(ctx, base);
	// 83180E94: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 83180E98: 817F0200  lwz r11, 0x200(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(512 as u32) ) } as u64;
	// 83180E9C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83180EA0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83180EA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83180EA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83180EAC: 480116E5  bl 0x83192590
	ctx.lr = 0x83180EB0;
	sub_83192590(ctx, base);
	// 83180EB0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83180EB4: 419A001C  beq cr6, 0x83180ed0
	if ctx.cr[6].eq {
	pc = 0x83180ED0; continue 'dispatch;
	}
	// 83180EB8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83180EBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83180EC0: 60840F1B  ori r4, r4, 0xf1b
	ctx.r[4].u64 = ctx.r[4].u64 | 3867;
	// 83180EC4: 48006635  bl 0x831874f8
	ctx.lr = 0x83180EC8;
	sub_831874F8(ctx, base);
	// 83180EC8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83180ECC: 480272EC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83180ED0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83180ED4: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83180ED8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83180EDC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83180EE0: 480272D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83180EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83180EE8 size=116
    let mut pc: u32 = 0x83180EE8;
    'dispatch: loop {
        match pc {
            0x83180EE8 => {
    //   block [0x83180EE8..0x83180F5C)
	// 83180EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83180EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83180EF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83180EF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83180EF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83180EFC: 48006ED5  bl 0x83187dd0
	ctx.lr = 0x83180F00;
	sub_83187DD0(ctx, base);
	// 83180F00: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83180F04: 419A001C  beq cr6, 0x83180f20
	if ctx.cr[6].eq {
	pc = 0x83180F20; continue 'dispatch;
	}
	// 83180F08: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83180F0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83180F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83180F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83180F18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83180F1C: 4E800020  blr
	return;
	// 83180F20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83180F24: 4BFFD67D  bl 0x8317e5a0
	ctx.lr = 0x83180F28;
	sub_8317E5A0(ctx, base);
	// 83180F28: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83180F2C: 419A0018  beq cr6, 0x83180f44
	if ctx.cr[6].eq {
	pc = 0x83180F44; continue 'dispatch;
	}
	// 83180F30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83180F34: 4BFFEEAD  bl 0x8317fde0
	ctx.lr = 0x83180F38;
	sub_8317FDE0(ctx, base);
	// 83180F38: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83180F3C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83180F40: 409A0008  bne cr6, 0x83180f48
	if !ctx.cr[6].eq {
	pc = 0x83180F48; continue 'dispatch;
	}
	// 83180F44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83180F48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83180F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83180F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83180F54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83180F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83180F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83180F60 size=552
    let mut pc: u32 = 0x83180F60;
    'dispatch: loop {
        match pc {
            0x83180F60 => {
    //   block [0x83180F60..0x83181188)
	// 83180F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83180F64: 480271FD  bl 0x831a8160
	ctx.lr = 0x83180F68;
	sub_831A8130(ctx, base);
	// 83180F68: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83180F6C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83180F70: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 83180F74: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83180F78: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83180F7C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83180F80: 83BC17D0  lwz r29, 0x17d0(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(6096 as u32) ) } as u64;
	// 83180F84: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 83180F88: 93DA0000  stw r30, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83180F8C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83180F90: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83180F94: 93DB0000  stw r30, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83180F98: 4800D6E9  bl 0x8318e680
	ctx.lr = 0x83180F9C;
	sub_8318E680(ctx, base);
	// 83180F9C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83180FA0: 409A01E0  bne cr6, 0x83181180
	if !ctx.cr[6].eq {
	pc = 0x83181180; continue 'dispatch;
	}
	// 83180FA4: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83180FA8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83180FAC: 419A01D0  beq cr6, 0x8318117c
	if ctx.cr[6].eq {
	pc = 0x8318117C; continue 'dispatch;
	}
	// 83180FB0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83180FB4: 388000CE  li r4, 0xce
	ctx.r[4].s64 = 206;
	// 83180FB8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83180FBC: 4BFFD7C5  bl 0x8317e780
	ctx.lr = 0x83180FC0;
	sub_8317E780(ctx, base);
	// 83180FC0: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83180FC4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83180FC8: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83180FCC: 419A0058  beq cr6, 0x83181024
	if ctx.cr[6].eq {
	pc = 0x83181024; continue 'dispatch;
	}
	// 83180FD0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83180FD4: 419A0014  beq cr6, 0x83180fe8
	if ctx.cr[6].eq {
	pc = 0x83180FE8; continue 'dispatch;
	}
	// 83180FD8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83180FDC: 4BFFD6ED  bl 0x8317e6c8
	ctx.lr = 0x83180FE0;
	sub_8317E6C8(ctx, base);
	// 83180FE0: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83180FE4: 48000020  b 0x83181004
	pc = 0x83181004; continue 'dispatch;
	// 83180FE8: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 83180FEC: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83180FF0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83180FF4: 356BFFFD  addic. r11, r11, -3
	ctx.xer.ca = (ctx.r[11].u32 > (!(-3 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -3;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83180FF8: 40800008  bge 0x83181000
	if !ctx.cr[0].lt {
	pc = 0x83181000; continue 'dispatch;
	}
	// 83180FFC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83181000: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83181004: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83181008: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318100C: 40990170  ble cr6, 0x8318117c
	if !ctx.cr[6].gt {
	pc = 0x8318117C; continue 'dispatch;
	}
	// 83181010: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83181014: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83181018: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318101C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83181020: 48027190  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83181024: 83610058  lwz r27, 0x58(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83181028: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8318102C: 576A0630  rlwinm r10, r27, 0, 0x18, 0x18
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	// 83181030: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83181034: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 83181038: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318103C: 409A0140  bne cr6, 0x8318117c
	if !ctx.cr[6].eq {
	pc = 0x8318117C; continue 'dispatch;
	}
	// 83181040: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 83181044: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83181048: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8318104C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83181050: 4800CD29  bl 0x8318dd78
	ctx.lr = 0x83181054;
	sub_8318DD78(ctx, base);
	// 83181054: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83181058: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8318105C: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83181060: 4BFFEEC1  bl 0x8317ff20
	ctx.lr = 0x83181064;
	sub_8317FF20(ctx, base);
	// 83181064: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83181068: 419A0060  beq cr6, 0x831810c8
	if ctx.cr[6].eq {
	pc = 0x831810C8; continue 'dispatch;
	}
	// 8318106C: 8141006C  lwz r10, 0x6c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 83181070: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83181074: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83181078: 409A0010  bne cr6, 0x83181088
	if !ctx.cr[6].eq {
	pc = 0x83181088; continue 'dispatch;
	}
	// 8318107C: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83181080: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83181084: 48000008  b 0x8318108c
	pc = 0x8318108C; continue 'dispatch;
	// 83181088: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8318108C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83181090: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83181094: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83181098: 419A0080  beq cr6, 0x83181118
	if ctx.cr[6].eq {
	pc = 0x83181118; continue 'dispatch;
	}
	// 8318109C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 831810A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831810A4: 388000CC  li r4, 0xcc
	ctx.r[4].s64 = 204;
	// 831810A8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 831810AC: 4BFFD81D  bl 0x8317e8c8
	ctx.lr = 0x831810B0;
	sub_8317E8C8(ctx, base);
	// 831810B0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 831810B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831810B8: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831810BC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831810C0: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 831810C4: 4800CCFD  bl 0x8318ddc0
	ctx.lr = 0x831810C8;
	sub_8318DDC0(ctx, base);
	// 831810C8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831810CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831810D0: 419A0048  beq cr6, 0x83181118
	if ctx.cr[6].eq {
	pc = 0x83181118; continue 'dispatch;
	}
	// 831810D4: 48011655  bl 0x83192728
	ctx.lr = 0x831810D8;
	sub_83192728(ctx, base);
	// 831810D8: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 831810DC: 419A004C  beq cr6, 0x83181128
	if ctx.cr[6].eq {
	pc = 0x83181128; continue 'dispatch;
	}
	// 831810E0: 2F030008  cmpwi cr6, r3, 8
	ctx.cr[6].compare_i32(ctx.r[3].s32, 8, &mut ctx.xer);
	// 831810E4: 409A0084  bne cr6, 0x83181168
	if !ctx.cr[6].eq {
	pc = 0x83181168; continue 'dispatch;
	}
	// 831810E8: 576B0672  rlwinm r11, r27, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	// 831810EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831810F0: 419A0078  beq cr6, 0x83181168
	if ctx.cr[6].eq {
	pc = 0x83181168; continue 'dispatch;
	}
	// 831810F4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 831810F8: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 831810FC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83181100: 4BFFD681  bl 0x8317e780
	ctx.lr = 0x83181104;
	sub_8317E780(ctx, base);
	// 83181104: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83181108: 419A0010  beq cr6, 0x83181118
	if ctx.cr[6].eq {
	pc = 0x83181118; continue 'dispatch;
	}
	// 8318110C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83181110: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83181114: 409A0054  bne cr6, 0x83181168
	if !ctx.cr[6].eq {
	pc = 0x83181168; continue 'dispatch;
	}
	// 83181118: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8318111C: 4BFFD605  bl 0x8317e720
	ctx.lr = 0x83181120;
	sub_8317E720(ctx, base);
	// 83181120: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83181124: 4802708C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83181128: 736B0048  andi. r11, r27, 0x48
	ctx.r[11].u64 = ctx.r[27].u64 & 72;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318112C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83181130: 419A0038  beq cr6, 0x83181168
	if ctx.cr[6].eq {
	pc = 0x83181168; continue 'dispatch;
	}
	// 83181134: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83181138: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8318113C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83181140: 4BFFD641  bl 0x8317e780
	ctx.lr = 0x83181144;
	sub_8317E780(ctx, base);
	// 83181144: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83181148: 419A0010  beq cr6, 0x83181158
	if ctx.cr[6].eq {
	pc = 0x83181158; continue 'dispatch;
	}
	// 8318114C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83181150: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83181154: 409A0014  bne cr6, 0x83181168
	if !ctx.cr[6].eq {
	pc = 0x83181168; continue 'dispatch;
	}
	// 83181158: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8318115C: 4BFFD5C5  bl 0x8317e720
	ctx.lr = 0x83181160;
	sub_8317E720(ctx, base);
	// 83181160: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83181164: 4802704C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83181168: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8318116C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83181170: 4BFFD559  bl 0x8317e6c8
	ctx.lr = 0x83181174;
	sub_8317E6C8(ctx, base);
	// 83181174: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83181178: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318117C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83181180: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83181184: 4802702C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83181188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83181188 size=240
    let mut pc: u32 = 0x83181188;
    'dispatch: loop {
        match pc {
            0x83181188 => {
    //   block [0x83181188..0x83181278)
	// 83181188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318118C: 48026FE1  bl 0x831a816c
	ctx.lr = 0x83181190;
	sub_831A8130(ctx, base);
	// 83181190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83181194: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83181198: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8318119C: 3BDF0D88  addi r30, r31, 0xd88
	ctx.r[30].s64 = ctx.r[31].s64 + 3464;
	// 831811A0: 83BF17C8  lwz r29, 0x17c8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6088 as u32) ) } as u64;
	// 831811A4: 4BFF826D  bl 0x83179410
	ctx.lr = 0x831811A8;
	sub_83179410(ctx, base);
	// 831811A8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831811AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831811B0: 409A00AC  bne cr6, 0x8318125c
	if !ctx.cr[6].eq {
	pc = 0x8318125C; continue 'dispatch;
	}
	// 831811B4: 4BFFF0CD  bl 0x83180280
	ctx.lr = 0x831811B8;
	sub_83180280(ctx, base);
	// 831811B8: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 831811BC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831811C0: 4099006C  ble cr6, 0x8318122c
	if !ctx.cr[6].gt {
	pc = 0x8318122C; continue 'dispatch;
	}
	// 831811C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 831811C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831811CC: 4800A905  bl 0x8318bad0
	ctx.lr = 0x831811D0;
	sub_8318BAD0(ctx, base);
	// 831811D0: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 831811D4: 395D0010  addi r10, r29, 0x10
	ctx.r[10].s64 = ctx.r[29].s64 + 16;
	// 831811D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831811DC: 917D0010  stw r11, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 831811E0: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 831811E4: 806BA32C  lwz r3, -0x5cd4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 831811E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831811EC: 419A0040  beq cr6, 0x8318122c
	if ctx.cr[6].eq {
	pc = 0x8318122C; continue 'dispatch;
	}
	// 831811F0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831811F4: 396BE728  addi r11, r11, -0x18d8
	ctx.r[11].s64 = ctx.r[11].s64 + -6360;
	// 831811F8: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 831811FC: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 83181200: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 83181204: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 83181208: 914B0024  stw r10, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 8318120C: 395E0164  addi r10, r30, 0x164
	ctx.r[10].s64 = ctx.r[30].s64 + 356;
	// 83181210: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 83181214: 395E0160  addi r10, r30, 0x160
	ctx.r[10].s64 = ctx.r[30].s64 + 352;
	// 83181218: 914B003C  stw r10, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 8318121C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83181220: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 83181224: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83181228: 4E800421  bctrl
	ctx.lr = 0x8318122C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318122C: 3C807FFF  lis r4, 0x7fff
	ctx.r[4].s64 = 2147418112;
	// 83181230: 387E003C  addi r3, r30, 0x3c
	ctx.r[3].s64 = ctx.r[30].s64 + 60;
	// 83181234: 6084FFFF  ori r4, r4, 0xffff
	ctx.r[4].u64 = ctx.r[4].u64 | 65535;
	// 83181238: 4BFFB2D1  bl 0x8317c508
	ctx.lr = 0x8318123C;
	sub_8317C508(ctx, base);
	// 8318123C: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 83181240: 387E0068  addi r3, r30, 0x68
	ctx.r[3].s64 = ctx.r[30].s64 + 104;
	// 83181244: 4BFFB2C5  bl 0x8317c508
	ctx.lr = 0x83181248;
	sub_8317C508(ctx, base);
	// 83181248: 396000C0  li r11, 0xc0
	ctx.r[11].s64 = 192;
	// 8318124C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83181250: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83181254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83181258: 48026F64  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8318125C: 4BFFD86D  bl 0x8317eac8
	ctx.lr = 0x83181260;
	sub_8317EAC8(ctx, base);
	// 83181260: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83181264: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83181268: 4098FF58  bge cr6, 0x831811c0
	if !ctx.cr[6].lt {
	pc = 0x831811C0; continue 'dispatch;
	}
	// 8318126C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83181270: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83181274: 48026F48  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83181278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83181278 size=120
    let mut pc: u32 = 0x83181278;
    'dispatch: loop {
        match pc {
            0x83181278 => {
    //   block [0x83181278..0x831812F0)
	// 83181278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318127C: 48026EE5  bl 0x831a8160
	ctx.lr = 0x83181280;
	sub_831A8130(ctx, base);
	// 83181280: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83181284: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83181288: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8318128C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83181290: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 83181294: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83181298: 835F17C8  lwz r26, 0x17c8(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6088 as u32) ) } as u64;
	// 8318129C: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 831812A0: 809F17D0  lwz r4, 0x17d0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6096 as u32) ) } as u64;
	// 831812A4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 831812A8: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 831812AC: F97D0000  std r11, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 831812B0: 419A0038  beq cr6, 0x831812e8
	if ctx.cr[6].eq {
	pc = 0x831812E8; continue 'dispatch;
	}
	// 831812B4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 831812B8: 4800DA71  bl 0x8318ed28
	ctx.lr = 0x831812BC;
	sub_8318ED28(ctx, base);
	// 831812BC: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831812C0: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 831812C4: 41980024  blt cr6, 0x831812e8
	if ctx.cr[6].lt {
	pc = 0x831812E8; continue 'dispatch;
	}
	// 831812C8: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 831812CC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 831812D0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 831812D4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 831812D8: 389A00A0  addi r4, r26, 0xa0
	ctx.r[4].s64 = ctx.r[26].s64 + 160;
	// 831812DC: 387F0D88  addi r3, r31, 0xd88
	ctx.r[3].s64 = ctx.r[31].s64 + 3464;
	// 831812E0: 4BFFF159  bl 0x83180438
	ctx.lr = 0x831812E4;
	sub_83180438(ctx, base);
	// 831812E4: F87E0000  std r3, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 831812E8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831812EC: 48026EC4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831812F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831812F0 size=160
    let mut pc: u32 = 0x831812F0;
    'dispatch: loop {
        match pc {
            0x831812F0 => {
    //   block [0x831812F0..0x83181390)
	// 831812F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831812F4: 48026E75  bl 0x831a8168
	ctx.lr = 0x831812F8;
	sub_831A8130(ctx, base);
	// 831812F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831812FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83181300: 38800034  li r4, 0x34
	ctx.r[4].s64 = 52;
	// 83181304: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83181308: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318130C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83181310: 4BFF8101  bl 0x83179410
	ctx.lr = 0x83181314;
	sub_83179410(ctx, base);
	// 83181314: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83181318: 409A0054  bne cr6, 0x8318136c
	if !ctx.cr[6].eq {
	pc = 0x8318136C; continue 'dispatch;
	}
	// 8318131C: 2F3D0000  cmpdi cr6, r29, 0
	ctx.cr[6].compare_i64(ctx.r[29].s64, 0, &mut ctx.xer);
	// 83181320: 40980038  bge cr6, 0x83181358
	if !ctx.cr[6].lt {
	pc = 0x83181358; continue 'dispatch;
	}
	// 83181324: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83181328: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318132C: 419A002C  beq cr6, 0x83181358
	if ctx.cr[6].eq {
	pc = 0x83181358; continue 'dispatch;
	}
	// 83181330: 897E0057  lbz r11, 0x57(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(87 as u32) ) } as u64;
	// 83181334: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83181338: 409A0020  bne cr6, 0x83181358
	if !ctx.cr[6].eq {
	pc = 0x83181358; continue 'dispatch;
	}
	// 8318133C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83181340: 419A0048  beq cr6, 0x83181388
	if ctx.cr[6].eq {
	pc = 0x83181388; continue 'dispatch;
	}
	// 83181344: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83181348: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318134C: 4BFFD9AD  bl 0x8317ecf8
	ctx.lr = 0x83181350;
	sub_8317ECF8(ctx, base);
	// 83181350: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83181354: 419A0034  beq cr6, 0x83181388
	if ctx.cr[6].eq {
	pc = 0x83181388; continue 'dispatch;
	}
	// 83181358: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8318135C: 38800034  li r4, 0x34
	ctx.r[4].s64 = 52;
	// 83181360: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181364: 4BFF80F5  bl 0x83179458
	ctx.lr = 0x83181368;
	sub_83179458(ctx, base);
	// 83181368: 4800000C  b 0x83181374
	pc = 0x83181374; continue 'dispatch;
	// 8318136C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83181370: 409A0018  bne cr6, 0x83181388
	if !ctx.cr[6].eq {
	pc = 0x83181388; continue 'dispatch;
	}
	// 83181374: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83181378: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8318137C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83181380: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181384: 4BFFF41D  bl 0x831807a0
	ctx.lr = 0x83181388;
	sub_831807A0(ctx, base);
	// 83181388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318138C: 48026E2C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83181390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83181390 size=788
    let mut pc: u32 = 0x83181390;
    'dispatch: loop {
        match pc {
            0x83181390 => {
    //   block [0x83181390..0x831816A4)
	// 83181390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83181394: 48026DBD  bl 0x831a8150
	ctx.lr = 0x83181398;
	sub_831A8130(ctx, base);
	// 83181398: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318139C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831813A0: 81240004  lwz r9, 4(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 831813A4: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831813A8: 397F1F38  addi r11, r31, 0x1f38
	ctx.r[11].s64 = ctx.r[31].s64 + 7992;
	// 831813AC: 38C9000F  addi r6, r9, 0xf
	ctx.r[6].s64 = ctx.r[9].s64 + 15;
	// 831813B0: 38EA000F  addi r7, r10, 0xf
	ctx.r[7].s64 = ctx.r[10].s64 + 15;
	// 831813B4: 83DF17C8  lwz r30, 0x17c8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6088 as u32) ) } as u64;
	// 831813B8: 831F1F78  lwz r24, 0x1f78(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8056 as u32) ) } as u64;
	// 831813BC: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831813C0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 831813C4: 3928000F  addi r9, r8, 0xf
	ctx.r[9].s64 = ctx.r[8].s64 + 15;
	// 831813C8: 808B001C  lwz r4, 0x1c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831813CC: 394A000F  addi r10, r10, 0xf
	ctx.r[10].s64 = ctx.r[10].s64 + 15;
	// 831813D0: 7D292670  srawi r9, r9, 4
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 4) as i64;
	// 831813D4: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 831813D8: 7D482670  srawi r8, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 831813DC: 552A2036  slwi r10, r9, 4
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831813E0: 7D280194  addze r9, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[9].s64 = tmp.s64;
	// 831813E4: 7CE82670  srawi r8, r7, 4
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[7].s32 >> 4) as i64;
	// 831813E8: 38EA007F  addi r7, r10, 0x7f
	ctx.r[7].s64 = ctx.r[10].s64 + 127;
	// 831813EC: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 831813F0: 55252036  slwi r5, r9, 4
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831813F4: 55082036  slwi r8, r8, 4
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831813F8: 7D030E70  srawi r3, r8, 1
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[8].s32 >> 1) as i64;
	// 831813FC: 3BA8007F  addi r29, r8, 0x7f
	ctx.r[29].s64 = ctx.r[8].s64 + 127;
	// 83181400: 7D030194  addze r8, r3
	tmp.s64 = ctx.r[3].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[3].u32);
	ctx.r[8].s64 = tmp.s64;
	// 83181404: 3908007F  addi r8, r8, 0x7f
	ctx.r[8].s64 = ctx.r[8].s64 + 127;
	// 83181408: 7D083E70  srawi r8, r8, 7
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 7) as i64;
	// 8318140C: 7C680194  addze r3, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[3].s64 = tmp.s64;
	// 83181410: 7CC82670  srawi r8, r6, 4
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[6].s32 >> 4) as i64;
	// 83181414: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 83181418: 55062036  slwi r6, r8, 4
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8318141C: 7CC60E70  srawi r6, r6, 1
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[6].s32 >> 1) as i64;
	// 83181420: 7CC60194  addze r6, r6
	tmp.s64 = ctx.r[6].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[6].u32);
	ctx.r[6].s64 = tmp.s64;
	// 83181424: 7FBD3E70  srawi r29, r29, 7
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[29].s32 >> 7) as i64;
	// 83181428: 7F8619D6  mullw r28, r6, r3
	ctx.r[28].s64 = (ctx.r[6].s32 as i64) * (ctx.r[3].s32 as i64);
	// 8318142C: 7F7D0194  addze r27, r29
	tmp.s64 = ctx.r[29].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[29].u32);
	ctx.r[27].s64 = tmp.s64;
	// 83181430: 7D4A0E70  srawi r10, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 83181434: 7F5B41D6  mullw r26, r27, r8
	ctx.r[26].s64 = (ctx.r[27].s32 as i64) * (ctx.r[8].s32 as i64);
	// 83181438: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8318143C: 394A007F  addi r10, r10, 0x7f
	ctx.r[10].s64 = ctx.r[10].s64 + 127;
	// 83181440: 7D4A3E70  srawi r10, r10, 7
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 7) as i64;
	// 83181444: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 83181448: 7CA60E70  srawi r6, r5, 1
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[5].s32 >> 1) as i64;
	// 8318144C: 7CC60194  addze r6, r6
	tmp.s64 = ctx.r[6].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[6].u32);
	ctx.r[6].s64 = tmp.s64;
	// 83181450: 7CE73E70  srawi r7, r7, 7
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 7) as i64;
	// 83181454: 7D4A31D6  mullw r10, r10, r6
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[6].s32 as i64);
	// 83181458: 7CE70194  addze r7, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[7].s64 = tmp.s64;
	// 8318145C: 7D2749D6  mullw r9, r7, r9
	ctx.r[9].s64 = (ctx.r[7].s32 as i64) * (ctx.r[9].s32 as i64);
	// 83181460: 55291838  slwi r9, r9, 3
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83181464: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83181468: 554A482C  slwi r10, r10, 9
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(9);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318146C: 38EA0100  addi r7, r10, 0x100
	ctx.r[7].s64 = ctx.r[10].s64 + 256;
	// 83181470: 574A1838  slwi r10, r26, 3
	ctx.r[10].u32 = ctx.r[26].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83181474: 7D4AE214  add r10, r10, r28
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 83181478: 5549482C  slwi r9, r10, 9
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(9);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318147C: 39290100  addi r9, r9, 0x100
	ctx.r[9].s64 = ctx.r[9].s64 + 256;
	// 83181480: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 83181484: 4099001C  ble cr6, 0x831814a0
	if !ctx.cr[6].gt {
	pc = 0x831814A0; continue 'dispatch;
	}
	// 83181488: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318148C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181490: 60840F17  ori r4, r4, 0xf17
	ctx.r[4].u64 = ctx.r[4].u64 | 3863;
	// 83181494: 48006065  bl 0x831874f8
	ctx.lr = 0x83181498;
	sub_831874F8(ctx, base);
	// 83181498: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8318149C: 48026D04  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
	// 831814A0: 832B0020  lwz r25, 0x20(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 831814A4: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 831814A8: 409A000C  bne cr6, 0x831814b4
	if !ctx.cr[6].eq {
	pc = 0x831814B4; continue 'dispatch;
	}
	// 831814AC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831814B0: 48000100  b 0x831815b0
	pc = 0x831815B0; continue 'dispatch;
	// 831814B4: 813F1FB8  lwz r9, 0x1fb8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8120 as u32) ) } as u64;
	// 831814B8: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 831814BC: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 831814C0: 419800A0  blt cr6, 0x83181560
	if ctx.cr[6].lt {
	pc = 0x83181560; continue 'dispatch;
	}
	// 831814C4: 80EB0008  lwz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831814C8: 5549402E  slwi r9, r10, 8
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831814CC: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 831814D0: 38E7000F  addi r7, r7, 0xf
	ctx.r[7].s64 = ctx.r[7].s64 + 15;
	// 831814D4: 38C90080  addi r6, r9, 0x80
	ctx.r[6].s64 = ctx.r[9].s64 + 128;
	// 831814D8: 7CE72670  srawi r7, r7, 4
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 4) as i64;
	// 831814DC: 3908000F  addi r8, r8, 0xf
	ctx.r[8].s64 = ctx.r[8].s64 + 15;
	// 831814E0: 7D270194  addze r9, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[9].s64 = tmp.s64;
	// 831814E4: 7D082670  srawi r8, r8, 4
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 4) as i64;
	// 831814E8: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831814EC: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 831814F0: 7D250E70  srawi r5, r9, 1
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[9].s32 >> 1) as i64;
	// 831814F4: 55172036  slwi r23, r8, 4
	ctx.r[23].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[23].u64 = ctx.r[23].u32 as u64;
	// 831814F8: 7CA50194  addze r5, r5
	tmp.s64 = ctx.r[5].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[5].u32);
	ctx.r[5].s64 = tmp.s64;
	// 831814FC: 3929007F  addi r9, r9, 0x7f
	ctx.r[9].s64 = ctx.r[9].s64 + 127;
	// 83181500: 38A5007F  addi r5, r5, 0x7f
	ctx.r[5].s64 = ctx.r[5].s64 + 127;
	// 83181504: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 83181508: 7CA53E70  srawi r5, r5, 7
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[5].s32 >> 7) as i64;
	// 8318150C: 7CA50194  addze r5, r5
	tmp.s64 = ctx.r[5].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[5].u32);
	ctx.r[5].s64 = tmp.s64;
	// 83181510: 7EF70E70  srawi r23, r23, 1
	ctx.xer.ca = (ctx.r[23].s32 < 0) && ((ctx.r[23].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[23].s64 = (ctx.r[23].s32 >> 1) as i64;
	// 83181514: 7EF70194  addze r23, r23
	tmp.s64 = ctx.r[23].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[23].u32);
	ctx.r[23].s64 = tmp.s64;
	// 83181518: 7D363E70  srawi r22, r9, 7
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[22].s64 = (ctx.r[9].s32 >> 7) as i64;
	// 8318151C: 7D25B9D6  mullw r9, r5, r23
	ctx.r[9].s64 = (ctx.r[5].s32 as i64) * (ctx.r[23].s32 as i64);
	// 83181520: 7CB60194  addze r5, r22
	tmp.s64 = ctx.r[22].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[22].u32);
	ctx.r[5].s64 = tmp.s64;
	// 83181524: 7D0541D6  mullw r8, r5, r8
	ctx.r[8].s64 = (ctx.r[5].s32 as i64) * (ctx.r[8].s32 as i64);
	// 83181528: 55081838  slwi r8, r8, 3
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318152C: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 83181530: 5529402E  slwi r9, r9, 8
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83181534: 39290080  addi r9, r9, 0x80
	ctx.r[9].s64 = ctx.r[9].s64 + 128;
	// 83181538: 7D2921D6  mullw r9, r9, r4
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[4].s32 as i64);
	// 8318153C: 7F074800  cmpw cr6, r7, r9
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83181540: 4199001C  bgt cr6, 0x8318155c
	if ctx.cr[6].gt {
	pc = 0x8318155C; continue 'dispatch;
	}
	// 83181544: 811F1FB8  lwz r8, 0x1fb8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8120 as u32) ) } as u64;
	// 83181548: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8318154C: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 83181550: 7F1D4000  cmpw cr6, r29, r8
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83181554: 4099FFE8  ble cr6, 0x8318153c
	if !ctx.cr[6].gt {
	pc = 0x8318153C; continue 'dispatch;
	}
	// 83181558: 48000008  b 0x83181560
	pc = 0x83181560; continue 'dispatch;
	// 8318155C: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 83181560: 7F1D2000  cmpw cr6, r29, r4
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[4].s32, &mut ctx.xer);
	// 83181564: 4198FF24  blt cr6, 0x83181488
	if ctx.cr[6].lt {
	pc = 0x83181488; continue 'dispatch;
	}
	// 83181568: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318156C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83181570: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83181574: 390A0080  addi r8, r10, 0x80
	ctx.r[8].s64 = ctx.r[10].s64 + 128;
	// 83181578: 7D485A14  add r10, r8, r11
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8318157C: 917F1F5C  stw r11, 0x1f5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8028 as u32), ctx.r[11].u32 ) };
	// 83181580: 915F1F60  stw r10, 0x1f60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8032 as u32), ctx.r[10].u32 ) };
	// 83181584: 4099002C  ble cr6, 0x831815b0
	if !ctx.cr[6].gt {
	pc = 0x831815B0; continue 'dispatch;
	}
	// 83181588: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8318158C: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 83181590: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83181594: 80FF1F64  lwz r7, 0x1f64(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8036 as u32) ) } as u64;
	// 83181598: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8318159C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831815A0: 7D49392E  stwx r10, r9, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32), ctx.r[10].u32) };
	// 831815A4: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 831815A8: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 831815AC: 409AFFE8  bne cr6, 0x83181594
	if !ctx.cr[6].eq {
	pc = 0x83181594; continue 'dispatch;
	}
	// 831815B0: 576B3830  slwi r11, r27, 7
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831815B4: 546A3830  slwi r10, r3, 7
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(7);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831815B8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 831815BC: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 831815C0: 57495828  slwi r9, r26, 0xb
	ctx.r[9].u32 = ctx.r[26].u32.wrapping_shl(11);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831815C4: 57883830  slwi r8, r28, 7
	ctx.r[8].u32 = ctx.r[28].u32.wrapping_shl(7);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831815C8: 389F1F5C  addi r4, r31, 0x1f5c
	ctx.r[4].s64 = ctx.r[31].s64 + 8028;
	// 831815CC: B17E00D6  sth r11, 0xd6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(214 as u32), ctx.r[11].u16 ) };
	// 831815D0: B15E00D4  sth r10, 0xd4(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(212 as u32), ctx.r[10].u16 ) };
	// 831815D4: 80FF1F5C  lwz r7, 0x1f5c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8028 as u32) ) } as u64;
	// 831815D8: B17E00E6  sth r11, 0xe6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(230 as u32), ctx.r[11].u16 ) };
	// 831815DC: 7D693A14  add r11, r9, r7
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 831815E0: B15E00E4  sth r10, 0xe4(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(228 as u32), ctx.r[10].u16 ) };
	// 831815E4: 7D4B4214  add r10, r11, r8
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 831815E8: 90FE00D0  stw r7, 0xd0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(208 as u32), ctx.r[7].u32 ) };
	// 831815EC: 917E00C8  stw r11, 0xc8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 831815F0: 915E00CC  stw r10, 0xcc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(204 as u32), ctx.r[10].u32 ) };
	// 831815F4: 817F1F60  lwz r11, 0x1f60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8032 as u32) ) } as u64;
	// 831815F8: 7D4B4A14  add r10, r11, r9
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 831815FC: 917E00E0  stw r11, 0xe0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 83181600: 7D6A4214  add r11, r10, r8
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 83181604: 915E00D8  stw r10, 0xd8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(216 as u32), ctx.r[10].u32 ) };
	// 83181608: 917E00DC  stw r11, 0xdc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(220 as u32), ctx.r[11].u32 ) };
	// 8318160C: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83181610: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83181614: 817F1FB8  lwz r11, 0x1fb8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8120 as u32) ) } as u64;
	// 83181618: 409A0060  bne cr6, 0x83181678
	if !ctx.cr[6].eq {
	pc = 0x83181678; continue 'dispatch;
	}
	// 8318161C: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 83181620: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83181624: 41980008  blt cr6, 0x8318162c
	if ctx.cr[6].lt {
	pc = 0x8318162C; continue 'dispatch;
	}
	// 83181628: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 8318162C: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 83181630: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83181634: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83181638: 917F1F68  stw r11, 0x1f68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8040 as u32), ctx.r[11].u32 ) };
	// 8318163C: 4BFFF6DD  bl 0x83180d18
	ctx.lr = 0x83181640;
	sub_83180D18(ctx, base);
	// 83181640: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83181644: 809F1F64  lwz r4, 0x1f64(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8036 as u32) ) } as u64;
	// 83181648: 38780200  addi r3, r24, 0x200
	ctx.r[3].s64 = ctx.r[24].s64 + 512;
	// 8318164C: 4BFFF6CD  bl 0x83180d18
	ctx.lr = 0x83181650;
	sub_83180D18(ctx, base);
	// 83181650: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181654: 48006835  bl 0x83187e88
	ctx.lr = 0x83181658;
	sub_83187E88(ctx, base);
	// 83181658: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318165C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181660: 917E00E8  stw r11, 0xe8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(232 as u32), ctx.r[11].u32 ) };
	// 83181664: 48006825  bl 0x83187e88
	ctx.lr = 0x83181668;
	sub_83187E88(ctx, base);
	// 83181668: 907E00EC  stw r3, 0xec(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(236 as u32), ctx.r[3].u32 ) };
	// 8318166C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83181670: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83181674: 48026B2C  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
	// 83181678: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8318167C: 41980008  blt cr6, 0x83181684
	if ctx.cr[6].lt {
	pc = 0x83181684; continue 'dispatch;
	}
	// 83181680: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 83181684: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83181688: 809F1F64  lwz r4, 0x1f64(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8036 as u32) ) } as u64;
	// 8318168C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83181690: 93BF1F68  stw r29, 0x1f68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8040 as u32), ctx.r[29].u32 ) };
	// 83181694: 4BFFF685  bl 0x83180d18
	ctx.lr = 0x83181698;
	sub_83180D18(ctx, base);
	// 83181698: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318169C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 831816A0: 48026B00  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831816A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831816A8 size=608
    let mut pc: u32 = 0x831816A8;
    'dispatch: loop {
        match pc {
            0x831816A8 => {
    //   block [0x831816A8..0x83181908)
	// 831816A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831816AC: 48026ABD  bl 0x831a8168
	ctx.lr = 0x831816B0;
	sub_831A8130(ctx, base);
	// 831816B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831816B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831816B8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831816BC: 3880002F  li r4, 0x2f
	ctx.r[4].s64 = 47;
	// 831816C0: 83DF17C8  lwz r30, 0x17c8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6088 as u32) ) } as u64;
	// 831816C4: 3B9E0014  addi r28, r30, 0x14
	ctx.r[28].s64 = ctx.r[30].s64 + 20;
	// 831816C8: 4BFF7D49  bl 0x83179410
	ctx.lr = 0x831816CC;
	sub_83179410(ctx, base);
	// 831816CC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831816D0: 409A000C  bne cr6, 0x831816dc
	if !ctx.cr[6].eq {
	pc = 0x831816DC; continue 'dispatch;
	}
	// 831816D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831816D8: 48026AE0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 831816DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831816E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831816E4: 48003715  bl 0x83184df8
	ctx.lr = 0x831816E8;
	sub_83184DF8(ctx, base);
	// 831816E8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831816EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831816F0: 409A0210  bne cr6, 0x83181900
	if !ctx.cr[6].eq {
	pc = 0x83181900; continue 'dispatch;
	}
	// 831816F4: 38800027  li r4, 0x27
	ctx.r[4].s64 = 39;
	// 831816F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831816FC: 4BFF7D15  bl 0x83179410
	ctx.lr = 0x83181700;
	sub_83179410(ctx, base);
	// 83181700: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83181704: 409A0010  bne cr6, 0x83181714
	if !ctx.cr[6].eq {
	pc = 0x83181714; continue 'dispatch;
	}
	// 83181708: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318170C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83181710: 48026AA8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83181714: 897C0058  lbz r11, 0x58(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(88 as u32) ) } as u64;
	// 83181718: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318171C: 419A0010  beq cr6, 0x8318172c
	if ctx.cr[6].eq {
	pc = 0x8318172C; continue 'dispatch;
	}
	// 83181720: 807E00F4  lwz r3, 0xf4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(244 as u32) ) } as u64;
	// 83181724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83181728: 48026A90  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8318172C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181730: 83DC0018  lwz r30, 0x18(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83181734: 4BFFDCAD  bl 0x8317f3e0
	ctx.lr = 0x83181738;
	sub_8317F3E0(ctx, base);
	// 83181738: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318173C: 419A0048  beq cr6, 0x83181784
	if ctx.cr[6].eq {
	pc = 0x83181784; continue 'dispatch;
	}
	// 83181740: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83181744: 806BA32C  lwz r3, -0x5cd4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83181748: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318174C: 419A0030  beq cr6, 0x8318177c
	if ctx.cr[6].eq {
	pc = 0x8318177C; continue 'dispatch;
	}
	// 83181750: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83181754: 394BAF08  addi r10, r11, -0x50f8
	ctx.r[10].s64 = ctx.r[11].s64 + -20728;
	// 83181758: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8318175C: 396BE5D8  addi r11, r11, -0x1a28
	ctx.r[11].s64 = ctx.r[11].s64 + -6696;
	// 83181760: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 83181764: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 83181768: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8318176C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83181770: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 83181774: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83181778: 4E800421  bctrl
	ctx.lr = 0x8318177C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318177C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 83181780: 4800016C  b 0x831818ec
	pc = 0x831818EC; continue 'dispatch;
	// 83181784: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83181788: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318178C: 4BFFDCBD  bl 0x8317f448
	ctx.lr = 0x83181790;
	sub_8317F448(ctx, base);
	// 83181790: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83181794: 419A0048  beq cr6, 0x831817dc
	if ctx.cr[6].eq {
	pc = 0x831817DC; continue 'dispatch;
	}
	// 83181798: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318179C: 806BA32C  lwz r3, -0x5cd4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 831817A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831817A4: 419A0030  beq cr6, 0x831817d4
	if ctx.cr[6].eq {
	pc = 0x831817D4; continue 'dispatch;
	}
	// 831817A8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831817AC: 394BAF00  addi r10, r11, -0x5100
	ctx.r[10].s64 = ctx.r[11].s64 + -20736;
	// 831817B0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831817B4: 396BE5D8  addi r11, r11, -0x1a28
	ctx.r[11].s64 = ctx.r[11].s64 + -6696;
	// 831817B8: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 831817BC: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 831817C0: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 831817C4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831817C8: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 831817CC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831817D0: 4E800421  bctrl
	ctx.lr = 0x831817D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831817D4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 831817D8: 48000114  b 0x831818ec
	pc = 0x831818EC; continue 'dispatch;
	// 831817DC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831817E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831817E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831817E8: 4BFFDCB1  bl 0x8317f498
	ctx.lr = 0x831817EC;
	sub_8317F498(ctx, base);
	// 831817EC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831817F0: 419A0048  beq cr6, 0x83181838
	if ctx.cr[6].eq {
	pc = 0x83181838; continue 'dispatch;
	}
	// 831817F4: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 831817F8: 806BA32C  lwz r3, -0x5cd4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 831817FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83181800: 419A0030  beq cr6, 0x83181830
	if ctx.cr[6].eq {
	pc = 0x83181830; continue 'dispatch;
	}
	// 83181804: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83181808: 394BAEF8  addi r10, r11, -0x5108
	ctx.r[10].s64 = ctx.r[11].s64 + -20744;
	// 8318180C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83181810: 396BE5D8  addi r11, r11, -0x1a28
	ctx.r[11].s64 = ctx.r[11].s64 + -6696;
	// 83181814: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 83181818: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8318181C: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 83181820: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83181824: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 83181828: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8318182C: 4E800421  bctrl
	ctx.lr = 0x83181830;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83181830: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 83181834: 480000B8  b 0x831818ec
	pc = 0x831818EC; continue 'dispatch;
	// 83181838: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318183C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181840: 4BFFDA99  bl 0x8317f2d8
	ctx.lr = 0x83181844;
	sub_8317F2D8(ctx, base);
	// 83181844: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83181848: 419A0048  beq cr6, 0x83181890
	if ctx.cr[6].eq {
	pc = 0x83181890; continue 'dispatch;
	}
	// 8318184C: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83181850: 806BA32C  lwz r3, -0x5cd4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83181854: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83181858: 419A0030  beq cr6, 0x83181888
	if ctx.cr[6].eq {
	pc = 0x83181888; continue 'dispatch;
	}
	// 8318185C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83181860: 394BAEF0  addi r10, r11, -0x5110
	ctx.r[10].s64 = ctx.r[11].s64 + -20752;
	// 83181864: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83181868: 396BE5D8  addi r11, r11, -0x1a28
	ctx.r[11].s64 = ctx.r[11].s64 + -6696;
	// 8318186C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 83181870: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 83181874: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 83181878: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318187C: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 83181880: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83181884: 4E800421  bctrl
	ctx.lr = 0x83181888;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83181888: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8318188C: 48000060  b 0x831818ec
	pc = 0x831818EC; continue 'dispatch;
	// 83181890: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83181894: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181898: 4BFFF0B9  bl 0x83180950
	ctx.lr = 0x8318189C;
	sub_83180950(ctx, base);
	// 8318189C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831818A0: 419A0048  beq cr6, 0x831818e8
	if ctx.cr[6].eq {
	pc = 0x831818E8; continue 'dispatch;
	}
	// 831818A4: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 831818A8: 806BA32C  lwz r3, -0x5cd4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 831818AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831818B0: 419A0030  beq cr6, 0x831818e0
	if ctx.cr[6].eq {
	pc = 0x831818E0; continue 'dispatch;
	}
	// 831818B4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831818B8: 394BAEE8  addi r10, r11, -0x5118
	ctx.r[10].s64 = ctx.r[11].s64 + -20760;
	// 831818BC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831818C0: 396BE5D8  addi r11, r11, -0x1a28
	ctx.r[11].s64 = ctx.r[11].s64 + -6696;
	// 831818C4: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 831818C8: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 831818CC: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 831818D0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831818D4: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 831818D8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831818DC: 4E800421  bctrl
	ctx.lr = 0x831818E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831818E0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 831818E4: 48000008  b 0x831818ec
	pc = 0x831818EC; continue 'dispatch;
	// 831818E8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831818EC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 831818F0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831818F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831818F8: 4BFFDA19  bl 0x8317f310
	ctx.lr = 0x831818FC;
	sub_8317F310(ctx, base);
	// 831818FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83181900: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83181904: 480268B4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83181908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83181908 size=756
    let mut pc: u32 = 0x83181908;
    'dispatch: loop {
        match pc {
            0x83181908 => {
    //   block [0x83181908..0x83181BFC)
	// 83181908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318190C: 4802683D  bl 0x831a8148
	ctx.lr = 0x83181910;
	sub_831A8130(ctx, base);
	// 83181910: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83181914: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83181918: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8318191C: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 83181920: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 83181924: 3A9F0950  addi r20, r31, 0x950
	ctx.r[20].s64 = ctx.r[31].s64 + 2384;
	// 83181928: 83BF17C8  lwz r29, 0x17c8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6088 as u32) ) } as u64;
	// 8318192C: 3ADD0014  addi r22, r29, 0x14
	ctx.r[22].s64 = ctx.r[29].s64 + 20;
	// 83181930: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 83181934: 82BD0000  lwz r21, 0(r29)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83181938: 4BFFDC51  bl 0x8317f588
	ctx.lr = 0x8318193C;
	sub_8317F588(ctx, base);
	// 8318193C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83181940: 409A02B0  bne cr6, 0x83181bf0
	if !ctx.cr[6].eq {
	pc = 0x83181BF0; continue 'dispatch;
	}
	// 83181944: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 83181948: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8318194C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83181950: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181954: 4BFFDF05  bl 0x8317f858
	ctx.lr = 0x83181958;
	sub_8317F858(ctx, base);
	// 83181958: 83810054  lwz r28, 0x54(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8318195C: 389F1F88  addi r4, r31, 0x1f88
	ctx.r[4].s64 = ctx.r[31].s64 + 8072;
	// 83181960: 807C0060  lwz r3, 0x60(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(96 as u32) ) } as u64;
	// 83181964: 4BFFE0C5  bl 0x8317fa28
	ctx.lr = 0x83181968;
	sub_8317FA28(ctx, base);
	// 83181968: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318196C: 4BFFDF6D  bl 0x8317f8d8
	ctx.lr = 0x83181970;
	sub_8317F8D8(ctx, base);
	// 83181970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181974: 48011ADD  bl 0x83193450
	ctx.lr = 0x83181978;
	sub_83193450(ctx, base);
	// 83181978: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 8318197C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83181980: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83181984: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83181988: 4BFB2DC1  bl 0x83134748
	ctx.lr = 0x8318198C;
	sub_83134748(ctx, base);
	// 8318198C: 3F408345  lis r26, -0x7cbb
	ctx.r[26].s64 = -2092630016;
	// 83181990: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 83181994: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83181998: 3BCBE420  addi r30, r11, -0x1be0
	ctx.r[30].s64 = ctx.r[11].s64 + -7136;
	// 8318199C: 807AA32C  lwz r3, -0x5cd4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 831819A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831819A4: 419A0034  beq cr6, 0x831819d8
	if ctx.cr[6].eq {
	pc = 0x831819D8; continue 'dispatch;
	}
	// 831819A8: 39760018  addi r11, r22, 0x18
	ctx.r[11].s64 = ctx.r[22].s64 + 24;
	// 831819AC: 92BE000C  stw r21, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[21].u32 ) };
	// 831819B0: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 831819B4: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 831819B8: 39760030  addi r11, r22, 0x30
	ctx.r[11].s64 = ctx.r[22].s64 + 48;
	// 831819BC: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 831819C0: 39760068  addi r11, r22, 0x68
	ctx.r[11].s64 = ctx.r[22].s64 + 104;
	// 831819C4: 917E0030  stw r11, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 831819C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831819CC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 831819D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831819D4: 4E800421  bctrl
	ctx.lr = 0x831819D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831819D8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 831819DC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 831819E0: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 831819E4: 4801176D  bl 0x83193150
	ctx.lr = 0x831819E8;
	sub_83193150(ctx, base);
	// 831819E8: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 831819EC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 831819F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831819F4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 831819F8: 4BFB2D51  bl 0x83134748
	ctx.lr = 0x831819FC;
	sub_83134748(ctx, base);
	// 831819FC: 7D581850  subf r10, r24, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[24].s64;
	// 83181A00: 817AA32C  lwz r11, -0x5cd4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83181A04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83181A08: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 83181A0C: 419A003C  beq cr6, 0x83181a48
	if ctx.cr[6].eq {
	pc = 0x83181A48; continue 'dispatch;
	}
	// 83181A10: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 83181A14: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83181A18: 389E006C  addi r4, r30, 0x6c
	ctx.r[4].s64 = ctx.r[30].s64 + 108;
	// 83181A1C: 915E0074  stw r10, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 83181A20: 39410098  addi r10, r1, 0x98
	ctx.r[10].s64 = ctx.r[1].s64 + 152;
	// 83181A24: 915E0080  stw r10, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 83181A28: 3941009C  addi r10, r1, 0x9c
	ctx.r[10].s64 = ctx.r[1].s64 + 156;
	// 83181A2C: 915E008C  stw r10, 0x8c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 83181A30: 394100A0  addi r10, r1, 0xa0
	ctx.r[10].s64 = ctx.r[1].s64 + 160;
	// 83181A34: 915E0098  stw r10, 0x98(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 83181A38: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83181A3C: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 83181A40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83181A44: 4E800421  bctrl
	ctx.lr = 0x83181A48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83181A48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181A4C: 48011A05  bl 0x83193450
	ctx.lr = 0x83181A50;
	sub_83193450(ctx, base);
	// 83181A50: 81760018  lwz r11, 0x18(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(24 as u32) ) } as u64;
	// 83181A54: 7C971850  subf r4, r23, r3
	ctx.r[4].s64 = ctx.r[3].s64 - ctx.r[23].s64;
	// 83181A58: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83181A5C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83181A60: 386B1E58  addi r3, r11, 0x1e58
	ctx.r[3].s64 = ctx.r[11].s64 + 7768;
	// 83181A64: 48011B35  bl 0x83193598
	ctx.lr = 0x83181A68;
	sub_83193598(ctx, base);
	// 83181A68: 815F0A04  lwz r10, 0xa04(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2564 as u32) ) } as u64;
	// 83181A6C: 81210098  lwz r9, 0x98(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 83181A70: 3CC0FF00  lis r6, -0x100
	ctx.r[6].s64 = -16777216;
	// 83181A74: 817F0A08  lwz r11, 0xa08(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2568 as u32) ) } as u64;
	// 83181A78: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83181A7C: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83181A80: 8141009C  lwz r10, 0x9c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 83181A84: 60C60F06  ori r6, r6, 0xf06
	ctx.r[6].u64 = ctx.r[6].u64 | 3846;
	// 83181A88: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83181A8C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83181A90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181A94: 913F0A04  stw r9, 0xa04(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2564 as u32), ctx.r[9].u32 ) };
	// 83181A98: 917F0A08  stw r11, 0xa08(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2568 as u32), ctx.r[11].u32 ) };
	// 83181A9C: 4BFFD13D  bl 0x8317ebd8
	ctx.lr = 0x83181AA0;
	sub_8317EBD8(ctx, base);
	// 83181AA0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83181AA4: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83181AA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181AAC: 4BFFD0D5  bl 0x8317eb80
	ctx.lr = 0x83181AB0;
	sub_8317EB80(ctx, base);
	// 83181AB0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83181AB4: 419A0018  beq cr6, 0x83181acc
	if ctx.cr[6].eq {
	pc = 0x83181ACC; continue 'dispatch;
	}
	// 83181AB8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83181ABC: 4800645D  bl 0x83187f18
	ctx.lr = 0x83181AC0;
	sub_83187F18(ctx, base);
	// 83181AC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83181AC4: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 83181AC8: 480266D0  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
	// 83181ACC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83181AD0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83181AD4: 40990108  ble cr6, 0x83181bdc
	if !ctx.cr[6].gt {
	pc = 0x83181BDC; continue 'dispatch;
	}
	// 83181AD8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83181ADC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181AE0: 480062F9  bl 0x83187dd8
	ctx.lr = 0x83181AE4;
	sub_83187DD8(ctx, base);
	// 83181AE4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83181AE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181AEC: 4BFFF085  bl 0x83180b70
	ctx.lr = 0x83181AF0;
	sub_83180B70(ctx, base);
	// 83181AF0: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 83181AF4: 917C0054  stw r11, 0x54(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83181AF8: 81410098  lwz r10, 0x98(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 83181AFC: 817D00FC  lwz r11, 0xfc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(252 as u32) ) } as u64;
	// 83181B00: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83181B04: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83181B08: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83181B0C: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83181B10: 917C004C  stw r11, 0x4c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 83181B14: 8161009C  lwz r11, 0x9c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 83181B18: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83181B1C: 917C0050  stw r11, 0x50(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83181B20: A16100A0  lhz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 83181B24: B17C00FC  sth r11, 0xfc(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(252 as u32), ctx.r[11].u16 ) };
	// 83181B28: 815D00F0  lwz r10, 0xf0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(240 as u32) ) } as u64;
	// 83181B2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83181B30: 409A0010  bne cr6, 0x83181b40
	if !ctx.cr[6].eq {
	pc = 0x83181B40; continue 'dispatch;
	}
	// 83181B34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83181B38: 4BFFD891  bl 0x8317f3c8
	ctx.lr = 0x83181B3C;
	sub_8317F3C8(ctx, base);
	// 83181B3C: 907D0004  stw r3, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83181B40: 81760038  lwz r11, 0x38(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(56 as u32) ) } as u64;
	// 83181B44: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83181B48: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83181B4C: 419A0014  beq cr6, 0x83181b60
	if ctx.cr[6].eq {
	pc = 0x83181B60; continue 'dispatch;
	}
	// 83181B50: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83181B54: 409A000C  bne cr6, 0x83181b60
	if !ctx.cr[6].eq {
	pc = 0x83181B60; continue 'dispatch;
	}
	// 83181B58: 939D00F0  stw r28, 0xf0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(240 as u32), ctx.r[28].u32 ) };
	// 83181B5C: 48000008  b 0x83181b64
	pc = 0x83181B64; continue 'dispatch;
	// 83181B60: 93DD00F0  stw r30, 0xf0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(240 as u32), ctx.r[30].u32 ) };
	// 83181B64: 817D00F0  lwz r11, 0xf0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(240 as u32) ) } as u64;
	// 83181B68: 93DD00F4  stw r30, 0xf4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(244 as u32), ctx.r[30].u32 ) };
	// 83181B6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83181B70: 93DD00F8  stw r30, 0xf8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(248 as u32), ctx.r[30].u32 ) };
	// 83181B74: 409A004C  bne cr6, 0x83181bc0
	if !ctx.cr[6].eq {
	pc = 0x83181BC0; continue 'dispatch;
	}
	// 83181B78: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83181B7C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83181B80: 409A0024  bne cr6, 0x83181ba4
	if !ctx.cr[6].eq {
	pc = 0x83181BA4; continue 'dispatch;
	}
	// 83181B84: 81760018  lwz r11, 0x18(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(24 as u32) ) } as u64;
	// 83181B88: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83181B8C: 419A000C  beq cr6, 0x83181b98
	if ctx.cr[6].eq {
	pc = 0x83181B98; continue 'dispatch;
	}
	// 83181B90: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83181B94: 409A0010  bne cr6, 0x83181ba4
	if !ctx.cr[6].eq {
	pc = 0x83181BA4; continue 'dispatch;
	}
	// 83181B98: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83181B9C: 480063AD  bl 0x83187f48
	ctx.lr = 0x83181BA0;
	sub_83187F48(ctx, base);
	// 83181BA0: 4800000C  b 0x83181bac
	pc = 0x83181BAC; continue 'dispatch;
	// 83181BA4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83181BA8: 48006389  bl 0x83187f30
	ctx.lr = 0x83181BAC;
	sub_83187F30(ctx, base);
	// 83181BAC: 38B4000C  addi r5, r20, 0xc
	ctx.r[5].s64 = ctx.r[20].s64 + 12;
	// 83181BB0: 38940008  addi r4, r20, 8
	ctx.r[4].s64 = ctx.r[20].s64 + 8;
	// 83181BB4: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83181BB8: 4800E1A1  bl 0x8318fd58
	ctx.lr = 0x83181BBC;
	sub_8318FD58(ctx, base);
	// 83181BBC: 93DD000C  stw r30, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83181BC0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83181BC4: 80B60018  lwz r5, 0x18(r22)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(24 as u32) ) } as u64;
	// 83181BC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181BCC: 48001745  bl 0x83183310
	ctx.lr = 0x83181BD0;
	sub_83183310(ctx, base);
	// 83181BD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83181BD4: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 83181BD8: 480265C0  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
	// 83181BDC: 817D00F0  lwz r11, 0xf0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(240 as u32) ) } as u64;
	// 83181BE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83181BE4: 409A000C  bne cr6, 0x83181bf0
	if !ctx.cr[6].eq {
	pc = 0x83181BF0; continue 'dispatch;
	}
	// 83181BE8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83181BEC: 4800632D  bl 0x83187f18
	ctx.lr = 0x83181BF0;
	sub_83187F18(ctx, base);
	// 83181BF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83181BF4: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 83181BF8: 480265A0  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83181C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83181C00 size=264
    let mut pc: u32 = 0x83181C00;
    'dispatch: loop {
        match pc {
            0x83181C00 => {
    //   block [0x83181C00..0x83181D08)
	// 83181C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83181C04: 48026565  bl 0x831a8168
	ctx.lr = 0x83181C08;
	sub_831A8130(ctx, base);
	// 83181C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83181C0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83181C10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83181C14: 48005EDD  bl 0x83187af0
	ctx.lr = 0x83181C18;
	sub_83187AF0(ctx, base);
	// 83181C18: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83181C1C: 409A00E4  bne cr6, 0x83181d00
	if !ctx.cr[6].eq {
	pc = 0x83181D00; continue 'dispatch;
	}
	// 83181C20: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83181C24: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 83181C28: 394000C0  li r10, 0xc0
	ctx.r[10].s64 = 192;
	// 83181C2C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83181C30: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83181C34: 93BF1F74  stw r29, 0x1f74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8052 as u32), ctx.r[29].u32 ) };
	// 83181C38: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83181C3C: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83181C40: 93BE00C0  stw r29, 0xc0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(192 as u32), ctx.r[29].u32 ) };
	// 83181C44: 913E00C4  stw r9, 0xc4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), ctx.r[9].u32 ) };
	// 83181C48: 93BF1F6C  stw r29, 0x1f6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8044 as u32), ctx.r[29].u32 ) };
	// 83181C4C: 93BF1F70  stw r29, 0x1f70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8048 as u32), ctx.r[29].u32 ) };
	// 83181C50: 93BE00E8  stw r29, 0xe8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(232 as u32), ctx.r[29].u32 ) };
	// 83181C54: 93BE00EC  stw r29, 0xec(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(236 as u32), ctx.r[29].u32 ) };
	// 83181C58: 93BE00F0  stw r29, 0xf0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(240 as u32), ctx.r[29].u32 ) };
	// 83181C5C: 93BE00F4  stw r29, 0xf4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(244 as u32), ctx.r[29].u32 ) };
	// 83181C60: 93BE00F8  stw r29, 0xf8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(248 as u32), ctx.r[29].u32 ) };
	// 83181C64: 93BE00FC  stw r29, 0xfc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(252 as u32), ctx.r[29].u32 ) };
	// 83181C68: 807F1FC8  lwz r3, 0x1fc8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8136 as u32) ) } as u64;
	// 83181C6C: 80BF1FB8  lwz r5, 0x1fb8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8120 as u32) ) } as u64;
	// 83181C70: 809F1F64  lwz r4, 0x1f64(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8036 as u32) ) } as u64;
	// 83181C74: 907F1F78  stw r3, 0x1f78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8056 as u32), ctx.r[3].u32 ) };
	// 83181C78: 4BFFF0A1  bl 0x83180d18
	ctx.lr = 0x83181C7C;
	sub_83180D18(ctx, base);
	// 83181C7C: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 83181C80: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 83181C84: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 83181C88: 4BFFDE61  bl 0x8317fae8
	ctx.lr = 0x83181C8C;
	sub_8317FAE8(ctx, base);
	// 83181C8C: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 83181C90: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 83181C94: 93BE0098  stw r29, 0x98(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(152 as u32), ctx.r[29].u32 ) };
	// 83181C98: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 83181C9C: 387E00A0  addi r3, r30, 0xa0
	ctx.r[3].s64 = ctx.r[30].s64 + 160;
	// 83181CA0: 939E0094  stw r28, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[28].u32 ) };
	// 83181CA4: 917E009C  stw r11, 0x9c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 83181CA8: 4BFFCF81  bl 0x8317ec28
	ctx.lr = 0x83181CAC;
	sub_8317EC28(ctx, base);
	// 83181CAC: 389F1F7C  addi r4, r31, 0x1f7c
	ctx.r[4].s64 = ctx.r[31].s64 + 8060;
	// 83181CB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181CB4: 48005FA5  bl 0x83187c58
	ctx.lr = 0x83181CB8;
	sub_83187C58(ctx, base);
	// 83181CB8: 815F1FB8  lwz r10, 0x1fb8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8120 as u32) ) } as u64;
	// 83181CBC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83181CC0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83181CC4: 40990034  ble cr6, 0x83181cf8
	if !ctx.cr[6].gt {
	pc = 0x83181CF8; continue 'dispatch;
	}
	// 83181CC8: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 83181CCC: 811F1F90  lwz r8, 0x1f90(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8080 as u32) ) } as u64;
	// 83181CD0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83181CD4: 813F1F78  lwz r9, 0x1f78(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8056 as u32) ) } as u64;
	// 83181CD8: 7D08EA14  add r8, r8, r29
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[29].u64;
	// 83181CDC: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83181CE0: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 83181CE4: 394A0100  addi r10, r10, 0x100
	ctx.r[10].s64 = ctx.r[10].s64 + 256;
	// 83181CE8: 91090060  stw r8, 0x60(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 83181CEC: 813F1FB8  lwz r9, 0x1fb8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8120 as u32) ) } as u64;
	// 83181CF0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83181CF4: 4198FFD8  blt cr6, 0x83181ccc
	if ctx.cr[6].lt {
	pc = 0x83181CCC; continue 'dispatch;
	}
	// 83181CF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83181CFC: FB9E0100  std r28, 0x100(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(256 as u32), ctx.r[28].u64 ) };
	// 83181D00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83181D04: 480264B4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83181D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83181D08 size=128
    let mut pc: u32 = 0x83181D08;
    'dispatch: loop {
        match pc {
            0x83181D08 => {
    //   block [0x83181D08..0x83181D88)
	// 83181D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83181D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83181D10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83181D14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83181D18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83181D1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83181D20: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83181D24: 83DF17C8  lwz r30, 0x17c8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6088 as u32) ) } as u64;
	// 83181D28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83181D2C: 4BFFF115  bl 0x83180e40
	ctx.lr = 0x83181D30;
	sub_83180E40(ctx, base);
	// 83181D30: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83181D34: 409A003C  bne cr6, 0x83181d70
	if !ctx.cr[6].eq {
	pc = 0x83181D70; continue 'dispatch;
	}
	// 83181D38: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83181D3C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83181D40: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83181D44: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83181D48: 419A001C  beq cr6, 0x83181d64
	if ctx.cr[6].eq {
	pc = 0x83181D64; continue 'dispatch;
	}
	// 83181D4C: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 83181D50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181D54: 4BFF76BD  bl 0x83179410
	ctx.lr = 0x83181D58;
	sub_83179410(ctx, base);
	// 83181D58: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83181D5C: 396000C8  li r11, 0xc8
	ctx.r[11].s64 = 200;
	// 83181D60: 409A0008  bne cr6, 0x83181d68
	if !ctx.cr[6].eq {
	pc = 0x83181D68; continue 'dispatch;
	}
	// 83181D64: 396000C0  li r11, 0xc0
	ctx.r[11].s64 = 192;
	// 83181D68: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83181D6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83181D70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83181D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83181D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83181D7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83181D80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83181D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83181D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83181D88 size=108
    let mut pc: u32 = 0x83181D88;
    'dispatch: loop {
        match pc {
            0x83181D88 => {
    //   block [0x83181D88..0x83181DF4)
	// 83181D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83181D8C: 480263E1  bl 0x831a816c
	ctx.lr = 0x83181D90;
	sub_831A8130(ctx, base);
	// 83181D90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83181D94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83181D98: 83DF17D4  lwz r30, 0x17d4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6100 as u32) ) } as u64;
	// 83181D9C: 83BF17D0  lwz r29, 0x17d0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6096 as u32) ) } as u64;
	// 83181DA0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83181DA4: 4800C1FD  bl 0x8318dfa0
	ctx.lr = 0x83181DA8;
	sub_8318DFA0(ctx, base);
	// 83181DA8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83181DAC: 419A0040  beq cr6, 0x83181dec
	if ctx.cr[6].eq {
	pc = 0x83181DEC; continue 'dispatch;
	}
	// 83181DB0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83181DB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181DB8: 4800C1E9  bl 0x8318dfa0
	ctx.lr = 0x83181DBC;
	sub_8318DFA0(ctx, base);
	// 83181DBC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83181DC0: 409A002C  bne cr6, 0x83181dec
	if !ctx.cr[6].eq {
	pc = 0x83181DEC; continue 'dispatch;
	}
	// 83181DC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181DC8: 4BFFF121  bl 0x83180ee8
	ctx.lr = 0x83181DCC;
	sub_83180EE8(ctx, base);
	// 83181DCC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83181DD0: 419A001C  beq cr6, 0x83181dec
	if ctx.cr[6].eq {
	pc = 0x83181DEC; continue 'dispatch;
	}
	// 83181DD4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83181DD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83181DDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181DE0: 4800C1A9  bl 0x8318df88
	ctx.lr = 0x83181DE4;
	sub_8318DF88(ctx, base);
	// 83181DE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181DE8: 4BFFC831  bl 0x8317e618
	ctx.lr = 0x83181DEC;
	sub_8317E618(ctx, base);
	// 83181DEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83181DF0: 480263CC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83181DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83181DF8 size=80
    let mut pc: u32 = 0x83181DF8;
    'dispatch: loop {
        match pc {
            0x83181DF8 => {
    //   block [0x83181DF8..0x83181E48)
	// 83181DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83181DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83181E00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83181E04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83181E08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83181E0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83181E10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83181E14: 4BFFF375  bl 0x83181188
	ctx.lr = 0x83181E18;
	sub_83181188(ctx, base);
	// 83181E18: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83181E1C: 419A0014  beq cr6, 0x83181e30
	if ctx.cr[6].eq {
	pc = 0x83181E30; continue 'dispatch;
	}
	// 83181E20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83181E24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181E28: 4BFFE4D9  bl 0x83180300
	ctx.lr = 0x83181E2C;
	sub_83180300(ctx, base);
	// 83181E2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83181E30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83181E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83181E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83181E3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83181E40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83181E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83181E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83181E48 size=256
    let mut pc: u32 = 0x83181E48;
    'dispatch: loop {
        match pc {
            0x83181E48 => {
    //   block [0x83181E48..0x83181F48)
	// 83181E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83181E4C: 48026315  bl 0x831a8160
	ctx.lr = 0x83181E50;
	sub_831A8130(ctx, base);
	// 83181E50: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83181E54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83181E58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83181E5C: 3BBF0910  addi r29, r31, 0x910
	ctx.r[29].s64 = ctx.r[31].s64 + 2320;
	// 83181E60: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83181E64: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83181E68: 839F17C8  lwz r28, 0x17c8(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6088 as u32) ) } as u64;
	// 83181E6C: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 83181E70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83181E74: 419A0010  beq cr6, 0x83181e84
	if ctx.cr[6].eq {
	pc = 0x83181E84; continue 'dispatch;
	}
	// 83181E78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83181E7C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83181E80: 48026330  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83181E84: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83181E88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83181E8C: 48010AA5  bl 0x83192930
	ctx.lr = 0x83181E90;
	sub_83192930(ctx, base);
	// 83181E90: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83181E94: 419A001C  beq cr6, 0x83181eb0
	if ctx.cr[6].eq {
	pc = 0x83181EB0; continue 'dispatch;
	}
	// 83181E98: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83181E9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181EA0: 60840F16  ori r4, r4, 0xf16
	ctx.r[4].u64 = ctx.r[4].u64 | 3862;
	// 83181EA4: 48005655  bl 0x831874f8
	ctx.lr = 0x83181EA8;
	sub_831874F8(ctx, base);
	// 83181EA8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83181EAC: 48026304  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83181EB0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83181EB4: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 83181EB8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83181EBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83181EC0: 48010AD1  bl 0x83192990
	ctx.lr = 0x83181EC4;
	sub_83192990(ctx, base);
	// 83181EC4: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 83181EC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181ECC: 4BFF7545  bl 0x83179410
	ctx.lr = 0x83181ED0;
	sub_83179410(ctx, base);
	// 83181ED0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83181ED4: 409A000C  bne cr6, 0x83181ee0
	if !ctx.cr[6].eq {
	pc = 0x83181EE0; continue 'dispatch;
	}
	// 83181ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83181EDC: 48000030  b 0x83181f0c
	pc = 0x83181F0C; continue 'dispatch;
	// 83181EE0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83181EE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181EE8: 4800BF19  bl 0x8318de00
	ctx.lr = 0x83181EEC;
	sub_8318DE00(ctx, base);
	// 83181EEC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83181EF0: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 83181EF4: 409A000C  bne cr6, 0x83181f00
	if !ctx.cr[6].eq {
	pc = 0x83181F00; continue 'dispatch;
	}
	// 83181EF8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83181EFC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83181F00: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83181F04: 41980008  blt cr6, 0x83181f0c
	if ctx.cr[6].lt {
	pc = 0x83181F0C; continue 'dispatch;
	}
	// 83181F08: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83181F0C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83181F10: 917C009C  stw r11, 0x9c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 83181F14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181F18: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83181F1C: 4BFFE98D  bl 0x831808a8
	ctx.lr = 0x83181F20;
	sub_831808A8(ctx, base);
	// 83181F20: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83181F24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83181F28: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83181F2C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83181F30: 4BFFD371  bl 0x8317f2a0
	ctx.lr = 0x83181F34;
	sub_8317F2A0(ctx, base);
	// 83181F34: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83181F38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181F3C: 4BFFF455  bl 0x83181390
	ctx.lr = 0x83181F40;
	sub_83181390(ctx, base);
	// 83181F40: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83181F44: 4802626C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83181F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83181F48 size=264
    let mut pc: u32 = 0x83181F48;
    'dispatch: loop {
        match pc {
            0x83181F48 => {
    //   block [0x83181F48..0x83182050)
	// 83181F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83181F4C: 48026221  bl 0x831a816c
	ctx.lr = 0x83181F50;
	sub_831A8130(ctx, base);
	// 83181F50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83181F54: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 83181F58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83181F5C: 4BFF74B5  bl 0x83179410
	ctx.lr = 0x83181F60;
	sub_83179410(ctx, base);
	// 83181F60: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83181F64: 419A00E0  beq cr6, 0x83182044
	if ctx.cr[6].eq {
	pc = 0x83182044; continue 'dispatch;
	}
	// 83181F68: 3BBF1BB0  addi r29, r31, 0x1bb0
	ctx.r[29].s64 = ctx.r[31].s64 + 7088;
	// 83181F6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181F70: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83181F74: 93BF17C8  stw r29, 0x17c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(6088 as u32), ctx.r[29].u32 ) };
	// 83181F78: 4BFFFC89  bl 0x83181c00
	ctx.lr = 0x83181F7C;
	sub_83181C00(ctx, base);
	// 83181F7C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83181F80: 409A00C8  bne cr6, 0x83182048
	if !ctx.cr[6].eq {
	pc = 0x83182048; continue 'dispatch;
	}
	// 83181F84: 4800E365  bl 0x831902e8
	ctx.lr = 0x83181F88;
	sub_831902E8(ctx, base);
	// 83181F88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83181F8C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83181F90: 409A0018  bne cr6, 0x83181fa8
	if !ctx.cr[6].eq {
	pc = 0x83181FA8; continue 'dispatch;
	}
	// 83181F94: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83181F98: 60840F0A  ori r4, r4, 0xf0a
	ctx.r[4].u64 = ctx.r[4].u64 | 3850;
	// 83181F9C: 4800555D  bl 0x831874f8
	ctx.lr = 0x83181FA0;
	sub_831874F8(ctx, base);
	// 83181FA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83181FA4: 48026218  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83181FA8: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 83181FAC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83181FB0: 388BFAF8  addi r4, r11, -0x508
	ctx.r[4].s64 = ctx.r[11].s64 + -1288;
	// 83181FB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83181FB8: 48011421  bl 0x831933d8
	ctx.lr = 0x83181FBC;
	sub_831933D8(ctx, base);
	// 83181FBC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83181FC0: 419A0024  beq cr6, 0x83181fe4
	if ctx.cr[6].eq {
	pc = 0x83181FE4; continue 'dispatch;
	}
	// 83181FC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83181FC8: 4BFFDB51  bl 0x8317fb18
	ctx.lr = 0x83181FCC;
	sub_8317FB18(ctx, base);
	// 83181FCC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83181FD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83181FD4: 60840F0B  ori r4, r4, 0xf0b
	ctx.r[4].u64 = ctx.r[4].u64 | 3851;
	// 83181FD8: 48005521  bl 0x831874f8
	ctx.lr = 0x83181FDC;
	sub_831874F8(ctx, base);
	// 83181FDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83181FE0: 480261DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83181FE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83181FE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83181FEC: 4BFF7425  bl 0x83179410
	ctx.lr = 0x83181FF0;
	sub_83179410(ctx, base);
	// 83181FF0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83181FF4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83181FF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83181FFC: 4800E045  bl 0x83190040
	ctx.lr = 0x83182000;
	sub_83190040(ctx, base);
	// 83182000: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83182004: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182008: 4BFF7409  bl 0x83179410
	ctx.lr = 0x8318200C;
	sub_83179410(ctx, base);
	// 8318200C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83182010: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83182014: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83182018: 4800E029  bl 0x83190040
	ctx.lr = 0x8318201C;
	sub_83190040(ctx, base);
	// 8318201C: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83182020: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83182024: 80BF0038  lwz r5, 0x38(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83182028: 4800E019  bl 0x83190040
	ctx.lr = 0x8318202C;
	sub_83190040(ctx, base);
	// 8318202C: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83182030: 480013B1  bl 0x831833e0
	ctx.lr = 0x83182034;
	sub_831833E0(ctx, base);
	// 83182034: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83182038: 419A000C  beq cr6, 0x83182044
	if ctx.cr[6].eq {
	pc = 0x83182044; continue 'dispatch;
	}
	// 8318203C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182040: 48006461  bl 0x831884a0
	ctx.lr = 0x83182044;
	sub_831884A0(ctx, base);
	// 83182044: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83182048: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318204C: 48026170  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83182050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83182050 size=896
    let mut pc: u32 = 0x83182050;
    'dispatch: loop {
        match pc {
            0x83182050 => {
    //   block [0x83182050..0x831823D0)
	// 83182050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83182054: 480260F1  bl 0x831a8144
	ctx.lr = 0x83182058;
	sub_831A8130(ctx, base);
	// 83182058: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318205C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83182060: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 83182064: 397F1F7C  addi r11, r31, 0x1f7c
	ctx.r[11].s64 = ctx.r[31].s64 + 8060;
	// 83182068: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 8318206C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83182070: 83BF17C8  lwz r29, 0x17c8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6088 as u32) ) } as u64;
	// 83182074: 7CD33378  mr r19, r6
	ctx.r[19].u64 = ctx.r[6].u64;
	// 83182078: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 8318207C: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83182080: 3BDD0014  addi r30, r29, 0x14
	ctx.r[30].s64 = ctx.r[29].s64 + 20;
	// 83182084: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83182088: 3ACB0010  addi r22, r11, 0x10
	ctx.r[22].s64 = ctx.r[11].s64 + 16;
	// 8318208C: 833D0000  lwz r25, 0(r29)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83182090: 92AB0010  stw r21, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[21].u32 ) };
	// 83182094: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83182098: 4800E409  bl 0x831904a0
	ctx.lr = 0x8318209C;
	sub_831904A0(ctx, base);
	// 8318209C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 831820A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831820A4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831820A8: 4BFB26A1  bl 0x83134748
	ctx.lr = 0x831820AC;
	sub_83134748(ctx, base);
	// 831820AC: 3EE08345  lis r23, -0x7cbb
	ctx.r[23].s64 = -2092630016;
	// 831820B0: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 831820B4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831820B8: 3B6BE348  addi r27, r11, -0x1cb8
	ctx.r[27].s64 = ctx.r[11].s64 + -7352;
	// 831820BC: 8077A32C  lwz r3, -0x5cd4(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 831820C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831820C4: 419A0024  beq cr6, 0x831820e8
	if ctx.cr[6].eq {
	pc = 0x831820E8; continue 'dispatch;
	}
	// 831820C8: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 831820CC: 933B000C  stw r25, 0xc(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(12 as u32), ctx.r[25].u32 ) };
	// 831820D0: 389B0004  addi r4, r27, 4
	ctx.r[4].s64 = ctx.r[27].s64 + 4;
	// 831820D4: 917B0018  stw r11, 0x18(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 831820D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831820DC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 831820E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831820E4: 4E800421  bctrl
	ctx.lr = 0x831820E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831820E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831820EC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 831820F0: 480101E9  bl 0x831922d8
	ctx.lr = 0x831820F4;
	sub_831922D8(ctx, base);
	// 831820F4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831820F8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 831820FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83182100: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83182104: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83182108: 4BFB2641  bl 0x83134748
	ctx.lr = 0x8318210C;
	sub_83134748(ctx, base);
	// 8318210C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83182110: 3CC0FF00  lis r6, -0x100
	ctx.r[6].s64 = -16777216;
	// 83182114: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83182118: 7CAB1850  subf r5, r11, r3
	ctx.r[5].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 8318211C: 60C60F04  ori r6, r6, 0xf04
	ctx.r[6].u64 = ctx.r[6].u64 | 3844;
	// 83182120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182124: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 83182128: 4BFFCAB1  bl 0x8317ebd8
	ctx.lr = 0x8318212C;
	sub_8317EBD8(ctx, base);
	// 8318212C: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83182130: 8077A32C  lwz r3, -0x5cd4(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83182134: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83182138: 419A002C  beq cr6, 0x83182164
	if ctx.cr[6].eq {
	pc = 0x83182164; continue 'dispatch;
	}
	// 8318213C: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 83182140: 389B006C  addi r4, r27, 0x6c
	ctx.r[4].s64 = ctx.r[27].s64 + 108;
	// 83182144: 917B0074  stw r11, 0x74(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83182148: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8318214C: 935B0080  stw r26, 0x80(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(128 as u32), ctx.r[26].u32 ) };
	// 83182150: 917B008C  stw r11, 0x8c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 83182154: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83182158: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8318215C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83182160: 4E800421  bctrl
	ctx.lr = 0x83182164;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83182164: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182168: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8318216C: 4BFFCA15  bl 0x8317eb80
	ctx.lr = 0x83182170;
	sub_8317EB80(ctx, base);
	// 83182170: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83182174: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83182178: 409A0250  bne cr6, 0x831823c8
	if !ctx.cr[6].eq {
	pc = 0x831823C8; continue 'dispatch;
	}
	// 8318217C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83182180: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 83182184: 409A0010  bne cr6, 0x83182194
	if !ctx.cr[6].eq {
	pc = 0x83182194; continue 'dispatch;
	}
	// 83182188: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318218C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 83182190: 48026004  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
	// 83182194: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83182198: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8318219C: 4801072D  bl 0x831928c8
	ctx.lr = 0x831821A0;
	sub_831928C8(ctx, base);
	// 831821A0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831821A4: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 831821A8: 419A001C  beq cr6, 0x831821c4
	if ctx.cr[6].eq {
	pc = 0x831821C4; continue 'dispatch;
	}
	// 831821AC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 831821B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831821B4: 60840F05  ori r4, r4, 0xf05
	ctx.r[4].u64 = ctx.r[4].u64 | 3845;
	// 831821B8: 48005341  bl 0x831874f8
	ctx.lr = 0x831821BC;
	sub_831874F8(ctx, base);
	// 831821BC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 831821C0: 48025FD4  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
	// 831821C4: 567B0672  rlwinm r27, r19, 0, 0x19, 0x19
	ctx.r[27].u64 = ctx.r[19].u32 as u64 & 0xFFFFFFFFu64;
	// 831821C8: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 831821CC: 419A0030  beq cr6, 0x831821fc
	if ctx.cr[6].eq {
	pc = 0x831821FC; continue 'dispatch;
	}
	// 831821D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831821D4: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831821D8: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831821DC: 48005E5D  bl 0x83188038
	ctx.lr = 0x831821E0;
	sub_83188038(ctx, base);
	// 831821E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831821E4: 419A0018  beq cr6, 0x831821fc
	if ctx.cr[6].eq {
	pc = 0x831821FC; continue 'dispatch;
	}
	// 831821E8: 3960FFFE  li r11, -2
	ctx.r[11].s64 = -2;
	// 831821EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831821F0: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831821F4: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 831821F8: 48025F9C  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
	// 831821FC: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83182200: 3A9E0018  addi r20, r30, 0x18
	ctx.r[20].s64 = ctx.r[30].s64 + 24;
	// 83182204: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 83182208: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8318220C: 409A000C  bne cr6, 0x83182218
	if !ctx.cr[6].eq {
	pc = 0x83182218; continue 'dispatch;
	}
	// 83182210: 92BD00FC  stw r21, 0xfc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(252 as u32), ctx.r[21].u32 ) };
	// 83182214: 48000058  b 0x8318226c
	pc = 0x8318226C; continue 'dispatch;
	// 83182218: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8318221C: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 83182220: 409A004C  bne cr6, 0x8318226c
	if !ctx.cr[6].eq {
	pc = 0x8318226C; continue 'dispatch;
	}
	// 83182224: 815D00EC  lwz r10, 0xec(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(236 as u32) ) } as u64;
	// 83182228: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8318222C: 419A0040  beq cr6, 0x8318226c
	if ctx.cr[6].eq {
	pc = 0x8318226C; continue 'dispatch;
	}
	// 83182230: 814A007C  lwz r10, 0x7c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(124 as u32) ) } as u64;
	// 83182234: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83182238: 409A001C  bne cr6, 0x83182254
	if !ctx.cr[6].eq {
	pc = 0x83182254; continue 'dispatch;
	}
	// 8318223C: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83182240: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83182244: 40980028  bge cr6, 0x8318226c
	if !ctx.cr[6].lt {
	pc = 0x8318226C; continue 'dispatch;
	}
	// 83182248: 2F0A0200  cmpwi cr6, r10, 0x200
	ctx.cr[6].compare_i32(ctx.r[10].s32, 512, &mut ctx.xer);
	// 8318224C: 40980020  bge cr6, 0x8318226c
	if !ctx.cr[6].lt {
	pc = 0x8318226C; continue 'dispatch;
	}
	// 83182250: 48000018  b 0x83182268
	pc = 0x83182268; continue 'dispatch;
	// 83182254: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83182258: 409A0014  bne cr6, 0x8318226c
	if !ctx.cr[6].eq {
	pc = 0x8318226C; continue 'dispatch;
	}
	// 8318225C: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83182260: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83182264: 41980008  blt cr6, 0x8318226c
	if ctx.cr[6].lt {
	pc = 0x8318226C; continue 'dispatch;
	}
	// 83182268: 939D00FC  stw r28, 0xfc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(252 as u32), ctx.r[28].u32 ) };
	// 8318226C: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 83182270: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83182274: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83182278: 4800E241  bl 0x831904b8
	ctx.lr = 0x8318227C;
	sub_831904B8(ctx, base);
	// 8318227C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83182280: 815D0094  lwz r10, 0x94(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(148 as u32) ) } as u64;
	// 83182284: 3B5E0030  addi r26, r30, 0x30
	ctx.r[26].s64 = ctx.r[30].s64 + 48;
	// 83182288: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8318228C: 419A0010  beq cr6, 0x8318229c
	if ctx.cr[6].eq {
	pc = 0x8318229C; continue 'dispatch;
	}
	// 83182290: 917D0094  stw r11, 0x94(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 83182294: 939D0098  stw r28, 0x98(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(152 as u32), ctx.r[28].u32 ) };
	// 83182298: 48000008  b 0x831822a0
	pc = 0x831822A0; continue 'dispatch;
	// 8318229C: 92BD0098  stw r21, 0x98(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(152 as u32), ctx.r[21].u32 ) };
	// 831822A0: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 831822A4: 419A0048  beq cr6, 0x831822ec
	if ctx.cr[6].eq {
	pc = 0x831822EC; continue 'dispatch;
	}
	// 831822A8: 839F0D54  lwz r28, 0xd54(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3412 as u32) ) } as u64;
	// 831822AC: 837F0D58  lwz r27, 0xd58(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3416 as u32) ) } as u64;
	// 831822B0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 831822B4: 419A0038  beq cr6, 0x831822ec
	if ctx.cr[6].eq {
	pc = 0x831822EC; continue 'dispatch;
	}
	// 831822B8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 831822BC: 80980004  lwz r4, 4(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 831822C0: 80780000  lwz r3, 0(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 831822C4: 48010515  bl 0x831927d8
	ctx.lr = 0x831822C8;
	sub_831927D8(ctx, base);
	// 831822C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831822CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831822D0: 419A001C  beq cr6, 0x831822ec
	if ctx.cr[6].eq {
	pc = 0x831822EC; continue 'dispatch;
	}
	// 831822D4: 80980000  lwz r4, 0(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 831822D8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 831822DC: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 831822E0: 38AB0004  addi r5, r11, 4
	ctx.r[5].s64 = ctx.r[11].s64 + 4;
	// 831822E4: 7F8903A6  mtctr r28
	ctx.ctr.u64 = ctx.r[28].u64;
	// 831822E8: 4E800421  bctrl
	ctx.lr = 0x831822EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831822EC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 831822F0: 80980004  lwz r4, 4(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 831822F4: 80780000  lwz r3, 0(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 831822F8: 480104E1  bl 0x831927d8
	ctx.lr = 0x831822FC;
	sub_831927D8(ctx, base);
	// 831822FC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83182300: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 83182304: 811D0098  lwz r8, 0x98(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(152 as u32) ) } as u64;
	// 83182308: 38C10068  addi r6, r1, 0x68
	ctx.r[6].s64 = ctx.r[1].s64 + 104;
	// 8318230C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83182310: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182314: 4BFFEF65  bl 0x83181278
	ctx.lr = 0x83182318;
	sub_83181278(ctx, base);
	// 83182318: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318231C: E9410060  ld r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 83182320: 7D6B9838  and r11, r11, r19
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[19].u64;
	// 83182324: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83182328: F95D0100  std r10, 0x100(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(256 as u32), ctx.r[10].u64 ) };
	// 8318232C: 419AFE5C  beq cr6, 0x83182188
	if ctx.cr[6].eq {
	pc = 0x83182188; continue 'dispatch;
	}
	// 83182330: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83182334: 80BD0098  lwz r5, 0x98(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(152 as u32) ) } as u64;
	// 83182338: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318233C: 4BFFE265  bl 0x831805a0
	ctx.lr = 0x83182340;
	sub_831805A0(ctx, base);
	// 83182340: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83182344: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182348: E8A10068  ld r5, 0x68(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 8318234C: 80DD0098  lwz r6, 0x98(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(152 as u32) ) } as u64;
	// 83182350: 4BFFEFA1  bl 0x831812f0
	ctx.lr = 0x83182354;
	sub_831812F0(ctx, base);
	// 83182354: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83182358: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318235C: 4BFFCD8D  bl 0x8317f0e8
	ctx.lr = 0x83182360;
	sub_8317F0E8(ctx, base);
	// 83182360: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83182364: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182368: 4BFFCE31  bl 0x8317f198
	ctx.lr = 0x8318236C;
	sub_8317F198(ctx, base);
	// 8318236C: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 83182370: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83182374: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83182378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318237C: 4BFFFACD  bl 0x83181e48
	ctx.lr = 0x83182380;
	sub_83181E48(ctx, base);
	// 83182380: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83182384: 8077A32C  lwz r3, -0x5cd4(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83182388: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318238C: 419A0038  beq cr6, 0x831823c4
	if ctx.cr[6].eq {
	pc = 0x831823C4; continue 'dispatch;
	}
	// 83182390: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83182394: 395E0068  addi r10, r30, 0x68
	ctx.r[10].s64 = ctx.r[30].s64 + 104;
	// 83182398: 396BE4F8  addi r11, r11, -0x1b08
	ctx.r[11].s64 = ctx.r[11].s64 + -6920;
	// 8318239C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 831823A0: 928B000C  stw r20, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[20].u32 ) };
	// 831823A4: 934B0018  stw r26, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[26].u32 ) };
	// 831823A8: 914B0024  stw r10, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 831823AC: 395F0E6C  addi r10, r31, 0xe6c
	ctx.r[10].s64 = ctx.r[31].s64 + 3692;
	// 831823B0: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 831823B4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831823B8: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 831823BC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831823C0: 4E800421  bctrl
	ctx.lr = 0x831823C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831823C4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831823C8: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 831823CC: 48025DC8  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831823D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831823D0 size=700
    let mut pc: u32 = 0x831823D0;
    'dispatch: loop {
        match pc {
            0x831823D0 => {
    //   block [0x831823D0..0x8318268C)
	// 831823D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831823D4: 48025D85  bl 0x831a8158
	ctx.lr = 0x831823D8;
	sub_831A8130(ctx, base);
	// 831823D8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831823DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831823E0: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 831823E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831823E8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 831823EC: 3BBF0950  addi r29, r31, 0x950
	ctx.r[29].s64 = ctx.r[31].s64 + 2384;
	// 831823F0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 831823F4: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831823F8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 831823FC: 839F17C8  lwz r28, 0x17c8(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6088 as u32) ) } as u64;
	// 83182400: 809F17D0  lwz r4, 0x17d0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6096 as u32) ) } as u64;
	// 83182404: 917D0028  stw r11, 0x28(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83182408: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318240C: 2F0B00CC  cmpwi cr6, r11, 0xcc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 204, &mut ctx.xer);
	// 83182410: 409A0010  bne cr6, 0x83182420
	if !ctx.cr[6].eq {
	pc = 0x83182420; continue 'dispatch;
	}
	// 83182414: 817C00F8  lwz r11, 0xf8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(248 as u32) ) } as u64;
	// 83182418: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318241C: 409A0008  bne cr6, 0x83182424
	if !ctx.cr[6].eq {
	pc = 0x83182424; continue 'dispatch;
	}
	// 83182420: 73DE00CC  andi. r30, r30, 0xcc
	ctx.r[30].u64 = ctx.r[30].u64 & 204;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83182424: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83182428: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318242C: 4800BA6D  bl 0x8318de98
	ctx.lr = 0x83182430;
	sub_8318DE98(ctx, base);
	// 83182430: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 83182434: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 83182438: 419A0010  beq cr6, 0x83182448
	if ctx.cr[6].eq {
	pc = 0x83182448; continue 'dispatch;
	}
	// 8318243C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83182440: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83182444: 48025D64  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83182448: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8318244C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83182450: 419AFFEC  beq cr6, 0x8318243c
	if ctx.cr[6].eq {
	pc = 0x8318243C; continue 'dispatch;
	}
	// 83182454: 73CB00C8  andi. r11, r30, 0xc8
	ctx.r[11].u64 = ctx.r[30].u64 & 200;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83182458: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318245C: 419A0010  beq cr6, 0x8318246c
	if ctx.cr[6].eq {
	pc = 0x8318246C; continue 'dispatch;
	}
	// 83182460: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83182464: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182468: 48005971  bl 0x83187dd8
	ctx.lr = 0x8318246C;
	sub_83187DD8(ctx, base);
	// 8318246C: 2F1E0080  cmpwi cr6, r30, 0x80
	ctx.cr[6].compare_i32(ctx.r[30].s32, 128, &mut ctx.xer);
	// 83182470: 409A0070  bne cr6, 0x831824e0
	if !ctx.cr[6].eq {
	pc = 0x831824E0; continue 'dispatch;
	}
	// 83182474: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182478: 4BFFC5F9  bl 0x8317ea70
	ctx.lr = 0x8318247C;
	sub_8317EA70(ctx, base);
	// 8318247C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182480: 48009571  bl 0x8318b9f0
	ctx.lr = 0x83182484;
	sub_8318B9F0(ctx, base);
	// 83182484: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83182488: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318248C: 419A0028  beq cr6, 0x831824b4
	if ctx.cr[6].eq {
	pc = 0x831824B4; continue 'dispatch;
	}
	// 83182490: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83182494: 4BFFF965  bl 0x83181df8
	ctx.lr = 0x83182498;
	sub_83181DF8(ctx, base);
	// 83182498: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318249C: 409A01E4  bne cr6, 0x83182680
	if !ctx.cr[6].eq {
	pc = 0x83182680; continue 'dispatch;
	}
	// 831824A0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831824A4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 831824A8: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831824AC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 831824B0: 48025CF8  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 831824B4: 480095C5  bl 0x8318ba78
	ctx.lr = 0x831824B8;
	sub_8318BA78(ctx, base);
	// 831824B8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831824BC: 419A0024  beq cr6, 0x831824e0
	if ctx.cr[6].eq {
	pc = 0x831824E0; continue 'dispatch;
	}
	// 831824C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831824C4: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831824C8: 4BFFDE39  bl 0x83180300
	ctx.lr = 0x831824CC;
	sub_83180300(ctx, base);
	// 831824CC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831824D0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 831824D4: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831824D8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 831824DC: 48025CCC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 831824E0: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 831824E4: 409A007C  bne cr6, 0x83182560
	if !ctx.cr[6].eq {
	pc = 0x83182560; continue 'dispatch;
	}
	// 831824E8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 831824EC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 831824F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831824F4: 4BFFDEED  bl 0x831803e0
	ctx.lr = 0x831824F8;
	sub_831803E0(ctx, base);
	// 831824F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831824FC: 419A0018  beq cr6, 0x83182514
	if ctx.cr[6].eq {
	pc = 0x83182514; continue 'dispatch;
	}
	// 83182500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182504: 480058BD  bl 0x83187dc0
	ctx.lr = 0x83182508;
	sub_83187DC0(ctx, base);
	// 83182508: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8318250C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83182510: 48025C98  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83182514: 2F1B0004  cmpwi cr6, r27, 4
	ctx.cr[6].compare_i32(ctx.r[27].s32, 4, &mut ctx.xer);
	// 83182518: 41990048  bgt cr6, 0x83182560
	if ctx.cr[6].gt {
	pc = 0x83182560; continue 'dispatch;
	}
	// 8318251C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83182520: 917D0028  stw r11, 0x28(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83182524: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83182528: 806BA32C  lwz r3, -0x5cd4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 8318252C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83182530: 419A0150  beq cr6, 0x83182680
	if ctx.cr[6].eq {
	pc = 0x83182680; continue 'dispatch;
	}
	// 83182534: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83182538: 396BE648  addi r11, r11, -0x19b8
	ctx.r[11].s64 = ctx.r[11].s64 + -6584;
	// 8318253C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 83182540: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 83182544: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83182548: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 8318254C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83182550: 4E800421  bctrl
	ctx.lr = 0x83182554;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83182554: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83182558: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8318255C: 48025C4C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83182560: 73CB004C  andi. r11, r30, 0x4c
	ctx.r[11].u64 = ctx.r[30].u64 & 76;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83182564: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83182568: 419A008C  beq cr6, 0x831825f4
	if ctx.cr[6].eq {
	pc = 0x831825F4; continue 'dispatch;
	}
	// 8318256C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83182570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182574: 4BFFC4A5  bl 0x8317ea18
	ctx.lr = 0x83182578;
	sub_8317EA18(ctx, base);
	// 83182578: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 8318257C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83182580: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83182584: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83182588: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318258C: 4BFFFAC5  bl 0x83182050
	ctx.lr = 0x83182590;
	sub_83182050(ctx, base);
	// 83182590: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 83182594: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 83182598: 409A00E8  bne cr6, 0x83182680
	if !ctx.cr[6].eq {
	pc = 0x83182680; continue 'dispatch;
	}
	// 8318259C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831825A0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831825A4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831825A8: 409A0020  bne cr6, 0x831825c8
	if !ctx.cr[6].eq {
	pc = 0x831825C8; continue 'dispatch;
	}
	// 831825AC: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 831825B0: 7D4AF038  and r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[30].u64;
	// 831825B4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831825B8: 419A000C  beq cr6, 0x831825c4
	if ctx.cr[6].eq {
	pc = 0x831825C4; continue 'dispatch;
	}
	// 831825BC: 394000CC  li r10, 0xcc
	ctx.r[10].s64 = 204;
	// 831825C0: 915C0008  stw r10, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 831825C4: 917C00F8  stw r11, 0xf8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(248 as u32), ctx.r[11].u32 ) };
	// 831825C8: 2F1E0040  cmpwi cr6, r30, 0x40
	ctx.cr[6].compare_i32(ctx.r[30].s32, 64, &mut ctx.xer);
	// 831825CC: 409A00B0  bne cr6, 0x8318267c
	if !ctx.cr[6].eq {
	pc = 0x8318267C; continue 'dispatch;
	}
	// 831825D0: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831825D4: 2F0AFFFE  cmpwi cr6, r10, -2
	ctx.cr[6].compare_i32(ctx.r[10].s32, -2, &mut ctx.xer);
	// 831825D8: 409A00A4  bne cr6, 0x8318267c
	if !ctx.cr[6].eq {
	pc = 0x8318267C; continue 'dispatch;
	}
	// 831825DC: 394000C0  li r10, 0xc0
	ctx.r[10].s64 = 192;
	// 831825E0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 831825E4: 915C0008  stw r10, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 831825E8: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831825EC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 831825F0: 48025BB8  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 831825F4: 57CB07BC  rlwinm r11, r30, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	// 831825F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831825FC: 419A005C  beq cr6, 0x83182658
	if ctx.cr[6].eq {
	pc = 0x83182658; continue 'dispatch;
	}
	// 83182600: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83182604: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182608: 4BFFC411  bl 0x8317ea18
	ctx.lr = 0x8318260C;
	sub_8317EA18(ctx, base);
	// 8318260C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83182610: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182614: 4BFFF095  bl 0x831816a8
	ctx.lr = 0x83182618;
	sub_831816A8(ctx, base);
	// 83182618: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318261C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83182620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182624: 419A0024  beq cr6, 0x83182648
	if ctx.cr[6].eq {
	pc = 0x83182648; continue 'dispatch;
	}
	// 83182628: 4BFFE481  bl 0x83180aa8
	ctx.lr = 0x8318262C;
	sub_83180AA8(ctx, base);
	// 8318262C: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 83182630: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 83182634: 409A004C  bne cr6, 0x83182680
	if !ctx.cr[6].eq {
	pc = 0x83182680; continue 'dispatch;
	}
	// 83182638: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8318263C: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83182640: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83182644: 48025B64  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83182648: 4BFFF2C1  bl 0x83181908
	ctx.lr = 0x8318264C;
	sub_83181908(ctx, base);
	// 8318264C: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 83182650: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83182654: 48025B54  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83182658: 2F1E0080  cmpwi cr6, r30, 0x80
	ctx.cr[6].compare_i32(ctx.r[30].s32, 128, &mut ctx.xer);
	// 8318265C: 419A0024  beq cr6, 0x83182680
	if ctx.cr[6].eq {
	pc = 0x83182680; continue 'dispatch;
	}
	// 83182660: 38A000CC  li r5, 0xcc
	ctx.r[5].s64 = 204;
	// 83182664: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83182668: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318266C: 4BFFE555  bl 0x83180bc0
	ctx.lr = 0x83182670;
	sub_83180BC0(ctx, base);
	// 83182670: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83182674: 4099000C  ble cr6, 0x83182680
	if !ctx.cr[6].gt {
	pc = 0x83182680; continue 'dispatch;
	}
	// 83182678: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8318267C: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83182680: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83182684: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83182688: 48025B20  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83182690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83182690 size=156
    let mut pc: u32 = 0x83182690;
    'dispatch: loop {
        match pc {
            0x83182690 => {
    //   block [0x83182690..0x8318272C)
	// 83182690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83182694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83182698: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318269C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831826A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831826A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831826A8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831826AC: 817E0064  lwz r11, 0x64(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 831826B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831826B4: 409A0054  bne cr6, 0x83182708
	if !ctx.cr[6].eq {
	pc = 0x83182708; continue 'dispatch;
	}
	// 831826B8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 831826BC: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 831826C0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 831826C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831826C8: 4BFFE899  bl 0x83180f60
	ctx.lr = 0x831826CC;
	sub_83180F60(ctx, base);
	// 831826CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831826D0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831826D4: 409A0034  bne cr6, 0x83182708
	if !ctx.cr[6].eq {
	pc = 0x83182708; continue 'dispatch;
	}
	// 831826D8: 38E1005C  addi r7, r1, 0x5c
	ctx.r[7].s64 = ctx.r[1].s64 + 92;
	// 831826DC: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831826E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831826E4: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831826E8: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 831826EC: 4BFFFCE5  bl 0x831823d0
	ctx.lr = 0x831826F0;
	sub_831823D0(ctx, base);
	// 831826F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831826F4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831826F8: 409A0010  bne cr6, 0x83182708
	if !ctx.cr[6].eq {
	pc = 0x83182708; continue 'dispatch;
	}
	// 831826FC: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83182700: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83182704: 409AFFA8  bne cr6, 0x831826ac
	if !ctx.cr[6].eq {
	pc = 0x831826AC; continue 'dispatch;
	}
	// 83182708: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318270C: 4BFFD37D  bl 0x8317fa88
	ctx.lr = 0x83182710;
	sub_8317FA88(ctx, base);
	// 83182710: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83182718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318271C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83182720: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83182724: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83182728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83182730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83182730 size=148
    let mut pc: u32 = 0x83182730;
    'dispatch: loop {
        match pc {
            0x83182730 => {
    //   block [0x83182730..0x831827C4)
	// 83182730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83182734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83182738: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318273C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83182740: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83182744: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 83182748: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318274C: 4BFF6CC5  bl 0x83179410
	ctx.lr = 0x83182750;
	sub_83179410(ctx, base);
	// 83182750: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83182754: 409A000C  bne cr6, 0x83182760
	if !ctx.cr[6].eq {
	pc = 0x83182760; continue 'dispatch;
	}
	// 83182758: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318275C: 48000050  b 0x831827ac
	pc = 0x831827AC; continue 'dispatch;
	// 83182760: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182764: 4BFFBE35  bl 0x8317e598
	ctx.lr = 0x83182768;
	sub_8317E598(ctx, base);
	// 83182768: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8318276C: 419AFFEC  beq cr6, 0x83182758
	if ctx.cr[6].eq {
	pc = 0x83182758; continue 'dispatch;
	}
	// 83182770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182774: 4BFFBD45  bl 0x8317e4b8
	ctx.lr = 0x83182778;
	sub_8317E4B8(ctx, base);
	// 83182778: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8318277C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83182780: 409A000C  bne cr6, 0x8318278c
	if !ctx.cr[6].eq {
	pc = 0x8318278C; continue 'dispatch;
	}
	// 83182784: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182788: 4BFFBD91  bl 0x8317e518
	ctx.lr = 0x8318278C;
	sub_8317E518(ctx, base);
	// 8318278C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182790: 4BFFFF01  bl 0x83182690
	ctx.lr = 0x83182794;
	sub_83182690(ctx, base);
	// 83182794: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83182798: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318279C: 4BFFF5ED  bl 0x83181d88
	ctx.lr = 0x831827A0;
	sub_83181D88(ctx, base);
	// 831827A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831827A4: 4BFFD70D  bl 0x8317feb0
	ctx.lr = 0x831827A8;
	sub_8317FEB0(ctx, base);
	// 831827A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831827AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831827B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831827B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831827B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831827BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831827C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831827C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831827C8 size=144
    let mut pc: u32 = 0x831827C8;
    'dispatch: loop {
        match pc {
            0x831827C8 => {
    //   block [0x831827C8..0x83182858)
	// 831827C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831827CC: 4802599D  bl 0x831a8168
	ctx.lr = 0x831827D0;
	sub_831A8130(ctx, base);
	// 831827D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831827D4: 3F808345  lis r28, -0x7cbb
	ctx.r[28].s64 = -2092630016;
	// 831827D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831827DC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831827E0: 3BEBE198  addi r31, r11, -0x1e68
	ctx.r[31].s64 = ctx.r[11].s64 + -7784;
	// 831827E4: 807CA32C  lwz r3, -0x5cd4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 831827E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831827EC: 419A001C  beq cr6, 0x83182808
	if ctx.cr[6].eq {
	pc = 0x83182808; continue 'dispatch;
	}
	// 831827F0: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 831827F4: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 831827F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831827FC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83182800: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83182804: 4E800421  bctrl
	ctx.lr = 0x83182808;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83182808: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318280C: 4BFFFF25  bl 0x83182730
	ctx.lr = 0x83182810;
	sub_83182730(ctx, base);
	// 83182810: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83182814: 807CA32C  lwz r3, -0x5cd4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83182818: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318281C: 419A0030  beq cr6, 0x8318284c
	if ctx.cr[6].eq {
	pc = 0x8318284C; continue 'dispatch;
	}
	// 83182820: 397E09A0  addi r11, r30, 0x9a0
	ctx.r[11].s64 = ctx.r[30].s64 + 2464;
	// 83182824: 389F006C  addi r4, r31, 0x6c
	ctx.r[4].s64 = ctx.r[31].s64 + 108;
	// 83182828: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8318282C: 397E09A8  addi r11, r30, 0x9a8
	ctx.r[11].s64 = ctx.r[30].s64 + 2472;
	// 83182830: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83182834: 397E09B0  addi r11, r30, 0x9b0
	ctx.r[11].s64 = ctx.r[30].s64 + 2480;
	// 83182838: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 8318283C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83182840: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83182844: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83182848: 4E800421  bctrl
	ctx.lr = 0x8318284C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318284C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83182850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83182854: 48025964  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83182858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83182858 size=16
    let mut pc: u32 = 0x83182858;
    'dispatch: loop {
        match pc {
            0x83182858 => {
    //   block [0x83182858..0x83182868)
	// 83182858: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318285C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83182860: 60840201  ori r4, r4, 0x201
	ctx.r[4].u64 = ctx.r[4].u64 | 513;
	// 83182864: 48004C94  b 0x831874f8
	sub_831874F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83182868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83182868 size=4
    let mut pc: u32 = 0x83182868;
    'dispatch: loop {
        match pc {
            0x83182868 => {
    //   block [0x83182868..0x8318286C)
	// 83182868: 4BFFA5F8  b 0x8317ce60
	sub_8317CE60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83182870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83182870 size=44
    let mut pc: u32 = 0x83182870;
    'dispatch: loop {
        match pc {
            0x83182870 => {
    //   block [0x83182870..0x8318289C)
	// 83182870: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 83182874: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83182878: 419A0024  beq cr6, 0x8318289c
	if ctx.cr[6].eq {
		sub_8318289C(ctx, base);
		return;
	}
	// 8318287C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83182880: 419A001C  beq cr6, 0x8318289c
	if ctx.cr[6].eq {
		sub_8318289C(ctx, base);
		return;
	}
	// 83182884: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83182888: 419A0014  beq cr6, 0x8318289c
	if ctx.cr[6].eq {
		sub_8318289C(ctx, base);
		return;
	}
	// 8318288C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83182890: 419A000C  beq cr6, 0x8318289c
	if ctx.cr[6].eq {
		sub_8318289C(ctx, base);
		return;
	}
	// 83182894: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83182898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318289C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318289C size=28
    let mut pc: u32 = 0x8318289C;
    'dispatch: loop {
        match pc {
            0x8318289C => {
    //   block [0x8318289C..0x831828B8)
	// 8318289C: 81630058  lwz r11, 0x58(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 831828A0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831828A4: 419AFFF0  beq cr6, 0x83182894
	if ctx.cr[6].eq {
		sub_83182870(ctx, base);
		return;
	}
	// 831828A8: 81630044  lwz r11, 0x44(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 831828AC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 831828B0: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 831828B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831828B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831828B8 size=8
    let mut pc: u32 = 0x831828B8;
    'dispatch: loop {
        match pc {
            0x831828B8 => {
    //   block [0x831828B8..0x831828C0)
	// 831828B8: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 831828BC: 480094BC  b 0x8318bd78
	sub_8318BD78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831828C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831828C0 size=16
    let mut pc: u32 = 0x831828C0;
    'dispatch: loop {
        match pc {
            0x831828C0 => {
    //   block [0x831828C0..0x831828D0)
	// 831828C0: 8163004C  lwz r11, 0x4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 831828C4: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 831828C8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 831828CC: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831828D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831828D0 size=16
    let mut pc: u32 = 0x831828D0;
    'dispatch: loop {
        match pc {
            0x831828D0 => {
    //   block [0x831828D0..0x831828E0)
	// 831828D0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 831828D4: 4099000C  ble cr6, 0x831828e0
	if !ctx.cr[6].gt {
		sub_831828E0(ctx, base);
		return;
	}
	// 831828D8: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 831828DC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831828E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831828E0 size=8
    let mut pc: u32 = 0x831828E0;
    'dispatch: loop {
        match pc {
            0x831828E0 => {
    //   block [0x831828E0..0x831828E8)
	// 831828E0: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 831828E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831828E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831828E8 size=164
    let mut pc: u32 = 0x831828E8;
    'dispatch: loop {
        match pc {
            0x831828E8 => {
    //   block [0x831828E8..0x8318298C)
	// 831828E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831828EC: 48025881  bl 0x831a816c
	ctx.lr = 0x831828F0;
	sub_831A8130(ctx, base);
	// 831828F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831828F4: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 831828F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831828FC: 4BFF6B15  bl 0x83179410
	ctx.lr = 0x83182900;
	sub_83179410(ctx, base);
	// 83182900: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83182904: 409A000C  bne cr6, 0x83182910
	if !ctx.cr[6].eq {
	pc = 0x83182910; continue 'dispatch;
	}
	// 83182908: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8318290C: 48000024  b 0x83182930
	pc = 0x83182930; continue 'dispatch;
	// 83182910: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83182914: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182918: 48009521  bl 0x8318be38
	ctx.lr = 0x8318291C;
	sub_8318BE38(ctx, base);
	// 8318291C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83182920: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83182924: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182928: 48009531  bl 0x8318be58
	ctx.lr = 0x8318292C;
	sub_8318BE58(ctx, base);
	// 8318292C: 7C7DF378  or r29, r3, r30
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[30].u64;
	// 83182930: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83182934: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182938: 4BFF6AD9  bl 0x83179410
	ctx.lr = 0x8318293C;
	sub_83179410(ctx, base);
	// 8318293C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83182940: 409A000C  bne cr6, 0x8318294c
	if !ctx.cr[6].eq {
	pc = 0x8318294C; continue 'dispatch;
	}
	// 83182944: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83182948: 48000024  b 0x8318296c
	pc = 0x8318296C; continue 'dispatch;
	// 8318294C: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83182950: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182954: 480094E5  bl 0x8318be38
	ctx.lr = 0x83182958;
	sub_8318BE38(ctx, base);
	// 83182958: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8318295C: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83182960: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182964: 480094F5  bl 0x8318be58
	ctx.lr = 0x83182968;
	sub_8318BE58(ctx, base);
	// 83182968: 7C6BF378  or r11, r3, r30
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[30].u64;
	// 8318296C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83182970: 419A0010  beq cr6, 0x83182980
	if ctx.cr[6].eq {
	pc = 0x83182980; continue 'dispatch;
	}
	// 83182974: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83182978: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8318297C: 409A0008  bne cr6, 0x83182984
	if !ctx.cr[6].eq {
	pc = 0x83182984; continue 'dispatch;
	}
	// 83182980: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83182984: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83182988: 48025834  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83182990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83182990 size=176
    let mut pc: u32 = 0x83182990;
    'dispatch: loop {
        match pc {
            0x83182990 => {
    //   block [0x83182990..0x83182A40)
	// 83182990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83182994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83182998: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318299C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831829A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831829A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831829A8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831829AC: 817F0A20  lwz r11, 0xa20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2592 as u32) ) } as u64;
	// 831829B0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831829B4: 409A002C  bne cr6, 0x831829e0
	if !ctx.cr[6].eq {
	pc = 0x831829E0; continue 'dispatch;
	}
	// 831829B8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831829BC: 4800B465  bl 0x8318de20
	ctx.lr = 0x831829C0;
	sub_8318DE20(ctx, base);
	// 831829C0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831829C4: 409A001C  bne cr6, 0x831829e0
	if !ctx.cr[6].eq {
	pc = 0x831829E0; continue 'dispatch;
	}
	// 831829C8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831829CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831829D0: 4800B441  bl 0x8318de10
	ctx.lr = 0x831829D4;
	sub_8318DE10(ctx, base);
	// 831829D4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831829D8: 409A0008  bne cr6, 0x831829e0
	if !ctx.cr[6].eq {
	pc = 0x831829E0; continue 'dispatch;
	}
	// 831829DC: 93DF0A20  stw r30, 0xa20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2592 as u32), ctx.r[30].u32 ) };
	// 831829E0: 817F0A24  lwz r11, 0xa24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 831829E4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831829E8: 409A0030  bne cr6, 0x83182a18
	if !ctx.cr[6].eq {
	pc = 0x83182A18; continue 'dispatch;
	}
	// 831829EC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 831829F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831829F4: 4800B42D  bl 0x8318de20
	ctx.lr = 0x831829F8;
	sub_8318DE20(ctx, base);
	// 831829F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831829FC: 409A001C  bne cr6, 0x83182a18
	if !ctx.cr[6].eq {
	pc = 0x83182A18; continue 'dispatch;
	}
	// 83182A00: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83182A04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182A08: 4800B409  bl 0x8318de10
	ctx.lr = 0x83182A0C;
	sub_8318DE10(ctx, base);
	// 83182A0C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83182A10: 409A0008  bne cr6, 0x83182a18
	if !ctx.cr[6].eq {
	pc = 0x83182A18; continue 'dispatch;
	}
	// 83182A14: 93DF0A24  stw r30, 0xa24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2596 as u32), ctx.r[30].u32 ) };
	// 83182A18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182A1C: 80BF0A24  lwz r5, 0xa24(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 83182A20: 809F0A20  lwz r4, 0xa20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2592 as u32) ) } as u64;
	// 83182A24: 48010C35  bl 0x83193658
	ctx.lr = 0x83182A28;
	sub_83193658(ctx, base);
	// 83182A28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83182A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83182A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83182A34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83182A38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83182A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83182A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83182A40 size=160
    let mut pc: u32 = 0x83182A40;
    'dispatch: loop {
        match pc {
            0x83182A40 => {
    //   block [0x83182A40..0x83182AE0)
	// 83182A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83182A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83182A48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83182A4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83182A50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83182A54: 817F0A24  lwz r11, 0xa24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 83182A58: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83182A5C: 409A001C  bne cr6, 0x83182a78
	if !ctx.cr[6].eq {
	pc = 0x83182A78; continue 'dispatch;
	}
	// 83182A60: 817F0A48  lwz r11, 0xa48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2632 as u32) ) } as u64;
	// 83182A64: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83182A68: 409A0010  bne cr6, 0x83182a78
	if !ctx.cr[6].eq {
	pc = 0x83182A78; continue 'dispatch;
	}
	// 83182A6C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83182A70: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83182A74: 4BFF69E5  bl 0x83179458
	ctx.lr = 0x83182A78;
	sub_83179458(ctx, base);
	// 83182A78: 817F0A20  lwz r11, 0xa20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2592 as u32) ) } as u64;
	// 83182A7C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83182A80: 409A0020  bne cr6, 0x83182aa0
	if !ctx.cr[6].eq {
	pc = 0x83182AA0; continue 'dispatch;
	}
	// 83182A84: 817F0A48  lwz r11, 0xa48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2632 as u32) ) } as u64;
	// 83182A88: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83182A8C: 409A0014  bne cr6, 0x83182aa0
	if !ctx.cr[6].eq {
	pc = 0x83182AA0; continue 'dispatch;
	}
	// 83182A90: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83182A94: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83182A98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182A9C: 4BFF69BD  bl 0x83179458
	ctx.lr = 0x83182AA0;
	sub_83179458(ctx, base);
	// 83182AA0: 817F0A48  lwz r11, 0xa48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2632 as u32) ) } as u64;
	// 83182AA4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83182AA8: 409A0024  bne cr6, 0x83182acc
	if !ctx.cr[6].eq {
	pc = 0x83182ACC; continue 'dispatch;
	}
	// 83182AAC: 817F106C  lwz r11, 0x106c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4204 as u32) ) } as u64;
	// 83182AB0: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83182AB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182AB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83182ABC: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 83182AC0: 409A0008  bne cr6, 0x83182ac8
	if !ctx.cr[6].eq {
	pc = 0x83182AC8; continue 'dispatch;
	}
	// 83182AC4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83182AC8: 4BFF6991  bl 0x83179458
	ctx.lr = 0x83182ACC;
	sub_83179458(ctx, base);
	// 83182ACC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83182AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83182AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83182AD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83182ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83182AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83182AE0 size=200
    let mut pc: u32 = 0x83182AE0;
    'dispatch: loop {
        match pc {
            0x83182AE0 => {
    //   block [0x83182AE0..0x83182BA8)
	// 83182AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83182AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83182AE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83182AEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83182AF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83182AF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83182AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83182AFC: 815E0A24  lwz r10, 0xa24(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2596 as u32) ) } as u64;
	// 83182B00: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83182B04: 409A0008  bne cr6, 0x83182b0c
	if !ctx.cr[6].eq {
	pc = 0x83182B0C; continue 'dispatch;
	}
	// 83182B08: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83182B0C: 815E0A20  lwz r10, 0xa20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2592 as u32) ) } as u64;
	// 83182B10: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83182B14: 409A0008  bne cr6, 0x83182b1c
	if !ctx.cr[6].eq {
	pc = 0x83182B1C; continue 'dispatch;
	}
	// 83182B18: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 83182B1C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83182B20: 419A005C  beq cr6, 0x83182b7c
	if ctx.cr[6].eq {
	pc = 0x83182B7C; continue 'dispatch;
	}
	// 83182B24: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83182B28: 419A004C  beq cr6, 0x83182b74
	if ctx.cr[6].eq {
	pc = 0x83182B74; continue 'dispatch;
	}
	// 83182B2C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83182B30: 409A003C  bne cr6, 0x83182b6c
	if !ctx.cr[6].eq {
	pc = 0x83182B6C; continue 'dispatch;
	}
	// 83182B34: 38800019  li r4, 0x19
	ctx.r[4].s64 = 25;
	// 83182B38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83182B3C: 4BFF68D5  bl 0x83179410
	ctx.lr = 0x83182B40;
	sub_83179410(ctx, base);
	// 83182B40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83182B44: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83182B48: 409A0038  bne cr6, 0x83182b80
	if !ctx.cr[6].eq {
	pc = 0x83182B80; continue 'dispatch;
	}
	// 83182B4C: 4801118D  bl 0x83193cd8
	ctx.lr = 0x83182B50;
	sub_83193CD8(ctx, base);
	// 83182B50: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83182B54: 409A0018  bne cr6, 0x83182b6c
	if !ctx.cr[6].eq {
	pc = 0x83182B6C; continue 'dispatch;
	}
	// 83182B58: 38800048  li r4, 0x48
	ctx.r[4].s64 = 72;
	// 83182B5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83182B60: 4BFF68B1  bl 0x83179410
	ctx.lr = 0x83182B64;
	sub_83179410(ctx, base);
	// 83182B64: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83182B68: 409A0018  bne cr6, 0x83182b80
	if !ctx.cr[6].eq {
	pc = 0x83182B80; continue 'dispatch;
	}
	// 83182B6C: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 83182B70: 48000010  b 0x83182b80
	pc = 0x83182B80; continue 'dispatch;
	// 83182B74: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 83182B78: 48000008  b 0x83182b80
	pc = 0x83182B80; continue 'dispatch;
	// 83182B7C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83182B80: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83182B84: 38800019  li r4, 0x19
	ctx.r[4].s64 = 25;
	// 83182B88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83182B8C: 4BFF68CD  bl 0x83179458
	ctx.lr = 0x83182B90;
	sub_83179458(ctx, base);
	// 83182B90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83182B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83182B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83182B9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83182BA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83182BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83182BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83182BA8 size=168
    let mut pc: u32 = 0x83182BA8;
    'dispatch: loop {
        match pc {
            0x83182BA8 => {
    //   block [0x83182BA8..0x83182C50)
	// 83182BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83182BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83182BB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83182BB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83182BB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83182BBC: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 83182BC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83182BC4: 4BFF684D  bl 0x83179410
	ctx.lr = 0x83182BC8;
	sub_83179410(ctx, base);
	// 83182BC8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83182BCC: 419A0020  beq cr6, 0x83182bec
	if ctx.cr[6].eq {
	pc = 0x83182BEC; continue 'dispatch;
	}
	// 83182BD0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83182BD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83182BD8: 48009281  bl 0x8318be58
	ctx.lr = 0x83182BDC;
	sub_8318BE58(ctx, base);
	// 83182BDC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83182BE0: 419A000C  beq cr6, 0x83182bec
	if ctx.cr[6].eq {
	pc = 0x83182BEC; continue 'dispatch;
	}
	// 83182BE4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83182BE8: 48000050  b 0x83182c38
	pc = 0x83182C38; continue 'dispatch;
	// 83182BEC: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83182BF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83182BF4: 4BFF681D  bl 0x83179410
	ctx.lr = 0x83182BF8;
	sub_83179410(ctx, base);
	// 83182BF8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83182BFC: 419A0018  beq cr6, 0x83182c14
	if ctx.cr[6].eq {
	pc = 0x83182C14; continue 'dispatch;
	}
	// 83182C00: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83182C04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83182C08: 48009251  bl 0x8318be58
	ctx.lr = 0x83182C0C;
	sub_8318BE58(ctx, base);
	// 83182C0C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83182C10: 409AFFD4  bne cr6, 0x83182be4
	if !ctx.cr[6].eq {
	pc = 0x83182BE4; continue 'dispatch;
	}
	// 83182C14: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83182C18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83182C1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83182C20: 4800B3B9  bl 0x8318dfd8
	ctx.lr = 0x83182C24;
	sub_8318DFD8(ctx, base);
	// 83182C24: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83182C28: 409AFFBC  bne cr6, 0x83182be4
	if !ctx.cr[6].eq {
	pc = 0x83182BE4; continue 'dispatch;
	}
	// 83182C2C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83182C30: 2F1F0008  cmpwi cr6, r31, 8
	ctx.cr[6].compare_i32(ctx.r[31].s32, 8, &mut ctx.xer);
	// 83182C34: 4198FFE4  blt cr6, 0x83182c18
	if ctx.cr[6].lt {
	pc = 0x83182C18; continue 'dispatch;
	}
	// 83182C38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83182C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83182C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83182C44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83182C48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83182C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83182C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83182C50 size=148
    let mut pc: u32 = 0x83182C50;
    'dispatch: loop {
        match pc {
            0x83182C50 => {
    //   block [0x83182C50..0x83182CE4)
	// 83182C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83182C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83182C58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83182C5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83182C60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83182C64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83182C68: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83182C6C: 817F17D0  lwz r11, 0x17d0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6096 as u32) ) } as u64;
	// 83182C70: 1D6B0074  mulli r11, r11, 0x74
	ctx.r[11].s64 = ctx.r[11].s64 * 116;
	// 83182C74: 7FCBFA14  add r30, r11, r31
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83182C78: 807E13AC  lwz r3, 0x13ac(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(5036 as u32) ) } as u64;
	// 83182C7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83182C80: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83182C84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83182C88: 4E800421  bctrl
	ctx.lr = 0x83182C8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83182C8C: 817E13B4  lwz r11, 0x13b4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(5044 as u32) ) } as u64;
	// 83182C90: 39200064  li r9, 0x64
	ctx.r[9].s64 = 100;
	// 83182C94: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83182C98: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83182C9C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83182CA0: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83182CA4: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 83182CA8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83182CAC: 4098001C  bge cr6, 0x83182cc8
	if !ctx.cr[6].lt {
	pc = 0x83182CC8; continue 'dispatch;
	}
	// 83182CB0: 38800046  li r4, 0x46
	ctx.r[4].s64 = 70;
	// 83182CB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182CB8: 4BFF6759  bl 0x83179410
	ctx.lr = 0x83182CBC;
	sub_83179410(ctx, base);
	// 83182CBC: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83182CC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83182CC4: 41980008  blt cr6, 0x83182ccc
	if ctx.cr[6].lt {
	pc = 0x83182CCC; continue 'dispatch;
	}
	// 83182CC8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83182CCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83182CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83182CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83182CD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83182CDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83182CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83182CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83182CE8 size=112
    let mut pc: u32 = 0x83182CE8;
    'dispatch: loop {
        match pc {
            0x83182CE8 => {
    //   block [0x83182CE8..0x83182D58)
	// 83182CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83182CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83182CF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83182CF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83182CF8: 81631814  lwz r11, 0x1814(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6164 as u32) ) } as u64;
	// 83182CFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83182D00: 1D6B0074  mulli r11, r11, 0x74
	ctx.r[11].s64 = ctx.r[11].s64 * 116;
	// 83182D04: 7FEB1A14  add r31, r11, r3
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 83182D08: 807F13AC  lwz r3, 0x13ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5036 as u32) ) } as u64;
	// 83182D0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83182D10: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83182D14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83182D18: 4E800421  bctrl
	ctx.lr = 0x83182D1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83182D1C: 817F13B4  lwz r11, 0x13b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5044 as u32) ) } as u64;
	// 83182D20: 39200064  li r9, 0x64
	ctx.r[9].s64 = 100;
	// 83182D24: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83182D28: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83182D2C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83182D30: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 83182D34: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83182D38: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83182D3C: 40980008  bge cr6, 0x83182d44
	if !ctx.cr[6].lt {
	pc = 0x83182D44; continue 'dispatch;
	}
	// 83182D40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83182D44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83182D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83182D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83182D50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83182D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83182D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83182D58 size=116
    let mut pc: u32 = 0x83182D58;
    'dispatch: loop {
        match pc {
            0x83182D58 => {
    //   block [0x83182D58..0x83182DCC)
	// 83182D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83182D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83182D60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83182D64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83182D68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83182D6C: 83E30A5C  lwz r31, 0xa5c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2652 as u32) ) } as u64;
	// 83182D70: 83C30A60  lwz r30, 0xa60(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2656 as u32) ) } as u64;
	// 83182D74: 2F1FFFFC  cmpwi cr6, r31, -4
	ctx.cr[6].compare_i32(ctx.r[31].s32, -4, &mut ctx.xer);
	// 83182D78: 409A000C  bne cr6, 0x83182d84
	if !ctx.cr[6].eq {
	pc = 0x83182D84; continue 'dispatch;
	}
	// 83182D7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83182D80: 48000034  b 0x83182db4
	pc = 0x83182DB4; continue 'dispatch;
	// 83182D84: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83182D88: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83182D8C: 4BFF8A9D  bl 0x8317b828
	ctx.lr = 0x83182D90;
	sub_8317B828(ctx, base);
	// 83182D90: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83182D94: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83182D98: 4198FFE4  blt cr6, 0x83182d7c
	if ctx.cr[6].lt {
	pc = 0x83182D7C; continue 'dispatch;
	}
	// 83182D9C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83182DA0: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83182DA4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83182DA8: 4800CC61  bl 0x8318fa08
	ctx.lr = 0x83182DAC;
	sub_8318FA08(ctx, base);
	// 83182DAC: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 83182DB0: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83182DB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83182DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83182DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83182DC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83182DC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83182DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83182DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83182DD0 size=236
    let mut pc: u32 = 0x83182DD0;
    'dispatch: loop {
        match pc {
            0x83182DD0 => {
    //   block [0x83182DD0..0x83182E74)
	// 83182DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83182DD4: 48025391  bl 0x831a8164
	ctx.lr = 0x83182DD8;
	sub_831A8130(ctx, base);
	// 83182DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83182DDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83182DE0: 817E0A24  lwz r11, 0xa24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2596 as u32) ) } as u64;
	// 83182DE4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83182DE8: 409A001C  bne cr6, 0x83182e04
	if !ctx.cr[6].eq {
	pc = 0x83182E04; continue 'dispatch;
	}
	// 83182DEC: 817E0A20  lwz r11, 0xa20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2592 as u32) ) } as u64;
	// 83182DF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83182DF4: 409A0010  bne cr6, 0x83182e04
	if !ctx.cr[6].eq {
	pc = 0x83182E04; continue 'dispatch;
	}
	// 83182DF8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83182DFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83182E00: 480253B4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83182E04: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83182E08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83182E0C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83182E10: 48009049  bl 0x8318be58
	ctx.lr = 0x83182E14;
	sub_8318BE58(ctx, base);
	// 83182E14: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83182E18: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83182E1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83182E20: 48009039  bl 0x8318be58
	ctx.lr = 0x83182E24;
	sub_8318BE58(ctx, base);
	// 83182E24: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83182E28: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83182E2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83182E30: 48009029  bl 0x8318be58
	ctx.lr = 0x83182E34;
	sub_8318BE58(ctx, base);
	// 83182E34: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83182E38: 38800019  li r4, 0x19
	ctx.r[4].s64 = 25;
	// 83182E3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83182E40: 4BFF65D1  bl 0x83179410
	ctx.lr = 0x83182E44;
	sub_83179410(ctx, base);
	// 83182E44: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 83182E48: 41990048  bgt cr6, 0x83182e90
	if ctx.cr[6].gt {
	pc = 0x83182E90; continue 'dispatch;
	}
	// 83182E4C: 3D808318  lis r12, -0x7ce8
	ctx.r[12].s64 = -2095579136;
	// 83182E50: 398C2E64  addi r12, r12, 0x2e64
	ctx.r[12].s64 = ctx.r[12].s64 + 11876;
	// 83182E54: 5460103A  slwi r0, r3, 2
	ctx.r[0].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 83182E58: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83182E5C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 83182E60: 4E800420  bctr
	match ctx.r[3].u64 {
		0 => {
	pc = 0x83182E8C; continue 'dispatch;
		},
		1 => {
	pc = 0x83182E74; continue 'dispatch;
		},
		2 => {
	pc = 0x83182E7C; continue 'dispatch;
		},
		3 => {
	pc = 0x83182E84; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 83182E64: 83182E8C  lwz r24, 0x2e8c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(11916 as u32) ) } as u64;
	// 83182E68: 83182E74  lwz r24, 0x2e74(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(11892 as u32) ) } as u64;
	// 83182E6C: 83182E7C  lwz r24, 0x2e7c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(11900 as u32) ) } as u64;
	// 83182E70: 83182E84  lwz r24, 0x2e84(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(11908 as u32) ) } as u64;
            }
            0x83182E74 => {
    //   block [0x83182E74..0x83182E7C)
	// 83182E74: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 83182E78: 48000018  b 0x83182e90
	pc = 0x83182E90; continue 'dispatch;
            }
            0x83182E7C => {
    //   block [0x83182E7C..0x83182E84)
	// 83182E7C: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 83182E80: 48000010  b 0x83182e90
	pc = 0x83182E90; continue 'dispatch;
            }
            0x83182E84 => {
    //   block [0x83182E84..0x83182E8C)
	// 83182E84: 7F9FEB78  or r31, r28, r29
	ctx.r[31].u64 = ctx.r[28].u64 | ctx.r[29].u64;
	// 83182E88: 48000008  b 0x83182e90
	pc = 0x83182E90; continue 'dispatch;
            }
            0x83182E8C => {
    //   block [0x83182E8C..0x83182EBC)
	// 83182E8C: 7F9FE838  and r31, r28, r29
	ctx.r[31].u64 = ctx.r[28].u64 & ctx.r[29].u64;
	// 83182E90: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83182E94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83182E98: 48008FD1  bl 0x8318be68
	ctx.lr = 0x83182E9C;
	sub_8318BE68(ctx, base);
	// 83182E9C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83182EA0: 419A0008  beq cr6, 0x83182ea8
	if ctx.cr[6].eq {
	pc = 0x83182EA8; continue 'dispatch;
	}
	// 83182EA4: 7F7FF838  and r31, r27, r31
	ctx.r[31].u64 = ctx.r[27].u64 & ctx.r[31].u64;
	// 83182EA8: 7FEB0034  cntlzw r11, r31
	ctx.r[11].u64 = if ctx.r[31].u32 == 0 { 32 } else { ctx.r[31].u32.leading_zeros() as u64 };
	// 83182EAC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83182EB0: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 83182EB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83182EB8: 480252FC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83182EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83182EC0 size=144
    let mut pc: u32 = 0x83182EC0;
    'dispatch: loop {
        match pc {
            0x83182EC0 => {
    //   block [0x83182EC0..0x83182F50)
	// 83182EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83182EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83182EC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83182ECC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83182ED0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83182ED4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83182ED8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83182EDC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83182EE0: 48008F89  bl 0x8318be68
	ctx.lr = 0x83182EE4;
	sub_8318BE68(ctx, base);
	// 83182EE4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83182EE8: 419A004C  beq cr6, 0x83182f34
	if ctx.cr[6].eq {
	pc = 0x83182F34; continue 'dispatch;
	}
	// 83182EEC: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83182EF0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83182EF4: 41980038  blt cr6, 0x83182f2c
	if ctx.cr[6].lt {
	pc = 0x83182F2C; continue 'dispatch;
	}
	// 83182EF8: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83182EFC: 40990014  ble cr6, 0x83182f10
	if !ctx.cr[6].gt {
	pc = 0x83182F10; continue 'dispatch;
	}
	// 83182F00: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 83182F04: 409A0028  bne cr6, 0x83182f2c
	if !ctx.cr[6].eq {
	pc = 0x83182F2C; continue 'dispatch;
	}
	// 83182F08: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83182F0C: 4800002C  b 0x83182f38
	pc = 0x83182F38; continue 'dispatch;
	// 83182F10: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83182F14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83182F18: 48008F41  bl 0x8318be58
	ctx.lr = 0x83182F1C;
	sub_8318BE58(ctx, base);
	// 83182F1C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83182F20: 419A0014  beq cr6, 0x83182f34
	if ctx.cr[6].eq {
	pc = 0x83182F34; continue 'dispatch;
	}
	// 83182F24: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83182F28: 48000010  b 0x83182f38
	pc = 0x83182F38; continue 'dispatch;
	// 83182F2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83182F30: 48000008  b 0x83182f38
	pc = 0x83182F38; continue 'dispatch;
	// 83182F34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83182F38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83182F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83182F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83182F44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83182F48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83182F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83182F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83182F50 size=100
    let mut pc: u32 = 0x83182F50;
    'dispatch: loop {
        match pc {
            0x83182F50 => {
    //   block [0x83182F50..0x83182FB4)
	// 83182F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83182F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83182F58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83182F5C: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 83182F60: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83182F64: 409A003C  bne cr6, 0x83182fa0
	if !ctx.cr[6].eq {
	pc = 0x83182FA0; continue 'dispatch;
	}
	// 83182F68: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 83182F6C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83182F70: 419A0030  beq cr6, 0x83182fa0
	if ctx.cr[6].eq {
	pc = 0x83182FA0; continue 'dispatch;
	}
	// 83182F74: 81630970  lwz r11, 0x970(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2416 as u32) ) } as u64;
	// 83182F78: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83182F7C: 419A0024  beq cr6, 0x83182fa0
	if ctx.cr[6].eq {
	pc = 0x83182FA0; continue 'dispatch;
	}
	// 83182F80: 4BFF9641  bl 0x8317c5c0
	ctx.lr = 0x83182F84;
	sub_8317C5C0(ctx, base);
	// 83182F84: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 83182F88: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83182F8C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 83182F90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83182F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83182F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83182F9C: 4E800020  blr
	return;
	// 83182FA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83182FA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83182FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83182FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83182FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83182FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83182FB8 size=192
    let mut pc: u32 = 0x83182FB8;
    'dispatch: loop {
        match pc {
            0x83182FB8 => {
    //   block [0x83182FB8..0x83183078)
	// 83182FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83182FBC: 480251B1  bl 0x831a816c
	ctx.lr = 0x83182FC0;
	sub_831A8130(ctx, base);
	// 83182FC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83182FC4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83182FC8: 38800036  li r4, 0x36
	ctx.r[4].s64 = 54;
	// 83182FCC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83182FD0: 4BFF6441  bl 0x83179410
	ctx.lr = 0x83182FD4;
	sub_83179410(ctx, base);
	// 83182FD4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83182FD8: 409A0098  bne cr6, 0x83183070
	if !ctx.cr[6].eq {
	pc = 0x83183070; continue 'dispatch;
	}
	// 83182FDC: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 83182FE0: 419A0090  beq cr6, 0x83183070
	if ctx.cr[6].eq {
	pc = 0x83183070; continue 'dispatch;
	}
	// 83182FE4: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 83182FE8: 617FFFFF  ori r31, r11, 0xffff
	ctx.r[31].u64 = ctx.r[11].u64 | 65535;
	// 83182FEC: 817D0DC4  lwz r11, 0xdc4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(3524 as u32) ) } as u64;
	// 83182FF0: 7D5FF3D6  divw r10, r31, r30
	ctx.r[10].s32 = ctx.r[31].s32 / ctx.r[30].s32;
	// 83182FF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83182FF8: 1D6A03E8  mulli r11, r10, 0x3e8
	ctx.r[11].s64 = ctx.r[10].s64 * 1000;
	// 83182FFC: 3BCBFC18  addi r30, r11, -0x3e8
	ctx.r[30].s64 = ctx.r[11].s64 + -1000;
	// 83183000: 419A0028  beq cr6, 0x83183028
	if ctx.cr[6].eq {
	pc = 0x83183028; continue 'dispatch;
	}
	// 83183004: 817D0DEC  lwz r11, 0xdec(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(3564 as u32) ) } as u64;
	// 83183008: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8318300C: 419A001C  beq cr6, 0x83183028
	if ctx.cr[6].eq {
	pc = 0x83183028; continue 'dispatch;
	}
	// 83183010: 7D7F5BD6  divw r11, r31, r11
	ctx.r[11].s32 = ctx.r[31].s32 / ctx.r[11].s32;
	// 83183014: 1D6B03E8  mulli r11, r11, 0x3e8
	ctx.r[11].s64 = ctx.r[11].s64 * 1000;
	// 83183018: 396BFC18  addi r11, r11, -0x3e8
	ctx.r[11].s64 = ctx.r[11].s64 + -1000;
	// 8318301C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83183020: 40990008  ble cr6, 0x83183028
	if !ctx.cr[6].gt {
	pc = 0x83183028; continue 'dispatch;
	}
	// 83183024: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83183028: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8318302C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83183030: 4BFF63E1  bl 0x83179410
	ctx.lr = 0x83183034;
	sub_83179410(ctx, base);
	// 83183034: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83183038: 409A0028  bne cr6, 0x83183060
	if !ctx.cr[6].eq {
	pc = 0x83183060; continue 'dispatch;
	}
	// 8318303C: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83183040: 396BA100  addi r11, r11, -0x5f00
	ctx.r[11].s64 = ctx.r[11].s64 + -24320;
	// 83183044: 816B01B8  lwz r11, 0x1b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(440 as u32) ) } as u64;
	// 83183048: 7D7F5BD6  divw r11, r31, r11
	ctx.r[11].s32 = ctx.r[31].s32 / ctx.r[11].s32;
	// 8318304C: 1D6B03E8  mulli r11, r11, 0x3e8
	ctx.r[11].s64 = ctx.r[11].s64 * 1000;
	// 83183050: 396BFC18  addi r11, r11, -0x3e8
	ctx.r[11].s64 = ctx.r[11].s64 + -1000;
	// 83183054: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83183058: 40990008  ble cr6, 0x83183060
	if !ctx.cr[6].gt {
	pc = 0x83183060; continue 'dispatch;
	}
	// 8318305C: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83183060: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83183064: 38800036  li r4, 0x36
	ctx.r[4].s64 = 54;
	// 83183068: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8318306C: 4BFF63ED  bl 0x83179458
	ctx.lr = 0x83183070;
	sub_83179458(ctx, base);
	// 83183070: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83183074: 48025148  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83183078 size=28
    let mut pc: u32 = 0x83183078;
    'dispatch: loop {
        match pc {
            0x83183078 => {
    //   block [0x83183078..0x83183094)
	// 83183078: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318307C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83183080: 409A0014  bne cr6, 0x83183094
	if !ctx.cr[6].eq {
		sub_83183094(ctx, base);
		return;
	}
	// 83183084: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83183088: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318308C: 60840204  ori r4, r4, 0x204
	ctx.r[4].u64 = ctx.r[4].u64 | 516;
	// 83183090: 48004468  b 0x831874f8
	sub_831874F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183094(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83183094 size=16
    let mut pc: u32 = 0x83183094;
    'dispatch: loop {
        match pc {
            0x83183094 => {
    //   block [0x83183094..0x831830A4)
	// 83183094: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 83183098: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318309C: 2B0B1FF0  cmplwi cr6, r11, 0x1ff0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8176 as u32, &mut ctx.xer);
	// 831830A0: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831830A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831830A4 size=12
    let mut pc: u32 = 0x831830A4;
    'dispatch: loop {
        match pc {
            0x831830A4 => {
    //   block [0x831830A4..0x831830B0)
	// 831830A4: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 831830A8: 60840205  ori r4, r4, 0x205
	ctx.r[4].u64 = ctx.r[4].u64 | 517;
	// 831830AC: 4800444C  b 0x831874f8
	sub_831874F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831830B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831830B0 size=4
    let mut pc: u32 = 0x831830B0;
    'dispatch: loop {
        match pc {
            0x831830B0 => {
    //   block [0x831830B0..0x831830B4)
	// 831830B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831830B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831830B8 size=28
    let mut pc: u32 = 0x831830B8;
    'dispatch: loop {
        match pc {
            0x831830B8 => {
    //   block [0x831830B8..0x831830D4)
	// 831830B8: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 831830BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831830C0: 394BA100  addi r10, r11, -0x5f00
	ctx.r[10].s64 = ctx.r[11].s64 + -24320;
	// 831830C4: 396A020C  addi r11, r10, 0x20c
	ctx.r[11].s64 = ctx.r[10].s64 + 524;
	// 831830C8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831830CC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 831830D0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831830D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831830D4 size=28
    let mut pc: u32 = 0x831830D4;
    'dispatch: loop {
        match pc {
            0x831830D4 => {
    //   block [0x831830D4..0x831830F0)
	// 831830D4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831830D8: 392A022C  addi r9, r10, 0x22c
	ctx.r[9].s64 = ctx.r[10].s64 + 556;
	// 831830DC: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 831830E0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 831830E4: 4198FFE4  blt cr6, 0x831830c8
	if ctx.cr[6].lt {
		sub_831830B8(ctx, base);
		return;
	}
	// 831830E8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831830EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831830F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831830F0 size=96
    let mut pc: u32 = 0x831830F0;
    'dispatch: loop {
        match pc {
            0x831830F0 => {
    //   block [0x831830F0..0x83183150)
	// 831830F0: 3D408340  lis r10, -0x7cc0
	ctx.r[10].s64 = -2092957696;
	// 831830F4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831830F8: 394AC648  addi r10, r10, -0x39b8
	ctx.r[10].s64 = ctx.r[10].s64 + -14776;
	// 831830FC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83183100: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83183104: 419A004C  beq cr6, 0x83183150
	if ctx.cr[6].eq {
		sub_83183150(ctx, base);
		return;
	}
	// 83183108: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318310C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83183110: 40990040  ble cr6, 0x83183150
	if !ctx.cr[6].gt {
		sub_83183150(ctx, base);
		return;
	}
	// 83183114: 912B1FB0  stw r9, 0x1fb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8112 as u32), ctx.r[9].u32 ) };
	// 83183118: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318311C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83183120: 912B1FB4  stw r9, 0x1fb4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8116 as u32), ctx.r[9].u32 ) };
	// 83183124: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 83183128: 912B1FB8  stw r9, 0x1fb8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8120 as u32), ctx.r[9].u32 ) };
	// 8318312C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 83183130: 912B1FBC  stw r9, 0x1fbc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8124 as u32), ctx.r[9].u32 ) };
	// 83183134: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83183138: 912B1FC0  stw r9, 0x1fc0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8128 as u32), ctx.r[9].u32 ) };
	// 8318313C: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 83183140: 912B1FC4  stw r9, 0x1fc4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8132 as u32), ctx.r[9].u32 ) };
	// 83183144: 814A0018  lwz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 83183148: 914B1FC8  stw r10, 0x1fc8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8136 as u32), ctx.r[10].u32 ) };
	// 8318314C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83183150 size=8
    let mut pc: u32 = 0x83183150;
    'dispatch: loop {
        match pc {
            0x83183150 => {
    //   block [0x83183150..0x83183158)
	// 83183150: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83183154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83183158 size=12
    let mut pc: u32 = 0x83183158;
    'dispatch: loop {
        match pc {
            0x83183158 => {
    //   block [0x83183158..0x83183164)
	// 83183158: 1D630194  mulli r11, r3, 0x194
	ctx.r[11].s64 = ctx.r[3].s64 * 404;
	// 8318315C: 386B0200  addi r3, r11, 0x200
	ctx.r[3].s64 = ctx.r[11].s64 + 512;
	// 83183160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83183168 size=160
    let mut pc: u32 = 0x83183168;
    'dispatch: loop {
        match pc {
            0x83183168 => {
    //   block [0x83183168..0x83183208)
	// 83183168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318316C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83183170: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83183174: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 83183178: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8318317C: 419A0088  beq cr6, 0x83183204
	if ctx.cr[6].eq {
	pc = 0x83183204; continue 'dispatch;
	}
	// 83183180: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 83183184: 4BFFFFD5  bl 0x83183158
	ctx.lr = 0x83183188;
	sub_83183158(ctx, base);
	// 83183188: 7F041800  cmpw cr6, r4, r3
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8318318C: 41980078  blt cr6, 0x83183204
	if ctx.cr[6].lt {
	pc = 0x83183204; continue 'dispatch;
	}
	// 83183190: 2F050002  cmpwi cr6, r5, 2
	ctx.cr[6].compare_i32(ctx.r[5].s32, 2, &mut ctx.xer);
	// 83183194: 41980070  blt cr6, 0x83183204
	if ctx.cr[6].lt {
	pc = 0x83183204; continue 'dispatch;
	}
	// 83183198: 3949007F  addi r10, r9, 0x7f
	ctx.r[10].s64 = ctx.r[9].s64 + 127;
	// 8318319C: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 831831A0: 554A0030  rlwinm r10, r10, 0, 0, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 831831A4: 396BC648  addi r11, r11, -0x39b8
	ctx.r[11].s64 = ctx.r[11].s64 + -14776;
	// 831831A8: 1D050088  mulli r8, r5, 0x88
	ctx.r[8].s64 = ctx.r[5].s64 * 136;
	// 831831AC: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 831831B0: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 831831B4: 54A71838  slwi r7, r5, 3
	ctx.r[7].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831831B8: 394A007F  addi r10, r10, 0x7f
	ctx.r[10].s64 = ctx.r[10].s64 + 127;
	// 831831BC: 54A8103A  slwi r8, r5, 2
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831831C0: 554A0030  rlwinm r10, r10, 0, 0, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 831831C4: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 831831C8: 7D475214  add r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 831831CC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831831D0: 394A007F  addi r10, r10, 0x7f
	ctx.r[10].s64 = ctx.r[10].s64 + 127;
	// 831831D4: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 831831D8: 554A0030  rlwinm r10, r10, 0, 0, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 831831DC: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 831831E0: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 831831E4: 90AB0008  stw r5, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 831831E8: 394A007F  addi r10, r10, 0x7f
	ctx.r[10].s64 = ctx.r[10].s64 + 127;
	// 831831EC: 554A0030  rlwinm r10, r10, 0, 0, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 831831F0: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 831831F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831831F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831831FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83183200: 4E800020  blr
	return;
	// 83183204: 48000000  b 0x83183204
	pc = 0x83183204; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83183208 size=12
    let mut pc: u32 = 0x83183208;
    'dispatch: loop {
        match pc {
            0x83183208 => {
    //   block [0x83183208..0x83183214)
	// 83183208: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318320C: 386BC648  addi r3, r11, -0x39b8
	ctx.r[3].s64 = ctx.r[11].s64 + -14776;
	// 83183210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83183218 size=112
    let mut pc: u32 = 0x83183218;
    'dispatch: loop {
        match pc {
            0x83183218 => {
    //   block [0x83183218..0x83183288)
	// 83183218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318321C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83183220: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83183224: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83183228: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 8318322C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83183230: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83183234: 4800FE55  bl 0x83193088
	ctx.lr = 0x83183238;
	sub_83193088(ctx, base);
	// 83183238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318323C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 83183240: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83183244: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83183248: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8318324C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83183250: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83183254: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83183258: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8318325C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83183260: 913F001C  stw r9, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 83183264: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83183268: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 8318326C: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 83183270: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 83183274: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83183278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318327C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83183280: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83183284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83183288 size=20
    let mut pc: u32 = 0x83183288;
    'dispatch: loop {
        match pc {
            0x83183288 => {
    //   block [0x83183288..0x8318329C)
	// 83183288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318328C: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 83183290: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 83183294: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 83183298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831832A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831832A0 size=108
    let mut pc: u32 = 0x831832A0;
    'dispatch: loop {
        match pc {
            0x831832A0 => {
    //   block [0x831832A0..0x8318330C)
	// 831832A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831832A4: 48024EC9  bl 0x831a816c
	ctx.lr = 0x831832A8;
	sub_831A8130(ctx, base);
	// 831832A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831832AC: 38A00038  li r5, 0x38
	ctx.r[5].s64 = 56;
	// 831832B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831832B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831832B8: 4800FDD1  bl 0x83193088
	ctx.lr = 0x831832BC;
	sub_83193088(ctx, base);
	// 831832BC: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 831832C0: 3BC00005  li r30, 5
	ctx.r[30].s64 = 5;
	// 831832C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831832C8: 480102B1  bl 0x83193578
	ctx.lr = 0x831832CC;
	sub_83193578(ctx, base);
	// 831832CC: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 831832D0: 3BBD0020  addi r29, r29, 0x20
	ctx.r[29].s64 = ctx.r[29].s64 + 32;
	// 831832D4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 831832D8: 409AFFEC  bne cr6, 0x831832c4
	if !ctx.cr[6].eq {
	pc = 0x831832C4; continue 'dispatch;
	}
	// 831832DC: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 831832E0: 48010299  bl 0x83193578
	ctx.lr = 0x831832E4;
	sub_83193578(ctx, base);
	// 831832E4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 831832E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831832EC: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831832F0: D01F00DC  stfs f0, 0xdc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), tmp.u32 ) };
	// 831832F4: F97F00C0  std r11, 0xc0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u64 ) };
	// 831832F8: F97F00C8  std r11, 0xc8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[11].u64 ) };
	// 831832FC: F97F00D0  std r11, 0xd0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u64 ) };
	// 83183300: 917F00D8  stw r11, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[11].u32 ) };
	// 83183304: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83183308: 48024EB4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83183310 size=36
    let mut pc: u32 = 0x83183310;
    'dispatch: loop {
        match pc {
            0x83183310 => {
    //   block [0x83183310..0x83183334)
	// 83183310: 81230950  lwz r9, 0x950(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2384 as u32) ) } as u64;
	// 83183314: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 83183318: 81630D5C  lwz r11, 0xd5c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3420 as u32) ) } as u64;
	// 8318331C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83183320: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83183324: 38A30950  addi r5, r3, 0x950
	ctx.r[5].s64 = ctx.r[3].s64 + 2384;
	// 83183328: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318332C: 91430950  stw r10, 0x950(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(2384 as u32), ctx.r[10].u32 ) };
	// 83183330: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183334(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83183334 size=12
    let mut pc: u32 = 0x83183334;
    'dispatch: loop {
        match pc {
            0x83183334 => {
    //   block [0x83183334..0x83183340)
	// 83183334: 80630D60  lwz r3, 0xd60(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3424 as u32) ) } as u64;
	// 83183338: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318333C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83183340 size=4
    let mut pc: u32 = 0x83183340;
    'dispatch: loop {
        match pc {
            0x83183340 => {
    //   block [0x83183340..0x83183344)
	// 83183340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83183348 size=36
    let mut pc: u32 = 0x83183348;
    'dispatch: loop {
        match pc {
            0x83183348 => {
    //   block [0x83183348..0x8318336C)
	// 83183348: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318334C: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 83183350: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83183354: 810B0954  lwz r8, 0x954(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2388 as u32) ) } as u64;
	// 83183358: 814B0D64  lwz r10, 0xd64(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3428 as u32) ) } as u64;
	// 8318335C: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 83183360: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83183364: 912B0954  stw r9, 0x954(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(2388 as u32), ctx.r[9].u32 ) };
	// 83183368: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318336C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318336C size=16
    let mut pc: u32 = 0x8318336C;
    'dispatch: loop {
        match pc {
            0x8318336C => {
    //   block [0x8318336C..0x8318337C)
	// 8318336C: 806B0D68  lwz r3, 0xd68(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3432 as u32) ) } as u64;
	// 83183370: 38AB0950  addi r5, r11, 0x950
	ctx.r[5].s64 = ctx.r[11].s64 + 2384;
	// 83183374: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83183378: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318337C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318337C size=4
    let mut pc: u32 = 0x8318337C;
    'dispatch: loop {
        match pc {
            0x8318337C => {
    //   block [0x8318337C..0x83183380)
	// 8318337C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83183380 size=8
    let mut pc: u32 = 0x83183380;
    'dispatch: loop {
        match pc {
            0x83183380 => {
    //   block [0x83183380..0x83183388)
	// 83183380: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83183384: 480089F4  b 0x8318bd78
	sub_8318BD78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83183388 size=20
    let mut pc: u32 = 0x83183388;
    'dispatch: loop {
        match pc {
            0x83183388 => {
    //   block [0x83183388..0x8318339C)
	// 83183388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318338C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83183390: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83183394: 9163004C  stw r11, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 83183398: 480089E0  b 0x8318bd78
	sub_8318BD78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831833A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831833A0 size=20
    let mut pc: u32 = 0x831833A0;
    'dispatch: loop {
        match pc {
            0x831833A0 => {
    //   block [0x831833A0..0x831833B4)
	// 831833A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831833A4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 831833A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831833AC: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 831833B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831833B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831833B8 size=20
    let mut pc: u32 = 0x831833B8;
    'dispatch: loop {
        match pc {
            0x831833B8 => {
    //   block [0x831833B8..0x831833CC)
	// 831833B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831833BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831833C0: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 831833C4: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 831833C8: 48008A20  b 0x8318bde8
	sub_8318BDE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831833D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831833D0 size=16
    let mut pc: u32 = 0x831833D0;
    'dispatch: loop {
        match pc {
            0x831833D0 => {
    //   block [0x831833D0..0x831833E0)
	// 831833D0: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 831833D4: 396BA100  addi r11, r11, -0x5f00
	ctx.r[11].s64 = ctx.r[11].s64 + -24320;
	// 831833D8: 906B0204  stw r3, 0x204(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(516 as u32), ctx.r[3].u32 ) };
	// 831833DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831833E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831833E0 size=16
    let mut pc: u32 = 0x831833E0;
    'dispatch: loop {
        match pc {
            0x831833E0 => {
    //   block [0x831833E0..0x831833F0)
	// 831833E0: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 831833E4: 396BA100  addi r11, r11, -0x5f00
	ctx.r[11].s64 = ctx.r[11].s64 + -24320;
	// 831833E8: 806B0204  lwz r3, 0x204(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(516 as u32) ) } as u64;
	// 831833EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831833F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831833F0 size=96
    let mut pc: u32 = 0x831833F0;
    'dispatch: loop {
        match pc {
            0x831833F0 => {
    //   block [0x831833F0..0x83183450)
	// 831833F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831833F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831833F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831833FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83183400: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83183404: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83183408: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8318340C: 409A0020  bne cr6, 0x8318342c
	if !ctx.cr[6].eq {
	pc = 0x8318342C; continue 'dispatch;
	}
	// 83183410: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83183414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83183418: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 8318341C: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83183420: 480089C9  bl 0x8318bde8
	ctx.lr = 0x83183424;
	sub_8318BDE8(ctx, base);
	// 83183424: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83183428: 409A0014  bne cr6, 0x8318343c
	if !ctx.cr[6].eq {
	pc = 0x8318343C; continue 'dispatch;
	}
	// 8318342C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83183430: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83183434: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83183438: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8318343C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83183440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83183444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83183448: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318344C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83183450 size=108
    let mut pc: u32 = 0x83183450;
    'dispatch: loop {
        match pc {
            0x83183450 => {
    //   block [0x83183450..0x831834BC)
	// 83183450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83183454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83183458: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318345C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83183460: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83183464: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83183468: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318346C: 48003E75  bl 0x831872e0
	ctx.lr = 0x83183470;
	sub_831872E0(ctx, base);
	// 83183470: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83183474: 419A0018  beq cr6, 0x8318348c
	if ctx.cr[6].eq {
	pc = 0x8318348C; continue 'dispatch;
	}
	// 83183478: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318347C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83183480: 60840134  ori r4, r4, 0x134
	ctx.r[4].u64 = ctx.r[4].u64 | 308;
	// 83183484: 48004075  bl 0x831874f8
	ctx.lr = 0x83183488;
	sub_831874F8(ctx, base);
	// 83183488: 4800001C  b 0x831834a4
	pc = 0x831834A4; continue 'dispatch;
	// 8318348C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83183490: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83183494: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 83183498: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318349C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831834A0: 48008949  bl 0x8318bde8
	ctx.lr = 0x831834A4;
	sub_8318BDE8(ctx, base);
	// 831834A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831834A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831834AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831834B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831834B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831834B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831834C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831834C0 size=120
    let mut pc: u32 = 0x831834C0;
    'dispatch: loop {
        match pc {
            0x831834C0 => {
    //   block [0x831834C0..0x83183538)
	// 831834C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831834C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831834C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831834CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831834D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831834D4: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 831834D8: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 831834DC: 48003E05  bl 0x831872e0
	ctx.lr = 0x831834E0;
	sub_831872E0(ctx, base);
	// 831834E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831834E4: 419A0028  beq cr6, 0x8318350c
	if ctx.cr[6].eq {
	pc = 0x8318350C; continue 'dispatch;
	}
	// 831834E8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 831834EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831834F0: 60840135  ori r4, r4, 0x135
	ctx.r[4].u64 = ctx.r[4].u64 | 309;
	// 831834F4: 48004005  bl 0x831874f8
	ctx.lr = 0x831834F8;
	sub_831874F8(ctx, base);
	// 831834F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831834FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83183500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83183504: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83183508: 4E800020  blr
	return;
	// 8318350C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 83183510: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 83183514: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 83183518: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318351C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183520: 480088C9  bl 0x8318bde8
	ctx.lr = 0x83183524;
	sub_8318BDE8(ctx, base);
	// 83183524: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83183528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318352C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83183530: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83183534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83183538 size=132
    let mut pc: u32 = 0x83183538;
    'dispatch: loop {
        match pc {
            0x83183538 => {
    //   block [0x83183538..0x831835BC)
	// 83183538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318353C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83183540: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83183544: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83183548: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318354C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83183550: 48003D91  bl 0x831872e0
	ctx.lr = 0x83183554;
	sub_831872E0(ctx, base);
	// 83183554: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83183558: 419A0018  beq cr6, 0x83183570
	if ctx.cr[6].eq {
	pc = 0x83183570; continue 'dispatch;
	}
	// 8318355C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83183560: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83183564: 6084013D  ori r4, r4, 0x13d
	ctx.r[4].u64 = ctx.r[4].u64 | 317;
	// 83183568: 48003F91  bl 0x831874f8
	ctx.lr = 0x8318356C;
	sub_831874F8(ctx, base);
	// 8318356C: 48000038  b 0x831835a4
	pc = 0x831835A4; continue 'dispatch;
	// 83183570: 83DF174C  lwz r30, 0x174c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5964 as u32) ) } as u64;
	// 83183574: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183578: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318357C: 4800AA5D  bl 0x8318dfd8
	ctx.lr = 0x83183580;
	sub_8318DFD8(ctx, base);
	// 83183580: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83183584: 419A001C  beq cr6, 0x831835a0
	if ctx.cr[6].eq {
	pc = 0x831835A0; continue 'dispatch;
	}
	// 83183588: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8318358C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83183590: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183594: 4800AA2D  bl 0x8318dfc0
	ctx.lr = 0x83183598;
	sub_8318DFC0(ctx, base);
	// 83183598: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8318359C: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 831835A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831835A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831835A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831835AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831835B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831835B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831835B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831835C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831835C0 size=140
    let mut pc: u32 = 0x831835C0;
    'dispatch: loop {
        match pc {
            0x831835C0 => {
    //   block [0x831835C0..0x8318364C)
	// 831835C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831835C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831835C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831835CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831835D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831835D4: 4800FE7D  bl 0x83193450
	ctx.lr = 0x831835D8;
	sub_83193450(ctx, base);
	// 831835D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831835DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831835E0: F97F1F20  std r11, 0x1f20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(7968 as u32), ctx.r[11].u64 ) };
	// 831835E4: 4800FF3D  bl 0x83193520
	ctx.lr = 0x831835E8;
	sub_83193520(ctx, base);
	// 831835E8: E95F1F20  ld r10, 0x1f20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(7968 as u32) ) };
	// 831835EC: E93F1F18  ld r9, 0x1f18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(7960 as u32) ) };
	// 831835F0: 817F0968  lwz r11, 0x968(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2408 as u32) ) } as u64;
	// 831835F4: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 831835F8: F87F1F28  std r3, 0x1f28(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(7976 as u32), ctx.r[3].u64 ) };
	// 831835FC: 2F2A0000  cmpdi cr6, r10, 0
	ctx.cr[6].compare_i64(ctx.r[10].s64, 0, &mut ctx.xer);
	// 83183600: 917F1F30  stw r11, 0x1f30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7984 as u32), ctx.r[11].u32 ) };
	// 83183604: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 83183608: 419A0030  beq cr6, 0x83183638
	if ctx.cr[6].eq {
	pc = 0x83183638; continue 'dispatch;
	}
	// 8318360C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 83183610: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83183614: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83183618: 7D6B19D2  mulld r11, r11, r3
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[3].s64;
	// 8318361C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 83183620: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 83183624: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83183628: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8318362C: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 83183630: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 83183634: D01F1F34  stfs f0, 0x1f34(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7988 as u32), tmp.u32 ) };
	// 83183638: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318363C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83183640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83183644: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83183648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83183650 size=28
    let mut pc: u32 = 0x83183650;
    'dispatch: loop {
        match pc {
            0x83183650 => {
    //   block [0x83183650..0x8318366C)
	// 83183650: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83183654: 816A005C  lwz r11, 0x5c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(92 as u32) ) } as u64;
	// 83183658: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318365C: 409A0010  bne cr6, 0x8318366c
	if !ctx.cr[6].eq {
		sub_8318366C(ctx, base);
		return;
	}
	// 83183660: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83183664: 908A005C  stw r4, 0x5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 83183668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318366C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318366C size=16
    let mut pc: u32 = 0x8318366C;
    'dispatch: loop {
        match pc {
            0x8318366C => {
    //   block [0x8318366C..0x8318367C)
	// 8318366C: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 83183670: 409A000C  bne cr6, 0x8318367c
	if !ctx.cr[6].eq {
		sub_8318367C(ctx, base);
		return;
	}
	// 83183674: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83183678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318367C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318367C size=16
    let mut pc: u32 = 0x8318367C;
    'dispatch: loop {
        match pc {
            0x8318367C => {
    //   block [0x8318367C..0x8318368C)
	// 8318367C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83183680: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 83183684: 60840207  ori r4, r4, 0x207
	ctx.r[4].u64 = ctx.r[4].u64 | 519;
	// 83183688: 48003E70  b 0x831874f8
	sub_831874F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83183690 size=96
    let mut pc: u32 = 0x83183690;
    'dispatch: loop {
        match pc {
            0x83183690 => {
    //   block [0x83183690..0x831836F0)
	// 83183690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83183694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83183698: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318369C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831836A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831836A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831836A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831836AC: 48003C35  bl 0x831872e0
	ctx.lr = 0x831836B0;
	sub_831872E0(ctx, base);
	// 831836B0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831836B4: 419A0018  beq cr6, 0x831836cc
	if ctx.cr[6].eq {
	pc = 0x831836CC; continue 'dispatch;
	}
	// 831836B8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 831836BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831836C0: 60840139  ori r4, r4, 0x139
	ctx.r[4].u64 = ctx.r[4].u64 | 313;
	// 831836C4: 48003E35  bl 0x831874f8
	ctx.lr = 0x831836C8;
	sub_831874F8(ctx, base);
	// 831836C8: 48000010  b 0x831836d8
	pc = 0x831836D8; continue 'dispatch;
	// 831836CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831836D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831836D4: 4800B205  bl 0x8318e8d8
	ctx.lr = 0x831836D8;
	sub_8318E8D8(ctx, base);
	// 831836D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831836DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831836E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831836E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831836E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831836EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831836F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831836F0 size=44
    let mut pc: u32 = 0x831836F0;
    'dispatch: loop {
        match pc {
            0x831836F0 => {
    //   block [0x831836F0..0x8318371C)
	// 831836F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831836F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831836F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831836FC: 4BFFF15D  bl 0x83182858
	ctx.lr = 0x83183700;
	sub_83182858(ctx, base);
	// 83183700: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 83183704: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83183708: 916AA348  stw r11, -0x5cb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-23736 as u32), ctx.r[11].u32 ) };
	// 8318370C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83183710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83183714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83183718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83183720 size=52
    let mut pc: u32 = 0x83183720;
    'dispatch: loop {
        match pc {
            0x83183720 => {
    //   block [0x83183720..0x83183754)
	// 83183720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83183724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83183728: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318372C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83183730: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83183734: 4BFFF185  bl 0x831828b8
	ctx.lr = 0x83183738;
	sub_831828B8(ctx, base);
	// 83183738: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318373C: 480104CD  bl 0x83193c08
	ctx.lr = 0x83183740;
	sub_83193C08(ctx, base);
	// 83183740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83183744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83183748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318374C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83183750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83183758 size=140
    let mut pc: u32 = 0x83183758;
    'dispatch: loop {
        match pc {
            0x83183758 => {
    //   block [0x83183758..0x831837E4)
	// 83183758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318375C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83183760: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83183764: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83183768: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318376C: 4BFFF225  bl 0x83182990
	ctx.lr = 0x83183770;
	sub_83182990(ctx, base);
	// 83183770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183774: 4BFFF2CD  bl 0x83182a40
	ctx.lr = 0x83183778;
	sub_83182A40(ctx, base);
	// 83183778: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318377C: 4BFFF365  bl 0x83182ae0
	ctx.lr = 0x83183780;
	sub_83182AE0(ctx, base);
	// 83183780: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83183784: 806BA32C  lwz r3, -0x5cd4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83183788: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318378C: 419A0044  beq cr6, 0x831837d0
	if ctx.cr[6].eq {
	pc = 0x831837D0; continue 'dispatch;
	}
	// 83183790: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83183794: 395F0A20  addi r10, r31, 0xa20
	ctx.r[10].s64 = ctx.r[31].s64 + 2592;
	// 83183798: 396BE568  addi r11, r11, -0x1a98
	ctx.r[11].s64 = ctx.r[11].s64 + -6808;
	// 8318379C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 831837A0: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 831837A4: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 831837A8: 395F0A24  addi r10, r31, 0xa24
	ctx.r[10].s64 = ctx.r[31].s64 + 2596;
	// 831837AC: 914B0024  stw r10, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 831837B0: 395F0A48  addi r10, r31, 0xa48
	ctx.r[10].s64 = ctx.r[31].s64 + 2632;
	// 831837B4: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 831837B8: 395F0A70  addi r10, r31, 0xa70
	ctx.r[10].s64 = ctx.r[31].s64 + 2672;
	// 831837BC: 914B003C  stw r10, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 831837C0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831837C4: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 831837C8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831837CC: 4E800421  bctrl
	ctx.lr = 0x831837D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831837D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831837D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831837D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831837DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831837E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831837E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831837E8 size=116
    let mut pc: u32 = 0x831837E8;
    'dispatch: loop {
        match pc {
            0x831837E8 => {
    //   block [0x831837E8..0x8318385C)
	// 831837E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831837EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831837F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831837F4: 81630A44  lwz r11, 0xa44(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2628 as u32) ) } as u64;
	// 831837F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831837FC: 409A0018  bne cr6, 0x83183814
	if !ctx.cr[6].eq {
	pc = 0x83183814; continue 'dispatch;
	}
	// 83183800: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83183804: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83183808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318380C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83183810: 4E800020  blr
	return;
	// 83183814: 81630A20  lwz r11, 0xa20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2592 as u32) ) } as u64;
	// 83183818: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318381C: 419AFFE4  beq cr6, 0x83183800
	if ctx.cr[6].eq {
	pc = 0x83183800; continue 'dispatch;
	}
	// 83183820: 81631040  lwz r11, 0x1040(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4160 as u32) ) } as u64;
	// 83183824: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83183828: 409AFFD8  bne cr6, 0x83183800
	if !ctx.cr[6].eq {
	pc = 0x83183800; continue 'dispatch;
	}
	// 8318382C: 8163105C  lwz r11, 0x105c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4188 as u32) ) } as u64;
	// 83183830: 81430AC0  lwz r10, 0xac0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2752 as u32) ) } as u64;
	// 83183834: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83183838: 4098FFC8  bge cr6, 0x83183800
	if !ctx.cr[6].lt {
	pc = 0x83183800; continue 'dispatch;
	}
	// 8318383C: 4BFFF595  bl 0x83182dd0
	ctx.lr = 0x83183840;
	sub_83182DD0(ctx, base);
	// 83183840: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 83183844: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83183848: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8318384C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83183850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83183854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83183858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83183860 size=380
    let mut pc: u32 = 0x83183860;
    'dispatch: loop {
        match pc {
            0x83183860 => {
    //   block [0x83183860..0x831839DC)
	// 83183860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83183864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83183868: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318386C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83183870: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83183874: 38800043  li r4, 0x43
	ctx.r[4].s64 = 67;
	// 83183878: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318387C: 4BFF5B95  bl 0x83179410
	ctx.lr = 0x83183880;
	sub_83179410(ctx, base);
	// 83183880: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83183884: 419A013C  beq cr6, 0x831839c0
	if ctx.cr[6].eq {
	pc = 0x831839C0; continue 'dispatch;
	}
	// 83183888: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8318388C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183890: 4BFF5B81  bl 0x83179410
	ctx.lr = 0x83183894;
	sub_83179410(ctx, base);
	// 83183894: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83183898: 419A0128  beq cr6, 0x831839c0
	if ctx.cr[6].eq {
	pc = 0x831839C0; continue 'dispatch;
	}
	// 8318389C: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 831838A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831838A4: 409A011C  bne cr6, 0x831839c0
	if !ctx.cr[6].eq {
	pc = 0x831839C0; continue 'dispatch;
	}
	// 831838A8: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 831838AC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 831838B0: 409A0110  bne cr6, 0x831839c0
	if !ctx.cr[6].eq {
	pc = 0x831839C0; continue 'dispatch;
	}
	// 831838B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831838B8: 4BFFF2F1  bl 0x83182ba8
	ctx.lr = 0x831838BC;
	sub_83182BA8(ctx, base);
	// 831838BC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831838C0: 409A0100  bne cr6, 0x831839c0
	if !ctx.cr[6].eq {
	pc = 0x831839C0; continue 'dispatch;
	}
	// 831838C4: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 831838C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831838CC: 4BFF5B45  bl 0x83179410
	ctx.lr = 0x831838D0;
	sub_83179410(ctx, base);
	// 831838D0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831838D4: 409A0010  bne cr6, 0x831838e4
	if !ctx.cr[6].eq {
	pc = 0x831838E4; continue 'dispatch;
	}
	// 831838D8: 817F0978  lwz r11, 0x978(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2424 as u32) ) } as u64;
	// 831838DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831838E0: 419A00E0  beq cr6, 0x831839c0
	if ctx.cr[6].eq {
	pc = 0x831839C0; continue 'dispatch;
	}
	// 831838E4: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 831838E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831838EC: 4BFF5B25  bl 0x83179410
	ctx.lr = 0x831838F0;
	sub_83179410(ctx, base);
	// 831838F0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831838F4: 409A0018  bne cr6, 0x8318390c
	if !ctx.cr[6].eq {
	pc = 0x8318390C; continue 'dispatch;
	}
	// 831838F8: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 831838FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183900: 4800AE91  bl 0x8318e790
	ctx.lr = 0x83183904;
	sub_8318E790(ctx, base);
	// 83183904: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83183908: 419900B8  bgt cr6, 0x831839c0
	if ctx.cr[6].gt {
	pc = 0x831839C0; continue 'dispatch;
	}
	// 8318390C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83183910: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183914: 48008555  bl 0x8318be68
	ctx.lr = 0x83183918;
	sub_8318BE68(ctx, base);
	// 83183918: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318391C: 419A0018  beq cr6, 0x83183934
	if ctx.cr[6].eq {
	pc = 0x83183934; continue 'dispatch;
	}
	// 83183920: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83183924: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183928: 4800AE69  bl 0x8318e790
	ctx.lr = 0x8318392C;
	sub_8318E790(ctx, base);
	// 8318392C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83183930: 41990090  bgt cr6, 0x831839c0
	if ctx.cr[6].gt {
	pc = 0x831839C0; continue 'dispatch;
	}
	// 83183934: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 83183938: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318393C: 4BFF5AD5  bl 0x83179410
	ctx.lr = 0x83183940;
	sub_83179410(ctx, base);
	// 83183940: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83183944: 409A0014  bne cr6, 0x83183958
	if !ctx.cr[6].eq {
	pc = 0x83183958; continue 'dispatch;
	}
	// 83183948: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318394C: 4BFFF305  bl 0x83182c50
	ctx.lr = 0x83183950;
	sub_83182C50(ctx, base);
	// 83183950: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83183954: 409A006C  bne cr6, 0x831839c0
	if !ctx.cr[6].eq {
	pc = 0x831839C0; continue 'dispatch;
	}
	// 83183958: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8318395C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83183960: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183964: 4BFF7EC5  bl 0x8317b828
	ctx.lr = 0x83183968;
	sub_8317B828(ctx, base);
	// 83183968: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318396C: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 83183970: 83DF1010  lwz r30, 0x1010(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4112 as u32) ) } as u64;
	// 83183974: 83FF1014  lwz r31, 0x1014(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4116 as u32) ) } as u64;
	// 83183978: 4BFF5A99  bl 0x83179410
	ctx.lr = 0x8318397C;
	sub_83179410(ctx, base);
	// 8318397C: 3CA0000F  lis r5, 0xf
	ctx.r[5].s64 = 983040;
	// 83183980: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83183984: 60A54240  ori r5, r5, 0x4240
	ctx.r[5].u64 = ctx.r[5].u64 | 16960;
	// 83183988: 48007E51  bl 0x8318b7d8
	ctx.lr = 0x8318398C;
	sub_8318B7D8(ctx, base);
	// 8318398C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83183990: 7CA3F050  subf r5, r3, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[3].s64;
	// 83183994: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83183998: 40990028  ble cr6, 0x831839c0
	if !ctx.cr[6].gt {
	pc = 0x831839C0; continue 'dispatch;
	}
	// 8318399C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 831839A0: 40990020  ble cr6, 0x831839c0
	if !ctx.cr[6].gt {
	pc = 0x831839C0; continue 'dispatch;
	}
	// 831839A4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 831839A8: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831839AC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 831839B0: 4800C059  bl 0x8318fa08
	ctx.lr = 0x831839B4;
	sub_8318FA08(ctx, base);
	// 831839B4: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 831839B8: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 831839BC: 48000008  b 0x831839c4
	pc = 0x831839C4; continue 'dispatch;
	// 831839C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831839C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831839C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831839CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831839D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831839D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831839D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831839E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831839E0 size=224
    let mut pc: u32 = 0x831839E0;
    'dispatch: loop {
        match pc {
            0x831839E0 => {
    //   block [0x831839E0..0x83183AC0)
	// 831839E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831839E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831839E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831839EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831839F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831839F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831839F8: 4BFFF1B1  bl 0x83182ba8
	ctx.lr = 0x831839FC;
	sub_83182BA8(ctx, base);
	// 831839FC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83183A00: 419A000C  beq cr6, 0x83183a0c
	if ctx.cr[6].eq {
	pc = 0x83183A0C; continue 'dispatch;
	}
	// 83183A04: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83183A08: 480000A0  b 0x83183aa8
	pc = 0x83183AA8; continue 'dispatch;
	// 83183A0C: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 83183A10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183A14: 4BFF59FD  bl 0x83179410
	ctx.lr = 0x83183A18;
	sub_83179410(ctx, base);
	// 83183A18: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83183A1C: 409A0014  bne cr6, 0x83183a30
	if !ctx.cr[6].eq {
	pc = 0x83183A30; continue 'dispatch;
	}
	// 83183A20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183A24: 4BFFF22D  bl 0x83182c50
	ctx.lr = 0x83183A28;
	sub_83182C50(ctx, base);
	// 83183A28: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83183A2C: 409AFFD8  bne cr6, 0x83183a04
	if !ctx.cr[6].eq {
	pc = 0x83183A04; continue 'dispatch;
	}
	// 83183A30: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83183A34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183A38: 4BFF59D9  bl 0x83179410
	ctx.lr = 0x83183A3C;
	sub_83179410(ctx, base);
	// 83183A3C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83183A40: 409A0014  bne cr6, 0x83183a54
	if !ctx.cr[6].eq {
	pc = 0x83183A54; continue 'dispatch;
	}
	// 83183A44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183A48: 4BFFF2A1  bl 0x83182ce8
	ctx.lr = 0x83183A4C;
	sub_83182CE8(ctx, base);
	// 83183A4C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83183A50: 409AFFB4  bne cr6, 0x83183a04
	if !ctx.cr[6].eq {
	pc = 0x83183A04; continue 'dispatch;
	}
	// 83183A54: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83183A58: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83183A5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183A60: 4BFF7DC9  bl 0x8317b828
	ctx.lr = 0x83183A64;
	sub_8317B828(ctx, base);
	// 83183A64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183A68: 38800045  li r4, 0x45
	ctx.r[4].s64 = 69;
	// 83183A6C: 83DF1010  lwz r30, 0x1010(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4112 as u32) ) } as u64;
	// 83183A70: 83FF1014  lwz r31, 0x1014(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4116 as u32) ) } as u64;
	// 83183A74: 4BFF599D  bl 0x83179410
	ctx.lr = 0x83183A78;
	sub_83179410(ctx, base);
	// 83183A78: 3CA0000F  lis r5, 0xf
	ctx.r[5].s64 = 983040;
	// 83183A7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83183A80: 60A54240  ori r5, r5, 0x4240
	ctx.r[5].u64 = ctx.r[5].u64 | 16960;
	// 83183A84: 48007D55  bl 0x8318b7d8
	ctx.lr = 0x83183A88;
	sub_8318B7D8(ctx, base);
	// 83183A88: 7CA3F050  subf r5, r3, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[3].s64;
	// 83183A8C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83183A90: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83183A94: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83183A98: 4800BF71  bl 0x8318fa08
	ctx.lr = 0x83183A9C;
	sub_8318FA08(ctx, base);
	// 83183A9C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 83183AA0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83183AA4: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 83183AA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83183AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83183AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83183AB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83183AB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83183ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83183AC0 size=196
    let mut pc: u32 = 0x83183AC0;
    'dispatch: loop {
        match pc {
            0x83183AC0 => {
    //   block [0x83183AC0..0x83183B84)
	// 83183AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83183AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83183AC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83183ACC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83183AD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83183AD4: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83183AD8: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83183ADC: 409A0090  bne cr6, 0x83183b6c
	if !ctx.cr[6].eq {
	pc = 0x83183B6C; continue 'dispatch;
	}
	// 83183AE0: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83183AE4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83183AE8: 419A0084  beq cr6, 0x83183b6c
	if ctx.cr[6].eq {
	pc = 0x83183B6C; continue 'dispatch;
	}
	// 83183AEC: 817F0970  lwz r11, 0x970(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2416 as u32) ) } as u64;
	// 83183AF0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83183AF4: 419A0078  beq cr6, 0x83183b6c
	if ctx.cr[6].eq {
	pc = 0x83183B6C; continue 'dispatch;
	}
	// 83183AF8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83183AFC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83183B00: 4BFF9041  bl 0x8317cb40
	ctx.lr = 0x83183B04;
	sub_8317CB40(ctx, base);
	// 83183B04: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83183B08: 409A0064  bne cr6, 0x83183b6c
	if !ctx.cr[6].eq {
	pc = 0x83183B6C; continue 'dispatch;
	}
	// 83183B0C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83183B10: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83183B14: 41980058  blt cr6, 0x83183b6c
	if ctx.cr[6].lt {
	pc = 0x83183B6C; continue 'dispatch;
	}
	// 83183B18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183B1C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83183B20: 4BFFF499  bl 0x83182fb8
	ctx.lr = 0x83183B24;
	sub_83182FB8(ctx, base);
	// 83183B24: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83183B28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183B2C: 4BFF8975  bl 0x8317c4a0
	ctx.lr = 0x83183B30;
	sub_8317C4A0(ctx, base);
	// 83183B30: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83183B34: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83183B38: 419A0034  beq cr6, 0x83183b6c
	if ctx.cr[6].eq {
	pc = 0x83183B6C; continue 'dispatch;
	}
	// 83183B3C: 388003E8  li r4, 0x3e8
	ctx.r[4].s64 = 1000;
	// 83183B40: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83183B44: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83183B48: 4800BEC1  bl 0x8318fa08
	ctx.lr = 0x83183B4C;
	sub_8318FA08(ctx, base);
	// 83183B4C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 83183B50: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83183B54: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 83183B58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83183B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83183B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83183B64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83183B68: 4E800020  blr
	return;
	// 83183B6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83183B70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83183B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83183B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83183B7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83183B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83183B88 size=72
    let mut pc: u32 = 0x83183B88;
    'dispatch: loop {
        match pc {
            0x83183B88 => {
    //   block [0x83183B88..0x83183BD0)
	// 83183B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83183B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83183B90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83183B94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83183B98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83183B9C: 4BFFF855  bl 0x831833f0
	ctx.lr = 0x83183BA0;
	sub_831833F0(ctx, base);
	// 83183BA0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83183BA4: 409A0018  bne cr6, 0x83183bbc
	if !ctx.cr[6].eq {
	pc = 0x83183BBC; continue 'dispatch;
	}
	// 83183BA8: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 83183BAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183BB0: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 83183BB4: 4BFFFA0D  bl 0x831835c0
	ctx.lr = 0x83183BB8;
	sub_831835C0(ctx, base);
	// 83183BB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83183BBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83183BC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83183BC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83183BC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83183BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83183BD0 size=136
    let mut pc: u32 = 0x83183BD0;
    'dispatch: loop {
        match pc {
            0x83183BD0 => {
    //   block [0x83183BD0..0x83183C58)
	// 83183BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83183BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83183BD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83183BDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83183BE0: 38A0002A  li r5, 0x2a
	ctx.r[5].s64 = 42;
	// 83183BE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83183BE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83183BEC: 4800F49D  bl 0x83193088
	ctx.lr = 0x83183BF0;
	sub_83193088(ctx, base);
	// 83183BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83183BF4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 83183BF8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83183BFC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83183C00: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83183C04: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83183C08: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83183C0C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83183C10: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83183C14: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83183C18: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83183C1C: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83183C20: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83183C24: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83183C28: 4BFFF661  bl 0x83183288
	ctx.lr = 0x83183C2C;
	sub_83183288(ctx, base);
	// 83183C2C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83183C30: 4BFFF659  bl 0x83183288
	ctx.lr = 0x83183C34;
	sub_83183288(ctx, base);
	// 83183C34: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 83183C38: 4BFFF651  bl 0x83183288
	ctx.lr = 0x83183C3C;
	sub_83183288(ctx, base);
	// 83183C3C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83183C40: 4BFFF649  bl 0x83183288
	ctx.lr = 0x83183C44;
	sub_83183288(ctx, base);
	// 83183C44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83183C48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83183C4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83183C50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83183C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83183C58 size=192
    let mut pc: u32 = 0x83183C58;
    'dispatch: loop {
        match pc {
            0x83183C58 => {
    //   block [0x83183C58..0x83183D18)
	// 83183C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83183C5C: 4802450D  bl 0x831a8168
	ctx.lr = 0x83183C60;
	sub_831A8130(ctx, base);
	// 83183C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83183C64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83183C68: 48003679  bl 0x831872e0
	ctx.lr = 0x83183C6C;
	sub_831872E0(ctx, base);
	// 83183C6C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83183C70: 419A001C  beq cr6, 0x83183c8c
	if ctx.cr[6].eq {
	pc = 0x83183C8C; continue 'dispatch;
	}
	// 83183C74: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83183C78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83183C7C: 60840132  ori r4, r4, 0x132
	ctx.r[4].u64 = ctx.r[4].u64 | 306;
	// 83183C80: 48003879  bl 0x831874f8
	ctx.lr = 0x83183C84;
	sub_831874F8(ctx, base);
	// 83183C84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83183C88: 48024530  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83183C8C: 3FA08345  lis r29, -0x7cbb
	ctx.r[29].s64 = -2092630016;
	// 83183C90: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83183C94: 3BCBD850  addi r30, r11, -0x27b0
	ctx.r[30].s64 = ctx.r[11].s64 + -10160;
	// 83183C98: 807DA32C  lwz r3, -0x5cd4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83183C9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83183CA0: 419A001C  beq cr6, 0x83183cbc
	if ctx.cr[6].eq {
	pc = 0x83183CBC; continue 'dispatch;
	}
	// 83183CA4: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 83183CA8: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83183CAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83183CB0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83183CB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83183CB8: 4E800421  bctrl
	ctx.lr = 0x83183CBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83183CBC: 3880002F  li r4, 0x2f
	ctx.r[4].s64 = 47;
	// 83183CC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183CC4: 4BFF574D  bl 0x83179410
	ctx.lr = 0x83183CC8;
	sub_83179410(ctx, base);
	// 83183CC8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83183CCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183CD0: 409A000C  bne cr6, 0x83183cdc
	if !ctx.cr[6].eq {
	pc = 0x83183CDC; continue 'dispatch;
	}
	// 83183CD4: 48001255  bl 0x83184f28
	ctx.lr = 0x83183CD8;
	sub_83184F28(ctx, base);
	// 83183CD8: 48000008  b 0x83183ce0
	pc = 0x83183CE0; continue 'dispatch;
	// 83183CDC: 4BFFF6C5  bl 0x831833a0
	ctx.lr = 0x83183CE0;
	sub_831833A0(ctx, base);
	// 83183CE0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83183CE4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83183CE8: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 83183CEC: 807DA32C  lwz r3, -0x5cd4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83183CF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83183CF4: 419A0018  beq cr6, 0x83183d0c
	if ctx.cr[6].eq {
	pc = 0x83183D0C; continue 'dispatch;
	}
	// 83183CF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83183CFC: 389E006C  addi r4, r30, 0x6c
	ctx.r[4].s64 = ctx.r[30].s64 + 108;
	// 83183D00: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83183D04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83183D08: 4E800421  bctrl
	ctx.lr = 0x83183D0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83183D0C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83183D10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83183D14: 480244A4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83183D18 size=120
    let mut pc: u32 = 0x83183D18;
    'dispatch: loop {
        match pc {
            0x83183D18 => {
    //   block [0x83183D18..0x83183D90)
	// 83183D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83183D1C: 48024451  bl 0x831a816c
	ctx.lr = 0x83183D20;
	sub_831A8130(ctx, base);
	// 83183D20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83183D24: 3FC08345  lis r30, -0x7cbb
	ctx.r[30].s64 = -2092630016;
	// 83183D28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83183D2C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83183D30: 3BABDAD8  addi r29, r11, -0x2528
	ctx.r[29].s64 = ctx.r[11].s64 + -9512;
	// 83183D34: 807EA32C  lwz r3, -0x5cd4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83183D38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83183D3C: 419A001C  beq cr6, 0x83183d58
	if ctx.cr[6].eq {
	pc = 0x83183D58; continue 'dispatch;
	}
	// 83183D40: 93FD000C  stw r31, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 83183D44: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 83183D48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83183D4C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83183D50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83183D54: 4E800421  bctrl
	ctx.lr = 0x83183D58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83183D58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183D5C: 4BFFF7DD  bl 0x83183538
	ctx.lr = 0x83183D60;
	sub_83183538(ctx, base);
	// 83183D60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83183D64: 807EA32C  lwz r3, -0x5cd4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83183D68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83183D6C: 419A0018  beq cr6, 0x83183d84
	if ctx.cr[6].eq {
	pc = 0x83183D84; continue 'dispatch;
	}
	// 83183D70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83183D74: 389D006C  addi r4, r29, 0x6c
	ctx.r[4].s64 = ctx.r[29].s64 + 108;
	// 83183D78: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83183D7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83183D80: 4E800421  bctrl
	ctx.lr = 0x83183D84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83183D84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183D88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83183D8C: 48024430  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83183D90 size=360
    let mut pc: u32 = 0x83183D90;
    'dispatch: loop {
        match pc {
            0x83183D90 => {
    //   block [0x83183D90..0x83183EF8)
	// 83183D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83183D94: 480243D1  bl 0x831a8164
	ctx.lr = 0x83183D98;
	sub_831A8130(ctx, base);
	// 83183D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83183D9C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83183DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83183DA4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83183DA8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83183DAC: 48003535  bl 0x831872e0
	ctx.lr = 0x83183DB0;
	sub_831872E0(ctx, base);
	// 83183DB0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83183DB4: 419A001C  beq cr6, 0x83183dd0
	if ctx.cr[6].eq {
	pc = 0x83183DD0; continue 'dispatch;
	}
	// 83183DB8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83183DBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83183DC0: 60840136  ori r4, r4, 0x136
	ctx.r[4].u64 = ctx.r[4].u64 | 310;
	// 83183DC4: 48003735  bl 0x831874f8
	ctx.lr = 0x83183DC8;
	sub_831874F8(ctx, base);
	// 83183DC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83183DCC: 480243E8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83183DD0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83183DD4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83183DD8: 4BFFF879  bl 0x83183650
	ctx.lr = 0x83183DDC;
	sub_83183650(ctx, base);
	// 83183DDC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83183DE0: 409A0110  bne cr6, 0x83183ef0
	if !ctx.cr[6].eq {
	pc = 0x83183EF0; continue 'dispatch;
	}
	// 83183DE4: 3F808345  lis r28, -0x7cbb
	ctx.r[28].s64 = -2092630016;
	// 83183DE8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83183DEC: 3BEBDBB0  addi r31, r11, -0x2450
	ctx.r[31].s64 = ctx.r[11].s64 + -9296;
	// 83183DF0: 807CA32C  lwz r3, -0x5cd4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83183DF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83183DF8: 419A001C  beq cr6, 0x83183e14
	if ctx.cr[6].eq {
	pc = 0x83183E14; continue 'dispatch;
	}
	// 83183DFC: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 83183E00: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 83183E04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83183E08: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83183E0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83183E10: 4E800421  bctrl
	ctx.lr = 0x83183E14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83183E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83183E18: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83183E1C: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 83183E20: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83183E24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83183E28: 48007FC1  bl 0x8318bde8
	ctx.lr = 0x83183E2C;
	sub_8318BDE8(ctx, base);
	// 83183E2C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83183E30: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83183E34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83183E38: 419A0060  beq cr6, 0x83183e98
	if ctx.cr[6].eq {
	pc = 0x83183E98; continue 'dispatch;
	}
	// 83183E3C: 817D0968  lwz r11, 0x968(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(2408 as u32) ) } as u64;
	// 83183E40: 815D096C  lwz r10, 0x96c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(2412 as u32) ) } as u64;
	// 83183E44: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83183E48: 409A0024  bne cr6, 0x83183e6c
	if !ctx.cr[6].eq {
	pc = 0x83183E6C; continue 'dispatch;
	}
	// 83183E4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83183E50: 409A0010  bne cr6, 0x83183e60
	if !ctx.cr[6].eq {
	pc = 0x83183E60; continue 'dispatch;
	}
	// 83183E54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83183E58: 4800F5F9  bl 0x83193450
	ctx.lr = 0x83183E5C;
	sub_83193450(ctx, base);
	// 83183E5C: F87D1F18  std r3, 0x1f18(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(7960 as u32), ctx.r[3].u64 ) };
	// 83183E60: 817D0968  lwz r11, 0x968(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(2408 as u32) ) } as u64;
	// 83183E64: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83183E68: 917D0968  stw r11, 0x968(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(2408 as u32), ctx.r[11].u32 ) };
	// 83183E6C: 807CA32C  lwz r3, -0x5cd4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83183E70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83183E74: 419A0054  beq cr6, 0x83183ec8
	if ctx.cr[6].eq {
	pc = 0x83183EC8; continue 'dispatch;
	}
	// 83183E78: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83183E7C: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83183E80: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83183E84: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 83183E88: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83183E8C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83183E90: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 83183E94: 4800001C  b 0x83183eb0
	pc = 0x83183EB0; continue 'dispatch;
	// 83183E98: 807CA32C  lwz r3, -0x5cd4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83183E9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83183EA0: 419A0028  beq cr6, 0x83183ec8
	if ctx.cr[6].eq {
	pc = 0x83183EC8; continue 'dispatch;
	}
	// 83183EA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83183EA8: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83183EAC: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83183EB0: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 83183EB4: 389F006C  addi r4, r31, 0x6c
	ctx.r[4].s64 = ctx.r[31].s64 + 108;
	// 83183EB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83183EBC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83183EC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83183EC4: 4E800421  bctrl
	ctx.lr = 0x83183EC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83183EC8: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83183ECC: 814BA348  lwz r10, -0x5cb8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23736 as u32) ) } as u64;
	// 83183ED0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83183ED4: 419A0018  beq cr6, 0x83183eec
	if ctx.cr[6].eq {
	pc = 0x83183EEC; continue 'dispatch;
	}
	// 83183ED8: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83183EDC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83183EE0: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83183EE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83183EE8: 4E800421  bctrl
	ctx.lr = 0x83183EEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83183EEC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83183EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83183EF4: 480242C0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83183EF8 size=224
    let mut pc: u32 = 0x83183EF8;
    'dispatch: loop {
        match pc {
            0x83183EF8 => {
    //   block [0x83183EF8..0x83183FD8)
	// 83183EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83183EFC: 4802426D  bl 0x831a8168
	ctx.lr = 0x83183F00;
	sub_831A8130(ctx, base);
	// 83183F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83183F04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83183F08: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83183F0C: 480033D5  bl 0x831872e0
	ctx.lr = 0x83183F10;
	sub_831872E0(ctx, base);
	// 83183F10: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83183F14: 419A001C  beq cr6, 0x83183f30
	if ctx.cr[6].eq {
	pc = 0x83183F30; continue 'dispatch;
	}
	// 83183F18: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83183F1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83183F20: 60840137  ori r4, r4, 0x137
	ctx.r[4].u64 = ctx.r[4].u64 | 311;
	// 83183F24: 480035D5  bl 0x831874f8
	ctx.lr = 0x83183F28;
	sub_831874F8(ctx, base);
	// 83183F28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83183F2C: 4802428C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83183F30: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83183F34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183F38: 4BFFF719  bl 0x83183650
	ctx.lr = 0x83183F3C;
	sub_83183650(ctx, base);
	// 83183F3C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83183F40: 409A0090  bne cr6, 0x83183fd0
	if !ctx.cr[6].eq {
	pc = 0x83183FD0; continue 'dispatch;
	}
	// 83183F44: 3FA08345  lis r29, -0x7cbb
	ctx.r[29].s64 = -2092630016;
	// 83183F48: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83183F4C: 3BCBDC88  addi r30, r11, -0x2378
	ctx.r[30].s64 = ctx.r[11].s64 + -9080;
	// 83183F50: 807DA32C  lwz r3, -0x5cd4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83183F54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83183F58: 419A0020  beq cr6, 0x83183f78
	if ctx.cr[6].eq {
	pc = 0x83183F78; continue 'dispatch;
	}
	// 83183F5C: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 83183F60: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83183F64: 939E0018  stw r28, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 83183F68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83183F6C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83183F70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83183F74: 4E800421  bctrl
	ctx.lr = 0x83183F78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83183F78: 817F096C  lwz r11, 0x96c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2412 as u32) ) } as u64;
	// 83183F7C: 815F0968  lwz r10, 0x968(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2408 as u32) ) } as u64;
	// 83183F80: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83183F84: 4098000C  bge cr6, 0x83183f90
	if !ctx.cr[6].lt {
	pc = 0x83183F90; continue 'dispatch;
	}
	// 83183F88: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83183F8C: 917F096C  stw r11, 0x96c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2412 as u32), ctx.r[11].u32 ) };
	// 83183F90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83183F94: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83183F98: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 83183F9C: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83183FA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183FA4: 48007E45  bl 0x8318bde8
	ctx.lr = 0x83183FA8;
	sub_8318BDE8(ctx, base);
	// 83183FA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83183FAC: 807DA32C  lwz r3, -0x5cd4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83183FB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83183FB4: 419A0018  beq cr6, 0x83183fcc
	if ctx.cr[6].eq {
	pc = 0x83183FCC; continue 'dispatch;
	}
	// 83183FB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83183FBC: 389E006C  addi r4, r30, 0x6c
	ctx.r[4].s64 = ctx.r[30].s64 + 108;
	// 83183FC0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83183FC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83183FC8: 4E800421  bctrl
	ctx.lr = 0x83183FCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83183FCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83183FD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83183FD4: 480241E4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83183FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83183FD8 size=172
    let mut pc: u32 = 0x83183FD8;
    'dispatch: loop {
        match pc {
            0x83183FD8 => {
    //   block [0x83183FD8..0x8318403C)
	// 83183FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83183FDC: 48024191  bl 0x831a816c
	ctx.lr = 0x83183FE0;
	sub_831A8130(ctx, base);
	// 83183FE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83183FE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83183FE8: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83183FEC: 83BF004C  lwz r29, 0x4c(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83183FF0: 4BFFE8F9  bl 0x831828e8
	ctx.lr = 0x83183FF4;
	sub_831828E8(ctx, base);
	// 83183FF4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83183FF8: 419A0080  beq cr6, 0x83184078
	if ctx.cr[6].eq {
	pc = 0x83184078; continue 'dispatch;
	}
	// 83183FFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184000: 4BFFF759  bl 0x83183758
	ctx.lr = 0x83184004;
	sub_83183758(ctx, base);
	// 83184004: 397DFFFE  addi r11, r29, -2
	ctx.r[11].s64 = ctx.r[29].s64 + -2;
	// 83184008: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 8318400C: 4199006C  bgt cr6, 0x83184078
	if ctx.cr[6].gt {
	pc = 0x83184078; continue 'dispatch;
	}
	// 83184010: 3D808318  lis r12, -0x7ce8
	ctx.r[12].s64 = -2095579136;
	// 83184014: 398C4028  addi r12, r12, 0x4028
	ctx.r[12].s64 = ctx.r[12].s64 + 16424;
	// 83184018: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8318401C: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83184020: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 83184024: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x8318403C; continue 'dispatch;
		},
		1 => {
	pc = 0x83184074; continue 'dispatch;
		},
		2 => {
	pc = 0x8318404C; continue 'dispatch;
		},
		3 => {
	pc = 0x83184078; continue 'dispatch;
		},
		4 => {
	pc = 0x8318404C; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 83184028: 8318403C  lwz r24, 0x403c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16444 as u32) ) } as u64;
	// 8318402C: 83184074  lwz r24, 0x4074(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16500 as u32) ) } as u64;
	// 83184030: 8318404C  lwz r24, 0x404c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16460 as u32) ) } as u64;
	// 83184034: 83184078  lwz r24, 0x4078(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16504 as u32) ) } as u64;
	// 83184038: 8318404C  lwz r24, 0x404c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16460 as u32) ) } as u64;
            }
            0x8318403C => {
    //   block [0x8318403C..0x8318404C)
	// 8318403C: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 83184040: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83184044: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83184048: 48024174  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            0x8318404C => {
    //   block [0x8318404C..0x83184074)
	// 8318404C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184050: 4BFFF799  bl 0x831837e8
	ctx.lr = 0x83184054;
	sub_831837E8(ctx, base);
	// 83184054: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83184058: 419A001C  beq cr6, 0x83184074
	if ctx.cr[6].eq {
	pc = 0x83184074; continue 'dispatch;
	}
	// 8318405C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184060: 4BFFF359  bl 0x831833b8
	ctx.lr = 0x83184064;
	sub_831833B8(ctx, base);
	// 83184064: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 83184068: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318406C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83184070: 4802414C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            0x83184074 => {
    //   block [0x83184074..0x83184078)
	// 83184074: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	pc = 0x83184078; continue 'dispatch;
            }
            0x83184078 => {
    //   block [0x83184078..0x83184084)
	// 83184078: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318407C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83184080: 4802413C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83184088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83184088 size=160
    let mut pc: u32 = 0x83184088;
    'dispatch: loop {
        match pc {
            0x83184088 => {
    //   block [0x83184088..0x831840E0)
	// 83184088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318408C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83184090: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83184094: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83184098: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318409C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831840A0: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 831840A4: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 831840A8: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 831840AC: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 831840B0: 4199005C  bgt cr6, 0x8318410c
	if ctx.cr[6].gt {
	pc = 0x8318410C; continue 'dispatch;
	}
	// 831840B4: 3D808318  lis r12, -0x7ce8
	ctx.r[12].s64 = -2095579136;
	// 831840B8: 398C40CC  addi r12, r12, 0x40cc
	ctx.r[12].s64 = ctx.r[12].s64 + 16588;
	// 831840BC: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 831840C0: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 831840C4: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831840C8: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x831840E0; continue 'dispatch;
		},
		1 => {
	pc = 0x831840E8; continue 'dispatch;
		},
		2 => {
	pc = 0x831840F0; continue 'dispatch;
		},
		3 => {
	pc = 0x8318410C; continue 'dispatch;
		},
		4 => {
	pc = 0x831840F0; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 831840CC: 831840E0  lwz r24, 0x40e0(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16608 as u32) ) } as u64;
	// 831840D0: 831840E8  lwz r24, 0x40e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16616 as u32) ) } as u64;
	// 831840D4: 831840F0  lwz r24, 0x40f0(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16624 as u32) ) } as u64;
	// 831840D8: 8318410C  lwz r24, 0x410c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16652 as u32) ) } as u64;
	// 831840DC: 831840F0  lwz r24, 0x40f0(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16624 as u32) ) } as u64;
            }
            0x831840E0 => {
    //   block [0x831840E0..0x831840E8)
	// 831840E0: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 831840E4: 48000028  b 0x8318410c
	pc = 0x8318410C; continue 'dispatch;
            }
            0x831840E8 => {
    //   block [0x831840E8..0x831840F0)
	// 831840E8: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 831840EC: 48000020  b 0x8318410c
	pc = 0x8318410C; continue 'dispatch;
            }
            0x831840F0 => {
    //   block [0x831840F0..0x8318410C)
	// 831840F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831840F4: 4BFFF6F5  bl 0x831837e8
	ctx.lr = 0x831840F8;
	sub_831837E8(ctx, base);
	// 831840F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831840FC: 419A0010  beq cr6, 0x8318410c
	if ctx.cr[6].eq {
	pc = 0x8318410C; continue 'dispatch;
	}
	// 83184100: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184104: 4BFFF2B5  bl 0x831833b8
	ctx.lr = 0x83184108;
	sub_831833B8(ctx, base);
	// 83184108: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	pc = 0x8318410C; continue 'dispatch;
            }
            0x8318410C => {
    //   block [0x8318410C..0x83184128)
	// 8318410C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83184110: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83184114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83184118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318411C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83184120: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83184124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83184128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83184128 size=140
    let mut pc: u32 = 0x83184128;
    'dispatch: loop {
        match pc {
            0x83184128 => {
    //   block [0x83184128..0x831841B4)
	// 83184128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318412C: 48024041  bl 0x831a816c
	ctx.lr = 0x83184130;
	sub_831A8130(ctx, base);
	// 83184130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83184134: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83184138: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318413C: 480032B5  bl 0x831873f0
	ctx.lr = 0x83184140;
	sub_831873F0(ctx, base);
	// 83184140: 3BFE0950  addi r31, r30, 0x950
	ctx.r[31].s64 = ctx.r[30].s64 + 2384;
	// 83184144: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83184148: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318414C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83184150: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83184154: 409A002C  bne cr6, 0x83184180
	if !ctx.cr[6].eq {
	pc = 0x83184180; continue 'dispatch;
	}
	// 83184158: 4BFFF709  bl 0x83183860
	ctx.lr = 0x8318415C;
	sub_83183860(ctx, base);
	// 8318415C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83184160: 419A0040  beq cr6, 0x831841a0
	if ctx.cr[6].eq {
	pc = 0x831841A0; continue 'dispatch;
	}
	// 83184164: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83184168: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8318416C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83184170: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83184174: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 83184178: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8318417C: 48000018  b 0x83184194
	pc = 0x83184194; continue 'dispatch;
	// 83184180: 4BFFF861  bl 0x831839e0
	ctx.lr = 0x83184184;
	sub_831839E0(ctx, base);
	// 83184184: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83184188: 419A0018  beq cr6, 0x831841a0
	if ctx.cr[6].eq {
	pc = 0x831841A0; continue 'dispatch;
	}
	// 8318418C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83184190: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 83184194: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83184198: 48000F41  bl 0x831850d8
	ctx.lr = 0x8318419C;
	sub_831850D8(ctx, base);
	// 8318419C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831841A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831841A4: 4800325D  bl 0x83187400
	ctx.lr = 0x831841A8;
	sub_83187400(ctx, base);
	// 831841A8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831841AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831841B0: 4802400C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831841B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831841B8 size=140
    let mut pc: u32 = 0x831841B8;
    'dispatch: loop {
        match pc {
            0x831841B8 => {
    //   block [0x831841B8..0x83184244)
	// 831841B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831841BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831841C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831841C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831841C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831841CC: 4BFFEB8D  bl 0x83182d58
	ctx.lr = 0x831841D0;
	sub_83182D58(ctx, base);
	// 831841D0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831841D4: 409A0054  bne cr6, 0x83184228
	if !ctx.cr[6].eq {
	pc = 0x83184228; continue 'dispatch;
	}
	// 831841D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831841DC: 4BFFEBF5  bl 0x83182dd0
	ctx.lr = 0x831841E0;
	sub_83182DD0(ctx, base);
	// 831841E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831841E4: 409A0044  bne cr6, 0x83184228
	if !ctx.cr[6].eq {
	pc = 0x83184228; continue 'dispatch;
	}
	// 831841E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831841EC: 4BFFED65  bl 0x83182f50
	ctx.lr = 0x831841F0;
	sub_83182F50(ctx, base);
	// 831841F0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831841F4: 409A0034  bne cr6, 0x83184228
	if !ctx.cr[6].eq {
	pc = 0x83184228; continue 'dispatch;
	}
	// 831841F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831841FC: 4BFFF8C5  bl 0x83183ac0
	ctx.lr = 0x83184200;
	sub_83183AC0(ctx, base);
	// 83184200: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83184204: 409A0024  bne cr6, 0x83184228
	if !ctx.cr[6].eq {
	pc = 0x83184228; continue 'dispatch;
	}
	// 83184208: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 8318420C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83184210: 409A0018  bne cr6, 0x83184228
	if !ctx.cr[6].eq {
	pc = 0x83184228; continue 'dispatch;
	}
	// 83184214: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83184218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318421C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83184220: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83184224: 4E800020  blr
	return;
	// 83184228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318422C: 4BFFF95D  bl 0x83183b88
	ctx.lr = 0x83184230;
	sub_83183B88(ctx, base);
	// 83184230: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83184234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83184238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318423C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83184240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83184248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83184248 size=424
    let mut pc: u32 = 0x83184248;
    'dispatch: loop {
        match pc {
            0x83184248 => {
    //   block [0x83184248..0x831843F0)
	// 83184248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318424C: 48023F19  bl 0x831a8164
	ctx.lr = 0x83184250;
	sub_831A8130(ctx, base);
	// 83184250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83184254: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83184258: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8318425C: 83FD003C  lwz r31, 0x3c(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 83184260: 817D0040  lwz r11, 0x40(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 83184264: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83184268: 5565F0BE  srwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8318426C: 419A0178  beq cr6, 0x831843e4
	if ctx.cr[6].eq {
	pc = 0x831843E4; continue 'dispatch;
	}
	// 83184270: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83184274: 40990170  ble cr6, 0x831843e4
	if !ctx.cr[6].gt {
	pc = 0x831843E4; continue 'dispatch;
	}
	// 83184278: 2B0B3FE0  cmplwi cr6, r11, 0x3fe0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16352 as u32, &mut ctx.xer);
	// 8318427C: 41990168  bgt cr6, 0x831843e4
	if ctx.cr[6].gt {
	pc = 0x831843E4; continue 'dispatch;
	}
	// 83184280: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 83184284: 812AA344  lwz r9, -0x5cbc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-23740 as u32) ) } as u64;
	// 83184288: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318428C: 419A000C  beq cr6, 0x83184298
	if ctx.cr[6].eq {
	pc = 0x83184298; continue 'dispatch;
	}
	// 83184290: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83184294: 409A0150  bne cr6, 0x831843e4
	if !ctx.cr[6].eq {
	pc = 0x831843E4; continue 'dispatch;
	}
	// 83184298: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318429C: 916AA344  stw r11, -0x5cbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-23740 as u32), ctx.r[11].u32 ) };
	// 831842A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831842A4: 4800EDE5  bl 0x83193088
	ctx.lr = 0x831842A8;
	sub_83193088(ctx, base);
	// 831842A8: 397F001F  addi r11, r31, 0x1f
	ctx.r[11].s64 = ctx.r[31].s64 + 31;
	// 831842AC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831842B0: 557F0034  rlwinm r31, r11, 0, 0, 0x1a
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831842B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831842B8: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 831842BC: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 831842C0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 831842C4: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 831842C8: 556B0030  rlwinm r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831842CC: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831842D0: 4BFFEE21  bl 0x831830f0
	ctx.lr = 0x831842D4;
	sub_831830F0(ctx, base);
	// 831842D4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831842D8: 409A010C  bne cr6, 0x831843e4
	if !ctx.cr[6].eq {
	pc = 0x831843E4; continue 'dispatch;
	}
	// 831842DC: 38A00044  li r5, 0x44
	ctx.r[5].s64 = 68;
	// 831842E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831842E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831842E8: 48024229  bl 0x831a8510
	ctx.lr = 0x831842EC;
	sub_831A8510(ctx, base);
	// 831842EC: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 831842F0: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 831842F4: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 831842F8: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 831842FC: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 83184300: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 83184304: 939F0044  stw r28, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[28].u32 ) };
	// 83184308: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8318430C: 93DF0064  stw r30, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 83184310: 4800ACF1  bl 0x8318f000
	ctx.lr = 0x83184314;
	sub_8318F000(ctx, base);
	// 83184314: 387F0910  addi r3, r31, 0x910
	ctx.r[3].s64 = ctx.r[31].s64 + 2320;
	// 83184318: 4BFFEF01  bl 0x83183218
	ctx.lr = 0x8318431C;
	sub_83183218(ctx, base);
	// 8318431C: 387F0950  addi r3, r31, 0x950
	ctx.r[3].s64 = ctx.r[31].s64 + 2384;
	// 83184320: 4BFFF8B1  bl 0x83183bd0
	ctx.lr = 0x83184324;
	sub_83183BD0(ctx, base);
	// 83184324: 387F1E58  addi r3, r31, 0x1e58
	ctx.r[3].s64 = ctx.r[31].s64 + 7768;
	// 83184328: 4BFFEF79  bl 0x831832a0
	ctx.lr = 0x8318432C;
	sub_831832A0(ctx, base);
	// 8318432C: 387F09F8  addi r3, r31, 0x9f8
	ctx.r[3].s64 = ctx.r[31].s64 + 2552;
	// 83184330: 4800A671  bl 0x8318e9a0
	ctx.lr = 0x83184334;
	sub_8318E9A0(ctx, base);
	// 83184334: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83184338: 38A00190  li r5, 0x190
	ctx.r[5].s64 = 400;
	// 8318433C: 3BCBA100  addi r30, r11, -0x5f00
	ctx.r[30].s64 = ctx.r[11].s64 + -24320;
	// 83184340: 387F0A0C  addi r3, r31, 0xa0c
	ctx.r[3].s64 = ctx.r[31].s64 + 2572;
	// 83184344: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83184348: 4800A651  bl 0x8318e998
	ctx.lr = 0x8318434C;
	sub_8318E998(ctx, base);
	// 8318434C: 38A00190  li r5, 0x190
	ctx.r[5].s64 = 400;
	// 83184350: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83184354: 387F0B9C  addi r3, r31, 0xb9c
	ctx.r[3].s64 = ctx.r[31].s64 + 2972;
	// 83184358: 4800A641  bl 0x8318e998
	ctx.lr = 0x8318435C;
	sub_8318E998(ctx, base);
	// 8318435C: 38A0005C  li r5, 0x5c
	ctx.r[5].s64 = 92;
	// 83184360: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83184364: 387F0D2C  addi r3, r31, 0xd2c
	ctx.r[3].s64 = ctx.r[31].s64 + 3372;
	// 83184368: 48023E79  bl 0x831a81e0
	ctx.lr = 0x8318436C;
	sub_831A81E0(ctx, base);
	// 8318436C: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 83184370: 389F0D88  addi r4, r31, 0xd88
	ctx.r[4].s64 = ctx.r[31].s64 + 3464;
	// 83184374: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 83184378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318437C: 917F0D7C  stw r11, 0xd7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3452 as u32), ctx.r[11].u32 ) };
	// 83184380: 4BFF85E1  bl 0x8317c960
	ctx.lr = 0x83184384;
	sub_8317C960(ctx, base);
	// 83184384: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83184388: 389F1398  addi r4, r31, 0x1398
	ctx.r[4].s64 = ctx.r[31].s64 + 5016;
	// 8318438C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184390: 4800A431  bl 0x8318e7c0
	ctx.lr = 0x83184394;
	sub_8318E7C0(ctx, base);
	// 83184394: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83184398: 409A004C  bne cr6, 0x831843e4
	if !ctx.cr[6].eq {
	pc = 0x831843E4; continue 'dispatch;
	}
	// 8318439C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 831843A0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831843A4: 389F1738  addi r4, r31, 0x1738
	ctx.r[4].s64 = ctx.r[31].s64 + 5944;
	// 831843A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831843AC: 48007E5D  bl 0x8318c208
	ctx.lr = 0x831843B0;
	sub_8318C208(ctx, base);
	// 831843B0: 387F1E38  addi r3, r31, 0x1e38
	ctx.r[3].s64 = ctx.r[31].s64 + 7736;
	// 831843B4: 48000A1D  bl 0x83184dd0
	ctx.lr = 0x831843B8;
	sub_83184DD0(ctx, base);
	// 831843B8: 387F1E28  addi r3, r31, 0x1e28
	ctx.r[3].s64 = ctx.r[31].s64 + 7720;
	// 831843BC: 4800F225  bl 0x831935e0
	ctx.lr = 0x831843C0;
	sub_831935E0(ctx, base);
	// 831843C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831843C4: 4BFFEFBD  bl 0x83183380
	ctx.lr = 0x831843C8;
	sub_83183380(ctx, base);
	// 831843C8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831843CC: 409A0018  bne cr6, 0x831843e4
	if !ctx.cr[6].eq {
	pc = 0x831843E4; continue 'dispatch;
	}
	// 831843D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831843D4: 939F004C  stw r28, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[28].u32 ) };
	// 831843D8: 939F0048  stw r28, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[28].u32 ) };
	// 831843DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831843E0: 48023DD4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 831843E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831843E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831843EC: 48023DC8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831843F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831843F0 size=1044
    let mut pc: u32 = 0x831843F0;
    'dispatch: loop {
        match pc {
            0x831843F0 => {
    //   block [0x831843F0..0x83184804)
	// 831843F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831843F4: 48023D3D  bl 0x831a8130
	ctx.lr = 0x831843F8;
	sub_831A8130(ctx, base);
	// 831843F8: 9421FC20  stwu r1, -0x3e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-992 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831843FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83184400: 39C00000  li r14, 0
	ctx.r[14].s64 = 0;
	// 83184404: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 83184408: 38A00044  li r5, 0x44
	ctx.r[5].s64 = 68;
	// 8318440C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83184410: 7DCF7378  mr r15, r14
	ctx.r[15].u64 = ctx.r[14].u64;
	// 83184414: 480240FD  bl 0x831a8510
	ctx.lr = 0x83184418;
	sub_831A8510(ctx, base);
	// 83184418: 827F0A2C  lwz r19, 0xa2c(r31)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2604 as u32) ) } as u64;
	// 8318441C: 2F130000  cmpwi cr6, r19, 0
	ctx.cr[6].compare_i32(ctx.r[19].s32, 0, &mut ctx.xer);
	// 83184420: 419A0014  beq cr6, 0x83184434
	if ctx.cr[6].eq {
	pc = 0x83184434; continue 'dispatch;
	}
	// 83184424: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 83184428: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318442C: 4BFFF025  bl 0x83183450
	ctx.lr = 0x83184430;
	sub_83183450(ctx, base);
	// 83184430: 81E100B4  lwz r15, 0xb4(r1)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 83184434: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 83184438: 4800ABE9  bl 0x8318f020
	ctx.lr = 0x8318443C;
	sub_8318F020(ctx, base);
	// 8318443C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184440: 48009D19  bl 0x8318e158
	ctx.lr = 0x83184444;
	sub_8318E158(ctx, base);
	// 83184444: 389F0D2C  addi r4, r31, 0xd2c
	ctx.r[4].s64 = ctx.r[31].s64 + 3372;
	// 83184448: 38610150  addi r3, r1, 0x150
	ctx.r[3].s64 = ctx.r[1].s64 + 336;
	// 8318444C: 837F0058  lwz r27, 0x58(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83184450: 38A0005C  li r5, 0x5c
	ctx.r[5].s64 = 92;
	// 83184454: 480240BD  bl 0x831a8510
	ctx.lr = 0x83184458;
	sub_831A8510(ctx, base);
	// 83184458: 393F1358  addi r9, r31, 0x1358
	ctx.r[9].s64 = ctx.r[31].s64 + 4952;
	// 8318445C: 395F1364  addi r10, r31, 0x1364
	ctx.r[10].s64 = ctx.r[31].s64 + 4964;
	// 83184460: 839F1E28  lwz r28, 0x1e28(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7720 as u32) ) } as u64;
	// 83184464: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 83184468: 835F09F8  lwz r26, 0x9f8(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2552 as u32) ) } as u64;
	// 8318446C: 833F09FC  lwz r25, 0x9fc(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2556 as u32) ) } as u64;
	// 83184470: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83184474: 831F1064  lwz r24, 0x1064(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4196 as u32) ) } as u64;
	// 83184478: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318447C: 80EA0008  lwz r7, 8(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 83184480: 82FF1068  lwz r23, 0x1068(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4200 as u32) ) } as u64;
	// 83184484: 82DF106C  lwz r22, 0x106c(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4204 as u32) ) } as u64;
	// 83184488: 82BF107C  lwz r21, 0x107c(r31)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4220 as u32) ) } as u64;
	// 8318448C: 91010060  stw r8, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 83184490: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83184494: 81290008  lwz r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 83184498: 829F1080  lwz r20, 0x1080(r31)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4224 as u32) ) } as u64;
	// 8318449C: 825F0DA0  lwz r18, 0xda0(r31)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3488 as u32) ) } as u64;
	// 831844A0: 823F1038  lwz r17, 0x1038(r31)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4152 as u32) ) } as u64;
	// 831844A4: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 831844A8: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 831844AC: 813F1374  lwz r9, 0x1374(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4980 as u32) ) } as u64;
	// 831844B0: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 831844B4: 821F103C  lwz r16, 0x103c(r31)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4156 as u32) ) } as u64;
	// 831844B8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 831844BC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831844C0: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 831844C4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831844C8: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 831844CC: 90EB0008  stw r7, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 831844D0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 831844D4: 419A002C  beq cr6, 0x83184500
	if ctx.cr[6].eq {
	pc = 0x83184500; continue 'dispatch;
	}
	// 831844D8: 817C0DD0  lwz r11, 0xdd0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(3536 as u32) ) } as u64;
	// 831844DC: 81DC0DD4  lwz r14, 0xdd4(r28)
	ctx.r[14].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(3540 as u32) ) } as u64;
	// 831844E0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 831844E4: 817C0DC4  lwz r11, 0xdc4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(3524 as u32) ) } as u64;
	// 831844E8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 831844EC: 817C0DC8  lwz r11, 0xdc8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(3528 as u32) ) } as u64;
	// 831844F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 831844F4: 817C0DCC  lwz r11, 0xdcc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(3532 as u32) ) } as u64;
	// 831844F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831844FC: 48000014  b 0x83184510
	pc = 0x83184510; continue 'dispatch;
	// 83184500: 91C10054  stw r14, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[14].u32 ) };
	// 83184504: 91C1005C  stw r14, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[14].u32 ) };
	// 83184508: 91C10050  stw r14, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[14].u32 ) };
	// 8318450C: 91C10058  stw r14, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[14].u32 ) };
	// 83184510: 395F1FA0  addi r10, r31, 0x1fa0
	ctx.r[10].s64 = ctx.r[31].s64 + 8096;
	// 83184514: 80DF1448  lwz r6, 0x1448(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5192 as u32) ) } as u64;
	// 83184518: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 8318451C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 83184520: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 83184524: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184528: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318452C: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83184530: 80EA0008  lwz r7, 8(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 83184534: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 83184538: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8318453C: 54C92036  slwi r9, r6, 4
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83184540: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 83184544: 90EB0008  stw r7, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 83184548: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8318454C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 83184550: 813F1444  lwz r9, 0x1444(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5188 as u32) ) } as u64;
	// 83184554: 91210070  stw r9, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 83184558: 4BFF9E41  bl 0x8317e398
	ctx.lr = 0x8318455C;
	sub_8317E398(ctx, base);
	// 8318455C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83184560: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184564: 4BFFEE25  bl 0x83183388
	ctx.lr = 0x83184568;
	sub_83183388(ctx, base);
	// 83184568: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318456C: 409A0290  bne cr6, 0x831847fc
	if !ctx.cr[6].eq {
	pc = 0x831847FC; continue 'dispatch;
	}
	// 83184570: 38A00190  li r5, 0x190
	ctx.r[5].s64 = 400;
	// 83184574: 389F0B9C  addi r4, r31, 0xb9c
	ctx.r[4].s64 = ctx.r[31].s64 + 2972;
	// 83184578: 386101B0  addi r3, r1, 0x1b0
	ctx.r[3].s64 = ctx.r[1].s64 + 432;
	// 8318457C: 4800A41D  bl 0x8318e998
	ctx.lr = 0x83184580;
	sub_8318E998(ctx, base);
	// 83184580: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83184584: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 83184588: 3BCBC648  addi r30, r11, -0x39b8
	ctx.r[30].s64 = ctx.r[11].s64 + -14776;
	// 8318458C: 397F1FB0  addi r11, r31, 0x1fb0
	ctx.r[11].s64 = ctx.r[31].s64 + 8112;
	// 83184590: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 83184594: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83184598: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318459C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831845A0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831845A4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 831845A8: 4200FFF0  bdnz 0x83184598
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83184598; continue 'dispatch;
	}
	// 831845AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831845B0: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 831845B4: 4BFFFC95  bl 0x83184248
	ctx.lr = 0x831845B8;
	sub_83184248(ctx, base);
	// 831845B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831845BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831845C0: 409A0018  bne cr6, 0x831845d8
	if !ctx.cr[6].eq {
	pc = 0x831845D8; continue 'dispatch;
	}
	// 831845C4: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 831845C8: 60840202  ori r4, r4, 0x202
	ctx.r[4].u64 = ctx.r[4].u64 | 514;
	// 831845CC: 48002F2D  bl 0x831874f8
	ctx.lr = 0x831845D0;
	sub_831874F8(ctx, base);
	// 831845D0: 382103E0  addi r1, r1, 0x3e0
	ctx.r[1].s64 = ctx.r[1].s64 + 992;
	// 831845D4: 48023BAC  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
	// 831845D8: 395F1FB0  addi r10, r31, 0x1fb0
	ctx.r[10].s64 = ctx.r[31].s64 + 8112;
	// 831845DC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 831845E0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 831845E4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831845E8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831845EC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831845F0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831845F4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 831845F8: 4200FFF0  bdnz 0x831845e8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831845E8; continue 'dispatch;
	}
	// 831845FC: 38A00190  li r5, 0x190
	ctx.r[5].s64 = 400;
	// 83184600: 388101B0  addi r4, r1, 0x1b0
	ctx.r[4].s64 = ctx.r[1].s64 + 432;
	// 83184604: 387F0A0C  addi r3, r31, 0xa0c
	ctx.r[3].s64 = ctx.r[31].s64 + 2572;
	// 83184608: 4800A391  bl 0x8318e998
	ctx.lr = 0x8318460C;
	sub_8318E998(ctx, base);
	// 8318460C: 38A00190  li r5, 0x190
	ctx.r[5].s64 = 400;
	// 83184610: 388101B0  addi r4, r1, 0x1b0
	ctx.r[4].s64 = ctx.r[1].s64 + 432;
	// 83184614: 387F0B9C  addi r3, r31, 0xb9c
	ctx.r[3].s64 = ctx.r[31].s64 + 2972;
	// 83184618: 4800A381  bl 0x8318e998
	ctx.lr = 0x8318461C;
	sub_8318E998(ctx, base);
	// 8318461C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83184620: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 83184624: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184628: 4BFF9DE9  bl 0x8317e410
	ctx.lr = 0x8318462C;
	sub_8317E410(ctx, base);
	// 8318462C: 2F130000  cmpwi cr6, r19, 0
	ctx.cr[6].compare_i32(ctx.r[19].s32, 0, &mut ctx.xer);
	// 83184630: 419A0038  beq cr6, 0x83184668
	if ctx.cr[6].eq {
	pc = 0x83184668; continue 'dispatch;
	}
	// 83184634: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 83184638: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318463C: 4BFFEE15  bl 0x83183450
	ctx.lr = 0x83184640;
	sub_83183450(ctx, base);
	// 83184640: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83184644: 409A01B8  bne cr6, 0x831847fc
	if !ctx.cr[6].eq {
	pc = 0x831847FC; continue 'dispatch;
	}
	// 83184648: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 8318464C: 80A100B4  lwz r5, 0xb4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 83184650: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184654: 4BFFEE6D  bl 0x831834c0
	ctx.lr = 0x83184658;
	sub_831834C0(ctx, base);
	// 83184658: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318465C: 409A01A0  bne cr6, 0x831847fc
	if !ctx.cr[6].eq {
	pc = 0x831847FC; continue 'dispatch;
	}
	// 83184660: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184664: 4BFFF6B5  bl 0x83183d18
	ctx.lr = 0x83184668;
	sub_83183D18(ctx, base);
	// 83184668: 387F0D2C  addi r3, r31, 0xd2c
	ctx.r[3].s64 = ctx.r[31].s64 + 3372;
	// 8318466C: 937F0058  stw r27, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 83184670: 38810150  addi r4, r1, 0x150
	ctx.r[4].s64 = ctx.r[1].s64 + 336;
	// 83184674: 38A0005C  li r5, 0x5c
	ctx.r[5].s64 = 92;
	// 83184678: 48023E99  bl 0x831a8510
	ctx.lr = 0x8318467C;
	sub_831A8510(ctx, base);
	// 8318467C: 80BF0D74  lwz r5, 0xd74(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 83184680: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83184684: 419A0014  beq cr6, 0x83184698
	if ctx.cr[6].eq {
	pc = 0x83184698; continue 'dispatch;
	}
	// 83184688: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318468C: 80DF0D78  lwz r6, 0xd78(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3448 as u32) ) } as u64;
	// 83184690: 809F0D7C  lwz r4, 0xd7c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3452 as u32) ) } as u64;
	// 83184694: 4BFF9DD5  bl 0x8317e468
	ctx.lr = 0x83184698;
	sub_8317E468(ctx, base);
	// 83184698: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8318469C: 419A0014  beq cr6, 0x831846b0
	if ctx.cr[6].eq {
	pc = 0x831846B0; continue 'dispatch;
	}
	// 831846A0: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 831846A4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 831846A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831846AC: 48002F15  bl 0x831875c0
	ctx.lr = 0x831846B0;
	sub_831875C0(ctx, base);
	// 831846B0: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 831846B4: 419A0014  beq cr6, 0x831846c8
	if ctx.cr[6].eq {
	pc = 0x831846C8; continue 'dispatch;
	}
	// 831846B8: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 831846BC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 831846C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831846C4: 4BFF7225  bl 0x8317b8e8
	ctx.lr = 0x831846C8;
	sub_8317B8E8(ctx, base);
	// 831846C8: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 831846CC: 419A0018  beq cr6, 0x831846e4
	if ctx.cr[6].eq {
	pc = 0x831846E4; continue 'dispatch;
	}
	// 831846D0: 7E86A378  mr r6, r20
	ctx.r[6].u64 = ctx.r[20].u64;
	// 831846D4: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 831846D8: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 831846DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831846E0: 4BFF7259  bl 0x8317b938
	ctx.lr = 0x831846E4;
	sub_8317B938(ctx, base);
	// 831846E4: 2B120000  cmplwi cr6, r18, 0
	ctx.cr[6].compare_u32(ctx.r[18].u32, 0 as u32, &mut ctx.xer);
	// 831846E8: 419A0010  beq cr6, 0x831846f8
	if ctx.cr[6].eq {
	pc = 0x831846F8; continue 'dispatch;
	}
	// 831846EC: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 831846F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831846F4: 4BFF719D  bl 0x8317b890
	ctx.lr = 0x831846F8;
	sub_8317B890(ctx, base);
	// 831846F8: 7F118000  cmpw cr6, r17, r16
	ctx.cr[6].compare_i32(ctx.r[17].s32, ctx.r[16].s32, &mut ctx.xer);
	// 831846FC: 419A0014  beq cr6, 0x83184710
	if ctx.cr[6].eq {
	pc = 0x83184710; continue 'dispatch;
	}
	// 83184700: 7E058378  mr r5, r16
	ctx.r[5].u64 = ctx.r[16].u64;
	// 83184704: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 83184708: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318470C: 48000835  bl 0x83184f40
	ctx.lr = 0x83184710;
	sub_83184F40(ctx, base);
	// 83184710: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83184714: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83184718: 419A003C  beq cr6, 0x83184754
	if ctx.cr[6].eq {
	pc = 0x83184754; continue 'dispatch;
	}
	// 8318471C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83184720: 391F1358  addi r8, r31, 0x1358
	ctx.r[8].s64 = ctx.r[31].s64 + 4952;
	// 83184724: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83184728: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8318472C: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83184730: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83184734: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83184738: 814BA34C  lwz r10, -0x5cb4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23732 as u32) ) } as u64;
	// 8318473C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83184740: 419A0014  beq cr6, 0x83184754
	if ctx.cr[6].eq {
	pc = 0x83184754; continue 'dispatch;
	}
	// 83184744: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83184748: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318474C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83184750: 4E800421  bctrl
	ctx.lr = 0x83184754;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83184754: 80A10094  lwz r5, 0x94(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 83184758: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8318475C: 419A0010  beq cr6, 0x8318476c
	if ctx.cr[6].eq {
	pc = 0x8318476C; continue 'dispatch;
	}
	// 83184760: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184764: 80810090  lwz r4, 0x90(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 83184768: 4BFF7A21  bl 0x8317c188
	ctx.lr = 0x8318476C;
	sub_8317C188(ctx, base);
	// 8318476C: 80810074  lwz r4, 0x74(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83184770: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 83184774: 419A000C  beq cr6, 0x83184780
	if ctx.cr[6].eq {
	pc = 0x83184780; continue 'dispatch;
	}
	// 83184778: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318477C: 4BFF7D1D  bl 0x8317c498
	ctx.lr = 0x83184780;
	sub_8317C498(ctx, base);
	// 83184780: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83184784: 419A0044  beq cr6, 0x831847c8
	if ctx.cr[6].eq {
	pc = 0x831847C8; continue 'dispatch;
	}
	// 83184788: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8318478C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184790: 4800EE71  bl 0x83193600
	ctx.lr = 0x83184794;
	sub_83193600(ctx, base);
	// 83184794: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184798: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8318479C: 4800F335  bl 0x83193ad0
	ctx.lr = 0x831847A0;
	sub_83193AD0(ctx, base);
	// 831847A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831847A4: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831847A8: 4800F251  bl 0x831939f8
	ctx.lr = 0x831847AC;
	sub_831939F8(ctx, base);
	// 831847AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831847B0: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831847B4: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 831847B8: 4800F2B1  bl 0x83193a68
	ctx.lr = 0x831847BC;
	sub_83193A68(ctx, base);
	// 831847BC: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 831847C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831847C4: 4800EFC5  bl 0x83193788
	ctx.lr = 0x831847C8;
	sub_83193788(ctx, base);
	// 831847C8: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 831847CC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831847D0: 419A0010  beq cr6, 0x831847e0
	if ctx.cr[6].eq {
	pc = 0x831847E0; continue 'dispatch;
	}
	// 831847D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831847D8: 80A1006C  lwz r5, 0x6c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 831847DC: 4800A3CD  bl 0x8318eba8
	ctx.lr = 0x831847E0;
	sub_8318EBA8(ctx, base);
	// 831847E0: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 831847E4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831847E8: 419A0010  beq cr6, 0x831847f8
	if ctx.cr[6].eq {
	pc = 0x831847F8; continue 'dispatch;
	}
	// 831847EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831847F0: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 831847F4: 4BFF505D  bl 0x83179850
	ctx.lr = 0x831847F8;
	sub_83179850(ctx, base);
	// 831847F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831847FC: 382103E0  addi r1, r1, 0x3e0
	ctx.r[1].s64 = ctx.r[1].s64 + 992;
	// 83184800: 48023980  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83184808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83184808 size=112
    let mut pc: u32 = 0x83184808;
    'dispatch: loop {
        match pc {
            0x83184808 => {
    //   block [0x83184808..0x83184878)
	// 83184808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318480C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83184810: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83184814: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83184818: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318481C: 4BFFF99D  bl 0x831841b8
	ctx.lr = 0x83184820;
	sub_831841B8(ctx, base);
	// 83184820: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83184824: 419A001C  beq cr6, 0x83184840
	if ctx.cr[6].eq {
	pc = 0x83184840; continue 'dispatch;
	}
	// 83184828: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8318482C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83184830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83184834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83184838: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318483C: 4E800020  blr
	return;
	// 83184840: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184844: 4BFFF8E5  bl 0x83184128
	ctx.lr = 0x83184848;
	sub_83184128(ctx, base);
	// 83184848: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318484C: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83184850: 409A0014  bne cr6, 0x83184864
	if !ctx.cr[6].eq {
	pc = 0x83184864; continue 'dispatch;
	}
	// 83184854: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83184858: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 8318485C: 409A0008  bne cr6, 0x83184864
	if !ctx.cr[6].eq {
	pc = 0x83184864; continue 'dispatch;
	}
	// 83184860: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 83184864: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83184868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318486C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83184870: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83184874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83184878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83184878 size=116
    let mut pc: u32 = 0x83184878;
    'dispatch: loop {
        match pc {
            0x83184878 => {
    //   block [0x83184878..0x831848EC)
	// 83184878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318487C: 480238F1  bl 0x831a816c
	ctx.lr = 0x83184880;
	sub_831A8130(ctx, base);
	// 83184880: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83184884: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83184888: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8318488C: 4BFFE7ED  bl 0x83183078
	ctx.lr = 0x83184890;
	sub_83183078(ctx, base);
	// 83184890: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83184894: 409A0024  bne cr6, 0x831848b8
	if !ctx.cr[6].eq {
	pc = 0x831848B8; continue 'dispatch;
	}
	// 83184898: 4BFFE821  bl 0x831830b8
	ctx.lr = 0x8318489C;
	sub_831830B8(ctx, base);
	// 8318489C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831848A0: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 831848A4: 409A0020  bne cr6, 0x831848c4
	if !ctx.cr[6].eq {
	pc = 0x831848C4; continue 'dispatch;
	}
	// 831848A8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 831848AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831848B0: 60840206  ori r4, r4, 0x206
	ctx.r[4].u64 = ctx.r[4].u64 | 518;
	// 831848B4: 48002C45  bl 0x831874f8
	ctx.lr = 0x831848B8;
	sub_831874F8(ctx, base);
	// 831848B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831848BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831848C0: 480238FC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831848C4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831848C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831848CC: 4BFFF97D  bl 0x83184248
	ctx.lr = 0x831848D0;
	sub_83184248(ctx, base);
	// 831848D0: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 831848D4: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831848D8: 396BA100  addi r11, r11, -0x5f00
	ctx.r[11].s64 = ctx.r[11].s64 + -24320;
	// 831848DC: 396B020C  addi r11, r11, 0x20c
	ctx.r[11].s64 = ctx.r[11].s64 + 524;
	// 831848E0: 7C6A592E  stwx r3, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 831848E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831848E8: 480238D4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831848F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831848F0 size=136
    let mut pc: u32 = 0x831848F0;
    'dispatch: loop {
        match pc {
            0x831848F0 => {
    //   block [0x831848F0..0x83184978)
	// 831848F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831848F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831848F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831848FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83184900: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83184904: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83184908: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8318490C: 409A001C  bne cr6, 0x83184928
	if !ctx.cr[6].eq {
	pc = 0x83184928; continue 'dispatch;
	}
	// 83184910: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83184914: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83184918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318491C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83184920: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83184924: 4E800020  blr
	return;
	// 83184928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318492C: 4BFFEAC5  bl 0x831833f0
	ctx.lr = 0x83184930;
	sub_831833F0(ctx, base);
	// 83184930: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83184934: 409A0030  bne cr6, 0x83184964
	if !ctx.cr[6].eq {
	pc = 0x83184964; continue 'dispatch;
	}
	// 83184938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318493C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83184940: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 83184944: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83184948: 4BFFEA89  bl 0x831833d0
	ctx.lr = 0x8318494C;
	sub_831833D0(ctx, base);
	// 8318494C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184950: 4BFFFAA1  bl 0x831843f0
	ctx.lr = 0x83184954;
	sub_831843F0(ctx, base);
	// 83184954: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83184958: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318495C: 4BFFEA75  bl 0x831833d0
	ctx.lr = 0x83184960;
	sub_831833D0(ctx, base);
	// 83184960: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 83184964: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83184968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318496C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83184970: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83184974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83184978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83184978 size=288
    let mut pc: u32 = 0x83184978;
    'dispatch: loop {
        match pc {
            0x83184978 => {
    //   block [0x83184978..0x83184A38)
	// 83184978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318497C: 480237F1  bl 0x831a816c
	ctx.lr = 0x83184980;
	sub_831A8130(ctx, base);
	// 83184980: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83184984: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83184988: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8318498C: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 83184990: 419A001C  beq cr6, 0x831849ac
	if ctx.cr[6].eq {
	pc = 0x831849AC; continue 'dispatch;
	}
	// 83184994: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 83184998: 419A0014  beq cr6, 0x831849ac
	if ctx.cr[6].eq {
	pc = 0x831849AC; continue 'dispatch;
	}
	// 8318499C: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 831849A0: 419A000C  beq cr6, 0x831849ac
	if ctx.cr[6].eq {
	pc = 0x831849AC; continue 'dispatch;
	}
	// 831849A4: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 831849A8: 409A00E8  bne cr6, 0x83184a90
	if !ctx.cr[6].eq {
	pc = 0x83184A90; continue 'dispatch;
	}
	// 831849AC: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 831849B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831849B4: 419A00DC  beq cr6, 0x83184a90
	if ctx.cr[6].eq {
	pc = 0x83184A90; continue 'dispatch;
	}
	// 831849B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831849BC: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 831849C0: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 831849C4: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 831849C8: 419A00C8  beq cr6, 0x83184a90
	if ctx.cr[6].eq {
	pc = 0x83184A90; continue 'dispatch;
	}
	// 831849CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831849D0: 4800EA81  bl 0x83193450
	ctx.lr = 0x831849D4;
	sub_83193450(ctx, base);
	// 831849D4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831849D8: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 831849DC: 419A0014  beq cr6, 0x831849f0
	if ctx.cr[6].eq {
	pc = 0x831849F0; continue 'dispatch;
	}
	// 831849E0: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 831849E4: 419A000C  beq cr6, 0x831849f0
	if ctx.cr[6].eq {
	pc = 0x831849F0; continue 'dispatch;
	}
	// 831849E8: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 831849EC: 409A000C  bne cr6, 0x831849f8
	if !ctx.cr[6].eq {
	pc = 0x831849F8; continue 'dispatch;
	}
	// 831849F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831849F4: 4BFFED2D  bl 0x83183720
	ctx.lr = 0x831849F8;
	sub_83183720(ctx, base);
	// 831849F8: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 831849FC: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 83184A00: 2B0A0005  cmplwi cr6, r10, 5
	ctx.cr[6].compare_u32(ctx.r[10].u32, 5 as u32, &mut ctx.xer);
	// 83184A04: 41990070  bgt cr6, 0x83184a74
	if ctx.cr[6].gt {
	pc = 0x83184A74; continue 'dispatch;
	}
	// 83184A08: 3D808318  lis r12, -0x7ce8
	ctx.r[12].s64 = -2095579136;
	// 83184A0C: 398C4A20  addi r12, r12, 0x4a20
	ctx.r[12].s64 = ctx.r[12].s64 + 18976;
	// 83184A10: 5540103A  slwi r0, r10, 2
	ctx.r[0].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 83184A14: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83184A18: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 83184A1C: 4E800420  bctr
	match ctx.r[10].u64 {
		0 => {
	pc = 0x83184A38; continue 'dispatch;
		},
		1 => {
	pc = 0x83184A44; continue 'dispatch;
		},
		2 => {
	pc = 0x83184A50; continue 'dispatch;
		},
		3 => {
	pc = 0x83184A5C; continue 'dispatch;
		},
		4 => {
	pc = 0x83184A74; continue 'dispatch;
		},
		5 => {
	pc = 0x83184A68; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 83184A20: 83184A38  lwz r24, 0x4a38(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(19000 as u32) ) } as u64;
	// 83184A24: 83184A44  lwz r24, 0x4a44(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(19012 as u32) ) } as u64;
	// 83184A28: 83184A50  lwz r24, 0x4a50(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(19024 as u32) ) } as u64;
	// 83184A2C: 83184A5C  lwz r24, 0x4a5c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(19036 as u32) ) } as u64;
	// 83184A30: 83184A74  lwz r24, 0x4a74(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(19060 as u32) ) } as u64;
	// 83184A34: 83184A68  lwz r24, 0x4a68(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(19048 as u32) ) } as u64;
            }
            0x83184A38 => {
    //   block [0x83184A38..0x83184A44)
	// 83184A38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184A3C: 4BFFDE85  bl 0x831828c0
	ctx.lr = 0x83184A40;
	sub_831828C0(ctx, base);
	// 83184A40: 48000030  b 0x83184a70
	pc = 0x83184A70; continue 'dispatch;
            }
            0x83184A44 => {
    //   block [0x83184A44..0x83184A50)
	// 83184A44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184A48: 4BFFF591  bl 0x83183fd8
	ctx.lr = 0x83184A4C;
	sub_83183FD8(ctx, base);
	// 83184A4C: 48000024  b 0x83184a70
	pc = 0x83184A70; continue 'dispatch;
            }
            0x83184A50 => {
    //   block [0x83184A50..0x83184A5C)
	// 83184A50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184A54: 4BFFF635  bl 0x83184088
	ctx.lr = 0x83184A58;
	sub_83184088(ctx, base);
	// 83184A58: 48000018  b 0x83184a70
	pc = 0x83184A70; continue 'dispatch;
            }
            0x83184A5C => {
    //   block [0x83184A5C..0x83184A68)
	// 83184A5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184A60: 4BFFFDA9  bl 0x83184808
	ctx.lr = 0x83184A64;
	sub_83184808(ctx, base);
	// 83184A64: 4800000C  b 0x83184a70
	pc = 0x83184A70; continue 'dispatch;
            }
            0x83184A68 => {
    //   block [0x83184A68..0x83184A74)
	// 83184A68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184A6C: 4BFED3AD  bl 0x83171e18
	ctx.lr = 0x83184A70;
	sub_83171E18(ctx, base);
	// 83184A70: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	pc = 0x83184A74; continue 'dispatch;
            }
            0x83184A74 => {
    //   block [0x83184A74..0x83184A98)
	// 83184A74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184A78: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83184A7C: 4800E9D5  bl 0x83193450
	ctx.lr = 0x83184A80;
	sub_83193450(ctx, base);
	// 83184A80: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83184A84: 387F1EF8  addi r3, r31, 0x1ef8
	ctx.r[3].s64 = ctx.r[31].s64 + 7928;
	// 83184A88: 7C9D5850  subf r4, r29, r11
	ctx.r[4].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 83184A8C: 4800EB0D  bl 0x83193598
	ctx.lr = 0x83184A90;
	sub_83193598(ctx, base);
	// 83184A90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83184A94: 48023728  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83184A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83184A98 size=268
    let mut pc: u32 = 0x83184A98;
    'dispatch: loop {
        match pc {
            0x83184A98 => {
    //   block [0x83184A98..0x83184BA4)
	// 83184A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83184A9C: 480236A9  bl 0x831a8144
	ctx.lr = 0x83184AA0;
	sub_831A8130(ctx, base);
	// 83184AA0: 9421FCF0  stwu r1, -0x310(r1)
	ea = ctx.r[1].u32.wrapping_add(-784 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83184AA4: 3FC08345  lis r30, -0x7cbb
	ctx.r[30].s64 = -2092630016;
	// 83184AA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83184AAC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83184AB0: 817EA32C  lwz r11, -0x5cd4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83184AB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83184AB8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83184ABC: 3BABD6A0  addi r29, r11, -0x2960
	ctx.r[29].s64 = ctx.r[11].s64 + -10592;
	// 83184AC0: 419A00A4  beq cr6, 0x83184b64
	if ctx.cr[6].eq {
	pc = 0x83184B64; continue 'dispatch;
	}
	// 83184AC4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83184AC8: 837F0040  lwz r27, 0x40(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83184ACC: 835F003C  lwz r26, 0x3c(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83184AD0: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 83184AD4: 388BB0B8  addi r4, r11, -0x4f48
	ctx.r[4].s64 = ctx.r[11].s64 + -20296;
	// 83184AD8: 833F0034  lwz r25, 0x34(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83184ADC: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83184AE0: 831F0030  lwz r24, 0x30(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83184AE4: 82FF002C  lwz r23, 0x2c(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83184AE8: 82DF0028  lwz r22, 0x28(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83184AEC: 82BF0024  lwz r21, 0x24(r31)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83184AF0: 829F0020  lwz r20, 0x20(r31)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83184AF4: 827F001C  lwz r19, 0x1c(r31)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83184AF8: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83184AFC: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83184B00: 811F0010  lwz r8, 0x10(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83184B04: 80FF000C  lwz r7, 0xc(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83184B08: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83184B0C: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83184B10: 9361009C  stw r27, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[27].u32 ) };
	// 83184B14: 93410094  stw r26, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[26].u32 ) };
	// 83184B18: 9161008C  stw r11, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 83184B1C: 93210084  stw r25, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[25].u32 ) };
	// 83184B20: 9301007C  stw r24, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[24].u32 ) };
	// 83184B24: 92E10074  stw r23, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[23].u32 ) };
	// 83184B28: 92C1006C  stw r22, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[22].u32 ) };
	// 83184B2C: 92A10064  stw r21, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[21].u32 ) };
	// 83184B30: 9281005C  stw r20, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[20].u32 ) };
	// 83184B34: 92610054  stw r19, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[19].u32 ) };
	// 83184B38: 48023FA1  bl 0x831a8ad8
	ctx.lr = 0x83184B3C;
	sub_831A8AD8(ctx, base);
	// 83184B3C: 807EA32C  lwz r3, -0x5cd4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83184B40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83184B44: 419A0020  beq cr6, 0x83184b64
	if ctx.cr[6].eq {
	pc = 0x83184B64; continue 'dispatch;
	}
	// 83184B48: 396100A0  addi r11, r1, 0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + 160;
	// 83184B4C: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 83184B50: 917D000C  stw r11, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83184B54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83184B58: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83184B5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83184B60: 4E800421  bctrl
	ctx.lr = 0x83184B64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83184B64: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83184B68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184B6C: 4BFFFD0D  bl 0x83184878
	ctx.lr = 0x83184B70;
	sub_83184878(ctx, base);
	// 83184B70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83184B74: 807EA32C  lwz r3, -0x5cd4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83184B78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83184B7C: 419A001C  beq cr6, 0x83184b98
	if ctx.cr[6].eq {
	pc = 0x83184B98; continue 'dispatch;
	}
	// 83184B80: 93FD0074  stw r31, 0x74(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 83184B84: 389D006C  addi r4, r29, 0x6c
	ctx.r[4].s64 = ctx.r[29].s64 + 108;
	// 83184B88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83184B8C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83184B90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83184B94: 4E800421  bctrl
	ctx.lr = 0x83184B98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83184B98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184B9C: 38210310  addi r1, r1, 0x310
	ctx.r[1].s64 = ctx.r[1].s64 + 784;
	// 83184BA0: 480235F4  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83184BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83184BA8 size=228
    let mut pc: u32 = 0x83184BA8;
    'dispatch: loop {
        match pc {
            0x83184BA8 => {
    //   block [0x83184BA8..0x83184C8C)
	// 83184BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83184BAC: 480235BD  bl 0x831a8168
	ctx.lr = 0x83184BB0;
	sub_831A8130(ctx, base);
	// 83184BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83184BB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83184BB8: 48002729  bl 0x831872e0
	ctx.lr = 0x83184BBC;
	sub_831872E0(ctx, base);
	// 83184BBC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83184BC0: 419A001C  beq cr6, 0x83184bdc
	if ctx.cr[6].eq {
	pc = 0x83184BDC; continue 'dispatch;
	}
	// 83184BC4: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83184BC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83184BCC: 60840131  ori r4, r4, 0x131
	ctx.r[4].u64 = ctx.r[4].u64 | 305;
	// 83184BD0: 48002929  bl 0x831874f8
	ctx.lr = 0x83184BD4;
	sub_831874F8(ctx, base);
	// 83184BD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83184BD8: 480235E0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83184BDC: 3F808345  lis r28, -0x7cbb
	ctx.r[28].s64 = -2092630016;
	// 83184BE0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83184BE4: 3BCBD778  addi r30, r11, -0x2888
	ctx.r[30].s64 = ctx.r[11].s64 + -10376;
	// 83184BE8: 807CA32C  lwz r3, -0x5cd4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83184BEC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83184BF0: 419A001C  beq cr6, 0x83184c0c
	if ctx.cr[6].eq {
	pc = 0x83184C0C; continue 'dispatch;
	}
	// 83184BF4: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 83184BF8: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83184BFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83184C00: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83184C04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83184C08: 4E800421  bctrl
	ctx.lr = 0x83184C0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83184C0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184C10: 4BFFFCE1  bl 0x831848f0
	ctx.lr = 0x83184C14;
	sub_831848F0(ctx, base);
	// 83184C14: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 83184C18: 4800A409  bl 0x8318f020
	ctx.lr = 0x83184C1C;
	sub_8318F020(ctx, base);
	// 83184C1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184C20: 48009539  bl 0x8318e158
	ctx.lr = 0x83184C24;
	sub_8318E158(ctx, base);
	// 83184C24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184C28: 4BFFE761  bl 0x83183388
	ctx.lr = 0x83184C2C;
	sub_83183388(ctx, base);
	// 83184C2C: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83184C30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83184C34: 394BA100  addi r10, r11, -0x5f00
	ctx.r[10].s64 = ctx.r[11].s64 + -24320;
	// 83184C38: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83184C3C: 396A020C  addi r11, r10, 0x20c
	ctx.r[11].s64 = ctx.r[10].s64 + 524;
	// 83184C40: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83184C44: 7F08F840  cmplw cr6, r8, r31
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83184C48: 409A0008  bne cr6, 0x83184c50
	if !ctx.cr[6].eq {
	pc = 0x83184C50; continue 'dispatch;
	}
	// 83184C4C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83184C50: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83184C54: 390A022C  addi r8, r10, 0x22c
	ctx.r[8].s64 = ctx.r[10].s64 + 556;
	// 83184C58: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83184C5C: 4198FFE4  blt cr6, 0x83184c40
	if ctx.cr[6].lt {
	pc = 0x83184C40; continue 'dispatch;
	}
	// 83184C60: 807CA32C  lwz r3, -0x5cd4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83184C64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83184C68: 419A0018  beq cr6, 0x83184c80
	if ctx.cr[6].eq {
	pc = 0x83184C80; continue 'dispatch;
	}
	// 83184C6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83184C70: 389E006C  addi r4, r30, 0x6c
	ctx.r[4].s64 = ctx.r[30].s64 + 108;
	// 83184C74: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83184C78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83184C7C: 4E800421  bctrl
	ctx.lr = 0x83184C80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83184C80: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83184C84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83184C88: 48023530  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83184C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83184C90 size=164
    let mut pc: u32 = 0x83184C90;
    'dispatch: loop {
        match pc {
            0x83184C90 => {
    //   block [0x83184C90..0x83184D34)
	// 83184C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83184C94: 480234D5  bl 0x831a8168
	ctx.lr = 0x83184C98;
	sub_831A8130(ctx, base);
	// 83184C98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83184C9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83184CA0: 48002641  bl 0x831872e0
	ctx.lr = 0x83184CA4;
	sub_831872E0(ctx, base);
	// 83184CA4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83184CA8: 419A001C  beq cr6, 0x83184cc4
	if ctx.cr[6].eq {
	pc = 0x83184CC4; continue 'dispatch;
	}
	// 83184CAC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83184CB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83184CB4: 60840133  ori r4, r4, 0x133
	ctx.r[4].u64 = ctx.r[4].u64 | 307;
	// 83184CB8: 48002841  bl 0x831874f8
	ctx.lr = 0x83184CBC;
	sub_831874F8(ctx, base);
	// 83184CBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83184CC0: 480234F8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83184CC4: 3FC08345  lis r30, -0x7cbb
	ctx.r[30].s64 = -2092630016;
	// 83184CC8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83184CCC: 3BABD928  addi r29, r11, -0x26d8
	ctx.r[29].s64 = ctx.r[11].s64 + -9944;
	// 83184CD0: 807EA32C  lwz r3, -0x5cd4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83184CD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83184CD8: 419A001C  beq cr6, 0x83184cf4
	if ctx.cr[6].eq {
	pc = 0x83184CF4; continue 'dispatch;
	}
	// 83184CDC: 93FD000C  stw r31, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 83184CE0: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 83184CE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83184CE8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83184CEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83184CF0: 4E800421  bctrl
	ctx.lr = 0x83184CF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83184CF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184CF8: 4BFFFBF9  bl 0x831848f0
	ctx.lr = 0x83184CFC;
	sub_831848F0(ctx, base);
	// 83184CFC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83184D00: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83184D04: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 83184D08: 807EA32C  lwz r3, -0x5cd4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83184D0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83184D10: 419A0018  beq cr6, 0x83184d28
	if ctx.cr[6].eq {
	pc = 0x83184D28; continue 'dispatch;
	}
	// 83184D14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83184D18: 389D006C  addi r4, r29, 0x6c
	ctx.r[4].s64 = ctx.r[29].s64 + 108;
	// 83184D1C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83184D20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83184D24: 4E800421  bctrl
	ctx.lr = 0x83184D28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83184D28: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83184D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83184D30: 48023488  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83184D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83184D38 size=152
    let mut pc: u32 = 0x83184D38;
    'dispatch: loop {
        match pc {
            0x83184D38 => {
    //   block [0x83184D38..0x83184DD0)
	// 83184D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83184D3C: 48023431  bl 0x831a816c
	ctx.lr = 0x83184D40;
	sub_831A8130(ctx, base);
	// 83184D40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83184D44: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83184D48: 48002599  bl 0x831872e0
	ctx.lr = 0x83184D4C;
	sub_831872E0(ctx, base);
	// 83184D4C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83184D50: 419A001C  beq cr6, 0x83184d6c
	if ctx.cr[6].eq {
	pc = 0x83184D6C; continue 'dispatch;
	}
	// 83184D54: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83184D58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83184D5C: 60840138  ori r4, r4, 0x138
	ctx.r[4].u64 = ctx.r[4].u64 | 312;
	// 83184D60: 48002799  bl 0x831874f8
	ctx.lr = 0x83184D64;
	sub_831874F8(ctx, base);
	// 83184D64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83184D68: 48023454  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83184D6C: 3FC08345  lis r30, -0x7cbb
	ctx.r[30].s64 = -2092630016;
	// 83184D70: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83184D74: 3BEBDF10  addi r31, r11, -0x20f0
	ctx.r[31].s64 = ctx.r[11].s64 + -8432;
	// 83184D78: 807EA32C  lwz r3, -0x5cd4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83184D7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83184D80: 419A001C  beq cr6, 0x83184d9c
	if ctx.cr[6].eq {
	pc = 0x83184D9C; continue 'dispatch;
	}
	// 83184D84: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 83184D88: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 83184D8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83184D90: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83184D94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83184D98: 4E800421  bctrl
	ctx.lr = 0x83184D9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83184D9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83184DA0: 4BFFFBD9  bl 0x83184978
	ctx.lr = 0x83184DA4;
	sub_83184978(ctx, base);
	// 83184DA4: 807EA32C  lwz r3, -0x5cd4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83184DA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83184DAC: 419A0018  beq cr6, 0x83184dc4
	if ctx.cr[6].eq {
	pc = 0x83184DC4; continue 'dispatch;
	}
	// 83184DB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83184DB4: 389F006C  addi r4, r31, 0x6c
	ctx.r[4].s64 = ctx.r[31].s64 + 108;
	// 83184DB8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83184DBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83184DC0: 4E800421  bctrl
	ctx.lr = 0x83184DC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83184DC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83184DC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83184DCC: 480233F0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83184DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83184DD0 size=36
    let mut pc: u32 = 0x83184DD0;
    'dispatch: loop {
        match pc {
            0x83184DD0 => {
    //   block [0x83184DD0..0x83184DF4)
	// 83184DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83184DD4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83184DD8: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83184DDC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83184DE0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83184DE4: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83184DE8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83184DEC: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83184DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83184DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83184DF8 size=248
    let mut pc: u32 = 0x83184DF8;
    'dispatch: loop {
        match pc {
            0x83184DF8 => {
    //   block [0x83184DF8..0x83184EF0)
	// 83184DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83184DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83184E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83184E04: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83184E08: 816A1E38  lwz r11, 0x1e38(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(7736 as u32) ) } as u64;
	// 83184E0C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83184E10: 816A1E38  lwz r11, 0x1e38(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(7736 as u32) ) } as u64;
	// 83184E14: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83184E18: 419A00C4  beq cr6, 0x83184edc
	if ctx.cr[6].eq {
	pc = 0x83184EDC; continue 'dispatch;
	}
	// 83184E1C: 816A0954  lwz r11, 0x954(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2388 as u32) ) } as u64;
	// 83184E20: 812A1E50  lwz r9, 0x1e50(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(7760 as u32) ) } as u64;
	// 83184E24: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83184E28: 40980054  bge cr6, 0x83184e7c
	if !ctx.cr[6].lt {
	pc = 0x83184E7C; continue 'dispatch;
	}
	// 83184E2C: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83184E30: 806BA32C  lwz r3, -0x5cd4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83184E34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83184E38: 419A0030  beq cr6, 0x83184e68
	if ctx.cr[6].eq {
	pc = 0x83184E68; continue 'dispatch;
	}
	// 83184E3C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83184E40: 392BB110  addi r9, r11, -0x4ef0
	ctx.r[9].s64 = ctx.r[11].s64 + -20208;
	// 83184E44: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83184E48: 396BE5D8  addi r11, r11, -0x1a28
	ctx.r[11].s64 = ctx.r[11].s64 + -6696;
	// 83184E4C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 83184E50: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83184E54: 912B0018  stw r9, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 83184E58: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83184E5C: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 83184E60: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83184E64: 4E800421  bctrl
	ctx.lr = 0x83184E68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83184E68: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83184E6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83184E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83184E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83184E78: 4E800020  blr
	return;
	// 83184E7C: 816A0950  lwz r11, 0x950(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2384 as u32) ) } as u64;
	// 83184E80: 812A1E40  lwz r9, 0x1e40(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(7744 as u32) ) } as u64;
	// 83184E84: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83184E88: 41980054  blt cr6, 0x83184edc
	if ctx.cr[6].lt {
	pc = 0x83184EDC; continue 'dispatch;
	}
	// 83184E8C: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83184E90: 806BA32C  lwz r3, -0x5cd4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83184E94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83184E98: 419A0030  beq cr6, 0x83184ec8
	if ctx.cr[6].eq {
	pc = 0x83184EC8; continue 'dispatch;
	}
	// 83184E9C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83184EA0: 392BB104  addi r9, r11, -0x4efc
	ctx.r[9].s64 = ctx.r[11].s64 + -20220;
	// 83184EA4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83184EA8: 396BE5D8  addi r11, r11, -0x1a28
	ctx.r[11].s64 = ctx.r[11].s64 + -6696;
	// 83184EAC: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 83184EB0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83184EB4: 912B0018  stw r9, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 83184EB8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83184EBC: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 83184EC0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83184EC4: 4E800421  bctrl
	ctx.lr = 0x83184EC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83184EC8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83184ECC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83184ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83184ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83184ED8: 4E800020  blr
	return;
	// 83184EDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83184EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83184EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83184EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83184EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83184EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83184EF0 size=56
    let mut pc: u32 = 0x83184EF0;
    'dispatch: loop {
        match pc {
            0x83184EF0 => {
    //   block [0x83184EF0..0x83184F28)
	// 83184EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83184EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83184EF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83184EFC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83184F00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83184F04: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83184F08: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 83184F0C: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83184F10: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83184F14: 48006ED5  bl 0x8318bde8
	ctx.lr = 0x83184F18;
	sub_8318BDE8(ctx, base);
	// 83184F18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83184F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83184F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83184F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83184F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83184F28 size=20
    let mut pc: u32 = 0x83184F28;
    'dispatch: loop {
        match pc {
            0x83184F28 => {
    //   block [0x83184F28..0x83184F3C)
	// 83184F28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83184F2C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 83184F30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83184F34: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 83184F38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83184F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83184F40 size=104
    let mut pc: u32 = 0x83184F40;
    'dispatch: loop {
        match pc {
            0x83184F40 => {
    //   block [0x83184F40..0x83184FA8)
	// 83184F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83184F44: 48023229  bl 0x831a816c
	ctx.lr = 0x83184F48;
	sub_831A8130(ctx, base);
	// 83184F48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83184F4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83184F50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83184F54: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83184F58: 48002389  bl 0x831872e0
	ctx.lr = 0x83184F5C;
	sub_831872E0(ctx, base);
	// 83184F5C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83184F60: 419A001C  beq cr6, 0x83184f7c
	if ctx.cr[6].eq {
	pc = 0x83184F7C; continue 'dispatch;
	}
	// 83184F64: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83184F68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83184F6C: 60840144  ori r4, r4, 0x144
	ctx.r[4].u64 = ctx.r[4].u64 | 324;
	// 83184F70: 48002589  bl 0x831874f8
	ctx.lr = 0x83184F74;
	sub_831874F8(ctx, base);
	// 83184F74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83184F78: 48023244  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83184F7C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83184F80: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83184F84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184F88: 4BFF71D9  bl 0x8317c160
	ctx.lr = 0x83184F8C;
	sub_8317C160(ctx, base);
	// 83184F8C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83184F90: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83184F94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184F98: 480004E9  bl 0x83185480
	ctx.lr = 0x83184F9C;
	sub_83185480(ctx, base);
	// 83184F9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83184FA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83184FA4: 48023218  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83184FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83184FA8 size=104
    let mut pc: u32 = 0x83184FA8;
    'dispatch: loop {
        match pc {
            0x83184FA8 => {
    //   block [0x83184FA8..0x83185010)
	// 83184FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83184FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83184FB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83184FB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83184FB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83184FBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83184FC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83184FC4: 4800231D  bl 0x831872e0
	ctx.lr = 0x83184FC8;
	sub_831872E0(ctx, base);
	// 83184FC8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83184FCC: 419A0018  beq cr6, 0x83184fe4
	if ctx.cr[6].eq {
	pc = 0x83184FE4; continue 'dispatch;
	}
	// 83184FD0: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83184FD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83184FD8: 60840145  ori r4, r4, 0x145
	ctx.r[4].u64 = ctx.r[4].u64 | 325;
	// 83184FDC: 4800251D  bl 0x831874f8
	ctx.lr = 0x83184FE0;
	sub_831874F8(ctx, base);
	// 83184FE0: 48000018  b 0x83184ff8
	pc = 0x83184FF8; continue 'dispatch;
	// 83184FE4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83184FE8: 3880001E  li r4, 0x1e
	ctx.r[4].s64 = 30;
	// 83184FEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83184FF0: 4BFF4611  bl 0x83179600
	ctx.lr = 0x83184FF4;
	sub_83179600(ctx, base);
	// 83184FF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83184FF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83184FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83185004: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83185008: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318500C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185010 size=104
    let mut pc: u32 = 0x83185010;
    'dispatch: loop {
        match pc {
            0x83185010 => {
    //   block [0x83185010..0x83185078)
	// 83185010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83185018: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318501C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83185020: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185024: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83185028: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318502C: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83185030: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83185034: 419A0014  beq cr6, 0x83185048
	if ctx.cr[6].eq {
	pc = 0x83185048; continue 'dispatch;
	}
	// 83185038: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8318503C: 419A000C  beq cr6, 0x83185048
	if ctx.cr[6].eq {
	pc = 0x83185048; continue 'dispatch;
	}
	// 83185040: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83185044: 4800001C  b 0x83185060
	pc = 0x83185060; continue 'dispatch;
	// 83185048: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318504C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83185050: 4BFF77E1  bl 0x8317c830
	ctx.lr = 0x83185054;
	sub_8317C830(ctx, base);
	// 83185054: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83185058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318505C: 4BFFFE95  bl 0x83184ef0
	ctx.lr = 0x83185060;
	sub_83184EF0(ctx, base);
	// 83185060: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83185064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318506C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83185070: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83185074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185078 size=96
    let mut pc: u32 = 0x83185078;
    'dispatch: loop {
        match pc {
            0x83185078 => {
    //   block [0x83185078..0x831850D8)
	// 83185078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318507C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83185080: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83185084: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185088: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318508C: 48002255  bl 0x831872e0
	ctx.lr = 0x83185090;
	sub_831872E0(ctx, base);
	// 83185090: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83185094: 419A0028  beq cr6, 0x831850bc
	if ctx.cr[6].eq {
	pc = 0x831850BC; continue 'dispatch;
	}
	// 83185098: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318509C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831850A0: 60840143  ori r4, r4, 0x143
	ctx.r[4].u64 = ctx.r[4].u64 | 323;
	// 831850A4: 48002455  bl 0x831874f8
	ctx.lr = 0x831850A8;
	sub_831874F8(ctx, base);
	// 831850A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831850AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831850B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831850B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831850B8: 4E800020  blr
	return;
	// 831850BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831850C0: 4BFFFE69  bl 0x83184f28
	ctx.lr = 0x831850C4;
	sub_83184F28(ctx, base);
	// 831850C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831850C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831850CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831850D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831850D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831850D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831850D8 size=28
    let mut pc: u32 = 0x831850D8;
    'dispatch: loop {
        match pc {
            0x831850D8 => {
    //   block [0x831850D8..0x831850F4)
	// 831850D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831850DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831850E0: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 831850E4: 41980048  blt cr6, 0x8318512c
	if ctx.cr[6].lt {
		sub_8318512C(ctx, base);
		return;
	}
	// 831850E8: 419A0024  beq cr6, 0x8318510c
	if ctx.cr[6].eq {
		sub_8318510C(ctx, base);
		return;
	}
	// 831850EC: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 831850F0: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831850F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831850F4 size=12
    let mut pc: u32 = 0x831850F4;
    'dispatch: loop {
        match pc {
            0x831850F4 => {
    //   block [0x831850F4..0x83185100)
	// 831850F4: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 831850F8: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 831850FC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83185100 size=12
    let mut pc: u32 = 0x83185100;
    'dispatch: loop {
        match pc {
            0x83185100 => {
    //   block [0x83185100..0x8318510C)
	// 83185100: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83185104: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83185108: 4BFFFF08  b 0x83185010
	sub_83185010(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318510C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318510C size=20
    let mut pc: u32 = 0x8318510C;
    'dispatch: loop {
        match pc {
            0x8318510C => {
    //   block [0x8318510C..0x83185120)
	// 8318510C: 814B0054  lwz r10, 0x54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 83185110: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 83185114: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83185118: 912B0054  stw r9, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8318511C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83185120 size=12
    let mut pc: u32 = 0x83185120;
    'dispatch: loop {
        match pc {
            0x83185120 => {
    //   block [0x83185120..0x8318512C)
	// 83185120: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83185124: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83185128: 4BFFFEE8  b 0x83185010
	sub_83185010(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318512C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318512C size=16
    let mut pc: u32 = 0x8318512C;
    'dispatch: loop {
        match pc {
            0x8318512C => {
    //   block [0x8318512C..0x8318513C)
	// 8318512C: 814B0054  lwz r10, 0x54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 83185130: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83185134: 914B0054  stw r10, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 83185138: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318513C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318513C size=12
    let mut pc: u32 = 0x8318513C;
    'dispatch: loop {
        match pc {
            0x8318513C => {
    //   block [0x8318513C..0x83185148)
	// 8318513C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83185140: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83185144: 4BFFFECC  b 0x83185010
	sub_83185010(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83185148 size=4
    let mut pc: u32 = 0x83185148;
    'dispatch: loop {
        match pc {
            0x83185148 => {
    //   block [0x83185148..0x8318514C)
	// 83185148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185150 size=156
    let mut pc: u32 = 0x83185150;
    'dispatch: loop {
        match pc {
            0x83185150 => {
    //   block [0x83185150..0x831851EC)
	// 83185150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83185158: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318515C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83185160: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185164: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83185168: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318516C: 48002175  bl 0x831872e0
	ctx.lr = 0x83185170;
	sub_831872E0(ctx, base);
	// 83185170: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83185174: 419A0018  beq cr6, 0x8318518c
	if ctx.cr[6].eq {
	pc = 0x8318518C; continue 'dispatch;
	}
	// 83185178: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318517C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83185180: 60840142  ori r4, r4, 0x142
	ctx.r[4].u64 = ctx.r[4].u64 | 322;
	// 83185184: 48002375  bl 0x831874f8
	ctx.lr = 0x83185188;
	sub_831874F8(ctx, base);
	// 83185188: 4800004C  b 0x831851d4
	pc = 0x831851D4; continue 'dispatch;
	// 8318518C: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83185190: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83185194: 409A001C  bne cr6, 0x831851b0
	if !ctx.cr[6].eq {
	pc = 0x831851B0; continue 'dispatch;
	}
	// 83185198: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318519C: 409A000C  bne cr6, 0x831851a8
	if !ctx.cr[6].eq {
	pc = 0x831851A8; continue 'dispatch;
	}
	// 831851A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831851A4: 48000030  b 0x831851d4
	pc = 0x831851D4; continue 'dispatch;
	// 831851A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831851AC: 48000014  b 0x831851c0
	pc = 0x831851C0; continue 'dispatch;
	// 831851B0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 831851B4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 831851B8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 831851BC: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 831851C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831851C4: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 831851C8: 4BFFFF11  bl 0x831850d8
	ctx.lr = 0x831851CC;
	sub_831850D8(ctx, base);
	// 831851CC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831851D0: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 831851D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831851D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831851DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831851E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831851E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831851E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831851F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831851F0 size=20
    let mut pc: u32 = 0x831851F0;
    'dispatch: loop {
        match pc {
            0x831851F0 => {
    //   block [0x831851F0..0x83185204)
	// 831851F0: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 831851F4: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 831851F8: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 831851FC: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 83185200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83185208 size=12
    let mut pc: u32 = 0x83185208;
    'dispatch: loop {
        match pc {
            0x83185208 => {
    //   block [0x83185208..0x83185214)
	// 83185208: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318520C: 60840601  ori r4, r4, 0x601
	ctx.r[4].u64 = ctx.r[4].u64 | 1537;
	// 83185210: 480022E8  b 0x831874f8
	sub_831874F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185218 size=168
    let mut pc: u32 = 0x83185218;
    'dispatch: loop {
        match pc {
            0x83185218 => {
    //   block [0x83185218..0x831852C0)
	// 83185218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318521C: 48022F4D  bl 0x831a8168
	ctx.lr = 0x83185220;
	sub_831A8130(ctx, base);
	// 83185220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185224: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83185228: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318522C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83185230: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83185234: 480020AD  bl 0x831872e0
	ctx.lr = 0x83185238;
	sub_831872E0(ctx, base);
	// 83185238: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318523C: 419A001C  beq cr6, 0x83185258
	if ctx.cr[6].eq {
	pc = 0x83185258; continue 'dispatch;
	}
	// 83185240: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83185244: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83185248: 60840191  ori r4, r4, 0x191
	ctx.r[4].u64 = ctx.r[4].u64 | 401;
	// 8318524C: 480022AD  bl 0x831874f8
	ctx.lr = 0x83185250;
	sub_831874F8(ctx, base);
	// 83185250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83185254: 48022F64  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83185258: 815F1968  lwz r10, 0x1968(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6504 as u32) ) } as u64;
	// 8318525C: 813F1960  lwz r9, 0x1960(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6496 as u32) ) } as u64;
	// 83185260: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 83185264: 409A001C  bne cr6, 0x83185280
	if !ctx.cr[6].eq {
	pc = 0x83185280; continue 'dispatch;
	}
	// 83185268: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318526C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83185270: 60840602  ori r4, r4, 0x602
	ctx.r[4].u64 = ctx.r[4].u64 | 1538;
	// 83185274: 48002285  bl 0x831874f8
	ctx.lr = 0x83185278;
	sub_831874F8(ctx, base);
	// 83185278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318527C: 48022F3C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83185280: 57CB2036  slwi r11, r30, 4
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83185284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83185288: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8318528C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83185290: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83185294: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83185298: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8318529C: 4BFFFF55  bl 0x831851f0
	ctx.lr = 0x831852A0;
	sub_831851F0(ctx, base);
	// 831852A0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 831852A4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 831852A8: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 831852AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831852B0: 48008911  bl 0x8318dbc0
	ctx.lr = 0x831852B4;
	sub_8318DBC0(ctx, base);
	// 831852B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831852B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831852BC: 48022EFC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831852C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831852C0 size=108
    let mut pc: u32 = 0x831852C0;
    'dispatch: loop {
        match pc {
            0x831852C0 => {
    //   block [0x831852C0..0x8318532C)
	// 831852C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831852C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831852C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831852CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831852D0: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 831852D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831852D8: 48006B81  bl 0x8318be58
	ctx.lr = 0x831852DC;
	sub_8318BE58(ctx, base);
	// 831852DC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831852E0: 419A0038  beq cr6, 0x83185318
	if ctx.cr[6].eq {
	pc = 0x83185318; continue 'dispatch;
	}
	// 831852E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831852E8: 809F1968  lwz r4, 0x1968(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6504 as u32) ) } as u64;
	// 831852EC: 48008CED  bl 0x8318dfd8
	ctx.lr = 0x831852F0;
	sub_8318DFD8(ctx, base);
	// 831852F0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831852F4: 409A0024  bne cr6, 0x83185318
	if !ctx.cr[6].eq {
	pc = 0x83185318; continue 'dispatch;
	}
	// 831852F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831852FC: 4BECE3F5  bl 0x830536f0
	ctx.lr = 0x83185300;
	sub_830536F0(ctx, base);
	// 83185300: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83185304: 419A0014  beq cr6, 0x83185318
	if ctx.cr[6].eq {
	pc = 0x83185318; continue 'dispatch;
	}
	// 83185308: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8318530C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 83185310: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83185314: 48006B35  bl 0x8318be48
	ctx.lr = 0x83185318;
	sub_8318BE48(ctx, base);
	// 83185318: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318531C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83185324: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83185328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185330 size=108
    let mut pc: u32 = 0x83185330;
    'dispatch: loop {
        match pc {
            0x83185330 => {
    //   block [0x83185330..0x8318539C)
	// 83185330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83185338: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318533C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185340: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 83185344: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83185348: 48006AF1  bl 0x8318be38
	ctx.lr = 0x8318534C;
	sub_8318BE38(ctx, base);
	// 8318534C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83185350: 419A0038  beq cr6, 0x83185388
	if ctx.cr[6].eq {
	pc = 0x83185388; continue 'dispatch;
	}
	// 83185354: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83185358: 809F1968  lwz r4, 0x1968(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6504 as u32) ) } as u64;
	// 8318535C: 48008C45  bl 0x8318dfa0
	ctx.lr = 0x83185360;
	sub_8318DFA0(ctx, base);
	// 83185360: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83185364: 409A0024  bne cr6, 0x83185388
	if !ctx.cr[6].eq {
	pc = 0x83185388; continue 'dispatch;
	}
	// 83185368: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318536C: 4BECE385  bl 0x830536f0
	ctx.lr = 0x83185370;
	sub_830536F0(ctx, base);
	// 83185370: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83185374: 419A0014  beq cr6, 0x83185388
	if ctx.cr[6].eq {
	pc = 0x83185388; continue 'dispatch;
	}
	// 83185378: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8318537C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 83185380: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83185384: 48006AA5  bl 0x8318be28
	ctx.lr = 0x83185388;
	sub_8318BE28(ctx, base);
	// 83185388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318538C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83185394: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83185398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831853A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831853A0 size=20
    let mut pc: u32 = 0x831853A0;
    'dispatch: loop {
        match pc {
            0x831853A0 => {
    //   block [0x831853A0..0x831853B4)
	// 831853A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831853A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831853A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831853AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831853B0: 4BFFFE40  b 0x831851f0
	sub_831851F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831853B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831853B8 size=60
    let mut pc: u32 = 0x831853B8;
    'dispatch: loop {
        match pc {
            0x831853B8 => {
    //   block [0x831853B8..0x831853F4)
	// 831853B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831853BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831853C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831853C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831853C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831853CC: 4BFFFEF5  bl 0x831852c0
	ctx.lr = 0x831853D0;
	sub_831852C0(ctx, base);
	// 831853D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831853D4: 4BFFFF5D  bl 0x83185330
	ctx.lr = 0x831853D8;
	sub_83185330(ctx, base);
	// 831853D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831853DC: 4BFF90AD  bl 0x8317e488
	ctx.lr = 0x831853E0;
	sub_8317E488(ctx, base);
	// 831853E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831853E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831853E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831853EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831853F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831853F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831853F8 size=84
    let mut pc: u32 = 0x831853F8;
    'dispatch: loop {
        match pc {
            0x831853F8 => {
    //   block [0x831853F8..0x8318544C)
	// 831853F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831853FC: 48022D6D  bl 0x831a8168
	ctx.lr = 0x83185400;
	sub_831A8130(ctx, base);
	// 83185400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185404: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83185408: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8318540C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83185410: 3BC40004  addi r30, r4, 4
	ctx.r[30].s64 = ctx.r[4].s64 + 4;
	// 83185414: 93E40000  stw r31, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83185418: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318541C: 4BFFFF85  bl 0x831853a0
	ctx.lr = 0x83185420;
	sub_831853A0(ctx, base);
	// 83185420: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83185424: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83185428: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8318542C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83185430: 48008791  bl 0x8318dbc0
	ctx.lr = 0x83185434;
	sub_8318DBC0(ctx, base);
	// 83185434: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83185438: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 8318543C: 2F1F0003  cmpwi cr6, r31, 3
	ctx.cr[6].compare_i32(ctx.r[31].s32, 3, &mut ctx.xer);
	// 83185440: 4198FFD8  blt cr6, 0x83185418
	if ctx.cr[6].lt {
	pc = 0x83185418; continue 'dispatch;
	}
	// 83185444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83185448: 48022D70  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185450 size=48
    let mut pc: u32 = 0x83185450;
    'dispatch: loop {
        match pc {
            0x83185450 => {
    //   block [0x83185450..0x83185480)
	// 83185450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83185458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318545C: 38831DF0  addi r4, r3, 0x1df0
	ctx.r[4].s64 = ctx.r[3].s64 + 7664;
	// 83185460: 80A31968  lwz r5, 0x1968(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6504 as u32) ) } as u64;
	// 83185464: 90831960  stw r4, 0x1960(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(6496 as u32), ctx.r[4].u32 ) };
	// 83185468: 4BFFFF91  bl 0x831853f8
	ctx.lr = 0x8318546C;
	sub_831853F8(ctx, base);
	// 8318546C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83185470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83185474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318547C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185480 size=84
    let mut pc: u32 = 0x83185480;
    'dispatch: loop {
        match pc {
            0x83185480 => {
    //   block [0x83185480..0x831854D4)
	// 83185480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185484: 48022CE9  bl 0x831a816c
	ctx.lr = 0x83185488;
	sub_831A8130(ctx, base);
	// 83185488: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318548C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83185490: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83185494: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83185498: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318549C: 4BFF3F75  bl 0x83179410
	ctx.lr = 0x831854A0;
	sub_83179410(ctx, base);
	// 831854A0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831854A4: 419A0028  beq cr6, 0x831854cc
	if ctx.cr[6].eq {
	pc = 0x831854CC; continue 'dispatch;
	}
	// 831854A8: 817F191C  lwz r11, 0x191c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6428 as u32) ) } as u64;
	// 831854AC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 831854B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831854B4: 419A0018  beq cr6, 0x831854cc
	if ctx.cr[6].eq {
	pc = 0x831854CC; continue 'dispatch;
	}
	// 831854B8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831854BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831854C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831854C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831854C8: 4E800421  bctrl
	ctx.lr = 0x831854CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831854CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831854D0: 48022CEC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831854D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831854D8 size=92
    let mut pc: u32 = 0x831854D8;
    'dispatch: loop {
        match pc {
            0x831854D8 => {
    //   block [0x831854D8..0x83185534)
	// 831854D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831854DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831854E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831854E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831854E8: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 831854EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831854F0: 48006949  bl 0x8318be38
	ctx.lr = 0x831854F4;
	sub_8318BE38(ctx, base);
	// 831854F4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831854F8: 419A0028  beq cr6, 0x83185520
	if ctx.cr[6].eq {
	pc = 0x83185520; continue 'dispatch;
	}
	// 831854FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83185500: 809F1924  lwz r4, 0x1924(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6436 as u32) ) } as u64;
	// 83185504: 48008A9D  bl 0x8318dfa0
	ctx.lr = 0x83185508;
	sub_8318DFA0(ctx, base);
	// 83185508: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8318550C: 409A0014  bne cr6, 0x83185520
	if !ctx.cr[6].eq {
	pc = 0x83185520; continue 'dispatch;
	}
	// 83185510: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83185514: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83185518: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318551C: 4800690D  bl 0x8318be28
	ctx.lr = 0x83185520;
	sub_8318BE28(ctx, base);
	// 83185520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83185524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318552C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83185530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185538 size=92
    let mut pc: u32 = 0x83185538;
    'dispatch: loop {
        match pc {
            0x83185538 => {
    //   block [0x83185538..0x83185594)
	// 83185538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318553C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83185540: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83185544: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185548: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8318554C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83185550: 48006909  bl 0x8318be58
	ctx.lr = 0x83185554;
	sub_8318BE58(ctx, base);
	// 83185554: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83185558: 419A0028  beq cr6, 0x83185580
	if ctx.cr[6].eq {
	pc = 0x83185580; continue 'dispatch;
	}
	// 8318555C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83185560: 809F1924  lwz r4, 0x1924(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6436 as u32) ) } as u64;
	// 83185564: 48008A75  bl 0x8318dfd8
	ctx.lr = 0x83185568;
	sub_8318DFD8(ctx, base);
	// 83185568: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8318556C: 409A0014  bne cr6, 0x83185580
	if !ctx.cr[6].eq {
	pc = 0x83185580; continue 'dispatch;
	}
	// 83185570: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83185574: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83185578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318557C: 480068CD  bl 0x8318be48
	ctx.lr = 0x83185580;
	sub_8318BE48(ctx, base);
	// 83185580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83185584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318558C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83185590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185598 size=40
    let mut pc: u32 = 0x83185598;
    'dispatch: loop {
        match pc {
            0x83185598 => {
    //   block [0x83185598..0x831855C0)
	// 83185598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318559C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831855A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831855A4: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 831855A8: 4BFF3E69  bl 0x83179410
	ctx.lr = 0x831855AC;
	sub_83179410(ctx, base);
	// 831855AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831855B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831855B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831855B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831855BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831855C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831855C0 size=100
    let mut pc: u32 = 0x831855C0;
    'dispatch: loop {
        match pc {
            0x831855C0 => {
    //   block [0x831855C0..0x83185624)
	// 831855C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831855C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831855C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831855CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831855D0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 831855D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831855D8: 4BFF3E39  bl 0x83179410
	ctx.lr = 0x831855DC;
	sub_83179410(ctx, base);
	// 831855DC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831855E0: 409A0018  bne cr6, 0x831855f8
	if !ctx.cr[6].eq {
	pc = 0x831855F8; continue 'dispatch;
	}
	// 831855E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831855E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831855EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831855F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831855F4: 4E800020  blr
	return;
	// 831855F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831855FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83185600: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 83185604: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83185608: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318560C: 480067DD  bl 0x8318bde8
	ctx.lr = 0x83185610;
	sub_8318BDE8(ctx, base);
	// 83185610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83185614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318561C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83185620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185628 size=100
    let mut pc: u32 = 0x83185628;
    'dispatch: loop {
        match pc {
            0x83185628 => {
    //   block [0x83185628..0x8318568C)
	// 83185628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318562C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83185630: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83185634: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185638: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8318563C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83185640: 4BFF3DD1  bl 0x83179410
	ctx.lr = 0x83185644;
	sub_83179410(ctx, base);
	// 83185644: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83185648: 409A0018  bne cr6, 0x83185660
	if !ctx.cr[6].eq {
	pc = 0x83185660; continue 'dispatch;
	}
	// 8318564C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83185650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83185658: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318565C: 4E800020  blr
	return;
	// 83185660: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83185664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83185668: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 8318566C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83185670: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83185674: 48006775  bl 0x8318bde8
	ctx.lr = 0x83185678;
	sub_8318BDE8(ctx, base);
	// 83185678: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318567C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83185684: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83185688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185690 size=92
    let mut pc: u32 = 0x83185690;
    'dispatch: loop {
        match pc {
            0x83185690 => {
    //   block [0x83185690..0x831856EC)
	// 83185690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83185698: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318569C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831856A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831856A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831856A8: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 831856AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831856B0: 4BFF3D61  bl 0x83179410
	ctx.lr = 0x831856B4;
	sub_83179410(ctx, base);
	// 831856B4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831856B8: 419A001C  beq cr6, 0x831856d4
	if ctx.cr[6].eq {
	pc = 0x831856D4; continue 'dispatch;
	}
	// 831856BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831856C0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 831856C4: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 831856C8: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 831856CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831856D0: 48006719  bl 0x8318bde8
	ctx.lr = 0x831856D4;
	sub_8318BDE8(ctx, base);
	// 831856D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831856D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831856DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831856E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831856E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831856E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831856F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831856F0 size=12
    let mut pc: u32 = 0x831856F0;
    'dispatch: loop {
        match pc {
            0x831856F0 => {
    //   block [0x831856F0..0x831856FC)
	// 831856F0: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 831856F4: 60840A01  ori r4, r4, 0xa01
	ctx.r[4].u64 = ctx.r[4].u64 | 2561;
	// 831856F8: 48001E00  b 0x831874f8
	sub_831874F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185700 size=56
    let mut pc: u32 = 0x83185700;
    'dispatch: loop {
        match pc {
            0x83185700 => {
    //   block [0x83185700..0x83185738)
	// 83185700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83185708: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318570C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185710: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83185714: 4BFFFDC5  bl 0x831854d8
	ctx.lr = 0x83185718;
	sub_831854D8(ctx, base);
	// 83185718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318571C: 4BFFFE1D  bl 0x83185538
	ctx.lr = 0x83185720;
	sub_83185538(ctx, base);
	// 83185720: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83185724: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83185728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318572C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83185730: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83185734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185738 size=68
    let mut pc: u32 = 0x83185738;
    'dispatch: loop {
        match pc {
            0x83185738 => {
    //   block [0x83185738..0x8318577C)
	// 83185738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318573C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83185740: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83185744: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185748: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8318574C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83185750: 4BFF3CC1  bl 0x83179410
	ctx.lr = 0x83185754;
	sub_83179410(ctx, base);
	// 83185754: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83185758: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318575C: 419A000C  beq cr6, 0x83185768
	if ctx.cr[6].eq {
	pc = 0x83185768; continue 'dispatch;
	}
	// 83185760: 397F1DD4  addi r11, r31, 0x1dd4
	ctx.r[11].s64 = ctx.r[31].s64 + 7636;
	// 83185764: 917F191C  stw r11, 0x191c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(6428 as u32), ctx.r[11].u32 ) };
	// 83185768: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318576C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83185774: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83185778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185780 size=84
    let mut pc: u32 = 0x83185780;
    'dispatch: loop {
        match pc {
            0x83185780 => {
    //   block [0x83185780..0x831857D4)
	// 83185780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83185788: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318578C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185790: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83185794: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83185798: 4BFF3C79  bl 0x83179410
	ctx.lr = 0x8318579C;
	sub_83179410(ctx, base);
	// 8318579C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831857A0: 409A0018  bne cr6, 0x831857b8
	if !ctx.cr[6].eq {
	pc = 0x831857B8; continue 'dispatch;
	}
	// 831857A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831857A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831857AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831857B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831857B4: 4E800020  blr
	return;
	// 831857B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831857BC: 4BFFFF45  bl 0x83185700
	ctx.lr = 0x831857C0;
	sub_83185700(ctx, base);
	// 831857C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831857C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831857C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831857CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831857D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831857D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831857D8 size=116
    let mut pc: u32 = 0x831857D8;
    'dispatch: loop {
        match pc {
            0x831857D8 => {
    //   block [0x831857D8..0x8318584C)
	// 831857D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831857DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831857E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831857E4: 39600120  li r11, 0x120
	ctx.r[11].s64 = 288;
	// 831857E8: 2F040120  cmpwi cr6, r4, 0x120
	ctx.cr[6].compare_i32(ctx.r[4].s32, 288, &mut ctx.xer);
	// 831857EC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831857F0: 40980018  bge cr6, 0x83185808
	if !ctx.cr[6].lt {
	pc = 0x83185808; continue 'dispatch;
	}
	// 831857F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831857F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831857FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83185804: 4E800020  blr
	return;
	// 83185808: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318580C: 2B0B0080  cmplwi cr6, r11, 0x80
	ctx.cr[6].compare_u32(ctx.r[11].u32, 128 as u32, &mut ctx.xer);
	// 83185810: 409AFFE4  bne cr6, 0x831857f4
	if !ctx.cr[6].eq {
	pc = 0x831857F4; continue 'dispatch;
	}
	// 83185814: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 83185818: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318581C: 409AFFD8  bne cr6, 0x831857f4
	if !ctx.cr[6].eq {
	pc = 0x831857F4; continue 'dispatch;
	}
	// 83185820: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83185824: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 83185828: 388B1CC4  addi r4, r11, 0x1cc4
	ctx.r[4].s64 = ctx.r[11].s64 + 7364;
	// 8318582C: 3863011A  addi r3, r3, 0x11a
	ctx.r[3].s64 = ctx.r[3].s64 + 282;
	// 83185830: 480275A1  bl 0x831acdd0
	ctx.lr = 0x83185834;
	sub_831ACDD0(ctx, base);
	// 83185834: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 83185838: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8318583C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83185840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83185848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83185850 size=12
    let mut pc: u32 = 0x83185850;
    'dispatch: loop {
        match pc {
            0x83185850 => {
    //   block [0x83185850..0x8318585C)
	// 83185850: 8163180C  lwz r11, 0x180c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6156 as u32) ) } as u64;
	// 83185854: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83185858: 4BFA8050  b 0x8312d8a8
	sub_8312D8A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83185860 size=12
    let mut pc: u32 = 0x83185860;
    'dispatch: loop {
        match pc {
            0x83185860 => {
    //   block [0x83185860..0x8318586C)
	// 83185860: 8163180C  lwz r11, 0x180c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6156 as u32) ) } as u64;
	// 83185864: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83185868: 4BFA8078  b 0x8312d8e0
	sub_8312D8E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83185870 size=12
    let mut pc: u32 = 0x83185870;
    'dispatch: loop {
        match pc {
            0x83185870 => {
    //   block [0x83185870..0x8318587C)
	// 83185870: 8163180C  lwz r11, 0x180c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6156 as u32) ) } as u64;
	// 83185874: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83185878: 4BFA80D0  b 0x8312d948
	sub_8312D948(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83185880 size=12
    let mut pc: u32 = 0x83185880;
    'dispatch: loop {
        match pc {
            0x83185880 => {
    //   block [0x83185880..0x8318588C)
	// 83185880: 8163180C  lwz r11, 0x180c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6156 as u32) ) } as u64;
	// 83185884: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83185888: 4BFA8148  b 0x8312d9d0
	sub_8312D9D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83185890 size=336
    let mut pc: u32 = 0x83185890;
    'dispatch: loop {
        match pc {
            0x83185890 => {
    //   block [0x83185890..0x831859E0)
	// 83185890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185894: 480228D5  bl 0x831a8168
	ctx.lr = 0x83185898;
	sub_831A8130(ctx, base);
	// 83185898: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318589C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831858A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831858A4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 831858A8: 817D180C  lwz r11, 0x180c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(6156 as u32) ) } as u64;
	// 831858AC: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831858B0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 831858B4: 419A0124  beq cr6, 0x831859d8
	if ctx.cr[6].eq {
	pc = 0x831859D8; continue 'dispatch;
	}
	// 831858B8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831858BC: 409A0014  bne cr6, 0x831858d0
	if !ctx.cr[6].eq {
	pc = 0x831858D0; continue 'dispatch;
	}
	// 831858C0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 831858C4: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 831858C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831858CC: 480000F4  b 0x831859c0
	pc = 0x831859C0; continue 'dispatch;
	// 831858D0: 7F1EF800  cmpw cr6, r30, r31
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[31].s32, &mut ctx.xer);
	// 831858D4: 409A0010  bne cr6, 0x831858e4
	if !ctx.cr[6].eq {
	pc = 0x831858E4; continue 'dispatch;
	}
	// 831858D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831858DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831858E0: 480000E0  b 0x831859c0
	pc = 0x831859C0; continue 'dispatch;
	// 831858E4: 7FEA07B4  extsw r10, r31
	ctx.r[10].s64 = ctx.r[31].s32 as i64;
	// 831858E8: 7FCB07B4  extsw r11, r30
	ctx.r[11].s64 = ctx.r[30].s32 as i64;
	// 831858EC: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 831858F0: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 831858F4: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831858F8: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 831858FC: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 83185900: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83185904: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 83185908: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8318590C: FC206824  fdiv f1, f0, f13
	ctx.f[1].f64 = ctx.f[0].f64 / ctx.f[13].f64;
	// 83185910: 480289C1  bl 0x831ae2d0
	ctx.lr = 0x83185914;
	sub_831AE2D0(ctx, base);
	// 83185914: FDA00818  frsp f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 83185918: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318591C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 83185920: C00BB1DC  lfs f0, -0x4e24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20004 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83185924: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83185928: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8318592C: C1AB9528  lfs f13, -0x6ad8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83185930: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 83185934: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 83185938: 7DA057AE  stfiwx f13, 0, r10
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 8318593C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83185940: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 83185944: 1D640064  mulli r11, r4, 0x64
	ctx.r[11].s64 = ctx.r[4].s64 * 100;
	// 83185948: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8318594C: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 83185950: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83185954: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83185958: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8318595C: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 83185960: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 83185964: C1AB9450  lfs f13, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83185968: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 8318596C: ED60682A  fadds f11, f0, f13
	ctx.f[11].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 83185970: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 83185974: FD80601E  fctiwz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 83185978: 7D8057AE  stfiwx f12, 0, r10
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 8318597C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83185980: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 83185984: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 83185988: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8318598C: C9810058  lfd f12, 0x58(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83185990: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 83185994: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 83185998: FF0C5800  fcmpu cr6, f12, f11
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[11].f64);
	// 8318599C: 40990018  ble cr6, 0x831859b4
	if !ctx.cr[6].gt {
	pc = 0x831859B4; continue 'dispatch;
	}
	// 831859A0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 831859A4: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 831859A8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831859AC: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 831859B0: 48000010  b 0x831859c0
	pc = 0x831859C0; continue 'dispatch;
	// 831859B4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 831859B8: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 831859BC: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831859C0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831859C4: 4BFA82FD  bl 0x8312dcc0
	ctx.lr = 0x831859C8;
	sub_8312DCC0(ctx, base);
	// 831859C8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831859CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831859D0: 387D1088  addi r3, r29, 0x1088
	ctx.r[3].s64 = ctx.r[29].s64 + 4232;
	// 831859D4: 4800E4FD  bl 0x83193ed0
	ctx.lr = 0x831859D8;
	sub_83193ED0(ctx, base);
	// 831859D8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831859DC: 480227DC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831859E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831859E0 size=84
    let mut pc: u32 = 0x831859E0;
    'dispatch: loop {
        match pc {
            0x831859E0 => {
    //   block [0x831859E0..0x83185A34)
	// 831859E0: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 831859E4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831859E8: 396BC664  addi r11, r11, -0x399c
	ctx.r[11].s64 = ctx.r[11].s64 + -14748;
	// 831859EC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831859F0: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831859F4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 831859F8: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831859FC: 394A001F  addi r10, r10, 0x1f
	ctx.r[10].s64 = ctx.r[10].s64 + 31;
	// 83185A00: 554A0034  rlwinm r10, r10, 0, 0, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 83185A04: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83185A08: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83185A0C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83185A10: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83185A14: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83185A18: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83185A1C: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83185A20: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83185A24: 394A001F  addi r10, r10, 0x1f
	ctx.r[10].s64 = ctx.r[10].s64 + 31;
	// 83185A28: 554A0034  rlwinm r10, r10, 0, 0, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 83185A2C: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 83185A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185A38 size=60
    let mut pc: u32 = 0x83185A38;
    'dispatch: loop {
        match pc {
            0x83185A38 => {
    //   block [0x83185A38..0x83185A74)
	// 83185A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83185A40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185A44: 4BFA54ED  bl 0x8312af30
	ctx.lr = 0x83185A48;
	sub_8312AF30(ctx, base);
	// 83185A48: 4807E931  bl 0x83204378
	ctx.lr = 0x83185A4C;
	sub_83204378(ctx, base);
	// 83185A4C: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83185A50: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 83185A54: 386BC664  addi r3, r11, -0x399c
	ctx.r[3].s64 = ctx.r[11].s64 + -14748;
	// 83185A58: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83185A5C: 4800D62D  bl 0x83193088
	ctx.lr = 0x83185A60;
	sub_83193088(ctx, base);
	// 83185A60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83185A64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83185A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83185A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185A78 size=40
    let mut pc: u32 = 0x83185A78;
    'dispatch: loop {
        match pc {
            0x83185A78 => {
    //   block [0x83185A78..0x83185AA0)
	// 83185A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83185A80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185A84: 4807E90D  bl 0x83204390
	ctx.lr = 0x83185A88;
	sub_83204390(ctx, base);
	// 83185A88: 4BFA55E9  bl 0x8312b070
	ctx.lr = 0x83185A8C;
	sub_8312B070(ctx, base);
	// 83185A8C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83185A90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83185A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83185A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83185AA0 size=12
    let mut pc: u32 = 0x83185AA0;
    'dispatch: loop {
        match pc {
            0x83185AA0 => {
    //   block [0x83185AA0..0x83185AAC)
	// 83185AA0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 83185AA4: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 83185AA8: 480060A8  b 0x8318bb50
	sub_8318BB50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83185AB0 size=28
    let mut pc: u32 = 0x83185AB0;
    'dispatch: loop {
        match pc {
            0x83185AB0 => {
    //   block [0x83185AB0..0x83185ACC)
	// 83185AB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83185AB4: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 83185AB8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83185ABC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83185AC0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83185AC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83185AC8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185AD0 size=128
    let mut pc: u32 = 0x83185AD0;
    'dispatch: loop {
        match pc {
            0x83185AD0 => {
    //   block [0x83185AD0..0x83185B50)
	// 83185AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83185AD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83185ADC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83185AE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185AE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83185AE8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83185AEC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83185AF0: 419A0030  beq cr6, 0x83185b20
	if ctx.cr[6].eq {
	pc = 0x83185B20; continue 'dispatch;
	}
	// 83185AF4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83185AF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83185AFC: 4BFAF695  bl 0x83135190
	ctx.lr = 0x83185B00;
	sub_83135190(ctx, base);
	// 83185B00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83185B04: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83185B08: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83185B0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83185B10: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83185B14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83185B18: 4E800421  bctrl
	ctx.lr = 0x83185B1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83185B1C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83185B20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83185B24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83185B28: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83185B2C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83185B30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83185B34: 4E800421  bctrl
	ctx.lr = 0x83185B38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83185B38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83185B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83185B44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83185B48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83185B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185B50 size=140
    let mut pc: u32 = 0x83185B50;
    'dispatch: loop {
        match pc {
            0x83185B50 => {
    //   block [0x83185B50..0x83185BDC)
	// 83185B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185B54: 48022611  bl 0x831a8164
	ctx.lr = 0x83185B58;
	sub_831A8130(ctx, base);
	// 83185B58: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185B5C: 83C3180C  lwz r30, 0x180c(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6156 as u32) ) } as u64;
	// 83185B60: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83185B64: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83185B68: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83185B6C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83185B70: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83185B74: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83185B78: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83185B7C: 4BFFFF35  bl 0x83185ab0
	ctx.lr = 0x83185B80;
	sub_83185AB0(ctx, base);
	// 83185B80: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83185B84: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83185B88: 41980008  blt cr6, 0x83185b90
	if ctx.cr[6].lt {
	pc = 0x83185B90; continue 'dispatch;
	}
	// 83185B8C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 83185B90: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 83185B94: 616B9000  ori r11, r11, 0x9000
	ctx.r[11].u64 = ctx.r[11].u64 | 36864;
	// 83185B98: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83185B9C: 41980008  blt cr6, 0x83185ba4
	if ctx.cr[6].lt {
	pc = 0x83185BA4; continue 'dispatch;
	}
	// 83185BA0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 83185BA4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83185BA8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83185BAC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83185BB0: 48008DE9  bl 0x8318e998
	ctx.lr = 0x83185BB4;
	sub_8318E998(ctx, base);
	// 83185BB4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83185BB8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83185BBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83185BC0: 4BFFFF11  bl 0x83185ad0
	ctx.lr = 0x83185BC4;
	sub_83185AD0(ctx, base);
	// 83185BC4: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 83185BC8: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83185BCC: 917E0048  stw r11, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83185BD0: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83185BD4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83185BD8: 480225DC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185BE0 size=112
    let mut pc: u32 = 0x83185BE0;
    'dispatch: loop {
        match pc {
            0x83185BE0 => {
    //   block [0x83185BE0..0x83185C50)
	// 83185BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185BE4: 48022585  bl 0x831a8168
	ctx.lr = 0x83185BE8;
	sub_831A8130(ctx, base);
	// 83185BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185BEC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83185BF0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83185BF4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83185BF8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83185BFC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83185C00: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83185C04: 40990040  ble cr6, 0x83185c44
	if !ctx.cr[6].gt {
	pc = 0x83185C44; continue 'dispatch;
	}
	// 83185C08: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83185C0C: 38800012  li r4, 0x12
	ctx.r[4].s64 = 18;
	// 83185C10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83185C14: 4BFA773D  bl 0x8312d350
	ctx.lr = 0x83185C18;
	sub_8312D350(ctx, base);
	// 83185C18: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83185C1C: 409A0020  bne cr6, 0x83185c3c
	if !ctx.cr[6].eq {
	pc = 0x83185C3C; continue 'dispatch;
	}
	// 83185C20: 3BFF0012  addi r31, r31, 0x12
	ctx.r[31].s64 = ctx.r[31].s64 + 18;
	// 83185C24: 3BDE0012  addi r30, r30, 0x12
	ctx.r[30].s64 = ctx.r[30].s64 + 18;
	// 83185C28: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 83185C2C: 4198FFDC  blt cr6, 0x83185c08
	if ctx.cr[6].lt {
	pc = 0x83185C08; continue 'dispatch;
	}
	// 83185C30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83185C34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83185C38: 48022580  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83185C3C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83185C40: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83185C44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83185C48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83185C4C: 4802256C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83185C50 size=12
    let mut pc: u32 = 0x83185C50;
    'dispatch: loop {
        match pc {
            0x83185C50 => {
    //   block [0x83185C50..0x83185C5C)
	// 83185C50: 8163180C  lwz r11, 0x180c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6156 as u32) ) } as u64;
	// 83185C54: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83185C58: 4BFA8210  b 0x8312de68
	sub_8312DE68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185C60 size=200
    let mut pc: u32 = 0x83185C60;
    'dispatch: loop {
        match pc {
            0x83185C60 => {
    //   block [0x83185C60..0x83185D28)
	// 83185C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185C64: 480224F5  bl 0x831a8158
	ctx.lr = 0x83185C68;
	sub_831A8130(ctx, base);
	// 83185C68: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185C6C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83185C70: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 83185C74: 3B3D0024  addi r25, r29, 0x24
	ctx.r[25].s64 = ctx.r[29].s64 + 36;
	// 83185C78: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83185C7C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83185C80: 7F9D2A14  add r28, r29, r5
	ctx.r[28].u64 = ctx.r[29].u64 + ctx.r[5].u64;
	// 83185C84: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 83185C88: 7F1DC840  cmplw cr6, r29, r25
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[25].u32, &mut ctx.xer);
	// 83185C8C: 40980090  bge cr6, 0x83185d1c
	if !ctx.cr[6].lt {
	pc = 0x83185D1C; continue 'dispatch;
	}
	// 83185C90: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 83185C94: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83185C98: 40980084  bge cr6, 0x83185d1c
	if !ctx.cr[6].lt {
	pc = 0x83185D1C; continue 'dispatch;
	}
	// 83185C9C: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83185CA0: 2B0B0080  cmplwi cr6, r11, 0x80
	ctx.cr[6].compare_u32(ctx.r[11].u32, 128 as u32, &mut ctx.xer);
	// 83185CA4: 4098001C  bge cr6, 0x83185cc0
	if !ctx.cr[6].lt {
	pc = 0x83185CC0; continue 'dispatch;
	}
	// 83185CA8: 3BFF0012  addi r31, r31, 0x12
	ctx.r[31].s64 = ctx.r[31].s64 + 18;
	// 83185CAC: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83185CB0: 4198FFEC  blt cr6, 0x83185c9c
	if ctx.cr[6].lt {
	pc = 0x83185C9C; continue 'dispatch;
	}
	// 83185CB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83185CB8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83185CBC: 480224EC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83185CC0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83185CC4: 38800012  li r4, 0x12
	ctx.r[4].s64 = 18;
	// 83185CC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83185CCC: 4BFA7685  bl 0x8312d350
	ctx.lr = 0x83185CD0;
	sub_8312D350(ctx, base);
	// 83185CD0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83185CD4: 419A0014  beq cr6, 0x83185ce8
	if ctx.cr[6].eq {
	pc = 0x83185CE8; continue 'dispatch;
	}
	// 83185CD8: 7F1BF840  cmplw cr6, r27, r31
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83185CDC: 4098000C  bge cr6, 0x83185ce8
	if !ctx.cr[6].lt {
	pc = 0x83185CE8; continue 'dispatch;
	}
	// 83185CE0: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 83185CE4: 7FDAF378  mr r26, r30
	ctx.r[26].u64 = ctx.r[30].u64;
	// 83185CE8: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 83185CEC: 7F1EC840  cmplw cr6, r30, r25
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[25].u32, &mut ctx.xer);
	// 83185CF0: 4198FFA0  blt cr6, 0x83185c90
	if ctx.cr[6].lt {
	pc = 0x83185C90; continue 'dispatch;
	}
	// 83185CF4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 83185CF8: 409A0020  bne cr6, 0x83185d18
	if !ctx.cr[6].eq {
	pc = 0x83185D18; continue 'dispatch;
	}
	// 83185CFC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83185D00: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83185D04: 60840C0A  ori r4, r4, 0xc0a
	ctx.r[4].u64 = ctx.r[4].u64 | 3082;
	// 83185D08: 480017F1  bl 0x831874f8
	ctx.lr = 0x83185D0C;
	sub_831874F8(ctx, base);
	// 83185D0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83185D10: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83185D14: 48022494  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83185D18: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 83185D1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83185D20: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83185D24: 48022484  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185D28 size=52
    let mut pc: u32 = 0x83185D28;
    'dispatch: loop {
        match pc {
            0x83185D28 => {
    //   block [0x83185D28..0x83185D5C)
	// 83185D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83185D30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185D34: 8163180C  lwz r11, 0x180c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6156 as u32) ) } as u64;
	// 83185D38: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83185D3C: 4BFA7975  bl 0x8312d6b0
	ctx.lr = 0x83185D40;
	sub_8312D6B0(ctx, base);
	// 83185D40: 3963FFFD  addi r11, r3, -3
	ctx.r[11].s64 = ctx.r[3].s64 + -3;
	// 83185D44: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83185D48: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83185D4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83185D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83185D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185D60 size=80
    let mut pc: u32 = 0x83185D60;
    'dispatch: loop {
        match pc {
            0x83185D60 => {
    //   block [0x83185D60..0x83185DB0)
	// 83185D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83185D68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83185D6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83185D70: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185D74: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83185D78: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83185D7C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83185D80: 80831814  lwz r4, 0x1814(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6164 as u32) ) } as u64;
	// 83185D84: 480088FD  bl 0x8318e680
	ctx.lr = 0x83185D88;
	sub_8318E680(ctx, base);
	// 83185D88: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83185D8C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83185D90: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83185D94: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83185D98: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83185D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83185DA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83185DA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83185DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83185DB0 size=12
    let mut pc: u32 = 0x83185DB0;
    'dispatch: loop {
        match pc {
            0x83185DB0 => {
    //   block [0x83185DB0..0x83185DBC)
	// 83185DB0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83185DB4: 80831814  lwz r4, 0x1814(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6164 as u32) ) } as u64;
	// 83185DB8: 48008BD8  b 0x8318e990
	sub_8318E990(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185DC0 size=176
    let mut pc: u32 = 0x83185DC0;
    'dispatch: loop {
        match pc {
            0x83185DC0 => {
    //   block [0x83185DC0..0x83185E70)
	// 83185DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83185DC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83185DCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83185DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185DD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83185DD8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83185DDC: 83DF180C  lwz r30, 0x180c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6156 as u32) ) } as u64;
	// 83185DE0: 809F1814  lwz r4, 0x1814(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6164 as u32) ) } as u64;
	// 83185DE4: 480080B5  bl 0x8318de98
	ctx.lr = 0x83185DE8;
	sub_8318DE98(ctx, base);
	// 83185DE8: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83185DEC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83185DF0: 419A0068  beq cr6, 0x83185e58
	if ctx.cr[6].eq {
	pc = 0x83185E58; continue 'dispatch;
	}
	// 83185DF4: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83185DF8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83185DFC: 480086AD  bl 0x8318e4a8
	ctx.lr = 0x83185E00;
	sub_8318E4A8(ctx, base);
	// 83185E00: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83185E04: E87F09B8  ld r3, 0x9b8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2488 as u32) ) };
	// 83185E08: 48008319  bl 0x8318e120
	ctx.lr = 0x83185E0C;
	sub_8318E120(ctx, base);
	// 83185E0C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83185E10: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83185E14: E87F09C0  ld r3, 0x9c0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2496 as u32) ) };
	// 83185E18: F97F09B8  std r11, 0x9b8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2488 as u32), ctx.r[11].u64 ) };
	// 83185E1C: 48008305  bl 0x8318e120
	ctx.lr = 0x83185E20;
	sub_8318E120(ctx, base);
	// 83185E20: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83185E24: F87F09C0  std r3, 0x9c0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2496 as u32), ctx.r[3].u64 ) };
	// 83185E28: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83185E2C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83185E30: 48008679  bl 0x8318e4a8
	ctx.lr = 0x83185E34;
	sub_8318E4A8(ctx, base);
	// 83185E34: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83185E38: E87F09D0  ld r3, 0x9d0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2512 as u32) ) };
	// 83185E3C: 480082E5  bl 0x8318e120
	ctx.lr = 0x83185E40;
	sub_8318E120(ctx, base);
	// 83185E40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83185E44: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83185E48: E87F09D8  ld r3, 0x9d8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2520 as u32) ) };
	// 83185E4C: F97F09D0  std r11, 0x9d0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2512 as u32), ctx.r[11].u64 ) };
	// 83185E50: 480082D1  bl 0x8318e120
	ctx.lr = 0x83185E54;
	sub_8318E120(ctx, base);
	// 83185E54: F87F09D8  std r3, 0x9d8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2520 as u32), ctx.r[3].u64 ) };
	// 83185E58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83185E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83185E64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83185E68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83185E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83185E70 size=8
    let mut pc: u32 = 0x83185E70;
    'dispatch: loop {
        match pc {
            0x83185E70 => {
    //   block [0x83185E70..0x83185E78)
	// 83185E70: 80831814  lwz r4, 0x1814(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6164 as u32) ) } as u64;
	// 83185E74: 48008164  b 0x8318dfd8
	sub_8318DFD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83185E78 size=8
    let mut pc: u32 = 0x83185E78;
    'dispatch: loop {
        match pc {
            0x83185E78 => {
    //   block [0x83185E78..0x83185E80)
	// 83185E78: 80831818  lwz r4, 0x1818(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6168 as u32) ) } as u64;
	// 83185E7C: 4800815C  b 0x8318dfd8
	sub_8318DFD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83185E80 size=12
    let mut pc: u32 = 0x83185E80;
    'dispatch: loop {
        match pc {
            0x83185E80 => {
    //   block [0x83185E80..0x83185E8C)
	// 83185E80: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83185E84: 80831818  lwz r4, 0x1818(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6168 as u32) ) } as u64;
	// 83185E88: 48008138  b 0x8318dfc0
	sub_8318DFC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185E90 size=56
    let mut pc: u32 = 0x83185E90;
    'dispatch: loop {
        match pc {
            0x83185E90 => {
    //   block [0x83185E90..0x83185EC8)
	// 83185E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83185E98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185E9C: 4BFA7815  bl 0x8312d6b0
	ctx.lr = 0x83185EA0;
	sub_8312D6B0(ctx, base);
	// 83185EA0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83185EA4: 419A0010  beq cr6, 0x83185eb4
	if ctx.cr[6].eq {
	pc = 0x83185EB4; continue 'dispatch;
	}
	// 83185EA8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83185EAC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83185EB0: 409A0008  bne cr6, 0x83185eb8
	if !ctx.cr[6].eq {
	pc = 0x83185EB8; continue 'dispatch;
	}
	// 83185EB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83185EB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83185EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83185EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185EC8 size=88
    let mut pc: u32 = 0x83185EC8;
    'dispatch: loop {
        match pc {
            0x83185EC8 => {
    //   block [0x83185EC8..0x83185F20)
	// 83185EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83185ED0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83185ED4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83185ED8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185EDC: 83E3180C  lwz r31, 0x180c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6156 as u32) ) } as u64;
	// 83185EE0: 3880001B  li r4, 0x1b
	ctx.r[4].s64 = 27;
	// 83185EE4: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83185EE8: 4BFF3529  bl 0x83179410
	ctx.lr = 0x83185EEC;
	sub_83179410(ctx, base);
	// 83185EEC: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83185EF0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83185EF4: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 83185EF8: 419A0010  beq cr6, 0x83185f08
	if ctx.cr[6].eq {
	pc = 0x83185F08; continue 'dispatch;
	}
	// 83185EFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83185F00: 909F0044  stw r4, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[4].u32 ) };
	// 83185F04: 4BFA7B5D  bl 0x8312da60
	ctx.lr = 0x83185F08;
	sub_8312DA60(ctx, base);
	// 83185F08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83185F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83185F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83185F14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83185F18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83185F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83185F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83185F20 size=452
    let mut pc: u32 = 0x83185F20;
    'dispatch: loop {
        match pc {
            0x83185F20 => {
    //   block [0x83185F20..0x831860E4)
	// 83185F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83185F24: 48022249  bl 0x831a816c
	ctx.lr = 0x83185F28;
	sub_831A8130(ctx, base);
	// 83185F28: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83185F2C: 3D408340  lis r10, -0x7cc0
	ctx.r[10].s64 = -2092957696;
	// 83185F30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83185F34: 394AC664  addi r10, r10, -0x399c
	ctx.r[10].s64 = ctx.r[10].s64 + -14748;
	// 83185F38: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83185F3C: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 83185F40: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83185F44: 419A0188  beq cr6, 0x831860cc
	if ctx.cr[6].eq {
	pc = 0x831860CC; continue 'dispatch;
	}
	// 83185F48: 812A0018  lwz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 83185F4C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83185F50: 419A017C  beq cr6, 0x831860cc
	if ctx.cr[6].eq {
	pc = 0x831860CC; continue 'dispatch;
	}
	// 83185F54: 392B0008  addi r9, r11, 8
	ctx.r[9].s64 = ctx.r[11].s64 + 8;
	// 83185F58: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 83185F5C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83185F60: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83185F64: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83185F68: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83185F6C: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 83185F70: 4200FFF0  bdnz 0x83185f60
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83185F60; continue 'dispatch;
	}
	// 83185F74: 3D208318  lis r9, -0x7ce8
	ctx.r[9].s64 = -2095579136;
	// 83185F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83185F7C: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 83185F80: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 83185F84: 39295B50  addi r9, r9, 0x5b50
	ctx.r[9].s64 = ctx.r[9].s64 + 23376;
	// 83185F88: 3880003F  li r4, 0x3f
	ctx.r[4].s64 = 63;
	// 83185F8C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83185F90: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83185F94: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83185F98: 3BFD1088  addi r31, r29, 0x1088
	ctx.r[31].s64 = ctx.r[29].s64 + 4232;
	// 83185F9C: 910B0024  stw r8, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[8].u32 ) };
	// 83185FA0: 90EB0028  stw r7, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[7].u32 ) };
	// 83185FA4: 914B002C  stw r10, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 83185FA8: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 83185FAC: 914B0034  stw r10, 0x34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 83185FB0: 914B0038  stw r10, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 83185FB4: 912B003C  stw r9, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[9].u32 ) };
	// 83185FB8: 914B0040  stw r10, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 83185FBC: 910B0044  stw r8, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[8].u32 ) };
	// 83185FC0: 914B0048  stw r10, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 83185FC4: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 83185FC8: 4BFF3449  bl 0x83179410
	ctx.lr = 0x83185FCC;
	sub_83179410(ctx, base);
	// 83185FCC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83185FD0: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 83185FD4: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 83185FD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83185FDC: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 83185FE0: 3D60000F  lis r11, 0xf
	ctx.r[11].s64 = 983040;
	// 83185FE4: 617E4240  ori r30, r11, 0x4240
	ctx.r[30].u64 = ctx.r[11].u64 | 16960;
	// 83185FE8: FBC10058  std r30, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u64 ) };
	// 83185FEC: 4BFF3425  bl 0x83179410
	ctx.lr = 0x83185FF0;
	sub_83179410(ctx, base);
	// 83185FF0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83185FF4: FBC10068  std r30, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u64 ) };
	// 83185FF8: 38800041  li r4, 0x41
	ctx.r[4].s64 = 65;
	// 83185FFC: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 83186000: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83186004: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 83186008: 4BFF3409  bl 0x83179410
	ctx.lr = 0x8318600C;
	sub_83179410(ctx, base);
	// 8318600C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83186010: FBC10078  std r30, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[30].u64 ) };
	// 83186014: 38800042  li r4, 0x42
	ctx.r[4].s64 = 66;
	// 83186018: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8318601C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83186020: F9610070  std r11, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u64 ) };
	// 83186024: 4BFF33ED  bl 0x83179410
	ctx.lr = 0x83186028;
	sub_83179410(ctx, base);
	// 83186028: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318602C: FBC10088  std r30, 0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[30].u64 ) };
	// 83186030: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186034: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 83186038: F9610080  std r11, 0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u64 ) };
	// 8318603C: 4800E0FD  bl 0x83194138
	ctx.lr = 0x83186040;
	sub_83194138(ctx, base);
	// 83186040: 38800048  li r4, 0x48
	ctx.r[4].s64 = 72;
	// 83186044: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83186048: 4BFF33C9  bl 0x83179410
	ctx.lr = 0x8318604C;
	sub_83179410(ctx, base);
	// 8318604C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83186050: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186054: 4BCB9EB5  bl 0x82e3ff08
	ctx.lr = 0x83186058;
	sub_82E3FF08(ctx, base);
	// 83186058: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8318605C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186060: 4800DDD1  bl 0x83193e30
	ctx.lr = 0x83186064;
	sub_83193E30(ctx, base);
	// 83186064: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 83186068: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318606C: 4800DDDD  bl 0x83193e48
	ctx.lr = 0x83186070;
	sub_83193E48(ctx, base);
	// 83186070: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 83186074: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186078: 4800DDE9  bl 0x83193e60
	ctx.lr = 0x8318607C;
	sub_83193E60(ctx, base);
	// 8318607C: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 83186080: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186084: 4800DDF5  bl 0x83193e78
	ctx.lr = 0x83186088;
	sub_83193E78(ctx, base);
	// 83186088: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 8318608C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83186090: 4BFF3381  bl 0x83179410
	ctx.lr = 0x83186094;
	sub_83179410(ctx, base);
	// 83186094: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83186098: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318609C: 4800DDF5  bl 0x83193e90
	ctx.lr = 0x831860A0;
	sub_83193E90(ctx, base);
	// 831860A0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 831860A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831860A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831860AC: 4800DE25  bl 0x83193ed0
	ctx.lr = 0x831860B0;
	sub_83193ED0(ctx, base);
	// 831860B0: 3880003D  li r4, 0x3d
	ctx.r[4].s64 = 61;
	// 831860B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831860B8: 4BFF3359  bl 0x83179410
	ctx.lr = 0x831860BC;
	sub_83179410(ctx, base);
	// 831860BC: 4800DC65  bl 0x83193d20
	ctx.lr = 0x831860C0;
	sub_83193D20(ctx, base);
	// 831860C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831860C4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 831860C8: 480220F4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831860CC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 831860D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831860D4: 60840C06  ori r4, r4, 0xc06
	ctx.r[4].u64 = ctx.r[4].u64 | 3078;
	// 831860D8: 48001421  bl 0x831874f8
	ctx.lr = 0x831860DC;
	sub_831874F8(ctx, base);
	// 831860DC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 831860E0: 480220DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831860E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831860E8 size=16
    let mut pc: u32 = 0x831860E8;
    'dispatch: loop {
        match pc {
            0x831860E8 => {
    //   block [0x831860E8..0x831860F8)
	// 831860E8: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 831860EC: 396BA100  addi r11, r11, -0x5f00
	ctx.r[11].s64 = ctx.r[11].s64 + -24320;
	// 831860F0: 806B0208  lwz r3, 0x208(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(520 as u32) ) } as u64;
	// 831860F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831860F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831860F8 size=144
    let mut pc: u32 = 0x831860F8;
    'dispatch: loop {
        match pc {
            0x831860F8 => {
    //   block [0x831860F8..0x83186188)
	// 831860F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831860FC: 4802206D  bl 0x831a8168
	ctx.lr = 0x83186100;
	sub_831A8130(ctx, base);
	// 83186100: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83186104: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83186108: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8318610C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186110: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83186114: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83186118: 4BFA7599  bl 0x8312d6b0
	ctx.lr = 0x8318611C;
	sub_8312D6B0(ctx, base);
	// 8318611C: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83186120: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83186124: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83186128: 906BA33C  stw r3, -0x5cc4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-23748 as u32), ctx.r[3].u32 ) };
	// 8318612C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186130: 4BFA75D9  bl 0x8312d708
	ctx.lr = 0x83186134;
	sub_8312D708(ctx, base);
	// 83186134: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83186138: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8318613C: F9610070  std r11, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u64 ) };
	// 83186140: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83186144: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 83186148: F9610078  std r11, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u64 ) };
	// 8318614C: 4800DAFD  bl 0x83193c48
	ctx.lr = 0x83186150;
	sub_83193C48(ctx, base);
	// 83186150: F8610060  std r3, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u64 ) };
	// 83186154: 4800DBAD  bl 0x83193d00
	ctx.lr = 0x83186158;
	sub_83193D00(ctx, base);
	// 83186158: F8610068  std r3, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u64 ) };
	// 8318615C: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 83186160: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 83186164: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 83186168: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318616C: 4800E57D  bl 0x831946e8
	ctx.lr = 0x83186170;
	sub_831946E8(ctx, base);
	// 83186170: E9610080  ld r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) };
	// 83186174: E9410088  ld r10, 0x88(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) };
	// 83186178: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318617C: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83186180: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83186184: 48022034  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83186188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83186188 size=36
    let mut pc: u32 = 0x83186188;
    'dispatch: loop {
        match pc {
            0x83186188 => {
    //   block [0x83186188..0x831861AC)
	// 83186188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318618C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83186190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83186194: 4BFA7FE5  bl 0x8312e178
	ctx.lr = 0x83186198;
	sub_8312E178(ctx, base);
	// 83186198: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318619C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831861A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831861A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831861A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831861B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831861B0 size=20
    let mut pc: u32 = 0x831861B0;
    'dispatch: loop {
        match pc {
            0x831861B0 => {
    //   block [0x831861B0..0x831861C4)
	// 831861B0: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 831861B4: 396BA100  addi r11, r11, -0x5f00
	ctx.r[11].s64 = ctx.r[11].s64 + -24320;
	// 831861B8: 906B0208  stw r3, 0x208(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(520 as u32), ctx.r[3].u32 ) };
	// 831861BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831861C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831861C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831861C8 size=44
    let mut pc: u32 = 0x831861C8;
    'dispatch: loop {
        match pc {
            0x831861C8 => {
    //   block [0x831861C8..0x831861F4)
	// 831861C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831861CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831861D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831861D4: 8163180C  lwz r11, 0x180c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6156 as u32) ) } as u64;
	// 831861D8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831861DC: 4BFA749D  bl 0x8312d678
	ctx.lr = 0x831861E0;
	sub_8312D678(ctx, base);
	// 831861E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831861E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831861E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831861EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831861F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831861F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831861F8 size=72
    let mut pc: u32 = 0x831861F8;
    'dispatch: loop {
        match pc {
            0x831861F8 => {
    //   block [0x831861F8..0x83186240)
	// 831861F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831861FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83186200: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83186204: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83186208: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318620C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83186210: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83186214: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83186218: 4BFA7A61  bl 0x8312dc78
	ctx.lr = 0x8318621C;
	sub_8312DC78(ctx, base);
	// 8318621C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83186220: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83186224: 48004845  bl 0x8318aa68
	ctx.lr = 0x83186228;
	sub_8318AA68(ctx, base);
	// 83186228: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318622C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83186230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83186234: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83186238: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318623C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83186240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83186240 size=84
    let mut pc: u32 = 0x83186240;
    'dispatch: loop {
        match pc {
            0x83186240 => {
    //   block [0x83186240..0x83186294)
	// 83186240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83186244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83186248: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318624C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83186250: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83186254: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83186258: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8318625C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83186260: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83186264: 4BFF5BC5  bl 0x8317be28
	ctx.lr = 0x83186268;
	sub_8317BE28(ctx, base);
	// 83186268: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318626C: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83186270: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83186274: 48005565  bl 0x8318b7d8
	ctx.lr = 0x83186278;
	sub_8318B7D8(ctx, base);
	// 83186278: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8318627C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83186280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83186284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83186288: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318628C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83186290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83186298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83186298 size=12
    let mut pc: u32 = 0x83186298;
    'dispatch: loop {
        match pc {
            0x83186298 => {
    //   block [0x83186298..0x831862A4)
	// 83186298: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318629C: 60840C03  ori r4, r4, 0xc03
	ctx.r[4].u64 = ctx.r[4].u64 | 3075;
	// 831862A0: 48001258  b 0x831874f8
	sub_831874F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831862A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831862A8 size=20
    let mut pc: u32 = 0x831862A8;
    'dispatch: loop {
        match pc {
            0x831862A8 => {
    //   block [0x831862A8..0x831862BC)
	// 831862A8: 81631E28  lwz r11, 0x1e28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7720 as u32) ) } as u64;
	// 831862AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831862B0: 409A000C  bne cr6, 0x831862bc
	if !ctx.cr[6].eq {
		sub_831862BC(ctx, base);
		return;
	}
	// 831862B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831862B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831862BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831862BC size=20
    let mut pc: u32 = 0x831862BC;
    'dispatch: loop {
        match pc {
            0x831862BC => {
    //   block [0x831862BC..0x831862D0)
	// 831862BC: 8143180C  lwz r10, 0x180c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6156 as u32) ) } as u64;
	// 831862C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831862C4: 814A0040  lwz r10, 0x40(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 831862C8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831862CC: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831862D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831862D0 size=8
    let mut pc: u32 = 0x831862D0;
    'dispatch: loop {
        match pc {
            0x831862D0 => {
    //   block [0x831862D0..0x831862D8)
	// 831862D0: 386B0D0C  addi r3, r11, 0xd0c
	ctx.r[3].s64 = ctx.r[11].s64 + 3340;
	// 831862D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831862D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831862D8 size=124
    let mut pc: u32 = 0x831862D8;
    'dispatch: loop {
        match pc {
            0x831862D8 => {
    //   block [0x831862D8..0x83186354)
	// 831862D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831862DC: 48021E91  bl 0x831a816c
	ctx.lr = 0x831862E0;
	sub_831A8130(ctx, base);
	// 831862E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831862E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831862E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831862EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831862F0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831862F4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 831862F8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831862FC: 83BF180C  lwz r29, 0x180c(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6156 as u32) ) } as u64;
	// 83186300: 4BFFFA61  bl 0x83185d60
	ctx.lr = 0x83186304;
	sub_83185D60(ctx, base);
	// 83186304: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83186308: 409A0044  bne cr6, 0x8318634c
	if !ctx.cr[6].eq {
	pc = 0x8318634C; continue 'dispatch;
	}
	// 8318630C: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83186310: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 83186314: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83186318: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318631C: 90BE0000  stw r5, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 83186320: 817D003C  lwz r11, 0x3c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 83186324: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83186328: 4E800421  bctrl
	ctx.lr = 0x8318632C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318632C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186330: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83186334: 4BFFFA7D  bl 0x83185db0
	ctx.lr = 0x83186338;
	sub_83185DB0(ctx, base);
	// 83186338: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318633C: 409A0010  bne cr6, 0x8318634c
	if !ctx.cr[6].eq {
	pc = 0x8318634C; continue 'dispatch;
	}
	// 83186340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186344: 4BFFFA7D  bl 0x83185dc0
	ctx.lr = 0x83186348;
	sub_83185DC0(ctx, base);
	// 83186348: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318634C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83186350: 48021E6C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83186358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83186358 size=84
    let mut pc: u32 = 0x83186358;
    'dispatch: loop {
        match pc {
            0x83186358 => {
    //   block [0x83186358..0x831863AC)
	// 83186358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318635C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83186360: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83186364: 4BFFFF45  bl 0x831862a8
	ctx.lr = 0x83186368;
	sub_831862A8(ctx, base);
	// 83186368: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318636C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83186370: 409A0018  bne cr6, 0x83186388
	if !ctx.cr[6].eq {
	pc = 0x83186388; continue 'dispatch;
	}
	// 83186374: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83186378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318637C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83186380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83186384: 4E800020  blr
	return;
	// 83186388: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318638C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83186390: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83186394: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83186398: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318639C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831863A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831863A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831863A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831863B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831863B0 size=100
    let mut pc: u32 = 0x831863B0;
    'dispatch: loop {
        match pc {
            0x831863B0 => {
    //   block [0x831863B0..0x83186414)
	// 831863B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831863B4: 48021DB9  bl 0x831a816c
	ctx.lr = 0x831863B8;
	sub_831A8130(ctx, base);
	// 831863B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831863BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831863C0: 83DF1818  lwz r30, 0x1818(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6168 as u32) ) } as u64;
	// 831863C4: 83BF1814  lwz r29, 0x1814(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6164 as u32) ) } as u64;
	// 831863C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831863CC: 48007BD5  bl 0x8318dfa0
	ctx.lr = 0x831863D0;
	sub_8318DFA0(ctx, base);
	// 831863D0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831863D4: 419A0038  beq cr6, 0x8318640c
	if ctx.cr[6].eq {
	pc = 0x8318640C; continue 'dispatch;
	}
	// 831863D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831863DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831863E0: 48007BC1  bl 0x8318dfa0
	ctx.lr = 0x831863E4;
	sub_8318DFA0(ctx, base);
	// 831863E4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831863E8: 409A0024  bne cr6, 0x8318640c
	if !ctx.cr[6].eq {
	pc = 0x8318640C; continue 'dispatch;
	}
	// 831863EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831863F0: 4BFFF939  bl 0x83185d28
	ctx.lr = 0x831863F4;
	sub_83185D28(ctx, base);
	// 831863F4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831863F8: 419A0014  beq cr6, 0x8318640c
	if ctx.cr[6].eq {
	pc = 0x8318640C; continue 'dispatch;
	}
	// 831863FC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83186400: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83186404: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186408: 48007B81  bl 0x8318df88
	ctx.lr = 0x8318640C;
	sub_8318DF88(ctx, base);
	// 8318640C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83186410: 48021DAC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83186418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83186418 size=284
    let mut pc: u32 = 0x83186418;
    'dispatch: loop {
        match pc {
            0x83186418 => {
    //   block [0x83186418..0x83186534)
	// 83186418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318641C: 48021D41  bl 0x831a815c
	ctx.lr = 0x83186420;
	sub_831A8130(ctx, base);
	// 83186420: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83186424: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83186428: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 8318642C: 3B5F1088  addi r26, r31, 0x1088
	ctx.r[26].s64 = ctx.r[31].s64 + 4232;
	// 83186430: 837F180C  lwz r27, 0x180c(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6156 as u32) ) } as u64;
	// 83186434: 83BB0000  lwz r29, 0(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83186438: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8318643C: 4BFA7275  bl 0x8312d6b0
	ctx.lr = 0x83186440;
	sub_8312D6B0(ctx, base);
	// 83186440: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83186444: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83186448: 4BFA76E1  bl 0x8312db28
	ctx.lr = 0x8318644C;
	sub_8312DB28(ctx, base);
	// 8318644C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83186450: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83186454: 419A000C  beq cr6, 0x83186460
	if ctx.cr[6].eq {
	pc = 0x83186460; continue 'dispatch;
	}
	// 83186458: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318645C: 93CBC680  stw r30, -0x3980(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-14720 as u32), ctx.r[30].u32 ) };
	// 83186460: 3880001A  li r4, 0x1a
	ctx.r[4].s64 = 26;
	// 83186464: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186468: 4BFF2FA9  bl 0x83179410
	ctx.lr = 0x8318646C;
	sub_83179410(ctx, base);
	// 8318646C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83186470: 409A000C  bne cr6, 0x8318647c
	if !ctx.cr[6].eq {
	pc = 0x8318647C; continue 'dispatch;
	}
	// 83186474: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83186478: 48000044  b 0x831864bc
	pc = 0x831864BC; continue 'dispatch;
	// 8318647C: 2F1EFFFE  cmpwi cr6, r30, -2
	ctx.cr[6].compare_i32(ctx.r[30].s32, -2, &mut ctx.xer);
	// 83186480: 419A002C  beq cr6, 0x831864ac
	if ctx.cr[6].eq {
	pc = 0x831864AC; continue 'dispatch;
	}
	// 83186484: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 83186488: 419A0018  beq cr6, 0x831864a0
	if ctx.cr[6].eq {
	pc = 0x831864A0; continue 'dispatch;
	}
	// 8318648C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83186490: 419A002C  beq cr6, 0x831864bc
	if ctx.cr[6].eq {
	pc = 0x831864BC; continue 'dispatch;
	}
	// 83186494: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83186498: 60840C07  ori r4, r4, 0xc07
	ctx.r[4].u64 = ctx.r[4].u64 | 3079;
	// 8318649C: 48000018  b 0x831864b4
	pc = 0x831864B4; continue 'dispatch;
	// 831864A0: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 831864A4: 60840C08  ori r4, r4, 0xc08
	ctx.r[4].u64 = ctx.r[4].u64 | 3080;
	// 831864A8: 4800000C  b 0x831864b4
	pc = 0x831864B4; continue 'dispatch;
	// 831864AC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 831864B0: 60840C09  ori r4, r4, 0xc09
	ctx.r[4].u64 = ctx.r[4].u64 | 3081;
	// 831864B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831864B8: 48001041  bl 0x831874f8
	ctx.lr = 0x831864BC;
	sub_831874F8(ctx, base);
	// 831864BC: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 831864C0: 419A000C  beq cr6, 0x831864cc
	if ctx.cr[6].eq {
	pc = 0x831864CC; continue 'dispatch;
	}
	// 831864C4: 2F1C0005  cmpwi cr6, r28, 5
	ctx.cr[6].compare_i32(ctx.r[28].s32, 5, &mut ctx.xer);
	// 831864C8: 409A0018  bne cr6, 0x831864e0
	if !ctx.cr[6].eq {
	pc = 0x831864E0; continue 'dispatch;
	}
	// 831864CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831864D0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 831864D4: 4BEF8545  bl 0x8307ea18
	ctx.lr = 0x831864D8;
	sub_8307EA18(ctx, base);
	// 831864D8: 2F1C0005  cmpwi cr6, r28, 5
	ctx.cr[6].compare_i32(ctx.r[28].s32, 5, &mut ctx.xer);
	// 831864DC: 419A000C  beq cr6, 0x831864e8
	if ctx.cr[6].eq {
	pc = 0x831864E8; continue 'dispatch;
	}
	// 831864E0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 831864E4: 419A0010  beq cr6, 0x831864f4
	if ctx.cr[6].eq {
	pc = 0x831864F4; continue 'dispatch;
	}
	// 831864E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831864EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831864F0: 4BFFF991  bl 0x83185e80
	ctx.lr = 0x831864F4;
	sub_83185E80(ctx, base);
	// 831864F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831864F8: 4BFFF979  bl 0x83185e70
	ctx.lr = 0x831864FC;
	sub_83185E70(ctx, base);
	// 831864FC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83186500: 409A002C  bne cr6, 0x8318652c
	if !ctx.cr[6].eq {
	pc = 0x8318652C; continue 'dispatch;
	}
	// 83186504: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 83186508: 409A0024  bne cr6, 0x8318652c
	if !ctx.cr[6].eq {
	pc = 0x8318652C; continue 'dispatch;
	}
	// 8318650C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83186510: 4BFA7851  bl 0x8312dd60
	ctx.lr = 0x83186514;
	sub_8312DD60(ctx, base);
	// 83186514: 817B0048  lwz r11, 0x48(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 83186518: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318651C: 409A0010  bne cr6, 0x8318652c
	if !ctx.cr[6].eq {
	pc = 0x8318652C; continue 'dispatch;
	}
	// 83186520: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83186524: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186528: 4BFFF959  bl 0x83185e80
	ctx.lr = 0x8318652C;
	sub_83185E80(ctx, base);
	// 8318652C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83186530: 48021C7C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83186538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83186538 size=184
    let mut pc: u32 = 0x83186538;
    'dispatch: loop {
        match pc {
            0x83186538 => {
    //   block [0x83186538..0x831865F0)
	// 83186538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318653C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83186540: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83186544: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83186548: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318654C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 83186550: 4BFFFD59  bl 0x831862a8
	ctx.lr = 0x83186554;
	sub_831862A8(ctx, base);
	// 83186554: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83186558: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8318655C: 419A007C  beq cr6, 0x831865d8
	if ctx.cr[6].eq {
	pc = 0x831865D8; continue 'dispatch;
	}
	// 83186560: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83186564: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83186568: 409A0070  bne cr6, 0x831865d8
	if !ctx.cr[6].eq {
	pc = 0x831865D8; continue 'dispatch;
	}
	// 8318656C: 8169180C  lwz r11, 0x180c(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(6156 as u32) ) } as u64;
	// 83186570: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83186574: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83186578: 4BFFF919  bl 0x83185e90
	ctx.lr = 0x8318657C;
	sub_83185E90(ctx, base);
	// 8318657C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83186580: 419A0058  beq cr6, 0x831865d8
	if ctx.cr[6].eq {
	pc = 0x831865D8; continue 'dispatch;
	}
	// 83186584: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83186588: 4BFA7231  bl 0x8312d7b8
	ctx.lr = 0x8318658C;
	sub_8312D7B8(ctx, base);
	// 8318658C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83186590: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83186594: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83186598: 4BFA71A9  bl 0x8312d740
	ctx.lr = 0x8318659C;
	sub_8312D740(ctx, base);
	// 8318659C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831865A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831865A4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 831865A8: 4BFA7289  bl 0x8312d830
	ctx.lr = 0x831865AC;
	sub_8312D830(ctx, base);
	// 831865AC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831865B0: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831865B4: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 831865B8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 831865BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831865C0: 7D6A19D6  mullw r11, r10, r3
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[3].s32 as i64);
	// 831865C4: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831865C8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831865CC: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 831865D0: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 831865D4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831865D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831865DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831865E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831865E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831865E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831865EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831865F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831865F0 size=124
    let mut pc: u32 = 0x831865F0;
    'dispatch: loop {
        match pc {
            0x831865F0 => {
    //   block [0x831865F0..0x8318666C)
	// 831865F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831865F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831865F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831865FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83186600: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83186604: 4BFFCDDD  bl 0x831833e0
	ctx.lr = 0x83186608;
	sub_831833E0(ctx, base);
	// 83186608: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8318660C: 419A0018  beq cr6, 0x83186624
	if ctx.cr[6].eq {
	pc = 0x83186624; continue 'dispatch;
	}
	// 83186610: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83186614: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83186618: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318661C: 4BFA7E6D  bl 0x8312e488
	ctx.lr = 0x83186620;
	sub_8312E488(ctx, base);
	// 83186620: 48000008  b 0x83186628
	pc = 0x83186628; continue 'dispatch;
	// 83186624: 4BFFFAC5  bl 0x831860e8
	ctx.lr = 0x83186628;
	sub_831860E8(ctx, base);
	// 83186628: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318662C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83186630: 409A0018  bne cr6, 0x83186648
	if !ctx.cr[6].eq {
	pc = 0x83186648; continue 'dispatch;
	}
	// 83186634: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83186638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318663C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83186640: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83186644: 4E800020  blr
	return;
	// 83186648: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318664C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186650: 4BFA7471  bl 0x8312dac0
	ctx.lr = 0x83186654;
	sub_8312DAC0(ctx, base);
	// 83186654: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318665C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83186660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83186664: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83186668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83186670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83186670 size=132
    let mut pc: u32 = 0x83186670;
    'dispatch: loop {
        match pc {
            0x83186670 => {
    //   block [0x83186670..0x831866F4)
	// 83186670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83186674: 48021AF1  bl 0x831a8164
	ctx.lr = 0x83186678;
	sub_831A8130(ctx, base);
	// 83186678: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318667C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83186680: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83186684: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83186688: 83FE180C  lwz r31, 0x180c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(6156 as u32) ) } as u64;
	// 8318668C: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83186690: 4BFF51B9  bl 0x8317b848
	ctx.lr = 0x83186694;
	sub_8317B848(ctx, base);
	// 83186694: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83186698: 419A0050  beq cr6, 0x831866e8
	if ctx.cr[6].eq {
	pc = 0x831866E8; continue 'dispatch;
	}
	// 8318669C: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 831866A0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 831866A4: 409A0034  bne cr6, 0x831866d8
	if !ctx.cr[6].eq {
	pc = 0x831866D8; continue 'dispatch;
	}
	// 831866A8: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 831866AC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831866B0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831866B4: 387E1088  addi r3, r30, 0x1088
	ctx.r[3].s64 = ctx.r[30].s64 + 4232;
	// 831866B8: 4BFFFA41  bl 0x831860f8
	ctx.lr = 0x831866BC;
	sub_831860F8(ctx, base);
	// 831866BC: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 831866C0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831866C4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831866C8: 40980010  bge cr6, 0x831866d8
	if !ctx.cr[6].lt {
	pc = 0x831866D8; continue 'dispatch;
	}
	// 831866CC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831866D0: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 831866D4: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 831866D8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 831866DC: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831866E0: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 831866E4: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831866E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831866EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831866F0: 48021AC4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831866F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831866F8 size=168
    let mut pc: u32 = 0x831866F8;
    'dispatch: loop {
        match pc {
            0x831866F8 => {
    //   block [0x831866F8..0x831867A0)
	// 831866F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831866FC: 48021A71  bl 0x831a816c
	ctx.lr = 0x83186700;
	sub_831A8130(ctx, base);
	// 83186700: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83186704: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83186708: 817E180C  lwz r11, 0x180c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(6156 as u32) ) } as u64;
	// 8318670C: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83186710: 83AB0004  lwz r29, 4(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83186714: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83186718: 409A0010  bne cr6, 0x83186728
	if !ctx.cr[6].eq {
	pc = 0x83186728; continue 'dispatch;
	}
	// 8318671C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83186720: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83186724: 48021A98  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83186728: 3D408340  lis r10, -0x7cc0
	ctx.r[10].s64 = -2092957696;
	// 8318672C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83186730: 394AC664  addi r10, r10, -0x399c
	ctx.r[10].s64 = ctx.r[10].s64 + -14748;
	// 83186734: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 83186738: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8318673C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83186740: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83186744: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83186748: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8318674C: 4200FFF0  bdnz 0x8318673c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318673C; continue 'dispatch;
	}
	// 83186750: 4BFFCC91  bl 0x831833e0
	ctx.lr = 0x83186754;
	sub_831833E0(ctx, base);
	// 83186754: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83186758: 419A0010  beq cr6, 0x83186768
	if ctx.cr[6].eq {
	pc = 0x83186768; continue 'dispatch;
	}
	// 8318675C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186760: 4BFFFA29  bl 0x83186188
	ctx.lr = 0x83186764;
	sub_83186188(ctx, base);
	// 83186764: 48000014  b 0x83186778
	pc = 0x83186778; continue 'dispatch;
	// 83186768: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318676C: 4BFFFA5D  bl 0x831861c8
	ctx.lr = 0x83186770;
	sub_831861C8(ctx, base);
	// 83186770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186774: 4BFFFA3D  bl 0x831861b0
	ctx.lr = 0x83186778;
	sub_831861B0(ctx, base);
	// 83186778: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318677C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83186780: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83186784: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83186788: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318678C: 4E800421  bctrl
	ctx.lr = 0x83186790;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83186790: 4800D519  bl 0x83193ca8
	ctx.lr = 0x83186794;
	sub_83193CA8(ctx, base);
	// 83186794: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186798: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318679C: 48021A20  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831867A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831867A0 size=44
    let mut pc: u32 = 0x831867A0;
    'dispatch: loop {
        match pc {
            0x831867A0 => {
    //   block [0x831867A0..0x831867CC)
	// 831867A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831867A4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 831867A8: 388B1088  addi r4, r11, 0x1088
	ctx.r[4].s64 = ctx.r[11].s64 + 4232;
	// 831867AC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 831867B0: 814B180C  lwz r10, 0x180c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6156 as u32) ) } as u64;
	// 831867B4: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831867B8: 90AA002C  stw r5, 0x2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(44 as u32), ctx.r[5].u32 ) };
	// 831867BC: 409A0010  bne cr6, 0x831867cc
	if !ctx.cr[6].eq {
		sub_831867CC(ctx, base);
		return;
	}
	// 831867C0: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 831867C4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831867C8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831867CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831867CC size=8
    let mut pc: u32 = 0x831867CC;
    'dispatch: loop {
        match pc {
            0x831867CC => {
    //   block [0x831867CC..0x831867D4)
	// 831867CC: 4BFFFA2C  b 0x831861f8
	sub_831861F8(ctx, base);
	return;
	// 831867D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831867D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831867D8 size=36
    let mut pc: u32 = 0x831867D8;
    'dispatch: loop {
        match pc {
            0x831867D8 => {
    //   block [0x831867D8..0x831867FC)
	// 831867D8: 8163180C  lwz r11, 0x180c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6156 as u32) ) } as u64;
	// 831867DC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 831867E0: 38831088  addi r4, r3, 0x1088
	ctx.r[4].s64 = ctx.r[3].s64 + 4232;
	// 831867E4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 831867E8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831867EC: 409A0010  bne cr6, 0x831867fc
	if !ctx.cr[6].eq {
		sub_831867FC(ctx, base);
		return;
	}
	// 831867F0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 831867F4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831867F8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831867FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831867FC size=8
    let mut pc: u32 = 0x831867FC;
    'dispatch: loop {
        match pc {
            0x831867FC => {
    //   block [0x831867FC..0x83186804)
	// 831867FC: 4BFFF9FC  b 0x831861f8
	sub_831861F8(ctx, base);
	return;
	// 83186800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83186808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83186808 size=144
    let mut pc: u32 = 0x83186808;
    'dispatch: loop {
        match pc {
            0x83186808 => {
    //   block [0x83186808..0x83186898)
	// 83186808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318680C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83186810: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83186814: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83186818: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318681C: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83186820: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83186824: 4BFF2BED  bl 0x83179410
	ctx.lr = 0x83186828;
	sub_83179410(ctx, base);
	// 83186828: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318682C: 409A000C  bne cr6, 0x83186838
	if !ctx.cr[6].eq {
	pc = 0x83186838; continue 'dispatch;
	}
	// 83186830: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83186834: 4800004C  b 0x83186880
	pc = 0x83186880; continue 'dispatch;
	// 83186838: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318683C: 4BFFF63D  bl 0x83185e78
	ctx.lr = 0x83186840;
	sub_83185E78(ctx, base);
	// 83186840: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83186844: 419AFFEC  beq cr6, 0x83186830
	if ctx.cr[6].eq {
	pc = 0x83186830; continue 'dispatch;
	}
	// 83186848: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8318684C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186850: 4BFFFA89  bl 0x831862d8
	ctx.lr = 0x83186854;
	sub_831862D8(ctx, base);
	// 83186854: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83186858: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318685C: 4BFFFB55  bl 0x831863b0
	ctx.lr = 0x83186860;
	sub_831863B0(ctx, base);
	// 83186860: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186864: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83186868: 4BFFFBB1  bl 0x83186418
	ctx.lr = 0x8318686C;
	sub_83186418(ctx, base);
	// 8318686C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186870: 4BFFFCC9  bl 0x83186538
	ctx.lr = 0x83186874;
	sub_83186538(ctx, base);
	// 83186874: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186878: 4BFFF651  bl 0x83185ec8
	ctx.lr = 0x8318687C;
	sub_83185EC8(ctx, base);
	// 8318687C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83186880: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83186884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83186888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318688C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83186890: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83186894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83186898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83186898 size=536
    let mut pc: u32 = 0x83186898;
    'dispatch: loop {
        match pc {
            0x83186898 => {
    //   block [0x83186898..0x83186AB0)
	// 83186898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318689C: 480218B5  bl 0x831a8150
	ctx.lr = 0x831868A0;
	sub_831A8130(ctx, base);
	// 831868A0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831868A4: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 831868A8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 831868AC: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 831868B0: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 831868B4: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 831868B8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831868BC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 831868C0: 93560000  stw r26, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 831868C4: 83D9180C  lwz r30, 0x180c(r25)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(6156 as u32) ) } as u64;
	// 831868C8: 3BF90D88  addi r31, r25, 0xd88
	ctx.r[31].s64 = ctx.r[25].s64 + 3464;
	// 831868CC: 4BFFFA8D  bl 0x83186358
	ctx.lr = 0x831868D0;
	sub_83186358(ctx, base);
	// 831868D0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831868D4: 409A0044  bne cr6, 0x83186918
	if !ctx.cr[6].eq {
	pc = 0x83186918; continue 'dispatch;
	}
	// 831868D8: 83810050  lwz r28, 0x50(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831868DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831868E0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831868E4: 4BFF4E6D  bl 0x8317b750
	ctx.lr = 0x831868E8;
	sub_8317B750(ctx, base);
	// 831868E8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 831868EC: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 831868F0: 419801B8  blt cr6, 0x83186aa8
	if ctx.cr[6].lt {
	pc = 0x83186AA8; continue 'dispatch;
	}
	// 831868F4: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 831868F8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 831868FC: 4BFF2B15  bl 0x83179410
	ctx.lr = 0x83186900;
	sub_83179410(ctx, base);
	// 83186900: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83186904: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186908: 409A0024  bne cr6, 0x8318692c
	if !ctx.cr[6].eq {
	pc = 0x8318692C; continue 'dispatch;
	}
	// 8318690C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83186910: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83186914: 4BFF4F05  bl 0x8317b818
	ctx.lr = 0x83186918;
	sub_8317B818(ctx, base);
	// 83186918: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 8318691C: 396B5B50  addi r11, r11, 0x5b50
	ctx.r[11].s64 = ctx.r[11].s64 + 23376;
	// 83186920: 917E003C  stw r11, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 83186924: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83186928: 48021878  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
	// 8318692C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83186930: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83186934: 4BFF4E65  bl 0x8317b798
	ctx.lr = 0x83186938;
	sub_8317B798(ctx, base);
	// 83186938: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8318693C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83186940: 41980168  blt cr6, 0x83186aa8
	if ctx.cr[6].lt {
	pc = 0x83186AA8; continue 'dispatch;
	}
	// 83186944: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83186948: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8318694C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186950: 4BFF4EC9  bl 0x8317b818
	ctx.lr = 0x83186954;
	sub_8317B818(ctx, base);
	// 83186954: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83186958: 7D6BE850  subf r11, r11, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 8318695C: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 83186960: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83186964: 419800D8  blt cr6, 0x83186a3c
	if ctx.cr[6].lt {
	pc = 0x83186A3C; continue 'dispatch;
	}
	// 83186968: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 8318696C: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 83186970: 7D4B0194  addze r10, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[10].s64 = tmp.s64;
	// 83186974: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83186978: 7D4A59D6  mullw r10, r10, r11
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8318697C: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83186980: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83186984: 555F083C  slwi r31, r10, 1
	ctx.r[31].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 83186988: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8318698C: 40990064  ble cr6, 0x831869f0
	if !ctx.cr[6].gt {
	pc = 0x831869F0; continue 'dispatch;
	}
	// 83186990: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83186994: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83186998: 555C083C  slwi r28, r10, 1
	ctx.r[28].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 8318699C: 7D57E3D6  divw r10, r23, r28
	ctx.r[10].s32 = ctx.r[23].s32 / ctx.r[28].s32;
	// 831869A0: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 831869A4: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831869A8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831869AC: 557D083C  slwi r29, r11, 1
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 831869B0: 7F1DF800  cmpw cr6, r29, r31
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[31].s32, &mut ctx.xer);
	// 831869B4: 41980008  blt cr6, 0x831869bc
	if ctx.cr[6].lt {
	pc = 0x831869BC; continue 'dispatch;
	}
	// 831869B8: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 831869BC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831869C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831869C4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 831869C8: 4BFFF219  bl 0x83185be0
	ctx.lr = 0x831869CC;
	sub_83185BE0(ctx, base);
	// 831869CC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 831869D0: 7D7DF850  subf r11, r29, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[29].s64;
	// 831869D4: 815E0038  lwz r10, 0x38(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 831869D8: 7D3AE3D6  divw r9, r26, r28
	ctx.r[9].s32 = ctx.r[26].s32 / ctx.r[28].s32;
	// 831869DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831869E0: 552B2834  slwi r11, r9, 5
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831869E4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831869E8: 917E0038  stw r11, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 831869EC: 41990030  bgt cr6, 0x83186a1c
	if ctx.cr[6].gt {
	pc = 0x83186A1C; continue 'dispatch;
	}
	// 831869F0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831869F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831869F8: 419A0024  beq cr6, 0x83186a1c
	if ctx.cr[6].eq {
	pc = 0x83186A1C; continue 'dispatch;
	}
	// 831869FC: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 83186A00: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 83186A04: 396B5B50  addi r11, r11, 0x5b50
	ctx.r[11].s64 = ctx.r[11].s64 + 23376;
	// 83186A08: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 83186A0C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83186A10: 917E003C  stw r11, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 83186A14: 4BFA693D  bl 0x8312d350
	ctx.lr = 0x83186A18;
	sub_8312D350(ctx, base);
	// 83186A18: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83186A1C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83186A20: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83186A24: 419A0070  beq cr6, 0x83186a94
	if ctx.cr[6].eq {
	pc = 0x83186A94; continue 'dispatch;
	}
	// 83186A28: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83186A2C: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83186A30: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83186A34: 4BFF2A25  bl 0x83179458
	ctx.lr = 0x83186A38;
	sub_83179458(ctx, base);
	// 83186A38: 4800005C  b 0x83186a94
	pc = 0x83186A94; continue 'dispatch;
	// 83186A3C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83186A40: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83186A44: 419A0050  beq cr6, 0x83186a94
	if ctx.cr[6].eq {
	pc = 0x83186A94; continue 'dispatch;
	}
	// 83186A48: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 83186A4C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83186A50: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 83186A54: 557F2834  slwi r31, r11, 5
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 83186A58: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83186A5C: 4099002C  ble cr6, 0x83186a88
	if !ctx.cr[6].gt {
	pc = 0x83186A88; continue 'dispatch;
	}
	// 83186A60: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83186A64: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83186A68: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83186A6C: 4BFFF1E5  bl 0x83185c50
	ctx.lr = 0x83186A70;
	sub_83185C50(ctx, base);
	// 83186A70: 815E0038  lwz r10, 0x38(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83186A74: 7D63F850  subf r11, r3, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 83186A78: 7D435050  subf r10, r3, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[3].s64;
	// 83186A7C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83186A80: 915E0038  stw r10, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 83186A84: 41990010  bgt cr6, 0x83186a94
	if ctx.cr[6].gt {
	pc = 0x83186A94; continue 'dispatch;
	}
	// 83186A88: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 83186A8C: 396B5B50  addi r11, r11, 0x5b50
	ctx.r[11].s64 = ctx.r[11].s64 + 23376;
	// 83186A90: 917E003C  stw r11, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 83186A94: 93560000  stw r26, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 83186A98: 7F4B07B4  extsw r11, r26
	ctx.r[11].s64 = ctx.r[26].s32 as i64;
	// 83186A9C: E95909C8  ld r10, 0x9c8(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[25].u32.wrapping_add(2504 as u32) ) };
	// 83186AA0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83186AA4: F97909C8  std r11, 0x9c8(r25)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[25].u32.wrapping_add(2504 as u32), ctx.r[11].u64 ) };
	// 83186AA8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83186AAC: 480216F4  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83186AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83186AB0 size=164
    let mut pc: u32 = 0x83186AB0;
    'dispatch: loop {
        match pc {
            0x83186AB0 => {
    //   block [0x83186AB0..0x83186B54)
	// 83186AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83186AB4: 480216B1  bl 0x831a8164
	ctx.lr = 0x83186AB8;
	sub_831A8130(ctx, base);
	// 83186AB8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83186ABC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83186AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83186AC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83186AC8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83186ACC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83186AD0: 2F1E0120  cmpwi cr6, r30, 0x120
	ctx.cr[6].compare_i32(ctx.r[30].s32, 288, &mut ctx.xer);
	// 83186AD4: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83186AD8: 837F180C  lwz r27, 0x180c(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6156 as u32) ) } as u64;
	// 83186ADC: 41980070  blt cr6, 0x83186b4c
	if ctx.cr[6].lt {
	pc = 0x83186B4C; continue 'dispatch;
	}
	// 83186AE0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83186AE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83186AE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83186AEC: 4BFA67DD  bl 0x8312d2c8
	ctx.lr = 0x83186AF0;
	sub_8312D2C8(ctx, base);
	// 83186AF0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83186AF4: 419A000C  beq cr6, 0x83186b00
	if ctx.cr[6].eq {
	pc = 0x83186B00; continue 'dispatch;
	}
	// 83186AF8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83186AFC: 48000030  b 0x83186b2c
	pc = 0x83186B2C; continue 'dispatch;
	// 83186B00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186B04: 4800874D  bl 0x8318f250
	ctx.lr = 0x83186B08;
	sub_8318F250(ctx, base);
	// 83186B08: 2F03006C  cmpwi cr6, r3, 0x6c
	ctx.cr[6].compare_i32(ctx.r[3].s32, 108, &mut ctx.xer);
	// 83186B0C: 4198000C  blt cr6, 0x83186b18
	if ctx.cr[6].lt {
	pc = 0x83186B18; continue 'dispatch;
	}
	// 83186B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83186B14: 48000018  b 0x83186b2c
	pc = 0x83186B2C; continue 'dispatch;
	// 83186B18: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83186B1C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83186B20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186B24: 4BFFF13D  bl 0x83185c60
	ctx.lr = 0x83186B28;
	sub_83185C60(ctx, base);
	// 83186B28: 7D7D1850  subf r11, r29, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[29].s64;
	// 83186B2C: 3D408318  lis r10, -0x7ce8
	ctx.r[10].s64 = -2095579136;
	// 83186B30: 392A6898  addi r9, r10, 0x6898
	ctx.r[9].s64 = ctx.r[10].s64 + 26776;
	// 83186B34: 7D6A07B4  extsw r10, r11
	ctx.r[10].s64 = ctx.r[11].s32 as i64;
	// 83186B38: 913B003C  stw r9, 0x3c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(60 as u32), ctx.r[9].u32 ) };
	// 83186B3C: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83186B40: E97F09C8  ld r11, 0x9c8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2504 as u32) ) };
	// 83186B44: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83186B48: F97F09C8  std r11, 0x9c8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2504 as u32), ctx.r[11].u64 ) };
	// 83186B4C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83186B50: 48021664  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83186B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83186B58 size=264
    let mut pc: u32 = 0x83186B58;
    'dispatch: loop {
        match pc {
            0x83186B58 => {
    //   block [0x83186B58..0x83186C60)
	// 83186B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83186B5C: 48021609  bl 0x831a8164
	ctx.lr = 0x83186B60;
	sub_831A8130(ctx, base);
	// 83186B60: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83186B64: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83186B68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83186B6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83186B70: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83186B74: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83186B78: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83186B7C: 839E180C  lwz r28, 0x180c(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(6156 as u32) ) } as u64;
	// 83186B80: 480086D1  bl 0x8318f250
	ctx.lr = 0x83186B84;
	sub_8318F250(ctx, base);
	// 83186B84: 2F03006C  cmpwi cr6, r3, 0x6c
	ctx.cr[6].compare_i32(ctx.r[3].s32, 108, &mut ctx.xer);
	// 83186B88: 41980018  blt cr6, 0x83186ba0
	if ctx.cr[6].lt {
	pc = 0x83186BA0; continue 'dispatch;
	}
	// 83186B8C: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 83186B90: 396B6AB0  addi r11, r11, 0x6ab0
	ctx.r[11].s64 = ctx.r[11].s64 + 27312;
	// 83186B94: 917C003C  stw r11, 0x3c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 83186B98: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83186B9C: 48021618  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83186BA0: 38BFFFEE  addi r5, r31, -0x12
	ctx.r[5].s64 = ctx.r[31].s64 + -18;
	// 83186BA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83186BA8: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83186BAC: 4099005C  ble cr6, 0x83186c08
	if !ctx.cr[6].gt {
	pc = 0x83186C08; continue 'dispatch;
	}
	// 83186BB0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83186BB4: 38CBB1C8  addi r6, r11, -0x4e38
	ctx.r[6].s64 = ctx.r[11].s64 + -20024;
	// 83186BB8: 7D69EA14  add r11, r9, r29
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[29].u64;
	// 83186BBC: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 83186BC0: 38EB0012  addi r7, r11, 0x12
	ctx.r[7].s64 = ctx.r[11].s64 + 18;
	// 83186BC4: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83186BC8: 888A0000  lbz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83186BCC: 7D044051  subf. r8, r4, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[4].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83186BD0: 40820014  bne 0x83186be4
	if !ctx.cr[0].eq {
	pc = 0x83186BE4; continue 'dispatch;
	}
	// 83186BD4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83186BD8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83186BDC: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 83186BE0: 409AFFE4  bne cr6, 0x83186bc4
	if !ctx.cr[6].eq {
	pc = 0x83186BC4; continue 'dispatch;
	}
	// 83186BE4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83186BE8: 409A0014  bne cr6, 0x83186bfc
	if !ctx.cr[6].eq {
	pc = 0x83186BFC; continue 'dispatch;
	}
	// 83186BEC: 39290012  addi r9, r9, 0x12
	ctx.r[9].s64 = ctx.r[9].s64 + 18;
	// 83186BF0: 7F092800  cmpw cr6, r9, r5
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[5].s32, &mut ctx.xer);
	// 83186BF4: 4198FFC4  blt cr6, 0x83186bb8
	if ctx.cr[6].lt {
	pc = 0x83186BB8; continue 'dispatch;
	}
	// 83186BF8: 48000010  b 0x83186c08
	pc = 0x83186C08; continue 'dispatch;
	// 83186BFC: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 83186C00: 396B6AB0  addi r11, r11, 0x6ab0
	ctx.r[11].s64 = ctx.r[11].s64 + 27312;
	// 83186C04: 917C003C  stw r11, 0x3c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 83186C08: 913B0000  stw r9, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83186C0C: 7D2B07B4  extsw r11, r9
	ctx.r[11].s64 = ctx.r[9].s32 as i64;
	// 83186C10: E95E09C8  ld r10, 0x9c8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(2504 as u32) ) };
	// 83186C14: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83186C18: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83186C1C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83186C20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83186C24: F97E09C8  std r11, 0x9c8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(2504 as u32), ctx.r[11].u64 ) };
	// 83186C28: 4BFFF731  bl 0x83186358
	ctx.lr = 0x83186C2C;
	sub_83186358(ctx, base);
	// 83186C2C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83186C30: 409A0028  bne cr6, 0x83186c58
	if !ctx.cr[6].eq {
	pc = 0x83186C58; continue 'dispatch;
	}
	// 83186C34: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83186C38: 815C0038  lwz r10, 0x38(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(56 as u32) ) } as u64;
	// 83186C3C: 55681838  slwi r8, r11, 3
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83186C40: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 83186C44: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83186C48: 7D695BD6  divw r11, r9, r11
	ctx.r[11].s32 = ctx.r[9].s32 / ctx.r[11].s32;
	// 83186C4C: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83186C50: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83186C54: 917C0038  stw r11, 0x38(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83186C58: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83186C5C: 48021558  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83186C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83186C60 size=72
    let mut pc: u32 = 0x83186C60;
    'dispatch: loop {
        match pc {
            0x83186C60 => {
    //   block [0x83186C60..0x83186CA8)
	// 83186C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83186C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83186C68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83186C6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83186C70: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 83186C74: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83186C78: 388B6670  addi r4, r11, 0x6670
	ctx.r[4].s64 = ctx.r[11].s64 + 26224;
	// 83186C7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83186C80: 4BFF4D31  bl 0x8317b9b0
	ctx.lr = 0x83186C84;
	sub_8317B9B0(ctx, base);
	// 83186C84: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83186C88: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83186C8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186C90: 4BFF27C9  bl 0x83179458
	ctx.lr = 0x83186C94;
	sub_83179458(ctx, base);
	// 83186C94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83186C98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83186C9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83186CA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83186CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83186CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83186CA8 size=40
    let mut pc: u32 = 0x83186CA8;
    'dispatch: loop {
        match pc {
            0x83186CA8 => {
    //   block [0x83186CA8..0x83186CD0)
	// 83186CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83186CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83186CB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83186CB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83186CB8: 4BFFFAE9  bl 0x831867a0
	ctx.lr = 0x83186CBC;
	sub_831867A0(ctx, base);
	// 83186CBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83186CC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83186CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83186CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83186CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83186CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83186CD0 size=220
    let mut pc: u32 = 0x83186CD0;
    'dispatch: loop {
        match pc {
            0x83186CD0 => {
    //   block [0x83186CD0..0x83186DAC)
	// 83186CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83186CD4: 48021491  bl 0x831a8164
	ctx.lr = 0x83186CD8;
	sub_831A8130(ctx, base);
	// 83186CD8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83186CDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83186CE0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83186CE4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83186CE8: 83DF180C  lwz r30, 0x180c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6156 as u32) ) } as u64;
	// 83186CEC: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83186CF0: 4198009C  blt cr6, 0x83186d8c
	if ctx.cr[6].lt {
	pc = 0x83186D8C; continue 'dispatch;
	}
	// 83186CF4: 419A0080  beq cr6, 0x83186d74
	if ctx.cr[6].eq {
	pc = 0x83186D74; continue 'dispatch;
	}
	// 83186CF8: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 83186CFC: 409800A4  bge cr6, 0x83186da0
	if !ctx.cr[6].lt {
	pc = 0x83186DA0; continue 'dispatch;
	}
	// 83186D00: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83186D04: 4BFFF18D  bl 0x83185e90
	ctx.lr = 0x83186D08;
	sub_83185E90(ctx, base);
	// 83186D08: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83186D0C: 419A0094  beq cr6, 0x83186da0
	if ctx.cr[6].eq {
	pc = 0x83186DA0; continue 'dispatch;
	}
	// 83186D10: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83186D14: 4BFA6AA5  bl 0x8312d7b8
	ctx.lr = 0x83186D18;
	sub_8312D7B8(ctx, base);
	// 83186D18: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83186D1C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83186D20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186D24: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83186D28: 4BFFF519  bl 0x83186240
	ctx.lr = 0x83186D2C;
	sub_83186240(ctx, base);
	// 83186D2C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83186D30: 83610050  lwz r27, 0x50(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83186D34: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83186D38: 7F9B5A14  add r28, r27, r11
	ctx.r[28].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 83186D3C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83186D40: 4BFA7669  bl 0x8312e3a8
	ctx.lr = 0x83186D44;
	sub_8312E3A8(ctx, base);
	// 83186D44: 7D63E050  subf r11, r3, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[3].s64;
	// 83186D48: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 83186D4C: 387F1088  addi r3, r31, 0x1088
	ctx.r[3].s64 = ctx.r[31].s64 + 4232;
	// 83186D50: 917E0030  stw r11, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83186D54: 7F6B07B4  extsw r11, r27
	ctx.r[11].s64 = ctx.r[27].s32 as i64;
	// 83186D58: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 83186D5C: 7FAB07B4  extsw r11, r29
	ctx.r[11].s64 = ctx.r[29].s32 as i64;
	// 83186D60: F9610068  std r11, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u64 ) };
	// 83186D64: 4800D13D  bl 0x83193ea0
	ctx.lr = 0x83186D68;
	sub_83193EA0(ctx, base);
	// 83186D68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83186D6C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83186D70: 48021444  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83186D74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83186D78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186D7C: 4BFFFA5D  bl 0x831867d8
	ctx.lr = 0x83186D80;
	sub_831867D8(ctx, base);
	// 83186D80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83186D84: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83186D88: 4802142C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83186D8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83186D90: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83186D94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186D98: 917E0030  stw r11, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83186D9C: 4BFFFA3D  bl 0x831867d8
	ctx.lr = 0x83186DA0;
	sub_831867D8(ctx, base);
	// 83186DA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83186DA4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83186DA8: 4802140C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83186DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83186DB0 size=164
    let mut pc: u32 = 0x83186DB0;
    'dispatch: loop {
        match pc {
            0x83186DB0 => {
    //   block [0x83186DB0..0x83186E54)
	// 83186DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83186DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83186DB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83186DBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83186DC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83186DC4: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 83186DC8: 4BFFF4E1  bl 0x831862a8
	ctx.lr = 0x83186DCC;
	sub_831862A8(ctx, base);
	// 83186DCC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83186DD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83186DD4: 419A0064  beq cr6, 0x83186e38
	if ctx.cr[6].eq {
	pc = 0x83186E38; continue 'dispatch;
	}
	// 83186DD8: 83E9180C  lwz r31, 0x180c(r9)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(6156 as u32) ) } as u64;
	// 83186DDC: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83186DE0: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83186DE4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83186DE8: 409A0050  bne cr6, 0x83186e38
	if !ctx.cr[6].eq {
	pc = 0x83186E38; continue 'dispatch;
	}
	// 83186DEC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83186DF0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83186DF4: 419A0044  beq cr6, 0x83186e38
	if ctx.cr[6].eq {
	pc = 0x83186E38; continue 'dispatch;
	}
	// 83186DF8: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 83186DFC: 80CB0014  lwz r6, 0x14(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83186E00: 80AB0010  lwz r5, 0x10(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83186E04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83186E08: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83186E0C: 816AA338  lwz r11, -0x5cc8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-23752 as u32) ) } as u64;
	// 83186E10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83186E14: 4E800421  bctrl
	ctx.lr = 0x83186E18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83186E18: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83186E1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83186E20: 4BFA6F79  bl 0x8312dd98
	ctx.lr = 0x83186E24;
	sub_8312DD98(ctx, base);
	// 83186E24: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 83186E28: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83186E2C: 396B6B58  addi r11, r11, 0x6b58
	ctx.r[11].s64 = ctx.r[11].s64 + 27480;
	// 83186E30: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 83186E34: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 83186E38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83186E3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83186E40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83186E44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83186E48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83186E4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83186E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83186E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83186E58 size=144
    let mut pc: u32 = 0x83186E58;
    'dispatch: loop {
        match pc {
            0x83186E58 => {
    //   block [0x83186E58..0x83186EE8)
	// 83186E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83186E5C: 4802130D  bl 0x831a8168
	ctx.lr = 0x83186E60;
	sub_831A8130(ctx, base);
	// 83186E60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83186E64: 3F808345  lis r28, -0x7cbb
	ctx.r[28].s64 = -2092630016;
	// 83186E68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83186E6C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83186E70: 3BEBE270  addi r31, r11, -0x1d90
	ctx.r[31].s64 = ctx.r[11].s64 + -7568;
	// 83186E74: 807CA32C  lwz r3, -0x5cd4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83186E78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83186E7C: 419A001C  beq cr6, 0x83186e98
	if ctx.cr[6].eq {
	pc = 0x83186E98; continue 'dispatch;
	}
	// 83186E80: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83186E84: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 83186E88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83186E8C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83186E90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83186E94: 4E800421  bctrl
	ctx.lr = 0x83186E98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83186E98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83186E9C: 4BFFF96D  bl 0x83186808
	ctx.lr = 0x83186EA0;
	sub_83186808(ctx, base);
	// 83186EA0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83186EA4: 807CA32C  lwz r3, -0x5cd4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83186EA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83186EAC: 419A0030  beq cr6, 0x83186edc
	if ctx.cr[6].eq {
	pc = 0x83186EDC; continue 'dispatch;
	}
	// 83186EB0: 397E09B8  addi r11, r30, 0x9b8
	ctx.r[11].s64 = ctx.r[30].s64 + 2488;
	// 83186EB4: 389F006C  addi r4, r31, 0x6c
	ctx.r[4].s64 = ctx.r[31].s64 + 108;
	// 83186EB8: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83186EBC: 397E09C0  addi r11, r30, 0x9c0
	ctx.r[11].s64 = ctx.r[30].s64 + 2496;
	// 83186EC0: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83186EC4: 397E09C8  addi r11, r30, 0x9c8
	ctx.r[11].s64 = ctx.r[30].s64 + 2504;
	// 83186EC8: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 83186ECC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83186ED0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83186ED4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83186ED8: 4E800421  bctrl
	ctx.lr = 0x83186EDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83186EDC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83186EE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83186EE4: 480212D4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83186EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83186EE8 size=296
    let mut pc: u32 = 0x83186EE8;
    'dispatch: loop {
        match pc {
            0x83186EE8 => {
    //   block [0x83186EE8..0x83187010)
	// 83186EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83186EEC: 4802127D  bl 0x831a8168
	ctx.lr = 0x83186EF0;
	sub_831A8130(ctx, base);
	// 83186EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83186EF4: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83186EF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83186EFC: 4BFF2515  bl 0x83179410
	ctx.lr = 0x83186F00;
	sub_83179410(ctx, base);
	// 83186F00: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83186F04: 419A0100  beq cr6, 0x83187004
	if ctx.cr[6].eq {
	pc = 0x83187004; continue 'dispatch;
	}
	// 83186F08: 3BBF1D20  addi r29, r31, 0x1d20
	ctx.r[29].s64 = ctx.r[31].s64 + 7456;
	// 83186F0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186F10: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83186F14: 3B9D0008  addi r28, r29, 8
	ctx.r[28].s64 = ctx.r[29].s64 + 8;
	// 83186F18: 93BF180C  stw r29, 0x180c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(6156 as u32), ctx.r[29].u32 ) };
	// 83186F1C: 4BFFF005  bl 0x83185f20
	ctx.lr = 0x83186F20;
	sub_83185F20(ctx, base);
	// 83186F20: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83186F24: 409A00E4  bne cr6, 0x83187008
	if !ctx.cr[6].eq {
	pc = 0x83187008; continue 'dispatch;
	}
	// 83186F28: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83186F2C: 4BFFF6C5  bl 0x831865f0
	ctx.lr = 0x83186F30;
	sub_831865F0(ctx, base);
	// 83186F30: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83186F34: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83186F38: 409A0018  bne cr6, 0x83186f50
	if !ctx.cr[6].eq {
	pc = 0x83186F50; continue 'dispatch;
	}
	// 83186F3C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83186F40: 60840C04  ori r4, r4, 0xc04
	ctx.r[4].u64 = ctx.r[4].u64 | 3076;
	// 83186F44: 480005B5  bl 0x831874f8
	ctx.lr = 0x83186F48;
	sub_831874F8(ctx, base);
	// 83186F48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83186F4C: 4802126C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83186F50: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 83186F54: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83186F58: 388B5AA0  addi r4, r11, 0x5aa0
	ctx.r[4].s64 = ctx.r[11].s64 + 23200;
	// 83186F5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83186F60: 4BFA6ED1  bl 0x8312de30
	ctx.lr = 0x83186F64;
	sub_8312DE30(ctx, base);
	// 83186F64: 80BC0004  lwz r5, 4(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83186F68: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83186F6C: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83186F70: 4BFAD861  bl 0x831347d0
	ctx.lr = 0x83186F74;
	sub_831347D0(ctx, base);
	// 83186F74: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83186F78: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83186F7C: 409A0018  bne cr6, 0x83186f94
	if !ctx.cr[6].eq {
	pc = 0x83186F94; continue 'dispatch;
	}
	// 83186F80: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83186F84: 60840C05  ori r4, r4, 0xc05
	ctx.r[4].u64 = ctx.r[4].u64 | 3077;
	// 83186F88: 48000571  bl 0x831874f8
	ctx.lr = 0x83186F8C;
	sub_831874F8(ctx, base);
	// 83186F8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83186F90: 48021228  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83186F94: 397F1DD4  addi r11, r31, 0x1dd4
	ctx.r[11].s64 = ctx.r[31].s64 + 7636;
	// 83186F98: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83186F9C: 3CC08318  lis r6, -0x7ce8
	ctx.r[6].s64 = -2095579136;
	// 83186FA0: 909D0004  stw r4, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 83186FA4: 3CE08318  lis r7, -0x7ce8
	ctx.r[7].s64 = -2095579136;
	// 83186FA8: 3D008318  lis r8, -0x7ce8
	ctx.r[8].s64 = -2095579136;
	// 83186FAC: 3D208318  lis r9, -0x7ce8
	ctx.r[9].s64 = -2095579136;
	// 83186FB0: 3D408318  lis r10, -0x7ce8
	ctx.r[10].s64 = -2095579136;
	// 83186FB4: 917F191C  stw r11, 0x191c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(6428 as u32), ctx.r[11].u32 ) };
	// 83186FB8: 80BE000C  lwz r5, 0xc(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83186FBC: 38C65850  addi r6, r6, 0x5850
	ctx.r[6].s64 = ctx.r[6].s64 + 22608;
	// 83186FC0: 38E75860  addi r7, r7, 0x5860
	ctx.r[7].s64 = ctx.r[7].s64 + 22624;
	// 83186FC4: 39085870  addi r8, r8, 0x5870
	ctx.r[8].s64 = ctx.r[8].s64 + 22640;
	// 83186FC8: 39295880  addi r9, r9, 0x5880
	ctx.r[9].s64 = ctx.r[9].s64 + 22656;
	// 83186FCC: 394A5890  addi r10, r10, 0x5890
	ctx.r[10].s64 = ctx.r[10].s64 + 22672;
	// 83186FD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83186FD4: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 83186FD8: 90CB0004  stw r6, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 83186FDC: 90EB0008  stw r7, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 83186FE0: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 83186FE4: 912B0010  stw r9, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 83186FE8: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83186FEC: 4BFA7535  bl 0x8312e520
	ctx.lr = 0x83186FF0;
	sub_8312E520(ctx, base);
	// 83186FF0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83186FF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83186FF8: 4BFFF7A9  bl 0x831867a0
	ctx.lr = 0x83186FFC;
	sub_831867A0(ctx, base);
	// 83186FFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83187000: 4BFFFC61  bl 0x83186c60
	ctx.lr = 0x83187004;
	sub_83186C60(ctx, base);
	// 83187004: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83187008: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318700C: 480211AC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83187010 size=100
    let mut pc: u32 = 0x83187010;
    'dispatch: loop {
        match pc {
            0x83187010 => {
    //   block [0x83187010..0x83187074)
	// 83187010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83187014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83187018: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318701C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83187020: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83187024: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83187028: 4BFF23E9  bl 0x83179410
	ctx.lr = 0x8318702C;
	sub_83179410(ctx, base);
	// 8318702C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83187030: 409A001C  bne cr6, 0x8318704c
	if !ctx.cr[6].eq {
	pc = 0x8318704C; continue 'dispatch;
	}
	// 83187034: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83187038: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318703C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83187040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83187044: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83187048: 4E800020  blr
	return;
	// 8318704C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83187050: 4BFF5071  bl 0x8317c0c0
	ctx.lr = 0x83187054;
	sub_8317C0C0(ctx, base);
	// 83187054: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 83187058: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8318705C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 83187060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83187064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83187068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318706C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83187070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187078 size=12
    let mut pc: u32 = 0x83187078;
    'dispatch: loop {
        match pc {
            0x83187078 => {
    //   block [0x83187078..0x83187084)
	// 83187078: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318707C: 60840701  ori r4, r4, 0x701
	ctx.r[4].u64 = ctx.r[4].u64 | 1793;
	// 83187080: 48000478  b 0x831874f8
	sub_831874F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187088 size=40
    let mut pc: u32 = 0x83187088;
    'dispatch: loop {
        match pc {
            0x83187088 => {
    //   block [0x83187088..0x831870B0)
	// 83187088: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 8318708C: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 83187090: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83187094: 419A001C  beq cr6, 0x831870b0
	if ctx.cr[6].eq {
		sub_831870B0(ctx, base);
		return;
	}
	// 83187098: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8318709C: 419A0014  beq cr6, 0x831870b0
	if ctx.cr[6].eq {
		sub_831870B0(ctx, base);
		return;
	}
	// 831870A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831870A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831870A8: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831870AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831870B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831870B0 size=16
    let mut pc: u32 = 0x831870B0;
    'dispatch: loop {
        match pc {
            0x831870B0 => {
    //   block [0x831870B0..0x831870C0)
	// 831870B0: 816318E0  lwz r11, 0x18e0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6368 as u32) ) } as u64;
	// 831870B4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 831870B8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 831870BC: 48006E3C  b 0x8318def8
	sub_8318DEF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831870C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831870C0 size=20
    let mut pc: u32 = 0x831870C0;
    'dispatch: loop {
        match pc {
            0x831870C0 => {
    //   block [0x831870C0..0x831870D4)
	// 831870C0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 831870C4: 808318E0  lwz r4, 0x18e0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6368 as u32) ) } as u64;
	// 831870C8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 831870CC: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 831870D0: 48006E58  b 0x8318df28
	sub_8318DF28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831870D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831870D8 size=108
    let mut pc: u32 = 0x831870D8;
    'dispatch: loop {
        match pc {
            0x831870D8 => {
    //   block [0x831870D8..0x83187144)
	// 831870D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831870DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831870E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831870E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831870E8: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 831870EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831870F0: 48004D69  bl 0x8318be58
	ctx.lr = 0x831870F4;
	sub_8318BE58(ctx, base);
	// 831870F4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831870F8: 419A0038  beq cr6, 0x83187130
	if ctx.cr[6].eq {
	pc = 0x83187130; continue 'dispatch;
	}
	// 831870FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83187100: 809F18E0  lwz r4, 0x18e0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6368 as u32) ) } as u64;
	// 83187104: 48006ED5  bl 0x8318dfd8
	ctx.lr = 0x83187108;
	sub_8318DFD8(ctx, base);
	// 83187108: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8318710C: 409A0024  bne cr6, 0x83187130
	if !ctx.cr[6].eq {
	pc = 0x83187130; continue 'dispatch;
	}
	// 83187110: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83187114: 4BFFFEFD  bl 0x83187010
	ctx.lr = 0x83187118;
	sub_83187010(ctx, base);
	// 83187118: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318711C: 419A0014  beq cr6, 0x83187130
	if ctx.cr[6].eq {
	pc = 0x83187130; continue 'dispatch;
	}
	// 83187120: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83187124: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83187128: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318712C: 48004D1D  bl 0x8318be48
	ctx.lr = 0x83187130;
	sub_8318BE48(ctx, base);
	// 83187130: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83187134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83187138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318713C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83187140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83187148 size=108
    let mut pc: u32 = 0x83187148;
    'dispatch: loop {
        match pc {
            0x83187148 => {
    //   block [0x83187148..0x831871B4)
	// 83187148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318714C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83187150: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83187154: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83187158: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8318715C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83187160: 48004CD9  bl 0x8318be38
	ctx.lr = 0x83187164;
	sub_8318BE38(ctx, base);
	// 83187164: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83187168: 419A0038  beq cr6, 0x831871a0
	if ctx.cr[6].eq {
	pc = 0x831871A0; continue 'dispatch;
	}
	// 8318716C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83187170: 809F18E0  lwz r4, 0x18e0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6368 as u32) ) } as u64;
	// 83187174: 48006E2D  bl 0x8318dfa0
	ctx.lr = 0x83187178;
	sub_8318DFA0(ctx, base);
	// 83187178: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8318717C: 409A0024  bne cr6, 0x831871a0
	if !ctx.cr[6].eq {
	pc = 0x831871A0; continue 'dispatch;
	}
	// 83187180: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83187184: 4BECC56D  bl 0x830536f0
	ctx.lr = 0x83187188;
	sub_830536F0(ctx, base);
	// 83187188: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318718C: 419A0014  beq cr6, 0x831871a0
	if ctx.cr[6].eq {
	pc = 0x831871A0; continue 'dispatch;
	}
	// 83187190: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83187194: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83187198: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318719C: 48004C8D  bl 0x8318be28
	ctx.lr = 0x831871A0;
	sub_8318BE28(ctx, base);
	// 831871A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831871A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831871A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831871AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831871B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831871B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831871B8 size=100
    let mut pc: u32 = 0x831871B8;
    'dispatch: loop {
        match pc {
            0x831871B8 => {
    //   block [0x831871B8..0x8318721C)
	// 831871B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831871BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831871C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831871C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831871C8: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 831871CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831871D0: 4BFF2241  bl 0x83179410
	ctx.lr = 0x831871D4;
	sub_83179410(ctx, base);
	// 831871D4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831871D8: 409A0018  bne cr6, 0x831871f0
	if !ctx.cr[6].eq {
	pc = 0x831871F0; continue 'dispatch;
	}
	// 831871DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831871E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831871E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831871E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831871EC: 4E800020  blr
	return;
	// 831871F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831871F4: 4BFFFEE5  bl 0x831870d8
	ctx.lr = 0x831871F8;
	sub_831870D8(ctx, base);
	// 831871F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831871FC: 4BFFFF4D  bl 0x83187148
	ctx.lr = 0x83187200;
	sub_83187148(ctx, base);
	// 83187200: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83187204: 4BFF7285  bl 0x8317e488
	ctx.lr = 0x83187208;
	sub_8317E488(ctx, base);
	// 83187208: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318720C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83187210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83187214: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83187218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83187220 size=44
    let mut pc: u32 = 0x83187220;
    'dispatch: loop {
        match pc {
            0x83187220 => {
    //   block [0x83187220..0x8318724C)
	// 83187220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83187224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83187228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318722C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83187230: 8083174C  lwz r4, 0x174c(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(5964 as u32) ) } as u64;
	// 83187234: 48006D55  bl 0x8318df88
	ctx.lr = 0x83187238;
	sub_8318DF88(ctx, base);
	// 83187238: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318723C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83187240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83187244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83187248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187250 size=12
    let mut pc: u32 = 0x83187250;
    'dispatch: loop {
        match pc {
            0x83187250 => {
    //   block [0x83187250..0x8318725C)
	// 83187250: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83187254: 8083174C  lwz r4, 0x174c(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(5964 as u32) ) } as u64;
	// 83187258: 48007420  b 0x8318e678
	sub_8318E678(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187260 size=20
    let mut pc: u32 = 0x83187260;
    'dispatch: loop {
        match pc {
            0x83187260 => {
    //   block [0x83187260..0x83187274)
	// 83187260: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83187264: 80C50000  lwz r6, 0(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 83187268: 8083174C  lwz r4, 0x174c(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(5964 as u32) ) } as u64;
	// 8318726C: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83187270: 48007718  b 0x8318e988
	sub_8318E988(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187278 size=12
    let mut pc: u32 = 0x83187278;
    'dispatch: loop {
        match pc {
            0x83187278 => {
    //   block [0x83187278..0x83187284)
	// 83187278: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318727C: 60840501  ori r4, r4, 0x501
	ctx.r[4].u64 = ctx.r[4].u64 | 1281;
	// 83187280: 48000278  b 0x831874f8
	sub_831874F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187288 size=16
    let mut pc: u32 = 0x83187288;
    'dispatch: loop {
        match pc {
            0x83187288 => {
    //   block [0x83187288..0x83187298)
	// 83187288: 3964E030  addi r11, r4, -0x1fd0
	ctx.r[11].s64 = ctx.r[4].s64 + -8144;
	// 8318728C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83187290: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83187294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187298 size=16
    let mut pc: u32 = 0x83187298;
    'dispatch: loop {
        match pc {
            0x83187298 => {
    //   block [0x83187298..0x831872A8)
	// 83187298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318729C: 91630204  stw r11, 0x204(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(516 as u32), ctx.r[11].u32 ) };
	// 831872A0: 91630208  stw r11, 0x208(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(520 as u32), ctx.r[11].u32 ) };
	// 831872A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831872A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831872A8 size=24
    let mut pc: u32 = 0x831872A8;
    'dispatch: loop {
        match pc {
            0x831872A8 => {
    //   block [0x831872A8..0x831872C0)
	// 831872A8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831872AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831872B0: 409A0008  bne cr6, 0x831872b8
	if !ctx.cr[6].eq {
	pc = 0x831872B8; continue 'dispatch;
	}
	// 831872B4: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 831872B8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831872BC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831872C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831872C0 size=12
    let mut pc: u32 = 0x831872C0;
    'dispatch: loop {
        match pc {
            0x831872C0 => {
    //   block [0x831872C0..0x831872CC)
	// 831872C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831872C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831872C8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831872CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831872CC size=12
    let mut pc: u32 = 0x831872CC;
    'dispatch: loop {
        match pc {
            0x831872CC => {
    //   block [0x831872CC..0x831872D8)
	// 831872CC: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831872D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831872D4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831872D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831872D8 size=4
    let mut pc: u32 = 0x831872D8;
    'dispatch: loop {
        match pc {
            0x831872D8 => {
    //   block [0x831872D8..0x831872DC)
	// 831872D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831872E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831872E0 size=16
    let mut pc: u32 = 0x831872E0;
    'dispatch: loop {
        match pc {
            0x831872E0 => {
    //   block [0x831872E0..0x831872F0)
	// 831872E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831872E4: 409A000C  bne cr6, 0x831872f0
	if !ctx.cr[6].eq {
		sub_831872F0(ctx, base);
		return;
	}
	// 831872E8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831872EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831872F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831872F0 size=28
    let mut pc: u32 = 0x831872F0;
    'dispatch: loop {
        match pc {
            0x831872F0 => {
    //   block [0x831872F0..0x8318730C)
	// 831872F0: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 831872F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831872F8: 419AFFF0  beq cr6, 0x831872e8
	if ctx.cr[6].eq {
		sub_831872E0(ctx, base);
		return;
	}
	// 831872FC: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83187300: 906BA330  stw r3, -0x5cd0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-23760 as u32), ctx.r[3].u32 ) };
	// 83187304: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83187308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187310 size=4
    let mut pc: u32 = 0x83187310;
    'dispatch: loop {
        match pc {
            0x83187310 => {
    //   block [0x83187310..0x83187314)
	// 83187310: 4BFACDE0  b 0x831340f0
	sub_831340F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187318 size=4
    let mut pc: u32 = 0x83187318;
    'dispatch: loop {
        match pc {
            0x83187318 => {
    //   block [0x83187318..0x8318731C)
	// 83187318: 4BFACE40  b 0x83134158
	sub_83134158(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83187320 size=40
    let mut pc: u32 = 0x83187320;
    'dispatch: loop {
        match pc {
            0x83187320 => {
    //   block [0x83187320..0x83187348)
	// 83187320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83187324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83187328: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318732C: 4BFFC3C5  bl 0x831836f0
	ctx.lr = 0x83187330;
	sub_831836F0(ctx, base);
	// 83187330: 48007CB9  bl 0x8318efe8
	ctx.lr = 0x83187334;
	sub_8318EFE8(ctx, base);
	// 83187334: 480044ED  bl 0x8318b820
	ctx.lr = 0x83187338;
	sub_8318B820(ctx, base);
	// 83187338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318733C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83187340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83187344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83187348 size=36
    let mut pc: u32 = 0x83187348;
    'dispatch: loop {
        match pc {
            0x83187348 => {
    //   block [0x83187348..0x8318736C)
	// 83187348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318734C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83187350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83187354: 48004545  bl 0x8318b898
	ctx.lr = 0x83187358;
	sub_8318B898(ctx, base);
	// 83187358: 48007CA1  bl 0x8318eff8
	ctx.lr = 0x8318735C;
	sub_8318EFF8(ctx, base);
	// 8318735C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83187360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83187364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83187368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83187370 size=60
    let mut pc: u32 = 0x83187370;
    'dispatch: loop {
        match pc {
            0x83187370 => {
    //   block [0x83187370..0x831873AC)
	// 83187370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83187374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83187378: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318737C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83187380: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83187384: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 83187388: 3BEBC684  addi r31, r11, -0x397c
	ctx.r[31].s64 = ctx.r[11].s64 + -14716;
	// 8318738C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83187390: 4BFA7B01  bl 0x8312ee90
	ctx.lr = 0x83187394;
	sub_8312EE90(ctx, base);
	// 83187394: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 83187398: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318739C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831873A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831873A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831873A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831873B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831873B0 size=64
    let mut pc: u32 = 0x831873B0;
    'dispatch: loop {
        match pc {
            0x831873B0 => {
    //   block [0x831873B0..0x831873F0)
	// 831873B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831873B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831873B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831873BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831873C0: 3FE08340  lis r31, -0x7cc0
	ctx.r[31].s64 = -2092957696;
	// 831873C4: 807FC6A8  lwz r3, -0x3958(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-14680 as u32) ) } as u64;
	// 831873C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831873CC: 419A0010  beq cr6, 0x831873dc
	if ctx.cr[6].eq {
	pc = 0x831873DC; continue 'dispatch;
	}
	// 831873D0: 4BFA7BA1  bl 0x8312ef70
	ctx.lr = 0x831873D4;
	sub_8312EF70(ctx, base);
	// 831873D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831873D8: 917FC6A8  stw r11, -0x3958(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-14680 as u32), ctx.r[11].u32 ) };
	// 831873DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831873E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831873E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831873E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831873EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831873F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831873F0 size=12
    let mut pc: u32 = 0x831873F0;
    'dispatch: loop {
        match pc {
            0x831873F0 => {
    //   block [0x831873F0..0x831873FC)
	// 831873F0: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 831873F4: 806BC6A8  lwz r3, -0x3958(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14680 as u32) ) } as u64;
	// 831873F8: 4BFA7C08  b 0x8312f000
	sub_8312F000(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187400 size=12
    let mut pc: u32 = 0x83187400;
    'dispatch: loop {
        match pc {
            0x83187400 => {
    //   block [0x83187400..0x8318740C)
	// 83187400: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83187404: 806BC6A8  lwz r3, -0x3958(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14680 as u32) ) } as u64;
	// 83187408: 4BFA7C90  b 0x8312f098
	sub_8312F098(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83187410 size=224
    let mut pc: u32 = 0x83187410;
    'dispatch: loop {
        match pc {
            0x83187410 => {
    //   block [0x83187410..0x831874F0)
	// 83187410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83187414: 48020D51  bl 0x831a8164
	ctx.lr = 0x83187418;
	sub_831A8130(ctx, base);
	// 83187418: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318741C: 3F608345  lis r27, -0x7cbb
	ctx.r[27].s64 = -2092630016;
	// 83187420: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83187424: 3B8BD5C8  addi r28, r11, -0x2a38
	ctx.r[28].s64 = ctx.r[11].s64 + -10808;
	// 83187428: 807BA32C  lwz r3, -0x5cd4(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 8318742C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83187430: 419A0018  beq cr6, 0x83187448
	if ctx.cr[6].eq {
	pc = 0x83187448; continue 'dispatch;
	}
	// 83187434: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83187438: 389C0004  addi r4, r28, 4
	ctx.r[4].s64 = ctx.r[28].s64 + 4;
	// 8318743C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83187440: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83187444: 4E800421  bctrl
	ctx.lr = 0x83187448;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83187448: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318744C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83187450: 3BCBA100  addi r30, r11, -0x5f00
	ctx.r[30].s64 = ctx.r[11].s64 + -24320;
	// 83187454: 3BFE020C  addi r31, r30, 0x20c
	ctx.r[31].s64 = ctx.r[30].s64 + 524;
	// 83187458: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318745C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83187460: 419A000C  beq cr6, 0x8318746c
	if ctx.cr[6].eq {
	pc = 0x8318746C; continue 'dispatch;
	}
	// 83187464: 4BFFD745  bl 0x83184ba8
	ctx.lr = 0x83187468;
	sub_83184BA8(ctx, base);
	// 83187468: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8318746C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 83187470: 397E022C  addi r11, r30, 0x22c
	ctx.r[11].s64 = ctx.r[30].s64 + 556;
	// 83187474: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83187478: 4198FFE0  blt cr6, 0x83187458
	if ctx.cr[6].lt {
	pc = 0x83187458; continue 'dispatch;
	}
	// 8318747C: 387E01B0  addi r3, r30, 0x1b0
	ctx.r[3].s64 = ctx.r[30].s64 + 432;
	// 83187480: 4BF40661  bl 0x830c7ae0
	ctx.lr = 0x83187484;
	sub_830C7AE0(ctx, base);
	// 83187484: 387E01C4  addi r3, r30, 0x1c4
	ctx.r[3].s64 = ctx.r[30].s64 + 452;
	// 83187488: 4BF40659  bl 0x830c7ae0
	ctx.lr = 0x8318748C;
	sub_830C7AE0(ctx, base);
	// 8318748C: 387E01C8  addi r3, r30, 0x1c8
	ctx.r[3].s64 = ctx.r[30].s64 + 456;
	// 83187490: 48004A39  bl 0x8318bec8
	ctx.lr = 0x83187494;
	sub_8318BEC8(ctx, base);
	// 83187494: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83187498: 4BFFFF19  bl 0x831873b0
	ctx.lr = 0x8318749C;
	sub_831873B0(ctx, base);
	// 8318749C: 4BFFFEAD  bl 0x83187348
	ctx.lr = 0x831874A0;
	sub_83187348(ctx, base);
	// 831874A0: 4BFFFE79  bl 0x83187318
	ctx.lr = 0x831874A4;
	sub_83187318(ctx, base);
	// 831874A4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831874A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831874AC: 409A0008  bne cr6, 0x831874b4
	if !ctx.cr[6].eq {
	pc = 0x831874B4; continue 'dispatch;
	}
	// 831874B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831874B4: 817BA32C  lwz r11, -0x5cd4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 831874B8: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 831874BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831874C0: 419A0028  beq cr6, 0x831874e8
	if ctx.cr[6].eq {
	pc = 0x831874E8; continue 'dispatch;
	}
	// 831874C4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 831874C8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 831874CC: 389C006C  addi r4, r28, 0x6c
	ctx.r[4].s64 = ctx.r[28].s64 + 108;
	// 831874D0: 915C0074  stw r10, 0x74(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 831874D4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831874D8: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 831874DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831874E0: 4E800421  bctrl
	ctx.lr = 0x831874E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831874E4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831874E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831874EC: 48020CC8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831874F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831874F0 size=4
    let mut pc: u32 = 0x831874F0;
    'dispatch: loop {
        match pc {
            0x831874F0 => {
    //   block [0x831874F0..0x831874F4)
	// 831874F0: 480074B0  b 0x8318e9a0
	sub_8318E9A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831874F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831874F8 size=200
    let mut pc: u32 = 0x831874F8;
    'dispatch: loop {
        match pc {
            0x831874F8 => {
    //   block [0x831874F8..0x831875C0)
	// 831874F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831874FC: 48020C71  bl 0x831a816c
	ctx.lr = 0x83187500;
	sub_831A8130(ctx, base);
	// 83187500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83187504: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83187508: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 8318750C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83187510: 409A0010  bne cr6, 0x83187520
	if !ctx.cr[6].eq {
	pc = 0x83187520; continue 'dispatch;
	}
	// 83187514: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83187518: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318751C: 48020CA0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83187520: 3FA08345  lis r29, -0x7cbb
	ctx.r[29].s64 = -2092630016;
	// 83187524: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83187528: 3BCBDFE8  addi r30, r11, -0x2018
	ctx.r[30].s64 = ctx.r[11].s64 + -8216;
	// 8318752C: 807DA32C  lwz r3, -0x5cd4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83187530: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83187534: 419A0028  beq cr6, 0x8318755c
	if ctx.cr[6].eq {
	pc = 0x8318755C; continue 'dispatch;
	}
	// 83187538: 3961008C  addi r11, r1, 0x8c
	ctx.r[11].s64 = ctx.r[1].s64 + 140;
	// 8318753C: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 83187540: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83187544: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83187548: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318754C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83187550: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83187554: 4E800421  bctrl
	ctx.lr = 0x83187558;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83187558: 8081008C  lwz r4, 0x8c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 8318755C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83187560: 409A0018  bne cr6, 0x83187578
	if !ctx.cr[6].eq {
	pc = 0x83187578; continue 'dispatch;
	}
	// 83187564: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83187568: 396BA100  addi r11, r11, -0x5f00
	ctx.r[11].s64 = ctx.r[11].s64 + -24320;
	// 8318756C: 386B019C  addi r3, r11, 0x19c
	ctx.r[3].s64 = ctx.r[11].s64 + 412;
	// 83187570: 4BFFFD39  bl 0x831872a8
	ctx.lr = 0x83187574;
	sub_831872A8(ctx, base);
	// 83187574: 48000020  b 0x83187594
	pc = 0x83187594; continue 'dispatch;
	// 83187578: 387F09F8  addi r3, r31, 0x9f8
	ctx.r[3].s64 = ctx.r[31].s64 + 2552;
	// 8318757C: 4BFFFD2D  bl 0x831872a8
	ctx.lr = 0x83187580;
	sub_831872A8(ctx, base);
	// 83187580: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83187584: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83187588: 4099000C  ble cr6, 0x83187594
	if !ctx.cr[6].gt {
	pc = 0x83187594; continue 'dispatch;
	}
	// 8318758C: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 83187590: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83187594: 807DA32C  lwz r3, -0x5cd4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83187598: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318759C: 419A0018  beq cr6, 0x831875b4
	if ctx.cr[6].eq {
	pc = 0x831875B4; continue 'dispatch;
	}
	// 831875A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831875A4: 389E006C  addi r4, r30, 0x6c
	ctx.r[4].s64 = ctx.r[30].s64 + 108;
	// 831875A8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 831875AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831875B0: 4E800421  bctrl
	ctx.lr = 0x831875B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831875B4: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 831875B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831875BC: 48020C00  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831875C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831875C0 size=116
    let mut pc: u32 = 0x831875C0;
    'dispatch: loop {
        match pc {
            0x831875C0 => {
    //   block [0x831875C0..0x83187634)
	// 831875C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831875C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831875C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831875CC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 831875D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831875D4: 409A0014  bne cr6, 0x831875e8
	if !ctx.cr[6].eq {
	pc = 0x831875E8; continue 'dispatch;
	}
	// 831875D8: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 831875DC: 396BA100  addi r11, r11, -0x5f00
	ctx.r[11].s64 = ctx.r[11].s64 + -24320;
	// 831875E0: 386B019C  addi r3, r11, 0x19c
	ctx.r[3].s64 = ctx.r[11].s64 + 412;
	// 831875E4: 48000038  b 0x8318761c
	pc = 0x8318761C; continue 'dispatch;
	// 831875E8: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 831875EC: 4BFFFCF5  bl 0x831872e0
	ctx.lr = 0x831875F0;
	sub_831872E0(ctx, base);
	// 831875F0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831875F4: 419A0024  beq cr6, 0x83187618
	if ctx.cr[6].eq {
	pc = 0x83187618; continue 'dispatch;
	}
	// 831875F8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 831875FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83187600: 60840101  ori r4, r4, 0x101
	ctx.r[4].u64 = ctx.r[4].u64 | 257;
	// 83187604: 4BFFFEF5  bl 0x831874f8
	ctx.lr = 0x83187608;
	sub_831874F8(ctx, base);
	// 83187608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318760C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83187610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83187614: 4E800020  blr
	return;
	// 83187618: 386A09F8  addi r3, r10, 0x9f8
	ctx.r[3].s64 = ctx.r[10].s64 + 2552;
	// 8318761C: 48004CC5  bl 0x8318c2e0
	ctx.lr = 0x83187620;
	sub_8318C2E0(ctx, base);
	// 83187620: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83187624: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83187628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318762C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83187630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83187638 size=184
    let mut pc: u32 = 0x83187638;
    'dispatch: loop {
        match pc {
            0x83187638 => {
    //   block [0x83187638..0x831876F0)
	// 83187638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318763C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83187640: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83187644: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83187648: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318764C: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83187650: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83187654: 3BEBA100  addi r31, r11, -0x5f00
	ctx.r[31].s64 = ctx.r[11].s64 + -24320;
	// 83187658: 38A0008B  li r5, 0x8b
	ctx.r[5].s64 = 139;
	// 8318765C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83187660: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83187664: 4800BA25  bl 0x83193088
	ctx.lr = 0x83187668;
	sub_83193088(ctx, base);
	// 83187668: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318766C: 38A00190  li r5, 0x190
	ctx.r[5].s64 = 400;
	// 83187670: 388BAF20  addi r4, r11, -0x50e0
	ctx.r[4].s64 = ctx.r[11].s64 + -20704;
	// 83187674: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83187678: 48007321  bl 0x8318e998
	ctx.lr = 0x8318767C;
	sub_8318E998(ctx, base);
	// 8318767C: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 83187680: 387F019C  addi r3, r31, 0x19c
	ctx.r[3].s64 = ctx.r[31].s64 + 412;
	// 83187684: F97F0190  std r11, 0x190(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[11].u64 ) };
	// 83187688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318768C: 917F0198  stw r11, 0x198(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(408 as u32), ctx.r[11].u32 ) };
	// 83187690: 4BFFFE61  bl 0x831874f0
	ctx.lr = 0x83187694;
	sub_831874F0(ctx, base);
	// 83187694: 387F01B0  addi r3, r31, 0x1b0
	ctx.r[3].s64 = ctx.r[31].s64 + 432;
	// 83187698: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318769C: 4BFF3DBD  bl 0x8317b458
	ctx.lr = 0x831876A0;
	sub_8317B458(ctx, base);
	// 831876A0: 387F01C4  addi r3, r31, 0x1c4
	ctx.r[3].s64 = ctx.r[31].s64 + 452;
	// 831876A4: 48006AAD  bl 0x8318e150
	ctx.lr = 0x831876A8;
	sub_8318E150(ctx, base);
	// 831876A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831876AC: 4BFFFBED  bl 0x83187298
	ctx.lr = 0x831876B0;
	sub_83187298(ctx, base);
	// 831876B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831876B4: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 831876B8: 397F020C  addi r11, r31, 0x20c
	ctx.r[11].s64 = ctx.r[31].s64 + 524;
	// 831876BC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831876C0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831876C4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831876C8: 4200FFF8  bdnz 0x831876c0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831876C0; continue 'dispatch;
	}
	// 831876CC: 387F01C8  addi r3, r31, 0x1c8
	ctx.r[3].s64 = ctx.r[31].s64 + 456;
	// 831876D0: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831876D4: 480047B5  bl 0x8318be88
	ctx.lr = 0x831876D8;
	sub_8318BE88(ctx, base);
	// 831876D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831876DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831876E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831876E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831876E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831876EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831876F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831876F0 size=188
    let mut pc: u32 = 0x831876F0;
    'dispatch: loop {
        match pc {
            0x831876F0 => {
    //   block [0x831876F0..0x831877AC)
	// 831876F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831876F4: 48020A79  bl 0x831a816c
	ctx.lr = 0x831876F8;
	sub_831A8130(ctx, base);
	// 831876F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831876FC: 3FA08345  lis r29, -0x7cbb
	ctx.r[29].s64 = -2092630016;
	// 83187700: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83187704: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83187708: 3BCBD4F0  addi r30, r11, -0x2b10
	ctx.r[30].s64 = ctx.r[11].s64 + -11024;
	// 8318770C: 807DA32C  lwz r3, -0x5cd4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83187710: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83187714: 419A0020  beq cr6, 0x83187734
	if ctx.cr[6].eq {
	pc = 0x83187734; continue 'dispatch;
	}
	// 83187718: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8318771C: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83187720: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83187724: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83187728: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8318772C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83187730: 4E800421  bctrl
	ctx.lr = 0x83187734;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83187734: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83187738: 3D008340  lis r8, -0x7cc0
	ctx.r[8].s64 = -2092957696;
	// 8318773C: 396BB500  addi r11, r11, -0x4b00
	ctx.r[11].s64 = ctx.r[11].s64 + -19200;
	// 83187740: 3D208345  lis r9, -0x7cbb
	ctx.r[9].s64 = -2092630016;
	// 83187744: 39401FD0  li r10, 0x1fd0
	ctx.r[10].s64 = 8144;
	// 83187748: 9168C6A4  stw r11, -0x395c(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-14684 as u32), ctx.r[11].u32 ) };
	// 8318774C: 9149A334  stw r10, -0x5ccc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-23756 as u32), ctx.r[10].u32 ) };
	// 83187750: 4BFFFBC1  bl 0x83187310
	ctx.lr = 0x83187754;
	sub_83187310(ctx, base);
	// 83187754: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83187758: 4BFFFEE1  bl 0x83187638
	ctx.lr = 0x8318775C;
	sub_83187638(ctx, base);
	// 8318775C: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83187760: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83187764: 409A0010  bne cr6, 0x83187774
	if !ctx.cr[6].eq {
	pc = 0x83187774; continue 'dispatch;
	}
	// 83187768: 4BFFFBB9  bl 0x83187320
	ctx.lr = 0x8318776C;
	sub_83187320(ctx, base);
	// 8318776C: 4BFFFC05  bl 0x83187370
	ctx.lr = 0x83187770;
	sub_83187370(ctx, base);
	// 83187770: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83187774: 817DA32C  lwz r11, -0x5cd4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 83187778: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318777C: 419A0028  beq cr6, 0x831877a4
	if ctx.cr[6].eq {
	pc = 0x831877A4; continue 'dispatch;
	}
	// 83187780: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 83187784: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83187788: 389E006C  addi r4, r30, 0x6c
	ctx.r[4].s64 = ctx.r[30].s64 + 108;
	// 8318778C: 915E0074  stw r10, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 83187790: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83187794: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 83187798: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318779C: 4E800421  bctrl
	ctx.lr = 0x831877A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831877A0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831877A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831877A8: 48020A14  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831877B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831877B0 size=12
    let mut pc: u32 = 0x831877B0;
    'dispatch: loop {
        match pc {
            0x831877B0 => {
    //   block [0x831877B0..0x831877BC)
	// 831877B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831877B4: 91630494  stw r11, 0x494(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1172 as u32), ctx.r[11].u32 ) };
	// 831877B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831877C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831877C0 size=16
    let mut pc: u32 = 0x831877C0;
    'dispatch: loop {
        match pc {
            0x831877C0 => {
    //   block [0x831877C0..0x831877D0)
	// 831877C0: 81630494  lwz r11, 0x494(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1172 as u32) ) } as u64;
	// 831877C4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831877C8: 91630494  stw r11, 0x494(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1172 as u32), ctx.r[11].u32 ) };
	// 831877CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831877D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831877D0 size=16
    let mut pc: u32 = 0x831877D0;
    'dispatch: loop {
        match pc {
            0x831877D0 => {
    //   block [0x831877D0..0x831877E0)
	// 831877D0: 81630494  lwz r11, 0x494(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1172 as u32) ) } as u64;
	// 831877D4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831877D8: 91630494  stw r11, 0x494(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1172 as u32), ctx.r[11].u32 ) };
	// 831877DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831877E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831877E0 size=8
    let mut pc: u32 = 0x831877E0;
    'dispatch: loop {
        match pc {
            0x831877E0 => {
    //   block [0x831877E0..0x831877E8)
	// 831877E0: 80630494  lwz r3, 0x494(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1172 as u32) ) } as u64;
	// 831877E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831877E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831877E8 size=176
    let mut pc: u32 = 0x831877E8;
    'dispatch: loop {
        match pc {
            0x831877E8 => {
    //   block [0x831877E8..0x83187898)
	// 831877E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831877EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831877F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831877F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831877F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831877FC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83187800: 41980020  blt cr6, 0x83187820
	if ctx.cr[6].lt {
	pc = 0x83187820; continue 'dispatch;
	}
	// 83187804: 4BFFFFDD  bl 0x831877e0
	ctx.lr = 0x83187808;
	sub_831877E0(ctx, base);
	// 83187808: 817F0490  lwz r11, 0x490(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1168 as u32) ) } as u64;
	// 8318780C: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83187810: 41980028  blt cr6, 0x83187838
	if ctx.cr[6].lt {
	pc = 0x83187838; continue 'dispatch;
	}
	// 83187814: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83187818: 386BB550  addi r3, r11, -0x4ab0
	ctx.r[3].s64 = ctx.r[11].s64 + -19120;
	// 8318781C: 4BFF5925  bl 0x8317d140
	ctx.lr = 0x83187820;
	sub_8317D140(ctx, base);
	// 83187820: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83187824: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83187828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318782C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83187830: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83187834: 4E800020  blr
	return;
	// 83187838: 817F0480  lwz r11, 0x480(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1152 as u32) ) } as u64;
	// 8318783C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83187840: 419A0010  beq cr6, 0x83187850
	if ctx.cr[6].eq {
	pc = 0x83187850; continue 'dispatch;
	}
	// 83187844: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83187848: 4BFED261  bl 0x83174aa8
	ctx.lr = 0x8318784C;
	sub_83174AA8(ctx, base);
	// 8318784C: 4800000C  b 0x83187858
	pc = 0x83187858; continue 'dispatch;
	// 83187850: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83187854: 4BFED28D  bl 0x83174ae0
	ctx.lr = 0x83187858;
	sub_83174AE0(ctx, base);
	// 83187858: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318785C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83187860: 419AFFB4  beq cr6, 0x83187814
	if ctx.cr[6].eq {
	pc = 0x83187814; continue 'dispatch;
	}
	// 83187864: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83187868: 4BFFFF79  bl 0x831877e0
	ctx.lr = 0x8318786C;
	sub_831877E0(ctx, base);
	// 8318786C: 817F049C  lwz r11, 0x49c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1180 as u32) ) } as u64;
	// 83187870: 5469103A  slwi r9, r3, 2
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83187874: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83187878: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8318787C: 4BFFFF45  bl 0x831877c0
	ctx.lr = 0x83187880;
	sub_831877C0(ctx, base);
	// 83187880: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 83187884: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83187888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318788C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83187890: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83187894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83187898 size=76
    let mut pc: u32 = 0x83187898;
    'dispatch: loop {
        match pc {
            0x83187898 => {
    //   block [0x83187898..0x831878E4)
	// 83187898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318789C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831878A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831878A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831878A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831878AC: 817F0480  lwz r11, 0x480(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1152 as u32) ) } as u64;
	// 831878B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831878B4: 419A000C  beq cr6, 0x831878c0
	if ctx.cr[6].eq {
	pc = 0x831878C0; continue 'dispatch;
	}
	// 831878B8: 4BF40229  bl 0x830c7ae0
	ctx.lr = 0x831878BC;
	sub_830C7AE0(ctx, base);
	// 831878BC: 4800000C  b 0x831878c8
	pc = 0x831878C8; continue 'dispatch;
	// 831878C0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 831878C4: 4BFED265  bl 0x83174b28
	ctx.lr = 0x831878C8;
	sub_83174B28(ctx, base);
	// 831878C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831878CC: 4BFFFF05  bl 0x831877d0
	ctx.lr = 0x831878D0;
	sub_831877D0(ctx, base);
	// 831878D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831878D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831878D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831878DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831878E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831878E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831878E8 size=112
    let mut pc: u32 = 0x831878E8;
    'dispatch: loop {
        match pc {
            0x831878E8 => {
    //   block [0x831878E8..0x83187958)
	// 831878E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831878EC: 4802087D  bl 0x831a8168
	ctx.lr = 0x831878F0;
	sub_831A8130(ctx, base);
	// 831878F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831878F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831878F8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 831878FC: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 83187900: 817F0490  lwz r11, 0x490(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1168 as u32) ) } as u64;
	// 83187904: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83187908: 40990040  ble cr6, 0x83187948
	if !ctx.cr[6].gt {
	pc = 0x83187948; continue 'dispatch;
	}
	// 8318790C: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 83187910: 815F049C  lwz r10, 0x49c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1180 as u32) ) } as u64;
	// 83187914: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83187918: 557E103A  slwi r30, r11, 2
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8318791C: 7C8AF02E  lwzx r4, r10, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 83187920: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83187924: 419A0014  beq cr6, 0x83187938
	if ctx.cr[6].eq {
	pc = 0x83187938; continue 'dispatch;
	}
	// 83187928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318792C: 4BFFFF6D  bl 0x83187898
	ctx.lr = 0x83187930;
	sub_83187898(ctx, base);
	// 83187930: 817F049C  lwz r11, 0x49c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1180 as u32) ) } as u64;
	// 83187934: 7F8BF12E  stwx r28, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[28].u32) };
	// 83187938: 817F0490  lwz r11, 0x490(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1168 as u32) ) } as u64;
	// 8318793C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83187940: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83187944: 4198FFC8  blt cr6, 0x8318790c
	if ctx.cr[6].lt {
	pc = 0x8318790C; continue 'dispatch;
	}
	// 83187948: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318794C: 4BFED72D  bl 0x83175078
	ctx.lr = 0x83187950;
	sub_83175078(ctx, base);
	// 83187950: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83187954: 48020864  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187958 size=60
    let mut pc: u32 = 0x83187958;
    'dispatch: loop {
        match pc {
            0x83187958 => {
    //   block [0x83187958..0x83187994)
	// 83187958: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318795C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83187960: 394BC6B8  addi r10, r11, -0x3948
	ctx.r[10].s64 = ctx.r[11].s64 + -14664;
	// 83187964: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 83187968: 396A0008  addi r11, r10, 8
	ctx.r[11].s64 = ctx.r[10].s64 + 8;
	// 8318796C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83187970: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83187974: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83187978: 4200FFF8  bdnz 0x83187970
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83187970; continue 'dispatch;
	}
	// 8318797C: 396AFFF8  addi r11, r10, -8
	ctx.r[11].s64 = ctx.r[10].s64 + -8;
	// 83187980: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 83187984: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83187988: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8318798C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 83187990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83187998 size=196
    let mut pc: u32 = 0x83187998;
    'dispatch: loop {
        match pc {
            0x83187998 => {
    //   block [0x83187998..0x83187A5C)
	// 83187998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318799C: 480207D1  bl 0x831a816c
	ctx.lr = 0x831879A0;
	sub_831A8130(ctx, base);
	// 831879A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831879A4: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 831879A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831879AC: 3BEBC6B8  addi r31, r11, -0x3948
	ctx.r[31].s64 = ctx.r[11].s64 + -14664;
	// 831879B0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831879B4: 395F0008  addi r10, r31, 8
	ctx.r[10].s64 = ctx.r[31].s64 + 8;
	// 831879B8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 831879BC: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 831879C0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831879C4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831879C8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831879CC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831879D0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 831879D4: 4200FFF0  bdnz 0x831879c4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831879C4; continue 'dispatch;
	}
	// 831879D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831879DC: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 831879E0: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 831879E4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831879E8: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 831879EC: 556B0030  rlwinm r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831879F0: 917FFFF8  stw r11, -8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-8 as u32), ctx.r[11].u32 ) };
	// 831879F4: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 831879F8: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 831879FC: 556B0030  rlwinm r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83187A00: 917FFFFC  stw r11, -4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 83187A04: 4BFFB805  bl 0x83183208
	ctx.lr = 0x83187A08;
	sub_83183208(ctx, base);
	// 83187A08: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83187A0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83187A10: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83187A14: 419A0044  beq cr6, 0x83187a58
	if ctx.cr[6].eq {
	pc = 0x83187A58; continue 'dispatch;
	}
	// 83187A18: 813E001C  lwz r9, 0x1c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83187A1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83187A20: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83187A24: 4099002C  ble cr6, 0x83187a50
	if !ctx.cr[6].gt {
	pc = 0x83187A50; continue 'dispatch;
	}
	// 83187A28: 7D0BE850  subf r8, r11, r29
	ctx.r[8].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 83187A2C: 7D28582E  lwzx r9, r8, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83187A30: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83187A34: 3929007F  addi r9, r9, 0x7f
	ctx.r[9].s64 = ctx.r[9].s64 + 127;
	// 83187A38: 55290030  rlwinm r9, r9, 0, 0, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 83187A3C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83187A40: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83187A44: 813E001C  lwz r9, 0x1c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83187A48: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83187A4C: 4198FFE0  blt cr6, 0x83187a2c
	if ctx.cr[6].lt {
	pc = 0x83187A2C; continue 'dispatch;
	}
	// 83187A50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83187A54: 48020768  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83187A58: 48000000  b 0x83187a58
	pc = 0x83187A58; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187A60 size=28
    let mut pc: u32 = 0x83187A60;
    'dispatch: loop {
        match pc {
            0x83187A60 => {
    //   block [0x83187A60..0x83187A7C)
	// 83187A60: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83187A64: 394BC6B8  addi r10, r11, -0x3948
	ctx.r[10].s64 = ctx.r[11].s64 + -14664;
	// 83187A68: 810A0024  lwz r8, 0x24(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 83187A6C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83187A70: 4199000C  bgt cr6, 0x83187a7c
	if ctx.cr[6].gt {
		sub_83187A7C(ctx, base);
		return;
	}
	// 83187A74: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83187A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187A7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187A7C size=112
    let mut pc: u32 = 0x83187A7C;
    'dispatch: loop {
        match pc {
            0x83187A7C => {
    //   block [0x83187A7C..0x83187AEC)
	// 83187A7C: 816A0018  lwz r11, 0x18(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 83187A80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83187A84: 419A0010  beq cr6, 0x83187a94
	if ctx.cr[6].eq {
	pc = 0x83187A94; continue 'dispatch;
	}
	// 83187A88: 816A0028  lwz r11, 0x28(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 83187A8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83187A90: 409A0054  bne cr6, 0x83187ae4
	if !ctx.cr[6].eq {
	pc = 0x83187AE4; continue 'dispatch;
	}
	// 83187A94: 396AFFF8  addi r11, r10, -8
	ctx.r[11].s64 = ctx.r[10].s64 + -8;
	// 83187A98: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83187A9C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83187AA0: 419AFFD4  beq cr6, 0x83187a74
	if ctx.cr[6].eq {
		sub_83187A60(ctx, base);
		return;
	}
	// 83187AA4: 392AFFF8  addi r9, r10, -8
	ctx.r[9].s64 = ctx.r[10].s64 + -8;
	// 83187AA8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83187AAC: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 83187AB0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83187AB4: 4198FFE4  blt cr6, 0x83187a98
	if ctx.cr[6].lt {
	pc = 0x83187A98; continue 'dispatch;
	}
	// 83187AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83187ABC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83187AC0: 40990024  ble cr6, 0x83187ae4
	if !ctx.cr[6].gt {
	pc = 0x83187AE4; continue 'dispatch;
	}
	// 83187AC4: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83187AC8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83187ACC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83187AD0: 419AFFA4  beq cr6, 0x83187a74
	if ctx.cr[6].eq {
		sub_83187A60(ctx, base);
		return;
	}
	// 83187AD4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83187AD8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83187ADC: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83187AE0: 4198FFE8  blt cr6, 0x83187ac8
	if ctx.cr[6].lt {
	pc = 0x83187AC8; continue 'dispatch;
	}
	// 83187AE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83187AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83187AF0 size=140
    let mut pc: u32 = 0x83187AF0;
    'dispatch: loop {
        match pc {
            0x83187AF0 => {
    //   block [0x83187AF0..0x83187B7C)
	// 83187AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83187AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83187AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83187AFC: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 83187B00: 4BFFFF61  bl 0x83187a60
	ctx.lr = 0x83187B04;
	sub_83187A60(ctx, base);
	// 83187B04: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83187B08: 419A0024  beq cr6, 0x83187b2c
	if ctx.cr[6].eq {
	pc = 0x83187B2C; continue 'dispatch;
	}
	// 83187B0C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83187B10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83187B14: 60840F15  ori r4, r4, 0xf15
	ctx.r[4].u64 = ctx.r[4].u64 | 3861;
	// 83187B18: 4BFFF9E1  bl 0x831874f8
	ctx.lr = 0x83187B1C;
	sub_831874F8(ctx, base);
	// 83187B1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83187B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83187B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83187B28: 4E800020  blr
	return;
	// 83187B2C: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83187B30: 39471F38  addi r10, r7, 0x1f38
	ctx.r[10].s64 = ctx.r[7].s64 + 7992;
	// 83187B34: 390BC6B0  addi r8, r11, -0x3950
	ctx.r[8].s64 = ctx.r[11].s64 + -14672;
	// 83187B38: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 83187B3C: 39680010  addi r11, r8, 0x10
	ctx.r[11].s64 = ctx.r[8].s64 + 16;
	// 83187B40: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83187B44: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83187B48: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83187B4C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83187B50: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83187B54: 4200FFF0  bdnz 0x83187b44
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83187B44; continue 'dispatch;
	}
	// 83187B58: E9680000  ld r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 83187B5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83187B60: F9671F5C  std r11, 0x1f5c(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8028 as u32), ctx.r[11].u64 ) };
	// 83187B64: 81671FC4  lwz r11, 0x1fc4(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8132 as u32) ) } as u64;
	// 83187B68: 91671F64  stw r11, 0x1f64(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8036 as u32), ctx.r[11].u32 ) };
	// 83187B6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83187B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83187B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83187B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187B80 size=84
    let mut pc: u32 = 0x83187B80;
    'dispatch: loop {
        match pc {
            0x83187B80 => {
    //   block [0x83187B80..0x83187BD4)
	// 83187B80: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83187B84: 39431F38  addi r10, r3, 0x1f38
	ctx.r[10].s64 = ctx.r[3].s64 + 7992;
	// 83187B88: 396BC6AC  addi r11, r11, -0x3954
	ctx.r[11].s64 = ctx.r[11].s64 + -14676;
	// 83187B8C: 390B0014  addi r8, r11, 0x14
	ctx.r[8].s64 = ctx.r[11].s64 + 20;
	// 83187B90: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 83187B94: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 83187B98: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83187B9C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83187BA0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83187BA4: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83187BA8: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 83187BAC: 4200FFF0  bdnz 0x83187b9c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83187B9C; continue 'dispatch;
	}
	// 83187BB0: E9431F5C  ld r10, 0x1f5c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8028 as u32) ) };
	// 83187BB4: F94B0004  std r10, 4(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u64 ) };
	// 83187BB8: 81431F7C  lwz r10, 0x1f7c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8060 as u32) ) } as u64;
	// 83187BBC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83187BC0: 81431F80  lwz r10, 0x1f80(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8064 as u32) ) } as u64;
	// 83187BC4: 914B0038  stw r10, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 83187BC8: 81431F84  lwz r10, 0x1f84(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8068 as u32) ) } as u64;
	// 83187BCC: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83187BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83187BD8 size=128
    let mut pc: u32 = 0x83187BD8;
    'dispatch: loop {
        match pc {
            0x83187BD8 => {
    //   block [0x83187BD8..0x83187C58)
	// 83187BD8: 3964000F  addi r11, r4, 0xf
	ctx.r[11].s64 = ctx.r[4].s64 + 15;
	// 83187BDC: 90660008  stw r3, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83187BE0: 3925000F  addi r9, r5, 0xf
	ctx.r[9].s64 = ctx.r[5].s64 + 15;
	// 83187BE4: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 83187BE8: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83187BEC: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83187BF0: 394B007F  addi r10, r11, 0x7f
	ctx.r[10].s64 = ctx.r[11].s64 + 127;
	// 83187BF4: 7D4A3E70  srawi r10, r10, 7
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 7) as i64;
	// 83187BF8: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 83187BFC: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83187C00: 55483830  slwi r8, r10, 7
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(7);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83187C04: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83187C08: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 83187C0C: 7D6B3E70  srawi r11, r11, 7
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 7) as i64;
	// 83187C10: B106000E  sth r8, 0xe(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(14 as u32), ctx.r[8].u16 ) };
	// 83187C14: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83187C18: 7D292670  srawi r9, r9, 4
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 4) as i64;
	// 83187C1C: 55683830  slwi r8, r11, 7
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83187C20: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 83187C24: 7D4951D6  mullw r10, r9, r10
	ctx.r[10].s64 = (ctx.r[9].s32 as i64) * (ctx.r[10].s32 as i64);
	// 83187C28: B106000C  sth r8, 0xc(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[8].u16 ) };
	// 83187C2C: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83187C30: 554A5828  slwi r10, r10, 0xb
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(11);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83187C34: 7D290E70  srawi r9, r9, 1
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 1) as i64;
	// 83187C38: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 83187C3C: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 83187C40: 7D6959D6  mullw r11, r9, r11
	ctx.r[11].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 83187C44: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83187C48: 556B3830  slwi r11, r11, 7
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83187C4C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83187C50: 91660004  stw r11, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83187C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


