pub fn sub_83041DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041DE0 size=40
    let mut pc: u32 = 0x83041DE0;
    'dispatch: loop {
        match pc {
            0x83041DE0 => {
    //   block [0x83041DE0..0x83041E08)
	// 83041DE0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83041DE4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83041DE8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83041DEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041DF0: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83041DF4: 4BF97085  bl 0x82fd8e78
	ctx.lr = 0x83041DF8;
	sub_82FD8E78(ctx, base);
	// 83041DF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83041DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83041E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83041E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83041E08 size=8
    let mut pc: u32 = 0x83041E08;
    'dispatch: loop {
        match pc {
            0x83041E08 => {
    //   block [0x83041E08..0x83041E10)
	// 83041E08: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83041E0C: 8215E4EC  lwz r16, -0x1b14(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-6932 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041E10 size=380
    let mut pc: u32 = 0x83041E10;
    'dispatch: loop {
        match pc {
            0x83041E10 => {
    //   block [0x83041E10..0x83041F8C)
	// 83041E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83041E14: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83041E18: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83041E1C: 48166345  bl 0x831a8160
	ctx.lr = 0x83041E20;
	sub_831A8130(ctx, base);
	// 83041E20: 3BE1FF00  addi r31, r1, -0x100
	ctx.r[31].s64 = ctx.r[1].s64 + -256;
	// 83041E24: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041E28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83041E2C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83041E30: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83041E34: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 83041E38: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83041E3C: 93DF0114  stw r30, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[30].u32 ) };
	// 83041E40: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83041E44: 41820018  beq 0x83041e5c
	if ctx.cr[0].eq {
	pc = 0x83041E5C; continue 'dispatch;
	}
	// 83041E48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83041E4C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83041E50: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83041E54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83041E58: 4E800421  bctrl
	ctx.lr = 0x83041E5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83041E5C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83041E60: 556B0739  rlwinm. r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83041E64: 418200A4  beq 0x83041f08
	if ctx.cr[0].eq {
	pc = 0x83041F08; continue 'dispatch;
	}
	// 83041E68: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83041E6C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83041E70: 40820044  bne 0x83041eb4
	if !ctx.cr[0].eq {
	pc = 0x83041EB4; continue 'dispatch;
	}
	// 83041E74: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 83041E78: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83041E7C: 4BF9641D  bl 0x82fd8298
	ctx.lr = 0x83041E80;
	sub_82FD8298(ctx, base);
	// 83041E80: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83041E84: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83041E88: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83041E8C: 41820020  beq 0x83041eac
	if ctx.cr[0].eq {
	pc = 0x83041EAC; continue 'dispatch;
	}
	// 83041E90: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83041E94: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83041E98: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83041E9C: 38ABD918  addi r5, r11, -0x26e8
	ctx.r[5].s64 = ctx.r[11].s64 + -9960;
	// 83041EA0: 48048249  bl 0x8308a0e8
	ctx.lr = 0x83041EA4;
	sub_8308A0E8(ctx, base);
	// 83041EA4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83041EA8: 48000008  b 0x83041eb0
	pc = 0x83041EB0; continue 'dispatch;
	// 83041EAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83041EB0: 907E0028  stw r3, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 83041EB4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83041EB8: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83041EBC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83041EC0: 48048851  bl 0x8308a710
	ctx.lr = 0x83041EC4;
	sub_8308A710(ctx, base);
	// 83041EC4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83041EC8: 40820040  bne 0x83041f08
	if !ctx.cr[0].eq {
	pc = 0x83041F08; continue 'dispatch;
	}
	// 83041ECC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83041ED0: 811E0024  lwz r8, 0x24(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83041ED4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83041ED8: 93410054  stw r26, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u32 ) };
	// 83041EDC: 388BE460  addi r4, r11, -0x1ba0
	ctx.r[4].s64 = ctx.r[11].s64 + -7072;
	// 83041EE0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83041EE4: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 83041EE8: 38C000EE  li r6, 0xee
	ctx.r[6].s64 = 238;
	// 83041EEC: 38A000BC  li r5, 0xbc
	ctx.r[5].s64 = 188;
	// 83041EF0: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83041EF4: 4BFD3C95  bl 0x83015b88
	ctx.lr = 0x83041EF8;
	sub_83015B88(ctx, base);
	// 83041EF8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83041EFC: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83041F00: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83041F04: 4816ED25  bl 0x831b0c28
	ctx.lr = 0x83041F08;
	sub_831B0C28(ctx, base);
	// 83041F08: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83041F0C: 40820078  bne 0x83041f84
	if !ctx.cr[0].eq {
	pc = 0x83041F84; continue 'dispatch;
	}
	// 83041F10: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83041F14: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83041F18: 838B9778  lwz r28, -0x6888(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26760 as u32) ) } as u64;
	// 83041F1C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83041F20: 419A0030  beq cr6, 0x83041f50
	if ctx.cr[6].eq {
	pc = 0x83041F50; continue 'dispatch;
	}
	// 83041F24: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83041F28: 3BAB9678  addi r29, r11, -0x6988
	ctx.r[29].s64 = ctx.r[11].s64 + -27016;
	// 83041F2C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83041F30: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83041F34: 4BF91D0D  bl 0x82fd3c40
	ctx.lr = 0x83041F38;
	sub_82FD3C40(ctx, base);
	// 83041F38: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83041F3C: 40820014  bne 0x83041f50
	if !ctx.cr[0].eq {
	pc = 0x83041F50; continue 'dispatch;
	}
	// 83041F40: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83041F44: 3BBD0040  addi r29, r29, 0x40
	ctx.r[29].s64 = ctx.r[29].s64 + 64;
	// 83041F48: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83041F4C: 4198FFE0  blt cr6, 0x83041f2c
	if ctx.cr[6].lt {
	pc = 0x83041F2C; continue 'dispatch;
	}
	// 83041F50: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83041F54: 409A0030  bne cr6, 0x83041f84
	if !ctx.cr[6].eq {
	pc = 0x83041F84; continue 'dispatch;
	}
	// 83041F58: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83041F5C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83041F60: 388BE460  addi r4, r11, -0x1ba0
	ctx.r[4].s64 = ctx.r[11].s64 + -7072;
	// 83041F64: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 83041F68: 38A000CD  li r5, 0xcd
	ctx.r[5].s64 = 205;
	// 83041F6C: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 83041F70: 4BFFFE29  bl 0x83041d98
	ctx.lr = 0x83041F74;
	sub_83041D98(ctx, base);
	// 83041F74: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83041F78: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 83041F7C: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83041F80: 4816ECA9  bl 0x831b0c28
	ctx.lr = 0x83041F84;
	sub_831B0C28(ctx, base);
	// 83041F84: 383F0100  addi r1, r31, 0x100
	ctx.r[1].s64 = ctx.r[31].s64 + 256;
	// 83041F88: 48166228  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041F8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83041F8C size=8
    let mut pc: u32 = 0x83041F8C;
    'dispatch: loop {
        match pc {
            0x83041F8C => {
    //   block [0x83041F8C..0x83041F94)
	// 83041F8C: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83041F90: 8215E4EC  lwz r16, -0x1b14(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-6932 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041F94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041F94 size=88
    let mut pc: u32 = 0x83041F94;
    'dispatch: loop {
        match pc {
            0x83041F94 => {
    //   block [0x83041F94..0x83041FEC)
	// 83041F94: 3BECFF00  addi r31, r12, -0x100
	ctx.r[31].s64 = ctx.r[12].s64 + -256;
	// 83041F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83041F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83041FA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041FA4: 817F0114  lwz r11, 0x114(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 83041FA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83041FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83041FB0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83041FB4: 38C0012B  li r6, 0x12b
	ctx.r[6].s64 = 299;
	// 83041FB8: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83041FBC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83041FC0: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83041FC4: 388BE460  addi r4, r11, -0x1ba0
	ctx.r[4].s64 = ctx.r[11].s64 + -7072;
	// 83041FC8: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 83041FCC: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83041FD0: 38A000B2  li r5, 0xb2
	ctx.r[5].s64 = 178;
	// 83041FD4: 80EB0010  lwz r7, 0x10(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83041FD8: 4BFD3BB1  bl 0x83015b88
	ctx.lr = 0x83041FDC;
	sub_83015B88(ctx, base);
	// 83041FDC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83041FE0: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83041FE4: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83041FE8: 4816EC41  bl 0x831b0c28
	ctx.lr = 0x83041FEC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041FEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041FEC size=48
    let mut pc: u32 = 0x83041FEC;
    'dispatch: loop {
        match pc {
            0x83041FEC => {
    //   block [0x83041FEC..0x8304201C)
	// 83041FEC: 3BECFF00  addi r31, r12, -0x100
	ctx.r[31].s64 = ctx.r[12].s64 + -256;
	// 83041FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83041FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83041FF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041FFC: 817F0114  lwz r11, 0x114(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 83042000: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83042004: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83042008: 4BF962D9  bl 0x82fd82e0
	ctx.lr = 0x8304200C;
	sub_82FD82E0(ctx, base);
	// 8304200C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83042010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83042014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83042018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042020 size=196
    let mut pc: u32 = 0x83042020;
    'dispatch: loop {
        match pc {
            0x83042020 => {
    //   block [0x83042020..0x830420E4)
	// 83042020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83042024: 48166149  bl 0x831a816c
	ctx.lr = 0x83042028;
	sub_831A8130(ctx, base);
	// 83042028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304202C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83042030: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83042034: 3BEB9678  addi r31, r11, -0x6988
	ctx.r[31].s64 = ctx.r[11].s64 + -27016;
	// 83042038: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304203C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83042040: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83042044: 4BF91BFD  bl 0x82fd3c40
	ctx.lr = 0x83042048;
	sub_82FD3C40(ctx, base);
	// 83042048: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304204C: 40820074  bne 0x830420c0
	if !ctx.cr[0].eq {
	pc = 0x830420C0; continue 'dispatch;
	}
	// 83042050: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83042054: 389F0080  addi r4, r31, 0x80
	ctx.r[4].s64 = ctx.r[31].s64 + 128;
	// 83042058: 4BF91BE9  bl 0x82fd3c40
	ctx.lr = 0x8304205C;
	sub_82FD3C40(ctx, base);
	// 8304205C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83042060: 40820060  bne 0x830420c0
	if !ctx.cr[0].eq {
	pc = 0x830420C0; continue 'dispatch;
	}
	// 83042064: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83042068: 389F0040  addi r4, r31, 0x40
	ctx.r[4].s64 = ctx.r[31].s64 + 64;
	// 8304206C: 4BF91BD5  bl 0x82fd3c40
	ctx.lr = 0x83042070;
	sub_82FD3C40(ctx, base);
	// 83042070: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83042074: 40820018  bne 0x8304208c
	if !ctx.cr[0].eq {
	pc = 0x8304208C; continue 'dispatch;
	}
	// 83042078: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304207C: 389F00C0  addi r4, r31, 0xc0
	ctx.r[4].s64 = ctx.r[31].s64 + 192;
	// 83042080: 4BF91BC1  bl 0x82fd3c40
	ctx.lr = 0x83042084;
	sub_82FD3C40(ctx, base);
	// 83042084: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83042088: 41820054  beq 0x830420dc
	if ctx.cr[0].eq {
	pc = 0x830420DC; continue 'dispatch;
	}
	// 8304208C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83042090: 389F0040  addi r4, r31, 0x40
	ctx.r[4].s64 = ctx.r[31].s64 + 64;
	// 83042094: 4BF91BAD  bl 0x82fd3c40
	ctx.lr = 0x83042098;
	sub_82FD3C40(ctx, base);
	// 83042098: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304209C: 40820018  bne 0x830420b4
	if !ctx.cr[0].eq {
	pc = 0x830420B4; continue 'dispatch;
	}
	// 830420A0: 389F00C0  addi r4, r31, 0xc0
	ctx.r[4].s64 = ctx.r[31].s64 + 192;
	// 830420A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830420A8: 4BF91B99  bl 0x82fd3c40
	ctx.lr = 0x830420AC;
	sub_82FD3C40(ctx, base);
	// 830420AC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830420B0: 4182002C  beq 0x830420dc
	if ctx.cr[0].eq {
	pc = 0x830420DC; continue 'dispatch;
	}
	// 830420B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830420B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830420BC: 48166100  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 830420C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830420C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830420C8: 4BF91B79  bl 0x82fd3c40
	ctx.lr = 0x830420CC;
	sub_82FD3C40(ctx, base);
	// 830420CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830420D0: 4082FFE4  bne 0x830420b4
	if !ctx.cr[0].eq {
	pc = 0x830420B4; continue 'dispatch;
	}
	// 830420D4: 389F0080  addi r4, r31, 0x80
	ctx.r[4].s64 = ctx.r[31].s64 + 128;
	// 830420D8: 4BFFFFCC  b 0x830420a4
	pc = 0x830420A4; continue 'dispatch;
	// 830420DC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830420E0: 4BFFFFD8  b 0x830420b8
	pc = 0x830420B8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830420E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830420E8 size=8
    let mut pc: u32 = 0x830420E8;
    'dispatch: loop {
        match pc {
            0x830420E8 => {
    //   block [0x830420E8..0x830420F0)
	// 830420E8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830420EC: 8215E584  lwz r16, -0x1a7c(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-6780 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830420F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830420F0 size=176
    let mut pc: u32 = 0x830420F0;
    'dispatch: loop {
        match pc {
            0x830420F0 => {
    //   block [0x830420F0..0x830421A0)
	// 830420F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830420F4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830420F8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830420FC: 4816606D  bl 0x831a8168
	ctx.lr = 0x83042100;
	sub_831A8130(ctx, base);
	// 83042100: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83042104: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042108: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304210C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83042110: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83042114: 409A0008  bne cr6, 0x8304211c
	if !ctx.cr[6].eq {
	pc = 0x8304211C; continue 'dispatch;
	}
	// 83042118: 83A30004  lwz r29, 4(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304211C: 54CB063F  clrlwi. r11, r6, 0x18
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83042120: 41820028  beq 0x83042148
	if ctx.cr[0].eq {
	pc = 0x83042148; continue 'dispatch;
	}
	// 83042124: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83042128: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8304212C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83042130: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83042134: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83042138: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8304213C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83042140: 4E800421  bctrl
	ctx.lr = 0x83042144;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83042144: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83042148: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8304214C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83042150: 3BCB9678  addi r30, r11, -0x6988
	ctx.r[30].s64 = ctx.r[11].s64 + -27016;
	// 83042154: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83042158: 4BF91AE9  bl 0x82fd3c40
	ctx.lr = 0x8304215C;
	sub_82FD3C40(ctx, base);
	// 8304215C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83042160: 4082002C  bne 0x8304218c
	if !ctx.cr[0].eq {
	pc = 0x8304218C; continue 'dispatch;
	}
	// 83042164: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83042168: 389E0080  addi r4, r30, 0x80
	ctx.r[4].s64 = ctx.r[30].s64 + 128;
	// 8304216C: 4BF91AD5  bl 0x82fd3c40
	ctx.lr = 0x83042170;
	sub_82FD3C40(ctx, base);
	// 83042170: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83042174: 40820018  bne 0x8304218c
	if !ctx.cr[0].eq {
	pc = 0x8304218C; continue 'dispatch;
	}
	// 83042178: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304217C: 387E0040  addi r3, r30, 0x40
	ctx.r[3].s64 = ctx.r[30].s64 + 64;
	// 83042180: 48000014  b 0x83042194
	pc = 0x83042194; continue 'dispatch;
	// 83042184: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83042188: 48000010  b 0x83042198
	pc = 0x83042198; continue 'dispatch;
	// 8304218C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83042190: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83042194: 4BF8E9ED  bl 0x82fd0b80
	ctx.lr = 0x83042198;
	sub_82FD0B80(ctx, base);
	// 83042198: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8304219C: 4816601C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830421A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830421A0 size=8
    let mut pc: u32 = 0x830421A0;
    'dispatch: loop {
        match pc {
            0x830421A0 => {
    //   block [0x830421A0..0x830421A8)
	// 830421A0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830421A4: 8215E584  lwz r16, -0x1a7c(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-6780 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830421A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830421A8 size=20
    let mut pc: u32 = 0x830421A8;
    'dispatch: loop {
        match pc {
            0x830421A8 => {
    //   block [0x830421A8..0x830421BC)
	// 830421A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830421AC: 3C608304  lis r3, -0x7cfc
	ctx.r[3].s64 = -2096889856;
	// 830421B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830421B4: 38632184  addi r3, r3, 0x2184
	ctx.r[3].s64 = ctx.r[3].s64 + 8580;
	// 830421B8: 48166000  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830421C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830421C0 size=8
    let mut pc: u32 = 0x830421C0;
    'dispatch: loop {
        match pc {
            0x830421C0 => {
    //   block [0x830421C0..0x830421C8)
	// 830421C0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830421C4: 8215E5E8  lwz r16, -0x1a18(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-6680 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830421C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830421C8 size=96
    let mut pc: u32 = 0x830421C8;
    'dispatch: loop {
        match pc {
            0x830421C8 => {
    //   block [0x830421C8..0x83042228)
	// 830421C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830421CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830421D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830421D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830421D8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830421DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830421E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830421E4: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 830421E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830421EC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830421F0: 4BF960A9  bl 0x82fd8298
	ctx.lr = 0x830421F4;
	sub_82FD8298(ctx, base);
	// 830421F4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830421F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830421FC: 41820010  beq 0x8304220c
	if ctx.cr[0].eq {
	pc = 0x8304220C; continue 'dispatch;
	}
	// 83042200: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83042204: 4BFCD7B5  bl 0x8300f9b8
	ctx.lr = 0x83042208;
	sub_8300F9B8(ctx, base);
	// 83042208: 48000008  b 0x83042210
	pc = 0x83042210; continue 'dispatch;
	// 8304220C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83042210: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83042214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83042218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304221C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83042220: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83042224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042228 size=44
    let mut pc: u32 = 0x83042228;
    'dispatch: loop {
        match pc {
            0x83042228 => {
    //   block [0x83042228..0x83042254)
	// 83042228: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8304222C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83042230: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83042234: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042238: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8304223C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83042240: 4BF960A1  bl 0x82fd82e0
	ctx.lr = 0x83042244;
	sub_82FD82E0(ctx, base);
	// 83042244: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83042248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304224C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83042250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83042258 size=12
    let mut pc: u32 = 0x83042258;
    'dispatch: loop {
        match pc {
            0x83042258 => {
    //   block [0x83042258..0x83042264)
	// 83042258: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304225C: 386B32E4  addi r3, r11, 0x32e4
	ctx.r[3].s64 = ctx.r[11].s64 + 13028;
	// 83042260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83042268 size=4
    let mut pc: u32 = 0x83042268;
    'dispatch: loop {
        match pc {
            0x83042268 => {
    //   block [0x83042268..0x8304226C)
	// 83042268: 4BFFF3E0  b 0x83041648
	sub_83041648(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83042270 size=8
    let mut pc: u32 = 0x83042270;
    'dispatch: loop {
        match pc {
            0x83042270 => {
    //   block [0x83042270..0x83042278)
	// 83042270: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83042274: 8215E640  lwz r16, -0x19c0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-6592 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042278 size=400
    let mut pc: u32 = 0x83042278;
    'dispatch: loop {
        match pc {
            0x83042278 => {
    //   block [0x83042278..0x83042408)
	// 83042278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304227C: 48165EE9  bl 0x831a8164
	ctx.lr = 0x83042280;
	sub_831A8130(ctx, base);
	// 83042280: 3BE1FF10  addi r31, r1, -0xf0
	ctx.r[31].s64 = ctx.r[1].s64 + -240;
	// 83042284: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042288: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8304228C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83042290: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 83042294: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 83042298: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8304229C: 937F0104  stw r27, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[27].u32 ) };
	// 830422A0: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 830422A4: 4BFFEBED  bl 0x83040e90
	ctx.lr = 0x830422A8;
	sub_83040E90(ctx, base);
	// 830422A8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830422AC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830422B0: 396B292C  addi r11, r11, 0x292c
	ctx.r[11].s64 = ctx.r[11].s64 + 10540;
	// 830422B4: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830422B8: 419A0144  beq cr6, 0x830423fc
	if ctx.cr[6].eq {
	pc = 0x830423FC; continue 'dispatch;
	}
	// 830422BC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830422C0: 419A005C  beq cr6, 0x8304231c
	if ctx.cr[6].eq {
	pc = 0x8304231C; continue 'dispatch;
	}
	// 830422C4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830422C8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830422CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830422D0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830422D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830422D8: 4E800421  bctrl
	ctx.lr = 0x830422DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830422DC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830422E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830422E4: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 830422E8: 38EBE618  addi r7, r11, -0x19e8
	ctx.r[7].s64 = ctx.r[11].s64 + -6632;
	// 830422EC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830422F0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830422F4: 388BE460  addi r4, r11, -0x1ba0
	ctx.r[4].s64 = ctx.r[11].s64 + -7072;
	// 830422F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830422FC: 38C000AB  li r6, 0xab
	ctx.r[6].s64 = 171;
	// 83042300: 38A0007E  li r5, 0x7e
	ctx.r[5].s64 = 126;
	// 83042304: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 83042308: 4BFFF8E9  bl 0x83041bf0
	ctx.lr = 0x8304230C;
	sub_83041BF0(ctx, base);
	// 8304230C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83042310: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 83042314: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 83042318: 4816E911  bl 0x831b0c28
	ctx.lr = 0x8304231C;
	sub_831B0C28(ctx, base);
	// 8304231C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83042320: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83042324: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83042328: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304232C: 4BFA4815  bl 0x82fe6b40
	ctx.lr = 0x83042330;
	sub_82FE6B40(ctx, base);
	// 83042330: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83042334: 3BABCF74  addi r29, r11, -0x308c
	ctx.r[29].s64 = ctx.r[11].s64 + -12428;
	// 83042338: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8304233C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83042340: 409A001C  bne cr6, 0x8304235c
	if !ctx.cr[6].eq {
	pc = 0x8304235C; continue 'dispatch;
	}
	// 83042344: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 83042348: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8304234C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83042350: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83042354: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83042358: 419A0008  beq cr6, 0x83042360
	if ctx.cr[6].eq {
	pc = 0x83042360; continue 'dispatch;
	}
	// 8304235C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83042360: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83042364: 41820090  beq 0x830423f4
	if ctx.cr[0].eq {
	pc = 0x830423F4; continue 'dispatch;
	}
	// 83042368: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304236C: 4BFCB60D  bl 0x8300d978
	ctx.lr = 0x83042370;
	sub_8300D978(ctx, base);
	// 83042370: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83042374: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83042378: 480036D1  bl 0x83045a48
	ctx.lr = 0x8304237C;
	sub_83045A48(ctx, base);
	// 8304237C: 83DF0088  lwz r30, 0x88(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 83042380: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83042384: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83042388: 4BF918B9  bl 0x82fd3c40
	ctx.lr = 0x8304238C;
	sub_82FD3C40(ctx, base);
	// 8304238C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83042390: 41820028  beq 0x830423b8
	if ctx.cr[0].eq {
	pc = 0x830423B8; continue 'dispatch;
	}
	// 83042394: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83042398: 809F0090  lwz r4, 0x90(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8304239C: 4BFFF70D  bl 0x83041aa8
	ctx.lr = 0x830423A0;
	sub_83041AA8(ctx, base);
	// 830423A0: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 830423A4: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 830423A8: 917B0010  stw r11, 0x10(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830423AC: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830423B0: 48003251  bl 0x83045600
	ctx.lr = 0x830423B4;
	sub_83045600(ctx, base);
	// 830423B4: 4BFFFF84  b 0x83042338
	pc = 0x83042338; continue 'dispatch;
	// 830423B8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830423BC: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 830423C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830423C4: 388BE460  addi r4, r11, -0x1ba0
	ctx.r[4].s64 = ctx.r[11].s64 + -7072;
	// 830423C8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830423CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830423D0: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 830423D4: 38C000AB  li r6, 0xab
	ctx.r[6].s64 = 171;
	// 830423D8: 38A00095  li r5, 0x95
	ctx.r[5].s64 = 149;
	// 830423DC: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 830423E0: 4BFFF739  bl 0x83041b18
	ctx.lr = 0x830423E4;
	sub_83041B18(ctx, base);
	// 830423E4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830423E8: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 830423EC: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 830423F0: 4816E839  bl 0x831b0c28
	ctx.lr = 0x830423F4;
	sub_831B0C28(ctx, base);
	// 830423F4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830423F8: 4BFCBDD1  bl 0x8300e1c8
	ctx.lr = 0x830423FC;
	sub_8300E1C8(ctx, base);
	// 830423FC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83042400: 383F00F0  addi r1, r31, 0xf0
	ctx.r[1].s64 = ctx.r[31].s64 + 240;
	// 83042404: 48165DB0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042408 size=40
    let mut pc: u32 = 0x83042408;
    'dispatch: loop {
        match pc {
            0x83042408 => {
    //   block [0x83042408..0x83042430)
	// 83042408: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 8304240C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83042410: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83042414: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042418: 807F0104  lwz r3, 0x104(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 8304241C: 4BFFF5BD  bl 0x830419d8
	ctx.lr = 0x83042420;
	sub_830419D8(ctx, base);
	// 83042420: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83042424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83042428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304242C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042430 size=40
    let mut pc: u32 = 0x83042430;
    'dispatch: loop {
        match pc {
            0x83042430 => {
    //   block [0x83042430..0x83042458)
	// 83042430: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83042434: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83042438: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304243C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042440: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83042444: 4BFCBD85  bl 0x8300e1c8
	ctx.lr = 0x83042448;
	sub_8300E1C8(ctx, base);
	// 83042448: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304244C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83042450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83042454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042458 size=40
    let mut pc: u32 = 0x83042458;
    'dispatch: loop {
        match pc {
            0x83042458 => {
    //   block [0x83042458..0x83042480)
	// 83042458: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 8304245C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83042460: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83042464: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042468: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 8304246C: 48003195  bl 0x83045600
	ctx.lr = 0x83042470;
	sub_83045600(ctx, base);
	// 83042470: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83042474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83042478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304247C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042480 size=128
    let mut pc: u32 = 0x83042480;
    'dispatch: loop {
        match pc {
            0x83042480 => {
    //   block [0x83042480..0x83042500)
	// 83042480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83042484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83042488: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304248C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042490: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83042494: 897F0040  lbz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83042498: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304249C: 40820024  bne 0x830424c0
	if !ctx.cr[0].eq {
	pc = 0x830424C0; continue 'dispatch;
	}
	// 830424A0: 807F0044  lwz r3, 0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 830424A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830424A8: 41820018  beq 0x830424c0
	if ctx.cr[0].eq {
	pc = 0x830424C0; continue 'dispatch;
	}
	// 830424AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830424B0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830424B4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830424B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830424BC: 4E800421  bctrl
	ctx.lr = 0x830424C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830424C0: 897F0041  lbz r11, 0x41(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(65 as u32) ) } as u64;
	// 830424C4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830424C8: 40820024  bne 0x830424ec
	if !ctx.cr[0].eq {
	pc = 0x830424EC; continue 'dispatch;
	}
	// 830424CC: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 830424D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830424D4: 41820018  beq 0x830424ec
	if ctx.cr[0].eq {
	pc = 0x830424EC; continue 'dispatch;
	}
	// 830424D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830424DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830424E0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830424E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830424E8: 4E800421  bctrl
	ctx.lr = 0x830424EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830424EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830424F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830424F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830424F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830424FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042500 size=104
    let mut pc: u32 = 0x83042500;
    'dispatch: loop {
        match pc {
            0x83042500 => {
    //   block [0x83042500..0x83042568)
	// 83042500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83042504: 48165C69  bl 0x831a816c
	ctx.lr = 0x83042508;
	sub_831A8130(ctx, base);
	// 83042508: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304250C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83042510: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83042514: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83042518: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8304251C: 419A0044  beq cr6, 0x83042560
	if ctx.cr[6].eq {
	pc = 0x83042560; continue 'dispatch;
	}
	// 83042520: 897F0040  lbz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83042524: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83042528: 40820024  bne 0x8304254c
	if !ctx.cr[0].eq {
	pc = 0x8304254C; continue 'dispatch;
	}
	// 8304252C: 807F0044  lwz r3, 0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83042530: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83042534: 41820018  beq 0x8304254c
	if ctx.cr[0].eq {
	pc = 0x8304254C; continue 'dispatch;
	}
	// 83042538: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304253C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83042540: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83042544: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83042548: 4E800421  bctrl
	ctx.lr = 0x8304254C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304254C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83042550: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 83042554: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 83042558: 9BBF0040  stb r29, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[29].u8 ) };
	// 8304255C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83042560: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83042564: 48165C58  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83042568 size=8
    let mut pc: u32 = 0x83042568;
    'dispatch: loop {
        match pc {
            0x83042568 => {
    //   block [0x83042568..0x83042570)
	// 83042568: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304256C: 8215E6D0  lwz r16, -0x1930(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-6448 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042570 size=72
    let mut pc: u32 = 0x83042570;
    'dispatch: loop {
        match pc {
            0x83042570 => {
    //   block [0x83042570..0x830425B8)
	// 83042570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83042574: 48165BF9  bl 0x831a816c
	ctx.lr = 0x83042578;
	sub_831A8130(ctx, base);
	// 83042578: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8304257C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042580: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83042584: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83042588: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8304258C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83042590: 4BF969A1  bl 0x82fd8f30
	ctx.lr = 0x83042594;
	sub_82FD8F30(ctx, base);
	// 83042594: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83042598: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304259C: 396BE374  addi r11, r11, -0x1c8c
	ctx.r[11].s64 = ctx.r[11].s64 + -7308;
	// 830425A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830425A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830425A8: 4BF96D11  bl 0x82fd92b8
	ctx.lr = 0x830425AC;
	sub_82FD92B8(ctx, base);
	// 830425AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830425B0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830425B4: 48165C08  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830425B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830425B8 size=40
    let mut pc: u32 = 0x830425B8;
    'dispatch: loop {
        match pc {
            0x830425B8 => {
    //   block [0x830425B8..0x830425E0)
	// 830425B8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830425BC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830425C0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830425C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830425C8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830425CC: 4BF968AD  bl 0x82fd8e78
	ctx.lr = 0x830425D0;
	sub_82FD8E78(ctx, base);
	// 830425D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830425D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830425D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830425DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830425E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830425E0 size=104
    let mut pc: u32 = 0x830425E0;
    'dispatch: loop {
        match pc {
            0x830425E0 => {
    //   block [0x830425E0..0x83042648)
	// 830425E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830425E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830425E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830425EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830425F0: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 830425F4: 38E00019  li r7, 0x19
	ctx.r[7].s64 = 25;
	// 830425F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830425FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83042600: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83042604: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83042608: 4BFFE889  bl 0x83040e90
	ctx.lr = 0x8304260C;
	sub_83040E90(ctx, base);
	// 8304260C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83042610: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83042614: 394BE700  addi r10, r11, -0x1900
	ctx.r[10].s64 = ctx.r[11].s64 + -6400;
	// 83042618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304261C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83042620: 997F0040  stb r11, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 83042624: 997F0041  stb r11, 0x41(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(65 as u32), ctx.r[11].u8 ) };
	// 83042628: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8304262C: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83042630: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 83042634: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83042638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304263C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83042640: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83042644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83042648 size=24
    let mut pc: u32 = 0x83042648;
    'dispatch: loop {
        match pc {
            0x83042648 => {
    //   block [0x83042648..0x83042660)
	// 83042648: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304264C: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 83042650: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83042654: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83042658: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304265C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83042660 size=8
    let mut pc: u32 = 0x83042660;
    'dispatch: loop {
        match pc {
            0x83042660 => {
    //   block [0x83042660..0x83042668)
	// 83042660: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83042664: 8215E738  lwz r16, -0x18c8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-6344 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042668 size=80
    let mut pc: u32 = 0x83042668;
    'dispatch: loop {
        match pc {
            0x83042668 => {
    //   block [0x83042668..0x830426B8)
	// 83042668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304266C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83042670: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83042674: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83042678: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8304267C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042680: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83042684: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83042688: 396BE700  addi r11, r11, -0x1900
	ctx.r[11].s64 = ctx.r[11].s64 + -6400;
	// 8304268C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83042690: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83042694: 4BFFFDED  bl 0x83042480
	ctx.lr = 0x83042698;
	sub_83042480(ctx, base);
	// 83042698: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304269C: 4BFFF33D  bl 0x830419d8
	ctx.lr = 0x830426A0;
	sub_830419D8(ctx, base);
	// 830426A0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830426A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830426A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830426AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830426B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830426B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830426B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830426B8 size=40
    let mut pc: u32 = 0x830426B8;
    'dispatch: loop {
        match pc {
            0x830426B8 => {
    //   block [0x830426B8..0x830426E0)
	// 830426B8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830426BC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830426C0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830426C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830426C8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830426CC: 4BFFF30D  bl 0x830419d8
	ctx.lr = 0x830426D0;
	sub_830419D8(ctx, base);
	// 830426D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830426D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830426D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830426DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830426E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830426E0 size=8
    let mut pc: u32 = 0x830426E0;
    'dispatch: loop {
        match pc {
            0x830426E0 => {
    //   block [0x830426E0..0x830426E8)
	// 830426E0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830426E4: 8215E7C8  lwz r16, -0x1838(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-6200 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830426E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830426E8 size=160
    let mut pc: u32 = 0x830426E8;
    'dispatch: loop {
        match pc {
            0x830426E8 => {
    //   block [0x830426E8..0x83042788)
	// 830426E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830426EC: 48165A7D  bl 0x831a8168
	ctx.lr = 0x830426F0;
	sub_831A8130(ctx, base);
	// 830426F0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830426F4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830426F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830426FC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83042700: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83042704: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 83042708: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 8304270C: 38E00019  li r7, 0x19
	ctx.r[7].s64 = 25;
	// 83042710: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83042714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83042718: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8304271C: 4BFFE775  bl 0x83040e90
	ctx.lr = 0x83042720;
	sub_83040E90(ctx, base);
	// 83042720: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83042724: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83042728: 394BE700  addi r10, r11, -0x1900
	ctx.r[10].s64 = ctx.r[11].s64 + -6400;
	// 8304272C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83042730: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83042734: 997E0040  stb r11, 0x40(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 83042738: 997E0041  stb r11, 0x41(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(65 as u32), ctx.r[11].u8 ) };
	// 8304273C: 917E0044  stw r11, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 83042740: 917E004C  stw r11, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 83042744: 917E0048  stw r11, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83042748: 409A0030  bne cr6, 0x83042778
	if !ctx.cr[6].eq {
	pc = 0x83042778; continue 'dispatch;
	}
	// 8304274C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83042750: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 83042754: 388BE770  addi r4, r11, -0x1890
	ctx.r[4].s64 = ctx.r[11].s64 + -6288;
	// 83042758: 38C000EB  li r6, 0xeb
	ctx.r[6].s64 = 235;
	// 8304275C: 38A000A9  li r5, 0xa9
	ctx.r[5].s64 = 169;
	// 83042760: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83042764: 4BFFFE0D  bl 0x83042570
	ctx.lr = 0x83042768;
	sub_83042570(ctx, base);
	// 83042768: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304276C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83042770: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 83042774: 4816E4B5  bl 0x831b0c28
	ctx.lr = 0x83042778;
	sub_831B0C28(ctx, base);
	// 83042778: 939E0048  stw r28, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[28].u32 ) };
	// 8304277C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83042780: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83042784: 48165A34  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042788 size=40
    let mut pc: u32 = 0x83042788;
    'dispatch: loop {
        match pc {
            0x83042788 => {
    //   block [0x83042788..0x830427B0)
	// 83042788: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8304278C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83042790: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83042794: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042798: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8304279C: 4BFFF23D  bl 0x830419d8
	ctx.lr = 0x830427A0;
	sub_830419D8(ctx, base);
	// 830427A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830427A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830427A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830427AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830427B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830427B0 size=8
    let mut pc: u32 = 0x830427B0;
    'dispatch: loop {
        match pc {
            0x830427B0 => {
    //   block [0x830427B0..0x830427B8)
	// 830427B0: 80630044  lwz r3, 0x44(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 830427B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830427B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830427B8 size=8
    let mut pc: u32 = 0x830427B8;
    'dispatch: loop {
        match pc {
            0x830427B8 => {
    //   block [0x830427B8..0x830427C0)
	// 830427B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830427BC: 8215E800  lwz r16, -0x1800(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-6144 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830427C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830427C0 size=96
    let mut pc: u32 = 0x830427C0;
    'dispatch: loop {
        match pc {
            0x830427C0 => {
    //   block [0x830427C0..0x83042820)
	// 830427C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830427C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830427C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830427CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830427D0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830427D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830427D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830427DC: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 830427E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830427E4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830427E8: 4BF95AB1  bl 0x82fd8298
	ctx.lr = 0x830427EC;
	sub_82FD8298(ctx, base);
	// 830427EC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830427F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830427F4: 41820010  beq 0x83042804
	if ctx.cr[0].eq {
	pc = 0x83042804; continue 'dispatch;
	}
	// 830427F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830427FC: 4BFFFDE5  bl 0x830425e0
	ctx.lr = 0x83042800;
	sub_830425E0(ctx, base);
	// 83042800: 48000008  b 0x83042808
	pc = 0x83042808; continue 'dispatch;
	// 83042804: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83042808: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8304280C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83042810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83042814: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83042818: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304281C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042820 size=44
    let mut pc: u32 = 0x83042820;
    'dispatch: loop {
        match pc {
            0x83042820 => {
    //   block [0x83042820..0x8304284C)
	// 83042820: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83042824: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83042828: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304282C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042830: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83042834: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83042838: 4BF95AA9  bl 0x82fd82e0
	ctx.lr = 0x8304283C;
	sub_82FD82E0(ctx, base);
	// 8304283C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83042840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83042844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83042848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83042850 size=12
    let mut pc: u32 = 0x83042850;
    'dispatch: loop {
        match pc {
            0x83042850 => {
    //   block [0x83042850..0x8304285C)
	// 83042850: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83042854: 386B3324  addi r3, r11, 0x3324
	ctx.r[3].s64 = ctx.r[11].s64 + 13092;
	// 83042858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042860 size=208
    let mut pc: u32 = 0x83042860;
    'dispatch: loop {
        match pc {
            0x83042860 => {
    //   block [0x83042860..0x83042930)
	// 83042860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83042864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83042868: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304286C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83042870: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042874: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83042878: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8304287C: 4BFFEDCD  bl 0x83041648
	ctx.lr = 0x83042880;
	sub_83041648(ctx, base);
	// 83042880: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83042884: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83042888: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8304288C: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83042890: 41820040  beq 0x830428d0
	if ctx.cr[0].eq {
	pc = 0x830428D0; continue 'dispatch;
	}
	// 83042894: 889E0040  lbz r4, 0x40(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83042898: 4BFB6969  bl 0x82ff9200
	ctx.lr = 0x8304289C;
	sub_82FF9200(ctx, base);
	// 8304289C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830428A0: 889E0041  lbz r4, 0x41(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(65 as u32) ) } as u64;
	// 830428A4: 4BFB695D  bl 0x82ff9200
	ctx.lr = 0x830428A8;
	sub_82FF9200(ctx, base);
	// 830428A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830428AC: 807E0044  lwz r3, 0x44(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 830428B0: 4800A211  bl 0x8304cac0
	ctx.lr = 0x830428B4;
	sub_8304CAC0(ctx, base);
	// 830428B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830428B8: 807E0048  lwz r3, 0x48(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 830428BC: 4800A2ED  bl 0x8304cba8
	ctx.lr = 0x830428C0;
	sub_8304CBA8(ctx, base);
	// 830428C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830428C4: 809E004C  lwz r4, 0x4c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 830428C8: 4BFFE9A1  bl 0x83041268
	ctx.lr = 0x830428CC;
	sub_83041268(ctx, base);
	// 830428CC: 4800004C  b 0x83042918
	pc = 0x83042918; continue 'dispatch;
	// 830428D0: 389E0040  addi r4, r30, 0x40
	ctx.r[4].s64 = ctx.r[30].s64 + 64;
	// 830428D4: 4BFB6BAD  bl 0x82ff9480
	ctx.lr = 0x830428D8;
	sub_82FF9480(ctx, base);
	// 830428D8: 389E0041  addi r4, r30, 0x41
	ctx.r[4].s64 = ctx.r[30].s64 + 65;
	// 830428DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830428E0: 4BFB6BA1  bl 0x82ff9480
	ctx.lr = 0x830428E4;
	sub_82FF9480(ctx, base);
	// 830428E4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 830428E8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830428EC: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 830428F0: 387E0044  addi r3, r30, 0x44
	ctx.r[3].s64 = ctx.r[30].s64 + 68;
	// 830428F4: 4800AC85  bl 0x8304d578
	ctx.lr = 0x830428F8;
	sub_8304D578(ctx, base);
	// 830428F8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 830428FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83042900: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83042904: 387E0048  addi r3, r30, 0x48
	ctx.r[3].s64 = ctx.r[30].s64 + 72;
	// 83042908: 4800B191  bl 0x8304da98
	ctx.lr = 0x8304290C;
	sub_8304DA98(ctx, base);
	// 8304290C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83042910: 4BFFEA21  bl 0x83041330
	ctx.lr = 0x83042914;
	sub_83041330(ctx, base);
	// 83042914: 907E004C  stw r3, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[3].u32 ) };
	// 83042918: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304291C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83042920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83042924: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83042928: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304292C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042930 size=112
    let mut pc: u32 = 0x83042930;
    'dispatch: loop {
        match pc {
            0x83042930 => {
    //   block [0x83042930..0x830429A0)
	// 83042930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83042934: 48165839  bl 0x831a816c
	ctx.lr = 0x83042938;
	sub_831A8130(ctx, base);
	// 83042938: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304293C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83042940: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 83042944: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83042948: 4082000C  bne 0x83042954
	if !ctx.cr[0].eq {
	pc = 0x83042954; continue 'dispatch;
	}
	// 8304294C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83042950: 48000048  b 0x83042998
	pc = 0x83042998; continue 'dispatch;
	// 83042954: 83CB0008  lwz r30, 8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83042958: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8304295C: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83042960: 41820034  beq 0x83042994
	if ctx.cr[0].eq {
	pc = 0x83042994; continue 'dispatch;
	}
	// 83042964: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83042968: 807D0048  lwz r3, 0x48(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 8304296C: 4BFE9F05  bl 0x8302c870
	ctx.lr = 0x83042970;
	sub_8302C870(ctx, base);
	// 83042970: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83042974: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83042978: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304297C: 4E800421  bctrl
	ctx.lr = 0x83042980;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83042980: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83042984: 4182FFC8  beq 0x8304294c
	if ctx.cr[0].eq {
	pc = 0x8304294C; continue 'dispatch;
	}
	// 83042988: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8304298C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83042990: 4198FFD4  blt cr6, 0x83042964
	if ctx.cr[6].lt {
	pc = 0x83042964; continue 'dispatch;
	}
	// 83042994: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83042998: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304299C: 48165820  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830429A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830429A0 size=128
    let mut pc: u32 = 0x830429A0;
    'dispatch: loop {
        match pc {
            0x830429A0 => {
    //   block [0x830429A0..0x83042A20)
	// 830429A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830429A4: 481657C5  bl 0x831a8168
	ctx.lr = 0x830429A8;
	sub_831A8130(ctx, base);
	// 830429A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830429AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830429B0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830429B4: 7F1CF040  cmplw cr6, r28, r30
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[30].u32, &mut ctx.xer);
	// 830429B8: 409A000C  bne cr6, 0x830429c4
	if !ctx.cr[6].eq {
	pc = 0x830429C4; continue 'dispatch;
	}
	// 830429BC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830429C0: 48000058  b 0x83042a18
	pc = 0x83042A18; continue 'dispatch;
	// 830429C4: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 830429C8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830429CC: 41820048  beq 0x83042a14
	if ctx.cr[0].eq {
	pc = 0x83042A14; continue 'dispatch;
	}
	// 830429D0: 83AB0008  lwz r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830429D4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830429D8: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830429DC: 41820038  beq 0x83042a14
	if ctx.cr[0].eq {
	pc = 0x83042A14; continue 'dispatch;
	}
	// 830429E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830429E4: 807E0048  lwz r3, 0x48(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 830429E8: 4BFE9E89  bl 0x8302c870
	ctx.lr = 0x830429EC;
	sub_8302C870(ctx, base);
	// 830429EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830429F0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830429F4: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830429F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830429FC: 4E800421  bctrl
	ctx.lr = 0x83042A00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83042A00: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83042A04: 4082FFB8  bne 0x830429bc
	if !ctx.cr[0].eq {
	pc = 0x830429BC; continue 'dispatch;
	}
	// 83042A08: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83042A0C: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83042A10: 4198FFD0  blt cr6, 0x830429e0
	if ctx.cr[6].lt {
	pc = 0x830429E0; continue 'dispatch;
	}
	// 83042A14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83042A18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83042A1C: 4816579C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042A20 size=76
    let mut pc: u32 = 0x83042A20;
    'dispatch: loop {
        match pc {
            0x83042A20 => {
    //   block [0x83042A20..0x83042A6C)
	// 83042A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83042A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83042A28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83042A2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83042A30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042A34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83042A38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83042A3C: 4BFFFC2D  bl 0x83042668
	ctx.lr = 0x83042A40;
	sub_83042668(ctx, base);
	// 83042A40: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83042A44: 4182000C  beq 0x83042a50
	if ctx.cr[0].eq {
	pc = 0x83042A50; continue 'dispatch;
	}
	// 83042A48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83042A4C: 4BF95895  bl 0x82fd82e0
	ctx.lr = 0x83042A50;
	sub_82FD82E0(ctx, base);
	// 83042A50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83042A54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83042A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83042A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83042A60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83042A64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83042A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83042A70 size=8
    let mut pc: u32 = 0x83042A70;
    'dispatch: loop {
        match pc {
            0x83042A70 => {
    //   block [0x83042A70..0x83042A78)
	// 83042A70: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83042A74: 8215E8D4  lwz r16, -0x172c(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-5932 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042A78 size=828
    let mut pc: u32 = 0x83042A78;
    'dispatch: loop {
        match pc {
            0x83042A78 => {
    //   block [0x83042A78..0x83042DB4)
	// 83042A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83042A7C: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83042A80: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83042A84: 481656D5  bl 0x831a8158
	ctx.lr = 0x83042A88;
	sub_831A8130(ctx, base);
	// 83042A88: 3BE1FEC0  addi r31, r1, -0x140
	ctx.r[31].s64 = ctx.r[1].s64 + -320;
	// 83042A8C: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042A90: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83042A94: 98DF016F  stb r6, 0x16f(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(367 as u32), ctx.r[6].u8 ) };
	// 83042A98: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 83042A9C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83042AA0: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 83042AA4: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83042AA8: 93DF0154  stw r30, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[30].u32 ) };
	// 83042AAC: 931F015C  stw r24, 0x15c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[24].u32 ) };
	// 83042AB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83042AB4: 939F0164  stw r28, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[28].u32 ) };
	// 83042AB8: 933F0174  stw r25, 0x174(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), ctx.r[25].u32 ) };
	// 83042ABC: 41820068  beq 0x83042b24
	if ctx.cr[0].eq {
	pc = 0x83042B24; continue 'dispatch;
	}
	// 83042AC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83042AC4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83042AC8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83042ACC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83042AD0: 4E800421  bctrl
	ctx.lr = 0x83042AD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83042AD4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83042AD8: 556B0739  rlwinm. r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83042ADC: 418201A4  beq 0x83042c80
	if ctx.cr[0].eq {
	pc = 0x83042C80; continue 'dispatch;
	}
	// 83042AE0: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83042AE4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83042AE8: 40820144  bne 0x83042c2c
	if !ctx.cr[0].eq {
	pc = 0x83042C2C; continue 'dispatch;
	}
	// 83042AEC: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 83042AF0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83042AF4: 4BF957A5  bl 0x82fd8298
	ctx.lr = 0x83042AF8;
	sub_82FD8298(ctx, base);
	// 83042AF8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83042AFC: 907F0078  stw r3, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[3].u32 ) };
	// 83042B00: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83042B04: 41820120  beq 0x83042c24
	if ctx.cr[0].eq {
	pc = 0x83042C24; continue 'dispatch;
	}
	// 83042B08: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83042B0C: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83042B10: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83042B14: 38ABD918  addi r5, r11, -0x26e8
	ctx.r[5].s64 = ctx.r[11].s64 + -9960;
	// 83042B18: 480475D1  bl 0x8308a0e8
	ctx.lr = 0x83042B1C;
	sub_8308A0E8(ctx, base);
	// 83042B1C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83042B20: 48000108  b 0x83042c28
	pc = 0x83042C28; continue 'dispatch;
	// 83042B24: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83042B28: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83042B2C: 9B7F0060  stb r27, 0x60(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[27].u8 ) };
	// 83042B30: 93BF0064  stw r29, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 83042B34: 807E0048  lwz r3, 0x48(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 83042B38: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83042B3C: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83042B40: 409800A0  bge cr6, 0x83042be0
	if !ctx.cr[6].lt {
	pc = 0x83042BE0; continue 'dispatch;
	}
	// 83042B44: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83042B48: 40820098  bne 0x83042be0
	if !ctx.cr[0].eq {
	pc = 0x83042BE0; continue 'dispatch;
	}
	// 83042B4C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83042B50: 4BFE9D21  bl 0x8302c870
	ctx.lr = 0x83042B54;
	sub_8302C870(ctx, base);
	// 83042B54: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83042B58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83042B5C: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 83042B60: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83042B64: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83042B68: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83042B6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83042B70: 4E800421  bctrl
	ctx.lr = 0x83042B74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83042B74: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83042B78: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 83042B7C: 807E0048  lwz r3, 0x48(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 83042B80: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83042B84: 9B7F0060  stb r27, 0x60(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[27].u8 ) };
	// 83042B88: 4BFE9CE9  bl 0x8302c870
	ctx.lr = 0x83042B8C;
	sub_8302C870(ctx, base);
	// 83042B8C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83042B90: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83042B94: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83042B98: 909E004C  stw r4, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[4].u32 ) };
	// 83042B9C: 419A001C  beq cr6, 0x83042bb8
	if ctx.cr[6].eq {
	pc = 0x83042BB8; continue 'dispatch;
	}
	// 83042BA0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83042BA4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83042BA8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83042BAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83042BB0: 4E800421  bctrl
	ctx.lr = 0x83042BB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83042BB4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83042BB8: 4800001C  b 0x83042bd4
	pc = 0x83042BD4; continue 'dispatch;
	// 83042BBC: 83DF0154  lwz r30, 0x154(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 83042BC0: 833F0174  lwz r25, 0x174(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(372 as u32) ) } as u64;
	// 83042BC4: 839F0164  lwz r28, 0x164(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) } as u64;
	// 83042BC8: 831F015C  lwz r24, 0x15c(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 83042BCC: 8B7F0060  lbz r27, 0x60(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83042BD0: 83BF0064  lwz r29, 0x64(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 83042BD4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83042BD8: 93BF0064  stw r29, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 83042BDC: 4BFFFF58  b 0x83042b34
	pc = 0x83042B34; continue 'dispatch;
	// 83042BE0: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83042BE4: 4082FEF0  bne 0x83042ad4
	if !ctx.cr[0].eq {
	pc = 0x83042AD4; continue 'dispatch;
	}
	// 83042BE8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83042BEC: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 83042BF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83042BF4: 388BE770  addi r4, r11, -0x1890
	ctx.r[4].s64 = ctx.r[11].s64 + -6288;
	// 83042BF8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83042BFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83042C00: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 83042C04: 38C00103  li r6, 0x103
	ctx.r[6].s64 = 259;
	// 83042C08: 38A00179  li r5, 0x179
	ctx.r[5].s64 = 377;
	// 83042C0C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83042C10: 4BFD2F79  bl 0x83015b88
	ctx.lr = 0x83042C14;
	sub_83015B88(ctx, base);
	// 83042C14: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83042C18: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83042C1C: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83042C20: 4816E009  bl 0x831b0c28
	ctx.lr = 0x83042C24;
	sub_831B0C28(ctx, base);
	// 83042C24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83042C28: 907E0028  stw r3, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 83042C2C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 83042C30: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83042C34: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83042C38: 48047AD9  bl 0x8308a710
	ctx.lr = 0x83042C3C;
	sub_8308A710(ctx, base);
	// 83042C3C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83042C40: 40820040  bne 0x83042c80
	if !ctx.cr[0].eq {
	pc = 0x83042C80; continue 'dispatch;
	}
	// 83042C44: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83042C48: 811E0024  lwz r8, 0x24(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83042C4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83042C50: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 83042C54: 388BE770  addi r4, r11, -0x1890
	ctx.r[4].s64 = ctx.r[11].s64 + -6288;
	// 83042C58: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83042C5C: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 83042C60: 38C000EE  li r6, 0xee
	ctx.r[6].s64 = 238;
	// 83042C64: 38A00193  li r5, 0x193
	ctx.r[5].s64 = 403;
	// 83042C68: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 83042C6C: 4BFD2F1D  bl 0x83015b88
	ctx.lr = 0x83042C70;
	sub_83015B88(ctx, base);
	// 83042C70: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83042C74: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 83042C78: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83042C7C: 4816DFAD  bl 0x831b0c28
	ctx.lr = 0x83042C80;
	sub_831B0C28(ctx, base);
	// 83042C80: 897F016F  lbz r11, 0x16f(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(367 as u32) ) } as u64;
	// 83042C84: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83042C88: 40820124  bne 0x83042dac
	if !ctx.cr[0].eq {
	pc = 0x83042DAC; continue 'dispatch;
	}
	// 83042C8C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83042C90: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83042C94: 41820118  beq 0x83042dac
	if ctx.cr[0].eq {
	pc = 0x83042DAC; continue 'dispatch;
	}
	// 83042C98: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 83042C9C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83042CA0: 4182010C  beq 0x83042dac
	if ctx.cr[0].eq {
	pc = 0x83042DAC; continue 'dispatch;
	}
	// 83042CA4: 83BE0048  lwz r29, 0x48(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 83042CA8: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 83042CAC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83042CB0: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83042CB4: 93BF0074  stw r29, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 83042CB8: 939F0078  stw r28, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[28].u32 ) };
	// 83042CBC: 937F0068  stw r27, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 83042CC0: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83042CC4: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83042CC8: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 83042CCC: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 83042CD0: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83042CD4: 4098009C  bge cr6, 0x83042d70
	if !ctx.cr[6].lt {
	pc = 0x83042D70; continue 'dispatch;
	}
	// 83042CD8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83042CDC: 935F0064  stw r26, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[26].u32 ) };
	// 83042CE0: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 83042CE4: 7F1A5840  cmplw cr6, r26, r11
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83042CE8: 4098007C  bge cr6, 0x83042d64
	if !ctx.cr[6].lt {
	pc = 0x83042D64; continue 'dispatch;
	}
	// 83042CEC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83042CF0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83042CF4: 4BFE9B7D  bl 0x8302c870
	ctx.lr = 0x83042CF8;
	sub_8302C870(ctx, base);
	// 83042CF8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83042CFC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83042D00: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83042D04: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83042D08: 4BFE9B69  bl 0x8302c870
	ctx.lr = 0x83042D0C;
	sub_8302C870(ctx, base);
	// 83042D0C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83042D10: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83042D14: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83042D18: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 83042D1C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83042D20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83042D24: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83042D28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83042D2C: 4E800421  bctrl
	ctx.lr = 0x83042D30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83042D30: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83042D34: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83042D38: 419A0074  beq cr6, 0x83042dac
	if ctx.cr[6].eq {
	pc = 0x83042DAC; continue 'dispatch;
	}
	// 83042D3C: 4800001C  b 0x83042d58
	pc = 0x83042D58; continue 'dispatch;
	// 83042D40: 833F0174  lwz r25, 0x174(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(372 as u32) ) } as u64;
	// 83042D44: 831F015C  lwz r24, 0x15c(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 83042D48: 83BF0074  lwz r29, 0x74(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 83042D4C: 839F0078  lwz r28, 0x78(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83042D50: 837F0068  lwz r27, 0x68(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83042D54: 835F0064  lwz r26, 0x64(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 83042D58: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 83042D5C: 935F0064  stw r26, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[26].u32 ) };
	// 83042D60: 4BFFFF80  b 0x83042ce0
	pc = 0x83042CE0; continue 'dispatch;
	// 83042D64: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 83042D68: 937F0068  stw r27, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 83042D6C: 4BFFFF60  b 0x83042ccc
	pc = 0x83042CCC; continue 'dispatch;
	// 83042D70: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83042D74: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 83042D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83042D7C: 388BE770  addi r4, r11, -0x1890
	ctx.r[4].s64 = ctx.r[11].s64 + -6288;
	// 83042D80: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83042D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83042D88: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 83042D8C: 38C000F4  li r6, 0xf4
	ctx.r[6].s64 = 244;
	// 83042D90: 38A001B8  li r5, 0x1b8
	ctx.r[5].s64 = 440;
	// 83042D94: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 83042D98: 4BFD2DF1  bl 0x83015b88
	ctx.lr = 0x83042D9C;
	sub_83015B88(ctx, base);
	// 83042D9C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83042DA0: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 83042DA4: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83042DA8: 4816DE81  bl 0x831b0c28
	ctx.lr = 0x83042DAC;
	sub_831B0C28(ctx, base);
	// 83042DAC: 383F0140  addi r1, r31, 0x140
	ctx.r[1].s64 = ctx.r[31].s64 + 320;
	// 83042DB0: 481653F8  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042DB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83042DB4 size=8
    let mut pc: u32 = 0x83042DB4;
    'dispatch: loop {
        match pc {
            0x83042DB4 => {
    //   block [0x83042DB4..0x83042DBC)
	// 83042DB4: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83042DB8: 8215E8D4  lwz r16, -0x172c(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-5932 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042DBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042DBC size=20
    let mut pc: u32 = 0x83042DBC;
    'dispatch: loop {
        match pc {
            0x83042DBC => {
    //   block [0x83042DBC..0x83042DD0)
	// 83042DBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042DC0: 3C608304  lis r3, -0x7cfc
	ctx.r[3].s64 = -2096889856;
	// 83042DC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83042DC8: 38632BBC  addi r3, r3, 0x2bbc
	ctx.r[3].s64 = ctx.r[3].s64 + 11196;
	// 83042DCC: 481653DC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83042DD0 size=8
    let mut pc: u32 = 0x83042DD0;
    'dispatch: loop {
        match pc {
            0x83042DD0 => {
    //   block [0x83042DD0..0x83042DD8)
	// 83042DD0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83042DD4: 8215E8D4  lwz r16, -0x172c(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-5932 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042DD8 size=84
    let mut pc: u32 = 0x83042DD8;
    'dispatch: loop {
        match pc {
            0x83042DD8 => {
    //   block [0x83042DD8..0x83042E2C)
	// 83042DD8: 3BECFEC0  addi r31, r12, -0x140
	ctx.r[31].s64 = ctx.r[12].s64 + -320;
	// 83042DDC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83042DE0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83042DE4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042DE8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83042DEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83042DF0: 388BE770  addi r4, r11, -0x1890
	ctx.r[4].s64 = ctx.r[11].s64 + -6288;
	// 83042DF4: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83042DF8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83042DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83042E00: 38C0012B  li r6, 0x12b
	ctx.r[6].s64 = 299;
	// 83042E04: 38A00189  li r5, 0x189
	ctx.r[5].s64 = 393;
	// 83042E08: 80EB0010  lwz r7, 0x10(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83042E0C: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 83042E10: 817F0174  lwz r11, 0x174(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(372 as u32) ) } as u64;
	// 83042E14: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83042E18: 4BFD2D71  bl 0x83015b88
	ctx.lr = 0x83042E1C;
	sub_83015B88(ctx, base);
	// 83042E1C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83042E20: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 83042E24: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83042E28: 4816DE01  bl 0x831b0c28
	ctx.lr = 0x83042E2C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042E34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042E34 size=20
    let mut pc: u32 = 0x83042E34;
    'dispatch: loop {
        match pc {
            0x83042E34 => {
    //   block [0x83042E34..0x83042E48)
	// 83042E34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042E38: 3C608304  lis r3, -0x7cfc
	ctx.r[3].s64 = -2096889856;
	// 83042E3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83042E40: 38632D40  addi r3, r3, 0x2d40
	ctx.r[3].s64 = ctx.r[3].s64 + 11584;
	// 83042E44: 48165364  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042E48 size=48
    let mut pc: u32 = 0x83042E48;
    'dispatch: loop {
        match pc {
            0x83042E48 => {
    //   block [0x83042E48..0x83042E78)
	// 83042E48: 3BECFEC0  addi r31, r12, -0x140
	ctx.r[31].s64 = ctx.r[12].s64 + -320;
	// 83042E4C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83042E50: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83042E54: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042E58: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 83042E5C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83042E60: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83042E64: 4BF9547D  bl 0x82fd82e0
	ctx.lr = 0x83042E68;
	sub_82FD82E0(ctx, base);
	// 83042E68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83042E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83042E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83042E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042E78 size=124
    let mut pc: u32 = 0x83042E78;
    'dispatch: loop {
        match pc {
            0x83042E78 => {
    //   block [0x83042E78..0x83042EF4)
	// 83042E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83042E7C: 481652E5  bl 0x831a8160
	ctx.lr = 0x83042E80;
	sub_831A8130(ctx, base);
	// 83042E80: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042E84: 83C30048  lwz r30, 0x48(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 83042E88: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83042E8C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83042E90: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83042E94: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83042E98: 835E0008  lwz r26, 8(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83042E9C: 281A0000  cmplwi r26, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83042EA0: 41820040  beq 0x83042ee0
	if ctx.cr[0].eq {
	pc = 0x83042EE0; continue 'dispatch;
	}
	// 83042EA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83042EA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83042EAC: 4BFE99C5  bl 0x8302c870
	ctx.lr = 0x83042EB0;
	sub_8302C870(ctx, base);
	// 83042EB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83042EB4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83042EB8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83042EBC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83042EC0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83042EC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83042EC8: 4E800421  bctrl
	ctx.lr = 0x83042ECC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83042ECC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83042ED0: 4182001C  beq 0x83042eec
	if ctx.cr[0].eq {
	pc = 0x83042EEC; continue 'dispatch;
	}
	// 83042ED4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83042ED8: 7F1FD040  cmplw cr6, r31, r26
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[26].u32, &mut ctx.xer);
	// 83042EDC: 4198FFC8  blt cr6, 0x83042ea4
	if ctx.cr[6].lt {
	pc = 0x83042EA4; continue 'dispatch;
	}
	// 83042EE0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83042EE4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83042EE8: 481652C8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83042EEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83042EF0: 4BFFFFF4  b 0x83042ee4
	pc = 0x83042EE4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83042EF8 size=8
    let mut pc: u32 = 0x83042EF8;
    'dispatch: loop {
        match pc {
            0x83042EF8 => {
    //   block [0x83042EF8..0x83042F00)
	// 83042EF8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83042EFC: 8215EA28  lwz r16, -0x15d8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-5592 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83042F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83042F00 size=308
    let mut pc: u32 = 0x83042F00;
    'dispatch: loop {
        match pc {
            0x83042F00 => {
    //   block [0x83042F00..0x83043034)
	// 83042F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83042F04: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83042F08: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83042F0C: 48165259  bl 0x831a8164
	ctx.lr = 0x83042F10;
	sub_831A8130(ctx, base);
	// 83042F10: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83042F14: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83042F18: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83042F1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83042F20: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83042F24: 937F00AC  stw r27, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[27].u32 ) };
	// 83042F28: 409A0008  bne cr6, 0x83042f30
	if !ctx.cr[6].eq {
	pc = 0x83042F30; continue 'dispatch;
	}
	// 83042F2C: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83042F30: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83042F34: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 83042F38: 54CB063F  clrlwi. r11, r6, 0x18
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83042F3C: 939F0058  stw r28, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 83042F40: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83042F44: 4182002C  beq 0x83042f70
	if ctx.cr[0].eq {
	pc = 0x83042F70; continue 'dispatch;
	}
	// 83042F48: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83042F4C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 83042F50: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83042F54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83042F58: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83042F5C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83042F60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83042F64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83042F68: 4E800421  bctrl
	ctx.lr = 0x83042F6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83042F6C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83042F70: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83042F74: 48000018  b 0x83042f8c
	pc = 0x83042F8C; continue 'dispatch;
	// 83042F78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83042F7C: 480000B0  b 0x8304302c
	pc = 0x8304302C; continue 'dispatch;
	// 83042F80: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 83042F84: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83042F88: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83042F8C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83042F90: 4082FFF0  bne 0x83042f80
	if !ctx.cr[0].eq {
	pc = 0x83042F80; continue 'dispatch;
	}
	// 83042F94: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83042F98: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83042F9C: 807D0048  lwz r3, 0x48(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 83042FA0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83042FA4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83042FA8: 40980080  bge cr6, 0x83043028
	if !ctx.cr[6].lt {
	pc = 0x83043028; continue 'dispatch;
	}
	// 83042FAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83042FB0: 4BFE98C1  bl 0x8302c870
	ctx.lr = 0x83042FB4;
	sub_8302C870(ctx, base);
	// 83042FB4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83042FB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83042FBC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83042FC0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83042FC4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83042FC8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83042FCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83042FD0: 4E800421  bctrl
	ctx.lr = 0x83042FD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83042FD4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83042FD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83042FDC: 807D0048  lwz r3, 0x48(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 83042FE0: 4BFE9891  bl 0x8302c870
	ctx.lr = 0x83042FE4;
	sub_8302C870(ctx, base);
	// 83042FE4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83042FE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83042FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83042FF0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83042FF4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83042FF8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83042FFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83043000: 4E800421  bctrl
	ctx.lr = 0x83043004;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83043004: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83043008: 48000024  b 0x8304302c
	pc = 0x8304302C; continue 'dispatch;
	// 8304300C: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83043010: 837F00AC  lwz r27, 0xac(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 83043014: 3BCB0001  addi r30, r11, 1
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	// 83043018: 839F0058  lwz r28, 0x58(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304301C: 83BF0054  lwz r29, 0x54(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83043020: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83043024: 4BFFFF78  b 0x83042f9c
	pc = 0x83042F9C; continue 'dispatch;
	// 83043028: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304302C: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83043030: 48165184  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043034(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83043034 size=8
    let mut pc: u32 = 0x83043034;
    'dispatch: loop {
        match pc {
            0x83043034 => {
    //   block [0x83043034..0x8304303C)
	// 83043034: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83043038: 8215EA28  lwz r16, -0x15d8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-5592 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304303C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304303C size=20
    let mut pc: u32 = 0x8304303C;
    'dispatch: loop {
        match pc {
            0x8304303C => {
    //   block [0x8304303C..0x83043050)
	// 8304303C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043040: 3C608304  lis r3, -0x7cfc
	ctx.r[3].s64 = -2096889856;
	// 83043044: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83043048: 38632F78  addi r3, r3, 0x2f78
	ctx.r[3].s64 = ctx.r[3].s64 + 12152;
	// 8304304C: 48165168  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83043050 size=8
    let mut pc: u32 = 0x83043050;
    'dispatch: loop {
        match pc {
            0x83043050 => {
    //   block [0x83043050..0x83043058)
	// 83043050: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83043054: 8215EA28  lwz r16, -0x15d8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-5592 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83043058 size=20
    let mut pc: u32 = 0x83043058;
    'dispatch: loop {
        match pc {
            0x83043058 => {
    //   block [0x83043058..0x8304306C)
	// 83043058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304305C: 3C608304  lis r3, -0x7cfc
	ctx.r[3].s64 = -2096889856;
	// 83043060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83043064: 3863300C  addi r3, r3, 0x300c
	ctx.r[3].s64 = ctx.r[3].s64 + 12300;
	// 83043068: 4816514C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83043070 size=8
    let mut pc: u32 = 0x83043070;
    'dispatch: loop {
        match pc {
            0x83043070 => {
    //   block [0x83043070..0x83043078)
	// 83043070: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83043074: 8215EB0C  lwz r16, -0x14f4(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-5364 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83043078 size=468
    let mut pc: u32 = 0x83043078;
    'dispatch: loop {
        match pc {
            0x83043078 => {
    //   block [0x83043078..0x8304324C)
	// 83043078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304307C: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83043080: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83043084: 481650DD  bl 0x831a8160
	ctx.lr = 0x83043088;
	sub_831A8130(ctx, base);
	// 83043088: 3BE1FEE0  addi r31, r1, -0x120
	ctx.r[31].s64 = ctx.r[1].s64 + -288;
	// 8304308C: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043090: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83043094: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 83043098: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8304309C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830430A0: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 830430A4: 93BF0134  stw r29, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[29].u32 ) };
	// 830430A8: 937F0154  stw r27, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[27].u32 ) };
	// 830430AC: 419A0010  beq cr6, 0x830430bc
	if ctx.cr[6].eq {
	pc = 0x830430BC; continue 'dispatch;
	}
	// 830430B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830430B4: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 830430B8: 4BFFF449  bl 0x83042500
	ctx.lr = 0x830430BC;
	sub_83042500(ctx, base);
	// 830430BC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830430C0: 419A015C  beq cr6, 0x8304321c
	if ctx.cr[6].eq {
	pc = 0x8304321C; continue 'dispatch;
	}
	// 830430C4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830430C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830430CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830430D0: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 830430D4: 4BFA3A6D  bl 0x82fe6b40
	ctx.lr = 0x830430D8;
	sub_82FE6B40(ctx, base);
	// 830430D8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830430DC: 3B8BCF74  addi r28, r11, -0x308c
	ctx.r[28].s64 = ctx.r[11].s64 + -12428;
	// 830430E0: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830430E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830430E8: 409A001C  bne cr6, 0x83043104
	if !ctx.cr[6].eq {
	pc = 0x83043104; continue 'dispatch;
	}
	// 830430EC: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830430F0: 815F007C  lwz r10, 0x7c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 830430F4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830430F8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830430FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83043100: 419A0008  beq cr6, 0x83043108
	if ctx.cr[6].eq {
	pc = 0x83043108; continue 'dispatch;
	}
	// 83043104: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83043108: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304310C: 4182009C  beq 0x830431a8
	if ctx.cr[0].eq {
	pc = 0x830431A8; continue 'dispatch;
	}
	// 83043110: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83043114: 4BFCA865  bl 0x8300d978
	ctx.lr = 0x83043118;
	sub_8300D978(ctx, base);
	// 83043118: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304311C: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83043120: 48002929  bl 0x83045a48
	ctx.lr = 0x83043124;
	sub_83045A48(ctx, base);
	// 83043124: 83DF0098  lwz r30, 0x98(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 83043128: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304312C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83043130: 4BF90B11  bl 0x82fd3c40
	ctx.lr = 0x83043134;
	sub_82FD3C40(ctx, base);
	// 83043134: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83043138: 41820034  beq 0x8304316c
	if ctx.cr[0].eq {
	pc = 0x8304316C; continue 'dispatch;
	}
	// 8304313C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83043140: 809F00A0  lwz r4, 0xa0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 83043144: 4BFFE965  bl 0x83041aa8
	ctx.lr = 0x83043148;
	sub_83041AA8(ctx, base);
	// 83043148: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8304314C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83043150: 41820010  beq 0x83043160
	if ctx.cr[0].eq {
	pc = 0x83043160; continue 'dispatch;
	}
	// 83043154: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 83043158: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 8304315C: 917D0010  stw r11, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83043160: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83043164: 4800249D  bl 0x83045600
	ctx.lr = 0x83043168;
	sub_83045600(ctx, base);
	// 83043168: 4BFFFF78  b 0x830430e0
	pc = 0x830430E0; continue 'dispatch;
	// 8304316C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83043170: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 83043174: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83043178: 388BE770  addi r4, r11, -0x1890
	ctx.r[4].s64 = ctx.r[11].s64 + -6288;
	// 8304317C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83043180: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83043184: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 83043188: 38C000AB  li r6, 0xab
	ctx.r[6].s64 = 171;
	// 8304318C: 38A00105  li r5, 0x105
	ctx.r[5].s64 = 261;
	// 83043190: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 83043194: 4BFFE985  bl 0x83041b18
	ctx.lr = 0x83043198;
	sub_83041B18(ctx, base);
	// 83043198: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304319C: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 830431A0: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 830431A4: 4816DA85  bl 0x831b0c28
	ctx.lr = 0x830431A8;
	sub_831B0C28(ctx, base);
	// 830431A8: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830431AC: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830431B0: 41820064  beq 0x83043214
	if ctx.cr[0].eq {
	pc = 0x83043214; continue 'dispatch;
	}
	// 830431B4: 817D0044  lwz r11, 0x44(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(68 as u32) ) } as u64;
	// 830431B8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830431BC: 41820058  beq 0x83043214
	if ctx.cr[0].eq {
	pc = 0x83043214; continue 'dispatch;
	}
	// 830431C0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830431C4: 838B0008  lwz r28, 8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830431C8: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 830431CC: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 830431D0: 40980044  bge cr6, 0x83043214
	if !ctx.cr[6].lt {
	pc = 0x83043214; continue 'dispatch;
	}
	// 830431D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830431D8: 807D0044  lwz r3, 0x44(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(68 as u32) ) } as u64;
	// 830431DC: 4BFE9695  bl 0x8302c870
	ctx.lr = 0x830431E0;
	sub_8302C870(ctx, base);
	// 830431E0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830431E4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830431E8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830431EC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830431F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830431F4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830431F8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 830431FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83043200: 4E800421  bctrl
	ctx.lr = 0x83043204;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83043204: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83043208: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8304320C: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 83043210: 4BFFFFBC  b 0x830431cc
	pc = 0x830431CC; continue 'dispatch;
	// 83043214: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83043218: 4BFCAFB1  bl 0x8300e1c8
	ctx.lr = 0x8304321C;
	sub_8300E1C8(ctx, base);
	// 8304321C: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 83043220: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83043224: 41820020  beq 0x83043244
	if ctx.cr[0].eq {
	pc = 0x83043244; continue 'dispatch;
	}
	// 83043228: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304322C: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83043230: 40820014  bne 0x83043244
	if !ctx.cr[0].eq {
	pc = 0x83043244; continue 'dispatch;
	}
	// 83043234: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83043238: 809A0044  lwz r4, 0x44(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(68 as u32) ) } as u64;
	// 8304323C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83043240: 4BFFF2C1  bl 0x83042500
	ctx.lr = 0x83043244;
	sub_83042500(ctx, base);
	// 83043244: 383F0120  addi r1, r31, 0x120
	ctx.r[1].s64 = ctx.r[31].s64 + 288;
	// 83043248: 48164F68  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304324C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304324C size=8
    let mut pc: u32 = 0x8304324C;
    'dispatch: loop {
        match pc {
            0x8304324C => {
    //   block [0x8304324C..0x83043254)
	// 8304324C: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83043250: 8215EB0C  lwz r16, -0x14f4(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-5364 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043254(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83043254 size=96
    let mut pc: u32 = 0x83043254;
    'dispatch: loop {
        match pc {
            0x83043254 => {
    //   block [0x83043254..0x830432B4)
	// 83043254: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 83043258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304325C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83043260: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043264: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 83043268: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304326C: 83DF0154  lwz r30, 0x154(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 83043270: 806B0044  lwz r3, 0x44(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 83043274: 4BFE95FD  bl 0x8302c870
	ctx.lr = 0x83043278;
	sub_8302C870(ctx, base);
	// 83043278: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304327C: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 83043280: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 83043284: 388BE770  addi r4, r11, -0x1890
	ctx.r[4].s64 = ctx.r[11].s64 + -6288;
	// 83043288: 38C000B5  li r6, 0xb5
	ctx.r[6].s64 = 181;
	// 8304328C: 38A0012B  li r5, 0x12b
	ctx.r[5].s64 = 299;
	// 83043290: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 83043294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83043298: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304329C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830432A0: 4BFFE879  bl 0x83041b18
	ctx.lr = 0x830432A4;
	sub_83041B18(ctx, base);
	// 830432A4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830432A8: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 830432AC: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 830432B0: 4816D979  bl 0x831b0c28
	ctx.lr = 0x830432B4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830432B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830432B4 size=40
    let mut pc: u32 = 0x830432B4;
    'dispatch: loop {
        match pc {
            0x830432B4 => {
    //   block [0x830432B4..0x830432DC)
	// 830432B4: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 830432B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830432BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830432C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830432C4: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 830432C8: 4BFCAF01  bl 0x8300e1c8
	ctx.lr = 0x830432CC;
	sub_8300E1C8(ctx, base);
	// 830432CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830432D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830432D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830432D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830432DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830432DC size=40
    let mut pc: u32 = 0x830432DC;
    'dispatch: loop {
        match pc {
            0x830432DC => {
    //   block [0x830432DC..0x83043304)
	// 830432DC: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 830432E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830432E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830432E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830432EC: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 830432F0: 48002311  bl 0x83045600
	ctx.lr = 0x830432F4;
	sub_83045600(ctx, base);
	// 830432F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830432F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830432FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83043300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83043308 size=8
    let mut pc: u32 = 0x83043308;
    'dispatch: loop {
        match pc {
            0x83043308 => {
    //   block [0x83043308..0x83043310)
	// 83043308: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304330C: 8215EBE4  lwz r16, -0x141c(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-5148 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83043310 size=292
    let mut pc: u32 = 0x83043310;
    'dispatch: loop {
        match pc {
            0x83043310 => {
    //   block [0x83043310..0x83043434)
	// 83043310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83043314: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83043318: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 8304331C: 48164E3D  bl 0x831a8158
	ctx.lr = 0x83043320;
	sub_831A8130(ctx, base);
	// 83043320: 3BE1FE90  addi r31, r1, -0x170
	ctx.r[31].s64 = ctx.r[1].s64 + -368;
	// 83043324: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043328: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304332C: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 83043330: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 83043334: 38E00019  li r7, 0x19
	ctx.r[7].s64 = 25;
	// 83043338: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304333C: 93DF0184  stw r30, 0x184(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(388 as u32), ctx.r[30].u32 ) };
	// 83043340: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83043344: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83043348: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 8304334C: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 83043350: 4BFFDB41  bl 0x83040e90
	ctx.lr = 0x83043354;
	sub_83040E90(ctx, base);
	// 83043354: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83043358: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8304335C: 9B3E0041  stb r25, 0x41(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(65 as u32), ctx.r[25].u8 ) };
	// 83043360: 394BE700  addi r10, r11, -0x1900
	ctx.r[10].s64 = ctx.r[11].s64 + -6400;
	// 83043364: 935E0048  stw r26, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[26].u32 ) };
	// 83043368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304336C: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83043370: 997E0040  stb r11, 0x40(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 83043374: 917E0044  stw r11, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 83043378: 917E004C  stw r11, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8304337C: 409A0030  bne cr6, 0x830433ac
	if !ctx.cr[6].eq {
	pc = 0x830433AC; continue 'dispatch;
	}
	// 83043380: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83043384: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 83043388: 388BE770  addi r4, r11, -0x1890
	ctx.r[4].s64 = ctx.r[11].s64 + -6288;
	// 8304338C: 38C000EC  li r6, 0xec
	ctx.r[6].s64 = 236;
	// 83043390: 38A000C8  li r5, 0xc8
	ctx.r[5].s64 = 200;
	// 83043394: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83043398: 4BFFF1D9  bl 0x83042570
	ctx.lr = 0x8304339C;
	sub_83042570(ctx, base);
	// 8304339C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830433A0: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830433A4: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 830433A8: 4816D881  bl 0x831b0c28
	ctx.lr = 0x830433AC;
	sub_831B0C28(ctx, base);
	// 830433AC: 807C0018  lwz r3, 0x18(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 830433B0: 2F030019  cmpwi cr6, r3, 0x19
	ctx.cr[6].compare_i32(ctx.r[3].s32, 25, &mut ctx.xer);
	// 830433B4: 419A0054  beq cr6, 0x83043408
	if ctx.cr[6].eq {
	pc = 0x83043408; continue 'dispatch;
	}
	// 830433B8: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 830433BC: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 830433C0: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 830433C4: 389F00A0  addi r4, r31, 0xa0
	ctx.r[4].s64 = ctx.r[31].s64 + 160;
	// 830433C8: 4BF8E4A9  bl 0x82fd1870
	ctx.lr = 0x830433CC;
	sub_82FD1870(ctx, base);
	// 830433CC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830433D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830433D4: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 830433D8: 388BE770  addi r4, r11, -0x1890
	ctx.r[4].s64 = ctx.r[11].s64 + -6288;
	// 830433DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830433E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830433E4: 38FF00A0  addi r7, r31, 0xa0
	ctx.r[7].s64 = ctx.r[31].s64 + 160;
	// 830433E8: 38C000ED  li r6, 0xed
	ctx.r[6].s64 = 237;
	// 830433EC: 38A000D2  li r5, 0xd2
	ctx.r[5].s64 = 210;
	// 830433F0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830433F4: 4BFFE725  bl 0x83041b18
	ctx.lr = 0x830433F8;
	sub_83041B18(ctx, base);
	// 830433F8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830433FC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83043400: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 83043404: 4816D825  bl 0x831b0c28
	ctx.lr = 0x83043408;
	sub_831B0C28(ctx, base);
	// 83043408: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8304340C: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 83043410: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83043414: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83043418: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304341C: 4BFFFC5D  bl 0x83043078
	ctx.lr = 0x83043420;
	sub_83043078(ctx, base);
	// 83043420: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83043424: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83043428: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304342C: 383F0170  addi r1, r31, 0x170
	ctx.r[1].s64 = ctx.r[31].s64 + 368;
	// 83043430: 48164D78  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043434(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83043434 size=8
    let mut pc: u32 = 0x83043434;
    'dispatch: loop {
        match pc {
            0x83043434 => {
    //   block [0x83043434..0x8304343C)
	// 83043434: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83043438: 8215EBE4  lwz r16, -0x141c(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-5148 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304343C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304343C size=24
    let mut pc: u32 = 0x8304343C;
    'dispatch: loop {
        match pc {
            0x8304343C => {
    //   block [0x8304343C..0x83043454)
	// 8304343C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83043440: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83043444: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043448: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8304344C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83043450: 4816D7D9  bl 0x831b0c28
	ctx.lr = 0x83043454;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304345C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304345C size=36
    let mut pc: u32 = 0x8304345C;
    'dispatch: loop {
        match pc {
            0x8304345C => {
    //   block [0x8304345C..0x83043480)
	// 8304345C: 3BECFE90  addi r31, r12, -0x170
	ctx.r[31].s64 = ctx.r[12].s64 + -368;
	// 83043460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83043464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83043468: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304346C: 807F0184  lwz r3, 0x184(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(388 as u32) ) } as u64;
	// 83043470: 4BFFF011  bl 0x83042480
	ctx.lr = 0x83043474;
	sub_83042480(ctx, base);
	// 83043474: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83043478: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304347C: 4816D7AD  bl 0x831b0c28
	ctx.lr = 0x83043480;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83043480 size=40
    let mut pc: u32 = 0x83043480;
    'dispatch: loop {
        match pc {
            0x83043480 => {
    //   block [0x83043480..0x830434A8)
	// 83043480: 3BECFE90  addi r31, r12, -0x170
	ctx.r[31].s64 = ctx.r[12].s64 + -368;
	// 83043484: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83043488: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304348C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043490: 807F0184  lwz r3, 0x184(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(388 as u32) ) } as u64;
	// 83043494: 4BFFE545  bl 0x830419d8
	ctx.lr = 0x83043498;
	sub_830419D8(ctx, base);
	// 83043498: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304349C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830434A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830434A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830434A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830434A8 size=8
    let mut pc: u32 = 0x830434A8;
    'dispatch: loop {
        match pc {
            0x830434A8 => {
    //   block [0x830434A8..0x830434B0)
	// 830434A8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830434AC: 8215EC48  lwz r16, -0x13b8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-5048 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830434B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830434B0 size=112
    let mut pc: u32 = 0x830434B0;
    'dispatch: loop {
        match pc {
            0x830434B0 => {
    //   block [0x830434B0..0x83043520)
	// 830434B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830434B4: 48164CAD  bl 0x831a8160
	ctx.lr = 0x830434B8;
	sub_831A8130(ctx, base);
	// 830434B8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830434BC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830434C0: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 830434C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830434C8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830434CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830434D0: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 830434D4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830434D8: 93BF00C4  stw r29, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[29].u32 ) };
	// 830434DC: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 830434E0: 4BF94DB9  bl 0x82fd8298
	ctx.lr = 0x830434E4;
	sub_82FD8298(ctx, base);
	// 830434E4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830434E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830434EC: 41820028  beq 0x83043514
	if ctx.cr[0].eq {
	pc = 0x83043514; continue 'dispatch;
	}
	// 830434F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830434F4: 813E0048  lwz r9, 0x48(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 830434F8: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 830434FC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83043500: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83043504: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83043508: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304350C: 4BFFFE05  bl 0x83043310
	ctx.lr = 0x83043510;
	sub_83043310(ctx, base);
	// 83043510: 48000008  b 0x83043518
	pc = 0x83043518; continue 'dispatch;
	// 83043514: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83043518: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8304351C: 48164C94  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83043520 size=44
    let mut pc: u32 = 0x83043520;
    'dispatch: loop {
        match pc {
            0x83043520 => {
    //   block [0x83043520..0x8304354C)
	// 83043520: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83043524: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83043528: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304352C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043530: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 83043534: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83043538: 4BF94DA9  bl 0x82fd82e0
	ctx.lr = 0x8304353C;
	sub_82FD82E0(ctx, base);
	// 8304353C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83043540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83043544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83043548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83043550 size=88
    let mut pc: u32 = 0x83043550;
    'dispatch: loop {
        match pc {
            0x83043550 => {
    //   block [0x83043550..0x830435A8)
	// 83043550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83043554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83043558: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304355C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043560: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 83043564: 38E00018  li r7, 0x18
	ctx.r[7].s64 = 24;
	// 83043568: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8304356C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83043570: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83043574: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83043578: 48047351  bl 0x8308a8c8
	ctx.lr = 0x8304357C;
	sub_8308A8C8(ctx, base);
	// 8304357C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83043580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83043584: 396BECA8  addi r11, r11, -0x1358
	ctx.r[11].s64 = ctx.r[11].s64 + -4952;
	// 83043588: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304358C: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 83043590: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83043594: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83043598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304359C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830435A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830435A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830435A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830435A8 size=8
    let mut pc: u32 = 0x830435A8;
    'dispatch: loop {
        match pc {
            0x830435A8 => {
    //   block [0x830435A8..0x830435B0)
	// 830435A8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830435AC: 8215ED58  lwz r16, -0x12a8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-4776 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830435B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830435B0 size=148
    let mut pc: u32 = 0x830435B0;
    'dispatch: loop {
        match pc {
            0x830435B0 => {
    //   block [0x830435B0..0x83043644)
	// 830435B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830435B4: 48164BB1  bl 0x831a8164
	ctx.lr = 0x830435B8;
	sub_831A8130(ctx, base);
	// 830435B8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 830435BC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830435C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830435C4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 830435C8: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 830435CC: 38E00018  li r7, 0x18
	ctx.r[7].s64 = 24;
	// 830435D0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830435D4: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 830435D8: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 830435DC: 480472ED  bl 0x8308a8c8
	ctx.lr = 0x830435E0;
	sub_8308A8C8(ctx, base);
	// 830435E0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830435E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830435E8: 396BECA8  addi r11, r11, -0x1358
	ctx.r[11].s64 = ctx.r[11].s64 + -4952;
	// 830435EC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830435F0: 915E0054  stw r10, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830435F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830435F8: 409A0030  bne cr6, 0x83043628
	if !ctx.cr[6].eq {
	pc = 0x83043628; continue 'dispatch;
	}
	// 830435FC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83043600: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 83043604: 388BED00  addi r4, r11, -0x1300
	ctx.r[4].s64 = ctx.r[11].s64 + -4864;
	// 83043608: 38C000EA  li r6, 0xea
	ctx.r[6].s64 = 234;
	// 8304360C: 38A0009C  li r5, 0x9c
	ctx.r[5].s64 = 156;
	// 83043610: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83043614: 4BFFEF5D  bl 0x83042570
	ctx.lr = 0x83043618;
	sub_83042570(ctx, base);
	// 83043618: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304361C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83043620: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 83043624: 4816D605  bl 0x831b0c28
	ctx.lr = 0x83043628;
	sub_831B0C28(ctx, base);
	// 83043628: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8304362C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83043630: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83043634: 48048625  bl 0x8308bc58
	ctx.lr = 0x83043638;
	sub_8308BC58(ctx, base);
	// 83043638: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304363C: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 83043640: 48164B74  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043644(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83043644 size=40
    let mut pc: u32 = 0x83043644;
    'dispatch: loop {
        match pc {
            0x83043644 => {
    //   block [0x83043644..0x8304366C)
	// 83043644: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83043648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304364C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83043650: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043654: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83043658: 48047179  bl 0x8308a7d0
	ctx.lr = 0x8304365C;
	sub_8308A7D0(ctx, base);
	// 8304365C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83043660: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83043664: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83043668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83043670 size=8
    let mut pc: u32 = 0x83043670;
    'dispatch: loop {
        match pc {
            0x83043670 => {
    //   block [0x83043670..0x83043678)
	// 83043670: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83043674: 8215ED90  lwz r16, -0x1270(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-4720 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83043678 size=104
    let mut pc: u32 = 0x83043678;
    'dispatch: loop {
        match pc {
            0x83043678 => {
    //   block [0x83043678..0x830436E0)
	// 83043678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304367C: 48164AE5  bl 0x831a8160
	ctx.lr = 0x83043680;
	sub_831A8130(ctx, base);
	// 83043680: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83043684: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043688: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8304368C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83043690: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83043694: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83043698: 38600058  li r3, 0x58
	ctx.r[3].s64 = 88;
	// 8304369C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830436A0: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 830436A4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 830436A8: 4BF94BF1  bl 0x82fd8298
	ctx.lr = 0x830436AC;
	sub_82FD8298(ctx, base);
	// 830436AC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830436B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830436B4: 41820020  beq 0x830436d4
	if ctx.cr[0].eq {
	pc = 0x830436D4; continue 'dispatch;
	}
	// 830436B8: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 830436BC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 830436C0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830436C4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830436C8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830436CC: 4BFFFEE5  bl 0x830435b0
	ctx.lr = 0x830436D0;
	sub_830435B0(ctx, base);
	// 830436D0: 48000008  b 0x830436d8
	pc = 0x830436D8; continue 'dispatch;
	// 830436D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830436D8: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830436DC: 48164AD4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830436E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830436E0 size=44
    let mut pc: u32 = 0x830436E0;
    'dispatch: loop {
        match pc {
            0x830436E0 => {
    //   block [0x830436E0..0x8304370C)
	// 830436E0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830436E4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830436E8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830436EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830436F0: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 830436F4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830436F8: 4BF94BE9  bl 0x82fd82e0
	ctx.lr = 0x830436FC;
	sub_82FD82E0(ctx, base);
	// 830436FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83043700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83043704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83043708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83043710 size=20
    let mut pc: u32 = 0x83043710;
    'dispatch: loop {
        match pc {
            0x83043710 => {
    //   block [0x83043710..0x83043724)
	// 83043710: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 83043714: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83043718: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 8304371C: 419AFFF4  beq cr6, 0x83043710
	if ctx.cr[6].eq {
	pc = 0x83043710; continue 'dispatch;
	}
	// 83043720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83043728 size=16
    let mut pc: u32 = 0x83043728;
    'dispatch: loop {
        match pc {
            0x83043728 => {
    //   block [0x83043728..0x83043738)
	// 83043728: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8304372C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83043730: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 83043734: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83043738 size=8
    let mut pc: u32 = 0x83043738;
    'dispatch: loop {
        match pc {
            0x83043738 => {
    //   block [0x83043738..0x83043740)
	// 83043738: 48047320  b 0x8308aa58
	sub_8308AA58(ctx, base);
	return;
	// 8304373C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83043740 size=8
    let mut pc: u32 = 0x83043740;
    'dispatch: loop {
        match pc {
            0x83043740 => {
    //   block [0x83043740..0x83043748)
	// 83043740: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83043744: 8215EDC8  lwz r16, -0x1238(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-4664 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83043748 size=96
    let mut pc: u32 = 0x83043748;
    'dispatch: loop {
        match pc {
            0x83043748 => {
    //   block [0x83043748..0x830437A8)
	// 83043748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304374C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83043750: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83043754: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83043758: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8304375C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043760: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83043764: 38600058  li r3, 0x58
	ctx.r[3].s64 = 88;
	// 83043768: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304376C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83043770: 4BF94B29  bl 0x82fd8298
	ctx.lr = 0x83043774;
	sub_82FD8298(ctx, base);
	// 83043774: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83043778: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304377C: 41820010  beq 0x8304378c
	if ctx.cr[0].eq {
	pc = 0x8304378C; continue 'dispatch;
	}
	// 83043780: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83043784: 4BFFFDCD  bl 0x83043550
	ctx.lr = 0x83043788;
	sub_83043550(ctx, base);
	// 83043788: 48000008  b 0x83043790
	pc = 0x83043790; continue 'dispatch;
	// 8304378C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83043790: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83043794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83043798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304379C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830437A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830437A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830437A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830437A8 size=44
    let mut pc: u32 = 0x830437A8;
    'dispatch: loop {
        match pc {
            0x830437A8 => {
    //   block [0x830437A8..0x830437D4)
	// 830437A8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830437AC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830437B0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830437B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830437B8: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830437BC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830437C0: 4BF94B21  bl 0x82fd82e0
	ctx.lr = 0x830437C4;
	sub_82FD82E0(ctx, base);
	// 830437C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830437C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830437CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830437D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830437D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830437D8 size=12
    let mut pc: u32 = 0x830437D8;
    'dispatch: loop {
        match pc {
            0x830437D8 => {
    //   block [0x830437D8..0x830437E4)
	// 830437D8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830437DC: 386B332C  addi r3, r11, 0x332c
	ctx.r[3].s64 = ctx.r[11].s64 + 13100;
	// 830437E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830437E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830437E8 size=4
    let mut pc: u32 = 0x830437E8;
    'dispatch: loop {
        match pc {
            0x830437E8 => {
    //   block [0x830437E8..0x830437EC)
	// 830437E8: 48047410  b 0x8308abf8
	sub_8308ABF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830437F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830437F0 size=88
    let mut pc: u32 = 0x830437F0;
    'dispatch: loop {
        match pc {
            0x830437F0 => {
    //   block [0x830437F0..0x83043848)
	// 830437F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830437F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830437F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830437FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83043800: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043804: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83043808: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304380C: 396BECA8  addi r11, r11, -0x1358
	ctx.r[11].s64 = ctx.r[11].s64 + -4952;
	// 83043810: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83043814: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83043818: 48046FB9  bl 0x8308a7d0
	ctx.lr = 0x8304381C;
	sub_8308A7D0(ctx, base);
	// 8304381C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83043820: 4182000C  beq 0x8304382c
	if ctx.cr[0].eq {
	pc = 0x8304382C; continue 'dispatch;
	}
	// 83043824: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83043828: 4BF94AB9  bl 0x82fd82e0
	ctx.lr = 0x8304382C;
	sub_82FD82E0(ctx, base);
	// 8304382C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83043830: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83043834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83043838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304383C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83043840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83043844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83043848 size=8
    let mut pc: u32 = 0x83043848;
    'dispatch: loop {
        match pc {
            0x83043848 => {
    //   block [0x83043848..0x83043850)
	// 83043848: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304384C: 8215EE70  lwz r16, -0x1190(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-4496 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83043850 size=352
    let mut pc: u32 = 0x83043850;
    'dispatch: loop {
        match pc {
            0x83043850 => {
    //   block [0x83043850..0x830439B0)
	// 83043850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83043854: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83043858: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 8304385C: 481648F9  bl 0x831a8154
	ctx.lr = 0x83043860;
	sub_831A8130(ctx, base);
	// 83043860: 3BE1FF20  addi r31, r1, -0xe0
	ctx.r[31].s64 = ctx.r[1].s64 + -224;
	// 83043864: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043868: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304386C: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 83043870: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 83043874: 93BF00F4  stw r29, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[29].u32 ) };
	// 83043878: 931F00FC  stw r24, 0xfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[24].u32 ) };
	// 8304387C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83043880: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 83043884: 409A000C  bne cr6, 0x83043890
	if !ctx.cr[6].eq {
	pc = 0x83043890; continue 'dispatch;
	}
	// 83043888: 48047439  bl 0x8308acc0
	ctx.lr = 0x8304388C;
	sub_8308ACC0(ctx, base);
	// 8304388C: 4800011C  b 0x830439a8
	pc = 0x830439A8; continue 'dispatch;
	// 83043890: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 83043894: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83043898: 41820110  beq 0x830439a8
	if ctx.cr[0].eq {
	pc = 0x830439A8; continue 'dispatch;
	}
	// 8304389C: 817D0050  lwz r11, 0x50(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 830438A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830438A4: 41820104  beq 0x830439a8
	if ctx.cr[0].eq {
	pc = 0x830439A8; continue 'dispatch;
	}
	// 830438A8: 82EB0008  lwz r23, 8(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830438AC: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 830438B0: 933F0060  stw r25, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[25].u32 ) };
	// 830438B4: 7F19B800  cmpw cr6, r25, r23
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[23].s32, &mut ctx.xer);
	// 830438B8: 409800F0  bge cr6, 0x830439a8
	if !ctx.cr[6].lt {
	pc = 0x830439A8; continue 'dispatch;
	}
	// 830438BC: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830438C0: 807D0050  lwz r3, 0x50(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 830438C4: 4BFE8FAD  bl 0x8302c870
	ctx.lr = 0x830438C8;
	sub_8302C870(ctx, base);
	// 830438C8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830438CC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830438D0: 4BF8FF11  bl 0x82fd37e0
	ctx.lr = 0x830438D4;
	sub_82FD37E0(ctx, base);
	// 830438D4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830438D8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830438DC: 835B0008  lwz r26, 8(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 830438E0: 937F0064  stw r27, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 830438E4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830438E8: 7F1CD000  cmpw cr6, r28, r26
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[26].s32, &mut ctx.xer);
	// 830438EC: 40980054  bge cr6, 0x83043940
	if !ctx.cr[6].lt {
	pc = 0x83043940; continue 'dispatch;
	}
	// 830438F0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830438F4: 83DD001C  lwz r30, 0x1c(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 830438F8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830438FC: 4BFE8F75  bl 0x8302c870
	ctx.lr = 0x83043900;
	sub_8302C870(ctx, base);
	// 83043900: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83043904: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83043908: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304390C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83043910: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83043914: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 83043918: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8304391C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83043920: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83043924: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83043928: 4E800421  bctrl
	ctx.lr = 0x8304392C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304392C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83043930: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83043934: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83043938: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8304393C: 4BFFFFAC  b 0x830438e8
	pc = 0x830438E8; continue 'dispatch;
	// 83043940: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83043944: 419A0020  beq cr6, 0x83043964
	if ctx.cr[6].eq {
	pc = 0x83043964; continue 'dispatch;
	}
	// 83043948: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304394C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83043950: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83043954: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83043958: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304395C: 4E800421  bctrl
	ctx.lr = 0x83043960;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83043960: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83043964: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83043968: 807D0050  lwz r3, 0x50(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304396C: 4BFE8F05  bl 0x8302c870
	ctx.lr = 0x83043970;
	sub_8302C870(ctx, base);
	// 83043970: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83043974: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83043978: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304397C: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 83043980: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83043984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83043988: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304398C: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 83043990: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83043994: 4E800421  bctrl
	ctx.lr = 0x83043998;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83043998: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304399C: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 830439A0: 933F0060  stw r25, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[25].u32 ) };
	// 830439A4: 4BFFFF10  b 0x830438b4
	pc = 0x830438B4; continue 'dispatch;
	// 830439A8: 383F00E0  addi r1, r31, 0xe0
	ctx.r[1].s64 = ctx.r[31].s64 + 224;
	// 830439AC: 481647F8  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830439B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830439B0 size=8
    let mut pc: u32 = 0x830439B0;
    'dispatch: loop {
        match pc {
            0x830439B0 => {
    //   block [0x830439B0..0x830439B8)
	// 830439B0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830439B4: 8215EE70  lwz r16, -0x1190(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-4496 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830439B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830439B8 size=28
    let mut pc: u32 = 0x830439B8;
    'dispatch: loop {
        match pc {
            0x830439B8 => {
    //   block [0x830439B8..0x830439D4)
	// 830439B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830439BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830439C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830439C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830439C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830439CC: 4816D25D  bl 0x831b0c28
	ctx.lr = 0x830439D0;
	sub_831B0C28(ctx, base);
	// 830439D0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830439D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830439D4 size=8
    let mut pc: u32 = 0x830439D4;
    'dispatch: loop {
        match pc {
            0x830439D4 => {
    //   block [0x830439D4..0x830439DC)
	// 830439D4: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830439D8: 8215EE70  lwz r16, -0x1190(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-4496 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830439DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830439DC size=68
    let mut pc: u32 = 0x830439DC;
    'dispatch: loop {
        match pc {
            0x830439DC => {
    //   block [0x830439DC..0x83043A20)
	// 830439DC: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 830439E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830439E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830439E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830439EC: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 830439F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830439F4: 419A001C  beq cr6, 0x83043a10
	if ctx.cr[6].eq {
	pc = 0x83043A10; continue 'dispatch;
	}
	// 830439F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830439FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83043A00: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83043A04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83043A08: 4E800421  bctrl
	ctx.lr = 0x83043A0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83043A0C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83043A10: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83043A14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83043A18: 4816D211  bl 0x831b0c28
	ctx.lr = 0x83043A1C;
	sub_831B0C28(ctx, base);
	// 83043A1C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83043A28 size=96
    let mut pc: u32 = 0x83043A28;
    'dispatch: loop {
        match pc {
            0x83043A28 => {
    //   block [0x83043A28..0x83043A88)
	// 83043A28: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 83043A2C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83043A30: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83043A34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043A38: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 83043A3C: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83043A40: 83DF00FC  lwz r30, 0xfc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 83043A44: 806B0050  lwz r3, 0x50(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 83043A48: 4BFE8E29  bl 0x8302c870
	ctx.lr = 0x83043A4C;
	sub_8302C870(ctx, base);
	// 83043A4C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83043A50: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 83043A54: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 83043A58: 388BED00  addi r4, r11, -0x1300
	ctx.r[4].s64 = ctx.r[11].s64 + -4864;
	// 83043A5C: 38C000B5  li r6, 0xb5
	ctx.r[6].s64 = 181;
	// 83043A60: 38A001CE  li r5, 0x1ce
	ctx.r[5].s64 = 462;
	// 83043A64: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83043A68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83043A6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83043A70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83043A74: 4BFFE0A5  bl 0x83041b18
	ctx.lr = 0x83043A78;
	sub_83041B18(ctx, base);
	// 83043A78: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83043A7C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83043A80: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 83043A84: 4816D1A5  bl 0x831b0c28
	ctx.lr = 0x83043A88;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83043A90 size=304
    let mut pc: u32 = 0x83043A90;
    'dispatch: loop {
        match pc {
            0x83043A90 => {
    //   block [0x83043A90..0x83043BC0)
	// 83043A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83043A94: 481646C1  bl 0x831a8154
	ctx.lr = 0x83043A98;
	sub_831A8130(ctx, base);
	// 83043A98: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 83043A9C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043AA0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83043AA4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83043AA8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83043AAC: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83043AB0: 836B001C  lwz r27, 0x1c(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83043AB4: 48000008  b 0x83043abc
	pc = 0x83043ABC; continue 'dispatch;
	// 83043AB8: 837B001C  lwz r27, 0x1c(r27)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 83043ABC: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 83043AC0: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 83043AC4: 419AFFF4  beq cr6, 0x83043ab8
	if ctx.cr[6].eq {
	pc = 0x83043AB8; continue 'dispatch;
	}
	// 83043AC8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83043ACC: 4BF8FD15  bl 0x82fd37e0
	ctx.lr = 0x83043AD0;
	sub_82FD37E0(ctx, base);
	// 83043AD0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83043AD4: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83043AD8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83043ADC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83043AE0: 4BF8FD01  bl 0x82fd37e0
	ctx.lr = 0x83043AE4;
	sub_82FD37E0(ctx, base);
	// 83043AE4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83043AE8: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 83043AEC: 831D0008  lwz r24, 8(r29)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83043AF0: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83043AF4: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83043AF8: 4098002C  bge cr6, 0x83043b24
	if !ctx.cr[6].lt {
	pc = 0x83043B24; continue 'dispatch;
	}
	// 83043AFC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83043B00: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83043B04: 480251D5  bl 0x83068cd8
	ctx.lr = 0x83043B08;
	sub_83068CD8(ctx, base);
	// 83043B08: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 83043B0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83043B10: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 83043B14: 480251C5  bl 0x83068cd8
	ctx.lr = 0x83043B18;
	sub_83068CD8(ctx, base);
	// 83043B18: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83043B1C: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 83043B20: 48164684  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
	// 83043B24: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83043B28: 40990018  ble cr6, 0x83043b40
	if !ctx.cr[6].gt {
	pc = 0x83043B40; continue 'dispatch;
	}
	// 83043B2C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83043B30: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83043B34: 480251A5  bl 0x83068cd8
	ctx.lr = 0x83043B38;
	sub_83068CD8(ctx, base);
	// 83043B38: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 83043B3C: 4BFFFFD0  b 0x83043b0c
	pc = 0x83043B0C; continue 'dispatch;
	// 83043B40: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83043B44: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 83043B48: 40990054  ble cr6, 0x83043b9c
	if !ctx.cr[6].gt {
	pc = 0x83043B9C; continue 'dispatch;
	}
	// 83043B4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83043B50: 833B0000  lwz r25, 0(r27)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83043B54: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83043B58: 4BFE8D19  bl 0x8302c870
	ctx.lr = 0x83043B5C;
	sub_8302C870(ctx, base);
	// 83043B5C: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 83043B60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83043B64: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83043B68: 4BFE8D09  bl 0x8302c870
	ctx.lr = 0x83043B6C;
	sub_8302C870(ctx, base);
	// 83043B6C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83043B70: 81790024  lwz r11, 0x24(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(36 as u32) ) } as u64;
	// 83043B74: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83043B78: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 83043B7C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 83043B80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83043B84: 4E800421  bctrl
	ctx.lr = 0x83043B88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83043B88: 7C791B79  or. r25, r3, r3
	ctx.r[25].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 83043B8C: 40820024  bne 0x83043bb0
	if !ctx.cr[0].eq {
	pc = 0x83043BB0; continue 'dispatch;
	}
	// 83043B90: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83043B94: 7F1EC000  cmpw cr6, r30, r24
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[24].s32, &mut ctx.xer);
	// 83043B98: 4198FFB4  blt cr6, 0x83043b4c
	if ctx.cr[6].lt {
	pc = 0x83043B4C; continue 'dispatch;
	}
	// 83043B9C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83043BA0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83043BA4: 48025135  bl 0x83068cd8
	ctx.lr = 0x83043BA8;
	sub_83068CD8(ctx, base);
	// 83043BA8: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 83043BAC: 4BFFFF60  b 0x83043b0c
	pc = 0x83043B0C; continue 'dispatch;
	// 83043BB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83043BB4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83043BB8: 48025121  bl 0x83068cd8
	ctx.lr = 0x83043BBC;
	sub_83068CD8(ctx, base);
	// 83043BBC: 4BFFFF50  b 0x83043b0c
	pc = 0x83043B0C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83043BC0 size=40
    let mut pc: u32 = 0x83043BC0;
    'dispatch: loop {
        match pc {
            0x83043BC0 => {
    //   block [0x83043BC0..0x83043BE8)
	// 83043BC0: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 83043BC4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83043BC8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83043BCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043BD0: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 83043BD4: 4BFD4BB5  bl 0x83018788
	ctx.lr = 0x83043BD8;
	sub_83018788(ctx, base);
	// 83043BD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83043BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83043BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83043BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83043BE8 size=40
    let mut pc: u32 = 0x83043BE8;
    'dispatch: loop {
        match pc {
            0x83043BE8 => {
    //   block [0x83043BE8..0x83043C10)
	// 83043BE8: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 83043BEC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83043BF0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83043BF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043BF8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83043BFC: 4BFD4B8D  bl 0x83018788
	ctx.lr = 0x83043C00;
	sub_83018788(ctx, base);
	// 83043C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83043C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83043C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83043C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83043C10 size=8
    let mut pc: u32 = 0x83043C10;
    'dispatch: loop {
        match pc {
            0x83043C10 => {
    //   block [0x83043C10..0x83043C18)
	// 83043C10: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83043C14: 8215EFA8  lwz r16, -0x1058(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-4184 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83043C18 size=240
    let mut pc: u32 = 0x83043C18;
    'dispatch: loop {
        match pc {
            0x83043C18 => {
    //   block [0x83043C18..0x83043D08)
	// 83043C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83043C1C: 4816453D  bl 0x831a8158
	ctx.lr = 0x83043C20;
	sub_831A8130(ctx, base);
	// 83043C20: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83043C24: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043C28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83043C2C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83043C30: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 83043C34: 838B001C  lwz r28, 0x1c(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83043C38: 48000008  b 0x83043c40
	pc = 0x83043C40; continue 'dispatch;
	// 83043C3C: 839C001C  lwz r28, 0x1c(r28)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 83043C40: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83043C44: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 83043C48: 419AFFF4  beq cr6, 0x83043c3c
	if ctx.cr[6].eq {
	pc = 0x83043C3C; continue 'dispatch;
	}
	// 83043C4C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83043C50: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 83043C54: 4BF8FB8D  bl 0x82fd37e0
	ctx.lr = 0x83043C58;
	sub_82FD37E0(ctx, base);
	// 83043C58: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83043C5C: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 83043C60: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83043C64: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 83043C68: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83043C6C: 419A001C  beq cr6, 0x83043c88
	if ctx.cr[6].eq {
	pc = 0x83043C88; continue 'dispatch;
	}
	// 83043C70: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83043C74: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83043C78: 48025061  bl 0x83068cd8
	ctx.lr = 0x83043C7C;
	sub_83068CD8(ctx, base);
	// 83043C7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83043C80: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 83043C84: 48164524  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83043C88: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83043C8C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83043C90: 41820058  beq 0x83043ce8
	if ctx.cr[0].eq {
	pc = 0x83043CE8; continue 'dispatch;
	}
	// 83043C94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83043C98: 835C0000  lwz r26, 0(r28)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83043C9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83043CA0: 4BFE8BD1  bl 0x8302c870
	ctx.lr = 0x83043CA4;
	sub_8302C870(ctx, base);
	// 83043CA4: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 83043CA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83043CAC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83043CB0: 4BFE8BC1  bl 0x8302c870
	ctx.lr = 0x83043CB4;
	sub_8302C870(ctx, base);
	// 83043CB4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83043CB8: 817A0024  lwz r11, 0x24(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(36 as u32) ) } as u64;
	// 83043CBC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83043CC0: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 83043CC4: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 83043CC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83043CCC: 4E800421  bctrl
	ctx.lr = 0x83043CD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83043CD0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83043CD4: 4082001C  bne 0x83043cf0
	if !ctx.cr[0].eq {
	pc = 0x83043CF0; continue 'dispatch;
	}
	// 83043CD8: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 83043CDC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83043CE0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83043CE4: 4198FFB0  blt cr6, 0x83043c94
	if ctx.cr[6].lt {
	pc = 0x83043C94; continue 'dispatch;
	}
	// 83043CE8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 83043CEC: 48000008  b 0x83043cf4
	pc = 0x83043CF4; continue 'dispatch;
	// 83043CF0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83043CF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83043CF8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83043CFC: 48024FDD  bl 0x83068cd8
	ctx.lr = 0x83043D00;
	sub_83068CD8(ctx, base);
	// 83043D00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83043D04: 4BFFFF7C  b 0x83043c80
	pc = 0x83043C80; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83043D08 size=40
    let mut pc: u32 = 0x83043D08;
    'dispatch: loop {
        match pc {
            0x83043D08 => {
    //   block [0x83043D08..0x83043D30)
	// 83043D08: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83043D0C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83043D10: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83043D14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043D18: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83043D1C: 4BFD4A6D  bl 0x83018788
	ctx.lr = 0x83043D20;
	sub_83018788(ctx, base);
	// 83043D20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83043D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83043D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83043D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83043D30 size=76
    let mut pc: u32 = 0x83043D30;
    'dispatch: loop {
        match pc {
            0x83043D30 => {
    //   block [0x83043D30..0x83043D7C)
	// 83043D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83043D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83043D38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83043D3C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043D40: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83043D44: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83043D48: 4BF8FA99  bl 0x82fd37e0
	ctx.lr = 0x83043D4C;
	sub_82FD37E0(ctx, base);
	// 83043D4C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83043D50: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83043D54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83043D58: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83043D5C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83043D60: 48024F79  bl 0x83068cd8
	ctx.lr = 0x83043D64;
	sub_83068CD8(ctx, base);
	// 83043D64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83043D68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83043D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83043D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83043D74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83043D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83043D80 size=8
    let mut pc: u32 = 0x83043D80;
    'dispatch: loop {
        match pc {
            0x83043D80 => {
    //   block [0x83043D80..0x83043D88)
	// 83043D80: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83043D84: 8215F04C  lwz r16, -0xfb4(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-4020 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83043D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83043D88 size=916
    let mut pc: u32 = 0x83043D88;
    'dispatch: loop {
        match pc {
            0x83043D88 => {
    //   block [0x83043D88..0x8304411C)
	// 83043D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83043D8C: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83043D90: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83043D94: 481643BD  bl 0x831a8150
	ctx.lr = 0x83043D98;
	sub_831A8130(ctx, base);
	// 83043D98: 3BE1FD60  addi r31, r1, -0x2a0
	ctx.r[31].s64 = ctx.r[1].s64 + -672;
	// 83043D9C: 9421FD60  stwu r1, -0x2a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-672 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83043DA0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83043DA4: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83043DA8: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 83043DAC: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 83043DB0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83043DB4: 837E001C  lwz r27, 0x1c(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83043DB8: 7CF63B78  mr r22, r7
	ctx.r[22].u64 = ctx.r[7].u64;
	// 83043DBC: 93DF02B4  stw r30, 0x2b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(692 as u32), ctx.r[30].u32 ) };
	// 83043DC0: 93BF02DC  stw r29, 0x2dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(732 as u32), ctx.r[29].u32 ) };
	// 83043DC4: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 83043DC8: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 83043DCC: 409A0014  bne cr6, 0x83043de0
	if !ctx.cr[6].eq {
	pc = 0x83043DE0; continue 'dispatch;
	}
	// 83043DD0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 83043DD4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83043DD8: 4BFFFFB1  bl 0x83043d88
	ctx.lr = 0x83043DDC;
	sub_83043D88(ctx, base);
	// 83043DDC: 48000050  b 0x83043e2c
	pc = 0x83043E2C; continue 'dispatch;
	// 83043DE0: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 83043DE4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83043DE8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83043DEC: 41820040  beq 0x83043e2c
	if ctx.cr[0].eq {
	pc = 0x83043E2C; continue 'dispatch;
	}
	// 83043DF0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83043DF4: 833B0000  lwz r25, 0(r27)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83043DF8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83043DFC: 4BFE8A75  bl 0x8302c870
	ctx.lr = 0x83043E00;
	sub_8302C870(ctx, base);
	// 83043E00: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83043E04: 8179001C  lwz r11, 0x1c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(28 as u32) ) } as u64;
	// 83043E08: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83043E0C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83043E10: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83043E14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83043E18: 4E800421  bctrl
	ctx.lr = 0x83043E1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83043E1C: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 83043E20: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83043E24: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83043E28: 4198FFC8  blt cr6, 0x83043df0
	if ctx.cr[6].lt {
	pc = 0x83043DF0; continue 'dispatch;
	}
	// 83043E2C: 839E0010  lwz r28, 0x10(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83043E30: 578B0739  rlwinm. r11, r28, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83043E34: 418200A4  beq 0x83043ed8
	if ctx.cr[0].eq {
	pc = 0x83043ED8; continue 'dispatch;
	}
	// 83043E38: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83043E3C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83043E40: 40820044  bne 0x83043e84
	if !ctx.cr[0].eq {
	pc = 0x83043E84; continue 'dispatch;
	}
	// 83043E44: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 83043E48: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83043E4C: 4BF9444D  bl 0x82fd8298
	ctx.lr = 0x83043E50;
	sub_82FD8298(ctx, base);
	// 83043E50: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83043E54: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83043E58: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83043E5C: 41820020  beq 0x83043e7c
	if ctx.cr[0].eq {
	pc = 0x83043E7C; continue 'dispatch;
	}
	// 83043E60: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83043E64: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83043E68: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83043E6C: 38ABD918  addi r5, r11, -0x26e8
	ctx.r[5].s64 = ctx.r[11].s64 + -9960;
	// 83043E70: 48046279  bl 0x8308a0e8
	ctx.lr = 0x83043E74;
	sub_8308A0E8(ctx, base);
	// 83043E74: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83043E78: 48000008  b 0x83043e80
	pc = 0x83043E80; continue 'dispatch;
	// 83043E7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83043E80: 907E0028  stw r3, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 83043E84: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83043E88: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83043E8C: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 83043E90: 48046881  bl 0x8308a710
	ctx.lr = 0x83043E94;
	sub_8308A710(ctx, base);
	// 83043E94: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83043E98: 40820040  bne 0x83043ed8
	if !ctx.cr[0].eq {
	pc = 0x83043ED8; continue 'dispatch;
	}
	// 83043E9C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83043EA0: 811E0024  lwz r8, 0x24(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83043EA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83043EA8: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83043EAC: 388BED00  addi r4, r11, -0x1300
	ctx.r[4].s64 = ctx.r[11].s64 + -4864;
	// 83043EB0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83043EB4: 7EE7BB78  mr r7, r23
	ctx.r[7].u64 = ctx.r[23].u64;
	// 83043EB8: 38C000EE  li r6, 0xee
	ctx.r[6].s64 = 238;
	// 83043EBC: 38A0010E  li r5, 0x10e
	ctx.r[5].s64 = 270;
	// 83043EC0: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 83043EC4: 4BFD1CC5  bl 0x83015b88
	ctx.lr = 0x83043EC8;
	sub_83015B88(ctx, base);
	// 83043EC8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83043ECC: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 83043ED0: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83043ED4: 4816CD55  bl 0x831b0c28
	ctx.lr = 0x83043ED8;
	sub_831B0C28(ctx, base);
	// 83043ED8: 56CB063F  clrlwi. r11, r22, 0x18
	ctx.r[11].u64 = ctx.r[22].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83043EDC: 40820238  bne 0x83044114
	if !ctx.cr[0].eq {
	pc = 0x83044114; continue 'dispatch;
	}
	// 83043EE0: 80780008  lwz r3, 8(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 83043EE4: 578B077B  rlwinm. r11, r28, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83043EE8: 41820078  beq 0x83043f60
	if ctx.cr[0].eq {
	pc = 0x83043F60; continue 'dispatch;
	}
	// 83043EEC: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 83043EF0: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83043EF4: 4099006C  ble cr6, 0x83043f60
	if !ctx.cr[6].gt {
	pc = 0x83043F60; continue 'dispatch;
	}
	// 83043EF8: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 83043EFC: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 83043F00: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 83043F04: 389F01C0  addi r4, r31, 0x1c0
	ctx.r[4].s64 = ctx.r[31].s64 + 448;
	// 83043F08: 4BF8D961  bl 0x82fd1868
	ctx.lr = 0x83043F0C;
	sub_82FD1868(ctx, base);
	// 83043F0C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 83043F10: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 83043F14: 807E0044  lwz r3, 0x44(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 83043F18: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 83043F1C: 389F0130  addi r4, r31, 0x130
	ctx.r[4].s64 = ctx.r[31].s64 + 304;
	// 83043F20: 4BF8D949  bl 0x82fd1868
	ctx.lr = 0x83043F24;
	sub_82FD1868(ctx, base);
	// 83043F24: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83043F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83043F2C: 80FE0054  lwz r7, 0x54(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 83043F30: 388BED00  addi r4, r11, -0x1300
	ctx.r[4].s64 = ctx.r[11].s64 + -4864;
	// 83043F34: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83043F38: 393F0130  addi r9, r31, 0x130
	ctx.r[9].s64 = ctx.r[31].s64 + 304;
	// 83043F3C: 391F01C0  addi r8, r31, 0x1c0
	ctx.r[8].s64 = ctx.r[31].s64 + 448;
	// 83043F40: 38C000F1  li r6, 0xf1
	ctx.r[6].s64 = 241;
	// 83043F44: 38A00127  li r5, 0x127
	ctx.r[5].s64 = 295;
	// 83043F48: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83043F4C: 4BFD1C3D  bl 0x83015b88
	ctx.lr = 0x83043F50;
	sub_83015B88(ctx, base);
	// 83043F50: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83043F54: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83043F58: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83043F5C: 4816CCCD  bl 0x831b0c28
	ctx.lr = 0x83043F60;
	sub_831B0C28(ctx, base);
	// 83043F60: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83043F64: 41820078  beq 0x83043fdc
	if ctx.cr[0].eq {
	pc = 0x83043FDC; continue 'dispatch;
	}
	// 83043F68: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 83043F6C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83043F70: 4098006C  bge cr6, 0x83043fdc
	if !ctx.cr[6].lt {
	pc = 0x83043FDC; continue 'dispatch;
	}
	// 83043F74: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 83043F78: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 83043F7C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 83043F80: 389F0130  addi r4, r31, 0x130
	ctx.r[4].s64 = ctx.r[31].s64 + 304;
	// 83043F84: 4BF8D8E5  bl 0x82fd1868
	ctx.lr = 0x83043F88;
	sub_82FD1868(ctx, base);
	// 83043F88: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 83043F8C: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 83043F90: 807E0048  lwz r3, 0x48(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 83043F94: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 83043F98: 389F01C0  addi r4, r31, 0x1c0
	ctx.r[4].s64 = ctx.r[31].s64 + 448;
	// 83043F9C: 4BF8D8CD  bl 0x82fd1868
	ctx.lr = 0x83043FA0;
	sub_82FD1868(ctx, base);
	// 83043FA0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83043FA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83043FA8: 80FE0054  lwz r7, 0x54(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 83043FAC: 388BED00  addi r4, r11, -0x1300
	ctx.r[4].s64 = ctx.r[11].s64 + -4864;
	// 83043FB0: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83043FB4: 393F01C0  addi r9, r31, 0x1c0
	ctx.r[9].s64 = ctx.r[31].s64 + 448;
	// 83043FB8: 391F0130  addi r8, r31, 0x130
	ctx.r[8].s64 = ctx.r[31].s64 + 304;
	// 83043FBC: 38C000F2  li r6, 0xf2
	ctx.r[6].s64 = 242;
	// 83043FC0: 38A00137  li r5, 0x137
	ctx.r[5].s64 = 311;
	// 83043FC4: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 83043FC8: 4BFD1BC1  bl 0x83015b88
	ctx.lr = 0x83043FCC;
	sub_83015B88(ctx, base);
	// 83043FCC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83043FD0: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 83043FD4: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83043FD8: 4816CC51  bl 0x831b0c28
	ctx.lr = 0x83043FDC;
	sub_831B0C28(ctx, base);
	// 83043FDC: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83043FE0: 41820078  beq 0x83044058
	if ctx.cr[0].eq {
	pc = 0x83044058; continue 'dispatch;
	}
	// 83043FE4: 817E0040  lwz r11, 0x40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83043FE8: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83043FEC: 419A006C  beq cr6, 0x83044058
	if ctx.cr[6].eq {
	pc = 0x83044058; continue 'dispatch;
	}
	// 83043FF0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 83043FF4: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 83043FF8: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 83043FFC: 389F0130  addi r4, r31, 0x130
	ctx.r[4].s64 = ctx.r[31].s64 + 304;
	// 83044000: 4BF8D869  bl 0x82fd1868
	ctx.lr = 0x83044004;
	sub_82FD1868(ctx, base);
	// 83044004: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 83044008: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8304400C: 807E0040  lwz r3, 0x40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83044010: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 83044014: 389F01C0  addi r4, r31, 0x1c0
	ctx.r[4].s64 = ctx.r[31].s64 + 448;
	// 83044018: 4BF8D851  bl 0x82fd1868
	ctx.lr = 0x8304401C;
	sub_82FD1868(ctx, base);
	// 8304401C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83044020: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83044024: 80FE0054  lwz r7, 0x54(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 83044028: 388BED00  addi r4, r11, -0x1300
	ctx.r[4].s64 = ctx.r[11].s64 + -4864;
	// 8304402C: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83044030: 393F01C0  addi r9, r31, 0x1c0
	ctx.r[9].s64 = ctx.r[31].s64 + 448;
	// 83044034: 391F0130  addi r8, r31, 0x130
	ctx.r[8].s64 = ctx.r[31].s64 + 304;
	// 83044038: 38C000F3  li r6, 0xf3
	ctx.r[6].s64 = 243;
	// 8304403C: 38A00147  li r5, 0x147
	ctx.r[5].s64 = 327;
	// 83044040: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83044044: 4BFD1B45  bl 0x83015b88
	ctx.lr = 0x83044048;
	sub_83015B88(ctx, base);
	// 83044048: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304404C: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83044050: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83044054: 4816CBD5  bl 0x831b0c28
	ctx.lr = 0x83044058;
	sub_831B0C28(ctx, base);
	// 83044058: 578B06F7  rlwinm. r11, r28, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304405C: 418200B8  beq 0x83044114
	if ctx.cr[0].eq {
	pc = 0x83044114; continue 'dispatch;
	}
	// 83044060: 817E0050  lwz r11, 0x50(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 83044064: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83044068: 418200AC  beq 0x83044114
	if ctx.cr[0].eq {
	pc = 0x83044114; continue 'dispatch;
	}
	// 8304406C: 834B0008  lwz r26, 8(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83044070: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83044074: 2C1A0000  cmpwi r26, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83044078: 40810058  ble 0x830440d0
	if !ctx.cr[0].gt {
	pc = 0x830440D0; continue 'dispatch;
	}
	// 8304407C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83044080: 807E0050  lwz r3, 0x50(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 83044084: 837E0054  lwz r27, 0x54(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 83044088: 4BFE87E9  bl 0x8302c870
	ctx.lr = 0x8304408C;
	sub_8302C870(ctx, base);
	// 8304408C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83044090: 4BF8FBB1  bl 0x82fd3c40
	ctx.lr = 0x83044094;
	sub_82FD3C40(ctx, base);
	// 83044094: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83044098: 40820038  bne 0x830440d0
	if !ctx.cr[0].eq {
	pc = 0x830440D0; continue 'dispatch;
	}
	// 8304409C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830440A0: 807E0050  lwz r3, 0x50(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 830440A4: 4BFE87CD  bl 0x8302c870
	ctx.lr = 0x830440A8;
	sub_8302C870(ctx, base);
	// 830440A8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830440AC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830440B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830440B4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830440B8: 4BFFFB61  bl 0x83043c18
	ctx.lr = 0x830440BC;
	sub_83043C18(ctx, base);
	// 830440BC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830440C0: 40820010  bne 0x830440d0
	if !ctx.cr[0].eq {
	pc = 0x830440D0; continue 'dispatch;
	}
	// 830440C4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 830440C8: 7F1CD000  cmpw cr6, r28, r26
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[26].s32, &mut ctx.xer);
	// 830440CC: 4198FFB0  blt cr6, 0x8304407c
	if ctx.cr[6].lt {
	pc = 0x8304407C; continue 'dispatch;
	}
	// 830440D0: 7F1CD000  cmpw cr6, r28, r26
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[26].s32, &mut ctx.xer);
	// 830440D4: 409A0040  bne cr6, 0x83044114
	if !ctx.cr[6].eq {
	pc = 0x83044114; continue 'dispatch;
	}
	// 830440D8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830440DC: 80FE0054  lwz r7, 0x54(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 830440E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830440E4: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 830440E8: 388BED00  addi r4, r11, -0x1300
	ctx.r[4].s64 = ctx.r[11].s64 + -4864;
	// 830440EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830440F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830440F4: 38C000F4  li r6, 0xf4
	ctx.r[6].s64 = 244;
	// 830440F8: 38A00162  li r5, 0x162
	ctx.r[5].s64 = 354;
	// 830440FC: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 83044100: 4BFD1A89  bl 0x83015b88
	ctx.lr = 0x83044104;
	sub_83015B88(ctx, base);
	// 83044104: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83044108: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 8304410C: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83044110: 4816CB19  bl 0x831b0c28
	ctx.lr = 0x83044114;
	sub_831B0C28(ctx, base);
	// 83044114: 383F02A0  addi r1, r31, 0x2a0
	ctx.r[1].s64 = ctx.r[31].s64 + 672;
	// 83044118: 48164088  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304411C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304411C size=8
    let mut pc: u32 = 0x8304411C;
    'dispatch: loop {
        match pc {
            0x8304411C => {
    //   block [0x8304411C..0x83044124)
	// 8304411C: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83044120: 8215F04C  lwz r16, -0xfb4(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-4020 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044124(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044124 size=84
    let mut pc: u32 = 0x83044124;
    'dispatch: loop {
        match pc {
            0x83044124 => {
    //   block [0x83044124..0x83044178)
	// 83044124: 3BECFD60  addi r31, r12, -0x2a0
	ctx.r[31].s64 = ctx.r[12].s64 + -672;
	// 83044128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304412C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044130: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044134: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83044138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304413C: 388BED00  addi r4, r11, -0x1300
	ctx.r[4].s64 = ctx.r[11].s64 + -4864;
	// 83044140: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 83044144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83044148: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304414C: 38C0012B  li r6, 0x12b
	ctx.r[6].s64 = 299;
	// 83044150: 38A00103  li r5, 0x103
	ctx.r[5].s64 = 259;
	// 83044154: 80EB0010  lwz r7, 0x10(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83044158: 387F0110  addi r3, r31, 0x110
	ctx.r[3].s64 = ctx.r[31].s64 + 272;
	// 8304415C: 817F02DC  lwz r11, 0x2dc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(732 as u32) ) } as u64;
	// 83044160: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83044164: 4BFD1A25  bl 0x83015b88
	ctx.lr = 0x83044168;
	sub_83015B88(ctx, base);
	// 83044168: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304416C: 387F0110  addi r3, r31, 0x110
	ctx.r[3].s64 = ctx.r[31].s64 + 272;
	// 83044170: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83044174: 4816CAB5  bl 0x831b0c28
	ctx.lr = 0x83044178;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044178 size=48
    let mut pc: u32 = 0x83044178;
    'dispatch: loop {
        match pc {
            0x83044178 => {
    //   block [0x83044178..0x830441A8)
	// 83044178: 3BECFD60  addi r31, r12, -0x2a0
	ctx.r[31].s64 = ctx.r[12].s64 + -672;
	// 8304417C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044180: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044184: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044188: 817F02B4  lwz r11, 0x2b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(692 as u32) ) } as u64;
	// 8304418C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83044190: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83044194: 4BF9414D  bl 0x82fd82e0
	ctx.lr = 0x83044198;
	sub_82FD82E0(ctx, base);
	// 83044198: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304419C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830441A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830441A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830441A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830441A8 size=8
    let mut pc: u32 = 0x830441A8;
    'dispatch: loop {
        match pc {
            0x830441A8 => {
    //   block [0x830441A8..0x830441B0)
	// 830441A8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830441AC: 8215F120  lwz r16, -0xee0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-3808 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830441B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830441B0 size=640
    let mut pc: u32 = 0x830441B0;
    'dispatch: loop {
        match pc {
            0x830441B0 => {
    //   block [0x830441B0..0x83044430)
	// 830441B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830441B4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830441B8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830441BC: 48163F89  bl 0x831a8144
	ctx.lr = 0x830441C0;
	sub_831A8130(ctx, base);
	// 830441C0: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 830441C4: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830441C8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830441CC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830441D0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830441D4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 830441D8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830441DC: 409A0008  bne cr6, 0x830441e4
	if !ctx.cr[6].eq {
	pc = 0x830441E4; continue 'dispatch;
	}
	// 830441E0: 83DA0004  lwz r30, 4(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 830441E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830441E8: 93BA0054  stw r29, 0x54(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 830441EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830441F0: 4BF8F5F1  bl 0x82fd37e0
	ctx.lr = 0x830441F4;
	sub_82FD37E0(ctx, base);
	// 830441F4: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 830441F8: 929F0050  stw r20, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[20].u32 ) };
	// 830441FC: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83044200: 41820024  beq 0x83044224
	if ctx.cr[0].eq {
	pc = 0x83044224; continue 'dispatch;
	}
	// 83044204: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 83044208: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8304420C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83044210: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83044214: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 83044218: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8304421C: 4BFFFB6D  bl 0x83043d88
	ctx.lr = 0x83044220;
	sub_83043D88(ctx, base);
	// 83044220: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83044224: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83044228: 419A0040  beq cr6, 0x83044268
	if ctx.cr[6].eq {
	pc = 0x83044268; continue 'dispatch;
	}
	// 8304422C: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83044230: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83044234: 41820034  beq 0x83044268
	if ctx.cr[0].eq {
	pc = 0x83044268; continue 'dispatch;
	}
	// 83044238: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 8304423C: 48000014  b 0x83044250
	pc = 0x83044250; continue 'dispatch;
	// 83044240: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83044244: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83044248: 480001D8  b 0x83044420
	pc = 0x83044420; continue 'dispatch;
	// 8304424C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83044250: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83044254: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83044258: 4082FFF4  bne 0x8304424c
	if !ctx.cr[0].eq {
	pc = 0x8304424C; continue 'dispatch;
	}
	// 8304425C: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 83044260: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83044264: 48000008  b 0x8304426c
	pc = 0x8304426C; continue 'dispatch;
	// 83044268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304426C: 5577083C  slwi r23, r11, 1
	ctx.r[23].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[23].u64 = ctx.r[23].u32 as u64;
	// 83044270: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83044274: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83044278: 56E4083C  slwi r4, r23, 1
	ctx.r[4].u32 = ctx.r[23].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8304427C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83044280: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83044284: 4E800421  bctrl
	ctx.lr = 0x83044288;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83044288: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8304428C: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 83044290: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 83044294: B27C0000  sth r19, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[19].u16 ) };
	// 83044298: 82DA001C  lwz r22, 0x1c(r26)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 8304429C: 48000008  b 0x830442a4
	pc = 0x830442A4; continue 'dispatch;
	// 830442A0: 82D6001C  lwz r22, 0x1c(r22)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(28 as u32) ) } as u64;
	// 830442A4: 81760018  lwz r11, 0x18(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(24 as u32) ) } as u64;
	// 830442A8: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 830442AC: 419AFFF4  beq cr6, 0x830442a0
	if ctx.cr[6].eq {
	pc = 0x830442A0; continue 'dispatch;
	}
	// 830442B0: 7E759B78  mr r21, r19
	ctx.r[21].u64 = ctx.r[19].u64;
	// 830442B4: 81740008  lwz r11, 8(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(8 as u32) ) } as u64;
	// 830442B8: 7F155840  cmplw cr6, r21, r11
	ctx.cr[6].compare_u32(ctx.r[21].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830442BC: 40980144  bge cr6, 0x83044400
	if !ctx.cr[6].lt {
	pc = 0x83044400; continue 'dispatch;
	}
	// 830442C0: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 830442C4: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 830442C8: 4BFE85A9  bl 0x8302c870
	ctx.lr = 0x830442CC;
	sub_8302C870(ctx, base);
	// 830442CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830442D0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830442D4: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 830442D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830442DC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830442E0: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830442E4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830442E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830442EC: 4E800421  bctrl
	ctx.lr = 0x830442F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830442F0: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 830442F4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830442F8: 28180000  cmplwi r24, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830442FC: 41820034  beq 0x83044330
	if ctx.cr[0].eq {
	pc = 0x83044330; continue 'dispatch;
	}
	// 83044300: A1780000  lhz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 83044304: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83044308: 41820028  beq 0x83044330
	if ctx.cr[0].eq {
	pc = 0x83044330; continue 'dispatch;
	}
	// 8304430C: 39780002  addi r11, r24, 2
	ctx.r[11].s64 = ctx.r[24].s64 + 2;
	// 83044310: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83044314: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83044318: 4182000C  beq 0x83044324
	if ctx.cr[0].eq {
	pc = 0x83044324; continue 'dispatch;
	}
	// 8304431C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83044320: 4BFFFFF0  b 0x83044310
	pc = 0x83044310; continue 'dispatch;
	// 83044324: 7D785850  subf r11, r24, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[24].s64;
	// 83044328: 7D790E70  srawi r25, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[25].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8304432C: 48000008  b 0x83044334
	pc = 0x83044334; continue 'dispatch;
	// 83044330: 7E799B78  mr r25, r19
	ctx.r[25].u64 = ctx.r[19].u64;
	// 83044334: 39790002  addi r11, r25, 2
	ctx.r[11].s64 = ctx.r[25].s64 + 2;
	// 83044338: 56FA083C  slwi r26, r23, 1
	ctx.r[26].u32 = ctx.r[23].u32.wrapping_shl(1);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 8304433C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83044340: 7D5AE214  add r10, r26, r28
	ctx.r[10].u64 = ctx.r[26].u64 + ctx.r[28].u64;
	// 83044344: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 83044348: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8304434C: 41980064  blt cr6, 0x830443b0
	if ctx.cr[6].lt {
	pc = 0x830443B0; continue 'dispatch;
	}
	// 83044350: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83044354: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 83044358: 56E41838  slwi r4, r23, 3
	ctx.r[4].u32 = ctx.r[23].u32.wrapping_shl(3);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8304435C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83044360: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83044364: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83044368: 4E800421  bctrl
	ctx.lr = 0x8304436C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304436C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83044370: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83044374: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83044378: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8304437C: 48164195  bl 0x831a8510
	ctx.lr = 0x83044380;
	sub_831A8510(ctx, base);
	// 83044380: 7D7DD850  subf r11, r29, r27
	ctx.r[11].s64 = ctx.r[27].s64 - ctx.r[29].s64;
	// 83044384: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83044388: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304438C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83044390: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83044394: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83044398: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304439C: 7F6BE214  add r27, r11, r28
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 830443A0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 830443A4: 4E800421  bctrl
	ctx.lr = 0x830443A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830443A8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830443AC: 56F7103A  slwi r23, r23, 2
	ctx.r[23].u32 = ctx.r[23].u32.wrapping_shl(2);
	ctx.r[23].u64 = ctx.r[23].u32 as u64;
	// 830443B0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830443B4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830443B8: 4BF8D4E1  bl 0x82fd1898
	ctx.lr = 0x830443BC;
	sub_82FD1898(ctx, base);
	// 830443BC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830443C0: 39790001  addi r11, r25, 1
	ctx.r[11].s64 = ctx.r[25].s64 + 1;
	// 830443C4: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 830443C8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830443CC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830443D0: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 830443D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830443D8: 3B6B0002  addi r27, r11, 2
	ctx.r[27].s64 = ctx.r[11].s64 + 2;
	// 830443DC: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 830443E0: B27B0000  sth r19, 0(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[19].u16 ) };
	// 830443E4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830443E8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830443EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830443F0: 4E800421  bctrl
	ctx.lr = 0x830443F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830443F4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830443F8: 3AB50001  addi r21, r21, 1
	ctx.r[21].s64 = ctx.r[21].s64 + 1;
	// 830443FC: 4BFFFEB8  b 0x830442b4
	pc = 0x830442B4; continue 'dispatch;
	// 83044400: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83044404: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83044408: 480248D1  bl 0x83068cd8
	ctx.lr = 0x8304440C;
	sub_83068CD8(ctx, base);
	// 8304440C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83044410: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83044414: 48000014  b 0x83044428
	pc = 0x83044428; continue 'dispatch;
	// 83044418: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8304441C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83044420: 480248B9  bl 0x83068cd8
	ctx.lr = 0x83044424;
	sub_83068CD8(ctx, base);
	// 83044424: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83044428: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 8304442C: 48163D68  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83044430 size=8
    let mut pc: u32 = 0x83044430;
    'dispatch: loop {
        match pc {
            0x83044430 => {
    //   block [0x83044430..0x83044438)
	// 83044430: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83044434: 8215F120  lwz r16, -0xee0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-3808 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044438 size=20
    let mut pc: u32 = 0x83044438;
    'dispatch: loop {
        match pc {
            0x83044438 => {
    //   block [0x83044438..0x8304444C)
	// 83044438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304443C: 3C608304  lis r3, -0x7cfc
	ctx.r[3].s64 = -2096889856;
	// 83044440: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83044444: 38634240  addi r3, r3, 0x4240
	ctx.r[3].s64 = ctx.r[3].s64 + 16960;
	// 83044448: 48163D4C  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304444C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304444C size=8
    let mut pc: u32 = 0x8304444C;
    'dispatch: loop {
        match pc {
            0x8304444C => {
    //   block [0x8304444C..0x83044454)
	// 8304444C: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83044450: 8215F120  lwz r16, -0xee0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-3808 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044454(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044454 size=20
    let mut pc: u32 = 0x83044454;
    'dispatch: loop {
        match pc {
            0x83044454 => {
    //   block [0x83044454..0x83044468)
	// 83044454: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044458: 3C608304  lis r3, -0x7cfc
	ctx.r[3].s64 = -2096889856;
	// 8304445C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83044460: 38634418  addi r3, r3, 0x4418
	ctx.r[3].s64 = ctx.r[3].s64 + 17432;
	// 83044464: 48163D30  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044468 size=40
    let mut pc: u32 = 0x83044468;
    'dispatch: loop {
        match pc {
            0x83044468 => {
    //   block [0x83044468..0x83044490)
	// 83044468: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 8304446C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044470: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044474: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044478: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8304447C: 4BFD430D  bl 0x83018788
	ctx.lr = 0x83044480;
	sub_83018788(ctx, base);
	// 83044480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83044484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304448C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83044490 size=8
    let mut pc: u32 = 0x83044490;
    'dispatch: loop {
        match pc {
            0x83044490 => {
    //   block [0x83044490..0x83044498)
	// 83044490: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83044494: 8215F230  lwz r16, -0xdd0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-3536 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044498 size=100
    let mut pc: u32 = 0x83044498;
    'dispatch: loop {
        match pc {
            0x83044498 => {
    //   block [0x83044498..0x830444FC)
	// 83044498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304449C: 48163CC9  bl 0x831a8164
	ctx.lr = 0x830444A0;
	sub_831A8130(ctx, base);
	// 830444A0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830444A4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830444A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830444AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830444B0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830444B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830444B8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830444BC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830444C0: 93DD0054  stw r30, 0x54(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 830444C4: 4BF8F31D  bl 0x82fd37e0
	ctx.lr = 0x830444C8;
	sub_82FD37E0(ctx, base);
	// 830444C8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830444CC: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 830444D0: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 830444D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830444D8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830444DC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830444E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830444E4: 4BFFF8A5  bl 0x83043d88
	ctx.lr = 0x830444E8;
	sub_83043D88(ctx, base);
	// 830444E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830444EC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830444F0: 480247E9  bl 0x83068cd8
	ctx.lr = 0x830444F4;
	sub_83068CD8(ctx, base);
	// 830444F4: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830444F8: 48163CBC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830444FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830444FC size=40
    let mut pc: u32 = 0x830444FC;
    'dispatch: loop {
        match pc {
            0x830444FC => {
    //   block [0x830444FC..0x83044524)
	// 830444FC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83044500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304450C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83044510: 4BFD4279  bl 0x83018788
	ctx.lr = 0x83044514;
	sub_83018788(ctx, base);
	// 83044514: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83044518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304451C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83044528 size=8
    let mut pc: u32 = 0x83044528;
    'dispatch: loop {
        match pc {
            0x83044528 => {
    //   block [0x83044528..0x83044530)
	// 83044528: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304452C: 8215F278  lwz r16, -0xd88(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-3464 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044530 size=104
    let mut pc: u32 = 0x83044530;
    'dispatch: loop {
        match pc {
            0x83044530 => {
    //   block [0x83044530..0x83044598)
	// 83044530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044534: 48163C2D  bl 0x831a8160
	ctx.lr = 0x83044538;
	sub_831A8130(ctx, base);
	// 83044538: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8304453C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044540: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83044544: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83044548: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8304454C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83044550: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83044554: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83044558: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8304455C: 93DD0054  stw r30, 0x54(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 83044560: 4BF8F281  bl 0x82fd37e0
	ctx.lr = 0x83044564;
	sub_82FD37E0(ctx, base);
	// 83044564: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83044568: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 8304456C: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 83044570: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83044574: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83044578: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8304457C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83044580: 4BFFF809  bl 0x83043d88
	ctx.lr = 0x83044584;
	sub_83043D88(ctx, base);
	// 83044584: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83044588: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8304458C: 4802474D  bl 0x83068cd8
	ctx.lr = 0x83044590;
	sub_83068CD8(ctx, base);
	// 83044590: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83044594: 48163C1C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044598 size=40
    let mut pc: u32 = 0x83044598;
    'dispatch: loop {
        match pc {
            0x83044598 => {
    //   block [0x83044598..0x830445C0)
	// 83044598: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8304459C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830445A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830445A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830445A8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830445AC: 4BFD41DD  bl 0x83018788
	ctx.lr = 0x830445B0;
	sub_83018788(ctx, base);
	// 830445B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830445B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830445B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830445BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830445C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830445C0 size=80
    let mut pc: u32 = 0x830445C0;
    'dispatch: loop {
        match pc {
            0x830445C0 => {
    //   block [0x830445C0..0x83044610)
	// 830445C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830445C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830445C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830445CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830445D0: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 830445D4: 38E00016  li r7, 0x16
	ctx.r[7].s64 = 22;
	// 830445D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830445DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830445E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830445E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830445E8: 480078B1  bl 0x8304be98
	ctx.lr = 0x830445EC;
	sub_8304BE98(ctx, base);
	// 830445EC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830445F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830445F4: 396BF2D0  addi r11, r11, -0xd30
	ctx.r[11].s64 = ctx.r[11].s64 + -3376;
	// 830445F8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830445FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83044600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044608: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304460C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83044610 size=8
    let mut pc: u32 = 0x83044610;
    'dispatch: loop {
        match pc {
            0x83044610 => {
    //   block [0x83044610..0x83044618)
	// 83044610: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83044614: 8215F330  lwz r16, -0xcd0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-3280 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044618 size=84
    let mut pc: u32 = 0x83044618;
    'dispatch: loop {
        match pc {
            0x83044618 => {
    //   block [0x83044618..0x8304466C)
	// 83044618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304461C: 48163B4D  bl 0x831a8168
	ctx.lr = 0x83044620;
	sub_831A8130(ctx, base);
	// 83044620: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83044624: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044628: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304462C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83044630: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 83044634: 38E00016  li r7, 0x16
	ctx.r[7].s64 = 22;
	// 83044638: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 8304463C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83044640: 48007859  bl 0x8304be98
	ctx.lr = 0x83044644;
	sub_8304BE98(ctx, base);
	// 83044644: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83044648: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8304464C: 396BF2D0  addi r11, r11, -0xd30
	ctx.r[11].s64 = ctx.r[11].s64 + -3376;
	// 83044650: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83044654: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83044658: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304465C: 480475FD  bl 0x8308bc58
	ctx.lr = 0x83044660;
	sub_8308BC58(ctx, base);
	// 83044660: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83044664: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83044668: 48163B50  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304466C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304466C size=40
    let mut pc: u32 = 0x8304466C;
    'dispatch: loop {
        match pc {
            0x8304466C => {
    //   block [0x8304466C..0x83044694)
	// 8304466C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83044670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304467C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83044680: 48007769  bl 0x8304bde8
	ctx.lr = 0x83044684;
	sub_8304BDE8(ctx, base);
	// 83044684: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83044688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304468C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83044698 size=8
    let mut pc: u32 = 0x83044698;
    'dispatch: loop {
        match pc {
            0x83044698 => {
    //   block [0x83044698..0x830446A0)
	// 83044698: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304469C: 8215F368  lwz r16, -0xc98(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-3224 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830446A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830446A0 size=104
    let mut pc: u32 = 0x830446A0;
    'dispatch: loop {
        match pc {
            0x830446A0 => {
    //   block [0x830446A0..0x83044708)
	// 830446A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830446A4: 48163ABD  bl 0x831a8160
	ctx.lr = 0x830446A8;
	sub_831A8130(ctx, base);
	// 830446A8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830446AC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830446B0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 830446B4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830446B8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830446BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830446C0: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 830446C4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830446C8: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 830446CC: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 830446D0: 4BF93BC9  bl 0x82fd8298
	ctx.lr = 0x830446D4;
	sub_82FD8298(ctx, base);
	// 830446D4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830446D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830446DC: 41820020  beq 0x830446fc
	if ctx.cr[0].eq {
	pc = 0x830446FC; continue 'dispatch;
	}
	// 830446E0: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 830446E4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 830446E8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830446EC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830446F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830446F4: 4BFFFF25  bl 0x83044618
	ctx.lr = 0x830446F8;
	sub_83044618(ctx, base);
	// 830446F8: 48000008  b 0x83044700
	pc = 0x83044700; continue 'dispatch;
	// 830446FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83044700: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83044704: 48163AAC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044708 size=44
    let mut pc: u32 = 0x83044708;
    'dispatch: loop {
        match pc {
            0x83044708 => {
    //   block [0x83044708..0x83044734)
	// 83044708: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8304470C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044710: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044714: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044718: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8304471C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83044720: 4BF93BC1  bl 0x82fd82e0
	ctx.lr = 0x83044724;
	sub_82FD82E0(ctx, base);
	// 83044724: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83044728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304472C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044738 size=56
    let mut pc: u32 = 0x83044738;
    'dispatch: loop {
        match pc {
            0x83044738 => {
    //   block [0x83044738..0x83044770)
	// 83044738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304473C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044740: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044744: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83044748: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8304474C: 4BF8F4F5  bl 0x82fd3c40
	ctx.lr = 0x83044750;
	sub_82FD3C40(ctx, base);
	// 83044750: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 83044754: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83044758: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8304475C: 7C6B00D0  neg r3, r11
	ctx.r[3].s64 = -ctx.r[11].s64;
	// 83044760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83044764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304476C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044770 size=88
    let mut pc: u32 = 0x83044770;
    'dispatch: loop {
        match pc {
            0x83044770 => {
    //   block [0x83044770..0x830447C8)
	// 83044770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044778: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304477C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83044780: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044784: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83044788: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8304478C: 480463BD  bl 0x8308ab48
	ctx.lr = 0x83044790;
	sub_8308AB48(ctx, base);
	// 83044790: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83044794: 419A001C  beq cr6, 0x830447b0
	if ctx.cr[6].eq {
	pc = 0x830447B0; continue 'dispatch;
	}
	// 83044798: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304479C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830447A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830447A4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 830447A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830447AC: 4E800421  bctrl
	ctx.lr = 0x830447B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830447B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830447B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830447B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830447BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830447C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830447C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830447C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830447C8 size=128
    let mut pc: u32 = 0x830447C8;
    'dispatch: loop {
        match pc {
            0x830447C8 => {
    //   block [0x830447C8..0x83044848)
	// 830447C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830447CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830447D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830447D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830447D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830447DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830447E0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830447E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830447E8: 4BF8CCA9  bl 0x82fd1490
	ctx.lr = 0x830447EC;
	sub_82FD1490(ctx, base);
	// 830447EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830447F0: 40820040  bne 0x83044830
	if !ctx.cr[0].eq {
	pc = 0x83044830; continue 'dispatch;
	}
	// 830447F4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830447F8: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 830447FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83044800: 388BF398  addi r4, r11, -0xc68
	ctx.r[4].s64 = ctx.r[11].s64 + -3176;
	// 83044804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83044808: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304480C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 83044810: 38C000FD  li r6, 0xfd
	ctx.r[6].s64 = 253;
	// 83044814: 38A0009E  li r5, 0x9e
	ctx.r[5].s64 = 158;
	// 83044818: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304481C: 4BFD136D  bl 0x83015b88
	ctx.lr = 0x83044820;
	sub_83015B88(ctx, base);
	// 83044820: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83044824: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83044828: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 8304482C: 4816C3FD  bl 0x831b0c28
	ctx.lr = 0x83044830;
	sub_831B0C28(ctx, base);
	// 83044830: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83044834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304483C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83044840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83044844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83044848 size=8
    let mut pc: u32 = 0x83044848;
    'dispatch: loop {
        match pc {
            0x83044848 => {
    //   block [0x83044848..0x83044850)
	// 83044848: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304484C: 8215F3F0  lwz r16, -0xc10(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-3088 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044850 size=96
    let mut pc: u32 = 0x83044850;
    'dispatch: loop {
        match pc {
            0x83044850 => {
    //   block [0x83044850..0x830448B0)
	// 83044850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044858: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304485C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83044860: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83044864: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044868: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304486C: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 83044870: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83044874: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83044878: 4BF93A21  bl 0x82fd8298
	ctx.lr = 0x8304487C;
	sub_82FD8298(ctx, base);
	// 8304487C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83044880: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83044884: 41820010  beq 0x83044894
	if ctx.cr[0].eq {
	pc = 0x83044894; continue 'dispatch;
	}
	// 83044888: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304488C: 4BFFFD35  bl 0x830445c0
	ctx.lr = 0x83044890;
	sub_830445C0(ctx, base);
	// 83044890: 48000008  b 0x83044898
	pc = 0x83044898; continue 'dispatch;
	// 83044894: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83044898: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8304489C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830448A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830448A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830448A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830448AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830448B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830448B0 size=44
    let mut pc: u32 = 0x830448B0;
    'dispatch: loop {
        match pc {
            0x830448B0 => {
    //   block [0x830448B0..0x830448DC)
	// 830448B0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830448B4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830448B8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830448BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830448C0: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830448C4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830448C8: 4BF93A19  bl 0x82fd82e0
	ctx.lr = 0x830448CC;
	sub_82FD82E0(ctx, base);
	// 830448CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830448D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830448D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830448D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830448E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830448E0 size=12
    let mut pc: u32 = 0x830448E0;
    'dispatch: loop {
        match pc {
            0x830448E0 => {
    //   block [0x830448E0..0x830448EC)
	// 830448E0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830448E4: 386B3334  addi r3, r11, 0x3334
	ctx.r[3].s64 = ctx.r[11].s64 + 13108;
	// 830448E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830448F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830448F0 size=4
    let mut pc: u32 = 0x830448F0;
    'dispatch: loop {
        match pc {
            0x830448F0 => {
    //   block [0x830448F0..0x830448F4)
	// 830448F0: 4BFFEEF8  b 0x830437e8
	sub_830437E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830448F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830448F8 size=88
    let mut pc: u32 = 0x830448F8;
    'dispatch: loop {
        match pc {
            0x830448F8 => {
    //   block [0x830448F8..0x83044950)
	// 830448F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830448FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044900: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83044904: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83044908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304490C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83044910: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83044914: 396BF2D0  addi r11, r11, -0xd30
	ctx.r[11].s64 = ctx.r[11].s64 + -3376;
	// 83044918: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8304491C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83044920: 480074C9  bl 0x8304bde8
	ctx.lr = 0x83044924;
	sub_8304BDE8(ctx, base);
	// 83044924: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83044928: 4182000C  beq 0x83044934
	if ctx.cr[0].eq {
	pc = 0x83044934; continue 'dispatch;
	}
	// 8304492C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83044930: 4BF939B1  bl 0x82fd82e0
	ctx.lr = 0x83044934;
	sub_82FD82E0(ctx, base);
	// 83044934: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83044938: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304493C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044944: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83044948: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304494C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044950 size=80
    let mut pc: u32 = 0x83044950;
    'dispatch: loop {
        match pc {
            0x83044950 => {
    //   block [0x83044950..0x830449A0)
	// 83044950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044958: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304495C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044960: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 83044964: 38E00015  li r7, 0x15
	ctx.r[7].s64 = 21;
	// 83044968: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8304496C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83044970: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83044974: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83044978: 48007521  bl 0x8304be98
	ctx.lr = 0x8304497C;
	sub_8304BE98(ctx, base);
	// 8304497C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83044980: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83044984: 396BF438  addi r11, r11, -0xbc8
	ctx.r[11].s64 = ctx.r[11].s64 + -3016;
	// 83044988: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304498C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83044990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044998: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304499C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830449A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830449A0 size=8
    let mut pc: u32 = 0x830449A0;
    'dispatch: loop {
        match pc {
            0x830449A0 => {
    //   block [0x830449A0..0x830449A8)
	// 830449A0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830449A4: 8215F498  lwz r16, -0xb68(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-2920 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830449A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830449A8 size=84
    let mut pc: u32 = 0x830449A8;
    'dispatch: loop {
        match pc {
            0x830449A8 => {
    //   block [0x830449A8..0x830449FC)
	// 830449A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830449AC: 481637BD  bl 0x831a8168
	ctx.lr = 0x830449B0;
	sub_831A8130(ctx, base);
	// 830449B0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830449B4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830449B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830449BC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830449C0: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 830449C4: 38E00015  li r7, 0x15
	ctx.r[7].s64 = 21;
	// 830449C8: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 830449CC: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830449D0: 480074C9  bl 0x8304be98
	ctx.lr = 0x830449D4;
	sub_8304BE98(ctx, base);
	// 830449D4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830449D8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830449DC: 396BF438  addi r11, r11, -0xbc8
	ctx.r[11].s64 = ctx.r[11].s64 + -3016;
	// 830449E0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830449E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830449E8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830449EC: 4804726D  bl 0x8308bc58
	ctx.lr = 0x830449F0;
	sub_8308BC58(ctx, base);
	// 830449F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830449F4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830449F8: 481637C0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830449FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830449FC size=40
    let mut pc: u32 = 0x830449FC;
    'dispatch: loop {
        match pc {
            0x830449FC => {
    //   block [0x830449FC..0x83044A24)
	// 830449FC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83044A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044A08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044A0C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83044A10: 480073D9  bl 0x8304bde8
	ctx.lr = 0x83044A14;
	sub_8304BDE8(ctx, base);
	// 83044A14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83044A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83044A28 size=8
    let mut pc: u32 = 0x83044A28;
    'dispatch: loop {
        match pc {
            0x83044A28 => {
    //   block [0x83044A28..0x83044A30)
	// 83044A28: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83044A2C: 8215F4D0  lwz r16, -0xb30(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-2864 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044A30 size=104
    let mut pc: u32 = 0x83044A30;
    'dispatch: loop {
        match pc {
            0x83044A30 => {
    //   block [0x83044A30..0x83044A98)
	// 83044A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044A34: 4816372D  bl 0x831a8160
	ctx.lr = 0x83044A38;
	sub_831A8130(ctx, base);
	// 83044A38: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83044A3C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044A40: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 83044A44: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83044A48: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83044A4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83044A50: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 83044A54: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83044A58: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 83044A5C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83044A60: 4BF93839  bl 0x82fd8298
	ctx.lr = 0x83044A64;
	sub_82FD8298(ctx, base);
	// 83044A64: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83044A68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83044A6C: 41820020  beq 0x83044a8c
	if ctx.cr[0].eq {
	pc = 0x83044A8C; continue 'dispatch;
	}
	// 83044A70: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 83044A74: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83044A78: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83044A7C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83044A80: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83044A84: 4BFFFF25  bl 0x830449a8
	ctx.lr = 0x83044A88;
	sub_830449A8(ctx, base);
	// 83044A88: 48000008  b 0x83044a90
	pc = 0x83044A90; continue 'dispatch;
	// 83044A8C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83044A90: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83044A94: 4816371C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044A98 size=44
    let mut pc: u32 = 0x83044A98;
    'dispatch: loop {
        match pc {
            0x83044A98 => {
    //   block [0x83044A98..0x83044AC4)
	// 83044A98: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83044A9C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044AA0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044AA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044AA8: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 83044AAC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83044AB0: 4BF93831  bl 0x82fd82e0
	ctx.lr = 0x83044AB4;
	sub_82FD82E0(ctx, base);
	// 83044AB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83044AB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044ABC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044AC8 size=88
    let mut pc: u32 = 0x83044AC8;
    'dispatch: loop {
        match pc {
            0x83044AC8 => {
    //   block [0x83044AC8..0x83044B20)
	// 83044AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044AD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83044AD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83044AD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044ADC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83044AE0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83044AE4: 48046065  bl 0x8308ab48
	ctx.lr = 0x83044AE8;
	sub_8308AB48(ctx, base);
	// 83044AE8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83044AEC: 419A001C  beq cr6, 0x83044b08
	if ctx.cr[6].eq {
	pc = 0x83044B08; continue 'dispatch;
	}
	// 83044AF0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83044AF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83044AF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83044AFC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83044B00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83044B04: 4E800421  bctrl
	ctx.lr = 0x83044B08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83044B08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83044B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044B14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83044B18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83044B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044B20 size=128
    let mut pc: u32 = 0x83044B20;
    'dispatch: loop {
        match pc {
            0x83044B20 => {
    //   block [0x83044B20..0x83044BA0)
	// 83044B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044B28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83044B2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83044B30: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044B34: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83044B38: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83044B3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83044B40: 4BF8C951  bl 0x82fd1490
	ctx.lr = 0x83044B44;
	sub_82FD1490(ctx, base);
	// 83044B44: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83044B48: 40820040  bne 0x83044b88
	if !ctx.cr[0].eq {
	pc = 0x83044B88; continue 'dispatch;
	}
	// 83044B4C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83044B50: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 83044B54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83044B58: 388BF500  addi r4, r11, -0xb00
	ctx.r[4].s64 = ctx.r[11].s64 + -2816;
	// 83044B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83044B60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83044B64: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 83044B68: 38C000FD  li r6, 0xfd
	ctx.r[6].s64 = 253;
	// 83044B6C: 38A00095  li r5, 0x95
	ctx.r[5].s64 = 149;
	// 83044B70: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83044B74: 4BFD1015  bl 0x83015b88
	ctx.lr = 0x83044B78;
	sub_83015B88(ctx, base);
	// 83044B78: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83044B7C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83044B80: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83044B84: 4816C0A5  bl 0x831b0c28
	ctx.lr = 0x83044B88;
	sub_831B0C28(ctx, base);
	// 83044B88: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83044B8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044B90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044B94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83044B98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83044B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83044BA0 size=8
    let mut pc: u32 = 0x83044BA0;
    'dispatch: loop {
        match pc {
            0x83044BA0 => {
    //   block [0x83044BA0..0x83044BA8)
	// 83044BA0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83044BA4: 8215F558  lwz r16, -0xaa8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-2728 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044BA8 size=96
    let mut pc: u32 = 0x83044BA8;
    'dispatch: loop {
        match pc {
            0x83044BA8 => {
    //   block [0x83044BA8..0x83044C08)
	// 83044BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044BB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83044BB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83044BB8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83044BBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044BC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83044BC4: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 83044BC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83044BCC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83044BD0: 4BF936C9  bl 0x82fd8298
	ctx.lr = 0x83044BD4;
	sub_82FD8298(ctx, base);
	// 83044BD4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83044BD8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83044BDC: 41820010  beq 0x83044bec
	if ctx.cr[0].eq {
	pc = 0x83044BEC; continue 'dispatch;
	}
	// 83044BE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83044BE4: 4BFFFD6D  bl 0x83044950
	ctx.lr = 0x83044BE8;
	sub_83044950(ctx, base);
	// 83044BE8: 48000008  b 0x83044bf0
	pc = 0x83044BF0; continue 'dispatch;
	// 83044BEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83044BF0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83044BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044BFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83044C00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83044C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044C08 size=44
    let mut pc: u32 = 0x83044C08;
    'dispatch: loop {
        match pc {
            0x83044C08 => {
    //   block [0x83044C08..0x83044C34)
	// 83044C08: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83044C0C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044C10: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044C14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044C18: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83044C1C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83044C20: 4BF936C1  bl 0x82fd82e0
	ctx.lr = 0x83044C24;
	sub_82FD82E0(ctx, base);
	// 83044C24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83044C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83044C38 size=12
    let mut pc: u32 = 0x83044C38;
    'dispatch: loop {
        match pc {
            0x83044C38 => {
    //   block [0x83044C38..0x83044C44)
	// 83044C38: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83044C3C: 386B333C  addi r3, r11, 0x333c
	ctx.r[3].s64 = ctx.r[11].s64 + 13116;
	// 83044C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044C48 size=88
    let mut pc: u32 = 0x83044C48;
    'dispatch: loop {
        match pc {
            0x83044C48 => {
    //   block [0x83044C48..0x83044CA0)
	// 83044C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044C50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83044C54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83044C58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044C5C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83044C60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83044C64: 396BF438  addi r11, r11, -0xbc8
	ctx.r[11].s64 = ctx.r[11].s64 + -3016;
	// 83044C68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83044C6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83044C70: 48007179  bl 0x8304bde8
	ctx.lr = 0x83044C74;
	sub_8304BDE8(ctx, base);
	// 83044C74: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83044C78: 4182000C  beq 0x83044c84
	if ctx.cr[0].eq {
	pc = 0x83044C84; continue 'dispatch;
	}
	// 83044C7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83044C80: 4BF93661  bl 0x82fd82e0
	ctx.lr = 0x83044C84;
	sub_82FD82E0(ctx, base);
	// 83044C84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83044C88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83044C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044C94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83044C98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83044C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044CA0 size=80
    let mut pc: u32 = 0x83044CA0;
    'dispatch: loop {
        match pc {
            0x83044CA0 => {
    //   block [0x83044CA0..0x83044CF0)
	// 83044CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044CA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83044CAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044CB0: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 83044CB4: 38E00014  li r7, 0x14
	ctx.r[7].s64 = 20;
	// 83044CB8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83044CBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83044CC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83044CC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83044CC8: 480071D1  bl 0x8304be98
	ctx.lr = 0x83044CCC;
	sub_8304BE98(ctx, base);
	// 83044CCC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83044CD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83044CD4: 396BF5A0  addi r11, r11, -0xa60
	ctx.r[11].s64 = ctx.r[11].s64 + -2656;
	// 83044CD8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83044CDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83044CE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044CE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044CE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83044CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83044CF0 size=8
    let mut pc: u32 = 0x83044CF0;
    'dispatch: loop {
        match pc {
            0x83044CF0 => {
    //   block [0x83044CF0..0x83044CF8)
	// 83044CF0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83044CF4: 8215F600  lwz r16, -0xa00(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-2560 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044CF8 size=84
    let mut pc: u32 = 0x83044CF8;
    'dispatch: loop {
        match pc {
            0x83044CF8 => {
    //   block [0x83044CF8..0x83044D4C)
	// 83044CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044CFC: 4816346D  bl 0x831a8168
	ctx.lr = 0x83044D00;
	sub_831A8130(ctx, base);
	// 83044D00: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83044D04: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044D08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83044D0C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83044D10: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 83044D14: 38E00014  li r7, 0x14
	ctx.r[7].s64 = 20;
	// 83044D18: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83044D1C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83044D20: 48007179  bl 0x8304be98
	ctx.lr = 0x83044D24;
	sub_8304BE98(ctx, base);
	// 83044D24: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83044D28: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83044D2C: 396BF5A0  addi r11, r11, -0xa60
	ctx.r[11].s64 = ctx.r[11].s64 + -2656;
	// 83044D30: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83044D34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83044D38: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83044D3C: 48046F1D  bl 0x8308bc58
	ctx.lr = 0x83044D40;
	sub_8308BC58(ctx, base);
	// 83044D40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83044D44: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83044D48: 48163470  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044D4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044D4C size=40
    let mut pc: u32 = 0x83044D4C;
    'dispatch: loop {
        match pc {
            0x83044D4C => {
    //   block [0x83044D4C..0x83044D74)
	// 83044D4C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83044D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044D58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044D5C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83044D60: 48007089  bl 0x8304bde8
	ctx.lr = 0x83044D64;
	sub_8304BDE8(ctx, base);
	// 83044D64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83044D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83044D78 size=16
    let mut pc: u32 = 0x83044D78;
    'dispatch: loop {
        match pc {
            0x83044D78 => {
    //   block [0x83044D78..0x83044D88)
	// 83044D78: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83044D7C: 396BF5A0  addi r11, r11, -0xa60
	ctx.r[11].s64 = ctx.r[11].s64 + -2656;
	// 83044D80: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83044D84: 48007064  b 0x8304bde8
	sub_8304BDE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83044D88 size=8
    let mut pc: u32 = 0x83044D88;
    'dispatch: loop {
        match pc {
            0x83044D88 => {
    //   block [0x83044D88..0x83044D90)
	// 83044D88: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83044D8C: 8215F638  lwz r16, -0x9c8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-2504 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044D90 size=104
    let mut pc: u32 = 0x83044D90;
    'dispatch: loop {
        match pc {
            0x83044D90 => {
    //   block [0x83044D90..0x83044DF8)
	// 83044D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044D94: 481633CD  bl 0x831a8160
	ctx.lr = 0x83044D98;
	sub_831A8130(ctx, base);
	// 83044D98: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83044D9C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044DA0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 83044DA4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83044DA8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83044DAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83044DB0: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 83044DB4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83044DB8: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 83044DBC: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83044DC0: 4BF934D9  bl 0x82fd8298
	ctx.lr = 0x83044DC4;
	sub_82FD8298(ctx, base);
	// 83044DC4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83044DC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83044DCC: 41820020  beq 0x83044dec
	if ctx.cr[0].eq {
	pc = 0x83044DEC; continue 'dispatch;
	}
	// 83044DD0: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 83044DD4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83044DD8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83044DDC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83044DE0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83044DE4: 4BFFFF15  bl 0x83044cf8
	ctx.lr = 0x83044DE8;
	sub_83044CF8(ctx, base);
	// 83044DE8: 48000008  b 0x83044df0
	pc = 0x83044DF0; continue 'dispatch;
	// 83044DEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83044DF0: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83044DF4: 481633BC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044DF8 size=44
    let mut pc: u32 = 0x83044DF8;
    'dispatch: loop {
        match pc {
            0x83044DF8 => {
    //   block [0x83044DF8..0x83044E24)
	// 83044DF8: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83044DFC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044E00: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044E04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044E08: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 83044E0C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83044E10: 4BF934D1  bl 0x82fd82e0
	ctx.lr = 0x83044E14;
	sub_82FD82E0(ctx, base);
	// 83044E14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83044E18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044E1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044E28 size=88
    let mut pc: u32 = 0x83044E28;
    'dispatch: loop {
        match pc {
            0x83044E28 => {
    //   block [0x83044E28..0x83044E80)
	// 83044E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044E30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83044E34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83044E38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044E3C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83044E40: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83044E44: 48045D05  bl 0x8308ab48
	ctx.lr = 0x83044E48;
	sub_8308AB48(ctx, base);
	// 83044E48: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83044E4C: 419A001C  beq cr6, 0x83044e68
	if ctx.cr[6].eq {
	pc = 0x83044E68; continue 'dispatch;
	}
	// 83044E50: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83044E54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83044E58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83044E5C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83044E60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83044E64: 4E800421  bctrl
	ctx.lr = 0x83044E68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83044E68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83044E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044E74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83044E78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83044E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044E80 size=128
    let mut pc: u32 = 0x83044E80;
    'dispatch: loop {
        match pc {
            0x83044E80 => {
    //   block [0x83044E80..0x83044F00)
	// 83044E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044E88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83044E8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83044E90: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044E94: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83044E98: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83044E9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83044EA0: 4BF8C5F1  bl 0x82fd1490
	ctx.lr = 0x83044EA4;
	sub_82FD1490(ctx, base);
	// 83044EA4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83044EA8: 40820040  bne 0x83044ee8
	if !ctx.cr[0].eq {
	pc = 0x83044EE8; continue 'dispatch;
	}
	// 83044EAC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83044EB0: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 83044EB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83044EB8: 388BF668  addi r4, r11, -0x998
	ctx.r[4].s64 = ctx.r[11].s64 + -2456;
	// 83044EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83044EC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83044EC4: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 83044EC8: 38C000FD  li r6, 0xfd
	ctx.r[6].s64 = 253;
	// 83044ECC: 38A00095  li r5, 0x95
	ctx.r[5].s64 = 149;
	// 83044ED0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83044ED4: 4BFD0CB5  bl 0x83015b88
	ctx.lr = 0x83044ED8;
	sub_83015B88(ctx, base);
	// 83044ED8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83044EDC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83044EE0: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83044EE4: 4816BD45  bl 0x831b0c28
	ctx.lr = 0x83044EE8;
	sub_831B0C28(ctx, base);
	// 83044EE8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83044EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044EF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83044EF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83044EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83044F00 size=8
    let mut pc: u32 = 0x83044F00;
    'dispatch: loop {
        match pc {
            0x83044F00 => {
    //   block [0x83044F00..0x83044F08)
	// 83044F00: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83044F04: 8215F6B8  lwz r16, -0x948(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-2376 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044F08 size=96
    let mut pc: u32 = 0x83044F08;
    'dispatch: loop {
        match pc {
            0x83044F08 => {
    //   block [0x83044F08..0x83044F68)
	// 83044F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044F10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83044F14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83044F18: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83044F1C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044F20: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83044F24: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 83044F28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83044F2C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83044F30: 4BF93369  bl 0x82fd8298
	ctx.lr = 0x83044F34;
	sub_82FD8298(ctx, base);
	// 83044F34: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83044F38: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83044F3C: 41820010  beq 0x83044f4c
	if ctx.cr[0].eq {
	pc = 0x83044F4C; continue 'dispatch;
	}
	// 83044F40: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83044F44: 4BFFFD5D  bl 0x83044ca0
	ctx.lr = 0x83044F48;
	sub_83044CA0(ctx, base);
	// 83044F48: 48000008  b 0x83044f50
	pc = 0x83044F50; continue 'dispatch;
	// 83044F4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83044F50: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83044F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044F5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83044F60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83044F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044F68 size=44
    let mut pc: u32 = 0x83044F68;
    'dispatch: loop {
        match pc {
            0x83044F68 => {
    //   block [0x83044F68..0x83044F94)
	// 83044F68: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83044F6C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044F70: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044F74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044F78: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83044F7C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83044F80: 4BF93361  bl 0x82fd82e0
	ctx.lr = 0x83044F84;
	sub_82FD82E0(ctx, base);
	// 83044F84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83044F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83044F98 size=12
    let mut pc: u32 = 0x83044F98;
    'dispatch: loop {
        match pc {
            0x83044F98 => {
    //   block [0x83044F98..0x83044FA4)
	// 83044F98: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83044F9C: 386B3344  addi r3, r11, 0x3344
	ctx.r[3].s64 = ctx.r[11].s64 + 13124;
	// 83044FA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83044FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83044FA8 size=88
    let mut pc: u32 = 0x83044FA8;
    'dispatch: loop {
        match pc {
            0x83044FA8 => {
    //   block [0x83044FA8..0x83045000)
	// 83044FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83044FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83044FB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83044FB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83044FB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83044FBC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83044FC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83044FC4: 396BF5A0  addi r11, r11, -0xa60
	ctx.r[11].s64 = ctx.r[11].s64 + -2656;
	// 83044FC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83044FCC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83044FD0: 48006E19  bl 0x8304bde8
	ctx.lr = 0x83044FD4;
	sub_8304BDE8(ctx, base);
	// 83044FD4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83044FD8: 4182000C  beq 0x83044fe4
	if ctx.cr[0].eq {
	pc = 0x83044FE4; continue 'dispatch;
	}
	// 83044FDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83044FE0: 4BF93301  bl 0x82fd82e0
	ctx.lr = 0x83044FE4;
	sub_82FD82E0(ctx, base);
	// 83044FE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83044FE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83044FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83044FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83044FF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83044FF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83044FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045000 size=80
    let mut pc: u32 = 0x83045000;
    'dispatch: loop {
        match pc {
            0x83045000 => {
    //   block [0x83045000..0x83045050)
	// 83045000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045008: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304500C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045010: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 83045014: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 83045018: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8304501C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83045020: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83045024: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83045028: 48006E71  bl 0x8304be98
	ctx.lr = 0x8304502C;
	sub_8304BE98(ctx, base);
	// 8304502C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83045030: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83045034: 396BF700  addi r11, r11, -0x900
	ctx.r[11].s64 = ctx.r[11].s64 + -2304;
	// 83045038: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304503C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83045040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83045044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045048: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304504C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83045050 size=8
    let mut pc: u32 = 0x83045050;
    'dispatch: loop {
        match pc {
            0x83045050 => {
    //   block [0x83045050..0x83045058)
	// 83045050: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83045054: 8215F760  lwz r16, -0x8a0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-2208 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045058 size=84
    let mut pc: u32 = 0x83045058;
    'dispatch: loop {
        match pc {
            0x83045058 => {
    //   block [0x83045058..0x830450AC)
	// 83045058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304505C: 4816310D  bl 0x831a8168
	ctx.lr = 0x83045060;
	sub_831A8130(ctx, base);
	// 83045060: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83045064: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045068: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304506C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83045070: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 83045074: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 83045078: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 8304507C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83045080: 48006E19  bl 0x8304be98
	ctx.lr = 0x83045084;
	sub_8304BE98(ctx, base);
	// 83045084: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83045088: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8304508C: 396BF700  addi r11, r11, -0x900
	ctx.r[11].s64 = ctx.r[11].s64 + -2304;
	// 83045090: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83045094: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83045098: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304509C: 48046BBD  bl 0x8308bc58
	ctx.lr = 0x830450A0;
	sub_8308BC58(ctx, base);
	// 830450A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830450A4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830450A8: 48163110  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830450AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830450AC size=40
    let mut pc: u32 = 0x830450AC;
    'dispatch: loop {
        match pc {
            0x830450AC => {
    //   block [0x830450AC..0x830450D4)
	// 830450AC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830450B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830450B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830450B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830450BC: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830450C0: 48006D29  bl 0x8304bde8
	ctx.lr = 0x830450C4;
	sub_8304BDE8(ctx, base);
	// 830450C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830450C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830450CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830450D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830450D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830450D8 size=8
    let mut pc: u32 = 0x830450D8;
    'dispatch: loop {
        match pc {
            0x830450D8 => {
    //   block [0x830450D8..0x830450E0)
	// 830450D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830450DC: 8215F798  lwz r16, -0x868(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-2152 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830450E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830450E0 size=104
    let mut pc: u32 = 0x830450E0;
    'dispatch: loop {
        match pc {
            0x830450E0 => {
    //   block [0x830450E0..0x83045148)
	// 830450E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830450E4: 4816307D  bl 0x831a8160
	ctx.lr = 0x830450E8;
	sub_831A8130(ctx, base);
	// 830450E8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830450EC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830450F0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 830450F4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830450F8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830450FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83045100: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 83045104: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83045108: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 8304510C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83045110: 4BF93189  bl 0x82fd8298
	ctx.lr = 0x83045114;
	sub_82FD8298(ctx, base);
	// 83045114: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83045118: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304511C: 41820020  beq 0x8304513c
	if ctx.cr[0].eq {
	pc = 0x8304513C; continue 'dispatch;
	}
	// 83045120: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 83045124: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83045128: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8304512C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83045130: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83045134: 4BFFFF25  bl 0x83045058
	ctx.lr = 0x83045138;
	sub_83045058(ctx, base);
	// 83045138: 48000008  b 0x83045140
	pc = 0x83045140; continue 'dispatch;
	// 8304513C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83045140: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83045144: 4816306C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045148 size=44
    let mut pc: u32 = 0x83045148;
    'dispatch: loop {
        match pc {
            0x83045148 => {
    //   block [0x83045148..0x83045174)
	// 83045148: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8304514C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045150: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045154: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045158: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8304515C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83045160: 4BF93181  bl 0x82fd82e0
	ctx.lr = 0x83045164;
	sub_82FD82E0(ctx, base);
	// 83045164: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83045168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304516C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045178 size=128
    let mut pc: u32 = 0x83045178;
    'dispatch: loop {
        match pc {
            0x83045178 => {
    //   block [0x83045178..0x830451F8)
	// 83045178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304517C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045180: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83045184: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83045188: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304518C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83045190: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83045194: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83045198: 4BF8C2F9  bl 0x82fd1490
	ctx.lr = 0x8304519C;
	sub_82FD1490(ctx, base);
	// 8304519C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830451A0: 40820040  bne 0x830451e0
	if !ctx.cr[0].eq {
	pc = 0x830451E0; continue 'dispatch;
	}
	// 830451A4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830451A8: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 830451AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830451B0: 388BF7C8  addi r4, r11, -0x838
	ctx.r[4].s64 = ctx.r[11].s64 + -2104;
	// 830451B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830451B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830451BC: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 830451C0: 38C000FD  li r6, 0xfd
	ctx.r[6].s64 = 253;
	// 830451C4: 38A00092  li r5, 0x92
	ctx.r[5].s64 = 146;
	// 830451C8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830451CC: 4BFD09BD  bl 0x83015b88
	ctx.lr = 0x830451D0;
	sub_83015B88(ctx, base);
	// 830451D0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830451D4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830451D8: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 830451DC: 4816BA4D  bl 0x831b0c28
	ctx.lr = 0x830451E0;
	sub_831B0C28(ctx, base);
	// 830451E0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830451E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830451E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830451EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830451F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830451F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830451F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830451F8 size=8
    let mut pc: u32 = 0x830451F8;
    'dispatch: loop {
        match pc {
            0x830451F8 => {
    //   block [0x830451F8..0x83045200)
	// 830451F8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830451FC: 8215F820  lwz r16, -0x7e0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-2016 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045200 size=96
    let mut pc: u32 = 0x83045200;
    'dispatch: loop {
        match pc {
            0x83045200 => {
    //   block [0x83045200..0x83045260)
	// 83045200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045208: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304520C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83045210: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83045214: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045218: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304521C: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 83045220: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83045224: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83045228: 4BF93071  bl 0x82fd8298
	ctx.lr = 0x8304522C;
	sub_82FD8298(ctx, base);
	// 8304522C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83045230: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83045234: 41820010  beq 0x83045244
	if ctx.cr[0].eq {
	pc = 0x83045244; continue 'dispatch;
	}
	// 83045238: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304523C: 4BFFFDC5  bl 0x83045000
	ctx.lr = 0x83045240;
	sub_83045000(ctx, base);
	// 83045240: 48000008  b 0x83045248
	pc = 0x83045248; continue 'dispatch;
	// 83045244: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83045248: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8304524C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83045250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045254: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83045258: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304525C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045260 size=44
    let mut pc: u32 = 0x83045260;
    'dispatch: loop {
        match pc {
            0x83045260 => {
    //   block [0x83045260..0x8304528C)
	// 83045260: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83045264: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045268: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304526C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045270: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83045274: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83045278: 4BF93069  bl 0x82fd82e0
	ctx.lr = 0x8304527C;
	sub_82FD82E0(ctx, base);
	// 8304527C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83045280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83045284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83045290 size=12
    let mut pc: u32 = 0x83045290;
    'dispatch: loop {
        match pc {
            0x83045290 => {
    //   block [0x83045290..0x8304529C)
	// 83045290: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83045294: 386B334C  addi r3, r11, 0x334c
	ctx.r[3].s64 = ctx.r[11].s64 + 13132;
	// 83045298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830452A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830452A0 size=88
    let mut pc: u32 = 0x830452A0;
    'dispatch: loop {
        match pc {
            0x830452A0 => {
    //   block [0x830452A0..0x830452F8)
	// 830452A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830452A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830452A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830452AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830452B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830452B4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830452B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830452BC: 396BF700  addi r11, r11, -0x900
	ctx.r[11].s64 = ctx.r[11].s64 + -2304;
	// 830452C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830452C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830452C8: 48006B21  bl 0x8304bde8
	ctx.lr = 0x830452CC;
	sub_8304BDE8(ctx, base);
	// 830452CC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830452D0: 4182000C  beq 0x830452dc
	if ctx.cr[0].eq {
	pc = 0x830452DC; continue 'dispatch;
	}
	// 830452D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830452D8: 4BF93009  bl 0x82fd82e0
	ctx.lr = 0x830452DC;
	sub_82FD82E0(ctx, base);
	// 830452DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830452E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830452E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830452E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830452EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830452F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830452F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830452F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830452F8 size=80
    let mut pc: u32 = 0x830452F8;
    'dispatch: loop {
        match pc {
            0x830452F8 => {
    //   block [0x830452F8..0x83045348)
	// 830452F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830452FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045300: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83045304: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045308: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 8304530C: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 83045310: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83045314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83045318: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8304531C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83045320: 48006B79  bl 0x8304be98
	ctx.lr = 0x83045324;
	sub_8304BE98(ctx, base);
	// 83045324: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83045328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304532C: 396BF868  addi r11, r11, -0x798
	ctx.r[11].s64 = ctx.r[11].s64 + -1944;
	// 83045330: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83045334: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83045338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304533C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045340: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83045344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83045348 size=8
    let mut pc: u32 = 0x83045348;
    'dispatch: loop {
        match pc {
            0x83045348 => {
    //   block [0x83045348..0x83045350)
	// 83045348: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304534C: 8215F8C8  lwz r16, -0x738(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-1848 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045350 size=84
    let mut pc: u32 = 0x83045350;
    'dispatch: loop {
        match pc {
            0x83045350 => {
    //   block [0x83045350..0x830453A4)
	// 83045350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045354: 48162E15  bl 0x831a8168
	ctx.lr = 0x83045358;
	sub_831A8130(ctx, base);
	// 83045358: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304535C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045360: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83045364: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83045368: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8304536C: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 83045370: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83045374: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83045378: 48006B21  bl 0x8304be98
	ctx.lr = 0x8304537C;
	sub_8304BE98(ctx, base);
	// 8304537C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83045380: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83045384: 396BF868  addi r11, r11, -0x798
	ctx.r[11].s64 = ctx.r[11].s64 + -1944;
	// 83045388: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304538C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83045390: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83045394: 480468C5  bl 0x8308bc58
	ctx.lr = 0x83045398;
	sub_8308BC58(ctx, base);
	// 83045398: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304539C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830453A0: 48162E18  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830453A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830453A4 size=40
    let mut pc: u32 = 0x830453A4;
    'dispatch: loop {
        match pc {
            0x830453A4 => {
    //   block [0x830453A4..0x830453CC)
	// 830453A4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830453A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830453AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830453B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830453B4: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830453B8: 48006A31  bl 0x8304bde8
	ctx.lr = 0x830453BC;
	sub_8304BDE8(ctx, base);
	// 830453BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830453C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830453C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830453C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830453D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830453D0 size=8
    let mut pc: u32 = 0x830453D0;
    'dispatch: loop {
        match pc {
            0x830453D0 => {
    //   block [0x830453D0..0x830453D8)
	// 830453D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830453D4: 8215F900  lwz r16, -0x700(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-1792 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830453D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830453D8 size=104
    let mut pc: u32 = 0x830453D8;
    'dispatch: loop {
        match pc {
            0x830453D8 => {
    //   block [0x830453D8..0x83045440)
	// 830453D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830453DC: 48162D85  bl 0x831a8160
	ctx.lr = 0x830453E0;
	sub_831A8130(ctx, base);
	// 830453E0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830453E4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830453E8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 830453EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830453F0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830453F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830453F8: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 830453FC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83045400: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 83045404: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83045408: 4BF92E91  bl 0x82fd8298
	ctx.lr = 0x8304540C;
	sub_82FD8298(ctx, base);
	// 8304540C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83045410: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83045414: 41820020  beq 0x83045434
	if ctx.cr[0].eq {
	pc = 0x83045434; continue 'dispatch;
	}
	// 83045418: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 8304541C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83045420: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83045424: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83045428: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304542C: 4BFFFF25  bl 0x83045350
	ctx.lr = 0x83045430;
	sub_83045350(ctx, base);
	// 83045430: 48000008  b 0x83045438
	pc = 0x83045438; continue 'dispatch;
	// 83045434: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83045438: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8304543C: 48162D74  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045440 size=44
    let mut pc: u32 = 0x83045440;
    'dispatch: loop {
        match pc {
            0x83045440 => {
    //   block [0x83045440..0x8304546C)
	// 83045440: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83045444: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045448: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304544C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045450: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 83045454: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83045458: 4BF92E89  bl 0x82fd82e0
	ctx.lr = 0x8304545C;
	sub_82FD82E0(ctx, base);
	// 8304545C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83045460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83045464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83045470 size=4
    let mut pc: u32 = 0x83045470;
    'dispatch: loop {
        match pc {
            0x83045470 => {
    //   block [0x83045470..0x83045474)
	// 83045470: 480456D8  b 0x8308ab48
	sub_8308AB48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045478 size=128
    let mut pc: u32 = 0x83045478;
    'dispatch: loop {
        match pc {
            0x83045478 => {
    //   block [0x83045478..0x830454F8)
	// 83045478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304547C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045480: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83045484: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83045488: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304548C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83045490: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83045494: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83045498: 4BF8C039  bl 0x82fd14d0
	ctx.lr = 0x8304549C;
	sub_82FD14D0(ctx, base);
	// 8304549C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830454A0: 40820040  bne 0x830454e0
	if !ctx.cr[0].eq {
	pc = 0x830454E0; continue 'dispatch;
	}
	// 830454A4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830454A8: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 830454AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830454B0: 388BF930  addi r4, r11, -0x6d0
	ctx.r[4].s64 = ctx.r[11].s64 + -1744;
	// 830454B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830454B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830454BC: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 830454C0: 38C000FE  li r6, 0xfe
	ctx.r[6].s64 = 254;
	// 830454C4: 38A00094  li r5, 0x94
	ctx.r[5].s64 = 148;
	// 830454C8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830454CC: 4BFD06BD  bl 0x83015b88
	ctx.lr = 0x830454D0;
	sub_83015B88(ctx, base);
	// 830454D0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830454D4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830454D8: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 830454DC: 4816B74D  bl 0x831b0c28
	ctx.lr = 0x830454E0;
	sub_831B0C28(ctx, base);
	// 830454E0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830454E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830454E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830454EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830454F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830454F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830454F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830454F8 size=8
    let mut pc: u32 = 0x830454F8;
    'dispatch: loop {
        match pc {
            0x830454F8 => {
    //   block [0x830454F8..0x83045500)
	// 830454F8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830454FC: 8215F988  lwz r16, -0x678(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-1656 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045500 size=96
    let mut pc: u32 = 0x83045500;
    'dispatch: loop {
        match pc {
            0x83045500 => {
    //   block [0x83045500..0x83045560)
	// 83045500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045508: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304550C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83045510: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83045514: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045518: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304551C: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 83045520: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83045524: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83045528: 4BF92D71  bl 0x82fd8298
	ctx.lr = 0x8304552C;
	sub_82FD8298(ctx, base);
	// 8304552C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83045530: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83045534: 41820010  beq 0x83045544
	if ctx.cr[0].eq {
	pc = 0x83045544; continue 'dispatch;
	}
	// 83045538: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304553C: 4BFFFDBD  bl 0x830452f8
	ctx.lr = 0x83045540;
	sub_830452F8(ctx, base);
	// 83045540: 48000008  b 0x83045548
	pc = 0x83045548; continue 'dispatch;
	// 83045544: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83045548: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8304554C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83045550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045554: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83045558: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304555C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045560 size=44
    let mut pc: u32 = 0x83045560;
    'dispatch: loop {
        match pc {
            0x83045560 => {
    //   block [0x83045560..0x8304558C)
	// 83045560: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83045564: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045568: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304556C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045570: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83045574: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83045578: 4BF92D69  bl 0x82fd82e0
	ctx.lr = 0x8304557C;
	sub_82FD82E0(ctx, base);
	// 8304557C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83045580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83045584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83045590 size=12
    let mut pc: u32 = 0x83045590;
    'dispatch: loop {
        match pc {
            0x83045590 => {
    //   block [0x83045590..0x8304559C)
	// 83045590: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83045594: 386B3354  addi r3, r11, 0x3354
	ctx.r[3].s64 = ctx.r[11].s64 + 13140;
	// 83045598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830455A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830455A0 size=88
    let mut pc: u32 = 0x830455A0;
    'dispatch: loop {
        match pc {
            0x830455A0 => {
    //   block [0x830455A0..0x830455F8)
	// 830455A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830455A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830455A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830455AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830455B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830455B4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830455B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830455BC: 396BF868  addi r11, r11, -0x798
	ctx.r[11].s64 = ctx.r[11].s64 + -1944;
	// 830455C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830455C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830455C8: 48006821  bl 0x8304bde8
	ctx.lr = 0x830455CC;
	sub_8304BDE8(ctx, base);
	// 830455CC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830455D0: 4182000C  beq 0x830455dc
	if ctx.cr[0].eq {
	pc = 0x830455DC; continue 'dispatch;
	}
	// 830455D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830455D8: 4BF92D09  bl 0x82fd82e0
	ctx.lr = 0x830455DC;
	sub_82FD82E0(ctx, base);
	// 830455DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830455E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830455E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830455E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830455EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830455F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830455F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830455F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830455F8 size=8
    let mut pc: u32 = 0x830455F8;
    'dispatch: loop {
        match pc {
            0x830455F8 => {
    //   block [0x830455F8..0x83045600)
	// 830455F8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830455FC: 8215F9E0  lwz r16, -0x620(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-1568 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045600 size=128
    let mut pc: u32 = 0x83045600;
    'dispatch: loop {
        match pc {
            0x83045600 => {
    //   block [0x83045600..0x83045680)
	// 83045600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045608: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304560C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83045610: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83045614: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045618: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304561C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83045620: 396BF9C8  addi r11, r11, -0x638
	ctx.r[11].s64 = ctx.r[11].s64 + -1592;
	// 83045624: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83045628: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304562C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83045630: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83045634: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83045638: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304563C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83045640: 4E800421  bctrl
	ctx.lr = 0x83045644;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83045644: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83045648: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304564C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83045650: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83045654: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83045658: 4E800421  bctrl
	ctx.lr = 0x8304565C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304565C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83045660: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 83045664: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83045668: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8304566C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83045670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045674: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83045678: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304567C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045680 size=40
    let mut pc: u32 = 0x83045680;
    'dispatch: loop {
        match pc {
            0x83045680 => {
    //   block [0x83045680..0x830456A8)
	// 83045680: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83045684: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045688: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304568C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045690: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83045694: 480070CD  bl 0x8304c760
	ctx.lr = 0x83045698;
	sub_8304C760(ctx, base);
	// 83045698: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304569C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830456A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830456A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830456A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830456A8 size=172
    let mut pc: u32 = 0x830456A8;
    'dispatch: loop {
        match pc {
            0x830456A8 => {
    //   block [0x830456A8..0x83045754)
	// 830456A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830456AC: 48162AC1  bl 0x831a816c
	ctx.lr = 0x830456B0;
	sub_831A8130(ctx, base);
	// 830456B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830456B4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830456B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830456BC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830456C0: 419A0034  beq cr6, 0x830456f4
	if ctx.cr[6].eq {
	pc = 0x830456F4; continue 'dispatch;
	}
	// 830456C4: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830456C8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830456CC: 41820028  beq 0x830456f4
	if ctx.cr[0].eq {
	pc = 0x830456F4; continue 'dispatch;
	}
	// 830456D0: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 830456D4: 48000008  b 0x830456dc
	pc = 0x830456DC; continue 'dispatch;
	// 830456D8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830456DC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830456E0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830456E4: 4082FFF4  bne 0x830456d8
	if !ctx.cr[0].eq {
	pc = 0x830456D8; continue 'dispatch;
	}
	// 830456E8: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 830456EC: 7D7E0E70  srawi r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830456F0: 48000008  b 0x830456f8
	pc = 0x830456F8; continue 'dispatch;
	// 830456F4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830456F8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830456FC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83045700: 41980040  blt cr6, 0x83045740
	if ctx.cr[6].lt {
	pc = 0x83045740; continue 'dispatch;
	}
	// 83045704: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83045708: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304570C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83045710: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83045714: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83045718: 4E800421  bctrl
	ctx.lr = 0x8304571C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304571C: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 83045720: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83045724: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83045728: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8304572C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83045730: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83045734: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83045738: 4E800421  bctrl
	ctx.lr = 0x8304573C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304573C: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83045740: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83045744: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83045748: 4BF8C421  bl 0x82fd1b68
	ctx.lr = 0x8304574C;
	sub_82FD1B68(ctx, base);
	// 8304574C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83045750: 48162A6C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045758 size=172
    let mut pc: u32 = 0x83045758;
    'dispatch: loop {
        match pc {
            0x83045758 => {
    //   block [0x83045758..0x83045804)
	// 83045758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304575C: 48162A11  bl 0x831a816c
	ctx.lr = 0x83045760;
	sub_831A8130(ctx, base);
	// 83045760: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045764: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83045768: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304576C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83045770: 419A0034  beq cr6, 0x830457a4
	if ctx.cr[6].eq {
	pc = 0x830457A4; continue 'dispatch;
	}
	// 83045774: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83045778: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304577C: 41820028  beq 0x830457a4
	if ctx.cr[0].eq {
	pc = 0x830457A4; continue 'dispatch;
	}
	// 83045780: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 83045784: 48000008  b 0x8304578c
	pc = 0x8304578C; continue 'dispatch;
	// 83045788: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8304578C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83045790: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83045794: 4082FFF4  bne 0x83045788
	if !ctx.cr[0].eq {
	pc = 0x83045788; continue 'dispatch;
	}
	// 83045798: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 8304579C: 7D7E0E70  srawi r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830457A0: 48000008  b 0x830457a8
	pc = 0x830457A8; continue 'dispatch;
	// 830457A4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830457A8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830457AC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830457B0: 41980040  blt cr6, 0x830457f0
	if ctx.cr[6].lt {
	pc = 0x830457F0; continue 'dispatch;
	}
	// 830457B4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830457B8: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830457BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830457C0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830457C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830457C8: 4E800421  bctrl
	ctx.lr = 0x830457CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830457CC: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 830457D0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830457D4: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830457D8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830457DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830457E0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830457E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830457E8: 4E800421  bctrl
	ctx.lr = 0x830457EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830457EC: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 830457F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830457F4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830457F8: 4BF8C371  bl 0x82fd1b68
	ctx.lr = 0x830457FC;
	sub_82FD1B68(ctx, base);
	// 830457FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83045800: 481629BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045808 size=68
    let mut pc: u32 = 0x83045808;
    'dispatch: loop {
        match pc {
            0x83045808 => {
    //   block [0x83045808..0x8304584C)
	// 83045808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304580C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045810: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83045814: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83045818: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304581C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83045820: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83045824: 4BFFFE85  bl 0x830456a8
	ctx.lr = 0x83045828;
	sub_830456A8(ctx, base);
	// 83045828: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304582C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83045830: 4BFFFF29  bl 0x83045758
	ctx.lr = 0x83045834;
	sub_83045758(ctx, base);
	// 83045834: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83045838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304583C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045840: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83045844: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83045848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045850 size=104
    let mut pc: u32 = 0x83045850;
    'dispatch: loop {
        match pc {
            0x83045850 => {
    //   block [0x83045850..0x830458B8)
	// 83045850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045858: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304585C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045860: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83045864: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83045868: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8304586C: 4BF92A2D  bl 0x82fd8298
	ctx.lr = 0x83045870;
	sub_82FD8298(ctx, base);
	// 83045870: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83045874: 4182002C  beq 0x830458a0
	if ctx.cr[0].eq {
	pc = 0x830458A0; continue 'dispatch;
	}
	// 83045878: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304587C: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 83045880: 394BF9C8  addi r10, r11, -0x638
	ctx.r[10].s64 = ctx.r[11].s64 + -1592;
	// 83045884: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83045888: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8304588C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83045890: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83045894: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83045898: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8304589C: 48000008  b 0x830458a4
	pc = 0x830458A4; continue 'dispatch;
	// 830458A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830458A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830458A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830458AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830458B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830458B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830458B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830458B8 size=12
    let mut pc: u32 = 0x830458B8;
    'dispatch: loop {
        match pc {
            0x830458B8 => {
    //   block [0x830458B8..0x830458C4)
	// 830458B8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830458BC: 386B335C  addi r3, r11, 0x335c
	ctx.r[3].s64 = ctx.r[11].s64 + 13148;
	// 830458C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830458C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830458C8 size=148
    let mut pc: u32 = 0x830458C8;
    'dispatch: loop {
        match pc {
            0x830458C8 => {
    //   block [0x830458C8..0x8304595C)
	// 830458C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830458CC: 481628A1  bl 0x831a816c
	ctx.lr = 0x830458D0;
	sub_831A8130(ctx, base);
	// 830458D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830458D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830458D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830458DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830458E0: A97E0000  lha r11, 0(r30)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 830458E4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830458E8: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830458EC: 41820030  beq 0x8304591c
	if ctx.cr[0].eq {
	pc = 0x8304591C; continue 'dispatch;
	}
	// 830458F0: 3FA08214  lis r29, -0x7dec
	ctx.r[29].s64 = -2112618496;
	// 830458F4: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830458F8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830458FC: 88DDF638  lbz r6, -0x9c8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(-2504 as u32) ) } as u64;
	// 83045900: 4BFB4001  bl 0x82ff9900
	ctx.lr = 0x83045904;
	sub_82FF9900(ctx, base);
	// 83045904: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83045908: 88DDF638  lbz r6, -0x9c8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(-2504 as u32) ) } as u64;
	// 8304590C: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83045910: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83045914: 4BFB3FED  bl 0x82ff9900
	ctx.lr = 0x83045918;
	sub_82FF9900(ctx, base);
	// 83045918: 4800003C  b 0x83045954
	pc = 0x83045954; continue 'dispatch;
	// 8304591C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83045920: 3FA08214  lis r29, -0x7dec
	ctx.r[29].s64 = -2112618496;
	// 83045924: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83045928: 38BF000C  addi r5, r31, 0xc
	ctx.r[5].s64 = ctx.r[31].s64 + 12;
	// 8304592C: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 83045930: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83045934: 88FDF639  lbz r7, -0x9c7(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(-2503 as u32) ) } as u64;
	// 83045938: 4BFB41F1  bl 0x82ff9b28
	ctx.lr = 0x8304593C;
	sub_82FF9B28(ctx, base);
	// 8304593C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83045940: 88FDF639  lbz r7, -0x9c7(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(-2503 as u32) ) } as u64;
	// 83045944: 38BF0014  addi r5, r31, 0x14
	ctx.r[5].s64 = ctx.r[31].s64 + 20;
	// 83045948: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 8304594C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83045950: 4BFB41D9  bl 0x82ff9b28
	ctx.lr = 0x83045954;
	sub_82FF9B28(ctx, base);
	// 83045954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83045958: 48162864  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045960 size=76
    let mut pc: u32 = 0x83045960;
    'dispatch: loop {
        match pc {
            0x83045960 => {
    //   block [0x83045960..0x830459AC)
	// 83045960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045968: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304596C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83045970: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045974: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83045978: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8304597C: 4BFFFC85  bl 0x83045600
	ctx.lr = 0x83045980;
	sub_83045600(ctx, base);
	// 83045980: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83045984: 4182000C  beq 0x83045990
	if ctx.cr[0].eq {
	pc = 0x83045990; continue 'dispatch;
	}
	// 83045988: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304598C: 4BF92955  bl 0x82fd82e0
	ctx.lr = 0x83045990;
	sub_82FD82E0(ctx, base);
	// 83045990: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83045994: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83045998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304599C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830459A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830459A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830459A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830459B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830459B0 size=8
    let mut pc: u32 = 0x830459B0;
    'dispatch: loop {
        match pc {
            0x830459B0 => {
    //   block [0x830459B0..0x830459B8)
	// 830459B0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830459B4: 8215FA18  lwz r16, -0x5e8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-1512 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830459B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830459B8 size=92
    let mut pc: u32 = 0x830459B8;
    'dispatch: loop {
        match pc {
            0x830459B8 => {
    //   block [0x830459B8..0x83045A14)
	// 830459B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830459BC: 481627B1  bl 0x831a816c
	ctx.lr = 0x830459C0;
	sub_831A8130(ctx, base);
	// 830459C0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830459C4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830459C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830459CC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830459D0: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830459D4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830459D8: 90DE0004  stw r6, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 830459DC: 394BF9C8  addi r10, r11, -0x638
	ctx.r[10].s64 = ctx.r[11].s64 + -1592;
	// 830459E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830459E4: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830459E8: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830459EC: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830459F0: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830459F4: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830459F8: 4BFFFCB1  bl 0x830456a8
	ctx.lr = 0x830459FC;
	sub_830456A8(ctx, base);
	// 830459FC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83045A00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83045A04: 4BFFFD55  bl 0x83045758
	ctx.lr = 0x83045A08;
	sub_83045758(ctx, base);
	// 83045A08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83045A0C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83045A10: 481627AC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045A14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045A14 size=40
    let mut pc: u32 = 0x83045A14;
    'dispatch: loop {
        match pc {
            0x83045A14 => {
    //   block [0x83045A14..0x83045A3C)
	// 83045A14: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83045A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045A20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045A24: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83045A28: 48006D39  bl 0x8304c760
	ctx.lr = 0x83045A2C;
	sub_8304C760(ctx, base);
	// 83045A2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83045A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83045A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83045A40 size=8
    let mut pc: u32 = 0x83045A40;
    'dispatch: loop {
        match pc {
            0x83045A40 => {
    //   block [0x83045A40..0x83045A48)
	// 83045A40: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83045A44: 8215FA50  lwz r16, -0x5b0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-1456 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045A48 size=104
    let mut pc: u32 = 0x83045A48;
    'dispatch: loop {
        match pc {
            0x83045A48 => {
    //   block [0x83045A48..0x83045AB0)
	// 83045A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045A4C: 48162721  bl 0x831a816c
	ctx.lr = 0x83045A50;
	sub_831A8130(ctx, base);
	// 83045A50: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83045A54: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045A58: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83045A5C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83045A60: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83045A64: 3D408216  lis r10, -0x7dea
	ctx.r[10].s64 = -2112487424;
	// 83045A68: 392AF9C8  addi r9, r10, -0x638
	ctx.r[9].s64 = ctx.r[10].s64 + -1592;
	// 83045A6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83045A70: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83045A74: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83045A78: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83045A7C: 915E000C  stw r10, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83045A80: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83045A84: 915E0014  stw r10, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83045A88: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83045A8C: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83045A90: 83AB0010  lwz r29, 0x10(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83045A94: 4BFFFC15  bl 0x830456a8
	ctx.lr = 0x83045A98;
	sub_830456A8(ctx, base);
	// 83045A98: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83045A9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83045AA0: 4BFFFCB9  bl 0x83045758
	ctx.lr = 0x83045AA4;
	sub_83045758(ctx, base);
	// 83045AA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83045AA8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83045AAC: 48162710  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045AB0 size=40
    let mut pc: u32 = 0x83045AB0;
    'dispatch: loop {
        match pc {
            0x83045AB0 => {
    //   block [0x83045AB0..0x83045AD8)
	// 83045AB0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83045AB4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045AB8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045ABC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045AC0: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83045AC4: 48006C9D  bl 0x8304c760
	ctx.lr = 0x83045AC8;
	sub_8304C760(ctx, base);
	// 83045AC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83045ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83045AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045AD8 size=88
    let mut pc: u32 = 0x83045AD8;
    'dispatch: loop {
        match pc {
            0x83045AD8 => {
    //   block [0x83045AD8..0x83045B30)
	// 83045AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045AE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83045AE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045AE8: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 83045AEC: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 83045AF0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83045AF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83045AF8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83045AFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83045B00: 48046261  bl 0x8308bd60
	ctx.lr = 0x83045B04;
	sub_8308BD60(ctx, base);
	// 83045B04: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83045B08: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83045B0C: 396BFAB8  addi r11, r11, -0x548
	ctx.r[11].s64 = ctx.r[11].s64 + -1352;
	// 83045B10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83045B14: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 83045B18: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83045B1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83045B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83045B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045B28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83045B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83045B30 size=8
    let mut pc: u32 = 0x83045B30;
    'dispatch: loop {
        match pc {
            0x83045B30 => {
    //   block [0x83045B30..0x83045B38)
	// 83045B30: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83045B34: 8215FB28  lwz r16, -0x4d8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-1240 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045B38 size=84
    let mut pc: u32 = 0x83045B38;
    'dispatch: loop {
        match pc {
            0x83045B38 => {
    //   block [0x83045B38..0x83045B8C)
	// 83045B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045B3C: 4816262D  bl 0x831a8168
	ctx.lr = 0x83045B40;
	sub_831A8130(ctx, base);
	// 83045B40: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83045B44: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045B48: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83045B4C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83045B50: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 83045B54: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 83045B58: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83045B5C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83045B60: 48046201  bl 0x8308bd60
	ctx.lr = 0x83045B64;
	sub_8308BD60(ctx, base);
	// 83045B64: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83045B68: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83045B6C: 396BFAB8  addi r11, r11, -0x548
	ctx.r[11].s64 = ctx.r[11].s64 + -1352;
	// 83045B70: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83045B74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83045B78: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83045B7C: 48048A25  bl 0x8308e5a0
	ctx.lr = 0x83045B80;
	sub_8308E5A0(ctx, base);
	// 83045B80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83045B84: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83045B88: 48162630  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045B8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045B8C size=40
    let mut pc: u32 = 0x83045B8C;
    'dispatch: loop {
        match pc {
            0x83045B8C => {
    //   block [0x83045B8C..0x83045BB4)
	// 83045B8C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83045B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045B98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045B9C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83045BA0: 48046159  bl 0x8308bcf8
	ctx.lr = 0x83045BA4;
	sub_8308BCF8(ctx, base);
	// 83045BA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83045BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83045BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83045BB8 size=8
    let mut pc: u32 = 0x83045BB8;
    'dispatch: loop {
        match pc {
            0x83045BB8 => {
    //   block [0x83045BB8..0x83045BC0)
	// 83045BB8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83045BBC: 8215FB60  lwz r16, -0x4a0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-1184 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045BC0 size=104
    let mut pc: u32 = 0x83045BC0;
    'dispatch: loop {
        match pc {
            0x83045BC0 => {
    //   block [0x83045BC0..0x83045C28)
	// 83045BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045BC4: 4816259D  bl 0x831a8160
	ctx.lr = 0x83045BC8;
	sub_831A8130(ctx, base);
	// 83045BC8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83045BCC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045BD0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 83045BD4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83045BD8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83045BDC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83045BE0: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83045BE4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83045BE8: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 83045BEC: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83045BF0: 4BF926A9  bl 0x82fd8298
	ctx.lr = 0x83045BF4;
	sub_82FD8298(ctx, base);
	// 83045BF4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83045BF8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83045BFC: 41820020  beq 0x83045c1c
	if ctx.cr[0].eq {
	pc = 0x83045C1C; continue 'dispatch;
	}
	// 83045C00: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 83045C04: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83045C08: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83045C0C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83045C10: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83045C14: 4BFFFF25  bl 0x83045b38
	ctx.lr = 0x83045C18;
	sub_83045B38(ctx, base);
	// 83045C18: 48000008  b 0x83045c20
	pc = 0x83045C20; continue 'dispatch;
	// 83045C1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83045C20: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83045C24: 4816258C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045C28 size=44
    let mut pc: u32 = 0x83045C28;
    'dispatch: loop {
        match pc {
            0x83045C28 => {
    //   block [0x83045C28..0x83045C54)
	// 83045C28: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83045C2C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045C30: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045C34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045C38: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 83045C3C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83045C40: 4BF926A1  bl 0x82fd82e0
	ctx.lr = 0x83045C44;
	sub_82FD82E0(ctx, base);
	// 83045C44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83045C48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83045C4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83045C58 size=8
    let mut pc: u32 = 0x83045C58;
    'dispatch: loop {
        match pc {
            0x83045C58 => {
    //   block [0x83045C58..0x83045C60)
	// 83045C58: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83045C5C: 8215FBDC  lwz r16, -0x424(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-1060 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045C60 size=116
    let mut pc: u32 = 0x83045C60;
    'dispatch: loop {
        match pc {
            0x83045C60 => {
    //   block [0x83045C60..0x83045CD4)
	// 83045C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045C64: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83045C68: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83045C6C: 48162501  bl 0x831a816c
	ctx.lr = 0x83045C70;
	sub_831A8130(ctx, base);
	// 83045C70: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83045C74: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045C78: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83045C7C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83045C80: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83045C84: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 83045C88: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83045C8C: 4BF9260D  bl 0x82fd8298
	ctx.lr = 0x83045C90;
	sub_82FD8298(ctx, base);
	// 83045C90: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83045C94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83045C98: 41820018  beq 0x83045cb0
	if ctx.cr[0].eq {
	pc = 0x83045CB0; continue 'dispatch;
	}
	// 83045C9C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83045CA0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83045CA4: 48049F7D  bl 0x8308fc20
	ctx.lr = 0x83045CA8;
	sub_8308FC20(ctx, base);
	// 83045CA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83045CAC: 48000008  b 0x83045cb4
	pc = 0x83045CB4; continue 'dispatch;
	// 83045CB0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83045CB4: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83045CB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83045CBC: 4804B225  bl 0x83090ee0
	ctx.lr = 0x83045CC0;
	sub_83090EE0(ctx, base);
	// 83045CC0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83045CC4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83045CC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83045CCC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83045CD0: 481624EC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045CD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83045CD4 size=8
    let mut pc: u32 = 0x83045CD4;
    'dispatch: loop {
        match pc {
            0x83045CD4 => {
    //   block [0x83045CD4..0x83045CDC)
	// 83045CD4: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83045CD8: 8215FBDC  lwz r16, -0x424(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-1060 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045CDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045CDC size=24
    let mut pc: u32 = 0x83045CDC;
    'dispatch: loop {
        match pc {
            0x83045CDC => {
    //   block [0x83045CDC..0x83045CF4)
	// 83045CDC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045CE0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045CE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045CE8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83045CEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83045CF0: 4816AF39  bl 0x831b0c28
	ctx.lr = 0x83045CF4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045CFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045CFC size=60
    let mut pc: u32 = 0x83045CFC;
    'dispatch: loop {
        match pc {
            0x83045CFC => {
    //   block [0x83045CFC..0x83045D38)
	// 83045CFC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83045D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045D08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045D0C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83045D10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83045D14: 419A0018  beq cr6, 0x83045d2c
	if ctx.cr[6].eq {
	pc = 0x83045D2C; continue 'dispatch;
	}
	// 83045D18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83045D1C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83045D20: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83045D24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83045D28: 4E800421  bctrl
	ctx.lr = 0x83045D2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83045D2C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83045D30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83045D34: 4816AEF5  bl 0x831b0c28
	ctx.lr = 0x83045D38;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045D38 size=44
    let mut pc: u32 = 0x83045D38;
    'dispatch: loop {
        match pc {
            0x83045D38 => {
    //   block [0x83045D38..0x83045D64)
	// 83045D38: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83045D3C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045D40: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045D44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045D48: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83045D4C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83045D50: 4BF92591  bl 0x82fd82e0
	ctx.lr = 0x83045D54;
	sub_82FD82E0(ctx, base);
	// 83045D54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83045D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83045D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83045D68 size=8
    let mut pc: u32 = 0x83045D68;
    'dispatch: loop {
        match pc {
            0x83045D68 => {
    //   block [0x83045D68..0x83045D70)
	// 83045D68: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83045D6C: 4804B174  b 0x83090ee0
	sub_83090EE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83045D70 size=16
    let mut pc: u32 = 0x83045D70;
    'dispatch: loop {
        match pc {
            0x83045D70 => {
    //   block [0x83045D70..0x83045D80)
	// 83045D70: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83045D74: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83045D78: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 83045D7C: 4804A52C  b 0x830902a8
	sub_830902A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83045D80 size=8
    let mut pc: u32 = 0x83045D80;
    'dispatch: loop {
        match pc {
            0x83045D80 => {
    //   block [0x83045D80..0x83045D88)
	// 83045D80: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83045D84: 8215FC40  lwz r16, -0x3c0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-960 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045D88 size=96
    let mut pc: u32 = 0x83045D88;
    'dispatch: loop {
        match pc {
            0x83045D88 => {
    //   block [0x83045D88..0x83045DE8)
	// 83045D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045D90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83045D94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83045D98: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83045D9C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045DA0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83045DA4: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83045DA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83045DAC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83045DB0: 4BF924E9  bl 0x82fd8298
	ctx.lr = 0x83045DB4;
	sub_82FD8298(ctx, base);
	// 83045DB4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83045DB8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83045DBC: 41820010  beq 0x83045dcc
	if ctx.cr[0].eq {
	pc = 0x83045DCC; continue 'dispatch;
	}
	// 83045DC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83045DC4: 4BFFFD15  bl 0x83045ad8
	ctx.lr = 0x83045DC8;
	sub_83045AD8(ctx, base);
	// 83045DC8: 48000008  b 0x83045dd0
	pc = 0x83045DD0; continue 'dispatch;
	// 83045DCC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83045DD0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83045DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83045DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045DDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83045DE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83045DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045DE8 size=44
    let mut pc: u32 = 0x83045DE8;
    'dispatch: loop {
        match pc {
            0x83045DE8 => {
    //   block [0x83045DE8..0x83045E14)
	// 83045DE8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83045DEC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045DF0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045DF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045DF8: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83045DFC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83045E00: 4BF924E1  bl 0x82fd82e0
	ctx.lr = 0x83045E04;
	sub_82FD82E0(ctx, base);
	// 83045E04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83045E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83045E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83045E18 size=12
    let mut pc: u32 = 0x83045E18;
    'dispatch: loop {
        match pc {
            0x83045E18 => {
    //   block [0x83045E18..0x83045E24)
	// 83045E18: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83045E1C: 386B3364  addi r3, r11, 0x3364
	ctx.r[3].s64 = ctx.r[11].s64 + 13156;
	// 83045E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045E28 size=88
    let mut pc: u32 = 0x83045E28;
    'dispatch: loop {
        match pc {
            0x83045E28 => {
    //   block [0x83045E28..0x83045E80)
	// 83045E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045E30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83045E34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83045E38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045E3C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83045E40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83045E44: 396BFAB8  addi r11, r11, -0x548
	ctx.r[11].s64 = ctx.r[11].s64 + -1352;
	// 83045E48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83045E4C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83045E50: 48045EA9  bl 0x8308bcf8
	ctx.lr = 0x83045E54;
	sub_8308BCF8(ctx, base);
	// 83045E54: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83045E58: 4182000C  beq 0x83045e64
	if ctx.cr[0].eq {
	pc = 0x83045E64; continue 'dispatch;
	}
	// 83045E5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83045E60: 4BF92481  bl 0x82fd82e0
	ctx.lr = 0x83045E64;
	sub_82FD82E0(ctx, base);
	// 83045E64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83045E68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83045E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83045E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045E74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83045E78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83045E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045E80 size=88
    let mut pc: u32 = 0x83045E80;
    'dispatch: loop {
        match pc {
            0x83045E80 => {
    //   block [0x83045E80..0x83045ED8)
	// 83045E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045E88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83045E8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045E90: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 83045E94: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 83045E98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83045E9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83045EA0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83045EA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83045EA8: 48045EB9  bl 0x8308bd60
	ctx.lr = 0x83045EAC;
	sub_8308BD60(ctx, base);
	// 83045EAC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83045EB0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83045EB4: 396BFCA8  addi r11, r11, -0x358
	ctx.r[11].s64 = ctx.r[11].s64 + -856;
	// 83045EB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83045EBC: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 83045EC0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83045EC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83045EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83045ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045ED0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83045ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83045ED8 size=8
    let mut pc: u32 = 0x83045ED8;
    'dispatch: loop {
        match pc {
            0x83045ED8 => {
    //   block [0x83045ED8..0x83045EE0)
	// 83045ED8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83045EDC: 8215FD18  lwz r16, -0x2e8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-744 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045EE0 size=84
    let mut pc: u32 = 0x83045EE0;
    'dispatch: loop {
        match pc {
            0x83045EE0 => {
    //   block [0x83045EE0..0x83045F34)
	// 83045EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045EE4: 48162285  bl 0x831a8168
	ctx.lr = 0x83045EE8;
	sub_831A8130(ctx, base);
	// 83045EE8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83045EEC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045EF0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83045EF4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83045EF8: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 83045EFC: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 83045F00: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83045F04: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83045F08: 48045E59  bl 0x8308bd60
	ctx.lr = 0x83045F0C;
	sub_8308BD60(ctx, base);
	// 83045F0C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83045F10: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83045F14: 396BFCA8  addi r11, r11, -0x358
	ctx.r[11].s64 = ctx.r[11].s64 + -856;
	// 83045F18: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83045F1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83045F20: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83045F24: 4804867D  bl 0x8308e5a0
	ctx.lr = 0x83045F28;
	sub_8308E5A0(ctx, base);
	// 83045F28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83045F2C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83045F30: 48162288  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045F34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045F34 size=40
    let mut pc: u32 = 0x83045F34;
    'dispatch: loop {
        match pc {
            0x83045F34 => {
    //   block [0x83045F34..0x83045F5C)
	// 83045F34: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83045F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045F40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045F44: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83045F48: 48045DB1  bl 0x8308bcf8
	ctx.lr = 0x83045F4C;
	sub_8308BCF8(ctx, base);
	// 83045F4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83045F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83045F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83045F60 size=8
    let mut pc: u32 = 0x83045F60;
    'dispatch: loop {
        match pc {
            0x83045F60 => {
    //   block [0x83045F60..0x83045F68)
	// 83045F60: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83045F64: 8215FD50  lwz r16, -0x2b0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-688 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045F68 size=104
    let mut pc: u32 = 0x83045F68;
    'dispatch: loop {
        match pc {
            0x83045F68 => {
    //   block [0x83045F68..0x83045FD0)
	// 83045F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045F6C: 481621F5  bl 0x831a8160
	ctx.lr = 0x83045F70;
	sub_831A8130(ctx, base);
	// 83045F70: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83045F74: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045F78: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 83045F7C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83045F80: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83045F84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83045F88: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83045F8C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83045F90: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 83045F94: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83045F98: 4BF92301  bl 0x82fd8298
	ctx.lr = 0x83045F9C;
	sub_82FD8298(ctx, base);
	// 83045F9C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83045FA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83045FA4: 41820020  beq 0x83045fc4
	if ctx.cr[0].eq {
	pc = 0x83045FC4; continue 'dispatch;
	}
	// 83045FA8: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 83045FAC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83045FB0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83045FB4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83045FB8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83045FBC: 4BFFFF25  bl 0x83045ee0
	ctx.lr = 0x83045FC0;
	sub_83045EE0(ctx, base);
	// 83045FC0: 48000008  b 0x83045fc8
	pc = 0x83045FC8; continue 'dispatch;
	// 83045FC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83045FC8: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83045FCC: 481621E4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83045FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83045FD0 size=44
    let mut pc: u32 = 0x83045FD0;
    'dispatch: loop {
        match pc {
            0x83045FD0 => {
    //   block [0x83045FD0..0x83045FFC)
	// 83045FD0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83045FD4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83045FD8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83045FDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83045FE0: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 83045FE4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83045FE8: 4BF922F9  bl 0x82fd82e0
	ctx.lr = 0x83045FEC;
	sub_82FD82E0(ctx, base);
	// 83045FEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83045FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83045FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83045FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046000 size=8
    let mut pc: u32 = 0x83046000;
    'dispatch: loop {
        match pc {
            0x83046000 => {
    //   block [0x83046000..0x83046008)
	// 83046000: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83046004: 8215FDCC  lwz r16, -0x234(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-564 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046008 size=116
    let mut pc: u32 = 0x83046008;
    'dispatch: loop {
        match pc {
            0x83046008 => {
    //   block [0x83046008..0x8304607C)
	// 83046008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304600C: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83046010: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83046014: 48162159  bl 0x831a816c
	ctx.lr = 0x83046018;
	sub_831A8130(ctx, base);
	// 83046018: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304601C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046020: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83046024: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83046028: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304602C: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 83046030: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83046034: 4BF92265  bl 0x82fd8298
	ctx.lr = 0x83046038;
	sub_82FD8298(ctx, base);
	// 83046038: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304603C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83046040: 41820018  beq 0x83046058
	if ctx.cr[0].eq {
	pc = 0x83046058; continue 'dispatch;
	}
	// 83046044: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83046048: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304604C: 48049BD5  bl 0x8308fc20
	ctx.lr = 0x83046050;
	sub_8308FC20(ctx, base);
	// 83046050: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83046054: 48000008  b 0x8304605c
	pc = 0x8304605C; continue 'dispatch;
	// 83046058: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304605C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83046060: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83046064: 4804AE1D  bl 0x83090e80
	ctx.lr = 0x83046068;
	sub_83090E80(ctx, base);
	// 83046068: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304606C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83046070: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83046074: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83046078: 48162144  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304607C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304607C size=8
    let mut pc: u32 = 0x8304607C;
    'dispatch: loop {
        match pc {
            0x8304607C => {
    //   block [0x8304607C..0x83046084)
	// 8304607C: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83046080: 8215FDCC  lwz r16, -0x234(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-564 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046084(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046084 size=24
    let mut pc: u32 = 0x83046084;
    'dispatch: loop {
        match pc {
            0x83046084 => {
    //   block [0x83046084..0x8304609C)
	// 83046084: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046088: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304608C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046090: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83046094: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83046098: 4816AB91  bl 0x831b0c28
	ctx.lr = 0x8304609C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830460A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830460A4 size=60
    let mut pc: u32 = 0x830460A4;
    'dispatch: loop {
        match pc {
            0x830460A4 => {
    //   block [0x830460A4..0x830460E0)
	// 830460A4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830460A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830460AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830460B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830460B4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830460B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830460BC: 419A0018  beq cr6, 0x830460d4
	if ctx.cr[6].eq {
	pc = 0x830460D4; continue 'dispatch;
	}
	// 830460C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830460C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830460C8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830460CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830460D0: 4E800421  bctrl
	ctx.lr = 0x830460D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830460D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830460D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830460DC: 4816AB4D  bl 0x831b0c28
	ctx.lr = 0x830460E0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830460E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830460E0 size=44
    let mut pc: u32 = 0x830460E0;
    'dispatch: loop {
        match pc {
            0x830460E0 => {
    //   block [0x830460E0..0x8304610C)
	// 830460E0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830460E4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830460E8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830460EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830460F0: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830460F4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830460F8: 4BF921E9  bl 0x82fd82e0
	ctx.lr = 0x830460FC;
	sub_82FD82E0(ctx, base);
	// 830460FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83046100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046110 size=8
    let mut pc: u32 = 0x83046110;
    'dispatch: loop {
        match pc {
            0x83046110 => {
    //   block [0x83046110..0x83046118)
	// 83046110: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83046114: 4804AD6C  b 0x83090e80
	sub_83090E80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046118 size=8
    let mut pc: u32 = 0x83046118;
    'dispatch: loop {
        match pc {
            0x83046118 => {
    //   block [0x83046118..0x83046120)
	// 83046118: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304611C: 8215FE30  lwz r16, -0x1d0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-464 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046120 size=96
    let mut pc: u32 = 0x83046120;
    'dispatch: loop {
        match pc {
            0x83046120 => {
    //   block [0x83046120..0x83046180)
	// 83046120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046128: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304612C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83046130: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83046134: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046138: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304613C: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83046140: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83046144: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83046148: 4BF92151  bl 0x82fd8298
	ctx.lr = 0x8304614C;
	sub_82FD8298(ctx, base);
	// 8304614C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83046150: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83046154: 41820010  beq 0x83046164
	if ctx.cr[0].eq {
	pc = 0x83046164; continue 'dispatch;
	}
	// 83046158: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304615C: 4BFFFD25  bl 0x83045e80
	ctx.lr = 0x83046160;
	sub_83045E80(ctx, base);
	// 83046160: 48000008  b 0x83046168
	pc = 0x83046168; continue 'dispatch;
	// 83046164: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83046168: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8304616C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046174: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83046178: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304617C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046180 size=44
    let mut pc: u32 = 0x83046180;
    'dispatch: loop {
        match pc {
            0x83046180 => {
    //   block [0x83046180..0x830461AC)
	// 83046180: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83046184: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046188: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304618C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046190: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83046194: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83046198: 4BF92149  bl 0x82fd82e0
	ctx.lr = 0x8304619C;
	sub_82FD82E0(ctx, base);
	// 8304619C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830461A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830461A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830461A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830461B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830461B0 size=12
    let mut pc: u32 = 0x830461B0;
    'dispatch: loop {
        match pc {
            0x830461B0 => {
    //   block [0x830461B0..0x830461BC)
	// 830461B0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830461B4: 386B336C  addi r3, r11, 0x336c
	ctx.r[3].s64 = ctx.r[11].s64 + 13164;
	// 830461B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830461C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830461C0 size=88
    let mut pc: u32 = 0x830461C0;
    'dispatch: loop {
        match pc {
            0x830461C0 => {
    //   block [0x830461C0..0x83046218)
	// 830461C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830461C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830461C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830461CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830461D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830461D4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830461D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830461DC: 396BFCA8  addi r11, r11, -0x358
	ctx.r[11].s64 = ctx.r[11].s64 + -856;
	// 830461E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830461E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830461E8: 48045B11  bl 0x8308bcf8
	ctx.lr = 0x830461EC;
	sub_8308BCF8(ctx, base);
	// 830461EC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830461F0: 4182000C  beq 0x830461fc
	if ctx.cr[0].eq {
	pc = 0x830461FC; continue 'dispatch;
	}
	// 830461F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830461F8: 4BF920E9  bl 0x82fd82e0
	ctx.lr = 0x830461FC;
	sub_82FD82E0(ctx, base);
	// 830461FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83046200: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83046204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304620C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83046210: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83046214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046218 size=88
    let mut pc: u32 = 0x83046218;
    'dispatch: loop {
        match pc {
            0x83046218 => {
    //   block [0x83046218..0x83046270)
	// 83046218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304621C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046220: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83046224: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046228: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 8304622C: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 83046230: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83046234: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83046238: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8304623C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83046240: 48045B21  bl 0x8308bd60
	ctx.lr = 0x83046244;
	sub_8308BD60(ctx, base);
	// 83046244: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83046248: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8304624C: 396BFE90  addi r11, r11, -0x170
	ctx.r[11].s64 = ctx.r[11].s64 + -368;
	// 83046250: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83046254: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 83046258: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304625C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83046260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046268: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304626C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046270 size=8
    let mut pc: u32 = 0x83046270;
    'dispatch: loop {
        match pc {
            0x83046270 => {
    //   block [0x83046270..0x83046278)
	// 83046270: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83046274: 8215FF00  lwz r16, -0x100(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-256 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


