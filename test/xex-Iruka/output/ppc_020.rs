pub fn sub_82262960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82262960 size=144
    let mut pc: u32 = 0x82262960;
    'dispatch: loop {
        match pc {
            0x82262960 => {
    //   block [0x82262960..0x82262974)
	// 82262960: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82262964: 419A0084  beq cr6, 0x822629e8
	if ctx.cr[6].eq {
	pc = 0x822629E8; continue 'dispatch;
	}
	// 82262968: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8226296C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82262970: 394A9D14  addi r10, r10, -0x62ec
	ctx.r[10].s64 = ctx.r[10].s64 + -25324;
	pc = 0x82262974; continue 'dispatch;
            }
            0x82262974 => {
    //   block [0x82262974..0x82262998)
	// 82262974: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82262978: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226297C: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82262980: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82262984: 41820014  beq 0x82262998
	if ctx.cr[0].eq {
	pc = 0x82262998; continue 'dispatch;
	}
	// 82262988: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226298C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82262990: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82262994: 419AFFE0  beq cr6, 0x82262974
	if ctx.cr[6].eq {
	pc = 0x82262974; continue 'dispatch;
	}
	pc = 0x82262998; continue 'dispatch;
            }
            0x82262998 => {
    //   block [0x82262998..0x822629A8)
	// 82262998: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8226299C: 4082000C  bne 0x822629a8
	if !ctx.cr[0].eq {
	pc = 0x822629A8; continue 'dispatch;
	}
	// 822629A0: 60840002  ori r4, r4, 2
	ctx.r[4].u64 = ctx.r[4].u64 | 2;
	// 822629A4: 48000040  b 0x822629e4
	pc = 0x822629E4; continue 'dispatch;
            }
            0x822629A8 => {
    //   block [0x822629A8..0x822629B4)
	// 822629A8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 822629AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822629B0: 394A9D00  addi r10, r10, -0x6300
	ctx.r[10].s64 = ctx.r[10].s64 + -25344;
	pc = 0x822629B4; continue 'dispatch;
            }
            0x822629B4 => {
    //   block [0x822629B4..0x822629D8)
	// 822629B4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822629B8: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822629BC: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 822629C0: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 822629C4: 41820014  beq 0x822629d8
	if ctx.cr[0].eq {
	pc = 0x822629D8; continue 'dispatch;
	}
	// 822629C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 822629CC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822629D0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 822629D4: 419AFFE0  beq cr6, 0x822629b4
	if ctx.cr[6].eq {
	pc = 0x822629B4; continue 'dispatch;
	}
	pc = 0x822629D8; continue 'dispatch;
            }
            0x822629D8 => {
    //   block [0x822629D8..0x822629E4)
	// 822629D8: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 822629DC: 4082000C  bne 0x822629e8
	if !ctx.cr[0].eq {
	pc = 0x822629E8; continue 'dispatch;
	}
	// 822629E0: 60840100  ori r4, r4, 0x100
	ctx.r[4].u64 = ctx.r[4].u64 | 256;
	pc = 0x822629E4; continue 'dispatch;
            }
            0x822629E4 => {
    //   block [0x822629E4..0x822629E8)
	// 822629E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x822629E8; continue 'dispatch;
            }
            0x822629E8 => {
    //   block [0x822629E8..0x822629F0)
	// 822629E8: 48007CF4  b 0x8226a6dc
	crate::xam::XamLoaderLaunchTitle(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822629F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822629F0 size=108
    let mut pc: u32 = 0x822629F0;
    'dispatch: loop {
        match pc {
            0x822629F0 => {
    //   block [0x822629F0..0x82262A30)
	// 822629F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822629F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822629F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822629FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82262A00: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82262A04: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82262A08: 48007D65  bl 0x8226a76c
	ctx.lr = 0x82262A0C;
	// extern call 0x8226A76C → crate::xboxkrnl::RtlInitUnicodeString
	crate::xboxkrnl::RtlInitUnicodeString(ctx, base);
	// 82262A0C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82262A10: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82262A14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82262A18: 48007D45  bl 0x8226a75c
	ctx.lr = 0x82262A1C;
	// extern call 0x8226A75C → crate::xboxkrnl::RtlUnicodeStringToAnsiString
	crate::xboxkrnl::RtlUnicodeStringToAnsiString(ctx, base);
	// 82262A1C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82262A20: 40800010  bge 0x82262a30
	if !ctx.cr[0].lt {
	pc = 0x82262A30; continue 'dispatch;
	}
	// 82262A24: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82262A28: 396B0F75  addi r11, r11, 0xf75
	ctx.r[11].s64 = ctx.r[11].s64 + 3957;
	// 82262A2C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	pc = 0x82262A30; continue 'dispatch;
            }
            0x82262A30 => {
    //   block [0x82262A30..0x82262A48)
	// 82262A30: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82262A34: 4BFFFDED  bl 0x82262820
	ctx.lr = 0x82262A38;
	sub_82262820(ctx, base);
	// 82262A38: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82262A3C: 4198000C  blt cr6, 0x82262a48
	if ctx.cr[6].lt {
	pc = 0x82262A48; continue 'dispatch;
	}
	// 82262A40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82262A44: 48007D09  bl 0x8226a74c
	ctx.lr = 0x82262A48;
	// extern call 0x8226A74C → crate::xboxkrnl::RtlFreeAnsiString
	crate::xboxkrnl::RtlFreeAnsiString(ctx, base);
	pc = 0x82262A48; continue 'dispatch;
            }
            0x82262A48 => {
    //   block [0x82262A48..0x82262A5C)
	// 82262A48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82262A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82262A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82262A54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82262A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82262A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82262A60 size=60
    let mut pc: u32 = 0x82262A60;
    'dispatch: loop {
        match pc {
            0x82262A60 => {
    //   block [0x82262A60..0x82262A84)
	// 82262A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82262A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82262A68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82262A6C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82262A70: 48007D0D  bl 0x8226a77c
	ctx.lr = 0x82262A74;
	// extern call 0x8226A77C → crate::xboxkrnl::NtFlushBuffersFile
	crate::xboxkrnl::NtFlushBuffersFile(ctx, base);
	// 82262A74: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82262A78: 4180000C  blt 0x82262a84
	if ctx.cr[0].lt {
	pc = 0x82262A84; continue 'dispatch;
	}
	// 82262A7C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82262A80: 4800000C  b 0x82262a8c
	pc = 0x82262A8C; continue 'dispatch;
            }
            0x82262A84 => {
    //   block [0x82262A84..0x82262A8C)
	// 82262A84: 4BE24BED  bl 0x82087670
	ctx.lr = 0x82262A88;
	sub_82087670(ctx, base);
	// 82262A88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82262A8C; continue 'dispatch;
            }
            0x82262A8C => {
    //   block [0x82262A8C..0x82262A9C)
	// 82262A8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82262A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82262A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82262A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82262AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82262AA0 size=192
    let mut pc: u32 = 0x82262AA0;
    'dispatch: loop {
        match pc {
            0x82262AA0 => {
    //   block [0x82262AA0..0x82262ACC)
	// 82262AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82262AA4: 4BE2BFF5  bl 0x8208ea98
	ctx.lr = 0x82262AA8;
	sub_8208EA60(ctx, base);
	// 82262AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82262AAC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82262AB0: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82262AB4: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82262AB8: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82262ABC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82262AC0: 419A000C  beq cr6, 0x82262acc
	if ctx.cr[6].eq {
	pc = 0x82262ACC; continue 'dispatch;
	}
	// 82262AC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82262AC8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82262ACC; continue 'dispatch;
            }
            0x82262ACC => {
    //   block [0x82262ACC..0x82262AE8)
	// 82262ACC: 2B03FDE9  cmplwi cr6, r3, 0xfde9
	ctx.cr[6].compare_u32(ctx.r[3].u32, 65001 as u32, &mut ctx.xer);
	// 82262AD0: 409A0018  bne cr6, 0x82262ae8
	if !ctx.cr[6].eq {
	pc = 0x82262AE8; continue 'dispatch;
	}
	// 82262AD4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82262AD8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82262ADC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82262AE0: 48000371  bl 0x82262e50
	ctx.lr = 0x82262AE4;
	sub_82262E50(ctx, base);
	// 82262AE4: 48000074  b 0x82262b58
	pc = 0x82262B58; continue 'dispatch;
            }
            0x82262AE8 => {
    //   block [0x82262AE8..0x82262B00)
	// 82262AE8: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 82262AEC: 409A0014  bne cr6, 0x82262b00
	if !ctx.cr[6].eq {
	pc = 0x82262B00; continue 'dispatch;
	}
	// 82262AF0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82262AF4: 4BE2E4DD  bl 0x82090fd0
	ctx.lr = 0x82262AF8;
	sub_82090FD0(ctx, base);
	// 82262AF8: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82262AFC: 48000008  b 0x82262b04
	pc = 0x82262B04; continue 'dispatch;
            }
            0x82262B00 => {
    //   block [0x82262B00..0x82262B04)
	// 82262B00: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	pc = 0x82262B04; continue 'dispatch;
            }
            0x82262B04 => {
    //   block [0x82262B04..0x82262B0C)
	// 82262B04: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82262B08: 409A000C  bne cr6, 0x82262b14
	if !ctx.cr[6].eq {
	pc = 0x82262B14; continue 'dispatch;
	}
	pc = 0x82262B0C; continue 'dispatch;
            }
            0x82262B0C => {
    //   block [0x82262B0C..0x82262B14)
	// 82262B0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82262B10: 48000048  b 0x82262b58
	pc = 0x82262B58; continue 'dispatch;
            }
            0x82262B14 => {
    //   block [0x82262B14..0x82262B4C)
	// 82262B14: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82262B18: 41980034  blt cr6, 0x82262b4c
	if ctx.cr[6].lt {
	pc = 0x82262B4C; continue 'dispatch;
	}
	// 82262B1C: 7F1EF800  cmpw cr6, r30, r31
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82262B20: 4198002C  blt cr6, 0x82262b4c
	if ctx.cr[6].lt {
	pc = 0x82262B4C; continue 'dispatch;
	}
	// 82262B24: 57E7083C  slwi r7, r31, 1
	ctx.r[7].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82262B28: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82262B2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82262B30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82262B34: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82262B38: 48007A75  bl 0x8226a5ac
	ctx.lr = 0x82262B3C;
	// extern call 0x8226A5AC → crate::xboxkrnl::RtlUnicodeToMultiByteN
	crate::xboxkrnl::RtlUnicodeToMultiByteN(ctx, base);
	// 82262B3C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82262B40: 4080FFCC  bge 0x82262b0c
	if !ctx.cr[0].lt {
	pc = 0x82262B0C; continue 'dispatch;
	}
	// 82262B44: 48007A89  bl 0x8226a5cc
	ctx.lr = 0x82262B48;
	// extern call 0x8226A5CC → crate::xboxkrnl::RtlNtStatusToDosError
	crate::xboxkrnl::RtlNtStatusToDosError(ctx, base);
	// 82262B48: 48000008  b 0x82262b50
	pc = 0x82262B50; continue 'dispatch;
            }
            0x82262B4C => {
    //   block [0x82262B4C..0x82262B50)
	// 82262B4C: 3860007A  li r3, 0x7a
	ctx.r[3].s64 = 122;
	pc = 0x82262B50; continue 'dispatch;
            }
            0x82262B50 => {
    //   block [0x82262B50..0x82262B58)
	// 82262B50: 4BE24AF9  bl 0x82087648
	ctx.lr = 0x82262B54;
	sub_82087648(ctx, base);
	// 82262B54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82262B58; continue 'dispatch;
            }
            0x82262B58 => {
    //   block [0x82262B58..0x82262B60)
	// 82262B58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82262B5C: 4BE2BF8C  b 0x8208eae8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82262B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82262B60 size=100
    let mut pc: u32 = 0x82262B60;
    'dispatch: loop {
        match pc {
            0x82262B60 => {
    //   block [0x82262B60..0x82262BAC)
	// 82262B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82262B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82262B68: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82262B6C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82262B70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82262B74: 480078B9  bl 0x8226a42c
	ctx.lr = 0x82262B78;
	// extern call 0x8226A42C → crate::xboxkrnl::RtlInitAnsiString
	crate::xboxkrnl::RtlInitAnsiString(ctx, base);
	// 82262B78: 3960FFFD  li r11, -3
	ctx.r[11].s64 = -3;
	// 82262B7C: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 82262B80: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82262B84: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82262B88: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82262B8C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82262B90: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82262B94: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82262B98: 48007BF5  bl 0x8226a78c
	ctx.lr = 0x82262B9C;
	// extern call 0x8226A78C → crate::xboxkrnl::NtQueryFullAttributesFile
	crate::xboxkrnl::NtQueryFullAttributesFile(ctx, base);
	// 82262B9C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82262BA0: 4180000C  blt 0x82262bac
	if ctx.cr[0].lt {
	pc = 0x82262BAC; continue 'dispatch;
	}
	// 82262BA4: 806100A0  lwz r3, 0xa0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82262BA8: 4800000C  b 0x82262bb4
	pc = 0x82262BB4; continue 'dispatch;
            }
            0x82262BAC => {
    //   block [0x82262BAC..0x82262BB4)
	// 82262BAC: 4BE24AC5  bl 0x82087670
	ctx.lr = 0x82262BB0;
	sub_82087670(ctx, base);
	// 82262BB0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	pc = 0x82262BB4; continue 'dispatch;
            }
            0x82262BB4 => {
    //   block [0x82262BB4..0x82262BC4)
	// 82262BB4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82262BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82262BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82262BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82262BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82262BC8 size=140
    let mut pc: u32 = 0x82262BC8;
    'dispatch: loop {
        match pc {
            0x82262BC8 => {
    //   block [0x82262BC8..0x82262C24)
	// 82262BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82262BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82262BD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82262BD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82262BD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82262BDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82262BE0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82262BE4: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82262BE8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82262BEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82262BF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82262BF4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82262BF8: 48000389  bl 0x82262f80
	ctx.lr = 0x82262BFC;
	sub_82262F80(ctx, base);
	// 82262BFC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82262C00: 40820038  bne 0x82262c38
	if !ctx.cr[0].eq {
	pc = 0x82262C38; continue 'dispatch;
	}
	// 82262C04: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82262C08: 419A0024  beq cr6, 0x82262c2c
	if ctx.cr[6].eq {
	pc = 0x82262C2C; continue 'dispatch;
	}
	// 82262C0C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82262C10: 419A0014  beq cr6, 0x82262c24
	if ctx.cr[6].eq {
	pc = 0x82262C24; continue 'dispatch;
	}
	// 82262C14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82262C18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82262C1C: 4BE2E115  bl 0x82090d30
	ctx.lr = 0x82262C20;
	sub_82090D30(ctx, base);
	// 82262C20: 4800001C  b 0x82262c3c
	pc = 0x82262C3C; continue 'dispatch;
            }
            0x82262C24 => {
    //   block [0x82262C24..0x82262C2C)
	// 82262C24: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82262C28: 48000014  b 0x82262c3c
	pc = 0x82262C3C; continue 'dispatch;
            }
            0x82262C2C => {
    //   block [0x82262C2C..0x82262C38)
	// 82262C2C: 217F0000  subfic r11, r31, 0
	ctx.xer.ca = ctx.r[31].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[31].s64;
	// 82262C30: 7C6B5910  subfe r3, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[3].u32 = res;
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82262C34: 48000008  b 0x82262c3c
	pc = 0x82262C3C; continue 'dispatch;
            }
            0x82262C38 => {
    //   block [0x82262C38..0x82262C3C)
	// 82262C38: 3863FFFE  addi r3, r3, -2
	ctx.r[3].s64 = ctx.r[3].s64 + -2;
	pc = 0x82262C3C; continue 'dispatch;
            }
            0x82262C3C => {
    //   block [0x82262C3C..0x82262C54)
	// 82262C3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82262C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82262C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82262C48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82262C4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82262C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82262C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82262C58 size=212
    let mut pc: u32 = 0x82262C58;
    'dispatch: loop {
        match pc {
            0x82262C58 => {
    //   block [0x82262C58..0x82262C90)
	// 82262C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82262C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82262C60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82262C64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82262C68: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82262C6C: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82262C70: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82262C74: 2B03FDE9  cmplwi cr6, r3, 0xfde9
	ctx.cr[6].compare_u32(ctx.r[3].u32, 65001 as u32, &mut ctx.xer);
	// 82262C78: 409A0018  bne cr6, 0x82262c90
	if !ctx.cr[6].eq {
	pc = 0x82262C90; continue 'dispatch;
	}
	// 82262C7C: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 82262C80: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 82262C84: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82262C88: 480004D1  bl 0x82263158
	ctx.lr = 0x82262C8C;
	sub_82263158(ctx, base);
	// 82262C8C: 4800008C  b 0x82262d18
	pc = 0x82262D18; continue 'dispatch;
            }
            0x82262C90 => {
    //   block [0x82262C90..0x82262C9C)
	// 82262C90: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 82262C94: 409A002C  bne cr6, 0x82262cc0
	if !ctx.cr[6].eq {
	pc = 0x82262CC0; continue 'dispatch;
	}
	// 82262C98: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	pc = 0x82262C9C; continue 'dispatch;
            }
            0x82262C9C => {
    //   block [0x82262C9C..0x82262CC0)
	// 82262C9C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82262CA0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82262CA4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82262CA8: 409AFFF4  bne cr6, 0x82262c9c
	if !ctx.cr[6].eq {
	pc = 0x82262C9C; continue 'dispatch;
	}
	// 82262CAC: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82262CB0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82262CB4: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82262CB8: 3BEA0001  addi r31, r10, 1
	ctx.r[31].s64 = ctx.r[10].s64 + 1;
	// 82262CBC: 48000008  b 0x82262cc4
	pc = 0x82262CC4; continue 'dispatch;
            }
            0x82262CC0 => {
    //   block [0x82262CC0..0x82262CC4)
	// 82262CC0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	pc = 0x82262CC4; continue 'dispatch;
            }
            0x82262CC4 => {
    //   block [0x82262CC4..0x82262CCC)
	// 82262CC4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82262CC8: 409A000C  bne cr6, 0x82262cd4
	if !ctx.cr[6].eq {
	pc = 0x82262CD4; continue 'dispatch;
	}
	pc = 0x82262CCC; continue 'dispatch;
            }
            0x82262CCC => {
    //   block [0x82262CCC..0x82262CD4)
	// 82262CCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82262CD0: 48000048  b 0x82262d18
	pc = 0x82262D18; continue 'dispatch;
            }
            0x82262CD4 => {
    //   block [0x82262CD4..0x82262D0C)
	// 82262CD4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82262CD8: 41980034  blt cr6, 0x82262d0c
	if ctx.cr[6].lt {
	pc = 0x82262D0C; continue 'dispatch;
	}
	// 82262CDC: 7F08F800  cmpw cr6, r8, r31
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82262CE0: 4198002C  blt cr6, 0x82262d0c
	if ctx.cr[6].lt {
	pc = 0x82262D0C; continue 'dispatch;
	}
	// 82262CE4: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82262CE8: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82262CEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82262CF0: 5504083C  slwi r4, r8, 1
	ctx.r[4].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82262CF4: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82262CF8: 48007AA5  bl 0x8226a79c
	ctx.lr = 0x82262CFC;
	// extern call 0x8226A79C → crate::xboxkrnl::RtlMultiByteToUnicodeN
	crate::xboxkrnl::RtlMultiByteToUnicodeN(ctx, base);
	// 82262CFC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82262D00: 4080FFCC  bge 0x82262ccc
	if !ctx.cr[0].lt {
	pc = 0x82262CCC; continue 'dispatch;
	}
	// 82262D04: 480078C9  bl 0x8226a5cc
	ctx.lr = 0x82262D08;
	// extern call 0x8226A5CC → crate::xboxkrnl::RtlNtStatusToDosError
	crate::xboxkrnl::RtlNtStatusToDosError(ctx, base);
	// 82262D08: 48000008  b 0x82262d10
	pc = 0x82262D10; continue 'dispatch;
            }
            0x82262D0C => {
    //   block [0x82262D0C..0x82262D10)
	// 82262D0C: 3860007A  li r3, 0x7a
	ctx.r[3].s64 = 122;
	pc = 0x82262D10; continue 'dispatch;
            }
            0x82262D10 => {
    //   block [0x82262D10..0x82262D18)
	// 82262D10: 4BE24939  bl 0x82087648
	ctx.lr = 0x82262D14;
	sub_82087648(ctx, base);
	// 82262D14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82262D18; continue 'dispatch;
            }
            0x82262D18 => {
    //   block [0x82262D18..0x82262D2C)
	// 82262D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82262D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82262D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82262D24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82262D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82262D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82262D30 size=140
    let mut pc: u32 = 0x82262D30;
    'dispatch: loop {
        match pc {
            0x82262D30 => {
    //   block [0x82262D30..0x82262D84)
	// 82262D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82262D34: 4BE2BD69  bl 0x8208ea9c
	ctx.lr = 0x82262D38;
	sub_8208EA60(ctx, base);
	// 82262D38: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82262D3C: 3D608227  lis r11, -0x7dd9
	ctx.r[11].s64 = -2111373312;
	// 82262D40: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82262D44: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82262D48: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82262D4C: 38E00022  li r7, 0x22
	ctx.r[7].s64 = 34;
	// 82262D50: 816B1A0C  lwz r11, 0x1a0c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6668 as u32) ) } as u64;
	// 82262D54: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 82262D58: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82262D5C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82262D60: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82262D64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82262D68: 4E800421  bctrl
	ctx.lr = 0x82262D6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82262D6C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82262D70: 4180003C  blt 0x82262dac
	if ctx.cr[0].lt {
	pc = 0x82262DAC; continue 'dispatch;
	}
	// 82262D74: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82262D78: 419A000C  beq cr6, 0x82262d84
	if ctx.cr[6].eq {
	pc = 0x82262D84; continue 'dispatch;
	}
	// 82262D7C: E9610060  ld r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82262D80: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
            }
            0x82262D84 => {
    //   block [0x82262D84..0x82262D94)
	// 82262D84: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82262D88: 419A000C  beq cr6, 0x82262d94
	if ctx.cr[6].eq {
	pc = 0x82262D94; continue 'dispatch;
	}
	// 82262D8C: E9610068  ld r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82262D90: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	pc = 0x82262D94; continue 'dispatch;
            }
            0x82262D94 => {
    //   block [0x82262D94..0x82262DA4)
	// 82262D94: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82262D98: 419A000C  beq cr6, 0x82262da4
	if ctx.cr[6].eq {
	pc = 0x82262DA4; continue 'dispatch;
	}
	// 82262D9C: E9610070  ld r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 82262DA0: F97D0000  std r11, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	pc = 0x82262DA4; continue 'dispatch;
            }
            0x82262DA4 => {
    //   block [0x82262DA4..0x82262DAC)
	// 82262DA4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82262DA8: 4800000C  b 0x82262db4
	pc = 0x82262DB4; continue 'dispatch;
            }
            0x82262DAC => {
    //   block [0x82262DAC..0x82262DB4)
	// 82262DAC: 4BE248C5  bl 0x82087670
	ctx.lr = 0x82262DB0;
	sub_82087670(ctx, base);
	// 82262DB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82262DB4; continue 'dispatch;
            }
            0x82262DB4 => {
    //   block [0x82262DB4..0x82262DBC)
	// 82262DB4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82262DB8: 4BE2BD34  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82262DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82262DC0 size=16
    let mut pc: u32 = 0x82262DC0;
    'dispatch: loop {
        match pc {
            0x82262DC0 => {
    //   block [0x82262DC0..0x82262DD0)
	// 82262DC0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82262DC4: 816B05C4  lwz r11, 0x5c4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1476 as u32) ) } as u64;
	// 82262DC8: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82262DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82262DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82262DD0 size=16
    let mut pc: u32 = 0x82262DD0;
    'dispatch: loop {
        match pc {
            0x82262DD0 => {
    //   block [0x82262DD0..0x82262DE0)
	// 82262DD0: A0830000  lhz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82262DD4: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82262DD8: 48000578  b 0x82263350
	crate::recompiler::externs::call(ctx, base, 0x82263350);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82262DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82262DE0 size=108
    let mut pc: u32 = 0x82262DE0;
    'dispatch: loop {
        match pc {
            0x82262DE0 => {
    //   block [0x82262DE0..0x82262E2C)
	// 82262DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82262DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82262DE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82262DEC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82262DF0: 3D608227  lis r11, -0x7dd9
	ctx.r[11].s64 = -2111373312;
	// 82262DF4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82262DF8: 38E00022  li r7, 0x22
	ctx.r[7].s64 = 34;
	// 82262DFC: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 82262E00: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82262E04: 816B1A0C  lwz r11, 0x1a0c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6668 as u32) ) } as u64;
	// 82262E08: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82262E0C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82262E10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82262E14: 4E800421  bctrl
	ctx.lr = 0x82262E18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82262E18: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82262E1C: 40800010  bge 0x82262e2c
	if !ctx.cr[0].lt {
	pc = 0x82262E2C; continue 'dispatch;
	}
	// 82262E20: 4BE24851  bl 0x82087670
	ctx.lr = 0x82262E24;
	sub_82087670(ctx, base);
	// 82262E24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82262E28: 48000010  b 0x82262e38
	pc = 0x82262E38; continue 'dispatch;
            }
            0x82262E2C => {
    //   block [0x82262E2C..0x82262E38)
	// 82262E2C: E9610088  ld r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) };
	// 82262E30: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82262E34: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	pc = 0x82262E38; continue 'dispatch;
            }
            0x82262E38 => {
    //   block [0x82262E38..0x82262E4C)
	// 82262E38: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82262E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82262E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82262E44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82262E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82262E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82262E50 size=304
    let mut pc: u32 = 0x82262E50;
    'dispatch: loop {
        match pc {
            0x82262E50 => {
    //   block [0x82262E50..0x82262E7C)
	// 82262E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82262E54: 4BE2BC45  bl 0x8208ea98
	ctx.lr = 0x82262E58;
	sub_8208EA60(ctx, base);
	// 82262E58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82262E5C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82262E60: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82262E64: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82262E68: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82262E6C: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 82262E70: 409A000C  bne cr6, 0x82262e7c
	if !ctx.cr[6].eq {
	pc = 0x82262E7C; continue 'dispatch;
	}
	// 82262E74: 4BE2E15D  bl 0x82090fd0
	ctx.lr = 0x82262E78;
	sub_82090FD0(ctx, base);
	// 82262E78: 38830001  addi r4, r3, 1
	ctx.r[4].s64 = ctx.r[3].s64 + 1;
	pc = 0x82262E7C; continue 'dispatch;
            }
            0x82262E7C => {
    //   block [0x82262E7C..0x82262E88)
	// 82262E7C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82262E80: 409900D4  ble cr6, 0x82262f54
	if !ctx.cr[6].gt {
	pc = 0x82262F54; continue 'dispatch;
	}
	// 82262E84: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	pc = 0x82262E88; continue 'dispatch;
            }
            0x82262E88 => {
    //   block [0x82262E88..0x82262EB4)
	// 82262E88: A15C0000  lhz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82262E8C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82262E90: 2B0B007F  cmplwi cr6, r11, 0x7f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 127 as u32, &mut ctx.xer);
	// 82262E94: 41990020  bgt cr6, 0x82262eb4
	if ctx.cr[6].gt {
	pc = 0x82262EB4; continue 'dispatch;
	}
	// 82262E98: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82262E9C: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82262EA0: 419900A8  bgt cr6, 0x82262f48
	if ctx.cr[6].gt {
	pc = 0x82262F48; continue 'dispatch;
	}
	// 82262EA4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82262EA8: 419A00A0  beq cr6, 0x82262f48
	if ctx.cr[6].eq {
	pc = 0x82262F48; continue 'dispatch;
	}
	// 82262EAC: 995F0000  stb r10, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82262EB0: 48000094  b 0x82262f44
	pc = 0x82262F44; continue 'dispatch;
            }
            0x82262EB4 => {
    //   block [0x82262EB4..0x82262EDC)
	// 82262EB4: 2B0B07FF  cmplwi cr6, r11, 0x7ff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2047 as u32, &mut ctx.xer);
	// 82262EB8: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82262EBC: 41990020  bgt cr6, 0x82262edc
	if ctx.cr[6].gt {
	pc = 0x82262EDC; continue 'dispatch;
	}
	// 82262EC0: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82262EC4: 41990060  bgt cr6, 0x82262f24
	if ctx.cr[6].gt {
	pc = 0x82262F24; continue 'dispatch;
	}
	// 82262EC8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82262ECC: 419A0058  beq cr6, 0x82262f24
	if ctx.cr[6].eq {
	pc = 0x82262F24; continue 'dispatch;
	}
	// 82262ED0: 390000C0  li r8, 0xc0
	ctx.r[8].s64 = 192;
	// 82262ED4: 5148D6FE  rlwimi r8, r10, 0x1a, 0x1b, 0x1f
	ctx.r[8].u64 = (((ctx.r[10].u32).rotate_left(26) as u64) & 0x000000000000001F) | (ctx.r[8].u64 & 0xFFFFFFFFFFFFFFE0);
	// 82262ED8: 48000044  b 0x82262f1c
	pc = 0x82262F1C; continue 'dispatch;
            }
            0x82262EDC => {
    //   block [0x82262EDC..0x82262EFC)
	// 82262EDC: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82262EE0: 4199001C  bgt cr6, 0x82262efc
	if ctx.cr[6].gt {
	pc = 0x82262EFC; continue 'dispatch;
	}
	// 82262EE4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82262EE8: 419A0014  beq cr6, 0x82262efc
	if ctx.cr[6].eq {
	pc = 0x82262EFC; continue 'dispatch;
	}
	// 82262EEC: 390000E0  li r8, 0xe0
	ctx.r[8].s64 = 224;
	// 82262EF0: 5148A73E  rlwimi r8, r10, 0x14, 0x1c, 0x1f
	ctx.r[8].u64 = (((ctx.r[10].u32).rotate_left(20) as u64) & 0x000000000000000F) | (ctx.r[8].u64 & 0xFFFFFFFFFFFFFFF0);
	// 82262EF4: 991F0000  stb r8, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82262EF8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	pc = 0x82262EFC; continue 'dispatch;
            }
            0x82262EFC => {
    //   block [0x82262EFC..0x82262F1C)
	// 82262EFC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82262F00: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82262F04: 41990020  bgt cr6, 0x82262f24
	if ctx.cr[6].gt {
	pc = 0x82262F24; continue 'dispatch;
	}
	// 82262F08: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82262F0C: 419A0018  beq cr6, 0x82262f24
	if ctx.cr[6].eq {
	pc = 0x82262F24; continue 'dispatch;
	}
	// 82262F10: A15C0000  lhz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82262F14: 39000080  li r8, 0x80
	ctx.r[8].s64 = 128;
	// 82262F18: 5148D6BE  rlwimi r8, r10, 0x1a, 0x1a, 0x1f
	ctx.r[8].u64 = (((ctx.r[10].u32).rotate_left(26) as u64) & 0x000000000000003F) | (ctx.r[8].u64 & 0xFFFFFFFFFFFFFFC0);
	pc = 0x82262F1C; continue 'dispatch;
            }
            0x82262F1C => {
    //   block [0x82262F1C..0x82262F24)
	// 82262F1C: 991F0000  stb r8, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82262F20: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	pc = 0x82262F24; continue 'dispatch;
            }
            0x82262F24 => {
    //   block [0x82262F24..0x82262F44)
	// 82262F24: 3BCB0001  addi r30, r11, 1
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	// 82262F28: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82262F2C: 4199001C  bgt cr6, 0x82262f48
	if ctx.cr[6].gt {
	pc = 0x82262F48; continue 'dispatch;
	}
	// 82262F30: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82262F34: 419A0014  beq cr6, 0x82262f48
	if ctx.cr[6].eq {
	pc = 0x82262F48; continue 'dispatch;
	}
	// 82262F38: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82262F3C: 512B3832  rlwimi r11, r9, 7, 0, 0x19
	ctx.r[11].u64 = (((ctx.r[9].u32).rotate_left(7) as u64) & 0x00000000FFFFFFC0) | (ctx.r[11].u64 & 0xFFFFFFFF0000003F);
	// 82262F40: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	pc = 0x82262F44; continue 'dispatch;
            }
            0x82262F44 => {
    //   block [0x82262F44..0x82262F48)
	// 82262F44: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	pc = 0x82262F48; continue 'dispatch;
            }
            0x82262F48 => {
    //   block [0x82262F48..0x82262F54)
	// 82262F48: 3484FFFF  addic. r4, r4, -1
	ctx.xer.ca = (ctx.r[4].u32 > (!(-1 as u32)));
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82262F4C: 3B9C0002  addi r28, r28, 2
	ctx.r[28].s64 = ctx.r[28].s64 + 2;
	// 82262F50: 4082FF38  bne 0x82262e88
	if !ctx.cr[0].eq {
	pc = 0x82262E88; continue 'dispatch;
	}
	pc = 0x82262F54; continue 'dispatch;
            }
            0x82262F54 => {
    //   block [0x82262F54..0x82262F74)
	// 82262F54: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82262F58: 419A001C  beq cr6, 0x82262f74
	if ctx.cr[6].eq {
	pc = 0x82262F74; continue 'dispatch;
	}
	// 82262F5C: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82262F60: 40980014  bge cr6, 0x82262f74
	if !ctx.cr[6].lt {
	pc = 0x82262F74; continue 'dispatch;
	}
	// 82262F64: 3860007A  li r3, 0x7a
	ctx.r[3].s64 = 122;
	// 82262F68: 4BE246E1  bl 0x82087648
	ctx.lr = 0x82262F6C;
	sub_82087648(ctx, base);
	// 82262F6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82262F70: 48000008  b 0x82262f78
	pc = 0x82262F78; continue 'dispatch;
            }
            0x82262F74 => {
    //   block [0x82262F74..0x82262F78)
	// 82262F74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	pc = 0x82262F78; continue 'dispatch;
            }
            0x82262F78 => {
    //   block [0x82262F78..0x82262F80)
	// 82262F78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82262F7C: 4BE2BB6C  b 0x8208eae8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82262F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82262F80 size=468
    let mut pc: u32 = 0x82262F80;
    'dispatch: loop {
        match pc {
            0x82262F80 => {
    //   block [0x82262F80..0x82262FD4)
	// 82262F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82262F84: 4BE2BB11  bl 0x8208ea94
	ctx.lr = 0x82262F88;
	sub_8208EA60(ctx, base);
	// 82262F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82262F8C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82262F90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82262F94: 7D6A58F8  nor r10, r11, r11
	ctx.r[10].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82262F98: 557D07FE  clrlwi r29, r11, 0x1f
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82262F9C: 555EA7FE  rlwinm r30, r10, 0x14, 0x1f, 0x1f
	ctx.r[30].u64 = ctx.r[10].u32 as u64 & 0x00000FFFu64;
	// 82262FA0: 557C077A  rlwinm r28, r11, 0, 0x1d, 0x1d
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82262FA4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82262FA8: 419A0198  beq cr6, 0x82263140
	if ctx.cr[6].eq {
	pc = 0x82263140; continue 'dispatch;
	}
	// 82262FAC: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82262FB0: 419A0190  beq cr6, 0x82263140
	if ctx.cr[6].eq {
	pc = 0x82263140; continue 'dispatch;
	}
	// 82262FB4: 2F05FFFF  cmpwi cr6, r5, -1
	ctx.cr[6].compare_i32(ctx.r[5].s32, -1, &mut ctx.xer);
	// 82262FB8: 41980188  blt cr6, 0x82263140
	if ctx.cr[6].lt {
	pc = 0x82263140; continue 'dispatch;
	}
	// 82262FBC: 2F07FFFF  cmpwi cr6, r7, -1
	ctx.cr[6].compare_i32(ctx.r[7].s32, -1, &mut ctx.xer);
	// 82262FC0: 41980180  blt cr6, 0x82263140
	if ctx.cr[6].lt {
	pc = 0x82263140; continue 'dispatch;
	}
	// 82262FC4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82262FC8: 419A015C  beq cr6, 0x82263124
	if ctx.cr[6].eq {
	pc = 0x82263124; continue 'dispatch;
	}
	// 82262FCC: 3D608228  lis r11, -0x7dd8
	ctx.r[11].s64 = -2111307776;
	// 82262FD0: 3BEB65A8  addi r31, r11, 0x65a8
	ctx.r[31].s64 = ctx.r[11].s64 + 26024;
	pc = 0x82262FD4; continue 'dispatch;
            }
            0x82262FD4 => {
    //   block [0x82262FD4..0x82262FF4)
	// 82262FD4: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82262FD8: 419A0144  beq cr6, 0x8226311c
	if ctx.cr[6].eq {
	pc = 0x8226311C; continue 'dispatch;
	}
	// 82262FDC: 89440000  lbz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82262FE0: 7D4B0775  extsb. r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82262FE4: 40820010  bne 0x82262ff4
	if !ctx.cr[0].eq {
	pc = 0x82262FF4; continue 'dispatch;
	}
	// 82262FE8: 89260000  lbz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82262FEC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82262FF0: 419A0108  beq cr6, 0x822630f8
	if ctx.cr[6].eq {
	pc = 0x822630F8; continue 'dispatch;
	}
	pc = 0x82262FF4; continue 'dispatch;
            }
            0x82262FF4 => {
    //   block [0x82262FF4..0x82263048)
	// 82262FF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82262FF8: 419A0114  beq cr6, 0x8226310c
	if ctx.cr[6].eq {
	pc = 0x8226310C; continue 'dispatch;
	}
	// 82262FFC: 89660000  lbz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82263000: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82263004: 41820110  beq 0x82263114
	if ctx.cr[0].eq {
	pc = 0x82263114; continue 'dispatch;
	}
	// 82263008: 554915BA  rlwinm r9, r10, 2, 0x16, 0x1d
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 8226300C: 557B15BA  rlwinm r27, r11, 2, 0x16, 0x1d
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82263010: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82263014: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82263018: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8226301C: 7D09F82E  lwzx r8, r9, r31
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82263020: 7D3BF82E  lwzx r9, r27, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82263024: 419A0060  beq cr6, 0x82263084
	if ctx.cr[6].eq {
	pc = 0x82263084; continue 'dispatch;
	}
	// 82263028: 551B03DF  rlwinm. r27, r8, 0, 0xf, 0xf
	ctx.r[27].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8226302C: 4182003C  beq 0x82263068
	if ctx.cr[0].eq {
	pc = 0x82263068; continue 'dispatch;
	}
	// 82263030: 552B03DF  rlwinm. r11, r9, 0, 0xf, 0xf
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82263034: 41820020  beq 0x82263054
	if ctx.cr[0].eq {
	pc = 0x82263054; continue 'dispatch;
	}
	// 82263038: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8226303C: 4098000C  bge cr6, 0x82263048
	if !ctx.cr[6].lt {
	pc = 0x82263048; continue 'dispatch;
	}
	// 82263040: 38600007  li r3, 7
	ctx.r[3].s64 = 7;
	// 82263044: 48000098  b 0x822630dc
	pc = 0x822630DC; continue 'dispatch;
            }
            0x82263048 => {
    //   block [0x82263048..0x82263054)
	// 82263048: 40990094  ble cr6, 0x822630dc
	if !ctx.cr[6].gt {
	pc = 0x822630DC; continue 'dispatch;
	}
	// 8226304C: 3860000B  li r3, 0xb
	ctx.r[3].s64 = 11;
	// 82263050: 4800008C  b 0x822630dc
	pc = 0x822630DC; continue 'dispatch;
            }
            0x82263054 => {
    //   block [0x82263054..0x82263060)
	// 82263054: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82263058: 409A0008  bne cr6, 0x82263060
	if !ctx.cr[6].eq {
	pc = 0x82263060; continue 'dispatch;
	}
	// 8226305C: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	pc = 0x82263060; continue 'dispatch;
            }
            0x82263060 => {
    //   block [0x82263060..0x82263068)
	// 82263060: 60630001  ori r3, r3, 1
	ctx.r[3].u64 = ctx.r[3].u64 | 1;
	// 82263064: 48000080  b 0x822630e4
	pc = 0x822630E4; continue 'dispatch;
            }
            0x82263068 => {
    //   block [0x82263068..0x8226307C)
	// 82263068: 553B03DF  rlwinm. r27, r9, 0, 0xf, 0xf
	ctx.r[27].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8226306C: 41820018  beq 0x82263084
	if ctx.cr[0].eq {
	pc = 0x82263084; continue 'dispatch;
	}
	// 82263070: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82263074: 409A0008  bne cr6, 0x8226307c
	if !ctx.cr[6].eq {
	pc = 0x8226307C; continue 'dispatch;
	}
	// 82263078: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	pc = 0x8226307C; continue 'dispatch;
            }
            0x8226307C => {
    //   block [0x8226307C..0x82263084)
	// 8226307C: 60630002  ori r3, r3, 2
	ctx.r[3].u64 = ctx.r[3].u64 | 2;
	// 82263080: 4800001C  b 0x8226309c
	pc = 0x8226309C; continue 'dispatch;
            }
            0x82263084 => {
    //   block [0x82263084..0x8226309C)
	// 82263084: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82263088: 419A0020  beq cr6, 0x822630a8
	if ctx.cr[6].eq {
	pc = 0x822630A8; continue 'dispatch;
	}
	// 8226308C: 551B039D  rlwinm. r27, r8, 0, 0xe, 0xe
	ctx.r[27].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82263090: 40820054  bne 0x822630e4
	if !ctx.cr[0].eq {
	pc = 0x822630E4; continue 'dispatch;
	}
	// 82263094: 553B039D  rlwinm. r27, r9, 0, 0xe, 0xe
	ctx.r[27].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82263098: 41820010  beq 0x822630a8
	if ctx.cr[0].eq {
	pc = 0x822630A8; continue 'dispatch;
	}
	pc = 0x8226309C; continue 'dispatch;
            }
            0x8226309C => {
    //   block [0x8226309C..0x822630A8)
	// 8226309C: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 822630A0: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 822630A4: 48000048  b 0x822630ec
	pc = 0x822630EC; continue 'dispatch;
            }
            0x822630A8 => {
    //   block [0x822630A8..0x822630C0)
	// 822630A8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822630AC: 419A0024  beq cr6, 0x822630d0
	if ctx.cr[6].eq {
	pc = 0x822630D0; continue 'dispatch;
	}
	// 822630B0: 551B02D7  rlwinm. r27, r8, 0, 0xb, 0xb
	ctx.r[27].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 822630B4: 4182000C  beq 0x822630c0
	if ctx.cr[0].eq {
	pc = 0x822630C0; continue 'dispatch;
	}
	// 822630B8: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 822630BC: 65480008  oris r8, r10, 8
	ctx.r[8].u64 = ctx.r[10].u64 | 524288;
	pc = 0x822630C0; continue 'dispatch;
            }
            0x822630C0 => {
    //   block [0x822630C0..0x822630D0)
	// 822630C0: 552A02D7  rlwinm. r10, r9, 0, 0xb, 0xb
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822630C4: 4182000C  beq 0x822630d0
	if ctx.cr[0].eq {
	pc = 0x822630D0; continue 'dispatch;
	}
	// 822630C8: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 822630CC: 65690008  oris r9, r11, 8
	ctx.r[9].u64 = ctx.r[11].u64 | 524288;
	pc = 0x822630D0; continue 'dispatch;
            }
            0x822630D0 => {
    //   block [0x822630D0..0x822630DC)
	// 822630D0: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822630D4: 41980038  blt cr6, 0x8226310c
	if ctx.cr[6].lt {
	pc = 0x8226310C; continue 'dispatch;
	}
	// 822630D8: 4199003C  bgt cr6, 0x82263114
	if ctx.cr[6].gt {
	pc = 0x82263114; continue 'dispatch;
	}
	pc = 0x822630DC; continue 'dispatch;
            }
            0x822630DC => {
    //   block [0x822630DC..0x822630E4)
	// 822630DC: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 822630E0: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	pc = 0x822630E4; continue 'dispatch;
            }
            0x822630E4 => {
    //   block [0x822630E4..0x822630EC)
	// 822630E4: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 822630E8: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	pc = 0x822630EC; continue 'dispatch;
            }
            0x822630EC => {
    //   block [0x822630EC..0x822630F8)
	// 822630EC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 822630F0: 409AFEE4  bne cr6, 0x82262fd4
	if !ctx.cr[6].eq {
	pc = 0x82262FD4; continue 'dispatch;
	}
	// 822630F4: 48000030  b 0x82263124
	pc = 0x82263124; continue 'dispatch;
            }
            0x822630F8 => {
    //   block [0x822630F8..0x8226310C)
	// 822630F8: 3D608228  lis r11, -0x7dd8
	ctx.r[11].s64 = -2111307776;
	// 822630FC: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82263100: 396B69A8  addi r11, r11, 0x69a8
	ctx.r[11].s64 = ctx.r[11].s64 + 27048;
	// 82263104: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82263108: 48000044  b 0x8226314c
	pc = 0x8226314C; continue 'dispatch;
            }
            0x8226310C => {
    //   block [0x8226310C..0x82263114)
	// 8226310C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82263110: 4800003C  b 0x8226314c
	pc = 0x8226314C; continue 'dispatch;
            }
            0x82263114 => {
    //   block [0x82263114..0x8226311C)
	// 82263114: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82263118: 48000034  b 0x8226314c
	pc = 0x8226314C; continue 'dispatch;
            }
            0x8226311C => {
    //   block [0x8226311C..0x82263124)
	// 8226311C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82263120: 409A000C  bne cr6, 0x8226312c
	if !ctx.cr[6].eq {
	pc = 0x8226312C; continue 'dispatch;
	}
	pc = 0x82263124; continue 'dispatch;
            }
            0x82263124 => {
    //   block [0x82263124..0x8226312C)
	// 82263124: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82263128: 419AFFD0  beq cr6, 0x822630f8
	if ctx.cr[6].eq {
	pc = 0x822630F8; continue 'dispatch;
	}
	pc = 0x8226312C; continue 'dispatch;
            }
            0x8226312C => {
    //   block [0x8226312C..0x82263140)
	// 8226312C: 21650000  subfic r11, r5, 0
	ctx.xer.ca = ctx.r[5].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[5].s64;
	// 82263130: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82263134: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82263138: 386B0003  addi r3, r11, 3
	ctx.r[3].s64 = ctx.r[11].s64 + 3;
	// 8226313C: 48000010  b 0x8226314c
	pc = 0x8226314C; continue 'dispatch;
            }
            0x82263140 => {
    //   block [0x82263140..0x8226314C)
	// 82263140: 38600057  li r3, 0x57
	ctx.r[3].s64 = 87;
	// 82263144: 4BE24505  bl 0x82087648
	ctx.lr = 0x82263148;
	sub_82087648(ctx, base);
	// 82263148: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8226314C; continue 'dispatch;
            }
            0x8226314C => {
    //   block [0x8226314C..0x82263154)
	// 8226314C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82263150: 4BE2B994  b 0x8208eae4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82263158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82263158 size=504
    let mut pc: u32 = 0x82263158;
    'dispatch: loop {
        match pc {
            0x82263158 => {
    //   block [0x82263158..0x8226317C)
	// 82263158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226315C: 4BE2B91D  bl 0x8208ea78
	ctx.lr = 0x82263160;
	sub_8208EA60(ctx, base);
	// 82263160: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82263164: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82263168: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226316C: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82263170: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 82263174: 409A0028  bne cr6, 0x8226319c
	if !ctx.cr[6].eq {
	pc = 0x8226319C; continue 'dispatch;
	}
	// 82263178: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	pc = 0x8226317C; continue 'dispatch;
            }
            0x8226317C => {
    //   block [0x8226317C..0x8226319C)
	// 8226317C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82263180: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82263184: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82263188: 409AFFF4  bne cr6, 0x8226317c
	if !ctx.cr[6].eq {
	pc = 0x8226317C; continue 'dispatch;
	}
	// 8226318C: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82263190: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82263194: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82263198: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	pc = 0x8226319C; continue 'dispatch;
            }
            0x8226319C => {
    //   block [0x8226319C..0x822631D8)
	// 8226319C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 822631A0: 4099018C  ble cr6, 0x8226332c
	if !ctx.cr[6].gt {
	pc = 0x8226332C; continue 'dispatch;
	}
	// 822631A4: 3D208228  lis r9, -0x7dd8
	ctx.r[9].s64 = -2111307776;
	// 822631A8: 3D608228  lis r11, -0x7dd8
	ctx.r[11].s64 = -2111307776;
	// 822631AC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 822631B0: 3F408228  lis r26, -0x7dd8
	ctx.r[26].s64 = -2111307776;
	// 822631B4: 3F208228  lis r25, -0x7dd8
	ctx.r[25].s64 = -2111307776;
	// 822631B8: 3F608228  lis r27, -0x7dd8
	ctx.r[27].s64 = -2111307776;
	// 822631BC: 3F808228  lis r28, -0x7dd8
	ctx.r[28].s64 = -2111307776;
	// 822631C0: 3F008228  lis r24, -0x7dd8
	ctx.r[24].s64 = -2111307776;
	// 822631C4: 3EA08228  lis r21, -0x7dd8
	ctx.r[21].s64 = -2111307776;
	// 822631C8: 3E808228  lis r20, -0x7dd8
	ctx.r[20].s64 = -2111307776;
	// 822631CC: 3EC08228  lis r22, -0x7dd8
	ctx.r[22].s64 = -2111307776;
	// 822631D0: 3AE96A00  addi r23, r9, 0x6a00
	ctx.r[23].s64 = ctx.r[9].s64 + 27136;
	// 822631D4: 3BCB6A18  addi r30, r11, 0x6a18
	ctx.r[30].s64 = ctx.r[11].s64 + 27160;
	pc = 0x822631D8; continue 'dispatch;
            }
            0x822631D8 => {
    //   block [0x822631D8..0x8226322C)
	// 822631D8: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822631DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822631E0: 7D28F0AE  lbzx r9, r8, r30
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 822631E4: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 822631E8: 5529043E  clrlwi r9, r9, 0x10
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 822631EC: 7CFD4850  subf r7, r29, r9
	ctx.r[7].s64 = ctx.r[9].s64 - ctx.r[29].s64;
	// 822631F0: 7CE75214  add r7, r7, r10
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 822631F4: 7F072000  cmpw cr6, r7, r4
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[4].s32, &mut ctx.xer);
	// 822631F8: 41990134  bgt cr6, 0x8226332c
	if ctx.cr[6].gt {
	pc = 0x8226332C; continue 'dispatch;
	}
	// 822631FC: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82263200: 4198007C  blt cr6, 0x8226327c
	if ctx.cr[6].lt {
	pc = 0x8226327C; continue 'dispatch;
	}
	// 82263204: 419A0064  beq cr6, 0x82263268
	if ctx.cr[6].eq {
	pc = 0x82263268; continue 'dispatch;
	}
	// 82263208: 2B090003  cmplwi cr6, r9, 3
	ctx.cr[6].compare_u32(ctx.r[9].u32, 3 as u32, &mut ctx.xer);
	// 8226320C: 41980048  blt cr6, 0x82263254
	if ctx.cr[6].lt {
	pc = 0x82263254; continue 'dispatch;
	}
	// 82263210: 419A0030  beq cr6, 0x82263240
	if ctx.cr[6].eq {
	pc = 0x82263240; continue 'dispatch;
	}
	// 82263214: 2B090005  cmplwi cr6, r9, 5
	ctx.cr[6].compare_u32(ctx.r[9].u32, 5 as u32, &mut ctx.xer);
	// 82263218: 41980014  blt cr6, 0x8226322c
	if ctx.cr[6].lt {
	pc = 0x8226322C; continue 'dispatch;
	}
	// 8226321C: 409A0070  bne cr6, 0x8226328c
	if !ctx.cr[6].eq {
	pc = 0x8226328C; continue 'dispatch;
	}
	// 82263220: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82263224: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82263228: 550B3032  slwi r11, r8, 6
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	pc = 0x8226322C; continue 'dispatch;
            }
            0x8226322C => {
    //   block [0x8226322C..0x82263240)
	// 8226322C: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82263230: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82263234: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82263238: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8226323C: 556B3032  slwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	pc = 0x82263240; continue 'dispatch;
            }
            0x82263240 => {
    //   block [0x82263240..0x82263254)
	// 82263240: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82263244: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82263248: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8226324C: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82263250: 556B3032  slwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	pc = 0x82263254; continue 'dispatch;
            }
            0x82263254 => {
    //   block [0x82263254..0x82263268)
	// 82263254: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82263258: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8226325C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82263260: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82263264: 556B3032  slwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	pc = 0x82263268; continue 'dispatch;
            }
            0x82263268 => {
    //   block [0x82263268..0x8226327C)
	// 82263268: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226326C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82263270: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82263274: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82263278: 556B3032  slwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	pc = 0x8226327C; continue 'dispatch;
            }
            0x8226327C => {
    //   block [0x8226327C..0x8226328C)
	// 8226327C: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82263280: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82263284: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82263288: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	pc = 0x8226328C; continue 'dispatch;
            }
            0x8226328C => {
    //   block [0x8226328C..0x822632B4)
	// 8226328C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82263290: 811669E4  lwz r8, 0x69e4(r22)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(27108 as u32) ) } as u64;
	// 82263294: 7D29B82E  lwzx r9, r9, r23
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82263298: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8226329C: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 822632A0: 41990014  bgt cr6, 0x822632b4
	if ctx.cr[6].gt {
	pc = 0x822632B4; continue 'dispatch;
	}
	// 822632A4: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 822632A8: 7F033000  cmpw cr6, r3, r6
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[6].s32, &mut ctx.xer);
	// 822632AC: 41990078  bgt cr6, 0x82263324
	if ctx.cr[6].gt {
	pc = 0x82263324; continue 'dispatch;
	}
	// 822632B0: 4800006C  b 0x8226331c
	pc = 0x8226331C; continue 'dispatch;
            }
            0x822632B4 => {
    //   block [0x822632B4..0x822632D4)
	// 822632B4: 813469E8  lwz r9, 0x69e8(r20)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(27112 as u32) ) } as u64;
	// 822632B8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822632BC: 40990018  ble cr6, 0x822632d4
	if !ctx.cr[6].gt {
	pc = 0x822632D4; continue 'dispatch;
	}
	// 822632C0: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 822632C4: 7F033000  cmpw cr6, r3, r6
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[6].s32, &mut ctx.xer);
	// 822632C8: 4199005C  bgt cr6, 0x82263324
	if ctx.cr[6].gt {
	pc = 0x82263324; continue 'dispatch;
	}
	// 822632CC: 817569E0  lwz r11, 0x69e0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(27104 as u32) ) } as u64;
	// 822632D0: 4800004C  b 0x8226331c
	pc = 0x8226331C; continue 'dispatch;
            }
            0x822632D4 => {
    //   block [0x822632D4..0x82263300)
	// 822632D4: 813869F0  lwz r9, 0x69f0(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(27120 as u32) ) } as u64;
	// 822632D8: 38E30001  addi r7, r3, 1
	ctx.r[7].s64 = ctx.r[3].s64 + 1;
	// 822632DC: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 822632E0: 7F073000  cmpw cr6, r7, r6
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[6].s32, &mut ctx.xer);
	// 822632E4: 4199001C  bgt cr6, 0x82263300
	if ctx.cr[6].gt {
	pc = 0x82263300; continue 'dispatch;
	}
	// 822632E8: 813C69EC  lwz r9, 0x69ec(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(27116 as u32) ) } as u64;
	// 822632EC: 811B69F8  lwz r8, 0x69f8(r27)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(27128 as u32) ) } as u64;
	// 822632F0: 7D694C30  srw r9, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 822632F4: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 822632F8: B1250000  sth r9, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 822632FC: 38A50002  addi r5, r5, 2
	ctx.r[5].s64 = ctx.r[5].s64 + 2;
	pc = 0x82263300; continue 'dispatch;
            }
            0x82263300 => {
    //   block [0x82263300..0x8226331C)
	// 82263300: 38670001  addi r3, r7, 1
	ctx.r[3].s64 = ctx.r[7].s64 + 1;
	// 82263304: 7F033000  cmpw cr6, r3, r6
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82263308: 4199001C  bgt cr6, 0x82263324
	if ctx.cr[6].gt {
	pc = 0x82263324; continue 'dispatch;
	}
	// 8226330C: 813969F4  lwz r9, 0x69f4(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(27124 as u32) ) } as u64;
	// 82263310: 811A69FC  lwz r8, 0x69fc(r26)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(27132 as u32) ) } as u64;
	// 82263314: 7D2B5838  and r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 & ctx.r[11].u64;
	// 82263318: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	pc = 0x8226331C; continue 'dispatch;
            }
            0x8226331C => {
    //   block [0x8226331C..0x82263324)
	// 8226331C: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82263320: 38A50002  addi r5, r5, 2
	ctx.r[5].s64 = ctx.r[5].s64 + 2;
	pc = 0x82263324; continue 'dispatch;
            }
            0x82263324 => {
    //   block [0x82263324..0x8226332C)
	// 82263324: 7F1F2000  cmpw cr6, r31, r4
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82263328: 4198FEB0  blt cr6, 0x822631d8
	if ctx.cr[6].lt {
	pc = 0x822631D8; continue 'dispatch;
	}
	pc = 0x8226332C; continue 'dispatch;
            }
            0x8226332C => {
    //   block [0x8226332C..0x82263348)
	// 8226332C: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82263330: 419A0018  beq cr6, 0x82263348
	if ctx.cr[6].eq {
	pc = 0x82263348; continue 'dispatch;
	}
	// 82263334: 7F061800  cmpw cr6, r6, r3
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82263338: 40980010  bge cr6, 0x82263348
	if !ctx.cr[6].lt {
	pc = 0x82263348; continue 'dispatch;
	}
	// 8226333C: 3860007A  li r3, 0x7a
	ctx.r[3].s64 = 122;
	// 82263340: 4BE24309  bl 0x82087648
	ctx.lr = 0x82263344;
	sub_82087648(ctx, base);
	// 82263344: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82263348; continue 'dispatch;
            }
            0x82263348 => {
    //   block [0x82263348..0x82263350)
	// 82263348: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8226334C: 4BE2B77C  b 0x8208eac8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82263360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82263360 size=552
    let mut pc: u32 = 0x82263360;
    'dispatch: loop {
        match pc {
            0x82263360 => {
    //   block [0x82263360..0x82263390)
	// 82263360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82263364: 4BE2B731  bl 0x8208ea94
	ctx.lr = 0x82263368;
	sub_8208EA60(ctx, base);
	// 82263368: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8226336C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82263370: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82263374: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82263378: 93DF00AC  stw r30, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[30].u32 ) };
	// 8226337C: 7F6B0034  cntlzw r11, r27
	ctx.r[11].u64 = if ctx.r[27].u32 == 0 { 32 } else { ctx.r[27].u32.leading_zeros() as u64 };
	// 82263380: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82263384: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82263388: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226338C: 409A0030  bne cr6, 0x822633bc
	if !ctx.cr[6].eq {
	pc = 0x822633BC; continue 'dispatch;
	}
	pc = 0x82263390; continue 'dispatch;
            }
            0x82263390 => {
    //   block [0x82263390..0x822633BC)
	// 82263390: 4BE2F6E9  bl 0x82092a78
	ctx.lr = 0x82263394;
	sub_82092A78(ctx, base);
	// 82263394: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 82263398: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8226339C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822633A0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822633A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822633A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822633AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822633B0: 4BE2F589  bl 0x82092938
	ctx.lr = 0x822633B4;
	sub_82092938(ctx, base);
	// 822633B4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 822633B8: 48000170  b 0x82263528
	pc = 0x82263528; continue 'dispatch;
            }
            0x822633BC => {
    //   block [0x822633BC..0x82263434)
	// 822633BC: 7FCB0034  cntlzw r11, r30
	ctx.r[11].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 822633C0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822633C4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 822633C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822633CC: 419AFFC4  beq cr6, 0x82263390
	if ctx.cr[6].eq {
	pc = 0x82263390; continue 'dispatch;
	}
	// 822633D0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 822633D4: 556B0673  rlwinm. r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822633D8: 408200CC  bne 0x822634a4
	if !ctx.cr[0].eq {
	pc = 0x822634A4; continue 'dispatch;
	}
	// 822633DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822633E0: 4BE35681  bl 0x82098a60
	ctx.lr = 0x822633E4;
	sub_82098A60(ctx, base);
	// 822633E4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 822633E8: 419A004C  beq cr6, 0x82263434
	if ctx.cr[6].eq {
	pc = 0x82263434; continue 'dispatch;
	}
	// 822633EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822633F0: 4BE35671  bl 0x82098a60
	ctx.lr = 0x822633F4;
	sub_82098A60(ctx, base);
	// 822633F4: 2F03FFFE  cmpwi cr6, r3, -2
	ctx.cr[6].compare_i32(ctx.r[3].s32, -2, &mut ctx.xer);
	// 822633F8: 419A003C  beq cr6, 0x82263434
	if ctx.cr[6].eq {
	pc = 0x82263434; continue 'dispatch;
	}
	// 822633FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82263400: 4BE35661  bl 0x82098a60
	ctx.lr = 0x82263404;
	sub_82098A60(ctx, base);
	// 82263404: 7C6A2E70  srawi r10, r3, 5
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[3].s32 >> 5) as i64;
	// 82263408: 3D608229  lis r11, -0x7dd7
	ctx.r[11].s64 = -2111242240;
	// 8226340C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82263410: 555C103A  slwi r28, r10, 2
	ctx.r[28].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82263414: 3BABDD00  addi r29, r11, -0x2300
	ctx.r[29].s64 = ctx.r[11].s64 + -8960;
	// 82263418: 4BE35649  bl 0x82098a60
	ctx.lr = 0x8226341C;
	sub_82098A60(ctx, base);
	// 8226341C: 7D7CE82E  lwzx r11, r28, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82263420: 546A3572  rlwinm r10, r3, 6, 0x15, 0x19
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x03FFFFFFu64;
	// 82263424: 3D208227  lis r9, -0x7dd9
	ctx.r[9].s64 = -2111373312;
	// 82263428: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8226342C: 3B892380  addi r28, r9, 0x2380
	ctx.r[28].s64 = ctx.r[9].s64 + 9088;
	// 82263430: 48000018  b 0x82263448
	pc = 0x82263448; continue 'dispatch;
            }
            0x82263434 => {
    //   block [0x82263434..0x82263448)
	// 82263434: 3D408227  lis r10, -0x7dd9
	ctx.r[10].s64 = -2111373312;
	// 82263438: 3D608229  lis r11, -0x7dd7
	ctx.r[11].s64 = -2111242240;
	// 8226343C: 3B8A2380  addi r28, r10, 0x2380
	ctx.r[28].s64 = ctx.r[10].s64 + 9088;
	// 82263440: 3BABDD00  addi r29, r11, -0x2300
	ctx.r[29].s64 = ctx.r[11].s64 + -8960;
	// 82263444: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	pc = 0x82263448; continue 'dispatch;
            }
            0x82263448 => {
    //   block [0x82263448..0x82263498)
	// 82263448: 896B0028  lbz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8226344C: 556B003D  rlwinm. r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82263450: 4082FF40  bne 0x82263390
	if !ctx.cr[0].eq {
	pc = 0x82263390; continue 'dispatch;
	}
	// 82263454: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82263458: 4BE35609  bl 0x82098a60
	ctx.lr = 0x8226345C;
	sub_82098A60(ctx, base);
	// 8226345C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82263460: 419A0038  beq cr6, 0x82263498
	if ctx.cr[6].eq {
	pc = 0x82263498; continue 'dispatch;
	}
	// 82263464: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82263468: 4BE355F9  bl 0x82098a60
	ctx.lr = 0x8226346C;
	sub_82098A60(ctx, base);
	// 8226346C: 2F03FFFE  cmpwi cr6, r3, -2
	ctx.cr[6].compare_i32(ctx.r[3].s32, -2, &mut ctx.xer);
	// 82263470: 419A0028  beq cr6, 0x82263498
	if ctx.cr[6].eq {
	pc = 0x82263498; continue 'dispatch;
	}
	// 82263474: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82263478: 4BE355E9  bl 0x82098a60
	ctx.lr = 0x8226347C;
	sub_82098A60(ctx, base);
	// 8226347C: 7C6B2E70  srawi r11, r3, 5
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 5) as i64;
	// 82263480: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82263484: 557C103A  slwi r28, r11, 2
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82263488: 4BE355D9  bl 0x82098a60
	ctx.lr = 0x8226348C;
	sub_82098A60(ctx, base);
	// 8226348C: 7D5CE82E  lwzx r10, r28, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82263490: 546B3572  rlwinm r11, r3, 6, 0x15, 0x19
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x03FFFFFFu64;
	// 82263494: 7F8B5214  add r28, r11, r10
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	pc = 0x82263498; continue 'dispatch;
            }
            0x82263498 => {
    //   block [0x82263498..0x822634A4)
	// 82263498: 897C0028  lbz r11, 0x28(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) } as u64;
	// 8226349C: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822634A0: 4082FEF0  bne 0x82263390
	if !ctx.cr[0].eq {
	pc = 0x82263390; continue 'dispatch;
	}
	pc = 0x822634A4; continue 'dispatch;
            }
            0x822634A4 => {
    //   block [0x822634A4..0x822634A8)
	// 822634A4: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	pc = 0x822634A8; continue 'dispatch;
            }
            0x822634A8 => {
    //   block [0x822634A8..0x82263528)
	// 822634A8: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822634AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 822634B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822634B4: 409AFFF4  bne cr6, 0x822634a8
	if !ctx.cr[6].eq {
	pc = 0x822634A8; continue 'dispatch;
	}
	// 822634B8: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 822634BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822634C0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 822634C4: 557D003E  slwi r29, r11, 0
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 822634C8: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 822634CC: 4BE3414D  bl 0x82097618
	ctx.lr = 0x822634D0;
	sub_82097618(ctx, base);
	// 822634D0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 822634D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822634D8: 4BE34269  bl 0x82097740
	ctx.lr = 0x822634DC;
	sub_82097740(ctx, base);
	// 822634DC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822634E0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 822634E4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822634E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822634EC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 822634F0: 48002071  bl 0x82265560
	ctx.lr = 0x822634F4;
	sub_82265560(ctx, base);
	// 822634F4: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 822634F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822634FC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82263500: 4BE34319  bl 0x82097818
	ctx.lr = 0x82263504;
	sub_82097818(ctx, base);
	// 82263504: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82263508: 399F0090  addi r12, r31, 0x90
	ctx.r[12].s64 = ctx.r[31].s64 + 144;
	// 8226350C: 48000045  bl 0x82263550
	ctx.lr = 0x82263510;
	sub_82263360(ctx, base);
	// 82263510: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82263514: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82263518: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8226351C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82263520: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82263524: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	pc = 0x82263528; continue 'dispatch;
            }
            0x82263528 => {
    //   block [0x82263528..0x82263550)
	// 82263528: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8226352C: 4BE2B5B8  b 0x8208eae4
	sub_8208EAB0(ctx, base);
	return;
	// 82263530: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82263534: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82263538: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 8226353C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82263540: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 82263544: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82263548: 83DF00AC  lwz r30, 0xac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 8226354C: 4800001C  b 0x82263568
	pc = 0x82263568; continue 'dispatch;
            }
            0x82263550 => {
    //   block [0x82263550..0x82263568)
	// 82263550: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82263554: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82263558: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 8226355C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82263560: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 82263564: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	pc = 0x82263568; continue 'dispatch;
            }
            0x82263568 => {
    //   block [0x82263568..0x82263588)
	// 82263568: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226356C: 4BE3416D  bl 0x820976d8
	ctx.lr = 0x82263570;
	sub_820976D8(ctx, base);
	// 82263570: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 82263574: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82263578: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226357C: 8181FFE8  lwz r12, -0x18(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) } as u64;
	// 82263580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82263584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82263588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82263588 size=48
    let mut pc: u32 = 0x82263588;
    'dispatch: loop {
        match pc {
            0x82263588 => {
    //   block [0x82263588..0x822635B8)
	// 82263588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226358C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82263590: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82263594: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82263598: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226359C: 4BE2D4AD  bl 0x82090a48
	ctx.lr = 0x822635A0;
	sub_82090A48(ctx, base);
	// 822635A0: 93E30090  stw r31, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 822635A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822635A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822635AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822635B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822635B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822635B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822635B8 size=36
    let mut pc: u32 = 0x822635B8;
    'dispatch: loop {
        match pc {
            0x822635B8 => {
    //   block [0x822635B8..0x822635DC)
	// 822635B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822635BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822635C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822635C4: 4BE2D485  bl 0x82090a48
	ctx.lr = 0x822635C8;
	sub_82090A48(ctx, base);
	// 822635C8: 80630090  lwz r3, 0x90(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 822635CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822635D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822635D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822635D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822635E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822635E0 size=32
    let mut pc: u32 = 0x822635E0;
    'dispatch: loop {
        match pc {
            0x822635E0 => {
    //   block [0x822635E0..0x82263600)
	// 822635E0: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 822635E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822635E8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 822635EC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822635F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822635F4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 822635F8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822635FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82263600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82263600 size=8
    let mut pc: u32 = 0x82263600;
    'dispatch: loop {
        match pc {
            0x82263600 => {
    //   block [0x82263600..0x82263608)
	// 82263600: 38A00A40  li r5, 0xa40
	ctx.r[5].s64 = 2624;
	// 82263604: 4BE2C15C  b 0x8208f760
	sub_8208F760(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82263608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82263608 size=148
    let mut pc: u32 = 0x82263608;
    'dispatch: loop {
        match pc {
            0x82263608 => {
    //   block [0x82263608..0x82263648)
	// 82263608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226360C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82263610: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82263614: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82263618: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8226361C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82263620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82263624: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82263628: 480021A1  bl 0x822657c8
	ctx.lr = 0x8226362C;
	sub_822657C8(ctx, base);
	// 8226362C: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82263630: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82263634: 419A003C  beq cr6, 0x82263670
	if ctx.cr[6].eq {
	pc = 0x82263670; continue 'dispatch;
	}
	// 82263638: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8226363C: 1D6A0014  mulli r11, r10, 0x14
	ctx.r[11].s32 = ((ctx.r[10].s32 as i64 * 20 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82263640: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82263644: 396BFFF4  addi r11, r11, -0xc
	ctx.r[11].s64 = ctx.r[11].s64 + -12;
	pc = 0x82263648; continue 'dispatch;
            }
            0x82263648 => {
    //   block [0x82263648..0x82263664)
	// 82263648: 810BFFFC  lwz r8, -4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8226364C: 392BFFF8  addi r9, r11, -8
	ctx.r[9].s64 = ctx.r[11].s64 + -8;
	// 82263650: 7F034000  cmpw cr6, r3, r8
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82263654: 40990010  ble cr6, 0x82263664
	if !ctx.cr[6].gt {
	pc = 0x82263664; continue 'dispatch;
	}
	// 82263658: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226365C: 7F034000  cmpw cr6, r3, r8
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82263660: 40990034  ble cr6, 0x82263694
	if !ctx.cr[6].gt {
	pc = 0x82263694; continue 'dispatch;
	}
	pc = 0x82263664; continue 'dispatch;
            }
            0x82263664 => {
    //   block [0x82263664..0x82263670)
	// 82263664: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82263668: 396BFFEC  addi r11, r11, -0x14
	ctx.r[11].s64 = ctx.r[11].s64 + -20;
	// 8226366C: 4082FFDC  bne 0x82263648
	if !ctx.cr[0].eq {
	pc = 0x82263648; continue 'dispatch;
	}
	pc = 0x82263670; continue 'dispatch;
            }
            0x82263670 => {
    //   block [0x82263670..0x82263674)
	// 82263670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82263674; continue 'dispatch;
            }
            0x82263674 => {
    //   block [0x82263674..0x82263694)
	// 82263674: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82263678: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8226367C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82263680: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82263684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82263688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226368C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82263690: 4E800020  blr
	return;
            }
            0x82263694 => {
    //   block [0x82263694..0x8226369C)
	// 82263694: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82263698: 4BFFFFDC  b 0x82263674
	pc = 0x82263674; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822636A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822636A0 size=196
    let mut pc: u32 = 0x822636A0;
    'dispatch: loop {
        match pc {
            0x822636A0 => {
    //   block [0x822636A0..0x822636D4)
	// 822636A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822636A4: 4BE2B3F9  bl 0x8208ea9c
	ctx.lr = 0x822636A8;
	sub_8208EA60(ctx, base);
	// 822636A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822636AC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 822636B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822636B4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822636B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822636BC: 419A0018  beq cr6, 0x822636d4
	if ctx.cr[6].eq {
	pc = 0x822636D4; continue 'dispatch;
	}
	// 822636C0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822636C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822636C8: 419A000C  beq cr6, 0x822636d4
	if ctx.cr[6].eq {
	pc = 0x822636D4; continue 'dispatch;
	}
	// 822636CC: 83AB0004  lwz r29, 4(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822636D0: 48000008  b 0x822636d8
	pc = 0x822636D8; continue 'dispatch;
            }
            0x822636D4 => {
    //   block [0x822636D4..0x822636D8)
	// 822636D4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	pc = 0x822636D8; continue 'dispatch;
            }
            0x822636D8 => {
    //   block [0x822636D8..0x82263700)
	// 822636D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822636DC: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822636E0: 480020E9  bl 0x822657c8
	ctx.lr = 0x822636E4;
	sub_822657C8(ctx, base);
	// 822636E4: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822636E8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822636EC: 419A003C  beq cr6, 0x82263728
	if ctx.cr[6].eq {
	pc = 0x82263728; continue 'dispatch;
	}
	// 822636F0: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822636F4: 1D6A0014  mulli r11, r10, 0x14
	ctx.r[11].s32 = ((ctx.r[10].s32 as i64 * 20 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 822636F8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 822636FC: 396BFFF4  addi r11, r11, -0xc
	ctx.r[11].s64 = ctx.r[11].s64 + -12;
	pc = 0x82263700; continue 'dispatch;
            }
            0x82263700 => {
    //   block [0x82263700..0x8226371C)
	// 82263700: 810BFFFC  lwz r8, -4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82263704: 392BFFF8  addi r9, r11, -8
	ctx.r[9].s64 = ctx.r[11].s64 + -8;
	// 82263708: 7F034000  cmpw cr6, r3, r8
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8226370C: 40990010  ble cr6, 0x8226371c
	if !ctx.cr[6].gt {
	pc = 0x8226371C; continue 'dispatch;
	}
	// 82263710: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82263714: 7F034000  cmpw cr6, r3, r8
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82263718: 40990024  ble cr6, 0x8226373c
	if !ctx.cr[6].gt {
	pc = 0x8226373C; continue 'dispatch;
	}
	pc = 0x8226371C; continue 'dispatch;
            }
            0x8226371C => {
    //   block [0x8226371C..0x82263728)
	// 8226371C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82263720: 396BFFEC  addi r11, r11, -0x14
	ctx.r[11].s64 = ctx.r[11].s64 + -20;
	// 82263724: 4082FFDC  bne 0x82263700
	if !ctx.cr[0].eq {
	pc = 0x82263700; continue 'dispatch;
	}
	pc = 0x82263728; continue 'dispatch;
            }
            0x82263728 => {
    //   block [0x82263728..0x8226372C)
	// 82263728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x8226372C; continue 'dispatch;
            }
            0x8226372C => {
    //   block [0x8226372C..0x8226373C)
	// 8226372C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82263730: 409A0014  bne cr6, 0x82263744
	if !ctx.cr[6].eq {
	pc = 0x82263744; continue 'dispatch;
	}
	// 82263734: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82263738: 48000014  b 0x8226374c
	pc = 0x8226374C; continue 'dispatch;
            }
            0x8226373C => {
    //   block [0x8226373C..0x82263744)
	// 8226373C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82263740: 4BFFFFEC  b 0x8226372c
	pc = 0x8226372C; continue 'dispatch;
            }
            0x82263744 => {
    //   block [0x82263744..0x8226374C)
	// 82263744: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82263748: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	pc = 0x8226374C; continue 'dispatch;
            }
            0x8226374C => {
    //   block [0x8226374C..0x82263764)
	// 8226374C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82263750: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82263754: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82263758: 480022C1  bl 0x82265a18
	ctx.lr = 0x8226375C;
	sub_82265A18(ctx, base);
	// 8226375C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82263760: 4BE2B38C  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822637A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822637A0 size=116
    let mut pc: u32 = 0x822637A0;
    'dispatch: loop {
        match pc {
            0x822637A0 => {
    //   block [0x822637A0..0x82263814)
	// 822637A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822637A4: 4BE2B2E9  bl 0x8208ea8c
	ctx.lr = 0x822637A8;
	sub_8208EA60(ctx, base);
	// 822637A8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822637AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822637B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822637B4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822637B8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 822637BC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 822637C0: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 822637C4: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 822637C8: 4BE2D281  bl 0x82090a48
	ctx.lr = 0x822637CC;
	sub_82090A48(ctx, base);
	// 822637CC: 93C300B0  stw r30, 0xb0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), ctx.r[30].u32 ) };
	// 822637D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822637D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 822637D8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822637DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 822637E0: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 822637E4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 822637E8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822637EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822637F0: 48002F09  bl 0x822666f8
	ctx.lr = 0x822637F4;
	sub_822666F8(ctx, base);
	// 822637F4: 4BE2D255  bl 0x82090a48
	ctx.lr = 0x822637F8;
	sub_82090A48(ctx, base);
	// 822637F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822637FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82263800: 916300B0  stw r11, 0xb0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 82263804: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82263808: 91590000  stw r10, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8226380C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82263810: 4BE2B2CC  b 0x8208eadc
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82263820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82263820 size=192
    let mut pc: u32 = 0x82263820;
    'dispatch: loop {
        match pc {
            0x82263820 => {
    //   block [0x82263820..0x822638E0)
	// 82263820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82263824: 4BE2B279  bl 0x8208ea9c
	ctx.lr = 0x82263828;
	sub_8208EA60(ctx, base);
	// 82263828: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8226382C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82263830: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82263834: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82263838: 90BF00A4  stw r5, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 8226383C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82263840: 909F009C  stw r4, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[4].u32 ) };
	// 82263844: 90DF00AC  stw r6, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[6].u32 ) };
	// 82263848: 90FF00B4  stw r7, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[7].u32 ) };
	// 8226384C: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82263850: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82263854: 90BF005C  stw r5, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 82263858: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8226385C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82263860: 4BE2D1E9  bl 0x82090a48
	ctx.lr = 0x82263864;
	sub_82090A48(ctx, base);
	// 82263864: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82263868: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226386C: 389F0058  addi r4, r31, 0x58
	ctx.r[4].s64 = ctx.r[31].s64 + 88;
	// 82263870: 816B0074  lwz r11, 0x74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 82263874: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82263878: 4E800421  bctrl
	ctx.lr = 0x8226387C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226387C: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82263880: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82263884: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82263888: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8226388C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82263890: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82263894: 4BE2B258  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
	// 82263898: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8226389C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 822638A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822638A4: 9181FFF0  stw r12, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[12].u32 ) };
	// 822638A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822638AC: 393F0050  addi r9, r31, 0x50
	ctx.r[9].s64 = ctx.r[31].s64 + 80;
	// 822638B0: 811F00B4  lwz r8, 0xb4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 822638B4: 80FF00AC  lwz r7, 0xac(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 822638B8: 80DF00A4  lwz r6, 0xa4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 822638BC: 80BF009C  lwz r5, 0x9c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 822638C0: 809F0094  lwz r4, 0x94(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 822638C4: 4BFFFEDD  bl 0x822637a0
	ctx.lr = 0x822638C8;
	sub_822637A0(ctx, base);
	// 822638C8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 822638CC: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 822638D0: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 822638D4: 8181FFF0  lwz r12, -0x10(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 822638D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822638DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822638E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822638E0 size=344
    let mut pc: u32 = 0x822638E0;
    'dispatch: loop {
        match pc {
            0x822638E0 => {
    //   block [0x822638E0..0x82263924)
	// 822638E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822638E4: 4BE2B1A5  bl 0x8208ea88
	ctx.lr = 0x822638E8;
	sub_8208EA60(ctx, base);
	// 822638E8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822638EC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822638F0: 80880000  lwz r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 822638F4: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 822638F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822638FC: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82263900: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82263904: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82263908: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 8226390C: 7F1CC378  mr r28, r24
	ctx.r[28].u64 = ctx.r[24].u64;
	// 82263910: 48001EB9  bl 0x822657c8
	ctx.lr = 0x82263914;
	sub_822657C8(ctx, base);
	// 82263914: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82263918: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8226391C: 409A0008  bne cr6, 0x82263924
	if !ctx.cr[6].eq {
	pc = 0x82263924; continue 'dispatch;
	}
	// 82263920: 4BE33671  bl 0x82096f90
	ctx.lr = 0x82263924;
	sub_82096F90(ctx, base);
	pc = 0x82263924; continue 'dispatch;
            }
            0x82263924 => {
    //   block [0x82263924..0x8226394C)
	// 82263924: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82263928: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 8226392C: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82263930: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82263934: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82263938: 419A0038  beq cr6, 0x82263970
	if ctx.cr[6].eq {
	pc = 0x82263970; continue 'dispatch;
	}
	// 8226393C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82263940: 1D7E0014  mulli r11, r30, 0x14
	ctx.r[11].s32 = ((ctx.r[30].s32 as i64 * 20 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82263944: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82263948: 396BFFF4  addi r11, r11, -0xc
	ctx.r[11].s64 = ctx.r[11].s64 + -12;
	pc = 0x8226394C; continue 'dispatch;
            }
            0x8226394C => {
    //   block [0x8226394C..0x82263964)
	// 8226394C: 814BFFFC  lwz r10, -4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82263950: 7F1D5000  cmpw cr6, r29, r10
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82263954: 40990010  ble cr6, 0x82263964
	if !ctx.cr[6].gt {
	pc = 0x82263964; continue 'dispatch;
	}
	// 82263958: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226395C: 7F1D5000  cmpw cr6, r29, r10
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82263960: 40990010  ble cr6, 0x82263970
	if !ctx.cr[6].gt {
	pc = 0x82263970; continue 'dispatch;
	}
	pc = 0x82263964; continue 'dispatch;
            }
            0x82263964 => {
    //   block [0x82263964..0x82263970)
	// 82263964: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82263968: 396BFFEC  addi r11, r11, -0x14
	ctx.r[11].s64 = ctx.r[11].s64 + -20;
	// 8226396C: 4082FFE0  bne 0x8226394c
	if !ctx.cr[0].eq {
	pc = 0x8226394C; continue 'dispatch;
	}
	pc = 0x82263970; continue 'dispatch;
            }
            0x82263970 => {
    //   block [0x82263970..0x82263988)
	// 82263970: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82263974: 419A0014  beq cr6, 0x82263988
	if ctx.cr[6].eq {
	pc = 0x82263988; continue 'dispatch;
	}
	// 82263978: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8226397C: 1D690014  mulli r11, r9, 0x14
	ctx.r[11].s32 = ((ctx.r[9].s32 as i64 * 20 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82263980: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82263984: 3B8BFFEC  addi r28, r11, -0x14
	ctx.r[28].s64 = ctx.r[11].s64 + -20;
	pc = 0x82263988; continue 'dispatch;
            }
            0x82263988 => {
    //   block [0x82263988..0x82263998)
	// 82263988: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 8226398C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82263990: 419A0084  beq cr6, 0x82263a14
	if ctx.cr[6].eq {
	pc = 0x82263A14; continue 'dispatch;
	}
	// 82263994: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	pc = 0x82263998; continue 'dispatch;
            }
            0x82263998 => {
    //   block [0x82263998..0x822639C8)
	// 82263998: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8226399C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 822639A0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 822639A4: 419A0024  beq cr6, 0x822639c8
	if ctx.cr[6].eq {
	pc = 0x822639C8; continue 'dispatch;
	}
	// 822639A8: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822639AC: 80FC0004  lwz r7, 4(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822639B0: 7F083800  cmpw cr6, r8, r7
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[7].s32, &mut ctx.xer);
	// 822639B4: 40990044  ble cr6, 0x822639f8
	if !ctx.cr[6].gt {
	pc = 0x822639F8; continue 'dispatch;
	}
	// 822639B8: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822639BC: 80FC0008  lwz r7, 8(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 822639C0: 7F083800  cmpw cr6, r8, r7
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[7].s32, &mut ctx.xer);
	// 822639C4: 41990034  bgt cr6, 0x822639f8
	if ctx.cr[6].gt {
	pc = 0x822639F8; continue 'dispatch;
	}
	pc = 0x822639C8; continue 'dispatch;
            }
            0x822639C8 => {
    //   block [0x822639C8..0x822639F0)
	// 822639C8: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822639CC: 7F194000  cmpw cr6, r25, r8
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[8].s32, &mut ctx.xer);
	// 822639D0: 41980028  blt cr6, 0x822639f8
	if ctx.cr[6].lt {
	pc = 0x822639F8; continue 'dispatch;
	}
	// 822639D4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822639D8: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822639DC: 4199001C  bgt cr6, 0x822639f8
	if ctx.cr[6].gt {
	pc = 0x822639F8; continue 'dispatch;
	}
	// 822639E0: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822639E4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 822639E8: 409A0008  bne cr6, 0x822639f0
	if !ctx.cr[6].eq {
	pc = 0x822639F0; continue 'dispatch;
	}
	// 822639EC: 915B0000  stw r10, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x822639F0; continue 'dispatch;
            }
            0x822639F0 => {
    //   block [0x822639F0..0x822639F8)
	// 822639F0: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 822639F4: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x822639F8; continue 'dispatch;
            }
            0x822639F8 => {
    //   block [0x822639F8..0x82263A14)
	// 822639F8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822639FC: 39290014  addi r9, r9, 0x14
	ctx.r[9].s64 = ctx.r[9].s64 + 20;
	// 82263A00: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82263A04: 4198FF94  blt cr6, 0x82263998
	if ctx.cr[6].lt {
	pc = 0x82263998; continue 'dispatch;
	}
	// 82263A08: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82263A0C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82263A10: 409A0014  bne cr6, 0x82263a24
	if !ctx.cr[6].eq {
	pc = 0x82263A24; continue 'dispatch;
	}
	pc = 0x82263A14; continue 'dispatch;
            }
            0x82263A14 => {
    //   block [0x82263A14..0x82263A24)
	// 82263A14: 931B0000  stw r24, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 82263A18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82263A1C: 931A0000  stw r24, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 82263A20: 48000010  b 0x82263a30
	pc = 0x82263A30; continue 'dispatch;
            }
            0x82263A24 => {
    //   block [0x82263A24..0x82263A30)
	// 82263A24: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82263A28: 1D6B0014  mulli r11, r11, 0x14
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 20 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82263A2C: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	pc = 0x82263A30; continue 'dispatch;
            }
            0x82263A30 => {
    //   block [0x82263A30..0x82263A38)
	// 82263A30: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82263A34: 4BE2B0A4  b 0x8208ead8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82263A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82263A38 size=180
    let mut pc: u32 = 0x82263A38;
    'dispatch: loop {
        match pc {
            0x82263A38 => {
    //   block [0x82263A38..0x82263AA0)
	// 82263A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82263A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82263A40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82263A44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82263A48: 3D60E06D  lis r11, -0x1f93
	ctx.r[11].s64 = -529727488;
	// 82263A4C: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82263A50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82263A54: 616B7363  ori r11, r11, 0x7363
	ctx.r[11].u64 = ctx.r[11].u64 | 29539;
	// 82263A58: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82263A5C: 409A0058  bne cr6, 0x82263ab4
	if !ctx.cr[6].eq {
	pc = 0x82263AB4; continue 'dispatch;
	}
	// 82263A60: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82263A64: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82263A68: 409A004C  bne cr6, 0x82263ab4
	if !ctx.cr[6].eq {
	pc = 0x82263AB4; continue 'dispatch;
	}
	// 82263A6C: 3D401993  lis r10, 0x1993
	ctx.r[10].s64 = 429064192;
	// 82263A70: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82263A74: 614A0520  ori r10, r10, 0x520
	ctx.r[10].u64 = ctx.r[10].u64 | 1312;
	// 82263A78: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82263A7C: 419A0024  beq cr6, 0x82263aa0
	if ctx.cr[6].eq {
	pc = 0x82263AA0; continue 'dispatch;
	}
	// 82263A80: 3D401993  lis r10, 0x1993
	ctx.r[10].s64 = 429064192;
	// 82263A84: 614A0521  ori r10, r10, 0x521
	ctx.r[10].u64 = ctx.r[10].u64 | 1313;
	// 82263A88: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82263A8C: 419A0014  beq cr6, 0x82263aa0
	if ctx.cr[6].eq {
	pc = 0x82263AA0; continue 'dispatch;
	}
	// 82263A90: 3D401993  lis r10, 0x1993
	ctx.r[10].s64 = 429064192;
	// 82263A94: 614A0522  ori r10, r10, 0x522
	ctx.r[10].u64 = ctx.r[10].u64 | 1314;
	// 82263A98: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82263A9C: 409A0018  bne cr6, 0x82263ab4
	if !ctx.cr[6].eq {
	pc = 0x82263AB4; continue 'dispatch;
	}
	pc = 0x82263AA0; continue 'dispatch;
            }
            0x82263AA0 => {
    //   block [0x82263AA0..0x82263AB4)
	// 82263AA0: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82263AA4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82263AA8: 8164001C  lwz r11, 0x1c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 82263AAC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82263AB0: 4800000C  b 0x82263abc
	pc = 0x82263ABC; continue 'dispatch;
            }
            0x82263AB4 => {
    //   block [0x82263AB4..0x82263ABC)
	// 82263AB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82263AB8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82263ABC; continue 'dispatch;
            }
            0x82263ABC => {
    //   block [0x82263ABC..0x82263AEC)
	// 82263ABC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82263AC0: 4BE2CF89  bl 0x82090a48
	ctx.lr = 0x82263AC4;
	sub_82090A48(ctx, base);
	// 82263AC4: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82263AC8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82263ACC: 4BE2CF7D  bl 0x82090a48
	ctx.lr = 0x82263AD0;
	sub_82090A48(ctx, base);
	// 82263AD0: 93E30094  stw r31, 0x94(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), ctx.r[31].u32 ) };
	// 82263AD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82263AD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82263ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82263AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82263AE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82263AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82263AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82263AF0 size=88
    let mut pc: u32 = 0x82263AF0;
    'dispatch: loop {
        match pc {
            0x82263AF0 => {
    //   block [0x82263AF0..0x82263B10)
	// 82263AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82263AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82263AF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82263AFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82263B00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82263B04: 4BE2CF45  bl 0x82090a48
	ctx.lr = 0x82263B08;
	sub_82090A48(ctx, base);
	// 82263B08: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82263B0C: 48000014  b 0x82263b20
	pc = 0x82263B20; continue 'dispatch;
            }
            0x82263B10 => {
    //   block [0x82263B10..0x82263B20)
	// 82263B10: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82263B14: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82263B18: 419A0028  beq cr6, 0x82263b40
	if ctx.cr[6].eq {
	pc = 0x82263B40; continue 'dispatch;
	}
	// 82263B1C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	pc = 0x82263B20; continue 'dispatch;
            }
            0x82263B20 => {
    //   block [0x82263B20..0x82263B2C)
	// 82263B20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82263B24: 409AFFEC  bne cr6, 0x82263b10
	if !ctx.cr[6].eq {
	pc = 0x82263B10; continue 'dispatch;
	}
	// 82263B28: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x82263B2C; continue 'dispatch;
            }
            0x82263B2C => {
    //   block [0x82263B2C..0x82263B40)
	// 82263B2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82263B30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82263B34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82263B38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82263B3C: 4E800020  blr
	return;
            }
            0x82263B40 => {
    //   block [0x82263B40..0x82263B48)
	// 82263B40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82263B44: 4BFFFFE8  b 0x82263b2c
	pc = 0x82263B2C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82263B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82263B48 size=172
    let mut pc: u32 = 0x82263B48;
    'dispatch: loop {
        match pc {
            0x82263B48 => {
    //   block [0x82263B48..0x82263B74)
	// 82263B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82263B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82263B50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82263B54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82263B58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82263B5C: 4BE2CEED  bl 0x82090a48
	ctx.lr = 0x82263B60;
	sub_82090A48(ctx, base);
	// 82263B60: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82263B64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82263B68: 419A000C  beq cr6, 0x82263b74
	if ctx.cr[6].eq {
	pc = 0x82263B74; continue 'dispatch;
	}
	// 82263B6C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82263B70: 409A0008  bne cr6, 0x82263b78
	if !ctx.cr[6].eq {
	pc = 0x82263B78; continue 'dispatch;
	}
	pc = 0x82263B74; continue 'dispatch;
            }
            0x82263B74 => {
    //   block [0x82263B74..0x82263B78)
	// 82263B74: 4BE3341D  bl 0x82096f90
	ctx.lr = 0x82263B78;
	sub_82096F90(ctx, base);
	pc = 0x82263B78; continue 'dispatch;
            }
            0x82263B78 => {
    //   block [0x82263B78..0x82263B98)
	// 82263B78: 4BE2CED1  bl 0x82090a48
	ctx.lr = 0x82263B7C;
	sub_82090A48(ctx, base);
	// 82263B7C: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82263B80: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82263B84: 409A0014  bne cr6, 0x82263b98
	if !ctx.cr[6].eq {
	pc = 0x82263B98; continue 'dispatch;
	}
	// 82263B88: 4BE2CEC1  bl 0x82090a48
	ctx.lr = 0x82263B8C;
	sub_82090A48(ctx, base);
	// 82263B8C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82263B90: 91630094  stw r11, 0x94(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82263B94: 48000040  b 0x82263bd4
	pc = 0x82263BD4; continue 'dispatch;
            }
            0x82263B98 => {
    //   block [0x82263B98..0x82263BB0)
	// 82263B98: 4BE2CEB1  bl 0x82090a48
	ctx.lr = 0x82263B9C;
	sub_82090A48(ctx, base);
	// 82263B9C: 81230094  lwz r9, 0x94(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82263BA0: 39690008  addi r11, r9, 8
	ctx.r[11].s64 = ctx.r[9].s64 + 8;
	// 82263BA4: 81490008  lwz r10, 8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82263BA8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82263BAC: 419A0024  beq cr6, 0x82263bd0
	if ctx.cr[6].eq {
	pc = 0x82263BD0; continue 'dispatch;
	}
	pc = 0x82263BB0; continue 'dispatch;
            }
            0x82263BB0 => {
    //   block [0x82263BB0..0x82263BD0)
	// 82263BB0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82263BB4: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82263BB8: 419A0030  beq cr6, 0x82263be8
	if ctx.cr[6].eq {
	pc = 0x82263BE8; continue 'dispatch;
	}
	// 82263BBC: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82263BC0: 396A0008  addi r11, r10, 8
	ctx.r[11].s64 = ctx.r[10].s64 + 8;
	// 82263BC4: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82263BC8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82263BCC: 409AFFE4  bne cr6, 0x82263bb0
	if !ctx.cr[6].eq {
	pc = 0x82263BB0; continue 'dispatch;
	}
	pc = 0x82263BD0; continue 'dispatch;
            }
            0x82263BD0 => {
    //   block [0x82263BD0..0x82263BD4)
	// 82263BD0: 4BE333C1  bl 0x82096f90
	ctx.lr = 0x82263BD4;
	sub_82096F90(ctx, base);
	pc = 0x82263BD4; continue 'dispatch;
            }
            0x82263BD4 => {
    //   block [0x82263BD4..0x82263BE8)
	// 82263BD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82263BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82263BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82263BE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82263BE4: 4E800020  blr
	return;
            }
            0x82263BE8 => {
    //   block [0x82263BE8..0x82263BF4)
	// 82263BE8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82263BEC: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82263BF0: 4BFFFFE4  b 0x82263bd4
	pc = 0x82263BD4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82263BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82263BF8 size=248
    let mut pc: u32 = 0x82263BF8;
    'dispatch: loop {
        match pc {
            0x82263BF8 => {
    //   block [0x82263BF8..0x82263C48)
	// 82263BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82263BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82263C00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82263C04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82263C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82263C0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82263C10: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82263C14: 409A0034  bne cr6, 0x82263c48
	if !ctx.cr[6].eq {
	pc = 0x82263C48; continue 'dispatch;
	}
	// 82263C18: 4BE2EE61  bl 0x82092a78
	ctx.lr = 0x82263C1C;
	sub_82092A78(ctx, base);
	// 82263C1C: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 82263C20: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82263C24: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82263C28: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82263C2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82263C30: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82263C34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82263C38: 4BE2ED01  bl 0x82092938
	ctx.lr = 0x82263C3C;
	sub_82092938(ctx, base);
	// 82263C3C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82263C40: C82B06F0  lfd f1, 0x6f0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(1776 as u32) ) };
	// 82263C44: 48000094  b 0x82263cd8
	pc = 0x82263CD8; continue 'dispatch;
            }
            0x82263C48 => {
    //   block [0x82263C48..0x82263C54)
	// 82263C48: 3D608227  lis r11, -0x7dd9
	ctx.r[11].s64 = -2111373312;
	// 82263C4C: 3BCB24A8  addi r30, r11, 0x24a8
	ctx.r[30].s64 = ctx.r[11].s64 + 9384;
	// 82263C50: 816B24A8  lwz r11, 0x24a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9384 as u32) ) } as u64;
	pc = 0x82263C54; continue 'dispatch;
            }
            0x82263C54 => {
    //   block [0x82263C54..0x82263C78)
	// 82263C54: 814B00AC  lwz r10, 0xac(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 82263C58: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82263C5C: 4099001C  ble cr6, 0x82263c78
	if !ctx.cr[6].gt {
	pc = 0x82263C78; continue 'dispatch;
	}
	// 82263C60: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82263C64: 887F0000  lbz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82263C68: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82263C6C: 4BE35515  bl 0x82099180
	ctx.lr = 0x82263C70;
	sub_82099180(ctx, base);
	// 82263C70: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82263C74: 48000018  b 0x82263c8c
	pc = 0x82263C8C; continue 'dispatch;
            }
            0x82263C78 => {
    //   block [0x82263C78..0x82263C8C)
	// 82263C78: 895F0000  lbz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82263C7C: 812B00C8  lwz r9, 0xc8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 82263C80: 554A083E  rotlwi r10, r10, 1
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 82263C84: 7D4A4A2E  lhzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82263C88: 55430738  rlwinm r3, r10, 0, 0x1c, 0x1c
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	pc = 0x82263C8C; continue 'dispatch;
            }
            0x82263C8C => {
    //   block [0x82263C8C..0x82263C9C)
	// 82263C8C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82263C90: 419A000C  beq cr6, 0x82263c9c
	if ctx.cr[6].eq {
	pc = 0x82263C9C; continue 'dispatch;
	}
	// 82263C94: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82263C98: 4BFFFFBC  b 0x82263c54
	pc = 0x82263C54; continue 'dispatch;
            }
            0x82263C9C => {
    //   block [0x82263C9C..0x82263CA0)
	// 82263C9C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x82263CA0; continue 'dispatch;
            }
            0x82263CA0 => {
    //   block [0x82263CA0..0x82263CD8)
	// 82263CA0: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82263CA4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82263CA8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82263CAC: 409AFFF4  bne cr6, 0x82263ca0
	if !ctx.cr[6].eq {
	pc = 0x82263CA0; continue 'dispatch;
	}
	// 82263CB0: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82263CB4: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82263CB8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82263CBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82263CC0: 5565003E  slwi r5, r11, 0
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82263CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82263CC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82263CCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82263CD0: 48002C41  bl 0x82266910
	ctx.lr = 0x82263CD4;
	sub_82266910(ctx, base);
	// 82263CD4: C8230010  lfd f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	pc = 0x82263CD8; continue 'dispatch;
            }
            0x82263CD8 => {
    //   block [0x82263CD8..0x82263CF0)
	// 82263CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82263CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82263CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82263CE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82263CE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82263CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82263CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82263CF0 size=8
    let mut pc: u32 = 0x82263CF0;
    'dispatch: loop {
        match pc {
            0x82263CF0 => {
    //   block [0x82263CF0..0x82263CF8)
	// 82263CF0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82263CF4: 4BFFFF04  b 0x82263bf8
	sub_82263BF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82263CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82263CF8 size=24
    let mut pc: u32 = 0x82263CF8;
    'dispatch: loop {
        match pc {
            0x82263CF8 => {
    //   block [0x82263CF8..0x82263D10)
	// 82263CF8: 2F030061  cmpwi cr6, r3, 0x61
	ctx.cr[6].compare_i32(ctx.r[3].s32, 97, &mut ctx.xer);
	// 82263CFC: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
	// 82263D00: 2F03007A  cmpwi cr6, r3, 0x7a
	ctx.cr[6].compare_i32(ctx.r[3].s32, 122, &mut ctx.xer);
	// 82263D04: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
	// 82263D08: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82263D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82263D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82263D10 size=228
    let mut pc: u32 = 0x82263D10;
    'dispatch: loop {
        match pc {
            0x82263D10 => {
    //   block [0x82263D10..0x82263D40)
	// 82263D10: FC000A10  fabs f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 82263D14: 3D608228  lis r11, -0x7dd8
	ctx.r[11].s64 = -2111307776;
	// 82263D18: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 82263D1C: 396B6B18  addi r11, r11, 0x6b18
	ctx.r[11].s64 = ctx.r[11].s64 + 27416;
	// 82263D20: C1AB00B0  lfs f13, 0xb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82263D24: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82263D28: 40990024  ble cr6, 0x82263d4c
	if !ctx.cr[6].gt {
	pc = 0x82263D4C; continue 'dispatch;
	}
	// 82263D2C: C98B0030  lfd f12, 0x30(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82263D30: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82263D34: 4099000C  ble cr6, 0x82263d40
	if !ctx.cr[6].gt {
	pc = 0x82263D40; continue 'dispatch;
	}
	// 82263D38: C80B0008  lfd f0, 8(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82263D3C: 480000A0  b 0x82263ddc
	pc = 0x82263DDC; continue 'dispatch;
            }
            0x82263D40 => {
    //   block [0x82263D40..0x82263D4C)
	// 82263D40: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82263D44: FC0D0024  fdiv f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 / ctx.f[0].f64;
	// 82263D48: 48000008  b 0x82263d50
	pc = 0x82263D50; continue 'dispatch;
            }
            0x82263D4C => {
    //   block [0x82263D4C..0x82263D50)
	// 82263D4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x82263D50; continue 'dispatch;
            }
            0x82263D50 => {
    //   block [0x82263D50..0x82263D70)
	// 82263D50: C98B0018  lfd f12, 0x18(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82263D54: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82263D58: 40990018  ble cr6, 0x82263d70
	if !ctx.cr[6].gt {
	pc = 0x82263D70; continue 'dispatch;
	}
	// 82263D5C: C98B0028  lfd f12, 0x28(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82263D60: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82263D64: FD6C002A  fadd f11, f12, f0
	ctx.f[11].f64 = ctx.f[12].f64 + ctx.f[0].f64;
	// 82263D68: FC0C6838  fmsub f0, f12, f0, f13
	ctx.f[0].f64 = ctx.f[12].f64 * ctx.f[0].f64 - ctx.f[13].f64;
	// 82263D6C: FC005824  fdiv f0, f0, f11
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[11].f64;
	pc = 0x82263D70; continue 'dispatch;
            }
            0x82263D70 => {
    //   block [0x82263D70..0x82263DCC)
	// 82263D70: FCA00032  fmul f5, f0, f0
	ctx.f[5].f64 = ctx.f[0].f64 * ctx.f[0].f64;
	// 82263D74: C98B0048  lfd f12, 0x48(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) };
	// 82263D78: C9AB0050  lfd f13, 0x50(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 82263D7C: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82263D80: C96B0070  lfd f11, 0x70(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) };
	// 82263D84: C94B0040  lfd f10, 0x40(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 82263D88: C92B0068  lfd f9, 0x68(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) };
	// 82263D8C: C90B0038  lfd f8, 0x38(r11)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82263D90: C8EB0060  lfd f7, 0x60(r11)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	// 82263D94: C8CB0058  lfd f6, 0x58(r11)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	// 82263D98: FDAD617A  fmadd f13, f13, f5, f12
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[5].f64 + ctx.f[12].f64;
	// 82263D9C: FD8B282A  fadd f12, f11, f5
	ctx.f[12].f64 = ctx.f[11].f64 + ctx.f[5].f64;
	// 82263DA0: FDAD517A  fmadd f13, f13, f5, f10
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[5].f64 + ctx.f[10].f64;
	// 82263DA4: FD8C497A  fmadd f12, f12, f5, f9
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[5].f64 + ctx.f[9].f64;
	// 82263DA8: FDAD417A  fmadd f13, f13, f5, f8
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[5].f64 + ctx.f[8].f64;
	// 82263DAC: FD8C397A  fmadd f12, f12, f5, f7
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[5].f64 + ctx.f[7].f64;
	// 82263DB0: FDAD0172  fmul f13, f13, f5
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[5].f64;
	// 82263DB4: FD8C317A  fmadd f12, f12, f5, f6
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[5].f64 + ctx.f[6].f64;
	// 82263DB8: FDAD0032  fmul f13, f13, f0
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 82263DBC: FDAD6024  fdiv f13, f13, f12
	ctx.f[13].f64 = ctx.f[13].f64 / ctx.f[12].f64;
	// 82263DC0: FC0D002A  fadd f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	// 82263DC4: 40990008  ble cr6, 0x82263dcc
	if !ctx.cr[6].gt {
	pc = 0x82263DCC; continue 'dispatch;
	}
	// 82263DC8: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	pc = 0x82263DCC; continue 'dispatch;
            }
            0x82263DCC => {
    //   block [0x82263DCC..0x82263DDC)
	// 82263DCC: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82263DD0: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	// 82263DD4: 7DAA5CAE  lfdx f13, r10, r11
	ctx.f[13].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) };
	// 82263DD8: FC0D002A  fadd f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	pc = 0x82263DDC; continue 'dispatch;
            }
            0x82263DDC => {
    //   block [0x82263DDC..0x82263DF4)
	// 82263DDC: E961FFF0  ld r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82263DE0: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 82263DE4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82263DE8: FDA00050  fneg f13, f0
	ctx.f[13].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82263DEC: FC21682E  fsel f1, f1, f0, f13
	ctx.f[1].f64 = if ctx.f[1].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 82263DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82263DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82263DF8 size=72
    let mut pc: u32 = 0x82263DF8;
    'dispatch: loop {
        match pc {
            0x82263DF8 => {
    //   block [0x82263DF8..0x82263E40)
	// 82263DF8: 3D608228  lis r11, -0x7dd8
	ctx.r[11].s64 = -2111307776;
	// 82263DFC: D8210010  stfd f1, 0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.f[1].u64 ) };
	// 82263E00: D8410018  stfd f2, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[2].u64 ) };
	// 82263E04: 396B6B18  addi r11, r11, 0x6b18
	ctx.r[11].s64 = ctx.r[11].s64 + 27416;
	// 82263E08: C00B00A8  lfs f0, 0xa8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82263E0C: FF020000  fcmpu cr6, f2, f0
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[0].f64);
	// 82263E10: 409A0040  bne cr6, 0x82263e50
	if !ctx.cr[6].eq {
		sub_82263E48(ctx, base);
		return;
	}
	// 82263E14: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82263E18: 409A0030  bne cr6, 0x82263e48
	if !ctx.cr[6].eq {
		sub_82263E48(ctx, base);
		return;
	}
	// 82263E1C: 81410018  lwz r10, 0x18(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) } as u64;
	// 82263E20: 554A0001  rlwinm. r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82263E24: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
	// 82263E28: 81410010  lwz r10, 0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 82263E2C: 554A0001  rlwinm. r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82263E30: 41820010  beq 0x82263e40
	if ctx.cr[0].eq {
		sub_82263E40(ctx, base);
		return;
	}
	// 82263E34: C80B0010  lfd f0, 0x10(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82263E38: FC200050  fneg f1, f0
	ctx.f[1].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82263E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82263E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82263E40 size=8
    let mut pc: u32 = 0x82263E40;
    'dispatch: loop {
        match pc {
            0x82263E40 => {
    //   block [0x82263E40..0x82263E48)
	// 82263E40: C82B0010  lfd f1, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82263E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82263E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82263E48 size=224
    let mut pc: u32 = 0x82263E48;
    'dispatch: loop {
        match pc {
            0x82263E48 => {
    //   block [0x82263E48..0x82263E74)
	// 82263E48: C80B0008  lfd f0, 8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82263E4C: 480000BC  b 0x82263f08
	pc = 0x82263F08; continue 'dispatch;
	// 82263E50: FDA01210  fabs f13, f2
	ctx.f[13].u64 = ctx.f[2].u64 & !0x8000_0000_0000_0000u64;
	// 82263E54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82263E58: FC000A10  fabs f0, f1
	ctx.f[0].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 82263E5C: FD806890  fmr f12, f13
	ctx.f[12].f64 = ctx.f[13].f64;
	// 82263E60: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82263E64: 40990010  ble cr6, 0x82263e74
	if !ctx.cr[6].gt {
	pc = 0x82263E74; continue 'dispatch;
	}
	// 82263E68: FD800090  fmr f12, f0
	ctx.f[12].f64 = ctx.f[0].f64;
	// 82263E6C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82263E70: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
            }
            0x82263E74 => {
    //   block [0x82263E74..0x82263E9C)
	// 82263E74: FC006024  fdiv f0, f0, f12
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[12].f64;
	// 82263E78: C9AB0018  lfd f13, 0x18(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82263E7C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82263E80: 4099001C  ble cr6, 0x82263e9c
	if !ctx.cr[6].gt {
	pc = 0x82263E9C; continue 'dispatch;
	}
	// 82263E84: C9AB0028  lfd f13, 0x28(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82263E88: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82263E8C: C18B00B0  lfs f12, 0xb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82263E90: FD6D002A  fadd f11, f13, f0
	ctx.f[11].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	// 82263E94: FC0D6038  fmsub f0, f13, f0, f12
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[12].f64;
	// 82263E98: FC005824  fdiv f0, f0, f11
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[11].f64;
	pc = 0x82263E9C; continue 'dispatch;
            }
            0x82263E9C => {
    //   block [0x82263E9C..0x82263EF8)
	// 82263E9C: FCA00032  fmul f5, f0, f0
	ctx.f[5].f64 = ctx.f[0].f64 * ctx.f[0].f64;
	// 82263EA0: C98B0048  lfd f12, 0x48(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) };
	// 82263EA4: C9AB0050  lfd f13, 0x50(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 82263EA8: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82263EAC: C96B0070  lfd f11, 0x70(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) };
	// 82263EB0: C94B0040  lfd f10, 0x40(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 82263EB4: C92B0068  lfd f9, 0x68(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) };
	// 82263EB8: C90B0038  lfd f8, 0x38(r11)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82263EBC: C8EB0060  lfd f7, 0x60(r11)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	// 82263EC0: C8CB0058  lfd f6, 0x58(r11)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	// 82263EC4: FDAD617A  fmadd f13, f13, f5, f12
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[5].f64 + ctx.f[12].f64;
	// 82263EC8: FD8B282A  fadd f12, f11, f5
	ctx.f[12].f64 = ctx.f[11].f64 + ctx.f[5].f64;
	// 82263ECC: FDAD517A  fmadd f13, f13, f5, f10
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[5].f64 + ctx.f[10].f64;
	// 82263ED0: FD8C497A  fmadd f12, f12, f5, f9
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[5].f64 + ctx.f[9].f64;
	// 82263ED4: FDAD417A  fmadd f13, f13, f5, f8
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[5].f64 + ctx.f[8].f64;
	// 82263ED8: FD8C397A  fmadd f12, f12, f5, f7
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[5].f64 + ctx.f[7].f64;
	// 82263EDC: FDAD0172  fmul f13, f13, f5
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[5].f64;
	// 82263EE0: FD8C317A  fmadd f12, f12, f5, f6
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[5].f64 + ctx.f[6].f64;
	// 82263EE4: FDAD0032  fmul f13, f13, f0
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 82263EE8: FDAD6024  fdiv f13, f13, f12
	ctx.f[13].f64 = ctx.f[13].f64 / ctx.f[12].f64;
	// 82263EEC: FC0D002A  fadd f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	// 82263EF0: 40990008  ble cr6, 0x82263ef8
	if !ctx.cr[6].gt {
	pc = 0x82263EF8; continue 'dispatch;
	}
	// 82263EF4: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	pc = 0x82263EF8; continue 'dispatch;
            }
            0x82263EF8 => {
    //   block [0x82263EF8..0x82263F08)
	// 82263EF8: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82263EFC: 392B0080  addi r9, r11, 0x80
	ctx.r[9].s64 = ctx.r[11].s64 + 128;
	// 82263F00: 7DAA4CAE  lfdx f13, r10, r9
	ctx.f[13].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 82263F04: FC0D002A  fadd f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	pc = 0x82263F08; continue 'dispatch;
            }
            0x82263F08 => {
    //   block [0x82263F08..0x82263F28)
	// 82263F08: C9AB0010  lfd f13, 0x10(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82263F0C: 81610010  lwz r11, 0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 82263F10: FDAD0028  fsub f13, f13, f0
	ctx.f[13].f64 = ctx.f[13].f64 - ctx.f[0].f64;
	// 82263F14: 556B0001  rlwinm. r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82263F18: FC22682E  fsel f1, f2, f0, f13
	ctx.f[1].f64 = if ctx.f[2].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 82263F1C: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
	// 82263F20: FC200850  fneg f1, f1
	ctx.f[1].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 82263F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82263F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82263F28 size=216
    let mut pc: u32 = 0x82263F28;
    'dispatch: loop {
        match pc {
            0x82263F28 => {
    //   block [0x82263F28..0x82263F64)
	// 82263F28: FDA00A10  fabs f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 82263F2C: 3D608228  lis r11, -0x7dd8
	ctx.r[11].s64 = -2111307776;
	// 82263F30: 396B6BD0  addi r11, r11, 0x6bd0
	ctx.r[11].s64 = ctx.r[11].s64 + 27600;
	// 82263F34: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82263F38: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82263F3C: 40990028  ble cr6, 0x82263f64
	if !ctx.cr[6].gt {
	pc = 0x82263F64; continue 'dispatch;
	}
	// 82263F40: C18B0004  lfs f12, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82263F44: 21440001  subfic r10, r4, 1
	ctx.xer.ca = ctx.r[4].u32 <= 1 as u32;
	ctx.r[10].s64 = (1 as i64) - ctx.r[4].s64;
	// 82263F48: FD8C6828  fsub f12, f12, f13
	ctx.f[12].f64 = ctx.f[12].f64 - ctx.f[13].f64;
	// 82263F4C: C1AB0008  lfs f13, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82263F50: FC0C0032  fmul f0, f12, f0
	ctx.f[0].f64 = ctx.f[12].f64 * ctx.f[0].f64;
	// 82263F54: FD80002C  fsqrt f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64).sqrt();
	// 82263F58: FDAC0372  fmul f13, f12, f13
	ctx.f[13].f64 = ctx.f[12].f64 * ctx.f[13].f64;
	// 82263F5C: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 82263F60: 48000020  b 0x82263f80
	pc = 0x82263F80; continue 'dispatch;
            }
            0x82263F64 => {
    //   block [0x82263F64..0x82263F80)
	// 82263F64: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82263F68: FC0D0372  fmul f0, f13, f13
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[13].f64;
	// 82263F6C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82263F70: 409A0010  bne cr6, 0x82263f80
	if !ctx.cr[6].eq {
	pc = 0x82263F80; continue 'dispatch;
	}
	// 82263F74: C18B0000  lfs f12, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82263F78: FF016000  fcmpu cr6, f1, f12
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[12].f64);
	// 82263F7C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	pc = 0x82263F80; continue 'dispatch;
            }
            0x82263F80 => {
    //   block [0x82263F80..0x82264000)
	// 82263F80: C98B0050  lfd f12, 0x50(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 82263F84: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82263F88: C96B0048  lfd f11, 0x48(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) };
	// 82263F8C: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82263F90: FCAC583A  fmadd f5, f12, f0, f11
	ctx.f[5].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 82263F94: C98B0078  lfd f12, 0x78(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) };
	// 82263F98: C96B0040  lfd f11, 0x40(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 82263F9C: FC8C002A  fadd f4, f12, f0
	ctx.f[4].f64 = ctx.f[12].f64 + ctx.f[0].f64;
	// 82263FA0: C98B0070  lfd f12, 0x70(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) };
	// 82263FA4: C94B0038  lfd f10, 0x38(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82263FA8: C92B0068  lfd f9, 0x68(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) };
	// 82263FAC: C90B0030  lfd f8, 0x30(r11)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82263FB0: C8EB0060  lfd f7, 0x60(r11)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	// 82263FB4: C8CB0058  lfd f6, 0x58(r11)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	// 82263FB8: FD65583A  fmadd f11, f5, f0, f11
	ctx.f[11].f64 = ctx.f[5].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 82263FBC: FD84603A  fmadd f12, f4, f0, f12
	ctx.f[12].f64 = ctx.f[4].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 82263FC0: FD6B503A  fmadd f11, f11, f0, f10
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[10].f64;
	// 82263FC4: FD8C483A  fmadd f12, f12, f0, f9
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[9].f64;
	// 82263FC8: FD6B403A  fmadd f11, f11, f0, f8
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[8].f64;
	// 82263FCC: FD8C383A  fmadd f12, f12, f0, f7
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[7].f64;
	// 82263FD0: FD6B0032  fmul f11, f11, f0
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64;
	// 82263FD4: FC0C303A  fmadd f0, f12, f0, f6
	ctx.f[0].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[6].f64;
	// 82263FD8: FD8B0372  fmul f12, f11, f13
	ctx.f[12].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	// 82263FDC: FC0C0024  fdiv f0, f12, f0
	ctx.f[0].f64 = ctx.f[12].f64 / ctx.f[0].f64;
	// 82263FE0: FC00682A  fadd f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 + ctx.f[13].f64;
	// 82263FE4: 409A001C  bne cr6, 0x82264000
	if !ctx.cr[6].eq {
		sub_82264000(ctx, base);
		return;
	}
	// 82263FE8: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82263FEC: 7DAA5CAE  lfdx f13, r10, r11
	ctx.f[13].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) };
	// 82263FF0: FC0D002A  fadd f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	// 82263FF4: FDA00050  fneg f13, f0
	ctx.f[13].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82263FF8: FC21682E  fsel f1, f1, f0, f13
	ctx.f[1].f64 = if ctx.f[1].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 82263FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82264000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82264000 size=32
    let mut pc: u32 = 0x82264000;
    'dispatch: loop {
        match pc {
            0x82264000 => {
    //   block [0x82264000..0x82264020)
	// 82264000: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82264004: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82264008: 7DAA4CAE  lfdx f13, r10, r9
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 8226400C: 7D8A5CAE  lfdx f12, r10, r11
	ctx.f[12].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) };
	// 82264010: FDAD0028  fsub f13, f13, f0
	ctx.f[13].f64 = ctx.f[13].f64 - ctx.f[0].f64;
	// 82264014: FC0C002A  fadd f0, f12, f0
	ctx.f[0].f64 = ctx.f[12].f64 + ctx.f[0].f64;
	// 82264018: FC21036E  fsel f1, f1, f13, f0
	ctx.f[1].f64 = if ctx.f[1].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 8226401C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82264020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82264020 size=200
    let mut pc: u32 = 0x82264020;
    'dispatch: loop {
        match pc {
            0x82264020 => {
    //   block [0x82264020..0x8226405C)
	// 82264020: FDA00A10  fabs f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 82264024: 3D608228  lis r11, -0x7dd8
	ctx.r[11].s64 = -2111307776;
	// 82264028: 396B6BD0  addi r11, r11, 0x6bd0
	ctx.r[11].s64 = ctx.r[11].s64 + 27600;
	// 8226402C: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82264030: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82264034: 40990028  ble cr6, 0x8226405c
	if !ctx.cr[6].gt {
	pc = 0x8226405C; continue 'dispatch;
	}
	// 82264038: C18B0004  lfs f12, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8226403C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82264040: FD8C6828  fsub f12, f12, f13
	ctx.f[12].f64 = ctx.f[12].f64 - ctx.f[13].f64;
	// 82264044: C1AB0008  lfs f13, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82264048: FC0C0032  fmul f0, f12, f0
	ctx.f[0].f64 = ctx.f[12].f64 * ctx.f[0].f64;
	// 8226404C: FD80002C  fsqrt f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64).sqrt();
	// 82264050: FDAC0372  fmul f13, f12, f13
	ctx.f[13].f64 = ctx.f[12].f64 * ctx.f[13].f64;
	// 82264054: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 82264058: 48000018  b 0x82264070
	pc = 0x82264070; continue 'dispatch;
            }
            0x8226405C => {
    //   block [0x8226405C..0x82264070)
	// 8226405C: C18B0000  lfs f12, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82264060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82264064: FC0D0372  fmul f0, f13, f13
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[13].f64;
	// 82264068: FF016000  fcmpu cr6, f1, f12
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[12].f64);
	// 8226406C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	pc = 0x82264070; continue 'dispatch;
            }
            0x82264070 => {
    //   block [0x82264070..0x822640E8)
	// 82264070: C98B0050  lfd f12, 0x50(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 82264074: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82264078: C96B0048  lfd f11, 0x48(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) };
	// 8226407C: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82264080: FCAC583A  fmadd f5, f12, f0, f11
	ctx.f[5].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 82264084: C98B0078  lfd f12, 0x78(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) };
	// 82264088: C96B0040  lfd f11, 0x40(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 8226408C: FC8C002A  fadd f4, f12, f0
	ctx.f[4].f64 = ctx.f[12].f64 + ctx.f[0].f64;
	// 82264090: C98B0070  lfd f12, 0x70(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) };
	// 82264094: C94B0038  lfd f10, 0x38(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82264098: C92B0068  lfd f9, 0x68(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) };
	// 8226409C: C90B0030  lfd f8, 0x30(r11)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 822640A0: C8EB0060  lfd f7, 0x60(r11)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	// 822640A4: C8CB0058  lfd f6, 0x58(r11)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	// 822640A8: 7C6A4CAE  lfdx f3, r10, r9
	ctx.f[3].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 822640AC: FD65583A  fmadd f11, f5, f0, f11
	ctx.f[11].f64 = ctx.f[5].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 822640B0: FD84603A  fmadd f12, f4, f0, f12
	ctx.f[12].f64 = ctx.f[4].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 822640B4: FD6B503A  fmadd f11, f11, f0, f10
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[10].f64;
	// 822640B8: FD8C483A  fmadd f12, f12, f0, f9
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[9].f64;
	// 822640BC: FD6B403A  fmadd f11, f11, f0, f8
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[8].f64;
	// 822640C0: FD8C383A  fmadd f12, f12, f0, f7
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[7].f64;
	// 822640C4: FD6B0032  fmul f11, f11, f0
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64;
	// 822640C8: FC0C303A  fmadd f0, f12, f0, f6
	ctx.f[0].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[6].f64;
	// 822640CC: FD8B0372  fmul f12, f11, f13
	ctx.f[12].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	// 822640D0: FC0C0024  fdiv f0, f12, f0
	ctx.f[0].f64 = ctx.f[12].f64 / ctx.f[0].f64;
	// 822640D4: FC00682A  fadd f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 + ctx.f[13].f64;
	// 822640D8: FC00182A  fadd f0, f0, f3
	ctx.f[0].f64 = ctx.f[0].f64 + ctx.f[3].f64;
	// 822640DC: FDA00050  fneg f13, f0
	ctx.f[13].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 822640E0: FC21682E  fsel f1, f1, f0, f13
	ctx.f[1].f64 = if ctx.f[1].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 822640E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822640E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822640E8 size=8
    let mut pc: u32 = 0x822640E8;
    'dispatch: loop {
        match pc {
            0x822640E8 => {
    //   block [0x822640E8..0x822640F0)
	// 822640E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822640EC: 4BFFFE3C  b 0x82263f28
	sub_82263F28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822640F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822640F0 size=60
    let mut pc: u32 = 0x822640F0;
    'dispatch: loop {
        match pc {
            0x822640F0 => {
    //   block [0x822640F0..0x822640FC)
	// 822640F0: 88C30000  lbz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822640F4: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 822640F8: 4182001C  beq 0x82264114
	if ctx.cr[0].eq {
	pc = 0x82264114; continue 'dispatch;
	}
	pc = 0x822640FC; continue 'dispatch;
            }
            0x822640FC => {
    //   block [0x822640FC..0x82264114)
	// 822640FC: 2C860000  cmpwi cr1, r6, 0
	ctx.cr[1].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82264100: 7C062000  cmpw r6, r4
	ctx.cr[0].compare_i32(ctx.r[4].s32, ctx.r[0].s32, &mut ctx.xer);
	// 82264104: 41860020  beq cr1, 0x82264124
	if ctx.cr[1].eq {
	pc = 0x82264124; continue 'dispatch;
	}
	// 82264108: 41820020  beq 0x82264128
	if ctx.cr[0].eq {
	pc = 0x82264128; continue 'dispatch;
	}
	// 8226410C: 8CC30001  lbzu r6, 1(r3)
	ea = ctx.r[3].u32.wrapping_add(1 as u32);
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[3].u32 = ea;
	// 82264110: 4BFFFFEC  b 0x822640fc
	pc = 0x822640FC; continue 'dispatch;
            }
            0x82264114 => {
    //   block [0x82264114..0x82264124)
	// 82264114: 2C060000  cmpwi r6, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82264118: 41820010  beq 0x82264128
	if ctx.cr[0].eq {
	pc = 0x82264128; continue 'dispatch;
	}
	// 8226411C: 8CC30001  lbzu r6, 1(r3)
	ea = ctx.r[3].u32.wrapping_add(1 as u32);
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[3].u32 = ea;
	// 82264120: 4BFFFFF4  b 0x82264114
	pc = 0x82264114; continue 'dispatch;
            }
            0x82264124 => {
    //   block [0x82264124..0x82264128)
	// 82264124: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82264128; continue 'dispatch;
            }
            0x82264128 => {
    //   block [0x82264128..0x8226412C)
	// 82264128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82264130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82264130 size=140
    let mut pc: u32 = 0x82264130;
    'dispatch: loop {
        match pc {
            0x82264130 => {
    //   block [0x82264130..0x8226414C)
	// 82264130: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82264134: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82264138: 419A0014  beq cr6, 0x8226414c
	if ctx.cr[6].eq {
	pc = 0x8226414C; continue 'dispatch;
	}
	// 8226413C: 3940002D  li r10, 0x2d
	ctx.r[10].s64 = 45;
	// 82264140: 39640001  addi r11, r4, 1
	ctx.r[11].s64 = ctx.r[4].s64 + 1;
	// 82264144: 99440000  stb r10, 0(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82264148: 7C6300D0  neg r3, r3
	ctx.r[3].s64 = -ctx.r[3].s64;
	pc = 0x8226414C; continue 'dispatch;
            }
            0x8226414C => {
    //   block [0x8226414C..0x82264150)
	// 8226414C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	pc = 0x82264150; continue 'dispatch;
            }
            0x82264150 => {
    //   block [0x82264150..0x82264178)
	// 82264150: 7D432B96  divwu r10, r3, r5
	ctx.r[10].u32 = ctx.r[3].u32 / ctx.r[5].u32;
	// 82264154: 0CC50000  twi 6, r5, 0
	// 82264158: 7D4A29D6  mullw r10, r10, r5
	ctx.r[10].s32 = ((ctx.r[10].s32 as i64 * ctx.r[5].s32 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8226415C: 7D4A1850  subf r10, r10, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[10].s64;
	// 82264160: 7C632B96  divwu r3, r3, r5
	ctx.r[3].u32 = ctx.r[3].u32 / ctx.r[5].u32;
	// 82264164: 0CC50000  twi 6, r5, 0
	// 82264168: 2B0A0009  cmplwi cr6, r10, 9
	ctx.cr[6].compare_u32(ctx.r[10].u32, 9 as u32, &mut ctx.xer);
	// 8226416C: 4099000C  ble cr6, 0x82264178
	if !ctx.cr[6].gt {
	pc = 0x82264178; continue 'dispatch;
	}
	// 82264170: 394A0057  addi r10, r10, 0x57
	ctx.r[10].s64 = ctx.r[10].s64 + 87;
	// 82264174: 48000008  b 0x8226417c
	pc = 0x8226417C; continue 'dispatch;
            }
            0x82264178 => {
    //   block [0x82264178..0x8226417C)
	// 82264178: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	pc = 0x8226417C; continue 'dispatch;
            }
            0x8226417C => {
    //   block [0x8226417C..0x82264198)
	// 8226417C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82264180: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82264184: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82264188: 409AFFC8  bne cr6, 0x82264150
	if !ctx.cr[6].eq {
	pc = 0x82264150; continue 'dispatch;
	}
	// 8226418C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82264190: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82264194: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	pc = 0x82264198; continue 'dispatch;
            }
            0x82264198 => {
    //   block [0x82264198..0x822641BC)
	// 82264198: 89490000  lbz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226419C: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822641A0: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 822641A4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 822641A8: 99090000  stb r8, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 822641AC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 822641B0: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822641B4: 4198FFE4  blt cr6, 0x82264198
	if ctx.cr[6].lt {
	pc = 0x82264198; continue 'dispatch;
	}
	// 822641B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822641C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822641C0 size=44
    let mut pc: u32 = 0x822641C0;
    'dispatch: loop {
        match pc {
            0x822641C0 => {
    //   block [0x822641C0..0x822641EC)
	// 822641C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822641C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822641C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822641CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822641D0: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 822641D4: 4BFFFF5D  bl 0x82264130
	ctx.lr = 0x822641D8;
	sub_82264130(ctx, base);
	// 822641D8: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 822641DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822641E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822641E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822641E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822641F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822641F0 size=256
    let mut pc: u32 = 0x822641F0;
    'dispatch: loop {
        match pc {
            0x822641F0 => {
    //   block [0x822641F0..0x82264220)
	// 822641F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822641F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822641F8: DBC1FFE8  stfd f30, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[30].u64 ) };
	// 822641FC: DBE1FFF0  stfd f31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 82264200: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82264204: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82264208: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8226420C: C9AB06F0  lfd f13, 0x6f0(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(1776 as u32) ) };
	// 82264210: FC00F210  fabs f0, f30
	ctx.f[0].u64 = ctx.f[30].u64 & !0x8000_0000_0000_0000u64;
	// 82264214: FF1E6800  fcmpu cr6, f30, f13
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[13].f64);
	// 82264218: 409A0008  bne cr6, 0x82264220
	if !ctx.cr[6].eq {
	pc = 0x82264220; continue 'dispatch;
	}
	// 8226421C: 480000BC  b 0x822642d8
	pc = 0x822642D8; continue 'dispatch;
            }
            0x82264220 => {
    //   block [0x82264220..0x8226424C)
	// 82264220: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82264224: 396B9D58  addi r11, r11, -0x62a8
	ctx.r[11].s64 = ctx.r[11].s64 + -25256;
	// 82264228: C9ABFFE0  lfd f13, -0x20(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-32 as u32) ) };
	// 8226422C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82264230: 40990050  ble cr6, 0x82264280
	if !ctx.cr[6].gt {
	pc = 0x82264280; continue 'dispatch;
	}
	// 82264234: C9ABFFD8  lfd f13, -0x28(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-40 as u32) ) };
	// 82264238: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8226423C: 40990010  ble cr6, 0x8226424c
	if !ctx.cr[6].gt {
	pc = 0x8226424C; continue 'dispatch;
	}
	// 82264240: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82264244: C80B0718  lfd f0, 0x718(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(1816 as u32) ) };
	// 82264248: 48000088  b 0x822642d0
	pc = 0x822642D0; continue 'dispatch;
            }
            0x8226424C => {
    //   block [0x8226424C..0x82264280)
	// 8226424C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82264250: CBEB9390  lfd f31, -0x6c70(r11)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27760 as u32) ) };
	// 82264254: FC2007F2  fmul f1, f0, f31
	ctx.f[1].f64 = ctx.f[0].f64 * ctx.f[31].f64;
	// 82264258: 48000729  bl 0x82264980
	ctx.lr = 0x8226425C;
	sub_82264980(ctx, base);
	// 8226425C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82264260: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82264264: C80B0718  lfd f0, 0x718(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(1816 as u32) ) };
	// 82264268: FD81002A  fadd f12, f1, f0
	ctx.f[12].f64 = ctx.f[1].f64 + ctx.f[0].f64;
	// 8226426C: C9AA1070  lfd f13, 0x1070(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(4208 as u32) ) };
	// 82264270: FC006024  fdiv f0, f0, f12
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[12].f64;
	// 82264274: FC0D0028  fsub f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 - ctx.f[0].f64;
	// 82264278: FC0007F2  fmul f0, f0, f31
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[31].f64;
	// 8226427C: 48000054  b 0x822642d0
	pc = 0x822642D0; continue 'dispatch;
            }
            0x82264280 => {
    //   block [0x82264280..0x822642D0)
	// 82264280: FCC00032  fmul f6, f0, f0
	ctx.f[6].f64 = ctx.f[0].f64 * ctx.f[0].f64;
	// 82264284: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82264288: C98BFFF0  lfd f12, -0x10(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-16 as u32) ) };
	// 8226428C: 3D208205  lis r9, -0x7dfb
	ctx.r[9].s64 = -2113601536;
	// 82264290: C96B0010  lfd f11, 0x10(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82264294: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82264298: C92B0008  lfd f9, 8(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8226429C: C90B0000  lfd f8, 0(r11)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 822642A0: C9AA9D80  lfd f13, -0x6280(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-25216 as u32) ) };
	// 822642A4: C9499D78  lfd f10, -0x6288(r9)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(-25224 as u32) ) };
	// 822642A8: C8E80718  lfd f7, 0x718(r8)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(1816 as u32) ) };
	// 822642AC: FDA6637C  fnmsub f13, f6, f13, f12
	ctx.f[13].f64 = -(ctx.f[6].f64 * ctx.f[13].f64 - ctx.f[12].f64);
	// 822642B0: FD86582A  fadd f12, f6, f11
	ctx.f[12].f64 = ctx.f[6].f64 + ctx.f[11].f64;
	// 822642B4: FDAD51B8  fmsub f13, f13, f6, f10
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[6].f64 - ctx.f[10].f64;
	// 822642B8: FD8C49BA  fmadd f12, f12, f6, f9
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[6].f64 + ctx.f[9].f64;
	// 822642BC: FDAD01B2  fmul f13, f13, f6
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[6].f64;
	// 822642C0: FD8C41BA  fmadd f12, f12, f6, f8
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[6].f64 + ctx.f[8].f64;
	// 822642C4: FDAD6024  fdiv f13, f13, f12
	ctx.f[13].f64 = ctx.f[13].f64 / ctx.f[12].f64;
	// 822642C8: FDAD382A  fadd f13, f13, f7
	ctx.f[13].f64 = ctx.f[13].f64 + ctx.f[7].f64;
	// 822642CC: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	pc = 0x822642D0; continue 'dispatch;
            }
            0x822642D0 => {
    //   block [0x822642D0..0x822642D8)
	// 822642D0: FDA00050  fneg f13, f0
	ctx.f[13].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 822642D4: FC3E682E  fsel f1, f30, f0, f13
	ctx.f[1].f64 = if ctx.f[30].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	pc = 0x822642D8; continue 'dispatch;
            }
            0x822642D8 => {
    //   block [0x822642D8..0x822642F0)
	// 822642D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822642DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822642E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822642E4: CBC1FFE8  lfd f30, -0x18(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822642E8: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822642EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822642F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822642F0 size=212
    let mut pc: u32 = 0x822642F0;
    'dispatch: loop {
        match pc {
            0x822642F0 => {
    //   block [0x822642F0..0x822643A4)
	// 822642F0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822642F4: C80B06F0  lfd f0, 0x6f0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(1776 as u32) ) };
	// 822642F8: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 822642FC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82264300: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82264304: 3D408228  lis r10, -0x7dd8
	ctx.r[10].s64 = -2111307776;
	// 82264308: 396B9D88  addi r11, r11, -0x6278
	ctx.r[11].s64 = ctx.r[11].s64 + -25208;
	// 8226430C: 392A6C50  addi r9, r10, 0x6c50
	ctx.r[9].s64 = ctx.r[10].s64 + 27728;
	// 82264310: 3D008205  lis r8, -0x7dfb
	ctx.r[8].s64 = -2113601536;
	// 82264314: 3CE08205  lis r7, -0x7dfb
	ctx.r[7].s64 = -2113601536;
	// 82264318: C9AA6C50  lfd f13, 0x6c50(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(27728 as u32) ) };
	// 8226431C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82264320: C80B0000  lfd f0, 0(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82264324: 3CC08205  lis r6, -0x7dfb
	ctx.r[6].s64 = -2113601536;
	// 82264328: FC810032  fmul f4, f1, f0
	ctx.f[4].f64 = ctx.f[1].f64 * ctx.f[0].f64;
	// 8226432C: C9890008  lfd f12, 8(r9)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	// 82264330: C94B0020  lfd f10, 0x20(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82264334: C9689DF8  lfd f11, -0x6208(r8)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(-25096 as u32) ) };
	// 82264338: C92B0050  lfd f9, 0x50(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 8226433C: C9079DF0  lfd f8, -0x6210(r7)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(-25104 as u32) ) };
	// 82264340: C8EA9DE8  lfd f7, -0x6218(r10)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-25112 as u32) ) };
	// 82264344: C8CB0040  lfd f6, 0x40(r11)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 82264348: C80B0030  lfd f0, 0x30(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 8226434C: C8A69DE0  lfd f5, -0x6220(r6)
	ctx.f[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(-25120 as u32) ) };
	// 82264350: FC80265C  fctid f4, f4
	ctx.f[4].s64 = if ctx.f[4].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[4].f64 as i64 };
	// 82264354: D881FFF0  stfd f4, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[4].u64 ) };
	// 82264358: E941FFF0  ld r10, -0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226435C: 794A07E0  clrldi r10, r10, 0x3f
	ctx.r[10].u64 = ctx.r[10].u64 & 0x0000000000000001u64;
	// 82264360: FC80269C  fcfid f4, f4
	ctx.f[4].f64 = (ctx.f[4].s64 as f64);
	// 82264364: 2F2A0000  cmpdi cr6, r10, 0
	ctx.cr[6].compare_i64(ctx.r[10].s64, 0, &mut ctx.xer);
	// 82264368: FDAD093C  fnmsub f13, f13, f4, f1
	ctx.f[13].f64 = -(ctx.f[13].f64 * ctx.f[4].f64 - ctx.f[1].f64);
	// 8226436C: FDAC693C  fnmsub f13, f12, f4, f13
	ctx.f[13].f64 = -(ctx.f[12].f64 * ctx.f[4].f64 - ctx.f[13].f64);
	// 82264370: FD8D0372  fmul f12, f13, f13
	ctx.f[12].f64 = ctx.f[13].f64 * ctx.f[13].f64;
	// 82264374: FD6C52FC  fnmsub f11, f12, f11, f10
	ctx.f[11].f64 = -(ctx.f[12].f64 * ctx.f[11].f64 - ctx.f[10].f64);
	// 82264378: FD4C4278  fmsub f10, f12, f9, f8
	ctx.f[10].f64 = ctx.f[12].f64 * ctx.f[9].f64 - ctx.f[8].f64;
	// 8226437C: FD6B3B38  fmsub f11, f11, f12, f7
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[12].f64 - ctx.f[7].f64;
	// 82264380: FD4A333A  fmadd f10, f10, f12, f6
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[12].f64 + ctx.f[6].f64;
	// 82264384: FD6B033A  fmadd f11, f11, f12, f0
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[12].f64 + ctx.f[0].f64;
	// 82264388: FD4A2B38  fmsub f10, f10, f12, f5
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[12].f64 - ctx.f[5].f64;
	// 8226438C: FDAB0372  fmul f13, f11, f13
	ctx.f[13].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	// 82264390: FC0A033A  fmadd f0, f10, f12, f0
	ctx.f[0].f64 = ctx.f[10].f64 * ctx.f[12].f64 + ctx.f[0].f64;
	// 82264394: 419A0010  beq cr6, 0x822643a4
	if ctx.cr[6].eq {
	pc = 0x822643A4; continue 'dispatch;
	}
	// 82264398: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 8226439C: FC006824  fdiv f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[13].f64;
	// 822643A0: 48000008  b 0x822643a8
	pc = 0x822643A8; continue 'dispatch;
            }
            0x822643A4 => {
    //   block [0x822643A4..0x822643A8)
	// 822643A4: FC0D0024  fdiv f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 / ctx.f[0].f64;
	pc = 0x822643A8; continue 'dispatch;
            }
            0x822643A8 => {
    //   block [0x822643A8..0x822643C4)
	// 822643A8: FD600A10  fabs f11, f1
	ctx.f[11].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 822643AC: C9AB0010  lfd f13, 0x10(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 822643B0: 3D608227  lis r11, -0x7dd9
	ctx.r[11].s64 = -2111373312;
	// 822643B4: C98B1BC0  lfd f12, 0x1bc0(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(7104 as u32) ) };
	// 822643B8: FDAB6828  fsub f13, f11, f13
	ctx.f[13].f64 = ctx.f[11].f64 - ctx.f[13].f64;
	// 822643BC: FC2D032E  fsel f1, f13, f12, f0
	ctx.f[1].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[0].f64 };
	// 822643C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822643C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822643C8 size=616
    let mut pc: u32 = 0x822643C8;
    'dispatch: loop {
        match pc {
            0x822643C8 => {
    //   block [0x822643C8..0x82264440)
	// 822643C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822643CC: 4BE2A6D1  bl 0x8208ea9c
	ctx.lr = 0x822643D0;
	sub_8208EA60(ctx, base);
	// 822643D0: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 822643D4: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 822643D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822643DC: 3D60C007  lis r11, -0x3ff9
	ctx.r[11].s64 = -1073283072;
	// 822643E0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822643E4: 386000F8  li r3, 0xf8
	ctx.r[3].s64 = 248;
	// 822643E8: DBE100A0  stfd f31, 0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.f[31].u64 ) };
	// 822643EC: 617DFEFF  ori r29, r11, 0xfeff
	ctx.r[29].u64 = ctx.r[11].u64 | 65279;
	// 822643F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822643F4: 4BE327DD  bl 0x82096bd0
	ctx.lr = 0x822643F8;
	sub_82096BD0(ctx, base);
	// 822643F8: A16100A0  lhz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 822643FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82264400: 556B0476  rlwinm r11, r11, 0, 0x11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82264404: 2B0B7FF0  cmplwi cr6, r11, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32752 as u32, &mut ctx.xer);
	// 82264408: 409A0060  bne cr6, 0x82264468
	if !ctx.cr[6].eq {
	pc = 0x82264468; continue 'dispatch;
	}
	// 8226440C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82264410: 4BE2E071  bl 0x82092480
	ctx.lr = 0x82264414;
	sub_82092480(ctx, base);
	// 82264414: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82264418: 4081003C  ble 0x82264454
	if !ctx.cr[0].gt {
	pc = 0x82264454; continue 'dispatch;
	}
	// 8226441C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82264420: 40990020  ble cr6, 0x82264440
	if !ctx.cr[6].gt {
	pc = 0x82264440; continue 'dispatch;
	}
	// 82264424: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82264428: 409A002C  bne cr6, 0x82264454
	if !ctx.cr[6].eq {
	pc = 0x82264454; continue 'dispatch;
	}
	// 8226442C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82264430: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82264434: 3860001F  li r3, 0x1f
	ctx.r[3].s64 = 31;
	// 82264438: 4BE324D1  bl 0x82096908
	ctx.lr = 0x8226443C;
	sub_82096908(ctx, base);
	// 8226443C: 480001E4  b 0x82264620
	pc = 0x82264620; continue 'dispatch;
            }
            0x82264440 => {
    //   block [0x82264440..0x82264454)
	// 82264440: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82264444: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82264448: 4BE32789  bl 0x82096bd0
	ctx.lr = 0x8226444C;
	sub_82096BD0(ctx, base);
	// 8226444C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82264450: 480001D0  b 0x82264620
	pc = 0x82264620; continue 'dispatch;
            }
            0x82264454 => {
    //   block [0x82264454..0x82264468)
	// 82264454: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82264458: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 8226445C: C80B0718  lfd f0, 0x718(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(1816 as u32) ) };
	// 82264460: FC5F002A  fadd f2, f31, f0
	ctx.f[2].f64 = ctx.f[31].f64 + ctx.f[0].f64;
	// 82264464: 480001AC  b 0x82264610
	pc = 0x82264610; continue 'dispatch;
            }
            0x82264468 => {
    //   block [0x82264468..0x82264490)
	// 82264468: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8226446C: C80B06F0  lfd f0, 0x6f0(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(1776 as u32) ) };
	// 82264470: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82264474: 419AFFCC  beq cr6, 0x82264440
	if ctx.cr[6].eq {
	pc = 0x82264440; continue 'dispatch;
	}
	// 82264478: FDA0F850  fneg f13, f31
	ctx.f[13].u64 = ctx.f[31].u64 ^ 0x8000_0000_0000_0000u64;
	// 8226447C: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 82264480: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82264484: FC3F6FEE  fsel f1, f31, f31, f13
	ctx.f[1].f64 = if ctx.f[31].f64 >= 0.0 { ctx.f[31].f64 } else { ctx.f[13].f64 };
	// 82264488: 41980008  blt cr6, 0x82264490
	if ctx.cr[6].lt {
	pc = 0x82264490; continue 'dispatch;
	}
	// 8226448C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	pc = 0x82264490; continue 'dispatch;
            }
            0x82264490 => {
    //   block [0x82264490..0x822644D4)
	// 82264490: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82264494: CBCB0718  lfd f30, 0x718(r11)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(1816 as u32) ) };
	// 82264498: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8226449C: FF01F000  fcmpu cr6, f1, f30
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[30].f64);
	// 822644A0: 396B9E00  addi r11, r11, -0x6200
	ctx.r[11].s64 = ctx.r[11].s64 + -25088;
	// 822644A4: 409900A0  ble cr6, 0x82264544
	if !ctx.cr[6].gt {
	pc = 0x82264544; continue 'dispatch;
	}
	// 822644A8: C80B0008  lfd f0, 8(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 822644AC: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 822644B0: 40990060  ble cr6, 0x82264510
	if !ctx.cr[6].gt {
	pc = 0x82264510; continue 'dispatch;
	}
	// 822644B4: C80B0010  lfd f0, 0x10(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 822644B8: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 822644BC: 40990018  ble cr6, 0x822644d4
	if !ctx.cr[6].gt {
	pc = 0x822644D4; continue 'dispatch;
	}
	// 822644C0: 3D608227  lis r11, -0x7dd9
	ctx.r[11].s64 = -2111373312;
	// 822644C4: 38600011  li r3, 0x11
	ctx.r[3].s64 = 17;
	// 822644C8: C80B1BB8  lfd f0, 0x1bb8(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(7096 as u32) ) };
	// 822644CC: FC4007F2  fmul f2, f0, f31
	ctx.f[2].f64 = ctx.f[0].f64 * ctx.f[31].f64;
	// 822644D0: 48000140  b 0x82264610
	pc = 0x82264610; continue 'dispatch;
            }
            0x822644D4 => {
    //   block [0x822644D4..0x82264504)
	// 822644D4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822644D8: 48000589  bl 0x82264a60
	ctx.lr = 0x822644DC;
	sub_82264A60(ctx, base);
	// 822644DC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822644E0: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 822644E4: 90810050  stw r4, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 822644E8: 2F040400  cmpwi cr6, r4, 0x400
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1024, &mut ctx.xer);
	// 822644EC: 40990018  ble cr6, 0x82264504
	if !ctx.cr[6].gt {
	pc = 0x82264504; continue 'dispatch;
	}
	// 822644F0: 3884FA00  addi r4, r4, -0x600
	ctx.r[4].s64 = ctx.r[4].s64 + -1536;
	// 822644F4: 4BE2DF15  bl 0x82092408
	ctx.lr = 0x822644F8;
	sub_82092408(ctx, base);
	// 822644F8: FC400890  fmr f2, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[1].f64;
	// 822644FC: 38600011  li r3, 0x11
	ctx.r[3].s64 = 17;
	// 82264500: 48000110  b 0x82264610
	pc = 0x82264610; continue 'dispatch;
            }
            0x82264504 => {
    //   block [0x82264504..0x82264510)
	// 82264504: 4BE2DF05  bl 0x82092408
	ctx.lr = 0x82264508;
	sub_82092408(ctx, base);
	// 82264508: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8226450C: 48000028  b 0x82264534
	pc = 0x82264534; continue 'dispatch;
            }
            0x82264510 => {
    //   block [0x82264510..0x82264534)
	// 82264510: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82264514: 4800054D  bl 0x82264a60
	ctx.lr = 0x82264518;
	sub_82264A60(ctx, base);
	// 82264518: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8226451C: 4BE2DEED  bl 0x82092408
	ctx.lr = 0x82264520;
	sub_82092408(ctx, base);
	// 82264520: FDBE0824  fdiv f13, f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = ctx.f[30].f64 / ctx.f[1].f64;
	// 82264524: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82264528: C80B1070  lfd f0, 0x1070(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(4208 as u32) ) };
	// 8226452C: FDA16828  fsub f13, f1, f13
	ctx.f[13].f64 = ctx.f[1].f64 - ctx.f[13].f64;
	// 82264530: FFCD0032  fmul f30, f13, f0
	ctx.f[30].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	pc = 0x82264534; continue 'dispatch;
            }
            0x82264534 => {
    //   block [0x82264534..0x82264544)
	// 82264534: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82264538: 409800B4  bge cr6, 0x822645ec
	if !ctx.cr[6].lt {
	pc = 0x822645EC; continue 'dispatch;
	}
	// 8226453C: FFC0F050  fneg f30, f30
	ctx.f[30].u64 = ctx.f[30].u64 ^ 0x8000_0000_0000_0000u64;
	// 82264540: 480000AC  b 0x822645ec
	pc = 0x822645EC; continue 'dispatch;
            }
            0x82264544 => {
    //   block [0x82264544..0x8226457C)
	// 82264544: C80B0000  lfd f0, 0(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82264548: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8226454C: 40980048  bge cr6, 0x82264594
	if !ctx.cr[6].lt {
	pc = 0x82264594; continue 'dispatch;
	}
	// 82264550: FFC0F890  fmr f30, f31
	ctx.f[30].f64 = ctx.f[31].f64;
	// 82264554: DBC10058  stfd f30, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[30].u64 ) };
	// 82264558: A1610058  lhz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8226455C: 556B0477  rlwinm. r11, r11, 0, 0x11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82264560: 4082008C  bne 0x822645ec
	if !ctx.cr[0].eq {
	pc = 0x822645EC; continue 'dispatch;
	}
	// 82264564: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82264568: 556B033F  clrlwi. r11, r11, 0xc
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226456C: 40820010  bne 0x8226457c
	if !ctx.cr[0].eq {
	pc = 0x8226457C; continue 'dispatch;
	}
	// 82264570: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82264574: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82264578: 419A0074  beq cr6, 0x822645ec
	if ctx.cr[6].eq {
	pc = 0x822645EC; continue 'dispatch;
	}
	pc = 0x8226457C; continue 'dispatch;
            }
            0x8226457C => {
    //   block [0x8226457C..0x82264594)
	// 8226457C: 38800600  li r4, 0x600
	ctx.r[4].s64 = 1536;
	// 82264580: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82264584: 4BE2DEC5  bl 0x82092448
	ctx.lr = 0x82264588;
	sub_82092448(ctx, base);
	// 82264588: 38600012  li r3, 0x12
	ctx.r[3].s64 = 18;
	// 8226458C: FC400890  fmr f2, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[1].f64;
	// 82264590: 48000080  b 0x82264610
	pc = 0x82264610; continue 'dispatch;
            }
            0x82264594 => {
    //   block [0x82264594..0x822645EC)
	// 82264594: FCFF07F2  fmul f7, f31, f31
	ctx.f[7].f64 = ctx.f[31].f64 * ctx.f[31].f64;
	// 82264598: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8226459C: 3D208205  lis r9, -0x7dfb
	ctx.r[9].s64 = -2113601536;
	// 822645A0: C9AB0028  lfd f13, 0x28(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 822645A4: 3D008205  lis r8, -0x7dfb
	ctx.r[8].s64 = -2113601536;
	// 822645A8: C94B0040  lfd f10, 0x40(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 822645AC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 822645B0: 3CE08205  lis r7, -0x7dfb
	ctx.r[7].s64 = -2113601536;
	// 822645B4: C80A9E70  lfd f0, -0x6190(r10)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-24976 as u32) ) };
	// 822645B8: C9899E68  lfd f12, -0x6198(r9)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(-24984 as u32) ) };
	// 822645BC: C9689E60  lfd f11, -0x61a0(r8)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(-24992 as u32) ) };
	// 822645C0: C92B9E58  lfd f9, -0x61a8(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-25000 as u32) ) };
	// 822645C4: C9079E50  lfd f8, -0x61b0(r7)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(-25008 as u32) ) };
	// 822645C8: FC07683C  fnmsub f0, f7, f0, f13
	ctx.f[0].f64 = -(ctx.f[7].f64 * ctx.f[0].f64 - ctx.f[13].f64);
	// 822645CC: FDA76028  fsub f13, f7, f12
	ctx.f[13].f64 = ctx.f[7].f64 - ctx.f[12].f64;
	// 822645D0: FC0059F8  fmsub f0, f0, f7, f11
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[7].f64 - ctx.f[11].f64;
	// 822645D4: FDAD51FA  fmadd f13, f13, f7, f10
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[7].f64 + ctx.f[10].f64;
	// 822645D8: FC0049F8  fmsub f0, f0, f7, f9
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[7].f64 - ctx.f[9].f64;
	// 822645DC: FDAD41F8  fmsub f13, f13, f7, f8
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[7].f64 - ctx.f[8].f64;
	// 822645E0: FC006824  fdiv f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[13].f64;
	// 822645E4: FC00F1FA  fmadd f0, f0, f7, f30
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[7].f64 + ctx.f[30].f64;
	// 822645E8: FFC007F2  fmul f30, f0, f31
	ctx.f[30].f64 = ctx.f[0].f64 * ctx.f[31].f64;
	pc = 0x822645EC; continue 'dispatch;
            }
            0x822645EC => {
    //   block [0x822645EC..0x82264608)
	// 822645EC: 57EB0739  rlwinm. r11, r31, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822645F0: 41820018  beq 0x82264608
	if ctx.cr[0].eq {
	pc = 0x82264608; continue 'dispatch;
	}
	// 822645F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822645F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822645FC: 4BE325D5  bl 0x82096bd0
	ctx.lr = 0x82264600;
	sub_82096BD0(ctx, base);
	// 82264600: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82264604: 4800001C  b 0x82264620
	pc = 0x82264620; continue 'dispatch;
            }
            0x82264608 => {
    //   block [0x82264608..0x82264610)
	// 82264608: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226460C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	pc = 0x82264610; continue 'dispatch;
            }
            0x82264610 => {
    //   block [0x82264610..0x82264620)
	// 82264610: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82264614: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82264618: 3880001F  li r4, 0x1f
	ctx.r[4].s64 = 31;
	// 8226461C: 4BE323F5  bl 0x82096a10
	ctx.lr = 0x82264620;
	sub_82096A10(ctx, base);
	pc = 0x82264620; continue 'dispatch;
            }
            0x82264620 => {
    //   block [0x82264620..0x82264630)
	// 82264620: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82264624: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82264628: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8226462C: 4BE2A4C0  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82264630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82264630 size=456
    let mut pc: u32 = 0x82264630;
    'dispatch: loop {
        match pc {
            0x82264630 => {
    //   block [0x82264630..0x822646B0)
	// 82264630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82264634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82264638: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8226463C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82264640: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82264644: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82264648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226464C: 3D60C007  lis r11, -0x3ff9
	ctx.r[11].s64 = -1073283072;
	// 82264650: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82264654: 386000F8  li r3, 0xf8
	ctx.r[3].s64 = 248;
	// 82264658: DBE10090  stfd f31, 0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.f[31].u64 ) };
	// 8226465C: 617EFEFF  ori r30, r11, 0xfeff
	ctx.r[30].u64 = ctx.r[11].u64 | 65279;
	// 82264660: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82264664: 4BE3256D  bl 0x82096bd0
	ctx.lr = 0x82264668;
	sub_82096BD0(ctx, base);
	// 82264668: A1610090  lhz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 8226466C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82264670: 556B0476  rlwinm r11, r11, 0, 0x11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82264674: 2B0B7FF0  cmplwi cr6, r11, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32752 as u32, &mut ctx.xer);
	// 82264678: 409A0064  bne cr6, 0x822646dc
	if !ctx.cr[6].eq {
	pc = 0x822646DC; continue 'dispatch;
	}
	// 8226467C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82264680: 4BE2DE01  bl 0x82092480
	ctx.lr = 0x82264684;
	sub_82092480(ctx, base);
	// 82264684: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82264688: 40810040  ble 0x822646c8
	if !ctx.cr[0].gt {
	pc = 0x822646C8; continue 'dispatch;
	}
	// 8226468C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82264690: 40990020  ble cr6, 0x822646b0
	if !ctx.cr[6].gt {
	pc = 0x822646B0; continue 'dispatch;
	}
	// 82264694: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82264698: 409A0030  bne cr6, 0x822646c8
	if !ctx.cr[6].eq {
	pc = 0x822646C8; continue 'dispatch;
	}
	// 8226469C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822646A0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 822646A4: 38600013  li r3, 0x13
	ctx.r[3].s64 = 19;
	// 822646A8: 4BE32261  bl 0x82096908
	ctx.lr = 0x822646AC;
	sub_82096908(ctx, base);
	// 822646AC: 4800012C  b 0x822647d8
	pc = 0x822647D8; continue 'dispatch;
            }
            0x822646B0 => {
    //   block [0x822646B0..0x822646C8)
	// 822646B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822646B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822646B8: 4BE32519  bl 0x82096bd0
	ctx.lr = 0x822646BC;
	sub_82096BD0(ctx, base);
	// 822646BC: 3D608227  lis r11, -0x7dd9
	ctx.r[11].s64 = -2111373312;
	// 822646C0: C82B1BB8  lfd f1, 0x1bb8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(7096 as u32) ) };
	// 822646C4: 48000114  b 0x822647d8
	pc = 0x822647D8; continue 'dispatch;
            }
            0x822646C8 => {
    //   block [0x822646C8..0x822646DC)
	// 822646C8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822646CC: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 822646D0: C80B0718  lfd f0, 0x718(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(1816 as u32) ) };
	// 822646D4: FC5F002A  fadd f2, f31, f0
	ctx.f[2].f64 = ctx.f[31].f64 + ctx.f[0].f64;
	// 822646D8: 480000F0  b 0x822647c8
	pc = 0x822647C8; continue 'dispatch;
            }
            0x822646DC => {
    //   block [0x822646DC..0x82264704)
	// 822646DC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822646E0: C80B06F0  lfd f0, 0x6f0(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(1776 as u32) ) };
	// 822646E4: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 822646E8: 409A001C  bne cr6, 0x82264704
	if !ctx.cr[6].eq {
	pc = 0x82264704; continue 'dispatch;
	}
	// 822646EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822646F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822646F4: 4BE324DD  bl 0x82096bd0
	ctx.lr = 0x822646F8;
	sub_82096BD0(ctx, base);
	// 822646F8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822646FC: C82B0718  lfd f1, 0x718(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(1816 as u32) ) };
	// 82264700: 480000D8  b 0x822647d8
	pc = 0x822647D8; continue 'dispatch;
            }
            0x82264704 => {
    //   block [0x82264704..0x8226473C)
	// 82264704: FDA0F850  fneg f13, f31
	ctx.f[13].u64 = ctx.f[31].u64 ^ 0x8000_0000_0000_0000u64;
	// 82264708: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8226470C: 396B9E10  addi r11, r11, -0x61f0
	ctx.r[11].s64 = ctx.r[11].s64 + -25072;
	// 82264710: C80BFFF8  lfd f0, -8(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	// 82264714: FC3F6FEE  fsel f1, f31, f31, f13
	ctx.f[1].f64 = if ctx.f[31].f64 >= 0.0 { ctx.f[31].f64 } else { ctx.f[13].f64 };
	// 82264718: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8226471C: 4099005C  ble cr6, 0x82264778
	if !ctx.cr[6].gt {
	pc = 0x82264778; continue 'dispatch;
	}
	// 82264720: C80B0000  lfd f0, 0(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82264724: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82264728: 40990014  ble cr6, 0x8226473c
	if !ctx.cr[6].gt {
	pc = 0x8226473C; continue 'dispatch;
	}
	// 8226472C: 3D608227  lis r11, -0x7dd9
	ctx.r[11].s64 = -2111373312;
	// 82264730: 38600011  li r3, 0x11
	ctx.r[3].s64 = 17;
	// 82264734: C84B1BB8  lfd f2, 0x1bb8(r11)
	ctx.f[2].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(7096 as u32) ) };
	// 82264738: 48000090  b 0x822647c8
	pc = 0x822647C8; continue 'dispatch;
            }
            0x8226473C => {
    //   block [0x8226473C..0x8226476C)
	// 8226473C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82264740: 48000321  bl 0x82264a60
	ctx.lr = 0x82264744;
	sub_82264A60(ctx, base);
	// 82264744: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82264748: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 8226474C: 90810050  stw r4, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82264750: 2F040400  cmpwi cr6, r4, 0x400
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1024, &mut ctx.xer);
	// 82264754: 40990018  ble cr6, 0x8226476c
	if !ctx.cr[6].gt {
	pc = 0x8226476C; continue 'dispatch;
	}
	// 82264758: 3884FA00  addi r4, r4, -0x600
	ctx.r[4].s64 = ctx.r[4].s64 + -1536;
	// 8226475C: 4BE2DCAD  bl 0x82092408
	ctx.lr = 0x82264760;
	sub_82092408(ctx, base);
	// 82264760: FC400890  fmr f2, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[1].f64;
	// 82264764: 38600011  li r3, 0x11
	ctx.r[3].s64 = 17;
	// 82264768: 48000060  b 0x822647c8
	pc = 0x822647C8; continue 'dispatch;
            }
            0x8226476C => {
    //   block [0x8226476C..0x82264778)
	// 8226476C: 4BE2DC9D  bl 0x82092408
	ctx.lr = 0x82264770;
	sub_82092408(ctx, base);
	// 82264770: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82264774: 48000030  b 0x822647a4
	pc = 0x822647A4; continue 'dispatch;
            }
            0x82264778 => {
    //   block [0x82264778..0x822647A4)
	// 82264778: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8226477C: 480002E5  bl 0x82264a60
	ctx.lr = 0x82264780;
	sub_82264A60(ctx, base);
	// 82264780: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82264784: 4BE2DC85  bl 0x82092408
	ctx.lr = 0x82264788;
	sub_82092408(ctx, base);
	// 82264788: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8226478C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82264790: C80B0718  lfd f0, 0x718(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(1816 as u32) ) };
	// 82264794: FC000824  fdiv f0, f0, f1
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[1].f64;
	// 82264798: C9AA1070  lfd f13, 0x1070(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(4208 as u32) ) };
	// 8226479C: FC00082A  fadd f0, f0, f1
	ctx.f[0].f64 = ctx.f[0].f64 + ctx.f[1].f64;
	// 822647A0: FFC00372  fmul f30, f0, f13
	ctx.f[30].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	pc = 0x822647A4; continue 'dispatch;
            }
            0x822647A4 => {
    //   block [0x822647A4..0x822647C0)
	// 822647A4: 57EB0739  rlwinm. r11, r31, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822647A8: 41820018  beq 0x822647c0
	if ctx.cr[0].eq {
	pc = 0x822647C0; continue 'dispatch;
	}
	// 822647AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822647B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822647B4: 4BE3241D  bl 0x82096bd0
	ctx.lr = 0x822647B8;
	sub_82096BD0(ctx, base);
	// 822647B8: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 822647BC: 4800001C  b 0x822647d8
	pc = 0x822647D8; continue 'dispatch;
            }
            0x822647C0 => {
    //   block [0x822647C0..0x822647C8)
	// 822647C0: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 822647C4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	pc = 0x822647C8; continue 'dispatch;
            }
            0x822647C8 => {
    //   block [0x822647C8..0x822647D8)
	// 822647C8: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 822647CC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 822647D0: 38800013  li r4, 0x13
	ctx.r[4].s64 = 19;
	// 822647D4: 4BE3223D  bl 0x82096a10
	ctx.lr = 0x822647D8;
	sub_82096A10(ctx, base);
	pc = 0x822647D8; continue 'dispatch;
            }
            0x822647D8 => {
    //   block [0x822647D8..0x822647F8)
	// 822647D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822647DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822647E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822647E4: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 822647E8: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 822647EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822647F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822647F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822647F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822647F8 size=36
    let mut pc: u32 = 0x822647F8;
    'dispatch: loop {
        match pc {
            0x822647F8 => {
    //   block [0x822647F8..0x8226481C)
	// 822647F8: D8210010  stfd f1, 0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.f[1].u64 ) };
	// 822647FC: A1610010  lhz r11, 0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 82264800: 556B0476  rlwinm r11, r11, 0, 0x11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82264804: D8410018  stfd f2, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[2].u64 ) };
	// 82264808: 2B0B7FF0  cmplwi cr6, r11, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32752 as u32, &mut ctx.xer);
	// 8226480C: 409A0010  bne cr6, 0x8226481c
	if !ctx.cr[6].eq {
		sub_8226481C(ctx, base);
		return;
	}
	// 82264810: 3D608227  lis r11, -0x7dd9
	ctx.r[11].s64 = -2111373312;
	// 82264814: C82B1BC0  lfd f1, 0x1bc0(r11)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(7104 as u32) ) };
	// 82264818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226481C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226481C size=68
    let mut pc: u32 = 0x8226481C;
    'dispatch: loop {
        match pc {
            0x8226481C => {
    //   block [0x8226481C..0x82264848)
	// 8226481C: A1610018  lhz r11, 0x18(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) } as u64;
	// 82264820: 556B0476  rlwinm r11, r11, 0, 0x11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82264824: 2B0B7FF0  cmplwi cr6, r11, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32752 as u32, &mut ctx.xer);
	// 82264828: 409A0038  bne cr6, 0x82264860
	if !ctx.cr[6].eq {
		sub_82264860(ctx, base);
		return;
	}
	// 8226482C: 81610018  lwz r11, 0x18(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) } as u64;
	// 82264830: 3D207FF0  lis r9, 0x7ff0
	ctx.r[9].s64 = 2146435072;
	// 82264834: 8141001C  lwz r10, 0x1c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 82264838: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8226483C: 409A000C  bne cr6, 0x82264848
	if !ctx.cr[6].eq {
	pc = 0x82264848; continue 'dispatch;
	}
	// 82264840: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82264844: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	pc = 0x82264848; continue 'dispatch;
            }
            0x82264848 => {
    //   block [0x82264848..0x82264860)
	// 82264848: 3D20FFF0  lis r9, -0x10
	ctx.r[9].s64 = -1048576;
	// 8226484C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82264850: 409AFFC0  bne cr6, 0x82264810
	if !ctx.cr[6].eq {
		sub_822647F8(ctx, base);
		return;
	}
	// 82264854: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82264858: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8226485C: 4BFFFFB4  b 0x82264810
	sub_822647F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82264860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82264860 size=284
    let mut pc: u32 = 0x82264860;
    'dispatch: loop {
        match pc {
            0x82264860 => {
    //   block [0x82264860..0x822648BC)
	// 82264860: E9410018  ld r10, 0x18(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 82264864: E9210010  ld r9, 0x10(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) };
	// 82264868: 794B0040  clrldi r11, r10, 1
	ctx.r[11].u64 = ctx.r[10].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 8226486C: 79280040  clrldi r8, r9, 1
	ctx.r[8].u64 = ctx.r[9].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 82264870: 7F285840  cmpld cr6, r8, r11
	ctx.cr[6].compare_u64(ctx.r[8].u64, ctx.r[11].u64, &mut ctx.xer);
	// 82264874: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
	// 82264878: 79486560  rldicl r8, r10, 0xc, 0x35
	ctx.r[8].u64 = ctx.r[10].u64 & 0x000FFFFFFFFFFFFFu64;
	// 8226487C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82264880: 79250004  rldicr r5, r9, 0, 0
	ctx.r[5].u64 = (ctx.r[9].u64).rotate_left(0) & 0x8000000000000000;
	// 82264884: 794A5840  rldicl r10, r10, 0xb, 1
	ctx.r[10].u64 = ctx.r[10].u64 & 0x001FFFFFFFFFFFFFu64;
	// 82264888: 7967FFE6  rldicr r7, r11, 0x3f, 0x3f
	ctx.r[7].u64 = (ctx.r[11].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 8226488C: F8A1FFF0  std r5, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[5].u64 ) };
	// 82264890: 792B6560  rldicl r11, r9, 0xc, 0x35
	ctx.r[11].u64 = ctx.r[9].u64 & 0x000FFFFFFFFFFFFFu64;
	// 82264894: 794A0524  rldicr r10, r10, 0, 0x34
	ctx.r[10].u64 = (ctx.r[10].u64).rotate_left(0) & 0xFFFFFFFFFFFFF800;
	// 82264898: 2F280000  cmpdi cr6, r8, 0
	ctx.cr[6].compare_i64(ctx.r[8].s64, 0, &mut ctx.xer);
	// 8226489C: 409A0020  bne cr6, 0x822648bc
	if !ctx.cr[6].eq {
	pc = 0x822648BC; continue 'dispatch;
	}
	// 822648A0: 2B2A0000  cmpldi cr6, r10, 0
	ctx.cr[6].compare_u64(ctx.r[10].u64, 0, &mut ctx.xer);
	// 822648A4: 419AFF6C  beq cr6, 0x82264810
	if ctx.cr[6].eq {
		sub_822647F8(ctx, base);
		return;
	}
	// 822648A8: 7D480074  cntlzd r8, r10
	ctx.r[8].u64 = if ctx.r[10].u64 == 0 { 64 } else { ctx.r[10].u64.leading_zeros() as u64 };
	// 822648AC: 7D0807B4  extsw r8, r8
	ctx.r[8].s64 = ctx.r[8].s32 as i64;
	// 822648B0: 7D464036  sld r6, r10, r8
	if (ctx.r[8].u8 & 0x40) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = (ctx.r[10].u64) << ((ctx.r[8].u8 & 0x3F) as u32);
	}
	// 822648B4: 21080001  subfic r8, r8, 1
	ctx.xer.ca = ctx.r[8].u32 <= 1 as u32;
	ctx.r[8].s64 = (1 as i64) - ctx.r[8].s64;
	// 822648B8: 48000008  b 0x822648c0
	pc = 0x822648C0; continue 'dispatch;
            }
            0x822648BC => {
    //   block [0x822648BC..0x822648C0)
	// 822648BC: 7D463B78  or r6, r10, r7
	ctx.r[6].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	pc = 0x822648C0; continue 'dispatch;
            }
            0x822648C0 => {
    //   block [0x822648C0..0x822648EC)
	// 822648C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822648C4: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 822648C8: 792A584C  rldimi r10, r9, 0xb, 1
	ctx.r[10].u64 = ((ctx.r[9].u64).rotate_left(11) & 0x7FFFFFFFFFFFF800) | (ctx.r[10].u64 & 0x80000000000007FF);
	// 822648CC: 409A0020  bne cr6, 0x822648ec
	if !ctx.cr[6].eq {
	pc = 0x822648EC; continue 'dispatch;
	}
	// 822648D0: 2B2A0000  cmpldi cr6, r10, 0
	ctx.cr[6].compare_u64(ctx.r[10].u64, 0, &mut ctx.xer);
	// 822648D4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 822648D8: 7D4B0074  cntlzd r11, r10
	ctx.r[11].u64 = if ctx.r[10].u64 == 0 { 64 } else { ctx.r[10].u64.leading_zeros() as u64 };
	// 822648DC: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 822648E0: 7D4A5836  sld r10, r10, r11
	if (ctx.r[11].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[10].u64) << ((ctx.r[11].u8 & 0x3F) as u32);
	}
	// 822648E4: 216B0001  subfic r11, r11, 1
	ctx.xer.ca = ctx.r[11].u32 <= 1 as u32;
	ctx.r[11].s64 = (1 as i64) - ctx.r[11].s64;
	// 822648E8: 48000008  b 0x822648f0
	pc = 0x822648F0; continue 'dispatch;
            }
            0x822648EC => {
    //   block [0x822648EC..0x822648F0)
	// 822648EC: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	pc = 0x822648F0; continue 'dispatch;
            }
            0x822648F0 => {
    //   block [0x822648F0..0x822648F8)
	// 822648F0: 78C7F842  rldicl r7, r6, 0x3f, 1
	ctx.r[7].u64 = ctx.r[6].u64 & 0x0000000000000001u64;
	// 822648F4: 48000028  b 0x8226491c
	pc = 0x8226491C; continue 'dispatch;
            }
            0x822648F8 => {
    //   block [0x822648F8..0x82264908)
	// 822648F8: 7F2A3040  cmpld cr6, r10, r6
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[6].u64, &mut ctx.xer);
	// 822648FC: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 82264900: 40980008  bge cr6, 0x82264908
	if !ctx.cr[6].lt {
	pc = 0x82264908; continue 'dispatch;
	}
	// 82264904: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	pc = 0x82264908; continue 'dispatch;
            }
            0x82264908 => {
    //   block [0x82264908..0x8226491C)
	// 82264908: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 8226490C: 7D490074  cntlzd r9, r10
	ctx.r[9].u64 = if ctx.r[10].u64 == 0 { 64 } else { ctx.r[10].u64.leading_zeros() as u64 };
	// 82264910: 7D2907B4  extsw r9, r9
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 82264914: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82264918: 7D4A4836  sld r10, r10, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[10].u64) << ((ctx.r[9].u8 & 0x3F) as u32);
	}
	pc = 0x8226491C; continue 'dispatch;
            }
            0x8226491C => {
    //   block [0x8226491C..0x82264944)
	// 8226491C: 7F2B4000  cmpd cr6, r11, r8
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[8].s64, &mut ctx.xer);
	// 82264920: 4199FFD8  bgt cr6, 0x822648f8
	if ctx.cr[6].gt {
	pc = 0x822648F8; continue 'dispatch;
	}
	// 82264924: 409A0020  bne cr6, 0x82264944
	if !ctx.cr[6].eq {
	pc = 0x82264944; continue 'dispatch;
	}
	// 82264928: 7F2A3040  cmpld cr6, r10, r6
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[6].u64, &mut ctx.xer);
	// 8226492C: 41980018  blt cr6, 0x82264944
	if ctx.cr[6].lt {
	pc = 0x82264944; continue 'dispatch;
	}
	// 82264930: 7D465050  subf r10, r6, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[6].s64;
	// 82264934: 7D490074  cntlzd r9, r10
	ctx.r[9].u64 = if ctx.r[10].u64 == 0 { 64 } else { ctx.r[10].u64.leading_zeros() as u64 };
	// 82264938: 7D2907B4  extsw r9, r9
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 8226493C: 7D4A4836  sld r10, r10, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[10].u64) << ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 82264940: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	pc = 0x82264944; continue 'dispatch;
            }
            0x82264944 => {
    //   block [0x82264944..0x82264960)
	// 82264944: 2B2A0000  cmpldi cr6, r10, 0
	ctx.cr[6].compare_u64(ctx.r[10].u64, 0, &mut ctx.xer);
	// 82264948: 419A002C  beq cr6, 0x82264974
	if ctx.cr[6].eq {
	pc = 0x82264974; continue 'dispatch;
	}
	// 8226494C: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 82264950: 41990010  bgt cr6, 0x82264960
	if ctx.cr[6].gt {
	pc = 0x82264960; continue 'dispatch;
	}
	// 82264954: 212B0001  subfic r9, r11, 1
	ctx.xer.ca = ctx.r[11].u32 <= 1 as u32;
	ctx.r[9].s64 = (1 as i64) - ctx.r[11].s64;
	// 82264958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226495C: 7D4A4C36  srd r10, r10, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[10].u64) >> ((ctx.r[9].u8 & 0x3F) as u32);
	}
	pc = 0x82264960; continue 'dispatch;
            }
            0x82264960 => {
    //   block [0x82264960..0x82264974)
	// 82264960: 796BA2C6  sldi r11, r11, 0x34
	ctx.r[11].u64 = ctx.r[11].u64.wrapping_shl(52);
	ctx.r[11].u32 = ctx.r[11].u64 as u32;
	// 82264964: 794AAB02  rldicl r10, r10, 0x35, 0xc
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000000007FFu64;
	// 82264968: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8226496C: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82264970: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	pc = 0x82264974; continue 'dispatch;
            }
            0x82264974 => {
    //   block [0x82264974..0x8226497C)
	// 82264974: C821FFF0  lfd f1, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82264978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82264980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82264980 size=220
    let mut pc: u32 = 0x82264980;
    'dispatch: loop {
        match pc {
            0x82264980 => {
    //   block [0x82264980..0x82264A38)
	// 82264980: 3D608228  lis r11, -0x7dd8
	ctx.r[11].s64 = -2111307776;
	// 82264984: 396B6C60  addi r11, r11, 0x6c60
	ctx.r[11].s64 = ctx.r[11].s64 + 27744;
	// 82264988: C80B0020  lfd f0, 0x20(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 8226498C: FCA00072  fmul f5, f0, f1
	ctx.f[5].f64 = ctx.f[0].f64 * ctx.f[1].f64;
	// 82264990: C9AB0028  lfd f13, 0x28(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82264994: C98B0030  lfd f12, 0x30(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82264998: C94B0040  lfd f10, 0x40(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 8226499C: C96B0048  lfd f11, 0x48(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) };
	// 822649A0: C92B0060  lfd f9, 0x60(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	// 822649A4: C90B0058  lfd f8, 0x58(r11)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	// 822649A8: C8EB0038  lfd f7, 0x38(r11)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 822649AC: C8CB0050  lfd f6, 0x50(r11)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 822649B0: C00B006C  lfs f0, 0x6c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822649B4: FCA02E5C  fctid f5, f5
	ctx.f[5].s64 = if ctx.f[5].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[5].f64 as i64 };
	// 822649B8: FCA02E9C  fcfid f5, f5
	ctx.f[5].f64 = (ctx.f[5].s64 as f64);
	// 822649BC: FDAD097C  fnmsub f13, f13, f5, f1
	ctx.f[13].f64 = -(ctx.f[13].f64 * ctx.f[5].f64 - ctx.f[1].f64);
	// 822649C0: FC80281E  fctiwz f4, f5
	ctx.f[4].s64 = if ctx.f[5].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[5].f64.trunc() as i32 as i64 };
	// 822649C4: D881FFF0  stfd f4, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[4].u64 ) };
	// 822649C8: 8141FFF4  lwz r10, -0xc(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 822649CC: 354A0001  addic. r10, r10, 1
	ctx.xer.ca = (ctx.r[10].u32 > (!(1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822649D0: FDAC697C  fnmsub f13, f12, f5, f13
	ctx.f[13].f64 = -(ctx.f[12].f64 * ctx.f[5].f64 - ctx.f[13].f64);
	// 822649D4: FD8D0372  fmul f12, f13, f13
	ctx.f[12].f64 = ctx.f[13].f64 * ctx.f[13].f64;
	// 822649D8: FD6B533A  fmadd f11, f11, f12, f10
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[12].f64 + ctx.f[10].f64;
	// 822649DC: FD49433A  fmadd f10, f9, f12, f8
	ctx.f[10].f64 = ctx.f[9].f64 * ctx.f[12].f64 + ctx.f[8].f64;
	// 822649E0: FD6B3B3A  fmadd f11, f11, f12, f7
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[12].f64 + ctx.f[7].f64;
	// 822649E4: FD8A333A  fmadd f12, f10, f12, f6
	ctx.f[12].f64 = ctx.f[10].f64 * ctx.f[12].f64 + ctx.f[6].f64;
	// 822649E8: FDAB0372  fmul f13, f11, f13
	ctx.f[13].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	// 822649EC: FD8C6828  fsub f12, f12, f13
	ctx.f[12].f64 = ctx.f[12].f64 - ctx.f[13].f64;
	// 822649F0: FDAD6024  fdiv f13, f13, f12
	ctx.f[13].f64 = ctx.f[13].f64 / ctx.f[12].f64;
	// 822649F4: FC0D002A  fadd f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	// 822649F8: 41820040  beq 0x82264a38
	if ctx.cr[0].eq {
	pc = 0x82264A38; continue 'dispatch;
	}
	// 822649FC: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 82264A00: A101FFF0  lhz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82264A04: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 82264A08: A121FFF0  lhz r9, -0x10(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82264A0C: 7108800F  andi. r8, r8, 0x800f
	ctx.r[8].u64 = ctx.r[8].u64 & 32783;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82264A10: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 82264A14: 5529E57E  rlwinm r9, r9, 0x1c, 0x15, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000000Fu64;
	// 82264A18: 3929FC02  addi r9, r9, -0x3fe
	ctx.r[9].s64 = ctx.r[9].s64 + -1022;
	// 82264A1C: 7D290734  extsh r9, r9
	ctx.r[9].s64 = ctx.r[9].s16 as i64;
	// 82264A20: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82264A24: 394A03FE  addi r10, r10, 0x3fe
	ctx.r[10].s64 = ctx.r[10].s64 + 1022;
	// 82264A28: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82264A2C: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 82264A30: B141FFF0  sth r10, -0x10(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u16 ) };
	// 82264A34: C801FFF0  lfd f0, -0x10(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	pc = 0x82264A38; continue 'dispatch;
            }
            0x82264A38 => {
    //   block [0x82264A38..0x82264A5C)
	// 82264A38: C9AB0000  lfd f13, 0(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82264A3C: FD616828  fsub f11, f1, f13
	ctx.f[11].f64 = ctx.f[1].f64 - ctx.f[13].f64;
	// 82264A40: C9AB0008  lfd f13, 8(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82264A44: C98B0010  lfd f12, 0x10(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82264A48: FD4D0828  fsub f10, f13, f1
	ctx.f[10].f64 = ctx.f[13].f64 - ctx.f[1].f64;
	// 82264A4C: C1AB0068  lfs f13, 0x68(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82264A50: FC0B032E  fsel f0, f11, f12, f0
	ctx.f[0].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[0].f64 };
	// 82264A54: FC2A036E  fsel f1, f10, f13, f0
	ctx.f[1].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82264A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82264A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82264A60 size=212
    let mut pc: u32 = 0x82264A60;
    'dispatch: loop {
        match pc {
            0x82264A60 => {
    //   block [0x82264A60..0x82264B34)
	// 82264A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82264A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82264A68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82264A6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82264A70: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82264A74: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82264A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82264A7C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82264A80: 3D608228  lis r11, -0x7dd8
	ctx.r[11].s64 = -2111307776;
	// 82264A84: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82264A88: 3BCB6C60  addi r30, r11, 0x6c60
	ctx.r[30].s64 = ctx.r[11].s64 + 27744;
	// 82264A8C: C81E0020  lfd f0, 0x20(r30)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) };
	// 82264A90: FC2007F2  fmul f1, f0, f31
	ctx.f[1].f64 = ctx.f[0].f64 * ctx.f[31].f64;
	// 82264A94: 48001F3D  bl 0x822669d0
	ctx.lr = 0x82264A98;
	sub_822669D0(ctx, base);
	// 82264A98: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82264A9C: C81E0028  lfd f0, 0x28(r30)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) };
	// 82264AA0: C9BE0030  lfd f13, 0x30(r30)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) };
	// 82264AA4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82264AA8: C97E0040  lfd f11, 0x40(r30)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) };
	// 82264AAC: C99E0048  lfd f12, 0x48(r30)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) };
	// 82264AB0: C95E0060  lfd f10, 0x60(r30)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) };
	// 82264AB4: C93E0058  lfd f9, 0x58(r30)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) };
	// 82264AB8: C91E0038  lfd f8, 0x38(r30)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) };
	// 82264ABC: C8FE0050  lfd f7, 0x50(r30)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) };
	// 82264AC0: C8CB1070  lfd f6, 0x1070(r11)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(4208 as u32) ) };
	// 82264AC4: FC00FFBC  fnmsub f0, f0, f30, f31
	ctx.f[0].f64 = -(ctx.f[0].f64 * ctx.f[30].f64 - ctx.f[31].f64);
	// 82264AC8: FC0D07BC  fnmsub f0, f13, f30, f0
	ctx.f[0].f64 = -(ctx.f[13].f64 * ctx.f[30].f64 - ctx.f[0].f64);
	// 82264ACC: FDA00032  fmul f13, f0, f0
	ctx.f[13].f64 = ctx.f[0].f64 * ctx.f[0].f64;
	// 82264AD0: FD8C5B7A  fmadd f12, f12, f13, f11
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[13].f64 + ctx.f[11].f64;
	// 82264AD4: FD6A4B7A  fmadd f11, f10, f13, f9
	ctx.f[11].f64 = ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[9].f64;
	// 82264AD8: FD8C437A  fmadd f12, f12, f13, f8
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[13].f64 + ctx.f[8].f64;
	// 82264ADC: FDAB3B7A  fmadd f13, f11, f13, f7
	ctx.f[13].f64 = ctx.f[11].f64 * ctx.f[13].f64 + ctx.f[7].f64;
	// 82264AE0: FC0C0032  fmul f0, f12, f0
	ctx.f[0].f64 = ctx.f[12].f64 * ctx.f[0].f64;
	// 82264AE4: FDAD0028  fsub f13, f13, f0
	ctx.f[13].f64 = ctx.f[13].f64 - ctx.f[0].f64;
	// 82264AE8: FC006824  fdiv f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[13].f64;
	// 82264AEC: FFE0302A  fadd f31, f0, f6
	ctx.f[31].f64 = ctx.f[0].f64 + ctx.f[6].f64;
	// 82264AF0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82264AF4: 4BE2D93D  bl 0x82092430
	ctx.lr = 0x82264AF8;
	sub_82092430(ctx, base);
	// 82264AF8: FC00F01E  fctiwz f0, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[30].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[30].f64.trunc() as i32 as i64 };
	// 82264AFC: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82264B00: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82264B04: 7D635A14  add r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 82264B08: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82264B0C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82264B10: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82264B14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82264B18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82264B1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82264B20: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82264B24: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82264B28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82264B2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82264B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82264B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82264B38 size=228
    let mut pc: u32 = 0x82264B38;
    'dispatch: loop {
        match pc {
            0x82264B38 => {
    //   block [0x82264B38..0x82264B68)
	// 82264B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82264B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82264B40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82264B44: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82264B48: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82264B4C: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82264B50: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82264B54: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82264B58: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82264B5C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82264B60: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82264B64: 409A0030  bne cr6, 0x82264b94
	if !ctx.cr[6].eq {
	pc = 0x82264B94; continue 'dispatch;
	}
	pc = 0x82264B68; continue 'dispatch;
            }
            0x82264B68 => {
    //   block [0x82264B68..0x82264B94)
	// 82264B68: 4BE2DF11  bl 0x82092a78
	ctx.lr = 0x82264B6C;
	sub_82092A78(ctx, base);
	// 82264B6C: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 82264B70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82264B74: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82264B78: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82264B7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82264B80: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82264B84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82264B88: 4BE2DDB1  bl 0x82092938
	ctx.lr = 0x82264B8C;
	sub_82092938(ctx, base);
	// 82264B8C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82264B90: 48000078  b 0x82264c08
	pc = 0x82264C08; continue 'dispatch;
            }
            0x82264B94 => {
    //   block [0x82264B94..0x82264BF8)
	// 82264B94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82264B98: 419AFFD0  beq cr6, 0x82264b68
	if ctx.cr[6].eq {
	pc = 0x82264B68; continue 'dispatch;
	}
	// 82264B9C: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 82264BA0: 90610068  stw r3, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 82264BA4: 39400042  li r10, 0x42
	ctx.r[10].s64 = 66;
	// 82264BA8: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82264BAC: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82264BB0: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 82264BB4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82264BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82264BBC: 396100B0  addi r11, r1, 0xb0
	ctx.r[11].s64 = ctx.r[1].s64 + 176;
	// 82264BC0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82264BC4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82264BC8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82264BCC: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82264BD0: 4BE2F7D1  bl 0x820943a0
	ctx.lr = 0x82264BD4;
	sub_820943A0(ctx, base);
	// 82264BD4: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82264BD8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82264BDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82264BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82264BE4: 41800014  blt 0x82264bf8
	if ctx.cr[0].lt {
	pc = 0x82264BF8; continue 'dispatch;
	}
	// 82264BE8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82264BEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82264BF0: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82264BF4: 48000010  b 0x82264c04
	pc = 0x82264C04; continue 'dispatch;
            }
            0x82264BF8 => {
    //   block [0x82264BF8..0x82264C04)
	// 82264BF8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82264BFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82264C00: 4BE2DB59  bl 0x82092758
	ctx.lr = 0x82264C04;
	sub_82092758(ctx, base);
	pc = 0x82264C04; continue 'dispatch;
            }
            0x82264C04 => {
    //   block [0x82264C04..0x82264C08)
	// 82264C04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82264C08; continue 'dispatch;
            }
            0x82264C08 => {
    //   block [0x82264C08..0x82264C1C)
	// 82264C08: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82264C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82264C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82264C14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82264C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82264C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82264C20 size=272
    let mut pc: u32 = 0x82264C20;
    'dispatch: loop {
        match pc {
            0x82264C20 => {
    //   block [0x82264C20..0x82264C5C)
	// 82264C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82264C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82264C28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82264C2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82264C30: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82264C34: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82264C38: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82264C3C: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82264C40: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82264C44: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82264C48: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82264C4C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82264C50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82264C54: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82264C58: 409A0030  bne cr6, 0x82264c88
	if !ctx.cr[6].eq {
	pc = 0x82264C88; continue 'dispatch;
	}
	pc = 0x82264C5C; continue 'dispatch;
            }
            0x82264C5C => {
    //   block [0x82264C5C..0x82264C88)
	// 82264C5C: 4BE2DE1D  bl 0x82092a78
	ctx.lr = 0x82264C60;
	sub_82092A78(ctx, base);
	// 82264C60: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 82264C64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82264C68: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82264C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82264C70: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82264C74: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82264C78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82264C7C: 4BE2DCBD  bl 0x82092938
	ctx.lr = 0x82264C80;
	sub_82092938(ctx, base);
	// 82264C80: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82264C84: 48000094  b 0x82264d18
	pc = 0x82264D18; continue 'dispatch;
            }
            0x82264C88 => {
    //   block [0x82264C88..0x82264C98)
	// 82264C88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82264C8C: 419A000C  beq cr6, 0x82264c98
	if ctx.cr[6].eq {
	pc = 0x82264C98; continue 'dispatch;
	}
	// 82264C90: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82264C94: 419AFFC8  beq cr6, 0x82264c5c
	if ctx.cr[6].eq {
	pc = 0x82264C5C; continue 'dispatch;
	}
	pc = 0x82264C98; continue 'dispatch;
            }
            0x82264C98 => {
    //   block [0x82264C98..0x82264CBC)
	// 82264C98: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82264C9C: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 82264CA0: 390100C8  addi r8, r1, 0xc8
	ctx.r[8].s64 = ctx.r[1].s64 + 200;
	// 82264CA4: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82264CA8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82264CAC: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82264CB0: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82264CB4: 41990008  bgt cr6, 0x82264cbc
	if ctx.cr[6].gt {
	pc = 0x82264CBC; continue 'dispatch;
	}
	// 82264CB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	pc = 0x82264CBC; continue 'dispatch;
            }
            0x82264CBC => {
    //   block [0x82264CBC..0x82264D08)
	// 82264CBC: 39600042  li r11, 0x42
	ctx.r[11].s64 = 66;
	// 82264CC0: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82264CC4: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82264CC8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82264CCC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82264CD0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82264CD4: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82264CD8: 4BE2F6C9  bl 0x820943a0
	ctx.lr = 0x82264CDC;
	sub_820943A0(ctx, base);
	// 82264CDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82264CE0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82264CE4: 419A0030  beq cr6, 0x82264d14
	if ctx.cr[6].eq {
	pc = 0x82264D14; continue 'dispatch;
	}
	// 82264CE8: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82264CEC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82264CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82264CF4: 41800014  blt 0x82264d08
	if ctx.cr[0].lt {
	pc = 0x82264D08; continue 'dispatch;
	}
	// 82264CF8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82264CFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82264D00: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82264D04: 48000010  b 0x82264d14
	pc = 0x82264D14; continue 'dispatch;
            }
            0x82264D08 => {
    //   block [0x82264D08..0x82264D14)
	// 82264D08: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82264D0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82264D10: 4BE2DA49  bl 0x82092758
	ctx.lr = 0x82264D14;
	sub_82092758(ctx, base);
	pc = 0x82264D14; continue 'dispatch;
            }
            0x82264D14 => {
    //   block [0x82264D14..0x82264D18)
	// 82264D14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	pc = 0x82264D18; continue 'dispatch;
            }
            0x82264D18 => {
    //   block [0x82264D18..0x82264D30)
	// 82264D18: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82264D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82264D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82264D24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82264D28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82264D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82264D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82264D30 size=788
    let mut pc: u32 = 0x82264D30;
    'dispatch: loop {
        match pc {
            0x82264D30 => {
    //   block [0x82264D30..0x82264D5C)
	// 82264D30: 7C0802A6  mflr r0
	ctx.r[0].u64 = ctx.lr;
	// 82264D34: 9421FFB0  stwu r1, -0x50(r1)
	ea = ctx.r[1].u32.wrapping_add(-80 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82264D38: 90010008  stw r0, 8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(8 as u32), ctx.r[0].u32 ) };
	// 82264D3C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82264D40: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82264D44: 80030138  lwz r0, 0x138(r3)
	ctx.r[0].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(312 as u32) ) } as u64;
	// 82264D48: 2C800000  cmpwi cr1, r0, 0
	ctx.cr[1].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82264D4C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82264D50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82264D54: 40820008  bne 0x82264d5c
	if !ctx.cr[0].eq {
	pc = 0x82264D5C; continue 'dispatch;
	}
	// 82264D58: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	pc = 0x82264D5C; continue 'dispatch;
            }
            0x82264D5C => {
    //   block [0x82264D5C..0x8226501C)
	// 82264D5C: 408602C0  bne cr1, 0x8226501c
	if !ctx.cr[1].eq {
	pc = 0x8226501C; continue 'dispatch;
	}
	// 82264D60: 80670134  lwz r3, 0x134(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(308 as u32) ) } as u64;
	// 82264D64: 80870090  lwz r4, 0x90(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(144 as u32) ) } as u64;
	// 82264D68: 48001CB9  bl 0x82266a20
	ctx.lr = 0x82264D6C;
	sub_82266A20(ctx, base);
	// 82264D6C: C9C70000  lfd f14, 0(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[14].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	// 82264D70: C9E70008  lfd f15, 8(r7)
	ctx.f[15].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) };
	// 82264D74: CA070010  lfd f16, 0x10(r7)
	ctx.f[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(16 as u32) ) };
	// 82264D78: CA270018  lfd f17, 0x18(r7)
	ctx.f[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) };
	// 82264D7C: CA470020  lfd f18, 0x20(r7)
	ctx.f[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(32 as u32) ) };
	// 82264D80: CA670028  lfd f19, 0x28(r7)
	ctx.f[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(40 as u32) ) };
	// 82264D84: CA870030  lfd f20, 0x30(r7)
	ctx.f[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) };
	// 82264D88: CAA70038  lfd f21, 0x38(r7)
	ctx.f[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(56 as u32) ) };
	// 82264D8C: CAC70040  lfd f22, 0x40(r7)
	ctx.f[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(64 as u32) ) };
	// 82264D90: CAE70048  lfd f23, 0x48(r7)
	ctx.f[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(72 as u32) ) };
	// 82264D94: CB070050  lfd f24, 0x50(r7)
	ctx.f[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(80 as u32) ) };
	// 82264D98: CB270058  lfd f25, 0x58(r7)
	ctx.f[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(88 as u32) ) };
	// 82264D9C: CB470060  lfd f26, 0x60(r7)
	ctx.f[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(96 as u32) ) };
	// 82264DA0: CB670068  lfd f27, 0x68(r7)
	ctx.f[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(104 as u32) ) };
	// 82264DA4: CB870070  lfd f28, 0x70(r7)
	ctx.f[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(112 as u32) ) };
	// 82264DA8: CBA70078  lfd f29, 0x78(r7)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(120 as u32) ) };
	// 82264DAC: CBC70080  lfd f30, 0x80(r7)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(128 as u32) ) };
	// 82264DB0: CBE70088  lfd f31, 0x88(r7)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(136 as u32) ) };
	// 82264DB4: E9A70098  ld r13, 0x98(r7)
	ctx.r[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(152 as u32) ) };
	// 82264DB8: E9C700A0  ld r14, 0xa0(r7)
	ctx.r[14].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(160 as u32) ) };
	// 82264DBC: E9E700A8  ld r15, 0xa8(r7)
	ctx.r[15].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(168 as u32) ) };
	// 82264DC0: EA0700B0  ld r16, 0xb0(r7)
	ctx.r[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(176 as u32) ) };
	// 82264DC4: EA2700B8  ld r17, 0xb8(r7)
	ctx.r[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(184 as u32) ) };
	// 82264DC8: EA4700C0  ld r18, 0xc0(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(192 as u32) ) };
	// 82264DCC: EA6700C8  ld r19, 0xc8(r7)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(200 as u32) ) };
	// 82264DD0: EA8700D0  ld r20, 0xd0(r7)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(208 as u32) ) };
	// 82264DD4: EAA700D8  ld r21, 0xd8(r7)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(216 as u32) ) };
	// 82264DD8: EAC700E0  ld r22, 0xe0(r7)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(224 as u32) ) };
	// 82264DDC: EAE700E8  ld r23, 0xe8(r7)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(232 as u32) ) };
	// 82264DE0: EB0700F0  ld r24, 0xf0(r7)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(240 as u32) ) };
	// 82264DE4: EB2700F8  ld r25, 0xf8(r7)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(248 as u32) ) };
	// 82264DE8: EB470100  ld r26, 0x100(r7)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(256 as u32) ) };
	// 82264DEC: EB670108  ld r27, 0x108(r7)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(264 as u32) ) };
	// 82264DF0: EB870110  ld r28, 0x110(r7)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(272 as u32) ) };
	// 82264DF4: EBA70118  ld r29, 0x118(r7)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(280 as u32) ) };
	// 82264DF8: EBC70120  ld r30, 0x120(r7)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(288 as u32) ) };
	// 82264DFC: EBE70128  ld r31, 0x128(r7)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(296 as u32) ) };
	// 82264E00: 38600140  li r3, 0x140
	ctx.r[3].s64 = 320;
	pc = 0x8226501C; continue 'dispatch;
            }
            0x8226501C => {
    //   block [0x8226501C..0x82265044)
	// 8226501C: 80670004  lwz r3, 4(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82265020: 80870000  lwz r4, 0(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265024: 480019FD  bl 0x82266a20
	ctx.lr = 0x82265028;
	sub_82266A20(ctx, base);
	// 82265028: 80670000  lwz r3, 0(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226502C: 80870004  lwz r4, 4(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82265030: 4800577D  bl 0x8226a7ac
	ctx.lr = 0x82265034;
	// extern call 0x8226A7AC → crate::xboxkrnl::RtlUnwind
	crate::xboxkrnl::RtlUnwind(ctx, base);
	// 82265034: 80010008  lwz r0, 8(r1)
	ctx.r[0].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(8 as u32) ) } as u64;
	// 82265038: 7C0803A6  mtlr r0
	ctx.lr = ctx.r[0].u64;
	// 8226503C: 38210050  addi r1, r1, 0x50
	ctx.r[1].s64 = ctx.r[1].s64 + 80;
	// 82265040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82265050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82265050 size=716
    let mut pc: u32 = 0x82265050;
    'dispatch: loop {
        match pc {
            0x82265050 => {
    //   block [0x82265050..0x8226531C)
	// 82265050: 3C808229  lis r4, -0x7dd7
	ctx.r[4].s64 = -2111242240;
	// 82265054: 8004DCE8  lwz r0, -0x2318(r4)
	ctx.r[0].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-8984 as u32) ) } as u64;
	// 82265058: 2C000000  cmpwi r0, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8226505C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82265060: 4C820420  bnectr
	if !ctx.cr[0].eq {
		crate::rt::call_indirect(ctx.ctr.u32);
		return;
	}
	// 82265064: 7C0802A6  mflr r0
	ctx.r[0].u64 = ctx.lr;
	// 82265068: 7C800026  mfcr r4
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[4].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 8226506C: D9C30000  stfd f14, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.f[14].u64 ) };
	// 82265070: D9E30008  stfd f15, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.f[15].u64 ) };
	// 82265074: DA030010  stfd f16, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.f[16].u64 ) };
	// 82265078: DA230018  stfd f17, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.f[17].u64 ) };
	// 8226507C: DA430020  stfd f18, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.f[18].u64 ) };
	// 82265080: DA630028  stfd f19, 0x28(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.f[19].u64 ) };
	// 82265084: DA830030  stfd f20, 0x30(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.f[20].u64 ) };
	// 82265088: DAA30038  stfd f21, 0x38(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.f[21].u64 ) };
	// 8226508C: DAC30040  stfd f22, 0x40(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.f[22].u64 ) };
	// 82265090: DAE30048  stfd f23, 0x48(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.f[23].u64 ) };
	// 82265094: DB030050  stfd f24, 0x50(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.f[24].u64 ) };
	// 82265098: DB230058  stfd f25, 0x58(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.f[25].u64 ) };
	// 8226509C: DB430060  stfd f26, 0x60(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.f[26].u64 ) };
	// 822650A0: DB630068  stfd f27, 0x68(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.f[27].u64 ) };
	// 822650A4: DB830070  stfd f28, 0x70(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), ctx.f[28].u64 ) };
	// 822650A8: DBA30078  stfd f29, 0x78(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), ctx.f[29].u64 ) };
	// 822650AC: DBC30080  stfd f30, 0x80(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.f[30].u64 ) };
	// 822650B0: DBE30088  stfd f31, 0x88(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.f[31].u64 ) };
	// 822650B4: F9A30098  std r13, 0x98(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[13].u64 ) };
	// 822650B8: F9C300A0  std r14, 0xa0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), ctx.r[14].u64 ) };
	// 822650BC: F9E300A8  std r15, 0xa8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), ctx.r[15].u64 ) };
	// 822650C0: FA0300B0  std r16, 0xb0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), ctx.r[16].u64 ) };
	// 822650C4: FA2300B8  std r17, 0xb8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(184 as u32), ctx.r[17].u64 ) };
	// 822650C8: FA4300C0  std r18, 0xc0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(192 as u32), ctx.r[18].u64 ) };
	// 822650CC: FA6300C8  std r19, 0xc8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(200 as u32), ctx.r[19].u64 ) };
	// 822650D0: FA8300D0  std r20, 0xd0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(208 as u32), ctx.r[20].u64 ) };
	// 822650D4: FAA300D8  std r21, 0xd8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(216 as u32), ctx.r[21].u64 ) };
	// 822650D8: FAC300E0  std r22, 0xe0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(224 as u32), ctx.r[22].u64 ) };
	// 822650DC: FAE300E8  std r23, 0xe8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(232 as u32), ctx.r[23].u64 ) };
	// 822650E0: FB0300F0  std r24, 0xf0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(240 as u32), ctx.r[24].u64 ) };
	// 822650E4: FB2300F8  std r25, 0xf8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(248 as u32), ctx.r[25].u64 ) };
	// 822650E8: FB430100  std r26, 0x100(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(256 as u32), ctx.r[26].u64 ) };
	// 822650EC: FB630108  std r27, 0x108(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(264 as u32), ctx.r[27].u64 ) };
	// 822650F0: FB830110  std r28, 0x110(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(272 as u32), ctx.r[28].u64 ) };
	// 822650F4: FBA30118  std r29, 0x118(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(280 as u32), ctx.r[29].u64 ) };
	// 822650F8: FBC30120  std r30, 0x120(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(288 as u32), ctx.r[30].u64 ) };
	// 822650FC: FBE30128  std r31, 0x128(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(296 as u32), ctx.r[31].u64 ) };
	// 82265100: 38A00140  li r5, 0x140
	ctx.r[5].s64 = 320;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82265320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82265320 size=304
    let mut pc: u32 = 0x82265320;
    'dispatch: loop {
        match pc {
            0x82265320 => {
    //   block [0x82265320..0x822653BC)
	// 82265320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82265324: 4BE29775  bl 0x8208ea98
	ctx.lr = 0x82265328;
	sub_8208EA60(ctx, base);
	// 82265328: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8226532C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82265330: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82265334: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82265338: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8226533C: DBE100A0  stfd f31, 0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.f[31].u64 ) };
	// 82265340: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82265344: 4BE3188D  bl 0x82096bd0
	ctx.lr = 0x82265348;
	sub_82096BD0(ctx, base);
	// 82265348: 3D40C007  lis r10, -0x3ff9
	ctx.r[10].s64 = -1073283072;
	// 8226534C: 3D608228  lis r11, -0x7dd8
	ctx.r[11].s64 = -2111307776;
	// 82265350: 615CFEFF  ori r28, r10, 0xfeff
	ctx.r[28].u64 = ctx.r[10].u64 | 65279;
	// 82265354: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82265358: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8226535C: 806B6CD0  lwz r3, 0x6cd0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27856 as u32) ) } as u64;
	// 82265360: 4BE31871  bl 0x82096bd0
	ctx.lr = 0x82265364;
	sub_82096BD0(ctx, base);
	// 82265364: A16100A0  lhz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82265368: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226536C: 556A0476  rlwinm r10, r11, 0, 0x11, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82265370: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82265374: 2B0A7FF0  cmplwi cr6, r10, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32752 as u32, &mut ctx.xer);
	// 82265378: 409A0088  bne cr6, 0x82265400
	if !ctx.cr[6].eq {
	pc = 0x82265400; continue 'dispatch;
	}
	// 8226537C: 3D608227  lis r11, -0x7dd9
	ctx.r[11].s64 = -2111373312;
	// 82265380: C80B1BC0  lfd f0, 0x1bc0(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(7104 as u32) ) };
	// 82265384: D81F0000  stfd f0, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.f[0].u64 ) };
	// 82265388: 4BE2D0F9  bl 0x82092480
	ctx.lr = 0x8226538C;
	sub_82092480(ctx, base);
	// 8226538C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82265390: 40810048  ble 0x822653d8
	if !ctx.cr[0].gt {
	pc = 0x822653D8; continue 'dispatch;
	}
	// 82265394: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82265398: 40990024  ble cr6, 0x822653bc
	if !ctx.cr[6].gt {
	pc = 0x822653BC; continue 'dispatch;
	}
	// 8226539C: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 822653A0: 409A0038  bne cr6, 0x822653d8
	if !ctx.cr[6].eq {
	pc = 0x822653D8; continue 'dispatch;
	}
	// 822653A4: DBFF0000  stfd f31, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.f[31].u64 ) };
	// 822653A8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822653AC: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 822653B0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 822653B4: 4BE31555  bl 0x82096908
	ctx.lr = 0x822653B8;
	sub_82096908(ctx, base);
	// 822653B8: 4800008C  b 0x82265444
	pc = 0x82265444; continue 'dispatch;
            }
            0x822653BC => {
    //   block [0x822653BC..0x822653D8)
	// 822653BC: DBFF0000  stfd f31, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.f[31].u64 ) };
	// 822653C0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822653C4: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 822653C8: C82B06F0  lfd f1, 0x6f0(r11)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(1776 as u32) ) };
	// 822653CC: 4BE3086D  bl 0x82095c38
	ctx.lr = 0x822653D0;
	sub_82095C38(ctx, base);
	// 822653D0: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822653D4: 48000060  b 0x82265434
	pc = 0x82265434; continue 'dispatch;
            }
            0x822653D8 => {
    //   block [0x822653D8..0x82265400)
	// 822653D8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822653DC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 822653E0: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 822653E4: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 822653E8: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 822653EC: C80B0718  lfd f0, 0x718(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(1816 as u32) ) };
	// 822653F0: FC5F002A  fadd f2, f31, f0
	ctx.f[2].f64 = ctx.f[31].f64 + ctx.f[0].f64;
	// 822653F4: D85F0000  stfd f2, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.f[2].u64 ) };
	// 822653F8: 4BE31619  bl 0x82096a10
	ctx.lr = 0x822653FC;
	sub_82096A10(ctx, base);
	// 822653FC: 48000048  b 0x82265444
	pc = 0x82265444; continue 'dispatch;
            }
            0x82265400 => {
    //   block [0x82265400..0x82265434)
	// 82265400: 480015D1  bl 0x822669d0
	ctx.lr = 0x82265404;
	sub_822669D0(ctx, base);
	// 82265404: FFFF0828  fsub f31, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[31].f64 - ctx.f[1].f64;
	// 82265408: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8226540C: D83F0000  stfd f1, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.f[1].u64 ) };
	// 82265410: DBE10050  stfd f31, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[31].u64 ) };
	// 82265414: C80B06F0  lfd f0, 0x6f0(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(1776 as u32) ) };
	// 82265418: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8226541C: 409A0018  bne cr6, 0x82265434
	if !ctx.cr[6].eq {
	pc = 0x82265434; continue 'dispatch;
	}
	// 82265420: A1610050  lhz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82265424: 57AA0420  rlwinm r10, r29, 0, 0x10, 0x10
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	// 82265428: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8226542C: B1610050  sth r11, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 82265430: CBE10050  lfd f31, 0x50(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	pc = 0x82265434; continue 'dispatch;
            }
            0x82265434 => {
    //   block [0x82265434..0x82265444)
	// 82265434: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82265438: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226543C: 4BE31795  bl 0x82096bd0
	ctx.lr = 0x82265440;
	sub_82096BD0(ctx, base);
	// 82265440: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	pc = 0x82265444; continue 'dispatch;
            }
            0x82265444 => {
    //   block [0x82265444..0x82265450)
	// 82265444: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82265448: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8226544C: 4BE2969C  b 0x8208eae8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82265450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82265450 size=96
    let mut pc: u32 = 0x82265450;
    'dispatch: loop {
        match pc {
            0x82265450 => {
    //   block [0x82265450..0x82265458)
	// 82265450: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82265454: 419A005C  beq cr6, 0x822654b0
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x822654B0);
		return;
	}
	pc = 0x82265458; continue 'dispatch;
            }
            0x82265458 => {
    //   block [0x82265458..0x82265474)
	// 82265458: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226545C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82265460: 2F0A0041  cmpwi cr6, r10, 0x41
	ctx.cr[6].compare_i32(ctx.r[10].s32, 65, &mut ctx.xer);
	// 82265464: 41980010  blt cr6, 0x82265474
	if ctx.cr[6].lt {
	pc = 0x82265474; continue 'dispatch;
	}
	// 82265468: 2F0A005A  cmpwi cr6, r10, 0x5a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 90, &mut ctx.xer);
	// 8226546C: 41990008  bgt cr6, 0x82265474
	if ctx.cr[6].gt {
	pc = 0x82265474; continue 'dispatch;
	}
	// 82265470: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	pc = 0x82265474; continue 'dispatch;
            }
            0x82265474 => {
    //   block [0x82265474..0x82265490)
	// 82265474: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265478: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 8226547C: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82265480: 41980010  blt cr6, 0x82265490
	if ctx.cr[6].lt {
	pc = 0x82265490; continue 'dispatch;
	}
	// 82265484: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82265488: 41990008  bgt cr6, 0x82265490
	if ctx.cr[6].gt {
	pc = 0x82265490; continue 'dispatch;
	}
	// 8226548C: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	pc = 0x82265490; continue 'dispatch;
            }
            0x82265490 => {
    //   block [0x82265490..0x822654A8)
	// 82265490: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82265494: 41820014  beq 0x822654a8
	if ctx.cr[0].eq {
	pc = 0x822654A8; continue 'dispatch;
	}
	// 82265498: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8226549C: 419A000C  beq cr6, 0x822654a8
	if ctx.cr[6].eq {
	pc = 0x822654A8; continue 'dispatch;
	}
	// 822654A0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822654A4: 419AFFB4  beq cr6, 0x82265458
	if ctx.cr[6].eq {
	pc = 0x82265458; continue 'dispatch;
	}
	pc = 0x822654A8; continue 'dispatch;
            }
            0x822654A8 => {
    //   block [0x822654A8..0x822654B0)
	// 822654A8: 7C6B5050  subf r3, r11, r10
	ctx.r[3].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822654AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822654B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822654B8 size=164
    let mut pc: u32 = 0x822654B8;
    'dispatch: loop {
        match pc {
            0x822654B8 => {
    //   block [0x822654B8..0x822654D0)
	// 822654B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822654BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822654C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822654C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822654C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822654CC: 409A0034  bne cr6, 0x82265500
	if !ctx.cr[6].eq {
	pc = 0x82265500; continue 'dispatch;
	}
	pc = 0x822654D0; continue 'dispatch;
            }
            0x822654D0 => {
    //   block [0x822654D0..0x82265500)
	// 822654D0: 4BE2D5A9  bl 0x82092a78
	ctx.lr = 0x822654D4;
	sub_82092A78(ctx, base);
	// 822654D4: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 822654D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 822654DC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822654E0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822654E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822654E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822654EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822654F0: 4BE2D449  bl 0x82092938
	ctx.lr = 0x822654F4;
	sub_82092938(ctx, base);
	// 822654F4: 3C607FFF  lis r3, 0x7fff
	ctx.r[3].s64 = 2147418112;
	// 822654F8: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 822654FC: 4800004C  b 0x82265548
	pc = 0x82265548; continue 'dispatch;
            }
            0x82265500 => {
    //   block [0x82265500..0x82265544)
	// 82265500: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82265504: 419AFFCC  beq cr6, 0x822654d0
	if ctx.cr[6].eq {
	pc = 0x822654D0; continue 'dispatch;
	}
	// 82265508: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 8226550C: 617FFFFF  ori r31, r11, 0xffff
	ctx.r[31].u64 = ctx.r[11].u64 | 65535;
	// 82265510: 7F05F840  cmplw cr6, r5, r31
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82265514: 40990030  ble cr6, 0x82265544
	if !ctx.cr[6].gt {
	pc = 0x82265544; continue 'dispatch;
	}
	// 82265518: 4BE2D561  bl 0x82092a78
	ctx.lr = 0x8226551C;
	sub_82092A78(ctx, base);
	// 8226551C: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 82265520: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82265524: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82265528: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226552C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82265530: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82265534: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82265538: 4BE2D401  bl 0x82092938
	ctx.lr = 0x8226553C;
	sub_82092938(ctx, base);
	// 8226553C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82265540: 48000008  b 0x82265548
	pc = 0x82265548; continue 'dispatch;
            }
            0x82265544 => {
    //   block [0x82265544..0x82265548)
	// 82265544: 4BFFFF0D  bl 0x82265450
	ctx.lr = 0x82265548;
	sub_82265450(ctx, base);
	pc = 0x82265548; continue 'dispatch;
            }
            0x82265548 => {
    //   block [0x82265548..0x8226555C)
	// 82265548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226554C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82265550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82265554: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82265558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82265560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82265560 size=612
    let mut pc: u32 = 0x82265560;
    'dispatch: loop {
        match pc {
            0x82265560 => {
    //   block [0x82265560..0x82265594)
	// 82265560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82265564: 4BE29525  bl 0x8208ea88
	ctx.lr = 0x82265568;
	sub_8208EA60(ctx, base);
	// 82265568: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226556C: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82265570: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82265574: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82265578: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8226557C: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82265580: 419A0038  beq cr6, 0x822655b8
	if ctx.cr[6].eq {
	pc = 0x822655B8; continue 'dispatch;
	}
	// 82265584: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82265588: 419A0030  beq cr6, 0x822655b8
	if ctx.cr[6].eq {
	pc = 0x822655B8; continue 'dispatch;
	}
	// 8226558C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82265590: 409A0034  bne cr6, 0x822655c4
	if !ctx.cr[6].eq {
	pc = 0x822655C4; continue 'dispatch;
	}
	pc = 0x82265594; continue 'dispatch;
            }
            0x82265594 => {
    //   block [0x82265594..0x822655B8)
	// 82265594: 4BE2D4E5  bl 0x82092a78
	ctx.lr = 0x82265598;
	sub_82092A78(ctx, base);
	// 82265598: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 8226559C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 822655A0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822655A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822655A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822655AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822655B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822655B4: 4BE2D385  bl 0x82092938
	ctx.lr = 0x822655B8;
	sub_82092938(ctx, base);
	pc = 0x822655B8; continue 'dispatch;
            }
            0x822655B8 => {
    //   block [0x822655B8..0x822655BC)
	// 822655B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x822655BC; continue 'dispatch;
            }
            0x822655BC => {
    //   block [0x822655BC..0x822655C4)
	// 822655BC: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 822655C0: 4BE29518  b 0x8208ead8
	sub_8208EAB0(ctx, base);
	return;
            }
            0x822655C4 => {
    //   block [0x822655C4..0x82265620)
	// 822655C4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 822655C8: 419AFFCC  beq cr6, 0x82265594
	if ctx.cr[6].eq {
	pc = 0x82265594; continue 'dispatch;
	}
	// 822655CC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 822655D0: 0CD90000  twi 6, r25, 0
	// 822655D4: 7D6BCB96  divwu r11, r11, r25
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[25].u32;
	// 822655D8: 7F185840  cmplw cr6, r24, r11
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822655DC: 4199FFB8  bgt cr6, 0x82265594
	if ctx.cr[6].gt {
	pc = 0x82265594; continue 'dispatch;
	}
	// 822655E0: 7F59C1D6  mullw r26, r25, r24
	ctx.r[26].s32 = ((ctx.r[25].s32 as i64 * ctx.r[24].s32 as i64) as i32);
	ctx.r[26].s64 = ctx.r[26].s32 as i64;
	// 822655E4: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 822655E8: 4BE31EC1  bl 0x820974a8
	ctx.lr = 0x822655EC;
	sub_820974A8(ctx, base);
	// 822655EC: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 822655F0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822655F4: 419A0184  beq cr6, 0x82265778
	if ctx.cr[6].eq {
	pc = 0x82265778; continue 'dispatch;
	}
	// 822655F8: 4BE31EB1  bl 0x820974a8
	ctx.lr = 0x822655FC;
	sub_820974A8(ctx, base);
	// 822655FC: 39630040  addi r11, r3, 0x40
	ctx.r[11].s64 = ctx.r[3].s64 + 64;
	// 82265600: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82265604: 419A0174  beq cr6, 0x82265778
	if ctx.cr[6].eq {
	pc = 0x82265778; continue 'dispatch;
	}
	// 82265608: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8226560C: 716B010C  andi. r11, r11, 0x10c
	ctx.r[11].u64 = ctx.r[11].u64 & 268;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82265610: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82265614: 4182000C  beq 0x82265620
	if ctx.cr[0].eq {
	pc = 0x82265620; continue 'dispatch;
	}
	// 82265618: 837E0018  lwz r27, 0x18(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8226561C: 48000008  b 0x82265624
	pc = 0x82265624; continue 'dispatch;
            }
            0x82265620 => {
    //   block [0x82265620..0x82265624)
	// 82265620: 3B601000  li r27, 0x1000
	ctx.r[27].s64 = 4096;
	pc = 0x82265624; continue 'dispatch;
            }
            0x82265624 => {
    //   block [0x82265624..0x8226562C)
	// 82265624: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82265628: 419A0194  beq cr6, 0x822657bc
	if ctx.cr[6].eq {
	pc = 0x822657BC; continue 'dispatch;
	}
	pc = 0x8226562C; continue 'dispatch;
            }
            0x8226562C => {
    //   block [0x8226562C..0x82265658)
	// 8226562C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82265630: 716B0108  andi. r11, r11, 0x108
	ctx.r[11].u64 = ctx.r[11].u64 & 264;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82265634: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82265638: 41820054  beq 0x8226568c
	if ctx.cr[0].eq {
	pc = 0x8226568C; continue 'dispatch;
	}
	// 8226563C: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82265640: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82265644: 419A0048  beq cr6, 0x8226568c
	if ctx.cr[6].eq {
	pc = 0x8226568C; continue 'dispatch;
	}
	// 82265648: 419800C4  blt cr6, 0x8226570c
	if ctx.cr[6].lt {
	pc = 0x8226570C; continue 'dispatch;
	}
	// 8226564C: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82265650: 40980008  bge cr6, 0x82265658
	if !ctx.cr[6].lt {
	pc = 0x82265658; continue 'dispatch;
	}
	// 82265654: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	pc = 0x82265658; continue 'dispatch;
            }
            0x82265658 => {
    //   block [0x82265658..0x8226568C)
	// 82265658: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226565C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265660: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82265664: 4BE29B8D  bl 0x8208f1f0
	ctx.lr = 0x82265668;
	sub_8208F1F0(ctx, base);
	// 82265668: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226566C: 7FFDF850  subf r31, r29, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[29].s64;
	// 82265670: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82265674: 7F9DE214  add r28, r29, r28
	ctx.r[28].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 82265678: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8226567C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82265680: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82265684: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82265688: 480000D4  b 0x8226575c
	pc = 0x8226575C; continue 'dispatch;
            }
            0x8226568C => {
    //   block [0x8226568C..0x822656AC)
	// 8226568C: 7F1FD840  cmplw cr6, r31, r27
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82265690: 41980098  blt cr6, 0x82265728
	if ctx.cr[6].lt {
	pc = 0x82265728; continue 'dispatch;
	}
	// 82265694: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82265698: 419A0014  beq cr6, 0x822656ac
	if ctx.cr[6].eq {
	pc = 0x822656AC; continue 'dispatch;
	}
	// 8226569C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822656A0: 4BE33EF9  bl 0x82099598
	ctx.lr = 0x822656A4;
	sub_82099598(ctx, base);
	// 822656A4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 822656A8: 408200C0  bne 0x82265768
	if !ctx.cr[0].eq {
	pc = 0x82265768; continue 'dispatch;
	}
	pc = 0x822656AC; continue 'dispatch;
            }
            0x822656AC => {
    //   block [0x822656AC..0x822656CC)
	// 822656AC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 822656B0: 419A001C  beq cr6, 0x822656cc
	if ctx.cr[6].eq {
	pc = 0x822656CC; continue 'dispatch;
	}
	// 822656B4: 7D7FDB96  divwu r11, r31, r27
	ctx.r[11].u32 = ctx.r[31].u32 / ctx.r[27].u32;
	// 822656B8: 0CDB0000  twi 6, r27, 0
	// 822656BC: 7D6BD9D6  mullw r11, r11, r27
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * ctx.r[27].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 822656C0: 7D6BF850  subf r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 822656C4: 7FABF850  subf r29, r11, r31
	ctx.r[29].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 822656C8: 48000008  b 0x822656d0
	pc = 0x822656D0; continue 'dispatch;
            }
            0x822656CC => {
    //   block [0x822656CC..0x822656D0)
	// 822656CC: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	pc = 0x822656D0; continue 'dispatch;
            }
            0x822656D0 => {
    //   block [0x822656D0..0x822656FC)
	// 822656D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822656D4: 4BE3338D  bl 0x82098a60
	ctx.lr = 0x822656D8;
	sub_82098A60(ctx, base);
	// 822656D8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822656DC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822656E0: 4BE33169  bl 0x82098848
	ctx.lr = 0x822656E4;
	sub_82098848(ctx, base);
	// 822656E4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 822656E8: 419A0024  beq cr6, 0x8226570c
	if ctx.cr[6].eq {
	pc = 0x8226570C; continue 'dispatch;
	}
	// 822656EC: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 822656F0: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 822656F4: 41990008  bgt cr6, 0x822656fc
	if ctx.cr[6].gt {
	pc = 0x822656FC; continue 'dispatch;
	}
	// 822656F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	pc = 0x822656FC; continue 'dispatch;
            }
            0x822656FC => {
    //   block [0x822656FC..0x8226570C)
	// 822656FC: 7FEBF850  subf r31, r11, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82265700: 7F8BE214  add r28, r11, r28
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82265704: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82265708: 40980054  bge cr6, 0x8226575c
	if !ctx.cr[6].lt {
	pc = 0x8226575C; continue 'dispatch;
	}
	pc = 0x8226570C; continue 'dispatch;
            }
            0x8226570C => {
    //   block [0x8226570C..0x82265728)
	// 8226570C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82265710: 7D5FD050  subf r10, r31, r26
	ctx.r[10].s64 = ctx.r[26].s64 - ctx.r[31].s64;
	// 82265714: 0CD90000  twi 6, r25, 0
	// 82265718: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 8226571C: 7C6ACB96  divwu r3, r10, r25
	ctx.r[3].u32 = ctx.r[10].u32 / ctx.r[25].u32;
	// 82265720: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82265724: 4BFFFE98  b 0x822655bc
	pc = 0x822655BC; continue 'dispatch;
            }
            0x82265728 => {
    //   block [0x82265728..0x8226575C)
	// 82265728: 897C0000  lbz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226572C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82265730: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82265734: 4BE2D025  bl 0x82092758
	ctx.lr = 0x82265738;
	sub_82092758(ctx, base);
	// 82265738: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8226573C: 419A002C  beq cr6, 0x82265768
	if ctx.cr[6].eq {
	pc = 0x82265768; continue 'dispatch;
	}
	// 82265740: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82265744: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82265748: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8226574C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82265750: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 82265754: 41990008  bgt cr6, 0x8226575c
	if ctx.cr[6].gt {
	pc = 0x8226575C; continue 'dispatch;
	}
	// 82265758: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	pc = 0x8226575C; continue 'dispatch;
            }
            0x8226575C => {
    //   block [0x8226575C..0x82265768)
	// 8226575C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82265760: 409AFECC  bne cr6, 0x8226562c
	if !ctx.cr[6].eq {
	pc = 0x8226562C; continue 'dispatch;
	}
	// 82265764: 48000058  b 0x822657bc
	pc = 0x822657BC; continue 'dispatch;
            }
            0x82265768 => {
    //   block [0x82265768..0x82265778)
	// 82265768: 7D7FD050  subf r11, r31, r26
	ctx.r[11].s64 = ctx.r[26].s64 - ctx.r[31].s64;
	// 8226576C: 0CD90000  twi 6, r25, 0
	// 82265770: 7C6BCB96  divwu r3, r11, r25
	ctx.r[3].u32 = ctx.r[11].u32 / ctx.r[25].u32;
	// 82265774: 4BFFFE48  b 0x822655bc
	pc = 0x822655BC; continue 'dispatch;
            }
            0x82265778 => {
    //   block [0x82265778..0x82265780)
	// 82265778: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8226577C: 419A0040  beq cr6, 0x822657bc
	if ctx.cr[6].eq {
	pc = 0x822657BC; continue 'dispatch;
	}
	pc = 0x82265780; continue 'dispatch;
            }
            0x82265780 => {
    //   block [0x82265780..0x82265790)
	// 82265780: 2B1F00FF  cmplwi cr6, r31, 0xff
	ctx.cr[6].compare_u32(ctx.r[31].u32, 255 as u32, &mut ctx.xer);
	// 82265784: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82265788: 41980008  blt cr6, 0x82265790
	if ctx.cr[6].lt {
	pc = 0x82265790; continue 'dispatch;
	}
	// 8226578C: 3BC000FF  li r30, 0xff
	ctx.r[30].s64 = 255;
	pc = 0x82265790; continue 'dispatch;
            }
            0x82265790 => {
    //   block [0x82265790..0x822657BC)
	// 82265790: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82265794: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82265798: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8226579C: 4BE29A55  bl 0x8208f1f0
	ctx.lr = 0x822657A0;
	sub_8208F1F0(ctx, base);
	// 822657A0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 822657A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822657A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822657AC: 7D5E59AE  stbx r10, r30, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	// 822657B0: 4BFFD071  bl 0x82262820
	ctx.lr = 0x822657B4;
	sub_82262820(ctx, base);
	// 822657B4: 7FFEF851  subf. r31, r30, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822657B8: 4082FFC8  bne 0x82265780
	if !ctx.cr[0].eq {
	pc = 0x82265780; continue 'dispatch;
	}
	pc = 0x822657BC; continue 'dispatch;
            }
            0x822657BC => {
    //   block [0x822657BC..0x822657C4)
	// 822657BC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 822657C0: 4BFFFDFC  b 0x822655bc
	pc = 0x822655BC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822657C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822657C8 size=184
    let mut pc: u32 = 0x822657C8;
    'dispatch: loop {
        match pc {
            0x822657C8 => {
    //   block [0x822657C8..0x822657E8)
	// 822657C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822657CC: 4BE292D1  bl 0x8208ea9c
	ctx.lr = 0x822657D0;
	sub_8208EA60(ctx, base);
	// 822657D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822657D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822657D8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822657DC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822657E0: 409A0008  bne cr6, 0x822657e8
	if !ctx.cr[6].eq {
	pc = 0x822657E8; continue 'dispatch;
	}
	// 822657E4: 4BE317AD  bl 0x82096f90
	ctx.lr = 0x822657E8;
	sub_82096F90(ctx, base);
	pc = 0x822657E8; continue 'dispatch;
            }
            0x822657E8 => {
    //   block [0x822657E8..0x822657F8)
	// 822657E8: 83FE0014  lwz r31, 0x14(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 822657EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822657F0: 409A0008  bne cr6, 0x822657f8
	if !ctx.cr[6].eq {
	pc = 0x822657F8; continue 'dispatch;
	}
	// 822657F4: 4BE3179D  bl 0x82096f90
	ctx.lr = 0x822657F8;
	sub_82096F90(ctx, base);
	pc = 0x822657F8; continue 'dispatch;
            }
            0x822657F8 => {
    //   block [0x822657F8..0x82265808)
	// 822657F8: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 822657FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82265800: 409A0008  bne cr6, 0x82265808
	if !ctx.cr[6].eq {
	pc = 0x82265808; continue 'dispatch;
	}
	// 82265804: 4BE3178D  bl 0x82096f90
	ctx.lr = 0x82265808;
	sub_82096F90(ctx, base);
	pc = 0x82265808; continue 'dispatch;
            }
            0x82265808 => {
    //   block [0x82265808..0x8226581C)
	// 82265808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226580C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82265810: 419A002C  beq cr6, 0x8226583c
	if ctx.cr[6].eq {
	pc = 0x8226583C; continue 'dispatch;
	}
	// 82265814: 811E0018  lwz r8, 0x18(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82265818: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	pc = 0x8226581C; continue 'dispatch;
            }
            0x8226581C => {
    //   block [0x8226581C..0x8226583C)
	// 8226581C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265820: 7F1D4840  cmplw cr6, r29, r9
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82265824: 419A0030  beq cr6, 0x82265854
	if ctx.cr[6].eq {
	pc = 0x82265854; continue 'dispatch;
	}
	// 82265828: 4198003C  blt cr6, 0x82265864
	if ctx.cr[6].lt {
	pc = 0x82265864; continue 'dispatch;
	}
	// 8226582C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82265830: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82265834: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82265838: 4198FFE4  blt cr6, 0x8226581c
	if ctx.cr[6].lt {
	pc = 0x8226581C; continue 'dispatch;
	}
	pc = 0x8226583C; continue 'dispatch;
            }
            0x8226583C => {
    //   block [0x8226583C..0x82265848)
	// 8226583C: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82265840: 57EB1838  slwi r11, r31, 3
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82265844: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	pc = 0x82265848; continue 'dispatch;
            }
            0x82265848 => {
    //   block [0x82265848..0x8226584C)
	// 82265848: 806BFFFC  lwz r3, -4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	pc = 0x8226584C; continue 'dispatch;
            }
            0x8226584C => {
    //   block [0x8226584C..0x82265854)
	// 8226584C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82265850: 4BE2929C  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            0x82265854 => {
    //   block [0x82265854..0x82265864)
	// 82265854: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82265858: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8226585C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82265860: 4BFFFFEC  b 0x8226584c
	pc = 0x8226584C; continue 'dispatch;
            }
            0x82265864 => {
    //   block [0x82265864..0x82265874)
	// 82265864: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82265868: 409A000C  bne cr6, 0x82265874
	if !ctx.cr[6].eq {
	pc = 0x82265874; continue 'dispatch;
	}
	// 8226586C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82265870: 4BFFFFDC  b 0x8226584c
	pc = 0x8226584C; continue 'dispatch;
            }
            0x82265874 => {
    //   block [0x82265874..0x82265880)
	// 82265874: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82265878: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8226587C: 4BFFFFCC  b 0x82265848
	pc = 0x82265848; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82265880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82265880 size=48
    let mut pc: u32 = 0x82265880;
    'dispatch: loop {
        match pc {
            0x82265880 => {
    //   block [0x82265880..0x822658B0)
	// 82265880: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82265884: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82265888: 419A0028  beq cr6, 0x822658b0
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x822658B0);
		return;
	}
	// 8226588C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82265890: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82265894: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82265898: 419A0018  beq cr6, 0x822658b0
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x822658B0);
		return;
	}
	// 8226589C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822658A0: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 822658A4: 419A000C  beq cr6, 0x822658b0
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x822658B0);
		return;
	}
	// 822658A8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 822658AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822658C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822658C0 size=32
    let mut pc: u32 = 0x822658C0;
    'dispatch: loop {
        match pc {
            0x822658C0 => {
    //   block [0x822658C0..0x822658E0)
	// 822658C0: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 822658C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822658C8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 822658CC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822658D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822658D4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 822658D8: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 822658DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822658E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822658E0 size=96
    let mut pc: u32 = 0x822658E0;
    'dispatch: loop {
        match pc {
            0x822658E0 => {
    //   block [0x822658E0..0x8226590C)
	// 822658E0: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822658E4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822658E8: 419A009C  beq cr6, 0x82265984
	if ctx.cr[6].eq {
		sub_82265940(ctx, base);
		return;
	}
	// 822658EC: 892A0008  lbz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 822658F0: 396A0008  addi r11, r10, 8
	ctx.r[11].s64 = ctx.r[10].s64 + 8;
	// 822658F4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822658F8: 419A008C  beq cr6, 0x82265984
	if ctx.cr[6].eq {
		sub_82265940(ctx, base);
		return;
	}
	// 822658FC: 81240004  lwz r9, 4(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82265900: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82265904: 419A003C  beq cr6, 0x82265940
	if ctx.cr[6].eq {
		sub_82265940(ctx, base);
		return;
	}
	// 82265908: 39490008  addi r10, r9, 8
	ctx.r[10].s64 = ctx.r[9].s64 + 8;
	pc = 0x8226590C; continue 'dispatch;
            }
            0x8226590C => {
    //   block [0x8226590C..0x82265930)
	// 8226590C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265910: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265914: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82265918: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 8226591C: 41820014  beq 0x82265930
	if ctx.cr[0].eq {
	pc = 0x82265930; continue 'dispatch;
	}
	// 82265920: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82265924: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82265928: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8226592C: 419AFFE0  beq cr6, 0x8226590c
	if ctx.cr[6].eq {
	pc = 0x8226590C; continue 'dispatch;
	}
	pc = 0x82265930; continue 'dispatch;
            }
            0x82265930 => {
    //   block [0x82265930..0x82265940)
	// 82265930: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82265934: 4182000C  beq 0x82265940
	if ctx.cr[0].eq {
		sub_82265940(ctx, base);
		return;
	}
	// 82265938: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226593C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82265940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82265940 size=76
    let mut pc: u32 = 0x82265940;
    'dispatch: loop {
        match pc {
            0x82265940 => {
    //   block [0x82265940..0x82265958)
	// 82265940: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265944: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82265948: 41820010  beq 0x82265958
	if ctx.cr[0].eq {
	pc = 0x82265958; continue 'dispatch;
	}
	// 8226594C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265950: 556B0739  rlwinm. r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82265954: 4182FFE4  beq 0x82265938
	if ctx.cr[0].eq {
		sub_822658E0(ctx, base);
		return;
	}
	pc = 0x82265958; continue 'dispatch;
            }
            0x82265958 => {
    //   block [0x82265958..0x82265970)
	// 82265958: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226595C: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82265960: 41820010  beq 0x82265970
	if ctx.cr[0].eq {
	pc = 0x82265970; continue 'dispatch;
	}
	// 82265964: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265968: 554A07FF  clrlwi. r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8226596C: 4182FFCC  beq 0x82265938
	if ctx.cr[0].eq {
		sub_822658E0(ctx, base);
		return;
	}
	pc = 0x82265970; continue 'dispatch;
            }
            0x82265970 => {
    //   block [0x82265970..0x82265984)
	// 82265970: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82265974: 41820010  beq 0x82265984
	if ctx.cr[0].eq {
	pc = 0x82265984; continue 'dispatch;
	}
	// 82265978: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226597C: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82265980: 4182FFB8  beq 0x82265938
	if ctx.cr[0].eq {
		sub_822658E0(ctx, base);
		return;
	}
	pc = 0x82265984; continue 'dispatch;
            }
            0x82265984 => {
    //   block [0x82265984..0x8226598C)
	// 82265984: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82265988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82265990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82265990 size=124
    let mut pc: u32 = 0x82265990;
    'dispatch: loop {
        match pc {
            0x82265990 => {
    //   block [0x82265990..0x822659D4)
	// 82265990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82265994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82265998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226599C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822659A0: 3D40E043  lis r10, -0x1fbd
	ctx.r[10].s64 = -532480000;
	// 822659A4: 614A4F4D  ori r10, r10, 0x4f4d
	ctx.r[10].u64 = ctx.r[10].u64 | 20301;
	// 822659A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822659AC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822659B0: 419A0024  beq cr6, 0x822659d4
	if ctx.cr[6].eq {
	pc = 0x822659D4; continue 'dispatch;
	}
	// 822659B4: 3D40E06D  lis r10, -0x1f93
	ctx.r[10].s64 = -529727488;
	// 822659B8: 614A7363  ori r10, r10, 0x7363
	ctx.r[10].u64 = ctx.r[10].u64 | 29539;
	// 822659BC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822659C0: 409A0038  bne cr6, 0x822659f8
	if !ctx.cr[6].eq {
	pc = 0x822659F8; continue 'dispatch;
	}
	// 822659C4: 4BE2B085  bl 0x82090a48
	ctx.lr = 0x822659C8;
	sub_82090A48(ctx, base);
	// 822659C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822659CC: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 822659D0: 4BE31561  bl 0x82096f30
	ctx.lr = 0x822659D4;
	sub_82096F30(ctx, base);
	pc = 0x822659D4; continue 'dispatch;
            }
            0x822659D4 => {
    //   block [0x822659D4..0x822659F8)
	// 822659D4: 4BE2B075  bl 0x82090a48
	ctx.lr = 0x822659D8;
	sub_82090A48(ctx, base);
	// 822659D8: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 822659DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822659E0: 40990018  ble cr6, 0x822659f8
	if !ctx.cr[6].gt {
	pc = 0x822659F8; continue 'dispatch;
	}
	// 822659E4: 4BE2B065  bl 0x82090a48
	ctx.lr = 0x822659E8;
	sub_82090A48(ctx, base);
	// 822659E8: 39430084  addi r10, r3, 0x84
	ctx.r[10].s64 = ctx.r[3].s64 + 132;
	// 822659EC: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822659F0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 822659F4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x822659F8; continue 'dispatch;
            }
            0x822659F8 => {
    //   block [0x822659F8..0x82265A0C)
	// 822659F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822659FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82265A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82265A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82265A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82265A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82265A18 size=440
    let mut pc: u32 = 0x82265A18;
    'dispatch: loop {
        match pc {
            0x82265A18 => {
    //   block [0x82265A18..0x82265A64)
	// 82265A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82265A1C: 4BE29071  bl 0x8208ea8c
	ctx.lr = 0x82265A20;
	sub_8208EA60(ctx, base);
	// 82265A20: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82265A24: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82265A28: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82265A2C: 935F00B4  stw r26, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[26].u32 ) };
	// 82265A30: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82265A34: 933F00BC  stw r25, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[25].u32 ) };
	// 82265A38: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82265A3C: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 82265A40: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82265A44: 937F00CC  stw r27, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[27].u32 ) };
	// 82265A48: 4BFFFE39  bl 0x82265880
	ctx.lr = 0x82265A4C;
	sub_82265880(ctx, base);
	// 82265A4C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82265A50: 937F0058  stw r27, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 82265A54: 2F1BFFFF  cmpwi cr6, r27, -1
	ctx.cr[6].compare_i32(ctx.r[27].s32, -1, &mut ctx.xer);
	// 82265A58: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82265A5C: 419A002C  beq cr6, 0x82265a88
	if ctx.cr[6].eq {
	pc = 0x82265A88; continue 'dispatch;
	}
	// 82265A60: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	pc = 0x82265A64; continue 'dispatch;
            }
            0x82265A64 => {
    //   block [0x82265A64..0x82265A88)
	// 82265A64: 576B1838  slwi r11, r27, 3
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82265A68: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82265A6C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82265A70: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82265A74: 409A0014  bne cr6, 0x82265a88
	if !ctx.cr[6].eq {
	pc = 0x82265A88; continue 'dispatch;
	}
	// 82265A78: 836B0000  lwz r27, 0(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265A7C: 2F1BFFFF  cmpwi cr6, r27, -1
	ctx.cr[6].compare_i32(ctx.r[27].s32, -1, &mut ctx.xer);
	// 82265A80: 937F00CC  stw r27, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[27].u32 ) };
	// 82265A84: 409AFFE0  bne cr6, 0x82265a64
	if !ctx.cr[6].eq {
	pc = 0x82265A64; continue 'dispatch;
	}
	pc = 0x82265A88; continue 'dispatch;
            }
            0x82265A88 => {
    //   block [0x82265A88..0x82265A98)
	// 82265A88: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 82265A8C: 409A000C  bne cr6, 0x82265a98
	if !ctx.cr[6].eq {
	pc = 0x82265A98; continue 'dispatch;
	}
	// 82265A90: 2F1BFFFF  cmpwi cr6, r27, -1
	ctx.cr[6].compare_i32(ctx.r[27].s32, -1, &mut ctx.xer);
	// 82265A94: 419A00F4  beq cr6, 0x82265b88
	if ctx.cr[6].eq {
	pc = 0x82265B88; continue 'dispatch;
	}
	pc = 0x82265A98; continue 'dispatch;
            }
            0x82265A98 => {
    //   block [0x82265A98..0x82265AAC)
	// 82265A98: 4BE2AFB1  bl 0x82090a48
	ctx.lr = 0x82265A9C;
	sub_82090A48(ctx, base);
	// 82265A9C: 39430084  addi r10, r3, 0x84
	ctx.r[10].s64 = ctx.r[3].s64 + 132;
	// 82265AA0: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265AA4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82265AA8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82265AAC; continue 'dispatch;
            }
            0x82265AAC => {
    //   block [0x82265AAC..0x82265ACC)
	// 82265AAC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82265AB0: 7F1CD800  cmpw cr6, r28, r27
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82265AB4: 419A00A0  beq cr6, 0x82265b54
	if ctx.cr[6].eq {
	pc = 0x82265B54; continue 'dispatch;
	}
	// 82265AB8: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 82265ABC: 40990010  ble cr6, 0x82265acc
	if !ctx.cr[6].gt {
	pc = 0x82265ACC; continue 'dispatch;
	}
	// 82265AC0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82265AC4: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82265AC8: 41980008  blt cr6, 0x82265ad0
	if ctx.cr[6].lt {
	pc = 0x82265AD0; continue 'dispatch;
	}
	pc = 0x82265ACC; continue 'dispatch;
            }
            0x82265ACC => {
    //   block [0x82265ACC..0x82265AD0)
	// 82265ACC: 4BE314C5  bl 0x82096f90
	ctx.lr = 0x82265AD0;
	sub_82096F90(ctx, base);
	pc = 0x82265AD0; continue 'dispatch;
            }
            0x82265AD0 => {
    //   block [0x82265AD0..0x82265B28)
	// 82265AD0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82265AD4: 579D1838  slwi r29, r28, 3
	ctx.r[29].u32 = ctx.r[28].u32.wrapping_shl(3);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82265AD8: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82265ADC: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265AE0: 939F0054  stw r28, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82265AE4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82265AE8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82265AEC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82265AF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82265AF4: 419A0034  beq cr6, 0x82265b28
	if ctx.cr[6].eq {
	pc = 0x82265B28; continue 'dispatch;
	}
	// 82265AF8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82265AFC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82265B00: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82265B04: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82265B08: 4BFFFDB9  bl 0x822658c0
	ctx.lr = 0x82265B0C;
	sub_822658C0(ctx, base);
	// 82265B0C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82265B10: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82265B14: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82265B18: 38A00103  li r5, 0x103
	ctx.r[5].s64 = 259;
	// 82265B1C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82265B20: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82265B24: 48000F1D  bl 0x82266a40
	ctx.lr = 0x82265B28;
	sub_82266A40(ctx, base);
	pc = 0x82265B28; continue 'dispatch;
            }
            0x82265B28 => {
    //   block [0x82265B28..0x82265B4C)
	// 82265B28: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82265B2C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82265B30: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82265B34: 48000018  b 0x82265b4c
	pc = 0x82265B4C; continue 'dispatch;
	// 82265B38: 837F00CC  lwz r27, 0xcc(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82265B3C: 83DF00C4  lwz r30, 0xc4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82265B40: 833F00BC  lwz r25, 0xbc(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82265B44: 835F00B4  lwz r26, 0xb4(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82265B48: 839F0054  lwz r28, 0x54(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
            }
            0x82265B4C => {
    //   block [0x82265B4C..0x82265B54)
	// 82265B4C: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82265B50: 4BFFFF5C  b 0x82265aac
	pc = 0x82265AAC; continue 'dispatch;
            }
            0x82265B54 => {
    //   block [0x82265B54..0x82265B74)
	// 82265B54: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82265B58: 399F00A0  addi r12, r31, 0xa0
	ctx.r[12].s64 = ctx.r[31].s64 + 160;
	// 82265B5C: 48000035  bl 0x82265b90
	ctx.lr = 0x82265B60;
	sub_82265A18(ctx, base);
	// 82265B60: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82265B64: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82265B68: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82265B6C: 419A0008  beq cr6, 0x82265b74
	if ctx.cr[6].eq {
	pc = 0x82265B74; continue 'dispatch;
	}
	// 82265B70: 4BE31421  bl 0x82096f90
	ctx.lr = 0x82265B74;
	sub_82096F90(ctx, base);
	pc = 0x82265B74; continue 'dispatch;
            }
            0x82265B74 => {
    //   block [0x82265B74..0x82265B88)
	// 82265B74: 80DF0058  lwz r6, 0x58(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82265B78: 80BF00C4  lwz r5, 0xc4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82265B7C: 809F00BC  lwz r4, 0xbc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82265B80: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82265B84: 4BFFFD3D  bl 0x822658c0
	ctx.lr = 0x82265B88;
	sub_822658C0(ctx, base);
	pc = 0x82265B88; continue 'dispatch;
            }
            0x82265B88 => {
    //   block [0x82265B88..0x82265B90)
	// 82265B88: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82265B8C: 4BE28F50  b 0x8208eadc
	sub_8208EAB0(ctx, base);
	return;
            }
            0x82265B90 => {
    //   block [0x82265B90..0x82265BC0)
	// 82265B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82265B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82265B98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82265B9C: 4BE2AEAD  bl 0x82090a48
	ctx.lr = 0x82265BA0;
	sub_82090A48(ctx, base);
	// 82265BA0: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82265BA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82265BA8: 40990018  ble cr6, 0x82265bc0
	if !ctx.cr[6].gt {
	pc = 0x82265BC0; continue 'dispatch;
	}
	// 82265BAC: 4BE2AE9D  bl 0x82090a48
	ctx.lr = 0x82265BB0;
	sub_82090A48(ctx, base);
	// 82265BB0: 39430084  addi r10, r3, 0x84
	ctx.r[10].s64 = ctx.r[3].s64 + 132;
	// 82265BB4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265BB8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82265BBC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82265BC0; continue 'dispatch;
            }
            0x82265BC0 => {
    //   block [0x82265BC0..0x82265BD0)
	// 82265BC0: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82265BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82265BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82265BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82265BD0 size=36
    let mut pc: u32 = 0x82265BD0;
    'dispatch: loop {
        match pc {
            0x82265BD0 => {
    //   block [0x82265BD0..0x82265BF4)
	// 82265BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82265BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82265BD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82265BDC: 4BFFFDB5  bl 0x82265990
	ctx.lr = 0x82265BE0;
	sub_82265990(ctx, base);
	// 82265BE0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82265BE4: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82265BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82265BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82265BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82265BF8 size=24
    let mut pc: u32 = 0x82265BF8;
    'dispatch: loop {
        match pc {
            0x82265BF8 => {
    //   block [0x82265BF8..0x82265C10)
	// 82265BF8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82265BFC: 716B0066  andi. r11, r11, 0x66
	ctx.r[11].u64 = ctx.r[11].u64 & 102;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82265C00: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82265C04: 4082000C  bne 0x82265c10
	if !ctx.cr[0].eq {
		crate::recompiler::externs::call(ctx, base, 0x82265C10);
		return;
	}
	// 82265C08: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82265C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82265C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82265C40 size=172
    let mut pc: u32 = 0x82265C40;
    'dispatch: loop {
        match pc {
            0x82265C40 => {
    //   block [0x82265C40..0x82265CB4)
	// 82265C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82265C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82265C48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82265C4C: 3BE1FFA0  addi r31, r1, -0x60
	ctx.r[31].s64 = ctx.r[1].s64 + -96;
	// 82265C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82265C54: 989F007F  stb r4, 0x7f(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(127 as u32), ctx.r[4].u8 ) };
	// 82265C58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82265C5C: 419A0058  beq cr6, 0x82265cb4
	if ctx.cr[6].eq {
	pc = 0x82265CB4; continue 'dispatch;
	}
	// 82265C60: 3D60E06D  lis r11, -0x1f93
	ctx.r[11].s64 = -529727488;
	// 82265C64: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265C68: 616B7363  ori r11, r11, 0x7363
	ctx.r[11].u64 = ctx.r[11].u64 | 29539;
	// 82265C6C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82265C70: 409A0044  bne cr6, 0x82265cb4
	if !ctx.cr[6].eq {
	pc = 0x82265CB4; continue 'dispatch;
	}
	// 82265C74: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82265C78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82265C7C: 419A0038  beq cr6, 0x82265cb4
	if ctx.cr[6].eq {
	pc = 0x82265CB4; continue 'dispatch;
	}
	// 82265C80: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82265C84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82265C88: 419A002C  beq cr6, 0x82265cb4
	if ctx.cr[6].eq {
	pc = 0x82265CB4; continue 'dispatch;
	}
	// 82265C8C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82265C90: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82265C94: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82265C98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82265C9C: 4E800421  bctrl
	ctx.lr = 0x82265CA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82265CA0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82265CA4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82265CA8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82265CAC: 48000008  b 0x82265cb4
	pc = 0x82265CB4; continue 'dispatch;
	// 82265CB0: 4BE31281  bl 0x82096f30
	ctx.lr = 0x82265CB4;
	sub_82096F30(ctx, base);
            }
            0x82265CB4 => {
    //   block [0x82265CB4..0x82265CEC)
	// 82265CB4: 383F0060  addi r1, r31, 0x60
	ctx.r[1].s64 = ctx.r[31].s64 + 96;
	// 82265CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82265CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82265CC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82265CC4: 4E800020  blr
	return;
	// 82265CC8: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82265CCC: 3BECFFA0  addi r31, r12, -0x60
	ctx.r[31].s64 = ctx.r[12].s64 + -96;
	// 82265CD0: 897F007F  lbz r11, 0x7f(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(127 as u32) ) } as u64;
	// 82265CD4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82265CD8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82265CDC: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82265CE0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82265CE4: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82265CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82265CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82265CF0 size=196
    let mut pc: u32 = 0x82265CF0;
    'dispatch: loop {
        match pc {
            0x82265CF0 => {
    //   block [0x82265CF0..0x82265DA8)
	// 82265CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82265CF4: 4BE28D95  bl 0x8208ea88
	ctx.lr = 0x82265CF8;
	sub_8208EA60(ctx, base);
	// 82265CF8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82265CFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82265D00: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82265D04: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82265D08: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82265D0C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82265D10: 7D3F4B78  mr r31, r9
	ctx.r[31].u64 = ctx.r[9].u64;
	// 82265D14: 4BE2AD35  bl 0x82090a48
	ctx.lr = 0x82265D18;
	sub_82090A48(ctx, base);
	// 82265D18: 8323007C  lwz r25, 0x7c(r3)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) } as u64;
	// 82265D1C: 4BE2AD2D  bl 0x82090a48
	ctx.lr = 0x82265D20;
	sub_82090A48(ctx, base);
	// 82265D20: 83030080  lwz r24, 0x80(r3)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 82265D24: 4BE2AD25  bl 0x82090a48
	ctx.lr = 0x82265D28;
	sub_82090A48(ctx, base);
	// 82265D28: 93C3007C  stw r30, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 82265D2C: 4BE2AD1D  bl 0x82090a48
	ctx.lr = 0x82265D30;
	sub_82090A48(ctx, base);
	// 82265D30: 3960FFFE  li r11, -2
	ctx.r[11].s64 = -2;
	// 82265D34: 93830080  stw r28, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 82265D38: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82265D3C: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265D40: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 82265D44: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82265D48: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82265D4C: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82265D50: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82265D54: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82265D58: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82265D5C: 48000CE5  bl 0x82266a40
	ctx.lr = 0x82265D60;
	sub_82266A40(ctx, base);
	// 82265D60: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82265D64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82265D68: 4BFFDDE1  bl 0x82263b48
	ctx.lr = 0x82265D6C;
	sub_82263B48(ctx, base);
	// 82265D6C: 4BE2ACDD  bl 0x82090a48
	ctx.lr = 0x82265D70;
	sub_82090A48(ctx, base);
	// 82265D70: 9323007C  stw r25, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[25].u32 ) };
	// 82265D74: 4BE2ACD5  bl 0x82090a48
	ctx.lr = 0x82265D78;
	sub_82090A48(ctx, base);
	// 82265D78: 93030080  stw r24, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[24].u32 ) };
	// 82265D7C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265D80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82265D84: 419A0024  beq cr6, 0x82265da8
	if ctx.cr[6].eq {
	pc = 0x82265DA8; continue 'dispatch;
	}
	// 82265D88: 4BFFDD69  bl 0x82263af0
	ctx.lr = 0x82265D8C;
	sub_82263AF0(ctx, base);
	// 82265D8C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82265D90: 41820018  beq 0x82265da8
	if ctx.cr[0].eq {
	pc = 0x82265DA8; continue 'dispatch;
	}
	// 82265D94: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82265D98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82265D9C: 4BFFFEA5  bl 0x82265c40
	ctx.lr = 0x82265DA0;
	sub_82265C40(ctx, base);
	// 82265DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82265DA4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82265DA8; continue 'dispatch;
            }
            0x82265DA8 => {
    //   block [0x82265DA8..0x82265DB4)
	// 82265DA8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82265DAC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82265DB0: 4BE28D28  b 0x8208ead8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82265DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82265DC0 size=648
    let mut pc: u32 = 0x82265DC0;
    'dispatch: loop {
        match pc {
            0x82265DC0 => {
    //   block [0x82265DC0..0x82265E10)
	// 82265DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82265DC4: 4BE28CD5  bl 0x8208ea98
	ctx.lr = 0x82265DC8;
	sub_8208EA60(ctx, base);
	// 82265DC8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82265DCC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82265DD0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82265DD4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82265DD8: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82265DDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82265DE0: 915F005C  stw r10, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82265DE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82265DE8: 419A0248  beq cr6, 0x82266030
	if ctx.cr[6].eq {
	pc = 0x82266030; continue 'dispatch;
	}
	// 82265DEC: 896B0008  lbz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82265DF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82265DF4: 419A023C  beq cr6, 0x82266030
	if ctx.cr[6].eq {
	pc = 0x82266030; continue 'dispatch;
	}
	// 82265DF8: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82265DFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82265E00: 409A0010  bne cr6, 0x82265e10
	if !ctx.cr[6].eq {
	pc = 0x82265E10; continue 'dispatch;
	}
	// 82265E04: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265E08: 554A0001  rlwinm. r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82265E0C: 41820224  beq 0x82266030
	if ctx.cr[0].eq {
	pc = 0x82266030; continue 'dispatch;
	}
	pc = 0x82265E10; continue 'dispatch;
            }
            0x82265E10 => {
    //   block [0x82265E10..0x82265E24)
	// 82265E10: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265E14: 55490001  rlwinm. r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82265E18: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82265E1C: 40820008  bne 0x82265e24
	if !ctx.cr[0].eq {
	pc = 0x82265E24; continue 'dispatch;
	}
	// 82265E20: 7F8B2214  add r28, r11, r4
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	pc = 0x82265E24; continue 'dispatch;
            }
            0x82265E24 => {
    //   block [0x82265E24..0x82265EA4)
	// 82265E24: 60000000  nop
	// 82265E28: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82265E2C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82265E30: 554B0739  rlwinm. r11, r10, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82265E34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82265E38: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82265E3C: 41820070  beq 0x82265eac
	if ctx.cr[0].eq {
	pc = 0x82265EAC; continue 'dispatch;
	}
	// 82265E40: 48000D21  bl 0x82266b60
	ctx.lr = 0x82265E44;
	sub_82266B60(ctx, base);
	// 82265E44: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82265E48: 418201C8  beq 0x82266010
	if ctx.cr[0].eq {
	pc = 0x82266010; continue 'dispatch;
	}
	// 82265E4C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82265E50: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82265E54: 48000D0D  bl 0x82266b60
	ctx.lr = 0x82265E58;
	sub_82266B60(ctx, base);
	// 82265E58: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82265E5C: 418201B4  beq 0x82266010
	if ctx.cr[0].eq {
	pc = 0x82266010; continue 'dispatch;
	}
	// 82265E60: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82265E64: 397E0008  addi r11, r30, 8
	ctx.r[11].s64 = ctx.r[30].s64 + 8;
	// 82265E68: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82265E6C: 811E000C  lwz r8, 0xc(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82265E70: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82265E74: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82265E78: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82265E7C: 913F0050  stw r9, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82265E80: 41980024  blt cr6, 0x82265ea4
	if ctx.cr[6].lt {
	pc = 0x82265EA4; continue 'dispatch;
	}
	// 82265E84: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82265E88: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82265E8C: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82265E90: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82265E94: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82265E98: 7D285A14  add r9, r8, r11
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82265E9C: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82265EA0: 913F0050  stw r9, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	pc = 0x82265EA4; continue 'dispatch;
            }
            0x82265EA4 => {
    //   block [0x82265EA4..0x82265EAC)
	// 82265EA4: 913C0000  stw r9, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82265EA8: 4800016C  b 0x82266014
	pc = 0x82266014; continue 'dispatch;
            }
            0x82265EAC => {
    //   block [0x82265EAC..0x82265F3C)
	// 82265EAC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265EB0: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82265EB4: 41820090  beq 0x82265f44
	if ctx.cr[0].eq {
	pc = 0x82265F44; continue 'dispatch;
	}
	// 82265EB8: 48000CA9  bl 0x82266b60
	ctx.lr = 0x82265EBC;
	sub_82266B60(ctx, base);
	// 82265EBC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82265EC0: 41820150  beq 0x82266010
	if ctx.cr[0].eq {
	pc = 0x82266010; continue 'dispatch;
	}
	// 82265EC4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82265EC8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82265ECC: 48000C95  bl 0x82266b60
	ctx.lr = 0x82265ED0;
	sub_82266B60(ctx, base);
	// 82265ED0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82265ED4: 4182013C  beq 0x82266010
	if ctx.cr[0].eq {
	pc = 0x82266010; continue 'dispatch;
	}
	// 82265ED8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82265EDC: 80BE0014  lwz r5, 0x14(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82265EE0: 809D0018  lwz r4, 0x18(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82265EE4: 4BE2987D  bl 0x8208f760
	ctx.lr = 0x82265EE8;
	sub_8208F760(ctx, base);
	// 82265EE8: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82265EEC: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 82265EF0: 409A0124  bne cr6, 0x82266014
	if !ctx.cr[6].eq {
	pc = 0x82266014; continue 'dispatch;
	}
	// 82265EF4: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265EF8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82265EFC: 419A0118  beq cr6, 0x82266014
	if ctx.cr[6].eq {
	pc = 0x82266014; continue 'dispatch;
	}
	// 82265F00: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82265F04: 397E0008  addi r11, r30, 8
	ctx.r[11].s64 = ctx.r[30].s64 + 8;
	// 82265F08: 813E000C  lwz r9, 0xc(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82265F0C: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82265F10: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82265F14: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82265F18: 41980024  blt cr6, 0x82265f3c
	if ctx.cr[6].lt {
	pc = 0x82265F3C; continue 'dispatch;
	}
	// 82265F1C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82265F20: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82265F24: 7D09402E  lwzx r8, r9, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82265F28: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82265F2C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82265F30: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82265F34: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82265F38: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	pc = 0x82265F3C; continue 'dispatch;
            }
            0x82265F3C => {
    //   block [0x82265F3C..0x82265F44)
	// 82265F3C: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82265F40: 480000D4  b 0x82266014
	pc = 0x82266014; continue 'dispatch;
            }
            0x82265F44 => {
    //   block [0x82265F44..0x82265FB0)
	// 82265F44: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82265F48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82265F4C: 409A0074  bne cr6, 0x82265fc0
	if !ctx.cr[6].eq {
	pc = 0x82265FC0; continue 'dispatch;
	}
	// 82265F50: 48000C11  bl 0x82266b60
	ctx.lr = 0x82265F54;
	sub_82266B60(ctx, base);
	// 82265F54: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82265F58: 418200B8  beq 0x82266010
	if ctx.cr[0].eq {
	pc = 0x82266010; continue 'dispatch;
	}
	// 82265F5C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82265F60: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82265F64: 48000BFD  bl 0x82266b60
	ctx.lr = 0x82265F68;
	sub_82266B60(ctx, base);
	// 82265F68: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82265F6C: 418200A4  beq 0x82266010
	if ctx.cr[0].eq {
	pc = 0x82266010; continue 'dispatch;
	}
	// 82265F70: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82265F74: 397E0008  addi r11, r30, 8
	ctx.r[11].s64 = ctx.r[30].s64 + 8;
	// 82265F78: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82265F7C: 811E000C  lwz r8, 0xc(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82265F80: 7C895214  add r4, r9, r10
	ctx.r[4].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82265F84: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82265F88: 909F0058  stw r4, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 82265F8C: 41980024  blt cr6, 0x82265fb0
	if ctx.cr[6].lt {
	pc = 0x82265FB0; continue 'dispatch;
	}
	// 82265F90: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82265F94: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82265F98: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82265F9C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82265FA0: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82265FA4: 7C895A14  add r4, r9, r11
	ctx.r[4].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82265FA8: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82265FAC: 909F0058  stw r4, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	pc = 0x82265FB0; continue 'dispatch;
            }
            0x82265FB0 => {
    //   block [0x82265FB0..0x82265FC0)
	// 82265FB0: 80BE0014  lwz r5, 0x14(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82265FB4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82265FB8: 4BE297A9  bl 0x8208f760
	ctx.lr = 0x82265FBC;
	sub_8208F760(ctx, base);
	// 82265FBC: 48000058  b 0x82266014
	pc = 0x82266014; continue 'dispatch;
            }
            0x82265FC0 => {
    //   block [0x82265FC0..0x82266010)
	// 82265FC0: 48000BA1  bl 0x82266b60
	ctx.lr = 0x82265FC4;
	sub_82266B60(ctx, base);
	// 82265FC4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82265FC8: 41820048  beq 0x82266010
	if ctx.cr[0].eq {
	pc = 0x82266010; continue 'dispatch;
	}
	// 82265FCC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82265FD0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82265FD4: 48000B8D  bl 0x82266b60
	ctx.lr = 0x82265FD8;
	sub_82266B60(ctx, base);
	// 82265FD8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82265FDC: 41820034  beq 0x82266010
	if ctx.cr[0].eq {
	pc = 0x82266010; continue 'dispatch;
	}
	// 82265FE0: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82265FE4: 48000B7D  bl 0x82266b60
	ctx.lr = 0x82265FE8;
	sub_82266B60(ctx, base);
	// 82265FE8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82265FEC: 41820024  beq 0x82266010
	if ctx.cr[0].eq {
	pc = 0x82266010; continue 'dispatch;
	}
	// 82265FF0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82265FF4: 556B077A  rlwinm r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82265FF8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82265FFC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82266000: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82266004: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82266008: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8226600C: 48000008  b 0x82266014
	pc = 0x82266014; continue 'dispatch;
            }
            0x82266010 => {
    //   block [0x82266010..0x82266014)
	// 82266010: 4BE30F81  bl 0x82096f90
	ctx.lr = 0x82266014;
	sub_82096F90(ctx, base);
	pc = 0x82266014; continue 'dispatch;
            }
            0x82266014 => {
    //   block [0x82266014..0x82266028)
	// 82266014: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82266018: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8226601C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82266020: 48000008  b 0x82266028
	pc = 0x82266028; continue 'dispatch;
	// 82266024: 4BE30F0D  bl 0x82096f30
	ctx.lr = 0x82266028;
	sub_82096F30(ctx, base);
            }
            0x82266028 => {
    //   block [0x82266028..0x82266030)
	// 82266028: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8226602C: 48000008  b 0x82266034
	pc = 0x82266034; continue 'dispatch;
            }
            0x82266030 => {
    //   block [0x82266030..0x82266034)
	// 82266030: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82266034; continue 'dispatch;
            }
            0x82266034 => {
    //   block [0x82266034..0x82266048)
	// 82266034: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82266038: 4BE28AB0  b 0x8208eae8
	sub_8208EAB0(ctx, base);
	return;
	// 8226603C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82266040: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82266044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82266050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82266050 size=300
    let mut pc: u32 = 0x82266050;
    'dispatch: loop {
        match pc {
            0x82266050 => {
    //   block [0x82266050..0x8226607C)
	// 82266050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82266054: 4BE28A45  bl 0x8208ea98
	ctx.lr = 0x82266058;
	sub_8208EA60(ctx, base);
	// 82266058: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8226605C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82266060: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82266064: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82266068: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226606C: 556B0001  rlwinm. r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82266070: 4182000C  beq 0x8226607c
	if ctx.cr[0].eq {
	pc = 0x8226607C; continue 'dispatch;
	}
	// 82266074: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82266078: 4800000C  b 0x82266084
	pc = 0x82266084; continue 'dispatch;
            }
            0x8226607C => {
    //   block [0x8226607C..0x82266084)
	// 8226607C: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82266080: 7F8B2214  add r28, r11, r4
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	pc = 0x82266084; continue 'dispatch;
            }
            0x82266084 => {
    //   block [0x82266084..0x822660EC)
	// 82266084: 60000000  nop
	// 82266088: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8226608C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82266090: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82266094: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82266098: 4BFFFD29  bl 0x82265dc0
	ctx.lr = 0x8226609C;
	sub_82265DC0(ctx, base);
	// 8226609C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 822660A0: 419A0064  beq cr6, 0x82266104
	if ctx.cr[6].eq {
	pc = 0x82266104; continue 'dispatch;
	}
	// 822660A4: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 822660A8: 409A00AC  bne cr6, 0x82266154
	if !ctx.cr[6].eq {
	pc = 0x82266154; continue 'dispatch;
	}
	// 822660AC: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 822660B0: 397D0008  addi r11, r29, 8
	ctx.r[11].s64 = ctx.r[29].s64 + 8;
	// 822660B4: 813D0008  lwz r9, 8(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822660B8: 811D000C  lwz r8, 0xc(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 822660BC: 7C895214  add r4, r9, r10
	ctx.r[4].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 822660C0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 822660C4: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 822660C8: 41980024  blt cr6, 0x822660ec
	if ctx.cr[6].lt {
	pc = 0x822660EC; continue 'dispatch;
	}
	// 822660CC: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822660D0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822660D4: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822660D8: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822660DC: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 822660E0: 7C895A14  add r4, r9, r11
	ctx.r[4].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 822660E4: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822660E8: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	pc = 0x822660EC; continue 'dispatch;
            }
            0x822660EC => {
    //   block [0x822660EC..0x82266104)
	// 822660EC: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 822660F0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822660F4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822660F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822660FC: 4E800421  bctrl
	ctx.lr = 0x82266100;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82266100: 48000054  b 0x82266154
	pc = 0x82266154; continue 'dispatch;
            }
            0x82266104 => {
    //   block [0x82266104..0x82266144)
	// 82266104: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82266108: 397D0008  addi r11, r29, 8
	ctx.r[11].s64 = ctx.r[29].s64 + 8;
	// 8226610C: 813D0008  lwz r9, 8(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82266110: 811D000C  lwz r8, 0xc(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82266114: 7C895214  add r4, r9, r10
	ctx.r[4].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82266118: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8226611C: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 82266120: 41980024  blt cr6, 0x82266144
	if ctx.cr[6].lt {
	pc = 0x82266144; continue 'dispatch;
	}
	// 82266124: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82266128: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8226612C: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82266130: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82266134: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82266138: 7C895A14  add r4, r9, r11
	ctx.r[4].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8226613C: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82266140: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	pc = 0x82266144; continue 'dispatch;
            }
            0x82266144 => {
    //   block [0x82266144..0x82266154)
	// 82266144: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82266148: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8226614C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82266150: 4E800421  bctrl
	ctx.lr = 0x82266154;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82266154 => {
    //   block [0x82266154..0x82266168)
	// 82266154: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82266158: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8226615C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82266160: 48000008  b 0x82266168
	pc = 0x82266168; continue 'dispatch;
	// 82266164: 4BE30DCD  bl 0x82096f30
	ctx.lr = 0x82266168;
	sub_82096F30(ctx, base);
            }
            0x82266168 => {
    //   block [0x82266168..0x8226617C)
	// 82266168: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8226616C: 4BE2897C  b 0x8208eae8
	sub_8208EAB0(ctx, base);
	return;
	// 82266170: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82266174: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82266178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82266180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82266180 size=324
    let mut pc: u32 = 0x82266180;
    'dispatch: loop {
        match pc {
            0x82266180 => {
    //   block [0x82266180..0x822661F0)
	// 82266180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82266184: 4BE288F9  bl 0x8208ea7c
	ctx.lr = 0x82266188;
	sub_8208EA60(ctx, base);
	// 82266188: 9421F4F0  stwu r1, -0xb10(r1)
	ea = ctx.r[1].u32.wrapping_add(-2832 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226618C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82266190: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82266194: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82266198: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8226619C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822661A0: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 822661A4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 822661A8: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 822661AC: 7D374B78  mr r23, r9
	ctx.r[23].u64 = ctx.r[9].u64;
	// 822661B0: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 822661B4: 4BFFD42D  bl 0x822635e0
	ctx.lr = 0x822661B8;
	sub_822635E0(ctx, base);
	// 822661B8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 822661BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822661C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822661C4: 4BFFD445  bl 0x82263608
	ctx.lr = 0x822661C8;
	sub_82263608(ctx, base);
	// 822661C8: 7C761B79  or. r22, r3, r3
	ctx.r[22].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 822661CC: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 822661D0: 40820020  bne 0x822661f0
	if !ctx.cr[0].eq {
	pc = 0x822661F0; continue 'dispatch;
	}
	// 822661D4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 822661D8: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 822661DC: 3940FFFE  li r10, -2
	ctx.r[10].s64 = -2;
	// 822661E0: 92A10058  stw r21, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[21].u32 ) };
	// 822661E4: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822661E8: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822661EC: 92A1005C  stw r21, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[21].u32 ) };
	pc = 0x822661F0; continue 'dispatch;
            }
            0x822661F0 => {
    //   block [0x822661F0..0x8226620C)
	// 822661F0: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 822661F4: 419A0018  beq cr6, 0x8226620c
	if ctx.cr[6].eq {
	pc = 0x8226620C; continue 'dispatch;
	}
	// 822661F8: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 822661FC: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82266200: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82266204: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82266208: 4BFFFE49  bl 0x82266050
	ctx.lr = 0x8226620C;
	sub_82266050(ctx, base);
	pc = 0x8226620C; continue 'dispatch;
            }
            0x8226620C => {
    //   block [0x8226620C..0x8226623C)
	// 8226620C: 4BE2A83D  bl 0x82090a48
	ctx.lr = 0x82266210;
	sub_82090A48(ctx, base);
	// 82266210: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82266214: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82266218: 9163008C  stw r11, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 8226621C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82266220: 4BFFD819  bl 0x82263a38
	ctx.lr = 0x82266224;
	sub_82263A38(ctx, base);
	// 82266224: 81610B6C  lwz r11, 0xb6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2924 as u32) ) } as u64;
	// 82266228: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 8226622C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82266230: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82266234: 409A0008  bne cr6, 0x8226623c
	if !ctx.cr[6].eq {
	pc = 0x8226623C; continue 'dispatch;
	}
	// 82266238: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	pc = 0x8226623C; continue 'dispatch;
            }
            0x8226623C => {
    //   block [0x8226623C..0x822662B4)
	// 8226623C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82266240: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82266244: 4800093D  bl 0x82266b80
	ctx.lr = 0x82266248;
	sub_82266B80(ctx, base);
	// 82266248: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8226624C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82266250: 80D90000  lwz r6, 0(r25)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82266254: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82266258: 4BFFF7C1  bl 0x82265a18
	ctx.lr = 0x8226625C;
	sub_82265A18(ctx, base);
	// 8226625C: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82266260: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82266264: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82266268: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226626C: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 82266270: 4BFFF651  bl 0x822658c0
	ctx.lr = 0x82266274;
	sub_822658C0(ctx, base);
	// 82266274: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82266278: 4BE2A7D1  bl 0x82090a48
	ctx.lr = 0x8226627C;
	sub_82090A48(ctx, base);
	// 8226627C: 8143008C  lwz r10, 0x8c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 82266280: 7EE9BB78  mr r9, r23
	ctx.r[9].u64 = ctx.r[23].u64;
	// 82266284: 80F8000C  lwz r7, 0xc(r24)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(12 as u32) ) } as u64;
	// 82266288: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 8226628C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82266290: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82266294: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82266298: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8226629C: 48000991  bl 0x82266c2c
	ctx.lr = 0x822662A0;
	sub_82266C2C(ctx, base);
	// 822662A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822662A4: 41820018  beq 0x822662bc
	if ctx.cr[0].eq {
	pc = 0x822662BC; continue 'dispatch;
	}
	// 822662A8: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 822662AC: 409A0008  bne cr6, 0x822662b4
	if !ctx.cr[6].eq {
	pc = 0x822662B4; continue 'dispatch;
	}
	// 822662B0: 92BE0000  stw r21, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[21].u32 ) };
	pc = 0x822662B4; continue 'dispatch;
            }
            0x822662B4 => {
    //   block [0x822662B4..0x822662BC)
	// 822662B4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 822662B8: 480007E5  bl 0x82266a9c
	ctx.lr = 0x822662BC;
	sub_82266A9C(ctx, base);
	pc = 0x822662BC; continue 'dispatch;
            }
            0x822662BC => {
    //   block [0x822662BC..0x822662C4)
	// 822662BC: 38210B10  addi r1, r1, 0xb10
	ctx.r[1].s64 = ctx.r[1].s64 + 2832;
	// 822662C0: 4BE2880C  b 0x8208eacc
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822662C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822662C8 size=364
    let mut pc: u32 = 0x822662C8;
    'dispatch: loop {
        match pc {
            0x822662C8 => {
    //   block [0x822662C8..0x82266364)
	// 822662C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822662CC: 4BE287B9  bl 0x8208ea84
	ctx.lr = 0x822662D0;
	sub_8208EA60(ctx, base);
	// 822662D0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822662D4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 822662D8: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 822662DC: 7D585378  mr r24, r10
	ctx.r[24].u64 = ctx.r[10].u64;
	// 822662E0: 616B0003  ori r11, r11, 3
	ctx.r[11].u64 = ctx.r[11].u64 | 3;
	// 822662E4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822662E8: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822662EC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 822662F0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 822662F4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 822662F8: 7D174378  mr r23, r8
	ctx.r[23].u64 = ctx.r[8].u64;
	// 822662FC: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 82266300: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82266304: 419A0128  beq cr6, 0x8226642c
	if ctx.cr[6].eq {
	pc = 0x8226642C; continue 'dispatch;
	}
	// 82266308: 4BE2A741  bl 0x82090a48
	ctx.lr = 0x8226630C;
	sub_82090A48(ctx, base);
	// 8226630C: 81630074  lwz r11, 0x74(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) } as u64;
	// 82266310: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82266314: 419A0050  beq cr6, 0x82266364
	if ctx.cr[6].eq {
	pc = 0x82266364; continue 'dispatch;
	}
	// 82266318: 4BE2A731  bl 0x82090a48
	ctx.lr = 0x8226631C;
	sub_82090A48(ctx, base);
	// 8226631C: 81630074  lwz r11, 0x74(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) } as u64;
	// 82266320: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82266324: 419A0040  beq cr6, 0x82266364
	if ctx.cr[6].eq {
	pc = 0x82266364; continue 'dispatch;
	}
	// 82266328: 3D60E043  lis r11, -0x1fbd
	ctx.r[11].s64 = -532480000;
	// 8226632C: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82266330: 616B4F4D  ori r11, r11, 0x4f4d
	ctx.r[11].u64 = ctx.r[11].u64 | 20301;
	// 82266334: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82266338: 419A002C  beq cr6, 0x82266364
	if ctx.cr[6].eq {
	pc = 0x82266364; continue 'dispatch;
	}
	// 8226633C: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 82266340: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82266344: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82266348: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8226634C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82266350: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82266354: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82266358: 4BFFD4C9  bl 0x82263820
	ctx.lr = 0x8226635C;
	sub_82263820(ctx, base);
	// 8226635C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82266360: 408200CC  bne 0x8226642c
	if !ctx.cr[0].eq {
	pc = 0x8226642C; continue 'dispatch;
	}
	pc = 0x82266364; continue 'dispatch;
            }
            0x82266364 => {
    //   block [0x82266364..0x82266374)
	// 82266364: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82266368: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226636C: 409A0008  bne cr6, 0x82266374
	if !ctx.cr[6].eq {
	pc = 0x82266374; continue 'dispatch;
	}
	// 82266370: 4BE30C21  bl 0x82096f90
	ctx.lr = 0x82266374;
	sub_82096F90(ctx, base);
	pc = 0x82266374; continue 'dispatch;
            }
            0x82266374 => {
    //   block [0x82266374..0x8226639C)
	// 82266374: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82266378: 38E10074  addi r7, r1, 0x74
	ctx.r[7].s64 = ctx.r[1].s64 + 116;
	// 8226637C: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82266380: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82266384: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82266388: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8226638C: 4BFFD555  bl 0x822638e0
	ctx.lr = 0x82266390;
	sub_822638E0(ctx, base);
	// 82266390: 81210070  lwz r9, 0x70(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82266394: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82266398: 48000088  b 0x82266420
	pc = 0x82266420; continue 'dispatch;
            }
            0x8226639C => {
    //   block [0x8226639C..0x822663DC)
	// 8226639C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822663A0: 7F175800  cmpw cr6, r23, r11
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822663A4: 41980070  blt cr6, 0x82266414
	if ctx.cr[6].lt {
	pc = 0x82266414; continue 'dispatch;
	}
	// 822663A8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822663AC: 7F175800  cmpw cr6, r23, r11
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822663B0: 41990064  bgt cr6, 0x82266414
	if ctx.cr[6].gt {
	pc = 0x82266414; continue 'dispatch;
	}
	// 822663B4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822663B8: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822663BC: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822663C0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 822663C4: 814BFFF4  lwz r10, -0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12 as u32) ) } as u64;
	// 822663C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822663CC: 419A0010  beq cr6, 0x822663dc
	if ctx.cr[6].eq {
	pc = 0x822663DC; continue 'dispatch;
	}
	// 822663D0: 894A0008  lbz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 822663D4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822663D8: 409A003C  bne cr6, 0x82266414
	if !ctx.cr[6].eq {
	pc = 0x82266414; continue 'dispatch;
	}
	pc = 0x822663DC; continue 'dispatch;
            }
            0x822663DC => {
    //   block [0x822663DC..0x82266414)
	// 822663DC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 822663E0: 9301005C  stw r24, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[24].u32 ) };
	// 822663E4: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 822663E8: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 822663EC: 99010067  stb r8, 0x67(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(103 as u32), ctx.r[8].u8 ) };
	// 822663F0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 822663F4: 390BFFF0  addi r8, r11, -0x10
	ctx.r[8].s64 = ctx.r[11].s64 + -16;
	// 822663F8: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 822663FC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82266400: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82266404: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82266408: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8226640C: 4BFFFD75  bl 0x82266180
	ctx.lr = 0x82266410;
	sub_82266180(ctx, base);
	// 82266410: 81210070  lwz r9, 0x70(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	pc = 0x82266414; continue 'dispatch;
            }
            0x82266414 => {
    //   block [0x82266414..0x82266420)
	// 82266414: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82266418: 3BFF0014  addi r31, r31, 0x14
	ctx.r[31].s64 = ctx.r[31].s64 + 20;
	// 8226641C: 91210070  stw r9, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	pc = 0x82266420; continue 'dispatch;
            }
            0x82266420 => {
    //   block [0x82266420..0x8226642C)
	// 82266420: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82266424: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82266428: 4198FF74  blt cr6, 0x8226639c
	if ctx.cr[6].lt {
	pc = 0x8226639C; continue 'dispatch;
	}
	pc = 0x8226642C; continue 'dispatch;
            }
            0x8226642C => {
    //   block [0x8226642C..0x82266434)
	// 8226642C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82266430: 4BE286A4  b 0x8208ead4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82266438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82266438 size=692
    let mut pc: u32 = 0x82266438;
    'dispatch: loop {
        match pc {
            0x82266438 => {
    //   block [0x82266438..0x82266490)
	// 82266438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226643C: 4BE28631  bl 0x8208ea6c
	ctx.lr = 0x82266440;
	sub_8208EA60(ctx, base);
	// 82266440: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82266444: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82266448: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 8226644C: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82266450: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82266454: 7CB42B78  mr r20, r5
	ctx.r[20].u64 = ctx.r[5].u64;
	// 82266458: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 8226645C: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82266460: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82266464: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82266468: 7D324B78  mr r18, r9
	ctx.r[18].u64 = ctx.r[9].u64;
	// 8226646C: 7D515378  mr r17, r10
	ctx.r[17].u64 = ctx.r[10].u64;
	// 82266470: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 82266474: 4BFFF40D  bl 0x82265880
	ctx.lr = 0x82266478;
	sub_82265880(ctx, base);
	// 82266478: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 8226647C: 2F15FFFF  cmpwi cr6, r21, -1
	ctx.cr[6].compare_i32(ctx.r[21].s32, -1, &mut ctx.xer);
	// 82266480: 41980010  blt cr6, 0x82266490
	if ctx.cr[6].lt {
	pc = 0x82266490; continue 'dispatch;
	}
	// 82266484: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82266488: 7F155800  cmpw cr6, r21, r11
	ctx.cr[6].compare_i32(ctx.r[21].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8226648C: 41980008  blt cr6, 0x82266494
	if ctx.cr[6].lt {
	pc = 0x82266494; continue 'dispatch;
	}
	pc = 0x82266490; continue 'dispatch;
            }
            0x82266490 => {
    //   block [0x82266490..0x82266494)
	// 82266490: 4BE30B01  bl 0x82096f90
	ctx.lr = 0x82266494;
	sub_82096F90(ctx, base);
	pc = 0x82266494; continue 'dispatch;
            }
            0x82266494 => {
    //   block [0x82266494..0x822664E8)
	// 82266494: 3D60E06D  lis r11, -0x1f93
	ctx.r[11].s64 = -529727488;
	// 82266498: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226649C: 617C7363  ori r28, r11, 0x7363
	ctx.r[28].u64 = ctx.r[11].u64 | 29539;
	// 822664A0: 7F0AE040  cmplw cr6, r10, r28
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[28].u32, &mut ctx.xer);
	// 822664A4: 409A0208  bne cr6, 0x822666ac
	if !ctx.cr[6].eq {
	pc = 0x822666AC; continue 'dispatch;
	}
	// 822664A8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822664AC: 3D401993  lis r10, 0x1993
	ctx.r[10].s64 = 429064192;
	// 822664B0: 3D201993  lis r9, 0x1993
	ctx.r[9].s64 = 429064192;
	// 822664B4: 3D001993  lis r8, 0x1993
	ctx.r[8].s64 = 429064192;
	// 822664B8: 615E0520  ori r30, r10, 0x520
	ctx.r[30].u64 = ctx.r[10].u64 | 1312;
	// 822664BC: 613D0521  ori r29, r9, 0x521
	ctx.r[29].u64 = ctx.r[9].u64 | 1313;
	// 822664C0: 611B0522  ori r27, r8, 0x522
	ctx.r[27].u64 = ctx.r[8].u64 | 1314;
	// 822664C4: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 822664C8: 409A00AC  bne cr6, 0x82266574
	if !ctx.cr[6].eq {
	pc = 0x82266574; continue 'dispatch;
	}
	// 822664CC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822664D0: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 822664D4: 419A0014  beq cr6, 0x822664e8
	if ctx.cr[6].eq {
	pc = 0x822664E8; continue 'dispatch;
	}
	// 822664D8: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 822664DC: 419A000C  beq cr6, 0x822664e8
	if ctx.cr[6].eq {
	pc = 0x822664E8; continue 'dispatch;
	}
	// 822664E0: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 822664E4: 409A0090  bne cr6, 0x82266574
	if !ctx.cr[6].eq {
	pc = 0x82266574; continue 'dispatch;
	}
	pc = 0x822664E8; continue 'dispatch;
            }
            0x822664E8 => {
    //   block [0x822664E8..0x82266530)
	// 822664E8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 822664EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822664F0: 409A0084  bne cr6, 0x82266574
	if !ctx.cr[6].eq {
	pc = 0x82266574; continue 'dispatch;
	}
	// 822664F4: 4BE2A555  bl 0x82090a48
	ctx.lr = 0x822664F8;
	sub_82090A48(ctx, base);
	// 822664F8: 8163007C  lwz r11, 0x7c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) } as u64;
	// 822664FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82266500: 419A01A4  beq cr6, 0x822666a4
	if ctx.cr[6].eq {
	pc = 0x822666A4; continue 'dispatch;
	}
	// 82266504: 4BE2A545  bl 0x82090a48
	ctx.lr = 0x82266508;
	sub_82090A48(ctx, base);
	// 82266508: 83E3007C  lwz r31, 0x7c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) } as u64;
	// 8226650C: 4BE2A53D  bl 0x82090a48
	ctx.lr = 0x82266510;
	sub_82090A48(ctx, base);
	// 82266510: 82830080  lwz r20, 0x80(r3)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 82266514: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82266518: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226651C: 3A600001  li r19, 1
	ctx.r[19].s64 = 1;
	// 82266520: 48000641  bl 0x82266b60
	ctx.lr = 0x82266524;
	sub_82266B60(ctx, base);
	// 82266524: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82266528: 40820008  bne 0x82266530
	if !ctx.cr[0].eq {
	pc = 0x82266530; continue 'dispatch;
	}
	// 8226652C: 4BE30A65  bl 0x82096f90
	ctx.lr = 0x82266530;
	sub_82096F90(ctx, base);
	pc = 0x82266530; continue 'dispatch;
            }
            0x82266530 => {
    //   block [0x82266530..0x82266564)
	// 82266530: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82266534: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82266538: 409A0174  bne cr6, 0x822666ac
	if !ctx.cr[6].eq {
	pc = 0x822666AC; continue 'dispatch;
	}
	// 8226653C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82266540: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82266544: 409A0030  bne cr6, 0x82266574
	if !ctx.cr[6].eq {
	pc = 0x82266574; continue 'dispatch;
	}
	// 82266548: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8226654C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82266550: 419A0014  beq cr6, 0x82266564
	if ctx.cr[6].eq {
	pc = 0x82266564; continue 'dispatch;
	}
	// 82266554: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82266558: 419A000C  beq cr6, 0x82266564
	if ctx.cr[6].eq {
	pc = 0x82266564; continue 'dispatch;
	}
	// 8226655C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82266560: 409A0014  bne cr6, 0x82266574
	if !ctx.cr[6].eq {
	pc = 0x82266574; continue 'dispatch;
	}
	pc = 0x82266564; continue 'dispatch;
            }
            0x82266564 => {
    //   block [0x82266564..0x82266574)
	// 82266564: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82266568: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226656C: 409A0008  bne cr6, 0x82266574
	if !ctx.cr[6].eq {
	pc = 0x82266574; continue 'dispatch;
	}
	// 82266570: 4BE30A21  bl 0x82096f90
	ctx.lr = 0x82266574;
	sub_82096F90(ctx, base);
	pc = 0x82266574; continue 'dispatch;
            }
            0x82266574 => {
    //   block [0x82266574..0x822665A8)
	// 82266574: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82266578: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8226657C: 409A0130  bne cr6, 0x822666ac
	if !ctx.cr[6].eq {
	pc = 0x822666AC; continue 'dispatch;
	}
	// 82266580: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82266584: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82266588: 409A0124  bne cr6, 0x822666ac
	if !ctx.cr[6].eq {
	pc = 0x822666AC; continue 'dispatch;
	}
	// 8226658C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82266590: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82266594: 419A0014  beq cr6, 0x822665a8
	if ctx.cr[6].eq {
	pc = 0x822665A8; continue 'dispatch;
	}
	// 82266598: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8226659C: 419A000C  beq cr6, 0x822665a8
	if ctx.cr[6].eq {
	pc = 0x822665A8; continue 'dispatch;
	}
	// 822665A0: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 822665A4: 409A0108  bne cr6, 0x822666ac
	if !ctx.cr[6].eq {
	pc = 0x822666AC; continue 'dispatch;
	}
	pc = 0x822665A8; continue 'dispatch;
            }
            0x822665A8 => {
    //   block [0x822665A8..0x822665DC)
	// 822665A8: 8178000C  lwz r11, 0xc(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(12 as u32) ) } as u64;
	// 822665AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822665B0: 409900F4  ble cr6, 0x822666a4
	if !ctx.cr[6].gt {
	pc = 0x822666A4; continue 'dispatch;
	}
	// 822665B4: 7EC8B378  mr r8, r22
	ctx.r[8].u64 = ctx.r[22].u64;
	// 822665B8: 38E10074  addi r7, r1, 0x74
	ctx.r[7].s64 = ctx.r[1].s64 + 116;
	// 822665BC: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 822665C0: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 822665C4: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 822665C8: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 822665CC: 4BFFD315  bl 0x822638e0
	ctx.lr = 0x822665D0;
	sub_822638E0(ctx, base);
	// 822665D0: 83210070  lwz r25, 0x70(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 822665D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822665D8: 480000C0  b 0x82266698
	pc = 0x82266698; continue 'dispatch;
            }
            0x822665DC => {
    //   block [0x822665DC..0x82266614)
	// 822665DC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822665E0: 7F0BA800  cmpw cr6, r11, r21
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[21].s32, &mut ctx.xer);
	// 822665E4: 419900A8  bgt cr6, 0x8226668c
	if ctx.cr[6].gt {
	pc = 0x8226668C; continue 'dispatch;
	}
	// 822665E8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822665EC: 7F155800  cmpw cr6, r21, r11
	ctx.cr[6].compare_i32(ctx.r[21].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822665F0: 4199009C  bgt cr6, 0x8226668c
	if ctx.cr[6].gt {
	pc = 0x8226668C; continue 'dispatch;
	}
	// 822665F4: 835E000C  lwz r26, 0xc(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 822665F8: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 822665FC: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82266600: 4099008C  ble cr6, 0x8226668c
	if !ctx.cr[6].gt {
	pc = 0x8226668C; continue 'dispatch;
	}
	// 82266604: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82266608: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8226660C: 3B6B0004  addi r27, r11, 4
	ctx.r[27].s64 = ctx.r[11].s64 + 4;
	// 82266610: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82266614; continue 'dispatch;
            }
            0x82266614 => {
    //   block [0x82266614..0x82266628)
	// 82266614: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82266618: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 8226661C: 2C1C0000  cmpwi r28, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82266620: 40810028  ble 0x82266648
	if !ctx.cr[0].gt {
	pc = 0x82266648; continue 'dispatch;
	}
	// 82266624: 80BF001C  lwz r5, 0x1c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	pc = 0x82266628; continue 'dispatch;
            }
            0x82266628 => {
    //   block [0x82266628..0x82266648)
	// 82266628: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 8226662C: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82266630: 4BFFF2B1  bl 0x822658e0
	ctx.lr = 0x82266634;
	sub_822658E0(ctx, base);
	// 82266634: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82266638: 40820020  bne 0x82266658
	if !ctx.cr[0].eq {
	pc = 0x82266658; continue 'dispatch;
	}
	// 8226663C: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82266640: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 82266644: 4181FFE4  bgt 0x82266628
	if ctx.cr[0].gt {
	pc = 0x82266628; continue 'dispatch;
	}
	pc = 0x82266648; continue 'dispatch;
            }
            0x82266648 => {
    //   block [0x82266648..0x82266658)
	// 82266648: 375AFFFF  addic. r26, r26, -1
	ctx.xer.ca = (ctx.r[26].u32 > (!(-1 as u32)));
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8226664C: 38E70010  addi r7, r7, 0x10
	ctx.r[7].s64 = ctx.r[7].s64 + 16;
	// 82266650: 4181FFC4  bgt 0x82266614
	if ctx.cr[0].gt {
	pc = 0x82266614; continue 'dispatch;
	}
	// 82266654: 48000038  b 0x8226668c
	pc = 0x8226668C; continue 'dispatch;
            }
            0x82266658 => {
    //   block [0x82266658..0x8226668C)
	// 82266658: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 8226665C: 9A610067  stb r19, 0x67(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(103 as u32), ctx.r[19].u8 ) };
	// 82266660: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82266664: 9221005C  stw r17, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[17].u32 ) };
	// 82266668: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8226666C: 92410054  stw r18, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[18].u32 ) };
	// 82266670: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82266674: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 82266678: 7E85A378  mr r5, r20
	ctx.r[5].u64 = ctx.r[20].u64;
	// 8226667C: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82266680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82266684: 4BFFFAFD  bl 0x82266180
	ctx.lr = 0x82266688;
	sub_82266180(ctx, base);
	// 82266688: 83210070  lwz r25, 0x70(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	pc = 0x8226668C; continue 'dispatch;
            }
            0x8226668C => {
    //   block [0x8226668C..0x82266698)
	// 8226668C: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82266690: 3BDE0014  addi r30, r30, 0x14
	ctx.r[30].s64 = ctx.r[30].s64 + 20;
	// 82266694: 93210070  stw r25, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[25].u32 ) };
	pc = 0x82266698; continue 'dispatch;
            }
            0x82266698 => {
    //   block [0x82266698..0x822666A4)
	// 82266698: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8226669C: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822666A0: 4198FF3C  blt cr6, 0x822665dc
	if ctx.cr[6].lt {
	pc = 0x822665DC; continue 'dispatch;
	}
	pc = 0x822666A4; continue 'dispatch;
            }
            0x822666A4 => {
    //   block [0x822666A4..0x822666AC)
	// 822666A4: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 822666A8: 4BE28414  b 0x8208eabc
	sub_8208EAB0(ctx, base);
	return;
            }
            0x822666AC => {
    //   block [0x822666AC..0x822666E8)
	// 822666AC: 8178000C  lwz r11, 0xc(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(12 as u32) ) } as u64;
	// 822666B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822666B4: 4099FFF0  ble cr6, 0x822666a4
	if !ctx.cr[6].gt {
	pc = 0x822666A4; continue 'dispatch;
	}
	// 822666B8: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822666BC: 4082002C  bne 0x822666e8
	if !ctx.cr[0].eq {
	pc = 0x822666E8; continue 'dispatch;
	}
	// 822666C0: 7E2A8B78  mr r10, r17
	ctx.r[10].u64 = ctx.r[17].u64;
	// 822666C4: 7E499378  mr r9, r18
	ctx.r[9].u64 = ctx.r[18].u64;
	// 822666C8: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 822666CC: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 822666D0: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 822666D4: 7E85A378  mr r5, r20
	ctx.r[5].u64 = ctx.r[20].u64;
	// 822666D8: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 822666DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822666E0: 4BFFFBE9  bl 0x822662c8
	ctx.lr = 0x822666E4;
	sub_822662C8(ctx, base);
	// 822666E4: 4BFFFFC0  b 0x822666a4
	pc = 0x822666A4; continue 'dispatch;
            }
            0x822666E8 => {
    //   block [0x822666E8..0x822666EC)
	// 822666E8: 4BE30849  bl 0x82096f30
	ctx.lr = 0x822666EC;
	sub_82096F30(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822666F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822666F8 size=532
    let mut pc: u32 = 0x822666F8;
    'dispatch: loop {
        match pc {
            0x822666F8 => {
    //   block [0x822666F8..0x82266768)
	// 822666F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822666FC: 4BE28395  bl 0x8208ea90
	ctx.lr = 0x82266700;
	sub_8208EA60(ctx, base);
	// 82266700: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82266704: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82266708: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8226670C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82266710: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82266714: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82266718: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8226671C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82266720: 71470066  andi. r7, r10, 0x66
	ctx.r[7].u64 = ctx.r[10].u64 & 102;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82266724: 28070000  cmplwi r7, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82266728: 4182013C  beq 0x82266864
	if ctx.cr[0].eq {
	pc = 0x82266864; continue 'dispatch;
	}
	// 8226672C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82266730: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82266734: 419A01C0  beq cr6, 0x822668f4
	if ctx.cr[6].eq {
	pc = 0x822668F4; continue 'dispatch;
	}
	// 82266738: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8226673C: 409A01B8  bne cr6, 0x822668f4
	if !ctx.cr[6].eq {
	pc = 0x822668F4; continue 'dispatch;
	}
	// 82266740: 554B06B5  rlwinm. r11, r10, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82266744: 41820078  beq 0x822667bc
	if ctx.cr[0].eq {
	pc = 0x822667BC; continue 'dispatch;
	}
	// 82266748: 4BFFCE71  bl 0x822635b8
	ctx.lr = 0x8226674C;
	sub_822635B8(ctx, base);
	// 8226674C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82266750: 4182006C  beq 0x822667bc
	if ctx.cr[0].eq {
	pc = 0x822667BC; continue 'dispatch;
	}
	// 82266754: 4BE2A2F5  bl 0x82090a48
	ctx.lr = 0x82266758;
	sub_82090A48(ctx, base);
	// 82266758: 8163008C  lwz r11, 0x8c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 8226675C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82266760: 409A0008  bne cr6, 0x82266768
	if !ctx.cr[6].eq {
	pc = 0x82266768; continue 'dispatch;
	}
	// 82266764: 4BE3082D  bl 0x82096f90
	ctx.lr = 0x82266768;
	sub_82096F90(ctx, base);
	pc = 0x82266768; continue 'dispatch;
            }
            0x82266768 => {
    //   block [0x82266768..0x822667BC)
	// 82266768: 4BE2A2E1  bl 0x82090a48
	ctx.lr = 0x8226676C;
	sub_82090A48(ctx, base);
	// 8226676C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82266770: 8063008C  lwz r3, 0x8c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 82266774: 4BFFCE8D  bl 0x82263600
	ctx.lr = 0x82266778;
	sub_82263600(ctx, base);
	// 82266778: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226677C: 4BE2A2CD  bl 0x82090a48
	ctx.lr = 0x82266780;
	sub_82090A48(ctx, base);
	// 82266780: 8163008C  lwz r11, 0x8c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 82266784: FBAB0030  std r29, 0x30(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[29].u64 ) };
	// 82266788: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226678C: 4BE2A2BD  bl 0x82090a48
	ctx.lr = 0x82266790;
	sub_82090A48(ctx, base);
	// 82266790: 8163008C  lwz r11, 0x8c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 82266794: FBAB0038  std r29, 0x38(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[29].u64 ) };
	// 82266798: 83DE0008  lwz r30, 8(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8226679C: 4BE2A2AD  bl 0x82090a48
	ctx.lr = 0x822667A0;
	sub_82090A48(ctx, base);
	// 822667A0: 8163008C  lwz r11, 0x8c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 822667A4: FBCB0040  std r30, 0x40(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[30].u64 ) };
	// 822667A8: 4BFFCE11  bl 0x822635b8
	ctx.lr = 0x822667AC;
	sub_822635B8(ctx, base);
	// 822667AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822667B0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822667B4: 4BFFCE4D  bl 0x82263600
	ctx.lr = 0x822667B8;
	sub_82263600(ctx, base);
	// 822667B8: 4800013C  b 0x822668f4
	pc = 0x822668F4; continue 'dispatch;
            }
            0x822667BC => {
    //   block [0x822667BC..0x82266850)
	// 822667BC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822667C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822667C4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 822667C8: 4BFFCED9  bl 0x822636a0
	ctx.lr = 0x822667CC;
	sub_822636A0(ctx, base);
	// 822667CC: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 822667D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822667D4: 419A0120  beq cr6, 0x822668f4
	if ctx.cr[6].eq {
	pc = 0x822668F4; continue 'dispatch;
	}
	// 822667D8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822667DC: 83AB0004  lwz r29, 4(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822667E0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822667E4: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 822667E8: 419A010C  beq cr6, 0x822668f4
	if ctx.cr[6].eq {
	pc = 0x822668F4; continue 'dispatch;
	}
	// 822667EC: 83DD0008  lwz r30, 8(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822667F0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822667F4: 419A0100  beq cr6, 0x822668f4
	if ctx.cr[6].eq {
	pc = 0x822668F4; continue 'dispatch;
	}
	// 822667F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822667FC: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82266800: 4BFFD349  bl 0x82263b48
	ctx.lr = 0x82266804;
	sub_82263B48(ctx, base);
	// 82266804: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82266808: 4BFFD2E9  bl 0x82263af0
	ctx.lr = 0x8226680C;
	sub_82263AF0(ctx, base);
	// 8226680C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82266810: 41820048  beq 0x82266858
	if ctx.cr[0].eq {
	pc = 0x82266858; continue 'dispatch;
	}
	// 82266814: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82266818: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226681C: 419A003C  beq cr6, 0x82266858
	if ctx.cr[6].eq {
	pc = 0x82266858; continue 'dispatch;
	}
	// 82266820: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82266824: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82266828: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226682C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82266830: 4E800421  bctrl
	ctx.lr = 0x82266834;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82266834: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82266838: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8226683C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82266840: 48000010  b 0x82266850
	pc = 0x82266850; continue 'dispatch;
	// 82266844: 4BE306ED  bl 0x82096f30
	ctx.lr = 0x82266848;
	sub_82096F30(ctx, base);
	// 82266848: 83BF0050  lwz r29, 0x50(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8226684C: 83DF0054  lwz r30, 0x54(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
            }
            0x82266850 => {
    //   block [0x82266850..0x82266858)
	// 82266850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82266854: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82266858; continue 'dispatch;
            }
            0x82266858 => {
    //   block [0x82266858..0x82266864)
	// 82266858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226685C: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82266860: 48000094  b 0x822668f4
	pc = 0x822668F4; continue 'dispatch;
            }
            0x82266864 => {
    //   block [0x82266864..0x822668D4)
	// 82266864: 815D000C  lwz r10, 0xc(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82266868: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8226686C: 419A0088  beq cr6, 0x822668f4
	if ctx.cr[6].eq {
	pc = 0x822668F4; continue 'dispatch;
	}
	// 82266870: 3D40E06D  lis r10, -0x1f93
	ctx.r[10].s64 = -529727488;
	// 82266874: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82266878: 614A7363  ori r10, r10, 0x7363
	ctx.r[10].u64 = ctx.r[10].u64 | 29539;
	// 8226687C: 7F075040  cmplw cr6, r7, r10
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82266880: 409A0054  bne cr6, 0x822668d4
	if !ctx.cr[6].eq {
	pc = 0x822668D4; continue 'dispatch;
	}
	// 82266884: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82266888: 2B0A0003  cmplwi cr6, r10, 3
	ctx.cr[6].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	// 8226688C: 41980048  blt cr6, 0x822668d4
	if ctx.cr[6].lt {
	pc = 0x822668D4; continue 'dispatch;
	}
	// 82266890: 3D401993  lis r10, 0x1993
	ctx.r[10].s64 = 429064192;
	// 82266894: 80E30014  lwz r7, 0x14(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82266898: 614A0522  ori r10, r10, 0x522
	ctx.r[10].u64 = ctx.r[10].u64 | 1314;
	// 8226689C: 7F075040  cmplw cr6, r7, r10
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822668A0: 40990034  ble cr6, 0x822668d4
	if !ctx.cr[6].gt {
	pc = 0x822668D4; continue 'dispatch;
	}
	// 822668A4: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 822668A8: 834A0008  lwz r26, 8(r10)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 822668AC: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 822668B0: 419A0024  beq cr6, 0x822668d4
	if ctx.cr[6].eq {
	pc = 0x822668D4; continue 'dispatch;
	}
	// 822668B4: 556A063E  clrlwi r10, r11, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 822668B8: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 822668BC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 822668C0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822668C4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 822668C8: 7F4903A6  mtctr r26
	ctx.ctr.u64 = ctx.r[26].u64;
	// 822668CC: 4E800421  bctrl
	ctx.lr = 0x822668D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822668D0: 48000028  b 0x822668f8
	pc = 0x822668F8; continue 'dispatch;
            }
            0x822668D4 => {
    //   block [0x822668D4..0x822668F4)
	// 822668D4: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 822668D8: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 822668DC: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 822668E0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 822668E4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 822668E8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822668EC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 822668F0: 4BFFFB49  bl 0x82266438
	ctx.lr = 0x822668F4;
	sub_82266438(ctx, base);
	pc = 0x822668F4; continue 'dispatch;
            }
            0x822668F4 => {
    //   block [0x822668F4..0x822668F8)
	// 822668F4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x822668F8; continue 'dispatch;
            }
            0x822668F8 => {
    //   block [0x822668F8..0x8226690C)
	// 822668F8: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 822668FC: 4BE281E4  b 0x8208eae0
	sub_8208EAB0(ctx, base);
	return;
	// 82266900: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82266904: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82266908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82266910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82266910 size=180
    let mut pc: u32 = 0x82266910;
    'dispatch: loop {
        match pc {
            0x82266910 => {
    //   block [0x82266910..0x8226696C)
	// 82266910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82266914: 4BE28181  bl 0x8208ea94
	ctx.lr = 0x82266918;
	sub_8208EA60(ctx, base);
	// 82266918: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226691C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82266920: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82266924: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82266928: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8226692C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82266930: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82266934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82266938: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226693C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82266940: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82266944: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82266948: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8226694C: 4BE33E35  bl 0x8209a780
	ctx.lr = 0x82266950;
	sub_8209A780(ctx, base);
	// 82266950: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82266954: 57EB077B  rlwinm. r11, r31, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82266958: 41820014  beq 0x8226696c
	if ctx.cr[0].eq {
	pc = 0x8226696C; continue 'dispatch;
	}
	// 8226695C: 3BC00200  li r30, 0x200
	ctx.r[30].s64 = 512;
	// 82266960: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 82266964: 9361005C  stw r27, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[27].u32 ) };
	// 82266968: 48000038  b 0x822669a0
	pc = 0x822669A0; continue 'dispatch;
            }
            0x8226696C => {
    //   block [0x8226696C..0x82266988)
	// 8226696C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82266970: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82266974: 4BE3301D  bl 0x82099990
	ctx.lr = 0x82266978;
	sub_82099990(ctx, base);
	// 82266978: 57EB07BD  rlwinm. r11, r31, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226697C: 4082000C  bne 0x82266988
	if !ctx.cr[0].eq {
	pc = 0x82266988; continue 'dispatch;
	}
	// 82266980: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82266984: 409A0008  bne cr6, 0x8226698c
	if !ctx.cr[6].eq {
	pc = 0x8226698C; continue 'dispatch;
	}
	pc = 0x82266988; continue 'dispatch;
            }
            0x82266988 => {
    //   block [0x82266988..0x8226698C)
	// 82266988: 3BC00080  li r30, 0x80
	ctx.r[30].s64 = 128;
	pc = 0x8226698C; continue 'dispatch;
            }
            0x8226698C => {
    //   block [0x8226698C..0x8226699C)
	// 8226698C: 57EB07FF  clrlwi. r11, r31, 0x1f
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82266990: 4082000C  bne 0x8226699c
	if !ctx.cr[0].eq {
	pc = 0x8226699C; continue 'dispatch;
	}
	// 82266994: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82266998: 409A0008  bne cr6, 0x822669a0
	if !ctx.cr[6].eq {
	pc = 0x822669A0; continue 'dispatch;
	}
	pc = 0x8226699C; continue 'dispatch;
            }
            0x8226699C => {
    //   block [0x8226699C..0x822669A0)
	// 8226699C: 63DE0100  ori r30, r30, 0x100
	ctx.r[30].u64 = ctx.r[30].u64 | 256;
	pc = 0x822669A0; continue 'dispatch;
            }
            0x822669A0 => {
    //   block [0x822669A0..0x822669C4)
	// 822669A0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822669A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822669A8: E9410058  ld r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 822669AC: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 822669B0: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 822669B4: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822669B8: F95D0010  std r10, 0x10(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 822669BC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 822669C0: 4BE28124  b 0x8208eae4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822669D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822669D0 size=68
    let mut pc: u32 = 0x822669D0;
    'dispatch: loop {
        match pc {
            0x822669D0 => {
    //   block [0x822669D0..0x82266A08)
	// 822669D0: 3D808205  lis r12, -0x7dfb
	ctx.r[12].s64 = -2113601536;
	// 822669D4: C88C9E80  lfd f4, -0x6180(r12)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-24960 as u32) ) };
	// 822669D8: 3D808205  lis r12, -0x7dfb
	ctx.r[12].s64 = -2113601536;
	// 822669DC: C8AC9E88  lfd f5, -0x6178(r12)
	ctx.f[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-24952 as u32) ) };
	// 822669E0: FC012000  fcmpu cr0, f1, f4
	ctx.cr[0].compare_f64(ctx.f[1].f64, ctx.f[4].f64);
	// 822669E4: 41C2002C  beq- 0x82266a10
	if ctx.cr[0].eq {
	pc = 0x82266A10; continue 'dispatch;
	}
	// 822669E8: FCC00A10  fabs f6, f1
	ctx.f[6].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 822669EC: FC062800  fcmpu cr0, f6, f5
	ctx.cr[0].compare_f64(ctx.f[6].f64, ctx.f[5].f64);
	// 822669F0: 40C00020  bge- 0x82266a10
	if !ctx.cr[0].lt {
	pc = 0x82266A10; continue 'dispatch;
	}
	// 822669F4: FC012000  fcmpu cr0, f1, f4
	ctx.cr[0].compare_f64(ctx.f[1].f64, ctx.f[4].f64);
	// 822669F8: 41800010  blt 0x82266a08
	if ctx.cr[0].lt {
	pc = 0x82266A08; continue 'dispatch;
	}
	// 822669FC: FC81282A  fadd f4, f1, f5
	ctx.f[4].f64 = ctx.f[1].f64 + ctx.f[5].f64;
	// 82266A00: FC242828  fsub f1, f4, f5
	ctx.f[1].f64 = ctx.f[4].f64 - ctx.f[5].f64;
	// 82266A04: 4800000C  b 0x82266a10
	pc = 0x82266A10; continue 'dispatch;
            }
            0x82266A08 => {
    //   block [0x82266A08..0x82266A10)
	// 82266A08: FC812828  fsub f4, f1, f5
	ctx.f[4].f64 = ctx.f[1].f64 - ctx.f[5].f64;
	// 82266A0C: FC24282A  fadd f1, f4, f5
	ctx.f[1].f64 = ctx.f[4].f64 + ctx.f[5].f64;
	pc = 0x82266A10; continue 'dispatch;
            }
            0x82266A10 => {
    //   block [0x82266A10..0x82266A14)
	// 82266A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82266A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82266A20 size=24
    let mut pc: u32 = 0x82266A20;
    'dispatch: loop {
        match pc {
            0x82266A20 => {
    //   block [0x82266A20..0x82266A38)
	// 82266A20: 3D608228  lis r11, -0x7dd8
	ctx.r[11].s64 = -2111307776;
	// 82266A24: 396B6CE0  addi r11, r11, 0x6ce0
	ctx.r[11].s64 = ctx.r[11].s64 + 27872;
	// 82266A28: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82266A2C: 908B000C  stw r4, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82266A30: 90AB0008  stw r5, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82266A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82266A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82266A40 size=92
    let mut pc: u32 = 0x82266A40;
    'dispatch: loop {
        match pc {
            0x82266A40 => {
    //   block [0x82266A40..0x82266A80)
	// 82266A40: 7CE802A6  mflr r7
	ctx.r[7].u64 = ctx.lr;
	// 82266A44: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82266A48: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82266A4C: 90E1FFA8  stw r7, -0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.r[7].u32 ) };
	// 82266A50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82266A54: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82266A58: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82266A5C: 4BFFFFC5  bl 0x82266a20
	ctx.lr = 0x82266A60;
	sub_82266A20(ctx, base);
	// 82266A60: 7C8C2378  mr r12, r4
	ctx.r[12].u64 = ctx.r[4].u64;
	// 82266A64: 93C10004  stw r30, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82266A68: 7C6803A6  mtlr r3
	ctx.lr = ctx.r[3].u64;
	// 82266A6C: 4E800021  blrl
	unsafe { crate::rt::debugtrap() }
	// 82266A70: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82266A74: 281F0100  cmplwi r31, 0x100
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82266A78: 40820008  bne 0x82266a80
	if !ctx.cr[0].eq {
	pc = 0x82266A80; continue 'dispatch;
	}
	// 82266A7C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	pc = 0x82266A80; continue 'dispatch;
            }
            0x82266A80 => {
    //   block [0x82266A80..0x82266A9C)
	// 82266A80: 4BFFFFA1  bl 0x82266a20
	ctx.lr = 0x82266A84;
	sub_82266A20(ctx, base);
	// 82266A84: 80E10008  lwz r7, 8(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(8 as u32) ) } as u64;
	// 82266A88: 7CE803A6  mtlr r7
	ctx.lr = ctx.r[7].u64;
	// 82266A8C: EBC10050  ld r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82266A90: EBE10058  ld r31, 0x58(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82266A94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82266A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82266A9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82266A9C size=192
    let mut pc: u32 = 0x82266A9C;
    'dispatch: loop {
        match pc {
            0x82266A9C => {
    //   block [0x82266A9C..0x82266B5C)
	// 82266A9C: C9C40198  lfd f14, 0x198(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[14].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(408 as u32) ) };
	// 82266AA0: C9E401A0  lfd f15, 0x1a0(r4)
	ctx.f[15].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(416 as u32) ) };
	// 82266AA4: CA0401A8  lfd f16, 0x1a8(r4)
	ctx.f[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(424 as u32) ) };
	// 82266AA8: CA2401B0  lfd f17, 0x1b0(r4)
	ctx.f[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(432 as u32) ) };
	// 82266AAC: CA4401B8  lfd f18, 0x1b8(r4)
	ctx.f[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(440 as u32) ) };
	// 82266AB0: CA6401C0  lfd f19, 0x1c0(r4)
	ctx.f[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(448 as u32) ) };
	// 82266AB4: CA8401C8  lfd f20, 0x1c8(r4)
	ctx.f[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(456 as u32) ) };
	// 82266AB8: CAA401D0  lfd f21, 0x1d0(r4)
	ctx.f[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(464 as u32) ) };
	// 82266ABC: CAC401D8  lfd f22, 0x1d8(r4)
	ctx.f[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(472 as u32) ) };
	// 82266AC0: CAE401E0  lfd f23, 0x1e0(r4)
	ctx.f[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(480 as u32) ) };
	// 82266AC4: CB0401E8  lfd f24, 0x1e8(r4)
	ctx.f[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(488 as u32) ) };
	// 82266AC8: CB2401F0  lfd f25, 0x1f0(r4)
	ctx.f[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(496 as u32) ) };
	// 82266ACC: CB4401F8  lfd f26, 0x1f8(r4)
	ctx.f[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(504 as u32) ) };
	// 82266AD0: CB640200  lfd f27, 0x200(r4)
	ctx.f[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(512 as u32) ) };
	// 82266AD4: CB840208  lfd f28, 0x208(r4)
	ctx.f[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(520 as u32) ) };
	// 82266AD8: CBA40210  lfd f29, 0x210(r4)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(528 as u32) ) };
	// 82266ADC: CBC40218  lfd f30, 0x218(r4)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(536 as u32) ) };
	// 82266AE0: CBE40220  lfd f31, 0x220(r4)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(544 as u32) ) };
	// 82266AE4: E9C40088  ld r14, 0x88(r4)
	ctx.r[14].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(136 as u32) ) };
	// 82266AE8: E9E40090  ld r15, 0x90(r4)
	ctx.r[15].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(144 as u32) ) };
	// 82266AEC: EA040098  ld r16, 0x98(r4)
	ctx.r[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(152 as u32) ) };
	// 82266AF0: EA2400A0  ld r17, 0xa0(r4)
	ctx.r[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(160 as u32) ) };
	// 82266AF4: EA4400A8  ld r18, 0xa8(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(168 as u32) ) };
	// 82266AF8: EA6400B0  ld r19, 0xb0(r4)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(176 as u32) ) };
	// 82266AFC: EA8400B8  ld r20, 0xb8(r4)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(184 as u32) ) };
	// 82266B00: EAA400C0  ld r21, 0xc0(r4)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(192 as u32) ) };
	// 82266B04: EAC400C8  ld r22, 0xc8(r4)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(200 as u32) ) };
	// 82266B08: EAE400D0  ld r23, 0xd0(r4)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(208 as u32) ) };
	// 82266B0C: EB0400D8  ld r24, 0xd8(r4)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(216 as u32) ) };
	// 82266B10: EB2400E0  ld r25, 0xe0(r4)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(224 as u32) ) };
	// 82266B14: EB4400E8  ld r26, 0xe8(r4)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(232 as u32) ) };
	// 82266B18: EB6400F0  ld r27, 0xf0(r4)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(240 as u32) ) };
	// 82266B1C: EB8400F8  ld r28, 0xf8(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(248 as u32) ) };
	// 82266B20: EBA40100  ld r29, 0x100(r4)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(256 as u32) ) };
	// 82266B24: EBC40108  ld r30, 0x108(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(264 as u32) ) };
	// 82266B28: EBE40110  ld r31, 0x110(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(272 as u32) ) };
	// 82266B2C: C8040120  lfd f0, 0x120(r4)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(288 as u32) ) };
	// 82266B30: 80A40118  lwz r5, 0x118(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(280 as u32) ) } as u64;
	// 82266B34: 80C4011C  lwz r6, 0x11c(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(284 as u32) ) } as u64;
	// 82266B38: E8E40010  ld r7, 0x10(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	// 82266B3C: FDFE058E  mtfsf 0xff, f0
	ctx.fpscr.store_from_guest(ctx.f[0].u32);
	// 82266B40: 7C6803A6  mtlr r3
	ctx.lr = ctx.r[3].u64;
	// 82266B44: 7CAFF120  mtcrf 0xff, r5
	// mtcrf 0xFF, ctx.r[5]: CR update elided (TODO: implement MTCRF semantics)
	// 82266B48: 7CC103A6  mtxer r6
	ctx.xer.so = (ctx.r[6].u64 & 0x8000_0000) != 0;
	ctx.xer.ov = (ctx.r[6].u64 & 0x4000_0000) != 0;
	ctx.xer.ca = (ctx.r[6].u64 & 0x2000_0000) != 0;
	// 82266B4C: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82266B50: E8440028  ld r2, 0x28(r4)
	ctx.r[2].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	// 82266B54: E8240020  ld r1, 0x20(r4)
	ctx.r[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	// 82266B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82266B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82266B60 size=24
    let mut pc: u32 = 0x82266B60;
    'dispatch: loop {
        match pc {
            0x82266B60 => {
    //   block [0x82266B60..0x82266B70)
	// 82266B60: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82266B64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82266B68: 409A0008  bne cr6, 0x82266b70
	if !ctx.cr[6].eq {
	pc = 0x82266B70; continue 'dispatch;
	}
	// 82266B6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82266B70; continue 'dispatch;
            }
            0x82266B70 => {
    //   block [0x82266B70..0x82266B78)
	// 82266B70: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82266B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82266B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82266B80 size=164
    let mut pc: u32 = 0x82266B80;
    'dispatch: loop {
        match pc {
            0x82266B80 => {
    //   block [0x82266B80..0x82266C24)
	// 82266B80: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82266B84: 7FE802A6  mflr r31
	ctx.r[31].u64 = ctx.lr;
	// 82266B88: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82266B8C: FBA1FFE8  std r29, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[29].u64 ) };
	// 82266B90: FB81FFE0  std r28, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[28].u64 ) };
	// 82266B94: 9421EB10  stwu r1, -0x14f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-5360 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82266B98: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82266B9C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82266BA0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82266BA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82266BA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82266BAC: 38A00A40  li r5, 0xa40
	ctx.r[5].s64 = 2624;
	// 82266BB0: 4BE28641  bl 0x8208f1f0
	ctx.lr = 0x82266BB4;
	sub_8208F1F0(ctx, base);
	// 82266BB4: 38610A90  addi r3, r1, 0xa90
	ctx.r[3].s64 = ctx.r[1].s64 + 2704;
	// 82266BB8: 48003C05  bl 0x8226a7bc
	ctx.lr = 0x82266BBC;
	// extern call 0x8226A7BC → crate::xboxkrnl::RtlCaptureContext
	crate::xboxkrnl::RtlCaptureContext(ctx, base);
	// 82266BBC: 3C808226  lis r4, -0x7dda
	ctx.r[4].s64 = -2111438848;
	// 82266BC0: 38846BE4  addi r4, r4, 0x6be4
	ctx.r[4].s64 = ctx.r[4].s64 + 27620;
	// 82266BC4: 90810A98  stw r4, 0xa98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(2712 as u32), ctx.r[4].u32 ) };
	// 82266BC8: 4BFFC9C1  bl 0x82263588
	ctx.lr = 0x82266BCC;
	sub_82263588(ctx, base);
	// 82266BCC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82266BD0: 3C808226  lis r4, -0x7dda
	ctx.r[4].s64 = -2111438848;
	// 82266BD4: 38846BE4  addi r4, r4, 0x6be4
	ctx.r[4].s64 = ctx.r[4].s64 + 27620;
	// 82266BD8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82266BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82266BE0: 48003BCD  bl 0x8226a7ac
	ctx.lr = 0x82266BE4;
	// extern call 0x8226A7AC → crate::xboxkrnl::RtlUnwind
	crate::xboxkrnl::RtlUnwind(ctx, base);
	// 82266BE4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82266BE8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82266BEC: 38A00A40  li r5, 0xa40
	ctx.r[5].s64 = 2624;
	// 82266BF0: 4BE28601  bl 0x8208f1f0
	ctx.lr = 0x82266BF4;
	sub_8208F1F0(ctx, base);
	// 82266BF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82266BF8: 4BFFC991  bl 0x82263588
	ctx.lr = 0x82266BFC;
	sub_82263588(ctx, base);
	// 82266BFC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82266C00: 548507FA  rlwinm r5, r4, 0, 0x1f, 0x1d
	ctx.r[5].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82266C04: 90BE0004  stw r5, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82266C08: 7FE803A6  mtlr r31
	ctx.lr = ctx.r[31].u64;
	// 82266C0C: EB8114D0  ld r28, 0x14d0(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(5328 as u32) ) };
	// 82266C10: EBA114D8  ld r29, 0x14d8(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(5336 as u32) ) };
	// 82266C14: EBC114E0  ld r30, 0x14e0(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(5344 as u32) ) };
	// 82266C18: EBE114E8  ld r31, 0x14e8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(5352 as u32) ) };
	// 82266C1C: 382114F0  addi r1, r1, 0x14f0
	ctx.r[1].s64 = ctx.r[1].s64 + 5360;
	// 82266C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82266C2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82266C2C size=76
    let mut pc: u32 = 0x82266C2C;
    'dispatch: loop {
        match pc {
            0x82266C2C => {
    //   block [0x82266C2C..0x82266C78)
	// 82266C2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82266C30: 7FE802A6  mflr r31
	ctx.r[31].u64 = ctx.lr;
	// 82266C34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82266C38: 91410008  stw r10, 8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82266C3C: 4BFFF0B5  bl 0x82265cf0
	ctx.lr = 0x82266C40;
	sub_82265CF0(ctx, base);
	// 82266C40: 7FE803A6  mtlr r31
	ctx.lr = ctx.r[31].u64;
	// 82266C44: EBE10050  ld r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82266C48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82266C4C: 4E800020  blr
	return;
	// 82266C50: FBE1FFB8  std r31, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.r[31].u64 ) };
	// 82266C54: 7FE802A6  mflr r31
	ctx.r[31].u64 = ctx.lr;
	// 82266C58: 9421FFB0  stwu r1, -0x50(r1)
	ea = ctx.r[1].u32.wrapping_add(-80 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82266C5C: 80A4FFA8  lwz r5, -0x58(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-88 as u32) ) } as u64;
	// 82266C60: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82266C64: 4BFFEF95  bl 0x82265bf8
	ctx.lr = 0x82266C68;
	sub_82265BF8(ctx, base);
	// 82266C68: 7FE803A6  mtlr r31
	ctx.lr = ctx.r[31].u64;
	// 82266C6C: EBE10008  ld r31, 8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(8 as u32) ) };
	// 82266C70: 38210050  addi r1, r1, 0x50
	ctx.r[1].s64 = ctx.r[1].s64 + 80;
	// 82266C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82266C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82266C78 size=156
    let mut pc: u32 = 0x82266C78;
    'dispatch: loop {
        match pc {
            0x82266C78 => {
    //   block [0x82266C78..0x82266C94)
	// 82266C78: 80C30000  lwz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82266C7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82266C80: 54CB0421  rlwinm. r11, r6, 0, 0x10, 0x10
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82266C84: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
	// 82266C88: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82266C8C: 54C8843E  srwi r8, r6, 0x10
	ctx.r[8].u32 = ctx.r[6].u32.wrapping_shr(16);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82266C90: 54C7653E  srwi r7, r6, 0x14
	ctx.r[7].u32 = ctx.r[6].u32.wrapping_shr(20);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	pc = 0x82266C94; continue 'dispatch;
            }
            0x82266C94 => {
    //   block [0x82266C94..0x82266CE8)
	// 82266C94: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82266C98: 7D6B4830  slw r11, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82266C9C: 7D6A4038  and r10, r11, r8
	ctx.r[10].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 82266CA0: 7CEB5838  and r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 & ctx.r[11].u64;
	// 82266CA4: 554A073E  clrlwi r10, r10, 0x1c
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 82266CA8: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82266CAC: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82266CB0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82266CB4: 554AE7BC  rlwinm r10, r10, 0x1c, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 82266CB8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82266CBC: 694A0002  xori r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u64 ^ 2;
	// 82266CC0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82266CC4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82266CC8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82266CCC: 4198002C  blt cr6, 0x82266cf8
	if ctx.cr[6].lt {
	pc = 0x82266CF8; continue 'dispatch;
	}
	// 82266CD0: 419A0020  beq cr6, 0x82266cf0
	if ctx.cr[6].eq {
	pc = 0x82266CF0; continue 'dispatch;
	}
	// 82266CD4: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82266CD8: 41980010  blt cr6, 0x82266ce8
	if ctx.cr[6].lt {
	pc = 0x82266CE8; continue 'dispatch;
	}
	// 82266CDC: 409A0028  bne cr6, 0x82266d04
	if !ctx.cr[6].eq {
	pc = 0x82266D04; continue 'dispatch;
	}
	// 82266CE0: 60630008  ori r3, r3, 8
	ctx.r[3].u64 = ctx.r[3].u64 | 8;
	// 82266CE4: 48000020  b 0x82266d04
	pc = 0x82266D04; continue 'dispatch;
            }
            0x82266CE8 => {
    //   block [0x82266CE8..0x82266CF0)
	// 82266CE8: 60630001  ori r3, r3, 1
	ctx.r[3].u64 = ctx.r[3].u64 | 1;
	// 82266CEC: 48000018  b 0x82266d04
	pc = 0x82266D04; continue 'dispatch;
            }
            0x82266CF0 => {
    //   block [0x82266CF0..0x82266CF8)
	// 82266CF0: 60630002  ori r3, r3, 2
	ctx.r[3].u64 = ctx.r[3].u64 | 2;
	// 82266CF4: 48000010  b 0x82266d04
	pc = 0x82266D04; continue 'dispatch;
            }
            0x82266CF8 => {
    //   block [0x82266CF8..0x82266D04)
	// 82266CF8: 54CB0463  rlwinm. r11, r6, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82266CFC: 41820008  beq 0x82266d04
	if ctx.cr[0].eq {
	pc = 0x82266D04; continue 'dispatch;
	}
	// 82266D00: 60630004  ori r3, r3, 4
	ctx.r[3].u64 = ctx.r[3].u64 | 4;
	pc = 0x82266D04; continue 'dispatch;
            }
            0x82266D04 => {
    //   block [0x82266D04..0x82266D14)
	// 82266D04: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82266D08: 2F090004  cmpwi cr6, r9, 4
	ctx.cr[6].compare_i32(ctx.r[9].s32, 4, &mut ctx.xer);
	// 82266D0C: 4198FF88  blt cr6, 0x82266c94
	if ctx.cr[6].lt {
	pc = 0x82266C94; continue 'dispatch;
	}
	// 82266D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82266D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82266D18 size=112
    let mut pc: u32 = 0x82266D18;
    'dispatch: loop {
        match pc {
            0x82266D18 => {
    //   block [0x82266D18..0x82266D5C)
	// 82266D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82266D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82266D20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82266D24: 89630008  lbz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82266D28: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82266D2C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82266D30: 409A0044  bne cr6, 0x82266d74
	if !ctx.cr[6].eq {
	pc = 0x82266D74; continue 'dispatch;
	}
	// 82266D34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82266D38: 556A0421  rlwinm. r10, r11, 0, 0x10, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82266D3C: 4182002C  beq 0x82266d68
	if ctx.cr[0].eq {
	pc = 0x82266D68; continue 'dispatch;
	}
	// 82266D40: 4BFFFF39  bl 0x82266c78
	ctx.lr = 0x82266D44;
	sub_82266C78(ctx, base);
	// 82266D44: 546B07FF  clrlwi. r11, r3, 0x1f
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82266D48: 4082002C  bne 0x82266d74
	if !ctx.cr[0].eq {
	pc = 0x82266D74; continue 'dispatch;
	}
	// 82266D4C: 546B07BD  rlwinm. r11, r3, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82266D50: 4182000C  beq 0x82266d5c
	if ctx.cr[0].eq {
	pc = 0x82266D5C; continue 'dispatch;
	}
	// 82266D54: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82266D58: 48000020  b 0x82266d78
	pc = 0x82266D78; continue 'dispatch;
            }
            0x82266D5C => {
    //   block [0x82266D5C..0x82266D68)
	// 82266D5C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82266D60: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82266D64: 48000014  b 0x82266d78
	pc = 0x82266D78; continue 'dispatch;
            }
            0x82266D68 => {
    //   block [0x82266D68..0x82266D74)
	// 82266D68: 556B031F  rlwinm. r11, r11, 0, 0xc, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82266D6C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82266D70: 41820008  beq 0x82266d78
	if ctx.cr[0].eq {
	pc = 0x82266D78; continue 'dispatch;
	}
	pc = 0x82266D74; continue 'dispatch;
            }
            0x82266D74 => {
    //   block [0x82266D74..0x82266D78)
	// 82266D74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82266D78; continue 'dispatch;
            }
            0x82266D78 => {
    //   block [0x82266D78..0x82266D88)
	// 82266D78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82266D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82266D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82266D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82266D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82266D88 size=252
    let mut pc: u32 = 0x82266D88;
    'dispatch: loop {
        match pc {
            0x82266D88 => {
    //   block [0x82266D88..0x82266E0C)
	// 82266D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82266D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82266D90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82266D94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82266D98: 9421F790  stwu r1, -0x870(r1)
	ea = ctx.r[1].u32.wrapping_add(-2160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82266D9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82266DA0: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82266DA4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82266DA8: 419A00C4  beq cr6, 0x82266e6c
	if ctx.cr[6].eq {
	pc = 0x82266E6C; continue 'dispatch;
	}
	// 82266DAC: 817F202C  lwz r11, 0x202c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8236 as u32) ) } as u64;
	// 82266DB0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82266DB4: 419800B8  blt cr6, 0x82266e6c
	if ctx.cr[6].lt {
	pc = 0x82266E6C; continue 'dispatch;
	}
	// 82266DB8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82266DBC: 38800800  li r4, 0x800
	ctx.r[4].s64 = 2048;
	// 82266DC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82266DC4: 48003519  bl 0x8226a2dc
	ctx.lr = 0x82266DC8;
	// extern call 0x8226A2DC → crate::xboxkrnl::_vsnprintf
	crate::xboxkrnl::_vsnprintf(ctx, base);
	// 82266DC8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82266DCC: 408100A0  ble 0x82266e6c
	if !ctx.cr[0].gt {
	pc = 0x82266E6C; continue 'dispatch;
	}
	// 82266DD0: 897F2029  lbz r11, 0x2029(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8233 as u32) ) } as u64;
	// 82266DD4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82266DD8: 41820070  beq 0x82266e48
	if ctx.cr[0].eq {
	pc = 0x82266E48; continue 'dispatch;
	}
	// 82266DDC: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82266DE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82266DE4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82266DE8: 995F2029  stb r10, 0x2029(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8233 as u32), ctx.r[10].u8 ) };
	// 82266DEC: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 82266DF0: 419A0024  beq cr6, 0x82266e14
	if ctx.cr[6].eq {
	pc = 0x82266E14; continue 'dispatch;
	}
	// 82266DF4: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 82266DF8: 419A001C  beq cr6, 0x82266e14
	if ctx.cr[6].eq {
	pc = 0x82266E14; continue 'dispatch;
	}
	// 82266DFC: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 82266E00: 4198000C  blt cr6, 0x82266e0c
	if ctx.cr[6].lt {
	pc = 0x82266E0C; continue 'dispatch;
	}
	// 82266E04: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 82266E08: 4099000C  ble cr6, 0x82266e14
	if !ctx.cr[6].gt {
	pc = 0x82266E14; continue 'dispatch;
	}
	pc = 0x82266E0C; continue 'dispatch;
            }
            0x82266E0C => {
    //   block [0x82266E0C..0x82266E14)
	// 82266E0C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82266E10: 48000008  b 0x82266e18
	pc = 0x82266E18; continue 'dispatch;
            }
            0x82266E14 => {
    //   block [0x82266E14..0x82266E18)
	// 82266E14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82266E18; continue 'dispatch;
            }
            0x82266E18 => {
    //   block [0x82266E18..0x82266E48)
	// 82266E18: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82266E1C: 4082002C  bne 0x82266e48
	if !ctx.cr[0].eq {
	pc = 0x82266E48; continue 'dispatch;
	}
	// 82266E20: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82266E24: 807F201C  lwz r3, 0x201c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8220 as u32) ) } as u64;
	// 82266E28: 815F2018  lwz r10, 0x2018(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8216 as u32) ) } as u64;
	// 82266E2C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82266E30: 388BAF40  addi r4, r11, -0x50c0
	ctx.r[4].s64 = ctx.r[11].s64 + -20672;
	// 82266E34: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82266E38: 4E800421  bctrl
	ctx.lr = 0x82266E3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82266E3C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82266E40: 40800008  bge 0x82266e48
	if !ctx.cr[0].lt {
	pc = 0x82266E48; continue 'dispatch;
	}
	// 82266E44: 907F202C  stw r3, 0x202c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8236 as u32), ctx.r[3].u32 ) };
            }
            0x82266E48 => {
    //   block [0x82266E48..0x82266E6C)
	// 82266E48: 807F201C  lwz r3, 0x201c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8220 as u32) ) } as u64;
	// 82266E4C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82266E50: 817F2018  lwz r11, 0x2018(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8216 as u32) ) } as u64;
	// 82266E54: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82266E58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82266E5C: 4E800421  bctrl
	ctx.lr = 0x82266E60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82266E60: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82266E64: 40800008  bge 0x82266e6c
	if !ctx.cr[0].lt {
	pc = 0x82266E6C; continue 'dispatch;
	}
	// 82266E68: 907F202C  stw r3, 0x202c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8236 as u32), ctx.r[3].u32 ) };
            }
            0x82266E6C => {
    //   block [0x82266E6C..0x82266E84)
	// 82266E6C: 38210870  addi r1, r1, 0x870
	ctx.r[1].s64 = ctx.r[1].s64 + 2160;
	// 82266E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82266E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82266E78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82266E7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82266E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82266E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82266E88 size=80
    let mut pc: u32 = 0x82266E88;
    'dispatch: loop {
        match pc {
            0x82266E88 => {
    //   block [0x82266E88..0x82266ED8)
	// 82266E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82266E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82266E90: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82266E94: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82266E98: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82266E9C: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82266EA0: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82266EA4: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82266EA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82266EAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82266EB0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82266EB4: 99632028  stb r11, 0x2028(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8232 as u32), ctx.r[11].u8 ) };
	// 82266EB8: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82266EBC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82266EC0: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82266EC4: 4BFFFEC5  bl 0x82266d88
	ctx.lr = 0x82266EC8;
	sub_82266D88(ctx, base);
	// 82266EC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82266ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82266ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82266ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82266ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82266ED8 size=176
    let mut pc: u32 = 0x82266ED8;
    'dispatch: loop {
        match pc {
            0x82266ED8 => {
    //   block [0x82266ED8..0x82266F14)
	// 82266ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82266EDC: 4BE27BB9  bl 0x8208ea94
	ctx.lr = 0x82266EE0;
	sub_8208EA60(ctx, base);
	// 82266EE0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82266EE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82266EE8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82266EEC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82266EF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82266EF4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82266EF8: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82266EFC: 3BABFE08  addi r29, r11, -0x1f8
	ctx.r[29].s64 = ctx.r[11].s64 + -504;
	// 82266F00: 3B8AFE10  addi r28, r10, -0x1f0
	ctx.r[28].s64 = ctx.r[10].s64 + -496;
	// 82266F04: 5529D7FF  rlwinm. r9, r9, 0x1a, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82266F08: 4082000C  bne 0x82266f14
	if !ctx.cr[0].eq {
	pc = 0x82266F14; continue 'dispatch;
	}
	// 82266F0C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82266F10: 419A003C  beq cr6, 0x82266f4c
	if ctx.cr[6].eq {
	pc = 0x82266F4C; continue 'dispatch;
	}
	pc = 0x82266F14; continue 'dispatch;
            }
            0x82266F14 => {
    //   block [0x82266F14..0x82266F2C)
	// 82266F14: 54CB063F  clrlwi. r11, r6, 0x18
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82266F18: 41820014  beq 0x82266f2c
	if ctx.cr[0].eq {
	pc = 0x82266F2C; continue 'dispatch;
	}
	// 82266F1C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82266F20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82266F24: 388BAC70  addi r4, r11, -0x5390
	ctx.r[4].s64 = ctx.r[11].s64 + -21392;
	// 82266F28: 4BFFFF61  bl 0x82266e88
	ctx.lr = 0x82266F2C;
	sub_82266E88(ctx, base);
	pc = 0x82266F2C; continue 'dispatch;
            }
            0x82266F2C => {
    //   block [0x82266F2C..0x82266F3C)
	// 82266F2C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82266F30: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82266F34: 409A0008  bne cr6, 0x82266f3c
	if !ctx.cr[6].eq {
	pc = 0x82266F3C; continue 'dispatch;
	}
	// 82266F38: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	pc = 0x82266F3C; continue 'dispatch;
            }
            0x82266F3C => {
    //   block [0x82266F3C..0x82266F4C)
	// 82266F3C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82266F40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82266F44: 388BA094  addi r4, r11, -0x5f6c
	ctx.r[4].s64 = ctx.r[11].s64 + -24428;
	// 82266F48: 4BFFFF41  bl 0x82266e88
	ctx.lr = 0x82266F4C;
	sub_82266E88(ctx, base);
	pc = 0x82266F4C; continue 'dispatch;
            }
            0x82266F4C => {
    //   block [0x82266F4C..0x82266F60)
	// 82266F4C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82266F50: 556BD7FF  rlwinm. r11, r11, 0x1a, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82266F54: 4082000C  bne 0x82266f60
	if !ctx.cr[0].eq {
	pc = 0x82266F60; continue 'dispatch;
	}
	// 82266F58: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82266F5C: 409A0024  bne cr6, 0x82266f80
	if !ctx.cr[6].eq {
	pc = 0x82266F80; continue 'dispatch;
	}
	pc = 0x82266F60; continue 'dispatch;
            }
            0x82266F60 => {
    //   block [0x82266F60..0x82266F70)
	// 82266F60: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82266F64: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82266F68: 409A0008  bne cr6, 0x82266f70
	if !ctx.cr[6].eq {
	pc = 0x82266F70; continue 'dispatch;
	}
	// 82266F6C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	pc = 0x82266F70; continue 'dispatch;
            }
            0x82266F70 => {
    //   block [0x82266F70..0x82266F80)
	// 82266F70: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82266F74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82266F78: 388BA078  addi r4, r11, -0x5f88
	ctx.r[4].s64 = ctx.r[11].s64 + -24456;
	// 82266F7C: 4BFFFF0D  bl 0x82266e88
	ctx.lr = 0x82266F80;
	sub_82266E88(ctx, base);
	pc = 0x82266F80; continue 'dispatch;
            }
            0x82266F80 => {
    //   block [0x82266F80..0x82266F88)
	// 82266F80: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82266F84: 4BE27B60  b 0x8208eae4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82266F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82266F88 size=84
    let mut pc: u32 = 0x82266F88;
    'dispatch: loop {
        match pc {
            0x82266F88 => {
    //   block [0x82266F88..0x82266FC8)
	// 82266F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82266F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82266F90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82266F94: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82266F98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82266F9C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82266FA0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82266FA4: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82266FA8: 41820020  beq 0x82266fc8
	if ctx.cr[0].eq {
	pc = 0x82266FC8; continue 'dispatch;
	}
	// 82266FAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82266FB0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82266FB4: 997F2029  stb r11, 0x2029(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8233 as u32), ctx.r[11].u8 ) };
	// 82266FB8: 388A91C0  addi r4, r10, -0x6e40
	ctx.r[4].s64 = ctx.r[10].s64 + -28224;
	// 82266FBC: 4BFFFECD  bl 0x82266e88
	ctx.lr = 0x82266FC0;
	sub_82266E88(ctx, base);
	// 82266FC0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82266FC4: 997F2029  stb r11, 0x2029(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8233 as u32), ctx.r[11].u8 ) };
	pc = 0x82266FC8; continue 'dispatch;
            }
            0x82266FC8 => {
    //   block [0x82266FC8..0x82266FDC)
	// 82266FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82266FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82266FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82266FD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82266FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82266FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82266FE0 size=108
    let mut pc: u32 = 0x82266FE0;
    'dispatch: loop {
        match pc {
            0x82266FE0 => {
    //   block [0x82266FE0..0x82267018)
	// 82266FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82266FE4: 4BE27AB9  bl 0x8208ea9c
	ctx.lr = 0x82266FE8;
	sub_8208EA60(ctx, base);
	// 82266FE8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82266FEC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82266FF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82266FF4: 388B5820  addi r4, r11, 0x5820
	ctx.r[4].s64 = ctx.r[11].s64 + 22560;
	// 82266FF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82266FFC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82267000: 4BFFFF89  bl 0x82266f88
	ctx.lr = 0x82267004;
	sub_82266F88(ctx, base);
	// 82267004: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82267008: 41820010  beq 0x82267018
	if ctx.cr[0].eq {
	pc = 0x82267018; continue 'dispatch;
	}
	// 8226700C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82267010: 38CBA0B0  addi r6, r11, -0x5f50
	ctx.r[6].s64 = ctx.r[11].s64 + -24400;
	// 82267014: 4800000C  b 0x82267020
	pc = 0x82267020; continue 'dispatch;
            }
            0x82267018 => {
    //   block [0x82267018..0x82267020)
	// 82267018: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8226701C: 38CB0F75  addi r6, r11, 0xf75
	ctx.r[6].s64 = ctx.r[11].s64 + 3957;
	pc = 0x82267020; continue 'dispatch;
            }
            0x82267020 => {
    //   block [0x82267020..0x8226704C)
	// 82267020: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82267024: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82267028: 388BC25C  addi r4, r11, -0x3da4
	ctx.r[4].s64 = ctx.r[11].s64 + -15780;
	// 8226702C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82267030: 4BFFFE59  bl 0x82266e88
	ctx.lr = 0x82267034;
	sub_82266E88(ctx, base);
	// 82267034: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82267038: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226703C: 388B5828  addi r4, r11, 0x5828
	ctx.r[4].s64 = ctx.r[11].s64 + 22568;
	// 82267040: 4BFFFF49  bl 0x82266f88
	ctx.lr = 0x82267044;
	sub_82266F88(ctx, base);
	// 82267044: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82267048: 4BE27AA4  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82267050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82267050 size=100
    let mut pc: u32 = 0x82267050;
    'dispatch: loop {
        match pc {
            0x82267050 => {
    //   block [0x82267050..0x822670B4)
	// 82267050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82267054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82267058: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8226705C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82267060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82267064: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82267068: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8226706C: 388BA0B8  addi r4, r11, -0x5f48
	ctx.r[4].s64 = ctx.r[11].s64 + -24392;
	// 82267070: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82267074: 4BFFFF15  bl 0x82266f88
	ctx.lr = 0x82267078;
	sub_82266F88(ctx, base);
	// 82267078: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8226707C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82267080: 388B91C0  addi r4, r11, -0x6e40
	ctx.r[4].s64 = ctx.r[11].s64 + -28224;
	// 82267084: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82267088: 4BFFFE01  bl 0x82266e88
	ctx.lr = 0x8226708C;
	sub_82266E88(ctx, base);
	// 8226708C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82267090: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82267094: 388B5828  addi r4, r11, 0x5828
	ctx.r[4].s64 = ctx.r[11].s64 + 22568;
	// 82267098: 4BFFFEF1  bl 0x82266f88
	ctx.lr = 0x8226709C;
	sub_82266F88(ctx, base);
	// 8226709C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822670A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822670A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822670A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822670AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822670B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822670B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822670B8 size=80
    let mut pc: u32 = 0x822670B8;
    'dispatch: loop {
        match pc {
            0x822670B8 => {
    //   block [0x822670B8..0x822670F8)
	// 822670B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822670BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822670C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822670C4: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 822670C8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 822670CC: 554A06B5  rlwinm. r10, r10, 0, 0x1a, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822670D0: 41820028  beq 0x822670f8
	if ctx.cr[0].eq {
	pc = 0x822670F8; continue 'dispatch;
	}
	// 822670D4: 7D6A0E70  srawi r10, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 822670D8: 3D208205  lis r9, -0x7dfb
	ctx.r[9].s64 = -2113601536;
	// 822670DC: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 822670E0: 7D680E70  srawi r8, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 822670E4: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822670E8: 3889A0C0  addi r4, r9, -0x5f40
	ctx.r[4].s64 = ctx.r[9].s64 + -24384;
	// 822670EC: 7CA80194  addze r5, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[5].s64 = tmp.s64;
	// 822670F0: 7CCA5850  subf r6, r10, r11
	ctx.r[6].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 822670F4: 4BFFFD95  bl 0x82266e88
	ctx.lr = 0x822670F8;
	sub_82266E88(ctx, base);
	pc = 0x822670F8; continue 'dispatch;
            }
            0x822670F8 => {
    //   block [0x822670F8..0x82267108)
	// 822670F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822670FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82267100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82267104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82267108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82267108 size=68
    let mut pc: u32 = 0x82267108;
    'dispatch: loop {
        match pc {
            0x82267108 => {
    //   block [0x82267108..0x8226714C)
	// 82267108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226710C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82267110: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82267114: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82267118: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8226711C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82267120: 388B5830  addi r4, r11, 0x5830
	ctx.r[4].s64 = ctx.r[11].s64 + 22576;
	// 82267124: 4BFFFE65  bl 0x82266f88
	ctx.lr = 0x82267128;
	sub_82266F88(ctx, base);
	// 82267128: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8226712C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82267130: 388BA0F0  addi r4, r11, -0x5f10
	ctx.r[4].s64 = ctx.r[11].s64 + -24336;
	// 82267134: 4BFFFD55  bl 0x82266e88
	ctx.lr = 0x82267138;
	sub_82266E88(ctx, base);
	// 82267138: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226713C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82267140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82267144: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82267148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82267150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82267150 size=884
    let mut pc: u32 = 0x82267150;
    'dispatch: loop {
        match pc {
            0x82267150 => {
    //   block [0x82267150..0x822671A0)
	// 82267150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82267154: 4BE27929  bl 0x8208ea7c
	ctx.lr = 0x82267158;
	sub_8208EA60(ctx, base);
	// 82267158: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226715C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82267160: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82267164: 388AAF40  addi r4, r10, -0x50c0
	ctx.r[4].s64 = ctx.r[10].s64 + -20672;
	// 82267168: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8226716C: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82267170: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82267174: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82267178: 7D164378  mr r22, r8
	ctx.r[22].u64 = ctx.r[8].u64;
	// 8226717C: 7D374B78  mr r23, r9
	ctx.r[23].u64 = ctx.r[9].u64;
	// 82267180: 557F06BE  clrlwi r31, r11, 0x1a
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82267184: 4BFFFD05  bl 0x82266e88
	ctx.lr = 0x82267188;
	sub_82266E88(ctx, base);
	// 82267188: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 8226718C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82267190: 409A0010  bne cr6, 0x822671a0
	if !ctx.cr[6].eq {
	pc = 0x822671A0; continue 'dispatch;
	}
	// 82267194: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82267198: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8226719C: 409A0008  bne cr6, 0x822671a4
	if !ctx.cr[6].eq {
	pc = 0x822671A4; continue 'dispatch;
	}
	pc = 0x822671A0; continue 'dispatch;
            }
            0x822671A0 => {
    //   block [0x822671A0..0x822671A4)
	// 822671A0: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	pc = 0x822671A4; continue 'dispatch;
            }
            0x822671A4 => {
    //   block [0x822671A4..0x822671C0)
	// 822671A4: 557C063E  clrlwi r28, r11, 0x18
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 822671A8: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 822671AC: 409A001C  bne cr6, 0x822671c8
	if !ctx.cr[6].eq {
	pc = 0x822671C8; continue 'dispatch;
	}
	// 822671B0: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822671B4: 419A000C  beq cr6, 0x822671c0
	if ctx.cr[6].eq {
	pc = 0x822671C0; continue 'dispatch;
	}
	// 822671B8: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 822671BC: 409A000C  bne cr6, 0x822671c8
	if !ctx.cr[6].eq {
	pc = 0x822671C8; continue 'dispatch;
	}
	pc = 0x822671C0; continue 'dispatch;
            }
            0x822671C0 => {
    //   block [0x822671C0..0x822671C8)
	// 822671C0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822671C4: 48000008  b 0x822671cc
	pc = 0x822671CC; continue 'dispatch;
            }
            0x822671C8 => {
    //   block [0x822671C8..0x822671CC)
	// 822671C8: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	pc = 0x822671CC; continue 'dispatch;
            }
            0x822671CC => {
    //   block [0x822671CC..0x8226720C)
	// 822671CC: 557B063E  clrlwi r27, r11, 0x18
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 822671D0: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 822671D4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822671D8: 419A00AC  beq cr6, 0x82267284
	if ctx.cr[6].eq {
	pc = 0x82267284; continue 'dispatch;
	}
	// 822671DC: 2F1F0020  cmpwi cr6, r31, 0x20
	ctx.cr[6].compare_i32(ctx.r[31].s32, 32, &mut ctx.xer);
	// 822671E0: 419A0098  beq cr6, 0x82267278
	if ctx.cr[6].eq {
	pc = 0x82267278; continue 'dispatch;
	}
	// 822671E4: 40990044  ble cr6, 0x82267228
	if !ctx.cr[6].gt {
	pc = 0x82267228; continue 'dispatch;
	}
	// 822671E8: 2F1F0025  cmpwi cr6, r31, 0x25
	ctx.cr[6].compare_i32(ctx.r[31].s32, 37, &mut ctx.xer);
	// 822671EC: 4099002C  ble cr6, 0x82267218
	if !ctx.cr[6].gt {
	pc = 0x82267218; continue 'dispatch;
	}
	// 822671F0: 2F1F003E  cmpwi cr6, r31, 0x3e
	ctx.cr[6].compare_i32(ctx.r[31].s32, 62, &mut ctx.xer);
	// 822671F4: 419A0018  beq cr6, 0x8226720c
	if ctx.cr[6].eq {
	pc = 0x8226720C; continue 'dispatch;
	}
	// 822671F8: 2F1F003F  cmpwi cr6, r31, 0x3f
	ctx.cr[6].compare_i32(ctx.r[31].s32, 63, &mut ctx.xer);
	// 822671FC: 409A002C  bne cr6, 0x82267228
	if !ctx.cr[6].eq {
	pc = 0x82267228; continue 'dispatch;
	}
	// 82267200: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82267204: 38ABC77C  addi r5, r11, -0x3884
	ctx.r[5].s64 = ctx.r[11].s64 + -14468;
	// 82267208: 48000088  b 0x82267290
	pc = 0x82267290; continue 'dispatch;
            }
            0x8226720C => {
    //   block [0x8226720C..0x82267218)
	// 8226720C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82267210: 38ABC784  addi r5, r11, -0x387c
	ctx.r[5].s64 = ctx.r[11].s64 + -14460;
	// 82267214: 4800007C  b 0x82267290
	pc = 0x82267290; continue 'dispatch;
            }
            0x82267218 => {
    //   block [0x82267218..0x82267228)
	// 82267218: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8226721C: 3BDFFFDF  addi r30, r31, -0x21
	ctx.r[30].s64 = ctx.r[31].s64 + -33;
	// 82267220: 38AB1B98  addi r5, r11, 0x1b98
	ctx.r[5].s64 = ctx.r[11].s64 + 7064;
	// 82267224: 4800006C  b 0x82267290
	pc = 0x82267290; continue 'dispatch;
            }
            0x82267228 => {
    //   block [0x82267228..0x82267250)
	// 82267228: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 8226722C: 556BEFFF  rlwinm. r11, r11, 0x1d, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82267230: 4182003C  beq 0x8226726c
	if ctx.cr[0].eq {
	pc = 0x8226726C; continue 'dispatch;
	}
	// 82267234: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82267238: 556A277E  rlwinm r10, r11, 4, 0x1d, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0FFFFFFFu64;
	// 8226723C: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82267240: 40980010  bge cr6, 0x82267250
	if !ctx.cr[6].lt {
	pc = 0x82267250; continue 'dispatch;
	}
	// 82267244: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82267248: 38ABC764  addi r5, r11, -0x389c
	ctx.r[5].s64 = ctx.r[11].s64 + -14492;
	// 8226724C: 48000040  b 0x8226728c
	pc = 0x8226728C; continue 'dispatch;
            }
            0x82267250 => {
    //   block [0x82267250..0x8226726C)
	// 82267250: 2F1F003D  cmpwi cr6, r31, 0x3d
	ctx.cr[6].compare_i32(ctx.r[31].s32, 61, &mut ctx.xer);
	// 82267254: 409A0018  bne cr6, 0x8226726c
	if !ctx.cr[6].eq {
	pc = 0x8226726C; continue 'dispatch;
	}
	// 82267258: 556B0109  rlwinm. r11, r11, 0, 4, 4
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226725C: 41820010  beq 0x8226726c
	if ctx.cr[0].eq {
	pc = 0x8226726C; continue 'dispatch;
	}
	// 82267260: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82267264: 38ABC75C  addi r5, r11, -0x38a4
	ctx.r[5].s64 = ctx.r[11].s64 + -14500;
	// 82267268: 48000028  b 0x82267290
	pc = 0x82267290; continue 'dispatch;
            }
            0x8226726C => {
    //   block [0x8226726C..0x82267278)
	// 8226726C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82267270: 38ABC734  addi r5, r11, -0x38cc
	ctx.r[5].s64 = ctx.r[11].s64 + -14540;
	// 82267274: 48000018  b 0x8226728c
	pc = 0x8226728C; continue 'dispatch;
            }
            0x82267278 => {
    //   block [0x82267278..0x82267284)
	// 82267278: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8226727C: 38AB1B9C  addi r5, r11, 0x1b9c
	ctx.r[5].s64 = ctx.r[11].s64 + 7068;
	// 82267280: 48000010  b 0x82267290
	pc = 0x82267290; continue 'dispatch;
            }
            0x82267284 => {
    //   block [0x82267284..0x8226728C)
	// 82267284: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82267288: 38ABC798  addi r5, r11, -0x3868
	ctx.r[5].s64 = ctx.r[11].s64 + -14440;
	pc = 0x8226728C; continue 'dispatch;
            }
            0x8226728C => {
    //   block [0x8226728C..0x82267290)
	// 8226728C: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	pc = 0x82267290; continue 'dispatch;
            }
            0x82267290 => {
    //   block [0x82267290..0x822672B8)
	// 82267290: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82267294: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82267298: 388B91C0  addi r4, r11, -0x6e40
	ctx.r[4].s64 = ctx.r[11].s64 + -28224;
	// 8226729C: 4BFFFBED  bl 0x82266e88
	ctx.lr = 0x822672A0;
	sub_82266E88(ctx, base);
	// 822672A0: 579F063F  clrlwi. r31, r28, 0x18
	ctx.r[31].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822672A4: 41820014  beq 0x822672b8
	if ctx.cr[0].eq {
	pc = 0x822672B8; continue 'dispatch;
	}
	// 822672A8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822672AC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 822672B0: 388B8B1C  addi r4, r11, -0x74e4
	ctx.r[4].s64 = ctx.r[11].s64 + -29924;
	// 822672B4: 4BFFFBD5  bl 0x82266e88
	ctx.lr = 0x822672B8;
	sub_82266E88(ctx, base);
	pc = 0x822672B8; continue 'dispatch;
            }
            0x822672B8 => {
    //   block [0x822672B8..0x822672D8)
	// 822672B8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822672BC: 409A001C  bne cr6, 0x822672d8
	if !ctx.cr[6].eq {
	pc = 0x822672D8; continue 'dispatch;
	}
	// 822672C0: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 822672C4: 419A0014  beq cr6, 0x822672d8
	if ctx.cr[6].eq {
	pc = 0x822672D8; continue 'dispatch;
	}
	// 822672C8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 822672CC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 822672D0: 388BA0FC  addi r4, r11, -0x5f04
	ctx.r[4].s64 = ctx.r[11].s64 + -24324;
	// 822672D4: 4BFFFBB5  bl 0x82266e88
	ctx.lr = 0x822672D8;
	sub_82266E88(ctx, base);
	pc = 0x822672D8; continue 'dispatch;
            }
            0x822672D8 => {
    //   block [0x822672D8..0x822672F4)
	// 822672D8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 822672DC: 41980018  blt cr6, 0x822672f4
	if ctx.cr[6].lt {
	pc = 0x822672F4; continue 'dispatch;
	}
	// 822672E0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 822672E4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822672E8: 388B51AC  addi r4, r11, 0x51ac
	ctx.r[4].s64 = ctx.r[11].s64 + 20908;
	// 822672EC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 822672F0: 4BFFFB99  bl 0x82266e88
	ctx.lr = 0x822672F4;
	sub_82266E88(ctx, base);
	pc = 0x822672F4; continue 'dispatch;
            }
            0x822672F4 => {
    //   block [0x822672F4..0x8226730C)
	// 822672F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822672F8: 419A0014  beq cr6, 0x8226730c
	if ctx.cr[6].eq {
	pc = 0x8226730C; continue 'dispatch;
	}
	// 822672FC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82267300: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82267304: 388B8B14  addi r4, r11, -0x74ec
	ctx.r[4].s64 = ctx.r[11].s64 + -29932;
	// 82267308: 4BFFFB81  bl 0x82266e88
	ctx.lr = 0x8226730C;
	sub_82266E88(ctx, base);
	pc = 0x8226730C; continue 'dispatch;
            }
            0x8226730C => {
    //   block [0x8226730C..0x82267320)
	// 8226730C: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82267310: 41820010  beq 0x82267320
	if ctx.cr[0].eq {
	pc = 0x82267320; continue 'dispatch;
	}
	// 82267314: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82267318: 388BA0F8  addi r4, r11, -0x5f08
	ctx.r[4].s64 = ctx.r[11].s64 + -24328;
	// 8226731C: 48000198  b 0x822674b4
	pc = 0x822674B4; continue 'dispatch;
            }
            0x82267320 => {
    //   block [0x82267320..0x82267358)
	// 82267320: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82267324: 409A0078  bne cr6, 0x8226739c
	if !ctx.cr[6].eq {
	pc = 0x8226739C; continue 'dispatch;
	}
	// 82267328: 2F18000F  cmpwi cr6, r24, 0xf
	ctx.cr[6].compare_i32(ctx.r[24].s32, 15, &mut ctx.xer);
	// 8226732C: 419A0190  beq cr6, 0x822674bc
	if ctx.cr[6].eq {
	pc = 0x822674BC; continue 'dispatch;
	}
	// 82267330: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82267334: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82267338: 388B57EC  addi r4, r11, 0x57ec
	ctx.r[4].s64 = ctx.r[11].s64 + 22508;
	// 8226733C: 4BFFFB4D  bl 0x82266e88
	ctx.lr = 0x82267340;
	sub_82266E88(ctx, base);
	// 82267340: 570B07FF  clrlwi. r11, r24, 0x1f
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82267344: 41820014  beq 0x82267358
	if ctx.cr[0].eq {
	pc = 0x82267358; continue 'dispatch;
	}
	// 82267348: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8226734C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82267350: 388B1A80  addi r4, r11, 0x1a80
	ctx.r[4].s64 = ctx.r[11].s64 + 6784;
	// 82267354: 4BFFFB35  bl 0x82266e88
	ctx.lr = 0x82267358;
	sub_82266E88(ctx, base);
	pc = 0x82267358; continue 'dispatch;
            }
            0x82267358 => {
    //   block [0x82267358..0x82267370)
	// 82267358: 570B07BD  rlwinm. r11, r24, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226735C: 41820014  beq 0x82267370
	if ctx.cr[0].eq {
	pc = 0x82267370; continue 'dispatch;
	}
	// 82267360: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82267364: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82267368: 388B1A7C  addi r4, r11, 0x1a7c
	ctx.r[4].s64 = ctx.r[11].s64 + 6780;
	// 8226736C: 4BFFFB1D  bl 0x82266e88
	ctx.lr = 0x82267370;
	sub_82266E88(ctx, base);
	pc = 0x82267370; continue 'dispatch;
            }
            0x82267370 => {
    //   block [0x82267370..0x82267388)
	// 82267370: 570B077B  rlwinm. r11, r24, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82267374: 41820014  beq 0x82267388
	if ctx.cr[0].eq {
	pc = 0x82267388; continue 'dispatch;
	}
	// 82267378: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8226737C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82267380: 388B1A74  addi r4, r11, 0x1a74
	ctx.r[4].s64 = ctx.r[11].s64 + 6772;
	// 82267384: 4BFFFB05  bl 0x82266e88
	ctx.lr = 0x82267388;
	sub_82266E88(ctx, base);
	pc = 0x82267388; continue 'dispatch;
            }
            0x82267388 => {
    //   block [0x82267388..0x8226739C)
	// 82267388: 570B0739  rlwinm. r11, r24, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226738C: 41820130  beq 0x822674bc
	if ctx.cr[0].eq {
	pc = 0x822674BC; continue 'dispatch;
	}
	// 82267390: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82267394: 388B1A64  addi r4, r11, 0x1a64
	ctx.r[4].s64 = ctx.r[11].s64 + 6756;
	// 82267398: 4800011C  b 0x822674b4
	pc = 0x822674B4; continue 'dispatch;
            }
            0x8226739C => {
    //   block [0x8226739C..0x822673AC)
	// 8226739C: 2F18000F  cmpwi cr6, r24, 0xf
	ctx.cr[6].compare_i32(ctx.r[24].s32, 15, &mut ctx.xer);
	// 822673A0: 409A000C  bne cr6, 0x822673ac
	if !ctx.cr[6].eq {
	pc = 0x822673AC; continue 'dispatch;
	}
	// 822673A4: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 822673A8: 419A0114  beq cr6, 0x822674bc
	if ctx.cr[6].eq {
	pc = 0x822674BC; continue 'dispatch;
	}
	pc = 0x822673AC; continue 'dispatch;
            }
            0x822673AC => {
    //   block [0x822673AC..0x822673C8)
	// 822673AC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 822673B0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 822673B4: 388B57EC  addi r4, r11, 0x57ec
	ctx.r[4].s64 = ctx.r[11].s64 + 22508;
	// 822673B8: 4BFFFAD1  bl 0x82266e88
	ctx.lr = 0x822673BC;
	sub_82266E88(ctx, base);
	// 822673BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822673C0: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 822673C4: 392B62F0  addi r9, r11, 0x62f0
	ctx.r[9].s64 = ctx.r[11].s64 + 25328;
	pc = 0x822673C8; continue 'dispatch;
            }
            0x822673C8 => {
    //   block [0x822673C8..0x822673E8)
	// 822673C8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822673CC: 7D6B5030  slw r11, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 822673D0: 7D68C039  and. r8, r11, r24
	ctx.r[8].u64 = ctx.r[11].u64 & ctx.r[24].u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 822673D4: 4182001C  beq 0x822673f0
	if ctx.cr[0].eq {
	pc = 0x822673F0; continue 'dispatch;
	}
	// 822673D8: 7D6BB839  and. r11, r11, r23
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[23].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822673DC: 4182000C  beq 0x822673e8
	if ctx.cr[0].eq {
	pc = 0x822673E8; continue 'dispatch;
	}
	// 822673E0: 39600031  li r11, 0x31
	ctx.r[11].s64 = 49;
	// 822673E4: 48000030  b 0x82267414
	pc = 0x82267414; continue 'dispatch;
            }
            0x822673E8 => {
    //   block [0x822673E8..0x822673F0)
	// 822673E8: 7D6A48AE  lbzx r11, r10, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 822673EC: 48000028  b 0x82267414
	pc = 0x82267414; continue 'dispatch;
            }
            0x822673F0 => {
    //   block [0x822673F0..0x82267400)
	// 822673F0: 7D6BB839  and. r11, r11, r23
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[23].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822673F4: 4182000C  beq 0x82267400
	if ctx.cr[0].eq {
	pc = 0x82267400; continue 'dispatch;
	}
	// 822673F8: 3960005F  li r11, 0x5f
	ctx.r[11].s64 = 95;
	// 822673FC: 48000018  b 0x82267414
	pc = 0x82267414; continue 'dispatch;
            }
            0x82267400 => {
    //   block [0x82267400..0x82267414)
	// 82267400: 21760000  subfic r11, r22, 0
	ctx.xer.ca = ctx.r[22].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[22].s64;
	// 82267404: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82267408: 556B07F6  rlwinm r11, r11, 0, 0x1f, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8226740C: 556B06F2  rlwinm r11, r11, 0, 0x1b, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82267410: 396B005F  addi r11, r11, 0x5f
	ctx.r[11].s64 = ctx.r[11].s64 + 95;
	pc = 0x82267414; continue 'dispatch;
            }
            0x82267414 => {
    //   block [0x82267414..0x82267434)
	// 82267414: 39010090  addi r8, r1, 0x90
	ctx.r[8].s64 = ctx.r[1].s64 + 144;
	// 82267418: 7D6A41AE  stbx r11, r10, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[11].u8) };
	// 8226741C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82267420: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82267424: 4198FFA4  blt cr6, 0x822673c8
	if ctx.cr[6].lt {
	pc = 0x822673C8; continue 'dispatch;
	}
	// 82267428: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8226742C: 39010098  addi r8, r1, 0x98
	ctx.r[8].s64 = ctx.r[1].s64 + 152;
	// 82267430: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	pc = 0x82267434; continue 'dispatch;
            }
            0x82267434 => {
    //   block [0x82267434..0x82267460)
	// 82267434: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 82267438: 7D2A58AE  lbzx r9, r10, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8226743C: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 82267440: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 82267444: 419A002C  beq cr6, 0x82267470
	if ctx.cr[6].eq {
	pc = 0x82267470; continue 'dispatch;
	}
	// 82267448: 2F0B0031  cmpwi cr6, r11, 0x31
	ctx.cr[6].compare_i32(ctx.r[11].s32, 49, &mut ctx.xer);
	// 8226744C: 419A0024  beq cr6, 0x82267470
	if ctx.cr[6].eq {
	pc = 0x82267470; continue 'dispatch;
	}
	// 82267450: 2F0B005F  cmpwi cr6, r11, 0x5f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 95, &mut ctx.xer);
	// 82267454: 419A000C  beq cr6, 0x82267460
	if ctx.cr[6].eq {
	pc = 0x82267460; continue 'dispatch;
	}
	// 82267458: 99280000  stb r9, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8226745C: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	pc = 0x82267460; continue 'dispatch;
            }
            0x82267460 => {
    //   block [0x82267460..0x82267470)
	// 82267460: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82267464: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82267468: 4198FFCC  blt cr6, 0x82267434
	if ctx.cr[6].lt {
	pc = 0x82267434; continue 'dispatch;
	}
	// 8226746C: 48000008  b 0x82267474
	pc = 0x82267474; continue 'dispatch;
            }
            0x82267470 => {
    //   block [0x82267470..0x82267474)
	// 82267470: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	pc = 0x82267474; continue 'dispatch;
            }
            0x82267474 => {
    //   block [0x82267474..0x82267488)
	// 82267474: 54EB063F  clrlwi. r11, r7, 0x18
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82267478: 41820010  beq 0x82267488
	if ctx.cr[0].eq {
	pc = 0x82267488; continue 'dispatch;
	}
	// 8226747C: 9AA80000  stb r21, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[21].u8 ) };
	// 82267480: 38810098  addi r4, r1, 0x98
	ctx.r[4].s64 = ctx.r[1].s64 + 152;
	// 82267484: 48000030  b 0x822674b4
	pc = 0x822674B4; continue 'dispatch;
            }
            0x82267488 => {
    //   block [0x82267488..0x82267490)
	// 82267488: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8226748C: 9AA10094  stb r21, 0x94(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[21].u8 ) };
	pc = 0x82267490; continue 'dispatch;
            }
            0x82267490 => {
    //   block [0x82267490..0x822674B0)
	// 82267490: 39410090  addi r10, r1, 0x90
	ctx.r[10].s64 = ctx.r[1].s64 + 144;
	// 82267494: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82267498: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226749C: 2B09005F  cmplwi cr6, r9, 0x5f
	ctx.cr[6].compare_u32(ctx.r[9].u32, 95 as u32, &mut ctx.xer);
	// 822674A0: 409A0010  bne cr6, 0x822674b0
	if !ctx.cr[6].eq {
	pc = 0x822674B0; continue 'dispatch;
	}
	// 822674A4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822674A8: 9AAA0000  stb r21, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[21].u8 ) };
	// 822674AC: 4181FFE4  bgt 0x82267490
	if ctx.cr[0].gt {
	pc = 0x82267490; continue 'dispatch;
	}
	pc = 0x822674B0; continue 'dispatch;
            }
            0x822674B0 => {
    //   block [0x822674B0..0x822674B4)
	// 822674B0: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	pc = 0x822674B4; continue 'dispatch;
            }
            0x822674B4 => {
    //   block [0x822674B4..0x822674BC)
	// 822674B4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 822674B8: 4BFFF9D1  bl 0x82266e88
	ctx.lr = 0x822674BC;
	sub_82266E88(ctx, base);
	pc = 0x822674BC; continue 'dispatch;
            }
            0x822674BC => {
    //   block [0x822674BC..0x822674C4)
	// 822674BC: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 822674C0: 4BE2760C  b 0x8208eacc
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822674C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822674C8 size=716
    let mut pc: u32 = 0x822674C8;
    'dispatch: loop {
        match pc {
            0x822674C8 => {
    //   block [0x822674C8..0x822674F4)
	// 822674C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822674CC: 4BE275B1  bl 0x8208ea7c
	ctx.lr = 0x822674D0;
	sub_8208EA60(ctx, base);
	// 822674D0: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822674D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822674D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822674DC: 7CF63B78  mr r22, r7
	ctx.r[22].u64 = ctx.r[7].u64;
	// 822674E0: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 822674E4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 822674E8: 409A000C  bne cr6, 0x822674f4
	if !ctx.cr[6].eq {
	pc = 0x822674F4; continue 'dispatch;
	}
	// 822674EC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 822674F0: 409A0014  bne cr6, 0x82267504
	if !ctx.cr[6].eq {
	pc = 0x82267504; continue 'dispatch;
	}
	pc = 0x822674F4; continue 'dispatch;
            }
            0x822674F4 => {
    //   block [0x822674F4..0x82267504)
	// 822674F4: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 822674F8: 409A0014  bne cr6, 0x8226750c
	if !ctx.cr[6].eq {
	pc = 0x8226750C; continue 'dispatch;
	}
	// 822674FC: 54AB0631  rlwinm. r11, r5, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82267500: 4182000C  beq 0x8226750c
	if ctx.cr[0].eq {
	pc = 0x8226750C; continue 'dispatch;
	}
	pc = 0x82267504; continue 'dispatch;
            }
            0x82267504 => {
    //   block [0x82267504..0x8226750C)
	// 82267504: 3AA00001  li r21, 1
	ctx.r[21].s64 = 1;
	// 82267508: 48000008  b 0x82267510
	pc = 0x82267510; continue 'dispatch;
            }
            0x8226750C => {
    //   block [0x8226750C..0x82267510)
	// 8226750C: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	pc = 0x82267510; continue 'dispatch;
            }
            0x82267510 => {
    //   block [0x82267510..0x82267558)
	// 82267510: 7CAB3670  srawi r11, r5, 6
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[5].s32 >> 6) as i64;
	// 82267514: 215E0000  subfic r10, r30, 0
	ctx.xer.ca = ctx.r[30].u32 <= 0 as u32;
	ctx.r[10].s64 = (0 as i64) - ctx.r[30].s64;
	// 82267518: 7D67F038  and r7, r11, r30
	ctx.r[7].u64 = ctx.r[11].u64 & ctx.r[30].u64;
	// 8226751C: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82267520: 7FC80034  cntlzw r8, r30
	ctx.r[8].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 82267524: 554B0032  rlwinm r11, r10, 0, 0, 0x19
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82267528: 550ADFFE  rlwinm r10, r8, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 8226752C: 556B066E  rlwinm r11, r11, 0, 0x19, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82267530: 54FB07FE  clrlwi r27, r7, 0x1f
	ctx.r[27].u64 = ctx.r[7].u32 as u64 & 0x00000001u64;
	// 82267534: 396B00FF  addi r11, r11, 0xff
	ctx.r[11].s64 = ctx.r[11].s64 + 255;
	// 82267538: 7D573038  and r23, r10, r6
	ctx.r[23].u64 = ctx.r[10].u64 & ctx.r[6].u64;
	// 8226753C: 7D782838  and r24, r11, r5
	ctx.r[24].u64 = ctx.r[11].u64 & ctx.r[5].u64;
	// 82267540: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82267544: 419A0014  beq cr6, 0x82267558
	if ctx.cr[6].eq {
	pc = 0x82267558; continue 'dispatch;
	}
	// 82267548: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8226754C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82267550: 388BAC98  addi r4, r11, -0x5368
	ctx.r[4].s64 = ctx.r[11].s64 + -21352;
	// 82267554: 4BFFF935  bl 0x82266e88
	ctx.lr = 0x82267558;
	sub_82266E88(ctx, base);
	pc = 0x82267558; continue 'dispatch;
            }
            0x82267558 => {
    //   block [0x82267558..0x8226756C)
	// 82267558: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8226755C: 409A0010  bne cr6, 0x8226756c
	if !ctx.cr[6].eq {
	pc = 0x8226756C; continue 'dispatch;
	}
	// 82267560: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82267564: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82267568: 419A0008  beq cr6, 0x82267570
	if ctx.cr[6].eq {
	pc = 0x82267570; continue 'dispatch;
	}
	pc = 0x8226756C; continue 'dispatch;
            }
            0x8226756C => {
    //   block [0x8226756C..0x82267570)
	// 8226756C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	pc = 0x82267570; continue 'dispatch;
            }
            0x82267570 => {
    //   block [0x82267570..0x822675C0)
	// 82267570: 213E0000  subfic r9, r30, 0
	ctx.xer.ca = ctx.r[30].u32 <= 0 as u32;
	ctx.r[9].s64 = (0 as i64) - ctx.r[30].s64;
	// 82267574: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82267578: 7D294910  subfe r9, r9, r9
	let x = (!ctx.r[9].u32);
	let y = ctx.r[9].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[9].u32 = res;
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8226757C: 3B4B4E44  addi r26, r11, 0x4e44
	ctx.r[26].s64 = ctx.r[11].s64 + 20036;
	// 82267580: 552B073E  clrlwi r11, r9, 0x1c
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x0000000Fu64;
	// 82267584: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82267588: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226758C: 38AB0063  addi r5, r11, 0x63
	ctx.r[5].s64 = ctx.r[11].s64 + 99;
	// 82267590: 555E063E  clrlwi r30, r10, 0x18
	ctx.r[30].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82267594: 4BFFF8F5  bl 0x82266e88
	ctx.lr = 0x82267598;
	sub_82266E88(ctx, base);
	// 82267598: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8226759C: 7FD9F378  mr r25, r30
	ctx.r[25].u64 = ctx.r[30].u64;
	// 822675A0: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822675A4: 3BABA100  addi r29, r11, -0x5f00
	ctx.r[29].s64 = ctx.r[11].s64 + -24320;
	// 822675A8: 41820028  beq 0x822675d0
	if ctx.cr[0].eq {
	pc = 0x822675D0; continue 'dispatch;
	}
	// 822675AC: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 822675B0: 419A0010  beq cr6, 0x822675c0
	if ctx.cr[6].eq {
	pc = 0x822675C0; continue 'dispatch;
	}
	// 822675B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822675B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822675BC: 4BFFF8CD  bl 0x82266e88
	ctx.lr = 0x822675C0;
	sub_82266E88(ctx, base);
	pc = 0x822675C0; continue 'dispatch;
            }
            0x822675C0 => {
    //   block [0x822675C0..0x822675D0)
	// 822675C0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822675C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822675C8: 388B8B1C  addi r4, r11, -0x74e4
	ctx.r[4].s64 = ctx.r[11].s64 + -29924;
	// 822675CC: 4BFFF8BD  bl 0x82266e88
	ctx.lr = 0x822675D0;
	sub_82266E88(ctx, base);
	pc = 0x822675D0; continue 'dispatch;
            }
            0x822675D0 => {
    //   block [0x822675D0..0x82267600)
	// 822675D0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 822675D4: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 822675D8: 388B51AC  addi r4, r11, 0x51ac
	ctx.r[4].s64 = ctx.r[11].s64 + 20908;
	// 822675DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822675E0: 4BFFF8A9  bl 0x82266e88
	ctx.lr = 0x822675E4;
	sub_82266E88(ctx, base);
	// 822675E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822675E8: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 822675EC: 3BCB8B20  addi r30, r11, -0x74e0
	ctx.r[30].s64 = ctx.r[11].s64 + -29920;
	// 822675F0: 419A0010  beq cr6, 0x82267600
	if ctx.cr[6].eq {
	pc = 0x82267600; continue 'dispatch;
	}
	// 822675F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822675F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822675FC: 4BFFF88D  bl 0x82266e88
	ctx.lr = 0x82267600;
	sub_82266E88(ctx, base);
	pc = 0x82267600; continue 'dispatch;
            }
            0x82267600 => {
    //   block [0x82267600..0x8226761C)
	// 82267600: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82267604: 419A0024  beq cr6, 0x82267628
	if ctx.cr[6].eq {
	pc = 0x82267628; continue 'dispatch;
	}
	// 82267608: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 8226760C: 419A0010  beq cr6, 0x8226761c
	if ctx.cr[6].eq {
	pc = 0x8226761C; continue 'dispatch;
	}
	// 82267610: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82267614: 388B8B18  addi r4, r11, -0x74e8
	ctx.r[4].s64 = ctx.r[11].s64 + -29928;
	// 82267618: 48000008  b 0x82267620
	pc = 0x82267620; continue 'dispatch;
            }
            0x8226761C => {
    //   block [0x8226761C..0x82267620)
	// 8226761C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	pc = 0x82267620; continue 'dispatch;
            }
            0x82267620 => {
    //   block [0x82267620..0x82267628)
	// 82267620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82267624: 4BFFF865  bl 0x82266e88
	ctx.lr = 0x82267628;
	sub_82266E88(ctx, base);
	pc = 0x82267628; continue 'dispatch;
            }
            0x82267628 => {
    //   block [0x82267628..0x8226763C)
	// 82267628: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 8226762C: 419A0010  beq cr6, 0x8226763c
	if ctx.cr[6].eq {
	pc = 0x8226763C; continue 'dispatch;
	}
	// 82267630: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82267634: 388B8B14  addi r4, r11, -0x74ec
	ctx.r[4].s64 = ctx.r[11].s64 + -29932;
	// 82267638: 48000010  b 0x82267648
	pc = 0x82267648; continue 'dispatch;
            }
            0x8226763C => {
    //   block [0x8226763C..0x82267648)
	// 8226763C: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 82267640: 419A0010  beq cr6, 0x82267650
	if ctx.cr[6].eq {
	pc = 0x82267650; continue 'dispatch;
	}
	// 82267644: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	pc = 0x82267648; continue 'dispatch;
            }
            0x82267648 => {
    //   block [0x82267648..0x82267650)
	// 82267648: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226764C: 4BFFF83D  bl 0x82266e88
	ctx.lr = 0x82267650;
	sub_82266E88(ctx, base);
	pc = 0x82267650; continue 'dispatch;
            }
            0x82267650 => {
    //   block [0x82267650..0x822676B8)
	// 82267650: 83A10154  lwz r29, 0x154(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(340 as u32) ) } as u64;
	// 82267654: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 82267658: 4098006C  bge cr6, 0x822676c4
	if !ctx.cr[6].lt {
	pc = 0x822676C4; continue 'dispatch;
	}
	// 8226765C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82267660: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82267664: 388B57EC  addi r4, r11, 0x57ec
	ctx.r[4].s64 = ctx.r[11].s64 + 22508;
	// 82267668: 4BFFF821  bl 0x82266e88
	ctx.lr = 0x8226766C;
	sub_82266E88(ctx, base);
	// 8226766C: 7F8B3670  srawi r11, r28, 6
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[28].s32 >> 6) as i64;
	// 82267670: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82267674: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82267678: 3BCA1A44  addi r30, r10, 0x1a44
	ctx.r[30].s64 = ctx.r[10].s64 + 6724;
	// 8226767C: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82267680: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82267684: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82267688: 7D6BF0AE  lbzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8226768C: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 82267690: 4BFFF7F9  bl 0x82266e88
	ctx.lr = 0x82267694;
	sub_82266E88(ctx, base);
	// 82267694: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 82267698: 409900F4  ble cr6, 0x8226778c
	if !ctx.cr[6].gt {
	pc = 0x8226778C; continue 'dispatch;
	}
	// 8226769C: 8961015F  lbz r11, 0x15f(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(351 as u32) ) } as u64;
	// 822676A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822676A4: 41820014  beq 0x822676b8
	if ctx.cr[0].eq {
	pc = 0x822676B8; continue 'dispatch;
	}
	// 822676A8: 7F8B2670  srawi r11, r28, 4
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[28].s32 >> 4) as i64;
	// 822676AC: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 822676B0: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 822676B4: 48000008  b 0x822676bc
	pc = 0x822676BC; continue 'dispatch;
            }
            0x822676B8 => {
    //   block [0x822676B8..0x822676BC)
	// 822676B8: 578B07BE  clrlwi r11, r28, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000003u64;
	pc = 0x822676BC; continue 'dispatch;
            }
            0x822676BC => {
    //   block [0x822676BC..0x822676C4)
	// 822676BC: 7D6BF0AE  lbzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 822676C0: 480000BC  b 0x8226777c
	pc = 0x8226777C; continue 'dispatch;
            }
            0x822676C4 => {
    //   block [0x822676C4..0x82267738)
	// 822676C4: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 822676C8: 419A00C4  beq cr6, 0x8226778c
	if ctx.cr[6].eq {
	pc = 0x8226778C; continue 'dispatch;
	}
	// 822676CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 822676D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822676D4: 388B57EC  addi r4, r11, 0x57ec
	ctx.r[4].s64 = ctx.r[11].s64 + 22508;
	// 822676D8: 4BFFF7B1  bl 0x82266e88
	ctx.lr = 0x822676DC;
	sub_82266E88(ctx, base);
	// 822676DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822676E0: 7F891670  srawi r9, r28, 2
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[28].s32 >> 2) as i64;
	// 822676E4: 3BAB1A44  addi r29, r11, 0x1a44
	ctx.r[29].s64 = ctx.r[11].s64 + 6724;
	// 822676E8: 7F8A2670  srawi r10, r28, 4
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[28].s32 >> 4) as i64;
	// 822676EC: 7F8B3670  srawi r11, r28, 6
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[28].s32 >> 6) as i64;
	// 822676F0: 579E07BE  clrlwi r30, r28, 0x1e
	ctx.r[30].u64 = ctx.r[28].u32 as u64 & 0x00000003u64;
	// 822676F4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 822676F8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 822676FC: 557907BE  clrlwi r25, r11, 0x1e
	ctx.r[25].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82267700: 394AFFFE  addi r10, r10, -2
	ctx.r[10].s64 = ctx.r[10].s64 + -2;
	// 82267704: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82267708: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226770C: 553C07BE  clrlwi r28, r9, 0x1e
	ctx.r[28].u64 = ctx.r[9].u32 as u64 & 0x00000003u64;
	// 82267710: 555B07BE  clrlwi r27, r10, 0x1e
	ctx.r[27].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 82267714: 7D7EE8AE  lbzx r11, r30, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82267718: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 8226771C: 4BFFF76D  bl 0x82266e88
	ctx.lr = 0x82267720;
	sub_82266E88(ctx, base);
	// 82267720: 7F1CF000  cmpw cr6, r28, r30
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82267724: 409A0014  bne cr6, 0x82267738
	if !ctx.cr[6].eq {
	pc = 0x82267738; continue 'dispatch;
	}
	// 82267728: 7F1BF000  cmpw cr6, r27, r30
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8226772C: 409A000C  bne cr6, 0x82267738
	if !ctx.cr[6].eq {
	pc = 0x82267738; continue 'dispatch;
	}
	// 82267730: 7F19F000  cmpw cr6, r25, r30
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82267734: 419A0058  beq cr6, 0x8226778c
	if ctx.cr[6].eq {
	pc = 0x8226778C; continue 'dispatch;
	}
	pc = 0x82267738; continue 'dispatch;
            }
            0x82267738 => {
    //   block [0x82267738..0x8226775C)
	// 82267738: 7D7CE8AE  lbzx r11, r28, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8226773C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82267740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82267744: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 82267748: 4BFFF741  bl 0x82266e88
	ctx.lr = 0x8226774C;
	sub_82266E88(ctx, base);
	// 8226774C: 7F1BE000  cmpw cr6, r27, r28
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82267750: 409A000C  bne cr6, 0x8226775c
	if !ctx.cr[6].eq {
	pc = 0x8226775C; continue 'dispatch;
	}
	// 82267754: 7F19E000  cmpw cr6, r25, r28
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82267758: 419A0034  beq cr6, 0x8226778c
	if ctx.cr[6].eq {
	pc = 0x8226778C; continue 'dispatch;
	}
	pc = 0x8226775C; continue 'dispatch;
            }
            0x8226775C => {
    //   block [0x8226775C..0x8226777C)
	// 8226775C: 7D7BE8AE  lbzx r11, r27, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82267760: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82267764: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82267768: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 8226776C: 4BFFF71D  bl 0x82266e88
	ctx.lr = 0x82267770;
	sub_82266E88(ctx, base);
	// 82267770: 7F19D800  cmpw cr6, r25, r27
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82267774: 419A0018  beq cr6, 0x8226778c
	if ctx.cr[6].eq {
	pc = 0x8226778C; continue 'dispatch;
	}
	// 82267778: 7D79E8AE  lbzx r11, r25, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	pc = 0x8226777C; continue 'dispatch;
            }
            0x8226777C => {
    //   block [0x8226777C..0x8226778C)
	// 8226777C: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 82267780: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82267784: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82267788: 4BFFF701  bl 0x82266e88
	ctx.lr = 0x8226778C;
	sub_82266E88(ctx, base);
	pc = 0x8226778C; continue 'dispatch;
            }
            0x8226778C => {
    //   block [0x8226778C..0x82267794)
	// 8226778C: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82267790: 4BE2733C  b 0x8208eacc
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82267798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82267798 size=328
    let mut pc: u32 = 0x82267798;
    'dispatch: loop {
        match pc {
            0x82267798 => {
    //   block [0x82267798..0x822677CC)
	// 82267798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226779C: 4BE272F9  bl 0x8208ea94
	ctx.lr = 0x822677A0;
	sub_8208EA60(ctx, base);
	// 822677A0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822677A4: 7C8B3E70  srawi r11, r4, 7
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[4].s32 >> 7) as i64;
	// 822677A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822677AC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 822677B0: 549C06BE  clrlwi r28, r4, 0x1a
	ctx.r[28].u64 = ctx.r[4].u32 as u64 & 0x0000003Fu64;
	// 822677B4: 557B07FE  clrlwi r27, r11, 0x1f
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 822677B8: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 822677BC: 419A0010  beq cr6, 0x822677cc
	if ctx.cr[6].eq {
	pc = 0x822677CC; continue 'dispatch;
	}
	// 822677C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822677C4: 388BAC98  addi r4, r11, -0x5368
	ctx.r[4].s64 = ctx.r[11].s64 + -21352;
	// 822677C8: 4BFFF6C1  bl 0x82266e88
	ctx.lr = 0x822677CC;
	sub_82266E88(ctx, base);
	pc = 0x822677CC; continue 'dispatch;
            }
            0x822677CC => {
    //   block [0x822677CC..0x822677E8)
	// 822677CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822677D0: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 822677D4: 3BABAC60  addi r29, r11, -0x53a0
	ctx.r[29].s64 = ctx.r[11].s64 + -21408;
	// 822677D8: 419A0010  beq cr6, 0x822677e8
	if ctx.cr[6].eq {
	pc = 0x822677E8; continue 'dispatch;
	}
	// 822677DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822677E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822677E4: 4BFFF6A5  bl 0x82266e88
	ctx.lr = 0x822677E8;
	sub_82266E88(ctx, base);
	pc = 0x822677E8; continue 'dispatch;
            }
            0x822677E8 => {
    //   block [0x822677E8..0x822678B8)
	// 822677E8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822677EC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822677F0: 388B8B3C  addi r4, r11, -0x74c4
	ctx.r[4].s64 = ctx.r[11].s64 + -29892;
	// 822677F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822677F8: 4BFFF691  bl 0x82266e88
	ctx.lr = 0x822677FC;
	sub_82266E88(ctx, base);
	// 822677FC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82267800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82267804: 388B8B14  addi r4, r11, -0x74ec
	ctx.r[4].s64 = ctx.r[11].s64 + -29932;
	// 82267808: 4BFFF681  bl 0x82266e88
	ctx.lr = 0x8226780C;
	sub_82266E88(ctx, base);
	// 8226780C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82267810: 419A00A8  beq cr6, 0x822678b8
	if ctx.cr[6].eq {
	pc = 0x822678B8; continue 'dispatch;
	}
	// 82267814: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82267818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226781C: 388B57EC  addi r4, r11, 0x57ec
	ctx.r[4].s64 = ctx.r[11].s64 + 22508;
	// 82267820: 4BFFF669  bl 0x82266e88
	ctx.lr = 0x82267824;
	sub_82266E88(ctx, base);
	// 82267824: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82267828: 57CA07BE  clrlwi r10, r30, 0x1e
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000003u64;
	// 8226782C: 396B1A44  addi r11, r11, 0x1a44
	ctx.r[11].s64 = ctx.r[11].s64 + 6724;
	// 82267830: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82267834: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82267838: 3B894E44  addi r28, r9, 0x4e44
	ctx.r[28].s64 = ctx.r[9].s64 + 20036;
	// 8226783C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82267840: 7D6A58AE  lbzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82267844: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 82267848: 4BFFF641  bl 0x82266e88
	ctx.lr = 0x8226784C;
	sub_82266E88(ctx, base);
	// 8226784C: 7FCB1670  srawi r11, r30, 2
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[30].s32 >> 2) as i64;
	// 82267850: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82267854: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82267858: 394A8B34  addi r10, r10, -0x74cc
	ctx.r[10].s64 = ctx.r[10].s64 + -29900;
	// 8226785C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82267860: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82267864: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82267868: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 8226786C: 4BFFF61D  bl 0x82266e88
	ctx.lr = 0x82267870;
	sub_82266E88(ctx, base);
	// 82267870: 7FCB2670  srawi r11, r30, 4
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[30].s32 >> 4) as i64;
	// 82267874: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82267878: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 8226787C: 394A8B2C  addi r10, r10, -0x74d4
	ctx.r[10].s64 = ctx.r[10].s64 + -29908;
	// 82267880: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82267884: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82267888: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8226788C: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 82267890: 4BFFF5F9  bl 0x82266e88
	ctx.lr = 0x82267894;
	sub_82266E88(ctx, base);
	// 82267894: 7FCB3670  srawi r11, r30, 6
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[30].s32 >> 6) as i64;
	// 82267898: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8226789C: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 822678A0: 394A8B24  addi r10, r10, -0x74dc
	ctx.r[10].s64 = ctx.r[10].s64 + -29916;
	// 822678A4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822678A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822678AC: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822678B0: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 822678B4: 4BFFF5D5  bl 0x82266e88
	ctx.lr = 0x822678B8;
	sub_82266E88(ctx, base);
	pc = 0x822678B8; continue 'dispatch;
            }
            0x822678B8 => {
    //   block [0x822678B8..0x822678C8)
	// 822678B8: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 822678BC: 419A000C  beq cr6, 0x822678c8
	if ctx.cr[6].eq {
	pc = 0x822678C8; continue 'dispatch;
	}
	// 822678C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822678C4: 4800000C  b 0x822678d0
	pc = 0x822678D0; continue 'dispatch;
            }
            0x822678C8 => {
    //   block [0x822678C8..0x822678D0)
	// 822678C8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822678CC: 388B0F75  addi r4, r11, 0xf75
	ctx.r[4].s64 = ctx.r[11].s64 + 3957;
	pc = 0x822678D0; continue 'dispatch;
            }
            0x822678D0 => {
    //   block [0x822678D0..0x822678E0)
	// 822678D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822678D4: 4BFFF5B5  bl 0x82266e88
	ctx.lr = 0x822678D8;
	sub_82266E88(ctx, base);
	// 822678D8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 822678DC: 4BE27208  b 0x8208eae4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822678E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822678E0 size=992
    let mut pc: u32 = 0x822678E0;
    'dispatch: loop {
        match pc {
            0x822678E0 => {
    //   block [0x822678E0..0x82267928)
	// 822678E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822678E4: 4BE271A5  bl 0x8208ea88
	ctx.lr = 0x822678E8;
	sub_8208EA60(ctx, base);
	// 822678E8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822678EC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822678F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822678F4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 822678F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822678FC: 7F58D378  mr r24, r26
	ctx.r[24].u64 = ctx.r[26].u64;
	// 82267900: 7F59D378  mr r25, r26
	ctx.r[25].u64 = ctx.r[26].u64;
	// 82267904: 4BFFF415  bl 0x82266d18
	ctx.lr = 0x82267908;
	sub_82266D18(ctx, base);
	// 82267908: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226790C: 4182001C  beq 0x82267928
	if ctx.cr[0].eq {
	pc = 0x82267928; continue 'dispatch;
	}
	// 82267910: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82267914: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82267918: 388BC9D4  addi r4, r11, -0x362c
	ctx.r[4].s64 = ctx.r[11].s64 + -13868;
	// 8226791C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82267920: 4BFFF6C1  bl 0x82266fe0
	ctx.lr = 0x82267924;
	sub_82266FE0(ctx, base);
	// 82267924: 48000394  b 0x82267cb8
	pc = 0x82267CB8; continue 'dispatch;
            }
            0x82267928 => {
    //   block [0x82267928..0x822679B4)
	// 82267928: 897F0008  lbz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8226792C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82267930: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82267934: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82267938: 394A9FA8  addi r10, r10, -0x6058
	ctx.r[10].s64 = ctx.r[10].s64 + -24664;
	// 8226793C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82267940: 7F6B50AE  lbzx r27, r11, r10
	ctx.r[27].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82267944: 409A0078  bne cr6, 0x822679bc
	if !ctx.cr[6].eq {
	pc = 0x822679BC; continue 'dispatch;
	}
	// 82267948: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226794C: 556AF87E  srwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82267950: 7D4B5A78  xor r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82267954: 556B018D  rlwinm. r11, r11, 0, 6, 6
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82267958: 40820064  bne 0x822679bc
	if !ctx.cr[0].eq {
	pc = 0x822679BC; continue 'dispatch;
	}
	// 8226795C: 897F000A  lbz r11, 0xa(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 82267960: 895F0009  lbz r10, 9(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 82267964: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82267968: 409A0054  bne cr6, 0x822679bc
	if !ctx.cr[6].eq {
	pc = 0x822679BC; continue 'dispatch;
	}
	// 8226796C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82267970: 556AF842  rlwinm r10, r11, 0x1f, 1, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82267974: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82267978: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8226797C: 409A0040  bne cr6, 0x822679bc
	if !ctx.cr[6].eq {
	pc = 0x822679BC; continue 'dispatch;
	}
	// 82267980: 897F0006  lbz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82267984: 895F0005  lbz r10, 5(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 82267988: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8226798C: 409A0030  bne cr6, 0x822679bc
	if !ctx.cr[6].eq {
	pc = 0x822679BC; continue 'dispatch;
	}
	// 82267990: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82267994: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82267998: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8226799C: 419A0018  beq cr6, 0x822679b4
	if ctx.cr[6].eq {
	pc = 0x822679B4; continue 'dispatch;
	}
	// 822679A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822679A4: 556AF842  rlwinm r10, r11, 0x1f, 1, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 822679A8: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 822679AC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822679B0: 409A000C  bne cr6, 0x822679bc
	if !ctx.cr[6].eq {
	pc = 0x822679BC; continue 'dispatch;
	}
	pc = 0x822679B4; continue 'dispatch;
            }
            0x822679B4 => {
    //   block [0x822679B4..0x822679BC)
	// 822679B4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822679B8: 48000008  b 0x822679c0
	pc = 0x822679C0; continue 'dispatch;
            }
            0x822679BC => {
    //   block [0x822679BC..0x822679C0)
	// 822679BC: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	pc = 0x822679C0; continue 'dispatch;
            }
            0x822679C0 => {
    //   block [0x822679C0..0x822679D0)
	// 822679C0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822679C4: 4182001C  beq 0x822679e0
	if ctx.cr[0].eq {
	pc = 0x822679E0; continue 'dispatch;
	}
	// 822679C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822679CC: 388AC9D0  addi r4, r10, -0x3630
	ctx.r[4].s64 = ctx.r[10].s64 + -13872;
	pc = 0x822679D0; continue 'dispatch;
            }
            0x822679D0 => {
    //   block [0x822679D0..0x822679E0)
	// 822679D0: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822679D4: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 822679D8: 556507FE  clrlwi r5, r11, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 822679DC: 480000B4  b 0x82267a90
	pc = 0x82267A90; continue 'dispatch;
            }
            0x822679E0 => {
    //   block [0x822679E0..0x82267A58)
	// 822679E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822679E4: 556A46FE  rlwinm r10, r11, 8, 0x1b, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00FFFFFFu64;
	// 822679E8: 2B0A001D  cmplwi cr6, r10, 0x1d
	ctx.cr[6].compare_u32(ctx.r[10].u32, 29 as u32, &mut ctx.xer);
	// 822679EC: 409A0074  bne cr6, 0x82267a60
	if !ctx.cr[6].eq {
	pc = 0x82267A60; continue 'dispatch;
	}
	// 822679F0: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822679F4: 5507F87E  srwi r7, r8, 1
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 822679F8: 7CE84278  xor r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 ^ ctx.r[8].u64;
	// 822679FC: 5508018D  rlwinm. r8, r8, 0, 6, 6
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82267A00: 40820060  bne 0x82267a60
	if !ctx.cr[0].eq {
	pc = 0x82267A60; continue 'dispatch;
	}
	// 82267A04: 891F000A  lbz r8, 0xa(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 82267A08: 88FF0009  lbz r7, 9(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 82267A0C: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82267A10: 409A0050  bne cr6, 0x82267a60
	if !ctx.cr[6].eq {
	pc = 0x82267A60; continue 'dispatch;
	}
	// 82267A14: 5568003E  slwi r8, r11, 0
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82267A18: 5507F842  rlwinm r7, r8, 0x1f, 1, 1
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	// 82267A1C: 55080042  rlwinm r8, r8, 0, 1, 1
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82267A20: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82267A24: 409A003C  bne cr6, 0x82267a60
	if !ctx.cr[6].eq {
	pc = 0x82267A60; continue 'dispatch;
	}
	// 82267A28: 891F0006  lbz r8, 6(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82267A2C: 88FF0005  lbz r7, 5(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 82267A30: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82267A34: 409A002C  bne cr6, 0x82267a60
	if !ctx.cr[6].eq {
	pc = 0x82267A60; continue 'dispatch;
	}
	// 82267A38: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82267A3C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82267A40: 419A0018  beq cr6, 0x82267a58
	if ctx.cr[6].eq {
	pc = 0x82267A58; continue 'dispatch;
	}
	// 82267A44: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267A48: 5569F842  rlwinm r9, r11, 0x1f, 1, 1
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82267A4C: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82267A50: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82267A54: 409A000C  bne cr6, 0x82267a60
	if !ctx.cr[6].eq {
	pc = 0x82267A60; continue 'dispatch;
	}
	pc = 0x82267A58; continue 'dispatch;
            }
            0x82267A58 => {
    //   block [0x82267A58..0x82267A60)
	// 82267A58: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82267A5C: 48000008  b 0x82267a64
	pc = 0x82267A64; continue 'dispatch;
            }
            0x82267A60 => {
    //   block [0x82267A60..0x82267A64)
	// 82267A60: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	pc = 0x82267A64; continue 'dispatch;
            }
            0x82267A64 => {
    //   block [0x82267A64..0x82267A78)
	// 82267A64: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82267A68: 41820010  beq 0x82267a78
	if ctx.cr[0].eq {
	pc = 0x82267A78; continue 'dispatch;
	}
	// 82267A6C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82267A70: 388AC8FC  addi r4, r10, -0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + -14084;
	// 82267A74: 4BFFFF5C  b 0x822679d0
	pc = 0x822679D0; continue 'dispatch;
            }
            0x82267A78 => {
    //   block [0x82267A78..0x82267A90)
	// 82267A78: 3D608228  lis r11, -0x7dd8
	ctx.r[11].s64 = -2111307776;
	// 82267A7C: 893F0000  lbz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82267A80: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82267A84: 396B6D00  addi r11, r11, 0x6d00
	ctx.r[11].s64 = ctx.r[11].s64 + 27904;
	// 82267A88: 552507FE  clrlwi r5, r9, 0x1f
	ctx.r[5].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 82267A8C: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	pc = 0x82267A90; continue 'dispatch;
            }
            0x82267A90 => {
    //   block [0x82267A90..0x82267AF4)
	// 82267A90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82267A94: 4BFFF54D  bl 0x82266fe0
	ctx.lr = 0x82267A98;
	sub_82266FE0(ctx, base);
	// 82267A98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82267A9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82267AA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82267AA4: 5569673E  rlwinm r9, r11, 0xc, 0x1c, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	// 82267AA8: 556897FE  rlwinm r8, r11, 0x12, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 82267AAC: 5567D7FE  rlwinm r7, r11, 0x1a, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82267AB0: 55668FFE  rlwinm r6, r11, 0x11, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	// 82267AB4: 5565873E  rlwinm r5, r11, 0x10, 0x1c, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82267AB8: 556406BE  clrlwi r4, r11, 0x1a
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82267ABC: 4BFFF695  bl 0x82267150
	ctx.lr = 0x82267AC0;
	sub_82267150(ctx, base);
	// 82267AC0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82267AC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82267AC8: 3BAB4304  addi r29, r11, 0x4304
	ctx.r[29].s64 = ctx.r[11].s64 + 17156;
	// 82267ACC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82267AD0: 4BFFF3B9  bl 0x82266e88
	ctx.lr = 0x82267AD4;
	sub_82266E88(ctx, base);
	// 82267AD4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267AD8: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82267ADC: 40820018  bne 0x82267af4
	if !ctx.cr[0].eq {
	pc = 0x82267AF4; continue 'dispatch;
	}
	// 82267AE0: 556A0043  rlwinm. r10, r11, 0, 1, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82267AE4: 40820010  bne 0x82267af4
	if !ctx.cr[0].eq {
	pc = 0x82267AF4; continue 'dispatch;
	}
	// 82267AE8: 556B0085  rlwinm. r11, r11, 0, 2, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82267AEC: 41820008  beq 0x82267af4
	if ctx.cr[0].eq {
	pc = 0x82267AF4; continue 'dispatch;
	}
	// 82267AF0: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	pc = 0x82267AF4; continue 'dispatch;
            }
            0x82267AF4 => {
    //   block [0x82267AF4..0x82267B38)
	// 82267AF4: 3B800004  li r28, 4
	ctx.r[28].s64 = 4;
	// 82267AF8: 2B1B0001  cmplwi cr6, r27, 1
	ctx.cr[6].compare_u32(ctx.r[27].u32, 1 as u32, &mut ctx.xer);
	// 82267AFC: 41980074  blt cr6, 0x82267b70
	if ctx.cr[6].lt {
	pc = 0x82267B70; continue 'dispatch;
	}
	// 82267B00: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82267B04: 556B0FFF  rlwinm. r11, r11, 1, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82267B08: 7D6A58F8  nor r10, r11, r11
	ctx.r[10].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82267B0C: 555907FE  clrlwi r25, r10, 0x1f
	ctx.r[25].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82267B10: 40820028  bne 0x82267b38
	if !ctx.cr[0].eq {
	pc = 0x82267B38; continue 'dispatch;
	}
	// 82267B14: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82267B18: 419A0020  beq cr6, 0x82267b38
	if ctx.cr[6].eq {
	pc = 0x82267B38; continue 'dispatch;
	}
	// 82267B1C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267B20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82267B24: 88DF0005  lbz r6, 5(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 82267B28: 556537FE  rlwinm r5, r11, 6, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x03FFFFFFu64;
	// 82267B2C: 889F0009  lbz r4, 9(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 82267B30: 4BFFFC69  bl 0x82267798
	ctx.lr = 0x82267B34;
	sub_82267798(ctx, base);
	// 82267B34: 4800003C  b 0x82267b70
	pc = 0x82267B70; continue 'dispatch;
            }
            0x82267B38 => {
    //   block [0x82267B38..0x82267B70)
	// 82267B38: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267B3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82267B40: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82267B44: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82267B48: 556937FE  rlwinm r9, r11, 6, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x03FFFFFFu64;
	// 82267B4C: 5548CFFE  rlwinm r8, r10, 0x19, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x0000007Fu64;
	// 82267B50: 895F0005  lbz r10, 5(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 82267B54: 55671FFE  rlwinm r7, r11, 3, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 82267B58: 88BF0009  lbz r5, 9(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 82267B5C: 55660FFE  srwi r6, r11, 0x1f
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82267B60: 9B41005F  stb r26, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[26].u8 ) };
	// 82267B64: 54840FFE  srwi r4, r4, 0x1f
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shr(31);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82267B68: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82267B6C: 4BFFF95D  bl 0x822674c8
	ctx.lr = 0x82267B70;
	sub_822674C8(ctx, base);
	pc = 0x82267B70; continue 'dispatch;
            }
            0x82267B70 => {
    //   block [0x82267B70..0x82267B90)
	// 82267B70: 2B1B0002  cmplwi cr6, r27, 2
	ctx.cr[6].compare_u32(ctx.r[27].u32, 2 as u32, &mut ctx.xer);
	// 82267B74: 419800A0  blt cr6, 0x82267c14
	if ctx.cr[6].lt {
	pc = 0x82267C14; continue 'dispatch;
	}
	// 82267B78: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82267B7C: 556B0043  rlwinm. r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82267B80: 40820010  bne 0x82267b90
	if !ctx.cr[0].eq {
	pc = 0x82267B90; continue 'dispatch;
	}
	// 82267B84: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82267B88: 409A0008  bne cr6, 0x82267b90
	if !ctx.cr[6].eq {
	pc = 0x82267B90; continue 'dispatch;
	}
	// 82267B8C: 3B200002  li r25, 2
	ctx.r[25].s64 = 2;
	pc = 0x82267B90; continue 'dispatch;
            }
            0x82267B90 => {
    //   block [0x82267B90..0x82267BCC)
	// 82267B90: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82267B94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82267B98: 4BFFF2F1  bl 0x82266e88
	ctx.lr = 0x82267B9C;
	sub_82266E88(ctx, base);
	// 82267B9C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82267BA0: 556B0043  rlwinm. r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82267BA4: 40820028  bne 0x82267bcc
	if !ctx.cr[0].eq {
	pc = 0x82267BCC; continue 'dispatch;
	}
	// 82267BA8: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82267BAC: 419A0020  beq cr6, 0x82267bcc
	if ctx.cr[6].eq {
	pc = 0x82267BCC; continue 'dispatch;
	}
	// 82267BB0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267BB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82267BB8: 88DF0006  lbz r6, 6(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82267BBC: 55653FFE  rlwinm r5, r11, 7, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x01FFFFFFu64;
	// 82267BC0: 889F000A  lbz r4, 0xa(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 82267BC4: 4BFFFBD5  bl 0x82267798
	ctx.lr = 0x82267BC8;
	sub_82267798(ctx, base);
	// 82267BC8: 4800004C  b 0x82267c14
	pc = 0x82267C14; continue 'dispatch;
            }
            0x82267BCC => {
    //   block [0x82267BCC..0x82267BE0)
	// 82267BCC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267BD0: 2F190002  cmpwi cr6, r25, 2
	ctx.cr[6].compare_i32(ctx.r[25].s32, 2, &mut ctx.xer);
	// 82267BD4: 55660FFE  srwi r6, r11, 0x1f
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82267BD8: 419A0008  beq cr6, 0x82267be0
	if ctx.cr[6].eq {
	pc = 0x82267BE0; continue 'dispatch;
	}
	// 82267BDC: 556617FE  rlwinm r6, r11, 2, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	pc = 0x82267BE0; continue 'dispatch;
            }
            0x82267BE0 => {
    //   block [0x82267BE0..0x82267C14)
	// 82267BE0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267BE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82267BE8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82267BEC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82267BF0: 55693FFE  rlwinm r9, r11, 7, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x01FFFFFFu64;
	// 82267BF4: 5548CFFE  rlwinm r8, r10, 0x19, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x0000007Fu64;
	// 82267BF8: 895F0006  lbz r10, 6(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82267BFC: 55671FFE  rlwinm r7, r11, 3, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 82267C00: 88BF000A  lbz r5, 0xa(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 82267C04: 548417FE  rlwinm r4, r4, 2, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x3FFFFFFFu64;
	// 82267C08: 9B41005F  stb r26, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[26].u8 ) };
	// 82267C0C: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82267C10: 4BFFF8B9  bl 0x822674c8
	ctx.lr = 0x82267C14;
	sub_822674C8(ctx, base);
	pc = 0x82267C14; continue 'dispatch;
            }
            0x82267C14 => {
    //   block [0x82267C14..0x82267C34)
	// 82267C14: 2B1B0003  cmplwi cr6, r27, 3
	ctx.cr[6].compare_u32(ctx.r[27].u32, 3 as u32, &mut ctx.xer);
	// 82267C18: 419800A0  blt cr6, 0x82267cb8
	if ctx.cr[6].lt {
	pc = 0x82267CB8; continue 'dispatch;
	}
	// 82267C1C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82267C20: 556B0085  rlwinm. r11, r11, 0, 2, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82267C24: 40820010  bne 0x82267c34
	if !ctx.cr[0].eq {
	pc = 0x82267C34; continue 'dispatch;
	}
	// 82267C28: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82267C2C: 409A0008  bne cr6, 0x82267c34
	if !ctx.cr[6].eq {
	pc = 0x82267C34; continue 'dispatch;
	}
	// 82267C30: 3B200003  li r25, 3
	ctx.r[25].s64 = 3;
	pc = 0x82267C34; continue 'dispatch;
            }
            0x82267C34 => {
    //   block [0x82267C34..0x82267C70)
	// 82267C34: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82267C38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82267C3C: 4BFFF24D  bl 0x82266e88
	ctx.lr = 0x82267C40;
	sub_82266E88(ctx, base);
	// 82267C40: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82267C44: 556A0085  rlwinm. r10, r11, 0, 2, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82267C48: 40820028  bne 0x82267c70
	if !ctx.cr[0].eq {
	pc = 0x82267C70; continue 'dispatch;
	}
	// 82267C4C: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82267C50: 419A0020  beq cr6, 0x82267c70
	if ctx.cr[6].eq {
	pc = 0x82267C70; continue 'dispatch;
	}
	// 82267C54: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267C58: 5564063E  clrlwi r4, r11, 0x18
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82267C5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82267C60: 5546063E  clrlwi r6, r10, 0x18
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82267C64: 554547FE  rlwinm r5, r10, 8, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 82267C68: 4BFFFB31  bl 0x82267798
	ctx.lr = 0x82267C6C;
	sub_82267798(ctx, base);
	// 82267C6C: 4800004C  b 0x82267cb8
	pc = 0x82267CB8; continue 'dispatch;
            }
            0x82267C70 => {
    //   block [0x82267C70..0x82267C84)
	// 82267C70: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267C74: 2F190003  cmpwi cr6, r25, 3
	ctx.cr[6].compare_i32(ctx.r[25].s32, 3, &mut ctx.xer);
	// 82267C78: 55660FFE  srwi r6, r11, 0x1f
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82267C7C: 419A0008  beq cr6, 0x82267c84
	if ctx.cr[6].eq {
	pc = 0x82267C84; continue 'dispatch;
	}
	// 82267C80: 556617FE  rlwinm r6, r11, 2, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	pc = 0x82267C84; continue 'dispatch;
            }
            0x82267C84 => {
    //   block [0x82267C84..0x82267CB8)
	// 82267C84: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82267C88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82267C8C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82267C90: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267C94: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82267C98: 5485063E  clrlwi r5, r4, 0x18
	ctx.r[5].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82267C9C: 556A063E  clrlwi r10, r11, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82267CA0: 9B41005F  stb r26, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[26].u8 ) };
	// 82267CA4: 556947FE  rlwinm r9, r11, 8, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00FFFFFFu64;
	// 82267CA8: 55671FFE  rlwinm r7, r11, 3, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 82267CAC: 5508CFFE  rlwinm r8, r8, 0x19, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000007Fu64;
	// 82267CB0: 54841FFE  rlwinm r4, r4, 3, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x1FFFFFFFu64;
	// 82267CB4: 4BFFF815  bl 0x822674c8
	ctx.lr = 0x82267CB8;
	sub_822674C8(ctx, base);
	pc = 0x82267CB8; continue 'dispatch;
            }
            0x82267CB8 => {
    //   block [0x82267CB8..0x82267CC0)
	// 82267CB8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82267CBC: 4BE26E1C  b 0x8208ead8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82267CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82267CC0 size=912
    let mut pc: u32 = 0x82267CC0;
    'dispatch: loop {
        match pc {
            0x82267CC0 => {
    //   block [0x82267CC0..0x82267CFC)
	// 82267CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82267CC4: 4BE26DC1  bl 0x8208ea84
	ctx.lr = 0x82267CC8;
	sub_8208EA60(ctx, base);
	// 82267CC8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82267CCC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82267CD0: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82267CD4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82267CD8: 7F37CB78  mr r23, r25
	ctx.r[23].u64 = ctx.r[25].u64;
	// 82267CDC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267CE0: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82267CE4: 40820018  bne 0x82267cfc
	if !ctx.cr[0].eq {
	pc = 0x82267CFC; continue 'dispatch;
	}
	// 82267CE8: 556A0043  rlwinm. r10, r11, 0, 1, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82267CEC: 40820010  bne 0x82267cfc
	if !ctx.cr[0].eq {
	pc = 0x82267CFC; continue 'dispatch;
	}
	// 82267CF0: 556B0085  rlwinm. r11, r11, 0, 2, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82267CF4: 41820008  beq 0x82267cfc
	if ctx.cr[0].eq {
	pc = 0x82267CFC; continue 'dispatch;
	}
	// 82267CF8: 3AE00001  li r23, 1
	ctx.r[23].s64 = 1;
	pc = 0x82267CFC; continue 'dispatch;
            }
            0x82267CFC => {
    //   block [0x82267CFC..0x82267D1C)
	// 82267CFC: 897F0008  lbz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82267D00: 7F38CB78  mr r24, r25
	ctx.r[24].u64 = ctx.r[25].u64;
	// 82267D04: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82267D08: 2B0B000B  cmplwi cr6, r11, 0xb
	ctx.cr[6].compare_u32(ctx.r[11].u32, 11 as u32, &mut ctx.xer);
	// 82267D0C: 41980010  blt cr6, 0x82267d1c
	if ctx.cr[6].lt {
	pc = 0x82267D1C; continue 'dispatch;
	}
	// 82267D10: 2B0B000E  cmplwi cr6, r11, 0xe
	ctx.cr[6].compare_u32(ctx.r[11].u32, 14 as u32, &mut ctx.xer);
	// 82267D14: 41990008  bgt cr6, 0x82267d1c
	if ctx.cr[6].gt {
	pc = 0x82267D1C; continue 'dispatch;
	}
	// 82267D18: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	pc = 0x82267D1C; continue 'dispatch;
            }
            0x82267D1C => {
    //   block [0x82267D1C..0x82267D3C)
	// 82267D1C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267D20: 5709063F  clrlwi. r9, r24, 0x18
	ctx.r[9].u64 = ctx.r[24].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82267D24: 556AD1BE  srwi r10, r11, 6
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82267D28: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82267D2C: 554A07BE  clrlwi r10, r10, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 82267D30: 4182000C  beq 0x82267d3c
	if ctx.cr[0].eq {
	pc = 0x82267D3C; continue 'dispatch;
	}
	// 82267D34: 556BE13E  srwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82267D38: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	pc = 0x82267D3C; continue 'dispatch;
            }
            0x82267D3C => {
    //   block [0x82267D3C..0x82267D78)
	// 82267D3C: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82267D40: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82267D44: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82267D48: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82267D4C: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82267D50: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82267D54: 41820024  beq 0x82267d78
	if ctx.cr[0].eq {
	pc = 0x82267D78; continue 'dispatch;
	}
	// 82267D58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82267D5C: 3D201400  lis r9, 0x1400
	ctx.r[9].s64 = 335544320;
	// 82267D60: 5568000A  rlwinm r8, r11, 0, 0, 5
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82267D64: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82267D68: 409A0010  bne cr6, 0x82267d78
	if !ctx.cr[6].eq {
	pc = 0x82267D78; continue 'dispatch;
	}
	// 82267D6C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82267D70: 388A1D58  addi r4, r10, 0x1d58
	ctx.r[4].s64 = ctx.r[10].s64 + 7512;
	// 82267D74: 48000050  b 0x82267dc4
	pc = 0x82267DC4; continue 'dispatch;
            }
            0x82267D78 => {
    //   block [0x82267D78..0x82267DA0)
	// 82267D78: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82267D7C: 419A0058  beq cr6, 0x82267dd4
	if ctx.cr[6].eq {
	pc = 0x82267DD4; continue 'dispatch;
	}
	// 82267D80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82267D84: 3D205C00  lis r9, 0x5c00
	ctx.r[9].s64 = 1543503872;
	// 82267D88: 5568000A  rlwinm r8, r11, 0, 0, 5
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82267D8C: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82267D90: 409A0010  bne cr6, 0x82267da0
	if !ctx.cr[6].eq {
	pc = 0x82267DA0; continue 'dispatch;
	}
	// 82267D94: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82267D98: 388A1D50  addi r4, r10, 0x1d50
	ctx.r[4].s64 = ctx.r[10].s64 + 7504;
	// 82267D9C: 48000028  b 0x82267dc4
	pc = 0x82267DC4; continue 'dispatch;
            }
            0x82267DA0 => {
    //   block [0x82267DA0..0x82267DC4)
	// 82267DA0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82267DA4: 419A0030  beq cr6, 0x82267dd4
	if ctx.cr[6].eq {
	pc = 0x82267DD4; continue 'dispatch;
	}
	// 82267DA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82267DAC: 3D406000  lis r10, 0x6000
	ctx.r[10].s64 = 1610612736;
	// 82267DB0: 5569000A  rlwinm r9, r11, 0, 0, 5
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82267DB4: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82267DB8: 409A001C  bne cr6, 0x82267dd4
	if !ctx.cr[6].eq {
	pc = 0x82267DD4; continue 'dispatch;
	}
	// 82267DBC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82267DC0: 388A1D48  addi r4, r10, 0x1d48
	ctx.r[4].s64 = ctx.r[10].s64 + 7496;
	pc = 0x82267DC4; continue 'dispatch;
            }
            0x82267DC4 => {
    //   block [0x82267DC4..0x82267DD4)
	// 82267DC4: 55653FFE  rlwinm r5, r11, 7, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x01FFFFFFu64;
	// 82267DC8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82267DCC: 4BFFF215  bl 0x82266fe0
	ctx.lr = 0x82267DD0;
	sub_82266FE0(ctx, base);
	// 82267DD0: 48000028  b 0x82267df8
	pc = 0x82267DF8; continue 'dispatch;
            }
            0x82267DD4 => {
    //   block [0x82267DD4..0x82267DF8)
	// 82267DD4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82267DD8: 3D408228  lis r10, -0x7dd8
	ctx.r[10].s64 = -2111307776;
	// 82267DDC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82267DE0: 394A6D80  addi r10, r10, 0x6d80
	ctx.r[10].s64 = ctx.r[10].s64 + 28032;
	// 82267DE4: 5569463A  rlwinm r9, r11, 8, 0x18, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00FFFFFFu64;
	// 82267DE8: 55653FFE  rlwinm r5, r11, 7, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x01FFFFFFu64;
	// 82267DEC: 7C89502E  lwzx r4, r9, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82267DF0: 4BFFF1F1  bl 0x82266fe0
	ctx.lr = 0x82267DF4;
	sub_82266FE0(ctx, base);
	// 82267DF4: 7F3ACB78  mr r26, r25
	ctx.r[26].u64 = ctx.r[25].u64;
	pc = 0x82267DF8; continue 'dispatch;
            }
            0x82267DF8 => {
    //   block [0x82267DF8..0x82267E0C)
	// 82267DF8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82267DFC: 55668FFF  rlwinm. r6, r11, 0x11, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82267E00: 5567D7FE  rlwinm r7, r11, 0x1a, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82267E04: 40820008  bne 0x82267e0c
	if !ctx.cr[0].eq {
	pc = 0x82267E0C; continue 'dispatch;
	}
	// 82267E08: 556797FE  rlwinm r7, r11, 0x12, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	pc = 0x82267E0C; continue 'dispatch;
            }
            0x82267E0C => {
    //   block [0x82267E0C..0x82267E1C)
	// 82267E0C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82267E10: 556406BE  clrlwi r4, r11, 0x1a
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82267E14: 409A0008  bne cr6, 0x82267e1c
	if !ctx.cr[6].eq {
	pc = 0x82267E1C; continue 'dispatch;
	}
	// 82267E18: 5564C6BE  rlwinm r4, r11, 0x18, 0x1a, 0x1f
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	pc = 0x82267E1C; continue 'dispatch;
            }
            0x82267E1C => {
    //   block [0x82267E1C..0x82267E68)
	// 82267E1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82267E20: 5569873E  rlwinm r9, r11, 0x10, 0x1c, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82267E24: 556897FE  rlwinm r8, r11, 0x12, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 82267E28: 5565673E  rlwinm r5, r11, 0xc, 0x1c, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	// 82267E2C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82267E30: 4BFFF321  bl 0x82267150
	ctx.lr = 0x82267E34;
	sub_82267150(ctx, base);
	// 82267E34: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82267E38: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82267E3C: 3BAA4304  addi r29, r10, 0x4304
	ctx.r[29].s64 = ctx.r[10].s64 + 17156;
	// 82267E40: 3B8BA008  addi r28, r11, -0x5ff8
	ctx.r[28].s64 = ctx.r[11].s64 + -24568;
	// 82267E44: 397CFFC0  addi r11, r28, -0x40
	ctx.r[11].s64 = ctx.r[28].s64 + -64;
	// 82267E48: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82267E4C: 554A36BE  srwi r10, r10, 0x1a
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(26);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82267E50: 7FCA58AE  lbzx r30, r10, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82267E54: 2C1E0000  cmpwi r30, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82267E58: 40810010  ble 0x82267e68
	if !ctx.cr[0].gt {
	pc = 0x82267E68; continue 'dispatch;
	}
	// 82267E5C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82267E60: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82267E64: 4BFFF025  bl 0x82266e88
	ctx.lr = 0x82267E68;
	sub_82266E88(ctx, base);
	pc = 0x82267E68; continue 'dispatch;
            }
            0x82267E68 => {
    //   block [0x82267E68..0x82267E84)
	// 82267E68: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82267E6C: 409A00E4  bne cr6, 0x82267f50
	if !ctx.cr[6].eq {
	pc = 0x82267F50; continue 'dispatch;
	}
	// 82267E70: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82267E74: 556B0001  rlwinm. r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82267E78: 4082000C  bne 0x82267e84
	if !ctx.cr[0].eq {
	pc = 0x82267E84; continue 'dispatch;
	}
	// 82267E7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82267E80: 4800002C  b 0x82267eac
	pc = 0x82267EAC; continue 'dispatch;
            }
            0x82267E84 => {
    //   block [0x82267E84..0x82267E98)
	// 82267E84: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82267E88: 556A0043  rlwinm. r10, r11, 0, 1, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82267E8C: 4082000C  bne 0x82267e98
	if !ctx.cr[0].eq {
	pc = 0x82267E98; continue 'dispatch;
	}
	// 82267E90: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82267E94: 48000018  b 0x82267eac
	pc = 0x82267EAC; continue 'dispatch;
            }
            0x82267E98 => {
    //   block [0x82267E98..0x82267EAC)
	// 82267E98: 556B0084  rlwinm r11, r11, 0, 2, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82267E9C: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 82267EA0: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82267EA4: 556B07FA  rlwinm r11, r11, 0, 0x1f, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82267EA8: 394B0003  addi r10, r11, 3
	ctx.r[10].s64 = ctx.r[11].s64 + 3;
	pc = 0x82267EAC; continue 'dispatch;
            }
            0x82267EAC => {
    //   block [0x82267EAC..0x82267EDC)
	// 82267EAC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82267EB0: 55690085  rlwinm. r9, r11, 0, 2, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82267EB4: 40820028  bne 0x82267edc
	if !ctx.cr[0].eq {
	pc = 0x82267EDC; continue 'dispatch;
	}
	// 82267EB8: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82267EBC: 419A0020  beq cr6, 0x82267edc
	if ctx.cr[6].eq {
	pc = 0x82267EDC; continue 'dispatch;
	}
	// 82267EC0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267EC4: 5564063E  clrlwi r4, r11, 0x18
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82267EC8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82267ECC: 5546063E  clrlwi r6, r10, 0x18
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82267ED0: 554547FE  rlwinm r5, r10, 8, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 82267ED4: 4BFFF8C5  bl 0x82267798
	ctx.lr = 0x82267ED8;
	sub_82267798(ctx, base);
	// 82267ED8: 48000170  b 0x82268048
	pc = 0x82268048; continue 'dispatch;
            }
            0x82267EDC => {
    //   block [0x82267EDC..0x82267EEC)
	// 82267EDC: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82267EE0: 4182000C  beq 0x82267eec
	if ctx.cr[0].eq {
	pc = 0x82267EEC; continue 'dispatch;
	}
	// 82267EE4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82267EE8: 48000010  b 0x82267ef8
	pc = 0x82267EF8; continue 'dispatch;
            }
            0x82267EEC => {
    //   block [0x82267EEC..0x82267EF8)
	// 82267EEC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82267EF0: 556B36BE  srwi r11, r11, 0x1a
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(26);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82267EF4: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	pc = 0x82267EF8; continue 'dispatch;
            }
            0x82267EF8 => {
    //   block [0x82267EF8..0x82267F0C)
	// 82267EF8: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 82267EFC: 409A0010  bne cr6, 0x82267f0c
	if !ctx.cr[6].eq {
	pc = 0x82267F0C; continue 'dispatch;
	}
	// 82267F00: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267F04: 55460FFE  srwi r6, r10, 0x1f
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shr(31);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82267F08: 48000010  b 0x82267f18
	pc = 0x82267F18; continue 'dispatch;
            }
            0x82267F0C => {
    //   block [0x82267F0C..0x82267F18)
	// 82267F0C: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 82267F10: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267F14: 554617FE  rlwinm r6, r10, 2, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	pc = 0x82267F18; continue 'dispatch;
            }
            0x82267F18 => {
    //   block [0x82267F18..0x82267F50)
	// 82267F18: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267F1C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82267F20: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82267F24: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82267F28: 54EA063E  clrlwi r10, r7, 0x18
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 82267F2C: 54E947FE  rlwinm r9, r7, 8, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[7].u32 as u64 & 0x00FFFFFFu64;
	// 82267F30: 9B01005F  stb r24, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[24].u8 ) };
	// 82267F34: 5485063E  clrlwi r5, r4, 0x18
	ctx.r[5].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82267F38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82267F3C: 5508CFFE  rlwinm r8, r8, 0x19, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000007Fu64;
	// 82267F40: 54E71FFE  rlwinm r7, r7, 3, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x1FFFFFFFu64;
	// 82267F44: 54841FFE  rlwinm r4, r4, 3, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x1FFFFFFFu64;
	// 82267F48: 4BFFF581  bl 0x822674c8
	ctx.lr = 0x82267F4C;
	sub_822674C8(ctx, base);
	// 82267F4C: 480000FC  b 0x82268048
	pc = 0x82268048; continue 'dispatch;
            }
            0x82267F50 => {
    //   block [0x82267F50..0x82267F94)
	// 82267F50: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 82267F54: 409A00F4  bne cr6, 0x82268048
	if !ctx.cr[6].eq {
	pc = 0x82268048; continue 'dispatch;
	}
	// 82267F58: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82267F5C: 556A0FFF  rlwinm. r10, r11, 1, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82267F60: 556B17FE  rlwinm r11, r11, 2, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82267F64: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82267F68: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82267F6C: 41820028  beq 0x82267f94
	if ctx.cr[0].eq {
	pc = 0x82267F94; continue 'dispatch;
	}
	// 82267F70: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82267F74: 554A17BE  srwi r10, r10, 0x1e
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(30);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82267F78: 7D4A50F8  nor r10, r10, r10
	ctx.r[10].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 82267F7C: 554A0FBC  rlwinm r10, r10, 1, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	// 82267F80: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 82267F84: 409A0010  bne cr6, 0x82267f94
	if !ctx.cr[6].eq {
	pc = 0x82267F94; continue 'dispatch;
	}
	// 82267F88: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267F8C: 55660FFE  srwi r6, r11, 0x1f
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82267F90: 48000010  b 0x82267fa0
	pc = 0x82267FA0; continue 'dispatch;
            }
            0x82267F94 => {
    //   block [0x82267F94..0x82267FA0)
	// 82267F94: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82267F98: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267F9C: 556617FE  rlwinm r6, r11, 2, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	pc = 0x82267FA0; continue 'dispatch;
            }
            0x82267FA0 => {
    //   block [0x82267FA0..0x8226801C)
	// 82267FA0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82267FA4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82267FA8: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267FAC: 556536BE  srwi r5, r11, 0x1a
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shr(26);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82267FB0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82267FB4: 5568CFFE  rlwinm r8, r11, 0x19, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 82267FB8: 9B21005F  stb r25, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[25].u8 ) };
	// 82267FBC: 54EA063E  clrlwi r10, r7, 0x18
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 82267FC0: 54E947FE  rlwinm r9, r7, 8, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[7].u32 as u64 & 0x00FFFFFFu64;
	// 82267FC4: 54E71FFE  rlwinm r7, r7, 3, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x1FFFFFFFu64;
	// 82267FC8: 7D65E0AE  lbzx r11, r5, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82267FCC: 5465063E  clrlwi r5, r3, 0x18
	ctx.r[5].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82267FD0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82267FD4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82267FD8: 4BFFF4F1  bl 0x822674c8
	ctx.lr = 0x82267FDC;
	sub_822674C8(ctx, base);
	// 82267FDC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82267FE0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82267FE4: 4BFFEEA5  bl 0x82266e88
	ctx.lr = 0x82267FE8;
	sub_82266E88(ctx, base);
	// 82267FE8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82267FEC: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82267FF0: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82267FF4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82267FF8: 556A3632  rlwinm r10, r11, 6, 0x18, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x03FFFFFFu64;
	// 82267FFC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82268000: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 82268004: 556B0631  rlwinm. r11, r11, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268008: 5107F108  rlwimi r7, r8, 0x1e, 4, 4
	ctx.r[7].u64 = (((ctx.r[8].u32).rotate_left(30) as u64) & 0x0000000008000000) | (ctx.r[7].u64 & 0xFFFFFFFFF7FFFFFF);
	// 8226800C: 50E937BE  rlwimi r9, r7, 6, 0x1e, 0x1f
	ctx.r[9].u64 = (((ctx.r[7].u32).rotate_left(6) as u64) & 0x0000000000000003) | (ctx.r[9].u64 & 0xFFFFFFFFFFFFFFFC);
	// 82268010: 552506BE  clrlwi r5, r9, 0x1a
	ctx.r[5].u64 = ctx.r[9].u32 as u64 & 0x0000003Fu64;
	// 82268014: 41820008  beq 0x8226801c
	if ctx.cr[0].eq {
	pc = 0x8226801C; continue 'dispatch;
	}
	// 82268018: 60A50080  ori r5, r5, 0x80
	ctx.r[5].u64 = ctx.r[5].u64 | 128;
	pc = 0x8226801C; continue 'dispatch;
            }
            0x8226801C => {
    //   block [0x8226801C..0x82268048)
	// 8226801C: 893F0004  lbz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82268020: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82268024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82268028: 9B21005F  stb r25, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[25].u8 ) };
	// 8226802C: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 82268030: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82268034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82268038: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226803C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82268040: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82268044: 4BFFF485  bl 0x822674c8
	ctx.lr = 0x82268048;
	sub_822674C8(ctx, base);
	pc = 0x82268048; continue 'dispatch;
            }
            0x82268048 => {
    //   block [0x82268048..0x82268050)
	// 82268048: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8226804C: 4BE26A88  b 0x8208ead4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82268050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82268050 size=100
    let mut pc: u32 = 0x82268050;
    'dispatch: loop {
        match pc {
            0x82268050 => {
    //   block [0x82268050..0x82268088)
	// 82268050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82268054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82268058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226805C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82268060: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268064: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82268068: 394AFE08  addi r10, r10, -0x1f8
	ctx.r[10].s64 = ctx.r[10].s64 + -504;
	// 8226806C: 3929FE10  addi r9, r9, -0x1f0
	ctx.r[9].s64 = ctx.r[9].s64 + -496;
	// 82268070: 556BD7FF  rlwinm. r11, r11, 0x1a, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268074: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82268078: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8226807C: 4082000C  bne 0x82268088
	if !ctx.cr[0].eq {
	pc = 0x82268088; continue 'dispatch;
	}
	// 82268080: 7F053040  cmplw cr6, r5, r6
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82268084: 419A0020  beq cr6, 0x822680a4
	if ctx.cr[6].eq {
	pc = 0x822680A4; continue 'dispatch;
	}
	pc = 0x82268088; continue 'dispatch;
            }
            0x82268088 => {
    //   block [0x82268088..0x822680A4)
	// 82268088: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8226808C: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82268090: 3D208205  lis r9, -0x7dfb
	ctx.r[9].s64 = -2113601536;
	// 82268094: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82268098: 3889A108  addi r4, r9, -0x5ef8
	ctx.r[4].s64 = ctx.r[9].s64 + -24312;
	// 8226809C: 7CCB502E  lwzx r6, r11, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822680A0: 4BFFEDE9  bl 0x82266e88
	ctx.lr = 0x822680A4;
	sub_82266E88(ctx, base);
	pc = 0x822680A4; continue 'dispatch;
            }
            0x822680A4 => {
    //   block [0x822680A4..0x822680B4)
	// 822680A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822680A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822680AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822680B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822680B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822680B8 size=72
    let mut pc: u32 = 0x822680B8;
    'dispatch: loop {
        match pc {
            0x822680B8 => {
    //   block [0x822680B8..0x82268100)
	// 822680B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822680BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822680C0: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 822680C4: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 822680C8: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 822680CC: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 822680D0: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 822680D4: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 822680D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822680DC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 822680E0: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 822680E4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822680E8: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822680EC: 4BFFEC9D  bl 0x82266d88
	ctx.lr = 0x822680F0;
	sub_82266D88(ctx, base);
	// 822680F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822680F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822680F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822680FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82268100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82268100 size=88
    let mut pc: u32 = 0x82268100;
    'dispatch: loop {
        match pc {
            0x82268100 => {
    //   block [0x82268100..0x82268130)
	// 82268100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82268104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82268108: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226810C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82268110: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82268114: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268118: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226811C: 41820014  beq 0x82268130
	if ctx.cr[0].eq {
	pc = 0x82268130; continue 'dispatch;
	}
	// 82268120: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82268124: 388B5818  addi r4, r11, 0x5818
	ctx.r[4].s64 = ctx.r[11].s64 + 22552;
	// 82268128: 4BFFEE61  bl 0x82266f88
	ctx.lr = 0x8226812C;
	sub_82266F88(ctx, base);
	// 8226812C: 48000010  b 0x8226813c
	pc = 0x8226813C; continue 'dispatch;
            }
            0x82268130 => {
    //   block [0x82268130..0x8226813C)
	// 82268130: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82268134: 388B42A8  addi r4, r11, 0x42a8
	ctx.r[4].s64 = ctx.r[11].s64 + 17064;
	// 82268138: 4BFFFF81  bl 0x822680b8
	ctx.lr = 0x8226813C;
	sub_822680B8(ctx, base);
	pc = 0x8226813C; continue 'dispatch;
            }
            0x8226813C => {
    //   block [0x8226813C..0x82268158)
	// 8226813C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82268140: 997F2028  stb r11, 0x2028(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8232 as u32), ctx.r[11].u8 ) };
	// 82268144: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82268148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226814C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82268150: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82268154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82268158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82268158 size=108
    let mut pc: u32 = 0x82268158;
    'dispatch: loop {
        match pc {
            0x82268158 => {
    //   block [0x82268158..0x8226817C)
	// 82268158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226815C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82268160: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82268164: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82268168: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226816C: 897F2028  lbz r11, 0x2028(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8232 as u32) ) } as u64;
	// 82268170: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82268174: 40820008  bne 0x8226817c
	if !ctx.cr[0].eq {
	pc = 0x8226817C; continue 'dispatch;
	}
	// 82268178: 4BFFFF89  bl 0x82268100
	ctx.lr = 0x8226817C;
	sub_82268100(ctx, base);
	pc = 0x8226817C; continue 'dispatch;
            }
            0x8226817C => {
    //   block [0x8226817C..0x822681A4)
	// 8226817C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268180: 556ACFFF  rlwinm. r10, r11, 0x19, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82268184: 4182002C  beq 0x822681b0
	if ctx.cr[0].eq {
	pc = 0x822681B0; continue 'dispatch;
	}
	// 82268188: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226818C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268190: 41820014  beq 0x822681a4
	if ctx.cr[0].eq {
	pc = 0x822681A4; continue 'dispatch;
	}
	// 82268194: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82268198: 388B5818  addi r4, r11, 0x5818
	ctx.r[4].s64 = ctx.r[11].s64 + 22552;
	// 8226819C: 4BFFEDED  bl 0x82266f88
	ctx.lr = 0x822681A0;
	sub_82266F88(ctx, base);
	// 822681A0: 48000010  b 0x822681b0
	pc = 0x822681B0; continue 'dispatch;
            }
            0x822681A4 => {
    //   block [0x822681A4..0x822681B0)
	// 822681A4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 822681A8: 388BA110  addi r4, r11, -0x5ef0
	ctx.r[4].s64 = ctx.r[11].s64 + -24304;
	// 822681AC: 4BFFFF0D  bl 0x822680b8
	ctx.lr = 0x822681B0;
	sub_822680B8(ctx, base);
	pc = 0x822681B0; continue 'dispatch;
            }
            0x822681B0 => {
    //   block [0x822681B0..0x822681C4)
	// 822681B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822681B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822681B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822681BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822681C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822681C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822681C8 size=72
    let mut pc: u32 = 0x822681C8;
    'dispatch: loop {
        match pc {
            0x822681C8 => {
    //   block [0x822681C8..0x822681FC)
	// 822681C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822681CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822681D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822681D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822681D8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 822681DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822681E0: 388B5828  addi r4, r11, 0x5828
	ctx.r[4].s64 = ctx.r[11].s64 + 22568;
	// 822681E4: 4BFFEDA5  bl 0x82266f88
	ctx.lr = 0x822681E8;
	sub_82266F88(ctx, base);
	// 822681E8: 897F2028  lbz r11, 0x2028(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8232 as u32) ) } as u64;
	// 822681EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822681F0: 4082000C  bne 0x822681fc
	if !ctx.cr[0].eq {
	pc = 0x822681FC; continue 'dispatch;
	}
	// 822681F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822681F8: 4BFFFF09  bl 0x82268100
	ctx.lr = 0x822681FC;
	sub_82268100(ctx, base);
	pc = 0x822681FC; continue 'dispatch;
            }
            0x822681FC => {
    //   block [0x822681FC..0x82268210)
	// 822681FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82268200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82268204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82268208: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226820C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82268210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82268210 size=196
    let mut pc: u32 = 0x82268210;
    'dispatch: loop {
        match pc {
            0x82268210 => {
    //   block [0x82268210..0x8226824C)
	// 82268210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82268214: 4BE26885  bl 0x8208ea98
	ctx.lr = 0x82268218;
	sub_8208EA60(ctx, base);
	// 82268218: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 8226821C: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82268220: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82268224: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82268228: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226822C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82268230: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82268234: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82268238: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8226823C: 897F2028  lbz r11, 0x2028(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8232 as u32) ) } as u64;
	// 82268240: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82268244: 40820008  bne 0x8226824c
	if !ctx.cr[0].eq {
	pc = 0x8226824C; continue 'dispatch;
	}
	// 82268248: 4BFFFEB9  bl 0x82268100
	ctx.lr = 0x8226824C;
	sub_82268100(ctx, base);
	pc = 0x8226824C; continue 'dispatch;
            }
            0x8226824C => {
    //   block [0x8226824C..0x82268274)
	// 8226824C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268250: 4BFFEEB9  bl 0x82267108
	ctx.lr = 0x82268254;
	sub_82267108(ctx, base);
	// 82268254: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268258: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226825C: 41820018  beq 0x82268274
	if ctx.cr[0].eq {
	pc = 0x82268274; continue 'dispatch;
	}
	// 82268260: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82268264: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82268268: 388BA128  addi r4, r11, -0x5ed8
	ctx.r[4].s64 = ctx.r[11].s64 + -24280;
	// 8226826C: 4BFFEC1D  bl 0x82266e88
	ctx.lr = 0x82268270;
	sub_82266E88(ctx, base);
	// 82268270: 48000028  b 0x82268298
	pc = 0x82268298; continue 'dispatch;
            }
            0x82268274 => {
    //   block [0x82268274..0x82268298)
	// 82268274: 7FCB0E70  srawi r11, r30, 1
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[30].s32 >> 1) as i64;
	// 82268278: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8226827C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82268280: 7FC90E70  srawi r9, r30, 1
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[30].s32 >> 1) as i64;
	// 82268284: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82268288: 388AA120  addi r4, r10, -0x5ee0
	ctx.r[4].s64 = ctx.r[10].s64 + -24288;
	// 8226828C: 7CA90194  addze r5, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[5].s64 = tmp.s64;
	// 82268290: 7CCBF050  subf r6, r11, r30
	ctx.r[6].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 82268294: 4BFFEBF5  bl 0x82266e88
	ctx.lr = 0x82268298;
	sub_82266E88(ctx, base);
	pc = 0x82268298; continue 'dispatch;
            }
            0x82268298 => {
    //   block [0x82268298..0x822682D4)
	// 82268298: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8226829C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822682A0: 388BA114  addi r4, r11, -0x5eec
	ctx.r[4].s64 = ctx.r[11].s64 + -24300;
	// 822682A4: 4BFFEBE5  bl 0x82266e88
	ctx.lr = 0x822682A8;
	sub_82266E88(ctx, base);
	// 822682A8: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 822682AC: 394100D0  addi r10, r1, 0xd0
	ctx.r[10].s64 = ctx.r[1].s64 + 208;
	// 822682B0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822682B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822682B8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822682BC: 80A10070  lwz r5, 0x70(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 822682C0: 4BFFEAC9  bl 0x82266d88
	ctx.lr = 0x822682C4;
	sub_82266D88(ctx, base);
	// 822682C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822682C8: 4BFFFF01  bl 0x822681c8
	ctx.lr = 0x822682CC;
	sub_822681C8(ctx, base);
	// 822682CC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 822682D0: 4BE26818  b 0x8208eae8
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822682D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822682D8 size=116
    let mut pc: u32 = 0x822682D8;
    'dispatch: loop {
        match pc {
            0x822682D8 => {
    //   block [0x822682D8..0x8226830C)
	// 822682D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822682DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822682E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822682E4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822682E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822682EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822682F0: 388B4304  addi r4, r11, 0x4304
	ctx.r[4].s64 = ctx.r[11].s64 + 17156;
	// 822682F4: 4BFFEB95  bl 0x82266e88
	ctx.lr = 0x822682F8;
	sub_82266E88(ctx, base);
	// 822682F8: 897F2028  lbz r11, 0x2028(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8232 as u32) ) } as u64;
	// 822682FC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82268300: 4082000C  bne 0x8226830c
	if !ctx.cr[0].eq {
	pc = 0x8226830C; continue 'dispatch;
	}
	// 82268304: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268308: 4BFFFDF9  bl 0x82268100
	ctx.lr = 0x8226830C;
	sub_82268100(ctx, base);
	pc = 0x8226830C; continue 'dispatch;
            }
            0x8226830C => {
    //   block [0x8226830C..0x82268328)
	// 8226830C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268310: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268314: 41820014  beq 0x82268328
	if ctx.cr[0].eq {
	pc = 0x82268328; continue 'dispatch;
	}
	// 82268318: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8226831C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268320: 388BA0E0  addi r4, r11, -0x5f20
	ctx.r[4].s64 = ctx.r[11].s64 + -24352;
	// 82268324: 4BFFEB65  bl 0x82266e88
	ctx.lr = 0x82268328;
	sub_82266E88(ctx, base);
	pc = 0x82268328; continue 'dispatch;
            }
            0x82268328 => {
    //   block [0x82268328..0x8226834C)
	// 82268328: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8226832C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268330: 388B8C78  addi r4, r11, -0x7388
	ctx.r[4].s64 = ctx.r[11].s64 + -29576;
	// 82268334: 4BFFEB55  bl 0x82266e88
	ctx.lr = 0x82268338;
	sub_82266E88(ctx, base);
	// 82268338: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226833C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82268340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82268344: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82268348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82268350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82268350 size=3576
    let mut pc: u32 = 0x82268350;
    'dispatch: loop {
        match pc {
            0x82268350 => {
    //   block [0x82268350..0x8226839C)
	// 82268350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82268354: 4BE26731  bl 0x8208ea84
	ctx.lr = 0x82268358;
	sub_8208EA60(ctx, base);
	// 82268358: DBE1FFA8  stfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 8226835C: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82268360: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82268364: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82268368: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226836C: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 82268370: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82268374: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82268378: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8226837C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82268380: 409A002C  bne cr6, 0x822683ac
	if !ctx.cr[6].eq {
	pc = 0x822683AC; continue 'dispatch;
	}
	// 82268384: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268388: 556B0001  rlwinm. r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226838C: 41820010  beq 0x8226839c
	if ctx.cr[0].eq {
	pc = 0x8226839C; continue 'dispatch;
	}
	// 82268390: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82268394: 388BA0A8  addi r4, r11, -0x5f58
	ctx.r[4].s64 = ctx.r[11].s64 + -24408;
	// 82268398: 4800000C  b 0x822683a4
	pc = 0x822683A4; continue 'dispatch;
            }
            0x8226839C => {
    //   block [0x8226839C..0x822683A4)
	// 8226839C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 822683A0: 388BA0A0  addi r4, r11, -0x5f60
	ctx.r[4].s64 = ctx.r[11].s64 + -24416;
	pc = 0x822683A4; continue 'dispatch;
            }
            0x822683A4 => {
    //   block [0x822683A4..0x822683AC)
	// 822683A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822683A8: 4BFFEAE1  bl 0x82266e88
	ctx.lr = 0x822683AC;
	sub_82266E88(ctx, base);
	pc = 0x822683AC; continue 'dispatch;
            }
            0x822683AC => {
    //   block [0x822683AC..0x822683EC)
	// 822683AC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822683B0: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 822683B4: 556506FE  clrlwi r5, r11, 0x1b
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822683B8: 409A0838  bne cr6, 0x82268bf0
	if !ctx.cr[6].eq {
	pc = 0x82268BF0; continue 'dispatch;
	}
	// 822683BC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 822683C0: 419A084C  beq cr6, 0x82268c0c
	if ctx.cr[6].eq {
	pc = 0x82268C0C; continue 'dispatch;
	}
	// 822683C4: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 822683C8: 419A0030  beq cr6, 0x822683f8
	if ctx.cr[6].eq {
	pc = 0x822683F8; continue 'dispatch;
	}
	// 822683CC: 2F05000F  cmpwi cr6, r5, 0xf
	ctx.cr[6].compare_i32(ctx.r[5].s32, 15, &mut ctx.xer);
	// 822683D0: 4099001C  ble cr6, 0x822683ec
	if !ctx.cr[6].gt {
	pc = 0x822683EC; continue 'dispatch;
	}
	// 822683D4: 2F050013  cmpwi cr6, r5, 0x13
	ctx.cr[6].compare_i32(ctx.r[5].s32, 19, &mut ctx.xer);
	// 822683D8: 40990020  ble cr6, 0x822683f8
	if !ctx.cr[6].gt {
	pc = 0x822683F8; continue 'dispatch;
	}
	// 822683DC: 2F050017  cmpwi cr6, r5, 0x17
	ctx.cr[6].compare_i32(ctx.r[5].s32, 23, &mut ctx.xer);
	// 822683E0: 4099000C  ble cr6, 0x822683ec
	if !ctx.cr[6].gt {
	pc = 0x822683EC; continue 'dispatch;
	}
	// 822683E4: 2F05001A  cmpwi cr6, r5, 0x1a
	ctx.cr[6].compare_i32(ctx.r[5].s32, 26, &mut ctx.xer);
	// 822683E8: 40990010  ble cr6, 0x822683f8
	if !ctx.cr[6].gt {
	pc = 0x822683F8; continue 'dispatch;
	}
	pc = 0x822683EC; continue 'dispatch;
            }
            0x822683EC => {
    //   block [0x822683EC..0x822683F8)
	// 822683EC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 822683F0: 388BA22C  addi r4, r11, -0x5dd4
	ctx.r[4].s64 = ctx.r[11].s64 + -24020;
	// 822683F4: 4800080C  b 0x82268c00
	pc = 0x82268C00; continue 'dispatch;
            }
            0x822683F8 => {
    //   block [0x822683F8..0x82268438)
	// 822683F8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822683FC: 2B050013  cmplwi cr6, r5, 0x13
	ctx.cr[6].compare_u32(ctx.r[5].u32, 19 as u32, &mut ctx.xer);
	// 82268400: 388B8E74  addi r4, r11, -0x718c
	ctx.r[4].s64 = ctx.r[11].s64 + -29068;
	// 82268404: 41990070  bgt cr6, 0x82268474
	if ctx.cr[6].gt {
	pc = 0x82268474; continue 'dispatch;
	}
	// 82268408: 419A0060  beq cr6, 0x82268468
	if ctx.cr[6].eq {
	pc = 0x82268468; continue 'dispatch;
	}
	// 8226840C: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 82268410: 419A0040  beq cr6, 0x82268450
	if ctx.cr[6].eq {
	pc = 0x82268450; continue 'dispatch;
	}
	// 82268414: 2B050010  cmplwi cr6, r5, 0x10
	ctx.cr[6].compare_u32(ctx.r[5].u32, 16 as u32, &mut ctx.xer);
	// 82268418: 419A002C  beq cr6, 0x82268444
	if ctx.cr[6].eq {
	pc = 0x82268444; continue 'dispatch;
	}
	// 8226841C: 2B050011  cmplwi cr6, r5, 0x11
	ctx.cr[6].compare_u32(ctx.r[5].u32, 17 as u32, &mut ctx.xer);
	// 82268420: 419A0018  beq cr6, 0x82268438
	if ctx.cr[6].eq {
	pc = 0x82268438; continue 'dispatch;
	}
	// 82268424: 2B050012  cmplwi cr6, r5, 0x12
	ctx.cr[6].compare_u32(ctx.r[5].u32, 18 as u32, &mut ctx.xer);
	// 82268428: 409A0084  bne cr6, 0x822684ac
	if !ctx.cr[6].eq {
	pc = 0x822684AC; continue 'dispatch;
	}
	// 8226842C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82268430: 388B1C74  addi r4, r11, 0x1c74
	ctx.r[4].s64 = ctx.r[11].s64 + 7284;
	// 82268434: 48000078  b 0x822684ac
	pc = 0x822684AC; continue 'dispatch;
            }
            0x82268438 => {
    //   block [0x82268438..0x82268444)
	// 82268438: 3D408228  lis r10, -0x7dd8
	ctx.r[10].s64 = -2111307776;
	// 8226843C: 394A70A0  addi r10, r10, 0x70a0
	ctx.r[10].s64 = ctx.r[10].s64 + 28832;
	// 82268440: 48000018  b 0x82268458
	pc = 0x82268458; continue 'dispatch;
            }
            0x82268444 => {
    //   block [0x82268444..0x82268450)
	// 82268444: 3D408228  lis r10, -0x7dd8
	ctx.r[10].s64 = -2111307776;
	// 82268448: 394A7090  addi r10, r10, 0x7090
	ctx.r[10].s64 = ctx.r[10].s64 + 28816;
	// 8226844C: 4800000C  b 0x82268458
	pc = 0x82268458; continue 'dispatch;
            }
            0x82268450 => {
    //   block [0x82268450..0x82268458)
	// 82268450: 3D408228  lis r10, -0x7dd8
	ctx.r[10].s64 = -2111307776;
	// 82268454: 394A7080  addi r10, r10, 0x7080
	ctx.r[10].s64 = ctx.r[10].s64 + 28800;
	pc = 0x82268458; continue 'dispatch;
            }
            0x82268458 => {
    //   block [0x82268458..0x82268468)
	// 82268458: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8226845C: 556BA73A  rlwinm r11, r11, 0x14, 0x1c, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82268460: 7C8B502E  lwzx r4, r11, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82268464: 48000048  b 0x822684ac
	pc = 0x822684AC; continue 'dispatch;
            }
            0x82268468 => {
    //   block [0x82268468..0x82268474)
	// 82268468: 3D408228  lis r10, -0x7dd8
	ctx.r[10].s64 = -2111307776;
	// 8226846C: 394A7070  addi r10, r10, 0x7070
	ctx.r[10].s64 = ctx.r[10].s64 + 28784;
	// 82268470: 4BFFFFE8  b 0x82268458
	pc = 0x82268458; continue 'dispatch;
            }
            0x82268474 => {
    //   block [0x82268474..0x82268498)
	// 82268474: 2B050018  cmplwi cr6, r5, 0x18
	ctx.cr[6].compare_u32(ctx.r[5].u32, 24 as u32, &mut ctx.xer);
	// 82268478: 419A002C  beq cr6, 0x822684a4
	if ctx.cr[6].eq {
	pc = 0x822684A4; continue 'dispatch;
	}
	// 8226847C: 2B050019  cmplwi cr6, r5, 0x19
	ctx.cr[6].compare_u32(ctx.r[5].u32, 25 as u32, &mut ctx.xer);
	// 82268480: 419A0018  beq cr6, 0x82268498
	if ctx.cr[6].eq {
	pc = 0x82268498; continue 'dispatch;
	}
	// 82268484: 2B05001A  cmplwi cr6, r5, 0x1a
	ctx.cr[6].compare_u32(ctx.r[5].u32, 26 as u32, &mut ctx.xer);
	// 82268488: 409A0024  bne cr6, 0x822684ac
	if !ctx.cr[6].eq {
	pc = 0x822684AC; continue 'dispatch;
	}
	// 8226848C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82268490: 388B1C08  addi r4, r11, 0x1c08
	ctx.r[4].s64 = ctx.r[11].s64 + 7176;
	// 82268494: 48000018  b 0x822684ac
	pc = 0x822684AC; continue 'dispatch;
            }
            0x82268498 => {
    //   block [0x82268498..0x822684A4)
	// 82268498: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8226849C: 388B1C18  addi r4, r11, 0x1c18
	ctx.r[4].s64 = ctx.r[11].s64 + 7192;
	// 822684A0: 4800000C  b 0x822684ac
	pc = 0x822684AC; continue 'dispatch;
            }
            0x822684A4 => {
    //   block [0x822684A4..0x822684AC)
	// 822684A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822684A8: 388B1C28  addi r4, r11, 0x1c28
	ctx.r[4].s64 = ctx.r[11].s64 + 7208;
	pc = 0x822684AC; continue 'dispatch;
            }
            0x822684AC => {
    //   block [0x822684AC..0x8226850C)
	// 822684AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822684B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822684B4: 4BFFEB2D  bl 0x82266fe0
	ctx.lr = 0x822684B8;
	sub_82266FE0(ctx, base);
	// 822684B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822684BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822684C0: 388BAF40  addi r4, r11, -0x50c0
	ctx.r[4].s64 = ctx.r[11].s64 + -20672;
	// 822684C4: 4BFFE9C5  bl 0x82266e88
	ctx.lr = 0x822684C8;
	sub_82266E88(ctx, base);
	// 822684C8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 822684CC: 3D408228  lis r10, -0x7dd8
	ctx.r[10].s64 = -2111307776;
	// 822684D0: 3BCB4E44  addi r30, r11, 0x4e44
	ctx.r[30].s64 = ctx.r[11].s64 + 20036;
	// 822684D4: 3B8A6CF0  addi r28, r10, 0x6cf0
	ctx.r[28].s64 = ctx.r[10].s64 + 27888;
	// 822684D8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 822684DC: 3B6957EC  addi r27, r9, 0x57ec
	ctx.r[27].s64 = ctx.r[9].s64 + 22508;
	// 822684E0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822684E4: 556A06FE  clrlwi r10, r11, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822684E8: 2B0A0013  cmplwi cr6, r10, 0x13
	ctx.cr[6].compare_u32(ctx.r[10].u32, 19 as u32, &mut ctx.xer);
	// 822684EC: 419900E8  bgt cr6, 0x822685d4
	if ctx.cr[6].gt {
	pc = 0x822685D4; continue 'dispatch;
	}
	// 822684F0: 556A035B  rlwinm. r10, r11, 0, 0xd, 0xd
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822684F4: 5565A6BE  rlwinm r5, r11, 0x14, 0x1a, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 822684F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822684FC: 41820010  beq 0x8226850c
	if ctx.cr[0].eq {
	pc = 0x8226850C; continue 'dispatch;
	}
	// 82268500: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82268504: 388B8B48  addi r4, r11, -0x74b8
	ctx.r[4].s64 = ctx.r[11].s64 + -29880;
	// 82268508: 4800000C  b 0x82268514
	pc = 0x82268514; continue 'dispatch;
            }
            0x8226850C => {
    //   block [0x8226850C..0x82268514)
	// 8226850C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82268510: 388B8B44  addi r4, r11, -0x74bc
	ctx.r[4].s64 = ctx.r[11].s64 + -29884;
	pc = 0x82268514; continue 'dispatch;
            }
            0x82268514 => {
    //   block [0x82268514..0x82268548)
	// 82268514: 4BFFE975  bl 0x82266e88
	ctx.lr = 0x82268518;
	sub_82266E88(ctx, base);
	// 82268518: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226851C: 556A077F  clrlwi. r10, r11, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82268520: 40820028  bne 0x82268548
	if !ctx.cr[0].eq {
	pc = 0x82268548; continue 'dispatch;
	}
	// 82268524: 556A06B8  rlwinm r10, r11, 0, 0x1a, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82268528: 2B0A0008  cmplwi cr6, r10, 8
	ctx.cr[6].compare_u32(ctx.r[10].u32, 8 as u32, &mut ctx.xer);
	// 8226852C: 409A001C  bne cr6, 0x82268548
	if !ctx.cr[6].eq {
	pc = 0x82268548; continue 'dispatch;
	}
	// 82268530: 556A05F2  rlwinm r10, r11, 0, 0x17, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82268534: 2B0A0080  cmplwi cr6, r10, 0x80
	ctx.cr[6].compare_u32(ctx.r[10].u32, 128 as u32, &mut ctx.xer);
	// 82268538: 409A0010  bne cr6, 0x82268548
	if !ctx.cr[6].eq {
	pc = 0x82268548; continue 'dispatch;
	}
	// 8226853C: 556B052C  rlwinm r11, r11, 0, 0x14, 0x16
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82268540: 2B0B0600  cmplwi cr6, r11, 0x600
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1536 as u32, &mut ctx.xer);
	// 82268544: 419A0080  beq cr6, 0x822685c4
	if ctx.cr[6].eq {
	pc = 0x822685C4; continue 'dispatch;
	}
	pc = 0x82268548; continue 'dispatch;
            }
            0x82268548 => {
    //   block [0x82268548..0x822685C4)
	// 82268548: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8226854C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268550: 4BFFE939  bl 0x82266e88
	ctx.lr = 0x82268554;
	sub_82266E88(ctx, base);
	// 82268554: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82268558: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226855C: 556B077E  clrlwi r11, r11, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82268560: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268564: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82268568: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 8226856C: 4BFFE91D  bl 0x82266e88
	ctx.lr = 0x82268570;
	sub_82266E88(ctx, base);
	// 82268570: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82268574: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82268578: 556BEF7E  rlwinm r11, r11, 0x1d, 0x1d, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 8226857C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268580: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82268584: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 82268588: 4BFFE901  bl 0x82266e88
	ctx.lr = 0x8226858C;
	sub_82266E88(ctx, base);
	// 8226858C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82268590: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82268594: 556BD77E  rlwinm r11, r11, 0x1a, 0x1d, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82268598: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226859C: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 822685A0: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 822685A4: 4BFFE8E5  bl 0x82266e88
	ctx.lr = 0x822685A8;
	sub_82266E88(ctx, base);
	// 822685A8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822685AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822685B0: 556BBF7E  rlwinm r11, r11, 0x17, 0x1d, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 822685B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822685B8: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 822685BC: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 822685C0: 4BFFE8C9  bl 0x82266e88
	ctx.lr = 0x822685C4;
	sub_82266E88(ctx, base);
	pc = 0x822685C4; continue 'dispatch;
            }
            0x822685C4 => {
    //   block [0x822685C4..0x822685D4)
	// 822685C4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822685C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822685CC: 388B4304  addi r4, r11, 0x4304
	ctx.r[4].s64 = ctx.r[11].s64 + 17156;
	// 822685D0: 4BFFE8B9  bl 0x82266e88
	ctx.lr = 0x822685D4;
	sub_82266E88(ctx, base);
	pc = 0x822685D4; continue 'dispatch;
            }
            0x822685D4 => {
    //   block [0x822685D4..0x82268650)
	// 822685D4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822685D8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 822685DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822685E0: 5565DE72  rlwinm r5, r11, 0x1b, 0x19, 0x19
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822685E4: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 822685E8: 556BDEBE  rlwinm r11, r11, 0x1b, 0x1a, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822685EC: 9941005F  stb r10, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[10].u8 ) };
	// 822685F0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 822685F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 822685F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 822685FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82268600: 7CA55B78  or r5, r5, r11
	ctx.r[5].u64 = ctx.r[5].u64 | ctx.r[11].u64;
	// 82268604: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82268608: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226860C: 4BFFEEBD  bl 0x822674c8
	ctx.lr = 0x82268610;
	sub_822674C8(ctx, base);
	// 82268610: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82268614: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268618: 4BFFE871  bl 0x82266e88
	ctx.lr = 0x8226861C;
	sub_82266E88(ctx, base);
	// 8226861C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82268620: 556A06FE  clrlwi r10, r11, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82268624: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82268628: 419A0054  beq cr6, 0x8226867c
	if ctx.cr[6].eq {
	pc = 0x8226867C; continue 'dispatch;
	}
	// 8226862C: 556B37BE  rlwinm r11, r11, 6, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x03FFFFFFu64;
	// 82268630: 2B0A0012  cmplwi cr6, r10, 0x12
	ctx.cr[6].compare_u32(ctx.r[10].u32, 18 as u32, &mut ctx.xer);
	// 82268634: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82268638: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226863C: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82268640: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 82268644: 419A0080  beq cr6, 0x822686c4
	if ctx.cr[6].eq {
	pc = 0x822686C4; continue 'dispatch;
	}
	// 82268648: 2B0A0018  cmplwi cr6, r10, 0x18
	ctx.cr[6].compare_u32(ctx.r[10].u32, 24 as u32, &mut ctx.xer);
	// 8226864C: 419A009C  beq cr6, 0x822686e8
	if ctx.cr[6].eq {
	pc = 0x822686E8; continue 'dispatch;
	}
	pc = 0x82268650; continue 'dispatch;
            }
            0x82268650 => {
    //   block [0x82268650..0x8226867C)
	// 82268650: 4BFFE839  bl 0x82266e88
	ctx.lr = 0x82268654;
	sub_82266E88(ctx, base);
	// 82268654: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82268658: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226865C: 556B27BE  rlwinm r11, r11, 4, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0FFFFFFFu64;
	// 82268660: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268664: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82268668: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 8226866C: 4BFFE81D  bl 0x82266e88
	ctx.lr = 0x82268670;
	sub_82266E88(ctx, base);
	// 82268670: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82268674: 556B17BE  srwi r11, r11, 0x1e
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(30);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82268678: 48000060  b 0x822686d8
	pc = 0x822686D8; continue 'dispatch;
            }
            0x8226867C => {
    //   block [0x8226867C..0x822686B0)
	// 8226867C: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268680: 554A97BE  rlwinm r10, r10, 0x12, 0x1e, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00003FFFu64;
	// 82268684: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82268688: 4198004C  blt cr6, 0x822686d4
	if ctx.cr[6].lt {
	pc = 0x822686D4; continue 'dispatch;
	}
	// 8226868C: 419A0024  beq cr6, 0x822686b0
	if ctx.cr[6].eq {
	pc = 0x822686B0; continue 'dispatch;
	}
	// 82268690: 2B0A0004  cmplwi cr6, r10, 4
	ctx.cr[6].compare_u32(ctx.r[10].u32, 4 as u32, &mut ctx.xer);
	// 82268694: 40980058  bge cr6, 0x822686ec
	if !ctx.cr[6].lt {
	pc = 0x822686EC; continue 'dispatch;
	}
	// 82268698: 556B37BE  rlwinm r11, r11, 6, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x03FFFFFFu64;
	// 8226869C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822686A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822686A4: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 822686A8: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 822686AC: 4BFFFFA4  b 0x82268650
	pc = 0x82268650; continue 'dispatch;
            }
            0x822686B0 => {
    //   block [0x822686B0..0x822686C4)
	// 822686B0: 556B37BE  rlwinm r11, r11, 6, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x03FFFFFFu64;
	// 822686B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822686B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822686BC: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 822686C0: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	pc = 0x822686C4; continue 'dispatch;
            }
            0x822686C4 => {
    //   block [0x822686C4..0x822686D4)
	// 822686C4: 4BFFE7C5  bl 0x82266e88
	ctx.lr = 0x822686C8;
	sub_82266E88(ctx, base);
	// 822686C8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822686CC: 556B27BE  rlwinm r11, r11, 4, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0FFFFFFFu64;
	// 822686D0: 48000008  b 0x822686d8
	pc = 0x822686D8; continue 'dispatch;
            }
            0x822686D4 => {
    //   block [0x822686D4..0x822686D8)
	// 822686D4: 556B37BE  rlwinm r11, r11, 6, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x03FFFFFFu64;
	pc = 0x822686D8; continue 'dispatch;
            }
            0x822686D8 => {
    //   block [0x822686D8..0x822686E8)
	// 822686D8: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 822686DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822686E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822686E4: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	pc = 0x822686E8; continue 'dispatch;
            }
            0x822686E8 => {
    //   block [0x822686E8..0x822686EC)
	// 822686E8: 4BFFE7A1  bl 0x82266e88
	ctx.lr = 0x822686EC;
	sub_82266E88(ctx, base);
	pc = 0x822686EC; continue 'dispatch;
            }
            0x822686EC => {
    //   block [0x822686EC..0x82268710)
	// 822686EC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822686F0: 556A06FE  clrlwi r10, r11, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822686F4: 2B0A0013  cmplwi cr6, r10, 0x13
	ctx.cr[6].compare_u32(ctx.r[10].u32, 19 as u32, &mut ctx.xer);
	// 822686F8: 41990018  bgt cr6, 0x82268710
	if ctx.cr[6].gt {
	pc = 0x82268710; continue 'dispatch;
	}
	// 822686FC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82268700: 556566FE  rlwinm r5, r11, 0xc, 0x1b, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	// 82268704: 388AA224  addi r4, r10, -0x5ddc
	ctx.r[4].s64 = ctx.r[10].s64 + -24028;
	// 82268708: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226870C: 4BFFE77D  bl 0x82266e88
	ctx.lr = 0x82268710;
	sub_82266E88(ctx, base);
	pc = 0x82268710; continue 'dispatch;
            }
            0x82268710 => {
    //   block [0x82268710..0x82268764)
	// 82268710: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82268714: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82268718: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8226871C: 388A1754  addi r4, r10, 0x1754
	ctx.r[4].s64 = ctx.r[10].s64 + 5972;
	// 82268720: 55656FFE  rlwinm r5, r11, 0xd, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0007FFFFu64;
	// 82268724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268728: 4BFFF929  bl 0x82268050
	ctx.lr = 0x8226872C;
	sub_82268050(ctx, base);
	// 8226872C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82268730: 3B6B40D8  addi r27, r11, 0x40d8
	ctx.r[27].s64 = ctx.r[11].s64 + 16600;
	// 82268734: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82268738: 556A06FE  clrlwi r10, r11, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8226873C: 7D4AD8AE  lbzx r10, r10, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82268740: 714A0017  andi. r10, r10, 0x17
	ctx.r[10].u64 = ctx.r[10].u64 & 23;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82268744: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82268748: 4182001C  beq 0x82268764
	if ctx.cr[0].eq {
	pc = 0x82268764; continue 'dispatch;
	}
	// 8226874C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82268750: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82268754: 388A1764  addi r4, r10, 0x1764
	ctx.r[4].s64 = ctx.r[10].s64 + 5988;
	// 82268758: 55653FFE  rlwinm r5, r11, 7, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x01FFFFFFu64;
	// 8226875C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268760: 4BFFF8F1  bl 0x82268050
	ctx.lr = 0x82268764;
	sub_82268050(ctx, base);
	pc = 0x82268764; continue 'dispatch;
            }
            0x82268764 => {
    //   block [0x82268764..0x822687A0)
	// 82268764: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82268768: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8226876C: 554A06FE  clrlwi r10, r10, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82268770: 3BCBA108  addi r30, r11, -0x5ef8
	ctx.r[30].s64 = ctx.r[11].s64 + -24312;
	// 82268774: 7D6AD8AE  lbzx r11, r10, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82268778: 716B0013  andi. r11, r11, 0x13
	ctx.r[11].u64 = ctx.r[11].u64 & 19;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226877C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82268780: 41820040  beq 0x822687c0
	if ctx.cr[0].eq {
	pc = 0x822687C0; continue 'dispatch;
	}
	// 82268784: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268788: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226878C: 5569D7FF  rlwinm. r9, r11, 0x1a, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82268790: 554BA7BE  rlwinm r11, r10, 0x14, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000FFFu64;
	// 82268794: 4082000C  bne 0x822687a0
	if !ctx.cr[0].eq {
	pc = 0x822687A0; continue 'dispatch;
	}
	// 82268798: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8226879C: 419A0024  beq cr6, 0x822687c0
	if ctx.cr[6].eq {
	pc = 0x822687C0; continue 'dispatch;
	}
	pc = 0x822687A0; continue 'dispatch;
            }
            0x822687A0 => {
    //   block [0x822687A0..0x822687C0)
	// 822687A0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822687A4: 395C0190  addi r10, r28, 0x190
	ctx.r[10].s64 = ctx.r[28].s64 + 400;
	// 822687A8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822687AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822687B0: 38A91748  addi r5, r9, 0x1748
	ctx.r[5].s64 = ctx.r[9].s64 + 5960;
	// 822687B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822687B8: 7CCB502E  lwzx r6, r11, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822687BC: 4BFFE6CD  bl 0x82266e88
	ctx.lr = 0x822687C0;
	sub_82266E88(ctx, base);
	pc = 0x822687C0; continue 'dispatch;
            }
            0x822687C0 => {
    //   block [0x822687C0..0x822687F4)
	// 822687C0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822687C4: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822687C8: 7D6BD8AE  lbzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 822687CC: 716B0013  andi. r11, r11, 0x13
	ctx.r[11].u64 = ctx.r[11].u64 & 19;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822687D0: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 822687D4: 41820040  beq 0x82268814
	if ctx.cr[0].eq {
	pc = 0x82268814; continue 'dispatch;
	}
	// 822687D8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822687DC: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822687E0: 5569D7FF  rlwinm. r9, r11, 0x1a, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 822687E4: 554B97BE  rlwinm r11, r10, 0x12, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00003FFFu64;
	// 822687E8: 4082000C  bne 0x822687f4
	if !ctx.cr[0].eq {
	pc = 0x822687F4; continue 'dispatch;
	}
	// 822687EC: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 822687F0: 419A0024  beq cr6, 0x82268814
	if ctx.cr[6].eq {
	pc = 0x82268814; continue 'dispatch;
	}
	pc = 0x822687F4; continue 'dispatch;
            }
            0x822687F4 => {
    //   block [0x822687F4..0x82268814)
	// 822687F4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822687F8: 395C01A0  addi r10, r28, 0x1a0
	ctx.r[10].s64 = ctx.r[28].s64 + 416;
	// 822687FC: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82268800: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82268804: 38A9173C  addi r5, r9, 0x173c
	ctx.r[5].s64 = ctx.r[9].s64 + 5948;
	// 82268808: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226880C: 7CCB502E  lwzx r6, r11, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82268810: 4BFFE679  bl 0x82266e88
	ctx.lr = 0x82268814;
	sub_82266E88(ctx, base);
	pc = 0x82268814; continue 'dispatch;
            }
            0x82268814 => {
    //   block [0x82268814..0x82268848)
	// 82268814: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82268818: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8226881C: 7D6BD8AE  lbzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82268820: 716B0013  andi. r11, r11, 0x13
	ctx.r[11].u64 = ctx.r[11].u64 & 19;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268824: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82268828: 41820040  beq 0x82268868
	if ctx.cr[0].eq {
	pc = 0x82268868; continue 'dispatch;
	}
	// 8226882C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268830: A15D0004  lhz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82268834: 5569D7FF  rlwinm. r9, r11, 0x1a, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82268838: 554B07BE  clrlwi r11, r10, 0x1e
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 8226883C: 4082000C  bne 0x82268848
	if !ctx.cr[0].eq {
	pc = 0x82268848; continue 'dispatch;
	}
	// 82268840: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82268844: 419A0024  beq cr6, 0x82268868
	if ctx.cr[6].eq {
	pc = 0x82268868; continue 'dispatch;
	}
	pc = 0x82268848; continue 'dispatch;
            }
            0x82268848 => {
    //   block [0x82268848..0x82268868)
	// 82268848: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8226884C: 395C01B0  addi r10, r28, 0x1b0
	ctx.r[10].s64 = ctx.r[28].s64 + 432;
	// 82268850: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82268854: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82268858: 38A91730  addi r5, r9, 0x1730
	ctx.r[5].s64 = ctx.r[9].s64 + 5936;
	// 8226885C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268860: 7CCB502E  lwzx r6, r11, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82268864: 4BFFE625  bl 0x82266e88
	ctx.lr = 0x82268868;
	sub_82266E88(ctx, base);
	pc = 0x82268868; continue 'dispatch;
            }
            0x82268868 => {
    //   block [0x82268868..0x8226889C)
	// 82268868: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226886C: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82268870: 7D6BD8AE  lbzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82268874: 716B0017  andi. r11, r11, 0x17
	ctx.r[11].u64 = ctx.r[11].u64 & 23;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268878: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8226887C: 41820040  beq 0x822688bc
	if ctx.cr[0].eq {
	pc = 0x822688BC; continue 'dispatch;
	}
	// 82268880: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268884: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82268888: 5569D7FF  rlwinm. r9, r11, 0x1a, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8226888C: 554B777E  rlwinm r11, r10, 0xe, 0x1d, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0003FFFFu64;
	// 82268890: 4082000C  bne 0x8226889c
	if !ctx.cr[0].eq {
	pc = 0x8226889C; continue 'dispatch;
	}
	// 82268894: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82268898: 419A0024  beq cr6, 0x822688bc
	if ctx.cr[6].eq {
	pc = 0x822688BC; continue 'dispatch;
	}
	pc = 0x8226889C; continue 'dispatch;
            }
            0x8226889C => {
    //   block [0x8226889C..0x822688BC)
	// 8226889C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822688A0: 395C01C0  addi r10, r28, 0x1c0
	ctx.r[10].s64 = ctx.r[28].s64 + 448;
	// 822688A4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822688A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822688AC: 38A91724  addi r5, r9, 0x1724
	ctx.r[5].s64 = ctx.r[9].s64 + 5924;
	// 822688B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822688B4: 7CCB502E  lwzx r6, r11, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822688B8: 4BFFE5D1  bl 0x82266e88
	ctx.lr = 0x822688BC;
	sub_82266E88(ctx, base);
	pc = 0x822688BC; continue 'dispatch;
            }
            0x822688BC => {
    //   block [0x822688BC..0x82268900)
	// 822688BC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822688C0: 556B0422  rlwinm r11, r11, 0, 0x10, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 822688C4: 2B0B8000  cmplwi cr6, r11, 0x8000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32768 as u32, &mut ctx.xer);
	// 822688C8: 41980058  blt cr6, 0x82268920
	if ctx.cr[6].lt {
	pc = 0x82268920; continue 'dispatch;
	}
	// 822688CC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822688D0: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822688D4: 7D6BD8AE  lbzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 822688D8: 716B0017  andi. r11, r11, 0x17
	ctx.r[11].u64 = ctx.r[11].u64 & 23;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822688DC: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 822688E0: 41820040  beq 0x82268920
	if ctx.cr[0].eq {
	pc = 0x82268920; continue 'dispatch;
	}
	// 822688E4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822688E8: 895D0004  lbz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822688EC: 5569D7FF  rlwinm. r9, r11, 0x1a, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 822688F0: 554B07BE  clrlwi r11, r10, 0x1e
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 822688F4: 4082000C  bne 0x82268900
	if !ctx.cr[0].eq {
	pc = 0x82268900; continue 'dispatch;
	}
	// 822688F8: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 822688FC: 419A0024  beq cr6, 0x82268920
	if ctx.cr[6].eq {
	pc = 0x82268920; continue 'dispatch;
	}
	pc = 0x82268900; continue 'dispatch;
            }
            0x82268900 => {
    //   block [0x82268900..0x82268920)
	// 82268900: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82268904: 395C01E0  addi r10, r28, 0x1e0
	ctx.r[10].s64 = ctx.r[28].s64 + 480;
	// 82268908: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8226890C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82268910: 38A91714  addi r5, r9, 0x1714
	ctx.r[5].s64 = ctx.r[9].s64 + 5908;
	// 82268914: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268918: 7CCB502E  lwzx r6, r11, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8226891C: 4BFFE56D  bl 0x82266e88
	ctx.lr = 0x82268920;
	sub_82266E88(ctx, base);
	pc = 0x82268920; continue 'dispatch;
            }
            0x82268920 => {
    //   block [0x82268920..0x82268964)
	// 82268920: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268924: 556B0422  rlwinm r11, r11, 0, 0x10, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82268928: 2B0B8000  cmplwi cr6, r11, 0x8000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32768 as u32, &mut ctx.xer);
	// 8226892C: 41980058  blt cr6, 0x82268984
	if ctx.cr[6].lt {
	pc = 0x82268984; continue 'dispatch;
	}
	// 82268930: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82268934: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82268938: 7D6BD8AE  lbzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 8226893C: 716B0017  andi. r11, r11, 0x17
	ctx.r[11].u64 = ctx.r[11].u64 & 23;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268940: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82268944: 41820040  beq 0x82268984
	if ctx.cr[0].eq {
	pc = 0x82268984; continue 'dispatch;
	}
	// 82268948: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8226894C: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82268950: 5569D7FF  rlwinm. r9, r11, 0x1a, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82268954: 554B37BE  rlwinm r11, r10, 6, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x03FFFFFFu64;
	// 82268958: 4082000C  bne 0x82268964
	if !ctx.cr[0].eq {
	pc = 0x82268964; continue 'dispatch;
	}
	// 8226895C: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82268960: 419A0024  beq cr6, 0x82268984
	if ctx.cr[6].eq {
	pc = 0x82268984; continue 'dispatch;
	}
	pc = 0x82268964; continue 'dispatch;
            }
            0x82268964 => {
    //   block [0x82268964..0x82268984)
	// 82268964: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82268968: 395C01F0  addi r10, r28, 0x1f0
	ctx.r[10].s64 = ctx.r[28].s64 + 496;
	// 8226896C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82268970: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82268974: 38A91704  addi r5, r9, 0x1704
	ctx.r[5].s64 = ctx.r[9].s64 + 5892;
	// 82268978: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226897C: 7CCB502E  lwzx r6, r11, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82268980: 4BFFE509  bl 0x82266e88
	ctx.lr = 0x82268984;
	sub_82266E88(ctx, base);
	pc = 0x82268984; continue 'dispatch;
            }
            0x82268984 => {
    //   block [0x82268984..0x822689B8)
	// 82268984: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82268988: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8226898C: 7D6BD8AE  lbzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82268990: 716B0017  andi. r11, r11, 0x17
	ctx.r[11].u64 = ctx.r[11].u64 & 23;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268994: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82268998: 41820020  beq 0x822689b8
	if ctx.cr[0].eq {
	pc = 0x822689B8; continue 'dispatch;
	}
	// 8226899C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822689A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822689A4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 822689A8: 388A16F4  addi r4, r10, 0x16f4
	ctx.r[4].s64 = ctx.r[10].s64 + 5876;
	// 822689AC: 556527FE  rlwinm r5, r11, 4, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0FFFFFFFu64;
	// 822689B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822689B4: 4BFFF69D  bl 0x82268050
	ctx.lr = 0x822689B8;
	sub_82268050(ctx, base);
	pc = 0x822689B8; continue 'dispatch;
            }
            0x822689B8 => {
    //   block [0x822689B8..0x822689EC)
	// 822689B8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822689BC: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822689C0: 7D6BD8AE  lbzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 822689C4: 716B0017  andi. r11, r11, 0x17
	ctx.r[11].u64 = ctx.r[11].u64 & 23;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822689C8: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 822689CC: 41820020  beq 0x822689ec
	if ctx.cr[0].eq {
	pc = 0x822689EC; continue 'dispatch;
	}
	// 822689D0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822689D4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822689D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822689DC: 388A16E4  addi r4, r10, 0x16e4
	ctx.r[4].s64 = ctx.r[10].s64 + 5860;
	// 822689E0: 55651FBE  rlwinm r5, r11, 3, 0x1e, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 822689E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822689E8: 4BFFF669  bl 0x82268050
	ctx.lr = 0x822689EC;
	sub_82268050(ctx, base);
	pc = 0x822689EC; continue 'dispatch;
            }
            0x822689EC => {
    //   block [0x822689EC..0x82268A1C)
	// 822689EC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822689F0: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822689F4: 7D6BD8AE  lbzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 822689F8: 556B06FF  clrlwi. r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822689FC: 41820020  beq 0x82268a1c
	if ctx.cr[0].eq {
	pc = 0x82268A1C; continue 'dispatch;
	}
	// 82268A00: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268A04: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82268A08: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82268A0C: 388A16CC  addi r4, r10, 0x16cc
	ctx.r[4].s64 = ctx.r[10].s64 + 5836;
	// 82268A10: 556507FE  clrlwi r5, r11, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82268A14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268A18: 4BFFF639  bl 0x82268050
	ctx.lr = 0x82268A1C;
	sub_82268050(ctx, base);
	pc = 0x82268A1C; continue 'dispatch;
            }
            0x82268A1C => {
    //   block [0x82268A1C..0x82268A4C)
	// 82268A1C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82268A20: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82268A24: 7D6BD8AE  lbzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82268A28: 716B0013  andi. r11, r11, 0x13
	ctx.r[11].u64 = ctx.r[11].u64 & 19;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268A2C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82268A30: 41820060  beq 0x82268a90
	if ctx.cr[0].eq {
	pc = 0x82268A90; continue 'dispatch;
	}
	// 82268A34: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268A38: 556BD7FF  rlwinm. r11, r11, 0x1a, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268A3C: 40820010  bne 0x82268a4c
	if !ctx.cr[0].eq {
	pc = 0x82268A4C; continue 'dispatch;
	}
	// 82268A40: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268A44: 556B05FB  rlwinm. r11, r11, 0, 0x17, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268A48: 41820048  beq 0x82268a90
	if ctx.cr[0].eq {
	pc = 0x82268A90; continue 'dispatch;
	}
	pc = 0x82268A4C; continue 'dispatch;
            }
            0x82268A4C => {
    //   block [0x82268A4C..0x82268A90)
	// 82268A4C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268A50: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82268A54: 3D208205  lis r9, -0x7dfb
	ctx.r[9].s64 = -2113601536;
	// 82268A58: 556BB810  slwi r11, r11, 0x17
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(23);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82268A5C: 3889A214  addi r4, r9, -0x5dec
	ctx.r[4].s64 = ctx.r[9].s64 + -24044;
	// 82268A60: 7D6BCE70  srawi r11, r11, 0x19
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 25) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 25) as i64;
	// 82268A64: C00A0B24  lfs f0, 0xb24(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2852 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82268A68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268A6C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82268A70: F9610078  std r11, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u64 ) };
	// 82268A74: C9A10078  lfd f13, 0x78(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 82268A78: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82268A7C: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82268A80: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82268A84: D8210020  stfd f1, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.f[1].u64 ) };
	// 82268A88: E8A10020  ld r5, 0x20(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(32 as u32) ) };
	// 82268A8C: 4BFFE3FD  bl 0x82266e88
	ctx.lr = 0x82268A90;
	sub_82266E88(ctx, base);
	pc = 0x82268A90; continue 'dispatch;
            }
            0x82268A90 => {
    //   block [0x82268A90..0x82268AC8)
	// 82268A90: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82268A94: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82268A98: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82268A9C: C3EA06FC  lfs f31, 0x6fc(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1788 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82268AA0: 7D6BD8AE  lbzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82268AA4: 716B0013  andi. r11, r11, 0x13
	ctx.r[11].u64 = ctx.r[11].u64 & 19;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268AA8: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82268AAC: 41820058  beq 0x82268b04
	if ctx.cr[0].eq {
	pc = 0x82268B04; continue 'dispatch;
	}
	// 82268AB0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268AB4: 556BD7FF  rlwinm. r11, r11, 0x1a, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268AB8: 40820010  bne 0x82268ac8
	if !ctx.cr[0].eq {
	pc = 0x82268AC8; continue 'dispatch;
	}
	// 82268ABC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268AC0: 556B02DF  rlwinm. r11, r11, 0, 0xb, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268AC4: 41820040  beq 0x82268b04
	if ctx.cr[0].eq {
	pc = 0x82268B04; continue 'dispatch;
	}
	pc = 0x82268AC8; continue 'dispatch;
            }
            0x82268AC8 => {
    //   block [0x82268AC8..0x82268B04)
	// 82268AC8: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268ACC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82268AD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268AD4: 556B5828  slwi r11, r11, 0xb
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82268AD8: 388AA204  addi r4, r10, -0x5dfc
	ctx.r[4].s64 = ctx.r[10].s64 + -24060;
	// 82268ADC: 7D6BDE70  srawi r11, r11, 0x1b
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 27) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 27) as i64;
	// 82268AE0: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82268AE4: F9610078  std r11, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u64 ) };
	// 82268AE8: C8010078  lfd f0, 0x78(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 82268AEC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82268AF0: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82268AF4: EC2007F2  fmuls f1, f0, f31
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82268AF8: D8210020  stfd f1, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.f[1].u64 ) };
	// 82268AFC: E8A10020  ld r5, 0x20(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(32 as u32) ) };
	// 82268B00: 4BFFE389  bl 0x82266e88
	ctx.lr = 0x82268B04;
	sub_82266E88(ctx, base);
	pc = 0x82268B04; continue 'dispatch;
            }
            0x82268B04 => {
    //   block [0x82268B04..0x82268B40)
	// 82268B04: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268B08: 556A0422  rlwinm r10, r11, 0, 0x10, 0x11
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82268B0C: 2B0A4000  cmplwi cr6, r10, 0x4000
	ctx.cr[6].compare_u32(ctx.r[10].u32, 16384 as u32, &mut ctx.xer);
	// 82268B10: 41980068  blt cr6, 0x82268b78
	if ctx.cr[6].lt {
	pc = 0x82268B78; continue 'dispatch;
	}
	// 82268B14: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82268B18: 554A06FE  clrlwi r10, r10, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82268B1C: 7D4AD8AE  lbzx r10, r10, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82268B20: 714A0013  andi. r10, r10, 0x13
	ctx.r[10].u64 = ctx.r[10].u64 & 19;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82268B24: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82268B28: 41820050  beq 0x82268b78
	if ctx.cr[0].eq {
	pc = 0x82268B78; continue 'dispatch;
	}
	// 82268B2C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268B30: 554AD7FF  rlwinm. r10, r10, 0x1a, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82268B34: 4082000C  bne 0x82268b40
	if !ctx.cr[0].eq {
	pc = 0x82268B40; continue 'dispatch;
	}
	// 82268B38: 556A0195  rlwinm. r10, r11, 0, 6, 0xa
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82268B3C: 4182003C  beq 0x82268b78
	if ctx.cr[0].eq {
	pc = 0x82268B78; continue 'dispatch;
	}
	pc = 0x82268B40; continue 'dispatch;
            }
            0x82268B40 => {
    //   block [0x82268B40..0x82268B78)
	// 82268B40: 556B3032  slwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82268B44: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82268B48: 7D6BDE70  srawi r11, r11, 0x1b
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 27) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 27) as i64;
	// 82268B4C: 388AA1F4  addi r4, r10, -0x5e0c
	ctx.r[4].s64 = ctx.r[10].s64 + -24076;
	// 82268B50: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82268B54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268B58: F9610078  std r11, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u64 ) };
	// 82268B5C: C8010078  lfd f0, 0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 82268B60: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82268B64: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82268B68: EC2007F2  fmuls f1, f0, f31
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82268B6C: D8210020  stfd f1, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.f[1].u64 ) };
	// 82268B70: E8A10020  ld r5, 0x20(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(32 as u32) ) };
	// 82268B74: 4BFFE315  bl 0x82266e88
	ctx.lr = 0x82268B78;
	sub_82266E88(ctx, base);
	pc = 0x82268B78; continue 'dispatch;
            }
            0x82268B78 => {
    //   block [0x82268B78..0x82268BB4)
	// 82268B78: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268B7C: 556A0422  rlwinm r10, r11, 0, 0x10, 0x11
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82268B80: 2B0A8000  cmplwi cr6, r10, 0x8000
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32768 as u32, &mut ctx.xer);
	// 82268B84: 419805B8  blt cr6, 0x8226913c
	if ctx.cr[6].lt {
	pc = 0x8226913C; continue 'dispatch;
	}
	// 82268B88: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82268B8C: 554A06FE  clrlwi r10, r10, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82268B90: 7D4AD8AE  lbzx r10, r10, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82268B94: 714A0013  andi. r10, r10, 0x13
	ctx.r[10].u64 = ctx.r[10].u64 & 19;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82268B98: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82268B9C: 418205A0  beq 0x8226913c
	if ctx.cr[0].eq {
	pc = 0x8226913C; continue 'dispatch;
	}
	// 82268BA0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268BA4: 554AD7FF  rlwinm. r10, r10, 0x1a, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82268BA8: 4082000C  bne 0x82268bb4
	if !ctx.cr[0].eq {
	pc = 0x82268BB4; continue 'dispatch;
	}
	// 82268BAC: 556A004B  rlwinm. r10, r11, 0, 1, 5
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82268BB0: 4182058C  beq 0x8226913c
	if ctx.cr[0].eq {
	pc = 0x8226913C; continue 'dispatch;
	}
	pc = 0x82268BB4; continue 'dispatch;
            }
            0x82268BB4 => {
    //   block [0x82268BB4..0x82268BF0)
	// 82268BB4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82268BB8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82268BBC: 7D6BDE70  srawi r11, r11, 0x1b
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 27) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 27) as i64;
	// 82268BC0: 388AA1E4  addi r4, r10, -0x5e1c
	ctx.r[4].s64 = ctx.r[10].s64 + -24092;
	// 82268BC4: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82268BC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268BCC: F9610078  std r11, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u64 ) };
	// 82268BD0: C8010078  lfd f0, 0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 82268BD4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82268BD8: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82268BDC: EC2007F2  fmuls f1, f0, f31
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82268BE0: D8210020  stfd f1, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.f[1].u64 ) };
	// 82268BE4: E8A10020  ld r5, 0x20(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(32 as u32) ) };
	// 82268BE8: 4BFFE2A1  bl 0x82266e88
	ctx.lr = 0x82268BEC;
	sub_82266E88(ctx, base);
	// 82268BEC: 48000550  b 0x8226913c
	pc = 0x8226913C; continue 'dispatch;
            }
            0x82268BF0 => {
    //   block [0x82268BF0..0x82268C00)
	// 82268BF0: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82268BF4: 419A0018  beq cr6, 0x82268c0c
	if ctx.cr[6].eq {
	pc = 0x82268C0C; continue 'dispatch;
	}
	// 82268BF8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82268BFC: 388BA1C4  addi r4, r11, -0x5e3c
	ctx.r[4].s64 = ctx.r[11].s64 + -24124;
	pc = 0x82268C00; continue 'dispatch;
            }
            0x82268C00 => {
    //   block [0x82268C00..0x82268C0C)
	// 82268C00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268C04: 4BFFE285  bl 0x82266e88
	ctx.lr = 0x82268C08;
	sub_82266E88(ctx, base);
	// 82268C08: 48000534  b 0x8226913c
	pc = 0x8226913C; continue 'dispatch;
            }
            0x82268C0C => {
    //   block [0x82268C0C..0x82268C68)
	// 82268C0C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82268C10: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82268C14: 556B17FF  rlwinm. r11, r11, 2, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268C18: 9B610070  stb r27, 0x70(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[27].u8 ) };
	// 82268C1C: 7F7ADB78  mr r26, r27
	ctx.r[26].u64 = ctx.r[27].u64;
	// 82268C20: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 82268C24: 40820044  bne 0x82268c68
	if !ctx.cr[0].eq {
	pc = 0x82268C68; continue 'dispatch;
	}
	// 82268C28: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268C2C: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268C30: 40820038  bne 0x82268c68
	if !ctx.cr[0].eq {
	pc = 0x82268C68; continue 'dispatch;
	}
	// 82268C34: 817F2020  lwz r11, 0x2020(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8224 as u32) ) } as u64;
	// 82268C38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82268C3C: 419A002C  beq cr6, 0x82268c68
	if ctx.cr[6].eq {
	pc = 0x82268C68; continue 'dispatch;
	}
	// 82268C40: 807F2024  lwz r3, 0x2024(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8228 as u32) ) } as u64;
	// 82268C44: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82268C48: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82268C4C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82268C50: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82268C54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82268C58: 4E800421  bctrl
	ctx.lr = 0x82268C5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82268C5C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82268C60: 40820008  bne 0x82268c68
	if !ctx.cr[0].eq {
	pc = 0x82268C68; continue 'dispatch;
	}
	// 82268C64: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
            }
            0x82268C68 => {
    //   block [0x82268C68..0x82268C7C)
	// 82268C68: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82268C6C: 409A0010  bne cr6, 0x82268c7c
	if !ctx.cr[6].eq {
	pc = 0x82268C7C; continue 'dispatch;
	}
	// 82268C70: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268C74: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82268C78: 41820008  beq 0x82268c80
	if ctx.cr[0].eq {
	pc = 0x82268C80; continue 'dispatch;
	}
	pc = 0x82268C7C; continue 'dispatch;
            }
            0x82268C7C => {
    //   block [0x82268C7C..0x82268C80)
	// 82268C7C: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	pc = 0x82268C80; continue 'dispatch;
            }
            0x82268C80 => {
    //   block [0x82268C80..0x82268C98)
	// 82268C80: 557E063E  clrlwi r30, r11, 0x18
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82268C84: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82268C88: 419A0010  beq cr6, 0x82268c98
	if ctx.cr[6].eq {
	pc = 0x82268C98; continue 'dispatch;
	}
	// 82268C8C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82268C90: 388B1D30  addi r4, r11, 0x1d30
	ctx.r[4].s64 = ctx.r[11].s64 + 7472;
	// 82268C94: 48000020  b 0x82268cb4
	pc = 0x82268CB4; continue 'dispatch;
            }
            0x82268C98 => {
    //   block [0x82268C98..0x82268CAC)
	// 82268C98: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268C9C: 41820010  beq 0x82268cac
	if ctx.cr[0].eq {
	pc = 0x82268CAC; continue 'dispatch;
	}
	// 82268CA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82268CA4: 388B1D28  addi r4, r11, 0x1d28
	ctx.r[4].s64 = ctx.r[11].s64 + 7464;
	// 82268CA8: 4800000C  b 0x82268cb4
	pc = 0x82268CB4; continue 'dispatch;
            }
            0x82268CAC => {
    //   block [0x82268CAC..0x82268CB4)
	// 82268CAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82268CB0: 388B1D3C  addi r4, r11, 0x1d3c
	ctx.r[4].s64 = ctx.r[11].s64 + 7484;
	pc = 0x82268CB4; continue 'dispatch;
            }
            0x82268CB4 => {
    //   block [0x82268CB4..0x82268CF0)
	// 82268CB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82268CB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268CBC: 4BFFE325  bl 0x82266fe0
	ctx.lr = 0x82268CC0;
	sub_82266FE0(ctx, base);
	// 82268CC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82268CC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268CC8: 388BAF40  addi r4, r11, -0x50c0
	ctx.r[4].s64 = ctx.r[11].s64 + -20672;
	// 82268CCC: 4BFFE1BD  bl 0x82266e88
	ctx.lr = 0x82268CD0;
	sub_82266E88(ctx, base);
	// 82268CD0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82268CD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268CD8: 556A035B  rlwinm. r10, r11, 0, 0xd, 0xd
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82268CDC: 5565A6BE  rlwinm r5, r11, 0x14, 0x1a, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82268CE0: 41820010  beq 0x82268cf0
	if ctx.cr[0].eq {
	pc = 0x82268CF0; continue 'dispatch;
	}
	// 82268CE4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82268CE8: 388B8B48  addi r4, r11, -0x74b8
	ctx.r[4].s64 = ctx.r[11].s64 + -29880;
	// 82268CEC: 4800000C  b 0x82268cf8
	pc = 0x82268CF8; continue 'dispatch;
            }
            0x82268CF0 => {
    //   block [0x82268CF0..0x82268CF8)
	// 82268CF0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82268CF4: 388B8B44  addi r4, r11, -0x74bc
	ctx.r[4].s64 = ctx.r[11].s64 + -29884;
	pc = 0x82268CF8; continue 'dispatch;
            }
            0x82268CF8 => {
    //   block [0x82268CF8..0x82268D34)
	// 82268CF8: 4BFFE191  bl 0x82266e88
	ctx.lr = 0x82268CFC;
	sub_82266E88(ctx, base);
	// 82268CFC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82268D00: 3D408228  lis r10, -0x7dd8
	ctx.r[10].s64 = -2111307776;
	// 82268D04: 5569077F  clrlwi. r9, r11, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82268D08: 3B8A6CF0  addi r28, r10, 0x6cf0
	ctx.r[28].s64 = ctx.r[10].s64 + 27888;
	// 82268D0C: 40820028  bne 0x82268d34
	if !ctx.cr[0].eq {
	pc = 0x82268D34; continue 'dispatch;
	}
	// 82268D10: 556A06B8  rlwinm r10, r11, 0, 0x1a, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82268D14: 2B0A0008  cmplwi cr6, r10, 8
	ctx.cr[6].compare_u32(ctx.r[10].u32, 8 as u32, &mut ctx.xer);
	// 82268D18: 409A001C  bne cr6, 0x82268d34
	if !ctx.cr[6].eq {
	pc = 0x82268D34; continue 'dispatch;
	}
	// 82268D1C: 556A05F2  rlwinm r10, r11, 0, 0x17, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82268D20: 2B0A0080  cmplwi cr6, r10, 0x80
	ctx.cr[6].compare_u32(ctx.r[10].u32, 128 as u32, &mut ctx.xer);
	// 82268D24: 409A0010  bne cr6, 0x82268d34
	if !ctx.cr[6].eq {
	pc = 0x82268D34; continue 'dispatch;
	}
	// 82268D28: 556B052C  rlwinm r11, r11, 0, 0x14, 0x16
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82268D2C: 2B0B0600  cmplwi cr6, r11, 0x600
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1536 as u32, &mut ctx.xer);
	// 82268D30: 419A008C  beq cr6, 0x82268dbc
	if ctx.cr[6].eq {
	pc = 0x82268DBC; continue 'dispatch;
	}
	pc = 0x82268D34; continue 'dispatch;
            }
            0x82268D34 => {
    //   block [0x82268D34..0x82268DBC)
	// 82268D34: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82268D38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268D3C: 388B57EC  addi r4, r11, 0x57ec
	ctx.r[4].s64 = ctx.r[11].s64 + 22508;
	// 82268D40: 4BFFE149  bl 0x82266e88
	ctx.lr = 0x82268D44;
	sub_82266E88(ctx, base);
	// 82268D44: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82268D48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268D4C: 3B0B4E44  addi r24, r11, 0x4e44
	ctx.r[24].s64 = ctx.r[11].s64 + 20036;
	// 82268D50: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82268D54: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82268D58: 556B077E  clrlwi r11, r11, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82268D5C: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82268D60: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 82268D64: 4BFFE125  bl 0x82266e88
	ctx.lr = 0x82268D68;
	sub_82266E88(ctx, base);
	// 82268D68: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82268D6C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82268D70: 556BEF7E  rlwinm r11, r11, 0x1d, 0x1d, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82268D74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268D78: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82268D7C: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 82268D80: 4BFFE109  bl 0x82266e88
	ctx.lr = 0x82268D84;
	sub_82266E88(ctx, base);
	// 82268D84: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82268D88: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82268D8C: 556BD77E  rlwinm r11, r11, 0x1a, 0x1d, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82268D90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268D94: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82268D98: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 82268D9C: 4BFFE0ED  bl 0x82266e88
	ctx.lr = 0x82268DA0;
	sub_82266E88(ctx, base);
	// 82268DA0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82268DA4: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82268DA8: 556BBF7E  rlwinm r11, r11, 0x17, 0x1d, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 82268DAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268DB0: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82268DB4: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 82268DB8: 4BFFE0D1  bl 0x82266e88
	ctx.lr = 0x82268DBC;
	sub_82266E88(ctx, base);
	pc = 0x82268DBC; continue 'dispatch;
            }
            0x82268DBC => {
    //   block [0x82268DBC..0x82268DCC)
	// 82268DBC: 57D8063F  clrlwi. r24, r30, 0x18
	ctx.r[24].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82268DC0: 4082000C  bne 0x82268dcc
	if !ctx.cr[0].eq {
	pc = 0x82268DCC; continue 'dispatch;
	}
	// 82268DC4: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268DC8: 41820070  beq 0x82268e38
	if ctx.cr[0].eq {
	pc = 0x82268E38; continue 'dispatch;
	}
	pc = 0x82268DCC; continue 'dispatch;
            }
            0x82268DCC => {
    //   block [0x82268DCC..0x82268E38)
	// 82268DCC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82268DD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268DD4: 388B4304  addi r4, r11, 0x4304
	ctx.r[4].s64 = ctx.r[11].s64 + 17156;
	// 82268DD8: 4BFFE0B1  bl 0x82266e88
	ctx.lr = 0x82268DDC;
	sub_82266E88(ctx, base);
	// 82268DDC: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82268DE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82268DE4: 9B61005F  stb r27, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[27].u8 ) };
	// 82268DE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82268DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82268DF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82268DF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82268DF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82268DFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82268E00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268E04: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82268E08: 5565DE72  rlwinm r5, r11, 0x1b, 0x19, 0x19
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82268E0C: 556BDEBE  rlwinm r11, r11, 0x1b, 0x1a, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82268E10: 7CA55B78  or r5, r5, r11
	ctx.r[5].u64 = ctx.r[5].u64 | ctx.r[11].u64;
	// 82268E14: 4BFFE6B5  bl 0x822674c8
	ctx.lr = 0x82268E18;
	sub_822674C8(ctx, base);
	// 82268E18: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82268E1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268E20: 388B8C90  addi r4, r11, -0x7370
	ctx.r[4].s64 = ctx.r[11].s64 + -29552;
	// 82268E24: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82268E28: 556B17BE  srwi r11, r11, 0x1e
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(30);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82268E2C: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82268E30: 7D650774  extsb r5, r11
	ctx.r[5].s64 = ctx.r[11].s8 as i64;
	// 82268E34: 4BFFE055  bl 0x82266e88
	ctx.lr = 0x82268E38;
	sub_82266E88(ctx, base);
	pc = 0x82268E38; continue 'dispatch;
            }
            0x82268E38 => {
    //   block [0x82268E38..0x82268E68)
	// 82268E38: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82268E3C: 419A002C  beq cr6, 0x82268e68
	if ctx.cr[6].eq {
	pc = 0x82268E68; continue 'dispatch;
	}
	// 82268E40: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82268E44: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82268E48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268E4C: 556966FE  rlwinm r9, r11, 0xc, 0x1b, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	// 82268E50: 556B3FBE  rlwinm r11, r11, 7, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x01FFFFFFu64;
	// 82268E54: 1D290003  mulli r9, r9, 3
	ctx.r[9].s32 = ((ctx.r[9].s32 as i64 * 3 as i64) as i32);
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 82268E58: 2129005F  subfic r9, r9, 0x5f
	ctx.xer.ca = ctx.r[9].u32 <= 95 as u32;
	ctx.r[9].s64 = (95 as i64) - ctx.r[9].s64;
	// 82268E5C: 388AA1BC  addi r4, r10, -0x5e44
	ctx.r[4].s64 = ctx.r[10].s64 + -24132;
	// 82268E60: 7CAB4850  subf r5, r11, r9
	ctx.r[5].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82268E64: 4BFFE025  bl 0x82266e88
	ctx.lr = 0x82268E68;
	sub_82266E88(ctx, base);
	pc = 0x82268E68; continue 'dispatch;
            }
            0x82268E68 => {
    //   block [0x82268E68..0x82268EB0)
	// 82268E68: 575A063F  clrlwi. r26, r26, 0x18
	ctx.r[26].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82268E6C: 41820044  beq 0x82268eb0
	if ctx.cr[0].eq {
	pc = 0x82268EB0; continue 'dispatch;
	}
	// 82268E70: 89610070  lbz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82268E74: 395C0320  addi r10, r28, 0x320
	ctx.r[10].s64 = ctx.r[28].s64 + 800;
	// 82268E78: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82268E7C: 5568F0BA  rlwinm r8, r11, 0x1e, 2, 0x1d
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82268E80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268E84: 38897104  addi r4, r9, 0x7104
	ctx.r[4].s64 = ctx.r[9].s64 + 28932;
	// 82268E88: 557E073E  clrlwi r30, r11, 0x1c
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82268E8C: 7CA8502E  lwzx r5, r8, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82268E90: 4BFFDFF9  bl 0x82266e88
	ctx.lr = 0x82268E94;
	sub_82266E88(ctx, base);
	// 82268E94: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82268E98: 419A0018  beq cr6, 0x82268eb0
	if ctx.cr[6].eq {
	pc = 0x82268EB0; continue 'dispatch;
	}
	// 82268E9C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82268EA0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82268EA4: 388B51AC  addi r4, r11, 0x51ac
	ctx.r[4].s64 = ctx.r[11].s64 + 20908;
	// 82268EA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268EAC: 4BFFDFDD  bl 0x82266e88
	ctx.lr = 0x82268EB0;
	sub_82266E88(ctx, base);
	pc = 0x82268EB0; continue 'dispatch;
            }
            0x82268EB0 => {
    //   block [0x82268EB0..0x82268EDC)
	// 82268EB0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82268EB4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82268EB8: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82268EBC: 3BCBFE08  addi r30, r11, -0x1f8
	ctx.r[30].s64 = ctx.r[11].s64 + -504;
	// 82268EC0: 3B6AFE10  addi r27, r10, -0x1f0
	ctx.r[27].s64 = ctx.r[10].s64 + -496;
	// 82268EC4: 409A0084  bne cr6, 0x82268f48
	if !ctx.cr[6].eq {
	pc = 0x82268F48; continue 'dispatch;
	}
	// 82268EC8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268ECC: 556BD7FF  rlwinm. r11, r11, 0x1a, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268ED0: 4082000C  bne 0x82268edc
	if !ctx.cr[0].eq {
	pc = 0x82268EDC; continue 'dispatch;
	}
	// 82268ED4: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82268ED8: 409A002C  bne cr6, 0x82268f04
	if !ctx.cr[6].eq {
	pc = 0x82268F04; continue 'dispatch;
	}
	pc = 0x82268EDC; continue 'dispatch;
            }
            0x82268EDC => {
    //   block [0x82268EDC..0x82268EF4)
	// 82268EDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268EE0: 4BFFF3F9  bl 0x822682d8
	ctx.lr = 0x82268EE4;
	sub_822682D8(ctx, base);
	// 82268EE4: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82268EE8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82268EEC: 419A0008  beq cr6, 0x82268ef4
	if ctx.cr[6].eq {
	pc = 0x82268EF4; continue 'dispatch;
	}
	// 82268EF0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	pc = 0x82268EF4; continue 'dispatch;
            }
            0x82268EF4 => {
    //   block [0x82268EF4..0x82268F04)
	// 82268EF4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82268EF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268EFC: 388BA1A8  addi r4, r11, -0x5e58
	ctx.r[4].s64 = ctx.r[11].s64 + -24152;
	// 82268F00: 4BFFDF89  bl 0x82266e88
	ctx.lr = 0x82268F04;
	sub_82266E88(ctx, base);
	pc = 0x82268F04; continue 'dispatch;
            }
            0x82268F04 => {
    //   block [0x82268F04..0x82268F1C)
	// 82268F04: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268F08: 556BD7FF  rlwinm. r11, r11, 0x1a, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268F0C: 40820010  bne 0x82268f1c
	if !ctx.cr[0].eq {
	pc = 0x82268F1C; continue 'dispatch;
	}
	// 82268F10: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82268F14: 556B0421  rlwinm. r11, r11, 0, 0x10, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268F18: 41820030  beq 0x82268f48
	if ctx.cr[0].eq {
	pc = 0x82268F48; continue 'dispatch;
	}
	pc = 0x82268F1C; continue 'dispatch;
            }
            0x82268F1C => {
    //   block [0x82268F1C..0x82268F38)
	// 82268F1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268F20: 4BFFF3B9  bl 0x822682d8
	ctx.lr = 0x82268F24;
	sub_822682D8(ctx, base);
	// 82268F24: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82268F28: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82268F2C: 556B0421  rlwinm. r11, r11, 0, 0x10, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268F30: 40820008  bne 0x82268f38
	if !ctx.cr[0].eq {
	pc = 0x82268F38; continue 'dispatch;
	}
	// 82268F34: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	pc = 0x82268F38; continue 'dispatch;
            }
            0x82268F38 => {
    //   block [0x82268F38..0x82268F48)
	// 82268F38: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82268F3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268F40: 388BA198  addi r4, r11, -0x5e68
	ctx.r[4].s64 = ctx.r[11].s64 + -24168;
	// 82268F44: 4BFFDF45  bl 0x82266e88
	ctx.lr = 0x82268F48;
	sub_82266E88(ctx, base);
	pc = 0x82268F48; continue 'dispatch;
            }
            0x82268F48 => {
    //   block [0x82268F48..0x82268F60)
	// 82268F48: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268F4C: 556BD7FF  rlwinm. r11, r11, 0x1a, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268F50: 40820010  bne 0x82268f60
	if !ctx.cr[0].eq {
	pc = 0x82268F60; continue 'dispatch;
	}
	// 82268F54: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82268F58: 556B008F  rlwinm. r11, r11, 0, 2, 7
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268F5C: 41820028  beq 0x82268f84
	if ctx.cr[0].eq {
	pc = 0x82268F84; continue 'dispatch;
	}
	pc = 0x82268F60; continue 'dispatch;
            }
            0x82268F60 => {
    //   block [0x82268F60..0x82268F84)
	// 82268F60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268F64: 4BFFF375  bl 0x822682d8
	ctx.lr = 0x82268F68;
	sub_822682D8(ctx, base);
	// 82268F68: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82268F6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268F70: 388BA188  addi r4, r11, -0x5e78
	ctx.r[4].s64 = ctx.r[11].s64 + -24184;
	// 82268F74: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82268F78: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82268F7C: 7D65D670  srawi r5, r11, 0x1a
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 26) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[11].s32 >> 26) as i64;
	// 82268F80: 4BFFDF09  bl 0x82266e88
	ctx.lr = 0x82268F84;
	sub_82266E88(ctx, base);
	pc = 0x82268F84; continue 'dispatch;
            }
            0x82268F84 => {
    //   block [0x82268F84..0x82268FB4)
	// 82268F84: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82268F88: 409A01B4  bne cr6, 0x8226913c
	if !ctx.cr[6].eq {
	pc = 0x8226913C; continue 'dispatch;
	}
	// 82268F8C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82268F90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268F94: 388B5830  addi r4, r11, 0x5830
	ctx.r[4].s64 = ctx.r[11].s64 + 22576;
	// 82268F98: 4BFFDFF1  bl 0x82266f88
	ctx.lr = 0x82268F9C;
	sub_82266F88(ctx, base);
	// 82268F9C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268FA0: 556BD7FF  rlwinm. r11, r11, 0x1a, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268FA4: 40820010  bne 0x82268fb4
	if !ctx.cr[0].eq {
	pc = 0x82268FB4; continue 'dispatch;
	}
	// 82268FA8: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268FAC: 556B006F  rlwinm. r11, r11, 0, 1, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268FB0: 41820028  beq 0x82268fd8
	if ctx.cr[0].eq {
	pc = 0x82268FD8; continue 'dispatch;
	}
	pc = 0x82268FB4; continue 'dispatch;
            }
            0x82268FB4 => {
    //   block [0x82268FB4..0x82268FD8)
	// 82268FB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268FB8: 4BFFF321  bl 0x822682d8
	ctx.lr = 0x82268FBC;
	sub_822682D8(ctx, base);
	// 82268FBC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82268FC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268FC4: 388BA17C  addi r4, r11, -0x5e84
	ctx.r[4].s64 = ctx.r[11].s64 + -24196;
	// 82268FC8: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268FCC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82268FD0: 7D654E70  srawi r5, r11, 9
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 9) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[11].s32 >> 9) as i64;
	// 82268FD4: 4BFFDEB5  bl 0x82266e88
	ctx.lr = 0x82268FD8;
	sub_82266E88(ctx, base);
	pc = 0x82268FD8; continue 'dispatch;
            }
            0x82268FD8 => {
    //   block [0x82268FD8..0x82268FF0)
	// 82268FD8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82268FDC: 556BD7FF  rlwinm. r11, r11, 0x1a, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268FE0: 40820010  bne 0x82268ff0
	if !ctx.cr[0].eq {
	pc = 0x82268FF0; continue 'dispatch;
	}
	// 82268FE4: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82268FE8: 556B06BF  clrlwi. r11, r11, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82268FEC: 4182002C  beq 0x82269018
	if ctx.cr[0].eq {
	pc = 0x82269018; continue 'dispatch;
	}
	pc = 0x82268FF0; continue 'dispatch;
            }
            0x82268FF0 => {
    //   block [0x82268FF0..0x82269018)
	// 82268FF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82268FF4: 4BFFF2E5  bl 0x822682d8
	ctx.lr = 0x82268FF8;
	sub_822682D8(ctx, base);
	// 82268FF8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82268FFC: 397C0210  addi r11, r28, 0x210
	ctx.r[11].s64 = ctx.r[28].s64 + 528;
	// 82269000: 388AA16C  addi r4, r10, -0x5e94
	ctx.r[4].s64 = ctx.r[10].s64 + -24212;
	// 82269004: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269008: A15D0004  lhz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226900C: 554A163A  rlwinm r10, r10, 2, 0x18, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82269010: 7CAA582E  lwzx r5, r10, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82269014: 4BFFDE75  bl 0x82266e88
	ctx.lr = 0x82269018;
	sub_82266E88(ctx, base);
	pc = 0x82269018; continue 'dispatch;
            }
            0x82269018 => {
    //   block [0x82269018..0x82269038)
	// 82269018: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 8226901C: 419A0038  beq cr6, 0x82269054
	if ctx.cr[6].eq {
	pc = 0x82269054; continue 'dispatch;
	}
	// 82269020: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82269024: 556BD7FF  rlwinm. r11, r11, 0x1a, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269028: 40820010  bne 0x82269038
	if !ctx.cr[0].eq {
	pc = 0x82269038; continue 'dispatch;
	}
	// 8226902C: 897D000B  lbz r11, 0xb(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(11 as u32) ) } as u64;
	// 82269030: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82269034: 41820020  beq 0x82269054
	if ctx.cr[0].eq {
	pc = 0x82269054; continue 'dispatch;
	}
	pc = 0x82269038; continue 'dispatch;
            }
            0x82269038 => {
    //   block [0x82269038..0x82269054)
	// 82269038: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226903C: 4BFFF29D  bl 0x822682d8
	ctx.lr = 0x82269040;
	sub_822682D8(ctx, base);
	// 82269040: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82269044: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269048: 88BD000B  lbz r5, 0xb(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(11 as u32) ) } as u64;
	// 8226904C: 388BA160  addi r4, r11, -0x5ea0
	ctx.r[4].s64 = ctx.r[11].s64 + -24224;
	// 82269050: 4BFFDE39  bl 0x82266e88
	ctx.lr = 0x82269054;
	sub_82266E88(ctx, base);
	pc = 0x82269054; continue 'dispatch;
            }
            0x82269054 => {
    //   block [0x82269054..0x8226906C)
	// 82269054: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82269058: 556BD7FF  rlwinm. r11, r11, 0x1a, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226905C: 40820010  bne 0x8226906c
	if !ctx.cr[0].eq {
	pc = 0x8226906C; continue 'dispatch;
	}
	// 82269060: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82269064: 556B04E7  rlwinm. r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269068: 41820030  beq 0x82269098
	if ctx.cr[0].eq {
	pc = 0x82269098; continue 'dispatch;
	}
	pc = 0x8226906C; continue 'dispatch;
            }
            0x8226906C => {
    //   block [0x8226906C..0x82269088)
	// 8226906C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269070: 4BFFF269  bl 0x822682d8
	ctx.lr = 0x82269074;
	sub_822682D8(ctx, base);
	// 82269074: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82269078: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8226907C: 556B04E7  rlwinm. r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269080: 40820008  bne 0x82269088
	if !ctx.cr[0].eq {
	pc = 0x82269088; continue 'dispatch;
	}
	// 82269084: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	pc = 0x82269088; continue 'dispatch;
            }
            0x82269088 => {
    //   block [0x82269088..0x82269098)
	// 82269088: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8226908C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269090: 388BA154  addi r4, r11, -0x5eac
	ctx.r[4].s64 = ctx.r[11].s64 + -24236;
	// 82269094: 4BFFDDF5  bl 0x82266e88
	ctx.lr = 0x82269098;
	sub_82266E88(ctx, base);
	pc = 0x82269098; continue 'dispatch;
            }
            0x82269098 => {
    //   block [0x82269098..0x822690B0)
	// 82269098: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8226909C: 556BD7FF  rlwinm. r11, r11, 0x1a, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822690A0: 40820010  bne 0x822690b0
	if !ctx.cr[0].eq {
	pc = 0x822690B0; continue 'dispatch;
	}
	// 822690A4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822690A8: 556B04A5  rlwinm. r11, r11, 0, 0x12, 0x12
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822690AC: 4182003C  beq 0x822690e8
	if ctx.cr[0].eq {
	pc = 0x822690E8; continue 'dispatch;
	}
	pc = 0x822690B0; continue 'dispatch;
            }
            0x822690B0 => {
    //   block [0x822690B0..0x822690D0)
	// 822690B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822690B4: 4BFFF225  bl 0x822682d8
	ctx.lr = 0x822690B8;
	sub_822682D8(ctx, base);
	// 822690B8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822690BC: 556B04A5  rlwinm. r11, r11, 0, 0x12, 0x12
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822690C0: 41820010  beq 0x822690d0
	if ctx.cr[0].eq {
	pc = 0x822690D0; continue 'dispatch;
	}
	// 822690C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822690C8: 38AB1840  addi r5, r11, 0x1840
	ctx.r[5].s64 = ctx.r[11].s64 + 6208;
	// 822690CC: 4800000C  b 0x822690d8
	pc = 0x822690D8; continue 'dispatch;
            }
            0x822690D0 => {
    //   block [0x822690D0..0x822690D8)
	// 822690D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822690D4: 38AB1848  addi r5, r11, 0x1848
	ctx.r[5].s64 = ctx.r[11].s64 + 6216;
	pc = 0x822690D8; continue 'dispatch;
            }
            0x822690D8 => {
    //   block [0x822690D8..0x822690E8)
	// 822690D8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 822690DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822690E0: 388BA144  addi r4, r11, -0x5ebc
	ctx.r[4].s64 = ctx.r[11].s64 + -24252;
	// 822690E4: 4BFFDDA5  bl 0x82266e88
	ctx.lr = 0x822690E8;
	sub_82266E88(ctx, base);
	pc = 0x822690E8; continue 'dispatch;
            }
            0x822690E8 => {
    //   block [0x822690E8..0x82269108)
	// 822690E8: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 822690EC: 419A0040  beq cr6, 0x8226912c
	if ctx.cr[6].eq {
	pc = 0x8226912C; continue 'dispatch;
	}
	// 822690F0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822690F4: 556BD7FF  rlwinm. r11, r11, 0x1a, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822690F8: 40820010  bne 0x82269108
	if !ctx.cr[0].eq {
	pc = 0x82269108; continue 'dispatch;
	}
	// 822690FC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269100: 556B0089  rlwinm. r11, r11, 0, 2, 4
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269104: 41820028  beq 0x8226912c
	if ctx.cr[0].eq {
	pc = 0x8226912C; continue 'dispatch;
	}
	pc = 0x82269108; continue 'dispatch;
            }
            0x82269108 => {
    //   block [0x82269108..0x8226912C)
	// 82269108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226910C: 4BFFF1CD  bl 0x822682d8
	ctx.lr = 0x82269110;
	sub_822682D8(ctx, base);
	// 82269110: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82269114: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269118: 388BA130  addi r4, r11, -0x5ed0
	ctx.r[4].s64 = ctx.r[11].s64 + -24272;
	// 8226911C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269120: 556B2F7E  rlwinm r11, r11, 5, 0x1d, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x07FFFFFFu64;
	// 82269124: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82269128: 4BFFDD61  bl 0x82266e88
	ctx.lr = 0x8226912C;
	sub_82266E88(ctx, base);
	pc = 0x8226912C; continue 'dispatch;
            }
            0x8226912C => {
    //   block [0x8226912C..0x8226913C)
	// 8226912C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82269130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269134: 388B5828  addi r4, r11, 0x5828
	ctx.r[4].s64 = ctx.r[11].s64 + 22568;
	// 82269138: 4BFFDE51  bl 0x82266f88
	ctx.lr = 0x8226913C;
	sub_82266F88(ctx, base);
	pc = 0x8226913C; continue 'dispatch;
            }
            0x8226913C => {
    //   block [0x8226913C..0x82269148)
	// 8226913C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82269140: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 82269144: 4BE25990  b 0x8208ead4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82269148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82269148 size=3096
    let mut pc: u32 = 0x82269148;
    'dispatch: loop {
        match pc {
            0x82269148 => {
    //   block [0x82269148..0x822691C4)
	// 82269148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226914C: 4BE25931  bl 0x8208ea7c
	ctx.lr = 0x82269150;
	sub_8208EA60(ctx, base);
	// 82269150: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82269154: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82269158: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226915C: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82269160: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82269164: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269168: 41820884  beq 0x822699ec
	if ctx.cr[0].eq {
	pc = 0x822699EC; continue 'dispatch;
	}
	// 8226916C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82269170: 895F000C  lbz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82269174: 557EA73E  rlwinm r30, r11, 0x14, 0x1c, 0x1f
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82269178: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8226917C: 418200A0  beq 0x8226921c
	if ctx.cr[0].eq {
	pc = 0x8226921C; continue 'dispatch;
	}
	// 82269180: 4BFFEFD9  bl 0x82268158
	ctx.lr = 0x82269184;
	sub_82268158(ctx, base);
	// 82269184: 56CBD97E  srwi r11, r22, 5
	ctx.r[11].u32 = ctx.r[22].u32.wrapping_shr(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82269188: 56CA06FE  clrlwi r10, r22, 0x1b
	ctx.r[10].u64 = ctx.r[22].u32 as u64 & 0x0000001Fu64;
	// 8226918C: 396B0006  addi r11, r11, 6
	ctx.r[11].s64 = ctx.r[11].s64 + 6;
	// 82269190: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82269194: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82269198: 7D2A5030  slw r10, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8226919C: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 822691A0: 7D4B5839  and. r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822691A4: 41820058  beq 0x822691fc
	if ctx.cr[0].eq {
	pc = 0x822691FC; continue 'dispatch;
	}
	// 822691A8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822691AC: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822691B0: 41820014  beq 0x822691c4
	if ctx.cr[0].eq {
	pc = 0x822691C4; continue 'dispatch;
	}
	// 822691B4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 822691B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822691BC: 388BA2B0  addi r4, r11, -0x5d50
	ctx.r[4].s64 = ctx.r[11].s64 + -23888;
	// 822691C0: 4BFFDCC9  bl 0x82266e88
	ctx.lr = 0x822691C4;
	sub_82266E88(ctx, base);
	pc = 0x822691C4; continue 'dispatch;
            }
            0x822691C4 => {
    //   block [0x822691C4..0x822691FC)
	// 822691C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822691C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822691CC: 388BC93C  addi r4, r11, -0x36c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14020;
	// 822691D0: 4BFFDE81  bl 0x82267050
	ctx.lr = 0x822691D4;
	sub_82267050(ctx, base);
	// 822691D4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 822691D8: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 822691DC: 388BA2A8  addi r4, r11, -0x5d58
	ctx.r[4].s64 = ctx.r[11].s64 + -23896;
	// 822691E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822691E4: 4BFFDCA5  bl 0x82266e88
	ctx.lr = 0x822691E8;
	sub_82266E88(ctx, base);
	// 822691E8: 897F2028  lbz r11, 0x2028(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8232 as u32) ) } as u64;
	// 822691EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822691F0: 4082000C  bne 0x822691fc
	if !ctx.cr[0].eq {
	pc = 0x822691FC; continue 'dispatch;
	}
	// 822691F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822691F8: 4BFFEF09  bl 0x82268100
	ctx.lr = 0x822691FC;
	sub_82268100(ctx, base);
	pc = 0x822691FC; continue 'dispatch;
            }
            0x822691FC => {
    //   block [0x822691FC..0x82269210)
	// 822691FC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82269200: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269204: 4082000C  bne 0x82269210
	if !ctx.cr[0].eq {
	pc = 0x82269210; continue 'dispatch;
	}
	// 82269208: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8226920C: 419A0010  beq cr6, 0x8226921c
	if ctx.cr[6].eq {
	pc = 0x8226921C; continue 'dispatch;
	}
	pc = 0x82269210; continue 'dispatch;
            }
            0x82269210 => {
    //   block [0x82269210..0x8226921C)
	// 82269210: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82269214: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269218: 4BFFDEA1  bl 0x822670b8
	ctx.lr = 0x8226921C;
	sub_822670B8(ctx, base);
	pc = 0x8226921C; continue 'dispatch;
            }
            0x8226921C => {
    //   block [0x8226921C..0x8226927C)
	// 8226921C: 2B1E000F  cmplwi cr6, r30, 0xf
	ctx.cr[6].compare_u32(ctx.r[30].u32, 15 as u32, &mut ctx.xer);
	// 82269220: 41990788  bgt cr6, 0x822699a8
	if ctx.cr[6].gt {
	pc = 0x822699A8; continue 'dispatch;
	}
	// 82269224: 3D808205  lis r12, -0x7dfb
	ctx.r[12].s64 = -2113601536;
	// 82269228: 398CA058  addi r12, r12, -0x5fa8
	ctx.r[12].s64 = ctx.r[12].s64 + -24488;
	// 8226922C: 57C0083C  slwi r0, r30, 1
	ctx.r[0].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82269230: 7C0C022E  lhzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82269234: 3D808227  lis r12, -0x7dd9
	ctx.r[12].s64 = -2111373312;
	// 82269238: 398C924C  addi r12, r12, -0x6db4
	ctx.r[12].s64 = ctx.r[12].s64 + -28084;
	// 8226923C: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82269240: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82269244: 60000000  nop
	// 82269248: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 8226924C: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82269250: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82269254: 41820AEC  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 82269258: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8226925C: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269260: 41820AE0  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 82269264: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269268: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226926C: 388B70F0  addi r4, r11, 0x70f0
	ctx.r[4].s64 = ctx.r[11].s64 + 28912;
	// 82269270: 4BFFDC19  bl 0x82266e88
	ctx.lr = 0x82269274;
	sub_82266E88(ctx, base);
	// 82269274: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269278: 388B1C00  addi r4, r11, 0x1c00
	ctx.r[4].s64 = ctx.r[11].s64 + 7168;
            }
            0x8226927C => {
    //   block [0x8226927C..0x8226929C)
	// 8226927C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269280: 4BFFDDD1  bl 0x82267050
	ctx.lr = 0x82269284;
	sub_82267050(ctx, base);
	// 82269284: 48000ABC  b 0x82269d40
	pc = 0x82269D40; continue 'dispatch;
	// 82269288: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8226928C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82269290: 41820AB0  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 82269294: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269298: 388B70F0  addi r4, r11, 0x70f0
	ctx.r[4].s64 = ctx.r[11].s64 + 28912;
            }
            0x8226929C => {
    //   block [0x8226929C..0x822692AC)
	// 8226929C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822692A0: 4BFFDBE9  bl 0x82266e88
	ctx.lr = 0x822692A4;
	sub_82266E88(ctx, base);
	// 822692A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822692A8: 388B1BF8  addi r4, r11, 0x1bf8
	ctx.r[4].s64 = ctx.r[11].s64 + 7160;
	pc = 0x822692AC; continue 'dispatch;
            }
            0x822692AC => {
    //   block [0x822692AC..0x822692C8)
	// 822692AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822692B0: 4BFFDDA1  bl 0x82267050
	ctx.lr = 0x822692B4;
	sub_82267050(ctx, base);
	// 822692B4: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822692B8: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822692BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822692C0: 5565BFFE  rlwinm r5, r11, 0x17, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 822692C4: 55448FFE  rlwinm r4, r10, 0x11, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x00007FFFu64;
	pc = 0x822692C8; continue 'dispatch;
            }
            0x822692C8 => {
    //   block [0x822692C8..0x822692E8)
	// 822692C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822692CC: 4BFFDC0D  bl 0x82266ed8
	ctx.lr = 0x822692D0;
	sub_82266ED8(ctx, base);
	// 822692D0: 48000A70  b 0x82269d40
	pc = 0x82269D40; continue 'dispatch;
	// 822692D4: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822692D8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822692DC: 41820A64  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 822692E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822692E4: 388B70F0  addi r4, r11, 0x70f0
	ctx.r[4].s64 = ctx.r[11].s64 + 28912;
            }
            0x822692E8 => {
    //   block [0x822692E8..0x82269320)
	// 822692E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822692EC: 4BFFDB9D  bl 0x82266e88
	ctx.lr = 0x822692F0;
	sub_82266E88(ctx, base);
	// 822692F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822692F4: 388B1BF0  addi r4, r11, 0x1bf0
	ctx.r[4].s64 = ctx.r[11].s64 + 7152;
	// 822692F8: 4BFFFFB4  b 0x822692ac
	pc = 0x822692AC; continue 'dispatch;
	// 822692FC: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82269300: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82269304: 41820A3C  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 82269308: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8226930C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269310: 388B70F0  addi r4, r11, 0x70f0
	ctx.r[4].s64 = ctx.r[11].s64 + 28912;
	// 82269314: 4BFFDB75  bl 0x82266e88
	ctx.lr = 0x82269318;
	sub_82266E88(ctx, base);
	// 82269318: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8226931C: 388B1BE8  addi r4, r11, 0x1be8
	ctx.r[4].s64 = ctx.r[11].s64 + 7144;
            }
            0x82269320 => {
    //   block [0x82269320..0x82269354)
	// 82269320: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269324: 4BFFDD2D  bl 0x82267050
	ctx.lr = 0x82269328;
	sub_82267050(ctx, base);
	// 82269328: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8226932C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269330: 388BAF40  addi r4, r11, -0x50c0
	ctx.r[4].s64 = ctx.r[11].s64 + -20672;
	// 82269334: 4BFFDB55  bl 0x82266e88
	ctx.lr = 0x82269338;
	sub_82266E88(ctx, base);
	// 82269338: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226933C: 556B056B  rlwinm. r11, r11, 0, 0x15, 0x15
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269340: 40820014  bne 0x82269354
	if !ctx.cr[0].eq {
	pc = 0x82269354; continue 'dispatch;
	}
	// 82269344: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269348: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226934C: 388BACC4  addi r4, r11, -0x533c
	ctx.r[4].s64 = ctx.r[11].s64 + -21308;
	// 82269350: 4BFFDB39  bl 0x82266e88
	ctx.lr = 0x82269354;
	sub_82266E88(ctx, base);
	pc = 0x82269354; continue 'dispatch;
            }
            0x82269354 => {
    //   block [0x82269354..0x82269370)
	// 82269354: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82269358: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8226935C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269360: 388A904C  addi r4, r10, -0x6fb4
	ctx.r[4].s64 = ctx.r[10].s64 + -28596;
	// 82269364: 5565F63E  rlwinm r5, r11, 0x1e, 0x18, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82269368: 4BFFDB21  bl 0x82266e88
	ctx.lr = 0x8226936C;
	sub_82266E88(ctx, base);
	// 8226936C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	pc = 0x82269370; continue 'dispatch;
            }
            0x82269370 => {
    //   block [0x82269370..0x822693CC)
	// 82269370: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269374: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82269378: 55648FFE  rlwinm r4, r11, 0x11, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	// 8226937C: 4BFFFF4C  b 0x822692c8
	pc = 0x822692C8; continue 'dispatch;
	// 82269380: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82269384: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82269388: 418209B8  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 8226938C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269390: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269394: 388B70F0  addi r4, r11, 0x70f0
	ctx.r[4].s64 = ctx.r[11].s64 + 28912;
	// 82269398: 4BFFDAF1  bl 0x82266e88
	ctx.lr = 0x8226939C;
	sub_82266E88(ctx, base);
	// 8226939C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822693A0: 388B1BE0  addi r4, r11, 0x1be0
	ctx.r[4].s64 = ctx.r[11].s64 + 7136;
	// 822693A4: 4BFFFF7C  b 0x82269320
	pc = 0x82269320; continue 'dispatch;
	// 822693A8: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822693AC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822693B0: 41820990  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 822693B4: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822693B8: 556B056B  rlwinm. r11, r11, 0, 0x15, 0x15
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822693BC: 41820010  beq 0x822693cc
	if ctx.cr[0].eq {
	pc = 0x822693CC; continue 'dispatch;
	}
	// 822693C0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 822693C4: 388BA0A8  addi r4, r11, -0x5f58
	ctx.r[4].s64 = ctx.r[11].s64 + -24408;
	// 822693C8: 4BFFFED4  b 0x8226929c
	pc = 0x8226929C; continue 'dispatch;
            }
            0x822693CC => {
    //   block [0x822693CC..0x822693FC)
	// 822693CC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 822693D0: 388BA0A0  addi r4, r11, -0x5f60
	ctx.r[4].s64 = ctx.r[11].s64 + -24416;
	// 822693D4: 4BFFFEC8  b 0x8226929c
	pc = 0x8226929C; continue 'dispatch;
	// 822693D8: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822693DC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822693E0: 41820960  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 822693E4: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822693E8: 556B056B  rlwinm. r11, r11, 0, 0x15, 0x15
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822693EC: 41820010  beq 0x822693fc
	if ctx.cr[0].eq {
	pc = 0x822693FC; continue 'dispatch;
	}
	// 822693F0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 822693F4: 388BA0A8  addi r4, r11, -0x5f58
	ctx.r[4].s64 = ctx.r[11].s64 + -24408;
	// 822693F8: 4BFFFEF0  b 0x822692e8
	pc = 0x822692E8; continue 'dispatch;
            }
            0x822693FC => {
    //   block [0x822693FC..0x82269490)
	// 822693FC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82269400: 388BA0A0  addi r4, r11, -0x5f60
	ctx.r[4].s64 = ctx.r[11].s64 + -24416;
	// 82269404: 4BFFFEE4  b 0x822692e8
	pc = 0x822692E8; continue 'dispatch;
	// 82269408: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8226940C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82269410: 41820930  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 82269414: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226941C: 388B70F0  addi r4, r11, 0x70f0
	ctx.r[4].s64 = ctx.r[11].s64 + 28912;
	// 82269420: 4BFFDA69  bl 0x82266e88
	ctx.lr = 0x82269424;
	sub_82266E88(ctx, base);
	// 82269424: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269428: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226942C: 388BC950  addi r4, r11, -0x36b0
	ctx.r[4].s64 = ctx.r[11].s64 + -14000;
	// 82269430: 4BFFDC21  bl 0x82267050
	ctx.lr = 0x82269434;
	sub_82267050(ctx, base);
	// 82269434: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226943C: 388BAF40  addi r4, r11, -0x50c0
	ctx.r[4].s64 = ctx.r[11].s64 + -20672;
	// 82269440: 4BFFDA49  bl 0x82266e88
	ctx.lr = 0x82269444;
	sub_82266E88(ctx, base);
	// 82269444: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82269448: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226944C: 388BA2A0  addi r4, r11, -0x5d60
	ctx.r[4].s64 = ctx.r[11].s64 + -23904;
	// 82269450: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269454: 556506FE  clrlwi r5, r11, 0x1b
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82269458: 4BFFDA31  bl 0x82266e88
	ctx.lr = 0x8226945C;
	sub_82266E88(ctx, base);
	// 8226945C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82269460: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269464: 388BA0F4  addi r4, r11, -0x5f0c
	ctx.r[4].s64 = ctx.r[11].s64 + -24332;
	// 82269468: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226946C: 556504FE  clrlwi r5, r11, 0x13
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x00001FFFu64;
	// 82269470: 4BFFDA19  bl 0x82266e88
	ctx.lr = 0x82269474;
	sub_82266E88(ctx, base);
	// 82269474: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82269478: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226947C: 5569D7FF  rlwinm. r9, r11, 0x1a, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82269480: 554B9FFE  rlwinm r11, r10, 0x13, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00001FFFu64;
	// 82269484: 4082000C  bne 0x82269490
	if !ctx.cr[0].eq {
	pc = 0x82269490; continue 'dispatch;
	}
	// 82269488: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226948C: 419A0030  beq cr6, 0x822694bc
	if ctx.cr[6].eq {
	pc = 0x822694BC; continue 'dispatch;
	}
            }
            0x82269490 => {
    //   block [0x82269490..0x822694A4)
	// 82269490: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269494: 419A0010  beq cr6, 0x822694a4
	if ctx.cr[6].eq {
	pc = 0x822694A4; continue 'dispatch;
	}
	// 82269498: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8226949C: 38ABFE10  addi r5, r11, -0x1f0
	ctx.r[5].s64 = ctx.r[11].s64 + -496;
	// 822694A0: 4800000C  b 0x822694ac
	pc = 0x822694AC; continue 'dispatch;
            }
            0x822694A4 => {
    //   block [0x822694A4..0x822694AC)
	// 822694A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822694A8: 38ABFE08  addi r5, r11, -0x1f8
	ctx.r[5].s64 = ctx.r[11].s64 + -504;
	pc = 0x822694AC; continue 'dispatch;
            }
            0x822694AC => {
    //   block [0x822694AC..0x822694BC)
	// 822694AC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 822694B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822694B4: 388BA294  addi r4, r11, -0x5d6c
	ctx.r[4].s64 = ctx.r[11].s64 + -23916;
	// 822694B8: 4BFFD9D1  bl 0x82266e88
	ctx.lr = 0x822694BC;
	sub_82266E88(ctx, base);
	pc = 0x822694BC; continue 'dispatch;
            }
            0x822694BC => {
    //   block [0x822694BC..0x82269504)
	// 822694BC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822694C0: 5565AFFF  rlwinm. r5, r11, 0x15, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x000007FFu64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 822694C4: 4182087C  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 822694C8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822694CC: 388B8F98  addi r4, r11, -0x7068
	ctx.r[4].s64 = ctx.r[11].s64 + -28776;
	// 822694D0: 48000510  b 0x822699e0
	pc = 0x822699E0; continue 'dispatch;
	// 822694D4: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822694D8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822694DC: 41820864  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 822694E0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822694E4: 556B0295  rlwinm. r11, r11, 0, 0xa, 0xa
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822694E8: 41820028  beq 0x82269510
	if ctx.cr[0].eq {
	pc = 0x82269510; continue 'dispatch;
	}
	// 822694EC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822694F0: 556B056B  rlwinm. r11, r11, 0, 0x15, 0x15
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822694F4: 41820010  beq 0x82269504
	if ctx.cr[0].eq {
	pc = 0x82269504; continue 'dispatch;
	}
	// 822694F8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 822694FC: 388BA0A8  addi r4, r11, -0x5f58
	ctx.r[4].s64 = ctx.r[11].s64 + -24408;
	// 82269500: 48000018  b 0x82269518
	pc = 0x82269518; continue 'dispatch;
            }
            0x82269504 => {
    //   block [0x82269504..0x82269510)
	// 82269504: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82269508: 388BA0A0  addi r4, r11, -0x5f60
	ctx.r[4].s64 = ctx.r[11].s64 + -24416;
	// 8226950C: 4800000C  b 0x82269518
	pc = 0x82269518; continue 'dispatch;
            }
            0x82269510 => {
    //   block [0x82269510..0x82269518)
	// 82269510: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269514: 388B70F0  addi r4, r11, 0x70f0
	ctx.r[4].s64 = ctx.r[11].s64 + 28912;
	pc = 0x82269518; continue 'dispatch;
            }
            0x82269518 => {
    //   block [0x82269518..0x82269584)
	// 82269518: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226951C: 4BFFD96D  bl 0x82266e88
	ctx.lr = 0x82269520;
	sub_82266E88(ctx, base);
	// 82269520: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269524: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269528: 388BC944  addi r4, r11, -0x36bc
	ctx.r[4].s64 = ctx.r[11].s64 + -14012;
	// 8226952C: 4BFFDB25  bl 0x82267050
	ctx.lr = 0x82269530;
	sub_82267050(ctx, base);
	// 82269530: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269534: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269538: 388BAF40  addi r4, r11, -0x50c0
	ctx.r[4].s64 = ctx.r[11].s64 + -20672;
	// 8226953C: 4BFFD94D  bl 0x82266e88
	ctx.lr = 0x82269540;
	sub_82266E88(ctx, base);
	// 82269540: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82269544: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269548: 388BA2A0  addi r4, r11, -0x5d60
	ctx.r[4].s64 = ctx.r[11].s64 + -23904;
	// 8226954C: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269550: 556506FE  clrlwi r5, r11, 0x1b
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82269554: 4BFFD935  bl 0x82266e88
	ctx.lr = 0x82269558;
	sub_82266E88(ctx, base);
	// 82269558: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8226955C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269560: 388BA0F4  addi r4, r11, -0x5f0c
	ctx.r[4].s64 = ctx.r[11].s64 + -24332;
	// 82269564: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269568: 556504FE  clrlwi r5, r11, 0x13
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x00001FFFu64;
	// 8226956C: 4BFFD91D  bl 0x82266e88
	ctx.lr = 0x82269570;
	sub_82266E88(ctx, base);
	// 82269570: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82269574: 556B0529  rlwinm. r11, r11, 0, 0x14, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269578: 418207C8  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 8226957C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82269580: 388BA280  addi r4, r11, -0x5d80
	ctx.r[4].s64 = ctx.r[11].s64 + -23936;
	pc = 0x82269584; continue 'dispatch;
            }
            0x82269584 => {
    //   block [0x82269584..0x822695C8)
	// 82269584: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269588: 4BFFD901  bl 0x82266e88
	ctx.lr = 0x8226958C;
	sub_82266E88(ctx, base);
	// 8226958C: 480007B4  b 0x82269d40
	pc = 0x82269D40; continue 'dispatch;
	// 82269590: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82269594: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82269598: 418207A8  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 8226959C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822695A0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 822695A4: 556B0463  rlwinm. r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822695A8: 4182002C  beq 0x822695d4
	if ctx.cr[0].eq {
	pc = 0x822695D4; continue 'dispatch;
	}
	// 822695AC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822695B0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 822695B4: 556B056B  rlwinm. r11, r11, 0, 0x15, 0x15
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822695B8: 41820010  beq 0x822695c8
	if ctx.cr[0].eq {
	pc = 0x822695C8; continue 'dispatch;
	}
	// 822695BC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 822695C0: 388BA0A8  addi r4, r11, -0x5f58
	ctx.r[4].s64 = ctx.r[11].s64 + -24408;
	// 822695C4: 48000018  b 0x822695dc
	pc = 0x822695DC; continue 'dispatch;
            }
            0x822695C8 => {
    //   block [0x822695C8..0x822695D4)
	// 822695C8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 822695CC: 388BA0A0  addi r4, r11, -0x5f60
	ctx.r[4].s64 = ctx.r[11].s64 + -24416;
	// 822695D0: 4800000C  b 0x822695dc
	pc = 0x822695DC; continue 'dispatch;
            }
            0x822695D4 => {
    //   block [0x822695D4..0x822695DC)
	// 822695D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822695D8: 388B70F0  addi r4, r11, 0x70f0
	ctx.r[4].s64 = ctx.r[11].s64 + 28912;
	pc = 0x822695DC; continue 'dispatch;
            }
            0x822695DC => {
    //   block [0x822695DC..0x82269630)
	// 822695DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822695E0: 4BFFD8A9  bl 0x82266e88
	ctx.lr = 0x822695E4;
	sub_82266E88(ctx, base);
	// 822695E4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822695E8: 556B04A5  rlwinm. r11, r11, 0, 0x12, 0x12
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822695EC: 40820078  bne 0x82269664
	if !ctx.cr[0].eq {
	pc = 0x82269664; continue 'dispatch;
	}
	// 822695F0: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822695F4: 40820070  bne 0x82269664
	if !ctx.cr[0].eq {
	pc = 0x82269664; continue 'dispatch;
	}
	// 822695F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822695FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269600: 388B1BD8  addi r4, r11, 0x1bd8
	ctx.r[4].s64 = ctx.r[11].s64 + 7128;
	// 82269604: 4BFFDA4D  bl 0x82267050
	ctx.lr = 0x82269608;
	sub_82267050(ctx, base);
	// 82269608: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8226960C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269610: 388BAF40  addi r4, r11, -0x50c0
	ctx.r[4].s64 = ctx.r[11].s64 + -20672;
	// 82269614: 4BFFD875  bl 0x82266e88
	ctx.lr = 0x82269618;
	sub_82266E88(ctx, base);
	// 82269618: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226961C: 556B056B  rlwinm. r11, r11, 0, 0x15, 0x15
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269620: 41820010  beq 0x82269630
	if ctx.cr[0].eq {
	pc = 0x82269630; continue 'dispatch;
	}
	// 82269624: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82269628: 38AB0F75  addi r5, r11, 0xf75
	ctx.r[5].s64 = ctx.r[11].s64 + 3957;
	// 8226962C: 4800000C  b 0x82269638
	pc = 0x82269638; continue 'dispatch;
            }
            0x82269630 => {
    //   block [0x82269630..0x82269638)
	// 82269630: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269634: 38ABACC4  addi r5, r11, -0x533c
	ctx.r[5].s64 = ctx.r[11].s64 + -21308;
	pc = 0x82269638; continue 'dispatch;
            }
            0x82269638 => {
    //   block [0x82269638..0x82269664)
	// 82269638: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8226963C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269640: 388B91C0  addi r4, r11, -0x6e40
	ctx.r[4].s64 = ctx.r[11].s64 + -28224;
	// 82269644: 4BFFD845  bl 0x82266e88
	ctx.lr = 0x82269648;
	sub_82266E88(ctx, base);
	// 82269648: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8226964C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269650: 388BA278  addi r4, r11, -0x5d88
	ctx.r[4].s64 = ctx.r[11].s64 + -23944;
	// 82269654: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82269658: 5565F63E  rlwinm r5, r11, 0x1e, 0x18, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 8226965C: 4BFFD82D  bl 0x82266e88
	ctx.lr = 0x82269660;
	sub_82266E88(ctx, base);
	// 82269660: 48000024  b 0x82269684
	pc = 0x82269684; continue 'dispatch;
            }
            0x82269664 => {
    //   block [0x82269664..0x82269684)
	// 82269664: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269668: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226966C: 388BC960  addi r4, r11, -0x36a0
	ctx.r[4].s64 = ctx.r[11].s64 + -13984;
	// 82269670: 4BFFD9E1  bl 0x82267050
	ctx.lr = 0x82269674;
	sub_82267050(ctx, base);
	// 82269674: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269678: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226967C: 388BAF40  addi r4, r11, -0x50c0
	ctx.r[4].s64 = ctx.r[11].s64 + -20672;
	// 82269680: 4BFFD809  bl 0x82266e88
	ctx.lr = 0x82269684;
	sub_82266E88(ctx, base);
	pc = 0x82269684; continue 'dispatch;
            }
            0x82269684 => {
    //   block [0x82269684..0x822696F8)
	// 82269684: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269688: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8226968C: 388AA0F4  addi r4, r10, -0x5f0c
	ctx.r[4].s64 = ctx.r[10].s64 + -24332;
	// 82269690: 556504FE  clrlwi r5, r11, 0x13
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x00001FFFu64;
	// 82269694: 4800034C  b 0x822699e0
	pc = 0x822699E0; continue 'dispatch;
	// 82269698: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8226969C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822696A0: 418206A0  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 822696A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822696A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822696AC: 388B70F0  addi r4, r11, 0x70f0
	ctx.r[4].s64 = ctx.r[11].s64 + 28912;
	// 822696B0: 4BFFD7D9  bl 0x82266e88
	ctx.lr = 0x822696B4;
	sub_82266E88(ctx, base);
	// 822696B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822696B8: 388BC94C  addi r4, r11, -0x36b4
	ctx.r[4].s64 = ctx.r[11].s64 + -14004;
	// 822696BC: 4BFFFBC0  b 0x8226927c
	pc = 0x8226927C; continue 'dispatch;
	// 822696C0: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822696C4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822696C8: 41820678  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 822696CC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822696D0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 822696D4: 556B0463  rlwinm. r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822696D8: 4182002C  beq 0x82269704
	if ctx.cr[0].eq {
	pc = 0x82269704; continue 'dispatch;
	}
	// 822696DC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822696E0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 822696E4: 556B056B  rlwinm. r11, r11, 0, 0x15, 0x15
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822696E8: 41820010  beq 0x822696f8
	if ctx.cr[0].eq {
	pc = 0x822696F8; continue 'dispatch;
	}
	// 822696EC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 822696F0: 388BA0A8  addi r4, r11, -0x5f58
	ctx.r[4].s64 = ctx.r[11].s64 + -24408;
	// 822696F4: 48000018  b 0x8226970c
	pc = 0x8226970C; continue 'dispatch;
            }
            0x822696F8 => {
    //   block [0x822696F8..0x82269704)
	// 822696F8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 822696FC: 388BA0A0  addi r4, r11, -0x5f60
	ctx.r[4].s64 = ctx.r[11].s64 + -24416;
	// 82269700: 4800000C  b 0x8226970c
	pc = 0x8226970C; continue 'dispatch;
            }
            0x82269704 => {
    //   block [0x82269704..0x8226970C)
	// 82269704: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269708: 388B70F0  addi r4, r11, 0x70f0
	ctx.r[4].s64 = ctx.r[11].s64 + 28912;
	pc = 0x8226970C; continue 'dispatch;
            }
            0x8226970C => {
    //   block [0x8226970C..0x8226972C)
	// 8226970C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269710: 4BFFD779  bl 0x82266e88
	ctx.lr = 0x82269714;
	sub_82266E88(ctx, base);
	// 82269714: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226971C: 554A04A5  rlwinm. r10, r10, 0, 0x12, 0x12
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82269720: 4082000C  bne 0x8226972c
	if !ctx.cr[0].eq {
	pc = 0x8226972C; continue 'dispatch;
	}
	// 82269724: 57CA063F  clrlwi. r10, r30, 0x18
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82269728: 41820008  beq 0x82269730
	if ctx.cr[0].eq {
	pc = 0x82269730; continue 'dispatch;
	}
	pc = 0x8226972C; continue 'dispatch;
            }
            0x8226972C => {
    //   block [0x8226972C..0x82269730)
	// 8226972C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	pc = 0x82269730; continue 'dispatch;
            }
            0x82269730 => {
    //   block [0x82269730..0x8226975C)
	// 82269730: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269734: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269738: 41820024  beq 0x8226975c
	if ctx.cr[0].eq {
	pc = 0x8226975C; continue 'dispatch;
	}
	// 8226973C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269740: 388B1BCC  addi r4, r11, 0x1bcc
	ctx.r[4].s64 = ctx.r[11].s64 + 7116;
	// 82269744: 4BFFD90D  bl 0x82267050
	ctx.lr = 0x82269748;
	sub_82267050(ctx, base);
	// 82269748: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8226974C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269750: 388BAF40  addi r4, r11, -0x50c0
	ctx.r[4].s64 = ctx.r[11].s64 + -20672;
	// 82269754: 4BFFD735  bl 0x82266e88
	ctx.lr = 0x82269758;
	sub_82266E88(ctx, base);
	// 82269758: 48000068  b 0x822697c0
	pc = 0x822697C0; continue 'dispatch;
            }
            0x8226975C => {
    //   block [0x8226975C..0x82269790)
	// 8226975C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269760: 388B1BD0  addi r4, r11, 0x1bd0
	ctx.r[4].s64 = ctx.r[11].s64 + 7120;
	// 82269764: 4BFFD8ED  bl 0x82267050
	ctx.lr = 0x82269768;
	sub_82267050(ctx, base);
	// 82269768: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8226976C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269770: 388BAF40  addi r4, r11, -0x50c0
	ctx.r[4].s64 = ctx.r[11].s64 + -20672;
	// 82269774: 4BFFD715  bl 0x82266e88
	ctx.lr = 0x82269778;
	sub_82266E88(ctx, base);
	// 82269778: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226977C: 556B056B  rlwinm. r11, r11, 0, 0x15, 0x15
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269780: 41820010  beq 0x82269790
	if ctx.cr[0].eq {
	pc = 0x82269790; continue 'dispatch;
	}
	// 82269784: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82269788: 38AB0F75  addi r5, r11, 0xf75
	ctx.r[5].s64 = ctx.r[11].s64 + 3957;
	// 8226978C: 4800000C  b 0x82269798
	pc = 0x82269798; continue 'dispatch;
            }
            0x82269790 => {
    //   block [0x82269790..0x82269798)
	// 82269790: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269794: 38ABACC4  addi r5, r11, -0x533c
	ctx.r[5].s64 = ctx.r[11].s64 + -21308;
	pc = 0x82269798; continue 'dispatch;
            }
            0x82269798 => {
    //   block [0x82269798..0x822697C0)
	// 82269798: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8226979C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822697A0: 388B91C0  addi r4, r11, -0x6e40
	ctx.r[4].s64 = ctx.r[11].s64 + -28224;
	// 822697A4: 4BFFD6E5  bl 0x82266e88
	ctx.lr = 0x822697A8;
	sub_82266E88(ctx, base);
	// 822697A8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 822697AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822697B0: 388BA278  addi r4, r11, -0x5d88
	ctx.r[4].s64 = ctx.r[11].s64 + -23944;
	// 822697B4: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822697B8: 5565F63E  rlwinm r5, r11, 0x1e, 0x18, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 822697BC: 4BFFD6CD  bl 0x82266e88
	ctx.lr = 0x822697C0;
	sub_82266E88(ctx, base);
	pc = 0x822697C0; continue 'dispatch;
            }
            0x822697C0 => {
    //   block [0x822697C0..0x82269800)
	// 822697C0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822697C4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 822697C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822697CC: 388AA0F4  addi r4, r10, -0x5f0c
	ctx.r[4].s64 = ctx.r[10].s64 + -24332;
	// 822697D0: 556504FE  clrlwi r5, r11, 0x13
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x00001FFFu64;
	// 822697D4: 4BFFD6B5  bl 0x82266e88
	ctx.lr = 0x822697D8;
	sub_82266E88(ctx, base);
	// 822697D8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822697DC: 556A077B  rlwinm. r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822697E0: 41820030  beq 0x82269810
	if ctx.cr[0].eq {
	pc = 0x82269810; continue 'dispatch;
	}
	// 822697E4: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822697E8: 554AFFFF  rlwinm. r10, r10, 0x1f, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822697EC: 4082003C  bne 0x82269828
	if !ctx.cr[0].eq {
	pc = 0x82269828; continue 'dispatch;
	}
	// 822697F0: 556BD7FF  rlwinm. r11, r11, 0x1a, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822697F4: 4182001C  beq 0x82269810
	if ctx.cr[0].eq {
	pc = 0x82269810; continue 'dispatch;
	}
	// 822697F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822697FC: 38ABFE08  addi r5, r11, -0x1f8
	ctx.r[5].s64 = ctx.r[11].s64 + -504;
	pc = 0x82269800; continue 'dispatch;
            }
            0x82269800 => {
    //   block [0x82269800..0x82269810)
	// 82269800: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82269804: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269808: 388BA264  addi r4, r11, -0x5d9c
	ctx.r[4].s64 = ctx.r[11].s64 + -23964;
	// 8226980C: 4BFFD67D  bl 0x82266e88
	ctx.lr = 0x82269810;
	sub_82266E88(ctx, base);
	pc = 0x82269810; continue 'dispatch;
            }
            0x82269810 => {
    //   block [0x82269810..0x82269828)
	// 82269810: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82269814: 5565AFFF  rlwinm. r5, r11, 0x15, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x000007FFu64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82269818: 41820528  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 8226981C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82269820: 388B905C  addi r4, r11, -0x6fa4
	ctx.r[4].s64 = ctx.r[11].s64 + -28580;
	// 82269824: 480001BC  b 0x822699e0
	pc = 0x822699E0; continue 'dispatch;
            }
            0x82269828 => {
    //   block [0x82269828..0x822698C8)
	// 82269828: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8226982C: 38ABFE10  addi r5, r11, -0x1f0
	ctx.r[5].s64 = ctx.r[11].s64 + -496;
	// 82269830: 4BFFFFD0  b 0x82269800
	pc = 0x82269800; continue 'dispatch;
	// 82269834: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82269838: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8226983C: 41820504  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 82269840: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269844: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269848: 388B70F0  addi r4, r11, 0x70f0
	ctx.r[4].s64 = ctx.r[11].s64 + 28912;
	// 8226984C: 4BFFD63D  bl 0x82266e88
	ctx.lr = 0x82269850;
	sub_82266E88(ctx, base);
	// 82269850: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269854: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269858: 388B1BC4  addi r4, r11, 0x1bc4
	ctx.r[4].s64 = ctx.r[11].s64 + 7108;
	// 8226985C: 4BFFD7F5  bl 0x82267050
	ctx.lr = 0x82269860;
	sub_82267050(ctx, base);
	// 82269860: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269864: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269868: 388BAF40  addi r4, r11, -0x50c0
	ctx.r[4].s64 = ctx.r[11].s64 + -20672;
	// 8226986C: 4BFFD61D  bl 0x82266e88
	ctx.lr = 0x82269870;
	sub_82266E88(ctx, base);
	// 82269870: 3D608228  lis r11, -0x7dd8
	ctx.r[11].s64 = -2111307776;
	// 82269874: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82269878: 392B7050  addi r9, r11, 0x7050
	ctx.r[9].s64 = ctx.r[11].s64 + 28752;
	// 8226987C: 388A91C0  addi r4, r10, -0x6e40
	ctx.r[4].s64 = ctx.r[10].s64 + -28224;
	// 82269880: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269884: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82269888: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8226988C: 557EBFBE  rlwinm r30, r11, 0x17, 0x1e, 0x1f
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 82269890: 554AEFFE  rlwinm r10, r10, 0x1d, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82269894: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82269898: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8226989C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822698A0: 7CAB482E  lwzx r5, r11, r9
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 822698A4: 4BFFD5E5  bl 0x82266e88
	ctx.lr = 0x822698A8;
	sub_82266E88(ctx, base);
	// 822698A8: 2B1E0003  cmplwi cr6, r30, 3
	ctx.cr[6].compare_u32(ctx.r[30].u32, 3 as u32, &mut ctx.xer);
	// 822698AC: 409A001C  bne cr6, 0x822698c8
	if !ctx.cr[6].eq {
	pc = 0x822698C8; continue 'dispatch;
	}
	// 822698B0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822698B4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 822698B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822698BC: 388AA260  addi r4, r10, -0x5da0
	ctx.r[4].s64 = ctx.r[10].s64 + -23968;
	// 822698C0: 5565077E  clrlwi r5, r11, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 822698C4: 4BFFD5C5  bl 0x82266e88
	ctx.lr = 0x822698C8;
	sub_82266E88(ctx, base);
            }
            0x822698C8 => {
    //   block [0x822698C8..0x82269904)
	// 822698C8: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822698CC: 556B05EF  rlwinm. r11, r11, 0, 0x17, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822698D0: 41820470  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 822698D4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 822698D8: 388BA248  addi r4, r11, -0x5db8
	ctx.r[4].s64 = ctx.r[11].s64 + -23992;
	// 822698DC: 4BFFFCA8  b 0x82269584
	pc = 0x82269584; continue 'dispatch;
	// 822698E0: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822698E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822698E8: 41820458  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 822698EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822698F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822698F4: 388B70F0  addi r4, r11, 0x70f0
	ctx.r[4].s64 = ctx.r[11].s64 + 28912;
	// 822698F8: 4BFFD591  bl 0x82266e88
	ctx.lr = 0x822698FC;
	sub_82266E88(ctx, base);
	// 822698FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269900: 388B1BE8  addi r4, r11, 0x1be8
	ctx.r[4].s64 = ctx.r[11].s64 + 7144;
            }
            0x82269904 => {
    //   block [0x82269904..0x82269938)
	// 82269904: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269908: 4BFFD749  bl 0x82267050
	ctx.lr = 0x8226990C;
	sub_82267050(ctx, base);
	// 8226990C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269910: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269914: 388BAF40  addi r4, r11, -0x50c0
	ctx.r[4].s64 = ctx.r[11].s64 + -20672;
	// 82269918: 4BFFD571  bl 0x82266e88
	ctx.lr = 0x8226991C;
	sub_82266E88(ctx, base);
	// 8226991C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82269920: 556B056B  rlwinm. r11, r11, 0, 0x15, 0x15
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269924: 40820014  bne 0x82269938
	if !ctx.cr[0].eq {
	pc = 0x82269938; continue 'dispatch;
	}
	// 82269928: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8226992C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269930: 388BACC4  addi r4, r11, -0x533c
	ctx.r[4].s64 = ctx.r[11].s64 + -21308;
	// 82269934: 4BFFD555  bl 0x82266e88
	ctx.lr = 0x82269938;
	sub_82266E88(ctx, base);
	pc = 0x82269938; continue 'dispatch;
            }
            0x82269938 => {
    //   block [0x82269938..0x822699A8)
	// 82269938: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226993C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82269940: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269944: 388A904C  addi r4, r10, -0x6fb4
	ctx.r[4].s64 = ctx.r[10].s64 + -28596;
	// 82269948: 5565F63E  rlwinm r5, r11, 0x1e, 0x18, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 8226994C: 4BFFD53D  bl 0x82266e88
	ctx.lr = 0x82269950;
	sub_82266E88(ctx, base);
	// 82269950: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82269954: 4BFFFA1C  b 0x82269370
	pc = 0x82269370; continue 'dispatch;
	// 82269958: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8226995C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82269960: 418203E0  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 82269964: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269968: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226996C: 388B70F0  addi r4, r11, 0x70f0
	ctx.r[4].s64 = ctx.r[11].s64 + 28912;
	// 82269970: 4BFFD519  bl 0x82266e88
	ctx.lr = 0x82269974;
	sub_82266E88(ctx, base);
	// 82269974: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269978: 388B1BE0  addi r4, r11, 0x1be0
	ctx.r[4].s64 = ctx.r[11].s64 + 7136;
	// 8226997C: 4BFFFF88  b 0x82269904
	pc = 0x82269904; continue 'dispatch;
	// 82269980: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82269984: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82269988: 418203B8  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 8226998C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269990: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269994: 388B70F0  addi r4, r11, 0x70f0
	ctx.r[4].s64 = ctx.r[11].s64 + 28912;
	// 82269998: 4BFFD4F1  bl 0x82266e88
	ctx.lr = 0x8226999C;
	sub_82266E88(ctx, base);
	// 8226999C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822699A0: 388B1BBC  addi r4, r11, 0x1bbc
	ctx.r[4].s64 = ctx.r[11].s64 + 7100;
	// 822699A4: 4BFFF8D8  b 0x8226927c
	pc = 0x8226927C; continue 'dispatch;
            }
            0x822699A8 => {
    //   block [0x822699A8..0x822699E0)
	// 822699A8: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822699AC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822699B0: 41820390  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 822699B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822699B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822699BC: 388B70F0  addi r4, r11, 0x70f0
	ctx.r[4].s64 = ctx.r[11].s64 + 28912;
	// 822699C0: 4BFFD4C9  bl 0x82266e88
	ctx.lr = 0x822699C4;
	sub_82266E88(ctx, base);
	// 822699C4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822699C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822699CC: 388B90A0  addi r4, r11, -0x6f60
	ctx.r[4].s64 = ctx.r[11].s64 + -28512;
	// 822699D0: 4BFFD681  bl 0x82267050
	ctx.lr = 0x822699D4;
	sub_82267050(ctx, base);
	// 822699D4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 822699D8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822699DC: 388B9050  addi r4, r11, -0x6fb0
	ctx.r[4].s64 = ctx.r[11].s64 + -28592;
	pc = 0x822699E0; continue 'dispatch;
            }
            0x822699E0 => {
    //   block [0x822699E0..0x822699EC)
	// 822699E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822699E4: 4BFFD4A5  bl 0x82266e88
	ctx.lr = 0x822699E8;
	sub_82266E88(ctx, base);
	// 822699E8: 48000358  b 0x82269d40
	pc = 0x82269D40; continue 'dispatch;
            }
            0x822699EC => {
    //   block [0x822699EC..0x82269A04)
	// 822699EC: 57AB077D  rlwinm. r11, r29, 0, 0x1d, 0x1e
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822699F0: 41820350  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 822699F4: 817F2030  lwz r11, 0x2030(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8240 as u32) ) } as u64;
	// 822699F8: 7F0BB040  cmplw cr6, r11, r22
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[22].u32, &mut ctx.xer);
	// 822699FC: 41990008  bgt cr6, 0x82269a04
	if ctx.cr[6].gt {
	pc = 0x82269A04; continue 'dispatch;
	}
	// 82269A00: 7ECBB378  mr r11, r22
	ctx.r[11].u64 = ctx.r[22].u64;
	pc = 0x82269A04; continue 'dispatch;
            }
            0x82269A04 => {
    //   block [0x82269A04..0x82269A3C)
	// 82269A04: 57AA07BD  rlwinm. r10, r29, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82269A08: 917F2030  stw r11, 0x2030(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8240 as u32), ctx.r[11].u32 ) };
	// 82269A0C: 57B5DFFE  rlwinm r21, r29, 0x1b, 0x1f, 0x1f
	ctx.r[21].u64 = ctx.r[29].u32 as u64 & 0x0000001Fu64;
	// 82269A10: 418200AC  beq 0x82269abc
	if ctx.cr[0].eq {
	pc = 0x82269ABC; continue 'dispatch;
	}
	// 82269A14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269A18: 4BFFE741  bl 0x82268158
	ctx.lr = 0x82269A1C;
	sub_82268158(ctx, base);
	// 82269A1C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82269A20: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269A24: 41820018  beq 0x82269a3c
	if ctx.cr[0].eq {
	pc = 0x82269A3C; continue 'dispatch;
	}
	// 82269A28: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82269A2C: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 82269A30: 388BA0D0  addi r4, r11, -0x5f30
	ctx.r[4].s64 = ctx.r[11].s64 + -24368;
	// 82269A34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269A38: 4BFFD451  bl 0x82266e88
	ctx.lr = 0x82269A3C;
	sub_82266E88(ctx, base);
	pc = 0x82269A3C; continue 'dispatch;
            }
            0x82269A3C => {
    //   block [0x82269A3C..0x82269A7C)
	// 82269A3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269A40: 56AA063F  clrlwi. r10, r21, 0x18
	ctx.r[10].u64 = ctx.r[21].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82269A44: 3BCB70F0  addi r30, r11, 0x70f0
	ctx.r[30].s64 = ctx.r[11].s64 + 28912;
	// 82269A48: 41820050  beq 0x82269a98
	if ctx.cr[0].eq {
	pc = 0x82269A98; continue 'dispatch;
	}
	// 82269A4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82269A50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269A54: 4BFFD435  bl 0x82266e88
	ctx.lr = 0x82269A58;
	sub_82266E88(ctx, base);
	// 82269A58: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82269A5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269A60: 388B58F4  addi r4, r11, 0x58f4
	ctx.r[4].s64 = ctx.r[11].s64 + 22772;
	// 82269A64: 4BFFD5ED  bl 0x82267050
	ctx.lr = 0x82269A68;
	sub_82267050(ctx, base);
	// 82269A68: 897F2028  lbz r11, 0x2028(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8232 as u32) ) } as u64;
	// 82269A6C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82269A70: 4082000C  bne 0x82269a7c
	if !ctx.cr[0].eq {
	pc = 0x82269A7C; continue 'dispatch;
	}
	// 82269A74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269A78: 4BFFE689  bl 0x82268100
	ctx.lr = 0x82269A7C;
	sub_82268100(ctx, base);
	pc = 0x82269A7C; continue 'dispatch;
            }
            0x82269A7C => {
    //   block [0x82269A7C..0x82269A98)
	// 82269A7C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82269A80: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269A84: 41820014  beq 0x82269a98
	if ctx.cr[0].eq {
	pc = 0x82269A98; continue 'dispatch;
	}
	// 82269A88: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82269A8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269A90: 388BA0E0  addi r4, r11, -0x5f20
	ctx.r[4].s64 = ctx.r[11].s64 + -24352;
	// 82269A94: 4BFFD3F5  bl 0x82266e88
	ctx.lr = 0x82269A98;
	sub_82266E88(ctx, base);
	pc = 0x82269A98; continue 'dispatch;
            }
            0x82269A98 => {
    //   block [0x82269A98..0x82269ABC)
	// 82269A98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82269A9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269AA0: 4BFFD3E9  bl 0x82266e88
	ctx.lr = 0x82269AA4;
	sub_82266E88(ctx, base);
	// 82269AA4: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 82269AA8: 57A5E7FE  rlwinm r5, r29, 0x1c, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[29].u32 as u64 & 0x0000000Fu64;
	// 82269AAC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82269AB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269AB4: 4BFFE89D  bl 0x82268350
	ctx.lr = 0x82269AB8;
	sub_82268350(ctx, base);
	// 82269AB8: 48000288  b 0x82269d40
	pc = 0x82269D40; continue 'dispatch;
            }
            0x82269ABC => {
    //   block [0x82269ABC..0x82269B34)
	// 82269ABC: 57AB077B  rlwinm. r11, r29, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269AC0: 41820280  beq 0x82269d40
	if ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 82269AC4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82269AC8: 4BFFD251  bl 0x82266d18
	ctx.lr = 0x82269ACC;
	sub_82266D18(ctx, base);
	// 82269ACC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82269AD0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82269AD4: 3D208205  lis r9, -0x7dfb
	ctx.r[9].s64 = -2113601536;
	// 82269AD8: 3D008205  lis r8, -0x7dfb
	ctx.r[8].s64 = -2113601536;
	// 82269ADC: 3CE08205  lis r7, -0x7dfb
	ctx.r[7].s64 = -2113601536;
	// 82269AE0: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 82269AE4: 5465063F  clrlwi. r5, r3, 0x18
	ctx.r[5].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82269AE8: 3FC0C800  lis r30, -0x3800
	ctx.r[30].s64 = -939524096;
	// 82269AEC: 3B2BA0E0  addi r25, r11, -0x5f20
	ctx.r[25].s64 = ctx.r[11].s64 + -24352;
	// 82269AF0: 3B4A58F4  addi r26, r10, 0x58f4
	ctx.r[26].s64 = ctx.r[10].s64 + 22772;
	// 82269AF4: 3B69A0D0  addi r27, r9, -0x5f30
	ctx.r[27].s64 = ctx.r[9].s64 + -24368;
	// 82269AF8: 3B08A0A0  addi r24, r8, -0x5f60
	ctx.r[24].s64 = ctx.r[8].s64 + -24416;
	// 82269AFC: 3AE7A0A8  addi r23, r7, -0x5f58
	ctx.r[23].s64 = ctx.r[7].s64 + -24408;
	// 82269B00: 3BA670F0  addi r29, r6, 0x70f0
	ctx.r[29].s64 = ctx.r[6].s64 + 28912;
	// 82269B04: 41820048  beq 0x82269b4c
	if ctx.cr[0].eq {
	pc = 0x82269B4C; continue 'dispatch;
	}
	// 82269B08: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269B0C: 556A000A  rlwinm r10, r11, 0, 0, 5
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82269B10: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82269B14: 409A002C  bne cr6, 0x82269b40
	if !ctx.cr[6].eq {
	pc = 0x82269B40; continue 'dispatch;
	}
	// 82269B18: 556A0421  rlwinm. r10, r11, 0, 0x10, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82269B1C: 41820018  beq 0x82269b34
	if ctx.cr[0].eq {
	pc = 0x82269B34; continue 'dispatch;
	}
	// 82269B20: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82269B24: 4BFFD155  bl 0x82266c78
	ctx.lr = 0x82269B28;
	sub_82266C78(ctx, base);
	// 82269B28: 7C6B18F8  nor r11, r3, r3
	ctx.r[11].u64 = !(ctx.r[3].u64 | ctx.r[3].u64);
	// 82269B2C: 556BFFFE  rlwinm r11, r11, 0x1f, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82269B30: 48000014  b 0x82269b44
	pc = 0x82269B44; continue 'dispatch;
            }
            0x82269B34 => {
    //   block [0x82269B34..0x82269B40)
	// 82269B34: 556B0217  rlwinm. r11, r11, 0, 8, 0xb
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269B38: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82269B3C: 41820008  beq 0x82269b44
	if ctx.cr[0].eq {
	pc = 0x82269B44; continue 'dispatch;
	}
	pc = 0x82269B40; continue 'dispatch;
            }
            0x82269B40 => {
    //   block [0x82269B40..0x82269B44)
	// 82269B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82269B44; continue 'dispatch;
            }
            0x82269B44 => {
    //   block [0x82269B44..0x82269B4C)
	// 82269B44: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269B48: 418200B8  beq 0x82269c00
	if ctx.cr[0].eq {
	pc = 0x82269C00; continue 'dispatch;
	}
	pc = 0x82269B4C; continue 'dispatch;
            }
            0x82269B4C => {
    //   block [0x82269B4C..0x82269B70)
	// 82269B4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269B50: 4BFFE609  bl 0x82268158
	ctx.lr = 0x82269B54;
	sub_82268158(ctx, base);
	// 82269B54: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82269B58: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269B5C: 41820014  beq 0x82269b70
	if ctx.cr[0].eq {
	pc = 0x82269B70; continue 'dispatch;
	}
	// 82269B60: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82269B64: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 82269B68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269B6C: 4BFFD31D  bl 0x82266e88
	ctx.lr = 0x82269B70;
	sub_82266E88(ctx, base);
	pc = 0x82269B70; continue 'dispatch;
            }
            0x82269B70 => {
    //   block [0x82269B70..0x82269BA4)
	// 82269B70: 56AB063F  clrlwi. r11, r21, 0x18
	ctx.r[11].u64 = ctx.r[21].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269B74: 41820048  beq 0x82269bbc
	if ctx.cr[0].eq {
	pc = 0x82269BBC; continue 'dispatch;
	}
	// 82269B78: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82269B7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269B80: 4BFFD309  bl 0x82266e88
	ctx.lr = 0x82269B84;
	sub_82266E88(ctx, base);
	// 82269B84: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82269B88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269B8C: 4BFFD4C5  bl 0x82267050
	ctx.lr = 0x82269B90;
	sub_82267050(ctx, base);
	// 82269B90: 897F2028  lbz r11, 0x2028(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8232 as u32) ) } as u64;
	// 82269B94: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82269B98: 4082000C  bne 0x82269ba4
	if !ctx.cr[0].eq {
	pc = 0x82269BA4; continue 'dispatch;
	}
	// 82269B9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269BA0: 4BFFE561  bl 0x82268100
	ctx.lr = 0x82269BA4;
	sub_82268100(ctx, base);
	pc = 0x82269BA4; continue 'dispatch;
            }
            0x82269BA4 => {
    //   block [0x82269BA4..0x82269BBC)
	// 82269BA4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82269BA8: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269BAC: 41820010  beq 0x82269bbc
	if ctx.cr[0].eq {
	pc = 0x82269BBC; continue 'dispatch;
	}
	// 82269BB0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82269BB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269BB8: 4BFFD2D1  bl 0x82266e88
	ctx.lr = 0x82269BBC;
	sub_82266E88(ctx, base);
	pc = 0x82269BBC; continue 'dispatch;
            }
            0x82269BBC => {
    //   block [0x82269BBC..0x82269BEC)
	// 82269BBC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82269BC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269BC4: 4BFFD2C5  bl 0x82266e88
	ctx.lr = 0x82269BC8;
	sub_82266E88(ctx, base);
	// 82269BC8: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82269BCC: 556B2EFE  srwi r11, r11, 0x1b
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(27);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82269BD0: 556A07BE  clrlwi r10, r11, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82269BD4: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 82269BD8: 4198001C  blt cr6, 0x82269bf4
	if ctx.cr[6].lt {
	pc = 0x82269BF4; continue 'dispatch;
	}
	// 82269BDC: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269BE0: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82269BE4: 40820008  bne 0x82269bec
	if !ctx.cr[0].eq {
	pc = 0x82269BEC; continue 'dispatch;
	}
	// 82269BE8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	pc = 0x82269BEC; continue 'dispatch;
            }
            0x82269BEC => {
    //   block [0x82269BEC..0x82269BF4)
	// 82269BEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269BF0: 4BFFD299  bl 0x82266e88
	ctx.lr = 0x82269BF4;
	sub_82266E88(ctx, base);
	pc = 0x82269BF4; continue 'dispatch;
            }
            0x82269BF4 => {
    //   block [0x82269BF4..0x82269C00)
	// 82269BF4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82269BF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269BFC: 4BFFDCE5  bl 0x822678e0
	ctx.lr = 0x82269C00;
	sub_822678E0(ctx, base);
	pc = 0x82269C00; continue 'dispatch;
            }
            0x82269C00 => {
    //   block [0x82269C00..0x82269C2C)
	// 82269C00: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269C04: 556A000A  rlwinm r10, r11, 0, 0, 5
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82269C08: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82269C0C: 409A002C  bne cr6, 0x82269c38
	if !ctx.cr[6].eq {
	pc = 0x82269C38; continue 'dispatch;
	}
	// 82269C10: 556A0421  rlwinm. r10, r11, 0, 0x10, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82269C14: 41820018  beq 0x82269c2c
	if ctx.cr[0].eq {
	pc = 0x82269C2C; continue 'dispatch;
	}
	// 82269C18: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82269C1C: 4BFFD05D  bl 0x82266c78
	ctx.lr = 0x82269C20;
	sub_82266C78(ctx, base);
	// 82269C20: 7C6B18F8  nor r11, r3, r3
	ctx.r[11].u64 = !(ctx.r[3].u64 | ctx.r[3].u64);
	// 82269C24: 556BFFFE  rlwinm r11, r11, 0x1f, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82269C28: 48000014  b 0x82269c3c
	pc = 0x82269C3C; continue 'dispatch;
            }
            0x82269C2C => {
    //   block [0x82269C2C..0x82269C38)
	// 82269C2C: 556B0217  rlwinm. r11, r11, 0, 8, 0xb
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269C30: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82269C34: 41820008  beq 0x82269c3c
	if ctx.cr[0].eq {
	pc = 0x82269C3C; continue 'dispatch;
	}
	pc = 0x82269C38; continue 'dispatch;
            }
            0x82269C38 => {
    //   block [0x82269C38..0x82269C3C)
	// 82269C38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82269C3C; continue 'dispatch;
            }
            0x82269C3C => {
    //   block [0x82269C3C..0x82269C68)
	// 82269C3C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269C40: 40820100  bne 0x82269d40
	if !ctx.cr[0].eq {
	pc = 0x82269D40; continue 'dispatch;
	}
	// 82269C44: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82269C48: 4BFFD0D1  bl 0x82266d18
	ctx.lr = 0x82269C4C;
	sub_82266D18(ctx, base);
	// 82269C4C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269C50: 4082003C  bne 0x82269c8c
	if !ctx.cr[0].eq {
	pc = 0x82269C8C; continue 'dispatch;
	}
	// 82269C54: 897F2028  lbz r11, 0x2028(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8232 as u32) ) } as u64;
	// 82269C58: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82269C5C: 4082000C  bne 0x82269c68
	if !ctx.cr[0].eq {
	pc = 0x82269C68; continue 'dispatch;
	}
	// 82269C60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269C64: 4BFFE49D  bl 0x82268100
	ctx.lr = 0x82269C68;
	sub_82268100(ctx, base);
	pc = 0x82269C68; continue 'dispatch;
            }
            0x82269C68 => {
    //   block [0x82269C68..0x82269C80)
	// 82269C68: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82269C6C: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269C70: 41820010  beq 0x82269c80
	if ctx.cr[0].eq {
	pc = 0x82269C80; continue 'dispatch;
	}
	// 82269C74: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82269C78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269C7C: 4BFFD20D  bl 0x82266e88
	ctx.lr = 0x82269C80;
	sub_82266E88(ctx, base);
	pc = 0x82269C80; continue 'dispatch;
            }
            0x82269C80 => {
    //   block [0x82269C80..0x82269C8C)
	// 82269C80: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82269C84: 388B8EE4  addi r4, r11, -0x711c
	ctx.r[4].s64 = ctx.r[11].s64 + -28956;
	// 82269C88: 48000078  b 0x82269d00
	pc = 0x82269D00; continue 'dispatch;
            }
            0x82269C8C => {
    //   block [0x82269C8C..0x82269CB0)
	// 82269C8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269C90: 4BFFE4C9  bl 0x82268158
	ctx.lr = 0x82269C94;
	sub_82268158(ctx, base);
	// 82269C94: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82269C98: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269C9C: 41820014  beq 0x82269cb0
	if ctx.cr[0].eq {
	pc = 0x82269CB0; continue 'dispatch;
	}
	// 82269CA0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82269CA4: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 82269CA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269CAC: 4BFFD1DD  bl 0x82266e88
	ctx.lr = 0x82269CB0;
	sub_82266E88(ctx, base);
	pc = 0x82269CB0; continue 'dispatch;
            }
            0x82269CB0 => {
    //   block [0x82269CB0..0x82269CE4)
	// 82269CB0: 56AB063F  clrlwi. r11, r21, 0x18
	ctx.r[11].u64 = ctx.r[21].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269CB4: 41820048  beq 0x82269cfc
	if ctx.cr[0].eq {
	pc = 0x82269CFC; continue 'dispatch;
	}
	// 82269CB8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82269CBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269CC0: 4BFFD1C9  bl 0x82266e88
	ctx.lr = 0x82269CC4;
	sub_82266E88(ctx, base);
	// 82269CC4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82269CC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269CCC: 4BFFD385  bl 0x82267050
	ctx.lr = 0x82269CD0;
	sub_82267050(ctx, base);
	// 82269CD0: 897F2028  lbz r11, 0x2028(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8232 as u32) ) } as u64;
	// 82269CD4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82269CD8: 4082000C  bne 0x82269ce4
	if !ctx.cr[0].eq {
	pc = 0x82269CE4; continue 'dispatch;
	}
	// 82269CDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269CE0: 4BFFE421  bl 0x82268100
	ctx.lr = 0x82269CE4;
	sub_82268100(ctx, base);
	pc = 0x82269CE4; continue 'dispatch;
            }
            0x82269CE4 => {
    //   block [0x82269CE4..0x82269CFC)
	// 82269CE4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82269CE8: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269CEC: 41820010  beq 0x82269cfc
	if ctx.cr[0].eq {
	pc = 0x82269CFC; continue 'dispatch;
	}
	// 82269CF0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82269CF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269CF8: 4BFFD191  bl 0x82266e88
	ctx.lr = 0x82269CFC;
	sub_82266E88(ctx, base);
	pc = 0x82269CFC; continue 'dispatch;
            }
            0x82269CFC => {
    //   block [0x82269CFC..0x82269D00)
	// 82269CFC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	pc = 0x82269D00; continue 'dispatch;
            }
            0x82269D00 => {
    //   block [0x82269D00..0x82269D2C)
	// 82269D00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269D04: 4BFFD185  bl 0x82266e88
	ctx.lr = 0x82269D08;
	sub_82266E88(ctx, base);
	// 82269D08: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82269D0C: 556B2EFE  srwi r11, r11, 0x1b
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(27);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82269D10: 556A07BE  clrlwi r10, r11, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82269D14: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 82269D18: 4198001C  blt cr6, 0x82269d34
	if ctx.cr[6].lt {
	pc = 0x82269D34; continue 'dispatch;
	}
	// 82269D1C: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269D20: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82269D24: 40820008  bne 0x82269d2c
	if !ctx.cr[0].eq {
	pc = 0x82269D2C; continue 'dispatch;
	}
	// 82269D28: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	pc = 0x82269D2C; continue 'dispatch;
            }
            0x82269D2C => {
    //   block [0x82269D2C..0x82269D34)
	// 82269D2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269D30: 4BFFD159  bl 0x82266e88
	ctx.lr = 0x82269D34;
	sub_82266E88(ctx, base);
	pc = 0x82269D34; continue 'dispatch;
            }
            0x82269D34 => {
    //   block [0x82269D34..0x82269D40)
	// 82269D34: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82269D38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269D3C: 4BFFDF85  bl 0x82267cc0
	ctx.lr = 0x82269D40;
	sub_82267CC0(ctx, base);
	pc = 0x82269D40; continue 'dispatch;
            }
            0x82269D40 => {
    //   block [0x82269D40..0x82269D54)
	// 82269D40: 897F2028  lbz r11, 0x2028(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8232 as u32) ) } as u64;
	// 82269D44: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82269D48: 4082000C  bne 0x82269d54
	if !ctx.cr[0].eq {
	pc = 0x82269D54; continue 'dispatch;
	}
	// 82269D4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269D50: 4BFFE3B1  bl 0x82268100
	ctx.lr = 0x82269D54;
	sub_82268100(ctx, base);
	pc = 0x82269D54; continue 'dispatch;
            }
            0x82269D54 => {
    //   block [0x82269D54..0x82269D60)
	// 82269D54: 807F202C  lwz r3, 0x202c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8236 as u32) ) } as u64;
	// 82269D58: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82269D5C: 4BE24D70  b 0x8208eacc
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82269D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82269D60 size=172
    let mut pc: u32 = 0x82269D60;
    'dispatch: loop {
        match pc {
            0x82269D60 => {
    //   block [0x82269D60..0x82269DA0)
	// 82269D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82269D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82269D68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82269D6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82269D70: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82269D74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82269D78: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82269D7C: 548B07FF  clrlwi. r11, r4, 0x1f
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269D80: 41820070  beq 0x82269df0
	if ctx.cr[0].eq {
	pc = 0x82269DF0; continue 'dispatch;
	}
	// 82269D84: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82269D88: 556B0529  rlwinm. r11, r11, 0, 0x14, 0x14
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269D8C: 41820014  beq 0x82269da0
	if ctx.cr[0].eq {
	pc = 0x82269DA0; continue 'dispatch;
	}
	// 82269D90: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82269D94: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82269D98: 38CBA2B8  addi r6, r11, -0x5d48
	ctx.r[6].s64 = ctx.r[11].s64 + -23880;
	// 82269D9C: 4BFFE475  bl 0x82268210
	ctx.lr = 0x82269DA0;
	sub_82268210(ctx, base);
	pc = 0x82269DA0; continue 'dispatch;
            }
            0x82269DA0 => {
    //   block [0x82269DA0..0x82269DC8)
	// 82269DA0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82269DA4: 556BA73E  rlwinm r11, r11, 0x14, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82269DA8: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82269DAC: 419A001C  beq cr6, 0x82269dc8
	if ctx.cr[6].eq {
	pc = 0x82269DC8; continue 'dispatch;
	}
	// 82269DB0: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82269DB4: 419A0014  beq cr6, 0x82269dc8
	if ctx.cr[6].eq {
	pc = 0x82269DC8; continue 'dispatch;
	}
	// 82269DB8: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 82269DBC: 419A000C  beq cr6, 0x82269dc8
	if ctx.cr[6].eq {
	pc = 0x82269DC8; continue 'dispatch;
	}
	// 82269DC0: 2B0B000B  cmplwi cr6, r11, 0xb
	ctx.cr[6].compare_u32(ctx.r[11].u32, 11 as u32, &mut ctx.xer);
	// 82269DC4: 409A002C  bne cr6, 0x82269df0
	if !ctx.cr[6].eq {
	pc = 0x82269DF0; continue 'dispatch;
	}
	pc = 0x82269DC8; continue 'dispatch;
            }
            0x82269DC8 => {
    //   block [0x82269DC8..0x82269DF0)
	// 82269DC8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269DCC: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 82269DD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82269DD4: 554804FE  clrlwi r8, r10, 0x13
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x00001FFFu64;
	// 82269DD8: 550AE8FA  rlwinm r10, r8, 0x1d, 3, 0x1d
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x00000007u64;
	// 82269DDC: 550806FE  clrlwi r8, r8, 0x1b
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82269DE0: 7D294030  slw r9, r9, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82269DE4: 7D0A582E  lwzx r8, r10, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82269DE8: 7D294378  or r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82269DEC: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	pc = 0x82269DF0; continue 'dispatch;
            }
            0x82269DF0 => {
    //   block [0x82269DF0..0x82269E0C)
	// 82269DF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82269DF4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82269DF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82269DFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82269E00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82269E04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82269E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82269E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82269E40 size=76
    let mut pc: u32 = 0x82269E40;
    'dispatch: loop {
        match pc {
            0x82269E40 => {
    //   block [0x82269E40..0x82269E8C)
	// 82269E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82269E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82269E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82269E4C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82269E50: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82269E54: 3D408227  lis r10, -0x7dd9
	ctx.r[10].s64 = -2111373312;
	// 82269E58: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82269E5C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82269E60: 394A9E28  addi r10, r10, -0x61d8
	ctx.r[10].s64 = ctx.r[10].s64 + -25048;
	// 82269E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82269E68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82269E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82269E70: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82269E74: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82269E78: 4BDFE7E1  bl 0x82068658
	ctx.lr = 0x82269E7C;
	sub_82068658(ctx, base);
	// 82269E7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82269E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82269E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82269E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82269E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82269E90 size=244
    let mut pc: u32 = 0x82269E90;
    'dispatch: loop {
        match pc {
            0x82269E90 => {
    //   block [0x82269E90..0x82269F78)
	// 82269E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82269E94: 4BE24C01  bl 0x8208ea94
	ctx.lr = 0x82269E98;
	sub_8208EA60(ctx, base);
	// 82269E98: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82269E9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82269EA0: 7D0940F8  nor r9, r8, r8
	ctx.r[9].u64 = !(ctx.r[8].u64 | ctx.r[8].u64);
	// 82269EA4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82269EA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82269EAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82269EB0: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 82269EB4: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82269EB8: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82269EBC: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82269EC0: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82269EC4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82269EC8: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82269ECC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82269ED0: 995F000C  stb r10, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82269ED4: 993F000D  stb r9, 0xd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(13 as u32), ctx.r[9].u8 ) };
	// 82269ED8: 995F2028  stb r10, 0x2028(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8232 as u32), ctx.r[10].u8 ) };
	// 82269EDC: 917F2030  stw r11, 0x2030(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8240 as u32), ctx.r[11].u32 ) };
	// 82269EE0: 917F202C  stw r11, 0x202c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8236 as u32), ctx.r[11].u32 ) };
	// 82269EE4: 4BFFFF5D  bl 0x82269e40
	ctx.lr = 0x82269EE8;
	sub_82269E40(ctx, base);
	// 82269EE8: 3D608227  lis r11, -0x7dd9
	ctx.r[11].s64 = -2111373312;
	// 82269EEC: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82269EF0: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82269EF4: 394B9E10  addi r10, r11, -0x61f0
	ctx.r[10].s64 = ctx.r[11].s64 + -25072;
	// 82269EF8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82269EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82269F00: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82269F04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82269F08: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82269F0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82269F10: 4BDFE749  bl 0x82068658
	ctx.lr = 0x82269F14;
	sub_82068658(ctx, base);
	// 82269F14: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82269F18: 41800060  blt 0x82269f78
	if ctx.cr[0].lt {
	pc = 0x82269F78; continue 'dispatch;
	}
	// 82269F1C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82269F20: 556B05EF  rlwinm. r11, r11, 0, 0x17, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269F24: 40820054  bne 0x82269f78
	if !ctx.cr[0].eq {
	pc = 0x82269F78; continue 'dispatch;
	}
	// 82269F28: 817F2030  lwz r11, 0x2030(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8240 as u32) ) } as u64;
	// 82269F2C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82269F30: 1F8B000C  mulli r28, r11, 0xc
	ctx.r[28].s32 = ((ctx.r[11].s32 as i64 * 12 as i64) as i32);
	ctx.r[28].s64 = ctx.r[28].s32 as i64;
	// 82269F34: 397C000C  addi r11, r28, 0xc
	ctx.r[11].s64 = ctx.r[28].s64 + 12;
	// 82269F38: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82269F3C: 4199003C  bgt cr6, 0x82269f78
	if ctx.cr[6].gt {
	pc = 0x82269F78; continue 'dispatch;
	}
	// 82269F40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269F44: 4BFFE1BD  bl 0x82268100
	ctx.lr = 0x82269F48;
	sub_82268100(ctx, base);
	// 82269F48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269F4C: 7FBCF214  add r29, r28, r30
	ctx.r[29].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 82269F50: 4BFFD1B9  bl 0x82267108
	ctx.lr = 0x82269F54;
	sub_82267108(ctx, base);
	// 82269F54: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82269F58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269F5C: 80FD0008  lwz r7, 8(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82269F60: 388BA2DC  addi r4, r11, -0x5d24
	ctx.r[4].s64 = ctx.r[11].s64 + -23844;
	// 82269F64: 80DD0004  lwz r6, 4(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82269F68: 7CBCF02E  lwzx r5, r28, r30
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82269F6C: 4BFFCF1D  bl 0x82266e88
	ctx.lr = 0x82269F70;
	sub_82266E88(ctx, base);
	// 82269F70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269F74: 4BFFE255  bl 0x822681c8
	ctx.lr = 0x82269F78;
	sub_822681C8(ctx, base);
	pc = 0x82269F78; continue 'dispatch;
            }
            0x82269F78 => {
    //   block [0x82269F78..0x82269F84)
	// 82269F78: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82269F7C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82269F80: 4BE24B64  b 0x8208eae4
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82269F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82269F88 size=276
    let mut pc: u32 = 0x82269F88;
    'dispatch: loop {
        match pc {
            0x82269F88 => {
    //   block [0x82269F88..0x82269FDC)
	// 82269F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82269F8C: 4BE24B01  bl 0x8208ea8c
	ctx.lr = 0x82269F90;
	sub_8208EA60(ctx, base);
	// 82269F90: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82269F94: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82269F98: 9421DF30  stwu r1, -0x20d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-8400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82269F9C: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 82269FA0: 81412124  lwz r10, 0x2124(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(8484 as u32) ) } as u64;
	// 82269FA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82269FA8: 554B07BE  clrlwi r11, r10, 0x1e
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 82269FAC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82269FB0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82269FB4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82269FB8: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 82269FBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82269FC0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82269FC4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82269FC8: 419A0024  beq cr6, 0x82269fec
	if ctx.cr[6].eq {
	pc = 0x82269FEC; continue 'dispatch;
	}
	// 82269FCC: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82269FD0: 409A000C  bne cr6, 0x82269fdc
	if !ctx.cr[6].eq {
	pc = 0x82269FDC; continue 'dispatch;
	}
	// 82269FD4: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82269FD8: 48000014  b 0x82269fec
	pc = 0x82269FEC; continue 'dispatch;
            }
            0x82269FDC => {
    //   block [0x82269FDC..0x82269FEC)
	// 82269FDC: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82269FE0: 419A000C  beq cr6, 0x82269fec
	if ctx.cr[6].eq {
	pc = 0x82269FEC; continue 'dispatch;
	}
	// 82269FE4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 82269FE8: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	pc = 0x82269FEC; continue 'dispatch;
            }
            0x82269FEC => {
    //   block [0x82269FEC..0x82269FF8)
	// 82269FEC: 554B0739  rlwinm. r11, r10, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269FF0: 41820008  beq 0x82269ff8
	if ctx.cr[0].eq {
	pc = 0x82269FF8; continue 'dispatch;
	}
	// 82269FF4: 63FF0002  ori r31, r31, 2
	ctx.r[31].u64 = ctx.r[31].u64 | 2;
	pc = 0x82269FF8; continue 'dispatch;
            }
            0x82269FF8 => {
    //   block [0x82269FF8..0x8226A004)
	// 82269FF8: 554B077B  rlwinm. r11, r10, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82269FFC: 41820008  beq 0x8226a004
	if ctx.cr[0].eq {
	pc = 0x8226A004; continue 'dispatch;
	}
	// 8226A000: 63FF0004  ori r31, r31, 4
	ctx.r[31].u64 = ctx.r[31].u64 | 4;
	pc = 0x8226A004; continue 'dispatch;
            }
            0x8226A004 => {
    //   block [0x8226A004..0x8226A010)
	// 8226A004: 554B0673  rlwinm. r11, r10, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226A008: 41820008  beq 0x8226a010
	if ctx.cr[0].eq {
	pc = 0x8226A010; continue 'dispatch;
	}
	// 8226A00C: 63FF0020  ori r31, r31, 0x20
	ctx.r[31].u64 = ctx.r[31].u64 | 32;
	pc = 0x8226A010; continue 'dispatch;
            }
            0x8226A010 => {
    //   block [0x8226A010..0x8226A01C)
	// 8226A010: 554B06F7  rlwinm. r11, r10, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226A014: 41820008  beq 0x8226a01c
	if ctx.cr[0].eq {
	pc = 0x8226A01C; continue 'dispatch;
	}
	// 8226A018: 63FF0010  ori r31, r31, 0x10
	ctx.r[31].u64 = ctx.r[31].u64 | 16;
	pc = 0x8226A01C; continue 'dispatch;
            }
            0x8226A01C => {
    //   block [0x8226A01C..0x8226A028)
	// 8226A01C: 554B0631  rlwinm. r11, r10, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226A020: 41820008  beq 0x8226a028
	if ctx.cr[0].eq {
	pc = 0x8226A028; continue 'dispatch;
	}
	// 8226A024: 63FF0040  ori r31, r31, 0x40
	ctx.r[31].u64 = ctx.r[31].u64 | 64;
	pc = 0x8226A028; continue 'dispatch;
            }
            0x8226A028 => {
    //   block [0x8226A028..0x8226A034)
	// 8226A028: 554B05AD  rlwinm. r11, r10, 0, 0x16, 0x16
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226A02C: 41820008  beq 0x8226a034
	if ctx.cr[0].eq {
	pc = 0x8226A034; continue 'dispatch;
	}
	// 8226A030: 63FF0080  ori r31, r31, 0x80
	ctx.r[31].u64 = ctx.r[31].u64 | 128;
	pc = 0x8226A034; continue 'dispatch;
            }
            0x8226A034 => {
    //   block [0x8226A034..0x8226A040)
	// 8226A034: 554B056B  rlwinm. r11, r10, 0, 0x15, 0x15
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226A038: 41820008  beq 0x8226a040
	if ctx.cr[0].eq {
	pc = 0x8226A040; continue 'dispatch;
	}
	// 8226A03C: 63FF0100  ori r31, r31, 0x100
	ctx.r[31].u64 = ctx.r[31].u64 | 256;
	pc = 0x8226A040; continue 'dispatch;
            }
            0x8226A040 => {
    //   block [0x8226A040..0x8226A094)
	// 8226A040: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8226A044: 41980050  blt cr6, 0x8226a094
	if ctx.cr[6].lt {
	pc = 0x8226A094; continue 'dispatch;
	}
	// 8226A048: 38A02000  li r5, 0x2000
	ctx.r[5].s64 = 8192;
	// 8226A04C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8226A050: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8226A054: 4BE24B2D  bl 0x8208eb80
	ctx.lr = 0x8226A058;
	sub_8208EB80(ctx, base);
	// 8226A058: 8161212C  lwz r11, 0x212c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(8492 as u32) ) } as u64;
	// 8226A05C: 81412134  lwz r10, 0x2134(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(8500 as u32) ) } as u64;
	// 8226A060: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8226A064: 93412070  stw r26, 0x2070(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(8304 as u32), ctx.r[26].u32 ) };
	// 8226A068: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226A06C: 93212074  stw r25, 0x2074(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(8308 as u32), ctx.r[25].u32 ) };
	// 8226A070: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8226A074: 99212079  stb r9, 0x2079(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(8313 as u32), ctx.r[9].u8 ) };
	// 8226A078: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8226A07C: 91612068  stw r11, 0x2068(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(8296 as u32), ctx.r[11].u32 ) };
	// 8226A080: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226A084: 9141206C  stw r10, 0x206c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(8300 as u32), ctx.r[10].u32 ) };
	// 8226A088: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226A08C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8226A090: 4BFFFE01  bl 0x82269e90
	ctx.lr = 0x8226A094;
	sub_82269E90(ctx, base);
	pc = 0x8226A094; continue 'dispatch;
            }
            0x8226A094 => {
    //   block [0x8226A094..0x8226A09C)
	// 8226A094: 382120D0  addi r1, r1, 0x20d0
	ctx.r[1].s64 = ctx.r[1].s64 + 8400;
	// 8226A098: 4BE24A44  b 0x8208eadc
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A0A0 size=16
    let mut pc: u32 = 0x8226A0A0;
    'dispatch: loop {
        match pc {
            0x8226A0A0 => {
    //   block [0x8226A0A0..0x8226A0B0)
	// 8226A0A0: 3D608229  lis r11, -0x7dd7
	ctx.r[11].s64 = -2111242240;
	// 8226A0A4: 38800072  li r4, 0x72
	ctx.r[4].s64 = 114;
	// 8226A0A8: 386BDB8C  addi r3, r11, -0x2474
	ctx.r[3].s64 = ctx.r[11].s64 + -9332;
	// 8226A0AC: 4BFB2D94  b 0x8221ce40
	sub_8221CE40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A0B0 size=20
    let mut pc: u32 = 0x8226A0B0;
    'dispatch: loop {
        match pc {
            0x8226A0B0 => {
    //   block [0x8226A0B0..0x8226A0C4)
	// 8226A0B0: 3D608229  lis r11, -0x7dd7
	ctx.r[11].s64 = -2111242240;
	// 8226A0B4: 3D408229  lis r10, -0x7dd7
	ctx.r[10].s64 = -2111242240;
	// 8226A0B8: 388BDB8C  addi r4, r11, -0x2474
	ctx.r[4].s64 = ctx.r[11].s64 + -9332;
	// 8226A0BC: 386ADBB8  addi r3, r10, -0x2448
	ctx.r[3].s64 = ctx.r[10].s64 + -9288;
	// 8226A0C0: 4BFB39B8  b 0x8221da78
	crate::recompiler::externs::call(ctx, base, 0x8221DA78);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226A0C8 size=84
    let mut pc: u32 = 0x8226A0C8;
    'dispatch: loop {
        match pc {
            0x8226A0C8 => {
    //   block [0x8226A0C8..0x8226A0F0)
	// 8226A0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226A0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226A0D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226A0D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226A0D8: 3D608229  lis r11, -0x7dd7
	ctx.r[11].s64 = -2111242240;
	// 8226A0DC: 3BEBC6DC  addi r31, r11, -0x3924
	ctx.r[31].s64 = ctx.r[11].s64 + -14628;
	// 8226A0E0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226A0E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226A0E8: 419A0008  beq cr6, 0x8226a0f0
	if ctx.cr[6].eq {
	pc = 0x8226A0F0; continue 'dispatch;
	}
	// 8226A0EC: 4BE25AE5  bl 0x8208fbd0
	ctx.lr = 0x8226A0F0;
	sub_8208FBD0(ctx, base);
	pc = 0x8226A0F0; continue 'dispatch;
            }
            0x8226A0F0 => {
    //   block [0x8226A0F0..0x8226A11C)
	// 8226A0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226A0F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8226A0F8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8226A0FC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8226A100: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8226A104: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8226A108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226A10C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226A110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226A114: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226A118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A14C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A14C size=16
    let mut pc: u32 = 0x8226A14C;
    'dispatch: loop {
        match pc {
            0x8226A14C => {
    //   block [0x8226A14C..0x8226A15C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A15C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A15C size=16
    let mut pc: u32 = 0x8226A15C;
    'dispatch: loop {
        match pc {
            0x8226A15C => {
    //   block [0x8226A15C..0x8226A16C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A16C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A16C size=16
    let mut pc: u32 = 0x8226A16C;
    'dispatch: loop {
        match pc {
            0x8226A16C => {
    //   block [0x8226A16C..0x8226A17C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A17C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A17C size=16
    let mut pc: u32 = 0x8226A17C;
    'dispatch: loop {
        match pc {
            0x8226A17C => {
    //   block [0x8226A17C..0x8226A18C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A18C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A18C size=16
    let mut pc: u32 = 0x8226A18C;
    'dispatch: loop {
        match pc {
            0x8226A18C => {
    //   block [0x8226A18C..0x8226A19C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A19C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A19C size=16
    let mut pc: u32 = 0x8226A19C;
    'dispatch: loop {
        match pc {
            0x8226A19C => {
    //   block [0x8226A19C..0x8226A1AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A1AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A1AC size=16
    let mut pc: u32 = 0x8226A1AC;
    'dispatch: loop {
        match pc {
            0x8226A1AC => {
    //   block [0x8226A1AC..0x8226A1BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A1BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A1BC size=16
    let mut pc: u32 = 0x8226A1BC;
    'dispatch: loop {
        match pc {
            0x8226A1BC => {
    //   block [0x8226A1BC..0x8226A1CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A1CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A1CC size=16
    let mut pc: u32 = 0x8226A1CC;
    'dispatch: loop {
        match pc {
            0x8226A1CC => {
    //   block [0x8226A1CC..0x8226A1DC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A1DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A1DC size=16
    let mut pc: u32 = 0x8226A1DC;
    'dispatch: loop {
        match pc {
            0x8226A1DC => {
    //   block [0x8226A1DC..0x8226A1EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A1EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A1EC size=16
    let mut pc: u32 = 0x8226A1EC;
    'dispatch: loop {
        match pc {
            0x8226A1EC => {
    //   block [0x8226A1EC..0x8226A1FC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A1FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A1FC size=16
    let mut pc: u32 = 0x8226A1FC;
    'dispatch: loop {
        match pc {
            0x8226A1FC => {
    //   block [0x8226A1FC..0x8226A20C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A20C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A20C size=16
    let mut pc: u32 = 0x8226A20C;
    'dispatch: loop {
        match pc {
            0x8226A20C => {
    //   block [0x8226A20C..0x8226A21C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A21C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A21C size=16
    let mut pc: u32 = 0x8226A21C;
    'dispatch: loop {
        match pc {
            0x8226A21C => {
    //   block [0x8226A21C..0x8226A22C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A22C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A22C size=16
    let mut pc: u32 = 0x8226A22C;
    'dispatch: loop {
        match pc {
            0x8226A22C => {
    //   block [0x8226A22C..0x8226A23C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A23C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A23C size=16
    let mut pc: u32 = 0x8226A23C;
    'dispatch: loop {
        match pc {
            0x8226A23C => {
    //   block [0x8226A23C..0x8226A24C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A24C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A24C size=16
    let mut pc: u32 = 0x8226A24C;
    'dispatch: loop {
        match pc {
            0x8226A24C => {
    //   block [0x8226A24C..0x8226A25C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A25C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A25C size=16
    let mut pc: u32 = 0x8226A25C;
    'dispatch: loop {
        match pc {
            0x8226A25C => {
    //   block [0x8226A25C..0x8226A26C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A26C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A26C size=16
    let mut pc: u32 = 0x8226A26C;
    'dispatch: loop {
        match pc {
            0x8226A26C => {
    //   block [0x8226A26C..0x8226A27C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A27C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A27C size=16
    let mut pc: u32 = 0x8226A27C;
    'dispatch: loop {
        match pc {
            0x8226A27C => {
    //   block [0x8226A27C..0x8226A28C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A28C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A28C size=16
    let mut pc: u32 = 0x8226A28C;
    'dispatch: loop {
        match pc {
            0x8226A28C => {
    //   block [0x8226A28C..0x8226A29C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A29C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A29C size=16
    let mut pc: u32 = 0x8226A29C;
    'dispatch: loop {
        match pc {
            0x8226A29C => {
    //   block [0x8226A29C..0x8226A2AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A2AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A2AC size=16
    let mut pc: u32 = 0x8226A2AC;
    'dispatch: loop {
        match pc {
            0x8226A2AC => {
    //   block [0x8226A2AC..0x8226A2BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A2BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A2BC size=16
    let mut pc: u32 = 0x8226A2BC;
    'dispatch: loop {
        match pc {
            0x8226A2BC => {
    //   block [0x8226A2BC..0x8226A2CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A2CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A2CC size=16
    let mut pc: u32 = 0x8226A2CC;
    'dispatch: loop {
        match pc {
            0x8226A2CC => {
    //   block [0x8226A2CC..0x8226A2DC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A2DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A2DC size=16
    let mut pc: u32 = 0x8226A2DC;
    'dispatch: loop {
        match pc {
            0x8226A2DC => {
    //   block [0x8226A2DC..0x8226A2EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A2EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A2EC size=16
    let mut pc: u32 = 0x8226A2EC;
    'dispatch: loop {
        match pc {
            0x8226A2EC => {
    //   block [0x8226A2EC..0x8226A2FC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A2FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A2FC size=16
    let mut pc: u32 = 0x8226A2FC;
    'dispatch: loop {
        match pc {
            0x8226A2FC => {
    //   block [0x8226A2FC..0x8226A30C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A30C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A30C size=16
    let mut pc: u32 = 0x8226A30C;
    'dispatch: loop {
        match pc {
            0x8226A30C => {
    //   block [0x8226A30C..0x8226A31C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A31C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A31C size=16
    let mut pc: u32 = 0x8226A31C;
    'dispatch: loop {
        match pc {
            0x8226A31C => {
    //   block [0x8226A31C..0x8226A32C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A32C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A32C size=16
    let mut pc: u32 = 0x8226A32C;
    'dispatch: loop {
        match pc {
            0x8226A32C => {
    //   block [0x8226A32C..0x8226A33C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A33C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A33C size=16
    let mut pc: u32 = 0x8226A33C;
    'dispatch: loop {
        match pc {
            0x8226A33C => {
    //   block [0x8226A33C..0x8226A34C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A34C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A34C size=16
    let mut pc: u32 = 0x8226A34C;
    'dispatch: loop {
        match pc {
            0x8226A34C => {
    //   block [0x8226A34C..0x8226A35C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A35C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A35C size=16
    let mut pc: u32 = 0x8226A35C;
    'dispatch: loop {
        match pc {
            0x8226A35C => {
    //   block [0x8226A35C..0x8226A36C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A36C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A36C size=16
    let mut pc: u32 = 0x8226A36C;
    'dispatch: loop {
        match pc {
            0x8226A36C => {
    //   block [0x8226A36C..0x8226A37C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A37C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A37C size=16
    let mut pc: u32 = 0x8226A37C;
    'dispatch: loop {
        match pc {
            0x8226A37C => {
    //   block [0x8226A37C..0x8226A38C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A38C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A38C size=16
    let mut pc: u32 = 0x8226A38C;
    'dispatch: loop {
        match pc {
            0x8226A38C => {
    //   block [0x8226A38C..0x8226A39C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A39C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A39C size=16
    let mut pc: u32 = 0x8226A39C;
    'dispatch: loop {
        match pc {
            0x8226A39C => {
    //   block [0x8226A39C..0x8226A3AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A3AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A3AC size=16
    let mut pc: u32 = 0x8226A3AC;
    'dispatch: loop {
        match pc {
            0x8226A3AC => {
    //   block [0x8226A3AC..0x8226A3BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A3BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A3BC size=16
    let mut pc: u32 = 0x8226A3BC;
    'dispatch: loop {
        match pc {
            0x8226A3BC => {
    //   block [0x8226A3BC..0x8226A3CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A3CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A3CC size=16
    let mut pc: u32 = 0x8226A3CC;
    'dispatch: loop {
        match pc {
            0x8226A3CC => {
    //   block [0x8226A3CC..0x8226A3DC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A3DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A3DC size=16
    let mut pc: u32 = 0x8226A3DC;
    'dispatch: loop {
        match pc {
            0x8226A3DC => {
    //   block [0x8226A3DC..0x8226A3EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A3EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A3EC size=16
    let mut pc: u32 = 0x8226A3EC;
    'dispatch: loop {
        match pc {
            0x8226A3EC => {
    //   block [0x8226A3EC..0x8226A3FC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A3FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A3FC size=16
    let mut pc: u32 = 0x8226A3FC;
    'dispatch: loop {
        match pc {
            0x8226A3FC => {
    //   block [0x8226A3FC..0x8226A40C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A40C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A40C size=16
    let mut pc: u32 = 0x8226A40C;
    'dispatch: loop {
        match pc {
            0x8226A40C => {
    //   block [0x8226A40C..0x8226A41C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A41C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A41C size=16
    let mut pc: u32 = 0x8226A41C;
    'dispatch: loop {
        match pc {
            0x8226A41C => {
    //   block [0x8226A41C..0x8226A42C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A42C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A42C size=16
    let mut pc: u32 = 0x8226A42C;
    'dispatch: loop {
        match pc {
            0x8226A42C => {
    //   block [0x8226A42C..0x8226A43C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A43C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A43C size=16
    let mut pc: u32 = 0x8226A43C;
    'dispatch: loop {
        match pc {
            0x8226A43C => {
    //   block [0x8226A43C..0x8226A44C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A44C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A44C size=16
    let mut pc: u32 = 0x8226A44C;
    'dispatch: loop {
        match pc {
            0x8226A44C => {
    //   block [0x8226A44C..0x8226A45C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A45C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A45C size=16
    let mut pc: u32 = 0x8226A45C;
    'dispatch: loop {
        match pc {
            0x8226A45C => {
    //   block [0x8226A45C..0x8226A46C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A46C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A46C size=16
    let mut pc: u32 = 0x8226A46C;
    'dispatch: loop {
        match pc {
            0x8226A46C => {
    //   block [0x8226A46C..0x8226A47C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A47C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A47C size=16
    let mut pc: u32 = 0x8226A47C;
    'dispatch: loop {
        match pc {
            0x8226A47C => {
    //   block [0x8226A47C..0x8226A48C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A48C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A48C size=16
    let mut pc: u32 = 0x8226A48C;
    'dispatch: loop {
        match pc {
            0x8226A48C => {
    //   block [0x8226A48C..0x8226A49C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A49C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A49C size=16
    let mut pc: u32 = 0x8226A49C;
    'dispatch: loop {
        match pc {
            0x8226A49C => {
    //   block [0x8226A49C..0x8226A4AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A4AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A4AC size=16
    let mut pc: u32 = 0x8226A4AC;
    'dispatch: loop {
        match pc {
            0x8226A4AC => {
    //   block [0x8226A4AC..0x8226A4BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A4BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A4BC size=16
    let mut pc: u32 = 0x8226A4BC;
    'dispatch: loop {
        match pc {
            0x8226A4BC => {
    //   block [0x8226A4BC..0x8226A4CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A4CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A4CC size=16
    let mut pc: u32 = 0x8226A4CC;
    'dispatch: loop {
        match pc {
            0x8226A4CC => {
    //   block [0x8226A4CC..0x8226A4DC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A4DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A4DC size=16
    let mut pc: u32 = 0x8226A4DC;
    'dispatch: loop {
        match pc {
            0x8226A4DC => {
    //   block [0x8226A4DC..0x8226A4EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A4EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A4EC size=16
    let mut pc: u32 = 0x8226A4EC;
    'dispatch: loop {
        match pc {
            0x8226A4EC => {
    //   block [0x8226A4EC..0x8226A4FC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A4FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A4FC size=16
    let mut pc: u32 = 0x8226A4FC;
    'dispatch: loop {
        match pc {
            0x8226A4FC => {
    //   block [0x8226A4FC..0x8226A50C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A51C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A51C size=16
    let mut pc: u32 = 0x8226A51C;
    'dispatch: loop {
        match pc {
            0x8226A51C => {
    //   block [0x8226A51C..0x8226A52C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A52C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A52C size=16
    let mut pc: u32 = 0x8226A52C;
    'dispatch: loop {
        match pc {
            0x8226A52C => {
    //   block [0x8226A52C..0x8226A53C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A53C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A53C size=16
    let mut pc: u32 = 0x8226A53C;
    'dispatch: loop {
        match pc {
            0x8226A53C => {
    //   block [0x8226A53C..0x8226A54C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A54C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A54C size=16
    let mut pc: u32 = 0x8226A54C;
    'dispatch: loop {
        match pc {
            0x8226A54C => {
    //   block [0x8226A54C..0x8226A55C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A55C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A55C size=16
    let mut pc: u32 = 0x8226A55C;
    'dispatch: loop {
        match pc {
            0x8226A55C => {
    //   block [0x8226A55C..0x8226A56C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A56C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A56C size=16
    let mut pc: u32 = 0x8226A56C;
    'dispatch: loop {
        match pc {
            0x8226A56C => {
    //   block [0x8226A56C..0x8226A57C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A57C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A57C size=16
    let mut pc: u32 = 0x8226A57C;
    'dispatch: loop {
        match pc {
            0x8226A57C => {
    //   block [0x8226A57C..0x8226A58C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A58C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A58C size=16
    let mut pc: u32 = 0x8226A58C;
    'dispatch: loop {
        match pc {
            0x8226A58C => {
    //   block [0x8226A58C..0x8226A59C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A59C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A59C size=16
    let mut pc: u32 = 0x8226A59C;
    'dispatch: loop {
        match pc {
            0x8226A59C => {
    //   block [0x8226A59C..0x8226A5AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A5AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A5AC size=16
    let mut pc: u32 = 0x8226A5AC;
    'dispatch: loop {
        match pc {
            0x8226A5AC => {
    //   block [0x8226A5AC..0x8226A5BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A5BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A5BC size=16
    let mut pc: u32 = 0x8226A5BC;
    'dispatch: loop {
        match pc {
            0x8226A5BC => {
    //   block [0x8226A5BC..0x8226A5CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A5CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A5CC size=16
    let mut pc: u32 = 0x8226A5CC;
    'dispatch: loop {
        match pc {
            0x8226A5CC => {
    //   block [0x8226A5CC..0x8226A5DC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A5DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A5DC size=16
    let mut pc: u32 = 0x8226A5DC;
    'dispatch: loop {
        match pc {
            0x8226A5DC => {
    //   block [0x8226A5DC..0x8226A5EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A5EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A5EC size=16
    let mut pc: u32 = 0x8226A5EC;
    'dispatch: loop {
        match pc {
            0x8226A5EC => {
    //   block [0x8226A5EC..0x8226A5FC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A5FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A5FC size=16
    let mut pc: u32 = 0x8226A5FC;
    'dispatch: loop {
        match pc {
            0x8226A5FC => {
    //   block [0x8226A5FC..0x8226A60C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A60C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A60C size=16
    let mut pc: u32 = 0x8226A60C;
    'dispatch: loop {
        match pc {
            0x8226A60C => {
    //   block [0x8226A60C..0x8226A61C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A61C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A61C size=16
    let mut pc: u32 = 0x8226A61C;
    'dispatch: loop {
        match pc {
            0x8226A61C => {
    //   block [0x8226A61C..0x8226A62C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A62C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A62C size=16
    let mut pc: u32 = 0x8226A62C;
    'dispatch: loop {
        match pc {
            0x8226A62C => {
    //   block [0x8226A62C..0x8226A63C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A63C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A63C size=16
    let mut pc: u32 = 0x8226A63C;
    'dispatch: loop {
        match pc {
            0x8226A63C => {
    //   block [0x8226A63C..0x8226A64C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A64C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A64C size=16
    let mut pc: u32 = 0x8226A64C;
    'dispatch: loop {
        match pc {
            0x8226A64C => {
    //   block [0x8226A64C..0x8226A65C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A65C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A65C size=16
    let mut pc: u32 = 0x8226A65C;
    'dispatch: loop {
        match pc {
            0x8226A65C => {
    //   block [0x8226A65C..0x8226A66C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A66C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A66C size=16
    let mut pc: u32 = 0x8226A66C;
    'dispatch: loop {
        match pc {
            0x8226A66C => {
    //   block [0x8226A66C..0x8226A67C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A67C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A67C size=16
    let mut pc: u32 = 0x8226A67C;
    'dispatch: loop {
        match pc {
            0x8226A67C => {
    //   block [0x8226A67C..0x8226A68C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A68C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A68C size=16
    let mut pc: u32 = 0x8226A68C;
    'dispatch: loop {
        match pc {
            0x8226A68C => {
    //   block [0x8226A68C..0x8226A69C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A69C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A69C size=16
    let mut pc: u32 = 0x8226A69C;
    'dispatch: loop {
        match pc {
            0x8226A69C => {
    //   block [0x8226A69C..0x8226A6AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A6AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A6AC size=16
    let mut pc: u32 = 0x8226A6AC;
    'dispatch: loop {
        match pc {
            0x8226A6AC => {
    //   block [0x8226A6AC..0x8226A6BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A6EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A6EC size=16
    let mut pc: u32 = 0x8226A6EC;
    'dispatch: loop {
        match pc {
            0x8226A6EC => {
    //   block [0x8226A6EC..0x8226A6FC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A6FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A6FC size=16
    let mut pc: u32 = 0x8226A6FC;
    'dispatch: loop {
        match pc {
            0x8226A6FC => {
    //   block [0x8226A6FC..0x8226A70C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A70C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A70C size=16
    let mut pc: u32 = 0x8226A70C;
    'dispatch: loop {
        match pc {
            0x8226A70C => {
    //   block [0x8226A70C..0x8226A71C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A71C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A71C size=16
    let mut pc: u32 = 0x8226A71C;
    'dispatch: loop {
        match pc {
            0x8226A71C => {
    //   block [0x8226A71C..0x8226A72C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A72C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A72C size=16
    let mut pc: u32 = 0x8226A72C;
    'dispatch: loop {
        match pc {
            0x8226A72C => {
    //   block [0x8226A72C..0x8226A73C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A74C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A74C size=16
    let mut pc: u32 = 0x8226A74C;
    'dispatch: loop {
        match pc {
            0x8226A74C => {
    //   block [0x8226A74C..0x8226A75C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A75C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A75C size=16
    let mut pc: u32 = 0x8226A75C;
    'dispatch: loop {
        match pc {
            0x8226A75C => {
    //   block [0x8226A75C..0x8226A76C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A76C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A76C size=16
    let mut pc: u32 = 0x8226A76C;
    'dispatch: loop {
        match pc {
            0x8226A76C => {
    //   block [0x8226A76C..0x8226A77C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A77C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A77C size=16
    let mut pc: u32 = 0x8226A77C;
    'dispatch: loop {
        match pc {
            0x8226A77C => {
    //   block [0x8226A77C..0x8226A78C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A78C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A78C size=16
    let mut pc: u32 = 0x8226A78C;
    'dispatch: loop {
        match pc {
            0x8226A78C => {
    //   block [0x8226A78C..0x8226A79C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A79C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A79C size=16
    let mut pc: u32 = 0x8226A79C;
    'dispatch: loop {
        match pc {
            0x8226A79C => {
    //   block [0x8226A79C..0x8226A7AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A7AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A7AC size=16
    let mut pc: u32 = 0x8226A7AC;
    'dispatch: loop {
        match pc {
            0x8226A7AC => {
    //   block [0x8226A7AC..0x8226A7BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A7BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A7BC size=16
    let mut pc: u32 = 0x8226A7BC;
    'dispatch: loop {
        match pc {
            0x8226A7BC => {
    //   block [0x8226A7BC..0x8226A7CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


