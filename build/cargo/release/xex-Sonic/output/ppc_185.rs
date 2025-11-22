pub fn sub_82DECEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DECEE8 size=188
    let mut pc: u32 = 0x82DECEE8;
    'dispatch: loop {
        match pc {
            0x82DECEE8 => {
    //   block [0x82DECEE8..0x82DECFA4)
	// 82DECEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DECEEC: 483BB27D  bl 0x831a8168
	ctx.lr = 0x82DECEF0;
	sub_831A8130(ctx, base);
	// 82DECEF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DECEF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DECEF8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DECEFC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DECF00: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DECF04: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DECF08: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DECF0C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DECF10: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DECF14: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 82DECF18: 409A0084  bne cr6, 0x82decf9c
	if !ctx.cr[6].eq {
	pc = 0x82DECF9C; continue 'dispatch;
	}
	// 82DECF1C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DECF20: 93A40000  stw r29, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DECF24: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DECF28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DECF2C: 409A002C  bne cr6, 0x82decf58
	if !ctx.cr[6].eq {
	pc = 0x82DECF58; continue 'dispatch;
	}
	// 82DECF30: 838605B0  lwz r28, 0x5b0(r6)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DECF34: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82DECF38: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DECF3C: 4BFB8645  bl 0x82da5580
	ctx.lr = 0x82DECF40;
	sub_82DA5580(ctx, base);
	// 82DECF40: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82DECF44: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DECF48: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DECF4C: 93A30020  stw r29, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82DECF50: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DECF54: 48000044  b 0x82decf98
	pc = 0x82DECF98; continue 'dispatch;
	// 82DECF58: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DECF5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DECF60: 409A0030  bne cr6, 0x82decf90
	if !ctx.cr[6].eq {
	pc = 0x82DECF90; continue 'dispatch;
	}
	// 82DECF64: 83E605B0  lwz r31, 0x5b0(r6)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DECF68: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82DECF6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DECF70: 4BFB8611  bl 0x82da5580
	ctx.lr = 0x82DECF74;
	sub_82DA5580(ctx, base);
	// 82DECF74: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DECF78: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DECF7C: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82DECF80: 916A001C  stw r11, 0x1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82DECF84: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DECF88: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DECF8C: 93AB001C  stw r29, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82DECF90: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DECF94: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DECF98: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DECF9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DECFA0: 483BB218  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DECFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DECFA8 size=20
    let mut pc: u32 = 0x82DECFA8;
    'dispatch: loop {
        match pc {
            0x82DECFA8 => {
    //   block [0x82DECFA8..0x82DECFBC)
	// 82DECFA8: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DECFAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DECFB0: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DECFB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DECFB8: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DECFBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DECFBC size=36
    let mut pc: u32 = 0x82DECFBC;
    'dispatch: loop {
        match pc {
            0x82DECFBC => {
    //   block [0x82DECFBC..0x82DECFE0)
	// 82DECFBC: 39690008  addi r11, r9, 8
	ctx.r[11].s64 = ctx.r[9].s64 + 8;
	// 82DECFC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DECFC4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DECFC8: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DECFCC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DECFD0: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DECFD4: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DECFD8: 4198FFE8  blt cr6, 0x82decfc0
	if ctx.cr[6].lt {
	pc = 0x82DECFC0; continue 'dispatch;
	}
	// 82DECFDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DECFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DECFE0 size=76
    let mut pc: u32 = 0x82DECFE0;
    'dispatch: loop {
        match pc {
            0x82DECFE0 => {
    //   block [0x82DECFE0..0x82DED02C)
	// 82DECFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DECFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DECFE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DECFEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DECFF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DECFF4: 4BFFFE5D  bl 0x82dece50
	ctx.lr = 0x82DECFF8;
	sub_82DECE50(ctx, base);
	// 82DECFF8: 546BD97E  srwi r11, r3, 5
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shr(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DECFFC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED000: 546906FE  clrlwi r9, r3, 0x1b
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x0000001Fu64;
	// 82DED004: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82DED008: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DED00C: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DED010: 7D6B4C30  srw r11, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82DED014: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82DED018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DED01C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DED020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DED024: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DED028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DED030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DED030 size=84
    let mut pc: u32 = 0x82DED030;
    'dispatch: loop {
        match pc {
            0x82DED030 => {
    //   block [0x82DED030..0x82DED084)
	// 82DED030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DED034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DED038: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DED03C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DED040: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DED044: 4BFFFE0D  bl 0x82dece50
	ctx.lr = 0x82DED048;
	sub_82DECE50(ctx, base);
	// 82DED048: 546AD97E  srwi r10, r3, 5
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shr(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DED04C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED050: 546806FE  clrlwi r8, r3, 0x1b
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x0000001Fu64;
	// 82DED054: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82DED058: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DED05C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DED060: 7D294030  slw r9, r9, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82DED064: 7D0A582E  lwzx r8, r10, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DED068: 7D294378  or r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82DED06C: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82DED070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DED074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DED078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DED07C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DED080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DED088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DED088 size=16
    let mut pc: u32 = 0x82DED088;
    'dispatch: loop {
        match pc {
            0x82DED088 => {
    //   block [0x82DED088..0x82DED098)
	// 82DED088: 816400E4  lwz r11, 0xe4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DED08C: 80640038  lwz r3, 0x38(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DED090: 556BCFFF  rlwinm. r11, r11, 0x19, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DED094: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DED098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DED098 size=20
    let mut pc: u32 = 0x82DED098;
    'dispatch: loop {
        match pc {
            0x82DED098 => {
    //   block [0x82DED098..0x82DED0AC)
	// 82DED098: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DED09C: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DED0A0: 816B5538  lwz r11, 0x5538(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21816 as u32) ) } as u64;
	// 82DED0A4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DED0A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DED0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DED0B0 size=124
    let mut pc: u32 = 0x82DED0B0;
    'dispatch: loop {
        match pc {
            0x82DED0B0 => {
    //   block [0x82DED0B0..0x82DED12C)
	// 82DED0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DED0B4: 483BB0B1  bl 0x831a8164
	ctx.lr = 0x82DED0B8;
	sub_831A8130(ctx, base);
	// 82DED0B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DED0BC: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82DED0C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DED0C4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DED0C8: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DED0CC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DED0D0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DED0D4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DED0D8: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DED0DC: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82DED0E0: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DED0E4: 939F000C  stw r28, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82DED0E8: 816B553C  lwz r11, 0x553c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21820 as u32) ) } as u64;
	// 82DED0EC: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82DED0F0: 837D05B0  lwz r27, 0x5b0(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DED0F4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DED0F8: 4BFB8489  bl 0x82da5580
	ctx.lr = 0x82DED0FC;
	sub_82DA5580(ctx, base);
	// 82DED0FC: 37C30004  addic. r30, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DED100: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82DED104: 41820014  beq 0x82ded118
	if ctx.cr[0].eq {
	pc = 0x82DED118; continue 'dispatch;
	}
	// 82DED108: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DED10C: 809D05B0  lwz r4, 0x5b0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DED110: 4BFD1D49  bl 0x82dbee58
	ctx.lr = 0x82DED114;
	sub_82DBEE58(ctx, base);
	// 82DED114: 48000008  b 0x82ded11c
	pc = 0x82DED11C; continue 'dispatch;
	// 82DED118: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82DED11C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82DED120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DED124: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DED128: 483BB08C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DED130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DED130 size=384
    let mut pc: u32 = 0x82DED130;
    'dispatch: loop {
        match pc {
            0x82DED130 => {
    //   block [0x82DED130..0x82DED2B0)
	// 82DED130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DED134: 483BB01D  bl 0x831a8150
	ctx.lr = 0x82DED138;
	sub_831A8130(ctx, base);
	// 82DED138: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DED13C: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 82DED140: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82DED144: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DED148: 80770004  lwz r3, 4(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED14C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED150: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DED154: 40980014  bge cr6, 0x82ded168
	if !ctx.cr[6].lt {
	pc = 0x82DED168; continue 'dispatch;
	}
	// 82DED158: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED15C: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DED160: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DED164: 4800000C  b 0x82ded170
	pc = 0x82DED170; continue 'dispatch;
	// 82DED168: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DED16C: 4BFEF6AD  bl 0x82ddc818
	ctx.lr = 0x82DED170;
	sub_82DDC818(ctx, base);
	// 82DED170: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED174: 82C30000  lwz r22, 0(r3)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED178: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DED17C: 409A0010  bne cr6, 0x82ded18c
	if !ctx.cr[6].eq {
	pc = 0x82DED18C; continue 'dispatch;
	}
	// 82DED180: 81760008  lwz r11, 8(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED184: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DED188: 4800002C  b 0x82ded1b4
	pc = 0x82DED1B4; continue 'dispatch;
	// 82DED18C: 394B001C  addi r10, r11, 0x1c
	ctx.r[10].s64 = ctx.r[11].s64 + 28;
	// 82DED190: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED194: 48000010  b 0x82ded1a4
	pc = 0x82DED1A4; continue 'dispatch;
	// 82DED198: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED19C: 394B001C  addi r10, r11, 0x1c
	ctx.r[10].s64 = ctx.r[11].s64 + 28;
	// 82DED1A0: 812B001C  lwz r9, 0x1c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DED1A4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DED1A8: 409AFFF0  bne cr6, 0x82ded198
	if !ctx.cr[6].eq {
	pc = 0x82DED198; continue 'dispatch;
	}
	// 82DED1AC: 81560008  lwz r10, 8(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED1B0: 914B001C  stw r10, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82DED1B4: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82DED1B8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DED1BC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DED1C0: 4BFFFB99  bl 0x82decd58
	ctx.lr = 0x82DED1C4;
	sub_82DECD58(ctx, base);
	// 82DED1C4: 80770004  lwz r3, 4(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED1C8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED1CC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DED1D0: 40980014  bge cr6, 0x82ded1e4
	if !ctx.cr[6].lt {
	pc = 0x82DED1E4; continue 'dispatch;
	}
	// 82DED1D4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED1D8: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DED1DC: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DED1E0: 4800000C  b 0x82ded1ec
	pc = 0x82DED1EC; continue 'dispatch;
	// 82DED1E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DED1E8: 4BFEF631  bl 0x82ddc818
	ctx.lr = 0x82DED1EC;
	sub_82DDC818(ctx, base);
	// 82DED1EC: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED1F0: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82DED1F4: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 82DED1F8: 7F1AC378  mr r26, r24
	ctx.r[26].u64 = ctx.r[24].u64;
	// 82DED1FC: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED200: 83BB0008  lwz r29, 8(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED204: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DED208: 40990098  ble cr6, 0x82ded2a0
	if !ctx.cr[6].gt {
	pc = 0x82DED2A0; continue 'dispatch;
	}
	// 82DED20C: 3F808338  lis r28, -0x7cc8
	ctx.r[28].s64 = -2093481984;
	// 82DED210: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DED214: 2F1E0007  cmpwi cr6, r30, 7
	ctx.cr[6].compare_i32(ctx.r[30].s32, 7, &mut ctx.xer);
	// 82DED218: 409A000C  bne cr6, 0x82ded224
	if !ctx.cr[6].eq {
	pc = 0x82DED224; continue 'dispatch;
	}
	// 82DED21C: 83BD001C  lwz r29, 0x1c(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DED220: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82DED224: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DED228: 81390000  lwz r9, 0(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED22C: 817C553C  lwz r11, 0x553c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(21820 as u32) ) } as u64;
	// 82DED230: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DED234: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DED238: 7FEAE82E  lwzx r31, r10, r29
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DED23C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DED240: 7C89582E  lwzx r4, r9, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DED244: 4BFFFD9D  bl 0x82decfe0
	ctx.lr = 0x82DED248;
	sub_82DECFE0(ctx, base);
	// 82DED248: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DED24C: 40820044  bne 0x82ded290
	if !ctx.cr[0].eq {
	pc = 0x82DED290; continue 'dispatch;
	}
	// 82DED250: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82DED254: 80D70008  lwz r6, 8(r23)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED258: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DED25C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DED260: 4BFFFC89  bl 0x82decee8
	ctx.lr = 0x82DED264;
	sub_82DECEE8(ctx, base);
	// 82DED264: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DED268: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DED26C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DED270: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DED274: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DED278: 7FEB512E  stwx r31, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 82DED27C: 817C553C  lwz r11, 0x553c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(21820 as u32) ) } as u64;
	// 82DED280: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED284: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DED288: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DED28C: 4BFFFDA5  bl 0x82ded030
	ctx.lr = 0x82DED290;
	sub_82DED030(ctx, base);
	// 82DED290: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED294: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82DED298: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DED29C: 4198FF74  blt cr6, 0x82ded210
	if ctx.cr[6].lt {
	pc = 0x82DED210; continue 'dispatch;
	}
	// 82DED2A0: 93160004  stw r24, 4(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(4 as u32), ctx.r[24].u32 ) };
	// 82DED2A4: 93160008  stw r24, 8(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 82DED2A8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DED2AC: 483BAEF4  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DED2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DED2B0 size=284
    let mut pc: u32 = 0x82DED2B0;
    'dispatch: loop {
        match pc {
            0x82DED2B0 => {
    //   block [0x82DED2B0..0x82DED3CC)
	// 82DED2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DED2B4: 483BAEB9  bl 0x831a816c
	ctx.lr = 0x82DED2B8;
	sub_831A8130(ctx, base);
	// 82DED2B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DED2BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DED2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DED2C4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82DED2C8: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82DED2CC: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82DED2D0: 90BF0010  stw r5, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82DED2D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DED2D8: 419A00E8  beq cr6, 0x82ded3c0
	if ctx.cr[6].eq {
	pc = 0x82DED3C0; continue 'dispatch;
	}
	// 82DED2DC: 54A4103A  slwi r4, r5, 2
	ctx.r[4].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82DED2E0: 806605B0  lwz r3, 0x5b0(r6)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DED2E4: 4BFB829D  bl 0x82da5580
	ctx.lr = 0x82DED2E8;
	sub_82DA5580(ctx, base);
	// 82DED2E8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DED2EC: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DED2F0: 906B553C  stw r3, 0x553c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(21820 as u32), ctx.r[3].u32 ) };
	// 82DED2F4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED2F8: 83AB05B0  lwz r29, 0x5b0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DED2FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DED300: 4BFB8281  bl 0x82da5580
	ctx.lr = 0x82DED304;
	sub_82DA5580(ctx, base);
	// 82DED304: 37C30004  addic. r30, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DED308: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DED30C: 41820018  beq 0x82ded324
	if ctx.cr[0].eq {
	pc = 0x82DED324; continue 'dispatch;
	}
	// 82DED310: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED314: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DED318: 808B05B0  lwz r4, 0x5b0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DED31C: 4BFD1B3D  bl 0x82dbee58
	ctx.lr = 0x82DED320;
	sub_82DBEE58(ctx, base);
	// 82DED320: 48000008  b 0x82ded328
	pc = 0x82DED328; continue 'dispatch;
	// 82DED324: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DED328: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DED32C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DED330: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82DED334: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DED338: 40990064  ble cr6, 0x82ded39c
	if !ctx.cr[6].gt {
	pc = 0x82DED39C; continue 'dispatch;
	}
	// 82DED33C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED340: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 82DED344: 83CB05B0  lwz r30, 0x5b0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DED348: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DED34C: 4BFB8235  bl 0x82da5580
	ctx.lr = 0x82DED350;
	sub_82DA5580(ctx, base);
	// 82DED350: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DED354: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DED358: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DED35C: 4182001C  beq 0x82ded378
	if ctx.cr[0].eq {
	pc = 0x82DED378; continue 'dispatch;
	}
	// 82DED360: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DED364: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED368: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DED36C: 4BFFFD45  bl 0x82ded0b0
	ctx.lr = 0x82DED370;
	sub_82DED0B0(ctx, base);
	// 82DED370: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DED374: 48000008  b 0x82ded37c
	pc = 0x82DED37C; continue 'dispatch;
	// 82DED378: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DED37C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED380: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED384: 4BFEF495  bl 0x82ddc818
	ctx.lr = 0x82DED388;
	sub_82DDC818(ctx, base);
	// 82DED388: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DED38C: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DED390: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DED394: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DED398: 4198FFA4  blt cr6, 0x82ded33c
	if ctx.cr[6].lt {
	pc = 0x82DED33C; continue 'dispatch;
	}
	// 82DED39C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DED3A0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED3A4: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 82DED3A8: 7D6959D6  mullw r11, r9, r11
	ctx.r[11].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82DED3AC: 808A05B0  lwz r4, 0x5b0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DED3B0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82DED3B4: 7C6B0194  addze r3, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[3].s64 = tmp.s64;
	// 82DED3B8: 4BFD2771  bl 0x82dbfb28
	ctx.lr = 0x82DED3BC;
	sub_82DBFB28(ctx, base);
	// 82DED3BC: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DED3C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DED3C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DED3C8: 483BADF4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DED3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DED3D0 size=136
    let mut pc: u32 = 0x82DED3D0;
    'dispatch: loop {
        match pc {
            0x82DED3D0 => {
    //   block [0x82DED3D0..0x82DED458)
	// 82DED3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DED3D4: 483BAD99  bl 0x831a816c
	ctx.lr = 0x82DED3D8;
	sub_831A8130(ctx, base);
	// 82DED3D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DED3DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DED3E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DED3E4: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED3E8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED3EC: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DED3F0: 40980014  bge cr6, 0x82ded404
	if !ctx.cr[6].lt {
	pc = 0x82DED404; continue 'dispatch;
	}
	// 82DED3F4: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED3F8: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DED3FC: 7FAA5A14  add r29, r10, r11
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DED400: 48000010  b 0x82ded410
	pc = 0x82DED410; continue 'dispatch;
	// 82DED404: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DED408: 4BFEF411  bl 0x82ddc818
	ctx.lr = 0x82DED40C;
	sub_82DDC818(ctx, base);
	// 82DED40C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DED410: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED414: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED418: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DED41C: 40980014  bge cr6, 0x82ded430
	if !ctx.cr[6].lt {
	pc = 0x82DED430; continue 'dispatch;
	}
	// 82DED420: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED424: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DED428: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DED42C: 4800000C  b 0x82ded438
	pc = 0x82DED438; continue 'dispatch;
	// 82DED430: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DED434: 4BFEF3E5  bl 0x82ddc818
	ctx.lr = 0x82DED438;
	sub_82DDC818(ctx, base);
	// 82DED438: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED43C: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DED440: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED444: 4BFEF3D5  bl 0x82ddc818
	ctx.lr = 0x82DED448;
	sub_82DDC818(ctx, base);
	// 82DED448: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED44C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DED450: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DED454: 483BAD68  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DED458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DED458 size=368
    let mut pc: u32 = 0x82DED458;
    'dispatch: loop {
        match pc {
            0x82DED458 => {
    //   block [0x82DED458..0x82DED5C8)
	// 82DED458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DED45C: 483BAD0D  bl 0x831a8168
	ctx.lr = 0x82DED460;
	sub_831A8130(ctx, base);
	// 82DED460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DED464: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DED468: 839F0010  lwz r28, 0x10(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DED46C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED470: 397C0001  addi r11, r28, 1
	ctx.r[11].s64 = ctx.r[28].s64 + 1;
	// 82DED474: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED478: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DED47C: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 82DED480: 7D6959D6  mullw r11, r9, r11
	ctx.r[11].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82DED484: 808A05B0  lwz r4, 0x5b0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DED488: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82DED48C: 7FCB0194  addze r30, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[30].s64 = tmp.s64;
	// 82DED490: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DED494: 4BFD2695  bl 0x82dbfb28
	ctx.lr = 0x82DED498;
	sub_82DBFB28(ctx, base);
	// 82DED498: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DED49C: 7D4BF050  subf r10, r11, r30
	ctx.r[10].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 82DED4A0: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DED4A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DED4A8: 354A0001  addic. r10, r10, 1
	ctx.xer.ca = (ctx.r[10].u32 > (!(1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DED4AC: 40810054  ble 0x82ded500
	if !ctx.cr[0].gt {
	pc = 0x82DED500; continue 'dispatch;
	}
	// 82DED4B0: 556AD97E  srwi r10, r11, 5
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DED4B4: 556906FE  clrlwi r9, r11, 0x1b
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DED4B8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82DED4BC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DED4C0: 7D0AE82E  lwzx r8, r10, r29
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DED4C4: 7D084C30  srw r8, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[8].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82DED4C8: 550807FF  clrlwi. r8, r8, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DED4CC: 4182001C  beq 0x82ded4e8
	if ctx.cr[0].eq {
	pc = 0x82DED4E8; continue 'dispatch;
	}
	// 82DED4D0: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED4D4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DED4D8: 7CE94830  slw r9, r7, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[7].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82DED4DC: 7CEA402E  lwzx r7, r10, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DED4E0: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82DED4E4: 7D2A412E  stwx r9, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 82DED4E8: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DED4EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DED4F0: 7D4AF050  subf r10, r10, r30
	ctx.r[10].s64 = ctx.r[30].s64 - ctx.r[10].s64;
	// 82DED4F4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DED4F8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DED4FC: 4198FFB4  blt cr6, 0x82ded4b0
	if ctx.cr[6].lt {
	pc = 0x82DED4B0; continue 'dispatch;
	}
	// 82DED500: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82DED504: 419A0010  beq cr6, 0x82ded514
	if ctx.cr[6].eq {
	pc = 0x82DED514; continue 'dispatch;
	}
	// 82DED508: 389DFFFC  addi r4, r29, -4
	ctx.r[4].s64 = ctx.r[29].s64 + -4;
	// 82DED50C: 807DFFFC  lwz r3, -4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DED510: 4BFB81F9  bl 0x82da5708
	ctx.lr = 0x82DED514;
	sub_82DA5708(ctx, base);
	// 82DED514: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED518: 3FC08338  lis r30, -0x7cc8
	ctx.r[30].s64 = -2093481984;
	// 82DED51C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DED520: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82DED524: 83BE553C  lwz r29, 0x553c(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(21820 as u32) ) } as u64;
	// 82DED528: 806A05B0  lwz r3, 0x5b0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DED52C: 4BFB8055  bl 0x82da5580
	ctx.lr = 0x82DED530;
	sub_82DA5580(ctx, base);
	// 82DED530: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82DED534: 907E553C  stw r3, 0x553c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(21820 as u32), ctx.r[3].u32 ) };
	// 82DED538: 40990028  ble cr6, 0x82ded560
	if !ctx.cr[6].gt {
	pc = 0x82DED560; continue 'dispatch;
	}
	// 82DED53C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DED540: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82DED544: 48000008  b 0x82ded54c
	pc = 0x82DED54C; continue 'dispatch;
	// 82DED548: 807E553C  lwz r3, 0x553c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(21820 as u32) ) } as u64;
	// 82DED54C: 7D2BE82E  lwzx r9, r11, r29
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DED550: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DED554: 7D2B192E  stwx r9, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 82DED558: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DED55C: 4082FFEC  bne 0x82ded548
	if !ctx.cr[0].eq {
	pc = 0x82DED548; continue 'dispatch;
	}
	// 82DED560: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED564: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DED568: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DED56C: 4BFB819D  bl 0x82da5708
	ctx.lr = 0x82DED570;
	sub_82DA5708(ctx, base);
	// 82DED570: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED574: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 82DED578: 83CB05B0  lwz r30, 0x5b0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DED57C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DED580: 4BFB8001  bl 0x82da5580
	ctx.lr = 0x82DED584;
	sub_82DA5580(ctx, base);
	// 82DED584: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DED588: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DED58C: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DED590: 4182001C  beq 0x82ded5ac
	if ctx.cr[0].eq {
	pc = 0x82DED5AC; continue 'dispatch;
	}
	// 82DED594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DED598: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED59C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DED5A0: 4BFFFB11  bl 0x82ded0b0
	ctx.lr = 0x82DED5A4;
	sub_82DED0B0(ctx, base);
	// 82DED5A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DED5A8: 48000008  b 0x82ded5b0
	pc = 0x82DED5B0; continue 'dispatch;
	// 82DED5AC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DED5B0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED5B4: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED5B8: 4BFEF261  bl 0x82ddc818
	ctx.lr = 0x82DED5BC;
	sub_82DDC818(ctx, base);
	// 82DED5BC: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DED5C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DED5C4: 483BABF4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DED5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DED5C8 size=332
    let mut pc: u32 = 0x82DED5C8;
    'dispatch: loop {
        match pc {
            0x82DED5C8 => {
    //   block [0x82DED5C8..0x82DED714)
	// 82DED5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DED5CC: 483BABA1  bl 0x831a816c
	ctx.lr = 0x82DED5D0;
	sub_831A8130(ctx, base);
	// 82DED5D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DED5D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DED5D8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DED5DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DED5E0: 7F1EF800  cmpw cr6, r30, r31
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DED5E4: 419A0128  beq cr6, 0x82ded70c
	if ctx.cr[6].eq {
	pc = 0x82DED70C; continue 'dispatch;
	}
	// 82DED5E8: 4BFFF9F9  bl 0x82decfe0
	ctx.lr = 0x82DED5EC;
	sub_82DECFE0(ctx, base);
	// 82DED5EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DED5F0: 4082011C  bne 0x82ded70c
	if !ctx.cr[0].eq {
	pc = 0x82DED70C; continue 'dispatch;
	}
	// 82DED5F4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DED5F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DED5FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DED600: 4BFFFA31  bl 0x82ded030
	ctx.lr = 0x82DED604;
	sub_82DED030(ctx, base);
	// 82DED604: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED608: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED60C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DED610: 40980014  bge cr6, 0x82ded624
	if !ctx.cr[6].lt {
	pc = 0x82DED624; continue 'dispatch;
	}
	// 82DED614: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED618: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DED61C: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DED620: 4800000C  b 0x82ded62c
	pc = 0x82DED62C; continue 'dispatch;
	// 82DED624: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DED628: 4BFEF1F1  bl 0x82ddc818
	ctx.lr = 0x82DED62C;
	sub_82DDC818(ctx, base);
	// 82DED62C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82DED630: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED634: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DED638: 4BFFF721  bl 0x82decd58
	ctx.lr = 0x82DED63C;
	sub_82DECD58(ctx, base);
	// 82DED63C: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED640: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED644: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DED648: 40980014  bge cr6, 0x82ded65c
	if !ctx.cr[6].lt {
	pc = 0x82DED65C; continue 'dispatch;
	}
	// 82DED64C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED650: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DED654: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DED658: 4800000C  b 0x82ded664
	pc = 0x82DED664; continue 'dispatch;
	// 82DED65C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DED660: 4BFEF1B9  bl 0x82ddc818
	ctx.lr = 0x82DED664;
	sub_82DDC818(ctx, base);
	// 82DED664: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82DED668: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED66C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DED670: 80DD0008  lwz r6, 8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED674: 4BFFF875  bl 0x82decee8
	ctx.lr = 0x82DED678;
	sub_82DECEE8(ctx, base);
	// 82DED678: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DED67C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DED680: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DED684: 7FEB512E  stwx r31, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 82DED688: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED68C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED690: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DED694: 40980014  bge cr6, 0x82ded6a8
	if !ctx.cr[6].lt {
	pc = 0x82DED6A8; continue 'dispatch;
	}
	// 82DED698: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED69C: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DED6A0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DED6A4: 4800000C  b 0x82ded6b0
	pc = 0x82DED6B0; continue 'dispatch;
	// 82DED6A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DED6AC: 4BFEF16D  bl 0x82ddc818
	ctx.lr = 0x82DED6B0;
	sub_82DDC818(ctx, base);
	// 82DED6B0: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82DED6B4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED6B8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DED6BC: 4BFFF69D  bl 0x82decd58
	ctx.lr = 0x82DED6C0;
	sub_82DECD58(ctx, base);
	// 82DED6C0: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED6C4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED6C8: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DED6CC: 40980014  bge cr6, 0x82ded6e0
	if !ctx.cr[6].lt {
	pc = 0x82DED6E0; continue 'dispatch;
	}
	// 82DED6D0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED6D4: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DED6D8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DED6DC: 4800000C  b 0x82ded6e8
	pc = 0x82DED6E8; continue 'dispatch;
	// 82DED6E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DED6E4: 4BFEF135  bl 0x82ddc818
	ctx.lr = 0x82DED6E8;
	sub_82DDC818(ctx, base);
	// 82DED6E8: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82DED6EC: 80DD0008  lwz r6, 8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED6F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DED6F4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED6F8: 4BFFF7F1  bl 0x82decee8
	ctx.lr = 0x82DED6FC;
	sub_82DECEE8(ctx, base);
	// 82DED6FC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DED700: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DED704: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DED708: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82DED70C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DED710: 483BAAAC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DED718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DED718 size=112
    let mut pc: u32 = 0x82DED718;
    'dispatch: loop {
        match pc {
            0x82DED718 => {
    //   block [0x82DED718..0x82DED788)
	// 82DED718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DED71C: 483BAA51  bl 0x831a816c
	ctx.lr = 0x82DED720;
	sub_831A8130(ctx, base);
	// 82DED720: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DED724: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DED728: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DED72C: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DED730: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DED734: 4099004C  ble cr6, 0x82ded780
	if !ctx.cr[6].gt {
	pc = 0x82DED780; continue 'dispatch;
	}
	// 82DED738: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DED73C: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED740: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED744: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DED748: 40980010  bge cr6, 0x82ded758
	if !ctx.cr[6].lt {
	pc = 0x82DED758; continue 'dispatch;
	}
	// 82DED74C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED750: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DED754: 4800000C  b 0x82ded760
	pc = 0x82DED760; continue 'dispatch;
	// 82DED758: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DED75C: 4BFEF0BD  bl 0x82ddc818
	ctx.lr = 0x82DED760;
	sub_82DDC818(ctx, base);
	// 82DED760: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED764: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DED768: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DED76C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DED770: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DED774: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DED778: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DED77C: 4198FFC0  blt cr6, 0x82ded73c
	if ctx.cr[6].lt {
	pc = 0x82DED73C; continue 'dispatch;
	}
	// 82DED780: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DED784: 483BAA38  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DED788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DED788 size=180
    let mut pc: u32 = 0x82DED788;
    'dispatch: loop {
        match pc {
            0x82DED788 => {
    //   block [0x82DED788..0x82DED83C)
	// 82DED788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DED78C: 483BA9DD  bl 0x831a8168
	ctx.lr = 0x82DED790;
	sub_831A8130(ctx, base);
	// 82DED790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DED794: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DED798: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DED79C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DED7A0: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DED7A4: 409A000C  bne cr6, 0x82ded7b0
	if !ctx.cr[6].eq {
	pc = 0x82DED7B0; continue 'dispatch;
	}
	// 82DED7A8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DED7AC: 48000088  b 0x82ded834
	pc = 0x82DED834; continue 'dispatch;
	// 82DED7B0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DED7B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DED7B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DED7BC: 4BFFF825  bl 0x82decfe0
	ctx.lr = 0x82DED7C0;
	sub_82DECFE0(ctx, base);
	// 82DED7C0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DED7C4: 4082006C  bne 0x82ded830
	if !ctx.cr[0].eq {
	pc = 0x82DED830; continue 'dispatch;
	}
	// 82DED7C8: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82DED7CC: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82DED7D0: 41980010  blt cr6, 0x82ded7e0
	if ctx.cr[6].lt {
	pc = 0x82DED7E0; continue 'dispatch;
	}
	// 82DED7D4: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82DED7D8: 40990008  ble cr6, 0x82ded7e0
	if !ctx.cr[6].gt {
	pc = 0x82DED7E0; continue 'dispatch;
	}
	// 82DED7DC: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82DED7E0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DED7E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DED7E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DED7EC: 80AB553C  lwz r5, 0x553c(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21820 as u32) ) } as u64;
	// 82DED7F0: 48002389  bl 0x82defb78
	ctx.lr = 0x82DED7F4;
	sub_82DEFB78(ctx, base);
	// 82DED7F4: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED7F8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED7FC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DED800: 40980014  bge cr6, 0x82ded814
	if !ctx.cr[6].lt {
	pc = 0x82DED814; continue 'dispatch;
	}
	// 82DED804: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED808: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DED80C: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DED810: 4800000C  b 0x82ded81c
	pc = 0x82DED81C; continue 'dispatch;
	// 82DED814: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DED818: 4BFEF001  bl 0x82ddc818
	ctx.lr = 0x82DED81C;
	sub_82DDC818(ctx, base);
	// 82DED81C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DED820: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED824: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DED828: 4BFFF909  bl 0x82ded130
	ctx.lr = 0x82DED82C;
	sub_82DED130(ctx, base);
	// 82DED82C: 4BFFFF7C  b 0x82ded7a8
	pc = 0x82DED7A8; continue 'dispatch;
	// 82DED830: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DED834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DED838: 483BA980  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DED840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DED840 size=328
    let mut pc: u32 = 0x82DED840;
    'dispatch: loop {
        match pc {
            0x82DED840 => {
    //   block [0x82DED840..0x82DED988)
	// 82DED840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DED844: 483BA915  bl 0x831a8158
	ctx.lr = 0x82DED848;
	sub_831A8130(ctx, base);
	// 82DED848: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DED84C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DED850: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82DED854: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82DED858: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED85C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED860: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DED864: 40980014  bge cr6, 0x82ded878
	if !ctx.cr[6].lt {
	pc = 0x82DED878; continue 'dispatch;
	}
	// 82DED868: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED86C: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DED870: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DED874: 48000008  b 0x82ded87c
	pc = 0x82DED87C; continue 'dispatch;
	// 82DED878: 4BFEEFA1  bl 0x82ddc818
	ctx.lr = 0x82DED87C;
	sub_82DDC818(ctx, base);
	// 82DED87C: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED880: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 82DED884: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82DED888: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED88C: 83BB0008  lwz r29, 8(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED890: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DED894: 409900EC  ble cr6, 0x82ded980
	if !ctx.cr[6].gt {
	pc = 0x82DED980; continue 'dispatch;
	}
	// 82DED898: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DED89C: 2F1E0007  cmpwi cr6, r30, 7
	ctx.cr[6].compare_i32(ctx.r[30].s32, 7, &mut ctx.xer);
	// 82DED8A0: 409A000C  bne cr6, 0x82ded8ac
	if !ctx.cr[6].eq {
	pc = 0x82DED8AC; continue 'dispatch;
	}
	// 82DED8A4: 83BD001C  lwz r29, 0x1c(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DED8A8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DED8AC: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DED8B0: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED8B4: 7FEBE82E  lwzx r31, r11, r29
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DED8B8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED8BC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DED8C0: 40980014  bge cr6, 0x82ded8d4
	if !ctx.cr[6].lt {
	pc = 0x82DED8D4; continue 'dispatch;
	}
	// 82DED8C4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED8C8: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DED8CC: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DED8D0: 4800000C  b 0x82ded8dc
	pc = 0x82DED8DC; continue 'dispatch;
	// 82DED8D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DED8D8: 4BFEEF41  bl 0x82ddc818
	ctx.lr = 0x82DED8DC;
	sub_82DDC818(ctx, base);
	// 82DED8DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED8E0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DED8E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DED8E8: 40990088  ble cr6, 0x82ded970
	if !ctx.cr[6].gt {
	pc = 0x82DED970; continue 'dispatch;
	}
	// 82DED8EC: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED8F0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED8F4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DED8F8: 40980014  bge cr6, 0x82ded90c
	if !ctx.cr[6].lt {
	pc = 0x82DED90C; continue 'dispatch;
	}
	// 82DED8FC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED900: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DED904: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DED908: 4800000C  b 0x82ded914
	pc = 0x82DED914; continue 'dispatch;
	// 82DED90C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DED910: 4BFEEF09  bl 0x82ddc818
	ctx.lr = 0x82DED914;
	sub_82DDC818(ctx, base);
	// 82DED914: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED918: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DED91C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DED920: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DED924: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED928: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED92C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DED930: 40980014  bge cr6, 0x82ded944
	if !ctx.cr[6].lt {
	pc = 0x82DED944; continue 'dispatch;
	}
	// 82DED934: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED938: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DED93C: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DED940: 4800000C  b 0x82ded94c
	pc = 0x82DED94C; continue 'dispatch;
	// 82DED944: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DED948: 4BFEEED1  bl 0x82ddc818
	ctx.lr = 0x82DED94C;
	sub_82DDC818(ctx, base);
	// 82DED94C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DED950: 3958FFFF  addi r10, r24, -1
	ctx.r[10].s64 = ctx.r[24].s64 + -1;
	// 82DED954: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DED958: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DED95C: 409A0014  bne cr6, 0x82ded970
	if !ctx.cr[6].eq {
	pc = 0x82DED970; continue 'dispatch;
	}
	// 82DED960: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DED964: 80990004  lwz r4, 4(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED968: 4BFEEEB1  bl 0x82ddc818
	ctx.lr = 0x82DED96C;
	sub_82DDC818(ctx, base);
	// 82DED96C: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DED970: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED974: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82DED978: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DED97C: 4198FF1C  blt cr6, 0x82ded898
	if ctx.cr[6].lt {
	pc = 0x82DED898; continue 'dispatch;
	}
	// 82DED980: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DED984: 483BA824  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DED988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DED988 size=108
    let mut pc: u32 = 0x82DED988;
    'dispatch: loop {
        match pc {
            0x82DED988 => {
    //   block [0x82DED988..0x82DED9F4)
	// 82DED988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DED98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DED990: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DED994: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DED998: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DED99C: 48000024  b 0x82ded9c0
	pc = 0x82DED9C0; continue 'dispatch;
	// 82DED9A0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED9A4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DED9A8: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DED9AC: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DED9B0: 41820010  beq 0x82ded9c0
	if ctx.cr[0].eq {
	pc = 0x82DED9C0; continue 'dispatch;
	}
	// 82DED9B4: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 82DED9B8: 806BFFFC  lwz r3, -4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DED9BC: 4BFB7D4D  bl 0x82da5708
	ctx.lr = 0x82DED9C0;
	sub_82DA5708(ctx, base);
	// 82DED9C0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DED9C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DED9C8: 409AFFD8  bne cr6, 0x82ded9a0
	if !ctx.cr[6].eq {
	pc = 0x82DED9A0; continue 'dispatch;
	}
	// 82DED9CC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DED9D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DED9D4: 419A000C  beq cr6, 0x82ded9e0
	if ctx.cr[6].eq {
	pc = 0x82DED9E0; continue 'dispatch;
	}
	// 82DED9D8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DED9DC: 4BFE4B5D  bl 0x82dd2538
	ctx.lr = 0x82DED9E0;
	sub_82DD2538(ctx, base);
	// 82DED9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DED9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DED9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DED9EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DED9F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DED9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DED9F8 size=1712
    let mut pc: u32 = 0x82DED9F8;
    'dispatch: loop {
        match pc {
            0x82DED9F8 => {
    //   block [0x82DED9F8..0x82DEE0A8)
	// 82DED9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DED9FC: 483BA739  bl 0x831a8134
	ctx.lr = 0x82DEDA00;
	sub_831A8130(ctx, base);
	// 82DEDA00: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEDA04: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82DEDA08: 39E00001  li r15, 1
	ctx.r[15].s64 = 1;
	// 82DEDA0C: 8179000C  lwz r11, 0xc(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DEDA10: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEDA14: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 82DEDA18: 41990008  bgt cr6, 0x82deda20
	if ctx.cr[6].gt {
	pc = 0x82DEDA20; continue 'dispatch;
	}
	// 82DEDA1C: 7DFA7B78  mr r26, r15
	ctx.r[26].u64 = ctx.r[15].u64;
	// 82DEDA20: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEDA24: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DEDA28: 83CB05B0  lwz r30, 0x5b0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DEDA2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DEDA30: 4BFB7B51  bl 0x82da5580
	ctx.lr = 0x82DEDA34;
	sub_82DA5580(ctx, base);
	// 82DEDA34: 3A400000  li r18, 0
	ctx.r[18].s64 = 0;
	// 82DEDA38: 37E30004  addic. r31, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DEDA3C: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DEDA40: 4182001C  beq 0x82deda5c
	if ctx.cr[0].eq {
	pc = 0x82DEDA5C; continue 'dispatch;
	}
	// 82DEDA44: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEDA48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEDA4C: 808B05B0  lwz r4, 0x5b0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DEDA50: 4BFD1409  bl 0x82dbee58
	ctx.lr = 0x82DEDA54;
	sub_82DBEE58(ctx, base);
	// 82DEDA54: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 82DEDA58: 48000008  b 0x82deda60
	pc = 0x82DEDA60; continue 'dispatch;
	// 82DEDA5C: 7E5B9378  mr r27, r18
	ctx.r[27].u64 = ctx.r[18].u64;
	// 82DEDA60: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEDA64: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DEDA68: 83CB05B0  lwz r30, 0x5b0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DEDA6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DEDA70: 4BFB7B11  bl 0x82da5580
	ctx.lr = 0x82DEDA74;
	sub_82DA5580(ctx, base);
	// 82DEDA74: 37E30004  addic. r31, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DEDA78: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DEDA7C: 4182001C  beq 0x82deda98
	if ctx.cr[0].eq {
	pc = 0x82DEDA98; continue 'dispatch;
	}
	// 82DEDA80: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEDA84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEDA88: 808B05B0  lwz r4, 0x5b0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DEDA8C: 4BFD13CD  bl 0x82dbee58
	ctx.lr = 0x82DEDA90;
	sub_82DBEE58(ctx, base);
	// 82DEDA90: 7FF3FB78  mr r19, r31
	ctx.r[19].u64 = ctx.r[31].u64;
	// 82DEDA94: 48000008  b 0x82deda9c
	pc = 0x82DEDA9C; continue 'dispatch;
	// 82DEDA98: 7E539378  mr r19, r18
	ctx.r[19].u64 = ctx.r[18].u64;
	// 82DEDA9C: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEDAA0: 7E5C9378  mr r28, r18
	ctx.r[28].u64 = ctx.r[18].u64;
	// 82DEDAA4: 81590010  lwz r10, 0x10(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEDAA8: 5544103A  slwi r4, r10, 2
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82DEDAAC: 806B05AC  lwz r3, 0x5ac(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DEDAB0: 4BFB7AD1  bl 0x82da5580
	ctx.lr = 0x82DEDAB4;
	sub_82DA5580(ctx, base);
	// 82DEDAB4: 3E208338  lis r17, -0x7cc8
	ctx.r[17].s64 = -2093481984;
	// 82DEDAB8: 7E4B9378  mr r11, r18
	ctx.r[11].u64 = ctx.r[18].u64;
	// 82DEDABC: 90715538  stw r3, 0x5538(r17)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[17].u32.wrapping_add(21816 as u32), ctx.r[3].u32 ) };
	// 82DEDAC0: 81590010  lwz r10, 0x10(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEDAC4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DEDAC8: 4099003C  ble cr6, 0x82dedb04
	if !ctx.cr[6].gt {
	pc = 0x82DEDB04; continue 'dispatch;
	}
	// 82DEDACC: 7E4A9378  mr r10, r18
	ctx.r[10].u64 = ctx.r[18].u64;
	// 82DEDAD0: 48000008  b 0x82dedad8
	pc = 0x82DEDAD8; continue 'dispatch;
	// 82DEDAD4: 80715538  lwz r3, 0x5538(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(21816 as u32) ) } as u64;
	// 82DEDAD8: 8139000C  lwz r9, 0xc(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DEDADC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DEDAE0: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82DEDAE4: 41980008  blt cr6, 0x82dedaec
	if ctx.cr[6].lt {
	pc = 0x82DEDAEC; continue 'dispatch;
	}
	// 82DEDAE8: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82DEDAEC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DEDAF0: 7D2A192E  stwx r9, r10, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 82DEDAF4: 81390010  lwz r9, 0x10(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEDAF8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DEDAFC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DEDB00: 4198FFD4  blt cr6, 0x82dedad4
	if ctx.cr[6].lt {
	pc = 0x82DEDAD4; continue 'dispatch;
	}
	// 82DEDB04: 83F9000C  lwz r31, 0xc(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DEDB08: 3E008338  lis r16, -0x7cc8
	ctx.r[16].s64 = -2093481984;
	// 82DEDB0C: 81790010  lwz r11, 0x10(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEDB10: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DEDB14: 40980110  bge cr6, 0x82dedc24
	if !ctx.cr[6].lt {
	pc = 0x82DEDC24; continue 'dispatch;
	}
	// 82DEDB18: 57FD103A  slwi r29, r31, 2
	ctx.r[29].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82DEDB1C: 8170553C  lwz r11, 0x553c(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(21820 as u32) ) } as u64;
	// 82DEDB20: 7D7D582E  lwzx r11, r29, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DEDB24: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DEDB28: 409A00E8  bne cr6, 0x82dedc10
	if !ctx.cr[6].eq {
	pc = 0x82DEDC10; continue 'dispatch;
	}
	// 82DEDB2C: 80790004  lwz r3, 4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDB30: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82DEDB34: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDB38: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEDB3C: 40980010  bge cr6, 0x82dedb4c
	if !ctx.cr[6].lt {
	pc = 0x82DEDB4C; continue 'dispatch;
	}
	// 82DEDB40: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEDB44: 7FCBEA14  add r30, r11, r29
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DEDB48: 48000010  b 0x82dedb58
	pc = 0x82DEDB58; continue 'dispatch;
	// 82DEDB4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DEDB50: 4BFEECC9  bl 0x82ddc818
	ctx.lr = 0x82DEDB54;
	sub_82DDC818(ctx, base);
	// 82DEDB54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DEDB58: 80790004  lwz r3, 4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDB5C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDB60: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEDB64: 40980010  bge cr6, 0x82dedb74
	if !ctx.cr[6].lt {
	pc = 0x82DEDB74; continue 'dispatch;
	}
	// 82DEDB68: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEDB6C: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DEDB70: 4800000C  b 0x82dedb7c
	pc = 0x82DEDB7C; continue 'dispatch;
	// 82DEDB74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DEDB78: 4BFEECA1  bl 0x82ddc818
	ctx.lr = 0x82DEDB7C;
	sub_82DDC818(ctx, base);
	// 82DEDB7C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEDB80: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEDB84: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDB88: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DEDB8C: 80790004  lwz r3, 4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDB90: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDB94: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEDB98: 40980010  bge cr6, 0x82dedba8
	if !ctx.cr[6].lt {
	pc = 0x82DEDBA8; continue 'dispatch;
	}
	// 82DEDB9C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEDBA0: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DEDBA4: 4800000C  b 0x82dedbb0
	pc = 0x82DEDBB0; continue 'dispatch;
	// 82DEDBA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DEDBAC: 4BFEEC6D  bl 0x82ddc818
	ctx.lr = 0x82DEDBB0;
	sub_82DDC818(ctx, base);
	// 82DEDBB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEDBB4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DEDBB8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEDBBC: 409A0010  bne cr6, 0x82dedbcc
	if !ctx.cr[6].eq {
	pc = 0x82DEDBCC; continue 'dispatch;
	}
	// 82DEDBC0: 80930004  lwz r4, 4(r19)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDBC4: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82DEDBC8: 48000040  b 0x82dedc08
	pc = 0x82DEDC08; continue 'dispatch;
	// 82DEDBCC: 80790004  lwz r3, 4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDBD0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDBD4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEDBD8: 40980010  bge cr6, 0x82dedbe8
	if !ctx.cr[6].lt {
	pc = 0x82DEDBE8; continue 'dispatch;
	}
	// 82DEDBDC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEDBE0: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DEDBE4: 4800000C  b 0x82dedbf0
	pc = 0x82DEDBF0; continue 'dispatch;
	// 82DEDBE8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DEDBEC: 4BFEEC2D  bl 0x82ddc818
	ctx.lr = 0x82DEDBF0;
	sub_82DDC818(ctx, base);
	// 82DEDBF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEDBF4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DEDBF8: 7F0BD000  cmpw cr6, r11, r26
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82DEDBFC: 40980014  bge cr6, 0x82dedc10
	if !ctx.cr[6].lt {
	pc = 0x82DEDC10; continue 'dispatch;
	}
	// 82DEDC00: 809B0004  lwz r4, 4(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDC04: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DEDC08: 4BFEEC11  bl 0x82ddc818
	ctx.lr = 0x82DEDC0C;
	sub_82DDC818(ctx, base);
	// 82DEDC0C: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DEDC10: 81790010  lwz r11, 0x10(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEDC14: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DEDC18: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82DEDC1C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DEDC20: 4198FEFC  blt cr6, 0x82dedb1c
	if ctx.cr[6].lt {
	pc = 0x82DEDB1C; continue 'dispatch;
	}
	// 82DEDC24: 81730004  lwz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDC28: 7E549378  mr r20, r18
	ctx.r[20].u64 = ctx.r[18].u64;
	// 82DEDC2C: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82DEDC30: 419A0118  beq cr6, 0x82dedd48
	if ctx.cr[6].eq {
	pc = 0x82DEDD48; continue 'dispatch;
	}
	// 82DEDC34: 3BBAFFFF  addi r29, r26, -1
	ctx.r[29].s64 = ctx.r[26].s64 + -1;
	// 82DEDC38: 3A940001  addi r20, r20, 1
	ctx.r[20].s64 = ctx.r[20].s64 + 1;
	// 82DEDC3C: 48000064  b 0x82dedca0
	pc = 0x82DEDCA0; continue 'dispatch;
	// 82DEDC40: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DEDC44: 4BFD87DD  bl 0x82dc6420
	ctx.lr = 0x82DEDC48;
	sub_82DC6420(ctx, base);
	// 82DEDC48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DEDC4C: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82DEDC50: 80930004  lwz r4, 4(r19)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDC54: 4BFEEBC5  bl 0x82ddc818
	ctx.lr = 0x82DEDC58;
	sub_82DDC818(ctx, base);
	// 82DEDC58: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DEDC5C: 80790004  lwz r3, 4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDC60: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDC64: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEDC68: 40980014  bge cr6, 0x82dedc7c
	if !ctx.cr[6].lt {
	pc = 0x82DEDC7C; continue 'dispatch;
	}
	// 82DEDC6C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEDC70: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEDC74: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DEDC78: 4800000C  b 0x82dedc84
	pc = 0x82DEDC84; continue 'dispatch;
	// 82DEDC7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DEDC80: 4BFEEB99  bl 0x82ddc818
	ctx.lr = 0x82DEDC84;
	sub_82DDC818(ctx, base);
	// 82DEDC84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEDC88: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82DEDC8C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DEDC90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DEDC94: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DEDC98: 924B000C  stw r18, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[18].u32 ) };
	// 82DEDC9C: 4BFFFBA5  bl 0x82ded840
	ctx.lr = 0x82DEDCA0;
	sub_82DED840(ctx, base);
	// 82DEDCA0: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDCA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEDCA8: 409AFF98  bne cr6, 0x82dedc40
	if !ctx.cr[6].eq {
	pc = 0x82DEDC40; continue 'dispatch;
	}
	// 82DEDCAC: 81730004  lwz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDCB0: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82DEDCB4: 419A0094  beq cr6, 0x82dedd48
	if ctx.cr[6].eq {
	pc = 0x82DEDD48; continue 'dispatch;
	}
	// 82DEDCB8: 83F9000C  lwz r31, 0xc(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DEDCBC: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82DEDCC0: 81790010  lwz r11, 0x10(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEDCC4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DEDCC8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DEDCCC: 40980070  bge cr6, 0x82dedd3c
	if !ctx.cr[6].lt {
	pc = 0x82DEDD3C; continue 'dispatch;
	}
	// 82DEDCD0: 57FE103A  slwi r30, r31, 2
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DEDCD4: 8170553C  lwz r11, 0x553c(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(21820 as u32) ) } as u64;
	// 82DEDCD8: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DEDCDC: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DEDCE0: 409A0048  bne cr6, 0x82dedd28
	if !ctx.cr[6].eq {
	pc = 0x82DEDD28; continue 'dispatch;
	}
	// 82DEDCE4: 80790004  lwz r3, 4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDCE8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDCEC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEDCF0: 40980010  bge cr6, 0x82dedd00
	if !ctx.cr[6].lt {
	pc = 0x82DEDD00; continue 'dispatch;
	}
	// 82DEDCF4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEDCF8: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DEDCFC: 4800000C  b 0x82dedd08
	pc = 0x82DEDD08; continue 'dispatch;
	// 82DEDD00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DEDD04: 4BFEEB15  bl 0x82ddc818
	ctx.lr = 0x82DEDD08;
	sub_82DDC818(ctx, base);
	// 82DEDD08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEDD0C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DEDD10: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DEDD14: 409A0014  bne cr6, 0x82dedd28
	if !ctx.cr[6].eq {
	pc = 0x82DEDD28; continue 'dispatch;
	}
	// 82DEDD18: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DEDD1C: 809B0004  lwz r4, 4(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDD20: 4BFEEAF9  bl 0x82ddc818
	ctx.lr = 0x82DEDD24;
	sub_82DDC818(ctx, base);
	// 82DEDD24: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DEDD28: 81790010  lwz r11, 0x10(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEDD2C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DEDD30: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DEDD34: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DEDD38: 4198FF9C  blt cr6, 0x82dedcd4
	if ctx.cr[6].lt {
	pc = 0x82DEDCD4; continue 'dispatch;
	}
	// 82DEDD3C: 81730004  lwz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDD40: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82DEDD44: 409AFEF4  bne cr6, 0x82dedc38
	if !ctx.cr[6].eq {
	pc = 0x82DEDC38; continue 'dispatch;
	}
	// 82DEDD48: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEDD4C: 7E569378  mr r22, r18
	ctx.r[22].u64 = ctx.r[18].u64;
	// 82DEDD50: 814B0550  lwz r10, 0x550(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1360 as u32) ) } as u64;
	// 82DEDD54: 808B05B0  lwz r4, 0x5b0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DEDD58: 82EB0AB0  lwz r23, 0xab0(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2736 as u32) ) } as u64;
	// 82DEDD5C: 82AA0004  lwz r21, 4(r10)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDD60: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82DEDD64: 4BFD1DC5  bl 0x82dbfb28
	ctx.lr = 0x82DEDD68;
	sub_82DBFB28(ctx, base);
	// 82DEDD68: 81730004  lwz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDD6C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DEDD70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEDD74: 419A0268  beq cr6, 0x82dedfdc
	if ctx.cr[6].eq {
	pc = 0x82DEDFDC; continue 'dispatch;
	}
	// 82DEDD78: 83010050  lwz r24, 0x50(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DEDD7C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEDD80: 7E4A9378  mr r10, r18
	ctx.r[10].u64 = ctx.r[18].u64;
	// 82DEDD84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEDD88: 40990020  ble cr6, 0x82dedda8
	if !ctx.cr[6].gt {
	pc = 0x82DEDDA8; continue 'dispatch;
	}
	// 82DEDD8C: 397A0008  addi r11, r26, 8
	ctx.r[11].s64 = ctx.r[26].s64 + 8;
	// 82DEDD90: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DEDD94: 924B0000  stw r18, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[18].u32 ) };
	// 82DEDD98: 813A0000  lwz r9, 0(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEDD9C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DEDDA0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DEDDA4: 4198FFEC  blt cr6, 0x82dedd90
	if ctx.cr[6].lt {
	pc = 0x82DEDD90; continue 'dispatch;
	}
	// 82DEDDA8: 7E5F9378  mr r31, r18
	ctx.r[31].u64 = ctx.r[18].u64;
	// 82DEDDAC: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 82DEDDB0: 40990044  ble cr6, 0x82deddf4
	if !ctx.cr[6].gt {
	pc = 0x82DEDDF4; continue 'dispatch;
	}
	// 82DEDDB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DEDDB8: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DEDDBC: 4BFE28CD  bl 0x82dd0688
	ctx.lr = 0x82DEDDC0;
	sub_82DD0688(ctx, base);
	// 82DEDDC0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEDDC4: 40820024  bne 0x82dedde8
	if !ctx.cr[0].eq {
	pc = 0x82DEDDE8; continue 'dispatch;
	}
	// 82DEDDC8: 57EBD97E  srwi r11, r31, 5
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shr(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEDDCC: 57EA06FE  clrlwi r10, r31, 0x1b
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x0000001Fu64;
	// 82DEDDD0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82DEDDD4: 7DEA5030  slw r10, r15, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[15].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82DEDDD8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEDDDC: 7D2BD02E  lwzx r9, r11, r26
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82DEDDE0: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82DEDDE4: 7D4BD12E  stwx r10, r11, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32), ctx.r[10].u32) };
	// 82DEDDE8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DEDDEC: 7F1FA800  cmpw cr6, r31, r21
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[21].s32, &mut ctx.xer);
	// 82DEDDF0: 4198FFC4  blt cr6, 0x82deddb4
	if ctx.cr[6].lt {
	pc = 0x82DEDDB4; continue 'dispatch;
	}
	// 82DEDDF4: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82DEDDF8: 4BFD8629  bl 0x82dc6420
	ctx.lr = 0x82DEDDFC;
	sub_82DC6420(ctx, base);
	// 82DEDDFC: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDE00: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DEDE04: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDE08: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DEDE0C: 40980014  bge cr6, 0x82dede20
	if !ctx.cr[6].lt {
	pc = 0x82DEDE20; continue 'dispatch;
	}
	// 82DEDE10: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEDE14: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEDE18: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DEDE1C: 48000010  b 0x82dede2c
	pc = 0x82DEDE2C; continue 'dispatch;
	// 82DEDE20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DEDE24: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DEDE28: 4BFEE9F1  bl 0x82ddc818
	ctx.lr = 0x82DEDE2C;
	sub_82DDC818(ctx, base);
	// 82DEDE2C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEDE30: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DEDE34: 7E469378  mr r6, r18
	ctx.r[6].u64 = ctx.r[18].u64;
	// 82DEDE38: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDE3C: 80E90008  lwz r7, 8(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEDE40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEDE44: 40990064  ble cr6, 0x82dedea8
	if !ctx.cr[6].gt {
	pc = 0x82DEDEA8; continue 'dispatch;
	}
	// 82DEDE48: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DEDE4C: 2F0A0007  cmpwi cr6, r10, 7
	ctx.cr[6].compare_i32(ctx.r[10].s32, 7, &mut ctx.xer);
	// 82DEDE50: 409A000C  bne cr6, 0x82dede5c
	if !ctx.cr[6].eq {
	pc = 0x82DEDE5C; continue 'dispatch;
	}
	// 82DEDE54: 80E7001C  lwz r7, 0x1c(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DEDE58: 7E4A9378  mr r10, r18
	ctx.r[10].u64 = ctx.r[18].u64;
	// 82DEDE5C: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEDE60: 81115538  lwz r8, 0x5538(r17)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(21816 as u32) ) } as u64;
	// 82DEDE64: 7D6B382E  lwzx r11, r11, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82DEDE68: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEDE6C: 7D6B402E  lwzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DEDE70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEDE74: 41980024  blt cr6, 0x82dede98
	if ctx.cr[6].lt {
	pc = 0x82DEDE98; continue 'dispatch;
	}
	// 82DEDE78: 5568D97E  srwi r8, r11, 5
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(5);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DEDE7C: 556B06FE  clrlwi r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DEDE80: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 82DEDE84: 7DE55830  slw r5, r15, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[5].u64 = 0;
	} else {
		ctx.r[5].u64 = ((ctx.r[15].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DEDE88: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEDE8C: 7D0BD02E  lwzx r8, r11, r26
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82DEDE90: 7CA84378  or r8, r5, r8
	ctx.r[8].u64 = ctx.r[5].u64 | ctx.r[8].u64;
	// 82DEDE94: 7D0BD12E  stwx r8, r11, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32), ctx.r[8].u32) };
	// 82DEDE98: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDE9C: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82DEDEA0: 7F065800  cmpw cr6, r6, r11
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DEDEA4: 4198FFA4  blt cr6, 0x82dede48
	if ctx.cr[6].lt {
	pc = 0x82DEDE48; continue 'dispatch;
	}
	// 82DEDEA8: 80790004  lwz r3, 4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDEAC: 7E5B9378  mr r27, r18
	ctx.r[27].u64 = ctx.r[18].u64;
	// 82DEDEB0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDEB4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEDEB8: 40980014  bge cr6, 0x82dedecc
	if !ctx.cr[6].lt {
	pc = 0x82DEDECC; continue 'dispatch;
	}
	// 82DEDEBC: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEDEC0: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEDEC4: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DEDEC8: 4800000C  b 0x82deded4
	pc = 0x82DEDED4; continue 'dispatch;
	// 82DEDECC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DEDED0: 4BFEE949  bl 0x82ddc818
	ctx.lr = 0x82DEDED4;
	sub_82DDC818(ctx, base);
	// 82DEDED4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEDED8: 7E5F9378  mr r31, r18
	ctx.r[31].u64 = ctx.r[18].u64;
	// 82DEDEDC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEDEE0: 838B0004  lwz r28, 4(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDEE4: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82DEDEE8: 409900B4  ble cr6, 0x82dedf9c
	if !ctx.cr[6].gt {
	pc = 0x82DEDF9C; continue 'dispatch;
	}
	// 82DEDEEC: 7E5D9378  mr r29, r18
	ctx.r[29].u64 = ctx.r[18].u64;
	// 82DEDEF0: 80790004  lwz r3, 4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDEF4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDEF8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEDEFC: 40980014  bge cr6, 0x82dedf10
	if !ctx.cr[6].lt {
	pc = 0x82DEDF10; continue 'dispatch;
	}
	// 82DEDF00: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEDF04: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEDF08: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DEDF0C: 4800000C  b 0x82dedf18
	pc = 0x82DEDF18; continue 'dispatch;
	// 82DEDF10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DEDF14: 4BFEE905  bl 0x82ddc818
	ctx.lr = 0x82DEDF18;
	sub_82DDC818(ctx, base);
	// 82DEDF18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEDF1C: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEDF20: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDF24: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEDF28: 40980010  bge cr6, 0x82dedf38
	if !ctx.cr[6].lt {
	pc = 0x82DEDF38; continue 'dispatch;
	}
	// 82DEDF2C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEDF30: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DEDF34: 4800000C  b 0x82dedf40
	pc = 0x82DEDF40; continue 'dispatch;
	// 82DEDF38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DEDF3C: 4BFEE8DD  bl 0x82ddc818
	ctx.lr = 0x82DEDF40;
	sub_82DDC818(ctx, base);
	// 82DEDF40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEDF44: 81515538  lwz r10, 0x5538(r17)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(21816 as u32) ) } as u64;
	// 82DEDF48: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEDF4C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEDF50: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DEDF54: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEDF58: 41980024  blt cr6, 0x82dedf7c
	if ctx.cr[6].lt {
	pc = 0x82DEDF7C; continue 'dispatch;
	}
	// 82DEDF5C: 556AD97E  srwi r10, r11, 5
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DEDF60: 556906FE  clrlwi r9, r11, 0x1b
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DEDF64: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82DEDF68: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DEDF6C: 7D4AD02E  lwzx r10, r10, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82DEDF70: 7D4A4C30  srw r10, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82DEDF74: 554A07FF  clrlwi. r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DEDF78: 41820018  beq 0x82dedf90
	if ctx.cr[0].eq {
	pc = 0x82DEDF90; continue 'dispatch;
	}
	// 82DEDF7C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DEDF80: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82DEDF84: 7F1FE000  cmpw cr6, r31, r28
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82DEDF88: 4198FF68  blt cr6, 0x82dedef0
	if ctx.cr[6].lt {
	pc = 0x82DEDEF0; continue 'dispatch;
	}
	// 82DEDF8C: 48000010  b 0x82dedf9c
	pc = 0x82DEDF9C; continue 'dispatch;
	// 82DEDF90: 7D785B78  mr r24, r11
	ctx.r[24].u64 = ctx.r[11].u64;
	// 82DEDF94: 7DFB7B78  mr r27, r15
	ctx.r[27].u64 = ctx.r[15].u64;
	// 82DEDF98: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 82DEDF9C: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEDFA0: 40820018  bne 0x82dedfb8
	if !ctx.cr[0].eq {
	pc = 0x82DEDFB8; continue 'dispatch;
	}
	// 82DEDFA4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DEDFA8: 4BFE5F59  bl 0x82dd3f00
	ctx.lr = 0x82DEDFAC;
	sub_82DD3F00(ctx, base);
	// 82DEDFAC: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82DEDFB0: 2F18FFFF  cmpwi cr6, r24, -1
	ctx.cr[6].compare_i32(ctx.r[24].s32, -1, &mut ctx.xer);
	// 82DEDFB4: 419A00DC  beq cr6, 0x82dee090
	if ctx.cr[6].eq {
	pc = 0x82DEE090; continue 'dispatch;
	}
	// 82DEDFB8: 81715538  lwz r11, 0x5538(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(21816 as u32) ) } as u64;
	// 82DEDFBC: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DEDFC0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82DEDFC4: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DEDFC8: 7F0A592E  stwx r24, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[24].u32) };
	// 82DEDFCC: 4BFE26F5  bl 0x82dd06c0
	ctx.lr = 0x82DEDFD0;
	sub_82DD06C0(ctx, base);
	// 82DEDFD0: 81730004  lwz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEDFD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEDFD8: 409AFDA4  bne cr6, 0x82dedd7c
	if !ctx.cr[6].eq {
	pc = 0x82DEDD7C; continue 'dispatch;
	}
	// 82DEDFDC: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82DEDFE0: 419A0010  beq cr6, 0x82dedff0
	if ctx.cr[6].eq {
	pc = 0x82DEDFF0; continue 'dispatch;
	}
	// 82DEDFE4: 389AFFFC  addi r4, r26, -4
	ctx.r[4].s64 = ctx.r[26].s64 + -4;
	// 82DEDFE8: 807AFFFC  lwz r3, -4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DEDFEC: 4BFB771D  bl 0x82da5708
	ctx.lr = 0x82DEDFF0;
	sub_82DA5708(ctx, base);
	// 82DEDFF0: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DEDFF4: 4BFE281D  bl 0x82dd0810
	ctx.lr = 0x82DEDFF8;
	sub_82DD0810(ctx, base);
	// 82DEDFF8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DEDFFC: 8159000C  lwz r10, 0xc(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DEE000: 388BAAC0  addi r4, r11, -0x5540
	ctx.r[4].s64 = ctx.r[11].s64 + -21824;
	// 82DEE004: 80790008  lwz r3, 8(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE008: 81770820  lwz r11, 0x820(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(2080 as u32) ) } as u64;
	// 82DEE00C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DEE010: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82DEE014: 4B4D1FED  bl 0x822c0000
	ctx.lr = 0x82DEE018;
	sub_822C0000(ctx, base);
	// 82DEE018: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82DEE01C: 40990018  ble cr6, 0x82dee034
	if !ctx.cr[6].gt {
	pc = 0x82DEE034; continue 'dispatch;
	}
	// 82DEE020: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DEE024: 80790008  lwz r3, 8(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE028: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 82DEE02C: 388BAA7C  addi r4, r11, -0x5584
	ctx.r[4].s64 = ctx.r[11].s64 + -21892;
	// 82DEE030: 4B4D1FD1  bl 0x822c0000
	ctx.lr = 0x82DEE034;
	sub_822C0000(ctx, base);
	// 82DEE034: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DEE038: 80790008  lwz r3, 8(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE03C: 7E85A378  mr r5, r20
	ctx.r[5].u64 = ctx.r[20].u64;
	// 82DEE040: 388BAA38  addi r4, r11, -0x55c8
	ctx.r[4].s64 = ctx.r[11].s64 + -21960;
	// 82DEE044: 4B4D1FBD  bl 0x822c0000
	ctx.lr = 0x82DEE048;
	sub_822C0000(ctx, base);
	// 82DEE048: 8159000C  lwz r10, 0xc(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DEE04C: 81790010  lwz r11, 0x10(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEE050: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DEE054: 40980034  bge cr6, 0x82dee088
	if !ctx.cr[6].lt {
	pc = 0x82DEE088; continue 'dispatch;
	}
	// 82DEE058: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEE05C: 8110553C  lwz r8, 0x553c(r16)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(21820 as u32) ) } as u64;
	// 82DEE060: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DEE064: 81315538  lwz r9, 0x5538(r17)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(21816 as u32) ) } as u64;
	// 82DEE068: 7D0B402E  lwzx r8, r11, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DEE06C: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DEE070: 7D08482E  lwzx r8, r8, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DEE074: 7D0B492E  stwx r8, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 82DEE078: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DEE07C: 81390010  lwz r9, 0x10(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEE080: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DEE084: 4198FFD8  blt cr6, 0x82dee05c
	if ctx.cr[6].lt {
	pc = 0x82DEE05C; continue 'dispatch;
	}
	// 82DEE088: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82DEE08C: 483BA0F8  b 0x831a8184
	sub_831A8180(ctx, base);
	return;
	// 82DEE090: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE094: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82DEE098: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DEE09C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82DEE0A0: 914B0554  stw r10, 0x554(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1364 as u32), ctx.r[10].u32 ) };
	// 82DEE0A4: 483BFC3D  bl 0x831adce0
	ctx.lr = 0x82DEE0A8;
	sub_831ADCE0(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEE0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEE0A8 size=172
    let mut pc: u32 = 0x82DEE0A8;
    'dispatch: loop {
        match pc {
            0x82DEE0A8 => {
    //   block [0x82DEE0A8..0x82DEE154)
	// 82DEE0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEE0AC: 483BA0BD  bl 0x831a8168
	ctx.lr = 0x82DEE0B0;
	sub_831A8130(ctx, base);
	// 82DEE0B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEE0B4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DEE0B8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DEE0BC: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEE0C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEE0C4: 40990060  ble cr6, 0x82dee124
	if !ctx.cr[6].gt {
	pc = 0x82DEE124; continue 'dispatch;
	}
	// 82DEE0C8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DEE0CC: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE0D0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE0D4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEE0D8: 40980010  bge cr6, 0x82dee0e8
	if !ctx.cr[6].lt {
	pc = 0x82DEE0E8; continue 'dispatch;
	}
	// 82DEE0DC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE0E0: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DEE0E4: 4800000C  b 0x82dee0f0
	pc = 0x82DEE0F0; continue 'dispatch;
	// 82DEE0E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DEE0EC: 4BFEE72D  bl 0x82ddc818
	ctx.lr = 0x82DEE0F0;
	sub_82DDC818(ctx, base);
	// 82DEE0F0: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE0F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DEE0F8: 419A0018  beq cr6, 0x82dee110
	if ctx.cr[6].eq {
	pc = 0x82DEE110; continue 'dispatch;
	}
	// 82DEE0FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEE100: 4BFFF889  bl 0x82ded988
	ctx.lr = 0x82DEE104;
	sub_82DED988(ctx, base);
	// 82DEE104: 389FFFFC  addi r4, r31, -4
	ctx.r[4].s64 = ctx.r[31].s64 + -4;
	// 82DEE108: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DEE10C: 4BFB75FD  bl 0x82da5708
	ctx.lr = 0x82DEE110;
	sub_82DA5708(ctx, base);
	// 82DEE110: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEE114: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DEE118: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82DEE11C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DEE120: 4198FFAC  blt cr6, 0x82dee0cc
	if ctx.cr[6].lt {
	pc = 0x82DEE0CC; continue 'dispatch;
	}
	// 82DEE124: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE128: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DEE12C: 419A000C  beq cr6, 0x82dee138
	if ctx.cr[6].eq {
	pc = 0x82DEE138; continue 'dispatch;
	}
	// 82DEE130: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DEE134: 4BFE4405  bl 0x82dd2538
	ctx.lr = 0x82DEE138;
	sub_82DD2538(ctx, base);
	// 82DEE138: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE13C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DEE140: 808B553C  lwz r4, 0x553c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21820 as u32) ) } as u64;
	// 82DEE144: 806A05B0  lwz r3, 0x5b0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DEE148: 4BFB75C1  bl 0x82da5708
	ctx.lr = 0x82DEE14C;
	sub_82DA5708(ctx, base);
	// 82DEE14C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DEE150: 483BA068  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEE158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEE158 size=80
    let mut pc: u32 = 0x82DEE158;
    'dispatch: loop {
        match pc {
            0x82DEE158 => {
    //   block [0x82DEE158..0x82DEE1A8)
	// 82DEE158: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEE15C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEE160: 419A0054  beq cr6, 0x82dee1b4
	if ctx.cr[6].eq {
		sub_82DEE1A8(ctx, base);
		return;
	}
	// 82DEE164: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DEE168: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE16C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DEE170: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 82DEE174: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82DEE178: 7D2B2050  subf r9, r11, r4
	ctx.r[9].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82DEE17C: 7D0958AE  lbzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DEE180: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE184: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DEE188: 409A0034  bne cr6, 0x82dee1bc
	if !ctx.cr[6].eq {
		sub_82DEE1BC(ctx, base);
		return;
	}
	// 82DEE18C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DEE190: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DEE194: 2F0A000C  cmpwi cr6, r10, 0xc
	ctx.cr[6].compare_i32(ctx.r[10].s32, 12, &mut ctx.xer);
	// 82DEE198: 4198FFE4  blt cr6, 0x82dee17c
	if ctx.cr[6].lt {
	pc = 0x82DEE17C; continue 'dispatch;
	}
	// 82DEE19C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DEE1A0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEE1A4: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEE1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEE1A8 size=20
    let mut pc: u32 = 0x82DEE1A8;
    'dispatch: loop {
        match pc {
            0x82DEE1A8 => {
    //   block [0x82DEE1A8..0x82DEE1BC)
	// 82DEE1A8: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEE1AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DEE1B0: 409AFFB8  bne cr6, 0x82dee168
	if !ctx.cr[6].eq {
		sub_82DEE158(ctx, base);
		return;
	}
	// 82DEE1B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEE1B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEE1BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEE1BC size=8
    let mut pc: u32 = 0x82DEE1BC;
    'dispatch: loop {
        match pc {
            0x82DEE1BC => {
    //   block [0x82DEE1BC..0x82DEE1C4)
	// 82DEE1BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DEE1C0: 4BFFFFE0  b 0x82dee1a0
	sub_82DEE158(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEE1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEE1C8 size=212
    let mut pc: u32 = 0x82DEE1C8;
    'dispatch: loop {
        match pc {
            0x82DEE1C8 => {
    //   block [0x82DEE1C8..0x82DEE29C)
	// 82DEE1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEE1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DEE1D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DEE1D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DEE1D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEE1DC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DEE1E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DEE1E4: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE1E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DEE1EC: 409A000C  bne cr6, 0x82dee1f8
	if !ctx.cr[6].eq {
	pc = 0x82DEE1F8; continue 'dispatch;
	}
	// 82DEE1F0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DEE1F4: 48000088  b 0x82dee27c
	pc = 0x82DEE27C; continue 'dispatch;
	// 82DEE1F8: 80CB0004  lwz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE1FC: 83CB0008  lwz r30, 8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE200: 1D66000C  mulli r11, r6, 0xc
	ctx.r[11].s64 = ctx.r[6].s64 * 12;
	// 82DEE204: 7F06F000  cmpw cr6, r6, r30
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82DEE208: 40990008  ble cr6, 0x82dee210
	if !ctx.cr[6].gt {
	pc = 0x82DEE210; continue 'dispatch;
	}
	// 82DEE20C: 1D7F000C  mulli r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 * 12;
	// 82DEE210: 7C8B2A14  add r4, r11, r5
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82DEE214: 4BFFFF45  bl 0x82dee158
	ctx.lr = 0x82DEE218;
	sub_82DEE158(ctx, base);
	// 82DEE218: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DEE21C: 41820060  beq 0x82dee27c
	if ctx.cr[0].eq {
	pc = 0x82DEE27C; continue 'dispatch;
	}
	// 82DEE220: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE224: 1D5F000C  mulli r10, r31, 0xc
	ctx.r[10].s64 = ctx.r[31].s64 * 12;
	// 82DEE228: 7D665850  subf r11, r6, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 82DEE22C: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82DEE230: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DEE234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DEE238: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DEE23C: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 82DEE240: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82DEE244: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DEE248: 7D0A58AE  lbzx r8, r10, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DEE24C: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE250: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DEE254: 409A0040  bne cr6, 0x82dee294
	if !ctx.cr[6].eq {
	pc = 0x82DEE294; continue 'dispatch;
	}
	// 82DEE258: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DEE25C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DEE260: 2F09000C  cmpwi cr6, r9, 0xc
	ctx.cr[6].compare_i32(ctx.r[9].s32, 12, &mut ctx.xer);
	// 82DEE264: 4198FFE4  blt cr6, 0x82dee248
	if ctx.cr[6].lt {
	pc = 0x82DEE248; continue 'dispatch;
	}
	// 82DEE268: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DEE26C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DEE270: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DEE274: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DEE278: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82DEE27C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DEE280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DEE284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DEE288: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DEE28C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DEE290: 4E800020  blr
	return;
	// 82DEE294: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DEE298: 4BFFFFD4  b 0x82dee26c
	pc = 0x82DEE26C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEE2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEE2A0 size=184
    let mut pc: u32 = 0x82DEE2A0;
    'dispatch: loop {
        match pc {
            0x82DEE2A0 => {
    //   block [0x82DEE2A0..0x82DEE358)
	// 82DEE2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEE2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DEE2A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DEE2AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DEE2B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEE2B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DEE2B8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DEE2BC: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE2C0: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE2C4: 7F06F000  cmpw cr6, r6, r30
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82DEE2C8: 41990078  bgt cr6, 0x82dee340
	if ctx.cr[6].gt {
	pc = 0x82DEE340; continue 'dispatch;
	}
	// 82DEE2CC: 1D66000C  mulli r11, r6, 0xc
	ctx.r[11].s64 = ctx.r[6].s64 * 12;
	// 82DEE2D0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE2D4: 7C8B2A14  add r4, r11, r5
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82DEE2D8: 4BFFFE81  bl 0x82dee158
	ctx.lr = 0x82DEE2DC;
	sub_82DEE158(ctx, base);
	// 82DEE2DC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE2E0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE2E4: 7D26F050  subf r9, r6, r30
	ctx.r[9].s64 = ctx.r[30].s64 - ctx.r[6].s64;
	// 82DEE2E8: 7D0A5850  subf r8, r10, r11
	ctx.r[8].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DEE2EC: 7F084800  cmpw cr6, r8, r9
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DEE2F0: 41990050  bgt cr6, 0x82dee340
	if ctx.cr[6].gt {
	pc = 0x82DEE340; continue 'dispatch;
	}
	// 82DEE2F4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DEE2F8: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DEE2FC: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82DEE300: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DEE304: 5566003E  slwi r6, r11, 0
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82DEE308: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DEE30C: 7F06F000  cmpw cr6, r6, r30
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82DEE310: 41990010  bgt cr6, 0x82dee320
	if ctx.cr[6].gt {
	pc = 0x82DEE320; continue 'dispatch;
	}
	// 82DEE314: 1D66000C  mulli r11, r6, 0xc
	ctx.r[11].s64 = ctx.r[6].s64 * 12;
	// 82DEE318: 7C8B2A14  add r4, r11, r5
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82DEE31C: 4BFFFE3D  bl 0x82dee158
	ctx.lr = 0x82DEE320;
	sub_82DEE158(ctx, base);
	// 82DEE320: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE324: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE328: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE32C: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE330: 7CEA5850  subf r7, r10, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DEE334: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82DEE338: 7F074800  cmpw cr6, r7, r9
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DEE33C: 4099FFB8  ble cr6, 0x82dee2f4
	if !ctx.cr[6].gt {
	pc = 0x82DEE2F4; continue 'dispatch;
	}
	// 82DEE340: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DEE344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DEE348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DEE34C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DEE350: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DEE354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEE358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEE358 size=304
    let mut pc: u32 = 0x82DEE358;
    'dispatch: loop {
        match pc {
            0x82DEE358 => {
    //   block [0x82DEE358..0x82DEE488)
	// 82DEE358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEE35C: 483B9E05  bl 0x831a8160
	ctx.lr = 0x82DEE360;
	sub_831A8130(ctx, base);
	// 82DEE360: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEE364: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DEE368: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE36C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE370: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DEE374: 4099000C  ble cr6, 0x82dee380
	if !ctx.cr[6].gt {
	pc = 0x82DEE380; continue 'dispatch;
	}
	// 82DEE378: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE37C: 48000104  b 0x82dee480
	pc = 0x82DEE480; continue 'dispatch;
	// 82DEE380: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE384: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 82DEE388: 83EA0014  lwz r31, 0x14(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEE38C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DEE390: 7D0B2214  add r8, r11, r4
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82DEE394: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 82DEE398: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE39C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82DEE3A0: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 82DEE3A4: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82DEE3A8: 7D2B4050  subf r9, r11, r8
	ctx.r[9].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 82DEE3AC: 7CE958AE  lbzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DEE3B0: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE3B4: 7F063840  cmplw cr6, r6, r7
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82DEE3B8: 409A002C  bne cr6, 0x82dee3e4
	if !ctx.cr[6].eq {
	pc = 0x82DEE3E4; continue 'dispatch;
	}
	// 82DEE3BC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DEE3C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DEE3C4: 2F0A000C  cmpwi cr6, r10, 0xc
	ctx.cr[6].compare_i32(ctx.r[10].s32, 12, &mut ctx.xer);
	// 82DEE3C8: 4198FFE4  blt cr6, 0x82dee3ac
	if ctx.cr[6].lt {
	pc = 0x82DEE3AC; continue 'dispatch;
	}
	// 82DEE3CC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DEE3D0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEE3D4: 40820018  bne 0x82dee3ec
	if !ctx.cr[0].eq {
	pc = 0x82DEE3EC; continue 'dispatch;
	}
	// 82DEE3D8: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 82DEE3DC: 83FF0010  lwz r31, 0x10(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEE3E0: 4BFFFFB8  b 0x82dee398
	pc = 0x82DEE398; continue 'dispatch;
	// 82DEE3E4: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82DEE3E8: 4BFFFFE8  b 0x82dee3d0
	pc = 0x82DEE3D0; continue 'dispatch;
	// 82DEE3EC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DEE3F0: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 82DEE3F4: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE3F8: 834B05B0  lwz r26, 0x5b0(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DEE3FC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DEE400: 4BFB7181  bl 0x82da5580
	ctx.lr = 0x82DEE404;
	sub_82DA5580(ctx, base);
	// 82DEE404: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DEE408: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DEE40C: 934B0000  stw r26, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82DEE410: 4182001C  beq 0x82dee42c
	if ctx.cr[0].eq {
	pc = 0x82DEE42C; continue 'dispatch;
	}
	// 82DEE414: 3D604000  lis r11, 0x4000
	ctx.r[11].s64 = 1073741824;
	// 82DEE418: 93830008  stw r28, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DEE41C: 93830018  stw r28, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 82DEE420: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DEE424: 9383001C  stw r28, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82DEE428: 48000008  b 0x82dee430
	pc = 0x82DEE430; continue 'dispatch;
	// 82DEE42C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DEE430: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DEE434: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82DEE438: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE43C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE440: 7D6BE850  subf r11, r11, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 82DEE444: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DEE448: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DEE44C: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEE450: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEE454: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DEE458: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DEE45C: 93830020  stw r28, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[28].u32 ) };
	// 82DEE460: 93E30014  stw r31, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82DEE464: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DEE468: 939F0010  stw r28, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 82DEE46C: 409A0010  bne cr6, 0x82dee47c
	if !ctx.cr[6].eq {
	pc = 0x82DEE47C; continue 'dispatch;
	}
	// 82DEE470: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE474: 906B0014  stw r3, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82DEE478: 48000008  b 0x82dee480
	pc = 0x82DEE480; continue 'dispatch;
	// 82DEE47C: 907B0010  stw r3, 0x10(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82DEE480: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DEE484: 483B9D2C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEE488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEE488 size=272
    let mut pc: u32 = 0x82DEE488;
    'dispatch: loop {
        match pc {
            0x82DEE488 => {
    //   block [0x82DEE488..0x82DEE598)
	// 82DEE488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEE48C: 483B9CCD  bl 0x831a8158
	ctx.lr = 0x82DEE490;
	sub_831A8130(ctx, base);
	// 82DEE490: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEE494: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DEE498: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DEE49C: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82DEE4A0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DEE4A4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DEE4A8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DEE4AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DEE4B0: 7F9AE378  mr r26, r28
	ctx.r[26].u64 = ctx.r[28].u64;
	// 82DEE4B4: 4BFFFD15  bl 0x82dee1c8
	ctx.lr = 0x82DEE4B8;
	sub_82DEE1C8(ctx, base);
	// 82DEE4B8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DEE4BC: 408200D4  bne 0x82dee590
	if !ctx.cr[0].eq {
	pc = 0x82DEE590; continue 'dispatch;
	}
	// 82DEE4C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DEE4C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEE4C8: 4BFFFE91  bl 0x82dee358
	ctx.lr = 0x82DEE4CC;
	sub_82DEE358(ctx, base);
	// 82DEE4CC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DEE4D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DEE4D4: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 82DEE4D8: 830B05B0  lwz r24, 0x5b0(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DEE4DC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DEE4E0: 4BFB70A1  bl 0x82da5580
	ctx.lr = 0x82DEE4E4;
	sub_82DA5580(ctx, base);
	// 82DEE4E4: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEE4E8: 93030000  stw r24, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 82DEE4EC: 4182001C  beq 0x82dee508
	if ctx.cr[0].eq {
	pc = 0x82DEE508; continue 'dispatch;
	}
	// 82DEE4F0: 3D404000  lis r10, 0x4000
	ctx.r[10].s64 = 1073741824;
	// 82DEE4F4: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DEE4F8: 938B0018  stw r28, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 82DEE4FC: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DEE500: 938B001C  stw r28, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82DEE504: 48000008  b 0x82dee50c
	pc = 0x82DEE50C; continue 'dispatch;
	// 82DEE508: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82DEE50C: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82DEE510: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82DEE514: 938B0014  stw r28, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82DEE518: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEE51C: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DEE520: 938B0020  stw r28, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[28].u32 ) };
	// 82DEE524: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DEE528: 419A0008  beq cr6, 0x82dee530
	if ctx.cr[6].eq {
	pc = 0x82DEE530; continue 'dispatch;
	}
	// 82DEE52C: 93DA0020  stw r30, 0x20(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82DEE530: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE534: 7FDAF378  mr r26, r30
	ctx.r[26].u64 = ctx.r[30].u64;
	// 82DEE538: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DEE53C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEE540: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DEE544: 409A0020  bne cr6, 0x82dee564
	if !ctx.cr[6].eq {
	pc = 0x82DEE564; continue 'dispatch;
	}
	// 82DEE548: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE54C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE550: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DEE554: 41990010  bgt cr6, 0x82dee564
	if ctx.cr[6].gt {
	pc = 0x82DEE564; continue 'dispatch;
	}
	// 82DEE558: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DEE55C: 933F0000  stw r25, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82DEE560: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DEE564: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DEE568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEE56C: 4BFFFD35  bl 0x82dee2a0
	ctx.lr = 0x82DEE570;
	sub_82DEE2A0(ctx, base);
	// 82DEE570: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DEE574: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DEE578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEE57C: 4BFFFC4D  bl 0x82dee1c8
	ctx.lr = 0x82DEE580;
	sub_82DEE1C8(ctx, base);
	// 82DEE580: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DEE584: 4182FF3C  beq 0x82dee4c0
	if ctx.cr[0].eq {
	pc = 0x82DEE4C0; continue 'dispatch;
	}
	// 82DEE588: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE58C: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82DEE590: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DEE594: 483B9C14  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEE598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEE598 size=268
    let mut pc: u32 = 0x82DEE598;
    'dispatch: loop {
        match pc {
            0x82DEE598 => {
    //   block [0x82DEE598..0x82DEE6A4)
	// 82DEE598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEE59C: 483B9BC5  bl 0x831a8160
	ctx.lr = 0x82DEE5A0;
	sub_831A8130(ctx, base);
	// 82DEE5A0: 9421FDE0  stwu r1, -0x220(r1)
	ea = ctx.r[1].u32.wrapping_add(-544 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEE5A4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DEE5A8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DEE5AC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DEE5B0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DEE5B4: 38A0017C  li r5, 0x17c
	ctx.r[5].s64 = 380;
	// 82DEE5B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DEE5BC: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82DEE5C0: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82DEE5C4: 483B9C1D  bl 0x831a81e0
	ctx.lr = 0x82DEE5C8;
	sub_831A81E0(ctx, base);
	// 82DEE5C8: 1D7D000C  mulli r11, r29, 0xc
	ctx.r[11].s64 = ctx.r[29].s64 * 12;
	// 82DEE5CC: 7C6BDA14  add r3, r11, r27
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82DEE5D0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DEE5D4: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82DEE5D8: 483B9F39  bl 0x831a8510
	ctx.lr = 0x82DEE5DC;
	sub_831A8510(ctx, base);
	// 82DEE5DC: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 82DEE5E0: 835C05B0  lwz r26, 0x5b0(r28)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DEE5E4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DEE5E8: 4BFB6F99  bl 0x82da5580
	ctx.lr = 0x82DEE5EC;
	sub_82DA5580(ctx, base);
	// 82DEE5EC: 37E30004  addic. r31, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DEE5F0: 93430000  stw r26, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82DEE5F4: 4182001C  beq 0x82dee610
	if ctx.cr[0].eq {
	pc = 0x82DEE610; continue 'dispatch;
	}
	// 82DEE5F8: 3D604000  lis r11, 0x4000
	ctx.r[11].s64 = 1073741824;
	// 82DEE5FC: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82DEE600: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82DEE604: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DEE608: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82DEE60C: 48000008  b 0x82dee614
	pc = 0x82DEE614; continue 'dispatch;
	// 82DEE610: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82DEE614: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82DEE618: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82DEE61C: 3940FFFE  li r10, -2
	ctx.r[10].s64 = -2;
	// 82DEE620: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82DEE624: 37BD0001  addic. r29, r29, 1
	ctx.xer.ca = (ctx.r[29].u32 > (!(1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DEE628: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82DEE62C: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82DEE630: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82DEE634: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DEE638: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DEE63C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82DEE640: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DEE644: 40810054  ble 0x82dee698
	if !ctx.cr[0].gt {
	pc = 0x82DEE698; continue 'dispatch;
	}
	// 82DEE648: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DEE64C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEE650: 409A0014  bne cr6, 0x82dee664
	if !ctx.cr[6].eq {
	pc = 0x82DEE664; continue 'dispatch;
	}
	// 82DEE654: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DEE658: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82DEE65C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DEE660: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DEE664: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 82DEE668: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DEE66C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DEE670: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DEE674: 4BFFFC2D  bl 0x82dee2a0
	ctx.lr = 0x82DEE678;
	sub_82DEE2A0(ctx, base);
	// 82DEE678: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DEE67C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DEE680: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DEE684: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DEE688: 4BFFFE01  bl 0x82dee488
	ctx.lr = 0x82DEE68C;
	sub_82DEE488(ctx, base);
	// 82DEE68C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DEE690: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DEE694: 4198FFB4  blt cr6, 0x82dee648
	if ctx.cr[6].lt {
	pc = 0x82DEE648; continue 'dispatch;
	}
	// 82DEE698: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEE69C: 38210220  addi r1, r1, 0x220
	ctx.r[1].s64 = ctx.r[1].s64 + 544;
	// 82DEE6A0: 483B9B10  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEE6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEE6A8 size=92
    let mut pc: u32 = 0x82DEE6A8;
    'dispatch: loop {
        match pc {
            0x82DEE6A8 => {
    //   block [0x82DEE6A8..0x82DEE704)
	// 82DEE6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEE6AC: 483B9ABD  bl 0x831a8168
	ctx.lr = 0x82DEE6B0;
	sub_831A8130(ctx, base);
	// 82DEE6B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEE6B4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DEE6B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DEE6BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DEE6C0: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DEE6C4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DEE6C8: 839E05B0  lwz r28, 0x5b0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DEE6CC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DEE6D0: 4BFB6EB1  bl 0x82da5580
	ctx.lr = 0x82DEE6D4;
	sub_82DA5580(ctx, base);
	// 82DEE6D4: 37E30004  addic. r31, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DEE6D8: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DEE6DC: 41820014  beq 0x82dee6f0
	if ctx.cr[0].eq {
	pc = 0x82DEE6F0; continue 'dispatch;
	}
	// 82DEE6E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEE6E4: 809E05B0  lwz r4, 0x5b0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DEE6E8: 4BFD0771  bl 0x82dbee58
	ctx.lr = 0x82DEE6EC;
	sub_82DBEE58(ctx, base);
	// 82DEE6EC: 48000008  b 0x82dee6f4
	pc = 0x82DEE6F4; continue 'dispatch;
	// 82DEE6F0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DEE6F4: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DEE6F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DEE6FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DEE700: 483B9AB8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEE708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEE708 size=472
    let mut pc: u32 = 0x82DEE708;
    'dispatch: loop {
        match pc {
            0x82DEE708 => {
    //   block [0x82DEE708..0x82DEE8E0)
	// 82DEE708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEE70C: 483B9A49  bl 0x831a8154
	ctx.lr = 0x82DEE710;
	sub_831A8130(ctx, base);
	// 82DEE710: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEE714: 83E30060  lwz r31, 0x60(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DEE718: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DEE71C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE720: 7D775B78  mr r23, r11
	ctx.r[23].u64 = ctx.r[11].u64;
	// 82DEE724: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DEE728: 408101B0  ble 0x82dee8d8
	if !ctx.cr[0].gt {
	pc = 0x82DEE8D8; continue 'dispatch;
	}
	// 82DEE72C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82DEE730: 3B1B0001  addi r24, r27, 1
	ctx.r[24].s64 = ctx.r[27].s64 + 1;
	// 82DEE734: 7F1CC378  mr r28, r24
	ctx.r[28].u64 = ctx.r[24].u64;
	// 82DEE738: 7F185840  cmplw cr6, r24, r11
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEE73C: 4098018C  bge cr6, 0x82dee8c8
	if !ctx.cr[6].lt {
	pc = 0x82DEE8C8; continue 'dispatch;
	}
	// 82DEE740: 3B590004  addi r26, r25, 4
	ctx.r[26].s64 = ctx.r[25].s64 + 4;
	// 82DEE744: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEE748: 40980010  bge cr6, 0x82dee758
	if !ctx.cr[6].lt {
	pc = 0x82DEE758; continue 'dispatch;
	}
	// 82DEE74C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE750: 7C6BCA14  add r3, r11, r25
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 82DEE754: 48000010  b 0x82dee764
	pc = 0x82DEE764; continue 'dispatch;
	// 82DEE758: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DEE75C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEE760: 4BFEE0B9  bl 0x82ddc818
	ctx.lr = 0x82DEE764;
	sub_82DDC818(ctx, base);
	// 82DEE764: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE768: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE76C: 7F1B5040  cmplw cr6, r27, r10
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DEE770: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE774: 83CB0004  lwz r30, 4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE778: 40980010  bge cr6, 0x82dee788
	if !ctx.cr[6].lt {
	pc = 0x82DEE788; continue 'dispatch;
	}
	// 82DEE77C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE780: 7C6BCA14  add r3, r11, r25
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 82DEE784: 48000010  b 0x82dee794
	pc = 0x82DEE794; continue 'dispatch;
	// 82DEE788: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DEE78C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEE790: 4BFEE089  bl 0x82ddc818
	ctx.lr = 0x82DEE794;
	sub_82DDC818(ctx, base);
	// 82DEE794: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE798: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE79C: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DEE7A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE7A4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DEE7A8: 7FABF1D6  mullw r29, r11, r30
	ctx.r[29].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82DEE7AC: 40980010  bge cr6, 0x82dee7bc
	if !ctx.cr[6].lt {
	pc = 0x82DEE7BC; continue 'dispatch;
	}
	// 82DEE7B0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE7B4: 7C6BD214  add r3, r11, r26
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82DEE7B8: 48000010  b 0x82dee7c8
	pc = 0x82DEE7C8; continue 'dispatch;
	// 82DEE7BC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DEE7C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEE7C4: 4BFEE055  bl 0x82ddc818
	ctx.lr = 0x82DEE7C8;
	sub_82DDC818(ctx, base);
	// 82DEE7C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE7CC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE7D0: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DEE7D4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE7D8: 83CB0004  lwz r30, 4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE7DC: 40980010  bge cr6, 0x82dee7ec
	if !ctx.cr[6].lt {
	pc = 0x82DEE7EC; continue 'dispatch;
	}
	// 82DEE7E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE7E4: 7C6BD214  add r3, r11, r26
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82DEE7E8: 48000010  b 0x82dee7f8
	pc = 0x82DEE7F8; continue 'dispatch;
	// 82DEE7EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DEE7F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEE7F4: 4BFEE025  bl 0x82ddc818
	ctx.lr = 0x82DEE7F8;
	sub_82DDC818(ctx, base);
	// 82DEE7F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE7FC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE800: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DEE804: 7D6BF1D6  mullw r11, r11, r30
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82DEE808: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DEE80C: 409800A8  bge cr6, 0x82dee8b4
	if !ctx.cr[6].lt {
	pc = 0x82DEE8B4; continue 'dispatch;
	}
	// 82DEE810: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE814: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEE818: 40980010  bge cr6, 0x82dee828
	if !ctx.cr[6].lt {
	pc = 0x82DEE828; continue 'dispatch;
	}
	// 82DEE81C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE820: 7C6BCA14  add r3, r11, r25
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 82DEE824: 48000010  b 0x82dee834
	pc = 0x82DEE834; continue 'dispatch;
	// 82DEE828: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DEE82C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEE830: 4BFEDFE9  bl 0x82ddc818
	ctx.lr = 0x82DEE834;
	sub_82DDC818(ctx, base);
	// 82DEE834: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE838: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE83C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEE840: 40980010  bge cr6, 0x82dee850
	if !ctx.cr[6].lt {
	pc = 0x82DEE850; continue 'dispatch;
	}
	// 82DEE844: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE848: 7FCBD214  add r30, r11, r26
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82DEE84C: 48000014  b 0x82dee860
	pc = 0x82DEE860; continue 'dispatch;
	// 82DEE850: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DEE854: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEE858: 4BFEDFC1  bl 0x82ddc818
	ctx.lr = 0x82DEE85C;
	sub_82DDC818(ctx, base);
	// 82DEE85C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DEE860: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE864: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEE868: 40980010  bge cr6, 0x82dee878
	if !ctx.cr[6].lt {
	pc = 0x82DEE878; continue 'dispatch;
	}
	// 82DEE86C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE870: 7C6BCA14  add r3, r11, r25
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 82DEE874: 48000010  b 0x82dee884
	pc = 0x82DEE884; continue 'dispatch;
	// 82DEE878: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DEE87C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEE880: 4BFEDF99  bl 0x82ddc818
	ctx.lr = 0x82DEE884;
	sub_82DDC818(ctx, base);
	// 82DEE884: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE888: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DEE88C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE890: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEE894: 40980010  bge cr6, 0x82dee8a4
	if !ctx.cr[6].lt {
	pc = 0x82DEE8A4; continue 'dispatch;
	}
	// 82DEE898: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE89C: 7C6BD214  add r3, r11, r26
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82DEE8A0: 48000010  b 0x82dee8b0
	pc = 0x82DEE8B0; continue 'dispatch;
	// 82DEE8A4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DEE8A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEE8AC: 4BFEDF6D  bl 0x82ddc818
	ctx.lr = 0x82DEE8B0;
	sub_82DDC818(ctx, base);
	// 82DEE8B0: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DEE8B4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE8B8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82DEE8BC: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 82DEE8C0: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEE8C4: 4198FE80  blt cr6, 0x82dee744
	if ctx.cr[6].lt {
	pc = 0x82DEE744; continue 'dispatch;
	}
	// 82DEE8C8: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 82DEE8CC: 3B390004  addi r25, r25, 4
	ctx.r[25].s64 = ctx.r[25].s64 + 4;
	// 82DEE8D0: 7F18B800  cmpw cr6, r24, r23
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82DEE8D4: 4198FE5C  blt cr6, 0x82dee730
	if ctx.cr[6].lt {
	pc = 0x82DEE730; continue 'dispatch;
	}
	// 82DEE8D8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DEE8DC: 483B98C8  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEE8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEE8E0 size=652
    let mut pc: u32 = 0x82DEE8E0;
    'dispatch: loop {
        match pc {
            0x82DEE8E0 => {
    //   block [0x82DEE8E0..0x82DEEB6C)
	// 82DEE8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEE8E4: 483B9871  bl 0x831a8154
	ctx.lr = 0x82DEE8E8;
	sub_831A8130(ctx, base);
	// 82DEE8E8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEE8EC: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DEE8F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DEE8F4: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82DEE8F8: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DEE8FC: 83B905B0  lwz r29, 0x5b0(r25)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DEE900: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DEE904: 4BFB6C7D  bl 0x82da5580
	ctx.lr = 0x82DEE908;
	sub_82DA5580(ctx, base);
	// 82DEE908: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82DEE90C: 37E30004  addic. r31, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DEE910: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DEE914: 41820018  beq 0x82dee92c
	if ctx.cr[0].eq {
	pc = 0x82DEE92C; continue 'dispatch;
	}
	// 82DEE918: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEE91C: 809905B0  lwz r4, 0x5b0(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DEE920: 4BFD0539  bl 0x82dbee58
	ctx.lr = 0x82DEE924;
	sub_82DBEE58(ctx, base);
	// 82DEE924: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 82DEE928: 48000008  b 0x82dee930
	pc = 0x82DEE930; continue 'dispatch;
	// 82DEE92C: 7F1AC378  mr r26, r24
	ctx.r[26].u64 = ctx.r[24].u64;
	// 82DEE930: 931E0008  stw r24, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 82DEE934: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DEE938: 83B905B0  lwz r29, 0x5b0(r25)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DEE93C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DEE940: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 82DEE944: 4BFB6C3D  bl 0x82da5580
	ctx.lr = 0x82DEE948;
	sub_82DA5580(ctx, base);
	// 82DEE948: 37E30004  addic. r31, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DEE94C: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DEE950: 41820018  beq 0x82dee968
	if ctx.cr[0].eq {
	pc = 0x82DEE968; continue 'dispatch;
	}
	// 82DEE954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEE958: 809905B0  lwz r4, 0x5b0(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DEE95C: 4BFD04FD  bl 0x82dbee58
	ctx.lr = 0x82DEE960;
	sub_82DBEE58(ctx, base);
	// 82DEE960: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82DEE964: 48000008  b 0x82dee96c
	pc = 0x82DEE96C; continue 'dispatch;
	// 82DEE968: 7F1CC378  mr r28, r24
	ctx.r[28].u64 = ctx.r[24].u64;
	// 82DEE96C: 83FE0014  lwz r31, 0x14(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEE970: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 82DEE974: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEE978: 40820094  bne 0x82deea0c
	if !ctx.cr[0].eq {
	pc = 0x82DEEA0C; continue 'dispatch;
	}
	// 82DEE97C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEE980: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEE984: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEE988: 419A0058  beq cr6, 0x82dee9e0
	if ctx.cr[6].eq {
	pc = 0x82DEE9E0; continue 'dispatch;
	}
	// 82DEE98C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE990: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DEE994: 356B0001  addic. r11, r11, 1
	ctx.xer.ca = (ctx.r[11].u32 > (!(1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEE998: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DEE99C: 40800008  bge 0x82dee9a4
	if !ctx.cr[0].lt {
	pc = 0x82DEE9A4; continue 'dispatch;
	}
	// 82DEE9A0: 931F0008  stw r24, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 82DEE9A4: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE9A8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DEE9AC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE9B0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DEE9B4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DEE9B8: 809A0004  lwz r4, 4(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE9BC: 4BFEDE5D  bl 0x82ddc818
	ctx.lr = 0x82DEE9C0;
	sub_82DDC818(ctx, base);
	// 82DEE9C0: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DEE9C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DEE9C8: 809C0004  lwz r4, 4(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE9CC: 4BFEDE4D  bl 0x82ddc818
	ctx.lr = 0x82DEE9D0;
	sub_82DDC818(ctx, base);
	// 82DEE9D0: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DEE9D4: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82DEE9D8: 83FF0014  lwz r31, 0x14(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEE9DC: 4BFFFF98  b 0x82dee974
	pc = 0x82DEE974; continue 'dispatch;
	// 82DEE9E0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEE9E4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DEE9E8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DEE9EC: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEE9F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEE9F4: 409A0008  bne cr6, 0x82dee9fc
	if !ctx.cr[6].eq {
	pc = 0x82DEE9FC; continue 'dispatch;
	}
	// 82DEE9F8: 93FE0018  stw r31, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 82DEE9FC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82DEEA00: 419A0008  beq cr6, 0x82deea08
	if ctx.cr[6].eq {
	pc = 0x82DEEA08; continue 'dispatch;
	}
	// 82DEEA04: 93FB0020  stw r31, 0x20(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 82DEEA08: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 82DEEA0C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82DEEA10: 83FF0010  lwz r31, 0x10(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEEA14: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 82DEEA18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DEEA1C: 409AFF58  bne cr6, 0x82dee974
	if !ctx.cr[6].eq {
	pc = 0x82DEE974; continue 'dispatch;
	}
	// 82DEEA20: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DEEA24: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DEEA28: 409A0018  bne cr6, 0x82deea40
	if !ctx.cr[6].eq {
	pc = 0x82DEEA40; continue 'dispatch;
	}
	// 82DEEA2C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEEA30: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DEEA34: 419A0008  beq cr6, 0x82deea3c
	if ctx.cr[6].eq {
	pc = 0x82DEEA3C; continue 'dispatch;
	}
	// 82DEEA38: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DEEA3C: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82DEEA40: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEA44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEEA48: 419A0044  beq cr6, 0x82deea8c
	if ctx.cr[6].eq {
	pc = 0x82DEEA8C; continue 'dispatch;
	}
	// 82DEEA4C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DEEA50: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82DEEA54: 4BFD79CD  bl 0x82dc6420
	ctx.lr = 0x82DEEA58;
	sub_82DC6420(ctx, base);
	// 82DEEA58: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DEEA5C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82DEEA60: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEEA64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEEA68: 409AFF0C  bne cr6, 0x82dee974
	if !ctx.cr[6].eq {
	pc = 0x82DEE974; continue 'dispatch;
	}
	// 82DEEA6C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEEA70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEEA74: 419A000C  beq cr6, 0x82deea80
	if ctx.cr[6].eq {
	pc = 0x82DEEA80; continue 'dispatch;
	}
	// 82DEEA78: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEEA7C: 48000008  b 0x82deea84
	pc = 0x82DEEA84; continue 'dispatch;
	// 82DEEA80: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82DEEA84: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82DEEA88: 4BFFFEEC  b 0x82dee974
	pc = 0x82DEE974; continue 'dispatch;
	// 82DEEA8C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEA90: 7F1CC378  mr r28, r24
	ctx.r[28].u64 = ctx.r[24].u64;
	// 82DEEA94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEEA98: 419A00CC  beq cr6, 0x82deeb64
	if ctx.cr[6].eq {
	pc = 0x82DEEB64; continue 'dispatch;
	}
	// 82DEEA9C: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 82DEEAA0: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEEAA4: 40980010  bge cr6, 0x82deeab4
	if !ctx.cr[6].lt {
	pc = 0x82DEEAB4; continue 'dispatch;
	}
	// 82DEEAA8: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEEAAC: 7C6BDA14  add r3, r11, r27
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82DEEAB0: 48000010  b 0x82deeac0
	pc = 0x82DEEAC0; continue 'dispatch;
	// 82DEEAB4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DEEAB8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DEEABC: 4BFEDD5D  bl 0x82ddc818
	ctx.lr = 0x82DEEAC0;
	sub_82DDC818(ctx, base);
	// 82DEEAC0: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEEAC4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEEAC8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82DEEACC: 40990084  ble cr6, 0x82deeb50
	if !ctx.cr[6].gt {
	pc = 0x82DEEB50; continue 'dispatch;
	}
	// 82DEEAD0: 83F905B0  lwz r31, 0x5b0(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DEEAD4: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82DEEAD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEEADC: 4BFB6AA5  bl 0x82da5580
	ctx.lr = 0x82DEEAE0;
	sub_82DA5580(ctx, base);
	// 82DEEAE0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DEEAE4: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DEEAE8: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DEEAEC: 41820014  beq 0x82deeb00
	if ctx.cr[0].eq {
	pc = 0x82DEEB00; continue 'dispatch;
	}
	// 82DEEAF0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82DEEAF4: 4BFFFBB5  bl 0x82dee6a8
	ctx.lr = 0x82DEEAF8;
	sub_82DEE6A8(ctx, base);
	// 82DEEAF8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DEEAFC: 48000008  b 0x82deeb04
	pc = 0x82DEEB04; continue 'dispatch;
	// 82DEEB00: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82DEEB04: 80770060  lwz r3, 0x60(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DEEB08: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEB0C: 4BFEDD0D  bl 0x82ddc818
	ctx.lr = 0x82DEEB10;
	sub_82DDC818(ctx, base);
	// 82DEEB10: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DEEB14: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEEB18: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DEEB1C: 83FD0018  lwz r31, 0x18(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEEB20: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DEEB24: 419A0018  beq cr6, 0x82deeb3c
	if ctx.cr[6].eq {
	pc = 0x82DEEB3C; continue 'dispatch;
	}
	// 82DEEB28: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEB2C: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEB30: 4BFEDCE9  bl 0x82ddc818
	ctx.lr = 0x82DEEB34;
	sub_82DDC818(ctx, base);
	// 82DEEB34: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DEEB38: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DEEB3C: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DEEB40: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEEB44: 419A000C  beq cr6, 0x82deeb50
	if ctx.cr[6].eq {
	pc = 0x82DEEB50; continue 'dispatch;
	}
	// 82DEEB48: 83FF0020  lwz r31, 0x20(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DEEB4C: 4BFFFFD4  b 0x82deeb20
	pc = 0x82DEEB20; continue 'dispatch;
	// 82DEEB50: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEB54: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82DEEB58: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82DEEB5C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEEB60: 4198FF48  blt cr6, 0x82deeaa8
	if ctx.cr[6].lt {
	pc = 0x82DEEAA8; continue 'dispatch;
	}
	// 82DEEB64: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DEEB68: 483B963C  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEEB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEEB70 size=1260
    let mut pc: u32 = 0x82DEEB70;
    'dispatch: loop {
        match pc {
            0x82DEEB70 => {
    //   block [0x82DEEB70..0x82DEF05C)
	// 82DEEB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEEB74: 483B95C1  bl 0x831a8134
	ctx.lr = 0x82DEEB78;
	sub_831A8130(ctx, base);
	// 82DEEB78: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEEB7C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DEEB80: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DEEB84: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 82DEEB88: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 82DEEB8C: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 82DEEB90: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DEEB94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEEB98: 4BFFFA01  bl 0x82dee598
	ctx.lr = 0x82DEEB9C;
	sub_82DEE598(ctx, base);
	// 82DEEB9C: 837505B0  lwz r27, 0x5b0(r21)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DEEBA0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DEEBA4: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DEEBA8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DEEBAC: 4BFB69D5  bl 0x82da5580
	ctx.lr = 0x82DEEBB0;
	sub_82DA5580(ctx, base);
	// 82DEEBB0: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 82DEEBB4: 37C30004  addic. r30, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DEEBB8: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82DEEBBC: 41820014  beq 0x82deebd0
	if ctx.cr[0].eq {
	pc = 0x82DEEBD0; continue 'dispatch;
	}
	// 82DEEBC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DEEBC4: 809505B0  lwz r4, 0x5b0(r21)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82DEEBC8: 4BFD0291  bl 0x82dbee58
	ctx.lr = 0x82DEEBCC;
	sub_82DBEE58(ctx, base);
	// 82DEEBCC: 48000008  b 0x82deebd4
	pc = 0x82DEEBD4; continue 'dispatch;
	// 82DEEBD0: 7E7E9B78  mr r30, r19
	ctx.r[30].u64 = ctx.r[19].u64;
	// 82DEEBD4: 93D40060  stw r30, 0x60(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82DEEBD8: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 82DEEBDC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DEEBE0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DEEBE4: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 82DEEBE8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DEEBEC: 4BFFFCF5  bl 0x82dee8e0
	ctx.lr = 0x82DEEBF0;
	sub_82DEE8E0(ctx, base);
	// 82DEEBF0: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82DEEBF4: 4BFFFB15  bl 0x82dee708
	ctx.lr = 0x82DEEBF8;
	sub_82DEE708(ctx, base);
	// 82DEEBF8: 57E41838  slwi r4, r31, 3
	ctx.r[4].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82DEEBFC: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82DEEC00: 4BFB7091  bl 0x82da5c90
	ctx.lr = 0x82DEEC04;
	sub_82DA5C90(ctx, base);
	// 82DEEC04: 7C721B78  mr r18, r3
	ctx.r[18].u64 = ctx.r[3].u64;
	// 82DEEC08: 7E6B9B78  mr r11, r19
	ctx.r[11].u64 = ctx.r[19].u64;
	// 82DEEC0C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DEEC10: 40990020  ble cr6, 0x82deec30
	if !ctx.cr[6].gt {
	pc = 0x82DEEC30; continue 'dispatch;
	}
	// 82DEEC14: 7E4A9378  mr r10, r18
	ctx.r[10].u64 = ctx.r[18].u64;
	// 82DEEC18: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DEEC1C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DEEC20: 926A0004  stw r19, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[19].u32 ) };
	// 82DEEC24: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DEEC28: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DEEC2C: 4198FFEC  blt cr6, 0x82deec18
	if ctx.cr[6].lt {
	pc = 0x82DEEC18; continue 'dispatch;
	}
	// 82DEEC30: 81740060  lwz r11, 0x60(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DEEC34: 3A20FFFF  li r17, -1
	ctx.r[17].s64 = -1;
	// 82DEEC38: 7E709B78  mr r16, r19
	ctx.r[16].u64 = ctx.r[19].u64;
	// 82DEEC3C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEC40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEEC44: 409903E4  ble cr6, 0x82def028
	if !ctx.cr[6].gt {
	pc = 0x82DEF028; continue 'dispatch;
	}
	// 82DEEC48: 7E6F9B78  mr r15, r19
	ctx.r[15].u64 = ctx.r[19].u64;
	// 82DEEC4C: 80740060  lwz r3, 0x60(r20)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DEEC50: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEC54: 7F105840  cmplw cr6, r16, r11
	ctx.cr[6].compare_u32(ctx.r[16].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEEC58: 40980010  bge cr6, 0x82deec68
	if !ctx.cr[6].lt {
	pc = 0x82DEEC68; continue 'dispatch;
	}
	// 82DEEC5C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEEC60: 7C6B7A14  add r3, r11, r15
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[15].u64;
	// 82DEEC64: 4800000C  b 0x82deec70
	pc = 0x82DEEC70; continue 'dispatch;
	// 82DEEC68: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 82DEEC6C: 4BFEDBAD  bl 0x82ddc818
	ctx.lr = 0x82DEEC70;
	sub_82DDC818(ctx, base);
	// 82DEEC70: 82C30000  lwz r22, 0(r3)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEEC74: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82DEEC78: 7E7F9B78  mr r31, r19
	ctx.r[31].u64 = ctx.r[19].u64;
	// 82DEEC7C: 80760004  lwz r3, 4(r22)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEC80: 82F60000  lwz r23, 0(r22)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEEC84: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEC88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEEC8C: 40990088  ble cr6, 0x82deed14
	if !ctx.cr[6].gt {
	pc = 0x82DEED14; continue 'dispatch;
	}
	// 82DEEC90: 7E7E9B78  mr r30, r19
	ctx.r[30].u64 = ctx.r[19].u64;
	// 82DEEC94: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEC98: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEEC9C: 40980010  bge cr6, 0x82deecac
	if !ctx.cr[6].lt {
	pc = 0x82DEECAC; continue 'dispatch;
	}
	// 82DEECA0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEECA4: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DEECA8: 4800000C  b 0x82deecb4
	pc = 0x82DEECB4; continue 'dispatch;
	// 82DEECAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DEECB0: 4BFEDB69  bl 0x82ddc818
	ctx.lr = 0x82DEECB4;
	sub_82DDC818(ctx, base);
	// 82DEECB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEECB8: 7E6A9B78  mr r10, r19
	ctx.r[10].u64 = ctx.r[19].u64;
	// 82DEECBC: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82DEECC0: 4099003C  ble cr6, 0x82deecfc
	if !ctx.cr[6].gt {
	pc = 0x82DEECFC; continue 'dispatch;
	}
	// 82DEECC4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEECC8: 7D6B9214  add r11, r11, r18
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[18].u64;
	// 82DEECCC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DEECD0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEECD4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DEECD8: 409A0020  bne cr6, 0x82deecf8
	if !ctx.cr[6].eq {
	pc = 0x82DEECF8; continue 'dispatch;
	}
	// 82DEECDC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DEECE0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DEECE4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DEECE8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DEECEC: 7F0AB800  cmpw cr6, r10, r23
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82DEECF0: 4198FFE0  blt cr6, 0x82deecd0
	if ctx.cr[6].lt {
	pc = 0x82DEECD0; continue 'dispatch;
	}
	// 82DEECF4: 48000008  b 0x82deecfc
	pc = 0x82DEECFC; continue 'dispatch;
	// 82DEECF8: 7E7D9B78  mr r29, r19
	ctx.r[29].u64 = ctx.r[19].u64;
	// 82DEECFC: 80760004  lwz r3, 4(r22)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEED00: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DEED04: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DEED08: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEED0C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEED10: 4198FF84  blt cr6, 0x82deec94
	if ctx.cr[6].lt {
	pc = 0x82DEEC94; continue 'dispatch;
	}
	// 82DEED14: 80760004  lwz r3, 4(r22)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEED18: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEED1C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEED20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEED24: 40820088  bne 0x82deedac
	if !ctx.cr[0].eq {
	pc = 0x82DEEDAC; continue 'dispatch;
	}
	// 82DEED28: 7E7F9B78  mr r31, r19
	ctx.r[31].u64 = ctx.r[19].u64;
	// 82DEED2C: 409902E4  ble cr6, 0x82def010
	if !ctx.cr[6].gt {
	pc = 0x82DEF010; continue 'dispatch;
	}
	// 82DEED30: 7E7E9B78  mr r30, r19
	ctx.r[30].u64 = ctx.r[19].u64;
	// 82DEED34: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEED38: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEED3C: 40980010  bge cr6, 0x82deed4c
	if !ctx.cr[6].lt {
	pc = 0x82DEED4C; continue 'dispatch;
	}
	// 82DEED40: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEED44: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DEED48: 4800000C  b 0x82deed54
	pc = 0x82DEED54; continue 'dispatch;
	// 82DEED4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DEED50: 4BFEDAC9  bl 0x82ddc818
	ctx.lr = 0x82DEED54;
	sub_82DDC818(ctx, base);
	// 82DEED54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEED58: 7E6A9B78  mr r10, r19
	ctx.r[10].u64 = ctx.r[19].u64;
	// 82DEED5C: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82DEED60: 40990030  ble cr6, 0x82deed90
	if !ctx.cr[6].gt {
	pc = 0x82DEED90; continue 'dispatch;
	}
	// 82DEED64: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEED68: 7D6B9214  add r11, r11, r18
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[18].u64;
	// 82DEED6C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DEED70: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEED74: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 82DEED78: 409A0018  bne cr6, 0x82deed90
	if !ctx.cr[6].eq {
	pc = 0x82DEED90; continue 'dispatch;
	}
	// 82DEED7C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DEED80: 926B0000  stw r19, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[19].u32 ) };
	// 82DEED84: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DEED88: 7F0AB800  cmpw cr6, r10, r23
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82DEED8C: 4198FFE4  blt cr6, 0x82deed70
	if ctx.cr[6].lt {
	pc = 0x82DEED70; continue 'dispatch;
	}
	// 82DEED90: 80760004  lwz r3, 4(r22)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEED94: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DEED98: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DEED9C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEDA0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEEDA4: 4198FF90  blt cr6, 0x82deed34
	if ctx.cr[6].lt {
	pc = 0x82DEED34; continue 'dispatch;
	}
	// 82DEEDA8: 48000268  b 0x82def010
	pc = 0x82DEF010; continue 'dispatch;
	// 82DEEDAC: 3A310001  addi r17, r17, 1
	ctx.r[17].s64 = ctx.r[17].s64 + 1;
	// 82DEEDB0: 7E7A9B78  mr r26, r19
	ctx.r[26].u64 = ctx.r[19].u64;
	// 82DEEDB4: 4099025C  ble cr6, 0x82def010
	if !ctx.cr[6].gt {
	pc = 0x82DEF010; continue 'dispatch;
	}
	// 82DEEDB8: 7E789B78  mr r24, r19
	ctx.r[24].u64 = ctx.r[19].u64;
	// 82DEEDBC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEDC0: 7F1A5840  cmplw cr6, r26, r11
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEEDC4: 40980010  bge cr6, 0x82deedd4
	if !ctx.cr[6].lt {
	pc = 0x82DEEDD4; continue 'dispatch;
	}
	// 82DEEDC8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEEDCC: 7C6BC214  add r3, r11, r24
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82DEEDD0: 4800000C  b 0x82deeddc
	pc = 0x82DEEDDC; continue 'dispatch;
	// 82DEEDD4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DEEDD8: 4BFEDA41  bl 0x82ddc818
	ctx.lr = 0x82DEEDDC;
	sub_82DDC818(ctx, base);
	// 82DEEDDC: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEEDE0: 7E7C9B78  mr r28, r19
	ctx.r[28].u64 = ctx.r[19].u64;
	// 82DEEDE4: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82DEEDE8: 40990210  ble cr6, 0x82deeff8
	if !ctx.cr[6].gt {
	pc = 0x82DEEFF8; continue 'dispatch;
	}
	// 82DEEDEC: 3B37FFFF  addi r25, r23, -1
	ctx.r[25].s64 = ctx.r[23].s64 + -1;
	// 82DEEDF0: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82DEEDF4: 409A00E8  bne cr6, 0x82deeedc
	if !ctx.cr[6].eq {
	pc = 0x82DEEEDC; continue 'dispatch;
	}
	// 82DEEDF8: 81740034  lwz r11, 0x34(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DEEDFC: 576A1838  slwi r10, r27, 3
	ctx.r[10].u32 = ctx.r[27].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DEEE00: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DEEE04: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEEE08: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEE0C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DEEE10: 419A0048  beq cr6, 0x82deee58
	if ctx.cr[6].eq {
	pc = 0x82DEEE58; continue 'dispatch;
	}
	// 82DEEE14: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DEEE18: 419A003C  beq cr6, 0x82deee54
	if ctx.cr[6].eq {
	pc = 0x82DEEE54; continue 'dispatch;
	}
	// 82DEEE1C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DEEE20: 419A0038  beq cr6, 0x82deee58
	if ctx.cr[6].eq {
	pc = 0x82DEEE58; continue 'dispatch;
	}
	// 82DEEE24: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEEE28: 48000024  b 0x82deee4c
	pc = 0x82DEEE4C; continue 'dispatch;
	// 82DEEE2C: 83BD0008  lwz r29, 8(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEEE30: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DEEE34: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEEE38: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DEEE3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DEEE40: 4E800421  bctrl
	ctx.lr = 0x82DEEE44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DEEE44: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEEE48: 40820010  bne 0x82deee58
	if !ctx.cr[0].eq {
	pc = 0x82DEEE58; continue 'dispatch;
	}
	// 82DEEE4C: 7F1DF840  cmplw cr6, r29, r31
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DEEE50: 409AFFDC  bne cr6, 0x82deee2c
	if !ctx.cr[6].eq {
	pc = 0x82DEEE2C; continue 'dispatch;
	}
	// 82DEEE54: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82DEEE58: 81760004  lwz r11, 4(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEE5C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DEEE60: 83D505AC  lwz r30, 0x5ac(r21)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DEEE64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DEEE68: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEE6C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DEEE70: 7F1A5840  cmplw cr6, r26, r11
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEEE74: 409A002C  bne cr6, 0x82deeea0
	if !ctx.cr[6].eq {
	pc = 0x82DEEEA0; continue 'dispatch;
	}
	// 82DEEE78: 4BFB6709  bl 0x82da5580
	ctx.lr = 0x82DEEE7C;
	sub_82DA5580(ctx, base);
	// 82DEEE7C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DEEE80: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DEEE84: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DEEE88: 41820040  beq 0x82deeec8
	if ctx.cr[0].eq {
	pc = 0x82DEEEC8; continue 'dispatch;
	}
	// 82DEEE8C: 7EA6AB78  mr r6, r21
	ctx.r[6].u64 = ctx.r[21].u64;
	// 82DEEE90: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82DEEE94: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 82DEEE98: 4BFF1AD1  bl 0x82de0968
	ctx.lr = 0x82DEEE9C;
	sub_82DE0968(ctx, base);
	// 82DEEE9C: 48000030  b 0x82deeecc
	pc = 0x82DEEECC; continue 'dispatch;
	// 82DEEEA0: 4BFB66E1  bl 0x82da5580
	ctx.lr = 0x82DEEEA4;
	sub_82DA5580(ctx, base);
	// 82DEEEA4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DEEEA8: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DEEEAC: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DEEEB0: 41820018  beq 0x82deeec8
	if ctx.cr[0].eq {
	pc = 0x82DEEEC8; continue 'dispatch;
	}
	// 82DEEEB4: 7EA6AB78  mr r6, r21
	ctx.r[6].u64 = ctx.r[21].u64;
	// 82DEEEB8: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82DEEEBC: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 82DEEEC0: 4BFF1A51  bl 0x82de0910
	ctx.lr = 0x82DEEEC4;
	sub_82DE0910(ctx, base);
	// 82DEEEC4: 48000008  b 0x82deeecc
	pc = 0x82DEEECC; continue 'dispatch;
	// 82DEEEC8: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82DEEECC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82DEEED0: 807F03B4  lwz r3, 0x3b4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(948 as u32) ) } as u64;
	// 82DEEED4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DEEED8: 4BFF2E59  bl 0x82de1d30
	ctx.lr = 0x82DEEEDC;
	sub_82DE1D30(ctx, base);
	// 82DEEEDC: 7F1CC800  cmpw cr6, r28, r25
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[25].s32, &mut ctx.xer);
	// 82DEEEE0: 409A00DC  bne cr6, 0x82deefbc
	if !ctx.cr[6].eq {
	pc = 0x82DEEFBC; continue 'dispatch;
	}
	// 82DEEEE4: 7D5CDA14  add r10, r28, r27
	ctx.r[10].u64 = ctx.r[28].u64 + ctx.r[27].u64;
	// 82DEEEE8: 81740034  lwz r11, 0x34(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DEEEEC: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DEEEF0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DEEEF4: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEEEF8: 83CB0004  lwz r30, 4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEEFC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DEEF00: 419A0044  beq cr6, 0x82deef44
	if ctx.cr[6].eq {
	pc = 0x82DEEF44; continue 'dispatch;
	}
	// 82DEEF04: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DEEF08: 419A0040  beq cr6, 0x82deef48
	if ctx.cr[6].eq {
	pc = 0x82DEEF48; continue 'dispatch;
	}
	// 82DEEF0C: 7F1EF840  cmplw cr6, r30, r31
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DEEF10: 419A0034  beq cr6, 0x82deef44
	if ctx.cr[6].eq {
	pc = 0x82DEEF44; continue 'dispatch;
	}
	// 82DEEF14: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEF18: 48000024  b 0x82deef3c
	pc = 0x82DEEF3C; continue 'dispatch;
	// 82DEEF1C: 83BD0004  lwz r29, 4(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEF20: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DEEF24: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEEF28: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DEEF2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DEEF30: 4E800421  bctrl
	ctx.lr = 0x82DEEF34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DEEF34: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEEF38: 40820010  bne 0x82deef48
	if !ctx.cr[0].eq {
	pc = 0x82DEEF48; continue 'dispatch;
	}
	// 82DEEF3C: 7F1DF840  cmplw cr6, r29, r31
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DEEF40: 409AFFDC  bne cr6, 0x82deef1c
	if !ctx.cr[6].eq {
	pc = 0x82DEEF1C; continue 'dispatch;
	}
	// 82DEEF44: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82DEEF48: 81760004  lwz r11, 4(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEF4C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 82DEEF50: 83D505AC  lwz r30, 0x5ac(r21)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82DEEF54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DEEF58: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEF5C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DEEF60: 7F1A5840  cmplw cr6, r26, r11
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEEF64: 409A0024  bne cr6, 0x82deef88
	if !ctx.cr[6].eq {
	pc = 0x82DEEF88; continue 'dispatch;
	}
	// 82DEEF68: 4BFB6619  bl 0x82da5580
	ctx.lr = 0x82DEEF6C;
	sub_82DA5580(ctx, base);
	// 82DEEF6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DEEF70: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DEEF74: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DEEF78: 41820030  beq 0x82deefa8
	if ctx.cr[0].eq {
	pc = 0x82DEEFA8; continue 'dispatch;
	}
	// 82DEEF7C: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82DEEF80: 4BFF1A99  bl 0x82de0a18
	ctx.lr = 0x82DEEF84;
	sub_82DE0A18(ctx, base);
	// 82DEEF84: 48000028  b 0x82deefac
	pc = 0x82DEEFAC; continue 'dispatch;
	// 82DEEF88: 4BFB65F9  bl 0x82da5580
	ctx.lr = 0x82DEEF8C;
	sub_82DA5580(ctx, base);
	// 82DEEF8C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DEEF90: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DEEF94: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DEEF98: 41820010  beq 0x82deefa8
	if ctx.cr[0].eq {
	pc = 0x82DEEFA8; continue 'dispatch;
	}
	// 82DEEF9C: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82DEEFA0: 4BFF1A21  bl 0x82de09c0
	ctx.lr = 0x82DEEFA4;
	sub_82DE09C0(ctx, base);
	// 82DEEFA4: 48000008  b 0x82deefac
	pc = 0x82DEEFAC; continue 'dispatch;
	// 82DEEFA8: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82DEEFAC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82DEEFB0: 807F03B4  lwz r3, 0x3b4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(948 as u32) ) } as u64;
	// 82DEEFB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DEEFB8: 4BFF2D19  bl 0x82de1cd0
	ctx.lr = 0x82DEEFBC;
	sub_82DE1CD0(ctx, base);
	// 82DEEFBC: 7D7CDA14  add r11, r28, r27
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[27].u64;
	// 82DEEFC0: 81560004  lwz r10, 4(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEFC4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82DEEFC8: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEEFCC: 7F1CB800  cmpw cr6, r28, r23
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82DEEFD0: 7D2B9214  add r9, r11, r18
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[18].u64;
	// 82DEEFD4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEFD8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DEEFDC: 7D7A5850  subf r11, r26, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[26].s64;
	// 82DEEFE0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DEEFE4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DEEFE8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82DEEFEC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82DEEFF0: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DEEFF4: 4198FDFC  blt cr6, 0x82deedf0
	if ctx.cr[6].lt {
	pc = 0x82DEEDF0; continue 'dispatch;
	}
	// 82DEEFF8: 80760004  lwz r3, 4(r22)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEEFFC: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82DEF000: 3B180004  addi r24, r24, 4
	ctx.r[24].s64 = ctx.r[24].s64 + 4;
	// 82DEF004: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEF008: 7F1A5840  cmplw cr6, r26, r11
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEF00C: 4198FDB0  blt cr6, 0x82deedbc
	if ctx.cr[6].lt {
	pc = 0x82DEEDBC; continue 'dispatch;
	}
	// 82DEF010: 81740060  lwz r11, 0x60(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DEF014: 3A100001  addi r16, r16, 1
	ctx.r[16].s64 = ctx.r[16].s64 + 1;
	// 82DEF018: 39EF0004  addi r15, r15, 4
	ctx.r[15].s64 = ctx.r[15].s64 + 4;
	// 82DEF01C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEF020: 7F105840  cmplw cr6, r16, r11
	ctx.cr[6].compare_u32(ctx.r[16].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEF024: 4198FC28  blt cr6, 0x82deec4c
	if ctx.cr[6].lt {
	pc = 0x82DEEC4C; continue 'dispatch;
	}
	// 82DEF028: 38B10001  addi r5, r17, 1
	ctx.r[5].s64 = ctx.r[17].s64 + 1;
	// 82DEF02C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DEF030: 90B505FC  stw r5, 0x5fc(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(1532 as u32), ctx.r[5].u32 ) };
	// 82DEF034: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82DEF038: 388BAB04  addi r4, r11, -0x54fc
	ctx.r[4].s64 = ctx.r[11].s64 + -21756;
	// 82DEF03C: 4B4D0FC5  bl 0x822c0000
	ctx.lr = 0x82DEF040;
	sub_822C0000(ctx, base);
	// 82DEF040: 807505A4  lwz r3, 0x5a4(r21)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(1444 as u32) ) } as u64;
	// 82DEF044: 8175059C  lwz r11, 0x59c(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(1436 as u32) ) } as u64;
	// 82DEF048: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 82DEF04C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DEF050: 4E800421  bctrl
	ctx.lr = 0x82DEF054;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DEF054: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82DEF058: 483B912C  b 0x831a8184
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DEF060 size=248
    let mut pc: u32 = 0x82DEF060;
    'dispatch: loop {
        match pc {
            0x82DEF060 => {
    //   block [0x82DEF060..0x82DEF158)
	// 82DEF060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEF064: 483B90FD  bl 0x831a8160
	ctx.lr = 0x82DEF068;
	sub_831A8130(ctx, base);
	// 82DEF068: DBC1FFB8  stfd f30, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[30].u64 ) };
	// 82DEF06C: DBE1FFC0  stfd f31, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82DEF070: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEF074: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DEF078: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DEF07C: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEF080: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82DEF084: 4198000C  blt cr6, 0x82def090
	if ctx.cr[6].lt {
	pc = 0x82DEF090; continue 'dispatch;
	}
	// 82DEF088: 3B60002B  li r27, 0x2b
	ctx.r[27].s64 = 43;
	// 82DEF08C: 4800000C  b 0x82def098
	pc = 0x82DEF098; continue 'dispatch;
	// 82DEF090: 3B60002D  li r27, 0x2d
	ctx.r[27].s64 = 45;
	// 82DEF094: FC200850  fneg f1, f1
	ctx.f[1].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 82DEF098: FC00081E  fctiwz f0, f1
	ctx.f[0].s64 = if ctx.f[1].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[1].f64.trunc() as i32 as i64 };
	// 82DEF09C: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82DEF0A0: 83810054  lwz r28, 0x54(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DEF0A4: 7F8B07B4  extsw r11, r28
	ctx.r[11].s64 = ctx.r[28].s32 as i64;
	// 82DEF0A8: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82DEF0AC: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DEF0B0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DEF0B4: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 82DEF0B8: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DEF0BC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DEF0C0: C3CA89AC  lfs f30, -0x7654(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82DEF0C4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DEF0C8: 3BAB7690  addi r29, r11, 0x7690
	ctx.r[29].s64 = ctx.r[11].s64 + 30352;
	// 82DEF0CC: EFE10028  fsubs f31, f1, f0
	ctx.f[31].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DEF0D0: FC00F81E  fctiwz f0, f31
	ctx.f[0].s64 = if ctx.f[31].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[31].f64.trunc() as i32 as i64 };
	// 82DEF0D4: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82DEF0D8: 83C10054  lwz r30, 0x54(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DEF0DC: 7FCA07B4  extsw r10, r30
	ctx.r[10].s64 = ctx.r[30].s32 as i64;
	// 82DEF0E0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DEF0E4: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82DEF0E8: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DEF0EC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DEF0F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DEF0F4: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DEF0F8: 7C7F5A14  add r3, r31, r11
	ctx.r[3].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82DEF0FC: EC1F0028  fsubs f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DEF100: EFE007B2  fmuls f31, f0, f30
	ctx.f[31].f64 = (((ctx.f[0].f64 * ctx.f[30].f64) as f32) as f64);
	// 82DEF104: FC00F81E  fctiwz f0, f31
	ctx.f[0].s64 = if ctx.f[31].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[31].f64.trunc() as i32 as i64 };
	// 82DEF108: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 82DEF10C: 83C1005C  lwz r30, 0x5c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DEF110: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DEF114: 483B99C5  bl 0x831a8ad8
	ctx.lr = 0x82DEF118;
	sub_831A8AD8(ctx, base);
	// 82DEF118: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DEF11C: 2F1F0009  cmpwi cr6, r31, 9
	ctx.cr[6].compare_i32(ctx.r[31].s32, 9, &mut ctx.xer);
	// 82DEF120: 4198FFBC  blt cr6, 0x82def0dc
	if ctx.cr[6].lt {
	pc = 0x82DEF0DC; continue 'dispatch;
	}
	// 82DEF124: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DEF128: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82DEF12C: 99610069  stb r11, 0x69(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(105 as u32), ctx.r[11].u8 ) };
	// 82DEF130: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82DEF134: 388AAB2C  addi r4, r10, -0x54d4
	ctx.r[4].s64 = ctx.r[10].s64 + -21716;
	// 82DEF138: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DEF13C: 7F650774  extsb r5, r27
	ctx.r[5].s64 = ctx.r[27].s8 as i64;
	// 82DEF140: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DEF144: 483B9995  bl 0x831a8ad8
	ctx.lr = 0x82DEF148;
	sub_831A8AD8(ctx, base);
	// 82DEF148: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DEF14C: CBC1FFB8  lfd f30, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 82DEF150: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82DEF154: 483B905C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEF158 size=196
    let mut pc: u32 = 0x82DEF158;
    'dispatch: loop {
        match pc {
            0x82DEF158 => {
    //   block [0x82DEF158..0x82DEF21C)
	// 82DEF158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEF15C: 483B900D  bl 0x831a8168
	ctx.lr = 0x82DEF160;
	sub_831A8130(ctx, base);
	// 82DEF160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEF164: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DEF168: 3D608331  lis r11, -0x7ccf
	ctx.r[11].s64 = -2093940736;
	// 82DEF16C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DEF170: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DEF174: 3BAB2438  addi r29, r11, 0x2438
	ctx.r[29].s64 = ctx.r[11].s64 + 9272;
	// 82DEF178: 815F00E4  lwz r10, 0xe4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DEF17C: 554AFFFF  rlwinm. r10, r10, 0x1f, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DEF180: 4182000C  beq 0x82def18c
	if ctx.cr[0].eq {
	pc = 0x82DEF18C; continue 'dispatch;
	}
	// 82DEF184: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DEF188: 48000014  b 0x82def19c
	pc = 0x82DEF19C; continue 'dispatch;
	// 82DEF18C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEF190: 1D6B0034  mulli r11, r11, 0x34
	ctx.r[11].s64 = ctx.r[11].s64 * 52;
	// 82DEF194: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DEF198: 556BEFFE  rlwinm r11, r11, 0x1d, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82DEF19C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF1A0: 40820014  bne 0x82def1b4
	if !ctx.cr[0].eq {
	pc = 0x82DEF1B4; continue 'dispatch;
	}
	// 82DEF1A4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DEF1A8: 396BAB40  addi r11, r11, -0x54c0
	ctx.r[11].s64 = ctx.r[11].s64 + -21696;
	// 82DEF1AC: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82DEF1B0: 48000048  b 0x82def1f8
	pc = 0x82DEF1F8; continue 'dispatch;
	// 82DEF1B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEF1B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEF1BC: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DEF1C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DEF1C4: 4E800421  bctrl
	ctx.lr = 0x82DEF1C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DEF1C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF1CC: 40820038  bne 0x82def204
	if !ctx.cr[0].eq {
	pc = 0x82DEF204; continue 'dispatch;
	}
	// 82DEF1D0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEF1D4: 1D6B0034  mulli r11, r11, 0x34
	ctx.r[11].s64 = ctx.r[11].s64 * 52;
	// 82DEF1D8: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DEF1DC: 556BFFFF  rlwinm. r11, r11, 0x1f, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF1E0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DEF1E4: 396BAB40  addi r11, r11, -0x54c0
	ctx.r[11].s64 = ctx.r[11].s64 + -21696;
	// 82DEF1E8: 4182000C  beq 0x82def1f4
	if ctx.cr[0].eq {
	pc = 0x82DEF1F4; continue 'dispatch;
	}
	// 82DEF1EC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DEF1F0: 48000008  b 0x82def1f8
	pc = 0x82DEF1F8; continue 'dispatch;
	// 82DEF1F4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DEF1F8: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEF1FC: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DEF200: 7FCA582E  lwzx r30, r10, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DEF204: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82DEF208: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DEF20C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DEF210: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82DEF214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DEF218: 483B8FA0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEF220 size=228
    let mut pc: u32 = 0x82DEF220;
    'dispatch: loop {
        match pc {
            0x82DEF220 => {
    //   block [0x82DEF220..0x82DEF304)
	// 82DEF220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEF224: 483B8F49  bl 0x831a816c
	ctx.lr = 0x82DEF228;
	sub_831A8130(ctx, base);
	// 82DEF228: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEF22C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DEF230: 3D608331  lis r11, -0x7ccf
	ctx.r[11].s64 = -2093940736;
	// 82DEF234: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DEF238: 3BAB2438  addi r29, r11, 0x2438
	ctx.r[29].s64 = ctx.r[11].s64 + 9272;
	// 82DEF23C: 815F00E4  lwz r10, 0xe4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DEF240: 554AFFFF  rlwinm. r10, r10, 0x1f, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DEF244: 4182000C  beq 0x82def250
	if ctx.cr[0].eq {
	pc = 0x82DEF250; continue 'dispatch;
	}
	// 82DEF248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DEF24C: 48000014  b 0x82def260
	pc = 0x82DEF260; continue 'dispatch;
	// 82DEF250: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEF254: 1D6B0034  mulli r11, r11, 0x34
	ctx.r[11].s64 = ctx.r[11].s64 * 52;
	// 82DEF258: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DEF25C: 556BEFFE  rlwinm r11, r11, 0x1d, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82DEF260: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF264: 40820014  bne 0x82def278
	if !ctx.cr[0].eq {
	pc = 0x82DEF278; continue 'dispatch;
	}
	// 82DEF268: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DEF26C: 396BAB40  addi r11, r11, -0x54c0
	ctx.r[11].s64 = ctx.r[11].s64 + -21696;
	// 82DEF270: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82DEF274: 48000048  b 0x82def2bc
	pc = 0x82DEF2BC; continue 'dispatch;
	// 82DEF278: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEF27C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEF280: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DEF284: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DEF288: 4E800421  bctrl
	ctx.lr = 0x82DEF28C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DEF28C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF290: 4082003C  bne 0x82def2cc
	if !ctx.cr[0].eq {
	pc = 0x82DEF2CC; continue 'dispatch;
	}
	// 82DEF294: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEF298: 1D6B0034  mulli r11, r11, 0x34
	ctx.r[11].s64 = ctx.r[11].s64 * 52;
	// 82DEF29C: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DEF2A0: 556BFFFF  rlwinm. r11, r11, 0x1f, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF2A4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DEF2A8: 396BAB40  addi r11, r11, -0x54c0
	ctx.r[11].s64 = ctx.r[11].s64 + -21696;
	// 82DEF2AC: 4182000C  beq 0x82def2b8
	if ctx.cr[0].eq {
	pc = 0x82DEF2B8; continue 'dispatch;
	}
	// 82DEF2B0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DEF2B4: 48000008  b 0x82def2bc
	pc = 0x82DEF2BC; continue 'dispatch;
	// 82DEF2B8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DEF2BC: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEF2C0: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DEF2C4: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DEF2C8: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DEF2CC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEF2D0: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82DEF2D4: 409A0028  bne cr6, 0x82def2fc
	if !ctx.cr[6].eq {
	pc = 0x82DEF2FC; continue 'dispatch;
	}
	// 82DEF2D8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DEF2DC: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82DEF2E0: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 82DEF2E4: 38CBAC80  addi r6, r11, -0x5380
	ctx.r[6].s64 = ctx.r[11].s64 + -21376;
	// 82DEF2E8: 38AAAC70  addi r5, r10, -0x5390
	ctx.r[5].s64 = ctx.r[10].s64 + -21392;
	// 82DEF2EC: 38895E90  addi r4, r9, 0x5e90
	ctx.r[4].s64 = ctx.r[9].s64 + 24208;
	// 82DEF2F0: 38E00050  li r7, 0x50
	ctx.r[7].s64 = 80;
	// 82DEF2F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEF2F8: 4BEF3789  bl 0x82ce2a80
	ctx.lr = 0x82DEF2FC;
	sub_82CE2A80(ctx, base);
	// 82DEF2FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DEF300: 483B8EBC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEF308 size=544
    let mut pc: u32 = 0x82DEF308;
    'dispatch: loop {
        match pc {
            0x82DEF308 => {
    //   block [0x82DEF308..0x82DEF528)
	// 82DEF308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEF30C: 483B8E4D  bl 0x831a8158
	ctx.lr = 0x82DEF310;
	sub_831A8130(ctx, base);
	// 82DEF310: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEF314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DEF318: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82DEF31C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DEF320: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82DEF324: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82DEF328: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEF32C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEF330: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DEF334: 4E800421  bctrl
	ctx.lr = 0x82DEF338;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DEF338: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82DEF33C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DEF340: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DEF344: 3B6AAB80  addi r27, r10, -0x5480
	ctx.r[27].s64 = ctx.r[10].s64 + -21632;
	// 82DEF348: 3BABA778  addi r29, r11, -0x5888
	ctx.r[29].s64 = ctx.r[11].s64 + -22664;
	// 82DEF34C: 40810044  ble 0x82def390
	if !ctx.cr[0].gt {
	pc = 0x82DEF390; continue 'dispatch;
	}
	// 82DEF350: 817F00EC  lwz r11, 0xec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 82DEF354: 395D0006  addi r10, r29, 6
	ctx.r[10].s64 = ctx.r[29].s64 + 6;
	// 82DEF358: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DEF35C: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 82DEF360: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DEF364: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DEF368: 41820018  beq 0x82def380
	if ctx.cr[0].eq {
	pc = 0x82DEF380; continue 'dispatch;
	}
	// 82DEF36C: 895F009F  lbz r10, 0x9f(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(159 as u32) ) } as u64;
	// 82DEF370: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82DEF374: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DEF378: 41820008  beq 0x82def380
	if ctx.cr[0].eq {
	pc = 0x82DEF380; continue 'dispatch;
	}
	// 82DEF37C: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82DEF380: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEF384: 7FCBD82E  lwzx r30, r11, r27
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82DEF388: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82DEF38C: 419A0140  beq cr6, 0x82def4cc
	if ctx.cr[6].eq {
	pc = 0x82DEF4CC; continue 'dispatch;
	}
	// 82DEF390: 3D608331  lis r11, -0x7ccf
	ctx.r[11].s64 = -2093940736;
	// 82DEF394: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEF398: 3B8B2438  addi r28, r11, 0x2438
	ctx.r[28].s64 = ctx.r[11].s64 + 9272;
	// 82DEF39C: 1D6A0034  mulli r11, r10, 0x34
	ctx.r[11].s64 = ctx.r[10].s64 * 52;
	// 82DEF3A0: 7D6BE02E  lwzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DEF3A4: 556BFFFF  rlwinm. r11, r11, 0x1f, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF3A8: 418200AC  beq 0x82def454
	if ctx.cr[0].eq {
	pc = 0x82DEF454; continue 'dispatch;
	}
	// 82DEF3AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEF3B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEF3B4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEF3B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DEF3BC: 4E800421  bctrl
	ctx.lr = 0x82DEF3C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DEF3C0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82DEF3C4: 41990020  bgt cr6, 0x82def3e4
	if ctx.cr[6].gt {
	pc = 0x82DEF3E4; continue 'dispatch;
	}
	// 82DEF3C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEF3CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEF3D0: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DEF3D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DEF3D8: 4E800421  bctrl
	ctx.lr = 0x82DEF3DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DEF3DC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF3E0: 41820074  beq 0x82def454
	if ctx.cr[0].eq {
	pc = 0x82DEF454; continue 'dispatch;
	}
	// 82DEF3E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEF3E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEF3EC: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DEF3F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DEF3F4: 4E800421  bctrl
	ctx.lr = 0x82DEF3F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DEF3F8: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82DEF3FC: 395D0006  addi r10, r29, 6
	ctx.r[10].s64 = ctx.r[29].s64 + 6;
	// 82DEF400: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DEF404: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DEF408: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DEF40C: 392B003A  addi r9, r11, 0x3a
	ctx.r[9].s64 = ctx.r[11].s64 + 58;
	// 82DEF410: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DEF414: 7D29F82E  lwzx r9, r9, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DEF418: 81290050  lwz r9, 0x50(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DEF41C: 1D29000C  mulli r9, r9, 0xc
	ctx.r[9].s64 = ctx.r[9].s64 * 12;
	// 82DEF420: 7D4950AE  lbzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DEF424: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DEF428: 4182001C  beq 0x82def444
	if ctx.cr[0].eq {
	pc = 0x82DEF444; continue 'dispatch;
	}
	// 82DEF42C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DEF430: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82DEF434: 896B009E  lbz r11, 0x9e(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(158 as u32) ) } as u64;
	// 82DEF438: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DEF43C: 41820008  beq 0x82def444
	if ctx.cr[0].eq {
	pc = 0x82DEF444; continue 'dispatch;
	}
	// 82DEF440: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82DEF444: 1D7E0005  mulli r11, r30, 5
	ctx.r[11].s64 = ctx.r[30].s64 * 5;
	// 82DEF448: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DEF44C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEF450: 7FCBD82E  lwzx r30, r11, r27
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82DEF454: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82DEF458: 419A0074  beq cr6, 0x82def4cc
	if ctx.cr[6].eq {
	pc = 0x82DEF4CC; continue 'dispatch;
	}
	// 82DEF45C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEF460: 1D6B0034  mulli r11, r11, 0x34
	ctx.r[11].s64 = ctx.r[11].s64 * 52;
	// 82DEF464: 7D6BE02E  lwzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DEF468: 556BFFFF  rlwinm. r11, r11, 0x1f, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF46C: 41820060  beq 0x82def4cc
	if ctx.cr[0].eq {
	pc = 0x82DEF4CC; continue 'dispatch;
	}
	// 82DEF470: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEF474: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEF478: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEF47C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DEF480: 4E800421  bctrl
	ctx.lr = 0x82DEF484;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DEF484: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82DEF488: 40990044  ble cr6, 0x82def4cc
	if !ctx.cr[6].gt {
	pc = 0x82DEF4CC; continue 'dispatch;
	}
	// 82DEF48C: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 82DEF490: 395D0006  addi r10, r29, 6
	ctx.r[10].s64 = ctx.r[29].s64 + 6;
	// 82DEF494: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DEF498: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 82DEF49C: 7D4B50AE  lbzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DEF4A0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DEF4A4: 41820018  beq 0x82def4bc
	if ctx.cr[0].eq {
	pc = 0x82DEF4BC; continue 'dispatch;
	}
	// 82DEF4A8: 897F00A1  lbz r11, 0xa1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(161 as u32) ) } as u64;
	// 82DEF4AC: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82DEF4B0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DEF4B4: 41820008  beq 0x82def4bc
	if ctx.cr[0].eq {
	pc = 0x82DEF4BC; continue 'dispatch;
	}
	// 82DEF4B8: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82DEF4BC: 1D7E0005  mulli r11, r30, 5
	ctx.r[11].s64 = ctx.r[30].s64 * 5;
	// 82DEF4C0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DEF4C4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEF4C8: 7FCBD82E  lwzx r30, r11, r27
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82DEF4CC: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82DEF4D0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DEF4D4: 41990008  bgt cr6, 0x82def4dc
	if ctx.cr[6].gt {
	pc = 0x82DEF4DC; continue 'dispatch;
	}
	// 82DEF4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DEF4DC: 99780000  stb r11, 0(r24)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DEF4E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEF4E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEF4E8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEF4EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DEF4F0: 4E800421  bctrl
	ctx.lr = 0x82DEF4F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DEF4F4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF4F8: 4182000C  beq 0x82def504
	if ctx.cr[0].eq {
	pc = 0x82DEF504; continue 'dispatch;
	}
	// 82DEF4FC: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82DEF500: 41990018  bgt cr6, 0x82def518
	if ctx.cr[6].gt {
	pc = 0x82DEF518; continue 'dispatch;
	}
	// 82DEF504: 89780000  lbz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEF508: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DEF50C: 41820010  beq 0x82def51c
	if ctx.cr[0].eq {
	pc = 0x82DEF51C; continue 'dispatch;
	}
	// 82DEF510: 7F19D000  cmpw cr6, r25, r26
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82DEF514: 40980008  bge cr6, 0x82def51c
	if !ctx.cr[6].lt {
	pc = 0x82DEF51C; continue 'dispatch;
	}
	// 82DEF518: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 82DEF51C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DEF520: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DEF524: 483B8C84  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEF528 size=336
    let mut pc: u32 = 0x82DEF528;
    'dispatch: loop {
        match pc {
            0x82DEF528 => {
    //   block [0x82DEF528..0x82DEF678)
	// 82DEF528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEF52C: 483B8C39  bl 0x831a8164
	ctx.lr = 0x82DEF530;
	sub_831A8130(ctx, base);
	// 82DEF530: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEF534: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DEF538: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DEF53C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DEF540: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEF544: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DEF548: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DEF54C: 4E800421  bctrl
	ctx.lr = 0x82DEF550;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DEF550: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF554: 4182000C  beq 0x82def560
	if ctx.cr[0].eq {
	pc = 0x82DEF560; continue 'dispatch;
	}
	// 82DEF558: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DEF55C: 48000114  b 0x82def670
	pc = 0x82DEF670; continue 'dispatch;
	// 82DEF560: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DEF564: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DEF568: 4BFFFDA1  bl 0x82def308
	ctx.lr = 0x82DEF56C;
	sub_82DEF308(ctx, base);
	// 82DEF56C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DEF570: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 82DEF574: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82DEF578: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DEF57C: 3BAB5E90  addi r29, r11, 0x5e90
	ctx.r[29].s64 = ctx.r[11].s64 + 24208;
	// 82DEF580: 3B8AAC80  addi r28, r10, -0x5380
	ctx.r[28].s64 = ctx.r[10].s64 + -21376;
	// 82DEF584: 409A0020  bne cr6, 0x82def5a4
	if !ctx.cr[6].eq {
	pc = 0x82DEF5A4; continue 'dispatch;
	}
	// 82DEF588: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DEF58C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DEF590: 38ABAD00  addi r5, r11, -0x5300
	ctx.r[5].s64 = ctx.r[11].s64 + -21248;
	// 82DEF594: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DEF598: 38E000A9  li r7, 0xa9
	ctx.r[7].s64 = 169;
	// 82DEF59C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEF5A0: 4BEF34E1  bl 0x82ce2a80
	ctx.lr = 0x82DEF5A4;
	sub_82CE2A80(ctx, base);
	// 82DEF5A4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEF5A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF5AC: 419A0024  beq cr6, 0x82def5d0
	if ctx.cr[6].eq {
	pc = 0x82DEF5D0; continue 'dispatch;
	}
	// 82DEF5B0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DEF5B4: 419A001C  beq cr6, 0x82def5d0
	if ctx.cr[6].eq {
	pc = 0x82DEF5D0; continue 'dispatch;
	}
	// 82DEF5B8: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DEF5BC: 893B0008  lbz r9, 8(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEF5C0: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DEF5C4: 419A000C  beq cr6, 0x82def5d0
	if ctx.cr[6].eq {
	pc = 0x82DEF5D0; continue 'dispatch;
	}
	// 82DEF5C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEF5CC: 480000A4  b 0x82def670
	pc = 0x82DEF670; continue 'dispatch;
	// 82DEF5D0: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEF5D4: 3D208331  lis r9, -0x7ccf
	ctx.r[9].s64 = -2093940736;
	// 82DEF5D8: 1D4A0034  mulli r10, r10, 0x34
	ctx.r[10].s64 = ctx.r[10].s64 * 52;
	// 82DEF5DC: 39292438  addi r9, r9, 0x2438
	ctx.r[9].s64 = ctx.r[9].s64 + 9272;
	// 82DEF5E0: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DEF5E4: 554AF7FF  rlwinm. r10, r10, 0x1e, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DEF5E8: 41820038  beq 0x82def620
	if ctx.cr[0].eq {
	pc = 0x82DEF620; continue 'dispatch;
	}
	// 82DEF5EC: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 82DEF5F0: 40990020  ble cr6, 0x82def610
	if !ctx.cr[6].gt {
	pc = 0x82DEF610; continue 'dispatch;
	}
	// 82DEF5F4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DEF5F8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DEF5FC: 38ABACEC  addi r5, r11, -0x5314
	ctx.r[5].s64 = ctx.r[11].s64 + -21268;
	// 82DEF600: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DEF604: 38E000B6  li r7, 0xb6
	ctx.r[7].s64 = 182;
	// 82DEF608: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEF60C: 4BEF3475  bl 0x82ce2a80
	ctx.lr = 0x82DEF610;
	sub_82CE2A80(ctx, base);
	// 82DEF610: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEF614: 1D6B0005  mulli r11, r11, 5
	ctx.r[11].s64 = ctx.r[11].s64 * 5;
	// 82DEF618: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DEF61C: 48000034  b 0x82def650
	pc = 0x82DEF650; continue 'dispatch;
	// 82DEF620: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82DEF624: 40990020  ble cr6, 0x82def644
	if !ctx.cr[6].gt {
	pc = 0x82DEF644; continue 'dispatch;
	}
	// 82DEF628: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DEF62C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DEF630: 38ABACD8  addi r5, r11, -0x5328
	ctx.r[5].s64 = ctx.r[11].s64 + -21288;
	// 82DEF634: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DEF638: 38E000B9  li r7, 0xb9
	ctx.r[7].s64 = 185;
	// 82DEF63C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEF640: 4BEF3441  bl 0x82ce2a80
	ctx.lr = 0x82DEF644;
	sub_82CE2A80(ctx, base);
	// 82DEF644: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEF648: 1D5F0005  mulli r10, r31, 5
	ctx.r[10].s64 = ctx.r[31].s64 * 5;
	// 82DEF64C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DEF650: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82DEF654: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEF658: 394AAB80  addi r10, r10, -0x5480
	ctx.r[10].s64 = ctx.r[10].s64 + -21632;
	// 82DEF65C: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DEF660: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DEF664: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DEF668: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DEF66C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82DEF670: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DEF674: 483B8B40  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEF678 size=392
    let mut pc: u32 = 0x82DEF678;
    'dispatch: loop {
        match pc {
            0x82DEF678 => {
    //   block [0x82DEF678..0x82DEF800)
	// 82DEF678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEF67C: 483B8AE9  bl 0x831a8164
	ctx.lr = 0x82DEF680;
	sub_831A8130(ctx, base);
	// 82DEF680: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEF684: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DEF688: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DEF68C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DEF690: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEF694: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DEF698: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DEF69C: 4E800421  bctrl
	ctx.lr = 0x82DEF6A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DEF6A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF6A4: 40820154  bne 0x82def7f8
	if !ctx.cr[0].eq {
	pc = 0x82DEF7F8; continue 'dispatch;
	}
	// 82DEF6A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DEF6AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DEF6B0: 4BFFFC59  bl 0x82def308
	ctx.lr = 0x82DEF6B4;
	sub_82DEF308(ctx, base);
	// 82DEF6B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DEF6B8: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 82DEF6BC: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82DEF6C0: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DEF6C4: 3B8B5E90  addi r28, r11, 0x5e90
	ctx.r[28].s64 = ctx.r[11].s64 + 24208;
	// 82DEF6C8: 3B6AAC80  addi r27, r10, -0x5380
	ctx.r[27].s64 = ctx.r[10].s64 + -21376;
	// 82DEF6CC: 409A0020  bne cr6, 0x82def6ec
	if !ctx.cr[6].eq {
	pc = 0x82DEF6EC; continue 'dispatch;
	}
	// 82DEF6D0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DEF6D4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DEF6D8: 38ABAD00  addi r5, r11, -0x5300
	ctx.r[5].s64 = ctx.r[11].s64 + -21248;
	// 82DEF6DC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DEF6E0: 38E000CA  li r7, 0xca
	ctx.r[7].s64 = 202;
	// 82DEF6E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEF6E8: 4BEF3399  bl 0x82ce2a80
	ctx.lr = 0x82DEF6EC;
	sub_82CE2A80(ctx, base);
	// 82DEF6EC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEF6F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF6F4: 409A0010  bne cr6, 0x82def704
	if !ctx.cr[6].eq {
	pc = 0x82DEF704; continue 'dispatch;
	}
	// 82DEF6F8: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DEF6FC: 997D0008  stb r11, 8(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82DEF700: 48000038  b 0x82def738
	pc = 0x82DEF738; continue 'dispatch;
	// 82DEF704: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DEF708: 419A0030  beq cr6, 0x82def738
	if ctx.cr[6].eq {
	pc = 0x82DEF738; continue 'dispatch;
	}
	// 82DEF70C: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DEF710: 895D0008  lbz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEF714: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEF718: 419A0020  beq cr6, 0x82def738
	if ctx.cr[6].eq {
	pc = 0x82DEF738; continue 'dispatch;
	}
	// 82DEF71C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DEF720: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DEF724: 38ABAD14  addi r5, r11, -0x52ec
	ctx.r[5].s64 = ctx.r[11].s64 + -21228;
	// 82DEF728: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DEF72C: 38E000CF  li r7, 0xcf
	ctx.r[7].s64 = 207;
	// 82DEF730: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEF734: 4BEF334D  bl 0x82ce2a80
	ctx.lr = 0x82DEF738;
	sub_82CE2A80(ctx, base);
	// 82DEF738: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEF73C: 3D408331  lis r10, -0x7ccf
	ctx.r[10].s64 = -2093940736;
	// 82DEF740: 1D6B0034  mulli r11, r11, 0x34
	ctx.r[11].s64 = ctx.r[11].s64 * 52;
	// 82DEF744: 394A2438  addi r10, r10, 0x2438
	ctx.r[10].s64 = ctx.r[10].s64 + 9272;
	// 82DEF748: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DEF74C: 556BF7FF  rlwinm. r11, r11, 0x1e, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF750: 41820038  beq 0x82def788
	if ctx.cr[0].eq {
	pc = 0x82DEF788; continue 'dispatch;
	}
	// 82DEF754: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 82DEF758: 40990020  ble cr6, 0x82def778
	if !ctx.cr[6].gt {
	pc = 0x82DEF778; continue 'dispatch;
	}
	// 82DEF75C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DEF760: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DEF764: 38ABACEC  addi r5, r11, -0x5314
	ctx.r[5].s64 = ctx.r[11].s64 + -21268;
	// 82DEF768: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DEF76C: 38E000D3  li r7, 0xd3
	ctx.r[7].s64 = 211;
	// 82DEF770: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEF774: 4BEF330D  bl 0x82ce2a80
	ctx.lr = 0x82DEF778;
	sub_82CE2A80(ctx, base);
	// 82DEF778: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEF77C: 1D6B0005  mulli r11, r11, 5
	ctx.r[11].s64 = ctx.r[11].s64 * 5;
	// 82DEF780: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DEF784: 48000038  b 0x82def7bc
	pc = 0x82DEF7BC; continue 'dispatch;
	// 82DEF788: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEF78C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82DEF790: 40990020  ble cr6, 0x82def7b0
	if !ctx.cr[6].gt {
	pc = 0x82DEF7B0; continue 'dispatch;
	}
	// 82DEF794: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DEF798: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DEF79C: 38ABACD8  addi r5, r11, -0x5328
	ctx.r[5].s64 = ctx.r[11].s64 + -21288;
	// 82DEF7A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DEF7A4: 38E000D6  li r7, 0xd6
	ctx.r[7].s64 = 214;
	// 82DEF7A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEF7AC: 4BEF32D5  bl 0x82ce2a80
	ctx.lr = 0x82DEF7B0;
	sub_82CE2A80(ctx, base);
	// 82DEF7B0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEF7B4: 1D5F0005  mulli r10, r31, 5
	ctx.r[10].s64 = ctx.r[31].s64 * 5;
	// 82DEF7B8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DEF7BC: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82DEF7C0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEF7C4: 394AAB80  addi r10, r10, -0x5480
	ctx.r[10].s64 = ctx.r[10].s64 + -21632;
	// 82DEF7C8: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DEF7CC: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DEF7D0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEF7D4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82DEF7D8: 409A0020  bne cr6, 0x82def7f8
	if !ctx.cr[6].eq {
	pc = 0x82DEF7F8; continue 'dispatch;
	}
	// 82DEF7DC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DEF7E0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DEF7E4: 38ABAC70  addi r5, r11, -0x5390
	ctx.r[5].s64 = ctx.r[11].s64 + -21392;
	// 82DEF7E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DEF7EC: 38E000DA  li r7, 0xda
	ctx.r[7].s64 = 218;
	// 82DEF7F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEF7F4: 4BEF328D  bl 0x82ce2a80
	ctx.lr = 0x82DEF7F8;
	sub_82CE2A80(ctx, base);
	// 82DEF7F8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DEF7FC: 483B89B8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEF800 size=28
    let mut pc: u32 = 0x82DEF800;
    'dispatch: loop {
        match pc {
            0x82DEF800 => {
    //   block [0x82DEF800..0x82DEF81C)
	// 82DEF800: 814400E4  lwz r10, 0xe4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DEF804: 3D608331  lis r11, -0x7ccf
	ctx.r[11].s64 = -2093940736;
	// 82DEF808: 554AFFFF  rlwinm. r10, r10, 0x1f, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DEF80C: 396B2438  addi r11, r11, 0x2438
	ctx.r[11].s64 = ctx.r[11].s64 + 9272;
	// 82DEF810: 4182000C  beq 0x82def81c
	if ctx.cr[0].eq {
		sub_82DEF81C(ctx, base);
		return;
	}
	// 82DEF814: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DEF818: 48000014  b 0x82def82c
	sub_82DEF81C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF81C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEF81C size=40
    let mut pc: u32 = 0x82DEF81C;
    'dispatch: loop {
        match pc {
            0x82DEF81C => {
    //   block [0x82DEF81C..0x82DEF844)
	// 82DEF81C: 81440018  lwz r10, 0x18(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEF820: 1D4A0034  mulli r10, r10, 0x34
	ctx.r[10].s64 = ctx.r[10].s64 * 52;
	// 82DEF824: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DEF828: 554AEFFE  rlwinm r10, r10, 0x1d, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82DEF82C: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DEF830: 40820014  bne 0x82def844
	if !ctx.cr[0].eq {
		sub_82DEF844(ctx, base);
		return;
	}
	// 82DEF834: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEF838: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DEF83C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82DEF840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF844(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEF844 size=36
    let mut pc: u32 = 0x82DEF844;
    'dispatch: loop {
        match pc {
            0x82DEF844 => {
    //   block [0x82DEF844..0x82DEF868)
	// 82DEF844: 81440018  lwz r10, 0x18(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEF848: 1D4A0034  mulli r10, r10, 0x34
	ctx.r[10].s64 = ctx.r[10].s64 * 52;
	// 82DEF84C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DEF850: 556AF7FF  rlwinm. r10, r11, 0x1e, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DEF854: 41820014  beq 0x82def868
	if ctx.cr[0].eq {
		sub_82DEF868(ctx, base);
		return;
	}
	// 82DEF858: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEF85C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DEF860: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DEF864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEF868 size=8
    let mut pc: u32 = 0x82DEF868;
    'dispatch: loop {
        match pc {
            0x82DEF868 => {
    //   block [0x82DEF868..0x82DEF870)
	// 82DEF868: 556BFFFF  rlwinm. r11, r11, 0x1f, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF86C: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEF870 size=16
    let mut pc: u32 = 0x82DEF870;
    'dispatch: loop {
        match pc {
            0x82DEF870 => {
    //   block [0x82DEF870..0x82DEF880)
	// 82DEF870: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEF874: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DEF878: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DEF87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEF880 size=28
    let mut pc: u32 = 0x82DEF880;
    'dispatch: loop {
        match pc {
            0x82DEF880 => {
    //   block [0x82DEF880..0x82DEF89C)
	// 82DEF880: 814400E4  lwz r10, 0xe4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DEF884: 3D608331  lis r11, -0x7ccf
	ctx.r[11].s64 = -2093940736;
	// 82DEF888: 554AFFFF  rlwinm. r10, r10, 0x1f, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DEF88C: 396B2438  addi r11, r11, 0x2438
	ctx.r[11].s64 = ctx.r[11].s64 + 9272;
	// 82DEF890: 4182000C  beq 0x82def89c
	if ctx.cr[0].eq {
		sub_82DEF89C(ctx, base);
		return;
	}
	// 82DEF894: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DEF898: 48000014  b 0x82def8ac
	sub_82DEF89C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF89C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEF89C size=40
    let mut pc: u32 = 0x82DEF89C;
    'dispatch: loop {
        match pc {
            0x82DEF89C => {
    //   block [0x82DEF89C..0x82DEF8C4)
	// 82DEF89C: 81440018  lwz r10, 0x18(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEF8A0: 1D4A0034  mulli r10, r10, 0x34
	ctx.r[10].s64 = ctx.r[10].s64 * 52;
	// 82DEF8A4: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DEF8A8: 554AEFFE  rlwinm r10, r10, 0x1d, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82DEF8AC: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DEF8B0: 40820014  bne 0x82def8c4
	if !ctx.cr[0].eq {
		sub_82DEF8C4(ctx, base);
		return;
	}
	// 82DEF8B4: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEF8B8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DEF8BC: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82DEF8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF8C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEF8C4 size=36
    let mut pc: u32 = 0x82DEF8C4;
    'dispatch: loop {
        match pc {
            0x82DEF8C4 => {
    //   block [0x82DEF8C4..0x82DEF8E8)
	// 82DEF8C4: 81440018  lwz r10, 0x18(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEF8C8: 1D4A0034  mulli r10, r10, 0x34
	ctx.r[10].s64 = ctx.r[10].s64 * 52;
	// 82DEF8CC: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DEF8D0: 556AF7FF  rlwinm. r10, r11, 0x1e, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DEF8D4: 41820014  beq 0x82def8e8
	if ctx.cr[0].eq {
		sub_82DEF8E8(ctx, base);
		return;
	}
	// 82DEF8D8: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEF8DC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DEF8E0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DEF8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEF8E8 size=8
    let mut pc: u32 = 0x82DEF8E8;
    'dispatch: loop {
        match pc {
            0x82DEF8E8 => {
    //   block [0x82DEF8E8..0x82DEF8F0)
	// 82DEF8E8: 556BFFFF  rlwinm. r11, r11, 0x1f, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF8EC: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEF8F0 size=16
    let mut pc: u32 = 0x82DEF8F0;
    'dispatch: loop {
        match pc {
            0x82DEF8F0 => {
    //   block [0x82DEF8F0..0x82DEF900)
	// 82DEF8F0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEF8F4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DEF8F8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DEF8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEF900 size=44
    let mut pc: u32 = 0x82DEF900;
    'dispatch: loop {
        match pc {
            0x82DEF900 => {
    //   block [0x82DEF900..0x82DEF92C)
	// 82DEF900: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEF904: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF908: 40990024  ble cr6, 0x82def92c
	if !ctx.cr[6].gt {
		sub_82DEF92C(ctx, base);
		return;
	}
	// 82DEF90C: 816300E4  lwz r11, 0xe4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DEF910: 556BD7FF  rlwinm. r11, r11, 0x1a, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF914: 40820010  bne 0x82def924
	if !ctx.cr[0].eq {
	pc = 0x82DEF924; continue 'dispatch;
	}
	// 82DEF918: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DEF91C: 2F0B001F  cmpwi cr6, r11, 0x1f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 31, &mut ctx.xer);
	// 82DEF920: 409A000C  bne cr6, 0x82def92c
	if !ctx.cr[6].eq {
		sub_82DEF92C(ctx, base);
		return;
	}
	// 82DEF924: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DEF928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF92C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEF92C size=8
    let mut pc: u32 = 0x82DEF92C;
    'dispatch: loop {
        match pc {
            0x82DEF92C => {
    //   block [0x82DEF92C..0x82DEF934)
	// 82DEF92C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEF930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEF938 size=92
    let mut pc: u32 = 0x82DEF938;
    'dispatch: loop {
        match pc {
            0x82DEF938 => {
    //   block [0x82DEF938..0x82DEF994)
	// 82DEF938: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEF93C: 2F0B0068  cmpwi cr6, r11, 0x68
	ctx.cr[6].compare_i32(ctx.r[11].s32, 104, &mut ctx.xer);
	// 82DEF940: 419A0048  beq cr6, 0x82def988
	if ctx.cr[6].eq {
	pc = 0x82DEF988; continue 'dispatch;
	}
	// 82DEF944: 2F0B0069  cmpwi cr6, r11, 0x69
	ctx.cr[6].compare_i32(ctx.r[11].s32, 105, &mut ctx.xer);
	// 82DEF948: 419A0040  beq cr6, 0x82def988
	if ctx.cr[6].eq {
	pc = 0x82DEF988; continue 'dispatch;
	}
	// 82DEF94C: 2F0B006A  cmpwi cr6, r11, 0x6a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 106, &mut ctx.xer);
	// 82DEF950: 419A0038  beq cr6, 0x82def988
	if ctx.cr[6].eq {
	pc = 0x82DEF988; continue 'dispatch;
	}
	// 82DEF954: 2F0B006B  cmpwi cr6, r11, 0x6b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 107, &mut ctx.xer);
	// 82DEF958: 419A0030  beq cr6, 0x82def988
	if ctx.cr[6].eq {
	pc = 0x82DEF988; continue 'dispatch;
	}
	// 82DEF95C: 2F0B006C  cmpwi cr6, r11, 0x6c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 108, &mut ctx.xer);
	// 82DEF960: 419A0028  beq cr6, 0x82def988
	if ctx.cr[6].eq {
	pc = 0x82DEF988; continue 'dispatch;
	}
	// 82DEF964: 2F0B006D  cmpwi cr6, r11, 0x6d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 109, &mut ctx.xer);
	// 82DEF968: 419A0020  beq cr6, 0x82def988
	if ctx.cr[6].eq {
	pc = 0x82DEF988; continue 'dispatch;
	}
	// 82DEF96C: 2F0B006E  cmpwi cr6, r11, 0x6e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 110, &mut ctx.xer);
	// 82DEF970: 419A0018  beq cr6, 0x82def988
	if ctx.cr[6].eq {
	pc = 0x82DEF988; continue 'dispatch;
	}
	// 82DEF974: 2F0B006F  cmpwi cr6, r11, 0x6f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 111, &mut ctx.xer);
	// 82DEF978: 419A0010  beq cr6, 0x82def988
	if ctx.cr[6].eq {
	pc = 0x82DEF988; continue 'dispatch;
	}
	// 82DEF97C: 2F0B0070  cmpwi cr6, r11, 0x70
	ctx.cr[6].compare_i32(ctx.r[11].s32, 112, &mut ctx.xer);
	// 82DEF980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DEF984: 409A0008  bne cr6, 0x82def98c
	if !ctx.cr[6].eq {
	pc = 0x82DEF98C; continue 'dispatch;
	}
	// 82DEF988: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DEF98C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DEF990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEF998 size=52
    let mut pc: u32 = 0x82DEF998;
    'dispatch: loop {
        match pc {
            0x82DEF998 => {
    //   block [0x82DEF998..0x82DEF9CC)
	// 82DEF998: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEF99C: 2F0B0071  cmpwi cr6, r11, 0x71
	ctx.cr[6].compare_i32(ctx.r[11].s32, 113, &mut ctx.xer);
	// 82DEF9A0: 419A0020  beq cr6, 0x82def9c0
	if ctx.cr[6].eq {
	pc = 0x82DEF9C0; continue 'dispatch;
	}
	// 82DEF9A4: 2F0B0072  cmpwi cr6, r11, 0x72
	ctx.cr[6].compare_i32(ctx.r[11].s32, 114, &mut ctx.xer);
	// 82DEF9A8: 419A0018  beq cr6, 0x82def9c0
	if ctx.cr[6].eq {
	pc = 0x82DEF9C0; continue 'dispatch;
	}
	// 82DEF9AC: 2F0B0073  cmpwi cr6, r11, 0x73
	ctx.cr[6].compare_i32(ctx.r[11].s32, 115, &mut ctx.xer);
	// 82DEF9B0: 419A0010  beq cr6, 0x82def9c0
	if ctx.cr[6].eq {
	pc = 0x82DEF9C0; continue 'dispatch;
	}
	// 82DEF9B4: 2F0B0074  cmpwi cr6, r11, 0x74
	ctx.cr[6].compare_i32(ctx.r[11].s32, 116, &mut ctx.xer);
	// 82DEF9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DEF9BC: 409A0008  bne cr6, 0x82def9c4
	if !ctx.cr[6].eq {
	pc = 0x82DEF9C4; continue 'dispatch;
	}
	// 82DEF9C0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DEF9C4: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DEF9C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEF9D0 size=60
    let mut pc: u32 = 0x82DEF9D0;
    'dispatch: loop {
        match pc {
            0x82DEF9D0 => {
    //   block [0x82DEF9D0..0x82DEFA0C)
	// 82DEF9D0: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEF9D4: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 82DEF9D8: 419A0028  beq cr6, 0x82defa00
	if ctx.cr[6].eq {
	pc = 0x82DEFA00; continue 'dispatch;
	}
	// 82DEF9DC: 2F0B0063  cmpwi cr6, r11, 0x63
	ctx.cr[6].compare_i32(ctx.r[11].s32, 99, &mut ctx.xer);
	// 82DEF9E0: 419A0020  beq cr6, 0x82defa00
	if ctx.cr[6].eq {
	pc = 0x82DEFA00; continue 'dispatch;
	}
	// 82DEF9E4: 2F0B0064  cmpwi cr6, r11, 0x64
	ctx.cr[6].compare_i32(ctx.r[11].s32, 100, &mut ctx.xer);
	// 82DEF9E8: 419A0018  beq cr6, 0x82defa00
	if ctx.cr[6].eq {
	pc = 0x82DEFA00; continue 'dispatch;
	}
	// 82DEF9EC: 2F0B008E  cmpwi cr6, r11, 0x8e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 142, &mut ctx.xer);
	// 82DEF9F0: 419A0010  beq cr6, 0x82defa00
	if ctx.cr[6].eq {
	pc = 0x82DEFA00; continue 'dispatch;
	}
	// 82DEF9F4: 2F0B008F  cmpwi cr6, r11, 0x8f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 143, &mut ctx.xer);
	// 82DEF9F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DEF9FC: 409A0008  bne cr6, 0x82defa04
	if !ctx.cr[6].eq {
	pc = 0x82DEFA04; continue 'dispatch;
	}
	// 82DEFA00: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DEFA04: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DEFA08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEFA10 size=264
    let mut pc: u32 = 0x82DEFA10;
    'dispatch: loop {
        match pc {
            0x82DEFA10 => {
    //   block [0x82DEFA10..0x82DEFB18)
	// 82DEFA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEFA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DEFA18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DEFA1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DEFA20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEFA24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DEFA28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DEFA2C: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82DEFA30: 409A00C0  bne cr6, 0x82defaf0
	if !ctx.cr[6].eq {
	pc = 0x82DEFAF0; continue 'dispatch;
	}
	// 82DEFA34: 817E00E4  lwz r11, 0xe4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 82DEFA38: 556BBFFF  rlwinm. r11, r11, 0x17, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEFA3C: 41820024  beq 0x82defa60
	if ctx.cr[0].eq {
	pc = 0x82DEFA60; continue 'dispatch;
	}
	// 82DEFA40: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEFA44: 7F065800  cmpw cr6, r6, r11
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DEFA48: 409A0018  bne cr6, 0x82defa60
	if !ctx.cr[6].eq {
	pc = 0x82DEFA60; continue 'dispatch;
	}
	// 82DEFA4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DEFA50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DEFA54: 4BFEF005  bl 0x82ddea58
	ctx.lr = 0x82DEFA58;
	sub_82DDEA58(ctx, base);
	// 82DEFA58: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEFA5C: 408200A0  bne 0x82defafc
	if !ctx.cr[0].eq {
	pc = 0x82DEFAFC; continue 'dispatch;
	}
	// 82DEFA60: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEFA64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DEFA68: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DEFA6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DEFA70: 4E800421  bctrl
	ctx.lr = 0x82DEFA74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DEFA74: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEFA78: 41820070  beq 0x82defae8
	if ctx.cr[0].eq {
	pc = 0x82DEFAE8; continue 'dispatch;
	}
	// 82DEFA7C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEFA80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEFA84: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DEFA88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DEFA8C: 4E800421  bctrl
	ctx.lr = 0x82DEFA90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DEFA90: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEFA94: 41820010  beq 0x82defaa4
	if ctx.cr[0].eq {
	pc = 0x82DEFAA4; continue 'dispatch;
	}
	// 82DEFA98: 817F00AC  lwz r11, 0xac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DEFA9C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DEFAA0: 419A005C  beq cr6, 0x82defafc
	if ctx.cr[6].eq {
	pc = 0x82DEFAFC; continue 'dispatch;
	}
	// 82DEFAA4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEFAA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEFAAC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEFAB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DEFAB4: 4E800421  bctrl
	ctx.lr = 0x82DEFAB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DEFAB8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEFABC: 4182002C  beq 0x82defae8
	if ctx.cr[0].eq {
	pc = 0x82DEFAE8; continue 'dispatch;
	}
	// 82DEFAC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEFAC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEFAC8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DEFACC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DEFAD0: 4E800421  bctrl
	ctx.lr = 0x82DEFAD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DEFAD4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEFAD8: 41820010  beq 0x82defae8
	if ctx.cr[0].eq {
	pc = 0x82DEFAE8; continue 'dispatch;
	}
	// 82DEFADC: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82DEFAE0: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DEFAE4: 419A0018  beq cr6, 0x82defafc
	if ctx.cr[6].eq {
	pc = 0x82DEFAFC; continue 'dispatch;
	}
	// 82DEFAE8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DEFAEC: 48000014  b 0x82defb00
	pc = 0x82DEFB00; continue 'dispatch;
	// 82DEFAF0: 2F070001  cmpwi cr6, r7, 1
	ctx.cr[6].compare_i32(ctx.r[7].s32, 1, &mut ctx.xer);
	// 82DEFAF4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DEFAF8: 419A0008  beq cr6, 0x82defb00
	if ctx.cr[6].eq {
	pc = 0x82DEFB00; continue 'dispatch;
	}
	// 82DEFAFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEFB00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DEFB04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DEFB08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DEFB0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DEFB10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DEFB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEFB18 size=96
    let mut pc: u32 = 0x82DEFB18;
    'dispatch: loop {
        match pc {
            0x82DEFB18 => {
    //   block [0x82DEFB18..0x82DEFB78)
	// 82DEFB18: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DEFB1C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DEFB20: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82DEFB24: 7D4A202E  lwzx r10, r10, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82DEFB28: 7F0A1800  cmpw cr6, r10, r3
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82DEFB2C: 419A001C  beq cr6, 0x82defb48
	if ctx.cr[6].eq {
	pc = 0x82DEFB48; continue 'dispatch;
	}
	// 82DEFB30: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEFB34: 7D6B202E  lwzx r11, r11, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82DEFB38: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DEFB3C: 7D4A202E  lwzx r10, r10, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82DEFB40: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DEFB44: 409AFFEC  bne cr6, 0x82defb30
	if !ctx.cr[6].eq {
	pc = 0x82DEFB30; continue 'dispatch;
	}
	// 82DEFB48: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DEFB4C: 419A0020  beq cr6, 0x82defb6c
	if ctx.cr[6].eq {
	pc = 0x82DEFB6C; continue 'dispatch;
	}
	// 82DEFB50: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82DEFB54: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEFB58: 550A103A  slwi r10, r8, 2
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DEFB5C: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DEFB60: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82DEFB64: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DEFB68: 409AFFEC  bne cr6, 0x82defb54
	if !ctx.cr[6].eq {
	pc = 0x82DEFB54; continue 'dispatch;
	}
	// 82DEFB6C: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DEFB70: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DEFB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEFB78 size=12
    let mut pc: u32 = 0x82DEFB78;
    'dispatch: loop {
        match pc {
            0x82DEFB78 => {
    //   block [0x82DEFB78..0x82DEFB84)
	// 82DEFB78: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEFB7C: 7C6B292E  stwx r3, r11, r5
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32), ctx.r[3].u32) };
	// 82DEFB80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEFB88 size=48
    let mut pc: u32 = 0x82DEFB88;
    'dispatch: loop {
        match pc {
            0x82DEFB88 => {
    //   block [0x82DEFB88..0x82DEFBB8)
	// 82DEFB88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEFB8C: 556B0253  rlwinm. r11, r11, 0, 9, 9
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEFB90: 4182004C  beq 0x82defbdc
	if ctx.cr[0].eq {
		sub_82DEFBDC(ctx, base);
		return;
	}
	// 82DEFB94: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 82DEFB98: 41980038  blt cr6, 0x82defbd0
	if ctx.cr[6].lt {
		sub_82DEFBD0(ctx, base);
		return;
	}
	// 82DEFB9C: 419A0028  beq cr6, 0x82defbc4
	if ctx.cr[6].eq {
		sub_82DEFBC4(ctx, base);
		return;
	}
	// 82DEFBA0: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 82DEFBA4: 41980014  blt cr6, 0x82defbb8
	if ctx.cr[6].lt {
		sub_82DEFBB8(ctx, base);
		return;
	}
	// 82DEFBA8: 409A0034  bne cr6, 0x82defbdc
	if !ctx.cr[6].eq {
		sub_82DEFBDC(ctx, base);
		return;
	}
	// 82DEFBAC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEFBB0: 5563D7BE  rlwinm r3, r11, 0x1a, 0x1e, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82DEFBB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEFBB8 size=12
    let mut pc: u32 = 0x82DEFBB8;
    'dispatch: loop {
        match pc {
            0x82DEFBB8 => {
    //   block [0x82DEFBB8..0x82DEFBC4)
	// 82DEFBB8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEFBBC: 5563E7BE  rlwinm r3, r11, 0x1c, 0x1e, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82DEFBC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFBC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEFBC4 size=12
    let mut pc: u32 = 0x82DEFBC4;
    'dispatch: loop {
        match pc {
            0x82DEFBC4 => {
    //   block [0x82DEFBC4..0x82DEFBD0)
	// 82DEFBC4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEFBC8: 5563F7BE  rlwinm r3, r11, 0x1e, 0x1e, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82DEFBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEFBD0 size=12
    let mut pc: u32 = 0x82DEFBD0;
    'dispatch: loop {
        match pc {
            0x82DEFBD0 => {
    //   block [0x82DEFBD0..0x82DEFBDC)
	// 82DEFBD0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEFBD4: 556307BE  clrlwi r3, r11, 0x1e
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82DEFBD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFBDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEFBDC size=8
    let mut pc: u32 = 0x82DEFBDC;
    'dispatch: loop {
        match pc {
            0x82DEFBDC => {
    //   block [0x82DEFBDC..0x82DEFBE4)
	// 82DEFBDC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DEFBE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEFBE8 size=52
    let mut pc: u32 = 0x82DEFBE8;
    'dispatch: loop {
        match pc {
            0x82DEFBE8 => {
    //   block [0x82DEFBE8..0x82DEFC1C)
	// 82DEFBE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEFBEC: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 82DEFBF0: 556B0253  rlwinm. r11, r11, 0, 9, 9
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEFBF4: 4182004C  beq 0x82defc40
	if ctx.cr[0].eq {
		sub_82DEFC40(ctx, base);
		return;
	}
	// 82DEFBF8: 4198003C  blt cr6, 0x82defc34
	if ctx.cr[6].lt {
		sub_82DEFC34(ctx, base);
		return;
	}
	// 82DEFBFC: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 82DEFC00: 419A0028  beq cr6, 0x82defc28
	if ctx.cr[6].eq {
		sub_82DEFC28(ctx, base);
		return;
	}
	// 82DEFC04: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 82DEFC08: 41980014  blt cr6, 0x82defc1c
	if ctx.cr[6].lt {
		sub_82DEFC1C(ctx, base);
		return;
	}
	// 82DEFC0C: 409A004C  bne cr6, 0x82defc58
	if !ctx.cr[6].eq {
		sub_82DEFC40(ctx, base);
		return;
	}
	// 82DEFC10: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEFC14: 5563A77E  rlwinm r3, r11, 0x14, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82DEFC18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFC1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEFC1C size=12
    let mut pc: u32 = 0x82DEFC1C;
    'dispatch: loop {
        match pc {
            0x82DEFC1C => {
    //   block [0x82DEFC1C..0x82DEFC28)
	// 82DEFC1C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEFC20: 5563C77E  rlwinm r3, r11, 0x18, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DEFC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEFC28 size=12
    let mut pc: u32 = 0x82DEFC28;
    'dispatch: loop {
        match pc {
            0x82DEFC28 => {
    //   block [0x82DEFC28..0x82DEFC34)
	// 82DEFC28: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEFC2C: 5563E77E  rlwinm r3, r11, 0x1c, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82DEFC30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFC34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEFC34 size=12
    let mut pc: u32 = 0x82DEFC34;
    'dispatch: loop {
        match pc {
            0x82DEFC34 => {
    //   block [0x82DEFC34..0x82DEFC40)
	// 82DEFC34: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEFC38: 5563077E  clrlwi r3, r11, 0x1d
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82DEFC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEFC40 size=32
    let mut pc: u32 = 0x82DEFC40;
    'dispatch: loop {
        match pc {
            0x82DEFC40 => {
    //   block [0x82DEFC40..0x82DEFC60)
	// 82DEFC40: 41980038  blt cr6, 0x82defc78
	if ctx.cr[6].lt {
		sub_82DEFC78(ctx, base);
		return;
	}
	// 82DEFC44: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 82DEFC48: 419A0028  beq cr6, 0x82defc70
	if ctx.cr[6].eq {
		sub_82DEFC70(ctx, base);
		return;
	}
	// 82DEFC4C: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 82DEFC50: 41980018  blt cr6, 0x82defc68
	if ctx.cr[6].lt {
		sub_82DEFC68(ctx, base);
		return;
	}
	// 82DEFC54: 419A000C  beq cr6, 0x82defc60
	if ctx.cr[6].eq {
		sub_82DEFC60(ctx, base);
		return;
	}
	// 82DEFC58: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DEFC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEFC60 size=8
    let mut pc: u32 = 0x82DEFC60;
    'dispatch: loop {
        match pc {
            0x82DEFC60 => {
    //   block [0x82DEFC60..0x82DEFC68)
	// 82DEFC60: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82DEFC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEFC68 size=8
    let mut pc: u32 = 0x82DEFC68;
    'dispatch: loop {
        match pc {
            0x82DEFC68 => {
    //   block [0x82DEFC68..0x82DEFC70)
	// 82DEFC68: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82DEFC6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEFC70 size=8
    let mut pc: u32 = 0x82DEFC70;
    'dispatch: loop {
        match pc {
            0x82DEFC70 => {
    //   block [0x82DEFC70..0x82DEFC78)
	// 82DEFC70: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DEFC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEFC78 size=8
    let mut pc: u32 = 0x82DEFC78;
    'dispatch: loop {
        match pc {
            0x82DEFC78 => {
    //   block [0x82DEFC78..0x82DEFC80)
	// 82DEFC78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEFC7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEFC80 size=24
    let mut pc: u32 = 0x82DEFC80;
    'dispatch: loop {
        match pc {
            0x82DEFC80 => {
    //   block [0x82DEFC80..0x82DEFC98)
	// 82DEFC80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEFC84: 556B0253  rlwinm. r11, r11, 0, 9, 9
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEFC88: 41820010  beq 0x82defc98
	if ctx.cr[0].eq {
		sub_82DEFC98(ctx, base);
		return;
	}
	// 82DEFC8C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEFC90: 71638888  andi. r3, r11, 0x8888
	ctx.r[3].u64 = ctx.r[11].u64 & 34952;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DEFC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEFC98 size=8
    let mut pc: u32 = 0x82DEFC98;
    'dispatch: loop {
        match pc {
            0x82DEFC98 => {
    //   block [0x82DEFC98..0x82DEFCA0)
	// 82DEFC98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEFC9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEFCA0 size=200
    let mut pc: u32 = 0x82DEFCA0;
    'dispatch: loop {
        match pc {
            0x82DEFCA0 => {
    //   block [0x82DEFCA0..0x82DEFD68)
	// 82DEFCA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEFCA4: 483B84C1  bl 0x831a8164
	ctx.lr = 0x82DEFCA8;
	sub_831A8130(ctx, base);
	// 82DEFCA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEFCAC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DEFCB0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEFCB4: 556B0253  rlwinm. r11, r11, 0, 9, 9
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEFCB8: 4182009C  beq 0x82defd54
	if ctx.cr[0].eq {
	pc = 0x82DEFD54; continue 'dispatch;
	}
	// 82DEFCBC: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 82DEFCC0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82DEFCC4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DEFCC8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DEFCCC: 3BE95E90  addi r31, r9, 0x5e90
	ctx.r[31].s64 = ctx.r[9].s64 + 24208;
	// 82DEFCD0: 3BCA9FF4  addi r30, r10, -0x600c
	ctx.r[30].s64 = ctx.r[10].s64 + -24588;
	// 82DEFCD4: 3BABADE8  addi r29, r11, -0x5218
	ctx.r[29].s64 = ctx.r[11].s64 + -21016;
	// 82DEFCD8: 2B1B0001  cmplwi cr6, r27, 1
	ctx.cr[6].compare_u32(ctx.r[27].u32, 1 as u32, &mut ctx.xer);
	// 82DEFCDC: 41980054  blt cr6, 0x82defd30
	if ctx.cr[6].lt {
	pc = 0x82DEFD30; continue 'dispatch;
	}
	// 82DEFCE0: 419A0044  beq cr6, 0x82defd24
	if ctx.cr[6].eq {
	pc = 0x82DEFD24; continue 'dispatch;
	}
	// 82DEFCE4: 2B1B0003  cmplwi cr6, r27, 3
	ctx.cr[6].compare_u32(ctx.r[27].u32, 3 as u32, &mut ctx.xer);
	// 82DEFCE8: 41980030  blt cr6, 0x82defd18
	if ctx.cr[6].lt {
	pc = 0x82DEFD18; continue 'dispatch;
	}
	// 82DEFCEC: 419A0020  beq cr6, 0x82defd0c
	if ctx.cr[6].eq {
	pc = 0x82DEFD0C; continue 'dispatch;
	}
	// 82DEFCF0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DEFCF4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DEFCF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DEFCFC: 38E000A6  li r7, 0xa6
	ctx.r[7].s64 = 166;
	// 82DEFD00: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEFD04: 4BEF2D7D  bl 0x82ce2a80
	ctx.lr = 0x82DEFD08;
	sub_82CE2A80(ctx, base);
	// 82DEFD08: 48000040  b 0x82defd48
	pc = 0x82DEFD48; continue 'dispatch;
	// 82DEFD0C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEFD10: 556BD7BE  rlwinm r11, r11, 0x1a, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82DEFD14: 48000024  b 0x82defd38
	pc = 0x82DEFD38; continue 'dispatch;
	// 82DEFD18: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEFD1C: 556BE7BE  rlwinm r11, r11, 0x1c, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82DEFD20: 48000018  b 0x82defd38
	pc = 0x82DEFD38; continue 'dispatch;
	// 82DEFD24: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEFD28: 556BF7BE  rlwinm r11, r11, 0x1e, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82DEFD2C: 4800000C  b 0x82defd38
	pc = 0x82DEFD38; continue 'dispatch;
	// 82DEFD30: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEFD34: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82DEFD38: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82DEFD3C: 419A0024  beq cr6, 0x82defd60
	if ctx.cr[6].eq {
	pc = 0x82DEFD60; continue 'dispatch;
	}
	// 82DEFD40: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82DEFD44: 419A001C  beq cr6, 0x82defd60
	if ctx.cr[6].eq {
	pc = 0x82DEFD60; continue 'dispatch;
	}
	// 82DEFD48: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82DEFD4C: 2F1B0004  cmpwi cr6, r27, 4
	ctx.cr[6].compare_i32(ctx.r[27].s32, 4, &mut ctx.xer);
	// 82DEFD50: 4198FF88  blt cr6, 0x82defcd8
	if ctx.cr[6].lt {
	pc = 0x82DEFCD8; continue 'dispatch;
	}
	// 82DEFD54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEFD58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DEFD5C: 483B8458  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82DEFD60: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DEFD64: 4BFFFFF4  b 0x82defd58
	pc = 0x82DEFD58; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEFD68 size=88
    let mut pc: u32 = 0x82DEFD68;
    'dispatch: loop {
        match pc {
            0x82DEFD68 => {
    //   block [0x82DEFD68..0x82DEFDC0)
	// 82DEFD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEFD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DEFD70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DEFD74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEFD78: 39640007  addi r11, r4, 7
	ctx.r[11].s64 = ctx.r[4].s64 + 7;
	// 82DEFD7C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEFD80: 7FEB182E  lwzx r31, r11, r3
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DEFD84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEFD88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEFD8C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEFD90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DEFD94: 4E800421  bctrl
	ctx.lr = 0x82DEFD98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DEFD98: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEFD9C: 4182000C  beq 0x82defda8
	if ctx.cr[0].eq {
	pc = 0x82DEFDA8; continue 'dispatch;
	}
	// 82DEFDA0: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DEFDA4: 48000008  b 0x82defdac
	pc = 0x82DEFDAC; continue 'dispatch;
	// 82DEFDA8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DEFDAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DEFDB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DEFDB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DEFDB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DEFDBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEFDC0 size=136
    let mut pc: u32 = 0x82DEFDC0;
    'dispatch: loop {
        match pc {
            0x82DEFDC0 => {
    //   block [0x82DEFDC0..0x82DEFE48)
	// 82DEFDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEFDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DEFDC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DEFDCC: 9421FE10  stwu r1, -0x1f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-496 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEFDD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DEFDD4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DEFDD8: 419A0058  beq cr6, 0x82defe30
	if ctx.cr[6].eq {
	pc = 0x82DEFE30; continue 'dispatch;
	}
	// 82DEFDDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEFDE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEFDE4: 419A004C  beq cr6, 0x82defe30
	if ctx.cr[6].eq {
	pc = 0x82DEFE30; continue 'dispatch;
	}
	// 82DEFDE8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEFDEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEFDF0: 419A0040  beq cr6, 0x82defe30
	if ctx.cr[6].eq {
	pc = 0x82DEFE30; continue 'dispatch;
	}
	// 82DEFDF4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DEFDF8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82DEFDFC: 4800025D  bl 0x82df0058
	ctx.lr = 0x82DEFE00;
	sub_82DF0058(ctx, base);
	// 82DEFE00: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DEFE04: 40820030  bne 0x82defe34
	if !ctx.cr[0].eq {
	pc = 0x82DEFE34; continue 'dispatch;
	}
	// 82DEFE08: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DEFE0C: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEFE10: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEFE14: 386B5548  addi r3, r11, 0x5548
	ctx.r[3].s64 = ctx.r[11].s64 + 21832;
	// 82DEFE18: 480000D1  bl 0x82defee8
	ctx.lr = 0x82DEFE1C;
	sub_82DEFEE8(ctx, base);
	// 82DEFE1C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DEFE20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEFE24: 40800010  bge 0x82defe34
	if !ctx.cr[0].lt {
	pc = 0x82DEFE34; continue 'dispatch;
	}
	// 82DEFE28: 3860065B  li r3, 0x65b
	ctx.r[3].s64 = 1627;
	// 82DEFE2C: 48000008  b 0x82defe34
	pc = 0x82DEFE34; continue 'dispatch;
	// 82DEFE30: 38600057  li r3, 0x57
	ctx.r[3].s64 = 87;
	// 82DEFE34: 382101F0  addi r1, r1, 0x1f0
	ctx.r[1].s64 = ctx.r[1].s64 + 496;
	// 82DEFE38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DEFE3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DEFE40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DEFE44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEFE48 size=12
    let mut pc: u32 = 0x82DEFE48;
    'dispatch: loop {
        match pc {
            0x82DEFE48 => {
    //   block [0x82DEFE48..0x82DEFE54)
	// 82DEFE48: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82DEFE4C: 386B9D6C  addi r3, r11, -0x6294
	ctx.r[3].s64 = ctx.r[11].s64 + -25236;
	// 82DEFE50: 4BFFFF70  b 0x82defdc0
	sub_82DEFDC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEFE58 size=36
    let mut pc: u32 = 0x82DEFE58;
    'dispatch: loop {
        match pc {
            0x82DEFE58 => {
    //   block [0x82DEFE58..0x82DEFE7C)
	// 82DEFE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEFE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DEFE60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEFE64: 48000205  bl 0x82df0068
	ctx.lr = 0x82DEFE68;
	sub_82DF0068(ctx, base);
	// 82DEFE68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEFE6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DEFE70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DEFE74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DEFE78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEFE80 size=100
    let mut pc: u32 = 0x82DEFE80;
    'dispatch: loop {
        match pc {
            0x82DEFE80 => {
    //   block [0x82DEFE80..0x82DEFEE4)
	// 82DEFE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEFE84: 483B82D9  bl 0x831a815c
	ctx.lr = 0x82DEFE88;
	sub_831A8130(ctx, base);
	// 82DEFE88: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEFE8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DEFE90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DEFE94: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DEFE98: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DEFE9C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82DEFEA0: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82DEFEA4: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 82DEFEA8: 4BDDD9B1  bl 0x82bcd858
	ctx.lr = 0x82DEFEAC;
	sub_82BCD858(ctx, base);
	// 82DEFEAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DEFEB0: 4082002C  bne 0x82defedc
	if !ctx.cr[0].eq {
	pc = 0x82DEFEDC; continue 'dispatch;
	}
	// 82DEFEB4: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82DEFEB8: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 82DEFEBC: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82DEFEC0: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82DEFEC4: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82DEFEC8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DEFECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DEFED0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DEFED4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DEFED8: 48452E25  bl 0x83242cfc
	ctx.lr = 0x82DEFEDC;
	// extern call 0x83242CFC → crate::xam::XamUserReadProfileSettings
	crate::xam::XamUserReadProfileSettings(ctx, base);
	// 82DEFEDC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DEFEE0: 483B82CC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEFEE8 size=196
    let mut pc: u32 = 0x82DEFEE8;
    'dispatch: loop {
        match pc {
            0x82DEFEE8 => {
    //   block [0x82DEFEE8..0x82DEFFAC)
	// 82DEFEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEFEEC: 483B8281  bl 0x831a816c
	ctx.lr = 0x82DEFEF0;
	sub_831A8130(ctx, base);
	// 82DEFEF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEFEF4: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82DEFEF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DEFEFC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DEFF00: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DEFF04: 483B860D  bl 0x831a8510
	ctx.lr = 0x82DEFF08;
	sub_831A8510(ctx, base);
	// 82DEFF08: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEFF0C: 393E002C  addi r9, r30, 0x2c
	ctx.r[9].s64 = ctx.r[30].s64 + 44;
	// 82DEFF10: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEFF14: 41820010  beq 0x82deff24
	if ctx.cr[0].eq {
	pc = 0x82DEFF24; continue 'dispatch;
	}
	// 82DEFF18: 3FA08000  lis r29, -0x8000
	ctx.r[29].s64 = -2147483648;
	// 82DEFF1C: 63BD4005  ori r29, r29, 0x4005
	ctx.r[29].u64 = ctx.r[29].u64 | 16389;
	// 82DEFF20: 48000080  b 0x82deffa0
	pc = 0x82DEFFA0; continue 'dispatch;
	// 82DEFF24: A11F0026  lhz r8, 0x26(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(38 as u32) ) } as u64;
	// 82DEFF28: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DEFF2C: 4182000C  beq 0x82deff38
	if ctx.cr[0].eq {
	pc = 0x82DEFF38; continue 'dispatch;
	}
	// 82DEFF30: 913F003C  stw r9, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[9].u32 ) };
	// 82DEFF34: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82DEFF38: A15F0028  lhz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DEFF3C: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEFF40: 554B103E  rotlwi r11, r10, 2
	ctx.r[11].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82DEFF44: A0DF002A  lhz r6, 0x2a(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(42 as u32) ) } as u64;
	// 82DEFF48: 913F002C  stw r9, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[9].u32 ) };
	// 82DEFF4C: 7CEB3850  subf r7, r11, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[11].s64;
	// 82DEFF50: A0BF001A  lhz r5, 0x1a(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(26 as u32) ) } as u64;
	// 82DEFF54: 7D4651D6  mullw r10, r6, r10
	ctx.r[10].s64 = (ctx.r[6].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82DEFF58: A0DF0018  lhz r6, 0x18(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DEFF5C: A09F0020  lhz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DEFF60: 7D083850  subf r8, r8, r7
	ctx.r[8].s64 = ctx.r[7].s64 - ctx.r[8].s64;
	// 82DEFF64: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DEFF68: 7D2A4050  subf r9, r10, r8
	ctx.r[9].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 82DEFF6C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82DEFF70: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DEFF74: 3949FFD4  addi r10, r9, -0x2c
	ctx.r[10].s64 = ctx.r[9].s64 + -44;
	// 82DEFF78: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82DEFF7C: 7D2531D6  mullw r9, r5, r6
	ctx.r[9].s64 = (ctx.r[5].s32 as i64) * (ctx.r[6].s32 as i64);
	// 82DEFF80: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82DEFF84: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DEFF88: 548A083E  rotlwi r10, r4, 1
	ctx.r[10].u64 = ((ctx.r[4].u32).rotate_left(1)) as u64;
	// 82DEFF8C: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82DEFF90: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DEFF94: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DEFF98: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82DEFF9C: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82DEFFA0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DEFFA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DEFFA8: 483B8214  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEFFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DEFFB0 size=168
    let mut pc: u32 = 0x82DEFFB0;
    'dispatch: loop {
        match pc {
            0x82DEFFB0 => {
    //   block [0x82DEFFB0..0x82DF0058)
	// 82DEFFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEFFB4: 483B81B9  bl 0x831a816c
	ctx.lr = 0x82DEFFB8;
	sub_831A8130(ctx, base);
	// 82DEFFB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEFFBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DEFFC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DEFFC4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DEFFC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DEFFCC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DEFFD0: 4845260D  bl 0x832425dc
	ctx.lr = 0x82DEFFD4;
	// extern call 0x832425DC → crate::xam::XamGetSystemVersion
	crate::xam::XamGetSystemVersion(ctx, base);
	// 82DEFFD4: 3D60200A  lis r11, 0x200a
	ctx.r[11].s64 = 537526272;
	// 82DEFFD8: 616B3200  ori r11, r11, 0x3200
	ctx.r[11].u64 = ctx.r[11].u64 | 12800;
	// 82DEFFDC: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DEFFE0: 41980040  blt cr6, 0x82df0020
	if ctx.cr[6].lt {
	pc = 0x82DF0020; continue 'dispatch;
	}
	// 82DEFFE4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82DEFFE8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82DEFFEC: 386BB740  addi r3, r11, -0x48c0
	ctx.r[3].s64 = ctx.r[11].s64 + -18624;
	// 82DEFFF0: 48452E3D  bl 0x83242e2c
	ctx.lr = 0x82DEFFF4;
	// extern call 0x83242E2C → crate::xboxkrnl::XexGetModuleHandle
	crate::xboxkrnl::XexGetModuleHandle(ctx, base);
	// 82DEFFF4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DEFFF8: 41800028  blt 0x82df0020
	if ctx.cr[0].lt {
	pc = 0x82DF0020; continue 'dispatch;
	}
	// 82DEFFFC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DF0000: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DF0004: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82DF0008: 48452825  bl 0x8324282c
	ctx.lr = 0x82DF000C;
	// extern call 0x8324282C → crate::xboxkrnl::XexGetProcedureAddress
	crate::xboxkrnl::XexGetProcedureAddress(ctx, base);
	// 82DF000C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DF0010: 40800010  bge 0x82df0020
	if !ctx.cr[0].lt {
	pc = 0x82DF0020; continue 'dispatch;
	}
	// 82DF0014: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF0018: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DF001C: 48000008  b 0x82df0024
	pc = 0x82DF0024; continue 'dispatch;
	// 82DF0020: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF0024: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF0028: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF002C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF0030: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF0034: 419A0018  beq cr6, 0x82df004c
	if ctx.cr[6].eq {
	pc = 0x82DF004C; continue 'dispatch;
	}
	// 82DF0038: 3CC0201E  lis r6, 0x201e
	ctx.r[6].s64 = 538836992;
	// 82DF003C: 60C66000  ori r6, r6, 0x6000
	ctx.r[6].u64 = ctx.r[6].u64 | 24576;
	// 82DF0040: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF0044: 4E800421  bctrl
	ctx.lr = 0x82DF0048;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF0048: 48000008  b 0x82df0050
	pc = 0x82DF0050; continue 'dispatch;
	// 82DF004C: 48452CC1  bl 0x83242d0c
	ctx.lr = 0x82DF0050;
	// extern call 0x83242D0C → crate::xam::NetDll_WSAStartup
	crate::xam::NetDll_WSAStartup(ctx, base);
	// 82DF0050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF0054: 483B8168  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0058 size=16
    let mut pc: u32 = 0x82DF0058;
    'dispatch: loop {
        match pc {
            0x82DF0058 => {
    //   block [0x82DF0058..0x82DF0068)
	// 82DF0058: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DF005C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF0060: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF0064: 4BFFFF4C  b 0x82deffb0
	sub_82DEFFB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0068 size=8
    let mut pc: u32 = 0x82DF0068;
    'dispatch: loop {
        match pc {
            0x82DF0068 => {
    //   block [0x82DF0068..0x82DF0070)
	// 82DF0068: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF006C: 48452CB0  b 0x83242d1c
	sub_83242D1C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0070 size=20
    let mut pc: u32 = 0x82DF0070;
    'dispatch: loop {
        match pc {
            0x82DF0070 => {
    //   block [0x82DF0070..0x82DF0084)
	// 82DF0070: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82DF0074: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DF0078: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF007C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF0080: 48452CAC  b 0x83242d2c
	sub_83242D2C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0088 size=12
    let mut pc: u32 = 0x82DF0088;
    'dispatch: loop {
        match pc {
            0x82DF0088 => {
    //   block [0x82DF0088..0x82DF0094)
	// 82DF0088: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF008C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF0090: 48452CAC  b 0x83242d3c
	sub_83242D3C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0098 size=28
    let mut pc: u32 = 0x82DF0098;
    'dispatch: loop {
        match pc {
            0x82DF0098 => {
    //   block [0x82DF0098..0x82DF00B4)
	// 82DF0098: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82DF009C: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82DF00A0: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82DF00A4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DF00A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF00AC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF00B0: 48452C9C  b 0x83242d4c
	sub_83242D4C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF00B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF00B8 size=20
    let mut pc: u32 = 0x82DF00B8;
    'dispatch: loop {
        match pc {
            0x82DF00B8 => {
    //   block [0x82DF00B8..0x82DF00CC)
	// 82DF00B8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82DF00BC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DF00C0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF00C4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF00C8: 48452C94  b 0x83242d5c
	sub_83242D5C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF00D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF00D0 size=20
    let mut pc: u32 = 0x82DF00D0;
    'dispatch: loop {
        match pc {
            0x82DF00D0 => {
    //   block [0x82DF00D0..0x82DF00E4)
	// 82DF00D0: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82DF00D4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DF00D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF00DC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF00E0: 48452C8C  b 0x83242d6c
	sub_83242D6C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF00E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF00E8 size=16
    let mut pc: u32 = 0x82DF00E8;
    'dispatch: loop {
        match pc {
            0x82DF00E8 => {
    //   block [0x82DF00E8..0x82DF00F8)
	// 82DF00E8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DF00EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF00F0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF00F4: 48452C88  b 0x83242d7c
	sub_83242D7C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF00F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF00F8 size=20
    let mut pc: u32 = 0x82DF00F8;
    'dispatch: loop {
        match pc {
            0x82DF00F8 => {
    //   block [0x82DF00F8..0x82DF010C)
	// 82DF00F8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82DF00FC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DF0100: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF0104: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF0108: 48452C84  b 0x83242d8c
	sub_83242D8C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0110 size=28
    let mut pc: u32 = 0x82DF0110;
    'dispatch: loop {
        match pc {
            0x82DF0110 => {
    //   block [0x82DF0110..0x82DF012C)
	// 82DF0110: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82DF0114: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82DF0118: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82DF011C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DF0120: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF0124: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF0128: 48452C74  b 0x83242d9c
	sub_83242D9C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0130 size=24
    let mut pc: u32 = 0x82DF0130;
    'dispatch: loop {
        match pc {
            0x82DF0130 => {
    //   block [0x82DF0130..0x82DF0148)
	// 82DF0130: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82DF0134: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82DF0138: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DF013C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF0140: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF0144: 48452C68  b 0x83242dac
	sub_83242DAC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0148 size=24
    let mut pc: u32 = 0x82DF0148;
    'dispatch: loop {
        match pc {
            0x82DF0148 => {
    //   block [0x82DF0148..0x82DF0160)
	// 82DF0148: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82DF014C: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82DF0150: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DF0154: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF0158: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF015C: 48452C60  b 0x83242dbc
	sub_83242DBC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0160 size=4
    let mut pc: u32 = 0x82DF0160;
    'dispatch: loop {
        match pc {
            0x82DF0160 => {
    //   block [0x82DF0160..0x82DF0164)
	// 82DF0160: 48452C6C  b 0x83242dcc
	sub_83242DCC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0168 size=4
    let mut pc: u32 = 0x82DF0168;
    'dispatch: loop {
        match pc {
            0x82DF0168 => {
    //   block [0x82DF0168..0x82DF016C)
	// 82DF0168: 48452C74  b 0x83242ddc
	sub_83242DDC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0170 size=168
    let mut pc: u32 = 0x82DF0170;
    'dispatch: loop {
        match pc {
            0x82DF0170 => {
    //   block [0x82DF0170..0x82DF0218)
	// 82DF0170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0178: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF017C: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82DF0180: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DF0184: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82DF0188: 90A10060  stw r5, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[5].u32 ) };
	// 82DF018C: 409A0024  bne cr6, 0x82df01b0
	if !ctx.cr[6].eq {
	pc = 0x82DF01B0; continue 'dispatch;
	}
	// 82DF0190: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DF0194: 3D400008  lis r10, 8
	ctx.r[10].s64 = 524288;
	// 82DF0198: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DF019C: 614A0002  ori r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u64 | 2;
	// 82DF01A0: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82DF01A4: F92B0000  std r9, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DF01A8: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DF01AC: 48000008  b 0x82df01b4
	pc = 0x82DF01B4; continue 'dispatch;
	// 82DF01B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DF01B4: 3C800007  lis r4, 7
	ctx.r[4].s64 = 458752;
	// 82DF01B8: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82DF01BC: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82DF01C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF01C4: 6084001A  ori r4, r4, 0x1a
	ctx.r[4].u64 = ctx.r[4].u64 | 26;
	// 82DF01C8: 386000FA  li r3, 0xfa
	ctx.r[3].s64 = 250;
	// 82DF01CC: 48452C21  bl 0x83242dec
	ctx.lr = 0x82DF01D0;
	// extern call 0x83242DEC → crate::xboxkrnl::XMsgStartIORequestEx
	crate::xboxkrnl::XMsgStartIORequestEx(ctx, base);
	// 82DF01D0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DF01D4: 4180001C  blt 0x82df01f0
	if ctx.cr[0].lt {
	pc = 0x82DF01F0; continue 'dispatch;
	}
	// 82DF01D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF01DC: 483D4ECD  bl 0x831c50a8
	ctx.lr = 0x82DF01E0;
	sub_831C50A8(ctx, base);
	// 82DF01E0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DF01E4: 4180000C  blt 0x82df01f0
	if ctx.cr[0].lt {
	pc = 0x82DF01F0; continue 'dispatch;
	}
	// 82DF01E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF01EC: 4800001C  b 0x82df0208
	pc = 0x82DF0208; continue 'dispatch;
	// 82DF01F0: 546B00DE  rlwinm r11, r3, 0, 3, 0xf
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82DF01F4: 3D400007  lis r10, 7
	ctx.r[10].s64 = 458752;
	// 82DF01F8: 5463043E  clrlwi r3, r3, 0x10
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82DF01FC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DF0200: 419A0008  beq cr6, 0x82df0208
	if ctx.cr[6].eq {
	pc = 0x82DF0208; continue 'dispatch;
	}
	// 82DF0204: 3860065B  li r3, 0x65b
	ctx.r[3].s64 = 1627;
	// 82DF0208: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF020C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0218 size=104
    let mut pc: u32 = 0x82DF0218;
    'dispatch: loop {
        match pc {
            0x82DF0218 => {
    //   block [0x82DF0218..0x82DF0280)
	// 82DF0218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF021C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0220: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0224: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 82DF0228: 3C800007  lis r4, 7
	ctx.r[4].s64 = 458752;
	// 82DF022C: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82DF0230: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DF0234: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82DF0238: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DF023C: 6084001B  ori r4, r4, 0x1b
	ctx.r[4].u64 = ctx.r[4].u64 | 27;
	// 82DF0240: 386000FA  li r3, 0xfa
	ctx.r[3].s64 = 250;
	// 82DF0244: 48452AA9  bl 0x83242cec
	ctx.lr = 0x82DF0248;
	// extern call 0x83242CEC → crate::xboxkrnl::XMsgInProcessCall
	crate::xboxkrnl::XMsgInProcessCall(ctx, base);
	// 82DF0248: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DF024C: 4180000C  blt 0x82df0258
	if ctx.cr[0].lt {
	pc = 0x82DF0258; continue 'dispatch;
	}
	// 82DF0250: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF0254: 4800001C  b 0x82df0270
	pc = 0x82DF0270; continue 'dispatch;
	// 82DF0258: 546B00DE  rlwinm r11, r3, 0, 3, 0xf
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82DF025C: 3D400007  lis r10, 7
	ctx.r[10].s64 = 458752;
	// 82DF0260: 5463043E  clrlwi r3, r3, 0x10
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82DF0264: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DF0268: 419A0008  beq cr6, 0x82df0270
	if ctx.cr[6].eq {
	pc = 0x82DF0270; continue 'dispatch;
	}
	// 82DF026C: 3860065B  li r3, 0x65b
	ctx.r[3].s64 = 1627;
	// 82DF0270: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF0274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF027C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0280 size=68
    let mut pc: u32 = 0x82DF0280;
    'dispatch: loop {
        match pc {
            0x82DF0280 => {
    //   block [0x82DF0280..0x82DF02C4)
	// 82DF0280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0288: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF028C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0290: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF0294: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF0298: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DF029C: 396BAE40  addi r11, r11, -0x51c0
	ctx.r[11].s64 = ctx.r[11].s64 + -20928;
	// 82DF02A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF02A4: 41820008  beq 0x82df02ac
	if ctx.cr[0].eq {
	pc = 0x82DF02AC; continue 'dispatch;
	}
	// 82DF02A8: 4B4CFFC1  bl 0x822c0268
	ctx.lr = 0x82DF02AC;
	sub_822C0268(ctx, base);
	// 82DF02AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF02B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF02B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF02B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF02BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF02C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF02C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF02C8 size=100
    let mut pc: u32 = 0x82DF02C8;
    'dispatch: loop {
        match pc {
            0x82DF02C8 => {
    //   block [0x82DF02C8..0x82DF032C)
	// 82DF02C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF02CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF02D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF02D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF02D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF02DC: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82DF02E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF02E4: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82DF02E8: 4BFFFF31  bl 0x82df0218
	ctx.lr = 0x82DF02EC;
	sub_82DF0218(ctx, base);
	// 82DF02EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF02F0: 40820028  bne 0x82df0318
	if !ctx.cr[0].eq {
	pc = 0x82DF0318; continue 'dispatch;
	}
	// 82DF02F4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF02F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF02FC: 409A0014  bne cr6, 0x82df0310
	if !ctx.cr[6].eq {
	pc = 0x82DF0310; continue 'dispatch;
	}
	// 82DF0300: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DF0304: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF0308: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF030C: 419A0008  beq cr6, 0x82df0314
	if ctx.cr[6].eq {
	pc = 0x82DF0314; continue 'dispatch;
	}
	// 82DF0310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF0314: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF0318: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF031C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0324: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF0328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0330 size=216
    let mut pc: u32 = 0x82DF0330;
    'dispatch: loop {
        match pc {
            0x82DF0330 => {
    //   block [0x82DF0330..0x82DF0408)
	// 82DF0330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0334: 483B7E35  bl 0x831a8168
	ctx.lr = 0x82DF0338;
	sub_831A8130(ctx, base);
	// 82DF0338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF033C: 484522A1  bl 0x832425dc
	ctx.lr = 0x82DF0340;
	// extern call 0x832425DC → crate::xam::XamGetSystemVersion
	crate::xam::XamGetSystemVersion(ctx, base);
	// 82DF0340: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 82DF0344: 546A022E  rlwinm r10, r3, 0, 8, 0x17
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82DF0348: 616BA100  ori r11, r11, 0xa100
	ctx.r[11].u64 = ctx.r[11].u64 | 41216;
	// 82DF034C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF0350: 40980098  bge cr6, 0x82df03e8
	if !ctx.cr[6].lt {
	pc = 0x82DF03E8; continue 'dispatch;
	}
	// 82DF0354: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82DF0358: 3B8B9D74  addi r28, r11, -0x628c
	ctx.r[28].s64 = ctx.r[11].s64 + -25228;
	// 82DF035C: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 82DF0360: 4845260D  bl 0x8324296c
	ctx.lr = 0x82DF0364;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82DF0364: 3FA08338  lis r29, -0x7cc8
	ctx.r[29].s64 = -2093481984;
	// 82DF0368: 817D5598  lwz r11, 0x5598(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(21912 as u32) ) } as u64;
	// 82DF036C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82DF0370: 409A000C  bne cr6, 0x82df037c
	if !ctx.cr[6].eq {
	pc = 0x82DF037C; continue 'dispatch;
	}
	// 82DF0374: 3BC0065B  li r30, 0x65b
	ctx.r[30].s64 = 1627;
	// 82DF0378: 48000064  b 0x82df03dc
	pc = 0x82DF03DC; continue 'dispatch;
	// 82DF037C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82DF0380: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF0384: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82DF0388: 4BFFFE91  bl 0x82df0218
	ctx.lr = 0x82DF038C;
	sub_82DF0218(ctx, base);
	// 82DF038C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DF0390: 3FE08338  lis r31, -0x7cc8
	ctx.r[31].s64 = -2093481984;
	// 82DF0394: 40820038  bne 0x82df03cc
	if !ctx.cr[0].eq {
	pc = 0x82DF03CC; continue 'dispatch;
	}
	// 82DF0398: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF039C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DF03A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF03A4: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 82DF03A8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82DF03AC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DF03B0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DF03B4: 917F559C  stw r11, 0x559c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(21916 as u32), ctx.r[11].u32 ) };
	// 82DF03B8: 4BFFFDB9  bl 0x82df0170
	ctx.lr = 0x82DF03BC;
	sub_82DF0170(ctx, base);
	// 82DF03BC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DF03C0: 4082000C  bne 0x82df03cc
	if !ctx.cr[0].eq {
	pc = 0x82DF03CC; continue 'dispatch;
	}
	// 82DF03C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF03C8: 48000010  b 0x82df03d8
	pc = 0x82DF03D8; continue 'dispatch;
	// 82DF03CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF03D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF03D4: 915F559C  stw r10, 0x559c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(21916 as u32), ctx.r[10].u32 ) };
	// 82DF03D8: 917D5598  stw r11, 0x5598(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(21912 as u32), ctx.r[11].u32 ) };
	// 82DF03DC: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 82DF03E0: 4845257D  bl 0x8324295c
	ctx.lr = 0x82DF03E4;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82DF03E4: 48000018  b 0x82df03fc
	pc = 0x82DF03FC; continue 'dispatch;
	// 82DF03E8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DF03EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF03F0: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82DF03F4: 4BFFFD7D  bl 0x82df0170
	ctx.lr = 0x82DF03F8;
	sub_82DF0170(ctx, base);
	// 82DF03F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF03FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF0400: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF0404: 483B7DB4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0408 size=176
    let mut pc: u32 = 0x82DF0408;
    'dispatch: loop {
        match pc {
            0x82DF0408 => {
    //   block [0x82DF0408..0x82DF04B8)
	// 82DF0408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF040C: 483B7D5D  bl 0x831a8168
	ctx.lr = 0x82DF0410;
	sub_831A8130(ctx, base);
	// 82DF0410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0414: 484521C9  bl 0x832425dc
	ctx.lr = 0x82DF0418;
	// extern call 0x832425DC → crate::xam::XamGetSystemVersion
	crate::xam::XamGetSystemVersion(ctx, base);
	// 82DF0418: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 82DF041C: 546A022E  rlwinm r10, r3, 0, 8, 0x17
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82DF0420: 616BA100  ori r11, r11, 0xa100
	ctx.r[11].u64 = ctx.r[11].u64 | 41216;
	// 82DF0424: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF0428: 40980070  bge cr6, 0x82df0498
	if !ctx.cr[6].lt {
	pc = 0x82DF0498; continue 'dispatch;
	}
	// 82DF042C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82DF0430: 3B8B9D74  addi r28, r11, -0x628c
	ctx.r[28].s64 = ctx.r[11].s64 + -25228;
	// 82DF0434: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 82DF0438: 48452535  bl 0x8324296c
	ctx.lr = 0x82DF043C;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82DF043C: 3FE08338  lis r31, -0x7cc8
	ctx.r[31].s64 = -2093481984;
	// 82DF0440: 817F5598  lwz r11, 0x5598(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(21912 as u32) ) } as u64;
	// 82DF0444: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF0448: 409A000C  bne cr6, 0x82df0454
	if !ctx.cr[6].eq {
	pc = 0x82DF0454; continue 'dispatch;
	}
	// 82DF044C: 3BA0065B  li r29, 0x65b
	ctx.r[29].s64 = 1627;
	// 82DF0450: 4800003C  b 0x82df048c
	pc = 0x82DF048C; continue 'dispatch;
	// 82DF0454: 3FC08338  lis r30, -0x7cc8
	ctx.r[30].s64 = -2093481984;
	// 82DF0458: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF045C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82DF0460: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF0464: 817E559C  lwz r11, 0x559c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(21916 as u32) ) } as u64;
	// 82DF0468: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82DF046C: 419A0008  beq cr6, 0x82df0474
	if ctx.cr[6].eq {
	pc = 0x82DF0474; continue 'dispatch;
	}
	// 82DF0470: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82DF0474: 4BFFFCFD  bl 0x82df0170
	ctx.lr = 0x82DF0478;
	sub_82DF0170(ctx, base);
	// 82DF0478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF047C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF0480: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF0484: 917E559C  stw r11, 0x559c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(21916 as u32), ctx.r[11].u32 ) };
	// 82DF0488: 915F5598  stw r10, 0x5598(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(21912 as u32), ctx.r[10].u32 ) };
	// 82DF048C: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 82DF0490: 484524CD  bl 0x8324295c
	ctx.lr = 0x82DF0494;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82DF0494: 48000018  b 0x82df04ac
	pc = 0x82DF04AC; continue 'dispatch;
	// 82DF0498: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF049C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82DF04A0: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82DF04A4: 4BFFFCCD  bl 0x82df0170
	ctx.lr = 0x82DF04A8;
	sub_82DF0170(ctx, base);
	// 82DF04A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF04AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF04B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF04B4: 483B7D04  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF04B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF04B8 size=132
    let mut pc: u32 = 0x82DF04B8;
    'dispatch: loop {
        match pc {
            0x82DF04B8 => {
    //   block [0x82DF04B8..0x82DF053C)
	// 82DF04B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF04BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF04C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF04C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF04C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF04CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF04D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF04D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF04D8: 483C04A9  bl 0x831b0980
	ctx.lr = 0x82DF04DC;
	sub_831B0980(ctx, base);
	// 82DF04DC: 483BFF15  bl 0x831b03f0
	ctx.lr = 0x82DF04E0;
	sub_831B03F0(ctx, base);
	// 82DF04E0: 38830040  addi r4, r3, 0x40
	ctx.r[4].s64 = ctx.r[3].s64 + 64;
	// 82DF04E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF04E8: 483BFCE1  bl 0x831b01c8
	ctx.lr = 0x82DF04EC;
	sub_831B01C8(ctx, base);
	// 82DF04EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DF04F0: 409A000C  bne cr6, 0x82df04fc
	if !ctx.cr[6].eq {
	pc = 0x82DF04FC; continue 'dispatch;
	}
	// 82DF04F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF04F8: 3BEB94B4  addi r31, r11, -0x6b4c
	ctx.r[31].s64 = ctx.r[11].s64 + -27468;
	// 82DF04FC: 483BFEF5  bl 0x831b03f0
	ctx.lr = 0x82DF0500;
	sub_831B03F0(ctx, base);
	// 82DF0500: 38830040  addi r4, r3, 0x40
	ctx.r[4].s64 = ctx.r[3].s64 + 64;
	// 82DF0504: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF0508: 483BFCC1  bl 0x831b01c8
	ctx.lr = 0x82DF050C;
	sub_831B01C8(ctx, base);
	// 82DF050C: 483BFEE5  bl 0x831b03f0
	ctx.lr = 0x82DF0510;
	sub_831B03F0(ctx, base);
	// 82DF0510: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DF0514: 38830040  addi r4, r3, 0x40
	ctx.r[4].s64 = ctx.r[3].s64 + 64;
	// 82DF0518: 386B7688  addi r3, r11, 0x7688
	ctx.r[3].s64 = ctx.r[11].s64 + 30344;
	// 82DF051C: 483BFCAD  bl 0x831b01c8
	ctx.lr = 0x82DF0520;
	sub_831B01C8(ctx, base);
	// 82DF0520: 483BB629  bl 0x831abb48
	ctx.lr = 0x82DF0524;
	sub_831ABB48(ctx, base);
	// 82DF0524: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF0528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF052C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0530: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF0534: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF0538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0540 size=60
    let mut pc: u32 = 0x82DF0540;
    'dispatch: loop {
        match pc {
            0x82DF0540 => {
    //   block [0x82DF0540..0x82DF057C)
	// 82DF0540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF054C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF0550: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF0554: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF0558: 4E800421  bctrl
	ctx.lr = 0x82DF055C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF055C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF0560: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF0564: 386BAE44  addi r3, r11, -0x51bc
	ctx.r[3].s64 = ctx.r[11].s64 + -20924;
	// 82DF0568: 4BFFFF51  bl 0x82df04b8
	ctx.lr = 0x82DF056C;
	sub_82DF04B8(ctx, base);
	// 82DF056C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF0570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0580 size=4
    let mut pc: u32 = 0x82DF0580;
    'dispatch: loop {
        match pc {
            0x82DF0580 => {
    //   block [0x82DF0580..0x82DF0584)
	// 82DF0580: 4B4D03B8  b 0x822c0938
	sub_822C0938(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0588 size=144
    let mut pc: u32 = 0x82DF0588;
    'dispatch: loop {
        match pc {
            0x82DF0588 => {
    //   block [0x82DF0588..0x82DF0618)
	// 82DF0588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF058C: 483B7BE1  bl 0x831a816c
	ctx.lr = 0x82DF0590;
	sub_831A8130(ctx, base);
	// 82DF0590: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0594: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF0598: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 82DF059C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF05A0: 396BC37C  addi r11, r11, -0x3c84
	ctx.r[11].s64 = ctx.r[11].s64 + -15492;
	// 82DF05A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF05A8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF05AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF05B0: 419A004C  beq cr6, 0x82df05fc
	if ctx.cr[6].eq {
	pc = 0x82DF05FC; continue 'dispatch;
	}
	// 82DF05B4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DF05B8: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF05BC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF05C0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF05C4: 409AFFF4  bne cr6, 0x82df05b8
	if !ctx.cr[6].eq {
	pc = 0x82DF05B8; continue 'dispatch;
	}
	// 82DF05C8: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF05CC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF05D0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF05D4: 3BEB0001  addi r31, r11, 1
	ctx.r[31].s64 = ctx.r[11].s64 + 1;
	// 82DF05D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF05DC: 483C0585  bl 0x831b0b60
	ctx.lr = 0x82DF05E0;
	sub_831B0B60(ctx, base);
	// 82DF05E0: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82DF05E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF05E8: 4182001C  beq 0x82df0604
	if ctx.cr[0].eq {
	pc = 0x82DF0604; continue 'dispatch;
	}
	// 82DF05EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF05F0: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF05F4: 483C048D  bl 0x831b0a80
	ctx.lr = 0x82DF05F8;
	sub_831B0A80(ctx, base);
	// 82DF05F8: 4800000C  b 0x82df0604
	pc = 0x82DF0604; continue 'dispatch;
	// 82DF05FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF0600: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF0604: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF0608: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF060C: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF0610: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF0614: 483B7BA8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0618 size=12
    let mut pc: u32 = 0x82DF0618;
    'dispatch: loop {
        match pc {
            0x82DF0618 => {
    //   block [0x82DF0618..0x82DF0624)
	// 82DF0618: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF061C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF0620: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0624(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0624 size=12
    let mut pc: u32 = 0x82DF0624;
    'dispatch: loop {
        match pc {
            0x82DF0624 => {
    //   block [0x82DF0624..0x82DF0630)
	// 82DF0624: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF0628: 386BAE50  addi r3, r11, -0x51b0
	ctx.r[3].s64 = ctx.r[11].s64 + -20912;
	// 82DF062C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0630 size=104
    let mut pc: u32 = 0x82DF0630;
    'dispatch: loop {
        match pc {
            0x82DF0630 => {
    //   block [0x82DF0630..0x82DF0698)
	// 82DF0630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0638: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF063C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF0640: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0644: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF0648: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 82DF064C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF0650: 396BC37C  addi r11, r11, -0x3c84
	ctx.r[11].s64 = ctx.r[11].s64 + -15492;
	// 82DF0654: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF0658: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF065C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DF0660: 419A000C  beq cr6, 0x82df066c
	if ctx.cr[6].eq {
	pc = 0x82DF066C; continue 'dispatch;
	}
	// 82DF0664: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF0668: 483BB471  bl 0x831abad8
	ctx.lr = 0x82DF066C;
	sub_831ABAD8(ctx, base);
	// 82DF066C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF0670: 4182000C  beq 0x82df067c
	if ctx.cr[0].eq {
	pc = 0x82DF067C; continue 'dispatch;
	}
	// 82DF0674: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF0678: 4B4CFBF1  bl 0x822c0268
	ctx.lr = 0x82DF067C;
	sub_822C0268(ctx, base);
	// 82DF067C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF0680: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF0684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF068C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF0690: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF0694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0698 size=160
    let mut pc: u32 = 0x82DF0698;
    'dispatch: loop {
        match pc {
            0x82DF0698 => {
    //   block [0x82DF0698..0x82DF0738)
	// 82DF0698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF069C: 483B7AD1  bl 0x831a816c
	ctx.lr = 0x82DF06A0;
	sub_831A8130(ctx, base);
	// 82DF06A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF06A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF06A8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 82DF06AC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF06B0: 396BC37C  addi r11, r11, -0x3c84
	ctx.r[11].s64 = ctx.r[11].s64 + -15492;
	// 82DF06B4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF06B8: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF06BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF06C0: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF06C4: 419A0060  beq cr6, 0x82df0724
	if ctx.cr[6].eq {
	pc = 0x82DF0724; continue 'dispatch;
	}
	// 82DF06C8: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF06CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF06D0: 419A004C  beq cr6, 0x82df071c
	if ctx.cr[6].eq {
	pc = 0x82DF071C; continue 'dispatch;
	}
	// 82DF06D4: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82DF06D8: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF06DC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF06E0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF06E4: 409AFFF4  bne cr6, 0x82df06d8
	if !ctx.cr[6].eq {
	pc = 0x82DF06D8; continue 'dispatch;
	}
	// 82DF06E8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DF06EC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF06F0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF06F4: 3BEB0001  addi r31, r11, 1
	ctx.r[31].s64 = ctx.r[11].s64 + 1;
	// 82DF06F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF06FC: 483C0465  bl 0x831b0b60
	ctx.lr = 0x82DF0700;
	sub_831B0B60(ctx, base);
	// 82DF0700: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82DF0704: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF0708: 41820024  beq 0x82df072c
	if ctx.cr[0].eq {
	pc = 0x82DF072C; continue 'dispatch;
	}
	// 82DF070C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF0710: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF0714: 483C036D  bl 0x831b0a80
	ctx.lr = 0x82DF0718;
	sub_831B0A80(ctx, base);
	// 82DF0718: 48000014  b 0x82df072c
	pc = 0x82DF072C; continue 'dispatch;
	// 82DF071C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF0720: 48000008  b 0x82df0728
	pc = 0x82DF0728; continue 'dispatch;
	// 82DF0724: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF0728: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF072C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF0730: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF0734: 483B7A88  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0738 size=8
    let mut pc: u32 = 0x82DF0738;
    'dispatch: loop {
        match pc {
            0x82DF0738 => {
    //   block [0x82DF0738..0x82DF0740)
	// 82DF0738: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82DF073C: 8211AE80  lwz r16, -0x5180(r17)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-20864 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0740 size=76
    let mut pc: u32 = 0x82DF0740;
    'dispatch: loop {
        match pc {
            0x82DF0740 => {
    //   block [0x82DF0740..0x82DF078C)
	// 82DF0740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0748: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF074C: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 82DF0750: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0754: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF0758: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82DF075C: 388BAE64  addi r4, r11, -0x519c
	ctx.r[4].s64 = ctx.r[11].s64 + -20892;
	// 82DF0760: 4B4D5169  bl 0x822c58c8
	ctx.lr = 0x82DF0764;
	sub_822C58C8(ctx, base);
	// 82DF0764: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 82DF0768: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82DF076C: 4B4D505D  bl 0x822c57c8
	ctx.lr = 0x82DF0770;
	sub_822C57C8(ctx, base);
	// 82DF0770: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF0774: 3D408225  lis r10, -0x7ddb
	ctx.r[10].s64 = -2111504384;
	// 82DF0778: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 82DF077C: 388AC358  addi r4, r10, -0x3ca8
	ctx.r[4].s64 = ctx.r[10].s64 + -15528;
	// 82DF0780: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82DF0784: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82DF0788: 483C04A1  bl 0x831b0c28
	ctx.lr = 0x82DF078C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF078C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF078C size=40
    let mut pc: u32 = 0x82DF078C;
    'dispatch: loop {
        match pc {
            0x82DF078C => {
    //   block [0x82DF078C..0x82DF07B4)
	// 82DF078C: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 82DF0790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF079C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82DF07A0: 4B4D4C09  bl 0x822c53a8
	ctx.lr = 0x82DF07A4;
	sub_822C53A8(ctx, base);
	// 82DF07A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF07A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF07AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF07B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF07B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF07B8 size=8
    let mut pc: u32 = 0x82DF07B8;
    'dispatch: loop {
        match pc {
            0x82DF07B8 => {
    //   block [0x82DF07B8..0x82DF07C0)
	// 82DF07B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82DF07BC: 8211AEB8  lwz r16, -0x5148(r17)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-20808 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF07C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF07C0 size=68
    let mut pc: u32 = 0x82DF07C0;
    'dispatch: loop {
        match pc {
            0x82DF07C0 => {
    //   block [0x82DF07C0..0x82DF0804)
	// 82DF07C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF07C4: 483B79A9  bl 0x831a816c
	ctx.lr = 0x82DF07C8;
	sub_831A8130(ctx, base);
	// 82DF07C8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82DF07CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF07D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF07D4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF07D8: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82DF07DC: 4BFFFEBD  bl 0x82df0698
	ctx.lr = 0x82DF07E0;
	sub_82DF0698(ctx, base);
	// 82DF07E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF07E4: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82DF07E8: 396B9490  addi r11, r11, -0x6b70
	ctx.r[11].s64 = ctx.r[11].s64 + -27504;
	// 82DF07EC: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 82DF07F0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF07F4: 4B4D4F7D  bl 0x822c5770
	ctx.lr = 0x82DF07F8;
	sub_822C5770(ctx, base);
	// 82DF07F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF07FC: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82DF0800: 483B79BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0804(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0804 size=40
    let mut pc: u32 = 0x82DF0804;
    'dispatch: loop {
        match pc {
            0x82DF0804 => {
    //   block [0x82DF0804..0x82DF082C)
	// 82DF0804: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82DF0808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF080C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0810: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0814: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DF0818: 483B90F9  bl 0x831a9910
	ctx.lr = 0x82DF081C;
	sub_831A9910(ctx, base);
	// 82DF081C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF0820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0830 size=60
    let mut pc: u32 = 0x82DF0830;
    'dispatch: loop {
        match pc {
            0x82DF0830 => {
    //   block [0x82DF0830..0x82DF086C)
	// 82DF0830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0838: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF083C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0840: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF0844: 4BFFFF7D  bl 0x82df07c0
	ctx.lr = 0x82DF0848;
	sub_82DF07C0(ctx, base);
	// 82DF0848: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF084C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF0850: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 82DF0854: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF0858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF085C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0864: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF0868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0870 size=8
    let mut pc: u32 = 0x82DF0870;
    'dispatch: loop {
        match pc {
            0x82DF0870 => {
    //   block [0x82DF0870..0x82DF0878)
	// 82DF0870: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82DF0874: 8211AF08  lwz r16, -0x50f8(r17)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-20728 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0878 size=76
    let mut pc: u32 = 0x82DF0878;
    'dispatch: loop {
        match pc {
            0x82DF0878 => {
    //   block [0x82DF0878..0x82DF08C4)
	// 82DF0878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF087C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0880: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF0884: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 82DF0888: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF088C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF0890: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82DF0894: 388BAEE8  addi r4, r11, -0x5118
	ctx.r[4].s64 = ctx.r[11].s64 + -20760;
	// 82DF0898: 4B4D5031  bl 0x822c58c8
	ctx.lr = 0x82DF089C;
	sub_822C58C8(ctx, base);
	// 82DF089C: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 82DF08A0: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82DF08A4: 4B4D4F25  bl 0x822c57c8
	ctx.lr = 0x82DF08A8;
	sub_822C57C8(ctx, base);
	// 82DF08A8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF08AC: 3D408225  lis r10, -0x7ddb
	ctx.r[10].s64 = -2111504384;
	// 82DF08B0: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 82DF08B4: 388AC3B0  addi r4, r10, -0x3c50
	ctx.r[4].s64 = ctx.r[10].s64 + -15440;
	// 82DF08B8: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82DF08BC: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82DF08C0: 483C0369  bl 0x831b0c28
	ctx.lr = 0x82DF08C4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF08C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF08C4 size=40
    let mut pc: u32 = 0x82DF08C4;
    'dispatch: loop {
        match pc {
            0x82DF08C4 => {
    //   block [0x82DF08C4..0x82DF08EC)
	// 82DF08C4: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 82DF08C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF08CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF08D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF08D4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82DF08D8: 4B4D4AD1  bl 0x822c53a8
	ctx.lr = 0x82DF08DC;
	sub_822C53A8(ctx, base);
	// 82DF08DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF08E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF08E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF08E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF08F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF08F0 size=60
    let mut pc: u32 = 0x82DF08F0;
    'dispatch: loop {
        match pc {
            0x82DF08F0 => {
    //   block [0x82DF08F0..0x82DF092C)
	// 82DF08F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF08F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF08F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF08FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0900: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF0904: 4BFFFEBD  bl 0x82df07c0
	ctx.lr = 0x82DF0908;
	sub_82DF07C0(ctx, base);
	// 82DF0908: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF090C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF0910: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 82DF0914: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF0918: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF091C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0924: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF0928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0930 size=68
    let mut pc: u32 = 0x82DF0930;
    'dispatch: loop {
        match pc {
            0x82DF0930 => {
    //   block [0x82DF0930..0x82DF0974)
	// 82DF0930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0938: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF093C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0940: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF0944: 483C036D  bl 0x831b0cb0
	ctx.lr = 0x82DF0948;
	sub_831B0CB0(ctx, base);
	// 82DF0948: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF094C: 7D635A2E  lhzx r11, r3, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DF0950: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF0954: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF0958: 41820008  beq 0x82df0960
	if ctx.cr[0].eq {
	pc = 0x82DF0960; continue 'dispatch;
	}
	// 82DF095C: 483B889D  bl 0x831a91f8
	ctx.lr = 0x82DF0960;
	sub_831A91F8(ctx, base);
	// 82DF0960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF0964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF096C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF0970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0978 size=140
    let mut pc: u32 = 0x82DF0978;
    'dispatch: loop {
        match pc {
            0x82DF0978 => {
    //   block [0x82DF0978..0x82DF0A04)
	// 82DF0978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF097C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0980: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF0984: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF0988: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF098C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF0990: 483C0351  bl 0x831b0ce0
	ctx.lr = 0x82DF0994;
	sub_831B0CE0(ctx, base);
	// 82DF0994: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF0998: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF099C: 483C0335  bl 0x831b0cd0
	ctx.lr = 0x82DF09A0;
	sub_831B0CD0(ctx, base);
	// 82DF09A0: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82DF09A4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82DF09A8: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 82DF09AC: 483BF57D  bl 0x831aff28
	ctx.lr = 0x82DF09B0;
	sub_831AFF28(ctx, base);
	// 82DF09B0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DF09B4: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82DF09B8: 41820020  beq 0x82df09d8
	if ctx.cr[0].eq {
	pc = 0x82DF09D8; continue 'dispatch;
	}
	// 82DF09BC: 483C02F5  bl 0x831b0cb0
	ctx.lr = 0x82DF09C0;
	sub_831B0CB0(ctx, base);
	// 82DF09C0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF09C4: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82DF09C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF09CC: 483B7B45  bl 0x831a8510
	ctx.lr = 0x82DF09D0;
	sub_831A8510(ctx, base);
	// 82DF09D0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF09D4: 48000010  b 0x82df09e4
	pc = 0x82DF09E4; continue 'dispatch;
	// 82DF09D8: 483C02D9  bl 0x831b0cb0
	ctx.lr = 0x82DF09DC;
	sub_831B0CB0(ctx, base);
	// 82DF09DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF09E0: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82DF09E4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DF09E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF09EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF09F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF09F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF09F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF09FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF0A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0A08 size=76
    let mut pc: u32 = 0x82DF0A08;
    'dispatch: loop {
        match pc {
            0x82DF0A08 => {
    //   block [0x82DF0A08..0x82DF0A54)
	// 82DF0A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0A10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0A14: 548A043E  clrlwi r10, r4, 0x10
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82DF0A18: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF0A1C: 2B0A00FF  cmplwi cr6, r10, 0xff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 255 as u32, &mut ctx.xer);
	// 82DF0A20: 4099001C  ble cr6, 0x82df0a3c
	if !ctx.cr[6].gt {
	pc = 0x82DF0A3C; continue 'dispatch;
	}
	// 82DF0A24: 483C033D  bl 0x831b0d60
	ctx.lr = 0x82DF0A28;
	sub_831B0D60(ctx, base);
	// 82DF0A28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF0A2C: 3940002A  li r10, 0x2a
	ctx.r[10].s64 = 42;
	// 82DF0A30: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DF0A34: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF0A38: 4800000C  b 0x82df0a44
	pc = 0x82DF0A44; continue 'dispatch;
	// 82DF0A3C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF0A40: 988B0000  stb r4, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 82DF0A44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF0A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0A58 size=52
    let mut pc: u32 = 0x82DF0A58;
    'dispatch: loop {
        match pc {
            0x82DF0A58 => {
    //   block [0x82DF0A58..0x82DF0A8C)
	// 82DF0A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0A60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0A64: 483C027D  bl 0x831b0ce0
	ctx.lr = 0x82DF0A68;
	sub_831B0CE0(ctx, base);
	// 82DF0A68: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF0A6C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DF0A70: 483C0261  bl 0x831b0cd0
	ctx.lr = 0x82DF0A74;
	sub_831B0CD0(ctx, base);
	// 82DF0A74: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82DF0A78: E8610050  ld r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DF0A7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF0A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0A90 size=112
    let mut pc: u32 = 0x82DF0A90;
    'dispatch: loop {
        match pc {
            0x82DF0A90 => {
    //   block [0x82DF0A90..0x82DF0B00)
	// 82DF0A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0A94: 483B76D9  bl 0x831a816c
	ctx.lr = 0x82DF0A98;
	sub_831A8130(ctx, base);
	// 82DF0A98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0A9C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82DF0AA0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF0AA4: 390B9DC0  addi r8, r11, -0x6240
	ctx.r[8].s64 = ctx.r[11].s64 + -25152;
	// 82DF0AA8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82DF0AAC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DF0AB0: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82DF0AB4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF0AB8: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82DF0ABC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DF0AC0: 4082FFE8  bne 0x82df0aa8
	if !ctx.cr[0].eq {
	pc = 0x82DF0AA8; continue 'dispatch;
	}
	// 82DF0AC4: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82DF0AC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF0ACC: 409A0028  bne cr6, 0x82df0af4
	if !ctx.cr[6].eq {
	pc = 0x82DF0AF4; continue 'dispatch;
	}
	// 82DF0AD0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF0AD4: 3BCB55A8  addi r30, r11, 0x55a8
	ctx.r[30].s64 = ctx.r[11].s64 + 21928;
	// 82DF0AD8: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82DF0ADC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF0AE0: 48001021  bl 0x82df1b00
	ctx.lr = 0x82DF0AE4;
	sub_82DF1B00(ctx, base);
	// 82DF0AE4: 3BFF001C  addi r31, r31, 0x1c
	ctx.r[31].s64 = ctx.r[31].s64 + 28;
	// 82DF0AE8: 397E0070  addi r11, r30, 0x70
	ctx.r[11].s64 = ctx.r[30].s64 + 112;
	// 82DF0AEC: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DF0AF0: 4198FFEC  blt cr6, 0x82df0adc
	if ctx.cr[6].lt {
	pc = 0x82DF0ADC; continue 'dispatch;
	}
	// 82DF0AF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF0AF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF0AFC: 483B76C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0B00 size=128
    let mut pc: u32 = 0x82DF0B00;
    'dispatch: loop {
        match pc {
            0x82DF0B00 => {
    //   block [0x82DF0B00..0x82DF0B80)
	// 82DF0B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0B08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF0B0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF0B10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0B14: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82DF0B18: 390B9DC0  addi r8, r11, -0x6240
	ctx.r[8].s64 = ctx.r[11].s64 + -25152;
	// 82DF0B1C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82DF0B20: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DF0B24: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82DF0B28: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DF0B2C: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82DF0B30: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DF0B34: 4082FFE8  bne 0x82df0b1c
	if !ctx.cr[0].eq {
	pc = 0x82DF0B1C; continue 'dispatch;
	}
	// 82DF0B38: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82DF0B3C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF0B40: 40980028  bge cr6, 0x82df0b68
	if !ctx.cr[6].lt {
	pc = 0x82DF0B68; continue 'dispatch;
	}
	// 82DF0B44: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF0B48: 3BCB55A8  addi r30, r11, 0x55a8
	ctx.r[30].s64 = ctx.r[11].s64 + 21928;
	// 82DF0B4C: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82DF0B50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF0B54: 4B4CF4AD  bl 0x822c0000
	ctx.lr = 0x82DF0B58;
	sub_822C0000(ctx, base);
	// 82DF0B58: 3BFF001C  addi r31, r31, 0x1c
	ctx.r[31].s64 = ctx.r[31].s64 + 28;
	// 82DF0B5C: 397E0070  addi r11, r30, 0x70
	ctx.r[11].s64 = ctx.r[30].s64 + 112;
	// 82DF0B60: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DF0B64: 4198FFEC  blt cr6, 0x82df0b50
	if ctx.cr[6].lt {
	pc = 0x82DF0B50; continue 'dispatch;
	}
	// 82DF0B68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF0B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0B74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF0B78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF0B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0B80 size=72
    let mut pc: u32 = 0x82DF0B80;
    'dispatch: loop {
        match pc {
            0x82DF0B80 => {
    //   block [0x82DF0B80..0x82DF0BC8)
	// 82DF0B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0B88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF0B8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0B90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF0B94: 548B07BE  clrlwi r11, r4, 0x1e
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00000003u64;
	// 82DF0B98: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82DF0B9C: 1D4B001C  mulli r10, r11, 0x1c
	ctx.r[10].s64 = ctx.r[11].s64 * 28;
	// 82DF0BA0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF0BA4: 396955A8  addi r11, r9, 0x55a8
	ctx.r[11].s64 = ctx.r[9].s64 + 21928;
	// 82DF0BA8: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DF0BAC: 48000F5D  bl 0x82df1b08
	ctx.lr = 0x82DF0BB0;
	sub_82DF1B08(ctx, base);
	// 82DF0BB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF0BB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF0BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0BC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF0BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0BC8 size=24
    let mut pc: u32 = 0x82DF0BC8;
    'dispatch: loop {
        match pc {
            0x82DF0BC8 => {
    //   block [0x82DF0BC8..0x82DF0BE0)
	// 82DF0BC8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF0BCC: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF0BD0: 396B55A8  addi r11, r11, 0x55a8
	ctx.r[11].s64 = ctx.r[11].s64 + 21928;
	// 82DF0BD4: 1D4A001C  mulli r10, r10, 0x1c
	ctx.r[10].s64 = ctx.r[10].s64 * 28;
	// 82DF0BD8: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DF0BDC: 48000F34  b 0x82df1b10
	sub_82DF1B10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0BE0 size=108
    let mut pc: u32 = 0x82DF0BE0;
    'dispatch: loop {
        match pc {
            0x82DF0BE0 => {
    //   block [0x82DF0BE0..0x82DF0C4C)
	// 82DF0BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0BE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF0BEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF0BF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0BF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF0BF8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF0BFC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF0C00: 4B5B04B1  bl 0x823a10b0
	ctx.lr = 0x82DF0C04;
	sub_823A10B0(ctx, base);
	// 82DF0C04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF0C08: 41820018  beq 0x82df0c20
	if ctx.cr[0].eq {
	pc = 0x82DF0C20; continue 'dispatch;
	}
	// 82DF0C0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF0C10: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF0C14: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF0C18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF0C1C: 4E800421  bctrl
	ctx.lr = 0x82DF0C20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF0C20: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF0C24: 4182000C  beq 0x82df0c30
	if ctx.cr[0].eq {
	pc = 0x82DF0C30; continue 'dispatch;
	}
	// 82DF0C28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF0C2C: 4B4CF63D  bl 0x822c0268
	ctx.lr = 0x82DF0C30;
	sub_822C0268(ctx, base);
	// 82DF0C30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF0C34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF0C38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0C3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0C40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF0C44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF0C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0C50 size=72
    let mut pc: u32 = 0x82DF0C50;
    'dispatch: loop {
        match pc {
            0x82DF0C50 => {
    //   block [0x82DF0C50..0x82DF0C98)
	// 82DF0C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0C58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0C5C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF0C60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF0C64: 419A0024  beq cr6, 0x82df0c88
	if ctx.cr[6].eq {
	pc = 0x82DF0C88; continue 'dispatch;
	}
	// 82DF0C68: 4B5B0449  bl 0x823a10b0
	ctx.lr = 0x82DF0C6C;
	sub_823A10B0(ctx, base);
	// 82DF0C6C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF0C70: 41820018  beq 0x82df0c88
	if ctx.cr[0].eq {
	pc = 0x82DF0C88; continue 'dispatch;
	}
	// 82DF0C74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF0C78: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF0C7C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF0C80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF0C84: 4E800421  bctrl
	ctx.lr = 0x82DF0C88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF0C88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF0C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0C98 size=76
    let mut pc: u32 = 0x82DF0C98;
    'dispatch: loop {
        match pc {
            0x82DF0C98 => {
    //   block [0x82DF0C98..0x82DF0CE4)
	// 82DF0C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0CA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF0CA4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0CA8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF0CAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF0CB0: 4BFFFED1  bl 0x82df0b80
	ctx.lr = 0x82DF0CB4;
	sub_82DF0B80(ctx, base);
	// 82DF0CB4: 3FE08338  lis r31, -0x7cc8
	ctx.r[31].s64 = -2093481984;
	// 82DF0CB8: 387F5620  addi r3, r31, 0x5620
	ctx.r[3].s64 = ctx.r[31].s64 + 22048;
	// 82DF0CBC: 4BFFFF95  bl 0x82df0c50
	ctx.lr = 0x82DF0CC0;
	sub_82DF0C50(ctx, base);
	// 82DF0CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF0CC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF0CC8: 917F5620  stw r11, 0x5620(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(22048 as u32), ctx.r[11].u32 ) };
	// 82DF0CCC: 4BFFFEFD  bl 0x82df0bc8
	ctx.lr = 0x82DF0CD0;
	sub_82DF0BC8(ctx, base);
	// 82DF0CD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF0CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0CDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF0CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0CE8 size=12
    let mut pc: u32 = 0x82DF0CE8;
    'dispatch: loop {
        match pc {
            0x82DF0CE8 => {
    //   block [0x82DF0CE8..0x82DF0CF4)
	// 82DF0CE8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF0CEC: 806B5620  lwz r3, 0x5620(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22048 as u32) ) } as u64;
	// 82DF0CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0CF8 size=84
    let mut pc: u32 = 0x82DF0CF8;
    'dispatch: loop {
        match pc {
            0x82DF0CF8 => {
    //   block [0x82DF0CF8..0x82DF0D4C)
	// 82DF0CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0D00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF0D04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0D08: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF0D0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF0D10: 894B5644  lbz r10, 0x5644(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(22084 as u32) ) } as u64;
	// 82DF0D14: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF0D18: 40820018  bne 0x82df0d30
	if !ctx.cr[0].eq {
	pc = 0x82DF0D30; continue 'dispatch;
	}
	// 82DF0D1C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DF0D20: 3D2082DF  lis r9, -0x7d21
	ctx.r[9].s64 = -2099314688;
	// 82DF0D24: 994B5644  stb r10, 0x5644(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(22084 as u32), ctx.r[10].u8 ) };
	// 82DF0D28: 38690C98  addi r3, r9, 0xc98
	ctx.r[3].s64 = ctx.r[9].s64 + 3224;
	// 82DF0D2C: 48000DED  bl 0x82df1b18
	ctx.lr = 0x82DF0D30;
	sub_82DF1B18(ctx, base);
	// 82DF0D30: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF0D34: 93EB5620  stw r31, 0x5620(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(22048 as u32), ctx.r[31].u32 ) };
	// 82DF0D38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF0D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0D44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF0D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0D50 size=8
    let mut pc: u32 = 0x82DF0D50;
    'dispatch: loop {
        match pc {
            0x82DF0D50 => {
    //   block [0x82DF0D50..0x82DF0D58)
	// 82DF0D50: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82DF0D54: 8211AF40  lwz r16, -0x50c0(r17)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-20672 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0D58 size=140
    let mut pc: u32 = 0x82DF0D58;
    'dispatch: loop {
        match pc {
            0x82DF0D58 => {
    //   block [0x82DF0D58..0x82DF0DE4)
	// 82DF0D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0D5C: 483B740D  bl 0x831a8168
	ctx.lr = 0x82DF0D60;
	sub_831A8130(ctx, base);
	// 82DF0D60: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82DF0D64: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0D68: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DF0D6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF0D70: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82DF0D74: 4BFFFE0D  bl 0x82df0b80
	ctx.lr = 0x82DF0D78;
	sub_82DF0B80(ctx, base);
	// 82DF0D78: 83BC000C  lwz r29, 0xc(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF0D7C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82DF0D80: 419A004C  beq cr6, 0x82df0dcc
	if ctx.cr[6].eq {
	pc = 0x82DF0DCC; continue 'dispatch;
	}
	// 82DF0D84: 57BE103A  slwi r30, r29, 2
	ctx.r[30].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DF0D88: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF0D8C: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 82DF0D90: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 82DF0D94: 7D4BF02E  lwzx r10, r11, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DF0D98: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF0D9C: 419A0028  beq cr6, 0x82df0dc4
	if ctx.cr[6].eq {
	pc = 0x82DF0DC4; continue 'dispatch;
	}
	// 82DF0DA0: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DF0DA4: 4B5B030D  bl 0x823a10b0
	ctx.lr = 0x82DF0DA8;
	sub_823A10B0(ctx, base);
	// 82DF0DA8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF0DAC: 41820018  beq 0x82df0dc4
	if ctx.cr[0].eq {
	pc = 0x82DF0DC4; continue 'dispatch;
	}
	// 82DF0DB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF0DB4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF0DB8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF0DBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF0DC0: 4E800421  bctrl
	ctx.lr = 0x82DF0DC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF0DC4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82DF0DC8: 409AFFC0  bne cr6, 0x82df0d88
	if !ctx.cr[6].eq {
	pc = 0x82DF0D88; continue 'dispatch;
	}
	// 82DF0DCC: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF0DD0: 483BAD09  bl 0x831abad8
	ctx.lr = 0x82DF0DD4;
	sub_831ABAD8(ctx, base);
	// 82DF0DD4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82DF0DD8: 4BFFFDF1  bl 0x82df0bc8
	ctx.lr = 0x82DF0DDC;
	sub_82DF0BC8(ctx, base);
	// 82DF0DDC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82DF0DE0: 483B73D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0DE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0DE4 size=40
    let mut pc: u32 = 0x82DF0DE4;
    'dispatch: loop {
        match pc {
            0x82DF0DE4 => {
    //   block [0x82DF0DE4..0x82DF0E0C)
	// 82DF0DE4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82DF0DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0DF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0DF4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82DF0DF8: 4BFFFDD1  bl 0x82df0bc8
	ctx.lr = 0x82DF0DFC;
	sub_82DF0BC8(ctx, base);
	// 82DF0DFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF0E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0E10 size=92
    let mut pc: u32 = 0x82DF0E10;
    'dispatch: loop {
        match pc {
            0x82DF0E10 => {
    //   block [0x82DF0E10..0x82DF0E6C)
	// 82DF0E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0E18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF0E1C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0E20: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF0E24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF0E28: 4BFFFD59  bl 0x82df0b80
	ctx.lr = 0x82DF0E2C;
	sub_82DF0B80(ctx, base);
	// 82DF0E2C: 3FE08338  lis r31, -0x7cc8
	ctx.r[31].s64 = -2093481984;
	// 82DF0E30: 48000014  b 0x82df0e44
	pc = 0x82DF0E44; continue 'dispatch;
	// 82DF0E34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF0E38: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF0E3C: 917F561C  stw r11, 0x561c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(22044 as u32), ctx.r[11].u32 ) };
	// 82DF0E40: 4BFFFDA1  bl 0x82df0be0
	ctx.lr = 0x82DF0E44;
	sub_82DF0BE0(ctx, base);
	// 82DF0E44: 807F561C  lwz r3, 0x561c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(22044 as u32) ) } as u64;
	// 82DF0E48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF0E4C: 409AFFE8  bne cr6, 0x82df0e34
	if !ctx.cr[6].eq {
	pc = 0x82DF0E34; continue 'dispatch;
	}
	// 82DF0E50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF0E54: 4BFFFD75  bl 0x82df0bc8
	ctx.lr = 0x82DF0E58;
	sub_82DF0BC8(ctx, base);
	// 82DF0E58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF0E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0E64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF0E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0E70 size=120
    let mut pc: u32 = 0x82DF0E70;
    'dispatch: loop {
        match pc {
            0x82DF0E70 => {
    //   block [0x82DF0E70..0x82DF0EE8)
	// 82DF0E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0E78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF0E7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF0E80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0E84: 3FE08338  lis r31, -0x7cc8
	ctx.r[31].s64 = -2093481984;
	// 82DF0E88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF0E8C: 817F561C  lwz r11, 0x561c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(22044 as u32) ) } as u64;
	// 82DF0E90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF0E94: 409A0010  bne cr6, 0x82df0ea4
	if !ctx.cr[6].eq {
	pc = 0x82DF0EA4; continue 'dispatch;
	}
	// 82DF0E98: 3D6082DF  lis r11, -0x7d21
	ctx.r[11].s64 = -2099314688;
	// 82DF0E9C: 386B0E10  addi r3, r11, 0xe10
	ctx.r[3].s64 = ctx.r[11].s64 + 3600;
	// 82DF0EA0: 48000C79  bl 0x82df1b18
	ctx.lr = 0x82DF0EA4;
	sub_82DF1B18(ctx, base);
	// 82DF0EA4: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82DF0EA8: 4B4CFA91  bl 0x822c0938
	ctx.lr = 0x82DF0EAC;
	sub_822C0938(ctx, base);
	// 82DF0EAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF0EB0: 41820018  beq 0x82df0ec8
	if ctx.cr[0].eq {
	pc = 0x82DF0EC8; continue 'dispatch;
	}
	// 82DF0EB4: 817F561C  lwz r11, 0x561c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(22044 as u32) ) } as u64;
	// 82DF0EB8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DF0EBC: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82DF0EC0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF0EC4: 48000008  b 0x82df0ecc
	pc = 0x82DF0ECC; continue 'dispatch;
	// 82DF0EC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF0ECC: 915F561C  stw r10, 0x561c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(22044 as u32), ctx.r[10].u32 ) };
	// 82DF0ED0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF0ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0EDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF0EE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF0EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0EE8 size=12
    let mut pc: u32 = 0x82DF0EE8;
    'dispatch: loop {
        match pc {
            0x82DF0EE8 => {
    //   block [0x82DF0EE8..0x82DF0EF4)
	// 82DF0EE8: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF0EEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF0EF0: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0EF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0EF4 size=24
    let mut pc: u32 = 0x82DF0EF4;
    'dispatch: loop {
        match pc {
            0x82DF0EF4 => {
    //   block [0x82DF0EF4..0x82DF0F0C)
	// 82DF0EF4: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DF0EF8: 3963003C  addi r11, r3, 0x3c
	ctx.r[11].s64 = ctx.r[3].s64 + 60;
	// 82DF0EFC: 2B0A0010  cmplwi cr6, r10, 0x10
	ctx.cr[6].compare_u32(ctx.r[10].u32, 16 as u32, &mut ctx.xer);
	// 82DF0F00: 4198000C  blt cr6, 0x82df0f0c
	if ctx.cr[6].lt {
		sub_82DF0F0C(ctx, base);
		return;
	}
	// 82DF0F04: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF0F08: 48000008  b 0x82df0f10
	sub_82DF0F0C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0F0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0F0C size=12
    let mut pc: u32 = 0x82DF0F0C;
    'dispatch: loop {
        match pc {
            0x82DF0F0C => {
    //   block [0x82DF0F0C..0x82DF0F18)
	// 82DF0F0C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82DF0F10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF0F14: 483BFF2C  b 0x831b0e40
	sub_831B0E40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0F18 size=4
    let mut pc: u32 = 0x82DF0F18;
    'dispatch: loop {
        match pc {
            0x82DF0F18 => {
    //   block [0x82DF0F18..0x82DF0F1C)
	// 82DF0F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0F20 size=8
    let mut pc: u32 = 0x82DF0F20;
    'dispatch: loop {
        match pc {
            0x82DF0F20 => {
    //   block [0x82DF0F20..0x82DF0F28)
	// 82DF0F20: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82DF0F24: 8211AF90  lwz r16, -0x5070(r17)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-20592 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0F28 size=116
    let mut pc: u32 = 0x82DF0F28;
    'dispatch: loop {
        match pc {
            0x82DF0F28 => {
    //   block [0x82DF0F28..0x82DF0F9C)
	// 82DF0F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0F30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF0F34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF0F38: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82DF0F3C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0F40: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF0F44: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF0F48: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82DF0F4C: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF0F50: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82DF0F54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF0F58: 989E0014  stb r4, 0x14(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[4].u8 ) };
	// 82DF0F5C: 394AAF80  addi r10, r10, -0x5080
	ctx.r[10].s64 = ctx.r[10].s64 + -20608;
	// 82DF0F60: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF0F64: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82DF0F68: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF0F6C: 387E0018  addi r3, r30, 0x18
	ctx.r[3].s64 = ctx.r[30].s64 + 24;
	// 82DF0F70: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DF0F74: 3889D70C  addi r4, r9, -0x28f4
	ctx.r[4].s64 = ctx.r[9].s64 + -10484;
	// 82DF0F78: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DF0F7C: 4B4D494D  bl 0x822c58c8
	ctx.lr = 0x82DF0F80;
	sub_822C58C8(ctx, base);
	// 82DF0F80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF0F84: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82DF0F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0F90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF0F94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF0F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0F9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0F9C size=40
    let mut pc: u32 = 0x82DF0F9C;
    'dispatch: loop {
        match pc {
            0x82DF0F9C => {
    //   block [0x82DF0F9C..0x82DF0FC4)
	// 82DF0F9C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82DF0FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0FA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0FAC: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DF0FB0: 4B5B0171  bl 0x823a1120
	ctx.lr = 0x82DF0FB4;
	sub_823A1120(ctx, base);
	// 82DF0FB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF0FB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF0FBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF0FC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF0FC8 size=8
    let mut pc: u32 = 0x82DF0FC8;
    'dispatch: loop {
        match pc {
            0x82DF0FC8 => {
    //   block [0x82DF0FC8..0x82DF0FD0)
	// 82DF0FC8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82DF0FCC: 8211AFD0  lwz r16, -0x5030(r17)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-20528 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF0FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF0FD0 size=100
    let mut pc: u32 = 0x82DF0FD0;
    'dispatch: loop {
        match pc {
            0x82DF0FD0 => {
    //   block [0x82DF0FD0..0x82DF1034)
	// 82DF0FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF0FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF0FD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF0FDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF0FE0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82DF0FE4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF0FE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF0FEC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF0FF0: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82DF0FF4: 396BAF80  addi r11, r11, -0x5080
	ctx.r[11].s64 = ctx.r[11].s64 + -20608;
	// 82DF0FF8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF0FFC: 4BFFFD5D  bl 0x82df0d58
	ctx.lr = 0x82DF1000;
	sub_82DF0D58(ctx, base);
	// 82DF1000: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF1004: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF1008: 387E0018  addi r3, r30, 0x18
	ctx.r[3].s64 = ctx.r[30].s64 + 24;
	// 82DF100C: 4B4D3CD5  bl 0x822c4ce0
	ctx.lr = 0x82DF1010;
	sub_822C4CE0(ctx, base);
	// 82DF1010: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DF1014: 396BBF7C  addi r11, r11, -0x4084
	ctx.r[11].s64 = ctx.r[11].s64 + -16516;
	// 82DF1018: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF101C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82DF1020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF1024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF1028: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF102C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF1030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1034(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF1034 size=40
    let mut pc: u32 = 0x82DF1034;
    'dispatch: loop {
        match pc {
            0x82DF1034 => {
    //   block [0x82DF1034..0x82DF105C)
	// 82DF1034: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82DF1038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF103C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF1040: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF1044: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DF1048: 4B5B00D9  bl 0x823a1120
	ctx.lr = 0x82DF104C;
	sub_823A1120(ctx, base);
	// 82DF104C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF1050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF1054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF1058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF105C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF105C size=44
    let mut pc: u32 = 0x82DF105C;
    'dispatch: loop {
        match pc {
            0x82DF105C => {
    //   block [0x82DF105C..0x82DF1088)
	// 82DF105C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82DF1060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF1064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF1068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF106C: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DF1070: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 82DF1074: 4B4D4335  bl 0x822c53a8
	ctx.lr = 0x82DF1078;
	sub_822C53A8(ctx, base);
	// 82DF1078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF107C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF1080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF1084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF1088 size=76
    let mut pc: u32 = 0x82DF1088;
    'dispatch: loop {
        match pc {
            0x82DF1088 => {
    //   block [0x82DF1088..0x82DF10D4)
	// 82DF1088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF108C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF1090: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF1094: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF1098: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF109C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF10A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF10A4: 4BFFFF2D  bl 0x82df0fd0
	ctx.lr = 0x82DF10A8;
	sub_82DF0FD0(ctx, base);
	// 82DF10A8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF10AC: 4182000C  beq 0x82df10b8
	if ctx.cr[0].eq {
	pc = 0x82DF10B8; continue 'dispatch;
	}
	// 82DF10B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF10B4: 4B4CF1B5  bl 0x822c0268
	ctx.lr = 0x82DF10B8;
	sub_822C0268(ctx, base);
	// 82DF10B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF10BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF10C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF10C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF10C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF10CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF10D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF10D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF10D8 size=8
    let mut pc: u32 = 0x82DF10D8;
    'dispatch: loop {
        match pc {
            0x82DF10D8 => {
    //   block [0x82DF10D8..0x82DF10E0)
	// 82DF10D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82DF10DC: 8211B010  lwz r16, -0x4ff0(r17)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-20464 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF10E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF10E0 size=180
    let mut pc: u32 = 0x82DF10E0;
    'dispatch: loop {
        match pc {
            0x82DF10E0 => {
    //   block [0x82DF10E0..0x82DF1194)
	// 82DF10E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF10E4: 483B7089  bl 0x831a816c
	ctx.lr = 0x82DF10E8;
	sub_831A8130(ctx, base);
	// 82DF10E8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82DF10EC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF10F0: 3FA08338  lis r29, -0x7cc8
	ctx.r[29].s64 = -2093481984;
	// 82DF10F4: 83DD5620  lwz r30, 0x5620(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(22048 as u32) ) } as u64;
	// 82DF10F8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DF10FC: 409A008C  bne cr6, 0x82df1188
	if !ctx.cr[6].eq {
	pc = 0x82DF1188; continue 'dispatch;
	}
	// 82DF1100: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF1104: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82DF1108: 4BFFFA79  bl 0x82df0b80
	ctx.lr = 0x82DF110C;
	sub_82DF0B80(ctx, base);
	// 82DF110C: 83DD5620  lwz r30, 0x5620(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(22048 as u32) ) } as u64;
	// 82DF1110: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DF1114: 409A006C  bne cr6, 0x82df1180
	if !ctx.cr[6].eq {
	pc = 0x82DF1180; continue 'dispatch;
	}
	// 82DF1118: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 82DF111C: 4B4CF81D  bl 0x822c0938
	ctx.lr = 0x82DF1120;
	sub_822C0938(ctx, base);
	// 82DF1120: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82DF1124: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF1128: 41820014  beq 0x82df113c
	if ctx.cr[0].eq {
	pc = 0x82DF113C; continue 'dispatch;
	}
	// 82DF112C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF1130: 4BFFFDF9  bl 0x82df0f28
	ctx.lr = 0x82DF1134;
	sub_82DF0F28(ctx, base);
	// 82DF1134: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF1138: 48000008  b 0x82df1140
	pc = 0x82DF1140; continue 'dispatch;
	// 82DF113C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DF1140: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF1144: 4BFFFBB5  bl 0x82df0cf8
	ctx.lr = 0x82DF1148;
	sub_82DF0CF8(ctx, base);
	// 82DF1148: 3960003F  li r11, 0x3f
	ctx.r[11].s64 = 63;
	// 82DF114C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DF1150: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DF1154: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DF1158: 388AD080  addi r4, r10, -0x2f80
	ctx.r[4].s64 = ctx.r[10].s64 + -12160;
	// 82DF115C: 387E0018  addi r3, r30, 0x18
	ctx.r[3].s64 = ctx.r[30].s64 + 24;
	// 82DF1160: 4B4D4509  bl 0x822c5668
	ctx.lr = 0x82DF1164;
	sub_822C5668(ctx, base);
	// 82DF1164: 3FA08338  lis r29, -0x7cc8
	ctx.r[29].s64 = -2093481984;
	// 82DF1168: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF116C: 93DD5624  stw r30, 0x5624(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(22052 as u32), ctx.r[30].u32 ) };
	// 82DF1170: 4B5AFEE9  bl 0x823a1058
	ctx.lr = 0x82DF1174;
	sub_823A1058(ctx, base);
	// 82DF1174: 817D5624  lwz r11, 0x5624(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(22052 as u32) ) } as u64;
	// 82DF1178: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DF117C: 916A563C  stw r11, 0x563c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(22076 as u32), ctx.r[11].u32 ) };
	// 82DF1180: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82DF1184: 4BFFFA45  bl 0x82df0bc8
	ctx.lr = 0x82DF1188;
	sub_82DF0BC8(ctx, base);
	// 82DF1188: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF118C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82DF1190: 483B702C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1194(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF1194 size=40
    let mut pc: u32 = 0x82DF1194;
    'dispatch: loop {
        match pc {
            0x82DF1194 => {
    //   block [0x82DF1194..0x82DF11BC)
	// 82DF1194: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82DF1198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF119C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF11A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF11A4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82DF11A8: 4BFFFA21  bl 0x82df0bc8
	ctx.lr = 0x82DF11AC;
	sub_82DF0BC8(ctx, base);
	// 82DF11AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF11B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF11B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF11B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF11BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF11BC size=40
    let mut pc: u32 = 0x82DF11BC;
    'dispatch: loop {
        match pc {
            0x82DF11BC => {
    //   block [0x82DF11BC..0x82DF11E4)
	// 82DF11BC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82DF11C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF11C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF11C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF11CC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DF11D0: 4B4CF099  bl 0x822c0268
	ctx.lr = 0x82DF11D4;
	sub_822C0268(ctx, base);
	// 82DF11D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF11D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF11DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF11E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF11E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF11E8 size=196
    let mut pc: u32 = 0x82DF11E8;
    'dispatch: loop {
        match pc {
            0x82DF11E8 => {
    //   block [0x82DF11E8..0x82DF12AC)
	// 82DF11E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF11EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF11F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF11F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF11F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF11FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF1200: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF1204: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF1208: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF120C: 483BFC35  bl 0x831b0e40
	ctx.lr = 0x82DF1210;
	sub_831B0E40(ctx, base);
	// 82DF1210: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82DF1214: 4082000C  bne 0x82df1220
	if !ctx.cr[0].eq {
	pc = 0x82DF1220; continue 'dispatch;
	}
	// 82DF1218: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF121C: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 82DF1220: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DF1224: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1228: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF122C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF1230: 409AFFF4  bne cr6, 0x82df1224
	if !ctx.cr[6].eq {
	pc = 0x82DF1224; continue 'dispatch;
	}
	// 82DF1234: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82DF1238: 387E003C  addi r3, r30, 0x3c
	ctx.r[3].s64 = ctx.r[30].s64 + 60;
	// 82DF123C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF1240: 5565003E  slwi r5, r11, 0
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DF1244: 4B4D4425  bl 0x822c5668
	ctx.lr = 0x82DF1248;
	sub_822C5668(ctx, base);
	// 82DF1248: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DF124C: 419A0018  beq cr6, 0x82df1264
	if ctx.cr[6].eq {
	pc = 0x82DF1264; continue 'dispatch;
	}
	// 82DF1250: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF1254: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF1258: 483BFBE9  bl 0x831b0e40
	ctx.lr = 0x82DF125C;
	sub_831B0E40(ctx, base);
	// 82DF125C: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82DF1260: 4082000C  bne 0x82df126c
	if !ctx.cr[0].eq {
	pc = 0x82DF126C; continue 'dispatch;
	}
	// 82DF1264: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82DF1268: 388BD70C  addi r4, r11, -0x28f4
	ctx.r[4].s64 = ctx.r[11].s64 + -10484;
	// 82DF126C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DF1270: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1274: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF1278: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF127C: 409AFFF4  bne cr6, 0x82df1270
	if !ctx.cr[6].eq {
	pc = 0x82DF1270; continue 'dispatch;
	}
	// 82DF1280: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82DF1284: 387E0058  addi r3, r30, 0x58
	ctx.r[3].s64 = ctx.r[30].s64 + 88;
	// 82DF1288: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF128C: 5565003E  slwi r5, r11, 0
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DF1290: 4B4D43D9  bl 0x822c5668
	ctx.lr = 0x82DF1294;
	sub_822C5668(ctx, base);
	// 82DF1294: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF1298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF129C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF12A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF12A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF12A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF12B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF12B0 size=144
    let mut pc: u32 = 0x82DF12B0;
    'dispatch: loop {
        match pc {
            0x82DF12B0 => {
    //   block [0x82DF12B0..0x82DF1340)
	// 82DF12B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF12B4: 483B6EB9  bl 0x831a816c
	ctx.lr = 0x82DF12B8;
	sub_831A8130(ctx, base);
	// 82DF12B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF12BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF12C0: 83FE0020  lwz r31, 0x20(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DF12C4: 48000020  b 0x82df12e4
	pc = 0x82DF12E4; continue 'dispatch;
	// 82DF12C8: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF12CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF12D0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF12D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF12D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF12DC: 4E800421  bctrl
	ctx.lr = 0x82DF12E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF12E0: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF12E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DF12E8: 409AFFE0  bne cr6, 0x82df12c8
	if !ctx.cr[6].eq {
	pc = 0x82DF12C8; continue 'dispatch;
	}
	// 82DF12EC: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DF12F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF12F4: 419A0018  beq cr6, 0x82df130c
	if ctx.cr[6].eq {
	pc = 0x82DF130C; continue 'dispatch;
	}
	// 82DF12F8: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF12FC: 4B4CEF6D  bl 0x822c0268
	ctx.lr = 0x82DF1300;
	sub_822C0268(ctx, base);
	// 82DF1300: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF1304: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DF1308: 409AFFF0  bne cr6, 0x82df12f8
	if !ctx.cr[6].eq {
	pc = 0x82DF12F8; continue 'dispatch;
	}
	// 82DF130C: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DF1310: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DF1314: 93BE001C  stw r29, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82DF1318: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF131C: 419A0018  beq cr6, 0x82df1334
	if ctx.cr[6].eq {
	pc = 0x82DF1334; continue 'dispatch;
	}
	// 82DF1320: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1324: 4B4CEF45  bl 0x822c0268
	ctx.lr = 0x82DF1328;
	sub_822C0268(ctx, base);
	// 82DF1328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF132C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DF1330: 409AFFF0  bne cr6, 0x82df1320
	if !ctx.cr[6].eq {
	pc = 0x82DF1320; continue 'dispatch;
	}
	// 82DF1334: 93BE0020  stw r29, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82DF1338: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF133C: 483B6E80  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF1340 size=160
    let mut pc: u32 = 0x82DF1340;
    'dispatch: loop {
        match pc {
            0x82DF1340 => {
    //   block [0x82DF1340..0x82DF13E0)
	// 82DF1340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF1344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF1348: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF134C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF1350: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF1354: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82DF1358: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF135C: 4BFFF825  bl 0x82df0b80
	ctx.lr = 0x82DF1360;
	sub_82DF0B80(ctx, base);
	// 82DF1360: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF1364: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF1368: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF136C: 396B567C  addi r11, r11, 0x567c
	ctx.r[11].s64 = ctx.r[11].s64 + 22140;
	// 82DF1370: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF1374: 392BFFD8  addi r9, r11, -0x28
	ctx.r[9].s64 = ctx.r[11].s64 + -40;
	// 82DF1378: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF137C: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DF1380: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF1384: 419A0020  beq cr6, 0x82df13a4
	if ctx.cr[6].eq {
	pc = 0x82DF13A4; continue 'dispatch;
	}
	// 82DF1388: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DF138C: 419A0018  beq cr6, 0x82df13a4
	if ctx.cr[6].eq {
	pc = 0x82DF13A4; continue 'dispatch;
	}
	// 82DF1390: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF1394: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF1398: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DF139C: 2B0A0008  cmplwi cr6, r10, 8
	ctx.cr[6].compare_u32(ctx.r[10].u32, 8 as u32, &mut ctx.xer);
	// 82DF13A0: 4198FFD0  blt cr6, 0x82df1370
	if ctx.cr[6].lt {
	pc = 0x82DF1370; continue 'dispatch;
	}
	// 82DF13A4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF13A8: 392BFFD8  addi r9, r11, -0x28
	ctx.r[9].s64 = ctx.r[11].s64 + -40;
	// 82DF13AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF13B0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF13B4: 7FEA492E  stwx r31, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[31].u32) };
	// 82DF13B8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF13BC: 7D2B50AE  lbzx r9, r11, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DF13C0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF13C4: 7D2B51AE  stbx r9, r11, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u8) };
	// 82DF13C8: 4BFFF801  bl 0x82df0bc8
	ctx.lr = 0x82DF13CC;
	sub_82DF0BC8(ctx, base);
	// 82DF13CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF13D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF13D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF13D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF13DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF13E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF13E0 size=120
    let mut pc: u32 = 0x82DF13E0;
    'dispatch: loop {
        match pc {
            0x82DF13E0 => {
    //   block [0x82DF13E0..0x82DF1458)
	// 82DF13E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF13E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF13E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF13EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF13F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF13F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF13F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF13FC: 4099002C  ble cr6, 0x82df1428
	if !ctx.cr[6].gt {
	pc = 0x82DF1428; continue 'dispatch;
	}
	// 82DF1400: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF1404: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF1408: 396B567C  addi r11, r11, 0x567c
	ctx.r[11].s64 = ctx.r[11].s64 + 22140;
	// 82DF140C: 7D2B50AE  lbzx r9, r11, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DF1410: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82DF1414: 7D2B51AE  stbx r9, r11, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u8) };
	// 82DF1418: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF141C: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DF1420: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF1424: 41810020  bgt 0x82df1444
	if ctx.cr[0].gt {
	pc = 0x82DF1444; continue 'dispatch;
	}
	// 82DF1428: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF142C: 4BFFFE85  bl 0x82df12b0
	ctx.lr = 0x82DF1430;
	sub_82DF12B0(ctx, base);
	// 82DF1430: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DF1434: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF1438: 419A000C  beq cr6, 0x82df1444
	if ctx.cr[6].eq {
	pc = 0x82DF1444; continue 'dispatch;
	}
	// 82DF143C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF1440: 4B5B02D1  bl 0x823a1710
	ctx.lr = 0x82DF1444;
	sub_823A1710(ctx, base);
	// 82DF1444: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF1448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF144C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF1450: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF1454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF1458 size=60
    let mut pc: u32 = 0x82DF1458;
    'dispatch: loop {
        match pc {
            0x82DF1458 => {
    //   block [0x82DF1458..0x82DF1494)
	// 82DF1458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF145C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF1460: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF1464: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF1468: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF146C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82DF1470: 4B4CF4C9  bl 0x822c0938
	ctx.lr = 0x82DF1474;
	sub_822C0938(ctx, base);
	// 82DF1474: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DF1478: 48000689  bl 0x82df1b00
	ctx.lr = 0x82DF147C;
	sub_82DF1B00(ctx, base);
	// 82DF147C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF1480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF1484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF1488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF148C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF1490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF1498 size=56
    let mut pc: u32 = 0x82DF1498;
    'dispatch: loop {
        match pc {
            0x82DF1498 => {
    //   block [0x82DF1498..0x82DF14D0)
	// 82DF1498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF149C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF14A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF14A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF14A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF14AC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF14B0: 4B4CEB51  bl 0x822c0000
	ctx.lr = 0x82DF14B4;
	sub_822C0000(ctx, base);
	// 82DF14B4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF14B8: 4B4CEDB1  bl 0x822c0268
	ctx.lr = 0x82DF14BC;
	sub_822C0268(ctx, base);
	// 82DF14BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF14C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF14C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF14C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF14CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF14D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF14D0 size=8
    let mut pc: u32 = 0x82DF14D0;
    'dispatch: loop {
        match pc {
            0x82DF14D0 => {
    //   block [0x82DF14D0..0x82DF14D8)
	// 82DF14D0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF14D4: 48000634  b 0x82df1b08
	sub_82DF1B08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF14D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF14D8 size=8
    let mut pc: u32 = 0x82DF14D8;
    'dispatch: loop {
        match pc {
            0x82DF14D8 => {
    //   block [0x82DF14D8..0x82DF14E0)
	// 82DF14D8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF14DC: 48000634  b 0x82df1b10
	sub_82DF1B10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF14E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF14E0 size=68
    let mut pc: u32 = 0x82DF14E0;
    'dispatch: loop {
        match pc {
            0x82DF14E0 => {
    //   block [0x82DF14E0..0x82DF1524)
	// 82DF14E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF14E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF14E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF14EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF14F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF14F4: 483BF7BD  bl 0x831b0cb0
	ctx.lr = 0x82DF14F8;
	sub_831B0CB0(ctx, base);
	// 82DF14F8: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF14FC: 7D635A2E  lhzx r11, r3, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DF1500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF1504: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF1508: 41820008  beq 0x82df1510
	if ctx.cr[0].eq {
	pc = 0x82DF1510; continue 'dispatch;
	}
	// 82DF150C: 483BB155  bl 0x831ac660
	ctx.lr = 0x82DF1510;
	sub_831AC660(ctx, base);
	// 82DF1510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF1514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF1518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF151C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF1520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF1528 size=608
    let mut pc: u32 = 0x82DF1528;
    'dispatch: loop {
        match pc {
            0x82DF1528 => {
    //   block [0x82DF1528..0x82DF1788)
	// 82DF1528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF152C: 483B6C1D  bl 0x831a8148
	ctx.lr = 0x82DF1530;
	sub_831A8130(ctx, base);
	// 82DF1530: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF1534: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 82DF1538: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82DF153C: 7C942378  mr r20, r4
	ctx.r[20].u64 = ctx.r[4].u64;
	// 82DF1540: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DF1544: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DF1548: 2B150000  cmplwi cr6, r21, 0
	ctx.cr[6].compare_u32(ctx.r[21].u32, 0 as u32, &mut ctx.xer);
	// 82DF154C: 419A0008  beq cr6, 0x82df1554
	if ctx.cr[6].eq {
	pc = 0x82DF1554; continue 'dispatch;
	}
	// 82DF1550: 93B50000  stw r29, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DF1554: 7EDFB378  mr r31, r22
	ctx.r[31].u64 = ctx.r[22].u64;
	// 82DF1558: 483BF769  bl 0x831b0cc0
	ctx.lr = 0x82DF155C;
	sub_831B0CC0(ctx, base);
	// 82DF155C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82DF1560: 40990014  ble cr6, 0x82df1574
	if !ctx.cr[6].gt {
	pc = 0x82DF1574; continue 'dispatch;
	}
	// 82DF1564: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82DF1568: 887F0000  lbz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF156C: 483BF94D  bl 0x831b0eb8
	ctx.lr = 0x82DF1570;
	sub_831B0EB8(ctx, base);
	// 82DF1570: 48000018  b 0x82df1588
	pc = 0x82DF1588; continue 'dispatch;
	// 82DF1574: 483BF73D  bl 0x831b0cb0
	ctx.lr = 0x82DF1578;
	sub_831B0CB0(ctx, base);
	// 82DF1578: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF157C: 556B083E  rotlwi r11, r11, 1
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 82DF1580: 7D635A2E  lhzx r11, r3, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DF1584: 55630738  rlwinm r3, r11, 0, 0x1c, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DF1588: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DF158C: 419A000C  beq cr6, 0x82df1598
	if ctx.cr[6].eq {
	pc = 0x82DF1598; continue 'dispatch;
	}
	// 82DF1590: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DF1594: 4BFFFFC4  b 0x82df1558
	pc = 0x82DF1558; continue 'dispatch;
	// 82DF1598: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF159C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DF15A0: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 82DF15A4: 419A0014  beq cr6, 0x82df15b8
	if ctx.cr[6].eq {
	pc = 0x82DF15B8; continue 'dispatch;
	}
	// 82DF15A8: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 82DF15AC: 419A000C  beq cr6, 0x82df15b8
	if ctx.cr[6].eq {
	pc = 0x82DF15B8; continue 'dispatch;
	}
	// 82DF15B0: 3960002B  li r11, 0x2b
	ctx.r[11].s64 = 43;
	// 82DF15B4: 48000008  b 0x82df15bc
	pc = 0x82DF15BC; continue 'dispatch;
	// 82DF15B8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DF15BC: 7D770774  extsb r23, r11
	ctx.r[23].s64 = ctx.r[11].s8 as i64;
	// 82DF15C0: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82DF15C4: 419801AC  blt cr6, 0x82df1770
	if ctx.cr[6].lt {
	pc = 0x82DF1770; continue 'dispatch;
	}
	// 82DF15C8: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 82DF15CC: 419A01A4  beq cr6, 0x82df1770
	if ctx.cr[6].eq {
	pc = 0x82DF1770; continue 'dispatch;
	}
	// 82DF15D0: 2F1C0024  cmpwi cr6, r28, 0x24
	ctx.cr[6].compare_i32(ctx.r[28].s32, 36, &mut ctx.xer);
	// 82DF15D4: 4199019C  bgt cr6, 0x82df1770
	if ctx.cr[6].gt {
	pc = 0x82DF1770; continue 'dispatch;
	}
	// 82DF15D8: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82DF15DC: 40990034  ble cr6, 0x82df1610
	if !ctx.cr[6].gt {
	pc = 0x82DF1610; continue 'dispatch;
	}
	// 82DF15E0: 2F1C0010  cmpwi cr6, r28, 0x10
	ctx.cr[6].compare_i32(ctx.r[28].s32, 16, &mut ctx.xer);
	// 82DF15E4: 409A0068  bne cr6, 0x82df164c
	if !ctx.cr[6].eq {
	pc = 0x82DF164C; continue 'dispatch;
	}
	// 82DF15E8: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF15EC: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 82DF15F0: 409A005C  bne cr6, 0x82df164c
	if !ctx.cr[6].eq {
	pc = 0x82DF164C; continue 'dispatch;
	}
	// 82DF15F4: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DF15F8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DF15FC: 2F0B0078  cmpwi cr6, r11, 0x78
	ctx.cr[6].compare_i32(ctx.r[11].s32, 120, &mut ctx.xer);
	// 82DF1600: 419A0048  beq cr6, 0x82df1648
	if ctx.cr[6].eq {
	pc = 0x82DF1648; continue 'dispatch;
	}
	// 82DF1604: 2F0B0058  cmpwi cr6, r11, 0x58
	ctx.cr[6].compare_i32(ctx.r[11].s32, 88, &mut ctx.xer);
	// 82DF1608: 409A0044  bne cr6, 0x82df164c
	if !ctx.cr[6].eq {
	pc = 0x82DF164C; continue 'dispatch;
	}
	// 82DF160C: 4800003C  b 0x82df1648
	pc = 0x82DF1648; continue 'dispatch;
	// 82DF1610: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1614: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 82DF1618: 419A000C  beq cr6, 0x82df1624
	if ctx.cr[6].eq {
	pc = 0x82DF1624; continue 'dispatch;
	}
	// 82DF161C: 3B80000A  li r28, 0xa
	ctx.r[28].s64 = 10;
	// 82DF1620: 4800002C  b 0x82df164c
	pc = 0x82DF164C; continue 'dispatch;
	// 82DF1624: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DF1628: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DF162C: 2F0B0078  cmpwi cr6, r11, 0x78
	ctx.cr[6].compare_i32(ctx.r[11].s32, 120, &mut ctx.xer);
	// 82DF1630: 419A0014  beq cr6, 0x82df1644
	if ctx.cr[6].eq {
	pc = 0x82DF1644; continue 'dispatch;
	}
	// 82DF1634: 2F0B0058  cmpwi cr6, r11, 0x58
	ctx.cr[6].compare_i32(ctx.r[11].s32, 88, &mut ctx.xer);
	// 82DF1638: 419A000C  beq cr6, 0x82df1644
	if ctx.cr[6].eq {
	pc = 0x82DF1644; continue 'dispatch;
	}
	// 82DF163C: 3B800008  li r28, 8
	ctx.r[28].s64 = 8;
	// 82DF1640: 4800000C  b 0x82df164c
	pc = 0x82DF164C; continue 'dispatch;
	// 82DF1644: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 82DF1648: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82DF164C: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1650: 7FF8FB78  mr r24, r31
	ctx.r[24].u64 = ctx.r[31].u64;
	// 82DF1654: 4800000C  b 0x82df1660
	pc = 0x82DF1660; continue 'dispatch;
	// 82DF1658: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DF165C: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1660: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 82DF1664: 419AFFF4  beq cr6, 0x82df1658
	if ctx.cr[6].eq {
	pc = 0x82DF1658; continue 'dispatch;
	}
	// 82DF1668: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF166C: 7FF9FB78  mr r25, r31
	ctx.r[25].u64 = ctx.r[31].u64;
	// 82DF1670: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82DF1674: 483B7B85  bl 0x831a91f8
	ctx.lr = 0x82DF1678;
	sub_831A91F8(ctx, base);
	// 82DF1678: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF167C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF1680: 3BCBB064  addi r30, r11, -0x4f9c
	ctx.r[30].s64 = ctx.r[11].s64 + -20380;
	// 82DF1684: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DF1688: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF168C: 483B8A95  bl 0x831aa120
	ctx.lr = 0x82DF1690;
	sub_831AA120(ctx, base);
	// 82DF1690: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF1694: 41820044  beq 0x82df16d8
	if ctx.cr[0].eq {
	pc = 0x82DF16D8; continue 'dispatch;
	}
	// 82DF1698: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DF169C: 7D5E1850  subf r10, r30, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[30].s64;
	// 82DF16A0: 7D7DE1D6  mullw r11, r29, r28
	ctx.r[11].s64 = (ctx.r[29].s32 as i64) * (ctx.r[28].s32 as i64);
	// 82DF16A4: 893F0000  lbz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF16A8: 7D5B0774  extsb r27, r10
	ctx.r[27].s64 = ctx.r[10].s8 as i64;
	// 82DF16AC: 7D230774  extsb r3, r9
	ctx.r[3].s64 = ctx.r[9].s8 as i64;
	// 82DF16B0: 7FBAEB78  mr r26, r29
	ctx.r[26].u64 = ctx.r[29].u64;
	// 82DF16B4: 7FABDA14  add r29, r11, r27
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82DF16B8: 483B7B41  bl 0x831a91f8
	ctx.lr = 0x82DF16BC;
	sub_831A91F8(ctx, base);
	// 82DF16BC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF16C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF16C4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DF16C8: 483B8A59  bl 0x831aa120
	ctx.lr = 0x82DF16CC;
	sub_831AA120(ctx, base);
	// 82DF16CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF16D0: 4082FFC8  bne 0x82df1698
	if !ctx.cr[0].eq {
	pc = 0x82DF1698; continue 'dispatch;
	}
	// 82DF16D4: 4800000C  b 0x82df16e0
	pc = 0x82DF16E0; continue 'dispatch;
	// 82DF16D8: 83410054  lwz r26, 0x54(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DF16DC: 8B610050  lbz r27, 0x50(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF16E0: 7F18F840  cmplw cr6, r24, r31
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DF16E4: 419A008C  beq cr6, 0x82df1770
	if ctx.cr[6].eq {
	pc = 0x82DF1770; continue 'dispatch;
	}
	// 82DF16E8: 397E0028  addi r11, r30, 0x28
	ctx.r[11].s64 = ctx.r[30].s64 + 40;
	// 82DF16EC: 7D7C58AE  lbzx r11, r28, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DF16F0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DF16F4: 7D6BF850  subf r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82DF16F8: 7D795851  subf. r11, r25, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[25].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF16FC: 41800050  blt 0x82df174c
	if ctx.cr[0].lt {
	pc = 0x82DF174C; continue 'dispatch;
	}
	// 82DF1700: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF1704: 41990024  bgt cr6, 0x82df1728
	if ctx.cr[6].gt {
	pc = 0x82DF1728; continue 'dispatch;
	}
	// 82DF1708: 7F6B0774  extsb r11, r27
	ctx.r[11].s64 = ctx.r[27].s8 as i64;
	// 82DF170C: 7D6BE850  subf r11, r11, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 82DF1710: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF1714: 41980014  blt cr6, 0x82df1728
	if ctx.cr[6].lt {
	pc = 0x82DF1728; continue 'dispatch;
	}
	// 82DF1718: 7D6BE396  divwu r11, r11, r28
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[28].u32;
	// 82DF171C: 0CDC0000  twi 6, r28, 0
	// 82DF1720: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF1724: 419A0028  beq cr6, 0x82df174c
	if ctx.cr[6].eq {
	pc = 0x82DF174C; continue 'dispatch;
	}
	// 82DF1728: 483BF639  bl 0x831b0d60
	ctx.lr = 0x82DF172C;
	sub_831B0D60(ctx, base);
	// 82DF172C: 39600022  li r11, 0x22
	ctx.r[11].s64 = 34;
	// 82DF1730: 2B150000  cmplwi cr6, r21, 0
	ctx.cr[6].compare_u32(ctx.r[21].u32, 0 as u32, &mut ctx.xer);
	// 82DF1734: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF1738: 419A000C  beq cr6, 0x82df1744
	if ctx.cr[6].eq {
	pc = 0x82DF1744; continue 'dispatch;
	}
	// 82DF173C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF1740: 91750000  stw r11, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF1744: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 82DF1748: 3AE0002B  li r23, 0x2b
	ctx.r[23].s64 = 43;
	// 82DF174C: 7EEB0774  extsb r11, r23
	ctx.r[11].s64 = ctx.r[23].s8 as i64;
	// 82DF1750: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 82DF1754: 409A0008  bne cr6, 0x82df175c
	if !ctx.cr[6].eq {
	pc = 0x82DF175C; continue 'dispatch;
	}
	// 82DF1758: 7FBD00D0  neg r29, r29
	ctx.r[29].s64 = -ctx.r[29].s64;
	// 82DF175C: 2B140000  cmplwi cr6, r20, 0
	ctx.cr[6].compare_u32(ctx.r[20].u32, 0 as u32, &mut ctx.xer);
	// 82DF1760: 419A0008  beq cr6, 0x82df1768
	if ctx.cr[6].eq {
	pc = 0x82DF1768; continue 'dispatch;
	}
	// 82DF1764: 93F40000  stw r31, 0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DF1768: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF176C: 48000014  b 0x82df1780
	pc = 0x82DF1780; continue 'dispatch;
	// 82DF1770: 2B140000  cmplwi cr6, r20, 0
	ctx.cr[6].compare_u32(ctx.r[20].u32, 0 as u32, &mut ctx.xer);
	// 82DF1774: 419A0008  beq cr6, 0x82df177c
	if ctx.cr[6].eq {
	pc = 0x82DF177C; continue 'dispatch;
	}
	// 82DF1778: 92D40000  stw r22, 0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[22].u32 ) };
	// 82DF177C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF1780: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DF1784: 483B6A14  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF1788 size=300
    let mut pc: u32 = 0x82DF1788;
    'dispatch: loop {
        match pc {
            0x82DF1788 => {
    //   block [0x82DF1788..0x82DF18B4)
	// 82DF1788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF178C: 483B69D9  bl 0x831a8164
	ctx.lr = 0x82DF1790;
	sub_831A8130(ctx, base);
	// 82DF1790: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF1794: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF1798: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DF179C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DF17A0: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DF17A4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DF17A8: 409A0008  bne cr6, 0x82df17b0
	if !ctx.cr[6].eq {
	pc = 0x82DF17B0; continue 'dispatch;
	}
	// 82DF17AC: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 82DF17B0: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82DF17B4: 483BF4FD  bl 0x831b0cb0
	ctx.lr = 0x82DF17B8;
	sub_831B0CB0(ctx, base);
	// 82DF17B8: 897C0000  lbz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF17BC: 48000010  b 0x82df17cc
	pc = 0x82DF17CC; continue 'dispatch;
	// 82DF17C0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DF17C4: 483BF4ED  bl 0x831b0cb0
	ctx.lr = 0x82DF17C8;
	sub_831B0CB0(ctx, base);
	// 82DF17C8: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF17CC: 556B083E  rotlwi r11, r11, 1
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 82DF17D0: 7D635A2E  lhzx r11, r3, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DF17D4: 556B0739  rlwinm. r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF17D8: 4082FFE8  bne 0x82df17c0
	if !ctx.cr[0].eq {
	pc = 0x82DF17C0; continue 'dispatch;
	}
	// 82DF17DC: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF17E0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DF17E4: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 82DF17E8: 419A0014  beq cr6, 0x82df17fc
	if ctx.cr[6].eq {
	pc = 0x82DF17FC; continue 'dispatch;
	}
	// 82DF17EC: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 82DF17F0: 419A000C  beq cr6, 0x82df17fc
	if ctx.cr[6].eq {
	pc = 0x82DF17FC; continue 'dispatch;
	}
	// 82DF17F4: 3960002B  li r11, 0x2b
	ctx.r[11].s64 = 43;
	// 82DF17F8: 48000008  b 0x82df1800
	pc = 0x82DF1800; continue 'dispatch;
	// 82DF17FC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DF1800: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF1804: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DF1808: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF180C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF1810: 7D7D0774  extsb r29, r11
	ctx.r[29].s64 = ctx.r[11].s8 as i64;
	// 82DF1814: 4BFFFD15  bl 0x82df1528
	ctx.lr = 0x82DF1818;
	sub_82DF1528(ctx, base);
	// 82DF1818: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF181C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF1820: 409A0008  bne cr6, 0x82df1828
	if !ctx.cr[6].eq {
	pc = 0x82DF1828; continue 'dispatch;
	}
	// 82DF1824: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF1828: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF182C: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 82DF1830: 615FFFFF  ori r31, r10, 0xffff
	ctx.r[31].u64 = ctx.r[10].u64 | 65535;
	// 82DF1834: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF1838: 409A000C  bne cr6, 0x82df1844
	if !ctx.cr[6].eq {
	pc = 0x82DF1844; continue 'dispatch;
	}
	// 82DF183C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF1840: 409A002C  bne cr6, 0x82df186c
	if !ctx.cr[6].eq {
	pc = 0x82DF186C; continue 'dispatch;
	}
	// 82DF1844: 7FAB0774  extsb r11, r29
	ctx.r[11].s64 = ctx.r[29].s8 as i64;
	// 82DF1848: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 82DF184C: 409A000C  bne cr6, 0x82df1858
	if !ctx.cr[6].eq {
	pc = 0x82DF1858; continue 'dispatch;
	}
	// 82DF1850: 7F03F840  cmplw cr6, r3, r31
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DF1854: 41990018  bgt cr6, 0x82df186c
	if ctx.cr[6].gt {
	pc = 0x82DF186C; continue 'dispatch;
	}
	// 82DF1858: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 82DF185C: 409A0050  bne cr6, 0x82df18ac
	if !ctx.cr[6].eq {
	pc = 0x82DF18AC; continue 'dispatch;
	}
	// 82DF1860: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82DF1864: 7F035040  cmplw cr6, r3, r10
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF1868: 40990038  ble cr6, 0x82df18a0
	if !ctx.cr[6].gt {
	pc = 0x82DF18A0; continue 'dispatch;
	}
	// 82DF186C: 483BF4F5  bl 0x831b0d60
	ctx.lr = 0x82DF1870;
	sub_831B0D60(ctx, base);
	// 82DF1870: 39600022  li r11, 0x22
	ctx.r[11].s64 = 34;
	// 82DF1874: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82DF1878: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF187C: 419A000C  beq cr6, 0x82df1888
	if ctx.cr[6].eq {
	pc = 0x82DF1888; continue 'dispatch;
	}
	// 82DF1880: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF1884: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF1888: 7FAB0774  extsb r11, r29
	ctx.r[11].s64 = ctx.r[29].s8 as i64;
	// 82DF188C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82DF1890: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 82DF1894: 419A0018  beq cr6, 0x82df18ac
	if ctx.cr[6].eq {
	pc = 0x82DF18AC; continue 'dispatch;
	}
	// 82DF1898: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF189C: 48000010  b 0x82df18ac
	pc = 0x82DF18AC; continue 'dispatch;
	// 82DF18A0: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 82DF18A4: 409A0008  bne cr6, 0x82df18ac
	if !ctx.cr[6].eq {
	pc = 0x82DF18AC; continue 'dispatch;
	}
	// 82DF18A8: 7C6300D0  neg r3, r3
	ctx.r[3].s64 = -ctx.r[3].s64;
	// 82DF18AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF18B0: 483B6904  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF18B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF18B8 size=36
    let mut pc: u32 = 0x82DF18B8;
    'dispatch: loop {
        match pc {
            0x82DF18B8 => {
    //   block [0x82DF18B8..0x82DF18DC)
	// 82DF18B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF18BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF18C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF18C4: 4800001D  bl 0x82df18e0
	ctx.lr = 0x82DF18C8;
	sub_82DF18E0(ctx, base);
	// 82DF18C8: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82DF18CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF18D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF18D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF18D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF18E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF18E0 size=168
    let mut pc: u32 = 0x82DF18E0;
    'dispatch: loop {
        match pc {
            0x82DF18E0 => {
    //   block [0x82DF18E0..0x82DF1988)
	// 82DF18E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF18E4: 483B6881  bl 0x831a8164
	ctx.lr = 0x82DF18E8;
	sub_831A8130(ctx, base);
	// 82DF18E8: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82DF18EC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF18F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF18F4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF18F8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DF18FC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DF1900: 483BF461  bl 0x831b0d60
	ctx.lr = 0x82DF1904;
	sub_831B0D60(ctx, base);
	// 82DF1904: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1908: 483BF459  bl 0x831b0d60
	ctx.lr = 0x82DF190C;
	sub_831B0D60(ctx, base);
	// 82DF190C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF1910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF1914: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF1918: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF191C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF1920: 483B8DC9  bl 0x831aa6e8
	ctx.lr = 0x82DF1924;
	sub_831AA6E8(ctx, base);
	// 82DF1924: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DF1928: 483BF439  bl 0x831b0d60
	ctx.lr = 0x82DF192C;
	sub_831B0D60(ctx, base);
	// 82DF192C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1930: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF1934: 483BF42D  bl 0x831b0d60
	ctx.lr = 0x82DF1938;
	sub_831B0D60(ctx, base);
	// 82DF1938: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DF193C: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82DF1940: 4099001C  ble cr6, 0x82df195c
	if !ctx.cr[6].gt {
	pc = 0x82DF195C; continue 'dispatch;
	}
	// 82DF1944: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82DF1948: C80BD1E0  lfd f0, -0x2e20(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11808 as u32) ) };
	// 82DF194C: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DF1950: FFFF0032  fmul f31, f31, f0
	ctx.f[31].f64 = ctx.f[31].f64 * ctx.f[0].f64;
	// 82DF1954: 4181FFF8  bgt 0x82df194c
	if ctx.cr[0].gt {
	pc = 0x82DF194C; continue 'dispatch;
	}
	// 82DF1958: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DF195C: 4098001C  bge cr6, 0x82df1978
	if !ctx.cr[6].lt {
	pc = 0x82DF1978; continue 'dispatch;
	}
	// 82DF1960: 3D408211  lis r10, -0x7def
	ctx.r[10].s64 = -2112815104;
	// 82DF1964: 7D7F00D0  neg r11, r31
	ctx.r[11].s64 = -ctx.r[31].s64;
	// 82DF1968: C80A8E30  lfd f0, -0x71d0(r10)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-29136 as u32) ) };
	// 82DF196C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF1970: FFFF0032  fmul f31, f31, f0
	ctx.f[31].f64 = ctx.f[31].f64 * ctx.f[0].f64;
	// 82DF1974: 4082FFF8  bne 0x82df196c
	if !ctx.cr[0].eq {
	pc = 0x82DF196C; continue 'dispatch;
	}
	// 82DF1978: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DF197C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF1980: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82DF1984: 483B6830  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1988 size=4
    let mut pc: u32 = 0x82DF1988;
    'dispatch: loop {
        match pc {
            0x82DF1988 => {
    //   block [0x82DF1988..0x82DF198C)
	// 82DF1988: 4BFFFF58  b 0x82df18e0
	sub_82DF18E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF1990 size=40
    let mut pc: u32 = 0x82DF1990;
    'dispatch: loop {
        match pc {
            0x82DF1990 => {
    //   block [0x82DF1990..0x82DF19B8)
	// 82DF1990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF1994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF1998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF199C: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82DF19A0: 483BF581  bl 0x831b0f20
	ctx.lr = 0x82DF19A4;
	sub_831B0F20(ctx, base);
	// 82DF19A4: 7C630734  extsh r3, r3
	ctx.r[3].s64 = ctx.r[3].s16 as i64;
	// 82DF19A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF19AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF19B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF19B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF19B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF19B8 size=72
    let mut pc: u32 = 0x82DF19B8;
    'dispatch: loop {
        match pc {
            0x82DF19B8 => {
    //   block [0x82DF19B8..0x82DF1A00)
	// 82DF19B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF19BC: 483B67B1  bl 0x831a816c
	ctx.lr = 0x82DF19C0;
	sub_831A8130(ctx, base);
	// 82DF19C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF19C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF19C8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF19CC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DF19D0: 4800001C  b 0x82df19ec
	pc = 0x82DF19EC; continue 'dispatch;
	// 82DF19D4: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82DF19D8: A07F0000  lhz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF19DC: 483BF545  bl 0x831b0f20
	ctx.lr = 0x82DF19E0;
	sub_831B0F20(ctx, base);
	// 82DF19E0: B07E0000  sth r3, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 82DF19E4: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82DF19E8: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82DF19EC: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DF19F0: 4198FFE4  blt cr6, 0x82df19d4
	if ctx.cr[6].lt {
	pc = 0x82DF19D4; continue 'dispatch;
	}
	// 82DF19F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF19F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF19FC: 483B67C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1A00 size=12
    let mut pc: u32 = 0x82DF1A00;
    'dispatch: loop {
        match pc {
            0x82DF1A00 => {
    //   block [0x82DF1A00..0x82DF1A0C)
	// 82DF1A00: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82DF1A04: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 82DF1A08: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1A0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1A0C size=12
    let mut pc: u32 = 0x82DF1A0C;
    'dispatch: loop {
        match pc {
            0x82DF1A0C => {
    //   block [0x82DF1A0C..0x82DF1A18)
	// 82DF1A0C: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1A10: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF1A14: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1A18 size=8
    let mut pc: u32 = 0x82DF1A18;
    'dispatch: loop {
        match pc {
            0x82DF1A18 => {
    //   block [0x82DF1A18..0x82DF1A20)
	// 82DF1A18: 2B0B0100  cmplwi cr6, r11, 0x100
	ctx.cr[6].compare_u32(ctx.r[11].u32, 256 as u32, &mut ctx.xer);
	// 82DF1A1C: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1A20 size=8
    let mut pc: u32 = 0x82DF1A20;
    'dispatch: loop {
        match pc {
            0x82DF1A20 => {
    //   block [0x82DF1A20..0x82DF1A28)
	// 82DF1A20: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 82DF1A24: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1A28 size=8
    let mut pc: u32 = 0x82DF1A28;
    'dispatch: loop {
        match pc {
            0x82DF1A28 => {
    //   block [0x82DF1A28..0x82DF1A30)
	// 82DF1A28: 2B0B005A  cmplwi cr6, r11, 0x5a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 90 as u32, &mut ctx.xer);
	// 82DF1A2C: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1A30 size=12
    let mut pc: u32 = 0x82DF1A30;
    'dispatch: loop {
        match pc {
            0x82DF1A30 => {
    //   block [0x82DF1A30..0x82DF1A3C)
	// 82DF1A30: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82DF1A34: 5563043E  clrlwi r3, r11, 0x10
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82DF1A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1A40 size=12
    let mut pc: u32 = 0x82DF1A40;
    'dispatch: loop {
        match pc {
            0x82DF1A40 => {
    //   block [0x82DF1A40..0x82DF1A4C)
	// 82DF1A40: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82DF1A44: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 82DF1A48: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1A4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1A4C size=12
    let mut pc: u32 = 0x82DF1A4C;
    'dispatch: loop {
        match pc {
            0x82DF1A4C => {
    //   block [0x82DF1A4C..0x82DF1A58)
	// 82DF1A4C: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1A50: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF1A54: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1A58 size=8
    let mut pc: u32 = 0x82DF1A58;
    'dispatch: loop {
        match pc {
            0x82DF1A58 => {
    //   block [0x82DF1A58..0x82DF1A60)
	// 82DF1A58: 2B0B0100  cmplwi cr6, r11, 0x100
	ctx.cr[6].compare_u32(ctx.r[11].u32, 256 as u32, &mut ctx.xer);
	// 82DF1A5C: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1A60 size=8
    let mut pc: u32 = 0x82DF1A60;
    'dispatch: loop {
        match pc {
            0x82DF1A60 => {
    //   block [0x82DF1A60..0x82DF1A68)
	// 82DF1A60: 2B0B0061  cmplwi cr6, r11, 0x61
	ctx.cr[6].compare_u32(ctx.r[11].u32, 97 as u32, &mut ctx.xer);
	// 82DF1A64: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1A68 size=8
    let mut pc: u32 = 0x82DF1A68;
    'dispatch: loop {
        match pc {
            0x82DF1A68 => {
    //   block [0x82DF1A68..0x82DF1A70)
	// 82DF1A68: 2B0B007A  cmplwi cr6, r11, 0x7a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 122 as u32, &mut ctx.xer);
	// 82DF1A6C: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1A70 size=16
    let mut pc: u32 = 0x82DF1A70;
    'dispatch: loop {
        match pc {
            0x82DF1A70 => {
    //   block [0x82DF1A70..0x82DF1A80)
	// 82DF1A70: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 82DF1A74: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 82DF1A78: 5563043E  clrlwi r3, r11, 0x10
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82DF1A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1A80 size=52
    let mut pc: u32 = 0x82DF1A80;
    'dispatch: loop {
        match pc {
            0x82DF1A80 => {
    //   block [0x82DF1A80..0x82DF1AB4)
	// 82DF1A80: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82DF1A84: 419A0028  beq cr6, 0x82df1aac
	if ctx.cr[6].eq {
	pc = 0x82DF1AAC; continue 'dispatch;
	}
	// 82DF1A88: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82DF1A8C: 419A0020  beq cr6, 0x82df1aac
	if ctx.cr[6].eq {
	pc = 0x82DF1AAC; continue 'dispatch;
	}
	// 82DF1A90: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1A94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF1A98: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF1A9C: 40820018  bne 0x82df1ab4
	if !ctx.cr[0].eq {
		sub_82DF1AB4(ctx, base);
		return;
	}
	// 82DF1AA0: 419A000C  beq cr6, 0x82df1aac
	if ctx.cr[6].eq {
	pc = 0x82DF1AAC; continue 'dispatch;
	}
	// 82DF1AA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF1AA8: B1630000  sth r11, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82DF1AAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF1AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1AB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1AB4 size=20
    let mut pc: u32 = 0x82DF1AB4;
    'dispatch: loop {
        match pc {
            0x82DF1AB4 => {
    //   block [0x82DF1AB4..0x82DF1AC8)
	// 82DF1AB4: 419A000C  beq cr6, 0x82df1ac0
	if ctx.cr[6].eq {
	pc = 0x82DF1AC0; continue 'dispatch;
	}
	// 82DF1AB8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DF1ABC: B1630000  sth r11, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82DF1AC0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF1AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1AC8 size=8
    let mut pc: u32 = 0x82DF1AC8;
    'dispatch: loop {
        match pc {
            0x82DF1AC8 => {
    //   block [0x82DF1AC8..0x82DF1AD0)
	// 82DF1AC8: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82DF1ACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1AD0 size=20
    let mut pc: u32 = 0x82DF1AD0;
    'dispatch: loop {
        match pc {
            0x82DF1AD0 => {
    //   block [0x82DF1AD0..0x82DF1AE4)
	// 82DF1AD0: 3D6082DF  lis r11, -0x7d21
	ctx.r[11].s64 = -2099314688;
	// 82DF1AD4: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82DF1AD8: 396B1AC8  addi r11, r11, 0x1ac8
	ctx.r[11].s64 = ctx.r[11].s64 + 6856;
	// 82DF1ADC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF1AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1AE8 size=20
    let mut pc: u32 = 0x82DF1AE8;
    'dispatch: loop {
        match pc {
            0x82DF1AE8 => {
    //   block [0x82DF1AE8..0x82DF1AFC)
	// 82DF1AE8: 3D6082B8  lis r11, -0x7d48
	ctx.r[11].s64 = -2101870592;
	// 82DF1AEC: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82DF1AF0: 396B8D20  addi r11, r11, -0x72e0
	ctx.r[11].s64 = ctx.r[11].s64 + -29408;
	// 82DF1AF4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF1AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1B00 size=4
    let mut pc: u32 = 0x82DF1B00;
    'dispatch: loop {
        match pc {
            0x82DF1B00 => {
    //   block [0x82DF1B00..0x82DF1B04)
	// 82DF1B00: 48450EDC  b 0x832429dc
	sub_832429DC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1B08 size=4
    let mut pc: u32 = 0x82DF1B08;
    'dispatch: loop {
        match pc {
            0x82DF1B08 => {
    //   block [0x82DF1B08..0x82DF1B0C)
	// 82DF1B08: 48450E64  b 0x8324296c
	sub_8324296C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1B10 size=4
    let mut pc: u32 = 0x82DF1B10;
    'dispatch: loop {
        match pc {
            0x82DF1B10 => {
    //   block [0x82DF1B10..0x82DF1B14)
	// 82DF1B10: 48450E4C  b 0x8324295c
	sub_8324295C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1B18 size=20
    let mut pc: u32 = 0x82DF1B18;
    'dispatch: loop {
        match pc {
            0x82DF1B18 => {
    //   block [0x82DF1B18..0x82DF1B2C)
	// 82DF1B18: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 82DF1B1C: 816A9DDC  lwz r11, -0x6224(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-25124 as u32) ) } as u64;
	// 82DF1B20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF1B24: 409A0008  bne cr6, 0x82df1b2c
	if !ctx.cr[6].eq {
		sub_82DF1B2C(ctx, base);
		return;
	}
	// 82DF1B28: 483BA020  b 0x831abb48
	sub_831ABB48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1B2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1B2C size=28
    let mut pc: u32 = 0x82DF1B2C;
    'dispatch: loop {
        match pc {
            0x82DF1B2C => {
    //   block [0x82DF1B2C..0x82DF1B48)
	// 82DF1B2C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF1B30: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82DF1B34: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DF1B38: 916A9DDC  stw r11, -0x6224(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-25124 as u32), ctx.r[11].u32 ) };
	// 82DF1B3C: 396956A8  addi r11, r9, 0x56a8
	ctx.r[11].s64 = ctx.r[9].s64 + 22184;
	// 82DF1B40: 7C68592E  stwx r3, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 82DF1B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF1B48 size=124
    let mut pc: u32 = 0x82DF1B48;
    'dispatch: loop {
        match pc {
            0x82DF1B48 => {
    //   block [0x82DF1B48..0x82DF1BC4)
	// 82DF1B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF1B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF1B50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF1B54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF1B58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF1B5C: 3FC08332  lis r30, -0x7cce
	ctx.r[30].s64 = -2093875200;
	// 82DF1B60: 817E9DDC  lwz r11, -0x6224(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-25124 as u32) ) } as u64;
	// 82DF1B64: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 82DF1B68: 40980044  bge cr6, 0x82df1bac
	if !ctx.cr[6].lt {
	pc = 0x82DF1BAC; continue 'dispatch;
	}
	// 82DF1B6C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DF1B70: 3BEA56A8  addi r31, r10, 0x56a8
	ctx.r[31].s64 = ctx.r[10].s64 + 22184;
	// 82DF1B74: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF1B78: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF1B7C: 917E9DDC  stw r11, -0x6224(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-25124 as u32), ctx.r[11].u32 ) };
	// 82DF1B80: 7D4AF82E  lwzx r10, r10, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DF1B84: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF1B88: 419A001C  beq cr6, 0x82df1ba4
	if ctx.cr[6].eq {
	pc = 0x82DF1BA4; continue 'dispatch;
	}
	// 82DF1B8C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF1B90: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DF1B94: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DF1B98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF1B9C: 4E800421  bctrl
	ctx.lr = 0x82DF1BA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF1BA0: 817E9DDC  lwz r11, -0x6224(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-25124 as u32) ) } as u64;
	// 82DF1BA4: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 82DF1BA8: 4198FFCC  blt cr6, 0x82df1b74
	if ctx.cr[6].lt {
	pc = 0x82DF1B74; continue 'dispatch;
	}
	// 82DF1BAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF1BB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF1BB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF1BB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF1BBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF1BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1BC8 size=20
    let mut pc: u32 = 0x82DF1BC8;
    'dispatch: loop {
        match pc {
            0x82DF1BC8 => {
    //   block [0x82DF1BC8..0x82DF1BDC)
	// 82DF1BC8: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DF1BCC: 816A56E8  lwz r11, 0x56e8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(22248 as u32) ) } as u64;
	// 82DF1BD0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF1BD4: 916A56E8  stw r11, 0x56e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(22248 as u32), ctx.r[11].u32 ) };
	// 82DF1BD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1BE0 size=20
    let mut pc: u32 = 0x82DF1BE0;
    'dispatch: loop {
        match pc {
            0x82DF1BE0 => {
    //   block [0x82DF1BE0..0x82DF1BF4)
	// 82DF1BE0: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DF1BE4: 816A56E8  lwz r11, 0x56e8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(22248 as u32) ) } as u64;
	// 82DF1BE8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF1BEC: 916A56E8  stw r11, 0x56e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(22248 as u32), ctx.r[11].u32 ) };
	// 82DF1BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF1BF8 size=120
    let mut pc: u32 = 0x82DF1BF8;
    'dispatch: loop {
        match pc {
            0x82DF1BF8 => {
    //   block [0x82DF1BF8..0x82DF1C70)
	// 82DF1BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF1BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF1C00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF1C04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF1C08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF1C0C: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF1C10: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82DF1C14: 40820024  bne 0x82df1c38
	if !ctx.cr[0].eq {
	pc = 0x82DF1C38; continue 'dispatch;
	}
	// 82DF1C18: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF1C1C: 816B56E8  lwz r11, 0x56e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22248 as u32) ) } as u64;
	// 82DF1C20: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF1C24: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF1C28: 41990008  bgt cr6, 0x82df1c30
	if ctx.cr[6].gt {
	pc = 0x82DF1C30; continue 'dispatch;
	}
	// 82DF1C2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF1C30: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF1C34: 4182001C  beq 0x82df1c50
	if ctx.cr[0].eq {
	pc = 0x82DF1C50; continue 'dispatch;
	}
	// 82DF1C38: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82DF1C3C: 419A0014  beq cr6, 0x82df1c50
	if ctx.cr[6].eq {
	pc = 0x82DF1C50; continue 'dispatch;
	}
	// 82DF1C40: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DF1C44: 48000925  bl 0x82df2568
	ctx.lr = 0x82DF1C48;
	sub_82DF2568(ctx, base);
	// 82DF1C48: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF1C4C: 48000008  b 0x82df1c54
	pc = 0x82DF1C54; continue 'dispatch;
	// 82DF1C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF1C54: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82DF1C58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF1C5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF1C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF1C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF1C68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF1C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1C70 size=32
    let mut pc: u32 = 0x82DF1C70;
    'dispatch: loop {
        match pc {
            0x82DF1C70 => {
    //   block [0x82DF1C70..0x82DF1C90)
	// 82DF1C70: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1C74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF1C78: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF1C7C: 89440004  lbz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF1C80: 99430004  stb r10, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 82DF1C84: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF1C88: 99640004  stb r11, 4(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82DF1C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1C90 size=12
    let mut pc: u32 = 0x82DF1C90;
    'dispatch: loop {
        match pc {
            0x82DF1C90 => {
    //   block [0x82DF1C90..0x82DF1C9C)
	// 82DF1C90: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF1C94: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF1C98: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1C9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1C9C size=12
    let mut pc: u32 = 0x82DF1C9C;
    'dispatch: loop {
        match pc {
            0x82DF1C9C => {
    //   block [0x82DF1C9C..0x82DF1CA8)
	// 82DF1C9C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1CA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF1CA4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1CA8 size=8
    let mut pc: u32 = 0x82DF1CA8;
    'dispatch: loop {
        match pc {
            0x82DF1CA8 => {
    //   block [0x82DF1CA8..0x82DF1CB0)
	// 82DF1CA8: 48000760  b 0x82df2408
	sub_82DF2408(ctx, base);
	return;
	// 82DF1CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1CB0 size=24
    let mut pc: u32 = 0x82DF1CB0;
    'dispatch: loop {
        match pc {
            0x82DF1CB0 => {
    //   block [0x82DF1CB0..0x82DF1CC8)
	// 82DF1CB0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF1CB4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82DF1CB8: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82DF1CBC: 394AB19C  addi r10, r10, -0x4e64
	ctx.r[10].s64 = ctx.r[10].s64 + -20068;
	// 82DF1CC0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF1CC4: 48009464  b 0x82dfb128
	sub_82DFB128(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF1CC8 size=100
    let mut pc: u32 = 0x82DF1CC8;
    'dispatch: loop {
        match pc {
            0x82DF1CC8 => {
    //   block [0x82DF1CC8..0x82DF1D2C)
	// 82DF1CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF1CCC: 483B6499  bl 0x831a8164
	ctx.lr = 0x82DF1CD0;
	sub_831A8130(ctx, base);
	// 82DF1CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF1CD4: 549E043E  clrlwi r30, r4, 0x10
	ctx.r[30].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82DF1CD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF1CDC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DF1CE0: 397E000E  addi r11, r30, 0xe
	ctx.r[11].s64 = ctx.r[30].s64 + 14;
	// 82DF1CE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF1CE8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF1CEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF1CF0: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DF1CF4: 557D103A  slwi r29, r11, 2
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82DF1CF8: 4BDDFAC9  bl 0x82bd17c0
	ctx.lr = 0x82DF1CFC;
	sub_82BD17C0(ctx, base);
	// 82DF1CFC: 7D7EFA14  add r11, r30, r31
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 82DF1D00: 7C7DF92E  stwx r3, r29, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32), ctx.r[3].u32) };
	// 82DF1D04: 395E001F  addi r10, r30, 0x1f
	ctx.r[10].s64 = ctx.r[30].s64 + 31;
	// 82DF1D08: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF1D0C: 9B6B00C0  stb r27, 0xc0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(192 as u32), ctx.r[27].u8 ) };
	// 82DF1D10: 7D7DF82E  lwzx r11, r29, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DF1D14: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DF1D18: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DF1D1C: 7F8AF92E  stwx r28, r10, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[28].u32) };
	// 82DF1D20: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82DF1D24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF1D28: 483B648C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF1D30 size=88
    let mut pc: u32 = 0x82DF1D30;
    'dispatch: loop {
        match pc {
            0x82DF1D30 => {
    //   block [0x82DF1D30..0x82DF1D88)
	// 82DF1D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF1D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF1D38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF1D3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF1D40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF1D44: 548B043E  clrlwi r11, r4, 0x10
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82DF1D48: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF1D4C: 396B000E  addi r11, r11, 0xe
	ctx.r[11].s64 = ctx.r[11].s64 + 14;
	// 82DF1D50: 557F103A  slwi r31, r11, 2
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82DF1D54: 7C7FF02E  lwzx r3, r31, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DF1D58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF1D5C: 419A0010  beq cr6, 0x82df1d6c
	if ctx.cr[6].eq {
	pc = 0x82DF1D6C; continue 'dispatch;
	}
	// 82DF1D60: 4BDDFAE1  bl 0x82bd1840
	ctx.lr = 0x82DF1D64;
	sub_82BD1840(ctx, base);
	// 82DF1D64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF1D68: 7D7FF12E  stwx r11, r31, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u32) };
	// 82DF1D6C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF1D70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF1D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF1D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF1D7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF1D80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF1D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1D88 size=40
    let mut pc: u32 = 0x82DF1D88;
    'dispatch: loop {
        match pc {
            0x82DF1D88 => {
    //   block [0x82DF1D88..0x82DF1DB0)
	// 82DF1D88: 548B043E  clrlwi r11, r4, 0x10
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82DF1D8C: 7D4B1A14  add r10, r11, r3
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82DF1D90: 396B000E  addi r11, r11, 0xe
	ctx.r[11].s64 = ctx.r[11].s64 + 14;
	// 82DF1D94: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF1D98: 894A00C0  lbz r10, 0xc0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(192 as u32) ) } as u64;
	// 82DF1D9C: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82DF1DA0: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DF1DA4: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82DF1DA8: 69440001  xori r4, r10, 1
	ctx.r[4].u64 = ctx.r[10].u64 ^ 1;
	// 82DF1DAC: 4BDDE574  b 0x82bd0320
	sub_82BD0320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1DB0 size=24
    let mut pc: u32 = 0x82DF1DB0;
    'dispatch: loop {
        match pc {
            0x82DF1DB0 => {
    //   block [0x82DF1DB0..0x82DF1DC8)
	// 82DF1DB0: 548B043E  clrlwi r11, r4, 0x10
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82DF1DB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF1DB8: 396B000E  addi r11, r11, 0xe
	ctx.r[11].s64 = ctx.r[11].s64 + 14;
	// 82DF1DBC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF1DC0: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DF1DC4: 4BDDEE2C  b 0x82bd0bf0
	sub_82BD0BF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1DC8 size=8
    let mut pc: u32 = 0x82DF1DC8;
    'dispatch: loop {
        match pc {
            0x82DF1DC8 => {
    //   block [0x82DF1DC8..0x82DF1DD0)
	// 82DF1DC8: 386300D8  addi r3, r3, 0xd8
	ctx.r[3].s64 = ctx.r[3].s64 + 216;
	// 82DF1DCC: 48450BA0  b 0x8324296c
	sub_8324296C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1DD0 size=8
    let mut pc: u32 = 0x82DF1DD0;
    'dispatch: loop {
        match pc {
            0x82DF1DD0 => {
    //   block [0x82DF1DD0..0x82DF1DD8)
	// 82DF1DD0: 386300D8  addi r3, r3, 0xd8
	ctx.r[3].s64 = ctx.r[3].s64 + 216;
	// 82DF1DD4: 48450B88  b 0x8324295c
	sub_8324295C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1DD8 size=36
    let mut pc: u32 = 0x82DF1DD8;
    'dispatch: loop {
        match pc {
            0x82DF1DD8 => {
    //   block [0x82DF1DD8..0x82DF1DFC)
	// 82DF1DD8: 3964000E  addi r11, r4, 0xe
	ctx.r[11].s64 = ctx.r[4].s64 + 14;
	// 82DF1DDC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF1DE0: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DF1DE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF1DE8: 419A0014  beq cr6, 0x82df1dfc
	if ctx.cr[6].eq {
		sub_82DF1DFC(ctx, base);
		return;
	}
	// 82DF1DEC: 3964001F  addi r11, r4, 0x1f
	ctx.r[11].s64 = ctx.r[4].s64 + 31;
	// 82DF1DF0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF1DF4: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DF1DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1DFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1DFC size=8
    let mut pc: u32 = 0x82DF1DFC;
    'dispatch: loop {
        match pc {
            0x82DF1DFC => {
    //   block [0x82DF1DFC..0x82DF1E04)
	// 82DF1DFC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF1E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF1E08 size=160
    let mut pc: u32 = 0x82DF1E08;
    'dispatch: loop {
        match pc {
            0x82DF1E08 => {
    //   block [0x82DF1E08..0x82DF1EA8)
	// 82DF1E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF1E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF1E10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF1E14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF1E18: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF1E1C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82DF1E20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF1E24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF1E28: 57C4043E  clrlwi r4, r30, 0x10
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 82DF1E2C: 80AB9F88  lwz r5, -0x6078(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24696 as u32) ) } as u64;
	// 82DF1E30: 480000D1  bl 0x82df1f00
	ctx.lr = 0x82DF1E34;
	sub_82DF1F00(ctx, base);
	// 82DF1E34: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF1E38: 93DF00D4  stw r30, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[30].u32 ) };
	// 82DF1E3C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82DF1E40: 396BB19C  addi r11, r11, -0x4e64
	ctx.r[11].s64 = ctx.r[11].s64 + -20068;
	// 82DF1E44: 57C5103A  slwi r5, r30, 2
	ctx.r[5].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DF1E48: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF1E4C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF1E50: 483B6391  bl 0x831a81e0
	ctx.lr = 0x82DF1E54;
	sub_831A81E0(ctx, base);
	// 82DF1E54: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 82DF1E58: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF1E5C: 80BF00D4  lwz r5, 0xd4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82DF1E60: 483B6381  bl 0x831a81e0
	ctx.lr = 0x82DF1E64;
	sub_831A81E0(ctx, base);
	// 82DF1E64: 4BDDFA75  bl 0x82bd18d8
	ctx.lr = 0x82DF1E68;
	sub_82BD18D8(ctx, base);
	// 82DF1E68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF1E6C: 907F0038  stw r3, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 82DF1E70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF1E74: 997F00C0  stb r11, 0xc0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u8 ) };
	// 82DF1E78: 4BDDB339  bl 0x82bcd1b0
	ctx.lr = 0x82DF1E7C;
	sub_82BCD1B0(ctx, base);
	// 82DF1E7C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DF1E80: 387F00D8  addi r3, r31, 0xd8
	ctx.r[3].s64 = ctx.r[31].s64 + 216;
	// 82DF1E84: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DF1E88: 48450B55  bl 0x832429dc
	ctx.lr = 0x82DF1E8C;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 82DF1E8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF1E90: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF1E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF1E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF1E9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF1EA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF1EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1EA8 size=4
    let mut pc: u32 = 0x82DF1EA8;
    'dispatch: loop {
        match pc {
            0x82DF1EA8 => {
    //   block [0x82DF1EA8..0x82DF1EAC)
	// 82DF1EA8: 483BECB8  b 0x831b0b60
	sub_831B0B60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1EB0 size=4
    let mut pc: u32 = 0x82DF1EB0;
    'dispatch: loop {
        match pc {
            0x82DF1EB0 => {
    //   block [0x82DF1EB0..0x82DF1EB4)
	// 82DF1EB0: 483B9C28  b 0x831abad8
	sub_831ABAD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1EB8 size=40
    let mut pc: u32 = 0x82DF1EB8;
    'dispatch: loop {
        match pc {
            0x82DF1EB8 => {
    //   block [0x82DF1EB8..0x82DF1EE0)
	// 82DF1EB8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1EBC: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 82DF1EC0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82DF1EC4: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82DF1EC8: 5467043E  clrlwi r7, r3, 0x10
	ctx.r[7].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82DF1ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DF1ED0: 7CEB512E  stwx r7, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[7].u32) };
	// 82DF1ED4: 7CEB492E  stwx r7, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[7].u32) };
	// 82DF1ED8: 7CCB412E  stwx r6, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[6].u32) };
	// 82DF1EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1EE0 size=28
    let mut pc: u32 = 0x82DF1EE0;
    'dispatch: loop {
        match pc {
            0x82DF1EE0 => {
    //   block [0x82DF1EE0..0x82DF1EFC)
	// 82DF1EE0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1EE4: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82DF1EE8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DF1EEC: 5468043E  clrlwi r8, r3, 0x10
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82DF1EF0: 7D0B512E  stwx r8, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82DF1EF4: 7C8B492E  stwx r4, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[4].u32) };
	// 82DF1EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF1F00 size=72
    let mut pc: u32 = 0x82DF1F00;
    'dispatch: loop {
        match pc {
            0x82DF1F00 => {
    //   block [0x82DF1F00..0x82DF1F48)
	// 82DF1F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF1F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF1F08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF1F0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF1F10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF1F14: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF1F18: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82DF1F1C: 396BB1BC  addi r11, r11, -0x4e44
	ctx.r[11].s64 = ctx.r[11].s64 + -20036;
	// 82DF1F20: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82DF1F24: B09F0008  sth r4, 8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u16 ) };
	// 82DF1F28: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF1F2C: 4800939D  bl 0x82dfb2c8
	ctx.lr = 0x82DF1F30;
	sub_82DFB2C8(ctx, base);
	// 82DF1F30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF1F34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF1F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF1F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF1F40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF1F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1F48 size=20
    let mut pc: u32 = 0x82DF1F48;
    'dispatch: loop {
        match pc {
            0x82DF1F48 => {
    //   block [0x82DF1F48..0x82DF1F5C)
	// 82DF1F48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DF1F50: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1F54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF1F58: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1F60 size=8
    let mut pc: u32 = 0x82DF1F60;
    'dispatch: loop {
        match pc {
            0x82DF1F60 => {
    //   block [0x82DF1F60..0x82DF1F68)
	// 82DF1F60: A0630008  lhz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF1F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1F68 size=20
    let mut pc: u32 = 0x82DF1F68;
    'dispatch: loop {
        match pc {
            0x82DF1F68 => {
    //   block [0x82DF1F68..0x82DF1F7C)
	// 82DF1F68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1F6C: 5484043E  clrlwi r4, r4, 0x10
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82DF1F70: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF1F74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF1F78: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1F80 size=8
    let mut pc: u32 = 0x82DF1F80;
    'dispatch: loop {
        match pc {
            0x82DF1F80 => {
    //   block [0x82DF1F80..0x82DF1F88)
	// 82DF1F80: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 82DF1F84: 48009334  b 0x82dfb2b8
	sub_82DFB2B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1F88 size=8
    let mut pc: u32 = 0x82DF1F88;
    'dispatch: loop {
        match pc {
            0x82DF1F88 => {
    //   block [0x82DF1F88..0x82DF1F90)
	// 82DF1F88: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 82DF1F8C: 4800935C  b 0x82dfb2e8
	sub_82DFB2E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1F90 size=8
    let mut pc: u32 = 0x82DF1F90;
    'dispatch: loop {
        match pc {
            0x82DF1F90 => {
    //   block [0x82DF1F90..0x82DF1F98)
	// 82DF1F90: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 82DF1F94: 48009034  b 0x82dfafc8
	sub_82DFAFC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1F98 size=8
    let mut pc: u32 = 0x82DF1F98;
    'dispatch: loop {
        match pc {
            0x82DF1F98 => {
    //   block [0x82DF1F98..0x82DF1FA0)
	// 82DF1F98: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 82DF1F9C: 48008D14  b 0x82dfacb0
	sub_82DFACB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1FA0 size=8
    let mut pc: u32 = 0x82DF1FA0;
    'dispatch: loop {
        match pc {
            0x82DF1FA0 => {
    //   block [0x82DF1FA0..0x82DF1FA8)
	// 82DF1FA0: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 82DF1FA4: 482D5B3C  b 0x830c7ae0
	sub_830C7AE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1FA8 size=56
    let mut pc: u32 = 0x82DF1FA8;
    'dispatch: loop {
        match pc {
            0x82DF1FA8 => {
    //   block [0x82DF1FA8..0x82DF1FE0)
	// 82DF1FA8: 3D008332  lis r8, -0x7cce
	ctx.r[8].s64 = -2093875200;
	// 82DF1FAC: 0CC70000  twi 6, r7, 0
	// 82DF1FB0: 81489F94  lwz r10, -0x606c(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-24684 as u32) ) } as u64;
	// 82DF1FB4: 7D6A2A14  add r11, r10, r5
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82DF1FB8: 7D2B3B96  divwu r9, r11, r7
	ctx.r[9].u32 = ctx.r[11].u32 / ctx.r[7].u32;
	// 82DF1FBC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DF1FC0: 7D2939D6  mullw r9, r9, r7
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82DF1FC4: 7D295851  subf. r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DF1FC8: 40820018  bne 0x82df1fe0
	if !ctx.cr[0].eq {
		sub_82DF1FE0(ctx, base);
		return;
	}
	// 82DF1FCC: B0C50000  sth r6, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 82DF1FD0: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DF1FD4: 81489F94  lwz r10, -0x606c(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-24684 as u32) ) } as u64;
	// 82DF1FD8: B1450002  sth r10, 2(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 82DF1FDC: 48000044  b 0x82df2020
	sub_82DF1FE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF1FE0 size=72
    let mut pc: u32 = 0x82DF1FE0;
    'dispatch: loop {
        match pc {
            0x82DF1FE0 => {
    //   block [0x82DF1FE0..0x82DF2028)
	// 82DF1FE0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DF1FE4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DF1FE8: 419A0008  beq cr6, 0x82df1ff0
	if ctx.cr[6].eq {
	pc = 0x82DF1FF0; continue 'dispatch;
	}
	// 82DF1FEC: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82DF1FF0: 7C6B4B96  divwu r3, r11, r9
	ctx.r[3].u32 = ctx.r[11].u32 / ctx.r[9].u32;
	// 82DF1FF4: 0CC90000  twi 6, r9, 0
	// 82DF1FF8: 7D2349D6  mullw r9, r3, r9
	ctx.r[9].s64 = (ctx.r[3].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82DF1FFC: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82DF2000: 7D2B3850  subf r9, r11, r7
	ctx.r[9].s64 = ctx.r[7].s64 - ctx.r[11].s64;
	// 82DF2004: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DF2008: 7C6B2A14  add r3, r11, r5
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82DF200C: 7D6A1850  subf r11, r10, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[10].s64;
	// 82DF2010: B0CB0000  sth r6, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 82DF2014: 81489F94  lwz r10, -0x606c(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-24684 as u32) ) } as u64;
	// 82DF2018: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DF201C: B14B0002  sth r10, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 82DF2020: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF2024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF2028 size=156
    let mut pc: u32 = 0x82DF2028;
    'dispatch: loop {
        match pc {
            0x82DF2028 => {
    //   block [0x82DF2028..0x82DF20C4)
	// 82DF2028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF202C: 483B6139  bl 0x831a8164
	ctx.lr = 0x82DF2030;
	sub_831A8130(ctx, base);
	// 82DF2030: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF2034: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF2038: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82DF203C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DF2040: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF2044: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82DF2048: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DF204C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DF2050: 48008CC9  bl 0x82dfad18
	ctx.lr = 0x82DF2054;
	sub_82DFAD18(ctx, base);
	// 82DF2054: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF2058: 40820064  bne 0x82df20bc
	if !ctx.cr[0].eq {
	pc = 0x82DF20BC; continue 'dispatch;
	}
	// 82DF205C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2060: 556B073F  clrlwi. r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF2064: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82DF2068: 816B9F94  lwz r11, -0x606c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24684 as u32) ) } as u64;
	// 82DF206C: 4082000C  bne 0x82df2078
	if !ctx.cr[0].eq {
	pc = 0x82DF2078; continue 'dispatch;
	}
	// 82DF2070: 556A073E  clrlwi r10, r11, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82DF2074: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DF2078: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DF207C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DF2080: 38EB0010  addi r7, r11, 0x10
	ctx.r[7].s64 = ctx.r[11].s64 + 16;
	// 82DF2084: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2088: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DF208C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DF2090: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF2094: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2098: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF209C: 4E800421  bctrl
	ctx.lr = 0x82DF20A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF20A0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82DF20A4: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82DF20A8: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82DF20AC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DF20B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF20B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF20B8: 4BFFFEF1  bl 0x82df1fa8
	ctx.lr = 0x82DF20BC;
	sub_82DF1FA8(ctx, base);
	// 82DF20BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF20C0: 483B60F4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF20C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF20C8 size=188
    let mut pc: u32 = 0x82DF20C8;
    'dispatch: loop {
        match pc {
            0x82DF20C8 => {
    //   block [0x82DF20C8..0x82DF2184)
	// 82DF20C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF20CC: 483B6099  bl 0x831a8164
	ctx.lr = 0x82DF20D0;
	sub_831A8130(ctx, base);
	// 82DF20D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF20D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF20D8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DF20DC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DF20E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF20E4: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82DF20E8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DF20EC: 48008C2D  bl 0x82dfad18
	ctx.lr = 0x82DF20F0;
	sub_82DFAD18(ctx, base);
	// 82DF20F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF20F4: 40820088  bne 0x82df217c
	if !ctx.cr[0].eq {
	pc = 0x82DF217C; continue 'dispatch;
	}
	// 82DF20F8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF20FC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DF2100: 7D4B502E  lwzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DF2104: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF2108: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 82DF210C: 41990008  bgt cr6, 0x82df2114
	if ctx.cr[6].gt {
	pc = 0x82DF2114; continue 'dispatch;
	}
	// 82DF2110: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82DF2114: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DF2118: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF211C: 557D043E  clrlwi r29, r11, 0x10
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82DF2120: 554B073F  clrlwi. r11, r10, 0x1c
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF2124: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82DF2128: 816B9F94  lwz r11, -0x606c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24684 as u32) ) } as u64;
	// 82DF212C: 4082000C  bne 0x82df2138
	if !ctx.cr[0].eq {
	pc = 0x82DF2138; continue 'dispatch;
	}
	// 82DF2130: 556A073E  clrlwi r10, r11, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82DF2134: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DF2138: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DF213C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DF2140: 38EB0010  addi r7, r11, 0x10
	ctx.r[7].s64 = ctx.r[11].s64 + 16;
	// 82DF2144: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2148: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DF214C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DF2150: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF2154: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2158: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF215C: 4E800421  bctrl
	ctx.lr = 0x82DF2160;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF2160: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82DF2164: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82DF2168: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82DF216C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DF2170: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF2174: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF2178: 4BFFFE31  bl 0x82df1fa8
	ctx.lr = 0x82DF217C;
	sub_82DF1FA8(ctx, base);
	// 82DF217C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF2180: 483B6034  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF2188 size=120
    let mut pc: u32 = 0x82DF2188;
    'dispatch: loop {
        match pc {
            0x82DF2188 => {
    //   block [0x82DF2188..0x82DF2200)
	// 82DF2188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF218C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF2190: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF2194: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF2198: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF219C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF21A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF21A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DF21A8: 419A0040  beq cr6, 0x82df21e8
	if ctx.cr[6].eq {
	pc = 0x82DF21E8; continue 'dispatch;
	}
	// 82DF21AC: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82DF21B0: 48008DD1  bl 0x82dfaf80
	ctx.lr = 0x82DF21B4;
	sub_82DFAF80(ctx, base);
	// 82DF21B4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF21B8: 40820030  bne 0x82df21e8
	if !ctx.cr[0].eq {
	pc = 0x82DF21E8; continue 'dispatch;
	}
	// 82DF21BC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82DF21C0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF21C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF21C8: 816B9F94  lwz r11, -0x606c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24684 as u32) ) } as u64;
	// 82DF21CC: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF21D0: 7D6BF850  subf r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82DF21D4: A12B0002  lhz r9, 2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DF21D8: A08B0000  lhz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF21DC: 7CA9F850  subf r5, r9, r31
	ctx.r[5].s64 = ctx.r[31].s64 - ctx.r[9].s64;
	// 82DF21E0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82DF21E4: 4E800421  bctrl
	ctx.lr = 0x82DF21E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF21E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF21EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF21F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF21F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF21F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF21FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF2200 size=240
    let mut pc: u32 = 0x82DF2200;
    'dispatch: loop {
        match pc {
            0x82DF2200 => {
    //   block [0x82DF2200..0x82DF22F0)
	// 82DF2200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF2204: 483B5F5D  bl 0x831a8160
	ctx.lr = 0x82DF2208;
	sub_831A8130(ctx, base);
	// 82DF2208: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF220C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF2210: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82DF2214: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DF2218: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82DF221C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DF2220: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2224: 7D5F5B96  divwu r10, r31, r11
	ctx.r[10].u32 = ctx.r[31].u32 / ctx.r[11].u32;
	// 82DF2228: 0CCB0000  twi 6, r11, 0
	// 82DF222C: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82DF2230: 7D6BF851  subf. r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF2234: 40820018  bne 0x82df224c
	if !ctx.cr[0].eq {
	pc = 0x82DF224C; continue 'dispatch;
	}
	// 82DF2238: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF223C: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82DF2240: 48008AD9  bl 0x82dfad18
	ctx.lr = 0x82DF2244;
	sub_82DFAD18(ctx, base);
	// 82DF2244: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF2248: 408200A0  bne 0x82df22e8
	if !ctx.cr[0].eq {
	pc = 0x82DF22E8; continue 'dispatch;
	}
	// 82DF224C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2250: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DF2254: 7D4B502E  lwzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DF2258: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF225C: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 82DF2260: 41990008  bgt cr6, 0x82df2268
	if ctx.cr[6].gt {
	pc = 0x82DF2268; continue 'dispatch;
	}
	// 82DF2264: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82DF2268: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DF226C: 0CDF0000  twi 6, r31, 0
	// 82DF2270: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2274: 557C043E  clrlwi r28, r11, 0x10
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82DF2278: 7D6AFB96  divwu r11, r10, r31
	ctx.r[11].u32 = ctx.r[10].u32 / ctx.r[31].u32;
	// 82DF227C: 7D6BF9D6  mullw r11, r11, r31
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[31].s32 as i64);
	// 82DF2280: 7D6B5051  subf. r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF2284: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82DF2288: 816B9F94  lwz r11, -0x606c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24684 as u32) ) } as u64;
	// 82DF228C: 40820018  bne 0x82df22a4
	if !ctx.cr[0].eq {
	pc = 0x82DF22A4; continue 'dispatch;
	}
	// 82DF2290: 7D4BFB96  divwu r10, r11, r31
	ctx.r[10].u32 = ctx.r[11].u32 / ctx.r[31].u32;
	// 82DF2294: 0CDF0000  twi 6, r31, 0
	// 82DF2298: 7D4AF9D6  mullw r10, r10, r31
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[31].s32 as i64);
	// 82DF229C: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DF22A0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DF22A4: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DF22A8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DF22AC: 7CEBFA14  add r7, r11, r31
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DF22B0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF22B4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82DF22B8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DF22BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF22C0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF22C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF22C8: 4E800421  bctrl
	ctx.lr = 0x82DF22CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF22CC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82DF22D0: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82DF22D4: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82DF22D8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DF22DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF22E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF22E4: 4BFFFCC5  bl 0x82df1fa8
	ctx.lr = 0x82DF22E8;
	sub_82DF1FA8(ctx, base);
	// 82DF22E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF22EC: 483B5EC4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF22F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF22F0 size=208
    let mut pc: u32 = 0x82DF22F0;
    'dispatch: loop {
        match pc {
            0x82DF22F0 => {
    //   block [0x82DF22F0..0x82DF23C0)
	// 82DF22F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF22F4: 483B5E6D  bl 0x831a8160
	ctx.lr = 0x82DF22F8;
	sub_831A8130(ctx, base);
	// 82DF22F8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF22FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF2300: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82DF2304: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DF2308: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82DF230C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DF2310: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2314: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82DF2318: 7D5F5B96  divwu r10, r31, r11
	ctx.r[10].u32 = ctx.r[31].u32 / ctx.r[11].u32;
	// 82DF231C: 0CCB0000  twi 6, r11, 0
	// 82DF2320: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82DF2324: 7D6BF851  subf. r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF2328: 40820018  bne 0x82df2340
	if !ctx.cr[0].eq {
	pc = 0x82DF2340; continue 'dispatch;
	}
	// 82DF232C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF2330: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82DF2334: 480089E5  bl 0x82dfad18
	ctx.lr = 0x82DF2338;
	sub_82DFAD18(ctx, base);
	// 82DF2338: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF233C: 4082007C  bne 0x82df23b8
	if !ctx.cr[0].eq {
	pc = 0x82DF23B8; continue 'dispatch;
	}
	// 82DF2340: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2344: 0CDF0000  twi 6, r31, 0
	// 82DF2348: 7D4BFB96  divwu r10, r11, r31
	ctx.r[10].u32 = ctx.r[11].u32 / ctx.r[31].u32;
	// 82DF234C: 7D4AF9D6  mullw r10, r10, r31
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[31].s32 as i64);
	// 82DF2350: 7D6A5851  subf. r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF2354: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82DF2358: 816B9F94  lwz r11, -0x606c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24684 as u32) ) } as u64;
	// 82DF235C: 40820018  bne 0x82df2374
	if !ctx.cr[0].eq {
	pc = 0x82DF2374; continue 'dispatch;
	}
	// 82DF2360: 7D4BFB96  divwu r10, r11, r31
	ctx.r[10].u32 = ctx.r[11].u32 / ctx.r[31].u32;
	// 82DF2364: 0CDF0000  twi 6, r31, 0
	// 82DF2368: 7D4AF9D6  mullw r10, r10, r31
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[31].s32 as i64);
	// 82DF236C: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DF2370: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DF2374: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DF2378: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DF237C: 7CEBFA14  add r7, r11, r31
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DF2380: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2384: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82DF2388: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DF238C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF2390: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2394: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF2398: 4E800421  bctrl
	ctx.lr = 0x82DF239C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF239C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82DF23A0: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82DF23A4: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82DF23A8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DF23AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF23B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF23B4: 4BFFFBF5  bl 0x82df1fa8
	ctx.lr = 0x82DF23B8;
	sub_82DF1FA8(ctx, base);
	// 82DF23B8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF23BC: 483B5DF4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF23C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF23C0 size=24
    let mut pc: u32 = 0x82DF23C0;
    'dispatch: loop {
        match pc {
            0x82DF23C0 => {
    //   block [0x82DF23C0..0x82DF23D8)
	// 82DF23C0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DF23C4: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82DF23C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF23CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF23D0: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82DF23D4: 4BFFFCF4  b 0x82df20c8
	sub_82DF20C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF23D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF23D8 size=16
    let mut pc: u32 = 0x82DF23D8;
    'dispatch: loop {
        match pc {
            0x82DF23D8 => {
    //   block [0x82DF23D8..0x82DF23E8)
	// 82DF23D8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DF23DC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF23E0: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82DF23E4: 4BFFFDA4  b 0x82df2188
	sub_82DF2188(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF23E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF23E8 size=24
    let mut pc: u32 = 0x82DF23E8;
    'dispatch: loop {
        match pc {
            0x82DF23E8 => {
    //   block [0x82DF23E8..0x82DF2400)
	// 82DF23E8: 54CB043F  clrlwi. r11, r6, 0x10
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF23EC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DF23F0: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82DF23F4: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82DF23F8: 41820008  beq 0x82df2400
	if ctx.cr[0].eq {
		sub_82DF2400(ctx, base);
		return;
	}
	// 82DF23FC: 4BFFFC2C  b 0x82df2028
	sub_82DF2028(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF2400 size=8
    let mut pc: u32 = 0x82DF2400;
    'dispatch: loop {
        match pc {
            0x82DF2400 => {
    //   block [0x82DF2400..0x82DF2408)
	// 82DF2400: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82DF2404: 4BFFFCC4  b 0x82df20c8
	sub_82DF20C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF2408 size=24
    let mut pc: u32 = 0x82DF2408;
    'dispatch: loop {
        match pc {
            0x82DF2408 => {
    //   block [0x82DF2408..0x82DF2420)
	// 82DF2408: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF240C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2410: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2414: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF2418: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF241C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF2420 size=88
    let mut pc: u32 = 0x82DF2420;
    'dispatch: loop {
        match pc {
            0x82DF2420 => {
    //   block [0x82DF2420..0x82DF2478)
	// 82DF2420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF2424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF2428: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF242C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF2430: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF2434: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF2438: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF243C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2440: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF2444: 419A0008  beq cr6, 0x82df244c
	if ctx.cr[6].eq {
	pc = 0x82DF244C; continue 'dispatch;
	}
	// 82DF2448: 4B4CE449  bl 0x822c0890
	ctx.lr = 0x82DF244C;
	sub_822C0890(ctx, base);
	// 82DF244C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF2450: 4182000C  beq 0x82df245c
	if ctx.cr[0].eq {
	pc = 0x82DF245C; continue 'dispatch;
	}
	// 82DF2454: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF2458: 4BFFFF81  bl 0x82df23d8
	ctx.lr = 0x82DF245C;
	sub_82DF23D8(ctx, base);
	// 82DF245C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF2460: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF2464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF2468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF246C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF2470: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF2474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF2478 size=92
    let mut pc: u32 = 0x82DF2478;
    'dispatch: loop {
        match pc {
            0x82DF2478 => {
    //   block [0x82DF2478..0x82DF24D4)
	// 82DF2478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF247C: 483B5CF1  bl 0x831a816c
	ctx.lr = 0x82DF2480;
	sub_831A8130(ctx, base);
	// 82DF2480: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF2484: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DF2488: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF248C: 806B16D4  lwz r3, 0x16d4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5844 as u32) ) } as u64;
	// 82DF2490: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2494: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2498: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF249C: 4E800421  bctrl
	ctx.lr = 0x82DF24A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF24A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF24A4: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82DF24A8: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DF24AC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF24B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF24B4: 4B4DD02D  bl 0x822cf4e0
	ctx.lr = 0x82DF24B8;
	sub_822CF4E0(ctx, base);
	// 82DF24B8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF24BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF24C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF24C4: 4B4CDB3D  bl 0x822c0000
	ctx.lr = 0x82DF24C8;
	sub_822C0000(ctx, base);
	// 82DF24C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF24CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF24D0: 483B5CEC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF24D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF24D8 size=116
    let mut pc: u32 = 0x82DF24D8;
    'dispatch: loop {
        match pc {
            0x82DF24D8 => {
    //   block [0x82DF24D8..0x82DF254C)
	// 82DF24D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF24DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF24E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF24E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF24E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF24EC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DF24F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF24F4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF24F8: 816B16D4  lwz r11, 0x16d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5844 as u32) ) } as u64;
	// 82DF24FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF2500: 419A0034  beq cr6, 0x82df2534
	if ctx.cr[6].eq {
	pc = 0x82DF2534; continue 'dispatch;
	}
	// 82DF2504: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF2508: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DF250C: 388BB1D4  addi r4, r11, -0x4e2c
	ctx.r[4].s64 = ctx.r[11].s64 + -20012;
	// 82DF2510: 38A00026  li r5, 0x26
	ctx.r[5].s64 = 38;
	// 82DF2514: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82DF2518: 4BFFFED1  bl 0x82df23e8
	ctx.lr = 0x82DF251C;
	sub_82DF23E8(ctx, base);
	// 82DF251C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF2520: 4182000C  beq 0x82df252c
	if ctx.cr[0].eq {
	pc = 0x82DF252C; continue 'dispatch;
	}
	// 82DF2524: 4BFFFF55  bl 0x82df2478
	ctx.lr = 0x82DF2528;
	sub_82DF2478(ctx, base);
	// 82DF2528: 48000008  b 0x82df2530
	pc = 0x82DF2530; continue 'dispatch;
	// 82DF252C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF2530: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DF2534: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF2538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF253C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF2540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF2544: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF2548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF2550 size=12
    let mut pc: u32 = 0x82DF2550;
    'dispatch: loop {
        match pc {
            0x82DF2550 => {
    //   block [0x82DF2550..0x82DF255C)
	// 82DF2550: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2554: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF2558: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF255C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF255C size=8
    let mut pc: u32 = 0x82DF255C;
    'dispatch: loop {
        match pc {
            0x82DF255C => {
    //   block [0x82DF255C..0x82DF2564)
	// 82DF255C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF2560: 4BFFFEC0  b 0x82df2420
	sub_82DF2420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2564(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF2564 size=4
    let mut pc: u32 = 0x82DF2564;
    'dispatch: loop {
        match pc {
            0x82DF2564 => {
    //   block [0x82DF2564..0x82DF2568)
	// 82DF2564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF2568 size=124
    let mut pc: u32 = 0x82DF2568;
    'dispatch: loop {
        match pc {
            0x82DF2568 => {
    //   block [0x82DF2568..0x82DF25E4)
	// 82DF2568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF256C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF2570: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF2574: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF2578: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF257C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2580: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF2584: 409A0034  bne cr6, 0x82df25b8
	if !ctx.cr[6].eq {
	pc = 0x82DF25B8; continue 'dispatch;
	}
	// 82DF2588: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF258C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DF2590: 388BB1D4  addi r4, r11, -0x4e2c
	ctx.r[4].s64 = ctx.r[11].s64 + -20012;
	// 82DF2594: 38A00041  li r5, 0x41
	ctx.r[5].s64 = 65;
	// 82DF2598: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82DF259C: 4BFFFE4D  bl 0x82df23e8
	ctx.lr = 0x82DF25A0;
	sub_82DF23E8(ctx, base);
	// 82DF25A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF25A4: 4182000C  beq 0x82df25b0
	if ctx.cr[0].eq {
	pc = 0x82DF25B0; continue 'dispatch;
	}
	// 82DF25A8: 4BFFFED1  bl 0x82df2478
	ctx.lr = 0x82DF25AC;
	sub_82DF2478(ctx, base);
	// 82DF25AC: 48000008  b 0x82df25b4
	pc = 0x82DF25B4; continue 'dispatch;
	// 82DF25B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF25B4: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DF25B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF25BC: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF25C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF25C4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF25C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF25CC: 4E800421  bctrl
	ctx.lr = 0x82DF25D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF25D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF25D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF25D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF25DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF25E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF25E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF25E8 size=4
    let mut pc: u32 = 0x82DF25E8;
    'dispatch: loop {
        match pc {
            0x82DF25E8 => {
    //   block [0x82DF25E8..0x82DF25EC)
	// 82DF25E8: 4BDDA310  b 0x82bcc8f8
	sub_82BCC8F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF25F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF25F0 size=36
    let mut pc: u32 = 0x82DF25F0;
    'dispatch: loop {
        match pc {
            0x82DF25F0 => {
    //   block [0x82DF25F0..0x82DF2614)
	// 82DF25F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF25F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF25F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF25FC: 480072BD  bl 0x82df98b8
	ctx.lr = 0x82DF2600;
	sub_82DF98B8(ctx, base);
	// 82DF2600: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF2604: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF2608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF260C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF2610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF2618 size=8
    let mut pc: u32 = 0x82DF2618;
    'dispatch: loop {
        match pc {
            0x82DF2618 => {
    //   block [0x82DF2618..0x82DF2620)
	// 82DF2618: 8324280C  lwz r25, 0x280c(r4)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(10252 as u32) ) } as u64;
	// 82DF261C: 8224BE38  lwz r17, -0x41c8(r4)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-16840 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF2620 size=128
    let mut pc: u32 = 0x82DF2620;
    'dispatch: loop {
        match pc {
            0x82DF2620 => {
    //   block [0x82DF2620..0x82DF26A0)
	// 82DF2620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF2624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF2628: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF262C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF2630: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82DF2634: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF2638: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF263C: 39601000  li r11, 0x1000
	ctx.r[11].s64 = 4096;
	// 82DF2640: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DF2644: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DF2648: 48000B69  bl 0x82df31b0
	ctx.lr = 0x82DF264C;
	sub_82DF31B0(ctx, base);
	// 82DF264C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF2650: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82DF2654: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82DF2658: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DF265C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82DF2660: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82DF2664: 38DF0050  addi r6, r31, 0x50
	ctx.r[6].s64 = ctx.r[31].s64 + 80;
	// 82DF2668: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82DF266C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF2670: 3C60406D  lis r3, 0x406d
	ctx.r[3].s64 = 1080885248;
	// 82DF2674: 60631388  ori r3, r3, 0x1388
	ctx.r[3].u64 = ctx.r[3].u64 | 5000;
	// 82DF2678: 483D2A59  bl 0x831c50d0
	ctx.lr = 0x82DF267C;
	sub_831C50D0(ctx, base);
	// 82DF267C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82DF2680: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82DF2684: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82DF2688: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82DF268C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF2690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF2694: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF2698: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF269C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF26A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF26A0 size=12
    let mut pc: u32 = 0x82DF26A0;
    'dispatch: loop {
        match pc {
            0x82DF26A0 => {
    //   block [0x82DF26A0..0x82DF26AC)
	// 82DF26A0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DF26A4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82DF26A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF26B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF26B0 size=20
    let mut pc: u32 = 0x82DF26B0;
    'dispatch: loop {
        match pc {
            0x82DF26B0 => {
    //   block [0x82DF26B0..0x82DF26C4)
	// 82DF26B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF26B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DF26B8: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF26BC: 994B0010  stb r10, 0x10(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 82DF26C0: 483D2A90  b 0x831c5150
	sub_831C5150(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF26C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF26C8 size=12
    let mut pc: u32 = 0x82DF26C8;
    'dispatch: loop {
        match pc {
            0x82DF26C8 => {
    //   block [0x82DF26C8..0x82DF26D4)
	// 82DF26C8: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF26CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF26D0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF26D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF26D4 size=12
    let mut pc: u32 = 0x82DF26D4;
    'dispatch: loop {
        match pc {
            0x82DF26D4 => {
    //   block [0x82DF26D4..0x82DF26E0)
	// 82DF26D4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82DF26D8: 419A0008  beq cr6, 0x82df26e0
	if ctx.cr[6].eq {
		sub_82DF26E0(ctx, base);
		return;
	}
	// 82DF26DC: 4BDDB624  b 0x82bcdd00
	sub_82BCDD00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF26E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF26E0 size=8
    let mut pc: u32 = 0x82DF26E0;
    'dispatch: loop {
        match pc {
            0x82DF26E0 => {
    //   block [0x82DF26E0..0x82DF26E8)
	// 82DF26E0: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82DF26E4: 4BDDB61C  b 0x82bcdd00
	sub_82BCDD00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF26E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF26E8 size=4
    let mut pc: u32 = 0x82DF26E8;
    'dispatch: loop {
        match pc {
            0x82DF26E8 => {
    //   block [0x82DF26E8..0x82DF26EC)
	// 82DF26E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


