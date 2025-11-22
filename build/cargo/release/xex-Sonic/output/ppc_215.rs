pub fn sub_82F17E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F17E68 size=216
    let mut pc: u32 = 0x82F17E68;
    'dispatch: loop {
        match pc {
            0x82F17E68 => {
    //   block [0x82F17E68..0x82F17F40)
	// 82F17E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F17E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F17E70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F17E74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F17E78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F17E7C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F17E80: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F17E84: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F17E88: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82F17E8C: 388ABDE4  addi r4, r10, -0x421c
	ctx.r[4].s64 = ctx.r[10].s64 + -16924;
	// 82F17E90: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F17E94: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F17E98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F17E9C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F17EA0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F17EA4: 4E800421  bctrl
	ctx.lr = 0x82F17EA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F17EA8: 80DE0050  lwz r6, 0x50(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F17EAC: 391E0070  addi r8, r30, 0x70
	ctx.r[8].s64 = ctx.r[30].s64 + 112;
	// 82F17EB0: 7F064040  cmplw cr6, r6, r8
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F17EB4: 419A0030  beq cr6, 0x82f17ee4
	if ctx.cr[6].eq {
	pc = 0x82F17EE4; continue 'dispatch;
	}
	// 82F17EB8: 817E0054  lwz r11, 0x54(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F17EBC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F17EC0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F17EC4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F17EC8: 55673032  slwi r7, r11, 6
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F17ECC: 388ABDD0  addi r4, r10, -0x4230
	ctx.r[4].s64 = ctx.r[10].s64 + -16944;
	// 82F17ED0: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82F17ED4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F17ED8: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F17EDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F17EE0: 4E800421  bctrl
	ctx.lr = 0x82F17EE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F17EE4: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F17EE8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F17EEC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F17EF0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F17EF4: 55673032  slwi r7, r11, 6
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F17EF8: 80DE0058  lwz r6, 0x58(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F17EFC: 388ABDC0  addi r4, r10, -0x4240
	ctx.r[4].s64 = ctx.r[10].s64 + -16960;
	// 82F17F00: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82F17F04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F17F08: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F17F0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F17F10: 4E800421  bctrl
	ctx.lr = 0x82F17F14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F17F14: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F17F18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F17F1C: 812A0018  lwz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F17F20: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F17F24: 4E800421  bctrl
	ctx.lr = 0x82F17F28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F17F28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F17F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F17F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F17F34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F17F38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F17F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F17F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F17F40 size=244
    let mut pc: u32 = 0x82F17F40;
    'dispatch: loop {
        match pc {
            0x82F17F40 => {
    //   block [0x82F17F40..0x82F18034)
	// 82F17F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F17F44: 48290219  bl 0x831a815c
	ctx.lr = 0x82F17F48;
	sub_831A8130(ctx, base);
	// 82F17F48: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F17F4C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F17F50: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82F17F54: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82F17F58: 817B005C  lwz r11, 0x5c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F17F5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F17F60: 409900BC  ble cr6, 0x82f1801c
	if !ctx.cr[6].gt {
	pc = 0x82F1801C; continue 'dispatch;
	}
	// 82F17F64: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82F17F68: 817B0058  lwz r11, 0x58(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F17F6C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F17F70: 7FFC5A14  add r31, r28, r11
	ctx.r[31].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 82F17F74: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F17F78: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F17F7C: 4099008C  ble cr6, 0x82f18008
	if !ctx.cr[6].gt {
	pc = 0x82F18008; continue 'dispatch;
	}
	// 82F17F80: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F17F84: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F17F88: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 82F17F8C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82F17F90: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F17F94: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F17F98: 812A0018  lwz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F17F9C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F17FA0: 4E800421  bctrl
	ctx.lr = 0x82F17FA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F17FA4: 891F0018  lbz r8, 0x18(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F17FA8: 7D0B0774  extsb r11, r8
	ctx.r[11].s64 = ctx.r[8].s8 as i64;
	// 82F17FAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F17FB0: 409A0018  bne cr6, 0x82f17fc8
	if !ctx.cr[6].eq {
	pc = 0x82F17FC8; continue 'dispatch;
	}
	// 82F17FB4: 895F0019  lbz r10, 0x19(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 82F17FB8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F17FBC: 409A000C  bne cr6, 0x82f17fc8
	if !ctx.cr[6].eq {
	pc = 0x82F17FC8; continue 'dispatch;
	}
	// 82F17FC0: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 82F17FC4: 48000020  b 0x82f17fe4
	pc = 0x82F17FE4; continue 'dispatch;
	// 82F17FC8: 895F0019  lbz r10, 0x19(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 82F17FCC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F17FD0: 409A0010  bne cr6, 0x82f17fe0
	if !ctx.cr[6].eq {
	pc = 0x82F17FE0; continue 'dispatch;
	}
	// 82F17FD4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F17FD8: 396001D0  li r11, 0x1d0
	ctx.r[11].s64 = 464;
	// 82F17FDC: 409A0008  bne cr6, 0x82f17fe4
	if !ctx.cr[6].eq {
	pc = 0x82F17FE4; continue 'dispatch;
	}
	// 82F17FE0: 396001A0  li r11, 0x1a0
	ctx.r[11].s64 = 416;
	// 82F17FE4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82F17FE8: 41980040  blt cr6, 0x82f18028
	if ctx.cr[6].lt {
	pc = 0x82F18028; continue 'dispatch;
	}
	// 82F17FEC: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F17FF0: 41990038  bgt cr6, 0x82f18028
	if ctx.cr[6].gt {
	pc = 0x82F18028; continue 'dispatch;
	}
	// 82F17FF4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F17FF8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F17FFC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82F18000: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F18004: 4198FF80  blt cr6, 0x82f17f84
	if ctx.cr[6].lt {
	pc = 0x82F17F84; continue 'dispatch;
	}
	// 82F18008: 817B005C  lwz r11, 0x5c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F1800C: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82F18010: 3B9C0040  addi r28, r28, 0x40
	ctx.r[28].s64 = ctx.r[28].s64 + 64;
	// 82F18014: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F18018: 4198FF50  blt cr6, 0x82f17f68
	if ctx.cr[6].lt {
	pc = 0x82F17F68; continue 'dispatch;
	}
	// 82F1801C: 386000C0  li r3, 0xc0
	ctx.r[3].s64 = 192;
	// 82F18020: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82F18024: 48290188  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 82F18028: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82F1802C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82F18030: 4829017C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F18038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F18038 size=60
    let mut pc: u32 = 0x82F18038;
    'dispatch: loop {
        match pc {
            0x82F18038 => {
    //   block [0x82F18038..0x82F18074)
	// 82F18038: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1803C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82F18040: 5489007E  clrlwi r9, r4, 1
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x7FFFFFFFu64;
	// 82F18044: 7D475C30  srw r7, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[10].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82F18048: 210B0020  subfic r8, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[11].s64;
	// 82F1804C: 54860000  rlwinm r6, r4, 0, 0, 0
	ctx.r[6].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82F18050: 7D2B4430  srw r11, r9, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82F18054: 7CE72038  and r7, r7, r4
	ctx.r[7].u64 = ctx.r[7].u64 & ctx.r[4].u64;
	// 82F18058: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F1805C: 409A0020  bne cr6, 0x82f1807c
	if !ctx.cr[6].eq {
		sub_82F1807C(ctx, base);
		return;
	}
	// 82F18060: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F18064: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82F18068: 409A000C  bne cr6, 0x82f18074
	if !ctx.cr[6].eq {
		sub_82F18074(ctx, base);
		return;
	}
	// 82F1806C: 39630070  addi r11, r3, 0x70
	ctx.r[11].s64 = ctx.r[3].s64 + 112;
	// 82F18070: 48000018  b 0x82f18088
	sub_82F1807C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F18074(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F18074 size=8
    let mut pc: u32 = 0x82F18074;
    'dispatch: loop {
        match pc {
            0x82F18074 => {
    //   block [0x82F18074..0x82F1807C)
	// 82F18074: 81430050  lwz r10, 0x50(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F18078: 48000008  b 0x82f18080
	sub_82F1807C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1807C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1807C size=72
    let mut pc: u32 = 0x82F1807C;
    'dispatch: loop {
        match pc {
            0x82F1807C => {
    //   block [0x82F1807C..0x82F180C4)
	// 82F1807C: 81430058  lwz r10, 0x58(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F18080: 556B3032  slwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F18084: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F18088: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1808C: A10B0002  lhz r8, 2(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82F18090: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F18094: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F18098: 7D080734  extsh r8, r8
	ctx.r[8].s64 = ctx.r[8].s16 as i64;
	// 82F1809C: 419A0038  beq cr6, 0x82f180d4
	if ctx.cr[6].eq {
		sub_82F180D4(ctx, base);
		return;
	}
	// 82F180A0: 88CB0001  lbz r6, 1(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82F180A4: A0AB0008  lhz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F180A8: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 82F180AC: 7D6539D6  mullw r11, r5, r7
	ctx.r[11].s64 = (ctx.r[5].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82F180B0: 409A0014  bne cr6, 0x82f180c4
	if !ctx.cr[6].eq {
		sub_82F180C4(ctx, base);
		return;
	}
	// 82F180B4: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F180B8: 7D6B41D6  mullw r11, r11, r8
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82F180BC: 7C6B4A14  add r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F180C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F180C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F180C4 size=16
    let mut pc: u32 = 0x82F180C4;
    'dispatch: loop {
        match pc {
            0x82F180C4 => {
    //   block [0x82F180C4..0x82F180D4)
	// 82F180C4: 7D6B522E  lhzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F180C8: 7D6B41D6  mullw r11, r11, r8
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82F180CC: 7C6B4A14  add r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F180D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F180D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F180D4 size=8
    let mut pc: u32 = 0x82F180D4;
    'dispatch: loop {
        match pc {
            0x82F180D4 => {
    //   block [0x82F180D4..0x82F180DC)
	// 82F180D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F180D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F180E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F180E0 size=68
    let mut pc: u32 = 0x82F180E0;
    'dispatch: loop {
        match pc {
            0x82F180E0 => {
    //   block [0x82F180E0..0x82F18124)
	// 82F180E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F180E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F180E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F180EC: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82F180F0: 4BFFFF49  bl 0x82f18038
	ctx.lr = 0x82F180F4;
	sub_82F18038(ctx, base);
	// 82F180F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F180F8: 419A0018  beq cr6, 0x82f18110
	if ctx.cr[6].eq {
	pc = 0x82F18110; continue 'dispatch;
	}
	// 82F180FC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F18100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F18104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F18108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1810C: 4E800020  blr
	return;
	// 82F18110: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F18114: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F18118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1811C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F18120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F18128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F18128 size=164
    let mut pc: u32 = 0x82F18128;
    'dispatch: loop {
        match pc {
            0x82F18128 => {
    //   block [0x82F18128..0x82F181CC)
	// 82F18128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1812C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F18130: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F18134: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F18138: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F1813C: C0050000  lfs f0, 0(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F18140: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82F18144: C1A60000  lfs f13, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F18148: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82F1814C: C1850004  lfs f12, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F18150: C1250008  lfs f9, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82F18154: ED400372  fmuls f10, f0, f13
	ctx.f[10].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82F18158: C1660004  lfs f11, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F1815C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F18160: C0E60008  lfs f7, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82F18164: ED0C02F2  fmuls f8, f12, f11
	ctx.f[8].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 82F18168: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1816C: ECC901F2  fmuls f6, f9, f7
	ctx.f[6].f64 = (((ctx.f[9].f64 * ctx.f[7].f64) as f32) as f64);
	// 82F18170: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82F18174: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82F18178: D1410050  stfs f10, 0x50(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82F1817C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F18180: D1010054  stfs f8, 0x54(r1)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F18184: D0C10058  stfs f6, 0x58(r1)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F181D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F181D0 size=308
    let mut pc: u32 = 0x82F181D0;
    'dispatch: loop {
        match pc {
            0x82F181D0 => {
    //   block [0x82F181D0..0x82F18304)
	// 82F181D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F181D4: 4828FF89  bl 0x831a815c
	ctx.lr = 0x82F181D8;
	sub_831A8130(ctx, base);
	// 82F181D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F181DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F181E0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F181E4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F181E8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82F181EC: 390AC5B0  addi r8, r10, -0x3a50
	ctx.r[8].s64 = ctx.r[10].s64 + -14928;
	// 82F181F0: C00BBA78  lfs f0, -0x4588(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F181F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F181F8: D01B0000  stfs f0, 0(r27)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82F181FC: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F18308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F18308 size=244
    let mut pc: u32 = 0x82F18308;
    'dispatch: loop {
        match pc {
            0x82F18308 => {
    //   block [0x82F18308..0x82F183FC)
	// 82F18308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1830C: 4828FE59  bl 0x831a8164
	ctx.lr = 0x82F18310;
	sub_831A8130(ctx, base);
	// 82F18310: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82F18314: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F18318: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F1831C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F18320: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F18324: 3B7E0010  addi r27, r30, 0x10
	ctx.r[27].s64 = ctx.r[30].s64 + 16;
	// 82F18328: 392AC5B0  addi r9, r10, -0x3a50
	ctx.r[9].s64 = ctx.r[10].s64 + -14928;
	// 82F1832C: C00BBA78  lfs f0, -0x4588(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F18330: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F18334: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82F18338: 390100A0  addi r8, r1, 0xa0
	ctx.r[8].s64 = ctx.r[1].s64 + 160;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F18400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F18400 size=412
    let mut pc: u32 = 0x82F18400;
    'dispatch: loop {
        match pc {
            0x82F18400 => {
    //   block [0x82F18400..0x82F1859C)
	// 82F18400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F18404: 4828FD65  bl 0x831a8168
	ctx.lr = 0x82F18408;
	sub_831A8130(ctx, base);
	// 82F18408: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F185A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F185A0 size=376
    let mut pc: u32 = 0x82F185A0;
    'dispatch: loop {
        match pc {
            0x82F185A0 => {
    //   block [0x82F185A0..0x82F18718)
	// 82F185A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F185A4: 4828FBC1  bl 0x831a8164
	ctx.lr = 0x82F185A8;
	sub_831A8130(ctx, base);
	// 82F185A8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F185AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F185B0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F185B4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82F185B8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82F185BC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F185C0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F185C4: B37F0008  sth r27, 8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u16 ) };
	// 82F185C8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F185CC: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82F185D0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F185D4: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82F185D8: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82F185DC: B37F0002  sth r27, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[27].u16 ) };
	// 82F185E0: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 82F185E4: B17F000A  sth r11, 0xa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 82F185E8: 391F0030  addi r8, r31, 0x30
	ctx.r[8].s64 = ctx.r[31].s64 + 48;
	// 82F185EC: 937F000C  stw r27, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 82F185F0: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82F185F4: 80FD0000  lwz r7, 0(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F185F8: 90FF0030  stw r7, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82F185FC: 80DD0004  lwz r6, 4(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F18600: 90DF0034  stw r6, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[6].u32 ) };
	// 82F18604: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F18608: 907F0038  stw r3, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 82F1860C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F18610: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82F18614: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F18618: 4BF88119  bl 0x82ea0730
	ctx.lr = 0x82F1861C;
	sub_82EA0730(ctx, base);
	// 82F1861C: 57C900BE  clrlwi r9, r30, 2
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x3FFFFFFFu64;
	// 82F18620: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82F18624: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82F18628: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82F1862C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82F18630: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F18634: 4099001C  ble cr6, 0x82f18650
	if !ctx.cr[6].gt {
	pc = 0x82F18650; continue 'dispatch;
	}
	// 82F18638: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1863C: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F18640: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82F18644: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F18648: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82F1864C: 4082FFEC  bne 0x82f18638
	if !ctx.cr[0].eq {
	pc = 0x82F18638; continue 'dispatch;
	}
	// 82F18650: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82F18654: 40990034  ble cr6, 0x82f18688
	if !ctx.cr[6].gt {
	pc = 0x82F18688; continue 'dispatch;
	}
	// 82F18658: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 82F1865C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F18660: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F18664: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F18668: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F1866C: 419A0010  beq cr6, 0x82f1867c
	if ctx.cr[6].eq {
	pc = 0x82F1867C; continue 'dispatch;
	}
	// 82F18670: A12B0006  lhz r9, 6(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F18674: 39090001  addi r8, r9, 1
	ctx.r[8].s64 = ctx.r[9].s64 + 1;
	// 82F18678: B10B0006  sth r8, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F1867C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82F18680: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82F18684: 4082FFD8  bne 0x82f1865c
	if !ctx.cr[0].eq {
	pc = 0x82F1865C; continue 'dispatch;
	}
	// 82F18688: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82F1868C: 119F038C  vspltisw v12, -1
	for i in 0..4 {
		ctx.v[12].u32[i] = 4294967295;
	}
	// 82F18690: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82F18694: 392BFFE0  addi r9, r11, -0x20
	ctx.r[9].s64 = ctx.r[11].s64 + -32;
	// 82F18698: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82F1869C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F186A0: 3CC08338  lis r6, -0x7cc8
	ctx.r[6].s64 = -2093481984;
	// 82F186A4: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F18718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F18718 size=404
    let mut pc: u32 = 0x82F18718;
    'dispatch: loop {
        match pc {
            0x82F18718 => {
    //   block [0x82F18718..0x82F188AC)
	// 82F18718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1871C: 4828FA45  bl 0x831a8160
	ctx.lr = 0x82F18720;
	sub_831A8130(ctx, base);
	// 82F18720: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F18724: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F18728: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82F1872C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F18730: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82F18734: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82F18738: B37F0008  sth r27, 8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u16 ) };
	// 82F1873C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F18740: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82F18744: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82F18748: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82F1874C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F18750: B37F0002  sth r27, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[27].u16 ) };
	// 82F18754: B17F000A  sth r11, 0xa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 82F18758: 937F000C  stw r27, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 82F1875C: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82F18760: 4BF8ECF1  bl 0x82ea7450
	ctx.lr = 0x82F18764;
	sub_82EA7450(ctx, base);
	// 82F18764: 813E0030  lwz r9, 0x30(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82F18768: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1876C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F18770: 5784103A  slwi r4, r28, 2
	ctx.r[4].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82F18774: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 82F18778: 3BBF0030  addi r29, r31, 0x30
	ctx.r[29].s64 = ctx.r[31].s64 + 48;
	// 82F1877C: 913F0030  stw r9, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 82F18780: 811E0034  lwz r8, 0x34(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F18784: 911F0034  stw r8, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[8].u32 ) };
	// 82F18788: 80FE0038  lwz r7, 0x38(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82F1878C: 90FF0038  stw r7, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[7].u32 ) };
	// 82F18790: 80DE003C  lwz r6, 0x3c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82F18794: 90DF003C  stw r6, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[6].u32 ) };
	// 82F18798: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1879C: 4BF87F95  bl 0x82ea0730
	ctx.lr = 0x82F187A0;
	sub_82EA0730(ctx, base);
	// 82F187A0: 578900BE  clrlwi r9, r28, 2
	ctx.r[9].u64 = ctx.r[28].u32 as u64 & 0x3FFFFFFFu64;
	// 82F187A4: 939F0014  stw r28, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82F187A8: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82F187AC: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82F187B0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82F187B4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F187B8: 4099001C  ble cr6, 0x82f187d4
	if !ctx.cr[6].gt {
	pc = 0x82F187D4; continue 'dispatch;
	}
	// 82F187BC: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F187C0: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F187C4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82F187C8: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F187CC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82F187D0: 4082FFEC  bne 0x82f187bc
	if !ctx.cr[0].eq {
	pc = 0x82F187BC; continue 'dispatch;
	}
	// 82F187D4: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82F187D8: 40990030  ble cr6, 0x82f18808
	if !ctx.cr[6].gt {
	pc = 0x82F18808; continue 'dispatch;
	}
	// 82F187DC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F187E0: 7D6BD82E  lwzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82F187E4: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F187E8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F187EC: 419A0010  beq cr6, 0x82f187fc
	if ctx.cr[6].eq {
	pc = 0x82F187FC; continue 'dispatch;
	}
	// 82F187F0: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F187F4: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 82F187F8: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F187FC: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82F18800: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82F18804: 4082FFD8  bne 0x82f187dc
	if !ctx.cr[0].eq {
	pc = 0x82F187DC; continue 'dispatch;
	}
	// 82F18808: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F188B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F188B0 size=160
    let mut pc: u32 = 0x82F188B0;
    'dispatch: loop {
        match pc {
            0x82F188B0 => {
    //   block [0x82F188B0..0x82F18950)
	// 82F188B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F188B4: 4828F8B9  bl 0x831a816c
	ctx.lr = 0x82F188B8;
	sub_831A8130(ctx, base);
	// 82F188B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F188BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F188C0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F188C4: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F188C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F188CC: 4099005C  ble cr6, 0x82f18928
	if !ctx.cr[6].gt {
	pc = 0x82F18928; continue 'dispatch;
	}
	// 82F188D0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F188D4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F188D8: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F188DC: A1430004  lhz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F188E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F188E4: 419A0030  beq cr6, 0x82f18914
	if ctx.cr[6].eq {
	pc = 0x82F18914; continue 'dispatch;
	}
	// 82F188E8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F188EC: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F188F0: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F188F4: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F188F8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F188FC: 409A0018  bne cr6, 0x82f18914
	if !ctx.cr[6].eq {
	pc = 0x82F18914; continue 'dispatch;
	}
	// 82F18900: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F18904: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F18908: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1890C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F18910: 4E800421  bctrl
	ctx.lr = 0x82F18914;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F18914: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F18918: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F1891C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82F18920: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F18924: 4198FFB0  blt cr6, 0x82f188d4
	if ctx.cr[6].lt {
	pc = 0x82F188D4; continue 'dispatch;
	}
	// 82F18928: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1892C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F18930: 813E0014  lwz r9, 0x14(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F18934: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F18938: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F1893C: 5525103A  slwi r5, r9, 2
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F18940: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F18944: 4BF87E6D  bl 0x82ea07b0
	ctx.lr = 0x82F18948;
	sub_82EA07B0(ctx, base);
	// 82F18948: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1894C: 4828F870  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F18950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F18950 size=12
    let mut pc: u32 = 0x82F18950;
    'dispatch: loop {
        match pc {
            0x82F18950 => {
    //   block [0x82F18950..0x82F1895C)
	// 82F18950: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F18954: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82F18958: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1895C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1895C size=32
    let mut pc: u32 = 0x82F1895C;
    'dispatch: loop {
        match pc {
            0x82F1895C => {
    //   block [0x82F1895C..0x82F1897C)
	// 82F1895C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F18960: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F18964: 80830050  lwz r4, 0x50(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F18968: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1896C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F18970: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F18974: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F18978: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1897C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1897C size=4
    let mut pc: u32 = 0x82F1897C;
    'dispatch: loop {
        match pc {
            0x82F1897C => {
    //   block [0x82F1897C..0x82F18980)
	// 82F1897C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F18980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F18980 size=32
    let mut pc: u32 = 0x82F18980;
    'dispatch: loop {
        match pc {
            0x82F18980 => {
    //   block [0x82F18980..0x82F189A0)
	// 82F18980: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F18984: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F18988: 80830058  lwz r4, 0x58(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F1898C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F18990: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F18994: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F18998: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F1899C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F189A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F189A0 size=280
    let mut pc: u32 = 0x82F189A0;
    'dispatch: loop {
        match pc {
            0x82F189A0 => {
    //   block [0x82F189A0..0x82F18AB8)
	// 82F189A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F189A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F189A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F189AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F189B0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F189B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F189B8: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82F189BC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82F189C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F189C4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F189C8: 4800E0C9  bl 0x82f26a90
	ctx.lr = 0x82F189CC;
	sub_82F26A90(ctx, base);
	// 82F189CC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F189D0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F189D4: 395F0070  addi r10, r31, 0x70
	ctx.r[10].s64 = ctx.r[31].s64 + 112;
	// 82F189D8: 38E9BE40  addi r7, r9, -0x41c0
	ctx.r[7].s64 = ctx.r[9].s64 + -16832;
	// 82F189DC: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 82F189E0: 390BBE64  addi r8, r11, -0x419c
	ctx.r[8].s64 = ctx.r[11].s64 + -16796;
	// 82F189E4: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82F189E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F189EC: 3CC08000  lis r6, -0x8000
	ctx.r[6].s64 = -2147483648;
	// 82F189F0: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F189F4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82F189F8: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82F189FC: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82F18A00: 3CA08200  lis r5, -0x7e00
	ctx.r[5].s64 = -2113929216;
	// 82F18A04: 90DF0068  stw r6, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[6].u32 ) };
	// 82F18A08: 3C808212  lis r4, -0x7dee
	ctx.r[4].s64 = -2112749568;
	// 82F18A0C: 993F0071  stb r9, 0x71(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(113 as u32), ctx.r[9].u8 ) };
	// 82F18A10: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 82F18A14: B13F007A  sth r9, 0x7a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(122 as u32), ctx.r[9].u16 ) };
	// 82F18A18: 393F0020  addi r9, r31, 0x20
	ctx.r[9].s64 = ctx.r[31].s64 + 32;
	// 82F18A1C: 997F0070  stb r11, 0x70(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u8 ) };
	// 82F18A20: B17F0078  sth r11, 0x78(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u16 ) };
	// 82F18A24: C00508A8  lfs f0, 0x8a8(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F18A28: B17F0072  sth r11, 0x72(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(114 as u32), ctx.r[11].u16 ) };
	// 82F18A2C: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82F18A30: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82F18A34: 997F00A9  stb r11, 0xa9(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(169 as u32), ctx.r[11].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F18AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F18AB8 size=864
    let mut pc: u32 = 0x82F18AB8;
    'dispatch: loop {
        match pc {
            0x82F18AB8 => {
    //   block [0x82F18AB8..0x82F18E18)
	// 82F18AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F18ABC: 4828F6A1  bl 0x831a815c
	ctx.lr = 0x82F18AC0;
	sub_831A8130(ctx, base);
	// 82F18AC0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F18AC4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F18AC8: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82F18ACC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F18AD0: 4800DFC1  bl 0x82f26a90
	ctx.lr = 0x82F18AD4;
	sub_82F26A90(ctx, base);
	// 82F18AD4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F18AD8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F18ADC: 3BBF0070  addi r29, r31, 0x70
	ctx.r[29].s64 = ctx.r[31].s64 + 112;
	// 82F18AE0: 392BBE64  addi r9, r11, -0x419c
	ctx.r[9].s64 = ctx.r[11].s64 + -16796;
	// 82F18AE4: 397D0020  addi r11, r29, 0x20
	ctx.r[11].s64 = ctx.r[29].s64 + 32;
	// 82F18AE8: 390ABE40  addi r8, r10, -0x41c0
	ctx.r[8].s64 = ctx.r[10].s64 + -16832;
	// 82F18AEC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F18AF0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82F18AF4: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82F18AF8: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82F18AFC: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82F18B00: 939F0060  stw r28, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 82F18B04: 939F0064  stw r28, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82F18B08: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82F18B0C: 90FF0068  stw r7, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 82F18B10: 3B7F0060  addi r27, r31, 0x60
	ctx.r[27].s64 = ctx.r[31].s64 + 96;
	// 82F18B14: 9B9F0070  stb r28, 0x70(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[28].u8 ) };
	// 82F18B18: 3B5E0040  addi r26, r30, 0x40
	ctx.r[26].s64 = ctx.r[30].s64 + 64;
	// 82F18B1C: 9B3F0071  stb r25, 0x71(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(113 as u32), ctx.r[25].u8 ) };
	// 82F18B20: B39F0078  sth r28, 0x78(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[28].u16 ) };
	// 82F18B24: B39F0072  sth r28, 0x72(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(114 as u32), ctx.r[28].u16 ) };
	// 82F18B28: B33F007A  sth r25, 0x7a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(122 as u32), ctx.r[25].u16 ) };
	// 82F18B2C: 939F007C  stw r28, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[28].u32 ) };
	// 82F18B30: 939F0074  stw r28, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[28].u32 ) };
	// 82F18B34: 9B9F00A9  stb r28, 0xa9(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(169 as u32), ctx.r[28].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F18E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F18E18 size=384
    let mut pc: u32 = 0x82F18E18;
    'dispatch: loop {
        match pc {
            0x82F18E18 => {
    //   block [0x82F18E18..0x82F18F98)
	// 82F18E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F18E1C: 4828F345  bl 0x831a8160
	ctx.lr = 0x82F18E20;
	sub_831A8130(ctx, base);
	// 82F18E20: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F18E24: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F18E28: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F18E2C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F18E30: 390BBE64  addi r8, r11, -0x419c
	ctx.r[8].s64 = ctx.r[11].s64 + -16796;
	// 82F18E34: 38E9BE40  addi r7, r9, -0x41c0
	ctx.r[7].s64 = ctx.r[9].s64 + -16832;
	// 82F18E38: 815C005C  lwz r10, 0x5c(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F18E3C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82F18E40: 911C0000  stw r8, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F18E44: 90FC0010  stw r7, 0x10(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82F18E48: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F18E4C: 409900BC  ble cr6, 0x82f18f08
	if !ctx.cr[6].gt {
	pc = 0x82F18F08; continue 'dispatch;
	}
	// 82F18E50: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82F18E54: 817C0058  lwz r11, 0x58(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F18E58: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F18E5C: 7FEBDA14  add r31, r11, r27
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82F18E60: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F18E64: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F18E68: 4099005C  ble cr6, 0x82f18ec4
	if !ctx.cr[6].gt {
	pc = 0x82F18EC4; continue 'dispatch;
	}
	// 82F18E6C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F18E70: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F18E74: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F18E78: A1430004  lhz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F18E7C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F18E80: 419A0030  beq cr6, 0x82f18eb0
	if ctx.cr[6].eq {
	pc = 0x82F18EB0; continue 'dispatch;
	}
	// 82F18E84: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F18E88: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F18E8C: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F18E90: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F18E94: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F18E98: 409A0018  bne cr6, 0x82f18eb0
	if !ctx.cr[6].eq {
	pc = 0x82F18EB0; continue 'dispatch;
	}
	// 82F18E9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F18EA0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F18EA4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F18EA8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F18EAC: 4E800421  bctrl
	ctx.lr = 0x82F18EB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F18EB0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F18EB4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F18EB8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82F18EBC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F18EC0: 4198FFB0  blt cr6, 0x82f18e70
	if ctx.cr[6].lt {
	pc = 0x82F18E70; continue 'dispatch;
	}
	// 82F18EC4: A17C0004  lhz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F18EC8: 556A047E  clrlwi r10, r11, 0x11
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	// 82F18ECC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F18ED0: 419A0024  beq cr6, 0x82f18ef4
	if ctx.cr[6].eq {
	pc = 0x82F18EF4; continue 'dispatch;
	}
	// 82F18ED4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F18ED8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F18EDC: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F18EE0: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F18EE4: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F18EE8: 5525103A  slwi r5, r9, 2
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F18EEC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F18EF0: 4BF878C1  bl 0x82ea07b0
	ctx.lr = 0x82F18EF4;
	sub_82EA07B0(ctx, base);
	// 82F18EF4: 817C005C  lwz r11, 0x5c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F18EF8: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82F18EFC: 3B7B0040  addi r27, r27, 0x40
	ctx.r[27].s64 = ctx.r[27].s64 + 64;
	// 82F18F00: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F18F04: 4198FF50  blt cr6, 0x82f18e54
	if ctx.cr[6].lt {
	pc = 0x82F18E54; continue 'dispatch;
	}
	// 82F18F08: 817C0054  lwz r11, 0x54(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F18F0C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82F18F10: 40990024  ble cr6, 0x82f18f34
	if !ctx.cr[6].gt {
	pc = 0x82F18F34; continue 'dispatch;
	}
	// 82F18F14: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F18F18: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F18F1C: 809C0050  lwz r4, 0x50(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F18F20: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F18F24: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F18F28: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F18F2C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F18F30: 4E800421  bctrl
	ctx.lr = 0x82F18F34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F18F34: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F18F38: 3BC00014  li r30, 0x14
	ctx.r[30].s64 = 20;
	// 82F18F3C: 809C0058  lwz r4, 0x58(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F18F40: 7C7EF82E  lwzx r3, r30, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F18F44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F18F48: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F18F4C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F18F50: 4E800421  bctrl
	ctx.lr = 0x82F18F54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F18F54: 817C0068  lwz r11, 0x68(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(104 as u32) ) } as u64;
	// 82F18F58: 55690000  rlwinm r9, r11, 0, 0, 0
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F18F5C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F18F60: 409A0018  bne cr6, 0x82f18f78
	if !ctx.cr[6].eq {
	pc = 0x82F18F78; continue 'dispatch;
	}
	// 82F18F64: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F18F68: 809C0060  lwz r4, 0x60(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F18F6C: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82F18F70: 7C7EF82E  lwzx r3, r30, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F18F74: 4BF8783D  bl 0x82ea07b0
	ctx.lr = 0x82F18F78;
	sub_82EA07B0(ctx, base);
	// 82F18F78: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F18F7C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F18F80: 392BBCD8  addi r9, r11, -0x4328
	ctx.r[9].s64 = ctx.r[11].s64 + -17192;
	// 82F18F84: 390A9EAC  addi r8, r10, -0x6154
	ctx.r[8].s64 = ctx.r[10].s64 + -24916;
	// 82F18F88: 913C0010  stw r9, 0x10(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82F18F8C: 911C0000  stw r8, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F18F90: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82F18F94: 4828F21C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F18F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F18F98 size=420
    let mut pc: u32 = 0x82F18F98;
    'dispatch: loop {
        match pc {
            0x82F18F98 => {
    //   block [0x82F18F98..0x82F1913C)
	// 82F18F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F18F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F18FA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F18FA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F18FA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F18FAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F18FB0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F18FB4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F18FB8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F18FBC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F18FC0: 38EBBCD8  addi r7, r11, -0x4328
	ctx.r[7].s64 = ctx.r[11].s64 + -17192;
	// 82F18FC4: 38CABE64  addi r6, r10, -0x419c
	ctx.r[6].s64 = ctx.r[10].s64 + -16796;
	// 82F18FC8: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F18FCC: 38A9BE40  addi r5, r9, -0x41c0
	ctx.r[5].s64 = ctx.r[9].s64 + -16832;
	// 82F18FD0: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82F18FD4: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82F18FD8: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F18FDC: 90BF0010  stw r5, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82F18FE0: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82F18FE4: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82F18FE8: 419A0130  beq cr6, 0x82f19118
	if ctx.cr[6].eq {
	pc = 0x82F19118; continue 'dispatch;
	}
	// 82F18FEC: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F18FF0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F18FF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F18FF8: 40990034  ble cr6, 0x82f1902c
	if !ctx.cr[6].gt {
	pc = 0x82F1902C; continue 'dispatch;
	}
	// 82F18FFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F19000: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F19004: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F19008: 88EB0001  lbz r7, 1(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82F1900C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82F19010: 409A0008  bne cr6, 0x82f19018
	if !ctx.cr[6].eq {
	pc = 0x82F19018; continue 'dispatch;
	}
	// 82F19014: 990B0001  stb r8, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[8].u8 ) };
	// 82F19018: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F1901C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82F19020: 394A0040  addi r10, r10, 0x40
	ctx.r[10].s64 = ctx.r[10].s64 + 64;
	// 82F19024: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F19028: 4198FFD8  blt cr6, 0x82f19000
	if ctx.cr[6].lt {
	pc = 0x82F19000; continue 'dispatch;
	}
	// 82F1902C: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F19030: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F19034: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F19038: 40990034  ble cr6, 0x82f1906c
	if !ctx.cr[6].gt {
	pc = 0x82F1906C; continue 'dispatch;
	}
	// 82F1903C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F19040: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F19044: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F19048: 88EB0001  lbz r7, 1(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82F1904C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82F19050: 409A0008  bne cr6, 0x82f19058
	if !ctx.cr[6].eq {
	pc = 0x82F19058; continue 'dispatch;
	}
	// 82F19054: 990B0001  stb r8, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[8].u8 ) };
	// 82F19058: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F1905C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82F19060: 394A0040  addi r10, r10, 0x40
	ctx.r[10].s64 = ctx.r[10].s64 + 64;
	// 82F19064: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F19068: 4198FFD8  blt cr6, 0x82f19040
	if ctx.cr[6].lt {
	pc = 0x82F19040; continue 'dispatch;
	}
	// 82F1906C: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F19070: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82F19074: 409A0010  bne cr6, 0x82f19084
	if !ctx.cr[6].eq {
	pc = 0x82F19084; continue 'dispatch;
	}
	// 82F19078: 3BDF0070  addi r30, r31, 0x70
	ctx.r[30].s64 = ctx.r[31].s64 + 112;
	// 82F1907C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82F19080: 4800003C  b 0x82f190bc
	pc = 0x82F190BC; continue 'dispatch;
	// 82F19084: 40990048  ble cr6, 0x82f190cc
	if !ctx.cr[6].gt {
	pc = 0x82F190CC; continue 'dispatch;
	}
	// 82F19088: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1908C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F19090: 38C00025  li r6, 0x25
	ctx.r[6].s64 = 37;
	// 82F19094: 55653032  slwi r5, r11, 6
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F19098: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82F1909C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F190A0: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F190A4: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F190A8: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F190AC: 4E800421  bctrl
	ctx.lr = 0x82F190B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F190B0: 80DF0054  lwz r6, 0x54(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F190B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F190B8: 54C53032  slwi r5, r6, 6
	ctx.r[5].u32 = ctx.r[6].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F190BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F190C0: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F190C4: 4BF91685  bl 0x82eaa748
	ctx.lr = 0x82F190C8;
	sub_82EAA748(ctx, base);
	// 82F190C8: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82F190CC: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F190D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F190D4: 40990044  ble cr6, 0x82f19118
	if !ctx.cr[6].gt {
	pc = 0x82F19118; continue 'dispatch;
	}
	// 82F190D8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F190DC: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F190E0: 38C00025  li r6, 0x25
	ctx.r[6].s64 = 37;
	// 82F190E4: 55653032  slwi r5, r11, 6
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F190E8: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82F190EC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F190F0: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F190F4: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F190F8: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F190FC: 4E800421  bctrl
	ctx.lr = 0x82F19100;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F19100: 80DF005C  lwz r6, 0x5c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F19104: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F19108: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F1910C: 54C53032  slwi r5, r6, 6
	ctx.r[5].u32 = ctx.r[6].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F19110: 4BF91639  bl 0x82eaa748
	ctx.lr = 0x82F19114;
	sub_82EAA748(ctx, base);
	// 82F19114: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82F19118: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 82F1911C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F19120: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82F19124: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F19128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1912C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F19130: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F19134: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F19138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F19140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F19140 size=360
    let mut pc: u32 = 0x82F19140;
    'dispatch: loop {
        match pc {
            0x82F19140 => {
    //   block [0x82F19140..0x82F192A8)
	// 82F19140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F19144: 4828F009  bl 0x831a814c
	ctx.lr = 0x82F19148;
	sub_831A8130(ctx, base);
	// 82F19148: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1914C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F19150: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82F19154: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 82F19158: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F1915C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F19160: 817D0054  lwz r11, 0x54(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F19164: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F19168: 40990038  ble cr6, 0x82f191a0
	if !ctx.cr[6].gt {
	pc = 0x82F191A0; continue 'dispatch;
	}
	// 82F1916C: 815D0050  lwz r10, 0x50(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F19170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F19174: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F19178: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82F1917C: 93EA003C  stw r31, 0x3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(60 as u32), ctx.r[31].u32 ) };
	// 82F19180: 80FD0054  lwz r7, 0x54(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F19184: 815D0050  lwz r10, 0x50(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F19188: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F1918C: 81060010  lwz r8, 0x10(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F19190: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82F19194: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 82F19198: 7FE8FA14  add r31, r8, r31
	ctx.r[31].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 82F1919C: 4198FFD8  blt cr6, 0x82f19174
	if ctx.cr[6].lt {
	pc = 0x82F19174; continue 'dispatch;
	}
	// 82F191A0: 817D0068  lwz r11, 0x68(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 82F191A4: 3AFD0060  addi r23, r29, 0x60
	ctx.r[23].s64 = ctx.r[29].s64 + 96;
	// 82F191A8: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F191AC: 7F0AF800  cmpw cr6, r10, r31
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82F191B0: 40980014  bge cr6, 0x82f191c4
	if !ctx.cr[6].lt {
	pc = 0x82F191C4; continue 'dispatch;
	}
	// 82F191B4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82F191B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F191BC: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82F191C0: 4BF8D639  bl 0x82ea67f8
	ctx.lr = 0x82F191C4;
	sub_82EA67F8(ctx, base);
	// 82F191C4: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F191C8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F191CC: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82F191D0: 40980024  bge cr6, 0x82f191f4
	if !ctx.cr[6].lt {
	pc = 0x82F191F4; continue 'dispatch;
	}
	// 82F191D4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F191D8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F191DC: 41980008  blt cr6, 0x82f191e4
	if ctx.cr[6].lt {
	pc = 0x82F191E4; continue 'dispatch;
	}
	// 82F191E0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82F191E4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82F191E8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F191EC: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82F191F0: 4BF8D609  bl 0x82ea67f8
	ctx.lr = 0x82F191F4;
	sub_82EA67F8(ctx, base);
	// 82F191F4: 93F70004  stw r31, 4(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82F191F8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82F191FC: 817D0054  lwz r11, 0x54(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F19200: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82F19204: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F19208: 4099008C  ble cr6, 0x82f19294
	if !ctx.cr[6].gt {
	pc = 0x82F19294; continue 'dispatch;
	}
	// 82F1920C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82F19210: 817D0050  lwz r11, 0x50(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F19214: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F19218: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 82F1921C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F19220: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F19224: 4099005C  ble cr6, 0x82f19280
	if !ctx.cr[6].gt {
	pc = 0x82F19280; continue 'dispatch;
	}
	// 82F19228: 3975FFFB  addi r11, r21, -5
	ctx.r[11].s64 = ctx.r[21].s64 + -5;
	// 82F1922C: 577E083C  slwi r30, r27, 1
	ctx.r[30].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82F19230: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82F19234: 5549DFFE  rlwinm r9, r10, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82F19238: 69390001  xori r25, r9, 1
	ctx.r[25].u64 = ctx.r[9].u64 ^ 1;
	// 82F1923C: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F19240: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82F19244: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82F19248: 83970000  lwz r28, 0(r23)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1924C: 214B0020  subfic r10, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[10].s64 = (32 as i64) - ctx.r[11].s64;
	// 82F19250: 7F095030  slw r9, r24, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[24].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82F19254: 7D23FB78  or r3, r9, r31
	ctx.r[3].u64 = ctx.r[9].u64 | ctx.r[31].u64;
	// 82F19258: 4800E4C1  bl 0x82f27718
	ctx.lr = 0x82F1925C;
	sub_82F27718(ctx, base);
	// 82F1925C: 7C7CF32E  sthx r3, r28, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32), ctx.r[3].u16) };
	// 82F19260: 817D0050  lwz r11, 0x50(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F19264: 7D1A5A14  add r8, r26, r11
	ctx.r[8].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 82F19268: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82F1926C: 80E80010  lwz r7, 0x10(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F19270: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82F19274: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82F19278: 7F1F3800  cmpw cr6, r31, r7
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82F1927C: 4198FFC0  blt cr6, 0x82f1923c
	if ctx.cr[6].lt {
	pc = 0x82F1923C; continue 'dispatch;
	}
	// 82F19280: 817D0054  lwz r11, 0x54(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F19284: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 82F19288: 3B5A0040  addi r26, r26, 0x40
	ctx.r[26].s64 = ctx.r[26].s64 + 64;
	// 82F1928C: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F19290: 4198FF80  blt cr6, 0x82f19210
	if ctx.cr[6].lt {
	pc = 0x82F19210; continue 'dispatch;
	}
	// 82F19294: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F19298: 997D0014  stb r11, 0x14(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82F1929C: 9ABD006C  stb r21, 0x6c(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(108 as u32), ctx.r[21].u8 ) };
	// 82F192A0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82F192A4: 4828EEF8  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F192A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F192A8 size=200
    let mut pc: u32 = 0x82F192A8;
    'dispatch: loop {
        match pc {
            0x82F192A8 => {
    //   block [0x82F192A8..0x82F19370)
	// 82F192A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F192AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F192B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F192B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F192B8: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F192BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F192C0: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F192C4: 815F004C  lwz r10, 0x4c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82F192C8: 7D4A5A15  add. r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F192CC: 4082000C  bne 0x82f192d8
	if !ctx.cr[0].eq {
	pc = 0x82F192D8; continue 'dispatch;
	}
	// 82F192D0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82F192D4: 48000084  b 0x82f19358
	pc = 0x82F19358; continue 'dispatch;
	// 82F192D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F192DC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F192E0: 409A0008  bne cr6, 0x82f192e8
	if !ctx.cr[6].eq {
	pc = 0x82F192E8; continue 'dispatch;
	}
	// 82F192E4: 3FC08000  lis r30, -0x8000
	ctx.r[30].s64 = -2147483648;
	// 82F192E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F192EC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F192F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F192F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F192F8: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F192FC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F19300: 4E800421  bctrl
	ctx.lr = 0x82F19304;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F19304: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F19308: 2F090004  cmpwi cr6, r9, 4
	ctx.cr[6].compare_i32(ctx.r[9].s32, 4, &mut ctx.xer);
	// 82F1930C: 419A000C  beq cr6, 0x82f19318
	if ctx.cr[6].eq {
	pc = 0x82F19318; continue 'dispatch;
	}
	// 82F19310: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F19314: 48000044  b 0x82f19358
	pc = 0x82F19358; continue 'dispatch;
	// 82F19318: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82F1931C: 38C30040  addi r6, r3, 0x40
	ctx.r[6].s64 = ctx.r[3].s64 + 64;
	// 82F19320: 38A30030  addi r5, r3, 0x30
	ctx.r[5].s64 = ctx.r[3].s64 + 48;
	// 82F19324: 38830020  addi r4, r3, 0x20
	ctx.r[4].s64 = ctx.r[3].s64 + 32;
	// 82F19328: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F1932C: C02B0450  lfs f1, 0x450(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1104 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F19330: 4800E739  bl 0x82f27a68
	ctx.lr = 0x82F19334;
	sub_82F27A68(ctx, base);
	// 82F19334: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F19338: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F1933C: 419AFFD4  beq cr6, 0x82f19310
	if ctx.cr[6].eq {
	pc = 0x82F19310; continue 'dispatch;
	}
	// 82F19340: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F19344: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F19348: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1934C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F19350: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F19354: 4E800421  bctrl
	ctx.lr = 0x82F19358;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F19358: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 82F1935C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F19360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F19364: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F19368: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1936C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F19370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F19370 size=300
    let mut pc: u32 = 0x82F19370;
    'dispatch: loop {
        match pc {
            0x82F19370 => {
    //   block [0x82F19370..0x82F1949C)
	// 82F19370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F19374: 4828EDE9  bl 0x831a815c
	ctx.lr = 0x82F19378;
	sub_831A8130(ctx, base);
	// 82F19378: 9421FD60  stwu r1, -0x2a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-672 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1937C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F19380: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 82F19384: 548A007E  clrlwi r10, r4, 1
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x7FFFFFFFu64;
	// 82F19388: 549B0000  rlwinm r27, r4, 0, 0, 0
	ctx.r[27].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1938C: 3F408332  lis r26, -0x7cce
	ctx.r[26].s64 = -2093875200;
	// 82F19390: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F19394: 212B0020  subfic r9, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[9].s64 = (32 as i64) - ctx.r[11].s64;
	// 82F19398: 7F285C30  srw r8, r25, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[25].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82F1939C: 7D5E4C30  srw r30, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82F193A0: 7D1F2038  and r31, r8, r4
	ctx.r[31].u64 = ctx.r[8].u64 & ctx.r[4].u64;
	// 82F193A4: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82F193A8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82F193AC: 409A004C  bne cr6, 0x82f193f8
	if !ctx.cr[6].eq {
	pc = 0x82F193F8; continue 'dispatch;
	}
	// 82F193B0: 817D0040  lwz r11, 0x40(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F193B4: 57CA3032  slwi r10, r30, 6
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F193B8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F193BC: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F193C0: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82F193C4: 41980060  blt cr6, 0x82f19424
	if ctx.cr[6].lt {
	pc = 0x82F19424; continue 'dispatch;
	}
	// 82F193C8: 817D0044  lwz r11, 0x44(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F193CC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82F193D0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F193D4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82F193D8: 4198004C  blt cr6, 0x82f19424
	if ctx.cr[6].lt {
	pc = 0x82F19424; continue 'dispatch;
	}
	// 82F193DC: 817D004C  lwz r11, 0x4c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(76 as u32) ) } as u64;
	// 82F193E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F193E4: 419A00AC  beq cr6, 0x82f19490
	if ctx.cr[6].eq {
	pc = 0x82F19490; continue 'dispatch;
	}
	// 82F193E8: 3F608000  lis r27, -0x8000
	ctx.r[27].s64 = -2147483648;
	// 82F193EC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F193F0: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 82F193F4: 4BFFFFB0  b 0x82f193a4
	pc = 0x82F193A4; continue 'dispatch;
	// 82F193F8: 815D0048  lwz r10, 0x48(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F193FC: 57CB3032  slwi r11, r30, 6
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F19400: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F19404: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F19408: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82F1940C: 41980018  blt cr6, 0x82f19424
	if ctx.cr[6].lt {
	pc = 0x82F19424; continue 'dispatch;
	}
	// 82F19410: 817D004C  lwz r11, 0x4c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(76 as u32) ) } as u64;
	// 82F19414: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82F19418: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82F1941C: 40980074  bge cr6, 0x82f19490
	if !ctx.cr[6].lt {
	pc = 0x82F19490; continue 'dispatch;
	}
	// 82F19420: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F19424: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F19428: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F1942C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F19430: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F19434: 212B0020  subfic r9, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[9].s64 = (32 as i64) - ctx.r[11].s64;
	// 82F19438: 7FC84830  slw r8, r30, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[30].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82F1943C: 7D07DB78  or r7, r8, r27
	ctx.r[7].u64 = ctx.r[8].u64 | ctx.r[27].u64;
	// 82F19440: 80CA0014  lwz r6, 0x14(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F19444: 7CFCFB78  or r28, r7, r31
	ctx.r[28].u64 = ctx.r[7].u64 | ctx.r[31].u64;
	// 82F19448: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F1944C: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82F19450: 4E800421  bctrl
	ctx.lr = 0x82F19454;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F19454: 80A3000C  lwz r5, 0xc(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F19458: 2F050004  cmpwi cr6, r5, 4
	ctx.cr[6].compare_i32(ctx.r[5].s32, 4, &mut ctx.xer);
	// 82F1945C: 409A0028  bne cr6, 0x82f19484
	if !ctx.cr[6].eq {
	pc = 0x82F19484; continue 'dispatch;
	}
	// 82F19460: 38C30040  addi r6, r3, 0x40
	ctx.r[6].s64 = ctx.r[3].s64 + 64;
	// 82F19464: C03A0450  lfs f1, 0x450(r26)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(1104 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F19468: 38A30030  addi r5, r3, 0x30
	ctx.r[5].s64 = ctx.r[3].s64 + 48;
	// 82F1946C: 38830020  addi r4, r3, 0x20
	ctx.r[4].s64 = ctx.r[3].s64 + 32;
	// 82F19470: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F19474: 4800E5F5  bl 0x82f27a68
	ctx.lr = 0x82F19478;
	sub_82F27A68(ctx, base);
	// 82F19478: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1947C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F19480: 409AFF24  bne cr6, 0x82f193a4
	if !ctx.cr[6].eq {
	pc = 0x82F193A4; continue 'dispatch;
	}
	// 82F19484: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F19488: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 82F1948C: 4828ED20  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 82F19490: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F19494: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 82F19498: 4828ED14  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F194A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F194A0 size=1040
    let mut pc: u32 = 0x82F194A0;
    'dispatch: loop {
        match pc {
            0x82F194A0 => {
    //   block [0x82F194A0..0x82F198B0)
	// 82F194A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F194A4: 4828ECB9  bl 0x831a815c
	ctx.lr = 0x82F194A8;
	sub_831A8130(ctx, base);
	// 82F194A8: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F194AC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F194B0: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82F194B4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82F194B8: 7D455C30  srw r5, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[5].u64 = 0;
	} else {
		ctx.r[5].u64 = ((ctx.r[10].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82F194BC: 5489007E  clrlwi r9, r4, 1
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x7FFFFFFFu64;
	// 82F194C0: 54870000  rlwinm r7, r4, 0, 0, 0
	ctx.r[7].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82F194C4: 20CB0020  subfic r6, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[6].s64 = (32 as i64) - ctx.r[11].s64;
	// 82F194C8: 3903FFF0  addi r8, r3, -0x10
	ctx.r[8].s64 = ctx.r[3].s64 + -16;
	// 82F194CC: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82F194D0: 7D2B3430  srw r11, r9, r6
	if (ctx.r[6].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) >> ((ctx.r[6].u8 & 0x1F) as u32)) as u64;
	}
	// 82F194D4: 7CBC2038  and r28, r5, r4
	ctx.r[28].u64 = ctx.r[5].u64 & ctx.r[4].u64;
	// 82F194D8: 409A0284  bne cr6, 0x82f1975c
	if !ctx.cr[6].eq {
	pc = 0x82F1975C; continue 'dispatch;
	}
	// 82F194DC: 81480054  lwz r10, 0x54(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F194E0: 392100CF  addi r9, r1, 0xcf
	ctx.r[9].s64 = ctx.r[1].s64 + 207;
	// 82F194E4: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 82F194E8: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82F194EC: 55270032  rlwinm r7, r9, 0, 0, 0x19
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82F194F0: 409A000C  bne cr6, 0x82f194fc
	if !ctx.cr[6].eq {
	pc = 0x82F194FC; continue 'dispatch;
	}
	// 82F194F4: 39280070  addi r9, r8, 0x70
	ctx.r[9].s64 = ctx.r[8].s64 + 112;
	// 82F194F8: 4800005C  b 0x82f19554
	pc = 0x82F19554; continue 'dispatch;
	// 82F194FC: 81480050  lwz r10, 0x50(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F19500: 55693032  slwi r9, r11, 6
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F19504: 55663032  slwi r6, r11, 6
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82F19508: 7D695214  add r11, r9, r10
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82F1950C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82F19510: 7CA65850  subf r5, r6, r11
	ctx.r[5].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 82F19514: 7D4A2850  subf r10, r10, r5
	ctx.r[10].s64 = ctx.r[5].s64 - ctx.r[10].s64;
	// 82F19518: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82F1951C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F19520: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F19524: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F19528: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1952C: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F19530: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F19534: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82F19538: 90CAFFF8  stw r6, -8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), ctx.r[6].u32 ) };
	// 82F1953C: 90AAFFFC  stw r5, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[5].u32 ) };
	// 82F19540: 908A0000  stw r4, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82F19544: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F19548: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82F1954C: 4181FFD4  bgt 0x82f19520
	if ctx.cr[0].gt {
	pc = 0x82F19520; continue 'dispatch;
	}
	// 82F19550: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82F19554: 89490039  lbz r10, 0x39(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(57 as u32) ) } as u64;
	// 82F19558: 80E90034  lwz r7, 0x34(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F1955C: 7D460774  extsb r6, r10
	ctx.r[6].s64 = ctx.r[10].s8 as i64;
	// 82F19560: 81690030  lwz r11, 0x30(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(48 as u32) ) } as u64;
	// 82F19564: 7CE7E1D6  mullw r7, r7, r28
	ctx.r[7].s64 = (ctx.r[7].s32 as i64) * (ctx.r[28].s32 as i64);
	// 82F19568: 88A90038  lbz r5, 0x38(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(56 as u32) ) } as u64;
	// 82F1956C: 7CCAE038  and r10, r6, r28
	ctx.r[10].u64 = ctx.r[6].u64 & ctx.r[28].u64;
	// 82F19570: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82F19574: 69470001  xori r7, r10, 1
	ctx.r[7].u64 = ctx.r[10].u64 ^ 1;
	// 82F19578: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82F1957C: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82F19580: 7D430734  extsh r3, r10
	ctx.r[3].s64 = ctx.r[10].s16 as i64;
	// 82F19584: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 82F19588: 7CEA0734  extsh r10, r7
	ctx.r[10].s64 = ctx.r[7].s16 as i64;
	// 82F1958C: 409A001C  bne cr6, 0x82f195a8
	if !ctx.cr[6].eq {
	pc = 0x82F195A8; continue 'dispatch;
	}
	// 82F19590: 5546083C  slwi r6, r10, 1
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82F19594: A08B0000  lhz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F19598: 5467083C  slwi r7, r3, 1
	ctx.r[7].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1959C: 7FC65A2E  lhzx r30, r6, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F195A0: 7D475A2E  lhzx r10, r7, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F195A4: 48000018  b 0x82f195bc
	pc = 0x82F195BC; continue 'dispatch;
	// 82F195A8: 5546103A  slwi r6, r10, 2
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82F195AC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F195B0: 5467103A  slwi r7, r3, 2
	ctx.r[7].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F195B4: 7FC6582E  lwzx r30, r6, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F195B8: 7D47582E  lwzx r10, r7, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F195BC: 80C90014  lwz r6, 0x14(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F195C0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F195C4: 80A90018  lwz r5, 0x18(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F195C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82F195CC: 419A0054  beq cr6, 0x82f19620
	if ctx.cr[6].eq {
	pc = 0x82F19620; continue 'dispatch;
	}
	// 82F195D0: 3C608332  lis r3, -0x7cce
	ctx.r[3].s64 = -2093875200;
	// 82F195D4: 3F608204  lis r27, -0x7dfc
	ctx.r[27].s64 = -2113667072;
	// 82F195D8: 397F0050  addi r11, r31, 0x50
	ctx.r[11].s64 = ctx.r[31].s64 + 80;
	// 82F195DC: 3B7B6C64  addi r27, r27, 0x6c64
	ctx.r[27].s64 = ctx.r[27].s64 + 27748;
	// 82F195E0: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82F195E4: 3B200006  li r25, 6
	ctx.r[25].s64 = 6;
	// 82F195E8: C003F614  lfs f0, -0x9ec(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-2540 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F195EC: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F195F0: B35F0006  sth r26, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[26].u16 ) };
	// 82F195F4: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82F195F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F195FC: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82F19600: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82F19604: B0FF0014  sth r7, 0x14(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u16 ) };
	// 82F19608: 9B3F0016  stb r25, 0x16(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(22 as u32), ctx.r[25].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F198B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F198B0 size=24
    let mut pc: u32 = 0x82F198B0;
    'dispatch: loop {
        match pc {
            0x82F198B0 => {
    //   block [0x82F198B0..0x82F198C8)
	// 82F198B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82F198B4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F198B8: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82F198BC: 38AB0040  addi r5, r11, 0x40
	ctx.r[5].s64 = ctx.r[11].s64 + 64;
	// 82F198C0: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82F198C4: 48000514  b 0x82f19dd8
	sub_82F19DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F198C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F198C8 size=188
    let mut pc: u32 = 0x82F198C8;
    'dispatch: loop {
        match pc {
            0x82F198C8 => {
    //   block [0x82F198C8..0x82F19984)
	// 82F198C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F198CC: 4828E89D  bl 0x831a8168
	ctx.lr = 0x82F198D0;
	sub_831A8130(ctx, base);
	// 82F198D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F198D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F198D8: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F198DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F198E0: 409A001C  bne cr6, 0x82f198fc
	if !ctx.cr[6].eq {
	pc = 0x82F198FC; continue 'dispatch;
	}
	// 82F198E4: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82F198E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F198EC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82F198F0: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82F198F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F198F8: 4828E8C0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82F198FC: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F19900: 3B800014  li r28, 0x14
	ctx.r[28].s64 = 20;
	// 82F19904: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F19908: 38C00025  li r6, 0x25
	ctx.r[6].s64 = 37;
	// 82F1990C: 55653032  slwi r5, r11, 6
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F19910: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82F19914: 7C7CE82E  lwzx r3, r28, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82F19918: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1991C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F19920: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F19924: 4E800421  bctrl
	ctx.lr = 0x82F19928;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F19928: 813F0054  lwz r9, 0x54(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F1992C: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F19930: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F19934: 55253032  slwi r5, r9, 6
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F19938: 4BF90E11  bl 0x82eaa748
	ctx.lr = 0x82F1993C;
	sub_82EAA748(ctx, base);
	// 82F1993C: 811F0054  lwz r8, 0x54(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F19940: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 82F19944: 4099001C  ble cr6, 0x82f19960
	if !ctx.cr[6].gt {
	pc = 0x82F19960; continue 'dispatch;
	}
	// 82F19948: 7C7CE82E  lwzx r3, r28, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82F1994C: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F19950: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F19954: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F19958: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1995C: 4E800421  bctrl
	ctx.lr = 0x82F19960;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F19960: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F19964: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82F19968: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F1996C: 556A3032  slwi r10, r11, 6
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F19970: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82F19974: 7D6AF214  add r11, r10, r30
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82F19978: 386BFFC0  addi r3, r11, -0x40
	ctx.r[3].s64 = ctx.r[11].s64 + -64;
	// 82F1997C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F19980: 4828E838  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F19988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F19988 size=144
    let mut pc: u32 = 0x82F19988;
    'dispatch: loop {
        match pc {
            0x82F19988 => {
    //   block [0x82F19988..0x82F19A18)
	// 82F19988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1998C: 4828E7DD  bl 0x831a8168
	ctx.lr = 0x82F19990;
	sub_831A8130(ctx, base);
	// 82F19990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F19994: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F19998: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1999C: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 82F199A0: 38C00025  li r6, 0x25
	ctx.r[6].s64 = 37;
	// 82F199A4: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82F199A8: 817C005C  lwz r11, 0x5c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F199AC: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F199B0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F199B4: 55653032  slwi r5, r11, 6
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F199B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F199BC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F199C0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F199C4: 4E800421  bctrl
	ctx.lr = 0x82F199C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F199C8: 813C005C  lwz r9, 0x5c(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F199CC: 809C0058  lwz r4, 0x58(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F199D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F199D4: 55253032  slwi r5, r9, 6
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F199D8: 4BF90D71  bl 0x82eaa748
	ctx.lr = 0x82F199DC;
	sub_82EAA748(ctx, base);
	// 82F199DC: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F199E0: 809C0058  lwz r4, 0x58(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F199E4: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F199E8: 80E80004  lwz r7, 4(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F199EC: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F199F0: 4E800421  bctrl
	ctx.lr = 0x82F199F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F199F4: 817C005C  lwz r11, 0x5c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F199F8: 93FC0058  stw r31, 0x58(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82F199FC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F19A00: 556A3032  slwi r10, r11, 6
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F19A04: 917C005C  stw r11, 0x5c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82F19A08: 7D6AFA14  add r11, r10, r31
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82F19A0C: 386BFFC0  addi r3, r11, -0x40
	ctx.r[3].s64 = ctx.r[11].s64 + -64;
	// 82F19A10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F19A14: 4828E7A4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F19A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F19A18 size=348
    let mut pc: u32 = 0x82F19A18;
    'dispatch: loop {
        match pc {
            0x82F19A18 => {
    //   block [0x82F19A18..0x82F19B74)
	// 82F19A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F19A1C: 4828E74D  bl 0x831a8168
	ctx.lr = 0x82F19A20;
	sub_831A8130(ctx, base);
	// 82F19A20: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F19B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F19B78 size=448
    let mut pc: u32 = 0x82F19B78;
    'dispatch: loop {
        match pc {
            0x82F19B78 => {
    //   block [0x82F19B78..0x82F19D38)
	// 82F19B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F19B7C: 4828E5F1  bl 0x831a816c
	ctx.lr = 0x82F19B80;
	sub_831A8130(ctx, base);
	// 82F19B80: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F19D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F19D38 size=20
    let mut pc: u32 = 0x82F19D38;
    'dispatch: loop {
        match pc {
            0x82F19D38 => {
    //   block [0x82F19D38..0x82F19D4C)
	// 82F19D38: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F19D3C: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82F19D40: 7D490034  cntlzw r9, r10
	ctx.r[9].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82F19D44: 5523DFFE  rlwinm r3, r9, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82F19D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F19D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F19D50 size=8
    let mut pc: u32 = 0x82F19D50;
    'dispatch: loop {
        match pc {
            0x82F19D50 => {
    //   block [0x82F19D50..0x82F19D58)
	// 82F19D50: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82F19D54: 48000004  b 0x82f19d58
	sub_82F19D58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F19D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F19D58 size=124
    let mut pc: u32 = 0x82F19D58;
    'dispatch: loop {
        match pc {
            0x82F19D58 => {
    //   block [0x82F19D58..0x82F19DD4)
	// 82F19D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F19D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F19D60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F19D64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F19D68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F19D6C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F19D70: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82F19D74: 409A0008  bne cr6, 0x82f19d7c
	if !ctx.cr[6].eq {
	pc = 0x82F19D7C; continue 'dispatch;
	}
	// 82F19D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F19D7C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F19D80: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F19D84: 390ABCD8  addi r8, r10, -0x4328
	ctx.r[8].s64 = ctx.r[10].s64 + -17192;
	// 82F19D88: 38E99EAC  addi r7, r9, -0x6154
	ctx.r[7].s64 = ctx.r[9].s64 + -24916;
	// 82F19D8C: 548607FE  clrlwi r6, r4, 0x1f
	ctx.r[6].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82F19D90: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F19D94: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F19D98: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F19D9C: 419A0020  beq cr6, 0x82f19dbc
	if ctx.cr[6].eq {
	pc = 0x82F19DBC; continue 'dispatch;
	}
	// 82F19DA0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F19DA4: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F19DA8: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F19DAC: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F19DB0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F19DB4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F19DB8: 4BF869F9  bl 0x82ea07b0
	ctx.lr = 0x82F19DBC;
	sub_82EA07B0(ctx, base);
	// 82F19DBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F19DC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F19DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F19DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F19DCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F19DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F19DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F19DD8 size=176
    let mut pc: u32 = 0x82F19DD8;
    'dispatch: loop {
        match pc {
            0x82F19DD8 => {
    //   block [0x82F19DD8..0x82F19E88)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F19E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F19E88 size=8
    let mut pc: u32 = 0x82F19E88;
    'dispatch: loop {
        match pc {
            0x82F19E88 => {
    //   block [0x82F19E88..0x82F19E90)
	// 82F19E88: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82F19E8C: 48000004  b 0x82f19e90
	sub_82F19E90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F19E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F19E90 size=100
    let mut pc: u32 = 0x82F19E90;
    'dispatch: loop {
        match pc {
            0x82F19E90 => {
    //   block [0x82F19E90..0x82F19EF4)
	// 82F19E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F19E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F19E98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F19E9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F19EA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F19EA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F19EA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F19EAC: 4BFFEF6D  bl 0x82f18e18
	ctx.lr = 0x82F19EB0;
	sub_82F18E18(ctx, base);
	// 82F19EB0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F19EB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F19EB8: 419A0020  beq cr6, 0x82f19ed8
	if ctx.cr[6].eq {
	pc = 0x82F19ED8; continue 'dispatch;
	}
	// 82F19EBC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F19EC0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F19EC4: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F19EC8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F19ECC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F19ED0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F19ED4: 4BF868DD  bl 0x82ea07b0
	ctx.lr = 0x82F19ED8;
	sub_82EA07B0(ctx, base);
	// 82F19ED8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F19EDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F19EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F19EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F19EE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F19EEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F19EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F19EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F19EF8 size=8
    let mut pc: u32 = 0x82F19EF8;
    'dispatch: loop {
        match pc {
            0x82F19EF8 => {
    //   block [0x82F19EF8..0x82F19F00)
	// 82F19EF8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82F19EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F19F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F19F00 size=108
    let mut pc: u32 = 0x82F19F00;
    'dispatch: loop {
        match pc {
            0x82F19F00 => {
    //   block [0x82F19F00..0x82F19F6C)
	// 82F19F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F19F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F19F08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F19F0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F19F10: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F19F14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F19F18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F19F1C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F19F20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F19F24: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82F19F28: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F19F2C: 4E800421  bctrl
	ctx.lr = 0x82F19F30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F19F30: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F19F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F19F70 size=128
    let mut pc: u32 = 0x82F19F70;
    'dispatch: loop {
        match pc {
            0x82F19F70 => {
    //   block [0x82F19F70..0x82F19FF0)
	// 82F19F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F19F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F19F78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F19F7C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F19F80: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F19F84: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F19F88: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82F19F8C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F19F90: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82F19F94: 388969F0  addi r4, r9, 0x69f0
	ctx.r[4].s64 = ctx.r[9].s64 + 27120;
	// 82F19F98: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F19F9C: 810A001C  lwz r8, 0x1c(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F19FA0: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F19FA4: 4E800421  bctrl
	ctx.lr = 0x82F19FA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F19FA8: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82F19FAC: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82F19FB0: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82F19FB4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F19FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F19FF0 size=220
    let mut pc: u32 = 0x82F19FF0;
    'dispatch: loop {
        match pc {
            0x82F19FF0 => {
    //   block [0x82F19FF0..0x82F1A0CC)
	// 82F19FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F19FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F19FF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F19FFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1A000: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1A004: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F1A008: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1A00C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F1A010: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82F1A014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F1A018: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82F1A01C: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 82F1A020: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F1A024: C01F0004  lfs f0, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1A028: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F1A02C: 80EB0020  lwz r7, 0x20(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F1A030: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82F1A034: 912100A0  stw r9, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[9].u32 ) };
	// 82F1A038: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82F1A03C: 91010080  stw r8, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[8].u32 ) };
	// 82F1A040: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F1A044: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F1A048: 4E800421  bctrl
	ctx.lr = 0x82F1A04C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F1A04C: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F1A050: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F1A054: 419A0060  beq cr6, 0x82f1a0b4
	if ctx.cr[6].eq {
	pc = 0x82F1A0B4; continue 'dispatch;
	}
	// 82F1A058: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F1A05C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F1A060: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82F1A064: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1A068: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82F1A06C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82F1A070: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1A0D0 size=96
    let mut pc: u32 = 0x82F1A0D0;
    'dispatch: loop {
        match pc {
            0x82F1A0D0 => {
    //   block [0x82F1A0D0..0x82F1A130)
	// 82F1A0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1A0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1A0D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1A0DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1A0E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F1A0E4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1A0E8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82F1A0EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1A0F0: 388ABE90  addi r4, r10, -0x4170
	ctx.r[4].s64 = ctx.r[10].s64 + -16752;
	// 82F1A0F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1A0F8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F1A0FC: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1A100: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F1A104: 4E800421  bctrl
	ctx.lr = 0x82F1A108;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F1A108: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1A10C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1A110: 80E80018  lwz r7, 0x18(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1A114: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F1A118: 4E800421  bctrl
	ctx.lr = 0x82F1A11C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F1A11C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1A120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1A124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1A128: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1A12C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A130 size=8
    let mut pc: u32 = 0x82F1A130;
    'dispatch: loop {
        match pc {
            0x82F1A130 => {
    //   block [0x82F1A130..0x82F1A138)
	// 82F1A130: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82F1A134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A138 size=16
    let mut pc: u32 = 0x82F1A138;
    'dispatch: loop {
        match pc {
            0x82F1A138 => {
    //   block [0x82F1A138..0x82F1A148)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A148 size=8
    let mut pc: u32 = 0x82F1A148;
    'dispatch: loop {
        match pc {
            0x82F1A148 => {
    //   block [0x82F1A148..0x82F1A150)
	// 82F1A148: 3565FFFF  addic. r11, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F1A14C: 4D800020  bltlr
	if ctx.cr[0].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A150 size=28
    let mut pc: u32 = 0x82F1A150;
    'dispatch: loop {
        match pc {
            0x82F1A150 => {
    //   block [0x82F1A150..0x82F1A16C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A170 size=16
    let mut pc: u32 = 0x82F1A170;
    'dispatch: loop {
        match pc {
            0x82F1A170 => {
    //   block [0x82F1A170..0x82F1A180)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A180 size=16
    let mut pc: u32 = 0x82F1A180;
    'dispatch: loop {
        match pc {
            0x82F1A180 => {
    //   block [0x82F1A180..0x82F1A190)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1A190 size=60
    let mut pc: u32 = 0x82F1A190;
    'dispatch: loop {
        match pc {
            0x82F1A190 => {
    //   block [0x82F1A190..0x82F1A1CC)
	// 82F1A190: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82F1A194: C0030010  lfs f0, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1A198: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82F1A19C: EDA0082A  fadds f13, f0, f1
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 82F1A1A0: D1A1FFF0  stfs f13, -0x10(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82F1A1A4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A1D0 size=28
    let mut pc: u32 = 0x82F1A1D0;
    'dispatch: loop {
        match pc {
            0x82F1A1D0 => {
    //   block [0x82F1A1D0..0x82F1A1EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1A1F0 size=44
    let mut pc: u32 = 0x82F1A1F0;
    'dispatch: loop {
        match pc {
            0x82F1A1F0 => {
    //   block [0x82F1A1F0..0x82F1A21C)
	// 82F1A1F0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1A1F4: D0230010  stfs f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F1A1F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F1A1FC: 392BBEA4  addi r9, r11, -0x415c
	ctx.r[9].s64 = ctx.r[11].s64 + -16732;
	// 82F1A200: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82F1A204: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F1A208: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82F1A20C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1A210: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82F1A214: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82F1A218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F1A220 size=144
    let mut pc: u32 = 0x82F1A220;
    'dispatch: loop {
        match pc {
            0x82F1A220 => {
    //   block [0x82F1A220..0x82F1A2B0)
	// 82F1A220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1A224: 4828DF49  bl 0x831a816c
	ctx.lr = 0x82F1A228;
	sub_831A8130(ctx, base);
	// 82F1A228: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1A22C: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1A230: 3BC00018  li r30, 0x18
	ctx.r[30].s64 = 24;
	// 82F1A234: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F1A238: 7D7EF82E  lwzx r11, r30, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F1A23C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F1A240: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1A244: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F1A248: 40980020  bge cr6, 0x82f1a268
	if !ctx.cr[6].lt {
	pc = 0x82F1A268; continue 'dispatch;
	}
	// 82F1A24C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F1A250: 3909BEE4  addi r8, r9, -0x411c
	ctx.r[8].s64 = ctx.r[9].s64 + -16668;
	// 82F1A254: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F1A258: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F1A25C: 392A000C  addi r9, r10, 0xc
	ctx.r[9].s64 = ctx.r[10].s64 + 12;
	// 82F1A260: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F1A264: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82F1A268: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F1A26C: C0240010  lfs f1, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F1A270: 48000041  bl 0x82f1a2b0
	ctx.lr = 0x82F1A274;
	sub_82F1A2B0(ctx, base);
	// 82F1A274: 7D5EF82E  lwzx r10, r30, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F1A278: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1A27C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F1A280: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F1A284: 40980020  bge cr6, 0x82f1a2a4
	if !ctx.cr[6].lt {
	pc = 0x82F1A2A4; continue 'dispatch;
	}
	// 82F1A288: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F1A28C: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82F1A290: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F1A294: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F1A298: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F1A29C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F1A2A0: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F1A2A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F1A2A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1A2AC: 4828DF10  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A2B0 size=164
    let mut pc: u32 = 0x82F1A2B0;
    'dispatch: loop {
        match pc {
            0x82F1A2B0 => {
    //   block [0x82F1A2B0..0x82F1A354)
	// 82F1A2B0: 39450010  addi r10, r5, 0x10
	ctx.r[10].s64 = ctx.r[5].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A354(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A354 size=92
    let mut pc: u32 = 0x82F1A354;
    'dispatch: loop {
        match pc {
            0x82F1A354 => {
    //   block [0x82F1A354..0x82F1A3B0)
	// 82F1A354: FD405890  fmr f10, f11
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[10].f64 = ctx.f[11].f64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1A3B0 size=120
    let mut pc: u32 = 0x82F1A3B0;
    'dispatch: loop {
        match pc {
            0x82F1A3B0 => {
    //   block [0x82F1A3B0..0x82F1A428)
	// 82F1A3B0: EC0D0024  fdivs f0, f13, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82F1A3B4: D0060010  stfs f0, 0x10(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1A428 size=96
    let mut pc: u32 = 0x82F1A428;
    'dispatch: loop {
        match pc {
            0x82F1A428 => {
    //   block [0x82F1A428..0x82F1A488)
	// 82F1A428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1A42C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1A430: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1A434: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1A438: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1A43C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F1A440: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82F1A444: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82F1A448: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F1A44C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1A450: 419A0020  beq cr6, 0x82f1a470
	if ctx.cr[6].eq {
	pc = 0x82F1A470; continue 'dispatch;
	}
	// 82F1A454: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1A458: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1A45C: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F1A460: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1A464: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F1A468: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1A46C: 4BF86345  bl 0x82ea07b0
	ctx.lr = 0x82F1A470;
	sub_82EA07B0(ctx, base);
	// 82F1A470: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1A474: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1A478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1A47C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1A480: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1A484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A488 size=4
    let mut pc: u32 = 0x82F1A488;
    'dispatch: loop {
        match pc {
            0x82F1A488 => {
    //   block [0x82F1A488..0x82F1A48C)
	// 82F1A488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A490 size=104
    let mut pc: u32 = 0x82F1A490;
    'dispatch: loop {
        match pc {
            0x82F1A490 => {
    //   block [0x82F1A490..0x82F1A4F8)
	// 82F1A490: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82F1A494: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F1A498: 3945000C  addi r10, r5, 0xc
	ctx.r[10].s64 = ctx.r[5].s64 + 12;
	// 82F1A49C: 39660008  addi r11, r6, 8
	ctx.r[11].s64 = ctx.r[6].s64 + 8;
	// 82F1A4A0: 7D262850  subf r9, r6, r5
	ctx.r[9].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	// 82F1A4A4: 810AFFF4  lwz r8, -0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82F1A4A8: 910BFFF8  stw r8, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[8].u32 ) };
	// 82F1A4AC: 80EAFFF8  lwz r7, -8(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1A4B0: 90EBFFFC  stw r7, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[7].u32 ) };
	// 82F1A4B4: 7C89582E  lwzx r4, r9, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1A4B8: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82F1A4BC: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1A4C0: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82F1A4C4: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F1A4C8: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82F1A4CC: 4200FFD8  bdnz 0x82f1a4a4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F1A4A4; continue 'dispatch;
	}
	// 82F1A4D0: A1650000  lhz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1A4D4: 396BFFD8  addi r11, r11, -0x28
	ctx.r[11].s64 = ctx.r[11].s64 + -40;
	// 82F1A4D8: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82F1A4DC: 419901B8  bgt cr6, 0x82f1a694
	if ctx.cr[6].gt {
		sub_82F1A694(ctx, base);
		return;
	}
	// 82F1A4E0: 3D8082F2  lis r12, -0x7d0e
	ctx.r[12].s64 = -2098069504;
	// 82F1A4E4: 398CA4F8  addi r12, r12, -0x5b08
	ctx.r[12].s64 = ctx.r[12].s64 + -23304;
	// 82F1A4E8: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82F1A4EC: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82F1A4F0: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82F1A4F4: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
			// ERROR: 0x82F1A514
			return;
		},
		1 => {
			// ERROR: 0x82F1A558
			return;
		},
		2 => {
			// ERROR: 0x82F1A614
			return;
		},
		3 => {
			// ERROR: 0x82F1A658
			return;
		},
		4 => {
			// ERROR: 0x82F1A694
			return;
		},
		5 => {
			// ERROR: 0x82F1A594
			return;
		},
		6 => {
			// ERROR: 0x82F1A5D0
			return;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A4F8 size=96
    let mut pc: u32 = 0x82F1A4F8;
    'dispatch: loop {
        match pc {
            0x82F1A4F8 => {
    //   block [0x82F1A4F8..0x82F1A558)
	// 82F1A4F8: 82F1A514  lwz r23, -0x5aec(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-23276 as u32) ) } as u64;
	// 82F1A4FC: 82F1A558  lwz r23, -0x5aa8(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-23208 as u32) ) } as u64;
	// 82F1A500: 82F1A614  lwz r23, -0x59ec(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-23020 as u32) ) } as u64;
	// 82F1A504: 82F1A658  lwz r23, -0x59a8(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-22952 as u32) ) } as u64;
	// 82F1A508: 82F1A694  lwz r23, -0x596c(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-22892 as u32) ) } as u64;
	// 82F1A50C: 82F1A594  lwz r23, -0x5a6c(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-23148 as u32) ) } as u64;
	// 82F1A510: 82F1A5D0  lwz r23, -0x5a30(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-23088 as u32) ) } as u64;
	// 82F1A514: 81450018  lwz r10, 0x18(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1A518: 81650010  lwz r11, 0x10(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F1A51C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F1A520: 40990174  ble cr6, 0x82f1a694
	if !ctx.cr[6].gt {
		sub_82F1A694(ctx, base);
		return;
	}
	// 82F1A524: 91660018  stw r11, 0x18(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82F1A528: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82F1A52C: 81250018  lwz r9, 0x18(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1A530: 81650010  lwz r11, 0x10(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F1A534: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F1A538: 7CEB4850  subf r7, r11, r9
	ctx.r[7].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82F1A53C: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F1A540: 81450014  lwz r10, 0x14(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F1A544: 90E50018  stw r7, 0x18(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), ctx.r[7].u32 ) };
	// 82F1A548: 55092036  slwi r9, r8, 4
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F1A54C: 7CC95214  add r6, r9, r10
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82F1A550: 90C50014  stw r6, 0x14(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82F1A554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A558 size=60
    let mut pc: u32 = 0x82F1A558;
    'dispatch: loop {
        match pc {
            0x82F1A558 => {
    //   block [0x82F1A558..0x82F1A594)
	// 82F1A558: 81450018  lwz r10, 0x18(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1A55C: 81650010  lwz r11, 0x10(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F1A560: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F1A564: 40990130  ble cr6, 0x82f1a694
	if !ctx.cr[6].gt {
		sub_82F1A694(ctx, base);
		return;
	}
	// 82F1A568: 91660018  stw r11, 0x18(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82F1A56C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82F1A570: 81650014  lwz r11, 0x14(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F1A574: 81050010  lwz r8, 0x10(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F1A578: 81250018  lwz r9, 0x18(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1A57C: 550A3032  slwi r10, r8, 6
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F1A580: 7CE84850  subf r7, r8, r9
	ctx.r[7].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82F1A584: 7CCA5A14  add r6, r10, r11
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F1A588: 90E50018  stw r7, 0x18(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), ctx.r[7].u32 ) };
	// 82F1A58C: 90C50014  stw r6, 0x14(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82F1A590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A594(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A594 size=60
    let mut pc: u32 = 0x82F1A594;
    'dispatch: loop {
        match pc {
            0x82F1A594 => {
    //   block [0x82F1A594..0x82F1A5D0)
	// 82F1A594: 8145002C  lwz r10, 0x2c(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F1A598: 81650024  lwz r11, 0x24(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F1A59C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F1A5A0: 409900F4  ble cr6, 0x82f1a694
	if !ctx.cr[6].gt {
		sub_82F1A694(ctx, base);
		return;
	}
	// 82F1A5A4: 9166002C  stw r11, 0x2c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82F1A5A8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82F1A5AC: 81250024  lwz r9, 0x24(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F1A5B0: 8105002C  lwz r8, 0x2c(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F1A5B4: 81450028  lwz r10, 0x28(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F1A5B8: 552B3032  slwi r11, r9, 6
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F1A5BC: 7CE94050  subf r7, r9, r8
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 82F1A5C0: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F1A5C4: 90E5002C  stw r7, 0x2c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(44 as u32), ctx.r[7].u32 ) };
	// 82F1A5C8: 90C50028  stw r6, 0x28(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(40 as u32), ctx.r[6].u32 ) };
	// 82F1A5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A5D0 size=68
    let mut pc: u32 = 0x82F1A5D0;
    'dispatch: loop {
        match pc {
            0x82F1A5D0 => {
    //   block [0x82F1A5D0..0x82F1A614)
	// 82F1A5D0: 81450018  lwz r10, 0x18(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1A5D4: 81650010  lwz r11, 0x10(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F1A5D8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F1A5DC: 409900B8  ble cr6, 0x82f1a694
	if !ctx.cr[6].gt {
		sub_82F1A694(ctx, base);
		return;
	}
	// 82F1A5E0: 91660018  stw r11, 0x18(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82F1A5E4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82F1A5E8: 81650010  lwz r11, 0x10(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F1A5EC: 81050018  lwz r8, 0x18(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1A5F0: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F1A5F4: 81450014  lwz r10, 0x14(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F1A5F8: 7CEB4050  subf r7, r11, r8
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 82F1A5FC: 7CCB4A14  add r6, r11, r9
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F1A600: 90E50018  stw r7, 0x18(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), ctx.r[7].u32 ) };
	// 82F1A604: 54CB2036  slwi r11, r6, 4
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F1A608: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F1A60C: 90850014  stw r4, 0x14(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82F1A610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A614(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A614 size=68
    let mut pc: u32 = 0x82F1A614;
    'dispatch: loop {
        match pc {
            0x82F1A614 => {
    //   block [0x82F1A614..0x82F1A658)
	// 82F1A614: 8145002C  lwz r10, 0x2c(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F1A618: 81650024  lwz r11, 0x24(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F1A61C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F1A620: 40990074  ble cr6, 0x82f1a694
	if !ctx.cr[6].gt {
		sub_82F1A694(ctx, base);
		return;
	}
	// 82F1A624: 9166002C  stw r11, 0x2c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82F1A628: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82F1A62C: 81450028  lwz r10, 0x28(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F1A630: 81650024  lwz r11, 0x24(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F1A634: 8105002C  lwz r8, 0x2c(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F1A638: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F1A63C: 7CEB4050  subf r7, r11, r8
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 82F1A640: 7CCB4A14  add r6, r11, r9
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F1A644: 90E5002C  stw r7, 0x2c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(44 as u32), ctx.r[7].u32 ) };
	// 82F1A648: 54CB2036  slwi r11, r6, 4
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F1A64C: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F1A650: 90850028  stw r4, 0x28(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(40 as u32), ctx.r[4].u32 ) };
	// 82F1A654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A658 size=60
    let mut pc: u32 = 0x82F1A658;
    'dispatch: loop {
        match pc {
            0x82F1A658 => {
    //   block [0x82F1A658..0x82F1A694)
	// 82F1A658: 8145001C  lwz r10, 0x1c(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F1A65C: 81650014  lwz r11, 0x14(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F1A660: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F1A664: 40990030  ble cr6, 0x82f1a694
	if !ctx.cr[6].gt {
		sub_82F1A694(ctx, base);
		return;
	}
	// 82F1A668: 9166001C  stw r11, 0x1c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82F1A66C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82F1A670: 81250014  lwz r9, 0x14(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F1A674: 8105001C  lwz r8, 0x1c(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F1A678: 81450018  lwz r10, 0x18(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1A67C: 552B2834  slwi r11, r9, 5
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F1A680: 7CE94050  subf r7, r9, r8
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 82F1A684: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F1A688: 90E5001C  stw r7, 0x1c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(28 as u32), ctx.r[7].u32 ) };
	// 82F1A68C: 90C50018  stw r6, 0x18(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), ctx.r[6].u32 ) };
	// 82F1A690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A694(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A694 size=8
    let mut pc: u32 = 0x82F1A694;
    'dispatch: loop {
        match pc {
            0x82F1A694 => {
    //   block [0x82F1A694..0x82F1A69C)
	// 82F1A694: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F1A698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1A6A0 size=160
    let mut pc: u32 = 0x82F1A6A0;
    'dispatch: loop {
        match pc {
            0x82F1A6A0 => {
    //   block [0x82F1A6A0..0x82F1A6F4)
	// 82F1A6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1A6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1A6A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1A6AC: A1450000  lhz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1A6B0: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F1A6B4: 394AFFD8  addi r10, r10, -0x28
	ctx.r[10].s64 = ctx.r[10].s64 + -40;
	// 82F1A6B8: 2B0A0006  cmplwi cr6, r10, 6
	ctx.cr[6].compare_u32(ctx.r[10].u32, 6 as u32, &mut ctx.xer);
	// 82F1A6BC: 41990070  bgt cr6, 0x82f1a72c
	if ctx.cr[6].gt {
	pc = 0x82F1A72C; continue 'dispatch;
	}
	// 82F1A6C0: 3D8082F2  lis r12, -0x7d0e
	ctx.r[12].s64 = -2098069504;
	// 82F1A6C4: 398CA6D8  addi r12, r12, -0x5928
	ctx.r[12].s64 = ctx.r[12].s64 + -22824;
	// 82F1A6C8: 5540103A  slwi r0, r10, 2
	ctx.r[0].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82F1A6CC: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82F1A6D0: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82F1A6D4: 4E800420  bctr
	match ctx.r[10].u64 {
		0 => {
	pc = 0x82F1A6F4; continue 'dispatch;
		},
		1 => {
	pc = 0x82F1A6F4; continue 'dispatch;
		},
		2 => {
	pc = 0x82F1A6F4; continue 'dispatch;
		},
		3 => {
	pc = 0x82F1A6F4; continue 'dispatch;
		},
		4 => {
	pc = 0x82F1A704; continue 'dispatch;
		},
		5 => {
	pc = 0x82F1A6F4; continue 'dispatch;
		},
		6 => {
	pc = 0x82F1A6F4; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82F1A6D8: 82F1A6F4  lwz r23, -0x590c(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-22796 as u32) ) } as u64;
	// 82F1A6DC: 82F1A6F4  lwz r23, -0x590c(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-22796 as u32) ) } as u64;
	// 82F1A6E0: 82F1A6F4  lwz r23, -0x590c(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-22796 as u32) ) } as u64;
	// 82F1A6E4: 82F1A6F4  lwz r23, -0x590c(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-22796 as u32) ) } as u64;
	// 82F1A6E8: 82F1A704  lwz r23, -0x58fc(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-22780 as u32) ) } as u64;
	// 82F1A6EC: 82F1A6F4  lwz r23, -0x590c(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-22796 as u32) ) } as u64;
	// 82F1A6F0: 82F1A6F4  lwz r23, -0x590c(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-22796 as u32) ) } as u64;
            }
            0x82F1A6F4 => {
    //   block [0x82F1A6F4..0x82F1A704)
	// 82F1A6F4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1A6F8: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F1A6FC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F1A700: 4800001C  b 0x82f1a71c
	pc = 0x82F1A71C; continue 'dispatch;
            }
            0x82F1A704 => {
    //   block [0x82F1A704..0x82F1A740)
	// 82F1A704: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1A708: 81450028  lwz r10, 0x28(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F1A70C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82F1A710: 3D2AFFF1  addis r9, r10, -0xf
	ctx.r[9].s64 = ctx.r[10].s64 + -983040;
	// 82F1A714: 3529BDC0  addic. r9, r9, -0x4240
	ctx.xer.ca = (ctx.r[9].u32 > (!(-16960 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -16960;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F1A718: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1A71C: 40820010  bne 0x82f1a72c
	if !ctx.cr[0].eq {
	pc = 0x82F1A72C; continue 'dispatch;
	}
	// 82F1A720: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F1A724: 80650004  lwz r3, 4(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1A728: 4BF87F39  bl 0x82ea2660
	ctx.lr = 0x82F1A72C;
	sub_82EA2660(ctx, base);
	// 82F1A72C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82F1A730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1A734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1A738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1A73C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1A740 size=176
    let mut pc: u32 = 0x82F1A740;
    'dispatch: loop {
        match pc {
            0x82F1A740 => {
    //   block [0x82F1A740..0x82F1A7F0)
	// 82F1A740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1A744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1A748: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1A74C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1A750: 3D6082F3  lis r11, -0x7d0d
	ctx.r[11].s64 = -2098003968;
	// 82F1A754: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 82F1A758: 38AB8F60  addi r5, r11, -0x70a0
	ctx.r[5].s64 = ctx.r[11].s64 + -28832;
	// 82F1A75C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1A760: 48031931  bl 0x82f4c090
	ctx.lr = 0x82F1A764;
	sub_82F4C090(ctx, base);
	// 82F1A764: 3D4082F3  lis r10, -0x7d0d
	ctx.r[10].s64 = -2098003968;
	// 82F1A768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1A76C: 38AA8D98  addi r5, r10, -0x7268
	ctx.r[5].s64 = ctx.r[10].s64 + -29288;
	// 82F1A770: 38800029  li r4, 0x29
	ctx.r[4].s64 = 41;
	// 82F1A774: 4803191D  bl 0x82f4c090
	ctx.lr = 0x82F1A778;
	sub_82F4C090(ctx, base);
	// 82F1A778: 3D2082F3  lis r9, -0x7d0d
	ctx.r[9].s64 = -2098003968;
	// 82F1A77C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1A780: 38A989C8  addi r5, r9, -0x7638
	ctx.r[5].s64 = ctx.r[9].s64 + -30264;
	// 82F1A784: 3880002D  li r4, 0x2d
	ctx.r[4].s64 = 45;
	// 82F1A788: 48031909  bl 0x82f4c090
	ctx.lr = 0x82F1A78C;
	sub_82F4C090(ctx, base);
	// 82F1A78C: 3D0082F3  lis r8, -0x7d0d
	ctx.r[8].s64 = -2098003968;
	// 82F1A790: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1A794: 38A884C0  addi r5, r8, -0x7b40
	ctx.r[5].s64 = ctx.r[8].s64 + -31552;
	// 82F1A798: 3880002E  li r4, 0x2e
	ctx.r[4].s64 = 46;
	// 82F1A79C: 480318F5  bl 0x82f4c090
	ctx.lr = 0x82F1A7A0;
	sub_82F4C090(ctx, base);
	// 82F1A7A0: 3CE082F3  lis r7, -0x7d0d
	ctx.r[7].s64 = -2098003968;
	// 82F1A7A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1A7A8: 38A781C8  addi r5, r7, -0x7e38
	ctx.r[5].s64 = ctx.r[7].s64 + -32312;
	// 82F1A7AC: 3880002A  li r4, 0x2a
	ctx.r[4].s64 = 42;
	// 82F1A7B0: 480318E1  bl 0x82f4c090
	ctx.lr = 0x82F1A7B4;
	sub_82F4C090(ctx, base);
	// 82F1A7B4: 3CC082F3  lis r6, -0x7d0d
	ctx.r[6].s64 = -2098003968;
	// 82F1A7B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1A7BC: 38A68078  addi r5, r6, -0x7f88
	ctx.r[5].s64 = ctx.r[6].s64 + -32648;
	// 82F1A7C0: 3880002B  li r4, 0x2b
	ctx.r[4].s64 = 43;
	// 82F1A7C4: 480318CD  bl 0x82f4c090
	ctx.lr = 0x82F1A7C8;
	sub_82F4C090(ctx, base);
	// 82F1A7C8: 3CA082F2  lis r5, -0x7d0e
	ctx.r[5].s64 = -2098069504;
	// 82F1A7CC: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 82F1A7D0: 38A57B50  addi r5, r5, 0x7b50
	ctx.r[5].s64 = ctx.r[5].s64 + 31568;
	// 82F1A7D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1A7D8: 480318B9  bl 0x82f4c090
	ctx.lr = 0x82F1A7DC;
	sub_82F4C090(ctx, base);
	// 82F1A7DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1A7E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1A7E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1A7E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1A7EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1A7F0 size=156
    let mut pc: u32 = 0x82F1A7F0;
    'dispatch: loop {
        match pc {
            0x82F1A7F0 => {
    //   block [0x82F1A7F0..0x82F1A88C)
	// 82F1A7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1A7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1A7F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1A7FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1A800: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1A804: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F1A808: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F1A80C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F1A810: 419A001C  beq cr6, 0x82f1a82c
	if ctx.cr[6].eq {
	pc = 0x82F1A82C; continue 'dispatch;
	}
	// 82F1A814: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1A818: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1A81C: 419A0010  beq cr6, 0x82f1a82c
	if ctx.cr[6].eq {
	pc = 0x82F1A82C; continue 'dispatch;
	}
	// 82F1A820: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F1A824: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82F1A828: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F1A82C: 807E005C  lwz r3, 0x5c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F1A830: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1A834: 419A003C  beq cr6, 0x82f1a870
	if ctx.cr[6].eq {
	pc = 0x82F1A870; continue 'dispatch;
	}
	// 82F1A838: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1A83C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1A840: 419A0030  beq cr6, 0x82f1a870
	if ctx.cr[6].eq {
	pc = 0x82F1A870; continue 'dispatch;
	}
	// 82F1A844: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F1A848: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F1A84C: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F1A850: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F1A854: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F1A858: 409A0018  bne cr6, 0x82f1a870
	if !ctx.cr[6].eq {
	pc = 0x82F1A870; continue 'dispatch;
	}
	// 82F1A85C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1A860: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F1A864: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1A868: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1A86C: 4E800421  bctrl
	ctx.lr = 0x82F1A870;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F1A870: 93FE005C  stw r31, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82F1A874: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1A878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1A87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1A880: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1A884: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1A888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A890 size=8
    let mut pc: u32 = 0x82F1A890;
    'dispatch: loop {
        match pc {
            0x82F1A890 => {
    //   block [0x82F1A890..0x82F1A898)
	// 82F1A890: 8063004C  lwz r3, 0x4c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82F1A894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A898 size=12
    let mut pc: u32 = 0x82F1A898;
    'dispatch: loop {
        match pc {
            0x82F1A898 => {
    //   block [0x82F1A898..0x82F1A8A4)
	// 82F1A898: 548B2036  slwi r11, r4, 4
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F1A89C: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82F1A8A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1A8A8 size=8
    let mut pc: u32 = 0x82F1A8A8;
    'dispatch: loop {
        match pc {
            0x82F1A8A8 => {
    //   block [0x82F1A8A8..0x82F1A8B0)
	// 82F1A8A8: 38630050  addi r3, r3, 0x50
	ctx.r[3].s64 = ctx.r[3].s64 + 80;
	// 82F1A8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1A8B0 size=44
    let mut pc: u32 = 0x82F1A8B0;
    'dispatch: loop {
        match pc {
            0x82F1A8B0 => {
    //   block [0x82F1A8B0..0x82F1A8DC)
	// 82F1A8B0: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F1A8B4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82F1A8B8: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1A8BC: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82F1A8C0: C1AB0010  lfs f13, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F1A8C4: D1A40004  stfs f13, 4(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82F1A8C8: C18B0020  lfs f12, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F1A8CC: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1A8D0: D1840008  stfs f12, 8(r4)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82F1A8D4: D004000C  stfs f0, 0xc(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82F1A8D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1A8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1A8E0 size=452
    let mut pc: u32 = 0x82F1A8E0;
    'dispatch: loop {
        match pc {
            0x82F1A8E0 => {
    //   block [0x82F1A8E0..0x82F1AAA4)
	// 82F1A8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1A8E4: 4828D889  bl 0x831a816c
	ctx.lr = 0x82F1A8E8;
	sub_831A8130(ctx, base);
	// 82F1A8E8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82F1A8EC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F1A8F0: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82F1A8F4: 810A004C  lwz r8, 0x4c(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 82F1A8F8: 816A0040  lwz r11, 0x40(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F1A8FC: 38E8FFFF  addi r7, r8, -1
	ctx.r[7].s64 = ctx.r[8].s64 + -1;
	// 82F1A900: 2F070003  cmpwi cr6, r7, 3
	ctx.cr[6].compare_i32(ctx.r[7].s32, 3, &mut ctx.xer);
	// 82F1A904: 419800F0  blt cr6, 0x82f1a9f4
	if ctx.cr[6].lt {
	pc = 0x82F1A9F4; continue 'dispatch;
	}
	// 82F1A908: 39070001  addi r8, r7, 1
	ctx.r[8].s64 = ctx.r[7].s64 + 1;
	// 82F1A90C: 5508F0BE  srwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1A910: 5506103A  slwi r6, r8, 2
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82F1A914: 7CE63850  subf r7, r6, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[6].s64;
	// 82F1A918: 38C1FFA0  addi r6, r1, -0x60
	ctx.r[6].s64 = ctx.r[1].s64 + -96;
	// 82F1A91C: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1A920: C1AB0010  lfs f13, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F1A924: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 82F1A928: C18B0020  lfs f12, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F1A92C: 3881FFB0  addi r4, r1, -0x50
	ctx.r[4].s64 = ctx.r[1].s64 + -80;
	// 82F1A930: C16A0010  lfs f11, 0x10(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F1A934: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82F1A938: D001FFA0  stfs f0, -0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), tmp.u32 ) };
	// 82F1A93C: 3BE1FFC0  addi r31, r1, -0x40
	ctx.r[31].s64 = ctx.r[1].s64 + -64;
	// 82F1A940: D1A1FFA4  stfs f13, -0x5c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-92 as u32), tmp.u32 ) };
	// 82F1A944: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 82F1A948: D181FFA8  stfs f12, -0x58(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), tmp.u32 ) };
	// 82F1A94C: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82F1A950: D161FFAC  stfs f11, -0x54(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-84 as u32), tmp.u32 ) };
	// 82F1A954: 3BA1FFD0  addi r29, r1, -0x30
	ctx.r[29].s64 = ctx.r[1].s64 + -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1AAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1AAA8 size=60
    let mut pc: u32 = 0x82F1AAA8;
    'dispatch: loop {
        match pc {
            0x82F1AAA8 => {
    //   block [0x82F1AAA8..0x82F1AAE4)
	// 82F1AAA8: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82F1AAAC: 80E30040  lwz r7, 0x40(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F1AAB0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82F1AAB4: 392B9240  addi r9, r11, -0x6dc0
	ctx.r[9].s64 = ctx.r[11].s64 + -28096;
	// 82F1AAB8: 816A9250  lwz r11, -0x6db0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28080 as u32) ) } as u64;
	// 82F1AABC: 556807FE  clrlwi r8, r11, 0x1f
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82F1AAC0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82F1AAC4: 409A0020  bne cr6, 0x82f1aae4
	if !ctx.cr[6].eq {
		sub_82F1AAE4(ctx, base);
		return;
	}
	// 82F1AAC8: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F1AACC: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82F1AAD0: 38C8BF30  addi r6, r8, -0x40d0
	ctx.r[6].s64 = ctx.r[8].s64 + -16592;
	// 82F1AAD4: 916A9250  stw r11, -0x6db0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28080 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1AAE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1AAE4 size=164
    let mut pc: u32 = 0x82F1AAE4;
    'dispatch: loop {
        match pc {
            0x82F1AAE4 => {
    //   block [0x82F1AAE4..0x82F1AB88)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1AB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1AB88 size=192
    let mut pc: u32 = 0x82F1AB88;
    'dispatch: loop {
        match pc {
            0x82F1AB88 => {
    //   block [0x82F1AB88..0x82F1AC48)
	// 82F1AB88: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1AC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1AC48 size=512
    let mut pc: u32 = 0x82F1AC48;
    'dispatch: loop {
        match pc {
            0x82F1AC48 => {
    //   block [0x82F1AC48..0x82F1AE48)
	// 82F1AC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1AC4C: 4828D521  bl 0x831a816c
	ctx.lr = 0x82F1AC50;
	sub_831A8130(ctx, base);
	// 82F1AC50: 3965FFFF  addi r11, r5, -1
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	// 82F1AC54: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82F1AC58: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82F1AC5C: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82F1AC60: 41980178  blt cr6, 0x82f1add8
	if ctx.cr[6].lt {
	pc = 0x82F1ADD8; continue 'dispatch;
	}
	// 82F1AC64: 5548003A  rlwinm r8, r10, 0, 0, 0x1d
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1AC68: 39660018  addi r11, r6, 0x18
	ctx.r[11].s64 = ctx.r[6].s64 + 24;
	// 82F1AC6C: 5549F0BE  srwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F1AC70: 7FE8F850  subf r31, r8, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[8].s64;
	// 82F1AC74: A3C40000  lhz r30, 0(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1AC78: 80A30040  lwz r5, 0x40(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F1AC7C: 7FCA1670  srawi r10, r30, 2
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[30].s32 >> 2) as i64;
	// 82F1AC80: 57C807BE  clrlwi r8, r30, 0x1e
	ctx.r[8].u64 = ctx.r[30].u32 as u64 & 0x00000003u64;
	// 82F1AC84: 5547083C  slwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1AC88: 3BA80004  addi r29, r8, 4
	ctx.r[29].s64 = ctx.r[8].s64 + 4;
	// 82F1AC8C: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82F1AC90: 57C7173A  rlwinm r7, r30, 2, 0x1c, 0x1d
	ctx.r[7].u64 = ctx.r[30].u32 as u64 & 0x3FFFFFFFu64;
	// 82F1AC94: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F1AC98: 57BD103A  slwi r29, r29, 2
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82F1AC9C: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82F1ACA0: 38A80008  addi r5, r8, 8
	ctx.r[5].s64 = ctx.r[8].s64 + 8;
	// 82F1ACA4: 67C83F00  oris r8, r30, 0x3f00
	ctx.r[8].u64 = ctx.r[30].u64 | 1056964608;
	// 82F1ACA8: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F1ACAC: 7C07542E  lfsx f0, r7, r10
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1ACB0: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82F1ACB4: 7DBD542E  lfsx f13, r29, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F1ACB8: D1ABFFEC  stfs f13, -0x14(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-20 as u32), tmp.u32 ) };
	// 82F1ACBC: 7D85542E  lfsx f12, r5, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F1ACC0: D18BFFF0  stfs f12, -0x10(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82F1ACC4: 910BFFF4  stw r8, -0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-12 as u32), ctx.r[8].u32 ) };
	// 82F1ACC8: A0A40002  lhz r5, 2(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 82F1ACCC: 7CAA1670  srawi r10, r5, 2
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[5].s32 >> 2) as i64;
	// 82F1ACD0: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1ACD4: 80E30040  lwz r7, 0x40(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F1ACD8: 54BE173A  rlwinm r30, r5, 2, 0x1c, 0x1d
	ctx.r[30].u64 = ctx.r[5].u32 as u64 & 0x3FFFFFFFu64;
	// 82F1ACDC: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82F1ACE0: 54A807BE  clrlwi r8, r5, 0x1e
	ctx.r[8].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	// 82F1ACE4: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F1ACE8: 3BA80004  addi r29, r8, 4
	ctx.r[29].s64 = ctx.r[8].s64 + 4;
	// 82F1ACEC: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82F1ACF0: 57A7103A  slwi r7, r29, 2
	ctx.r[7].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1ACF4: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 82F1ACF8: 64A53F00  oris r5, r5, 0x3f00
	ctx.r[5].u64 = ctx.r[5].u64 | 1056964608;
	// 82F1ACFC: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1AD00: 7D7E542E  lfsx f11, r30, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F1AD04: D16BFFF8  stfs f11, -8(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82F1AD08: 7D47542E  lfsx f10, r7, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82F1AD0C: D14BFFFC  stfs f10, -4(r11)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82F1AD10: 7D28542E  lfsx f9, r8, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82F1AD14: D12B0000  stfs f9, 0(r11)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82F1AD18: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F1AD1C: A0A40004  lhz r5, 4(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1AD20: 7CAA1670  srawi r10, r5, 2
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[5].s32 >> 2) as i64;
	// 82F1AD24: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1AD28: 80E30040  lwz r7, 0x40(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F1AD2C: 54BE173A  rlwinm r30, r5, 2, 0x1c, 0x1d
	ctx.r[30].u64 = ctx.r[5].u32 as u64 & 0x3FFFFFFFu64;
	// 82F1AD30: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82F1AD34: 54A807BE  clrlwi r8, r5, 0x1e
	ctx.r[8].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	// 82F1AD38: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F1AD3C: 3BA80004  addi r29, r8, 4
	ctx.r[29].s64 = ctx.r[8].s64 + 4;
	// 82F1AD40: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82F1AD44: 57A7103A  slwi r7, r29, 2
	ctx.r[7].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1AD48: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 82F1AD4C: 64A53F00  oris r5, r5, 0x3f00
	ctx.r[5].u64 = ctx.r[5].u64 | 1056964608;
	// 82F1AD50: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1AD54: 7D1E542E  lfsx f8, r30, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82F1AD58: D10B0008  stfs f8, 8(r11)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82F1AD5C: 7CE7542E  lfsx f7, r7, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82F1AD60: D0EB000C  stfs f7, 0xc(r11)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82F1AD64: 7CC8542E  lfsx f6, r8, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82F1AD68: D0CB0010  stfs f6, 0x10(r11)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F1AD6C: 90AB0014  stw r5, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82F1AD70: A0A40006  lhz r5, 6(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F1AD74: 7CAA1670  srawi r10, r5, 2
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[5].s32 >> 2) as i64;
	// 82F1AD78: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1AD7C: 80E30040  lwz r7, 0x40(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F1AD80: 54BE173A  rlwinm r30, r5, 2, 0x1c, 0x1d
	ctx.r[30].u64 = ctx.r[5].u32 as u64 & 0x3FFFFFFFu64;
	// 82F1AD84: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82F1AD88: 54A807BE  clrlwi r8, r5, 0x1e
	ctx.r[8].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	// 82F1AD8C: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F1AD90: 3BA80004  addi r29, r8, 4
	ctx.r[29].s64 = ctx.r[8].s64 + 4;
	// 82F1AD94: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82F1AD98: 57A7103A  slwi r7, r29, 2
	ctx.r[7].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1AD9C: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 82F1ADA0: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1ADA4: 7CBE542E  lfsx f5, r30, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82F1ADA8: D0AB0018  stfs f5, 0x18(r11)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82F1ADAC: 7C87542E  lfsx f4, r7, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82F1ADB0: D08B001C  stfs f4, 0x1c(r11)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82F1ADB4: 7C68542E  lfsx f3, r8, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82F1ADB8: 64A73F00  oris r7, r5, 0x3f00
	ctx.r[7].u64 = ctx.r[5].u64 | 1056964608;
	// 82F1ADBC: D06B0020  stfs f3, 0x20(r11)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82F1ADC0: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F1ADC4: 90EB0024  stw r7, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[7].u32 ) };
	// 82F1ADC8: 38840008  addi r4, r4, 8
	ctx.r[4].s64 = ctx.r[4].s64 + 8;
	// 82F1ADCC: 38C60040  addi r6, r6, 0x40
	ctx.r[6].s64 = ctx.r[6].s64 + 64;
	// 82F1ADD0: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 82F1ADD4: 4082FEA0  bne 0x82f1ac74
	if !ctx.cr[0].eq {
	pc = 0x82F1AC74; continue 'dispatch;
	}
	// 82F1ADD8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82F1ADDC: 41980068  blt cr6, 0x82f1ae44
	if ctx.cr[6].lt {
	pc = 0x82F1AE44; continue 'dispatch;
	}
	// 82F1ADE0: A0E40000  lhz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1ADE4: 38840002  addi r4, r4, 2
	ctx.r[4].s64 = ctx.r[4].s64 + 2;
	// 82F1ADE8: 81230040  lwz r9, 0x40(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F1ADEC: 7CEB1670  srawi r11, r7, 2
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[7].s32 >> 2) as i64;
	// 82F1ADF0: 54EA07BE  clrlwi r10, r7, 0x1e
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	// 82F1ADF4: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1ADF8: 38AA0004  addi r5, r10, 4
	ctx.r[5].s64 = ctx.r[10].s64 + 4;
	// 82F1ADFC: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82F1AE00: 54E8173A  rlwinm r8, r7, 2, 0x1c, 0x1d
	ctx.r[8].u64 = ctx.r[7].u32 as u64 & 0x3FFFFFFFu64;
	// 82F1AE04: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F1AE08: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F1AE0C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F1AE10: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F1AE14: 64E93F00  oris r9, r7, 0x3f00
	ctx.r[9].u64 = ctx.r[7].u64 | 1056964608;
	// 82F1AE18: 5547103A  slwi r7, r10, 2
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1AE1C: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82F1AE20: 7C085C2E  lfsx f0, r8, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1AE24: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82F1AE28: 7DA55C2E  lfsx f13, r5, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F1AE2C: D1A60004  stfs f13, 4(r6)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82F1AE30: 7D875C2E  lfsx f12, r7, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F1AE34: D1860008  stfs f12, 8(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82F1AE38: 9126000C  stw r9, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82F1AE3C: 38C60010  addi r6, r6, 0x10
	ctx.r[6].s64 = ctx.r[6].s64 + 16;
	// 82F1AE40: 4080FFA0  bge 0x82f1ade0
	if !ctx.cr[0].lt {
	pc = 0x82F1ADE0; continue 'dispatch;
	}
	// 82F1AE44: 4828D378  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1AE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1AE48 size=16
    let mut pc: u32 = 0x82F1AE48;
    'dispatch: loop {
        match pc {
            0x82F1AE48 => {
    //   block [0x82F1AE48..0x82F1AE58)
	// 82F1AE48: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1AE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1AE58 size=224
    let mut pc: u32 = 0x82F1AE58;
    'dispatch: loop {
        match pc {
            0x82F1AE58 => {
    //   block [0x82F1AE58..0x82F1AF38)
	// 82F1AE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1AE5C: 4828D30D  bl 0x831a8168
	ctx.lr = 0x82F1AE60;
	sub_831A8130(ctx, base);
	// 82F1AE60: 39650003  addi r11, r5, 3
	ctx.r[11].s64 = ctx.r[5].s64 + 3;
	// 82F1AE64: D0230010  stfs f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F1AE68: 3FE08213  lis r31, -0x7ded
	ctx.r[31].s64 = -2112684032;
	// 82F1AE6C: 556A003A  rlwinm r10, r11, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1AE70: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82F1AE74: 7D4B1670  srawi r11, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82F1AE78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F1AE7C: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82F1AE80: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82F1AE84: 3BFFBF44  addi r31, r31, -0x40bc
	ctx.r[31].s64 = ctx.r[31].s64 + -16572;
	// 82F1AE88: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82F1AE8C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82F1AE90: B3A30006  sth r29, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[29].u16 ) };
	// 82F1AE94: 3B800007  li r28, 7
	ctx.r[28].s64 = 7;
	// 82F1AE98: 9383000C  stw r28, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82F1AE9C: 90830040  stw r4, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[4].u32 ) };
	// 82F1AEA0: 7D7F4B78  or r31, r11, r9
	ctx.r[31].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 82F1AEA4: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 82F1AEA8: 93E30048  stw r31, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 82F1AEAC: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82F1AEB0: 3BE1FFD0  addi r31, r1, -0x30
	ctx.r[31].s64 = ctx.r[1].s64 + -48;
	// 82F1AEB4: 90A3004C  stw r5, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[5].u32 ) };
	// 82F1AEB8: 38A1FFD0  addi r5, r1, -0x30
	ctx.r[5].s64 = ctx.r[1].s64 + -48;
	// 82F1AEBC: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 82F1AEC0: 90C30050  stw r6, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82F1AEC4: 90E30054  stw r7, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82F1AEC8: 39280010  addi r9, r8, 0x10
	ctx.r[9].s64 = ctx.r[8].s64 + 16;
	// 82F1AECC: C0049450  lfs f0, -0x6bb0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1AED0: 7CFEF378  or r30, r7, r30
	ctx.r[30].u64 = ctx.r[7].u64 | ctx.r[30].u64;
	// 82F1AED4: D001FFD0  stfs f0, -0x30(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), tmp.u32 ) };
	// 82F1AED8: 93C30058  stw r30, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1AF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1AF38 size=232
    let mut pc: u32 = 0x82F1AF38;
    'dispatch: loop {
        match pc {
            0x82F1AF38 => {
    //   block [0x82F1AF38..0x82F1B020)
	// 82F1AF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1AF3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1AF40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1AF44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1AF48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1AF4C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1AF50: 394BBF44  addi r10, r11, -0x40bc
	ctx.r[10].s64 = ctx.r[11].s64 + -16572;
	// 82F1AF54: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F1AF58: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F1AF5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1AF60: 419A003C  beq cr6, 0x82f1af9c
	if ctx.cr[6].eq {
	pc = 0x82F1AF9C; continue 'dispatch;
	}
	// 82F1AF64: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1AF68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1AF6C: 419A0030  beq cr6, 0x82f1af9c
	if ctx.cr[6].eq {
	pc = 0x82F1AF9C; continue 'dispatch;
	}
	// 82F1AF70: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F1AF74: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F1AF78: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F1AF7C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F1AF80: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F1AF84: 409A0018  bne cr6, 0x82f1af9c
	if !ctx.cr[6].eq {
	pc = 0x82F1AF9C; continue 'dispatch;
	}
	// 82F1AF88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1AF8C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F1AF90: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1AF94: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1AF98: 4E800421  bctrl
	ctx.lr = 0x82F1AF9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F1AF9C: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F1AFA0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1AFA4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F1AFA8: 409A0020  bne cr6, 0x82f1afc8
	if !ctx.cr[6].eq {
	pc = 0x82F1AFC8; continue 'dispatch;
	}
	// 82F1AFAC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1AFB0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F1AFB4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F1AFB8: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F1AFBC: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F1AFC0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F1AFC4: 4BF857ED  bl 0x82ea07b0
	ctx.lr = 0x82F1AFC8;
	sub_82EA07B0(ctx, base);
	// 82F1AFC8: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F1AFCC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1AFD0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F1AFD4: 409A002C  bne cr6, 0x82f1b000
	if !ctx.cr[6].eq {
	pc = 0x82F1B000; continue 'dispatch;
	}
	// 82F1AFD8: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F1AFDC: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1AFE0: 556B087C  rlwinm r11, r11, 1, 1, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82F1AFE4: 809F0040  lwz r4, 0x40(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F1AFE8: 39000014  li r8, 0x14
	ctx.r[8].s64 = 20;
	// 82F1AFEC: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F1AFF0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F1AFF4: 54E52036  slwi r5, r7, 4
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F1AFF8: 7C68482E  lwzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82F1AFFC: 4BF857B5  bl 0x82ea07b0
	ctx.lr = 0x82F1B000;
	sub_82EA07B0(ctx, base);
	// 82F1B000: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F1B004: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82F1B008: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F1B00C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1B010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1B014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1B018: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1B01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1B020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F1B020 size=228
    let mut pc: u32 = 0x82F1B020;
    'dispatch: loop {
        match pc {
            0x82F1B020 => {
    //   block [0x82F1B020..0x82F1B104)
	// 82F1B020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1B024: 4828D149  bl 0x831a816c
	ctx.lr = 0x82F1B028;
	sub_831A8130(ctx, base);
	// 82F1B028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1B02C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1B030: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1B034: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F1B038: 83BF004C  lwz r29, 0x4c(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82F1B03C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F1B040: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82F1B044: 40980024  bge cr6, 0x82f1b068
	if !ctx.cr[6].lt {
	pc = 0x82F1B068; continue 'dispatch;
	}
	// 82F1B048: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F1B04C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F1B050: 41980008  blt cr6, 0x82f1b058
	if ctx.cr[6].lt {
	pc = 0x82F1B058; continue 'dispatch;
	}
	// 82F1B054: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82F1B058: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82F1B05C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F1B060: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F1B064: 4BF8B795  bl 0x82ea67f8
	ctx.lr = 0x82F1B068;
	sub_82EA67F8(ctx, base);
	// 82F1B068: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82F1B06C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F1B070: 815F004C  lwz r10, 0x4c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82F1B074: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F1B078: 40990084  ble cr6, 0x82f1b0fc
	if !ctx.cr[6].gt {
	pc = 0x82F1B0FC; continue 'dispatch;
	}
	// 82F1B07C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82F1B080: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F1B084: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1B088: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82F1B08C: 7D6A1670  srawi r10, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82F1B090: 80DF0040  lwz r6, 0x40(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F1B094: 556707BE  clrlwi r7, r11, 0x1e
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82F1B098: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B09C: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82F1B0A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F1B0A4: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1B0A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F1B0AC: 7C6A4214  add r3, r10, r8
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82F1B0B0: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F1B0B4: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82F1B0B8: 390A0004  addi r8, r10, 4
	ctx.r[8].s64 = ctx.r[10].s64 + 4;
	// 82F1B0BC: 38EA0008  addi r7, r10, 8
	ctx.r[7].s64 = ctx.r[10].s64 + 8;
	// 82F1B0C0: 5543103A  slwi r3, r10, 2
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82F1B0C4: 550A103A  slwi r10, r8, 2
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F1B0C8: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1B0CC: 7C03342E  lfsx f0, r3, r6
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[6].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1B0D0: 7DAA342E  lfsx f13, r10, r6
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[6].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F1B0D4: 7D88342E  lfsx f12, r8, r6
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F1B0D8: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82F1B0DC: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F1B0E0: D1810058  stfs f12, 0x58(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1B108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F1B108 size=1208
    let mut pc: u32 = 0x82F1B108;
    'dispatch: loop {
        match pc {
            0x82F1B108 => {
    //   block [0x82F1B108..0x82F1B5C0)
	// 82F1B108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1B10C: 4828D039  bl 0x831a8144
	ctx.lr = 0x82F1B110;
	sub_831A8130(ctx, base);
	// 82F1B110: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1B114: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F1B118: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82F1B11C: 397C0003  addi r11, r28, 3
	ctx.r[11].s64 = ctx.r[28].s64 + 3;
	// 82F1B120: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82F1B124: 557B003A  rlwinm r27, r11, 0, 0, 0x1d
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1B128: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F1B12C: 7F691670  srawi r9, r27, 2
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[27].s32 >> 2) as i64;
	// 82F1B130: 939A004C  stw r28, 0x4c(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(76 as u32), ctx.r[28].u32 ) };
	// 82F1B134: 815A0048  lwz r10, 0x48(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F1B138: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82F1B13C: 7FA90194  addze r29, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[29].s64 = tmp.s64;
	// 82F1B140: 3BFA0040  addi r31, r26, 0x40
	ctx.r[31].s64 = ctx.r[26].s64 + 64;
	// 82F1B144: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82F1B148: 40980024  bge cr6, 0x82f1b16c
	if !ctx.cr[6].lt {
	pc = 0x82F1B16C; continue 'dispatch;
	}
	// 82F1B14C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F1B150: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F1B154: 41980008  blt cr6, 0x82f1b15c
	if ctx.cr[6].lt {
	pc = 0x82F1B15C; continue 'dispatch;
	}
	// 82F1B158: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82F1B15C: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 82F1B160: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F1B164: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1B168: 4BF8B691  bl 0x82ea67f8
	ctx.lr = 0x82F1B16C;
	sub_82EA67F8(ctx, base);
	// 82F1B16C: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82F1B170: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 82F1B174: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F1B178: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 82F1B17C: 4198017C  blt cr6, 0x82f1b2f8
	if ctx.cr[6].lt {
	pc = 0x82F1B2F8; continue 'dispatch;
	}
	// 82F1B180: 389CFFFD  addi r4, r28, -3
	ctx.r[4].s64 = ctx.r[28].s64 + -3;
	// 82F1B184: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F1B188: 5567F0BE  srwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1B18C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B190: 5568F87C  rlwinm r8, r11, 0x1f, 1, 0x1e
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82F1B194: C0090000  lfs f0, 0(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1B198: 556607BE  clrlwi r6, r11, 0x1e
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82F1B19C: 7D074214  add r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 82F1B1A0: 5545F0BE  srwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F1B1A4: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1B1A8: 5547F87C  rlwinm r7, r10, 0x1f, 1, 0x1e
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82F1B1AC: 7D083214  add r8, r8, r6
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[6].u64;
	// 82F1B1B0: 7CC53A14  add r6, r5, r7
	ctx.r[6].u64 = ctx.r[5].u64 + ctx.r[7].u64;
	// 82F1B1B4: 5505103A  slwi r5, r8, 2
	ctx.r[5].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F1B1B8: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82F1B1BC: 554707BE  clrlwi r7, r10, 0x1e
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 82F1B1C0: 3BA80004  addi r29, r8, 4
	ctx.r[29].s64 = ctx.r[8].s64 + 4;
	// 82F1B1C4: 7C051D2E  stfsx f0, r5, r3
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 82F1B1C8: 38680008  addi r3, r8, 8
	ctx.r[3].s64 = ctx.r[8].s64 + 8;
	// 82F1B1CC: 7D063A14  add r8, r6, r7
	ctx.r[8].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 82F1B1D0: 38AA0001  addi r5, r10, 1
	ctx.r[5].s64 = ctx.r[10].s64 + 1;
	// 82F1B1D4: 38E80004  addi r7, r8, 4
	ctx.r[7].s64 = ctx.r[8].s64 + 4;
	// 82F1B1D8: 38C80008  addi r6, r8, 8
	ctx.r[6].s64 = ctx.r[8].s64 + 8;
	// 82F1B1DC: 54F7103A  slwi r23, r7, 2
	ctx.r[23].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[23].u64 = ctx.r[23].u32 as u64;
	// 82F1B1E0: 5518103A  slwi r24, r8, 2
	ctx.r[24].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 82F1B1E4: 38EA0001  addi r7, r10, 1
	ctx.r[7].s64 = ctx.r[10].s64 + 1;
	// 82F1B1E8: 54A8F0BE  srwi r8, r5, 2
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shr(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1B1EC: 54D6103A  slwi r22, r6, 2
	ctx.r[22].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[22].u64 = ctx.r[22].u32 as u64;
	// 82F1B1F0: 54E607BE  clrlwi r6, r7, 0x1e
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	// 82F1B1F4: 5507083C  slwi r7, r8, 1
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1B1F8: 57BD103A  slwi r29, r29, 2
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82F1B1FC: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82F1B200: 5463103A  slwi r3, r3, 2
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82F1B204: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1B208: 38AA0002  addi r5, r10, 2
	ctx.r[5].s64 = ctx.r[10].s64 + 2;
	// 82F1B20C: 7D083214  add r8, r8, r6
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[6].u64;
	// 82F1B210: 54A7F0BE  srwi r7, r5, 2
	ctx.r[7].u32 = ctx.r[5].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1B214: 5514103A  slwi r20, r8, 2
	ctx.r[20].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[20].u64 = ctx.r[20].u32 as u64;
	// 82F1B218: 82BF0000  lwz r21, 0(r31)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B21C: C1A90004  lfs f13, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F1B220: 3A680004  addi r19, r8, 4
	ctx.r[19].s64 = ctx.r[8].s64 + 4;
	// 82F1B224: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 82F1B228: 54E5083C  slwi r5, r7, 1
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F1B22C: 38CAFFFE  addi r6, r10, -2
	ctx.r[6].s64 = ctx.r[10].s64 + -2;
	// 82F1B230: 7DBDAD2E  stfsx f13, r29, r21
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[21].u32), tmp.u32) };
	// 82F1B234: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B238: C1890008  lfs f12, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F1B23C: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82F1B240: 7D83ED2E  stfsx f12, r3, r29
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[29].u32), tmp.u32) };
	// 82F1B244: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B248: C1690000  lfs f11, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F1B24C: 551D103A  slwi r29, r8, 2
	ctx.r[29].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82F1B250: 7D781D2E  stfsx f11, r24, r3
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[24].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 82F1B254: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B258: C1490004  lfs f10, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82F1B25C: 7CE72A14  add r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[5].u64;
	// 82F1B260: 7D57452E  stfsx f10, r23, r8
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[23].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82F1B264: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B268: C1290008  lfs f9, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82F1B26C: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82F1B270: 7D362D2E  stfsx f9, r22, r5
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[22].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 82F1B274: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B278: C1090000  lfs f8, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82F1B27C: 54C807BE  clrlwi r8, r6, 0x1e
	ctx.r[8].u64 = ctx.r[6].u32 as u64 & 0x00000003u64;
	// 82F1B280: 7D141D2E  stfsx f8, r20, r3
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[20].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 82F1B284: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B288: 5673103A  slwi r19, r19, 2
	ctx.r[19].u32 = ctx.r[19].u32.wrapping_shl(2);
	ctx.r[19].u64 = ctx.r[19].u32 as u64;
	// 82F1B28C: C0E90004  lfs f7, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82F1B290: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1B294: 7D074214  add r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 82F1B298: 7CF3352E  stfsx f7, r19, r6
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[19].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 82F1B29C: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B2A0: C0C90008  lfs f6, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82F1B2A4: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82F1B2A8: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1B2AC: 7CDD2D2E  stfsx f6, r29, r5
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 82F1B2B0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B2B4: C0A90000  lfs f5, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82F1B2B8: 38C80004  addi r6, r8, 4
	ctx.r[6].s64 = ctx.r[8].s64 + 4;
	// 82F1B2BC: 7CA71D2E  stfsx f5, r7, r3
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 82F1B2C0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B2C4: C0890004  lfs f4, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82F1B2C8: 38A80008  addi r5, r8, 8
	ctx.r[5].s64 = ctx.r[8].s64 + 8;
	// 82F1B2CC: 54C8103A  slwi r8, r6, 2
	ctx.r[8].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1B2D0: 54A7103A  slwi r7, r5, 2
	ctx.r[7].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1B2D4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82F1B2D8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82F1B2DC: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82F1B2E0: 7C881D2E  stfsx f4, r8, r3
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 82F1B2E4: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B2E8: C0690008  lfs f3, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82F1B2EC: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82F1B2F0: 7C67352E  stfsx f3, r7, r6
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 82F1B2F4: 4198FE94  blt cr6, 0x82f1b188
	if ctx.cr[6].lt {
	pc = 0x82F1B188; continue 'dispatch;
	}
	// 82F1B2F8: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82F1B2FC: 40980064  bge cr6, 0x82f1b360
	if !ctx.cr[6].lt {
	pc = 0x82F1B360; continue 'dispatch;
	}
	// 82F1B300: 556AF0BE  srwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F1B304: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B308: 5567F87C  rlwinm r7, r11, 0x1f, 1, 0x1e
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82F1B30C: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1B310: 556807BE  clrlwi r8, r11, 0x1e
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82F1B314: 7CAA3A14  add r5, r10, r7
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82F1B318: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F1B31C: 54AA103A  slwi r10, r5, 2
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F1B320: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82F1B324: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82F1B328: 388A0004  addi r4, r10, 4
	ctx.r[4].s64 = ctx.r[10].s64 + 4;
	// 82F1B32C: 5543103A  slwi r3, r10, 2
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82F1B330: 5488103A  slwi r8, r4, 2
	ctx.r[8].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1B334: 38EA0008  addi r7, r10, 8
	ctx.r[7].s64 = ctx.r[10].s64 + 8;
	// 82F1B338: 54E5103A  slwi r5, r7, 2
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F1B33C: 7C03352E  stfsx f0, r3, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 82F1B340: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B344: C1A90004  lfs f13, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F1B348: 7DA8252E  stfsx f13, r8, r4
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 82F1B34C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B350: C1890008  lfs f12, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F1B354: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82F1B358: 7D851D2E  stfsx f12, r5, r3
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 82F1B35C: 4198FFA4  blt cr6, 0x82f1b300
	if ctx.cr[6].lt {
	pc = 0x82F1B300; continue 'dispatch;
	}
	// 82F1B360: 7D0BD850  subf r8, r11, r27
	ctx.r[8].s64 = ctx.r[27].s64 - ctx.r[11].s64;
	// 82F1B364: 7D5E4850  subf r10, r30, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[30].s64;
	// 82F1B368: 2F080004  cmpwi cr6, r8, 4
	ctx.cr[6].compare_i32(ctx.r[8].s32, 4, &mut ctx.xer);
	// 82F1B36C: 4198016C  blt cr6, 0x82f1b4d8
	if ctx.cr[6].lt {
	pc = 0x82F1B4D8; continue 'dispatch;
	}
	// 82F1B370: 387BFFFD  addi r3, r27, -3
	ctx.r[3].s64 = ctx.r[27].s64 + -3;
	// 82F1B374: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82F1B378: 7D681670  srawi r8, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82F1B37C: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B380: 556407BE  clrlwi r4, r11, 0x1e
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82F1B384: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1B388: 5507083C  slwi r7, r8, 1
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1B38C: 552607BE  clrlwi r6, r9, 0x1e
	ctx.r[6].u64 = ctx.r[9].u32 as u64 & 0x00000003u64;
	// 82F1B390: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82F1B394: 7D271670  srawi r7, r9, 2
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[9].s32 >> 2) as i64;
	// 82F1B398: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1B39C: 54E5083C  slwi r5, r7, 1
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F1B3A0: 7D082214  add r8, r8, r4
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 82F1B3A4: 7CE72A14  add r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[5].u64;
	// 82F1B3A8: 5505103A  slwi r5, r8, 2
	ctx.r[5].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F1B3AC: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1B3B0: 3B080008  addi r24, r8, 8
	ctx.r[24].s64 = ctx.r[8].s64 + 8;
	// 82F1B3B4: 38880004  addi r4, r8, 4
	ctx.r[4].s64 = ctx.r[8].s64 + 4;
	// 82F1B3B8: 7D073214  add r8, r7, r6
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 82F1B3BC: 7C05ED2E  stfsx f0, r5, r29
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[29].u32), tmp.u32) };
	// 82F1B3C0: 38A90001  addi r5, r9, 1
	ctx.r[5].s64 = ctx.r[9].s64 + 1;
	// 82F1B3C4: 38E80004  addi r7, r8, 4
	ctx.r[7].s64 = ctx.r[8].s64 + 4;
	// 82F1B3C8: 571D103A  slwi r29, r24, 2
	ctx.r[29].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82F1B3CC: 54F7103A  slwi r23, r7, 2
	ctx.r[23].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[23].u64 = ctx.r[23].u32 as u64;
	// 82F1B3D0: 38C80008  addi r6, r8, 8
	ctx.r[6].s64 = ctx.r[8].s64 + 8;
	// 82F1B3D4: 5518103A  slwi r24, r8, 2
	ctx.r[24].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 82F1B3D8: 38E90001  addi r7, r9, 1
	ctx.r[7].s64 = ctx.r[9].s64 + 1;
	// 82F1B3DC: 7CA81670  srawi r8, r5, 2
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[5].s32 >> 2) as i64;
	// 82F1B3E0: 54E507BE  clrlwi r5, r7, 0x1e
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	// 82F1B3E4: 5507083C  slwi r7, r8, 1
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1B3E8: 54D6103A  slwi r22, r6, 2
	ctx.r[22].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[22].u64 = ctx.r[22].u32 as u64;
	// 82F1B3EC: 38C90002  addi r6, r9, 2
	ctx.r[6].s64 = ctx.r[9].s64 + 2;
	// 82F1B3F0: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82F1B3F4: 7CC71670  srawi r7, r6, 2
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[6].s32 >> 2) as i64;
	// 82F1B3F8: 5484103A  slwi r4, r4, 2
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82F1B3FC: 54E6083C  slwi r6, r7, 1
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82F1B400: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1B404: 7CC73214  add r6, r7, r6
	ctx.r[6].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 82F1B408: 7D082A14  add r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[5].u64;
	// 82F1B40C: 3A89FFFE  addi r20, r9, -2
	ctx.r[20].s64 = ctx.r[9].s64 + -2;
	// 82F1B410: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B414: C1AA0004  lfs f13, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F1B418: 38A80004  addi r5, r8, 4
	ctx.r[5].s64 = ctx.r[8].s64 + 4;
	// 82F1B41C: 5515103A  slwi r21, r8, 2
	ctx.r[21].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[21].u64 = ctx.r[21].u32 as u64;
	// 82F1B420: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 82F1B424: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F1B428: 7DA43D2E  stfsx f13, r4, r7
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 82F1B42C: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B430: C18A0008  lfs f12, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F1B434: 5504103A  slwi r4, r8, 2
	ctx.r[4].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82F1B438: 7D9D3D2E  stfsx f12, r29, r7
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 82F1B43C: 568707BE  clrlwi r7, r20, 0x1e
	ctx.r[7].u64 = ctx.r[20].u32 as u64 & 0x00000003u64;
	// 82F1B440: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B444: C16A0000  lfs f11, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F1B448: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82F1B44C: 7D78452E  stfsx f11, r24, r8
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[24].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82F1B450: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B454: C14A0004  lfs f10, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82F1B458: 54C8103A  slwi r8, r6, 2
	ctx.r[8].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1B45C: 7D57ED2E  stfsx f10, r23, r29
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[23].u32.wrapping_add(ctx.r[29].u32), tmp.u32) };
	// 82F1B460: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B464: C12A0008  lfs f9, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82F1B468: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82F1B46C: 7D36352E  stfsx f9, r22, r6
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[22].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 82F1B470: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B474: C10A0000  lfs f8, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82F1B478: 5506103A  slwi r6, r8, 2
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82F1B47C: 7D153D2E  stfsx f8, r21, r7
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[21].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 82F1B480: 3BA80004  addi r29, r8, 4
	ctx.r[29].s64 = ctx.r[8].s64 + 4;
	// 82F1B484: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B488: C0EA0004  lfs f7, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82F1B48C: 7CE53D2E  stfsx f7, r5, r7
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 82F1B490: 57A7103A  slwi r7, r29, 2
	ctx.r[7].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1B494: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B498: C0CA0008  lfs f6, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82F1B49C: 7CC42D2E  stfsx f6, r4, r5
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 82F1B4A0: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B4A4: C0AA0000  lfs f5, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82F1B4A8: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 82F1B4AC: 7CA62D2E  stfsx f5, r6, r5
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 82F1B4B0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B4B4: C08A0004  lfs f4, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82F1B4B8: 7C87252E  stfsx f4, r7, r4
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 82F1B4BC: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B4C0: 5506103A  slwi r6, r8, 2
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82F1B4C4: C06A0008  lfs f3, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82F1B4C8: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82F1B4CC: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82F1B4D0: 7C663D2E  stfsx f3, r6, r7
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 82F1B4D4: 4198FEA4  blt cr6, 0x82f1b378
	if ctx.cr[6].lt {
	pc = 0x82F1B378; continue 'dispatch;
	}
	// 82F1B4D8: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82F1B4DC: 40980060  bge cr6, 0x82f1b53c
	if !ctx.cr[6].lt {
	pc = 0x82F1B53C; continue 'dispatch;
	}
	// 82F1B4E0: 7D691670  srawi r9, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82F1B4E4: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B4E8: 556807BE  clrlwi r8, r11, 0x1e
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82F1B4EC: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1B4F0: 5527083C  slwi r7, r9, 1
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1B4F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F1B4F8: 7CA93A14  add r5, r9, r7
	ctx.r[5].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 82F1B4FC: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82F1B500: 54A9103A  slwi r9, r5, 2
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F1B504: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82F1B508: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82F1B50C: 38690004  addi r3, r9, 4
	ctx.r[3].s64 = ctx.r[9].s64 + 4;
	// 82F1B510: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82F1B514: 5468103A  slwi r8, r3, 2
	ctx.r[8].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1B518: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1B51C: 7C04352E  stfsx f0, r4, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 82F1B520: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B524: C1AA0004  lfs f13, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F1B528: 7DA8352E  stfsx f13, r8, r6
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 82F1B52C: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B530: C18A0008  lfs f12, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F1B534: 7D872D2E  stfsx f12, r7, r5
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 82F1B538: 4198FFA8  blt cr6, 0x82f1b4e0
	if ctx.cr[6].lt {
	pc = 0x82F1B4E0; continue 'dispatch;
	}
	// 82F1B53C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82F1B540: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F1B544: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F1B548: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F1B54C: 4800AE5D  bl 0x82f263a8
	ctx.lr = 0x82F1B550;
	sub_82F263A8(ctx, base);
	// 82F1B550: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F1B554: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82F1B558: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82F1B55C: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82F1B560: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F1B564: C00B9450  lfs f0, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1B568: 397A0030  addi r11, r26, 0x30
	ctx.r[11].s64 = ctx.r[26].s64 + 48;
	// 82F1B56C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82F1B570: 395A0020  addi r10, r26, 0x20
	ctx.r[10].s64 = ctx.r[26].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1B5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1B5C0 size=32
    let mut pc: u32 = 0x82F1B5C0;
    'dispatch: loop {
        match pc {
            0x82F1B5C0 => {
    //   block [0x82F1B5C0..0x82F1B5E0)
	// 82F1B5C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82F1B5C4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F1B5C8: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82F1B5CC: 38AB0030  addi r5, r11, 0x30
	ctx.r[5].s64 = ctx.r[11].s64 + 48;
	// 82F1B5D0: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 82F1B5D4: C00B0010  lfs f0, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1B5D8: EC20082A  fadds f1, f0, f1
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 82F1B5DC: 4BFFE7FC  b 0x82f19dd8
	sub_82F19DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1B5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1B5E0 size=436
    let mut pc: u32 = 0x82F1B5E0;
    'dispatch: loop {
        match pc {
            0x82F1B5E0 => {
    //   block [0x82F1B5E0..0x82F1B794)
	// 82F1B5E0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82F1B5E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B5E8: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82F1B5EC: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F1B5F0: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1B5F4: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F1B5F8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1B5FC: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F1B600: 40980020  bge cr6, 0x82f1b620
	if !ctx.cr[6].lt {
	pc = 0x82F1B620; continue 'dispatch;
	}
	// 82F1B604: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F1B608: 38E8BF84  addi r7, r8, -0x407c
	ctx.r[7].s64 = ctx.r[8].s64 + -16508;
	// 82F1B60C: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F1B610: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F1B614: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82F1B618: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F1B61C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82F1B620: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82F1B624: 81040054  lwz r8, 0x54(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F1B628: 3FE08201  lis r31, -0x7dff
	ctx.r[31].s64 = -2113863680;
	// 82F1B62C: C1660010  lfs f11, 0x10(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F1B630: 3568FFFF  addic. r11, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F1B634: 38E60010  addi r7, r6, 0x10
	ctx.r[7].s64 = ctx.r[6].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1B798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1B798 size=232
    let mut pc: u32 = 0x82F1B798;
    'dispatch: loop {
        match pc {
            0x82F1B798 => {
    //   block [0x82F1B798..0x82F1B880)
	// 82F1B798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1B79C: 4828C9D1  bl 0x831a816c
	ctx.lr = 0x82F1B7A0;
	sub_831A8130(ctx, base);
	// 82F1B7A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1B7A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1B7A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1B7AC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1B7B0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F1B7B4: 388ABF9C  addi r4, r10, -0x4064
	ctx.r[4].s64 = ctx.r[10].s64 + -16484;
	// 82F1B7B8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B7BC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F1B7C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F1B7C4: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1B7C8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F1B7CC: 4E800421  bctrl
	ctx.lr = 0x82F1B7D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F1B7D0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1B7D4: 3BABBF94  addi r29, r11, -0x406c
	ctx.r[29].s64 = ctx.r[11].s64 + -16492;
	// 82F1B7D8: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F1B7DC: 55680000  rlwinm r8, r11, 0, 0, 0
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1B7E0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F1B7E4: 409A0044  bne cr6, 0x82f1b828
	if !ctx.cr[6].eq {
	pc = 0x82F1B828; continue 'dispatch;
	}
	// 82F1B7E8: 815F0044  lwz r10, 0x44(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F1B7EC: 556800BE  clrlwi r8, r11, 2
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F1B7F0: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B7F4: 5569087C  rlwinm r9, r11, 1, 1, 0x1e
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82F1B7F8: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F1B7FC: 80DF0040  lwz r6, 0x40(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F1B800: 7CA84A14  add r5, r8, r9
	ctx.r[5].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82F1B804: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F1B808: 54A82036  slwi r8, r5, 4
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1B80C: 81670008  lwz r11, 8(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F1B810: 54672036  slwi r7, r3, 4
	ctx.r[7].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1B814: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F1B818: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F1B81C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F1B820: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F1B824: 4E800421  bctrl
	ctx.lr = 0x82F1B828;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F1B828: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F1B82C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1B830: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F1B834: 409A0030  bne cr6, 0x82f1b864
	if !ctx.cr[6].eq {
	pc = 0x82F1B864; continue 'dispatch;
	}
	// 82F1B838: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B83C: 55682036  slwi r8, r11, 4
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1B840: 813F0054  lwz r9, 0x54(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F1B844: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F1B848: 80DF0050  lwz r6, 0x50(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F1B84C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F1B850: 55272036  slwi r7, r9, 4
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F1B854: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F1B858: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F1B85C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F1B860: 4E800421  bctrl
	ctx.lr = 0x82F1B864;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F1B864: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B868: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F1B86C: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1B870: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1B874: 4E800421  bctrl
	ctx.lr = 0x82F1B878;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F1B878: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1B87C: 4828C940  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1B880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F1B880 size=292
    let mut pc: u32 = 0x82F1B880;
    'dispatch: loop {
        match pc {
            0x82F1B880 => {
    //   block [0x82F1B880..0x82F1B9A4)
	// 82F1B880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1B884: 4828C8E5  bl 0x831a8168
	ctx.lr = 0x82F1B888;
	sub_831A8130(ctx, base);
	// 82F1B888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1B88C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1B890: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1B894: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F1B898: 392ABF44  addi r9, r10, -0x40bc
	ctx.r[9].s64 = ctx.r[10].s64 + -16572;
	// 82F1B89C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F1B8A0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82F1B8A4: D03F0010  stfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F1B8A8: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82F1B8AC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82F1B8B0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F1B8B4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1B8B8: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F1B8BC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F1B8C0: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82F1B8C4: 3BDF0050  addi r30, r31, 0x50
	ctx.r[30].s64 = ctx.r[31].s64 + 80;
	// 82F1B8C8: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82F1B8CC: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82F1B8D0: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82F1B8D4: 915F0058  stw r10, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82F1B8D8: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82F1B8DC: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82F1B8E0: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82F1B8E4: 80DD0004  lwz r6, 4(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1B8E8: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F1B8EC: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F1B8F0: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82F1B8F4: 40980060  bge cr6, 0x82f1b954
	if !ctx.cr[6].lt {
	pc = 0x82F1B954; continue 'dispatch;
	}
	// 82F1B8F8: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1B8FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F1B900: 409A0020  bne cr6, 0x82f1b920
	if !ctx.cr[6].eq {
	pc = 0x82F1B920; continue 'dispatch;
	}
	// 82F1B904: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B908: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F1B90C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F1B910: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B914: 55452036  slwi r5, r10, 4
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F1B918: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1B91C: 4BF84E95  bl 0x82ea07b0
	ctx.lr = 0x82F1B920;
	sub_82EA07B0(ctx, base);
	// 82F1B920: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B924: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1B928: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1B92C: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82F1B930: 55242036  slwi r4, r9, 4
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82F1B934: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1B938: 4BF84DF9  bl 0x82ea0730
	ctx.lr = 0x82F1B93C;
	sub_82EA0730(ctx, base);
	// 82F1B93C: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F1B940: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82F1B944: 55070042  rlwinm r7, r8, 0, 1, 1
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1B948: 80DD0004  lwz r6, 4(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1B94C: 7CE53378  or r5, r7, r6
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 82F1B950: 90BE0008  stw r5, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82F1B954: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1B958: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B95C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F1B960: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82F1B964: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B968: 4099001C  ble cr6, 0x82f1b984
	if !ctx.cr[6].gt {
	pc = 0x82F1B984; continue 'dispatch;
	}
	// 82F1B96C: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1B9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1B9A8 size=100
    let mut pc: u32 = 0x82F1B9A8;
    'dispatch: loop {
        match pc {
            0x82F1B9A8 => {
    //   block [0x82F1B9A8..0x82F1BA0C)
	// 82F1B9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1B9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1B9B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1B9B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1B9B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1B9BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1B9C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1B9C4: 4BFFF575  bl 0x82f1af38
	ctx.lr = 0x82F1B9C8;
	sub_82F1AF38(ctx, base);
	// 82F1B9C8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F1B9CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1B9D0: 419A0020  beq cr6, 0x82f1b9f0
	if ctx.cr[6].eq {
	pc = 0x82F1B9F0; continue 'dispatch;
	}
	// 82F1B9D4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1B9D8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1B9DC: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F1B9E0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1B9E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F1B9E8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1B9EC: 4BF84DC5  bl 0x82ea07b0
	ctx.lr = 0x82F1B9F0;
	sub_82EA07B0(ctx, base);
	// 82F1B9F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1B9F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1B9F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1B9FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1BA00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1BA04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1BA08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BA10 size=20
    let mut pc: u32 = 0x82F1BA10;
    'dispatch: loop {
        match pc {
            0x82F1BA10 => {
    //   block [0x82F1BA10..0x82F1BA24)
	// 82F1BA10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BA14: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1BA18: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BA1C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1BA20: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BA28 size=8
    let mut pc: u32 = 0x82F1BA28;
    'dispatch: loop {
        match pc {
            0x82F1BA28 => {
    //   block [0x82F1BA28..0x82F1BA30)
	// 82F1BA28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1BA2C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BA30 size=16
    let mut pc: u32 = 0x82F1BA30;
    'dispatch: loop {
        match pc {
            0x82F1BA30 => {
    //   block [0x82F1BA30..0x82F1BA40)
	// 82F1BA30: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1BA34: 394BBD10  addi r10, r11, -0x42f0
	ctx.r[10].s64 = ctx.r[11].s64 + -17136;
	// 82F1BA38: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F1BA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BA40 size=12
    let mut pc: u32 = 0x82F1BA40;
    'dispatch: loop {
        match pc {
            0x82F1BA40 => {
    //   block [0x82F1BA40..0x82F1BA4C)
	// 82F1BA40: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1BA44: 386BBD10  addi r3, r11, -0x42f0
	ctx.r[3].s64 = ctx.r[11].s64 + -17136;
	// 82F1BA48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BA50 size=20
    let mut pc: u32 = 0x82F1BA50;
    'dispatch: loop {
        match pc {
            0x82F1BA50 => {
    //   block [0x82F1BA50..0x82F1BA64)
	// 82F1BA50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BA54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1BA58: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BA5C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1BA60: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BA68 size=8
    let mut pc: u32 = 0x82F1BA68;
    'dispatch: loop {
        match pc {
            0x82F1BA68 => {
    //   block [0x82F1BA68..0x82F1BA70)
	// 82F1BA68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1BA6C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BA70 size=80
    let mut pc: u32 = 0x82F1BA70;
    'dispatch: loop {
        match pc {
            0x82F1BA70 => {
    //   block [0x82F1BA70..0x82F1BAC0)
	// 82F1BA70: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1BA74: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1BA78: 392BBD68  addi r9, r11, -0x4298
	ctx.r[9].s64 = ctx.r[11].s64 + -17048;
	// 82F1BA7C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F1BA80: 38EABD10  addi r7, r10, -0x42f0
	ctx.r[7].s64 = ctx.r[10].s64 + -17136;
	// 82F1BA84: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1BA88: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F1BA8C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82F1BA90: 90E30030  stw r7, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82F1BA94: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82F1BA98: 80830010  lwz r4, 0x10(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F1BA9C: 3960000B  li r11, 0xb
	ctx.r[11].s64 = 11;
	// 82F1BAA0: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BAC0 size=12
    let mut pc: u32 = 0x82F1BAC0;
    'dispatch: loop {
        match pc {
            0x82F1BAC0 => {
    //   block [0x82F1BAC0..0x82F1BACC)
	// 82F1BAC0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1BAC4: 386BBD68  addi r3, r11, -0x4298
	ctx.r[3].s64 = ctx.r[11].s64 + -17048;
	// 82F1BAC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BAD0 size=4
    let mut pc: u32 = 0x82F1BAD0;
    'dispatch: loop {
        match pc {
            0x82F1BAD0 => {
    //   block [0x82F1BAD0..0x82F1BAD4)
	// 82F1BAD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BAD8 size=4
    let mut pc: u32 = 0x82F1BAD8;
    'dispatch: loop {
        match pc {
            0x82F1BAD8 => {
    //   block [0x82F1BAD8..0x82F1BADC)
	// 82F1BAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BAE0 size=4
    let mut pc: u32 = 0x82F1BAE0;
    'dispatch: loop {
        match pc {
            0x82F1BAE0 => {
    //   block [0x82F1BAE0..0x82F1BAE4)
	// 82F1BAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BAE8 size=20
    let mut pc: u32 = 0x82F1BAE8;
    'dispatch: loop {
        match pc {
            0x82F1BAE8 => {
    //   block [0x82F1BAE8..0x82F1BAFC)
	// 82F1BAE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BAEC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1BAF0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BAF4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1BAF8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BB00 size=8
    let mut pc: u32 = 0x82F1BB00;
    'dispatch: loop {
        match pc {
            0x82F1BB00 => {
    //   block [0x82F1BB00..0x82F1BB08)
	// 82F1BB00: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1BB04: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BB08 size=56
    let mut pc: u32 = 0x82F1BB08;
    'dispatch: loop {
        match pc {
            0x82F1BB08 => {
    //   block [0x82F1BB08..0x82F1BB40)
	// 82F1BB08: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1BB0C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1BB10: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F1BB14: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F1BB18: 38EBBCD8  addi r7, r11, -0x4328
	ctx.r[7].s64 = ctx.r[11].s64 + -17192;
	// 82F1BB1C: 38CAC908  addi r6, r10, -0x36f8
	ctx.r[6].s64 = ctx.r[10].s64 + -14072;
	// 82F1BB20: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F1BB24: 38A9C8E4  addi r5, r9, -0x371c
	ctx.r[5].s64 = ctx.r[9].s64 + -14108;
	// 82F1BB28: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82F1BB2C: 38800016  li r4, 0x16
	ctx.r[4].s64 = 22;
	// 82F1BB30: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F1BB34: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82F1BB38: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F1BB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BB40 size=12
    let mut pc: u32 = 0x82F1BB40;
    'dispatch: loop {
        match pc {
            0x82F1BB40 => {
    //   block [0x82F1BB40..0x82F1BB4C)
	// 82F1BB40: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1BB44: 386BC908  addi r3, r11, -0x36f8
	ctx.r[3].s64 = ctx.r[11].s64 + -14072;
	// 82F1BB48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BB50 size=8
    let mut pc: u32 = 0x82F1BB50;
    'dispatch: loop {
        match pc {
            0x82F1BB50 => {
    //   block [0x82F1BB50..0x82F1BB58)
	// 82F1BB50: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82F1BB54: 480000DC  b 0x82f1bc30
	sub_82F1BC30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1BB58 size=212
    let mut pc: u32 = 0x82F1BB58;
    'dispatch: loop {
        match pc {
            0x82F1BB58 => {
    //   block [0x82F1BB58..0x82F1BC2C)
	// 82F1BB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1BB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1BB60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1BB64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1BB68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1BB6C: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82F1BB70: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1BB74: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F1BB78: 409A0020  bne cr6, 0x82f1bb98
	if !ctx.cr[6].eq {
	pc = 0x82F1BB98; continue 'dispatch;
	}
	// 82F1BB7C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BB80: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F1BB84: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F1BB88: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82F1BB8C: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F1BB90: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F1BB94: 4BF84C1D  bl 0x82ea07b0
	ctx.lr = 0x82F1BB98;
	sub_82EA07B0(ctx, base);
	// 82F1BB98: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F1BB9C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1BBA0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F1BBA4: 409A0020  bne cr6, 0x82f1bbc4
	if !ctx.cr[6].eq {
	pc = 0x82F1BBC4; continue 'dispatch;
	}
	// 82F1BBA8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BBAC: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F1BBB0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F1BBB4: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F1BBB8: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F1BBBC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F1BBC0: 4BF84BF1  bl 0x82ea07b0
	ctx.lr = 0x82F1BBC4;
	sub_82EA07B0(ctx, base);
	// 82F1BBC4: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F1BBC8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1BBCC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F1BBD0: 409A0020  bne cr6, 0x82f1bbf0
	if !ctx.cr[6].eq {
	pc = 0x82F1BBF0; continue 'dispatch;
	}
	// 82F1BBD4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BBD8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F1BBDC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F1BBE0: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1BBE4: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F1BBE8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F1BBEC: 4BF84BC5  bl 0x82ea07b0
	ctx.lr = 0x82F1BBF0;
	sub_82EA07B0(ctx, base);
	// 82F1BBF0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F1BBF4: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82F1BBF8: 409A0008  bne cr6, 0x82f1bc00
	if !ctx.cr[6].eq {
	pc = 0x82F1BC00; continue 'dispatch;
	}
	// 82F1BBFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F1BC00: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1BC04: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F1BC08: 390ABCD8  addi r8, r10, -0x4328
	ctx.r[8].s64 = ctx.r[10].s64 + -17192;
	// 82F1BC0C: 38E99EAC  addi r7, r9, -0x6154
	ctx.r[7].s64 = ctx.r[9].s64 + -24916;
	// 82F1BC10: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F1BC14: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F1BC18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1BC1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1BC20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1BC24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1BC28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1BC30 size=100
    let mut pc: u32 = 0x82F1BC30;
    'dispatch: loop {
        match pc {
            0x82F1BC30 => {
    //   block [0x82F1BC30..0x82F1BC94)
	// 82F1BC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1BC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1BC38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1BC3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1BC40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1BC44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1BC48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1BC4C: 4BFFFF0D  bl 0x82f1bb58
	ctx.lr = 0x82F1BC50;
	sub_82F1BB58(ctx, base);
	// 82F1BC50: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F1BC54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1BC58: 419A0020  beq cr6, 0x82f1bc78
	if ctx.cr[6].eq {
	pc = 0x82F1BC78; continue 'dispatch;
	}
	// 82F1BC5C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BC60: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1BC64: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F1BC68: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1BC6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F1BC70: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1BC74: 4BF84B3D  bl 0x82ea07b0
	ctx.lr = 0x82F1BC78;
	sub_82EA07B0(ctx, base);
	// 82F1BC78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1BC7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1BC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1BC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1BC88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1BC8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1BC90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BC98 size=4
    let mut pc: u32 = 0x82F1BC98;
    'dispatch: loop {
        match pc {
            0x82F1BC98 => {
    //   block [0x82F1BC98..0x82F1BC9C)
	// 82F1BC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BCA0 size=4
    let mut pc: u32 = 0x82F1BCA0;
    'dispatch: loop {
        match pc {
            0x82F1BCA0 => {
    //   block [0x82F1BCA0..0x82F1BCA4)
	// 82F1BCA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BCA8 size=20
    let mut pc: u32 = 0x82F1BCA8;
    'dispatch: loop {
        match pc {
            0x82F1BCA8 => {
    //   block [0x82F1BCA8..0x82F1BCBC)
	// 82F1BCA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BCAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1BCB0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BCB4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1BCB8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BCC0 size=8
    let mut pc: u32 = 0x82F1BCC0;
    'dispatch: loop {
        match pc {
            0x82F1BCC0 => {
    //   block [0x82F1BCC0..0x82F1BCC8)
	// 82F1BCC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1BCC4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BCC8 size=32
    let mut pc: u32 = 0x82F1BCC8;
    'dispatch: loop {
        match pc {
            0x82F1BCC8 => {
    //   block [0x82F1BCC8..0x82F1BCE8)
	// 82F1BCC8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1BCCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F1BCD0: 392BCA84  addi r9, r11, -0x357c
	ctx.r[9].s64 = ctx.r[11].s64 + -13692;
	// 82F1BCD4: 39000017  li r8, 0x17
	ctx.r[8].s64 = 23;
	// 82F1BCD8: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F1BCDC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1BCE0: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82F1BCE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BCE8 size=12
    let mut pc: u32 = 0x82F1BCE8;
    'dispatch: loop {
        match pc {
            0x82F1BCE8 => {
    //   block [0x82F1BCE8..0x82F1BCF4)
	// 82F1BCE8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1BCEC: 386BCA84  addi r3, r11, -0x357c
	ctx.r[3].s64 = ctx.r[11].s64 + -13692;
	// 82F1BCF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1BCF8 size=152
    let mut pc: u32 = 0x82F1BCF8;
    'dispatch: loop {
        match pc {
            0x82F1BCF8 => {
    //   block [0x82F1BCF8..0x82F1BD90)
	// 82F1BCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1BCFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1BD00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1BD04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1BD08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1BD0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1BD10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1BD14: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1BD18: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1BD1C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F1BD20: 409A0020  bne cr6, 0x82f1bd40
	if !ctx.cr[6].eq {
	pc = 0x82F1BD40; continue 'dispatch;
	}
	// 82F1BD24: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BD28: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F1BD2C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F1BD30: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F1BD34: 55652834  slwi r5, r11, 5
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F1BD38: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F1BD3C: 4BF84A75  bl 0x82ea07b0
	ctx.lr = 0x82F1BD40;
	sub_82EA07B0(ctx, base);
	// 82F1BD40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F1BD44: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F1BD48: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82F1BD4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F1BD50: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1BD54: 419A0020  beq cr6, 0x82f1bd74
	if ctx.cr[6].eq {
	pc = 0x82F1BD74; continue 'dispatch;
	}
	// 82F1BD58: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BD5C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1BD60: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F1BD64: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1BD68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F1BD6C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1BD70: 4BF84A41  bl 0x82ea07b0
	ctx.lr = 0x82F1BD74;
	sub_82EA07B0(ctx, base);
	// 82F1BD74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1BD78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1BD7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1BD80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1BD84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1BD88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1BD8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BD90 size=4
    let mut pc: u32 = 0x82F1BD90;
    'dispatch: loop {
        match pc {
            0x82F1BD90 => {
    //   block [0x82F1BD90..0x82F1BD94)
	// 82F1BD90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BD98 size=12
    let mut pc: u32 = 0x82F1BD98;
    'dispatch: loop {
        match pc {
            0x82F1BD98 => {
    //   block [0x82F1BD98..0x82F1BDA4)
	// 82F1BD98: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F1BD9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1BDA0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BDA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BDA4 size=8
    let mut pc: u32 = 0x82F1BDA4;
    'dispatch: loop {
        match pc {
            0x82F1BDA4 => {
    //   block [0x82F1BDA4..0x82F1BDAC)
	// 82F1BDA4: 4800E444  b 0x82f2a1e8
	sub_82F2A1E8(ctx, base);
	return;
	// 82F1BDA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BDB0 size=20
    let mut pc: u32 = 0x82F1BDB0;
    'dispatch: loop {
        match pc {
            0x82F1BDB0 => {
    //   block [0x82F1BDB0..0x82F1BDC4)
	// 82F1BDB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BDB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1BDB8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BDBC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1BDC0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1BDC8 size=44
    let mut pc: u32 = 0x82F1BDC8;
    'dispatch: loop {
        match pc {
            0x82F1BDC8 => {
    //   block [0x82F1BDC8..0x82F1BDF4)
	// 82F1BDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1BDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1BDD0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1BDD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1BDD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F1BDDC: 4800E40D  bl 0x82f2a1e8
	ctx.lr = 0x82F1BDE0;
	sub_82F2A1E8(ctx, base);
	// 82F1BDE0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F1BDE4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82F1BDE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1BDEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1BDF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BDF8 size=4
    let mut pc: u32 = 0x82F1BDF8;
    'dispatch: loop {
        match pc {
            0x82F1BDF8 => {
    //   block [0x82F1BDF8..0x82F1BDFC)
	// 82F1BDF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BE00 size=4
    let mut pc: u32 = 0x82F1BE00;
    'dispatch: loop {
        match pc {
            0x82F1BE00 => {
    //   block [0x82F1BE00..0x82F1BE04)
	// 82F1BE00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BE08 size=20
    let mut pc: u32 = 0x82F1BE08;
    'dispatch: loop {
        match pc {
            0x82F1BE08 => {
    //   block [0x82F1BE08..0x82F1BE1C)
	// 82F1BE08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BE0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1BE10: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BE14: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1BE18: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BE20 size=12
    let mut pc: u32 = 0x82F1BE20;
    'dispatch: loop {
        match pc {
            0x82F1BE20 => {
    //   block [0x82F1BE20..0x82F1BE2C)
	// 82F1BE20: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F1BE24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1BE28: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BE2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BE2C size=8
    let mut pc: u32 = 0x82F1BE2C;
    'dispatch: loop {
        match pc {
            0x82F1BE2C => {
    //   block [0x82F1BE2C..0x82F1BE34)
	// 82F1BE2C: 4800EBAC  b 0x82f2a9d8
	sub_82F2A9D8(ctx, base);
	return;
	// 82F1BE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1BE38 size=44
    let mut pc: u32 = 0x82F1BE38;
    'dispatch: loop {
        match pc {
            0x82F1BE38 => {
    //   block [0x82F1BE38..0x82F1BE64)
	// 82F1BE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1BE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1BE40: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1BE44: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1BE48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F1BE4C: 4800EB8D  bl 0x82f2a9d8
	ctx.lr = 0x82F1BE50;
	sub_82F2A9D8(ctx, base);
	// 82F1BE50: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F1BE54: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82F1BE58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1BE5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1BE60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BE68 size=4
    let mut pc: u32 = 0x82F1BE68;
    'dispatch: loop {
        match pc {
            0x82F1BE68 => {
    //   block [0x82F1BE68..0x82F1BE6C)
	// 82F1BE68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BE70 size=4
    let mut pc: u32 = 0x82F1BE70;
    'dispatch: loop {
        match pc {
            0x82F1BE70 => {
    //   block [0x82F1BE70..0x82F1BE74)
	// 82F1BE70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BE78 size=12
    let mut pc: u32 = 0x82F1BE78;
    'dispatch: loop {
        match pc {
            0x82F1BE78 => {
    //   block [0x82F1BE78..0x82F1BE84)
	// 82F1BE78: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F1BE7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1BE80: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BE84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BE84 size=8
    let mut pc: u32 = 0x82F1BE84;
    'dispatch: loop {
        match pc {
            0x82F1BE84 => {
    //   block [0x82F1BE84..0x82F1BE8C)
	// 82F1BE84: 4BFFD114  b 0x82f18f98
	sub_82F18F98(ctx, base);
	return;
	// 82F1BE88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BE90 size=20
    let mut pc: u32 = 0x82F1BE90;
    'dispatch: loop {
        match pc {
            0x82F1BE90 => {
    //   block [0x82F1BE90..0x82F1BEA4)
	// 82F1BE90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BE94: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1BE98: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BE9C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1BEA0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1BEA8 size=44
    let mut pc: u32 = 0x82F1BEA8;
    'dispatch: loop {
        match pc {
            0x82F1BEA8 => {
    //   block [0x82F1BEA8..0x82F1BED4)
	// 82F1BEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1BEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1BEB0: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1BEB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1BEB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F1BEBC: 4BFFD0DD  bl 0x82f18f98
	ctx.lr = 0x82F1BEC0;
	sub_82F18F98(ctx, base);
	// 82F1BEC0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F1BEC4: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82F1BEC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1BECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1BED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BED8 size=4
    let mut pc: u32 = 0x82F1BED8;
    'dispatch: loop {
        match pc {
            0x82F1BED8 => {
    //   block [0x82F1BED8..0x82F1BEDC)
	// 82F1BED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BEE0 size=4
    let mut pc: u32 = 0x82F1BEE0;
    'dispatch: loop {
        match pc {
            0x82F1BEE0 => {
    //   block [0x82F1BEE0..0x82F1BEE4)
	// 82F1BEE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BEE8 size=4
    let mut pc: u32 = 0x82F1BEE8;
    'dispatch: loop {
        match pc {
            0x82F1BEE8 => {
    //   block [0x82F1BEE8..0x82F1BEEC)
	// 82F1BEE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BEF0 size=8
    let mut pc: u32 = 0x82F1BEF0;
    'dispatch: loop {
        match pc {
            0x82F1BEF0 => {
    //   block [0x82F1BEF0..0x82F1BEF8)
	// 82F1BEF0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1BEF4: 48000004  b 0x82f1bef8
	sub_82F1BEF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1BEF8 size=160
    let mut pc: u32 = 0x82F1BEF8;
    'dispatch: loop {
        match pc {
            0x82F1BEF8 => {
    //   block [0x82F1BEF8..0x82F1BF98)
	// 82F1BEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1BEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1BF00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1BF04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1BF08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1BF0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1BF10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1BF14: 4BFFC99D  bl 0x82f188b0
	ctx.lr = 0x82F1BF18;
	sub_82F188B0(ctx, base);
	// 82F1BF18: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F1BF1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1BF20: 419A005C  beq cr6, 0x82f1bf7c
	if ctx.cr[6].eq {
	pc = 0x82F1BF7C; continue 'dispatch;
	}
	// 82F1BF24: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F1BF28: 419A0054  beq cr6, 0x82f1bf7c
	if ctx.cr[6].eq {
	pc = 0x82F1BF7C; continue 'dispatch;
	}
	// 82F1BF2C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BF30: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1BF34: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1BF38: 812B0064  lwz r9, 0x64(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82F1BF3C: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F1BF40: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F1BF44: 4198001C  blt cr6, 0x82f1bf60
	if ctx.cr[6].lt {
	pc = 0x82F1BF60; continue 'dispatch;
	}
	// 82F1BF48: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82F1BF4C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F1BF50: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82F1BF54: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F1BF58: 4BF84109  bl 0x82ea0060
	ctx.lr = 0x82F1BF5C;
	sub_82EA0060(ctx, base);
	// 82F1BF5C: 48000020  b 0x82f1bf7c
	pc = 0x82F1BF7C; continue 'dispatch;
	// 82F1BF60: 812B0064  lwz r9, 0x64(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82F1BF64: 394B0060  addi r10, r11, 0x60
	ctx.r[10].s64 = ctx.r[11].s64 + 96;
	// 82F1BF68: 814B0060  lwz r10, 0x60(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F1BF6C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82F1BF70: 912B0064  stw r9, 0x64(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82F1BF74: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F1BF78: 93EB0060  stw r31, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82F1BF7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1BF80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1BF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1BF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1BF8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1BF90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1BF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BF98 size=4
    let mut pc: u32 = 0x82F1BF98;
    'dispatch: loop {
        match pc {
            0x82F1BF98 => {
    //   block [0x82F1BF98..0x82F1BF9C)
	// 82F1BF98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BFA0 size=20
    let mut pc: u32 = 0x82F1BFA0;
    'dispatch: loop {
        match pc {
            0x82F1BFA0 => {
    //   block [0x82F1BFA0..0x82F1BFB4)
	// 82F1BFA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BFA4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1BFA8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1BFAC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1BFB0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BFB8 size=8
    let mut pc: u32 = 0x82F1BFB8;
    'dispatch: loop {
        match pc {
            0x82F1BFB8 => {
    //   block [0x82F1BFB8..0x82F1BFC0)
	// 82F1BFB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1BFBC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BFC0 size=32
    let mut pc: u32 = 0x82F1BFC0;
    'dispatch: loop {
        match pc {
            0x82F1BFC0 => {
    //   block [0x82F1BFC0..0x82F1BFE0)
	// 82F1BFC0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1BFC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F1BFC8: 392BBF44  addi r9, r11, -0x40bc
	ctx.r[9].s64 = ctx.r[11].s64 + -16572;
	// 82F1BFCC: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82F1BFD0: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F1BFD4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1BFD8: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82F1BFDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BFE0 size=12
    let mut pc: u32 = 0x82F1BFE0;
    'dispatch: loop {
        match pc {
            0x82F1BFE0 => {
    //   block [0x82F1BFE0..0x82F1BFEC)
	// 82F1BFE0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1BFE4: 386BBF44  addi r3, r11, -0x40bc
	ctx.r[3].s64 = ctx.r[11].s64 + -16572;
	// 82F1BFE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BFF0 size=4
    let mut pc: u32 = 0x82F1BFF0;
    'dispatch: loop {
        match pc {
            0x82F1BFF0 => {
    //   block [0x82F1BFF0..0x82F1BFF4)
	// 82F1BFF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1BFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1BFF8 size=4
    let mut pc: u32 = 0x82F1BFF8;
    'dispatch: loop {
        match pc {
            0x82F1BFF8 => {
    //   block [0x82F1BFF8..0x82F1BFFC)
	// 82F1BFF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C000 size=4
    let mut pc: u32 = 0x82F1C000;
    'dispatch: loop {
        match pc {
            0x82F1C000 => {
    //   block [0x82F1C000..0x82F1C004)
	// 82F1C000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C008 size=20
    let mut pc: u32 = 0x82F1C008;
    'dispatch: loop {
        match pc {
            0x82F1C008 => {
    //   block [0x82F1C008..0x82F1C01C)
	// 82F1C008: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C00C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1C010: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C014: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1C018: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C020 size=8
    let mut pc: u32 = 0x82F1C020;
    'dispatch: loop {
        match pc {
            0x82F1C020 => {
    //   block [0x82F1C020..0x82F1C028)
	// 82F1C020: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1C024: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C028 size=32
    let mut pc: u32 = 0x82F1C028;
    'dispatch: loop {
        match pc {
            0x82F1C028 => {
    //   block [0x82F1C028..0x82F1C048)
	// 82F1C028: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82F1C02C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F1C030: 392B6C64  addi r9, r11, 0x6c64
	ctx.r[9].s64 = ctx.r[11].s64 + 27748;
	// 82F1C034: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82F1C038: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F1C03C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1C040: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82F1C044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C048 size=12
    let mut pc: u32 = 0x82F1C048;
    'dispatch: loop {
        match pc {
            0x82F1C048 => {
    //   block [0x82F1C048..0x82F1C054)
	// 82F1C048: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82F1C04C: 386B6C64  addi r3, r11, 0x6c64
	ctx.r[3].s64 = ctx.r[11].s64 + 27748;
	// 82F1C050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C058 size=20
    let mut pc: u32 = 0x82F1C058;
    'dispatch: loop {
        match pc {
            0x82F1C058 => {
    //   block [0x82F1C058..0x82F1C06C)
	// 82F1C058: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C05C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1C060: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C064: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1C068: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C070 size=8
    let mut pc: u32 = 0x82F1C070;
    'dispatch: loop {
        match pc {
            0x82F1C070 => {
    //   block [0x82F1C070..0x82F1C078)
	// 82F1C070: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1C074: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C078 size=56
    let mut pc: u32 = 0x82F1C078;
    'dispatch: loop {
        match pc {
            0x82F1C078 => {
    //   block [0x82F1C078..0x82F1C0B0)
	// 82F1C078: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1C07C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1C080: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F1C084: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F1C088: 38EBBCD8  addi r7, r11, -0x4328
	ctx.r[7].s64 = ctx.r[11].s64 + -17192;
	// 82F1C08C: 38CAD728  addi r6, r10, -0x28d8
	ctx.r[6].s64 = ctx.r[10].s64 + -10456;
	// 82F1C090: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F1C094: 38A9D704  addi r5, r9, -0x28fc
	ctx.r[5].s64 = ctx.r[9].s64 + -10492;
	// 82F1C098: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82F1C09C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82F1C0A0: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F1C0A4: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82F1C0A8: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F1C0AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C0B0 size=12
    let mut pc: u32 = 0x82F1C0B0;
    'dispatch: loop {
        match pc {
            0x82F1C0B0 => {
    //   block [0x82F1C0B0..0x82F1C0BC)
	// 82F1C0B0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1C0B4: 386BD728  addi r3, r11, -0x28d8
	ctx.r[3].s64 = ctx.r[11].s64 + -10456;
	// 82F1C0B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C0C0 size=8
    let mut pc: u32 = 0x82F1C0C0;
    'dispatch: loop {
        match pc {
            0x82F1C0C0 => {
    //   block [0x82F1C0C0..0x82F1C0C8)
	// 82F1C0C0: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82F1C0C4: 48000004  b 0x82f1c0c8
	sub_82F1C0C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1C0C8 size=100
    let mut pc: u32 = 0x82F1C0C8;
    'dispatch: loop {
        match pc {
            0x82F1C0C8 => {
    //   block [0x82F1C0C8..0x82F1C12C)
	// 82F1C0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1C0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1C0D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1C0D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1C0D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1C0DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1C0E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1C0E4: 4800FDA5  bl 0x82f2be88
	ctx.lr = 0x82F1C0E8;
	sub_82F2BE88(ctx, base);
	// 82F1C0E8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F1C0EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1C0F0: 419A0020  beq cr6, 0x82f1c110
	if ctx.cr[6].eq {
	pc = 0x82F1C110; continue 'dispatch;
	}
	// 82F1C0F4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C0F8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1C0FC: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F1C100: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1C104: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F1C108: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1C10C: 4BF846A5  bl 0x82ea07b0
	ctx.lr = 0x82F1C110;
	sub_82EA07B0(ctx, base);
	// 82F1C110: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1C114: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1C118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1C11C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1C120: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1C124: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1C128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C130 size=20
    let mut pc: u32 = 0x82F1C130;
    'dispatch: loop {
        match pc {
            0x82F1C130 => {
    //   block [0x82F1C130..0x82F1C144)
	// 82F1C130: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C134: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1C138: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C13C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1C140: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C148 size=8
    let mut pc: u32 = 0x82F1C148;
    'dispatch: loop {
        match pc {
            0x82F1C148 => {
    //   block [0x82F1C148..0x82F1C150)
	// 82F1C148: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1C14C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C150 size=44
    let mut pc: u32 = 0x82F1C150;
    'dispatch: loop {
        match pc {
            0x82F1C150 => {
    //   block [0x82F1C150..0x82F1C17C)
	// 82F1C150: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1C154: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1C158: 392BD7E4  addi r9, r11, -0x281c
	ctx.r[9].s64 = ctx.r[11].s64 + -10268;
	// 82F1C15C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F1C160: 38EABD10  addi r7, r10, -0x42f0
	ctx.r[7].s64 = ctx.r[10].s64 + -17136;
	// 82F1C164: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1C168: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 82F1C16C: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F1C170: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82F1C174: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F1C178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C180 size=12
    let mut pc: u32 = 0x82F1C180;
    'dispatch: loop {
        match pc {
            0x82F1C180 => {
    //   block [0x82F1C180..0x82F1C18C)
	// 82F1C180: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1C184: 386BD7E4  addi r3, r11, -0x281c
	ctx.r[3].s64 = ctx.r[11].s64 + -10268;
	// 82F1C188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C190 size=20
    let mut pc: u32 = 0x82F1C190;
    'dispatch: loop {
        match pc {
            0x82F1C190 => {
    //   block [0x82F1C190..0x82F1C1A4)
	// 82F1C190: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C194: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1C198: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C19C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1C1A0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C1A8 size=8
    let mut pc: u32 = 0x82F1C1A8;
    'dispatch: loop {
        match pc {
            0x82F1C1A8 => {
    //   block [0x82F1C1A8..0x82F1C1B0)
	// 82F1C1A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1C1AC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C1B0 size=44
    let mut pc: u32 = 0x82F1C1B0;
    'dispatch: loop {
        match pc {
            0x82F1C1B0 => {
    //   block [0x82F1C1B0..0x82F1C1DC)
	// 82F1C1B0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1C1B4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1C1B8: 392BD884  addi r9, r11, -0x277c
	ctx.r[9].s64 = ctx.r[11].s64 + -10108;
	// 82F1C1BC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F1C1C0: 38EABD10  addi r7, r10, -0x42f0
	ctx.r[7].s64 = ctx.r[10].s64 + -17136;
	// 82F1C1C4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1C1C8: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 82F1C1CC: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F1C1D0: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82F1C1D4: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F1C1D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C1E0 size=12
    let mut pc: u32 = 0x82F1C1E0;
    'dispatch: loop {
        match pc {
            0x82F1C1E0 => {
    //   block [0x82F1C1E0..0x82F1C1EC)
	// 82F1C1E0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1C1E4: 386BD884  addi r3, r11, -0x277c
	ctx.r[3].s64 = ctx.r[11].s64 + -10108;
	// 82F1C1E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1C1F0 size=116
    let mut pc: u32 = 0x82F1C1F0;
    'dispatch: loop {
        match pc {
            0x82F1C1F0 => {
    //   block [0x82F1C1F0..0x82F1C264)
	// 82F1C1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1C1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1C1F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1C1FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1C200: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1C204: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1C208: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1C20C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82F1C210: 4BFFB459  bl 0x82f17668
	ctx.lr = 0x82F1C214;
	sub_82F17668(ctx, base);
	// 82F1C214: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F1C218: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F1C21C: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82F1C220: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F1C224: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1C228: 419A0020  beq cr6, 0x82f1c248
	if ctx.cr[6].eq {
	pc = 0x82F1C248; continue 'dispatch;
	}
	// 82F1C22C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C230: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1C234: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F1C238: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1C23C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F1C240: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1C244: 4BF8456D  bl 0x82ea07b0
	ctx.lr = 0x82F1C248;
	sub_82EA07B0(ctx, base);
	// 82F1C248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1C24C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1C250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1C254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1C258: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1C25C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1C260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C268 size=20
    let mut pc: u32 = 0x82F1C268;
    'dispatch: loop {
        match pc {
            0x82F1C268 => {
    //   block [0x82F1C268..0x82F1C27C)
	// 82F1C268: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C26C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1C270: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C274: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1C278: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C280 size=8
    let mut pc: u32 = 0x82F1C280;
    'dispatch: loop {
        match pc {
            0x82F1C280 => {
    //   block [0x82F1C280..0x82F1C288)
	// 82F1C280: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1C284: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C288 size=32
    let mut pc: u32 = 0x82F1C288;
    'dispatch: loop {
        match pc {
            0x82F1C288 => {
    //   block [0x82F1C288..0x82F1C2A8)
	// 82F1C288: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1C28C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F1C290: 392BD940  addi r9, r11, -0x26c0
	ctx.r[9].s64 = ctx.r[11].s64 + -9920;
	// 82F1C294: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 82F1C298: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F1C29C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1C2A0: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82F1C2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C2A8 size=12
    let mut pc: u32 = 0x82F1C2A8;
    'dispatch: loop {
        match pc {
            0x82F1C2A8 => {
    //   block [0x82F1C2A8..0x82F1C2B4)
	// 82F1C2A8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1C2AC: 386BD940  addi r3, r11, -0x26c0
	ctx.r[3].s64 = ctx.r[11].s64 + -9920;
	// 82F1C2B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1C2B8 size=100
    let mut pc: u32 = 0x82F1C2B8;
    'dispatch: loop {
        match pc {
            0x82F1C2B8 => {
    //   block [0x82F1C2B8..0x82F1C31C)
	// 82F1C2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1C2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1C2C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1C2C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1C2C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1C2CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1C2D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1C2D4: 48010F95  bl 0x82f2d268
	ctx.lr = 0x82F1C2D8;
	sub_82F2D268(ctx, base);
	// 82F1C2D8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F1C2DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1C2E0: 419A0020  beq cr6, 0x82f1c300
	if ctx.cr[6].eq {
	pc = 0x82F1C300; continue 'dispatch;
	}
	// 82F1C2E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C2E8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1C2EC: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F1C2F0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1C2F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F1C2F8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1C2FC: 4BF844B5  bl 0x82ea07b0
	ctx.lr = 0x82F1C300;
	sub_82EA07B0(ctx, base);
	// 82F1C300: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1C304: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1C308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1C30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1C310: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1C314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1C318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C320 size=12
    let mut pc: u32 = 0x82F1C320;
    'dispatch: loop {
        match pc {
            0x82F1C320 => {
    //   block [0x82F1C320..0x82F1C32C)
	// 82F1C320: 8964006C  lbz r11, 0x6c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(108 as u32) ) } as u64;
	// 82F1C324: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82F1C328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1C330 size=28
    let mut pc: u32 = 0x82F1C330;
    'dispatch: loop {
        match pc {
            0x82F1C330 => {
    //   block [0x82F1C330..0x82F1C34C)
	// 82F1C330: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F1C334: 81430060  lwz r10, 0x60(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F1C338: 7D6B29D6  mullw r11, r11, r5
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[5].s32 as i64);
	// 82F1C33C: 7D2B2214  add r9, r11, r4
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82F1C340: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F1C344: 7C28542E  lfsx f1, r8, r10
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F1C348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C350 size=784
    let mut pc: u32 = 0x82F1C350;
    'dispatch: loop {
        match pc {
            0x82F1C350 => {
    //   block [0x82F1C350..0x82F1C660)
	// 82F1C350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1C354: 4828BE05  bl 0x831a8158
	ctx.lr = 0x82F1C358;
	sub_831A8130(ctx, base);
	// 82F1C358: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1C35C: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1C360: 83E40000  lwz r31, 0(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C364: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82F1C368: 390BD980  addi r8, r11, -0x2680
	ctx.r[8].s64 = ctx.r[11].s64 + -9856;
	// 82F1C36C: 376AFFFF  addic. r27, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[27].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1C660 size=148
    let mut pc: u32 = 0x82F1C660;
    'dispatch: loop {
        match pc {
            0x82F1C660 => {
    //   block [0x82F1C660..0x82F1C6F4)
	// 82F1C660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1C664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1C668: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1C66C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1C670: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1C674: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1C678: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1C67C: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82F1C680: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1C684: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F1C688: 409A0020  bne cr6, 0x82f1c6a8
	if !ctx.cr[6].eq {
	pc = 0x82F1C6A8; continue 'dispatch;
	}
	// 82F1C68C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C690: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F1C694: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F1C698: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F1C69C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F1C6A0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F1C6A4: 4BF8410D  bl 0x82ea07b0
	ctx.lr = 0x82F1C6A8;
	sub_82EA07B0(ctx, base);
	// 82F1C6A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1C6AC: 48010BBD  bl 0x82f2d268
	ctx.lr = 0x82F1C6B0;
	sub_82F2D268(ctx, base);
	// 82F1C6B0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F1C6B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1C6B8: 419A0020  beq cr6, 0x82f1c6d8
	if ctx.cr[6].eq {
	pc = 0x82F1C6D8; continue 'dispatch;
	}
	// 82F1C6BC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C6C0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1C6C4: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F1C6C8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1C6CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F1C6D0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1C6D4: 4BF840DD  bl 0x82ea07b0
	ctx.lr = 0x82F1C6D8;
	sub_82EA07B0(ctx, base);
	// 82F1C6D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1C6DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1C6E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1C6E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1C6E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1C6EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1C6F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C6F8 size=12
    let mut pc: u32 = 0x82F1C6F8;
    'dispatch: loop {
        match pc {
            0x82F1C6F8 => {
    //   block [0x82F1C6F8..0x82F1C704)
	// 82F1C6F8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F1C6FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1C700: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C704(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C704 size=8
    let mut pc: u32 = 0x82F1C704;
    'dispatch: loop {
        match pc {
            0x82F1C704 => {
    //   block [0x82F1C704..0x82F1C70C)
	// 82F1C704: 48012574  b 0x82f2ec78
	sub_82F2EC78(ctx, base);
	return;
	// 82F1C708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C710 size=20
    let mut pc: u32 = 0x82F1C710;
    'dispatch: loop {
        match pc {
            0x82F1C710 => {
    //   block [0x82F1C710..0x82F1C724)
	// 82F1C710: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C714: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1C718: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C71C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1C720: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1C728 size=44
    let mut pc: u32 = 0x82F1C728;
    'dispatch: loop {
        match pc {
            0x82F1C728 => {
    //   block [0x82F1C728..0x82F1C754)
	// 82F1C728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1C72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1C730: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1C734: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1C738: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F1C73C: 4801253D  bl 0x82f2ec78
	ctx.lr = 0x82F1C740;
	sub_82F2EC78(ctx, base);
	// 82F1C740: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F1C744: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82F1C748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1C74C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1C750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C758 size=12
    let mut pc: u32 = 0x82F1C758;
    'dispatch: loop {
        match pc {
            0x82F1C758 => {
    //   block [0x82F1C758..0x82F1C764)
	// 82F1C758: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F1C75C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1C760: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C764(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C764 size=8
    let mut pc: u32 = 0x82F1C764;
    'dispatch: loop {
        match pc {
            0x82F1C764 => {
    //   block [0x82F1C764..0x82F1C76C)
	// 82F1C764: 48012544  b 0x82f2eca8
	sub_82F2ECA8(ctx, base);
	return;
	// 82F1C768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C770 size=20
    let mut pc: u32 = 0x82F1C770;
    'dispatch: loop {
        match pc {
            0x82F1C770 => {
    //   block [0x82F1C770..0x82F1C784)
	// 82F1C770: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C774: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1C778: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C77C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1C780: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1C788 size=44
    let mut pc: u32 = 0x82F1C788;
    'dispatch: loop {
        match pc {
            0x82F1C788 => {
    //   block [0x82F1C788..0x82F1C7B4)
	// 82F1C788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1C78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1C790: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1C794: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1C798: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F1C79C: 4801250D  bl 0x82f2eca8
	ctx.lr = 0x82F1C7A0;
	sub_82F2ECA8(ctx, base);
	// 82F1C7A0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F1C7A4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82F1C7A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1C7AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1C7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C7B8 size=12
    let mut pc: u32 = 0x82F1C7B8;
    'dispatch: loop {
        match pc {
            0x82F1C7B8 => {
    //   block [0x82F1C7B8..0x82F1C7C4)
	// 82F1C7B8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F1C7BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1C7C0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C7C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C7C4 size=8
    let mut pc: u32 = 0x82F1C7C4;
    'dispatch: loop {
        match pc {
            0x82F1C7C4 => {
    //   block [0x82F1C7C4..0x82F1C7CC)
	// 82F1C7C4: 4801335C  b 0x82f2fb20
	sub_82F2FB20(ctx, base);
	return;
	// 82F1C7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C7D0 size=20
    let mut pc: u32 = 0x82F1C7D0;
    'dispatch: loop {
        match pc {
            0x82F1C7D0 => {
    //   block [0x82F1C7D0..0x82F1C7E4)
	// 82F1C7D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C7D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1C7D8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C7DC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1C7E0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1C7E8 size=44
    let mut pc: u32 = 0x82F1C7E8;
    'dispatch: loop {
        match pc {
            0x82F1C7E8 => {
    //   block [0x82F1C7E8..0x82F1C814)
	// 82F1C7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1C7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1C7F0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1C7F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1C7F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F1C7FC: 48013325  bl 0x82f2fb20
	ctx.lr = 0x82F1C800;
	sub_82F2FB20(ctx, base);
	// 82F1C800: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F1C804: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82F1C808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1C80C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1C810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C818 size=12
    let mut pc: u32 = 0x82F1C818;
    'dispatch: loop {
        match pc {
            0x82F1C818 => {
    //   block [0x82F1C818..0x82F1C824)
	// 82F1C818: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F1C81C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1C820: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C824(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C824 size=8
    let mut pc: u32 = 0x82F1C824;
    'dispatch: loop {
        match pc {
            0x82F1C824 => {
    //   block [0x82F1C824..0x82F1C82C)
	// 82F1C824: 4801307C  b 0x82f2f8a0
	sub_82F2F8A0(ctx, base);
	return;
	// 82F1C828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C830 size=20
    let mut pc: u32 = 0x82F1C830;
    'dispatch: loop {
        match pc {
            0x82F1C830 => {
    //   block [0x82F1C830..0x82F1C844)
	// 82F1C830: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C834: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1C838: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C83C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1C840: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1C848 size=44
    let mut pc: u32 = 0x82F1C848;
    'dispatch: loop {
        match pc {
            0x82F1C848 => {
    //   block [0x82F1C848..0x82F1C874)
	// 82F1C848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1C84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1C850: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1C854: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1C858: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F1C85C: 48013045  bl 0x82f2f8a0
	ctx.lr = 0x82F1C860;
	sub_82F2F8A0(ctx, base);
	// 82F1C860: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F1C864: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F1C868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1C86C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1C870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C878 size=12
    let mut pc: u32 = 0x82F1C878;
    'dispatch: loop {
        match pc {
            0x82F1C878 => {
    //   block [0x82F1C878..0x82F1C884)
	// 82F1C878: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F1C87C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1C880: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C884(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C884 size=8
    let mut pc: u32 = 0x82F1C884;
    'dispatch: loop {
        match pc {
            0x82F1C884 => {
    //   block [0x82F1C884..0x82F1C88C)
	// 82F1C884: 480132B4  b 0x82f2fb38
	sub_82F2FB38(ctx, base);
	return;
	// 82F1C888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C890 size=20
    let mut pc: u32 = 0x82F1C890;
    'dispatch: loop {
        match pc {
            0x82F1C890 => {
    //   block [0x82F1C890..0x82F1C8A4)
	// 82F1C890: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C894: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1C898: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C89C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1C8A0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1C8A8 size=44
    let mut pc: u32 = 0x82F1C8A8;
    'dispatch: loop {
        match pc {
            0x82F1C8A8 => {
    //   block [0x82F1C8A8..0x82F1C8D4)
	// 82F1C8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1C8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1C8B0: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1C8B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1C8B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F1C8BC: 4801327D  bl 0x82f2fb38
	ctx.lr = 0x82F1C8C0;
	sub_82F2FB38(ctx, base);
	// 82F1C8C0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F1C8C4: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 82F1C8C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1C8CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1C8D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C8D8 size=20
    let mut pc: u32 = 0x82F1C8D8;
    'dispatch: loop {
        match pc {
            0x82F1C8D8 => {
    //   block [0x82F1C8D8..0x82F1C8EC)
	// 82F1C8D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C8DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1C8E0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C8E4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1C8E8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C8F0 size=8
    let mut pc: u32 = 0x82F1C8F0;
    'dispatch: loop {
        match pc {
            0x82F1C8F0 => {
    //   block [0x82F1C8F0..0x82F1C8F8)
	// 82F1C8F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1C8F4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C8F8 size=32
    let mut pc: u32 = 0x82F1C8F8;
    'dispatch: loop {
        match pc {
            0x82F1C8F8 => {
    //   block [0x82F1C8F8..0x82F1C918)
	// 82F1C8F8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1C8FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F1C900: 392BBEA4  addi r9, r11, -0x415c
	ctx.r[9].s64 = ctx.r[11].s64 + -16732;
	// 82F1C904: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82F1C908: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F1C90C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1C910: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82F1C914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C918 size=12
    let mut pc: u32 = 0x82F1C918;
    'dispatch: loop {
        match pc {
            0x82F1C918 => {
    //   block [0x82F1C918..0x82F1C924)
	// 82F1C918: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1C91C: 386BBEA4  addi r3, r11, -0x415c
	ctx.r[3].s64 = ctx.r[11].s64 + -16732;
	// 82F1C920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1C928 size=72
    let mut pc: u32 = 0x82F1C928;
    'dispatch: loop {
        match pc {
            0x82F1C928 => {
    //   block [0x82F1C928..0x82F1C970)
	// 82F1C928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1C92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1C930: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1C934: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1C938: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1C93C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1C940: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82F1C944: 392BDCE8  addi r9, r11, -0x2318
	ctx.r[9].s64 = ctx.r[11].s64 + -8984;
	// 82F1C948: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F1C94C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1C950: 419A000C  beq cr6, 0x82f1c95c
	if ctx.cr[6].eq {
	pc = 0x82F1C95C; continue 'dispatch;
	}
	// 82F1C954: 4B3A3915  bl 0x822c0268
	ctx.lr = 0x82F1C958;
	sub_822C0268(ctx, base);
	// 82F1C958: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1C95C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1C960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1C964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1C968: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1C96C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C970 size=20
    let mut pc: u32 = 0x82F1C970;
    'dispatch: loop {
        match pc {
            0x82F1C970 => {
    //   block [0x82F1C970..0x82F1C984)
	// 82F1C970: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C974: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1C978: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1C97C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1C980: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C988 size=8
    let mut pc: u32 = 0x82F1C988;
    'dispatch: loop {
        match pc {
            0x82F1C988 => {
    //   block [0x82F1C988..0x82F1C990)
	// 82F1C988: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1C98C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C990 size=48
    let mut pc: u32 = 0x82F1C990;
    'dispatch: loop {
        match pc {
            0x82F1C990 => {
    //   block [0x82F1C990..0x82F1C9C0)
	// 82F1C990: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1C994: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1C998: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F1C99C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F1C9A0: 38EBDCE8  addi r7, r11, -0x2318
	ctx.r[7].s64 = ctx.r[11].s64 + -8984;
	// 82F1C9A4: 38CADD08  addi r6, r10, -0x22f8
	ctx.r[6].s64 = ctx.r[10].s64 + -8952;
	// 82F1C9A8: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F1C9AC: 38A9DCF8  addi r5, r9, -0x2308
	ctx.r[5].s64 = ctx.r[9].s64 + -8968;
	// 82F1C9B0: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82F1C9B4: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F1C9B8: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82F1C9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1C9C0 size=12
    let mut pc: u32 = 0x82F1C9C0;
    'dispatch: loop {
        match pc {
            0x82F1C9C0 => {
    //   block [0x82F1C9C0..0x82F1C9CC)
	// 82F1C9C0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1C9C4: 386BDD08  addi r3, r11, -0x22f8
	ctx.r[3].s64 = ctx.r[11].s64 + -8952;
	// 82F1C9C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1C9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1C9D0 size=100
    let mut pc: u32 = 0x82F1C9D0;
    'dispatch: loop {
        match pc {
            0x82F1C9D0 => {
    //   block [0x82F1C9D0..0x82F1CA34)
	// 82F1C9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1C9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1C9D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1C9DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1C9E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1C9E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1C9E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1C9EC: 480147ED  bl 0x82f311d8
	ctx.lr = 0x82F1C9F0;
	sub_82F311D8(ctx, base);
	// 82F1C9F0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F1C9F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1C9F8: 419A0020  beq cr6, 0x82f1ca18
	if ctx.cr[6].eq {
	pc = 0x82F1CA18; continue 'dispatch;
	}
	// 82F1C9FC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CA00: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1CA04: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 82F1CA08: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1CA0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F1CA10: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1CA14: 4BF83D9D  bl 0x82ea07b0
	ctx.lr = 0x82F1CA18;
	sub_82EA07B0(ctx, base);
	// 82F1CA18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1CA1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1CA20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1CA24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1CA28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1CA2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1CA30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CA38 size=8
    let mut pc: u32 = 0x82F1CA38;
    'dispatch: loop {
        match pc {
            0x82F1CA38 => {
    //   block [0x82F1CA38..0x82F1CA40)
	// 82F1CA38: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82F1CA3C: 4BFFFF94  b 0x82f1c9d0
	sub_82F1C9D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CA40 size=20
    let mut pc: u32 = 0x82F1CA40;
    'dispatch: loop {
        match pc {
            0x82F1CA40 => {
    //   block [0x82F1CA40..0x82F1CA54)
	// 82F1CA40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CA44: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1CA48: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CA4C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1CA50: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CA58 size=8
    let mut pc: u32 = 0x82F1CA58;
    'dispatch: loop {
        match pc {
            0x82F1CA58 => {
    //   block [0x82F1CA58..0x82F1CA60)
	// 82F1CA58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1CA5C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CA60 size=32
    let mut pc: u32 = 0x82F1CA60;
    'dispatch: loop {
        match pc {
            0x82F1CA60 => {
    //   block [0x82F1CA60..0x82F1CA80)
	// 82F1CA60: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1CA64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F1CA68: 392BDD8C  addi r9, r11, -0x2274
	ctx.r[9].s64 = ctx.r[11].s64 + -8820;
	// 82F1CA6C: 3900001B  li r8, 0x1b
	ctx.r[8].s64 = 27;
	// 82F1CA70: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F1CA74: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1CA78: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82F1CA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CA80 size=12
    let mut pc: u32 = 0x82F1CA80;
    'dispatch: loop {
        match pc {
            0x82F1CA80 => {
    //   block [0x82F1CA80..0x82F1CA8C)
	// 82F1CA80: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1CA84: 386BDD8C  addi r3, r11, -0x2274
	ctx.r[3].s64 = ctx.r[11].s64 + -8820;
	// 82F1CA88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CA90 size=20
    let mut pc: u32 = 0x82F1CA90;
    'dispatch: loop {
        match pc {
            0x82F1CA90 => {
    //   block [0x82F1CA90..0x82F1CAA4)
	// 82F1CA90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CA94: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1CA98: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CA9C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1CAA0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CAA8 size=12
    let mut pc: u32 = 0x82F1CAA8;
    'dispatch: loop {
        match pc {
            0x82F1CAA8 => {
    //   block [0x82F1CAA8..0x82F1CAB4)
	// 82F1CAA8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F1CAAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1CAB0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CAB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CAB4 size=8
    let mut pc: u32 = 0x82F1CAB4;
    'dispatch: loop {
        match pc {
            0x82F1CAB4 => {
    //   block [0x82F1CAB4..0x82F1CABC)
	// 82F1CAB4: 4800003C  b 0x82f1caf0
	sub_82F1CAF0(ctx, base);
	return;
	// 82F1CAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1CAC0 size=44
    let mut pc: u32 = 0x82F1CAC0;
    'dispatch: loop {
        match pc {
            0x82F1CAC0 => {
    //   block [0x82F1CAC0..0x82F1CAEC)
	// 82F1CAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1CAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1CAC8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1CACC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1CAD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F1CAD4: 4800001D  bl 0x82f1caf0
	ctx.lr = 0x82F1CAD8;
	sub_82F1CAF0(ctx, base);
	// 82F1CAD8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F1CADC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82F1CAE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1CAE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1CAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1CAF0 size=116
    let mut pc: u32 = 0x82F1CAF0;
    'dispatch: loop {
        match pc {
            0x82F1CAF0 => {
    //   block [0x82F1CAF0..0x82F1CB64)
	// 82F1CAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1CAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1CAF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1CAFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1CB00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1CB04: 48000EBD  bl 0x82f1d9c0
	ctx.lr = 0x82F1CB08;
	sub_82F1D9C0(ctx, base);
	// 82F1CB08: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82F1CB0C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82F1CB10: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82F1CB14: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82F1CB18: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82F1CB1C: 38CB4294  addi r6, r11, 0x4294
	ctx.r[6].s64 = ctx.r[11].s64 + 17044;
	// 82F1CB20: 38AA4288  addi r5, r10, 0x4288
	ctx.r[5].s64 = ctx.r[10].s64 + 17032;
	// 82F1CB24: 3868427C  addi r3, r8, 0x427c
	ctx.r[3].s64 = ctx.r[8].s64 + 17020;
	// 82F1CB28: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F1CB2C: 38894268  addi r4, r9, 0x4268
	ctx.r[4].s64 = ctx.r[9].s64 + 17000;
	// 82F1CB30: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82F1CB34: 3967425C  addi r11, r7, 0x425c
	ctx.r[11].s64 = ctx.r[7].s64 + 16988;
	// 82F1CB38: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82F1CB3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F1CB40: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F1CB44: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82F1CB48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1CB4C: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82F1CB50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1CB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1CB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1CB5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1CB60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CB68 size=20
    let mut pc: u32 = 0x82F1CB68;
    'dispatch: loop {
        match pc {
            0x82F1CB68 => {
    //   block [0x82F1CB68..0x82F1CB7C)
	// 82F1CB68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CB6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1CB70: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CB74: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1CB78: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CB80 size=8
    let mut pc: u32 = 0x82F1CB80;
    'dispatch: loop {
        match pc {
            0x82F1CB80 => {
    //   block [0x82F1CB80..0x82F1CB88)
	// 82F1CB80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1CB84: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CB88 size=32
    let mut pc: u32 = 0x82F1CB88;
    'dispatch: loop {
        match pc {
            0x82F1CB88 => {
    //   block [0x82F1CB88..0x82F1CBA8)
	// 82F1CB88: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1CB8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F1CB90: 392BDE28  addi r9, r11, -0x21d8
	ctx.r[9].s64 = ctx.r[11].s64 + -8664;
	// 82F1CB94: 39000014  li r8, 0x14
	ctx.r[8].s64 = 20;
	// 82F1CB98: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F1CB9C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1CBA0: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82F1CBA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CBA8 size=12
    let mut pc: u32 = 0x82F1CBA8;
    'dispatch: loop {
        match pc {
            0x82F1CBA8 => {
    //   block [0x82F1CBA8..0x82F1CBB4)
	// 82F1CBA8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1CBAC: 386BDE28  addi r3, r11, -0x21d8
	ctx.r[3].s64 = ctx.r[11].s64 + -8664;
	// 82F1CBB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CBB8 size=20
    let mut pc: u32 = 0x82F1CBB8;
    'dispatch: loop {
        match pc {
            0x82F1CBB8 => {
    //   block [0x82F1CBB8..0x82F1CBCC)
	// 82F1CBB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CBBC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1CBC0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CBC4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1CBC8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CBD0 size=12
    let mut pc: u32 = 0x82F1CBD0;
    'dispatch: loop {
        match pc {
            0x82F1CBD0 => {
    //   block [0x82F1CBD0..0x82F1CBDC)
	// 82F1CBD0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F1CBD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1CBD8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CBDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CBDC size=8
    let mut pc: u32 = 0x82F1CBDC;
    'dispatch: loop {
        match pc {
            0x82F1CBDC => {
    //   block [0x82F1CBDC..0x82F1CBE4)
	// 82F1CBDC: 4800003C  b 0x82f1cc18
	sub_82F1CC18(ctx, base);
	return;
	// 82F1CBE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1CBE8 size=44
    let mut pc: u32 = 0x82F1CBE8;
    'dispatch: loop {
        match pc {
            0x82F1CBE8 => {
    //   block [0x82F1CBE8..0x82F1CC14)
	// 82F1CBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1CBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1CBF0: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1CBF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1CBF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F1CBFC: 4800001D  bl 0x82f1cc18
	ctx.lr = 0x82F1CC00;
	sub_82F1CC18(ctx, base);
	// 82F1CC00: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F1CC04: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82F1CC08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1CC0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1CC10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1CC18 size=116
    let mut pc: u32 = 0x82F1CC18;
    'dispatch: loop {
        match pc {
            0x82F1CC18 => {
    //   block [0x82F1CC18..0x82F1CC8C)
	// 82F1CC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1CC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1CC20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1CC24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1CC28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1CC2C: 48000D95  bl 0x82f1d9c0
	ctx.lr = 0x82F1CC30;
	sub_82F1D9C0(ctx, base);
	// 82F1CC30: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1CC34: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1CC38: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F1CC3C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F1CC40: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82F1CC44: 38CBDF24  addi r6, r11, -0x20dc
	ctx.r[6].s64 = ctx.r[11].s64 + -8412;
	// 82F1CC48: 38AADF18  addi r5, r10, -0x20e8
	ctx.r[5].s64 = ctx.r[10].s64 + -8424;
	// 82F1CC4C: 3868DF0C  addi r3, r8, -0x20f4
	ctx.r[3].s64 = ctx.r[8].s64 + -8436;
	// 82F1CC50: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F1CC54: 3889DEF8  addi r4, r9, -0x2108
	ctx.r[4].s64 = ctx.r[9].s64 + -8456;
	// 82F1CC58: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82F1CC5C: 3967DEEC  addi r11, r7, -0x2114
	ctx.r[11].s64 = ctx.r[7].s64 + -8468;
	// 82F1CC60: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82F1CC64: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82F1CC68: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F1CC6C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82F1CC70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1CC74: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82F1CC78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1CC7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1CC80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1CC84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1CC88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1CC90 size=100
    let mut pc: u32 = 0x82F1CC90;
    'dispatch: loop {
        match pc {
            0x82F1CC90 => {
    //   block [0x82F1CC90..0x82F1CCF4)
	// 82F1CC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1CC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1CC98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1CC9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1CCA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1CCA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1CCA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1CCAC: 4801507D  bl 0x82f31d28
	ctx.lr = 0x82F1CCB0;
	sub_82F31D28(ctx, base);
	// 82F1CCB0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F1CCB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1CCB8: 419A0020  beq cr6, 0x82f1ccd8
	if ctx.cr[6].eq {
	pc = 0x82F1CCD8; continue 'dispatch;
	}
	// 82F1CCBC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CCC0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1CCC4: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F1CCC8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1CCCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F1CCD0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1CCD4: 4BF83ADD  bl 0x82ea07b0
	ctx.lr = 0x82F1CCD8;
	sub_82EA07B0(ctx, base);
	// 82F1CCD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1CCDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1CCE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1CCE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1CCE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1CCEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1CCF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CCF8 size=8
    let mut pc: u32 = 0x82F1CCF8;
    'dispatch: loop {
        match pc {
            0x82F1CCF8 => {
    //   block [0x82F1CCF8..0x82F1CD00)
	// 82F1CCF8: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82F1CCFC: 4BFFFF94  b 0x82f1cc90
	sub_82F1CC90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CD00 size=8
    let mut pc: u32 = 0x82F1CD00;
    'dispatch: loop {
        match pc {
            0x82F1CD00 => {
    //   block [0x82F1CD00..0x82F1CD08)
	// 82F1CD00: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82F1CD04: 4BFFFF8C  b 0x82f1cc90
	sub_82F1CC90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CD08 size=8
    let mut pc: u32 = 0x82F1CD08;
    'dispatch: loop {
        match pc {
            0x82F1CD08 => {
    //   block [0x82F1CD08..0x82F1CD10)
	// 82F1CD08: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82F1CD0C: 4BFFFF84  b 0x82f1cc90
	sub_82F1CC90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CD10 size=8
    let mut pc: u32 = 0x82F1CD10;
    'dispatch: loop {
        match pc {
            0x82F1CD10 => {
    //   block [0x82F1CD10..0x82F1CD18)
	// 82F1CD10: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82F1CD14: 4BFFFF7C  b 0x82f1cc90
	sub_82F1CC90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CD18 size=20
    let mut pc: u32 = 0x82F1CD18;
    'dispatch: loop {
        match pc {
            0x82F1CD18 => {
    //   block [0x82F1CD18..0x82F1CD2C)
	// 82F1CD18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CD1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1CD20: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CD24: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1CD28: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1CD30 size=88
    let mut pc: u32 = 0x82F1CD30;
    'dispatch: loop {
        match pc {
            0x82F1CD30 => {
    //   block [0x82F1CD30..0x82F1CD88)
	// 82F1CD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1CD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1CD38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1CD3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1CD40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1CD44: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F1CD48: 419A002C  beq cr6, 0x82f1cd74
	if ctx.cr[6].eq {
	pc = 0x82F1CD74; continue 'dispatch;
	}
	// 82F1CD4C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F1CD50: 4800D499  bl 0x82f2a1e8
	ctx.lr = 0x82F1CD54;
	sub_82F2A1E8(ctx, base);
	// 82F1CD54: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1CD58: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1CD5C: 392BDF6C  addi r9, r11, -0x2094
	ctx.r[9].s64 = ctx.r[11].s64 + -8340;
	// 82F1CD60: 390ADF48  addi r8, r10, -0x20b8
	ctx.r[8].s64 = ctx.r[10].s64 + -8376;
	// 82F1CD64: 38E00016  li r7, 0x16
	ctx.r[7].s64 = 22;
	// 82F1CD68: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1CD6C: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82F1CD70: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82F1CD74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1CD78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1CD7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1CD80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1CD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1CD88 size=48
    let mut pc: u32 = 0x82F1CD88;
    'dispatch: loop {
        match pc {
            0x82F1CD88 => {
    //   block [0x82F1CD88..0x82F1CDB8)
	// 82F1CD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1CD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1CD90: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1CD94: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1CD98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F1CD9C: 4800D44D  bl 0x82f2a1e8
	ctx.lr = 0x82F1CDA0;
	sub_82F2A1E8(ctx, base);
	// 82F1CDA0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1CDA4: 386BDF6C  addi r3, r11, -0x2094
	ctx.r[3].s64 = ctx.r[11].s64 + -8340;
	// 82F1CDA8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82F1CDAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1CDB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1CDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1CDB8 size=172
    let mut pc: u32 = 0x82F1CDB8;
    'dispatch: loop {
        match pc {
            0x82F1CDB8 => {
    //   block [0x82F1CDB8..0x82F1CE64)
	// 82F1CDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1CDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1CDC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1CDC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1CDC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1CDCC: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F1CDD0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1CDD4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F1CDD8: 409A0020  bne cr6, 0x82f1cdf8
	if !ctx.cr[6].eq {
	pc = 0x82F1CDF8; continue 'dispatch;
	}
	// 82F1CDDC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CDE0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F1CDE4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F1CDE8: 809F0040  lwz r4, 0x40(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F1CDEC: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82F1CDF0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F1CDF4: 4BF839BD  bl 0x82ea07b0
	ctx.lr = 0x82F1CDF8;
	sub_82EA07B0(ctx, base);
	// 82F1CDF8: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82F1CDFC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1CE00: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F1CE04: 409A0024  bne cr6, 0x82f1ce28
	if !ctx.cr[6].eq {
	pc = 0x82F1CE28; continue 'dispatch;
	}
	// 82F1CE08: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CE0C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F1CE10: 556800BE  clrlwi r8, r11, 2
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F1CE14: 809F0034  lwz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F1CE18: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F1CE1C: 1CA80038  mulli r5, r8, 0x38
	ctx.r[5].s64 = ctx.r[8].s64 * 56;
	// 82F1CE20: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F1CE24: 4BF8398D  bl 0x82ea07b0
	ctx.lr = 0x82F1CE28;
	sub_82EA07B0(ctx, base);
	// 82F1CE28: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F1CE2C: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82F1CE30: 409A0008  bne cr6, 0x82f1ce38
	if !ctx.cr[6].eq {
	pc = 0x82F1CE38; continue 'dispatch;
	}
	// 82F1CE34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F1CE38: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1CE3C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F1CE40: 390ABCD8  addi r8, r10, -0x4328
	ctx.r[8].s64 = ctx.r[10].s64 + -17192;
	// 82F1CE44: 38E99EAC  addi r7, r9, -0x6154
	ctx.r[7].s64 = ctx.r[9].s64 + -24916;
	// 82F1CE48: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F1CE4C: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F1CE50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1CE54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1CE58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1CE5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1CE60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CE68 size=8
    let mut pc: u32 = 0x82F1CE68;
    'dispatch: loop {
        match pc {
            0x82F1CE68 => {
    //   block [0x82F1CE68..0x82F1CE70)
	// 82F1CE68: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82F1CE6C: 48000004  b 0x82f1ce70
	sub_82F1CE70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1CE70 size=100
    let mut pc: u32 = 0x82F1CE70;
    'dispatch: loop {
        match pc {
            0x82F1CE70 => {
    //   block [0x82F1CE70..0x82F1CED4)
	// 82F1CE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1CE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1CE78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1CE7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1CE80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1CE84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1CE88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1CE8C: 4BFFFF2D  bl 0x82f1cdb8
	ctx.lr = 0x82F1CE90;
	sub_82F1CDB8(ctx, base);
	// 82F1CE90: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F1CE94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1CE98: 419A0020  beq cr6, 0x82f1ceb8
	if ctx.cr[6].eq {
	pc = 0x82F1CEB8; continue 'dispatch;
	}
	// 82F1CE9C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CEA0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1CEA4: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F1CEA8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1CEAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F1CEB0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1CEB4: 4BF838FD  bl 0x82ea07b0
	ctx.lr = 0x82F1CEB8;
	sub_82EA07B0(ctx, base);
	// 82F1CEB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1CEBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1CEC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1CEC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1CEC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1CECC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1CED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CED8 size=20
    let mut pc: u32 = 0x82F1CED8;
    'dispatch: loop {
        match pc {
            0x82F1CED8 => {
    //   block [0x82F1CED8..0x82F1CEEC)
	// 82F1CED8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CEDC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1CEE0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CEE4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1CEE8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CEF0 size=8
    let mut pc: u32 = 0x82F1CEF0;
    'dispatch: loop {
        match pc {
            0x82F1CEF0 => {
    //   block [0x82F1CEF0..0x82F1CEF8)
	// 82F1CEF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1CEF4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CEF8 size=24
    let mut pc: u32 = 0x82F1CEF8;
    'dispatch: loop {
        match pc {
            0x82F1CEF8 => {
    //   block [0x82F1CEF8..0x82F1CF10)
	// 82F1CEF8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82F1CEFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F1CF00: 392B42A4  addi r9, r11, 0x42a4
	ctx.r[9].s64 = ctx.r[11].s64 + 17060;
	// 82F1CF04: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F1CF08: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1CF0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CF10 size=12
    let mut pc: u32 = 0x82F1CF10;
    'dispatch: loop {
        match pc {
            0x82F1CF10 => {
    //   block [0x82F1CF10..0x82F1CF1C)
	// 82F1CF10: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82F1CF14: 386B42A4  addi r3, r11, 0x42a4
	ctx.r[3].s64 = ctx.r[11].s64 + 17060;
	// 82F1CF18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CF20 size=20
    let mut pc: u32 = 0x82F1CF20;
    'dispatch: loop {
        match pc {
            0x82F1CF20 => {
    //   block [0x82F1CF20..0x82F1CF34)
	// 82F1CF20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CF24: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1CF28: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CF2C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1CF30: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CF38 size=12
    let mut pc: u32 = 0x82F1CF38;
    'dispatch: loop {
        match pc {
            0x82F1CF38 => {
    //   block [0x82F1CF38..0x82F1CF44)
	// 82F1CF38: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F1CF3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1CF40: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CF44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CF44 size=8
    let mut pc: u32 = 0x82F1CF44;
    'dispatch: loop {
        match pc {
            0x82F1CF44 => {
    //   block [0x82F1CF44..0x82F1CF4C)
	// 82F1CF44: 48001DDC  b 0x82f1ed20
	sub_82F1ED20(ctx, base);
	return;
	// 82F1CF48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1CF50 size=44
    let mut pc: u32 = 0x82F1CF50;
    'dispatch: loop {
        match pc {
            0x82F1CF50 => {
    //   block [0x82F1CF50..0x82F1CF7C)
	// 82F1CF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1CF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1CF58: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1CF5C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1CF60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F1CF64: 48001DBD  bl 0x82f1ed20
	ctx.lr = 0x82F1CF68;
	sub_82F1ED20(ctx, base);
	// 82F1CF68: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F1CF6C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82F1CF70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1CF74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1CF78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CF80 size=20
    let mut pc: u32 = 0x82F1CF80;
    'dispatch: loop {
        match pc {
            0x82F1CF80 => {
    //   block [0x82F1CF80..0x82F1CF94)
	// 82F1CF80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CF84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1CF88: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CF8C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1CF90: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CF98 size=8
    let mut pc: u32 = 0x82F1CF98;
    'dispatch: loop {
        match pc {
            0x82F1CF98 => {
    //   block [0x82F1CF98..0x82F1CFA0)
	// 82F1CF98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1CF9C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CFA0 size=24
    let mut pc: u32 = 0x82F1CFA0;
    'dispatch: loop {
        match pc {
            0x82F1CFA0 => {
    //   block [0x82F1CFA0..0x82F1CFB8)
	// 82F1CFA0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1CFA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F1CFA8: 392BE23C  addi r9, r11, -0x1dc4
	ctx.r[9].s64 = ctx.r[11].s64 + -7620;
	// 82F1CFAC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F1CFB0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1CFB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1CFB8 size=12
    let mut pc: u32 = 0x82F1CFB8;
    'dispatch: loop {
        match pc {
            0x82F1CFB8 => {
    //   block [0x82F1CFB8..0x82F1CFC4)
	// 82F1CFB8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1CFBC: 386BE23C  addi r3, r11, -0x1dc4
	ctx.r[3].s64 = ctx.r[11].s64 + -7620;
	// 82F1CFC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1CFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1CFC8 size=140
    let mut pc: u32 = 0x82F1CFC8;
    'dispatch: loop {
        match pc {
            0x82F1CFC8 => {
    //   block [0x82F1CFC8..0x82F1D054)
	// 82F1CFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1CFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1CFD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1CFD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1CFD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1CFDC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F1CFE0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1CFE4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F1CFE8: 409A0020  bne cr6, 0x82f1d008
	if !ctx.cr[6].eq {
	pc = 0x82F1D008; continue 'dispatch;
	}
	// 82F1CFEC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1CFF0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F1CFF4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F1CFF8: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F1CFFC: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F1D000: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F1D004: 4BF837AD  bl 0x82ea07b0
	ctx.lr = 0x82F1D008;
	sub_82EA07B0(ctx, base);
	// 82F1D008: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F1D00C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1D010: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F1D014: 409A0020  bne cr6, 0x82f1d034
	if !ctx.cr[6].eq {
	pc = 0x82F1D034; continue 'dispatch;
	}
	// 82F1D018: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D01C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F1D020: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F1D024: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F1D028: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82F1D02C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F1D030: 4BF83781  bl 0x82ea07b0
	ctx.lr = 0x82F1D034;
	sub_82EA07B0(ctx, base);
	// 82F1D034: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F1D038: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82F1D03C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F1D040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1D044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1D048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1D04C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1D050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1D058 size=100
    let mut pc: u32 = 0x82F1D058;
    'dispatch: loop {
        match pc {
            0x82F1D058 => {
    //   block [0x82F1D058..0x82F1D0BC)
	// 82F1D058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1D05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1D060: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1D064: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1D068: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1D06C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1D070: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1D074: 4BFFFF55  bl 0x82f1cfc8
	ctx.lr = 0x82F1D078;
	sub_82F1CFC8(ctx, base);
	// 82F1D078: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F1D07C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1D080: 419A0020  beq cr6, 0x82f1d0a0
	if ctx.cr[6].eq {
	pc = 0x82F1D0A0; continue 'dispatch;
	}
	// 82F1D084: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D088: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1D08C: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F1D090: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1D094: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F1D098: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1D09C: 4BF83715  bl 0x82ea07b0
	ctx.lr = 0x82F1D0A0;
	sub_82EA07B0(ctx, base);
	// 82F1D0A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1D0A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1D0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1D0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1D0B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1D0B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1D0B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D0C0 size=20
    let mut pc: u32 = 0x82F1D0C0;
    'dispatch: loop {
        match pc {
            0x82F1D0C0 => {
    //   block [0x82F1D0C0..0x82F1D0D4)
	// 82F1D0C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D0C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1D0C8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D0CC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1D0D0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D0D8 size=8
    let mut pc: u32 = 0x82F1D0D8;
    'dispatch: loop {
        match pc {
            0x82F1D0D8 => {
    //   block [0x82F1D0D8..0x82F1D0E0)
	// 82F1D0D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1D0DC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D0E0 size=44
    let mut pc: u32 = 0x82F1D0E0;
    'dispatch: loop {
        match pc {
            0x82F1D0E0 => {
    //   block [0x82F1D0E0..0x82F1D10C)
	// 82F1D0E0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1D0E4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1D0E8: 392BE2CC  addi r9, r11, -0x1d34
	ctx.r[9].s64 = ctx.r[11].s64 + -7476;
	// 82F1D0EC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F1D0F0: 38EABD10  addi r7, r10, -0x42f0
	ctx.r[7].s64 = ctx.r[10].s64 + -17136;
	// 82F1D0F4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1D0F8: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82F1D0FC: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F1D100: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82F1D104: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F1D108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D110 size=12
    let mut pc: u32 = 0x82F1D110;
    'dispatch: loop {
        match pc {
            0x82F1D110 => {
    //   block [0x82F1D110..0x82F1D11C)
	// 82F1D110: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1D114: 386BE2CC  addi r3, r11, -0x1d34
	ctx.r[3].s64 = ctx.r[11].s64 + -7476;
	// 82F1D118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D120 size=20
    let mut pc: u32 = 0x82F1D120;
    'dispatch: loop {
        match pc {
            0x82F1D120 => {
    //   block [0x82F1D120..0x82F1D134)
	// 82F1D120: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D124: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1D128: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D12C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1D130: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


