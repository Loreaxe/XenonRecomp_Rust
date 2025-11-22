pub fn sub_8317AAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317AAC0 size=160
    let mut pc: u32 = 0x8317AAC0;
    'dispatch: loop {
        match pc {
            0x8317AAC0 => {
    //   block [0x8317AAC0..0x8317AB60)
	// 8317AAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317AAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317AAC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317AACC: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8317AAD0: 4BFFFC69  bl 0x8317a738
	ctx.lr = 0x8317AAD4;
	sub_8317A738(ctx, base);
	// 8317AAD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317AAD8: 419A0078  beq cr6, 0x8317ab50
	if ctx.cr[6].eq {
	pc = 0x8317AB50; continue 'dispatch;
	}
	// 8317AADC: 81691784  lwz r11, 0x1784(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(6020 as u32) ) } as u64;
	// 8317AAE0: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8317AAE4: 79480040  clrldi r8, r10, 1
	ctx.r[8].u64 = ctx.r[10].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 8317AAE8: E94B0018  ld r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 8317AAEC: 7F2A4000  cmpd cr6, r10, r8
	ctx.cr[6].compare_i64(ctx.r[10].s64, ctx.r[8].s64, &mut ctx.xer);
	// 8317AAF0: 419A0060  beq cr6, 0x8317ab50
	if ctx.cr[6].eq {
	pc = 0x8317AB50; continue 'dispatch;
	}
	// 8317AAF4: E9030020  ld r8, 0x20(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 8317AAF8: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 8317AAFC: F9490EE0  std r10, 0xee0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(3808 as u32), ctx.r[10].u64 ) };
	// 8317AB00: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317AB04: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8317AB08: 409A0048  bne cr6, 0x8317ab50
	if !ctx.cr[6].eq {
	pc = 0x8317AB50; continue 'dispatch;
	}
	// 8317AB0C: 81490928  lwz r10, 0x928(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2344 as u32) ) } as u64;
	// 8317AB10: 1D4A0032  mulli r10, r10, 0x32
	ctx.r[10].s64 = ctx.r[10].s64 * 50;
	// 8317AB14: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8317AB18: 8149092C  lwz r10, 0x92c(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2348 as u32) ) } as u64;
	// 8317AB1C: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8317AB20: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317AB24: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8317AB28: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317AB2C: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8317AB30: E9490ED8  ld r10, 0xed8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(3800 as u32) ) };
	// 8317AB34: F9430018  std r10, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u64 ) };
	// 8317AB38: E94B0010  ld r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 8317AB3C: F9430020  std r10, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u64 ) };
	// 8317AB40: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8317AB44: 91430028  stw r10, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8317AB48: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8317AB4C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8317AB50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317AB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317AB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317AB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317AB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317AB60 size=156
    let mut pc: u32 = 0x8317AB60;
    'dispatch: loop {
        match pc {
            0x8317AB60 => {
    //   block [0x8317AB60..0x8317ABFC)
	// 8317AB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317AB64: 4802D609  bl 0x831a816c
	ctx.lr = 0x8317AB68;
	sub_831A8130(ctx, base);
	// 8317AB68: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317AB6C: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 8317AB70: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8317AB74: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8317AB78: 4BFFFBC1  bl 0x8317a738
	ctx.lr = 0x8317AB7C;
	sub_8317A738(ctx, base);
	// 8317AB7C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317AB80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317AB84: 419A0070  beq cr6, 0x8317abf4
	if ctx.cr[6].eq {
	pc = 0x8317ABF4; continue 'dispatch;
	}
	// 8317AB88: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317AB8C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8317AB90: 409A0064  bne cr6, 0x8317abf4
	if !ctx.cr[6].eq {
	pc = 0x8317ABF4; continue 'dispatch;
	}
	// 8317AB94: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8317AB98: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 8317AB9C: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 8317ABA0: 480142D9  bl 0x8318ee78
	ctx.lr = 0x8317ABA4;
	sub_8318EE78(ctx, base);
	// 8317ABA4: 2F1E00B0  cmpwi cr6, r30, 0xb0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 176, &mut ctx.xer);
	// 8317ABA8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8317ABAC: 41980008  blt cr6, 0x8317abb4
	if ctx.cr[6].lt {
	pc = 0x8317ABB4; continue 'dispatch;
	}
	// 8317ABB0: 38A000B0  li r5, 0xb0
	ctx.r[5].s64 = 176;
	// 8317ABB4: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8317ABB8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317ABBC: 4099001C  ble cr6, 0x8317abd8
	if !ctx.cr[6].gt {
	pc = 0x8317ABD8; continue 'dispatch;
	}
	// 8317ABC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317ABC4: 90BF0160  stw r5, 0x160(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(352 as u32), ctx.r[5].u32 ) };
	// 8317ABC8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8317ABCC: 48013DCD  bl 0x8318e998
	ctx.lr = 0x8317ABD0;
	sub_8318E998(ctx, base);
	// 8317ABD0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8317ABD4: 4802D5E8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317ABD8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8317ABDC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317ABE0: 40990014  ble cr6, 0x8317abf4
	if !ctx.cr[6].gt {
	pc = 0x8317ABF4; continue 'dispatch;
	}
	// 8317ABE4: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 8317ABE8: 90BF0164  stw r5, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[5].u32 ) };
	// 8317ABEC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8317ABF0: 48013DA9  bl 0x8318e998
	ctx.lr = 0x8317ABF4;
	sub_8318E998(ctx, base);
	// 8317ABF4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8317ABF8: 4802D5C4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317AC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317AC00 size=124
    let mut pc: u32 = 0x8317AC00;
    'dispatch: loop {
        match pc {
            0x8317AC00 => {
    //   block [0x8317AC00..0x8317AC7C)
	// 8317AC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317AC04: 4802D569  bl 0x831a816c
	ctx.lr = 0x8317AC08;
	sub_831A8130(ctx, base);
	// 8317AC08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317AC0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317AC10: 4BFFFB29  bl 0x8317a738
	ctx.lr = 0x8317AC14;
	sub_8317A738(ctx, base);
	// 8317AC14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317AC18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8317AC1C: 419A0054  beq cr6, 0x8317ac70
	if ctx.cr[6].eq {
	pc = 0x8317AC70; continue 'dispatch;
	}
	// 8317AC20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317AC24: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317AC28: 419A0048  beq cr6, 0x8317ac70
	if ctx.cr[6].eq {
	pc = 0x8317AC70; continue 'dispatch;
	}
	// 8317AC2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317AC30: 83BE1784  lwz r29, 0x1784(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(6020 as u32) ) } as u64;
	// 8317AC34: 48014CD5  bl 0x8318f908
	ctx.lr = 0x8317AC38;
	sub_8318F908(ctx, base);
	// 8317AC38: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8317AC3C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8317AC40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317AC44: 4BFFF4C5  bl 0x8317a108
	ctx.lr = 0x8317AC48;
	sub_8317A108(ctx, base);
	// 8317AC48: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317AC4C: 409A0028  bne cr6, 0x8317ac74
	if !ctx.cr[6].eq {
	pc = 0x8317AC74; continue 'dispatch;
	}
	// 8317AC50: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8317AC54: 917D002C  stw r11, 0x2c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8317AC58: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8317AC5C: 917D0030  stw r11, 0x30(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8317AC60: E97F0018  ld r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	// 8317AC64: F97E0ED8  std r11, 0xed8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(3800 as u32), ctx.r[11].u64 ) };
	// 8317AC68: E97F0020  ld r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	// 8317AC6C: F97D0010  std r11, 0x10(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 8317AC70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317AC74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317AC78: 4802D544  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317AC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317AC80 size=408
    let mut pc: u32 = 0x8317AC80;
    'dispatch: loop {
        match pc {
            0x8317AC80 => {
    //   block [0x8317AC80..0x8317AE18)
	// 8317AC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317AC84: 4802D4D1  bl 0x831a8154
	ctx.lr = 0x8317AC88;
	sub_831A8130(ctx, base);
	// 8317AC88: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317AC8C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8317AC90: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8317AC94: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 8317AC98: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8317AC9C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8317ACA0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8317ACA4: 93990000  stw r28, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8317ACA8: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8317ACAC: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8317ACB0: 7F98E378  mr r24, r28
	ctx.r[24].u64 = ctx.r[28].u64;
	// 8317ACB4: 83DD1784  lwz r30, 0x1784(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(6020 as u32) ) } as u64;
	// 8317ACB8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317ACBC: 4801423D  bl 0x8318eef8
	ctx.lr = 0x8317ACC0;
	sub_8318EEF8(ctx, base);
	// 8317ACC0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317ACC4: 419A0018  beq cr6, 0x8317acdc
	if ctx.cr[6].eq {
	pc = 0x8317ACDC; continue 'dispatch;
	}
	// 8317ACC8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317ACCC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317ACD0: 60840D06  ori r4, r4, 0xd06
	ctx.r[4].u64 = ctx.r[4].u64 | 3334;
	// 8317ACD4: 4800C825  bl 0x831874f8
	ctx.lr = 0x8317ACD8;
	sub_831874F8(ctx, base);
	// 8317ACD8: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8317ACDC: 80C10060  lwz r6, 0x60(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8317ACE0: 81010074  lwz r8, 0x74(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8317ACE4: 3966FF44  addi r11, r6, -0xbc
	ctx.r[11].s64 = ctx.r[6].s64 + -188;
	// 8317ACE8: 7D174378  mr r23, r8
	ctx.r[23].u64 = ctx.r[8].u64;
	// 8317ACEC: 2B0B0043  cmplwi cr6, r11, 0x43
	ctx.cr[6].compare_u32(ctx.r[11].u32, 67 as u32, &mut ctx.xer);
	// 8317ACF0: 41990108  bgt cr6, 0x8317adf8
	if ctx.cr[6].gt {
	pc = 0x8317ADF8; continue 'dispatch;
	}
	// 8317ACF4: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8317ACF8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8317ACFC: 419800F0  blt cr6, 0x8317adec
	if ctx.cr[6].lt {
	pc = 0x8317ADEC; continue 'dispatch;
	}
	// 8317AD00: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 8317AD04: 409800E8  bge cr6, 0x8317adec
	if !ctx.cr[6].lt {
	pc = 0x8317ADEC; continue 'dispatch;
	}
	// 8317AD08: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8317AD0C: 4098001C  bge cr6, 0x8317ad28
	if !ctx.cr[6].lt {
	pc = 0x8317AD28; continue 'dispatch;
	}
	// 8317AD10: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317AD14: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317AD18: 60840D0E  ori r4, r4, 0xd0e
	ctx.r[4].u64 = ctx.r[4].u64 | 3342;
	// 8317AD1C: 4800C7DD  bl 0x831874f8
	ctx.lr = 0x8317AD20;
	sub_831874F8(ctx, base);
	// 8317AD20: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8317AD24: 4802D480  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
	// 8317AD28: 409A001C  bne cr6, 0x8317ad44
	if !ctx.cr[6].eq {
	pc = 0x8317AD44; continue 'dispatch;
	}
	// 8317AD2C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8317AD30: 93990000  stw r28, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8317AD34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317AD38: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317AD3C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8317AD40: 4802D464  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
	// 8317AD44: 7F1A4000  cmpw cr6, r26, r8
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8317AD48: 4098001C  bge cr6, 0x8317ad64
	if !ctx.cr[6].lt {
	pc = 0x8317AD64; continue 'dispatch;
	}
	// 8317AD4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317AD50: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317AD54: 4BFFEEE5  bl 0x83179c38
	ctx.lr = 0x8317AD58;
	sub_83179C38(ctx, base);
	// 8317AD58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317AD5C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8317AD60: 4802D444  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
	// 8317AD64: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 8317AD68: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317AD6C: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8317AD70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317AD74: 419A0018  beq cr6, 0x8317ad8c
	if ctx.cr[6].eq {
	pc = 0x8317AD8C; continue 'dispatch;
	}
	// 8317AD78: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8317AD7C: 80BE0154  lwz r5, 0x154(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(340 as u32) ) } as u64;
	// 8317AD80: 809E0150  lwz r4, 0x150(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(336 as u32) ) } as u64;
	// 8317AD84: 4BFFFAB5  bl 0x8317a838
	ctx.lr = 0x8317AD88;
	sub_8317A838(ctx, base);
	// 8317AD88: 48000030  b 0x8317adb8
	pc = 0x8317ADB8; continue 'dispatch;
	// 8317AD8C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317AD90: E8E10050  ld r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8317AD94: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8317AD98: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8317AD9C: 396BACE0  addi r11, r11, -0x5320
	ctx.r[11].s64 = ctx.r[11].s64 + -21280;
	// 8317ADA0: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 8317ADA4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8317ADA8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317ADAC: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8317ADB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317ADB4: 4E800421  bctrl
	ctx.lr = 0x8317ADB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317ADB8: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317ADBC: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8317ADC0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8317ADC4: 4198001C  blt cr6, 0x8317ade0
	if ctx.cr[6].lt {
	pc = 0x8317ADE0; continue 'dispatch;
	}
	// 8317ADC8: 419A0014  beq cr6, 0x8317addc
	if ctx.cr[6].eq {
	pc = 0x8317ADDC; continue 'dispatch;
	}
	// 8317ADCC: 7D785B78  mr r24, r11
	ctx.r[24].u64 = ctx.r[11].u64;
	// 8317ADD0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8317ADD4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8317ADD8: 4802D3CC  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
	// 8317ADDC: 92F90000  stw r23, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 8317ADE0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8317ADE4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8317ADE8: 4802D3BC  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
	// 8317ADEC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317ADF0: 60840D10  ori r4, r4, 0xd10
	ctx.r[4].u64 = ctx.r[4].u64 | 3344;
	// 8317ADF4: 4800000C  b 0x8317ae00
	pc = 0x8317AE00; continue 'dispatch;
	// 8317ADF8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317ADFC: 60840D0F  ori r4, r4, 0xd0f
	ctx.r[4].u64 = ctx.r[4].u64 | 3343;
	// 8317AE00: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8317AE04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317AE08: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317AE0C: 4800C6ED  bl 0x831874f8
	ctx.lr = 0x8317AE10;
	sub_831874F8(ctx, base);
	// 8317AE10: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8317AE14: 4802D390  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317AE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317AE18 size=116
    let mut pc: u32 = 0x8317AE18;
    'dispatch: loop {
        match pc {
            0x8317AE18 => {
    //   block [0x8317AE18..0x8317AE8C)
	// 8317AE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317AE1C: 4802D34D  bl 0x831a8168
	ctx.lr = 0x8317AE20;
	sub_831A8130(ctx, base);
	// 8317AE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317AE24: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8317AE28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317AE2C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8317AE30: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8317AE34: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8317AE38: 48014B31  bl 0x8318f968
	ctx.lr = 0x8317AE3C;
	sub_8318F968(ctx, base);
	// 8317AE3C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317AE40: 419A0030  beq cr6, 0x8317ae70
	if ctx.cr[6].eq {
	pc = 0x8317AE70; continue 'dispatch;
	}
	// 8317AE44: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317AE48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317AE4C: 419A0018  beq cr6, 0x8317ae64
	if ctx.cr[6].eq {
	pc = 0x8317AE64; continue 'dispatch;
	}
	// 8317AE50: 38DD0012  addi r6, r29, 0x12
	ctx.r[6].s64 = ctx.r[29].s64 + 18;
	// 8317AE54: 38BEFFEE  addi r5, r30, -0x12
	ctx.r[5].s64 = ctx.r[30].s64 + -18;
	// 8317AE58: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317AE5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317AE60: 4BFFFBB1  bl 0x8317aa10
	ctx.lr = 0x8317AE64;
	sub_8317AA10(ctx, base);
	// 8317AE64: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317AE68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317AE6C: 4802D34C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8317AE70: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8317AE74: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8317AE78: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8317AE7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317AE80: 4BFFFB91  bl 0x8317aa10
	ctx.lr = 0x8317AE84;
	sub_8317AA10(ctx, base);
	// 8317AE84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317AE88: 4802D330  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317AE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317AE90 size=84
    let mut pc: u32 = 0x8317AE90;
    'dispatch: loop {
        match pc {
            0x8317AE90 => {
    //   block [0x8317AE90..0x8317AEE4)
	// 8317AE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317AE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317AE98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317AE9C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317AEA0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317AEA4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8317AEA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317AEAC: 4BFFF055  bl 0x83179f00
	ctx.lr = 0x8317AEB0;
	sub_83179F00(ctx, base);
	// 8317AEB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317AEB4: 4BFFF6FD  bl 0x8317a5b0
	ctx.lr = 0x8317AEB8;
	sub_8317A5B0(ctx, base);
	// 8317AEB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317AEBC: 4BFFF0C5  bl 0x83179f80
	ctx.lr = 0x8317AEC0;
	sub_83179F80(ctx, base);
	// 8317AEC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317AEC4: 4BFFEF35  bl 0x83179df8
	ctx.lr = 0x8317AEC8;
	sub_83179DF8(ctx, base);
	// 8317AEC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317AECC: 4BFFFBF5  bl 0x8317aac0
	ctx.lr = 0x8317AED0;
	sub_8317AAC0(ctx, base);
	// 8317AED0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317AED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317AED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317AEDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317AEE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317AEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8317AEE8 size=672
    let mut pc: u32 = 0x8317AEE8;
    'dispatch: loop {
        match pc {
            0x8317AEE8 => {
    //   block [0x8317AEE8..0x8317B188)
	// 8317AEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317AEEC: 4802D261  bl 0x831a814c
	ctx.lr = 0x8317AEF0;
	sub_831A8130(ctx, base);
	// 8317AEF0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317AEF4: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8317AEF8: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 8317AEFC: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8317AF00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317AF04: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 8317AF08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317AF0C: 92D90000  stw r22, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[22].u32 ) };
	// 8317AF10: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8317AF14: 92DC0000  stw r22, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[22].u32 ) };
	// 8317AF18: 7ED5B378  mr r21, r22
	ctx.r[21].u64 = ctx.r[22].u64;
	// 8317AF1C: 831F1784  lwz r24, 0x1784(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6020 as u32) ) } as u64;
	// 8317AF20: 83780000  lwz r27, 0(r24)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317AF24: 4BFFF575  bl 0x8317a498
	ctx.lr = 0x8317AF28;
	sub_8317A498(ctx, base);
	// 8317AF28: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317AF2C: 409A000C  bne cr6, 0x8317af38
	if !ctx.cr[6].eq {
	pc = 0x8317AF38; continue 'dispatch;
	}
	// 8317AF30: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8317AF34: 4802D268  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
	// 8317AF38: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 8317AF3C: 41980014  blt cr6, 0x8317af50
	if ctx.cr[6].lt {
	pc = 0x8317AF50; continue 'dispatch;
	}
	// 8317AF40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317AF44: 48014035  bl 0x8318ef78
	ctx.lr = 0x8317AF48;
	sub_8318EF78(ctx, base);
	// 8317AF48: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 8317AF4C: 48000008  b 0x8317af54
	pc = 0x8317AF54; continue 'dispatch;
	// 8317AF50: 7ED7B378  mr r23, r22
	ctx.r[23].u64 = ctx.r[22].u64;
	// 8317AF54: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8317AF58: 80BF0D40  lwz r5, 0xd40(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3392 as u32) ) } as u64;
	// 8317AF5C: 809F0D3C  lwz r4, 0xd3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3388 as u32) ) } as u64;
	// 8317AF60: 480117E9  bl 0x8318c748
	ctx.lr = 0x8317AF64;
	sub_8318C748(ctx, base);
	// 8317AF64: 817F0D4C  lwz r11, 0xd4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3404 as u32) ) } as u64;
	// 8317AF68: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8317AF6C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317AF70: 419A0014  beq cr6, 0x8317af84
	if ctx.cr[6].eq {
	pc = 0x8317AF84; continue 'dispatch;
	}
	// 8317AF74: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 8317AF78: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8317AF7C: 388B9868  addi r4, r11, -0x6798
	ctx.r[4].s64 = ctx.r[11].s64 + -26520;
	// 8317AF80: 4800000C  b 0x8317af8c
	pc = 0x8317AF8C; continue 'dispatch;
	// 8317AF84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8317AF88: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317AF8C: 480117F5  bl 0x8318c780
	ctx.lr = 0x8317AF90;
	sub_8318C780(ctx, base);
	// 8317AF90: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8317AF94: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 8317AF98: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8317AF9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8317AFA0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8317AFA4: 48011815  bl 0x8318c7b8
	ctx.lr = 0x8317AFA8;
	sub_8318C7B8(ctx, base);
	// 8317AFA8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317AFAC: 419A0018  beq cr6, 0x8317afc4
	if ctx.cr[6].eq {
	pc = 0x8317AFC4; continue 'dispatch;
	}
	// 8317AFB0: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317AFB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317AFB8: 60840D03  ori r4, r4, 0xd03
	ctx.r[4].u64 = ctx.r[4].u64 | 3331;
	// 8317AFBC: 4800C53D  bl 0x831874f8
	ctx.lr = 0x8317AFC0;
	sub_831874F8(ctx, base);
	// 8317AFC0: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 8317AFC4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317AFC8: 556B039C  rlwinm r11, r11, 0, 0xe, 0xe
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8317AFCC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317AFD0: 419A0018  beq cr6, 0x8317afe8
	if ctx.cr[6].eq {
	pc = 0x8317AFE8; continue 'dispatch;
	}
	// 8317AFD4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8317AFD8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8317AFDC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8317AFE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317AFE4: 4BFFFB7D  bl 0x8317ab60
	ctx.lr = 0x8317AFE8;
	sub_8317AB60(ctx, base);
	// 8317AFE8: 817F1FA0  lwz r11, 0x1fa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8096 as u32) ) } as u64;
	// 8317AFEC: 3F400008  lis r26, 8
	ctx.r[26].s64 = 524288;
	// 8317AFF0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317AFF4: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 8317AFF8: 7F0AD000  cmpw cr6, r10, r26
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[26].s32, &mut ctx.xer);
	// 8317AFFC: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8317B000: 556B057A  rlwinm r11, r11, 0, 0x15, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8317B004: 3B6B0004  addi r27, r11, 4
	ctx.r[27].s64 = ctx.r[11].s64 + 4;
	// 8317B008: 409A004C  bne cr6, 0x8317b054
	if !ctx.cr[6].eq {
	pc = 0x8317B054; continue 'dispatch;
	}
	// 8317B00C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317B010: 480109E1  bl 0x8318b9f0
	ctx.lr = 0x8317B014;
	sub_8318B9F0(ctx, base);
	// 8317B014: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317B018: 419A0020  beq cr6, 0x8317b038
	if ctx.cr[6].eq {
	pc = 0x8317B038; continue 'dispatch;
	}
	// 8317B01C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317B020: 4BFFEBA9  bl 0x83179bc8
	ctx.lr = 0x8317B024;
	sub_83179BC8(ctx, base);
	// 8317B024: 93790000  stw r27, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8317B028: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8317B02C: 93780158  stw r27, 0x158(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(344 as u32), ctx.r[27].u32 ) };
	// 8317B030: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8317B034: 4802D168  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
	// 8317B038: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317B03C: 7F0BD000  cmpw cr6, r11, r26
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[26].s32, &mut ctx.xer);
	// 8317B040: 409A0014  bne cr6, 0x8317b054
	if !ctx.cr[6].eq {
	pc = 0x8317B054; continue 'dispatch;
	}
	// 8317B044: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317B048: 480109D9  bl 0x8318ba20
	ctx.lr = 0x8317B04C;
	sub_8318BA20(ctx, base);
	// 8317B04C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317B050: 409AFFD4  bne cr6, 0x8317b024
	if !ctx.cr[6].eq {
	pc = 0x8317B024; continue 'dispatch;
	}
	// 8317B054: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 8317B058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317B05C: 409A008C  bne cr6, 0x8317b0e8
	if !ctx.cr[6].eq {
	pc = 0x8317B0E8; continue 'dispatch;
	}
	// 8317B060: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8317B064: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8317B068: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8317B06C: 4BFFF71D  bl 0x8317a788
	ctx.lr = 0x8317B070;
	sub_8317A788(ctx, base);
	// 8317B070: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317B074: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317B078: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317B07C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8317B080: 409900FC  ble cr6, 0x8317b17c
	if !ctx.cr[6].gt {
	pc = 0x8317B17C; continue 'dispatch;
	}
	// 8317B084: 81780158  lwz r11, 0x158(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(344 as u32) ) } as u64;
	// 8317B088: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317B08C: 419800F0  blt cr6, 0x8317b17c
	if ctx.cr[6].lt {
	pc = 0x8317B17C; continue 'dispatch;
	}
	// 8317B090: 811F0028  lwz r8, 0x28(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8317B094: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8317B098: 40980028  bge cr6, 0x8317b0c0
	if !ctx.cr[6].lt {
	pc = 0x8317B0C0; continue 'dispatch;
	}
	// 8317B09C: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317B0A0: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8317B0A4: 40990030  ble cr6, 0x8317b0d4
	if !ctx.cr[6].gt {
	pc = 0x8317B0D4; continue 'dispatch;
	}
	// 8317B0A8: 5509003E  slwi r9, r8, 0
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8317B0AC: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8317B0B0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317B0B4: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317B0B8: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317B0BC: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8317B0C0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317B0C4: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8317B0C8: 91780158  stw r11, 0x158(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(344 as u32), ctx.r[11].u32 ) };
	// 8317B0CC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8317B0D0: 4802D0CC  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
	// 8317B0D4: 91380158  stw r9, 0x158(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(344 as u32), ctx.r[9].u32 ) };
	// 8317B0D8: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8317B0DC: 92DC0000  stw r22, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[22].u32 ) };
	// 8317B0E0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8317B0E4: 4802D0B8  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
	// 8317B0E8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317B0EC: 556B035A  rlwinm r11, r11, 0, 0xd, 0xd
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8317B0F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317B0F4: 409A0048  bne cr6, 0x8317b13c
	if !ctx.cr[6].eq {
	pc = 0x8317B13C; continue 'dispatch;
	}
	// 8317B0F8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8317B0FC: 4BFFEB3D  bl 0x83179c38
	ctx.lr = 0x8317B100;
	sub_83179C38(ctx, base);
	// 8317B100: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8317B104: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317B108: 409A0074  bne cr6, 0x8317b17c
	if !ctx.cr[6].eq {
	pc = 0x8317B17C; continue 'dispatch;
	}
	// 8317B10C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8317B110: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317B114: 40990068  ble cr6, 0x8317b17c
	if !ctx.cr[6].gt {
	pc = 0x8317B17C; continue 'dispatch;
	}
	// 8317B118: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317B11C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317B120: 41990008  bgt cr6, 0x8317b128
	if ctx.cr[6].gt {
	pc = 0x8317B128; continue 'dispatch;
	}
	// 8317B124: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8317B128: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317B12C: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8317B130: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317B134: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8317B138: 4802D064  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
	// 8317B13C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317B140: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 8317B144: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 8317B148: 7C8BF214  add r4, r11, r30
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8317B14C: 7CABE850  subf r5, r11, r29
	ctx.r[5].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 8317B150: 4BFFFB31  bl 0x8317ac80
	ctx.lr = 0x8317B154;
	sub_8317AC80(ctx, base);
	// 8317B154: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8317B158: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 8317B15C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317B160: 409A0014  bne cr6, 0x8317b174
	if !ctx.cr[6].eq {
	pc = 0x8317B174; continue 'dispatch;
	}
	// 8317B164: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317B168: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8317B16C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317B170: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317B174: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8317B178: 91780158  stw r11, 0x158(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(344 as u32), ctx.r[11].u32 ) };
	// 8317B17C: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8317B180: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8317B184: 4802D018  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317B188 size=212
    let mut pc: u32 = 0x8317B188;
    'dispatch: loop {
        match pc {
            0x8317B188 => {
    //   block [0x8317B188..0x8317B25C)
	// 8317B188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317B18C: 4802CFE1  bl 0x831a816c
	ctx.lr = 0x8317B190;
	sub_831A8130(ctx, base);
	// 8317B190: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317B194: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317B198: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8317B19C: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 8317B1A0: 83BF0028  lwz r29, 0x28(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8317B1A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317B1A8: 409A00A0  bne cr6, 0x8317b248
	if !ctx.cr[6].eq {
	pc = 0x8317B248; continue 'dispatch;
	}
	// 8317B1AC: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8317B1B0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8317B1B4: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8317B1B8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8317B1BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317B1C0: 4BFFE591  bl 0x83179750
	ctx.lr = 0x8317B1C4;
	sub_83179750(ctx, base);
	// 8317B1C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317B1C8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8317B1CC: 409A007C  bne cr6, 0x8317b248
	if !ctx.cr[6].eq {
	pc = 0x8317B248; continue 'dispatch;
	}
	// 8317B1D0: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 8317B1D4: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317B1D8: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 8317B1DC: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317B1E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317B1E4: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8317B1E8: 4BFFFD01  bl 0x8317aee8
	ctx.lr = 0x8317B1EC;
	sub_8317AEE8(ctx, base);
	// 8317B1EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317B1F0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8317B1F4: 409A0054  bne cr6, 0x8317b248
	if !ctx.cr[6].eq {
	pc = 0x8317B248; continue 'dispatch;
	}
	// 8317B1F8: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8317B1FC: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8317B200: E91F0990  ld r8, 0x990(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2448 as u32) ) };
	// 8317B204: 7C8907B4  extsw r9, r4
	ctx.r[9].s64 = ctx.r[4].s32 as i64;
	// 8317B208: E95F0998  ld r10, 0x998(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2456 as u32) ) };
	// 8317B20C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8317B210: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8317B214: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317B218: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8317B21C: F93F0990  std r9, 0x990(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2448 as u32), ctx.r[9].u64 ) };
	// 8317B220: F97F0998  std r11, 0x998(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2456 as u32), ctx.r[11].u64 ) };
	// 8317B224: 419A0024  beq cr6, 0x8317b248
	if ctx.cr[6].eq {
	pc = 0x8317B248; continue 'dispatch;
	}
	// 8317B228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317B22C: 4BFFE5CD  bl 0x831797f8
	ctx.lr = 0x8317B230;
	sub_831797F8(ctx, base);
	// 8317B230: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317B234: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8317B238: 409A0010  bne cr6, 0x8317b248
	if !ctx.cr[6].eq {
	pc = 0x8317B248; continue 'dispatch;
	}
	// 8317B23C: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 8317B240: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317B244: 419AFF68  beq cr6, 0x8317b1ac
	if ctx.cr[6].eq {
	pc = 0x8317B1AC; continue 'dispatch;
	}
	// 8317B248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317B24C: 4BFFEDC5  bl 0x8317a010
	ctx.lr = 0x8317B250;
	sub_8317A010(ctx, base);
	// 8317B250: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317B254: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8317B258: 4802CF64  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317B260 size=112
    let mut pc: u32 = 0x8317B260;
    'dispatch: loop {
        match pc {
            0x8317B260 => {
    //   block [0x8317B260..0x8317B2D0)
	// 8317B260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317B264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317B268: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317B26C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317B270: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317B274: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317B278: 4BFFE8D1  bl 0x83179b48
	ctx.lr = 0x8317B27C;
	sub_83179B48(ctx, base);
	// 8317B27C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317B280: 409A000C  bne cr6, 0x8317b28c
	if !ctx.cr[6].eq {
	pc = 0x8317B28C; continue 'dispatch;
	}
	// 8317B284: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317B288: 48000030  b 0x8317b2b8
	pc = 0x8317B2B8; continue 'dispatch;
	// 8317B28C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317B290: 4BFFE4A9  bl 0x83179738
	ctx.lr = 0x8317B294;
	sub_83179738(ctx, base);
	// 8317B294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317B298: 4BFFFEF1  bl 0x8317b188
	ctx.lr = 0x8317B29C;
	sub_8317B188(ctx, base);
	// 8317B29C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8317B2A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317B2A4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8317B2A8: 409A000C  bne cr6, 0x8317b2b4
	if !ctx.cr[6].eq {
	pc = 0x8317B2B4; continue 'dispatch;
	}
	// 8317B2AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317B2B0: 4BFFFBE1  bl 0x8317ae90
	ctx.lr = 0x8317B2B4;
	sub_8317AE90(ctx, base);
	// 8317B2B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317B2B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317B2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317B2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317B2C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317B2C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317B2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317B2D0 size=144
    let mut pc: u32 = 0x8317B2D0;
    'dispatch: loop {
        match pc {
            0x8317B2D0 => {
    //   block [0x8317B2D0..0x8317B360)
	// 8317B2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317B2D4: 4802CE95  bl 0x831a8168
	ctx.lr = 0x8317B2D8;
	sub_831A8130(ctx, base);
	// 8317B2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317B2DC: 3F808345  lis r28, -0x7cbb
	ctx.r[28].s64 = -2092630016;
	// 8317B2E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317B2E4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8317B2E8: 3BEBE0C0  addi r31, r11, -0x1f40
	ctx.r[31].s64 = ctx.r[11].s64 + -8000;
	// 8317B2EC: 807CA32C  lwz r3, -0x5cd4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 8317B2F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317B2F4: 419A001C  beq cr6, 0x8317b310
	if ctx.cr[6].eq {
	pc = 0x8317B310; continue 'dispatch;
	}
	// 8317B2F8: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8317B2FC: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 8317B300: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317B304: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317B308: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317B30C: 4E800421  bctrl
	ctx.lr = 0x8317B310;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317B310: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317B314: 4BFFFF4D  bl 0x8317b260
	ctx.lr = 0x8317B318;
	sub_8317B260(ctx, base);
	// 8317B318: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8317B31C: 807CA32C  lwz r3, -0x5cd4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 8317B320: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317B324: 419A0030  beq cr6, 0x8317b354
	if ctx.cr[6].eq {
	pc = 0x8317B354; continue 'dispatch;
	}
	// 8317B328: 397E0988  addi r11, r30, 0x988
	ctx.r[11].s64 = ctx.r[30].s64 + 2440;
	// 8317B32C: 389F006C  addi r4, r31, 0x6c
	ctx.r[4].s64 = ctx.r[31].s64 + 108;
	// 8317B330: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8317B334: 397E0990  addi r11, r30, 0x990
	ctx.r[11].s64 = ctx.r[30].s64 + 2448;
	// 8317B338: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8317B33C: 397E0998  addi r11, r30, 0x998
	ctx.r[11].s64 = ctx.r[30].s64 + 2456;
	// 8317B340: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 8317B344: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317B348: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317B34C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317B350: 4E800421  bctrl
	ctx.lr = 0x8317B354;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317B354: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317B358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317B35C: 4802CE5C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317B360 size=44
    let mut pc: u32 = 0x8317B360;
    'dispatch: loop {
        match pc {
            0x8317B360 => {
    //   block [0x8317B360..0x8317B38C)
	// 8317B360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317B364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317B368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317B36C: 4BFB731D  bl 0x83132688
	ctx.lr = 0x8317B370;
	sub_83132688(ctx, base);
	// 8317B370: 3963FFFC  addi r11, r3, -4
	ctx.r[11].s64 = ctx.r[3].s64 + -4;
	// 8317B374: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8317B378: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317B37C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317B380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317B384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317B388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B390 size=8
    let mut pc: u32 = 0x8317B390;
    'dispatch: loop {
        match pc {
            0x8317B390 => {
    //   block [0x8317B390..0x8317B398)
	// 8317B390: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317B394: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B398 size=8
    let mut pc: u32 = 0x8317B398;
    'dispatch: loop {
        match pc {
            0x8317B398 => {
    //   block [0x8317B398..0x8317B3A0)
	// 8317B398: 4BFB7AD0  b 0x83132e68
	sub_83132E68(ctx, base);
	return;
	// 8317B39C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B3A0 size=4
    let mut pc: u32 = 0x8317B3A0;
    'dispatch: loop {
        match pc {
            0x8317B3A0 => {
    //   block [0x8317B3A0..0x8317B3A4)
	// 8317B3A0: 4BFB70C8  b 0x83132468
	sub_83132468(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B3A8 size=8
    let mut pc: u32 = 0x8317B3A8;
    'dispatch: loop {
        match pc {
            0x8317B3A8 => {
    //   block [0x8317B3A8..0x8317B3B0)
	// 8317B3A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317B3AC: 4BFB7B2C  b 0x83132ed8
	sub_83132ED8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B3B0 size=4
    let mut pc: u32 = 0x8317B3B0;
    'dispatch: loop {
        match pc {
            0x8317B3B0 => {
    //   block [0x8317B3B0..0x8317B3B4)
	// 8317B3B0: 4BFB7E80  b 0x83133230
	sub_83133230(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317B3B8 size=84
    let mut pc: u32 = 0x8317B3B8;
    'dispatch: loop {
        match pc {
            0x8317B3B8 => {
    //   block [0x8317B3B8..0x8317B40C)
	// 8317B3B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317B3BC: 4802CDA9  bl 0x831a8164
	ctx.lr = 0x8317B3C0;
	sub_831A8130(ctx, base);
	// 8317B3C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317B3C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317B3C8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8317B3CC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8317B3D0: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8317B3D4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8317B3D8: 4BFB7D89  bl 0x83133160
	ctx.lr = 0x8317B3DC;
	sub_83133160(ctx, base);
	// 8317B3DC: 7FCB07B4  extsw r11, r30
	ctx.r[11].s64 = ctx.r[30].s32 as i64;
	// 8317B3E0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8317B3E4: 79675D24  sldi r7, r11, 0xb
	ctx.r[7].u64 = ctx.r[11].u64.wrapping_shl(11);
	ctx.r[7].u32 = ctx.r[7].u64 as u32;
	// 8317B3E8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8317B3EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8317B3F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317B3F4: 4BFB7B35  bl 0x83132f28
	ctx.lr = 0x8317B3F8;
	sub_83132F28(ctx, base);
	// 8317B3F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8317B3FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317B400: 4BFB73A1  bl 0x831327a0
	ctx.lr = 0x8317B404;
	sub_831327A0(ctx, base);
	// 8317B404: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317B408: 4802CDAC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B410 size=4
    let mut pc: u32 = 0x8317B410;
    'dispatch: loop {
        match pc {
            0x8317B410 => {
    //   block [0x8317B410..0x8317B414)
	// 8317B410: 4BFB7B80  b 0x83132f90
	sub_83132F90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317B418 size=52
    let mut pc: u32 = 0x8317B418;
    'dispatch: loop {
        match pc {
            0x8317B418 => {
    //   block [0x8317B418..0x8317B44C)
	// 8317B418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317B41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317B420: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317B424: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317B428: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317B42C: 4BFB7BE5  bl 0x83133010
	ctx.lr = 0x8317B430;
	sub_83133010(ctx, base);
	// 8317B430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317B434: 4BFB7D2D  bl 0x83133160
	ctx.lr = 0x8317B438;
	sub_83133160(ctx, base);
	// 8317B438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317B43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317B440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317B444: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317B448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B450 size=4
    let mut pc: u32 = 0x8317B450;
    'dispatch: loop {
        match pc {
            0x8317B450 => {
    //   block [0x8317B450..0x8317B454)
	// 8317B450: 4BFB7238  b 0x83132688
	sub_83132688(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B458 size=84
    let mut pc: u32 = 0x8317B458;
    'dispatch: loop {
        match pc {
            0x8317B458 => {
    //   block [0x8317B458..0x8317B4AC)
	// 8317B458: 3D606666  lis r11, 0x6666
	ctx.r[11].s64 = 1717960704;
	// 8317B45C: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8317B460: 616A6667  ori r10, r11, 0x6667
	ctx.r[10].u64 = ctx.r[11].u64 | 26215;
	// 8317B464: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317B468: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317B46C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8317B470: 7D645096  mulhw r11, r4, r10
	ctx.r[11].s64 = ((ctx.r[4].s32 as i64 * ctx.r[10].s32 as i64) >> 32);
	// 8317B474: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 8317B478: 556A0FFE  srwi r10, r11, 0x1f
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8317B47C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317B480: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8317B484: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317B488: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317B48C: 7D6B2051  subf. r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317B490: 4082001C  bne 0x8317b4ac
	if !ctx.cr[0].eq {
		sub_8317B4AC(ctx, base);
		return;
	}
	// 8317B494: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 8317B498: 39400064  li r10, 0x64
	ctx.r[10].s64 = 100;
	// 8317B49C: 7D645BD6  divw r11, r4, r11
	ctx.r[11].s32 = ctx.r[4].s32 / ctx.r[11].s32;
	// 8317B4A0: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8317B4A4: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8317B4A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B4AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B4AC size=16
    let mut pc: u32 = 0x8317B4AC;
    'dispatch: loop {
        match pc {
            0x8317B4AC => {
    //   block [0x8317B4AC..0x8317B4BC)
	// 8317B4AC: 396003E8  li r11, 0x3e8
	ctx.r[11].s64 = 1000;
	// 8317B4B0: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 8317B4B4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8317B4B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B4C0 size=44
    let mut pc: u32 = 0x8317B4C0;
    'dispatch: loop {
        match pc {
            0x8317B4C0 => {
    //   block [0x8317B4C0..0x8317B4EC)
	// 8317B4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317B4C4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317B4C8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8317B4CC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8317B4D0: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8317B4D4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8317B4D8: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8317B4DC: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8317B4E0: B163001C  sth r11, 0x1c(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u16 ) };
	// 8317B4E4: B163001E  sth r11, 0x1e(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(30 as u32), ctx.r[11].u16 ) };
	// 8317B4E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B4F0 size=20
    let mut pc: u32 = 0x8317B4F0;
    'dispatch: loop {
        match pc {
            0x8317B4F0 => {
    //   block [0x8317B4F0..0x8317B504)
	// 8317B4F0: 81630298  lwz r11, 0x298(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(664 as u32) ) } as u64;
	// 8317B4F4: 2F0BFFFB  cmpwi cr6, r11, -5
	ctx.cr[6].compare_i32(ctx.r[11].s32, -5, &mut ctx.xer);
	// 8317B4F8: 409A000C  bne cr6, 0x8317b504
	if !ctx.cr[6].eq {
		sub_8317B504(ctx, base);
		return;
	}
	// 8317B4FC: 90830298  stw r4, 0x298(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(664 as u32), ctx.r[4].u32 ) };
	// 8317B500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B504(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B504 size=12
    let mut pc: u32 = 0x8317B504;
    'dispatch: loop {
        match pc {
            0x8317B504 => {
    //   block [0x8317B504..0x8317B510)
	// 8317B504: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 8317B508: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317B50C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8317B510 size=112
    let mut pc: u32 = 0x8317B510;
    'dispatch: loop {
        match pc {
            0x8317B510 => {
    //   block [0x8317B510..0x8317B580)
	// 8317B510: 814302A0  lwz r10, 0x2a0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(672 as u32) ) } as u64;
	// 8317B514: 90830298  stw r4, 0x298(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(664 as u32), ctx.r[4].u32 ) };
	// 8317B518: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317B51C: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8317B520: 41990008  bgt cr6, 0x8317b528
	if ctx.cr[6].gt {
	pc = 0x8317B528; continue 'dispatch;
	}
	// 8317B524: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8317B528: 814302A4  lwz r10, 0x2a4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(676 as u32) ) } as u64;
	// 8317B52C: 912302A0  stw r9, 0x2a0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(672 as u32), ctx.r[9].u32 ) };
	// 8317B530: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317B534: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8317B538: 41980008  blt cr6, 0x8317b540
	if ctx.cr[6].lt {
	pc = 0x8317B540; continue 'dispatch;
	}
	// 8317B53C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8317B540: 3D007FFF  lis r8, 0x7fff
	ctx.r[8].s64 = 2147418112;
	// 8317B544: 8143029C  lwz r10, 0x29c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(668 as u32) ) } as u64;
	// 8317B548: 912302A4  stw r9, 0x2a4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(676 as u32), ctx.r[9].u32 ) };
	// 8317B54C: 6108FFFF  ori r8, r8, 0xffff
	ctx.r[8].u64 = ctx.r[8].u64 | 65535;
	// 8317B550: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8317B554: 419A0024  beq cr6, 0x8317b578
	if ctx.cr[6].eq {
	pc = 0x8317B578; continue 'dispatch;
	}
	// 8317B558: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317B55C: 4099001C  ble cr6, 0x8317b578
	if !ctx.cr[6].gt {
	pc = 0x8317B578; continue 'dispatch;
	}
	// 8317B560: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8317B564: 7D291E70  srawi r9, r9, 3
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 3) as i64;
	// 8317B568: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 8317B56C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8317B570: 419A0008  beq cr6, 0x8317b578
	if ctx.cr[6].eq {
	pc = 0x8317B578; continue 'dispatch;
	}
	// 8317B574: 7D695050  subf r11, r9, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 8317B578: 9163029C  stw r11, 0x29c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(668 as u32), ctx.r[11].u32 ) };
	// 8317B57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B580 size=36
    let mut pc: u32 = 0x8317B580;
    'dispatch: loop {
        match pc {
            0x8317B580 => {
    //   block [0x8317B580..0x8317B5A4)
	// 8317B580: 81630298  lwz r11, 0x298(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(664 as u32) ) } as u64;
	// 8317B584: 8143029C  lwz r10, 0x29c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(668 as u32) ) } as u64;
	// 8317B588: 812302A0  lwz r9, 0x2a0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(672 as u32) ) } as u64;
	// 8317B58C: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8317B590: 7C695A14  add r3, r9, r11
	ctx.r[3].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8317B594: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8317B598: 4098000C  bge cr6, 0x8317b5a4
	if !ctx.cr[6].lt {
		sub_8317B5A4(ctx, base);
		return;
	}
	// 8317B59C: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 8317B5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B5A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B5A4 size=8
    let mut pc: u32 = 0x8317B5A4;
    'dispatch: loop {
        match pc {
            0x8317B5A4 => {
    //   block [0x8317B5A4..0x8317B5AC)
	// 8317B5A4: 7F041800  cmpw cr6, r4, r3
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8317B5A8: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B5AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B5AC size=12
    let mut pc: u32 = 0x8317B5AC;
    'dispatch: loop {
        match pc {
            0x8317B5AC => {
    //   block [0x8317B5AC..0x8317B5B8)
	// 8317B5AC: 3C607FFF  lis r3, 0x7fff
	ctx.r[3].s64 = 2147418112;
	// 8317B5B0: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 8317B5B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B5B8 size=20
    let mut pc: u32 = 0x8317B5B8;
    'dispatch: loop {
        match pc {
            0x8317B5B8 => {
    //   block [0x8317B5B8..0x8317B5CC)
	// 8317B5B8: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 8317B5BC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8317B5C0: 419A000C  beq cr6, 0x8317b5cc
	if ctx.cr[6].eq {
		sub_8317B5CC(ctx, base);
		return;
	}
	// 8317B5C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317B5C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B5CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B5CC size=28
    let mut pc: u32 = 0x8317B5CC;
    'dispatch: loop {
        match pc {
            0x8317B5CC => {
    //   block [0x8317B5CC..0x8317B5E8)
	// 8317B5CC: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317B5D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317B5D4: 409AFFF0  bne cr6, 0x8317b5c4
	if !ctx.cr[6].eq {
		sub_8317B5B8(ctx, base);
		return;
	}
	// 8317B5D8: 81630970  lwz r11, 0x970(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2416 as u32) ) } as u64;
	// 8317B5DC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8317B5E0: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317B5E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B5E8 size=20
    let mut pc: u32 = 0x8317B5E8;
    'dispatch: loop {
        match pc {
            0x8317B5E8 => {
    //   block [0x8317B5E8..0x8317B5FC)
	// 8317B5E8: 816402D4  lwz r11, 0x2d4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(724 as u32) ) } as u64;
	// 8317B5EC: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8317B5F0: 409A000C  bne cr6, 0x8317b5fc
	if !ctx.cr[6].eq {
		sub_8317B5FC(ctx, base);
		return;
	}
	// 8317B5F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317B5F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B5FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B5FC size=20
    let mut pc: u32 = 0x8317B5FC;
    'dispatch: loop {
        match pc {
            0x8317B5FC => {
    //   block [0x8317B5FC..0x8317B610)
	// 8317B5FC: 8163004C  lwz r11, 0x4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 8317B600: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 8317B604: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8317B608: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317B60C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317B610 size=72
    let mut pc: u32 = 0x8317B610;
    'dispatch: loop {
        match pc {
            0x8317B610 => {
    //   block [0x8317B610..0x8317B658)
	// 8317B610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317B614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317B618: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317B61C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317B620: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8317B624: 38800047  li r4, 0x47
	ctx.r[4].s64 = 71;
	// 8317B628: 4BFFDDE9  bl 0x83179410
	ctx.lr = 0x8317B62C;
	sub_83179410(ctx, base);
	// 8317B62C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317B630: 409A000C  bne cr6, 0x8317b63c
	if !ctx.cr[6].eq {
	pc = 0x8317B63C; continue 'dispatch;
	}
	// 8317B634: 817F02AC  lwz r11, 0x2ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(684 as u32) ) } as u64;
	// 8317B638: 48000008  b 0x8317b640
	pc = 0x8317B640; continue 'dispatch;
	// 8317B63C: 817F02EC  lwz r11, 0x2ec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(748 as u32) ) } as u64;
	// 8317B640: 917F02D8  stw r11, 0x2d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(728 as u32), ctx.r[11].u32 ) };
	// 8317B644: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317B648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317B64C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317B650: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317B654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317B658 size=108
    let mut pc: u32 = 0x8317B658;
    'dispatch: loop {
        match pc {
            0x8317B658 => {
    //   block [0x8317B658..0x8317B6C4)
	// 8317B658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317B65C: 4802CB11  bl 0x831a816c
	ctx.lr = 0x8317B660;
	sub_831A8130(ctx, base);
	// 8317B660: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317B664: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8317B668: 38800047  li r4, 0x47
	ctx.r[4].s64 = 71;
	// 8317B66C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8317B670: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8317B674: 4BFFDD9D  bl 0x83179410
	ctx.lr = 0x8317B678;
	sub_83179410(ctx, base);
	// 8317B678: 815F02D8  lwz r10, 0x2d8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(728 as u32) ) } as u64;
	// 8317B67C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317B680: 409A0028  bne cr6, 0x8317b6a8
	if !ctx.cr[6].eq {
	pc = 0x8317B6A8; continue 'dispatch;
	}
	// 8317B684: 817F02AC  lwz r11, 0x2ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(684 as u32) ) } as u64;
	// 8317B688: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8317B68C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317B690: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8317B694: 396BA100  addi r11, r11, -0x5f00
	ctx.r[11].s64 = ctx.r[11].s64 + -24320;
	// 8317B698: 816B01BC  lwz r11, 0x1bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(444 as u32) ) } as u64;
	// 8317B69C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317B6A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317B6A4: 4802CB18  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317B6A8: 817F02EC  lwz r11, 0x2ec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(748 as u32) ) } as u64;
	// 8317B6AC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8317B6B0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317B6B4: 817F02F0  lwz r11, 0x2f0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(752 as u32) ) } as u64;
	// 8317B6B8: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317B6BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317B6C0: 4802CAFC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317B6C8 size=136
    let mut pc: u32 = 0x8317B6C8;
    'dispatch: loop {
        match pc {
            0x8317B6C8 => {
    //   block [0x8317B6C8..0x8317B750)
	// 8317B6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317B6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317B6D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317B6D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317B6D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317B6DC: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8317B6E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317B6E4: 4BFFDD2D  bl 0x83179410
	ctx.lr = 0x8317B6E8;
	sub_83179410(ctx, base);
	// 8317B6E8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317B6EC: 419A0048  beq cr6, 0x8317b734
	if ctx.cr[6].eq {
	pc = 0x8317B734; continue 'dispatch;
	}
	// 8317B6F0: 38800033  li r4, 0x33
	ctx.r[4].s64 = 51;
	// 8317B6F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317B6F8: 4BFFDD19  bl 0x83179410
	ctx.lr = 0x8317B6FC;
	sub_83179410(ctx, base);
	// 8317B6FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317B700: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8317B704: 419A0030  beq cr6, 0x8317b734
	if ctx.cr[6].eq {
	pc = 0x8317B734; continue 'dispatch;
	}
	// 8317B708: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8317B70C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8317B710: 389F0D88  addi r4, r31, 0xd88
	ctx.r[4].s64 = ctx.r[31].s64 + 3464;
	// 8317B714: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317B718: 4BFFFF41  bl 0x8317b658
	ctx.lr = 0x8317B71C;
	sub_8317B658(ctx, base);
	// 8317B71C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317B720: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317B724: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317B728: 7D6A5BD6  divw r11, r10, r11
	ctx.r[11].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 8317B72C: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8317B730: 41990008  bgt cr6, 0x8317b738
	if ctx.cr[6].gt {
	pc = 0x8317B738; continue 'dispatch;
	}
	// 8317B734: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317B738: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317B73C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317B740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317B744: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317B748: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317B74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B750 size=20
    let mut pc: u32 = 0x8317B750;
    'dispatch: loop {
        match pc {
            0x8317B750 => {
    //   block [0x8317B750..0x8317B764)
	// 8317B750: E9630158  ld r11, 0x158(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(344 as u32) ) };
	// 8317B754: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 8317B758: 4098000C  bge cr6, 0x8317b764
	if !ctx.cr[6].lt {
		sub_8317B764(ctx, base);
		return;
	}
	// 8317B75C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8317B760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B764(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B764 size=48
    let mut pc: u32 = 0x8317B764;
    'dispatch: loop {
        match pc {
            0x8317B764 => {
    //   block [0x8317B764..0x8317B794)
	// 8317B764: E9630158  ld r11, 0x158(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(344 as u32) ) };
	// 8317B768: 7C8A07B4  extsw r10, r4
	ctx.r[10].s64 = ctx.r[4].s32 as i64;
	// 8317B76C: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 8317B770: 7D4A59D2  mulld r10, r10, r11
	ctx.r[10].s64 = ctx.r[10].s64 * ctx.r[11].s64;
	// 8317B774: 61295F90  ori r9, r9, 0x5f90
	ctx.r[9].u64 = ctx.r[9].u64 | 24464;
	// 8317B778: 7D2A4BD2  divd r9, r10, r9
	ctx.r[9].s64 = ctx.r[10].s64 / ctx.r[9].s64;
	// 8317B77C: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 8317B780: 7D2307B4  extsw r3, r9
	ctx.r[3].s64 = ctx.r[9].s32 as i64;
	// 8317B784: F96AA360  std r11, -0x5ca0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(-23712 as u32), ctx.r[11].u64 ) };
	// 8317B788: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8317B78C: 906BA36C  stw r3, -0x5c94(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-23700 as u32), ctx.r[3].u32 ) };
	// 8317B790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317B798 size=128
    let mut pc: u32 = 0x8317B798;
    'dispatch: loop {
        match pc {
            0x8317B798 => {
    //   block [0x8317B798..0x8317B818)
	// 8317B798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317B79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317B7A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317B7A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317B7A8: 81630118  lwz r11, 0x118(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(280 as u32) ) } as u64;
	// 8317B7AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317B7B0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317B7B4: 419A0010  beq cr6, 0x8317b7c4
	if ctx.cr[6].eq {
	pc = 0x8317B7C4; continue 'dispatch;
	}
	// 8317B7B8: 83E3013C  lwz r31, 0x13c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(316 as u32) ) } as u64;
	// 8317B7BC: 80A30140  lwz r5, 0x140(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(320 as u32) ) } as u64;
	// 8317B7C0: 48000014  b 0x8317b7d4
	pc = 0x8317B7D4; continue 'dispatch;
	// 8317B7C4: 83E30110  lwz r31, 0x110(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(272 as u32) ) } as u64;
	// 8317B7C8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8317B7CC: 41980034  blt cr6, 0x8317b800
	if ctx.cr[6].lt {
	pc = 0x8317B800; continue 'dispatch;
	}
	// 8317B7D0: 80A30114  lwz r5, 0x114(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(276 as u32) ) } as u64;
	// 8317B7D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317B7D8: 48010001  bl 0x8318b7d8
	ctx.lr = 0x8317B7DC;
	sub_8318B7D8(ctx, base);
	// 8317B7DC: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8317B7E0: 93EBA368  stw r31, -0x5c98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-23704 as u32), ctx.r[31].u32 ) };
	// 8317B7E4: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8317B7E8: 906BA35C  stw r3, -0x5ca4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-23716 as u32), ctx.r[3].u32 ) };
	// 8317B7EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317B7F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317B7F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317B7F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317B7FC: 4E800020  blr
	return;
	// 8317B800: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8317B804: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317B808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317B80C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317B810: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317B814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B818 size=12
    let mut pc: u32 = 0x8317B818;
    'dispatch: loop {
        match pc {
            0x8317B818 => {
    //   block [0x8317B818..0x8317B824)
	// 8317B818: 90830144  stw r4, 0x144(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(324 as u32), ctx.r[4].u32 ) };
	// 8317B81C: 90A30148  stw r5, 0x148(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(328 as u32), ctx.r[5].u32 ) };
	// 8317B820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B828 size=28
    let mut pc: u32 = 0x8317B828;
    'dispatch: loop {
        match pc {
            0x8317B828 => {
    //   block [0x8317B828..0x8317B844)
	// 8317B828: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317B82C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317B830: 814B1018  lwz r10, 0x1018(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4120 as u32) ) } as u64;
	// 8317B834: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8317B838: 816B101C  lwz r11, 0x101c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4124 as u32) ) } as u64;
	// 8317B83C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317B840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B848 size=60
    let mut pc: u32 = 0x8317B848;
    'dispatch: loop {
        match pc {
            0x8317B848 => {
    //   block [0x8317B848..0x8317B884)
	// 8317B848: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 8317B84C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8317B850: 419A0034  beq cr6, 0x8317b884
	if ctx.cr[6].eq {
		sub_8317B884(ctx, base);
		return;
	}
	// 8317B854: 2F0BFFFC  cmpwi cr6, r11, -4
	ctx.cr[6].compare_i32(ctx.r[11].s32, -4, &mut ctx.xer);
	// 8317B858: 419A002C  beq cr6, 0x8317b884
	if ctx.cr[6].eq {
		sub_8317B884(ctx, base);
		return;
	}
	// 8317B85C: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 8317B860: 419A0024  beq cr6, 0x8317b884
	if ctx.cr[6].eq {
		sub_8317B884(ctx, base);
		return;
	}
	// 8317B864: 2F0BFFFA  cmpwi cr6, r11, -6
	ctx.cr[6].compare_i32(ctx.r[11].s32, -6, &mut ctx.xer);
	// 8317B868: 419A001C  beq cr6, 0x8317b884
	if ctx.cr[6].eq {
		sub_8317B884(ctx, base);
		return;
	}
	// 8317B86C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8317B870: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8317B874: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317B878: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317B87C: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8317B880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B884(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B884 size=8
    let mut pc: u32 = 0x8317B884;
    'dispatch: loop {
        match pc {
            0x8317B884 => {
    //   block [0x8317B884..0x8317B88C)
	// 8317B884: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317B888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317B890 size=88
    let mut pc: u32 = 0x8317B890;
    'dispatch: loop {
        match pc {
            0x8317B890 => {
    //   block [0x8317B890..0x8317B8E8)
	// 8317B890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317B894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317B898: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317B89C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317B8A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317B8A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317B8A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317B8AC: 4800BA35  bl 0x831872e0
	ctx.lr = 0x8317B8B0;
	sub_831872E0(ctx, base);
	// 8317B8B0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317B8B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317B8B8: 419A0014  beq cr6, 0x8317b8cc
	if ctx.cr[6].eq {
	pc = 0x8317B8CC; continue 'dispatch;
	}
	// 8317B8BC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317B8C0: 60840124  ori r4, r4, 0x124
	ctx.r[4].u64 = ctx.r[4].u64 | 292;
	// 8317B8C4: 4800BC35  bl 0x831874f8
	ctx.lr = 0x8317B8C8;
	sub_831874F8(ctx, base);
	// 8317B8C8: 48000008  b 0x8317b8d0
	pc = 0x8317B8D0; continue 'dispatch;
	// 8317B8CC: 93DF0DA0  stw r30, 0xda0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3488 as u32), ctx.r[30].u32 ) };
	// 8317B8D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317B8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317B8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317B8DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317B8E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317B8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317B8E8 size=76
    let mut pc: u32 = 0x8317B8E8;
    'dispatch: loop {
        match pc {
            0x8317B8E8 => {
    //   block [0x8317B8E8..0x8317B934)
	// 8317B8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317B8EC: 4802C881  bl 0x831a816c
	ctx.lr = 0x8317B8F0;
	sub_831A8130(ctx, base);
	// 8317B8F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317B8F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317B8F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317B8FC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8317B900: 4800B9E1  bl 0x831872e0
	ctx.lr = 0x8317B904;
	sub_831872E0(ctx, base);
	// 8317B904: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317B908: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317B90C: 419A0018  beq cr6, 0x8317b924
	if ctx.cr[6].eq {
	pc = 0x8317B924; continue 'dispatch;
	}
	// 8317B910: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317B914: 60840123  ori r4, r4, 0x123
	ctx.r[4].u64 = ctx.r[4].u64 | 291;
	// 8317B918: 4800BBE1  bl 0x831874f8
	ctx.lr = 0x8317B91C;
	sub_831874F8(ctx, base);
	// 8317B91C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317B920: 4802C89C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317B924: 93DF1064  stw r30, 0x1064(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4196 as u32), ctx.r[30].u32 ) };
	// 8317B928: 93BF1068  stw r29, 0x1068(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4200 as u32), ctx.r[29].u32 ) };
	// 8317B92C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317B930: 4802C88C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317B938 size=116
    let mut pc: u32 = 0x8317B938;
    'dispatch: loop {
        match pc {
            0x8317B938 => {
    //   block [0x8317B938..0x8317B9AC)
	// 8317B938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317B93C: 4802C82D  bl 0x831a8168
	ctx.lr = 0x8317B940;
	sub_831A8130(ctx, base);
	// 8317B940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317B944: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317B948: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8317B94C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8317B950: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8317B954: 4800B98D  bl 0x831872e0
	ctx.lr = 0x8317B958;
	sub_831872E0(ctx, base);
	// 8317B958: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317B95C: 419A001C  beq cr6, 0x8317b978
	if ctx.cr[6].eq {
	pc = 0x8317B978; continue 'dispatch;
	}
	// 8317B960: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317B964: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317B968: 60840129  ori r4, r4, 0x129
	ctx.r[4].u64 = ctx.r[4].u64 | 297;
	// 8317B96C: 4800BB8D  bl 0x831874f8
	ctx.lr = 0x8317B970;
	sub_831874F8(ctx, base);
	// 8317B970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317B974: 4802C844  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8317B978: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8317B97C: 93DF107C  stw r30, 0x107c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4220 as u32), ctx.r[30].u32 ) };
	// 8317B980: 93BF1080  stw r29, 0x1080(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4224 as u32), ctx.r[29].u32 ) };
	// 8317B984: 419A0014  beq cr6, 0x8317b998
	if ctx.cr[6].eq {
	pc = 0x8317B998; continue 'dispatch;
	}
	// 8317B988: 939F106C  stw r28, 0x106c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4204 as u32), ctx.r[28].u32 ) };
	// 8317B98C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317B990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317B994: 4802C824  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8317B998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317B99C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317B9A0: 917F106C  stw r11, 0x106c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4204 as u32), ctx.r[11].u32 ) };
	// 8317B9A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317B9A8: 4802C810  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317B9B0 size=16
    let mut pc: u32 = 0x8317B9B0;
    'dispatch: loop {
        match pc {
            0x8317B9B0 => {
    //   block [0x8317B9B0..0x8317B9C0)
	// 8317B9B0: 39650362  addi r11, r5, 0x362
	ctx.r[11].s64 = ctx.r[5].s64 + 866;
	// 8317B9B4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317B9B8: 7C8B192E  stwx r4, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[4].u32) };
	// 8317B9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317B9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317B9C0 size=180
    let mut pc: u32 = 0x8317B9C0;
    'dispatch: loop {
        match pc {
            0x8317B9C0 => {
    //   block [0x8317B9C0..0x8317BA74)
	// 8317B9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317B9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317B9C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317B9CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317B9D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317B9D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317B9D8: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 8317B9DC: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317B9E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8317B9E4: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8317B9E8: 394AAD18  addi r10, r10, -0x52e8
	ctx.r[10].s64 = ctx.r[10].s64 + -21224;
	// 8317B9EC: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8317B9F0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8317B9F4: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8317B9F8: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8317B9FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8317BA00: 409A0028  bne cr6, 0x8317ba28
	if !ctx.cr[6].eq {
	pc = 0x8317BA28; continue 'dispatch;
	}
	// 8317BA04: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317BA08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317BA0C: 60840221  ori r4, r4, 0x221
	ctx.r[4].u64 = ctx.r[4].u64 | 545;
	// 8317BA10: 4800BAE9  bl 0x831874f8
	ctx.lr = 0x8317BA14;
	sub_831874F8(ctx, base);
	// 8317BA14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317BA18: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8317BA1C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317BA20: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8317BA24: 48000038  b 0x8317ba5c
	pc = 0x8317BA5C; continue 'dispatch;
	// 8317BA28: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317BA2C: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8317BA30: 419A0008  beq cr6, 0x8317ba38
	if ctx.cr[6].eq {
	pc = 0x8317BA38; continue 'dispatch;
	}
	// 8317BA34: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 8317BA38: 3D20821A  lis r9, -0x7de6
	ctx.r[9].s64 = -2112225280;
	// 8317BA3C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317BA40: 3929ACF0  addi r9, r9, -0x5310
	ctx.r[9].s64 = ctx.r[9].s64 + -21264;
	// 8317BA44: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8317BA48: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8317BA4C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8317BA50: 7C6B482E  lwzx r3, r11, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8317BA54: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8317BA58: 4E800421  bctrl
	ctx.lr = 0x8317BA5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317BA5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317BA60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317BA64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317BA68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317BA6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317BA70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317BA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8317BA78 size=108
    let mut pc: u32 = 0x8317BA78;
    'dispatch: loop {
        match pc {
            0x8317BA78 => {
    //   block [0x8317BA78..0x8317BAE4)
	// 8317BA78: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8317BA7C: 81240008  lwz r9, 8(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317BA80: 394003E8  li r10, 0x3e8
	ctx.r[10].s64 = 1000;
	// 8317BA84: 7D633BD6  divw r11, r3, r7
	ctx.r[11].s32 = ctx.r[3].s32 / ctx.r[7].s32;
	// 8317BA88: 83E4000C  lwz r31, 0xc(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317BA8C: 1C69003C  mulli r3, r9, 0x3c
	ctx.r[3].s64 = ctx.r[9].s64 * 60;
	// 8317BA90: 81240018  lwz r9, 0x18(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8317BA94: 81040010  lwz r8, 0x10(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8317BA98: 7D4A3BD6  divw r10, r10, r7
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[7].s32;
	// 8317BA9C: 80E40014  lwz r7, 0x14(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8317BAA0: 7C63FA14  add r3, r3, r31
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 8317BAA4: 7CE93A14  add r7, r9, r7
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 8317BAA8: A084001E  lhz r4, 0x1e(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(30 as u32) ) } as u64;
	// 8317BAAC: 1D23003C  mulli r9, r3, 0x3c
	ctx.r[9].s64 = ctx.r[3].s64 * 60;
	// 8317BAB0: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8317BAB4: 7C880734  extsh r8, r4
	ctx.r[8].s64 = ctx.r[4].s16 as i64;
	// 8317BAB8: 7D440E70  srawi r4, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 8317BABC: 7D4751D6  mullw r10, r7, r10
	ctx.r[10].s64 = (ctx.r[7].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8317BAC0: 7D2959D6  mullw r9, r9, r11
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8317BAC4: 7CE40194  addze r7, r4
	tmp.s64 = ctx.r[4].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[4].u32);
	ctx.r[7].s64 = tmp.s64;
	// 8317BAC8: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 8317BACC: 7D2839D6  mullw r9, r8, r7
	ctx.r[9].s64 = (ctx.r[8].s32 as i64) * (ctx.r[7].s32 as i64);
	// 8317BAD0: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8317BAD4: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8317BAD8: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317BADC: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8317BAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317BAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317BAE8 size=68
    let mut pc: u32 = 0x8317BAE8;
    'dispatch: loop {
        match pc {
            0x8317BAE8 => {
    //   block [0x8317BAE8..0x8317BB2C)
	// 8317BAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317BAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317BAF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317BAF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317BAF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317BAFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317BB00: 38605DC0  li r3, 0x5dc0
	ctx.r[3].s64 = 24000;
	// 8317BB04: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 8317BB08: 4BFFFF71  bl 0x8317ba78
	ctx.lr = 0x8317BB0C;
	sub_8317BA78(ctx, base);
	// 8317BB0C: 7D7EFBD6  divw r11, r30, r31
	ctx.r[11].s32 = ctx.r[30].s32 / ctx.r[31].s32;
	// 8317BB10: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317BB14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317BB18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317BB1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317BB20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317BB24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317BB28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317BB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317BB30 size=68
    let mut pc: u32 = 0x8317BB30;
    'dispatch: loop {
        match pc {
            0x8317BB30 => {
    //   block [0x8317BB30..0x8317BB74)
	// 8317BB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317BB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317BB38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317BB3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317BB40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317BB44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317BB48: 38607530  li r3, 0x7530
	ctx.r[3].s64 = 30000;
	// 8317BB4C: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 8317BB50: 4BFFFF29  bl 0x8317ba78
	ctx.lr = 0x8317BB54;
	sub_8317BA78(ctx, base);
	// 8317BB54: 7D7EFBD6  divw r11, r30, r31
	ctx.r[11].s32 = ctx.r[30].s32 / ctx.r[31].s32;
	// 8317BB58: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317BB5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317BB60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317BB64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317BB68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317BB6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317BB70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317BB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317BB78 size=72
    let mut pc: u32 = 0x8317BB78;
    'dispatch: loop {
        match pc {
            0x8317BB78 => {
    //   block [0x8317BB78..0x8317BBC0)
	// 8317BB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317BB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317BB80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317BB84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317BB88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317BB8C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317BB90: 3C600000  lis r3, 0
	ctx.r[3].s64 = 0;
	// 8317BB94: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 8317BB98: 6063EA60  ori r3, r3, 0xea60
	ctx.r[3].u64 = ctx.r[3].u64 | 60000;
	// 8317BB9C: 4BFFFEDD  bl 0x8317ba78
	ctx.lr = 0x8317BBA0;
	sub_8317BA78(ctx, base);
	// 8317BBA0: 7D7EFBD6  divw r11, r30, r31
	ctx.r[11].s32 = ctx.r[30].s32 / ctx.r[31].s32;
	// 8317BBA4: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317BBA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317BBAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317BBB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317BBB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317BBB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317BBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317BBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8317BBC0 size=140
    let mut pc: u32 = 0x8317BBC0;
    'dispatch: loop {
        match pc {
            0x8317BBC0 => {
    //   block [0x8317BBC0..0x8317BC4C)
	// 8317BBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317BBC4: 4802C5A5  bl 0x831a8168
	ctx.lr = 0x8317BBC8;
	sub_831A8130(ctx, base);
	// 8317BBC8: 392003E8  li r9, 0x3e8
	ctx.r[9].s64 = 1000;
	// 8317BBCC: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8317BBD0: 3D000000  lis r8, 0
	ctx.r[8].s64 = 0;
	// 8317BBD4: 83C40008  lwz r30, 8(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317BBD8: 7D293BD6  divw r9, r9, r7
	ctx.r[9].s32 = ctx.r[9].s32 / ctx.r[7].s32;
	// 8317BBDC: 8144000C  lwz r10, 0xc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317BBE0: 7C633BD6  divw r3, r3, r7
	ctx.r[3].s32 = ctx.r[3].s32 / ctx.r[7].s32;
	// 8317BBE4: 83E40018  lwz r31, 0x18(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8317BBE8: 5567083C  slwi r7, r11, 1
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8317BBEC: 6108A88A  ori r8, r8, 0xa88a
	ctx.r[8].u64 = ctx.r[8].u64 | 43146;
	// 8317BBF0: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 8317BBF4: 7FDE41D6  mullw r30, r30, r8
	ctx.r[30].s64 = (ctx.r[30].s32 as i64) * (ctx.r[8].s32 as i64);
	// 8317BBF8: 81040014  lwz r8, 0x14(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8317BBFC: A084001E  lhz r4, 0x1e(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(30 as u32) ) } as u64;
	// 8317BC00: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317BC04: 7C9C0734  extsh r28, r4
	ctx.r[28].s64 = ctx.r[4].s16 as i64;
	// 8317BC08: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 8317BC0C: 1C8A02CF  mulli r4, r10, 0x2cf
	ctx.r[4].s64 = ctx.r[10].s64 * 719;
	// 8317BC10: 3BA0000A  li r29, 0xa
	ctx.r[29].s64 = 10;
	// 8317BC14: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 8317BC18: 7CEAEBD6  divw r7, r10, r29
	ctx.r[7].s32 = ctx.r[10].s32 / ctx.r[29].s32;
	// 8317BC1C: 7D2A0E70  srawi r10, r9, 1
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[9].s32 >> 1) as i64;
	// 8317BC20: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 8317BC24: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8317BC28: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317BC2C: 7D5C51D6  mullw r10, r28, r10
	ctx.r[10].s64 = (ctx.r[28].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8317BC30: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8317BC34: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8317BC38: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8317BC3C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317BC40: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317BC44: 90660000  stw r3, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8317BC48: 4802C570  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317BC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8317BC50 size=136
    let mut pc: u32 = 0x8317BC50;
    'dispatch: loop {
        match pc {
            0x8317BC50 => {
    //   block [0x8317BC50..0x8317BCD8)
	// 8317BC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317BC54: 4802C515  bl 0x831a8168
	ctx.lr = 0x8317BC58;
	sub_831A8130(ctx, base);
	// 8317BC58: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8317BC5C: 3D000000  lis r8, 0
	ctx.r[8].s64 = 0;
	// 8317BC60: 83C40008  lwz r30, 8(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317BC64: 392003E8  li r9, 0x3e8
	ctx.r[9].s64 = 1000;
	// 8317BC68: 557C2036  slwi r28, r11, 4
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 8317BC6C: 8144000C  lwz r10, 0xc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317BC70: 6108D2BA  ori r8, r8, 0xd2ba
	ctx.r[8].u64 = ctx.r[8].u64 | 53946;
	// 8317BC74: 83E40018  lwz r31, 0x18(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8317BC78: 7D6BE050  subf r11, r11, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	// 8317BC7C: 7FDE41D6  mullw r30, r30, r8
	ctx.r[30].s64 = (ctx.r[30].s32 as i64) * (ctx.r[8].s32 as i64);
	// 8317BC80: 81040014  lwz r8, 0x14(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8317BC84: A084001E  lhz r4, 0x1e(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(30 as u32) ) } as u64;
	// 8317BC88: 7D293BD6  divw r9, r9, r7
	ctx.r[9].s32 = ctx.r[9].s32 / ctx.r[7].s32;
	// 8317BC8C: 7C633BD6  divw r3, r3, r7
	ctx.r[3].s32 = ctx.r[3].s32 / ctx.r[7].s32;
	// 8317BC90: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 8317BC94: 1CEA0383  mulli r7, r10, 0x383
	ctx.r[7].s64 = ctx.r[10].s64 * 899;
	// 8317BC98: 3BA0000A  li r29, 0xa
	ctx.r[29].s64 = 10;
	// 8317BC9C: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 8317BCA0: 7D3E0E70  srawi r30, r9, 1
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[9].s32 >> 1) as i64;
	// 8317BCA4: 7D4AEBD6  divw r10, r10, r29
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[29].s32;
	// 8317BCA8: 7C840734  extsh r4, r4
	ctx.r[4].s64 = ctx.r[4].s16 as i64;
	// 8317BCAC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317BCB0: 7CFE0194  addze r7, r30
	tmp.s64 = ctx.r[30].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[30].u32);
	ctx.r[7].s64 = tmp.s64;
	// 8317BCB4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317BCB8: 7D4439D6  mullw r10, r4, r7
	ctx.r[10].s64 = (ctx.r[4].s32 as i64) * (ctx.r[7].s32 as i64);
	// 8317BCBC: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8317BCC0: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8317BCC4: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8317BCC8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317BCCC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317BCD0: 90660000  stw r3, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8317BCD4: 4802C4E4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317BCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8317BCD8 size=132
    let mut pc: u32 = 0x8317BCD8;
    'dispatch: loop {
        match pc {
            0x8317BCD8 => {
    //   block [0x8317BCD8..0x8317BD5C)
	// 8317BCD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317BCDC: 4802C491  bl 0x831a816c
	ctx.lr = 0x8317BCE0;
	sub_831A8130(ctx, base);
	// 8317BCE0: 3D000001  lis r8, 1
	ctx.r[8].s64 = 65536;
	// 8317BCE4: 81240008  lwz r9, 8(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317BCE8: 83C40010  lwz r30, 0x10(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8317BCEC: 394003E8  li r10, 0x3e8
	ctx.r[10].s64 = 1000;
	// 8317BCF0: 6108A5AA  ori r8, r8, 0xa5aa
	ctx.r[8].u64 = ctx.r[8].u64 | 42410;
	// 8317BCF4: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317BCF8: 83E40018  lwz r31, 0x18(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8317BCFC: 7D4A3BD6  divw r10, r10, r7
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[7].s32;
	// 8317BD00: 7D2941D6  mullw r9, r9, r8
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[8].s32 as i64);
	// 8317BD04: A3A4001E  lhz r29, 0x1e(r4)
	ctx.r[29].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(30 as u32) ) } as u64;
	// 8317BD08: 81040014  lwz r8, 0x14(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8317BD0C: 1C9E001E  mulli r4, r30, 0x1e
	ctx.r[4].s64 = ctx.r[30].s64 * 30;
	// 8317BD10: 7D292214  add r9, r9, r4
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 8317BD14: 7C633BD6  divw r3, r3, r7
	ctx.r[3].s32 = ctx.r[3].s32 / ctx.r[7].s32;
	// 8317BD18: 1CEB0707  mulli r7, r11, 0x707
	ctx.r[7].s64 = ctx.r[11].s64 * 1799;
	// 8317BD1C: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 8317BD20: 7D293A14  add r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 8317BD24: 7D470E70  srawi r7, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 8317BD28: 7D6B23D6  divw r11, r11, r4
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[4].s32;
	// 8317BD2C: 7FBE0734  extsh r30, r29
	ctx.r[30].s64 = ctx.r[29].s16 as i64;
	// 8317BD30: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8317BD34: 7D270194  addze r9, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[9].s64 = tmp.s64;
	// 8317BD38: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317BD3C: 7D3E49D6  mullw r9, r30, r9
	ctx.r[9].s64 = (ctx.r[30].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8317BD40: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8317BD44: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8317BD48: 7D6B51D6  mullw r11, r11, r10
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8317BD4C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8317BD50: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317BD54: 90660000  stw r3, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8317BD58: 4802C464  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317BD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317BD60 size=108
    let mut pc: u32 = 0x8317BD60;
    'dispatch: loop {
        match pc {
            0x8317BD60 => {
    //   block [0x8317BD60..0x8317BDCC)
	// 8317BD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317BD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317BD68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317BD6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317BD70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317BD74: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8317BD78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317BD7C: 396BA100  addi r11, r11, -0x5f00
	ctx.r[11].s64 = ctx.r[11].s64 + -24320;
	// 8317BD80: 806B01BC  lwz r3, 0x1bc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(444 as u32) ) } as u64;
	// 8317BD84: 4800FA55  bl 0x8318b7d8
	ctx.lr = 0x8317BD88;
	sub_8318B7D8(ctx, base);
	// 8317BD88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317BD8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8317BD90: 4800B661  bl 0x831873f0
	ctx.lr = 0x8317BD94;
	sub_831873F0(ctx, base);
	// 8317BD94: 817F1034  lwz r11, 0x1034(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4148 as u32) ) } as u64;
	// 8317BD98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8317BD9C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8317BDA0: 917F1034  stw r11, 0x1034(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4148 as u32), ctx.r[11].u32 ) };
	// 8317BDA4: 817F105C  lwz r11, 0x105c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4188 as u32) ) } as u64;
	// 8317BDA8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8317BDAC: 917F105C  stw r11, 0x105c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4188 as u32), ctx.r[11].u32 ) };
	// 8317BDB0: 4800B651  bl 0x83187400
	ctx.lr = 0x8317BDB4;
	sub_83187400(ctx, base);
	// 8317BDB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317BDB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317BDBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317BDC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317BDC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317BDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317BDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317BDD0 size=88
    let mut pc: u32 = 0x8317BDD0;
    'dispatch: loop {
        match pc {
            0x8317BDD0 => {
    //   block [0x8317BDD0..0x8317BE28)
	// 8317BDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317BDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317BDD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317BDDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317BDE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317BDE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317BDE8: 807F1078  lwz r3, 0x1078(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4216 as u32) ) } as u64;
	// 8317BDEC: 4800F9ED  bl 0x8318b7d8
	ctx.lr = 0x8317BDF0;
	sub_8318B7D8(ctx, base);
	// 8317BDF0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317BDF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8317BDF8: 4800B5F9  bl 0x831873f0
	ctx.lr = 0x8317BDFC;
	sub_831873F0(ctx, base);
	// 8317BDFC: 817F1074  lwz r11, 0x1074(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4212 as u32) ) } as u64;
	// 8317BE00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8317BE04: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8317BE08: 917F1074  stw r11, 0x1074(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4212 as u32), ctx.r[11].u32 ) };
	// 8317BE0C: 4800B5F5  bl 0x83187400
	ctx.lr = 0x8317BE10;
	sub_83187400(ctx, base);
	// 8317BE10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317BE14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317BE18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317BE1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317BE20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317BE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317BE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317BE28 size=28
    let mut pc: u32 = 0x8317BE28;
    'dispatch: loop {
        match pc {
            0x8317BE28 => {
    //   block [0x8317BE28..0x8317BE44)
	// 8317BE28: 81630924  lwz r11, 0x924(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2340 as u32) ) } as u64;
	// 8317BE2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317BE30: 409A0014  bne cr6, 0x8317be44
	if !ctx.cr[6].eq {
		sub_8317BE44(ctx, base);
		return;
	}
	// 8317BE34: 39407512  li r10, 0x7512
	ctx.r[10].s64 = 29970;
	// 8317BE38: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317BE3C: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8317BE40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317BE44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317BE44 size=32
    let mut pc: u32 = 0x8317BE44;
    'dispatch: loop {
        match pc {
            0x8317BE44 => {
    //   block [0x8317BE44..0x8317BE64)
	// 8317BE44: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 8317BE48: 392003E8  li r9, 0x3e8
	ctx.r[9].s64 = 1000;
	// 8317BE4C: 394AACF0  addi r10, r10, -0x5310
	ctx.r[10].s64 = ctx.r[10].s64 + -21264;
	// 8317BE50: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317BE54: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8317BE58: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8317BE5C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317BE60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317BE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317BE68 size=124
    let mut pc: u32 = 0x8317BE68;
    'dispatch: loop {
        match pc {
            0x8317BE68 => {
    //   block [0x8317BE68..0x8317BEE4)
	// 8317BE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317BE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317BE70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317BE74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317BE78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317BE7C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317BE80: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8317BE84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317BE88: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317BE8C: 4800B455  bl 0x831872e0
	ctx.lr = 0x8317BE90;
	sub_831872E0(ctx, base);
	// 8317BE90: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317BE94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317BE98: 419A0014  beq cr6, 0x8317beac
	if ctx.cr[6].eq {
	pc = 0x8317BEAC; continue 'dispatch;
	}
	// 8317BE9C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317BEA0: 6084011B  ori r4, r4, 0x11b
	ctx.r[4].u64 = ctx.r[4].u64 | 283;
	// 8317BEA4: 4800B655  bl 0x831874f8
	ctx.lr = 0x8317BEA8;
	sub_831874F8(ctx, base);
	// 8317BEA8: 48000024  b 0x8317becc
	pc = 0x8317BECC; continue 'dispatch;
	// 8317BEAC: 815F0924  lwz r10, 0x924(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2340 as u32) ) } as u64;
	// 8317BEB0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8317BEB4: 419A0018  beq cr6, 0x8317becc
	if ctx.cr[6].eq {
	pc = 0x8317BECC; continue 'dispatch;
	}
	// 8317BEB8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317BEBC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8317BEC0: 396BACF0  addi r11, r11, -0x5310
	ctx.r[11].s64 = ctx.r[11].s64 + -21264;
	// 8317BEC4: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8317BEC8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317BECC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317BED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317BED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317BED8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317BEDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317BEE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317BEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317BEE8 size=144
    let mut pc: u32 = 0x8317BEE8;
    'dispatch: loop {
        match pc {
            0x8317BEE8 => {
    //   block [0x8317BEE8..0x8317BF78)
	// 8317BEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317BEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317BEF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317BEF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317BEF8: 39630D88  addi r11, r3, 0xd88
	ctx.r[11].s64 = ctx.r[3].s64 + 3464;
	// 8317BEFC: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 8317BF00: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8317BF04: 814B02D4  lwz r10, 0x2d4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(724 as u32) ) } as u64;
	// 8317BF08: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8317BF0C: 40980028  bge cr6, 0x8317bf34
	if !ctx.cr[6].lt {
	pc = 0x8317BF34; continue 'dispatch;
	}
	// 8317BF10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8317BF14: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8317BF18: 914B02D4  stw r10, 0x2d4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(724 as u32), ctx.r[10].u32 ) };
	// 8317BF1C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8317BF20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317BF24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317BF28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317BF2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317BF30: 4E800020  blr
	return;
	// 8317BF34: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 8317BF38: 816B02D4  lwz r11, 0x2d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(724 as u32) ) } as u64;
	// 8317BF3C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8317BF40: 394AA100  addi r10, r10, -0x5f00
	ctx.r[10].s64 = ctx.r[10].s64 + -24320;
	// 8317BF44: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 8317BF48: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 8317BF4C: 80CA01BC  lwz r6, 0x1bc(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(444 as u32) ) } as u64;
	// 8317BF50: 48013AB9  bl 0x8318fa08
	ctx.lr = 0x8317BF54;
	sub_8318FA08(ctx, base);
	// 8317BF54: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8317BF58: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317BF5C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8317BF60: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317BF64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317BF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317BF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317BF70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317BF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317BF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317BF78 size=324
    let mut pc: u32 = 0x8317BF78;
    'dispatch: loop {
        match pc {
            0x8317BF78 => {
    //   block [0x8317BF78..0x8317C0BC)
	// 8317BF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317BF7C: 4802C1DD  bl 0x831a8158
	ctx.lr = 0x8317BF80;
	sub_831A8130(ctx, base);
	// 8317BF80: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317BF84: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8317BF88: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8317BF8C: 39402710  li r10, 0x2710
	ctx.r[10].s64 = 10000;
	// 8317BF90: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8317BF94: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8317BF98: 817B0AC4  lwz r11, 0xac4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(2756 as u32) ) } as u64;
	// 8317BF9C: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8317BFA0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8317BFA4: 7D6BF1D6  mullw r11, r11, r30
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 8317BFA8: 7F8B53D6  divw r28, r11, r10
	ctx.r[28].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 8317BFAC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8317BFB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317BFB4: 7CBCCA14  add r5, r28, r25
	ctx.r[5].u64 = ctx.r[28].u64 + ctx.r[25].u64;
	// 8317BFB8: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 8317BFBC: 3BBB0D88  addi r29, r27, 0xd88
	ctx.r[29].s64 = ctx.r[27].s64 + 3464;
	// 8317BFC0: 48013A49  bl 0x8318fa08
	ctx.lr = 0x8317BFC4;
	sub_8318FA08(ctx, base);
	// 8317BFC4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317BFC8: 409A0014  bne cr6, 0x8317bfdc
	if !ctx.cr[6].eq {
	pc = 0x8317BFDC; continue 'dispatch;
	}
	// 8317BFCC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317BFD0: 91780000  stw r11, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317BFD4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8317BFD8: 4802C1D0  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 8317BFDC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8317BFE0: 7CBCC850  subf r5, r28, r25
	ctx.r[5].s64 = ctx.r[25].s64 - ctx.r[28].s64;
	// 8317BFE4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8317BFE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317BFEC: 48013A1D  bl 0x8318fa08
	ctx.lr = 0x8317BFF0;
	sub_8318FA08(ctx, base);
	// 8317BFF0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317BFF4: 419A003C  beq cr6, 0x8317c030
	if ctx.cr[6].eq {
	pc = 0x8317C030; continue 'dispatch;
	}
	// 8317BFF8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8317BFFC: 91780000  stw r11, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C000: 817D02D0  lwz r11, 0x2d0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(720 as u32) ) } as u64;
	// 8317C004: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8317C008: 419A00AC  beq cr6, 0x8317c0b4
	if ctx.cr[6].eq {
	pc = 0x8317C0B4; continue 'dispatch;
	}
	// 8317C00C: 817D02C8  lwz r11, 0x2c8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(712 as u32) ) } as u64;
	// 8317C010: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8317C014: 419A00A0  beq cr6, 0x8317c0b4
	if ctx.cr[6].eq {
	pc = 0x8317C0B4; continue 'dispatch;
	}
	// 8317C018: 817D02C4  lwz r11, 0x2c4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(708 as u32) ) } as u64;
	// 8317C01C: 93FD02C8  stw r31, 0x2c8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(712 as u32), ctx.r[31].u32 ) };
	// 8317C020: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8317C024: 917D02C4  stw r11, 0x2c4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(708 as u32), ctx.r[11].u32 ) };
	// 8317C028: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8317C02C: 4802C17C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 8317C030: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8317C034: 396BA100  addi r11, r11, -0x5f00
	ctx.r[11].s64 = ctx.r[11].s64 + -24320;
	// 8317C038: 816B01B8  lwz r11, 0x1b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(440 as u32) ) } as u64;
	// 8317C03C: 2B0BEA24  cmplwi cr6, r11, 0xea24
	ctx.cr[6].compare_u32(ctx.r[11].u32, 59940 as u32, &mut ctx.xer);
	// 8317C040: 409A0024  bne cr6, 0x8317c064
	if !ctx.cr[6].eq {
	pc = 0x8317C064; continue 'dispatch;
	}
	// 8317C044: 817B0924  lwz r11, 0x924(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(2340 as u32) ) } as u64;
	// 8317C048: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8317C04C: 41990018  bgt cr6, 0x8317c064
	if ctx.cr[6].gt {
	pc = 0x8317C064; continue 'dispatch;
	}
	// 8317C050: 817D02B0  lwz r11, 0x2b0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(688 as u32) ) } as u64;
	// 8317C054: 815D02B4  lwz r10, 0x2b4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(692 as u32) ) } as u64;
	// 8317C058: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8317C05C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8317C060: 419A0008  beq cr6, 0x8317c068
	if ctx.cr[6].eq {
	pc = 0x8317C068; continue 'dispatch;
	}
	// 8317C064: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317C068: 815D02C4  lwz r10, 0x2c4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(708 as u32) ) } as u64;
	// 8317C06C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317C070: 4199000C  bgt cr6, 0x8317c07c
	if ctx.cr[6].gt {
	pc = 0x8317C07C; continue 'dispatch;
	}
	// 8317C074: 817D02CC  lwz r11, 0x2cc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(716 as u32) ) } as u64;
	// 8317C078: 48000024  b 0x8317c09c
	pc = 0x8317C09C; continue 'dispatch;
	// 8317C07C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8317C080: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8317C084: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8317C088: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317C08C: 4801397D  bl 0x8318fa08
	ctx.lr = 0x8317C090;
	sub_8318FA08(ctx, base);
	// 8317C090: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8317C094: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317C098: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8317C09C: 91780000  stw r11, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C0A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317C0A4: 917D02C4  stw r11, 0x2c4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(708 as u32), ctx.r[11].u32 ) };
	// 8317C0A8: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317C0AC: 93FD02D0  stw r31, 0x2d0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(720 as u32), ctx.r[31].u32 ) };
	// 8317C0B0: 917D02CC  stw r11, 0x2cc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(716 as u32), ctx.r[11].u32 ) };
	// 8317C0B4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8317C0B8: 4802C0F0  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317C0C0 size=156
    let mut pc: u32 = 0x8317C0C0;
    'dispatch: loop {
        match pc {
            0x8317C0C0 => {
    //   block [0x8317C0C0..0x8317C15C)
	// 8317C0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317C0C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317C0C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317C0CC: 81630950  lwz r11, 0x950(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2384 as u32) ) } as u64;
	// 8317C0D0: 81031008  lwz r8, 0x1008(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4104 as u32) ) } as u64;
	// 8317C0D4: 8123100C  lwz r9, 0x100c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4108 as u32) ) } as u64;
	// 8317C0D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317C0DC: 409A0018  bne cr6, 0x8317c0f4
	if !ctx.cr[6].eq {
	pc = 0x8317C0F4; continue 'dispatch;
	}
	// 8317C0E0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317C0E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317C0E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317C0EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317C0F0: 4E800020  blr
	return;
	// 8317C0F4: 2F08FFFB  cmpwi cr6, r8, -5
	ctx.cr[6].compare_i32(ctx.r[8].s32, -5, &mut ctx.xer);
	// 8317C0F8: 409A0018  bne cr6, 0x8317c110
	if !ctx.cr[6].eq {
	pc = 0x8317C110; continue 'dispatch;
	}
	// 8317C0FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317C100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317C104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317C108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317C10C: 4E800020  blr
	return;
	// 8317C110: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317C114: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8317C118: 4BFFF711  bl 0x8317b828
	ctx.lr = 0x8317C11C;
	sub_8317B828(ctx, base);
	// 8317C11C: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 8317C120: 1D6907D0  mulli r11, r9, 0x7d0
	ctx.r[11].s64 = ctx.r[9].s64 * 2000;
	// 8317C124: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317C128: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317C12C: 614AEA24  ori r10, r10, 0xea24
	ctx.r[10].u64 = ctx.r[10].u64 | 59940;
	// 8317C130: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 8317C134: 7D6B53D6  divw r11, r11, r10
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 8317C138: 7C6B4214  add r3, r11, r8
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8317C13C: 480138CD  bl 0x8318fa08
	ctx.lr = 0x8317C140;
	sub_8318FA08(ctx, base);
	// 8317C140: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8317C144: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317C148: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8317C14C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317C150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317C154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317C158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317C160 size=12
    let mut pc: u32 = 0x8317C160;
    'dispatch: loop {
        match pc {
            0x8317C160 => {
    //   block [0x8317C160..0x8317C16C)
	// 8317C160: 90831038  stw r4, 0x1038(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4152 as u32), ctx.r[4].u32 ) };
	// 8317C164: 90A3103C  stw r5, 0x103c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4156 as u32), ctx.r[5].u32 ) };
	// 8317C168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317C170 size=20
    let mut pc: u32 = 0x8317C170;
    'dispatch: loop {
        match pc {
            0x8317C170 => {
    //   block [0x8317C170..0x8317C184)
	// 8317C170: 81631038  lwz r11, 0x1038(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4152 as u32) ) } as u64;
	// 8317C174: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C178: 8163103C  lwz r11, 0x103c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4156 as u32) ) } as u64;
	// 8317C17C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317C188 size=112
    let mut pc: u32 = 0x8317C188;
    'dispatch: loop {
        match pc {
            0x8317C188 => {
    //   block [0x8317C188..0x8317C1F8)
	// 8317C188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317C18C: 4802BFE1  bl 0x831a816c
	ctx.lr = 0x8317C190;
	sub_831A8130(ctx, base);
	// 8317C190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317C194: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317C198: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8317C19C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8317C1A0: 4800B141  bl 0x831872e0
	ctx.lr = 0x8317C1A4;
	sub_831872E0(ctx, base);
	// 8317C1A4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317C1A8: 419A001C  beq cr6, 0x8317c1c4
	if ctx.cr[6].eq {
	pc = 0x8317C1C4; continue 'dispatch;
	}
	// 8317C1AC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317C1B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317C1B4: 6084012B  ori r4, r4, 0x12b
	ctx.r[4].u64 = ctx.r[4].u64 | 299;
	// 8317C1B8: 4800B341  bl 0x831874f8
	ctx.lr = 0x8317C1BC;
	sub_831874F8(ctx, base);
	// 8317C1BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317C1C0: 4802BFFC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317C1C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8317C1C8: 4800B229  bl 0x831873f0
	ctx.lr = 0x8317C1CC;
	sub_831873F0(ctx, base);
	// 8317C1CC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8317C1D0: 93BF1364  stw r29, 0x1364(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4964 as u32), ctx.r[29].u32 ) };
	// 8317C1D4: 93DF1368  stw r30, 0x1368(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4968 as u32), ctx.r[30].u32 ) };
	// 8317C1D8: 419A000C  beq cr6, 0x8317c1e4
	if ctx.cr[6].eq {
	pc = 0x8317C1E4; continue 'dispatch;
	}
	// 8317C1DC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8317C1E0: 917F1370  stw r11, 0x1370(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4976 as u32), ctx.r[11].u32 ) };
	// 8317C1E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8317C1E8: 4800B219  bl 0x83187400
	ctx.lr = 0x8317C1EC;
	sub_83187400(ctx, base);
	// 8317C1EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317C1F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317C1F4: 4802BFC8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317C1F8 size=156
    let mut pc: u32 = 0x8317C1F8;
    'dispatch: loop {
        match pc {
            0x8317C1F8 => {
    //   block [0x8317C1F8..0x8317C294)
	// 8317C1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317C1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317C200: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317C204: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317C208: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317C20C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317C210: 817F1368  lwz r11, 0x1368(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4968 as u32) ) } as u64;
	// 8317C214: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317C218: 419A0060  beq cr6, 0x8317c278
	if ctx.cr[6].eq {
	pc = 0x8317C278; continue 'dispatch;
	}
	// 8317C21C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8317C220: 4800B1D1  bl 0x831873f0
	ctx.lr = 0x8317C224;
	sub_831873F0(ctx, base);
	// 8317C224: 817F1370  lwz r11, 0x1370(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4976 as u32) ) } as u64;
	// 8317C228: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8317C22C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8317C230: 409A000C  bne cr6, 0x8317c23c
	if !ctx.cr[6].eq {
	pc = 0x8317C23C; continue 'dispatch;
	}
	// 8317C234: 93DF1370  stw r30, 0x1370(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4976 as u32), ctx.r[30].u32 ) };
	// 8317C238: 93DF136C  stw r30, 0x136c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4972 as u32), ctx.r[30].u32 ) };
	// 8317C23C: 813F1370  lwz r9, 0x1370(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4976 as u32) ) } as u64;
	// 8317C240: 815F1364  lwz r10, 0x1364(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4964 as u32) ) } as u64;
	// 8317C244: 817F136C  lwz r11, 0x136c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4972 as u32) ) } as u64;
	// 8317C248: 811F1368  lwz r8, 0x1368(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4968 as u32) ) } as u64;
	// 8317C24C: 7D4A49D6  mullw r10, r10, r9
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8317C250: 7D2B41D6  mullw r9, r11, r8
	ctx.r[9].s64 = (ctx.r[11].s32 as i64) * (ctx.r[8].s32 as i64);
	// 8317C254: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8317C258: 41990010  bgt cr6, 0x8317c268
	if ctx.cr[6].gt {
	pc = 0x8317C268; continue 'dispatch;
	}
	// 8317C25C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8317C260: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8317C264: 917F136C  stw r11, 0x136c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4972 as u32), ctx.r[11].u32 ) };
	// 8317C268: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8317C26C: 4800B195  bl 0x83187400
	ctx.lr = 0x8317C270;
	sub_83187400(ctx, base);
	// 8317C270: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317C274: 48000008  b 0x8317c27c
	pc = 0x8317C27C; continue 'dispatch;
	// 8317C278: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317C27C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317C280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317C284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317C288: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317C28C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317C290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317C298 size=112
    let mut pc: u32 = 0x8317C298;
    'dispatch: loop {
        match pc {
            0x8317C298 => {
    //   block [0x8317C298..0x8317C308)
	// 8317C298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317C29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317C2A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317C2A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317C2A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317C2AC: 817F1368  lwz r11, 0x1368(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4968 as u32) ) } as u64;
	// 8317C2B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317C2B4: 419A0040  beq cr6, 0x8317c2f4
	if ctx.cr[6].eq {
	pc = 0x8317C2F4; continue 'dispatch;
	}
	// 8317C2B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8317C2BC: 4800B135  bl 0x831873f0
	ctx.lr = 0x8317C2C0;
	sub_831873F0(ctx, base);
	// 8317C2C0: 817F1370  lwz r11, 0x1370(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4976 as u32) ) } as u64;
	// 8317C2C4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8317C2C8: 419A0024  beq cr6, 0x8317c2ec
	if ctx.cr[6].eq {
	pc = 0x8317C2EC; continue 'dispatch;
	}
	// 8317C2CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8317C2D0: 815F1368  lwz r10, 0x1368(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4968 as u32) ) } as u64;
	// 8317C2D4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8317C2D8: 917F1370  stw r11, 0x1370(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4976 as u32), ctx.r[11].u32 ) };
	// 8317C2DC: 41980010  blt cr6, 0x8317c2ec
	if ctx.cr[6].lt {
	pc = 0x8317C2EC; continue 'dispatch;
	}
	// 8317C2E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317C2E4: 917F1370  stw r11, 0x1370(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4976 as u32), ctx.r[11].u32 ) };
	// 8317C2E8: 917F136C  stw r11, 0x136c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4972 as u32), ctx.r[11].u32 ) };
	// 8317C2EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8317C2F0: 4800B111  bl 0x83187400
	ctx.lr = 0x8317C2F4;
	sub_83187400(ctx, base);
	// 8317C2F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317C2F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317C2FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317C300: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317C304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8317C308 size=116
    let mut pc: u32 = 0x8317C308;
    'dispatch: loop {
        match pc {
            0x8317C308 => {
    //   block [0x8317C308..0x8317C37C)
	// 8317C308: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8317C30C: 7C6A07B4  extsw r10, r3
	ctx.r[10].s64 = ctx.r[3].s32 as i64;
	// 8317C310: 396BA100  addi r11, r11, -0x5f00
	ctx.r[11].s64 = ctx.r[11].s64 + -24320;
	// 8317C314: F941FFF8  std r10, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[10].u64 ) };
	// 8317C318: 816B01B8  lwz r11, 0x1b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(440 as u32) ) } as u64;
	// 8317C31C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8317C320: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 8317C324: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8317C328: C9A1FFF8  lfd f13, -8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8317C32C: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8317C330: C801FFF0  lfd f0, -0x10(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317C334: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8317C338: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8317C33C: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8317C340: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8317C344: C1AB0A90  lfs f13, 0xa90(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2704 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8317C348: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8317C34C: 40980030  bge cr6, 0x8317c37c
	if !ctx.cr[6].lt {
		sub_8317C37C(ctx, base);
		return;
	}
	// 8317C350: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8317C354: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8317C358: C1AB08A8  lfs f13, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8317C35C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8317C360: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8317C364: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8317C368: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8317C36C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8317C370: 7C0027AE  stfiwx f0, 0, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32, tmp.u32) };
	// 8317C374: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8317C378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C37C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8317C37C size=36
    let mut pc: u32 = 0x8317C37C;
    'dispatch: loop {
        match pc {
            0x8317C37C => {
    //   block [0x8317C37C..0x8317C3A0)
	// 8317C37C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8317C380: C1ABFD2C  lfs f13, -0x2d4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-724 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8317C384: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8317C388: 40980018  bge cr6, 0x8317c3a0
	if !ctx.cr[6].lt {
		sub_8317C3A0(ctx, base);
		return;
	}
	// 8317C38C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8317C390: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8317C394: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C398: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8317C39C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8317C3A0 size=32
    let mut pc: u32 = 0x8317C3A0;
    'dispatch: loop {
        match pc {
            0x8317C3A0 => {
    //   block [0x8317C3A0..0x8317C3C0)
	// 8317C3A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8317C3A4: C1AB0A94  lfs f13, 0xa94(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2708 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8317C3A8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8317C3AC: 40980014  bge cr6, 0x8317c3c0
	if !ctx.cr[6].lt {
		sub_8317C3C0(ctx, base);
		return;
	}
	// 8317C3B0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8317C3B4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C3B8: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8317C3C0 size=36
    let mut pc: u32 = 0x8317C3C0;
    'dispatch: loop {
        match pc {
            0x8317C3C0 => {
    //   block [0x8317C3C0..0x8317C3E4)
	// 8317C3C0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8317C3C4: C1AB7BC4  lfs f13, 0x7bc4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31684 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8317C3C8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8317C3CC: 40980018  bge cr6, 0x8317c3e4
	if !ctx.cr[6].lt {
		sub_8317C3E4(ctx, base);
		return;
	}
	// 8317C3D0: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8317C3D4: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 8317C3D8: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C3DC: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8317C3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C3E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8317C3E4 size=36
    let mut pc: u32 = 0x8317C3E4;
    'dispatch: loop {
        match pc {
            0x8317C3E4 => {
    //   block [0x8317C3E4..0x8317C408)
	// 8317C3E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8317C3E8: C1AB31D4  lfs f13, 0x31d4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12756 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8317C3EC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8317C3F0: 40980018  bge cr6, 0x8317c408
	if !ctx.cr[6].lt {
		sub_8317C408(ctx, base);
		return;
	}
	// 8317C3F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8317C3F8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8317C3FC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C400: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8317C404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8317C408 size=36
    let mut pc: u32 = 0x8317C408;
    'dispatch: loop {
        match pc {
            0x8317C408 => {
    //   block [0x8317C408..0x8317C42C)
	// 8317C408: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317C40C: C1ABAD64  lfs f13, -0x529c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21148 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8317C410: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8317C414: 40980018  bge cr6, 0x8317c42c
	if !ctx.cr[6].lt {
		sub_8317C42C(ctx, base);
		return;
	}
	// 8317C418: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8317C41C: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 8317C420: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C424: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8317C428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C42C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8317C42C size=36
    let mut pc: u32 = 0x8317C42C;
    'dispatch: loop {
        match pc {
            0x8317C42C => {
    //   block [0x8317C42C..0x8317C450)
	// 8317C42C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317C430: C1ABAD60  lfs f13, -0x52a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21152 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8317C434: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8317C438: 40980018  bge cr6, 0x8317c450
	if !ctx.cr[6].lt {
		sub_8317C450(ctx, base);
		return;
	}
	// 8317C43C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8317C440: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 8317C444: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C448: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8317C44C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8317C450 size=36
    let mut pc: u32 = 0x8317C450;
    'dispatch: loop {
        match pc {
            0x8317C450 => {
    //   block [0x8317C450..0x8317C474)
	// 8317C450: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8317C454: C1AB0824  lfs f13, 0x824(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2084 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8317C458: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8317C45C: 40980018  bge cr6, 0x8317c474
	if !ctx.cr[6].lt {
		sub_8317C474(ctx, base);
		return;
	}
	// 8317C460: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8317C464: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8317C468: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C46C: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8317C470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C474(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8317C474 size=32
    let mut pc: u32 = 0x8317C474;
    'dispatch: loop {
        match pc {
            0x8317C474 => {
    //   block [0x8317C474..0x8317C494)
	// 8317C474: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8317C478: C1AB9450  lfs f13, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8317C47C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8317C480: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 8317C484: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C488: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8317C48C: 7C002FAE  stfiwx f0, 0, r5
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32, tmp.u32) };
	// 8317C490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317C498 size=8
    let mut pc: u32 = 0x8317C498;
    'dispatch: loop {
        match pc {
            0x8317C498 => {
    //   block [0x8317C498..0x8317C4A0)
	// 8317C498: 90831374  stw r4, 0x1374(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4980 as u32), ctx.r[4].u32 ) };
	// 8317C49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317C4A0 size=104
    let mut pc: u32 = 0x8317C4A0;
    'dispatch: loop {
        match pc {
            0x8317C4A0 => {
    //   block [0x8317C4A0..0x8317C508)
	// 8317C4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317C4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317C4A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317C4AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317C4B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317C4B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317C4B8: 38800036  li r4, 0x36
	ctx.r[4].s64 = 54;
	// 8317C4BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317C4C0: 4BFFCF51  bl 0x83179410
	ctx.lr = 0x8317C4C4;
	sub_83179410(ctx, base);
	// 8317C4C4: 817F1374  lwz r11, 0x1374(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4980 as u32) ) } as u64;
	// 8317C4C8: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8317C4CC: 409A000C  bne cr6, 0x8317c4d8
	if !ctx.cr[6].eq {
	pc = 0x8317C4D8; continue 'dispatch;
	}
	// 8317C4D0: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8317C4D4: 4800001C  b 0x8317c4f0
	pc = 0x8317C4F0; continue 'dispatch;
	// 8317C4D8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8317C4DC: 419A0010  beq cr6, 0x8317c4ec
	if ctx.cr[6].eq {
	pc = 0x8317C4EC; continue 'dispatch;
	}
	// 8317C4E0: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317C4E4: 40980008  bge cr6, 0x8317c4ec
	if !ctx.cr[6].lt {
	pc = 0x8317C4EC; continue 'dispatch;
	}
	// 8317C4E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317C4EC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C4F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317C4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317C4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317C4FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317C500: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317C504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317C508 size=60
    let mut pc: u32 = 0x8317C508;
    'dispatch: loop {
        match pc {
            0x8317C508 => {
    //   block [0x8317C508..0x8317C544)
	// 8317C508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317C50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317C510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317C514: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8317C518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317C51C: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 8317C520: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C524: 4BFFEF9D  bl 0x8317b4c0
	ctx.lr = 0x8317C528;
	sub_8317B4C0(ctx, base);
	// 8317C528: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8317C52C: 908A0024  stw r4, 0x24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(36 as u32), ctx.r[4].u32 ) };
	// 8317C530: 916A0028  stw r11, 0x28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8317C534: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317C538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317C53C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317C540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317C548 size=120
    let mut pc: u32 = 0x8317C548;
    'dispatch: loop {
        match pc {
            0x8317C548 => {
    //   block [0x8317C548..0x8317C5C0)
	// 8317C548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317C54C: 4802BC21  bl 0x831a816c
	ctx.lr = 0x8317C550;
	sub_831A8130(ctx, base);
	// 8317C550: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317C554: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317C558: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8317C55C: 3BFE0D88  addi r31, r30, 0xd88
	ctx.r[31].s64 = ctx.r[30].s64 + 3464;
	// 8317C560: 396BA100  addi r11, r11, -0x5f00
	ctx.r[11].s64 = ctx.r[11].s64 + -24320;
	// 8317C564: 815F02B0  lwz r10, 0x2b0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(688 as u32) ) } as u64;
	// 8317C568: 816B01C0  lwz r11, 0x1c0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(448 as u32) ) } as u64;
	// 8317C56C: 813F02B4  lwz r9, 0x2b4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(692 as u32) ) } as u64;
	// 8317C570: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8317C574: 7FAB4BD6  divw r29, r11, r9
	ctx.r[29].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8317C578: 4BFFF041  bl 0x8317b5b8
	ctx.lr = 0x8317C57C;
	sub_8317B5B8(ctx, base);
	// 8317C57C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317C580: 419A0018  beq cr6, 0x8317c598
	if ctx.cr[6].eq {
	pc = 0x8317C598; continue 'dispatch;
	}
	// 8317C584: 817F02AC  lwz r11, 0x2ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(684 as u32) ) } as u64;
	// 8317C588: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317C58C: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8317C590: 917F02AC  stw r11, 0x2ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(684 as u32), ctx.r[11].u32 ) };
	// 8317C594: 4BFFFD05  bl 0x8317c298
	ctx.lr = 0x8317C598;
	sub_8317C298(ctx, base);
	// 8317C598: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8317C59C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317C5A0: 4BFFF049  bl 0x8317b5e8
	ctx.lr = 0x8317C5A4;
	sub_8317B5E8(ctx, base);
	// 8317C5A4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317C5A8: 419A0010  beq cr6, 0x8317c5b8
	if ctx.cr[6].eq {
	pc = 0x8317C5B8; continue 'dispatch;
	}
	// 8317C5AC: 817F02D4  lwz r11, 0x2d4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(724 as u32) ) } as u64;
	// 8317C5B0: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8317C5B4: 917F02D4  stw r11, 0x2d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(724 as u32), ctx.r[11].u32 ) };
	// 8317C5B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317C5BC: 4802BC00  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317C5C0 size=96
    let mut pc: u32 = 0x8317C5C0;
    'dispatch: loop {
        match pc {
            0x8317C5C0 => {
    //   block [0x8317C5C0..0x8317C620)
	// 8317C5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317C5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317C5C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317C5CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317C5D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317C5D4: 4BFFF0F5  bl 0x8317b6c8
	ctx.lr = 0x8317C5D8;
	sub_8317B6C8(ctx, base);
	// 8317C5D8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317C5DC: 419A002C  beq cr6, 0x8317c608
	if ctx.cr[6].eq {
	pc = 0x8317C608; continue 'dispatch;
	}
	// 8317C5E0: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317C5E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317C5E8: 60840222  ori r4, r4, 0x222
	ctx.r[4].u64 = ctx.r[4].u64 | 546;
	// 8317C5EC: 4800AF0D  bl 0x831874f8
	ctx.lr = 0x8317C5F0;
	sub_831874F8(ctx, base);
	// 8317C5F0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317C5F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317C5F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317C5FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317C600: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317C604: 4E800020  blr
	return;
	// 8317C608: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317C60C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317C610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317C614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317C618: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317C61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317C620 size=4
    let mut pc: u32 = 0x8317C620;
    'dispatch: loop {
        match pc {
            0x8317C620 => {
    //   block [0x8317C620..0x8317C624)
	// 8317C620: 4BFFF208  b 0x8317b828
	sub_8317B828(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317C628 size=60
    let mut pc: u32 = 0x8317C628;
    'dispatch: loop {
        match pc {
            0x8317C628 => {
    //   block [0x8317C628..0x8317C664)
	// 8317C628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317C62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317C630: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317C634: 4BFFF215  bl 0x8317b848
	ctx.lr = 0x8317C638;
	sub_8317B848(ctx, base);
	// 8317C638: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317C63C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317C640: 419A0014  beq cr6, 0x8317c654
	if ctx.cr[6].eq {
	pc = 0x8317C654; continue 'dispatch;
	}
	// 8317C644: 3960FFFE  li r11, -2
	ctx.r[11].s64 = -2;
	// 8317C648: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8317C64C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C650: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8317C654: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317C658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317C65C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317C660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317C668 size=72
    let mut pc: u32 = 0x8317C668;
    'dispatch: loop {
        match pc {
            0x8317C668 => {
    //   block [0x8317C668..0x8317C6B0)
	// 8317C668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317C66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317C670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317C674: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8317C678: 4BFFF1D1  bl 0x8317b848
	ctx.lr = 0x8317C67C;
	sub_8317B848(ctx, base);
	// 8317C67C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317C680: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317C684: 419A001C  beq cr6, 0x8317c6a0
	if ctx.cr[6].eq {
	pc = 0x8317C6A0; continue 'dispatch;
	}
	// 8317C688: 81691034  lwz r11, 0x1034(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4148 as u32) ) } as u64;
	// 8317C68C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C690: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8317C694: 396BA100  addi r11, r11, -0x5f00
	ctx.r[11].s64 = ctx.r[11].s64 + -24320;
	// 8317C698: 816B01BC  lwz r11, 0x1bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(444 as u32) ) } as u64;
	// 8317C69C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C6A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317C6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317C6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317C6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317C6B0 size=36
    let mut pc: u32 = 0x8317C6B0;
    'dispatch: loop {
        match pc {
            0x8317C6B0 => {
    //   block [0x8317C6B0..0x8317C6D4)
	// 8317C6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317C6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317C6B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317C6BC: 4BFFF18D  bl 0x8317b848
	ctx.lr = 0x8317C6C0;
	sub_8317B848(ctx, base);
	// 8317C6C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317C6C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317C6C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317C6CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317C6D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317C6D8 size=120
    let mut pc: u32 = 0x8317C6D8;
    'dispatch: loop {
        match pc {
            0x8317C6D8 => {
    //   block [0x8317C6D8..0x8317C750)
	// 8317C6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317C6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317C6E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317C6E4: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8317C6E8: 4BFFF161  bl 0x8317b848
	ctx.lr = 0x8317C6EC;
	sub_8317B848(ctx, base);
	// 8317C6EC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317C6F0: 409A0014  bne cr6, 0x8317c704
	if !ctx.cr[6].eq {
	pc = 0x8317C704; continue 'dispatch;
	}
	// 8317C6F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317C6F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317C6FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317C700: 4E800020  blr
	return;
	// 8317C704: 81691064  lwz r11, 0x1064(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4196 as u32) ) } as u64;
	// 8317C708: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317C70C: 409A0028  bne cr6, 0x8317c734
	if !ctx.cr[6].eq {
	pc = 0x8317C734; continue 'dispatch;
	}
	// 8317C710: 3960FFFE  li r11, -2
	ctx.r[11].s64 = -2;
	// 8317C714: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8317C718: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317C71C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C720: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8317C724: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317C728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317C72C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317C730: 4E800020  blr
	return;
	// 8317C734: 80691068  lwz r3, 0x1068(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4200 as u32) ) } as u64;
	// 8317C738: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317C73C: 4E800421  bctrl
	ctx.lr = 0x8317C740;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317C740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317C744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317C748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317C74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317C750 size=224
    let mut pc: u32 = 0x8317C750;
    'dispatch: loop {
        match pc {
            0x8317C750 => {
    //   block [0x8317C750..0x8317C830)
	// 8317C750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317C754: 4802BA15  bl 0x831a8168
	ctx.lr = 0x8317C758;
	sub_831A8130(ctx, base);
	// 8317C758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317C75C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317C760: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8317C764: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8317C768: 4BFFF0E1  bl 0x8317b848
	ctx.lr = 0x8317C76C;
	sub_8317B848(ctx, base);
	// 8317C76C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317C770: 409A000C  bne cr6, 0x8317c77c
	if !ctx.cr[6].eq {
	pc = 0x8317C77C; continue 'dispatch;
	}
	// 8317C774: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317C778: 4802BA40  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8317C77C: 3BFE0D88  addi r31, r30, 0xd88
	ctx.r[31].s64 = ctx.r[30].s64 + 3464;
	// 8317C780: 817F02E4  lwz r11, 0x2e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(740 as u32) ) } as u64;
	// 8317C784: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317C788: 409A0020  bne cr6, 0x8317c7a8
	if !ctx.cr[6].eq {
	pc = 0x8317C7A8; continue 'dispatch;
	}
	// 8317C78C: 3960FFFE  li r11, -2
	ctx.r[11].s64 = -2;
	// 8317C790: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8317C794: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317C798: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C79C: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8317C7A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317C7A4: 4802BA14  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8317C7A8: 807F02F8  lwz r3, 0x2f8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(760 as u32) ) } as u64;
	// 8317C7AC: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8317C7B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8317C7B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317C7B8: 4E800421  bctrl
	ctx.lr = 0x8317C7BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317C7BC: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8317C7C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317C7C4: 4BFFEDF5  bl 0x8317b5b8
	ctx.lr = 0x8317C7C8;
	sub_8317B5B8(ctx, base);
	// 8317C7C8: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317C7CC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317C7D0: 419A0038  beq cr6, 0x8317c808
	if ctx.cr[6].eq {
	pc = 0x8317C808; continue 'dispatch;
	}
	// 8317C7D4: 817F02E8  lwz r11, 0x2e8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(744 as u32) ) } as u64;
	// 8317C7D8: 2F0BFFFB  cmpwi cr6, r11, -5
	ctx.cr[6].compare_i32(ctx.r[11].s32, -5, &mut ctx.xer);
	// 8317C7DC: 419A002C  beq cr6, 0x8317c808
	if ctx.cr[6].eq {
	pc = 0x8317C808; continue 'dispatch;
	}
	// 8317C7E0: 817F02E8  lwz r11, 0x2e8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(744 as u32) ) } as u64;
	// 8317C7E4: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 8317C7E8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317C7EC: 40980010  bge cr6, 0x8317c7fc
	if !ctx.cr[6].lt {
	pc = 0x8317C7FC; continue 'dispatch;
	}
	// 8317C7F0: 815F02F4  lwz r10, 0x2f4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(756 as u32) ) } as u64;
	// 8317C7F4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8317C7F8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8317C7FC: 815F02EC  lwz r10, 0x2ec(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(748 as u32) ) } as u64;
	// 8317C800: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8317C804: 917F02EC  stw r11, 0x2ec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(748 as u32), ctx.r[11].u32 ) };
	// 8317C808: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317C80C: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8317C810: 913F02E8  stw r9, 0x2e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(744 as u32), ctx.r[9].u32 ) };
	// 8317C814: 917F02F0  stw r11, 0x2f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(752 as u32), ctx.r[11].u32 ) };
	// 8317C818: 817F02EC  lwz r11, 0x2ec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(748 as u32) ) } as u64;
	// 8317C81C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C820: 817F02F0  lwz r11, 0x2f0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(752 as u32) ) } as u64;
	// 8317C824: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317C82C: 4802B98C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317C830 size=80
    let mut pc: u32 = 0x8317C830;
    'dispatch: loop {
        match pc {
            0x8317C830 => {
    //   block [0x8317C830..0x8317C880)
	// 8317C830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317C834: 4802B939  bl 0x831a816c
	ctx.lr = 0x8317C838;
	sub_831A8130(ctx, base);
	// 8317C838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317C83C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317C840: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 8317C844: 409A0034  bne cr6, 0x8317c878
	if !ctx.cr[6].eq {
	pc = 0x8317C878; continue 'dispatch;
	}
	// 8317C848: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317C84C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8317C850: 4BFFF5D9  bl 0x8317be28
	ctx.lr = 0x8317C854;
	sub_8317BE28(ctx, base);
	// 8317C854: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317C858: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317C85C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8317C860: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8317C864: 4BFFF4FD  bl 0x8317bd60
	ctx.lr = 0x8317C868;
	sub_8317BD60(ctx, base);
	// 8317C868: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8317C86C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8317C870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317C874: 4BFFF55D  bl 0x8317bdd0
	ctx.lr = 0x8317C878;
	sub_8317BDD0(ctx, base);
	// 8317C878: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317C87C: 4802B940  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317C880 size=220
    let mut pc: u32 = 0x8317C880;
    'dispatch: loop {
        match pc {
            0x8317C880 => {
    //   block [0x8317C880..0x8317C95C)
	// 8317C880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317C884: 4802B8E9  bl 0x831a816c
	ctx.lr = 0x8317C888;
	sub_831A8130(ctx, base);
	// 8317C888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317C88C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317C890: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8317C894: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8317C898: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8317C89C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8317C8A0: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8317C8A4: 4BFFEF85  bl 0x8317b828
	ctx.lr = 0x8317C8A8;
	sub_8317B828(ctx, base);
	// 8317C8A8: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317C8AC: 2F060001  cmpwi cr6, r6, 1
	ctx.cr[6].compare_i32(ctx.r[6].s32, 1, &mut ctx.xer);
	// 8317C8B0: 409A003C  bne cr6, 0x8317c8ec
	if !ctx.cr[6].eq {
	pc = 0x8317C8EC; continue 'dispatch;
	}
	// 8317C8B4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317C8B8: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 8317C8BC: 409A0014  bne cr6, 0x8317c8d0
	if !ctx.cr[6].eq {
	pc = 0x8317C8D0; continue 'dispatch;
	}
	// 8317C8C0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8317C8C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C8C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317C8CC: 4802B8F0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317C8D0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8317C8D4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8317C8D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8317C8DC: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 8317C8E0: 4BFFF609  bl 0x8317bee8
	ctx.lr = 0x8317C8E4;
	sub_8317BEE8(ctx, base);
	// 8317C8E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317C8E8: 4802B8D4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317C8EC: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8317C8F0: 81090A48  lwz r8, 0xa48(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2632 as u32) ) } as u64;
	// 8317C8F4: 7D4639D6  mullw r10, r6, r7
	ctx.r[10].s64 = (ctx.r[6].s32 as i64) * (ctx.r[7].s32 as i64);
	// 8317C8F8: 396BA100  addi r11, r11, -0x5f00
	ctx.r[11].s64 = ctx.r[11].s64 + -24320;
	// 8317C8FC: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 8317C900: 816B01B8  lwz r11, 0x1b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(440 as u32) ) } as u64;
	// 8317C904: 7D6A5BD6  divw r11, r10, r11
	ctx.r[11].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 8317C908: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317C90C: 7CAB5214  add r5, r11, r10
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317C910: 419A0028  beq cr6, 0x8317c938
	if ctx.cr[6].eq {
	pc = 0x8317C938; continue 'dispatch;
	}
	// 8317C914: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8317C918: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8317C91C: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8317C920: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8317C924: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8317C928: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 8317C92C: 4BFFF64D  bl 0x8317bf78
	ctx.lr = 0x8317C930;
	sub_8317BF78(ctx, base);
	// 8317C930: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317C934: 4802B888  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317C938: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8317C93C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317C940: 480130C9  bl 0x8318fa08
	ctx.lr = 0x8317C944;
	sub_8318FA08(ctx, base);
	// 8317C944: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8317C948: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317C94C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8317C950: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317C954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317C958: 4802B864  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317C960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317C960 size=476
    let mut pc: u32 = 0x8317C960;
    'dispatch: loop {
        match pc {
            0x8317C960 => {
    //   block [0x8317C960..0x8317CB3C)
	// 8317C960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317C964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317C968: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317C96C: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 8317C970: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 8317C974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8317C978: 388BC628  addi r4, r11, -0x39d8
	ctx.r[4].s64 = ctx.r[11].s64 + -14808;
	// 8317C97C: 4BFFF035  bl 0x8317b9b0
	ctx.lr = 0x8317C980;
	sub_8317B9B0(ctx, base);
	// 8317C980: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 8317C984: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8317C988: 388BC668  addi r4, r11, -0x3998
	ctx.r[4].s64 = ctx.r[11].s64 + -14744;
	// 8317C98C: 4BFFF025  bl 0x8317b9b0
	ctx.lr = 0x8317C990;
	sub_8317B9B0(ctx, base);
	// 8317C990: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8317C994: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317C998: 4BFFF019  bl 0x8317b9b0
	ctx.lr = 0x8317C99C;
	sub_8317B9B0(ctx, base);
	// 8317C99C: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 8317C9A0: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8317C9A4: 388BC6B0  addi r4, r11, -0x3950
	ctx.r[4].s64 = ctx.r[11].s64 + -14672;
	// 8317C9A8: 4BFFF009  bl 0x8317b9b0
	ctx.lr = 0x8317C9AC;
	sub_8317B9B0(ctx, base);
	// 8317C9AC: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 8317C9B0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8317C9B4: 388BC6D8  addi r4, r11, -0x3928
	ctx.r[4].s64 = ctx.r[11].s64 + -14632;
	// 8317C9B8: 4BFFEFF9  bl 0x8317b9b0
	ctx.lr = 0x8317C9BC;
	sub_8317B9B0(ctx, base);
	// 8317C9BC: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 8317C9C0: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8317C9C4: 388BC750  addi r4, r11, -0x38b0
	ctx.r[4].s64 = ctx.r[11].s64 + -14512;
	// 8317C9C8: 4BFFEFE9  bl 0x8317b9b0
	ctx.lr = 0x8317C9CC;
	sub_8317B9B0(ctx, base);
	// 8317C9CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8317C9D0: 3869001C  addi r3, r9, 0x1c
	ctx.r[3].s64 = ctx.r[9].s64 + 28;
	// 8317C9D4: 91090018  stw r8, 0x18(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 8317C9D8: 4BFFEAE9  bl 0x8317b4c0
	ctx.lr = 0x8317C9DC;
	sub_8317B4C0(ctx, base);
	// 8317C9DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317C9E0: 386900C0  addi r3, r9, 0xc0
	ctx.r[3].s64 = ctx.r[9].s64 + 192;
	// 8317C9E4: 4BFFFB25  bl 0x8317c508
	ctx.lr = 0x8317C9E8;
	sub_8317C508(ctx, base);
	// 8317C9E8: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 8317C9EC: 3869003C  addi r3, r9, 0x3c
	ctx.r[3].s64 = ctx.r[9].s64 + 60;
	// 8317C9F0: 6167FFFF  ori r7, r11, 0xffff
	ctx.r[7].u64 = ctx.r[11].u64 | 65535;
	// 8317C9F4: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 8317C9F8: 4BFFFB11  bl 0x8317c508
	ctx.lr = 0x8317C9FC;
	sub_8317C508(ctx, base);
	// 8317C9FC: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 8317CA00: 38690068  addi r3, r9, 0x68
	ctx.r[3].s64 = ctx.r[9].s64 + 104;
	// 8317CA04: 4BFFFB05  bl 0x8317c508
	ctx.lr = 0x8317CA08;
	sub_8317C508(ctx, base);
	// 8317CA08: 38690094  addi r3, r9, 0x94
	ctx.r[3].s64 = ctx.r[9].s64 + 148;
	// 8317CA0C: 4BFFFAFD  bl 0x8317c508
	ctx.lr = 0x8317CA10;
	sub_8317C508(ctx, base);
	// 8317CA10: 386900EC  addi r3, r9, 0xec
	ctx.r[3].s64 = ctx.r[9].s64 + 236;
	// 8317CA14: 4BFFFAF5  bl 0x8317c508
	ctx.lr = 0x8317CA18;
	sub_8317C508(ctx, base);
	// 8317CA18: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 8317CA1C: 38690118  addi r3, r9, 0x118
	ctx.r[3].s64 = ctx.r[9].s64 + 280;
	// 8317CA20: 4BFFFAE9  bl 0x8317c508
	ctx.lr = 0x8317CA24;
	sub_8317C508(ctx, base);
	// 8317CA24: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8317CA28: 3949016C  addi r10, r9, 0x16c
	ctx.r[10].s64 = ctx.r[9].s64 + 364;
	// 8317CA2C: 91090144  stw r8, 0x144(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(324 as u32), ctx.r[8].u32 ) };
	// 8317CA30: 91090148  stw r8, 0x148(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(328 as u32), ctx.r[8].u32 ) };
	// 8317CA34: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8317CA38: 91090160  stw r8, 0x160(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(352 as u32), ctx.r[8].u32 ) };
	// 8317CA3C: 91090164  stw r8, 0x164(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(356 as u32), ctx.r[8].u32 ) };
	// 8317CA40: F9690150  std r11, 0x150(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(336 as u32), ctx.r[11].u64 ) };
	// 8317CA44: F9690158  std r11, 0x158(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(344 as u32), ctx.r[11].u64 ) };
	// 8317CA48: 91090168  stw r8, 0x168(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(360 as u32), ctx.r[8].u32 ) };
	// 8317CA4C: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 8317CA50: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8317CA54: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8317CA58: 4200FFF8  bdnz 0x8317ca50
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8317CA50; continue 'dispatch;
	}
	// 8317CA5C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8317CA60: 910901F0  stw r8, 0x1f0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(496 as u32), ctx.r[8].u32 ) };
	// 8317CA64: 38C90200  addi r6, r9, 0x200
	ctx.r[6].s64 = ctx.r[9].s64 + 512;
	// 8317CA68: 910901F4  stw r8, 0x1f4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(500 as u32), ctx.r[8].u32 ) };
	// 8317CA6C: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8317CA70: 910901F8  stw r8, 0x1f8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(504 as u32), ctx.r[8].u32 ) };
	// 8317CA74: 910901FC  stw r8, 0x1fc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(508 as u32), ctx.r[8].u32 ) };
	// 8317CA78: 914901EC  stw r10, 0x1ec(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(492 as u32), ctx.r[10].u32 ) };
	// 8317CA7C: 7CA903A6  mtctr r5
	ctx.ctr.u64 = ctx.r[5].u64;
	// 8317CA80: 91060000  stw r8, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8317CA84: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 8317CA88: 4200FFF8  bdnz 0x8317ca80
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8317CA80; continue 'dispatch;
	}
	// 8317CA8C: 38C0FFFB  li r6, -5
	ctx.r[6].s64 = -5;
	// 8317CA90: 910902AC  stw r8, 0x2ac(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(684 as u32), ctx.r[8].u32 ) };
	// 8317CA94: 916902D4  stw r11, 0x2d4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(724 as u32), ctx.r[11].u32 ) };
	// 8317CA98: 90E9029C  stw r7, 0x29c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(668 as u32), ctx.r[7].u32 ) };
	// 8317CA9C: 90E902A4  stw r7, 0x2a4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(676 as u32), ctx.r[7].u32 ) };
	// 8317CAA0: 38E00064  li r7, 0x64
	ctx.r[7].s64 = 100;
	// 8317CAA4: 80A902AC  lwz r5, 0x2ac(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(684 as u32) ) } as u64;
	// 8317CAA8: 90C902E8  stw r6, 0x2e8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(744 as u32), ctx.r[6].u32 ) };
	// 8317CAAC: 910902EC  stw r8, 0x2ec(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(748 as u32), ctx.r[8].u32 ) };
	// 8317CAB0: 914902F0  stw r10, 0x2f0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(752 as u32), ctx.r[10].u32 ) };
	// 8317CAB4: 90C90280  stw r6, 0x280(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(640 as u32), ctx.r[6].u32 ) };
	// 8317CAB8: 91490284  stw r10, 0x284(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(644 as u32), ctx.r[10].u32 ) };
	// 8317CABC: 90C90288  stw r6, 0x288(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(648 as u32), ctx.r[6].u32 ) };
	// 8317CAC0: 9149028C  stw r10, 0x28c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(652 as u32), ctx.r[10].u32 ) };
	// 8317CAC4: 91690290  stw r11, 0x290(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(656 as u32), ctx.r[11].u32 ) };
	// 8317CAC8: 91490294  stw r10, 0x294(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(660 as u32), ctx.r[10].u32 ) };
	// 8317CACC: 90C90298  stw r6, 0x298(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(664 as u32), ctx.r[6].u32 ) };
	// 8317CAD0: 910902A0  stw r8, 0x2a0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(672 as u32), ctx.r[8].u32 ) };
	// 8317CAD4: 910902A8  stw r8, 0x2a8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(680 as u32), ctx.r[8].u32 ) };
	// 8317CAD8: 914902B0  stw r10, 0x2b0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(688 as u32), ctx.r[10].u32 ) };
	// 8317CADC: 914902B4  stw r10, 0x2b4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(692 as u32), ctx.r[10].u32 ) };
	// 8317CAE0: 910902B8  stw r8, 0x2b8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(696 as u32), ctx.r[8].u32 ) };
	// 8317CAE4: 910902BC  stw r8, 0x2bc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(700 as u32), ctx.r[8].u32 ) };
	// 8317CAE8: 914902C0  stw r10, 0x2c0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(704 as u32), ctx.r[10].u32 ) };
	// 8317CAEC: 90E902C4  stw r7, 0x2c4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(708 as u32), ctx.r[7].u32 ) };
	// 8317CAF0: 916902C8  stw r11, 0x2c8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(712 as u32), ctx.r[11].u32 ) };
	// 8317CAF4: 910902CC  stw r8, 0x2cc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(716 as u32), ctx.r[8].u32 ) };
	// 8317CAF8: 916902D0  stw r11, 0x2d0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(720 as u32), ctx.r[11].u32 ) };
	// 8317CAFC: 90A902D8  stw r5, 0x2d8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(728 as u32), ctx.r[5].u32 ) };
	// 8317CB00: 910902E4  stw r8, 0x2e4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(740 as u32), ctx.r[8].u32 ) };
	// 8317CB04: 916902F4  stw r11, 0x2f4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(756 as u32), ctx.r[11].u32 ) };
	// 8317CB08: 910902F8  stw r8, 0x2f8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(760 as u32), ctx.r[8].u32 ) };
	// 8317CB0C: 910905D0  stw r8, 0x5d0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1488 as u32), ctx.r[8].u32 ) };
	// 8317CB10: 910905D4  stw r8, 0x5d4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1492 as u32), ctx.r[8].u32 ) };
	// 8317CB14: 910905D8  stw r8, 0x5d8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1496 as u32), ctx.r[8].u32 ) };
	// 8317CB18: 910905DC  stw r8, 0x5dc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1500 as u32), ctx.r[8].u32 ) };
	// 8317CB1C: 910905E0  stw r8, 0x5e0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1504 as u32), ctx.r[8].u32 ) };
	// 8317CB20: 910905E4  stw r8, 0x5e4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1508 as u32), ctx.r[8].u32 ) };
	// 8317CB24: 910905E8  stw r8, 0x5e8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1512 as u32), ctx.r[8].u32 ) };
	// 8317CB28: 916905EC  stw r11, 0x5ec(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1516 as u32), ctx.r[11].u32 ) };
	// 8317CB2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317CB30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317CB34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317CB38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317CB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317CB40 size=140
    let mut pc: u32 = 0x8317CB40;
    'dispatch: loop {
        match pc {
            0x8317CB40 => {
    //   block [0x8317CB40..0x8317CBCC)
	// 8317CB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317CB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317CB48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317CB4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317CB50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317CB54: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8317CB58: 39230D88  addi r9, r3, 0xd88
	ctx.r[9].s64 = ctx.r[3].s64 + 3464;
	// 8317CB5C: 4BFFFAC5  bl 0x8317c620
	ctx.lr = 0x8317CB60;
	sub_8317C620(ctx, base);
	// 8317CB60: 80850000  lwz r4, 0(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317CB64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317CB68: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 8317CB6C: 419A0044  beq cr6, 0x8317cbb0
	if ctx.cr[6].eq {
	pc = 0x8317CBB0; continue 'dispatch;
	}
	// 8317CB70: 81690148  lwz r11, 0x148(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(328 as u32) ) } as u64;
	// 8317CB74: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317CB78: 409A0014  bne cr6, 0x8317cb8c
	if !ctx.cr[6].eq {
	pc = 0x8317CB8C; continue 'dispatch;
	}
	// 8317CB7C: 81690144  lwz r11, 0x144(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(324 as u32) ) } as u64;
	// 8317CB80: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317CB84: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317CB88: 48000024  b 0x8317cbac
	pc = 0x8317CBAC; continue 'dispatch;
	// 8317CB8C: 81690118  lwz r11, 0x118(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(280 as u32) ) } as u64;
	// 8317CB90: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317CB94: 419A001C  beq cr6, 0x8317cbb0
	if ctx.cr[6].eq {
	pc = 0x8317CBB0; continue 'dispatch;
	}
	// 8317CB98: 80A90140  lwz r5, 0x140(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(320 as u32) ) } as u64;
	// 8317CB9C: 8069013C  lwz r3, 0x13c(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(316 as u32) ) } as u64;
	// 8317CBA0: 4800EC39  bl 0x8318b7d8
	ctx.lr = 0x8317CBA4;
	sub_8318B7D8(ctx, base);
	// 8317CBA4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317CBA8: 7D635A14  add r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 8317CBAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317CBB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317CBB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317CBB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317CBBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317CBC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317CBC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317CBC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317CBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317CBD0 size=116
    let mut pc: u32 = 0x8317CBD0;
    'dispatch: loop {
        match pc {
            0x8317CBD0 => {
    //   block [0x8317CBD0..0x8317CC44)
	// 8317CBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317CBD4: 4802B599  bl 0x831a816c
	ctx.lr = 0x8317CBD8;
	sub_831A8130(ctx, base);
	// 8317CBD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317CBDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317CBE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8317CBE4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317CBE8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8317CBEC: 4800A805  bl 0x831873f0
	ctx.lr = 0x8317CBF0;
	sub_831873F0(ctx, base);
	// 8317CBF0: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8317CBF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317CBF8: 4BFFC819  bl 0x83179410
	ctx.lr = 0x8317CBFC;
	sub_83179410(ctx, base);
	// 8317CBFC: 39630362  addi r11, r3, 0x362
	ctx.r[11].s64 = ctx.r[3].s64 + 866;
	// 8317CC00: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317CC04: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8317CC08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317CC0C: 409A000C  bne cr6, 0x8317cc18
	if !ctx.cr[6].eq {
	pc = 0x8317CC18; continue 'dispatch;
	}
	// 8317CC10: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 8317CC14: 396BC628  addi r11, r11, -0x39d8
	ctx.r[11].s64 = ctx.r[11].s64 + -14808;
	// 8317CC18: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8317CC1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8317CC20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317CC24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317CC28: 4E800421  bctrl
	ctx.lr = 0x8317CC2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317CC2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317CC30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8317CC34: 4800A7CD  bl 0x83187400
	ctx.lr = 0x8317CC38;
	sub_83187400(ctx, base);
	// 8317CC38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317CC3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317CC40: 4802B57C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317CC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317CC48 size=144
    let mut pc: u32 = 0x8317CC48;
    'dispatch: loop {
        match pc {
            0x8317CC48 => {
    //   block [0x8317CC48..0x8317CCD8)
	// 8317CC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317CC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317CC50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317CC54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317CC58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317CC5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317CC60: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8317CC64: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8317CC68: 3BFE0D88  addi r31, r30, 0xd88
	ctx.r[31].s64 = ctx.r[30].s64 + 3464;
	// 8317CC6C: 4BFFFF65  bl 0x8317cbd0
	ctx.lr = 0x8317CC70;
	sub_8317CBD0(ctx, base);
	// 8317CC70: 817F0290  lwz r11, 0x290(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(656 as u32) ) } as u64;
	// 8317CC74: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317CC78: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8317CC7C: 409A0014  bne cr6, 0x8317cc90
	if !ctx.cr[6].eq {
	pc = 0x8317CC90; continue 'dispatch;
	}
	// 8317CC80: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8317CC84: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317CC88: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8317CC8C: 419A002C  beq cr6, 0x8317ccb8
	if ctx.cr[6].eq {
	pc = 0x8317CCB8; continue 'dispatch;
	}
	// 8317CC90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8317CC94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317CC98: 4BFFE979  bl 0x8317b610
	ctx.lr = 0x8317CC9C;
	sub_8317B610(ctx, base);
	// 8317CC9C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317CCA0: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317CCA4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8317CCA8: 917F0290  stw r11, 0x290(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(656 as u32), ctx.r[11].u32 ) };
	// 8317CCAC: 915F0294  stw r10, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[10].u32 ) };
	// 8317CCB0: 913E0044  stw r9, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 8317CCB4: 4800000C  b 0x8317ccc0
	pc = 0x8317CCC0; continue 'dispatch;
	// 8317CCB8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8317CCBC: 917E0044  stw r11, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8317CCC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317CCC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317CCC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317CCCC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317CCD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317CCD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317CCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317CCD8 size=76
    let mut pc: u32 = 0x8317CCD8;
    'dispatch: loop {
        match pc {
            0x8317CCD8 => {
    //   block [0x8317CCD8..0x8317CD24)
	// 8317CCD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317CCDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317CCE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317CCE4: 81630A44  lwz r11, 0xa44(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2628 as u32) ) } as u64;
	// 8317CCE8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317CCEC: 419A0018  beq cr6, 0x8317cd04
	if ctx.cr[6].eq {
	pc = 0x8317CD04; continue 'dispatch;
	}
	// 8317CCF0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317CCF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317CCF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317CCFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317CD00: 4E800020  blr
	return;
	// 8317CD04: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8317CD08: 80E30ABC  lwz r7, 0xabc(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2748 as u32) ) } as u64;
	// 8317CD0C: 4BFFFB75  bl 0x8317c880
	ctx.lr = 0x8317CD10;
	sub_8317C880(ctx, base);
	// 8317CD10: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317CD14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317CD18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317CD1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317CD20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317CD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317CD28 size=72
    let mut pc: u32 = 0x8317CD28;
    'dispatch: loop {
        match pc {
            0x8317CD28 => {
    //   block [0x8317CD28..0x8317CD70)
	// 8317CD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317CD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317CD30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317CD34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317CD38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317CD3C: 4BFFF80D  bl 0x8317c548
	ctx.lr = 0x8317CD40;
	sub_8317C548(ctx, base);
	// 8317CD40: 38800047  li r4, 0x47
	ctx.r[4].s64 = 71;
	// 8317CD44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317CD48: 4BFFC6C9  bl 0x83179410
	ctx.lr = 0x8317CD4C;
	sub_83179410(ctx, base);
	// 8317CD4C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317CD50: 409A000C  bne cr6, 0x8317cd5c
	if !ctx.cr[6].eq {
	pc = 0x8317CD5C; continue 'dispatch;
	}
	// 8317CD54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317CD58: 4BFFFEF1  bl 0x8317cc48
	ctx.lr = 0x8317CD5C;
	sub_8317CC48(ctx, base);
	// 8317CD5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317CD60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317CD64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317CD68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317CD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317CD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317CD70 size=84
    let mut pc: u32 = 0x8317CD70;
    'dispatch: loop {
        match pc {
            0x8317CD70 => {
    //   block [0x8317CD70..0x8317CDC4)
	// 8317CD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317CD74: 4802B3F9  bl 0x831a816c
	ctx.lr = 0x8317CD78;
	sub_831A8130(ctx, base);
	// 8317CD78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317CD7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317CD80: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317CD84: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8317CD88: 4800A559  bl 0x831872e0
	ctx.lr = 0x8317CD8C;
	sub_831872E0(ctx, base);
	// 8317CD8C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317CD90: 419A001C  beq cr6, 0x8317cdac
	if ctx.cr[6].eq {
	pc = 0x8317CDAC; continue 'dispatch;
	}
	// 8317CD94: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317CD98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317CD9C: 60840121  ori r4, r4, 0x121
	ctx.r[4].u64 = ctx.r[4].u64 | 289;
	// 8317CDA0: 4800A759  bl 0x831874f8
	ctx.lr = 0x8317CDA4;
	sub_831874F8(ctx, base);
	// 8317CDA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317CDA8: 4802B414  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317CDAC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8317CDB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8317CDB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317CDB8: 4BFFFD89  bl 0x8317cb40
	ctx.lr = 0x8317CDBC;
	sub_8317CB40(ctx, base);
	// 8317CDBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317CDC0: 4802B3FC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317CDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317CDC8 size=120
    let mut pc: u32 = 0x8317CDC8;
    'dispatch: loop {
        match pc {
            0x8317CDC8 => {
    //   block [0x8317CDC8..0x8317CE40)
	// 8317CDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317CDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317CDD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317CDD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317CDD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317CDDC: 4800A505  bl 0x831872e0
	ctx.lr = 0x8317CDE0;
	sub_831872E0(ctx, base);
	// 8317CDE0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317CDE4: 419A0028  beq cr6, 0x8317ce0c
	if ctx.cr[6].eq {
	pc = 0x8317CE0C; continue 'dispatch;
	}
	// 8317CDE8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317CDEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317CDF0: 6084012A  ori r4, r4, 0x12a
	ctx.r[4].u64 = ctx.r[4].u64 | 298;
	// 8317CDF4: 4800A705  bl 0x831874f8
	ctx.lr = 0x8317CDF8;
	sub_831874F8(ctx, base);
	// 8317CDF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317CDFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317CE00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317CE04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317CE08: 4E800020  blr
	return;
	// 8317CE0C: 38800047  li r4, 0x47
	ctx.r[4].s64 = 71;
	// 8317CE10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317CE14: 4BFFC5FD  bl 0x83179410
	ctx.lr = 0x8317CE18;
	sub_83179410(ctx, base);
	// 8317CE18: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317CE1C: 419A000C  beq cr6, 0x8317ce28
	if ctx.cr[6].eq {
	pc = 0x8317CE28; continue 'dispatch;
	}
	// 8317CE20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317CE24: 4BFFFE25  bl 0x8317cc48
	ctx.lr = 0x8317CE28;
	sub_8317CC48(ctx, base);
	// 8317CE28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317CE2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317CE30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317CE34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317CE38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317CE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317CE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317CE40 size=16
    let mut pc: u32 = 0x8317CE40;
    'dispatch: loop {
        match pc {
            0x8317CE40 => {
    //   block [0x8317CE40..0x8317CE50)
	// 8317CE40: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8317CE44: 409A000C  bne cr6, 0x8317ce50
	if !ctx.cr[6].eq {
		sub_8317CE50(ctx, base);
		return;
	}
	// 8317CE48: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317CE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317CE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317CE50 size=12
    let mut pc: u32 = 0x8317CE50;
    'dispatch: loop {
        match pc {
            0x8317CE50 => {
    //   block [0x8317CE50..0x8317CE5C)
	// 8317CE50: 80A40018  lwz r5, 0x18(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8317CE54: 80840014  lwz r4, 0x14(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8317CE58: 4BFFFE80  b 0x8317ccd8
	sub_8317CCD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317CE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317CE60 size=88
    let mut pc: u32 = 0x8317CE60;
    'dispatch: loop {
        match pc {
            0x8317CE60 => {
    //   block [0x8317CE60..0x8317CEB8)
	// 8317CE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317CE64: 4802B309  bl 0x831a816c
	ctx.lr = 0x8317CE68;
	sub_831A8130(ctx, base);
	// 8317CE68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317CE6C: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8317CE70: 3BCBA100  addi r30, r11, -0x5f00
	ctx.r[30].s64 = ctx.r[11].s64 + -24320;
	// 8317CE74: 3BFE020C  addi r31, r30, 0x20c
	ctx.r[31].s64 = ctx.r[30].s64 + 524;
	// 8317CE78: 817E01B0  lwz r11, 0x1b0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(432 as u32) ) } as u64;
	// 8317CE7C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8317CE80: 917E01B0  stw r11, 0x1b0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(432 as u32), ctx.r[11].u32 ) };
	// 8317CE84: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317CE88: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317CE8C: 4800A455  bl 0x831872e0
	ctx.lr = 0x8317CE90;
	sub_831872E0(ctx, base);
	// 8317CE90: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8317CE94: 419A000C  beq cr6, 0x8317cea0
	if ctx.cr[6].eq {
	pc = 0x8317CEA0; continue 'dispatch;
	}
	// 8317CE98: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317CE9C: 4BFFFE8D  bl 0x8317cd28
	ctx.lr = 0x8317CEA0;
	sub_8317CD28(ctx, base);
	// 8317CEA0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8317CEA4: 397E022C  addi r11, r30, 0x22c
	ctx.r[11].s64 = ctx.r[30].s64 + 556;
	// 8317CEA8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317CEAC: 4198FFD8  blt cr6, 0x8317ce84
	if ctx.cr[6].lt {
	pc = 0x8317CE84; continue 'dispatch;
	}
	// 8317CEB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317CEB4: 4802B308  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317CEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317CEB8 size=104
    let mut pc: u32 = 0x8317CEB8;
    'dispatch: loop {
        match pc {
            0x8317CEB8 => {
    //   block [0x8317CEB8..0x8317CF20)
	// 8317CEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317CEBC: 4802B2B1  bl 0x831a816c
	ctx.lr = 0x8317CEC0;
	sub_831A8130(ctx, base);
	// 8317CEC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317CEC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317CEC8: 409A004C  bne cr6, 0x8317cf14
	if !ctx.cr[6].eq {
	pc = 0x8317CF14; continue 'dispatch;
	}
	// 8317CECC: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8317CED0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8317CED4: 3BCBA100  addi r30, r11, -0x5f00
	ctx.r[30].s64 = ctx.r[11].s64 + -24320;
	// 8317CED8: 3BFE020C  addi r31, r30, 0x20c
	ctx.r[31].s64 = ctx.r[30].s64 + 524;
	// 8317CEDC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317CEE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317CEE4: 419A0014  beq cr6, 0x8317cef8
	if ctx.cr[6].eq {
	pc = 0x8317CEF8; continue 'dispatch;
	}
	// 8317CEE8: 4BFFFEE1  bl 0x8317cdc8
	ctx.lr = 0x8317CEEC;
	sub_8317CDC8(ctx, base);
	// 8317CEEC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317CEF0: 419A0008  beq cr6, 0x8317cef8
	if ctx.cr[6].eq {
	pc = 0x8317CEF8; continue 'dispatch;
	}
	// 8317CEF4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8317CEF8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8317CEFC: 397E022C  addi r11, r30, 0x22c
	ctx.r[11].s64 = ctx.r[30].s64 + 556;
	// 8317CF00: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317CF04: 4198FFD8  blt cr6, 0x8317cedc
	if ctx.cr[6].lt {
	pc = 0x8317CEDC; continue 'dispatch;
	}
	// 8317CF08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317CF0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317CF10: 4802B2AC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317CF14: 4BFFFEB5  bl 0x8317cdc8
	ctx.lr = 0x8317CF18;
	sub_8317CDC8(ctx, base);
	// 8317CF18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317CF1C: 4802B2A0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317CF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317CF20 size=80
    let mut pc: u32 = 0x8317CF20;
    'dispatch: loop {
        match pc {
            0x8317CF20 => {
    //   block [0x8317CF20..0x8317CF70)
	// 8317CF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317CF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317CF28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317CF2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317CF30: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317CF34: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8317CF38: 3BEBA5B0  addi r31, r11, -0x5a50
	ctx.r[31].s64 = ctx.r[11].s64 + -23120;
	// 8317CF3C: 387FFEDC  addi r3, r31, -0x124
	ctx.r[3].s64 = ctx.r[31].s64 + -292;
	// 8317CF40: 4BFB1F51  bl 0x8312ee90
	ctx.lr = 0x8317CF44;
	sub_8312EE90(ctx, base);
	// 8317CF44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317CF48: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8317CF4C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8317CF50: 917FFEFC  stw r11, -0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-260 as u32), ctx.r[11].u32 ) };
	// 8317CF54: 917FFED8  stw r11, -0x128(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-296 as u32), ctx.r[11].u32 ) };
	// 8317CF58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317CF5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317CF60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317CF64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317CF68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317CF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317CF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317CF70 size=64
    let mut pc: u32 = 0x8317CF70;
    'dispatch: loop {
        match pc {
            0x8317CF70 => {
    //   block [0x8317CF70..0x8317CFB0)
	// 8317CF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317CF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317CF78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317CF7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317CF80: 3FE0833A  lis r31, -0x7cc6
	ctx.r[31].s64 = -2093350912;
	// 8317CF84: 807FA5B8  lwz r3, -0x5a48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-23112 as u32) ) } as u64;
	// 8317CF88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317CF8C: 419A0010  beq cr6, 0x8317cf9c
	if ctx.cr[6].eq {
	pc = 0x8317CF9C; continue 'dispatch;
	}
	// 8317CF90: 4BFB1FE1  bl 0x8312ef70
	ctx.lr = 0x8317CF94;
	sub_8312EF70(ctx, base);
	// 8317CF94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317CF98: 917FA5B8  stw r11, -0x5a48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-23112 as u32), ctx.r[11].u32 ) };
	// 8317CF9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317CFA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317CFA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317CFA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317CFAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317CFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317CFB0 size=72
    let mut pc: u32 = 0x8317CFB0;
    'dispatch: loop {
        match pc {
            0x8317CFB0 => {
    //   block [0x8317CFB0..0x8317CFF8)
	// 8317CFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317CFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317CFB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317CFBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317CFC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317CFC4: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8317CFC8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8317CFCC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8317CFD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8317CFD4: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 8317CFD8: 4BFA9B51  bl 0x83126b28
	ctx.lr = 0x8317CFDC;
	sub_83126B28(ctx, base);
	// 8317CFDC: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317CFE0: 93EBA4AC  stw r31, -0x5b54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-23380 as u32), ctx.r[31].u32 ) };
	// 8317CFE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317CFE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317CFEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317CFF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317CFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317CFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317CFF8 size=16
    let mut pc: u32 = 0x8317CFF8;
    'dispatch: loop {
        match pc {
            0x8317CFF8 => {
    //   block [0x8317CFF8..0x8317D008)
	// 8317CFF8: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317CFFC: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 8317D000: 808BA4AC  lwz r4, -0x5b54(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23380 as u32) ) } as u64;
	// 8317D004: 4BFA9A74  b 0x83126a78
	sub_83126A78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D008 size=56
    let mut pc: u32 = 0x8317D008;
    'dispatch: loop {
        match pc {
            0x8317D008 => {
    //   block [0x8317D008..0x8317D040)
	// 8317D008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D014: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8317D018: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8317D01C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8317D020: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 8317D024: 4BFA997D  bl 0x831269a0
	ctx.lr = 0x8317D028;
	sub_831269A0(ctx, base);
	// 8317D028: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317D02C: 906BA488  stw r3, -0x5b78(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-23416 as u32), ctx.r[3].u32 ) };
	// 8317D030: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D03C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317D040 size=16
    let mut pc: u32 = 0x8317D040;
    'dispatch: loop {
        match pc {
            0x8317D040 => {
    //   block [0x8317D040..0x8317D050)
	// 8317D040: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317D044: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 8317D048: 808BA488  lwz r4, -0x5b78(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23416 as u32) ) } as u64;
	// 8317D04C: 4BFA9A2C  b 0x83126a78
	sub_83126A78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D050 size=56
    let mut pc: u32 = 0x8317D050;
    'dispatch: loop {
        match pc {
            0x8317D050 => {
    //   block [0x8317D050..0x8317D088)
	// 8317D050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D05C: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8317D060: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8317D064: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8317D068: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 8317D06C: 4BFA9935  bl 0x831269a0
	ctx.lr = 0x8317D070;
	sub_831269A0(ctx, base);
	// 8317D070: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317D074: 906BA5B0  stw r3, -0x5a50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-23120 as u32), ctx.r[3].u32 ) };
	// 8317D078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D07C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317D088 size=16
    let mut pc: u32 = 0x8317D088;
    'dispatch: loop {
        match pc {
            0x8317D088 => {
    //   block [0x8317D088..0x8317D098)
	// 8317D088: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317D08C: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 8317D090: 808BA5B0  lwz r4, -0x5a50(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23120 as u32) ) } as u64;
	// 8317D094: 4BFA99E4  b 0x83126a78
	sub_83126A78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D098 size=44
    let mut pc: u32 = 0x8317D098;
    'dispatch: loop {
        match pc {
            0x8317D098 => {
    //   block [0x8317D098..0x8317D0C4)
	// 8317D098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D0A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D0A4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317D0A8: 806BA5B8  lwz r3, -0x5a48(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23112 as u32) ) } as u64;
	// 8317D0AC: 4BFB1F55  bl 0x8312f000
	ctx.lr = 0x8317D0B0;
	sub_8312F000(ctx, base);
	// 8317D0B0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317D0B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317D0C8 size=12
    let mut pc: u32 = 0x8317D0C8;
    'dispatch: loop {
        match pc {
            0x8317D0C8 => {
    //   block [0x8317D0C8..0x8317D0D4)
	// 8317D0C8: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317D0CC: 806BA5B8  lwz r3, -0x5a48(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23112 as u32) ) } as u64;
	// 8317D0D0: 4BFB1FC8  b 0x8312f098
	sub_8312F098(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D0D8 size=64
    let mut pc: u32 = 0x8317D0D8;
    'dispatch: loop {
        match pc {
            0x8317D0D8 => {
    //   block [0x8317D0D8..0x8317D118)
	// 8317D0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D0E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317D0E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D0E8: 3FE0833A  lis r31, -0x7cc6
	ctx.r[31].s64 = -2093350912;
	// 8317D0EC: 807FA5BC  lwz r3, -0x5a44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-23108 as u32) ) } as u64;
	// 8317D0F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317D0F4: 419A0010  beq cr6, 0x8317d104
	if ctx.cr[6].eq {
	pc = 0x8317D104; continue 'dispatch;
	}
	// 8317D0F8: 4BFB1E79  bl 0x8312ef70
	ctx.lr = 0x8317D0FC;
	sub_8312EF70(ctx, base);
	// 8317D0FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317D100: 917FA5BC  stw r11, -0x5a44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-23108 as u32), ctx.r[11].u32 ) };
	// 8317D104: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D10C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D110: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317D114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317D118 size=12
    let mut pc: u32 = 0x8317D118;
    'dispatch: loop {
        match pc {
            0x8317D118 => {
    //   block [0x8317D118..0x8317D124)
	// 8317D118: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317D11C: 806BA5BC  lwz r3, -0x5a44(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23108 as u32) ) } as u64;
	// 8317D120: 4BFB1EE0  b 0x8312f000
	sub_8312F000(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317D128 size=12
    let mut pc: u32 = 0x8317D128;
    'dispatch: loop {
        match pc {
            0x8317D128 => {
    //   block [0x8317D128..0x8317D134)
	// 8317D128: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317D12C: 806BA5BC  lwz r3, -0x5a44(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23108 as u32) ) } as u64;
	// 8317D130: 4BFB1F68  b 0x8312f098
	sub_8312F098(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317D138 size=4
    let mut pc: u32 = 0x8317D138;
    'dispatch: loop {
        match pc {
            0x8317D138 => {
    //   block [0x8317D138..0x8317D13C)
	// 8317D138: 4BFA97E8  b 0x83126920
	sub_83126920(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D140 size=192
    let mut pc: u32 = 0x8317D140;
    'dispatch: loop {
        match pc {
            0x8317D140 => {
    //   block [0x8317D140..0x8317D200)
	// 8317D140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D144: 4802B029  bl 0x831a816c
	ctx.lr = 0x8317D148;
	sub_831A8130(ctx, base);
	// 8317D148: F8810018  std r4, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.r[4].u64 ) };
	// 8317D14C: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 8317D150: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 8317D154: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 8317D158: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 8317D15C: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 8317D160: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 8317D164: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D168: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317D16C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317D170: 3BEBA4B0  addi r31, r11, -0x5b50
	ctx.r[31].s64 = ctx.r[11].s64 + -23376;
	// 8317D174: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 8317D178: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317D17C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317D180: 4802B061  bl 0x831a81e0
	ctx.lr = 0x8317D184;
	sub_831A81E0(ctx, base);
	// 8317D184: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8317D188: 39410098  addi r10, r1, 0x98
	ctx.r[10].s64 = ctx.r[1].s64 + 152;
	// 8317D18C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8317D190: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317D194: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8317D198: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317D19C: 4802EC65  bl 0x831abe00
	ctx.lr = 0x8317D1A0;
	sub_831ABE00(ctx, base);
	// 8317D1A0: 3FC08345  lis r30, -0x7cbb
	ctx.r[30].s64 = -2092630016;
	// 8317D1A4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8317D1A8: 3BABCDC0  addi r29, r11, -0x3240
	ctx.r[29].s64 = ctx.r[11].s64 + -12864;
	// 8317D1AC: 807EA374  lwz r3, -0x5c8c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 8317D1B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317D1B4: 419A001C  beq cr6, 0x8317d1d0
	if ctx.cr[6].eq {
	pc = 0x8317D1D0; continue 'dispatch;
	}
	// 8317D1B8: 93FD000C  stw r31, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8317D1BC: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 8317D1C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317D1C4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317D1C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317D1CC: 4E800421  bctrl
	ctx.lr = 0x8317D1D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317D1D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317D1D4: 4BFBB71D  bl 0x831388f0
	ctx.lr = 0x8317D1D8;
	sub_831388F0(ctx, base);
	// 8317D1D8: 807EA374  lwz r3, -0x5c8c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 8317D1DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317D1E0: 419A0018  beq cr6, 0x8317d1f8
	if ctx.cr[6].eq {
	pc = 0x8317D1F8; continue 'dispatch;
	}
	// 8317D1E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317D1E8: 389D006C  addi r4, r29, 0x6c
	ctx.r[4].s64 = ctx.r[29].s64 + 108;
	// 8317D1EC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317D1F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317D1F4: 4E800421  bctrl
	ctx.lr = 0x8317D1F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317D1F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317D1FC: 4802AFC0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D200 size=80
    let mut pc: u32 = 0x8317D200;
    'dispatch: loop {
        match pc {
            0x8317D200 => {
    //   block [0x8317D200..0x8317D250)
	// 8317D200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D208: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317D20C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D210: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317D214: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8317D218: 3BEBA468  addi r31, r11, -0x5b98
	ctx.r[31].s64 = ctx.r[11].s64 + -23448;
	// 8317D21C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317D220: 4BFB1C71  bl 0x8312ee90
	ctx.lr = 0x8317D224;
	sub_8312EE90(ctx, base);
	// 8317D224: 907F0154  stw r3, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[3].u32 ) };
	// 8317D228: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317D22C: 409A0010  bne cr6, 0x8317d23c
	if !ctx.cr[6].eq {
	pc = 0x8317D23C; continue 'dispatch;
	}
	// 8317D230: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317D234: 386BAD68  addi r3, r11, -0x5298
	ctx.r[3].s64 = ctx.r[11].s64 + -21144;
	// 8317D238: 4BFFFF09  bl 0x8317d140
	ctx.lr = 0x8317D23C;
	sub_8317D140(ctx, base);
	// 8317D23C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317D240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D248: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317D24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D250 size=128
    let mut pc: u32 = 0x8317D250;
    'dispatch: loop {
        match pc {
            0x8317D250 => {
    //   block [0x8317D250..0x8317D2D0)
	// 8317D250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D258: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317D25C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D260: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317D264: 387F04A0  addi r3, r31, 0x4a0
	ctx.r[3].s64 = ctx.r[31].s64 + 1184;
	// 8317D268: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317D26C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317D270: 409A0028  bne cr6, 0x8317d298
	if !ctx.cr[6].eq {
	pc = 0x8317D298; continue 'dispatch;
	}
	// 8317D274: 4BFF5DA5  bl 0x83173018
	ctx.lr = 0x8317D278;
	sub_83173018(ctx, base);
	// 8317D278: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 8317D27C: 409A001C  bne cr6, 0x8317d298
	if !ctx.cr[6].eq {
	pc = 0x8317D298; continue 'dispatch;
	}
	// 8317D280: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317D284: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D28C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D290: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317D294: 4E800020  blr
	return;
	// 8317D298: 387F04C4  addi r3, r31, 0x4c4
	ctx.r[3].s64 = ctx.r[31].s64 + 1220;
	// 8317D29C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317D2A0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317D2A4: 409A0014  bne cr6, 0x8317d2b8
	if !ctx.cr[6].eq {
	pc = 0x8317D2B8; continue 'dispatch;
	}
	// 8317D2A8: 4BFF5D71  bl 0x83173018
	ctx.lr = 0x8317D2AC;
	sub_83173018(ctx, base);
	// 8317D2AC: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 8317D2B0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317D2B4: 419A0008  beq cr6, 0x8317d2bc
	if ctx.cr[6].eq {
	pc = 0x8317D2BC; continue 'dispatch;
	}
	// 8317D2B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317D2BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D2C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D2C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D2C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317D2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D2D0 size=276
    let mut pc: u32 = 0x8317D2D0;
    'dispatch: loop {
        match pc {
            0x8317D2D0 => {
    //   block [0x8317D2D0..0x8317D3E4)
	// 8317D2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D2D4: 4802AE99  bl 0x831a816c
	ctx.lr = 0x8317D2D8;
	sub_831A8130(ctx, base);
	// 8317D2D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D2DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317D2E0: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8317D2E4: 4BFFC06D  bl 0x83179350
	ctx.lr = 0x8317D2E8;
	sub_83179350(ctx, base);
	// 8317D2E8: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8317D2EC: 409A00F0  bne cr6, 0x8317d3dc
	if !ctx.cr[6].eq {
	pc = 0x8317D3DC; continue 'dispatch;
	}
	// 8317D2F0: 3BBF04A0  addi r29, r31, 0x4a0
	ctx.r[29].s64 = ctx.r[31].s64 + 1184;
	// 8317D2F4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317D2F8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317D2FC: 409A0034  bne cr6, 0x8317d330
	if !ctx.cr[6].eq {
	pc = 0x8317D330; continue 'dispatch;
	}
	// 8317D300: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317D304: 4BFF5D15  bl 0x83173018
	ctx.lr = 0x8317D308;
	sub_83173018(ctx, base);
	// 8317D308: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8317D30C: 419A0024  beq cr6, 0x8317d330
	if ctx.cr[6].eq {
	pc = 0x8317D330; continue 'dispatch;
	}
	// 8317D310: 807D0014  lwz r3, 0x14(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 8317D314: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8317D318: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317D31C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317D320: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317D324: 4E800421  bctrl
	ctx.lr = 0x8317D328;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317D328: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317D32C: 409A00B0  bne cr6, 0x8317d3dc
	if !ctx.cr[6].eq {
	pc = 0x8317D3DC; continue 'dispatch;
	}
	// 8317D330: 3BDF04C4  addi r30, r31, 0x4c4
	ctx.r[30].s64 = ctx.r[31].s64 + 1220;
	// 8317D334: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317D338: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317D33C: 409A0034  bne cr6, 0x8317d370
	if !ctx.cr[6].eq {
	pc = 0x8317D370; continue 'dispatch;
	}
	// 8317D340: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317D344: 4BFF5CD5  bl 0x83173018
	ctx.lr = 0x8317D348;
	sub_83173018(ctx, base);
	// 8317D348: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8317D34C: 419A0024  beq cr6, 0x8317d370
	if ctx.cr[6].eq {
	pc = 0x8317D370; continue 'dispatch;
	}
	// 8317D350: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8317D354: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8317D358: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317D35C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317D360: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317D364: 4E800421  bctrl
	ctx.lr = 0x8317D368;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317D368: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317D36C: 409A0070  bne cr6, 0x8317d3dc
	if !ctx.cr[6].eq {
	pc = 0x8317D3DC; continue 'dispatch;
	}
	// 8317D370: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317D374: 4BFF8E1D  bl 0x83176190
	ctx.lr = 0x8317D378;
	sub_83176190(ctx, base);
	// 8317D378: 897F0082  lbz r11, 0x82(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(130 as u32) ) } as u64;
	// 8317D37C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317D380: 409A0010  bne cr6, 0x8317d390
	if !ctx.cr[6].eq {
	pc = 0x8317D390; continue 'dispatch;
	}
	// 8317D384: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317D388: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317D38C: 4BFF905D  bl 0x831763e8
	ctx.lr = 0x8317D390;
	sub_831763E8(ctx, base);
	// 8317D390: 897F0080  lbz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8317D394: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8317D398: 409A0020  bne cr6, 0x8317d3b8
	if !ctx.cr[6].eq {
	pc = 0x8317D3B8; continue 'dispatch;
	}
	// 8317D39C: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8317D3A0: 4800E5E1  bl 0x8318b980
	ctx.lr = 0x8317D3A4;
	sub_8318B980(ctx, base);
	// 8317D3A4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317D3A8: 419A0010  beq cr6, 0x8317d3b8
	if ctx.cr[6].eq {
	pc = 0x8317D3B8; continue 'dispatch;
	}
	// 8317D3AC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317D3B0: 386BAD98  addi r3, r11, -0x5268
	ctx.r[3].s64 = ctx.r[11].s64 + -21096;
	// 8317D3B4: 4BFFFD8D  bl 0x8317d140
	ctx.lr = 0x8317D3B8;
	sub_8317D140(ctx, base);
	// 8317D3B8: 897F0082  lbz r11, 0x82(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(130 as u32) ) } as u64;
	// 8317D3BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317D3C0: 409A001C  bne cr6, 0x8317d3dc
	if !ctx.cr[6].eq {
	pc = 0x8317D3DC; continue 'dispatch;
	}
	// 8317D3C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317D3C8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317D3CC: 4BFF5CBD  bl 0x83173088
	ctx.lr = 0x8317D3D0;
	sub_83173088(ctx, base);
	// 8317D3D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317D3D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317D3D8: 4BFF5CB1  bl 0x83173088
	ctx.lr = 0x8317D3DC;
	sub_83173088(ctx, base);
	// 8317D3DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317D3E0: 4802ADDC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D3E8 size=128
    let mut pc: u32 = 0x8317D3E8;
    'dispatch: loop {
        match pc {
            0x8317D3E8 => {
    //   block [0x8317D3E8..0x8317D468)
	// 8317D3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D3F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317D3F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D3F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317D3FC: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8317D400: 4BFFBF51  bl 0x83179350
	ctx.lr = 0x8317D404;
	sub_83179350(ctx, base);
	// 8317D404: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8317D408: 409A004C  bne cr6, 0x8317d454
	if !ctx.cr[6].eq {
	pc = 0x8317D454; continue 'dispatch;
	}
	// 8317D40C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317D410: 4BFF8D81  bl 0x83176190
	ctx.lr = 0x8317D414;
	sub_83176190(ctx, base);
	// 8317D414: 897F0082  lbz r11, 0x82(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(130 as u32) ) } as u64;
	// 8317D418: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317D41C: 409A0010  bne cr6, 0x8317d42c
	if !ctx.cr[6].eq {
	pc = 0x8317D42C; continue 'dispatch;
	}
	// 8317D420: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317D424: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317D428: 4BFF8FC1  bl 0x831763e8
	ctx.lr = 0x8317D42C;
	sub_831763E8(ctx, base);
	// 8317D42C: 897F0080  lbz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8317D430: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8317D434: 409A0020  bne cr6, 0x8317d454
	if !ctx.cr[6].eq {
	pc = 0x8317D454; continue 'dispatch;
	}
	// 8317D438: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8317D43C: 4800E545  bl 0x8318b980
	ctx.lr = 0x8317D440;
	sub_8318B980(ctx, base);
	// 8317D440: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317D444: 419A0010  beq cr6, 0x8317d454
	if ctx.cr[6].eq {
	pc = 0x8317D454; continue 'dispatch;
	}
	// 8317D448: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317D44C: 386BAD98  addi r3, r11, -0x5268
	ctx.r[3].s64 = ctx.r[11].s64 + -21096;
	// 8317D450: 4BFFFCF1  bl 0x8317d140
	ctx.lr = 0x8317D454;
	sub_8317D140(ctx, base);
	// 8317D454: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D45C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D460: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317D464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D468 size=236
    let mut pc: u32 = 0x8317D468;
    'dispatch: loop {
        match pc {
            0x8317D468 => {
    //   block [0x8317D468..0x8317D554)
	// 8317D468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D470: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317D474: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D478: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317D47C: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8317D480: 4BFFDFD1  bl 0x8317b450
	ctx.lr = 0x8317D484;
	sub_8317B450(ctx, base);
	// 8317D484: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8317D488: 409A001C  bne cr6, 0x8317d4a4
	if !ctx.cr[6].eq {
	pc = 0x8317D4A4; continue 'dispatch;
	}
	// 8317D48C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8317D490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D49C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317D4A0: 4E800020  blr
	return;
	// 8317D4A4: 817F0538  lwz r11, 0x538(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1336 as u32) ) } as u64;
	// 8317D4A8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317D4AC: 419A0090  beq cr6, 0x8317d53c
	if ctx.cr[6].eq {
	pc = 0x8317D53C; continue 'dispatch;
	}
	// 8317D4B0: 807F0450  lwz r3, 0x450(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1104 as u32) ) } as u64;
	// 8317D4B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317D4B8: 419A0014  beq cr6, 0x8317d4cc
	if ctx.cr[6].eq {
	pc = 0x8317D4CC; continue 'dispatch;
	}
	// 8317D4BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317D4C0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8317D4C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317D4C8: 4E800421  bctrl
	ctx.lr = 0x8317D4CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317D4CC: 80FF044C  lwz r7, 0x44c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1100 as u32) ) } as u64;
	// 8317D4D0: 80DF0448  lwz r6, 0x448(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1096 as u32) ) } as u64;
	// 8317D4D4: 80BF0444  lwz r5, 0x444(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1092 as u32) ) } as u64;
	// 8317D4D8: 809F0438  lwz r4, 0x438(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 8317D4DC: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8317D4E0: 4BFFDED9  bl 0x8317b3b8
	ctx.lr = 0x8317D4E4;
	sub_8317B3B8(ctx, base);
	// 8317D4E4: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8317D4E8: 4BFFDF29  bl 0x8317b410
	ctx.lr = 0x8317D4EC;
	sub_8317B410(ctx, base);
	// 8317D4EC: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8317D4F0: 409A0044  bne cr6, 0x8317d534
	if !ctx.cr[6].eq {
	pc = 0x8317D534; continue 'dispatch;
	}
	// 8317D4F4: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8317D4F8: 3860FF9A  li r3, -0x66
	ctx.r[3].s64 = -102;
	// 8317D4FC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8317D500: 4BFF60C9  bl 0x831735c8
	ctx.lr = 0x8317D504;
	sub_831735C8(ctx, base);
	// 8317D504: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317D508: 809F0438  lwz r4, 0x438(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 8317D50C: 386BADC4  addi r3, r11, -0x523c
	ctx.r[3].s64 = ctx.r[11].s64 + -21052;
	// 8317D510: 4BFFFC31  bl 0x8317d140
	ctx.lr = 0x8317D514;
	sub_8317D140(ctx, base);
	// 8317D514: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317D518: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8317D51C: 917F0440  stw r11, 0x440(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1088 as u32), ctx.r[11].u32 ) };
	// 8317D520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D52C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317D530: 4E800020  blr
	return;
	// 8317D534: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317D538: 4BFF6AF1  bl 0x83174028
	ctx.lr = 0x8317D53C;
	sub_83174028(ctx, base);
	// 8317D53C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317D540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D54C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317D550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D558 size=152
    let mut pc: u32 = 0x8317D558;
    'dispatch: loop {
        match pc {
            0x8317D558 => {
    //   block [0x8317D558..0x8317D5F0)
	// 8317D558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D55C: 4802AC0D  bl 0x831a8168
	ctx.lr = 0x8317D560;
	sub_831A8130(ctx, base);
	// 8317D560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D564: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317D568: 817F0538  lwz r11, 0x538(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1336 as u32) ) } as u64;
	// 8317D56C: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8317D570: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317D574: 409A0074  bne cr6, 0x8317d5e8
	if !ctx.cr[6].eq {
	pc = 0x8317D5E8; continue 'dispatch;
	}
	// 8317D578: 4BFFBDD9  bl 0x83179350
	ctx.lr = 0x8317D57C;
	sub_83179350(ctx, base);
	// 8317D57C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8317D580: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317D584: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317D588: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 8317D58C: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 8317D590: 4BFF56B9  bl 0x83172c48
	ctx.lr = 0x8317D594;
	sub_83172C48(ctx, base);
	// 8317D594: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317D598: 419A0010  beq cr6, 0x8317d5a8
	if ctx.cr[6].eq {
	pc = 0x8317D5A8; continue 'dispatch;
	}
	// 8317D59C: 387F04A0  addi r3, r31, 0x4a0
	ctx.r[3].s64 = ctx.r[31].s64 + 1184;
	// 8317D5A0: 4BFF5A79  bl 0x83173018
	ctx.lr = 0x8317D5A4;
	sub_83173018(ctx, base);
	// 8317D5A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317D5A8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8317D5AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317D5B0: 4BFF5699  bl 0x83172c48
	ctx.lr = 0x8317D5B4;
	sub_83172C48(ctx, base);
	// 8317D5B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317D5B8: 419A0010  beq cr6, 0x8317d5c8
	if ctx.cr[6].eq {
	pc = 0x8317D5C8; continue 'dispatch;
	}
	// 8317D5BC: 387F04C4  addi r3, r31, 0x4c4
	ctx.r[3].s64 = ctx.r[31].s64 + 1220;
	// 8317D5C0: 4BFF5A59  bl 0x83173018
	ctx.lr = 0x8317D5C4;
	sub_83173018(ctx, base);
	// 8317D5C4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8317D5C8: 2F1C0006  cmpwi cr6, r28, 6
	ctx.cr[6].compare_i32(ctx.r[28].s32, 6, &mut ctx.xer);
	// 8317D5CC: 409A001C  bne cr6, 0x8317d5e8
	if !ctx.cr[6].eq {
	pc = 0x8317D5E8; continue 'dispatch;
	}
	// 8317D5D0: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 8317D5D4: 419A0014  beq cr6, 0x8317d5e8
	if ctx.cr[6].eq {
	pc = 0x8317D5E8; continue 'dispatch;
	}
	// 8317D5D8: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 8317D5DC: 419A000C  beq cr6, 0x8317d5e8
	if ctx.cr[6].eq {
	pc = 0x8317D5E8; continue 'dispatch;
	}
	// 8317D5E0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8317D5E4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8317D5E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317D5EC: 4802ABCC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D5F0 size=108
    let mut pc: u32 = 0x8317D5F0;
    'dispatch: loop {
        match pc {
            0x8317D5F0 => {
    //   block [0x8317D5F0..0x8317D65C)
	// 8317D5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D5F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317D5FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317D600: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D604: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317D608: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 8317D60C: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8317D610: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317D614: 419A0014  beq cr6, 0x8317d628
	if ctx.cr[6].eq {
	pc = 0x8317D628; continue 'dispatch;
	}
	// 8317D618: 4BFFDD49  bl 0x8317b360
	ctx.lr = 0x8317D61C;
	sub_8317B360(ctx, base);
	// 8317D61C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317D620: 419A0008  beq cr6, 0x8317d628
	if ctx.cr[6].eq {
	pc = 0x8317D628; continue 'dispatch;
	}
	// 8317D624: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8317D628: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317D62C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317D630: 419A0014  beq cr6, 0x8317d644
	if ctx.cr[6].eq {
	pc = 0x8317D644; continue 'dispatch;
	}
	// 8317D634: 4BFFB465  bl 0x83178a98
	ctx.lr = 0x8317D638;
	sub_83178A98(ctx, base);
	// 8317D638: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317D63C: 409A0008  bne cr6, 0x8317d644
	if !ctx.cr[6].eq {
	pc = 0x8317D644; continue 'dispatch;
	}
	// 8317D640: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8317D644: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317D648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D64C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D650: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317D654: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317D658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D660 size=132
    let mut pc: u32 = 0x8317D660;
    'dispatch: loop {
        match pc {
            0x8317D660 => {
    //   block [0x8317D660..0x8317D6E4)
	// 8317D660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D668: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317D66C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D670: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317D674: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317D678: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317D67C: 419A0054  beq cr6, 0x8317d6d0
	if ctx.cr[6].eq {
	pc = 0x8317D6D0; continue 'dispatch;
	}
	// 8317D680: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8317D684: 419A004C  beq cr6, 0x8317d6d0
	if ctx.cr[6].eq {
	pc = 0x8317D6D0; continue 'dispatch;
	}
	// 8317D688: 4BFF5439  bl 0x83172ac0
	ctx.lr = 0x8317D68C;
	sub_83172AC0(ctx, base);
	// 8317D68C: 48005835  bl 0x83182ec0
	ctx.lr = 0x8317D690;
	sub_83182EC0(ctx, base);
	// 8317D690: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317D694: 409A003C  bne cr6, 0x8317d6d0
	if !ctx.cr[6].eq {
	pc = 0x8317D6D0; continue 'dispatch;
	}
	// 8317D698: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317D69C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317D6A0: 4BFF55A9  bl 0x83172c48
	ctx.lr = 0x8317D6A4;
	sub_83172C48(ctx, base);
	// 8317D6A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317D6A8: 419A000C  beq cr6, 0x8317d6b4
	if ctx.cr[6].eq {
	pc = 0x8317D6B4; continue 'dispatch;
	}
	// 8317D6AC: 387F04A0  addi r3, r31, 0x4a0
	ctx.r[3].s64 = ctx.r[31].s64 + 1184;
	// 8317D6B0: 4BFF5A79  bl 0x83173128
	ctx.lr = 0x8317D6B4;
	sub_83173128(ctx, base);
	// 8317D6B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8317D6B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317D6BC: 4BFF558D  bl 0x83172c48
	ctx.lr = 0x8317D6C0;
	sub_83172C48(ctx, base);
	// 8317D6C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317D6C4: 419A000C  beq cr6, 0x8317d6d0
	if ctx.cr[6].eq {
	pc = 0x8317D6D0; continue 'dispatch;
	}
	// 8317D6C8: 387F04C4  addi r3, r31, 0x4c4
	ctx.r[3].s64 = ctx.r[31].s64 + 1220;
	// 8317D6CC: 4BFF5A5D  bl 0x83173128
	ctx.lr = 0x8317D6D0;
	sub_83173128(ctx, base);
	// 8317D6D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D6D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D6D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D6DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317D6E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D6E8 size=36
    let mut pc: u32 = 0x8317D6E8;
    'dispatch: loop {
        match pc {
            0x8317D6E8 => {
    //   block [0x8317D6E8..0x8317D70C)
	// 8317D6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D6F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D6F4: 4BFF5C95  bl 0x83173388
	ctx.lr = 0x8317D6F8;
	sub_83173388(ctx, base);
	// 8317D6F8: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8317D6FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D710 size=140
    let mut pc: u32 = 0x8317D710;
    'dispatch: loop {
        match pc {
            0x8317D710 => {
    //   block [0x8317D710..0x8317D79C)
	// 8317D710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D718: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317D71C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317D720: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D724: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317D728: 3FE0833A  lis r31, -0x7cc6
	ctx.r[31].s64 = -2093350912;
	// 8317D72C: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 8317D730: 812BA5C0  lwz r9, -0x5a40(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23104 as u32) ) } as u64;
	// 8317D734: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8317D738: 912BA5C0  stw r9, -0x5a40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-23104 as u32), ctx.r[9].u32 ) };
	// 8317D73C: 817F9A74  lwz r11, -0x658c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-25996 as u32) ) } as u64;
	// 8317D740: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317D744: 816AA354  lwz r11, -0x5cac(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-23724 as u32) ) } as u64;
	// 8317D748: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8317D74C: 916AA354  stw r11, -0x5cac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-23724 as u32), ctx.r[11].u32 ) };
	// 8317D750: 409A0034  bne cr6, 0x8317d784
	if !ctx.cr[6].eq {
	pc = 0x8317D784; continue 'dispatch;
	}
	// 8317D754: 4BFF5C35  bl 0x83173388
	ctx.lr = 0x8317D758;
	sub_83173388(ctx, base);
	// 8317D758: 3BC3005C  addi r30, r3, 0x5c
	ctx.r[30].s64 = ctx.r[3].s64 + 92;
	// 8317D75C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317D760: 4BFFF9D9  bl 0x8317d138
	ctx.lr = 0x8317D764;
	sub_8317D138(ctx, base);
	// 8317D764: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317D768: 409A001C  bne cr6, 0x8317d784
	if !ctx.cr[6].eq {
	pc = 0x8317D784; continue 'dispatch;
	}
	// 8317D76C: 817F9A74  lwz r11, -0x658c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-25996 as u32) ) } as u64;
	// 8317D770: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317D774: 409A0008  bne cr6, 0x8317d77c
	if !ctx.cr[6].eq {
	pc = 0x8317D77C; continue 'dispatch;
	}
	// 8317D778: 480050F1  bl 0x83182868
	ctx.lr = 0x8317D77C;
	sub_83182868(ctx, base);
	// 8317D77C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317D780: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317D784: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317D788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D78C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D790: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317D794: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317D798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D7A0 size=56
    let mut pc: u32 = 0x8317D7A0;
    'dispatch: loop {
        match pc {
            0x8317D7A0 => {
    //   block [0x8317D7A0..0x8317D7D8)
	// 8317D7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D7A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D7AC: 4BFF5BDD  bl 0x83173388
	ctx.lr = 0x8317D7B0;
	sub_83173388(ctx, base);
	// 8317D7B0: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 8317D7B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317D7B8: 419A0010  beq cr6, 0x8317d7c8
	if ctx.cr[6].eq {
	pc = 0x8317D7C8; continue 'dispatch;
	}
	// 8317D7BC: 80630044  lwz r3, 0x44(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 8317D7C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317D7C4: 4E800421  bctrl
	ctx.lr = 0x8317D7C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317D7C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D7CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D7D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D7D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D7D8 size=56
    let mut pc: u32 = 0x8317D7D8;
    'dispatch: loop {
        match pc {
            0x8317D7D8 => {
    //   block [0x8317D7D8..0x8317D810)
	// 8317D7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D7E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D7E4: 4BFF5BA5  bl 0x83173388
	ctx.lr = 0x8317D7E8;
	sub_83173388(ctx, base);
	// 8317D7E8: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 8317D7EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317D7F0: 419A0010  beq cr6, 0x8317d800
	if ctx.cr[6].eq {
	pc = 0x8317D800; continue 'dispatch;
	}
	// 8317D7F4: 8063004C  lwz r3, 0x4c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 8317D7F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317D7FC: 4E800421  bctrl
	ctx.lr = 0x8317D800;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317D800: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D80C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D810 size=56
    let mut pc: u32 = 0x8317D810;
    'dispatch: loop {
        match pc {
            0x8317D810 => {
    //   block [0x8317D810..0x8317D848)
	// 8317D810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D818: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D81C: 4BFF5B6D  bl 0x83173388
	ctx.lr = 0x8317D820;
	sub_83173388(ctx, base);
	// 8317D820: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317D824: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317D828: 419A0010  beq cr6, 0x8317d838
	if ctx.cr[6].eq {
	pc = 0x8317D838; continue 'dispatch;
	}
	// 8317D82C: 80630054  lwz r3, 0x54(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317D830: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317D834: 4E800421  bctrl
	ctx.lr = 0x8317D838;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317D838: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D83C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D848 size=48
    let mut pc: u32 = 0x8317D848;
    'dispatch: loop {
        match pc {
            0x8317D848 => {
    //   block [0x8317D848..0x8317D878)
	// 8317D848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D850: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317D854: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D858: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317D85C: 4BFF5B2D  bl 0x83173388
	ctx.lr = 0x8317D860;
	sub_83173388(ctx, base);
	// 8317D860: 93E30058  stw r31, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8317D864: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D86C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D870: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317D874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D878 size=112
    let mut pc: u32 = 0x8317D878;
    'dispatch: loop {
        match pc {
            0x8317D878 => {
    //   block [0x8317D878..0x8317D8E8)
	// 8317D878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D880: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317D884: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D888: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317D88C: 4BFF4EBD  bl 0x83172748
	ctx.lr = 0x8317D890;
	sub_83172748(ctx, base);
	// 8317D890: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317D894: 419A0028  beq cr6, 0x8317d8bc
	if ctx.cr[6].eq {
	pc = 0x8317D8BC; continue 'dispatch;
	}
	// 8317D898: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317D89C: 386BADF0  addi r3, r11, -0x5210
	ctx.r[3].s64 = ctx.r[11].s64 + -21008;
	// 8317D8A0: 4BFFF8A1  bl 0x8317d140
	ctx.lr = 0x8317D8A4;
	sub_8317D140(ctx, base);
	// 8317D8A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317D8A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D8AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D8B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D8B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317D8B8: 4E800020  blr
	return;
	// 8317D8BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317D8C0: 4BF4BA19  bl 0x830c92d8
	ctx.lr = 0x8317D8C4;
	sub_830C92D8(ctx, base);
	// 8317D8C4: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 8317D8C8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8317D8CC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317D8D0: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8317D8D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D8D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D8DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D8E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317D8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D8E8 size=72
    let mut pc: u32 = 0x8317D8E8;
    'dispatch: loop {
        match pc {
            0x8317D8E8 => {
    //   block [0x8317D8E8..0x8317D930)
	// 8317D8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D8F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317D8F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317D8F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D8FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317D900: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8317D904: 4BFF5A85  bl 0x83173388
	ctx.lr = 0x8317D908;
	sub_83173388(ctx, base);
	// 8317D908: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8317D90C: 419A0008  beq cr6, 0x8317d914
	if ctx.cr[6].eq {
	pc = 0x8317D914; continue 'dispatch;
	}
	// 8317D910: 93FE0068  stw r31, 0x68(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 8317D914: 93E30024  stw r31, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 8317D918: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317D91C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D924: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317D928: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317D92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D930 size=36
    let mut pc: u32 = 0x8317D930;
    'dispatch: loop {
        match pc {
            0x8317D930 => {
    //   block [0x8317D930..0x8317D954)
	// 8317D930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D93C: 4BFF5A4D  bl 0x83173388
	ctx.lr = 0x8317D940;
	sub_83173388(ctx, base);
	// 8317D940: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317D944: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D94C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D958 size=52
    let mut pc: u32 = 0x8317D958;
    'dispatch: loop {
        match pc {
            0x8317D958 => {
    //   block [0x8317D958..0x8317D98C)
	// 8317D958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D960: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D964: 4BFF4D95  bl 0x831726f8
	ctx.lr = 0x8317D968;
	sub_831726F8(ctx, base);
	// 8317D968: 4BFFDA39  bl 0x8317b3a0
	ctx.lr = 0x8317D96C;
	sub_8317B3A0(ctx, base);
	// 8317D96C: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 8317D970: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8317D974: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317D978: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8317D97C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D990 size=48
    let mut pc: u32 = 0x8317D990;
    'dispatch: loop {
        match pc {
            0x8317D990 => {
    //   block [0x8317D990..0x8317D9C0)
	// 8317D990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D998: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317D99C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D9A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317D9A4: 4BFF59E5  bl 0x83173388
	ctx.lr = 0x8317D9A8;
	sub_83173388(ctx, base);
	// 8317D9A8: 93E32A6C  stw r31, 0x2a6c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10860 as u32), ctx.r[31].u32 ) };
	// 8317D9AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D9B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D9B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D9B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317D9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D9C0 size=36
    let mut pc: u32 = 0x8317D9C0;
    'dispatch: loop {
        match pc {
            0x8317D9C0 => {
    //   block [0x8317D9C0..0x8317D9E4)
	// 8317D9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D9C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D9CC: 4BFF59BD  bl 0x83173388
	ctx.lr = 0x8317D9D0;
	sub_83173388(ctx, base);
	// 8317D9D0: 80632A6C  lwz r3, 0x2a6c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10860 as u32) ) } as u64;
	// 8317D9D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317D9D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317D9DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317D9E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317D9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317D9E8 size=120
    let mut pc: u32 = 0x8317D9E8;
    'dispatch: loop {
        match pc {
            0x8317D9E8 => {
    //   block [0x8317D9E8..0x8317DA60)
	// 8317D9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317D9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317D9F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317D9F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317D9F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317D9FC: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317DA00: 816B9A74  lwz r11, -0x658c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25996 as u32) ) } as u64;
	// 8317DA04: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317DA08: 409A0040  bne cr6, 0x8317da48
	if !ctx.cr[6].eq {
	pc = 0x8317DA48; continue 'dispatch;
	}
	// 8317DA0C: 4BFF597D  bl 0x83173388
	ctx.lr = 0x8317DA10;
	sub_83173388(ctx, base);
	// 8317DA10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317DA14: 4BFFFFAD  bl 0x8317d9c0
	ctx.lr = 0x8317DA18;
	sub_8317D9C0(ctx, base);
	// 8317DA18: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317DA1C: 419A002C  beq cr6, 0x8317da48
	if ctx.cr[6].eq {
	pc = 0x8317DA48; continue 'dispatch;
	}
	// 8317DA20: 3BFF006C  addi r31, r31, 0x6c
	ctx.r[31].s64 = ctx.r[31].s64 + 108;
	// 8317DA24: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 8317DA28: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8317DA2C: 419A000C  beq cr6, 0x8317da38
	if ctx.cr[6].eq {
	pc = 0x8317DA38; continue 'dispatch;
	}
	// 8317DA30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317DA34: 4BFFB875  bl 0x831792a8
	ctx.lr = 0x8317DA38;
	sub_831792A8(ctx, base);
	// 8317DA38: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8317DA3C: 3BFF0540  addi r31, r31, 0x540
	ctx.r[31].s64 = ctx.r[31].s64 + 1344;
	// 8317DA40: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8317DA44: 409AFFE4  bne cr6, 0x8317da28
	if !ctx.cr[6].eq {
	pc = 0x8317DA28; continue 'dispatch;
	}
	// 8317DA48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317DA4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317DA50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317DA54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317DA58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317DA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317DA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317DA60 size=160
    let mut pc: u32 = 0x8317DA60;
    'dispatch: loop {
        match pc {
            0x8317DA60 => {
    //   block [0x8317DA60..0x8317DB00)
	// 8317DA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317DA64: 4802A709  bl 0x831a816c
	ctx.lr = 0x8317DA68;
	sub_831A8130(ctx, base);
	// 8317DA68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317DA6C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317DA70: 816B9A74  lwz r11, -0x658c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25996 as u32) ) } as u64;
	// 8317DA74: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317DA78: 409A0080  bne cr6, 0x8317daf8
	if !ctx.cr[6].eq {
	pc = 0x8317DAF8; continue 'dispatch;
	}
	// 8317DA7C: 4BFF590D  bl 0x83173388
	ctx.lr = 0x8317DA80;
	sub_83173388(ctx, base);
	// 8317DA80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317DA84: 4BFFFF3D  bl 0x8317d9c0
	ctx.lr = 0x8317DA88;
	sub_8317D9C0(ctx, base);
	// 8317DA88: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317DA8C: 419A006C  beq cr6, 0x8317daf8
	if ctx.cr[6].eq {
	pc = 0x8317DAF8; continue 'dispatch;
	}
	// 8317DA90: 3BFF006C  addi r31, r31, 0x6c
	ctx.r[31].s64 = ctx.r[31].s64 + 108;
	// 8317DA94: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 8317DA98: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8317DA9C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8317DAA0: 419A0048  beq cr6, 0x8317dae8
	if ctx.cr[6].eq {
	pc = 0x8317DAE8; continue 'dispatch;
	}
	// 8317DAA4: 817F0440  lwz r11, 0x440(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1088 as u32) ) } as u64;
	// 8317DAA8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317DAAC: 409A0018  bne cr6, 0x8317dac4
	if !ctx.cr[6].eq {
	pc = 0x8317DAC4; continue 'dispatch;
	}
	// 8317DAB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317DAB4: 4BFFF9B5  bl 0x8317d468
	ctx.lr = 0x8317DAB8;
	sub_8317D468(ctx, base);
	// 8317DAB8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317DABC: 409A0008  bne cr6, 0x8317dac4
	if !ctx.cr[6].eq {
	pc = 0x8317DAC4; continue 'dispatch;
	}
	// 8317DAC0: 93BF0440  stw r29, 0x440(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1088 as u32), ctx.r[29].u32 ) };
	// 8317DAC4: 817F0538  lwz r11, 0x538(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1336 as u32) ) } as u64;
	// 8317DAC8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317DACC: 409A001C  bne cr6, 0x8317dae8
	if !ctx.cr[6].eq {
	pc = 0x8317DAE8; continue 'dispatch;
	}
	// 8317DAD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317DAD4: 4BFF8BB5  bl 0x83176688
	ctx.lr = 0x8317DAD8;
	sub_83176688(ctx, base);
	// 8317DAD8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317DADC: 409A000C  bne cr6, 0x8317dae8
	if !ctx.cr[6].eq {
	pc = 0x8317DAE8; continue 'dispatch;
	}
	// 8317DAE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317DAE4: 4BFF8DC5  bl 0x831768a8
	ctx.lr = 0x8317DAE8;
	sub_831768A8(ctx, base);
	// 8317DAE8: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8317DAEC: 3BFF0540  addi r31, r31, 0x540
	ctx.r[31].s64 = ctx.r[31].s64 + 1344;
	// 8317DAF0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8317DAF4: 409AFFA8  bne cr6, 0x8317da9c
	if !ctx.cr[6].eq {
	pc = 0x8317DA9C; continue 'dispatch;
	}
	// 8317DAF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317DAFC: 4802A6C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317DB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317DB00 size=104
    let mut pc: u32 = 0x8317DB00;
    'dispatch: loop {
        match pc {
            0x8317DB00 => {
    //   block [0x8317DB00..0x8317DB68)
	// 8317DB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317DB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317DB08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317DB0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317DB10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317DB14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317DB18: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8317DB1C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8317DB20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317DB24: 4BFFFDC5  bl 0x8317d8e8
	ctx.lr = 0x8317DB28;
	sub_8317D8E8(ctx, base);
	// 8317DB28: 4BFA77C9  bl 0x831252f0
	ctx.lr = 0x8317DB2C;
	sub_831252F0(ctx, base);
	// 8317DB2C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317DB30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317DB34: 4BFFFDB5  bl 0x8317d8e8
	ctx.lr = 0x8317DB38;
	sub_8317D8E8(ctx, base);
	// 8317DB38: 817E006C  lwz r11, 0x6c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 8317DB3C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317DB40: 409A0010  bne cr6, 0x8317db50
	if !ctx.cr[6].eq {
	pc = 0x8317DB50; continue 'dispatch;
	}
	// 8317DB44: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8317DB48: 2F1F000A  cmpwi cr6, r31, 0xa
	ctx.cr[6].compare_i32(ctx.r[31].s32, 10, &mut ctx.xer);
	// 8317DB4C: 4198FFD0  blt cr6, 0x8317db1c
	if ctx.cr[6].lt {
	pc = 0x8317DB1C; continue 'dispatch;
	}
	// 8317DB50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317DB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317DB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317DB5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317DB60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317DB64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317DB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317DB68 size=28
    let mut pc: u32 = 0x8317DB68;
    'dispatch: loop {
        match pc {
            0x8317DB68 => {
    //   block [0x8317DB68..0x8317DB84)
	// 8317DB68: 816304A0  lwz r11, 0x4a0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1184 as u32) ) } as u64;
	// 8317DB6C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317DB70: 419A0014  beq cr6, 0x8317db84
	if ctx.cr[6].eq {
		sub_8317DB84(ctx, base);
		return;
	}
	// 8317DB74: 816304C4  lwz r11, 0x4c4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1220 as u32) ) } as u64;
	// 8317DB78: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317DB7C: 419A0008  beq cr6, 0x8317db84
	if ctx.cr[6].eq {
		sub_8317DB84(ctx, base);
		return;
	}
	// 8317DB80: 4BFFF868  b 0x8317d3e8
	sub_8317D3E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317DB84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317DB84 size=4
    let mut pc: u32 = 0x8317DB84;
    'dispatch: loop {
        match pc {
            0x8317DB84 => {
    //   block [0x8317DB84..0x8317DB88)
	// 8317DB84: 4BFFF74C  b 0x8317d2d0
	sub_8317D2D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317DB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317DB88 size=160
    let mut pc: u32 = 0x8317DB88;
    'dispatch: loop {
        match pc {
            0x8317DB88 => {
    //   block [0x8317DB88..0x8317DC28)
	// 8317DB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317DB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317DB90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317DB94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317DB98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317DB9C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317DBA0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317DBA4: 419A000C  beq cr6, 0x8317dbb0
	if ctx.cr[6].eq {
	pc = 0x8317DBB0; continue 'dispatch;
	}
	// 8317DBA8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8317DBAC: 409A0060  bne cr6, 0x8317dc0c
	if !ctx.cr[6].eq {
	pc = 0x8317DC0C; continue 'dispatch;
	}
	// 8317DBB0: 897F0081  lbz r11, 0x81(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(129 as u32) ) } as u64;
	// 8317DBB4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8317DBB8: 409A0024  bne cr6, 0x8317dbdc
	if !ctx.cr[6].eq {
	pc = 0x8317DBDC; continue 'dispatch;
	}
	// 8317DBBC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317DBC0: 4BFB7E71  bl 0x83135a30
	ctx.lr = 0x8317DBC4;
	sub_83135A30(ctx, base);
	// 8317DBC4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317DBC8: 409A0014  bne cr6, 0x8317dbdc
	if !ctx.cr[6].eq {
	pc = 0x8317DBDC; continue 'dispatch;
	}
	// 8317DBCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317DBD0: 4BFF8619  bl 0x831761e8
	ctx.lr = 0x8317DBD4;
	sub_831761E8(ctx, base);
	// 8317DBD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317DBD8: 997F0081  stb r11, 0x81(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(129 as u32), ctx.r[11].u8 ) };
	// 8317DBDC: 897F0080  lbz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8317DBE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317DBE4: 409A0028  bne cr6, 0x8317dc0c
	if !ctx.cr[6].eq {
	pc = 0x8317DC0C; continue 'dispatch;
	}
	// 8317DBE8: 897F0081  lbz r11, 0x81(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(129 as u32) ) } as u64;
	// 8317DBEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317DBF0: 409A001C  bne cr6, 0x8317dc0c
	if !ctx.cr[6].eq {
	pc = 0x8317DC0C; continue 'dispatch;
	}
	// 8317DBF4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317DBF8: 4BFB7E39  bl 0x83135a30
	ctx.lr = 0x8317DBFC;
	sub_83135A30(ctx, base);
	// 8317DBFC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317DC00: 409A000C  bne cr6, 0x8317dc0c
	if !ctx.cr[6].eq {
	pc = 0x8317DC0C; continue 'dispatch;
	}
	// 8317DC04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317DC08: 4BFF8951  bl 0x83176558
	ctx.lr = 0x8317DC0C;
	sub_83176558(ctx, base);
	// 8317DC0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317DC10: 4BFFFA51  bl 0x8317d660
	ctx.lr = 0x8317DC14;
	sub_8317D660(ctx, base);
	// 8317DC14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317DC18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317DC1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317DC20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317DC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317DC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317DC28 size=124
    let mut pc: u32 = 0x8317DC28;
    'dispatch: loop {
        match pc {
            0x8317DC28 => {
    //   block [0x8317DC28..0x8317DCA4)
	// 8317DC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317DC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317DC30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317DC34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317DC38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317DC3C: 3FE08345  lis r31, -0x7cbb
	ctx.r[31].s64 = -2092630016;
	// 8317DC40: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8317DC44: 3BCBD2D0  addi r30, r11, -0x2d30
	ctx.r[30].s64 = ctx.r[11].s64 + -11568;
	// 8317DC48: 807FA374  lwz r3, -0x5c8c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 8317DC4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317DC50: 419A0018  beq cr6, 0x8317dc68
	if ctx.cr[6].eq {
	pc = 0x8317DC68; continue 'dispatch;
	}
	// 8317DC54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317DC58: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 8317DC5C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317DC60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317DC64: 4E800421  bctrl
	ctx.lr = 0x8317DC68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317DC68: 4BFFFAA9  bl 0x8317d710
	ctx.lr = 0x8317DC6C;
	sub_8317D710(ctx, base);
	// 8317DC6C: 807FA374  lwz r3, -0x5c8c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 8317DC70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317DC74: 419A0018  beq cr6, 0x8317dc8c
	if ctx.cr[6].eq {
	pc = 0x8317DC8C; continue 'dispatch;
	}
	// 8317DC78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317DC7C: 389E006C  addi r4, r30, 0x6c
	ctx.r[4].s64 = ctx.r[30].s64 + 108;
	// 8317DC80: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317DC84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317DC88: 4E800421  bctrl
	ctx.lr = 0x8317DC8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317DC8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317DC90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317DC94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317DC98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317DC9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317DCA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317DCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317DCA8 size=12
    let mut pc: u32 = 0x8317DCA8;
    'dispatch: loop {
        match pc {
            0x8317DCA8 => {
    //   block [0x8317DCA8..0x8317DCB4)
	// 8317DCA8: 8163006C  lwz r11, 0x6c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 8317DCAC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317DCB0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317DCB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317DCB4 size=8
    let mut pc: u32 = 0x8317DCB4;
    'dispatch: loop {
        match pc {
            0x8317DCB4 => {
    //   block [0x8317DCB4..0x8317DCBC)
	// 8317DCB4: 4BFFFE4C  b 0x8317db00
	sub_8317DB00(ctx, base);
	return;
	// 8317DCB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317DCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317DCC0 size=108
    let mut pc: u32 = 0x8317DCC0;
    'dispatch: loop {
        match pc {
            0x8317DCC0 => {
    //   block [0x8317DCC0..0x8317DD2C)
	// 8317DCC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317DCC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317DCC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317DCCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317DCD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317DCD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317DCD8: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8317DCDC: 4BFF4B2D  bl 0x83172808
	ctx.lr = 0x8317DCE0;
	sub_83172808(ctx, base);
	// 8317DCE0: 817F0440  lwz r11, 0x440(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1088 as u32) ) } as u64;
	// 8317DCE4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317DCE8: 419A002C  beq cr6, 0x8317dd14
	if ctx.cr[6].eq {
	pc = 0x8317DD14; continue 'dispatch;
	}
	// 8317DCEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317DCF0: 4BFFFE79  bl 0x8317db68
	ctx.lr = 0x8317DCF4;
	sub_8317DB68(ctx, base);
	// 8317DCF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317DCF8: 4BFFB659  bl 0x83179350
	ctx.lr = 0x8317DCFC;
	sub_83179350(ctx, base);
	// 8317DCFC: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 8317DD00: 419A000C  beq cr6, 0x8317dd0c
	if ctx.cr[6].eq {
	pc = 0x8317DD0C; continue 'dispatch;
	}
	// 8317DD04: 2F030006  cmpwi cr6, r3, 6
	ctx.cr[6].compare_i32(ctx.r[3].s32, 6, &mut ctx.xer);
	// 8317DD08: 409A000C  bne cr6, 0x8317dd14
	if !ctx.cr[6].eq {
	pc = 0x8317DD14; continue 'dispatch;
	}
	// 8317DD0C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8317DD10: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8317DD14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317DD18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317DD1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317DD20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317DD24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317DD28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317DD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317DD30 size=96
    let mut pc: u32 = 0x8317DD30;
    'dispatch: loop {
        match pc {
            0x8317DD30 => {
    //   block [0x8317DD30..0x8317DD90)
	// 8317DD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317DD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317DD38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317DD3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317DD40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317DD44: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317DD48: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317DD4C: 419A0014  beq cr6, 0x8317dd60
	if ctx.cr[6].eq {
	pc = 0x8317DD60; continue 'dispatch;
	}
	// 8317DD50: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8317DD54: 409A0014  bne cr6, 0x8317dd68
	if !ctx.cr[6].eq {
	pc = 0x8317DD68; continue 'dispatch;
	}
	// 8317DD58: 4BFFF801  bl 0x8317d558
	ctx.lr = 0x8317DD5C;
	sub_8317D558(ctx, base);
	// 8317DD5C: 4800000C  b 0x8317dd68
	pc = 0x8317DD68; continue 'dispatch;
	// 8317DD60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317DD64: 4BFFFF5D  bl 0x8317dcc0
	ctx.lr = 0x8317DD68;
	sub_8317DCC0(ctx, base);
	// 8317DD68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317DD6C: 4BFFF885  bl 0x8317d5f0
	ctx.lr = 0x8317DD70;
	sub_8317D5F0(ctx, base);
	// 8317DD70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317DD74: 4BFFFE15  bl 0x8317db88
	ctx.lr = 0x8317DD78;
	sub_8317DB88(ctx, base);
	// 8317DD78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317DD7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317DD80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317DD84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317DD88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317DD8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317DD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317DD90 size=36
    let mut pc: u32 = 0x8317DD90;
    'dispatch: loop {
        match pc {
            0x8317DD90 => {
    //   block [0x8317DD90..0x8317DDB4)
	// 8317DD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317DD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317DD98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317DD9C: 4BFFFE8D  bl 0x8317dc28
	ctx.lr = 0x8317DDA0;
	sub_8317DC28(ctx, base);
	// 8317DDA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317DDA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317DDA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317DDAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317DDB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317DDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317DDB8 size=200
    let mut pc: u32 = 0x8317DDB8;
    'dispatch: loop {
        match pc {
            0x8317DDB8 => {
    //   block [0x8317DDB8..0x8317DE80)
	// 8317DDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317DDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317DDC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317DDC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317DDC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317DDCC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8317DDD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317DDD4: 4800C195  bl 0x83189f68
	ctx.lr = 0x8317DDD8;
	sub_83189F68(ctx, base);
	// 8317DDD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317DDDC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317DDE0: 419A0014  beq cr6, 0x8317ddf4
	if ctx.cr[6].eq {
	pc = 0x8317DDF4; continue 'dispatch;
	}
	// 8317DDE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317DDE8: 4800C181  bl 0x83189f68
	ctx.lr = 0x8317DDEC;
	sub_83189F68(ctx, base);
	// 8317DDEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317DDF0: 48000078  b 0x8317de68
	pc = 0x8317DE68; continue 'dispatch;
	// 8317DDF4: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8317DDF8: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8317DDFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8317DE00: 93EBA358  stw r31, -0x5ca8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-23720 as u32), ctx.r[31].u32 ) };
	// 8317DE04: 4800C19D  bl 0x83189fa0
	ctx.lr = 0x8317DE08;
	sub_83189FA0(ctx, base);
	// 8317DE08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317DE0C: 48006F2D  bl 0x83184d38
	ctx.lr = 0x8317DE10;
	sub_83184D38(ctx, base);
	// 8317DE10: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317DE14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317DE18: 4800C189  bl 0x83189fa0
	ctx.lr = 0x8317DE1C;
	sub_83189FA0(ctx, base);
	// 8317DE1C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317DE20: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317DE24: 409A000C  bne cr6, 0x8317de30
	if !ctx.cr[6].eq {
	pc = 0x8317DE30; continue 'dispatch;
	}
	// 8317DE28: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8317DE2C: 48000014  b 0x8317de40
	pc = 0x8317DE40; continue 'dispatch;
	// 8317DE30: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8317DE34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317DE38: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8317DE3C: 4BFFFEF5  bl 0x8317dd30
	ctx.lr = 0x8317DE40;
	sub_8317DD30(ctx, base);
	// 8317DE40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317DE44: 48004A2D  bl 0x83182870
	ctx.lr = 0x8317DE48;
	sub_83182870(ctx, base);
	// 8317DE48: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 8317DE4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317DE50: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8317DE54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317DE58: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317DE5C: 696A0001  xori r10, r11, 1
	ctx.r[10].u64 = ctx.r[11].u64 ^ 1;
	// 8317DE60: 4800C109  bl 0x83189f68
	ctx.lr = 0x8317DE64;
	sub_83189F68(ctx, base);
	// 8317DE64: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 8317DE68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317DE6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317DE70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317DE74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317DE78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317DE7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317DE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317DE80 size=84
    let mut pc: u32 = 0x8317DE80;
    'dispatch: loop {
        match pc {
            0x8317DE80 => {
    //   block [0x8317DE80..0x8317DED4)
	// 8317DE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317DE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317DE88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317DE8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317DE90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317DE94: 4BF49C4D  bl 0x830c7ae0
	ctx.lr = 0x8317DE98;
	sub_830C7AE0(ctx, base);
	// 8317DE98: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8317DE9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317DEA0: 4BFFFA49  bl 0x8317d8e8
	ctx.lr = 0x8317DEA4;
	sub_8317D8E8(ctx, base);
	// 8317DEA4: 4BF49C3D  bl 0x830c7ae0
	ctx.lr = 0x8317DEA8;
	sub_830C7AE0(ctx, base);
	// 8317DEA8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317DEAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317DEB0: 4BFFFA39  bl 0x8317d8e8
	ctx.lr = 0x8317DEB4;
	sub_8317D8E8(ctx, base);
	// 8317DEB4: 4BF49C2D  bl 0x830c7ae0
	ctx.lr = 0x8317DEB8;
	sub_830C7AE0(ctx, base);
	// 8317DEB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317DEBC: 4BFFFDED  bl 0x8317dca8
	ctx.lr = 0x8317DEC0;
	sub_8317DCA8(ctx, base);
	// 8317DEC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317DEC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317DEC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317DECC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317DED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317DED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317DED8 size=136
    let mut pc: u32 = 0x8317DED8;
    'dispatch: loop {
        match pc {
            0x8317DED8 => {
    //   block [0x8317DED8..0x8317DF60)
	// 8317DED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317DEDC: 4802A291  bl 0x831a816c
	ctx.lr = 0x8317DEE0;
	sub_831A8130(ctx, base);
	// 8317DEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317DEE4: 3FE08345  lis r31, -0x7cbb
	ctx.r[31].s64 = -2092630016;
	// 8317DEE8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8317DEEC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8317DEF0: 3BCBCE98  addi r30, r11, -0x3168
	ctx.r[30].s64 = ctx.r[11].s64 + -12648;
	// 8317DEF4: 807FA374  lwz r3, -0x5c8c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 8317DEF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317DEFC: 419A0018  beq cr6, 0x8317df14
	if ctx.cr[6].eq {
	pc = 0x8317DF14; continue 'dispatch;
	}
	// 8317DF00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317DF04: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 8317DF08: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317DF0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317DF10: 4E800421  bctrl
	ctx.lr = 0x8317DF14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317DF14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317DF18: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317DF1C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8317DF20: 4BFFFE71  bl 0x8317dd90
	ctx.lr = 0x8317DF24;
	sub_8317DD90(ctx, base);
	// 8317DF24: 817FA374  lwz r11, -0x5c8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 8317DF28: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8317DF2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317DF30: 419A0028  beq cr6, 0x8317df58
	if ctx.cr[6].eq {
	pc = 0x8317DF58; continue 'dispatch;
	}
	// 8317DF34: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8317DF38: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8317DF3C: 389E006C  addi r4, r30, 0x6c
	ctx.r[4].s64 = ctx.r[30].s64 + 108;
	// 8317DF40: 915E0074  stw r10, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 8317DF44: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317DF48: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317DF4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317DF50: 4E800421  bctrl
	ctx.lr = 0x8317DF54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317DF54: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317DF58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317DF5C: 4802A260  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317DF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317DF60 size=244
    let mut pc: u32 = 0x8317DF60;
    'dispatch: loop {
        match pc {
            0x8317DF60 => {
    //   block [0x8317DF60..0x8317E054)
	// 8317DF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317DF64: 4802A209  bl 0x831a816c
	ctx.lr = 0x8317DF68;
	sub_831A8130(ctx, base);
	// 8317DF68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317DF6C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317DF70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317DF74: 816B9A74  lwz r11, -0x658c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25996 as u32) ) } as u64;
	// 8317DF78: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317DF7C: 409A0018  bne cr6, 0x8317df94
	if !ctx.cr[6].eq {
	pc = 0x8317DF94; continue 'dispatch;
	}
	// 8317DF80: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8317DF84: 409A001C  bne cr6, 0x8317dfa0
	if !ctx.cr[6].eq {
	pc = 0x8317DFA0; continue 'dispatch;
	}
	// 8317DF88: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317DF8C: 386BAE24  addi r3, r11, -0x51dc
	ctx.r[3].s64 = ctx.r[11].s64 + -20956;
	// 8317DF90: 4BFFF1B1  bl 0x8317d140
	ctx.lr = 0x8317DF94;
	sub_8317D140(ctx, base);
	// 8317DF94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317DF98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317DF9C: 4802A220  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317DFA0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317DFA4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317DFA8: 409AFFEC  bne cr6, 0x8317df94
	if !ctx.cr[6].eq {
	pc = 0x8317DF94; continue 'dispatch;
	}
	// 8317DFAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317DFB0: 4BF4B329  bl 0x830c92d8
	ctx.lr = 0x8317DFB4;
	sub_830C92D8(ctx, base);
	// 8317DFB4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317DFB8: 419AFFDC  beq cr6, 0x8317df94
	if ctx.cr[6].eq {
	pc = 0x8317DF94; continue 'dispatch;
	}
	// 8317DFBC: 4BFFF975  bl 0x8317d930
	ctx.lr = 0x8317DFC0;
	sub_8317D930(ctx, base);
	// 8317DFC0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317DFC4: 419AFFD0  beq cr6, 0x8317df94
	if ctx.cr[6].eq {
	pc = 0x8317DF94; continue 'dispatch;
	}
	// 8317DFC8: 817F0538  lwz r11, 0x538(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1336 as u32) ) } as u64;
	// 8317DFCC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317DFD0: 419AFFC4  beq cr6, 0x8317df94
	if ctx.cr[6].eq {
	pc = 0x8317DF94; continue 'dispatch;
	}
	// 8317DFD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317DFD8: 4BFF6C79  bl 0x83174c50
	ctx.lr = 0x8317DFDC;
	sub_83174C50(ctx, base);
	// 8317DFDC: 3FA08345  lis r29, -0x7cbb
	ctx.r[29].s64 = -2092630016;
	// 8317DFE0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8317DFE4: 3BCBD1F8  addi r30, r11, -0x2e08
	ctx.r[30].s64 = ctx.r[11].s64 + -11784;
	// 8317DFE8: 807DA374  lwz r3, -0x5c8c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 8317DFEC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317DFF0: 419A001C  beq cr6, 0x8317e00c
	if ctx.cr[6].eq {
	pc = 0x8317E00C; continue 'dispatch;
	}
	// 8317DFF4: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8317DFF8: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 8317DFFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E000: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317E004: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317E008: 4E800421  bctrl
	ctx.lr = 0x8317E00C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317E00C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317E010: 4BFFFDA9  bl 0x8317ddb8
	ctx.lr = 0x8317E014;
	sub_8317DDB8(ctx, base);
	// 8317E014: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8317E018: 807DA374  lwz r3, -0x5c8c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 8317E01C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317E020: 419A0020  beq cr6, 0x8317e040
	if ctx.cr[6].eq {
	pc = 0x8317E040; continue 'dispatch;
	}
	// 8317E024: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8317E028: 389E006C  addi r4, r30, 0x6c
	ctx.r[4].s64 = ctx.r[30].s64 + 108;
	// 8317E02C: 917E0074  stw r11, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8317E030: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E034: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317E038: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317E03C: 4E800421  bctrl
	ctx.lr = 0x8317E040;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317E040: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317E044: 4BFF6C2D  bl 0x83174c70
	ctx.lr = 0x8317E048;
	sub_83174C70(ctx, base);
	// 8317E048: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317E04C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317E050: 4802A16C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317E058 size=64
    let mut pc: u32 = 0x8317E058;
    'dispatch: loop {
        match pc {
            0x8317E058 => {
    //   block [0x8317E058..0x8317E098)
	// 8317E058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317E05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317E060: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317E064: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317E068: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317E06C: 4BFFF0AD  bl 0x8317d118
	ctx.lr = 0x8317E070;
	sub_8317D118(ctx, base);
	// 8317E070: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317E074: 4BFFFEED  bl 0x8317df60
	ctx.lr = 0x8317E078;
	sub_8317DF60(ctx, base);
	// 8317E078: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317E07C: 4BFFF0AD  bl 0x8317d128
	ctx.lr = 0x8317E080;
	sub_8317D128(ctx, base);
	// 8317E080: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317E084: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317E088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317E08C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317E090: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317E094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317E098 size=184
    let mut pc: u32 = 0x8317E098;
    'dispatch: loop {
        match pc {
            0x8317E098 => {
    //   block [0x8317E098..0x8317E150)
	// 8317E098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317E09C: 4802A0D1  bl 0x831a816c
	ctx.lr = 0x8317E0A0;
	sub_831A8130(ctx, base);
	// 8317E0A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317E0A4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317E0A8: 816B9A74  lwz r11, -0x658c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25996 as u32) ) } as u64;
	// 8317E0AC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317E0B0: 419A0010  beq cr6, 0x8317e0c0
	if ctx.cr[6].eq {
	pc = 0x8317E0C0; continue 'dispatch;
	}
	// 8317E0B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317E0B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317E0BC: 4802A100  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317E0C0: 4BFF52C9  bl 0x83173388
	ctx.lr = 0x8317E0C4;
	sub_83173388(ctx, base);
	// 8317E0C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317E0C8: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 8317E0CC: 4BFFF06D  bl 0x8317d138
	ctx.lr = 0x8317E0D0;
	sub_8317D138(ctx, base);
	// 8317E0D0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317E0D4: 409AFFE0  bne cr6, 0x8317e0b4
	if !ctx.cr[6].eq {
	pc = 0x8317E0B4; continue 'dispatch;
	}
	// 8317E0D8: 4BFFF6C9  bl 0x8317d7a0
	ctx.lr = 0x8317E0DC;
	sub_8317D7A0(ctx, base);
	// 8317E0DC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8317E0E0: 4BFFF8E1  bl 0x8317d9c0
	ctx.lr = 0x8317E0E4;
	sub_8317D9C0(ctx, base);
	// 8317E0E4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317E0E8: 419A0038  beq cr6, 0x8317e120
	if ctx.cr[6].eq {
	pc = 0x8317E120; continue 'dispatch;
	}
	// 8317E0EC: 3BFF006C  addi r31, r31, 0x6c
	ctx.r[31].s64 = ctx.r[31].s64 + 108;
	// 8317E0F0: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 8317E0F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8317E0F8: 419A0018  beq cr6, 0x8317e110
	if ctx.cr[6].eq {
	pc = 0x8317E110; continue 'dispatch;
	}
	// 8317E0FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317E100: 4BFFFF59  bl 0x8317e058
	ctx.lr = 0x8317E104;
	sub_8317E058(ctx, base);
	// 8317E104: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317E108: 409A0008  bne cr6, 0x8317e110
	if !ctx.cr[6].eq {
	pc = 0x8317E110; continue 'dispatch;
	}
	// 8317E10C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8317E110: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8317E114: 3BFF0540  addi r31, r31, 0x540
	ctx.r[31].s64 = ctx.r[31].s64 + 1344;
	// 8317E118: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8317E11C: 409AFFD8  bne cr6, 0x8317e0f4
	if !ctx.cr[6].eq {
	pc = 0x8317E0F4; continue 'dispatch;
	}
	// 8317E120: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317E124: 4BFFF725  bl 0x8317d848
	ctx.lr = 0x8317E128;
	sub_8317D848(ctx, base);
	// 8317E128: 4BFFF6B1  bl 0x8317d7d8
	ctx.lr = 0x8317E12C;
	sub_8317D7D8(ctx, base);
	// 8317E12C: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 8317E130: 419A0014  beq cr6, 0x8317e144
	if ctx.cr[6].eq {
	pc = 0x8317E144; continue 'dispatch;
	}
	// 8317E134: 4BFFF7FD  bl 0x8317d930
	ctx.lr = 0x8317E138;
	sub_8317D930(ctx, base);
	// 8317E138: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317E13C: 419A0008  beq cr6, 0x8317e144
	if ctx.cr[6].eq {
	pc = 0x8317E144; continue 'dispatch;
	}
	// 8317E140: 4BFFF6D1  bl 0x8317d810
	ctx.lr = 0x8317E144;
	sub_8317D810(ctx, base);
	// 8317E144: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317E148: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317E14C: 4802A070  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317E150 size=128
    let mut pc: u32 = 0x8317E150;
    'dispatch: loop {
        match pc {
            0x8317E150 => {
    //   block [0x8317E150..0x8317E1D0)
	// 8317E150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317E154: 4802A019  bl 0x831a816c
	ctx.lr = 0x8317E158;
	sub_831A8130(ctx, base);
	// 8317E158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317E15C: 3FE08345  lis r31, -0x7cbb
	ctx.r[31].s64 = -2092630016;
	// 8317E160: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8317E164: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8317E168: 3BCBD120  addi r30, r11, -0x2ee0
	ctx.r[30].s64 = ctx.r[11].s64 + -12000;
	// 8317E16C: 807FA374  lwz r3, -0x5c8c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 8317E170: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317E174: 419A0018  beq cr6, 0x8317e18c
	if ctx.cr[6].eq {
	pc = 0x8317E18C; continue 'dispatch;
	}
	// 8317E178: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E17C: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 8317E180: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317E184: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317E188: 4E800421  bctrl
	ctx.lr = 0x8317E18C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317E18C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317E190: 4BFFFF09  bl 0x8317e098
	ctx.lr = 0x8317E194;
	sub_8317E098(ctx, base);
	// 8317E194: 817FA374  lwz r11, -0x5c8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 8317E198: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8317E19C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317E1A0: 419A0028  beq cr6, 0x8317e1c8
	if ctx.cr[6].eq {
	pc = 0x8317E1C8; continue 'dispatch;
	}
	// 8317E1A4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8317E1A8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8317E1AC: 389E006C  addi r4, r30, 0x6c
	ctx.r[4].s64 = ctx.r[30].s64 + 108;
	// 8317E1B0: 915E0074  stw r10, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 8317E1B4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E1B8: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317E1BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317E1C0: 4E800421  bctrl
	ctx.lr = 0x8317E1C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317E1C4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317E1C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317E1CC: 48029FF0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317E1D0 size=160
    let mut pc: u32 = 0x8317E1D0;
    'dispatch: loop {
        match pc {
            0x8317E1D0 => {
    //   block [0x8317E1D0..0x8317E270)
	// 8317E1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317E1D4: 48029F99  bl 0x831a816c
	ctx.lr = 0x8317E1D8;
	sub_831A8130(ctx, base);
	// 8317E1D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317E1DC: 3FA08345  lis r29, -0x7cbb
	ctx.r[29].s64 = -2092630016;
	// 8317E1E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317E1E4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8317E1E8: 3BCBCF70  addi r30, r11, -0x3090
	ctx.r[30].s64 = ctx.r[11].s64 + -12432;
	// 8317E1EC: 807DA374  lwz r3, -0x5c8c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 8317E1F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317E1F4: 419A0018  beq cr6, 0x8317e20c
	if ctx.cr[6].eq {
	pc = 0x8317E20C; continue 'dispatch;
	}
	// 8317E1F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E1FC: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 8317E200: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317E204: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317E208: 4E800421  bctrl
	ctx.lr = 0x8317E20C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317E20C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317E210: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317E214: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8317E218: 4BFFF7D1  bl 0x8317d9e8
	ctx.lr = 0x8317E21C;
	sub_8317D9E8(ctx, base);
	// 8317E21C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317E220: 4BFFF841  bl 0x8317da60
	ctx.lr = 0x8317E224;
	sub_8317DA60(ctx, base);
	// 8317E224: 4BFFF4C5  bl 0x8317d6e8
	ctx.lr = 0x8317E228;
	sub_8317D6E8(ctx, base);
	// 8317E228: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317E22C: 409A0010  bne cr6, 0x8317e23c
	if !ctx.cr[6].eq {
	pc = 0x8317E23C; continue 'dispatch;
	}
	// 8317E230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317E234: 4BFFFF1D  bl 0x8317e150
	ctx.lr = 0x8317E238;
	sub_8317E150(ctx, base);
	// 8317E238: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8317E23C: 807DA374  lwz r3, -0x5c8c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 8317E240: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317E244: 419A0020  beq cr6, 0x8317e264
	if ctx.cr[6].eq {
	pc = 0x8317E264; continue 'dispatch;
	}
	// 8317E248: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8317E24C: 389E006C  addi r4, r30, 0x6c
	ctx.r[4].s64 = ctx.r[30].s64 + 108;
	// 8317E250: 917E0074  stw r11, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8317E254: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E258: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317E25C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317E260: 4E800421  bctrl
	ctx.lr = 0x8317E264;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317E264: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317E268: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317E26C: 48029F50  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317E270 size=144
    let mut pc: u32 = 0x8317E270;
    'dispatch: loop {
        match pc {
            0x8317E270 => {
    //   block [0x8317E270..0x8317E300)
	// 8317E270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317E274: 48029EF9  bl 0x831a816c
	ctx.lr = 0x8317E278;
	sub_831A8130(ctx, base);
	// 8317E278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317E27C: 3FC08345  lis r30, -0x7cbb
	ctx.r[30].s64 = -2092630016;
	// 8317E280: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8317E284: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8317E288: 3BEBD048  addi r31, r11, -0x2fb8
	ctx.r[31].s64 = ctx.r[11].s64 + -12216;
	// 8317E28C: 807EA374  lwz r3, -0x5c8c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 8317E290: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317E294: 419A0018  beq cr6, 0x8317e2ac
	if ctx.cr[6].eq {
	pc = 0x8317E2AC; continue 'dispatch;
	}
	// 8317E298: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E29C: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 8317E2A0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317E2A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317E2A8: 4E800421  bctrl
	ctx.lr = 0x8317E2AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317E2AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317E2B0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8317E2B4: 4BFFF435  bl 0x8317d6e8
	ctx.lr = 0x8317E2B8;
	sub_8317D6E8(ctx, base);
	// 8317E2B8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317E2BC: 419A0010  beq cr6, 0x8317e2cc
	if ctx.cr[6].eq {
	pc = 0x8317E2CC; continue 'dispatch;
	}
	// 8317E2C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317E2C4: 4BFFFE8D  bl 0x8317e150
	ctx.lr = 0x8317E2C8;
	sub_8317E150(ctx, base);
	// 8317E2C8: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8317E2CC: 807EA374  lwz r3, -0x5c8c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 8317E2D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317E2D4: 419A0020  beq cr6, 0x8317e2f4
	if ctx.cr[6].eq {
	pc = 0x8317E2F4; continue 'dispatch;
	}
	// 8317E2D8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8317E2DC: 389F006C  addi r4, r31, 0x6c
	ctx.r[4].s64 = ctx.r[31].s64 + 108;
	// 8317E2E0: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8317E2E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E2E8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317E2EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317E2F0: 4E800421  bctrl
	ctx.lr = 0x8317E2F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317E2F4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317E2F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317E2FC: 48029EC0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317E300 size=148
    let mut pc: u32 = 0x8317E300;
    'dispatch: loop {
        match pc {
            0x8317E300 => {
    //   block [0x8317E300..0x8317E394)
	// 8317E300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317E304: 48029E69  bl 0x831a816c
	ctx.lr = 0x8317E308;
	sub_831A8130(ctx, base);
	// 8317E308: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317E30C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317E310: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8317E314: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8317E318: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8317E31C: 419A0034  beq cr6, 0x8317e350
	if ctx.cr[6].eq {
	pc = 0x8317E350; continue 'dispatch;
	}
	// 8317E320: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317E324: 48008FBD  bl 0x831872e0
	ctx.lr = 0x8317E328;
	sub_831872E0(ctx, base);
	// 8317E328: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317E32C: 419A001C  beq cr6, 0x8317e348
	if ctx.cr[6].eq {
	pc = 0x8317E348; continue 'dispatch;
	}
	// 8317E330: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317E334: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317E338: 60840181  ori r4, r4, 0x181
	ctx.r[4].u64 = ctx.r[4].u64 | 385;
	// 8317E33C: 480091BD  bl 0x831874f8
	ctx.lr = 0x8317E340;
	sub_831874F8(ctx, base);
	// 8317E340: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317E344: 48029E78  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317E348: 817F17C8  lwz r11, 0x17c8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6088 as u32) ) } as u64;
	// 8317E34C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E350: 2F1D0005  cmpwi cr6, r29, 5
	ctx.cr[6].compare_i32(ctx.r[29].s32, 5, &mut ctx.xer);
	// 8317E354: 409A0008  bne cr6, 0x8317e35c
	if !ctx.cr[6].eq {
	pc = 0x8317E35C; continue 'dispatch;
	}
	// 8317E358: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8317E35C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8317E360: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8317E364: 48011CDD  bl 0x83190040
	ctx.lr = 0x8317E368;
	sub_83190040(ctx, base);
	// 8317E368: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317E36C: 419A001C  beq cr6, 0x8317e388
	if ctx.cr[6].eq {
	pc = 0x8317E388; continue 'dispatch;
	}
	// 8317E370: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317E374: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317E378: 60840F12  ori r4, r4, 0xf12
	ctx.r[4].u64 = ctx.r[4].u64 | 3858;
	// 8317E37C: 4800917D  bl 0x831874f8
	ctx.lr = 0x8317E380;
	sub_831874F8(ctx, base);
	// 8317E380: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317E384: 48029E38  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317E388: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317E38C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317E390: 48029E2C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317E398 size=116
    let mut pc: u32 = 0x8317E398;
    'dispatch: loop {
        match pc {
            0x8317E398 => {
    //   block [0x8317E398..0x8317E40C)
	// 8317E398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317E39C: 48029DCD  bl 0x831a8168
	ctx.lr = 0x8317E3A0;
	sub_831A8130(ctx, base);
	// 8317E3A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317E3A4: 816317C8  lwz r11, 0x17c8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6088 as u32) ) } as u64;
	// 8317E3A8: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E3AC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8317E3B0: 409A0010  bne cr6, 0x8317e3c0
	if !ctx.cr[6].eq {
	pc = 0x8317E3C0; continue 'dispatch;
	}
	// 8317E3B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317E3B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317E3BC: 48029DFC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8317E3C0: 54BDF0BE  srwi r29, r5, 2
	ctx.r[29].u32 = ctx.r[5].u32.wrapping_shr(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8317E3C4: 2B1D0010  cmplwi cr6, r29, 0x10
	ctx.cr[6].compare_u32(ctx.r[29].u32, 16 as u32, &mut ctx.xer);
	// 8317E3C8: 40990008  ble cr6, 0x8317e3d0
	if !ctx.cr[6].gt {
	pc = 0x8317E3D0; continue 'dispatch;
	}
	// 8317E3CC: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 8317E3D0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8317E3D4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8317E3D8: 40990028  ble cr6, 0x8317e400
	if !ctx.cr[6].gt {
	pc = 0x8317E400; continue 'dispatch;
	}
	// 8317E3DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317E3E0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8317E3E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8317E3E8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8317E3EC: 48011CE5  bl 0x831900d0
	ctx.lr = 0x8317E3F0;
	sub_831900D0(ctx, base);
	// 8317E3F0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8317E3F4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8317E3F8: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8317E3FC: 4198FFE4  blt cr6, 0x8317e3e0
	if ctx.cr[6].lt {
	pc = 0x8317E3E0; continue 'dispatch;
	}
	// 8317E400: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317E404: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317E408: 48029DB0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317E410 size=88
    let mut pc: u32 = 0x8317E410;
    'dispatch: loop {
        match pc {
            0x8317E410 => {
    //   block [0x8317E410..0x8317E468)
	// 8317E410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317E414: 48029D55  bl 0x831a8168
	ctx.lr = 0x8317E418;
	sub_831A8130(ctx, base);
	// 8317E418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317E41C: 816317C8  lwz r11, 0x17c8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6088 as u32) ) } as u64;
	// 8317E420: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8317E424: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E428: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8317E42C: 419A0034  beq cr6, 0x8317e460
	if ctx.cr[6].eq {
	pc = 0x8317E460; continue 'dispatch;
	}
	// 8317E430: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8317E434: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8317E438: 40990028  ble cr6, 0x8317e460
	if !ctx.cr[6].gt {
	pc = 0x8317E460; continue 'dispatch;
	}
	// 8317E43C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317E440: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8317E444: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E448: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8317E44C: 48011BF5  bl 0x83190040
	ctx.lr = 0x8317E450;
	sub_83190040(ctx, base);
	// 8317E450: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8317E454: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8317E458: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8317E45C: 4198FFE4  blt cr6, 0x8317e440
	if ctx.cr[6].lt {
	pc = 0x8317E440; continue 'dispatch;
	}
	// 8317E460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317E464: 48029D54  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317E468 size=28
    let mut pc: u32 = 0x8317E468;
    'dispatch: loop {
        match pc {
            0x8317E468 => {
    //   block [0x8317E468..0x8317E484)
	// 8317E468: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317E46C: 814B17C8  lwz r10, 0x17c8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6088 as u32) ) } as u64;
	// 8317E470: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E474: 908B0D7C  stw r4, 0xd7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3452 as u32), ctx.r[4].u32 ) };
	// 8317E478: 90CB0D78  stw r6, 0xd78(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3448 as u32), ctx.r[6].u32 ) };
	// 8317E47C: 90AB0D74  stw r5, 0xd74(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3444 as u32), ctx.r[5].u32 ) };
	// 8317E480: 48011950  b 0x8318fdd0
	sub_8318FDD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317E488 size=8
    let mut pc: u32 = 0x8317E488;
    'dispatch: loop {
        match pc {
            0x8317E488 => {
    //   block [0x8317E488..0x8317E490)
	// 8317E488: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317E48C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317E490 size=36
    let mut pc: u32 = 0x8317E490;
    'dispatch: loop {
        match pc {
            0x8317E490 => {
    //   block [0x8317E490..0x8317E4B4)
	// 8317E490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317E494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317E498: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317E49C: 48011ACD  bl 0x8318ff68
	ctx.lr = 0x8317E4A0;
	sub_8318FF68(ctx, base);
	// 8317E4A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317E4A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317E4A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317E4AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317E4B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317E4B8 size=92
    let mut pc: u32 = 0x8317E4B8;
    'dispatch: loop {
        match pc {
            0x8317E4B8 => {
    //   block [0x8317E4B8..0x8317E514)
	// 8317E4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317E4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317E4C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317E4C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317E4C8: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 8317E4CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317E4D0: 4BFFAF41  bl 0x83179410
	ctx.lr = 0x8317E4D4;
	sub_83179410(ctx, base);
	// 8317E4D4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317E4D8: 419A0028  beq cr6, 0x8317e500
	if ctx.cr[6].eq {
	pc = 0x8317E500; continue 'dispatch;
	}
	// 8317E4DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317E4E0: 48010D99  bl 0x8318f278
	ctx.lr = 0x8317E4E4;
	sub_8318F278(ctx, base);
	// 8317E4E4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8317E4E8: 419A0018  beq cr6, 0x8317e500
	if ctx.cr[6].eq {
	pc = 0x8317E500; continue 'dispatch;
	}
	// 8317E4EC: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8317E4F0: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8317E4F4: 5565DFFE  rlwinm r5, r11, 0x1b, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317E4F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317E4FC: 4BFFFE05  bl 0x8317e300
	ctx.lr = 0x8317E500;
	sub_8317E300(ctx, base);
	// 8317E500: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317E504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317E508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317E50C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317E510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317E518 size=128
    let mut pc: u32 = 0x8317E518;
    'dispatch: loop {
        match pc {
            0x8317E518 => {
    //   block [0x8317E518..0x8317E598)
	// 8317E518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317E51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317E520: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317E524: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317E528: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317E52C: 814B0D80  lwz r10, 0xd80(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3456 as u32) ) } as u64;
	// 8317E530: 83EB17C8  lwz r31, 0x17c8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6088 as u32) ) } as u64;
	// 8317E534: 816B0D84  lwz r11, 0xd84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3460 as u32) ) } as u64;
	// 8317E538: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8317E53C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E540: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8317E544: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8317E548: 419A003C  beq cr6, 0x8317e584
	if ctx.cr[6].eq {
	pc = 0x8317E584; continue 'dispatch;
	}
	// 8317E54C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317E550: 419A0034  beq cr6, 0x8317e584
	if ctx.cr[6].eq {
	pc = 0x8317E584; continue 'dispatch;
	}
	// 8317E554: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317E558: 2F0B00C0  cmpwi cr6, r11, 0xc0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 192, &mut ctx.xer);
	// 8317E55C: 409A0028  bne cr6, 0x8317e584
	if !ctx.cr[6].eq {
	pc = 0x8317E584; continue 'dispatch;
	}
	// 8317E560: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317E564: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8317E568: 48014029  bl 0x83192590
	ctx.lr = 0x8317E56C;
	sub_83192590(ctx, base);
	// 8317E56C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317E570: 409A0014  bne cr6, 0x8317e584
	if !ctx.cr[6].eq {
	pc = 0x8317E584; continue 'dispatch;
	}
	// 8317E574: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8317E578: 394000C8  li r10, 0xc8
	ctx.r[10].s64 = 200;
	// 8317E57C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8317E580: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8317E584: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317E588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317E58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317E590: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317E594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317E598 size=8
    let mut pc: u32 = 0x8317E598;
    'dispatch: loop {
        match pc {
            0x8317E598 => {
    //   block [0x8317E598..0x8317E5A0)
	// 8317E598: 808317D4  lwz r4, 0x17d4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6100 as u32) ) } as u64;
	// 8317E59C: 4800FA3C  b 0x8318dfd8
	sub_8318DFD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317E5A0 size=120
    let mut pc: u32 = 0x8317E5A0;
    'dispatch: loop {
        match pc {
            0x8317E5A0 => {
    //   block [0x8317E5A0..0x8317E618)
	// 8317E5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317E5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317E5A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317E5AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317E5B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317E5B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317E5B8: 83DF0A68  lwz r30, 0xa68(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2664 as u32) ) } as u64;
	// 8317E5BC: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8317E5C0: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 8317E5C4: 419A000C  beq cr6, 0x8317e5d0
	if ctx.cr[6].eq {
	pc = 0x8317E5D0; continue 'dispatch;
	}
	// 8317E5C8: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8317E5CC: 40980008  bge cr6, 0x8317e5d4
	if !ctx.cr[6].lt {
	pc = 0x8317E5D4; continue 'dispatch;
	}
	// 8317E5D0: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 8317E5D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317E5D8: 48009809  bl 0x83187de0
	ctx.lr = 0x8317E5DC;
	sub_83187DE0(ctx, base);
	// 8317E5DC: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8317E5E0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8317E5E4: 409A000C  bne cr6, 0x8317e5f0
	if !ctx.cr[6].eq {
	pc = 0x8317E5F0; continue 'dispatch;
	}
	// 8317E5E8: 817F0968  lwz r11, 0x968(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2408 as u32) ) } as u64;
	// 8317E5EC: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8317E5F0: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8317E5F4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317E5F8: 40980008  bge cr6, 0x8317e600
	if !ctx.cr[6].lt {
	pc = 0x8317E600; continue 'dispatch;
	}
	// 8317E5FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317E600: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317E604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317E608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317E60C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317E610: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317E614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317E618 size=20
    let mut pc: u32 = 0x8317E618;
    'dispatch: loop {
        match pc {
            0x8317E618 => {
    //   block [0x8317E618..0x8317E62C)
	// 8317E618: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 8317E61C: 81430EC4  lwz r10, 0xec4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3780 as u32) ) } as u64;
	// 8317E620: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 8317E624: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317E628: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E62C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317E62C size=12
    let mut pc: u32 = 0x8317E62C;
    'dispatch: loop {
        match pc {
            0x8317E62C => {
    //   block [0x8317E62C..0x8317E638)
	// 8317E62C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8317E630: 91630EA0  stw r11, 0xea0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(3744 as u32), ctx.r[11].u32 ) };
	// 8317E634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317E638 size=124
    let mut pc: u32 = 0x8317E638;
    'dispatch: loop {
        match pc {
            0x8317E638 => {
    //   block [0x8317E638..0x8317E6B4)
	// 8317E638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317E63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317E640: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317E644: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317E648: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317E64C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317E650: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317E654: 4800977D  bl 0x83187dd0
	ctx.lr = 0x8317E658;
	sub_83187DD0(ctx, base);
	// 8317E658: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317E65C: 419A003C  beq cr6, 0x8317e698
	if ctx.cr[6].eq {
	pc = 0x8317E698; continue 'dispatch;
	}
	// 8317E660: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8317E664: 409A000C  bne cr6, 0x8317e670
	if !ctx.cr[6].eq {
	pc = 0x8317E670; continue 'dispatch;
	}
	// 8317E668: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317E66C: 48000030  b 0x8317e69c
	pc = 0x8317E69C; continue 'dispatch;
	// 8317E670: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8317E674: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317E678: 409A0020  bne cr6, 0x8317e698
	if !ctx.cr[6].eq {
	pc = 0x8317E698; continue 'dispatch;
	}
	// 8317E67C: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 8317E680: 409A0018  bne cr6, 0x8317e698
	if !ctx.cr[6].eq {
	pc = 0x8317E698; continue 'dispatch;
	}
	// 8317E684: 817F0968  lwz r11, 0x968(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2408 as u32) ) } as u64;
	// 8317E688: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317E68C: 815F096C  lwz r10, 0x96c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2412 as u32) ) } as u64;
	// 8317E690: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8317E694: 41990008  bgt cr6, 0x8317e69c
	if ctx.cr[6].gt {
	pc = 0x8317E69C; continue 'dispatch;
	}
	// 8317E698: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317E69C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317E6A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317E6A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317E6A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317E6AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317E6B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317E6B8 size=12
    let mut pc: u32 = 0x8317E6B8;
    'dispatch: loop {
        match pc {
            0x8317E6B8 => {
    //   block [0x8317E6B8..0x8317E6C4)
	// 8317E6B8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8317E6BC: 808317D4  lwz r4, 0x17d4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6100 as u32) ) } as u64;
	// 8317E6C0: 4800F900  b 0x8318dfc0
	sub_8318DFC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317E6C8 size=36
    let mut pc: u32 = 0x8317E6C8;
    'dispatch: loop {
        match pc {
            0x8317E6C8 => {
    //   block [0x8317E6C8..0x8317E6EC)
	// 8317E6C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E6CC: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8317E6D0: 4199001C  bgt cr6, 0x8317e6ec
	if ctx.cr[6].gt {
		sub_8317E6EC(ctx, base);
		return;
	}
	// 8317E6D4: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317E6D8: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8317E6DC: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8317E6E0: 4098000C  bge cr6, 0x8317e6ec
	if !ctx.cr[6].lt {
		sub_8317E6EC(ctx, base);
		return;
	}
	// 8317E6E4: 7C6B2050  subf r3, r11, r4
	ctx.r[3].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 8317E6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E6EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317E6EC size=44
    let mut pc: u32 = 0x8317E6EC;
    'dispatch: loop {
        match pc {
            0x8317E6EC => {
    //   block [0x8317E6EC..0x8317E718)
	// 8317E6EC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317E6F0: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8317E6F4: 41990024  bgt cr6, 0x8317e718
	if ctx.cr[6].gt {
		sub_8317E718(ctx, base);
		return;
	}
	// 8317E6F8: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317E6FC: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8317E700: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8317E704: 40980014  bge cr6, 0x8317e718
	if !ctx.cr[6].lt {
		sub_8317E718(ctx, base);
		return;
	}
	// 8317E708: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317E70C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8317E710: 7C6B2214  add r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 8317E714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317E718 size=8
    let mut pc: u32 = 0x8317E718;
    'dispatch: loop {
        match pc {
            0x8317E718 => {
    //   block [0x8317E718..0x8317E720)
	// 8317E718: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317E71C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317E720 size=96
    let mut pc: u32 = 0x8317E720;
    'dispatch: loop {
        match pc {
            0x8317E720 => {
    //   block [0x8317E720..0x8317E780)
	// 8317E720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317E724: 48029A49  bl 0x831a816c
	ctx.lr = 0x8317E728;
	sub_831A8130(ctx, base);
	// 8317E728: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317E72C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317E730: 83DF17D0  lwz r30, 0x17d0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6096 as u32) ) } as u64;
	// 8317E734: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8317E738: 4800F6C9  bl 0x8318de00
	ctx.lr = 0x8317E73C;
	sub_8318DE00(ctx, base);
	// 8317E73C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8317E740: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8317E744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317E748: 48010049  bl 0x8318e790
	ctx.lr = 0x8317E74C;
	sub_8318E790(ctx, base);
	// 8317E74C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8317E750: 7D43E850  subf r10, r3, r29
	ctx.r[10].s64 = ctx.r[29].s64 - ctx.r[3].s64;
	// 8317E754: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317E758: 4098001C  bge cr6, 0x8317e774
	if !ctx.cr[6].lt {
	pc = 0x8317E774; continue 'dispatch;
	}
	// 8317E75C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317E760: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317E764: 60840F1C  ori r4, r4, 0xf1c
	ctx.r[4].u64 = ctx.r[4].u64 | 3868;
	// 8317E768: 48008D91  bl 0x831874f8
	ctx.lr = 0x8317E76C;
	sub_831874F8(ctx, base);
	// 8317E76C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317E770: 48029A4C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317E774: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317E778: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317E77C: 48029A40  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317E780 size=324
    let mut pc: u32 = 0x8317E780;
    'dispatch: loop {
        match pc {
            0x8317E780 => {
    //   block [0x8317E780..0x8317E8C4)
	// 8317E780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317E784: 480299DD  bl 0x831a8160
	ctx.lr = 0x8317E788;
	sub_831A8130(ctx, base);
	// 8317E788: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317E78C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8317E790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317E794: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317E798: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8317E79C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8317E7A0: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317E7A4: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317E7A8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E7AC: 4801402D  bl 0x831927d8
	ctx.lr = 0x8317E7B0;
	sub_831927D8(ctx, base);
	// 8317E7B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317E7B4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8317E7B8: 419A001C  beq cr6, 0x8317e7d4
	if ctx.cr[6].eq {
	pc = 0x8317E7D4; continue 'dispatch;
	}
	// 8317E7BC: 48013F6D  bl 0x83192728
	ctx.lr = 0x8317E7C0;
	sub_83192728(ctx, base);
	// 8317E7C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317E7C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317E7C8: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317E7CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8317E7D0: 480299E0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 8317E7D4: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317E7D8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8317E7DC: 419A00DC  beq cr6, 0x8317e8b8
	if ctx.cr[6].eq {
	pc = 0x8317E8B8; continue 'dispatch;
	}
	// 8317E7E0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317E7E4: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 8317E7E8: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 8317E7EC: 41980008  blt cr6, 0x8317e7f4
	if ctx.cr[6].lt {
	pc = 0x8317E7F4; continue 'dispatch;
	}
	// 8317E7F0: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	// 8317E7F4: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 8317E7F8: 41980008  blt cr6, 0x8317e800
	if ctx.cr[6].lt {
	pc = 0x8317E800; continue 'dispatch;
	}
	// 8317E7FC: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 8317E800: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E804: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8317E808: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8317E80C: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 8317E810: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317E814: 48029CFD  bl 0x831a8510
	ctx.lr = 0x8317E818;
	sub_831A8510(ctx, base);
	// 8317E818: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8317E81C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8317E820: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317E824: 7C7D5A14  add r3, r29, r11
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 8317E828: 48029CE9  bl 0x831a8510
	ctx.lr = 0x8317E82C;
	sub_831A8510(ctx, base);
	// 8317E82C: 7D7EEA14  add r11, r30, r29
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 8317E830: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8317E834: 3B8BFFFD  addi r28, r11, -3
	ctx.r[28].s64 = ctx.r[11].s64 + -3;
	// 8317E838: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8317E83C: 40990028  ble cr6, 0x8317e864
	if !ctx.cr[6].gt {
	pc = 0x8317E864; continue 'dispatch;
	}
	// 8317E840: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8317E844: 7C7E5A14  add r3, r30, r11
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 8317E848: 48013EE1  bl 0x83192728
	ctx.lr = 0x8317E84C;
	sub_83192728(ctx, base);
	// 8317E84C: 7C6BD838  and r11, r3, r27
	ctx.r[11].u64 = ctx.r[3].u64 & ctx.r[27].u64;
	// 8317E850: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317E854: 409A0044  bne cr6, 0x8317e898
	if !ctx.cr[6].eq {
	pc = 0x8317E898; continue 'dispatch;
	}
	// 8317E858: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8317E85C: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8317E860: 4198FFE0  blt cr6, 0x8317e840
	if ctx.cr[6].lt {
	pc = 0x8317E840; continue 'dispatch;
	}
	// 8317E864: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8317E868: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317E86C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317E870: 48013F69  bl 0x831927d8
	ctx.lr = 0x8317E874;
	sub_831927D8(ctx, base);
	// 8317E874: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317E878: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8317E87C: 419A003C  beq cr6, 0x8317e8b8
	if ctx.cr[6].eq {
	pc = 0x8317E8B8; continue 'dispatch;
	}
	// 8317E880: 48013EA9  bl 0x83192728
	ctx.lr = 0x8317E884;
	sub_83192728(ctx, base);
	// 8317E884: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317E888: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317E88C: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317E890: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8317E894: 4802991C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 8317E898: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8317E89C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E8A0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317E8A4: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 8317E8A8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317E8AC: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8317E8B0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8317E8B4: 480298FC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 8317E8B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317E8BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8317E8C0: 480298F0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317E8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317E8C8 size=336
    let mut pc: u32 = 0x8317E8C8;
    'dispatch: loop {
        match pc {
            0x8317E8C8 => {
    //   block [0x8317E8C8..0x8317EA18)
	// 8317E8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317E8CC: 48029895  bl 0x831a8160
	ctx.lr = 0x8317E8D0;
	sub_831A8130(ctx, base);
	// 8317E8D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317E8D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317E8D8: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8317E8DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317E8E0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8317E8E4: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317E8E8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317E8EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317E8F0: 419A00C4  beq cr6, 0x8317e9b4
	if ctx.cr[6].eq {
	pc = 0x8317E9B4; continue 'dispatch;
	}
	// 8317E8F4: 5564003E  slwi r4, r11, 0
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8317E8F8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317E8FC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8317E900: 7C6B2214  add r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 8317E904: 48013E65  bl 0x83192768
	ctx.lr = 0x8317E908;
	sub_83192768(ctx, base);
	// 8317E908: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317E90C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8317E910: 419A001C  beq cr6, 0x8317e92c
	if ctx.cr[6].eq {
	pc = 0x8317E92C; continue 'dispatch;
	}
	// 8317E914: 48013E15  bl 0x83192728
	ctx.lr = 0x8317E918;
	sub_83192728(ctx, base);
	// 8317E918: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317E91C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317E920: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317E924: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8317E928: 48029888  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 8317E92C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317E930: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 8317E934: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 8317E938: 41980008  blt cr6, 0x8317e940
	if ctx.cr[6].lt {
	pc = 0x8317E940; continue 'dispatch;
	}
	// 8317E93C: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	// 8317E940: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317E944: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 8317E948: 41980008  blt cr6, 0x8317e950
	if ctx.cr[6].lt {
	pc = 0x8317E950; continue 'dispatch;
	}
	// 8317E94C: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 8317E950: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E954: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8317E958: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8317E95C: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 8317E960: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317E964: 48029BAD  bl 0x831a8510
	ctx.lr = 0x8317E968;
	sub_831A8510(ctx, base);
	// 8317E968: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8317E96C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8317E970: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317E974: 7C7D5A14  add r3, r29, r11
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 8317E978: 48029B99  bl 0x831a8510
	ctx.lr = 0x8317E97C;
	sub_831A8510(ctx, base);
	// 8317E97C: 7D7EEA14  add r11, r30, r29
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 8317E980: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8317E984: 3B8BFFFD  addi r28, r11, -3
	ctx.r[28].s64 = ctx.r[11].s64 + -3;
	// 8317E988: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8317E98C: 40990028  ble cr6, 0x8317e9b4
	if !ctx.cr[6].gt {
	pc = 0x8317E9B4; continue 'dispatch;
	}
	// 8317E990: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8317E994: 7C7E5A14  add r3, r30, r11
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 8317E998: 48013D91  bl 0x83192728
	ctx.lr = 0x8317E99C;
	sub_83192728(ctx, base);
	// 8317E99C: 7C6BD838  and r11, r3, r27
	ctx.r[11].u64 = ctx.r[3].u64 & ctx.r[27].u64;
	// 8317E9A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317E9A4: 409A0048  bne cr6, 0x8317e9ec
	if !ctx.cr[6].eq {
	pc = 0x8317E9EC; continue 'dispatch;
	}
	// 8317E9A8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8317E9AC: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8317E9B0: 4198FFE0  blt cr6, 0x8317e990
	if ctx.cr[6].lt {
	pc = 0x8317E990; continue 'dispatch;
	}
	// 8317E9B4: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317E9B8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8317E9BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E9C0: 7C6B2214  add r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 8317E9C4: 48013DA5  bl 0x83192768
	ctx.lr = 0x8317E9C8;
	sub_83192768(ctx, base);
	// 8317E9C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317E9CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8317E9D0: 419A003C  beq cr6, 0x8317ea0c
	if ctx.cr[6].eq {
	pc = 0x8317EA0C; continue 'dispatch;
	}
	// 8317E9D4: 48013D55  bl 0x83192728
	ctx.lr = 0x8317E9D8;
	sub_83192728(ctx, base);
	// 8317E9D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317E9DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317E9E0: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317E9E4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8317E9E8: 480297C8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 8317E9EC: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8317E9F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317E9F4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317E9F8: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 8317E9FC: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8317EA00: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317EA04: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8317EA08: 480297A8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 8317EA0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317EA10: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8317EA14: 4802979C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317EA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317EA18 size=88
    let mut pc: u32 = 0x8317EA18;
    'dispatch: loop {
        match pc {
            0x8317EA18 => {
    //   block [0x8317EA18..0x8317EA70)
	// 8317EA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317EA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317EA20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317EA24: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317EA28: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317EA2C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8317EA30: 808317D0  lwz r4, 0x17d0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6096 as u32) ) } as u64;
	// 8317EA34: 4800FC4D  bl 0x8318e680
	ctx.lr = 0x8317EA38;
	sub_8318E680(ctx, base);
	// 8317EA38: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317EA3C: 419A0010  beq cr6, 0x8317ea4c
	if ctx.cr[6].eq {
	pc = 0x8317EA4C; continue 'dispatch;
	}
	// 8317EA40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317EA44: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8317EA48: 48000010  b 0x8317ea58
	pc = 0x8317EA58; continue 'dispatch;
	// 8317EA4C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317EA50: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317EA54: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8317EA58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317EA5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317EA60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317EA64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317EA68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317EA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317EA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317EA70 size=88
    let mut pc: u32 = 0x8317EA70;
    'dispatch: loop {
        match pc {
            0x8317EA70 => {
    //   block [0x8317EA70..0x8317EAC8)
	// 8317EA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317EA74: 480296F9  bl 0x831a816c
	ctx.lr = 0x8317EA78;
	sub_831A8130(ctx, base);
	// 8317EA78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317EA7C: 3BE317C0  addi r31, r3, 0x17c0
	ctx.r[31].s64 = ctx.r[3].s64 + 6080;
	// 8317EA80: 3BC30D88  addi r30, r3, 0xd88
	ctx.r[30].s64 = ctx.r[3].s64 + 3464;
	// 8317EA84: 3BBE0094  addi r29, r30, 0x94
	ctx.r[29].s64 = ctx.r[30].s64 + 148;
	// 8317EA88: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8317EA8C: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8317EA90: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317EA94: 40980010  bge cr6, 0x8317eaa4
	if !ctx.cr[6].lt {
	pc = 0x8317EAA4; continue 'dispatch;
	}
	// 8317EA98: 4800F379  bl 0x8318de10
	ctx.lr = 0x8317EA9C;
	sub_8318DE10(ctx, base);
	// 8317EA9C: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 8317EAA0: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8317EAA4: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317EAA8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317EAAC: 40980014  bge cr6, 0x8317eac0
	if !ctx.cr[6].lt {
	pc = 0x8317EAC0; continue 'dispatch;
	}
	// 8317EAB0: 389E0068  addi r4, r30, 0x68
	ctx.r[4].s64 = ctx.r[30].s64 + 104;
	// 8317EAB4: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 8317EAB8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317EABC: 48029A55  bl 0x831a8510
	ctx.lr = 0x8317EAC0;
	sub_831A8510(ctx, base);
	// 8317EAC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317EAC4: 480296F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317EAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317EAC8 size=180
    let mut pc: u32 = 0x8317EAC8;
    'dispatch: loop {
        match pc {
            0x8317EAC8 => {
    //   block [0x8317EAC8..0x8317EB7C)
	// 8317EAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317EACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317EAD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317EAD4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317EAD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317EADC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317EAE0: 396BB190  addi r11, r11, -0x4e70
	ctx.r[11].s64 = ctx.r[11].s64 + -20080;
	// 8317EAE4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317EAE8: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317EAEC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8317EAF0: 419A005C  beq cr6, 0x8317eb4c
	if ctx.cr[6].eq {
	pc = 0x8317EB4C; continue 'dispatch;
	}
	// 8317EAF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317EAF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8317EAFC: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 8317EB00: 616BAC44  ori r11, r11, 0xac44
	ctx.r[11].u64 = ctx.r[11].u64 | 44100;
	// 8317EB04: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8317EB08: 817F0F7C  lwz r11, 0xf7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3964 as u32) ) } as u64;
	// 8317EB0C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317EB10: 809F0DEC  lwz r4, 0xdec(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3564 as u32) ) } as u64;
	// 8317EB14: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317EB18: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317EB1C: 907F0F7C  stw r3, 0xf7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3964 as u32), ctx.r[3].u32 ) };
	// 8317EB20: 4800CCB9  bl 0x8318b7d8
	ctx.lr = 0x8317EB24;
	sub_8318B7D8(ctx, base);
	// 8317EB24: 817F0EEC  lwz r11, 0xeec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3820 as u32) ) } as u64;
	// 8317EB28: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 8317EB2C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317EB30: 40980008  bge cr6, 0x8317eb38
	if !ctx.cr[6].lt {
	pc = 0x8317EB38; continue 'dispatch;
	}
	// 8317EB34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317EB38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317EB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317EB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317EB44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317EB48: 4E800020  blr
	return;
	// 8317EB4C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317EB50: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8317EB54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317EB58: 4800D099  bl 0x8318bbf0
	ctx.lr = 0x8317EB5C;
	sub_8318BBF0(ctx, base);
	// 8317EB5C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317EB60: 409AFFA8  bne cr6, 0x8317eb08
	if !ctx.cr[6].eq {
	pc = 0x8317EB08; continue 'dispatch;
	}
	// 8317EB64: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8317EB68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317EB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317EB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317EB74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317EB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317EB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317EB80 size=80
    let mut pc: u32 = 0x8317EB80;
    'dispatch: loop {
        match pc {
            0x8317EB80 => {
    //   block [0x8317EB80..0x8317EBD0)
	// 8317EB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317EB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317EB88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317EB8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317EB90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317EB94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317EB98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317EB9C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8317EBA0: 809F17D0  lwz r4, 0x17d0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6096 as u32) ) } as u64;
	// 8317EBA4: 4800F32D  bl 0x8318ded0
	ctx.lr = 0x8317EBA8;
	sub_8318DED0(ctx, base);
	// 8317EBA8: E95F09A8  ld r10, 0x9a8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2472 as u32) ) };
	// 8317EBAC: 7FCB07B4  extsw r11, r30
	ctx.r[11].s64 = ctx.r[30].s32 as i64;
	// 8317EBB0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317EBB4: F97F09A8  std r11, 0x9a8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2472 as u32), ctx.r[11].u64 ) };
	// 8317EBB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317EBBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317EBC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317EBC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317EBC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317EBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317EBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317EBD0 size=8
    let mut pc: u32 = 0x8317EBD0;
    'dispatch: loop {
        match pc {
            0x8317EBD0 => {
    //   block [0x8317EBD0..0x8317EBD8)
	// 8317EBD0: 808317D0  lwz r4, 0x17d0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6096 as u32) ) } as u64;
	// 8317EBD4: 4800F404  b 0x8318dfd8
	sub_8318DFD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317EBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317EBD8 size=36
    let mut pc: u32 = 0x8317EBD8;
    'dispatch: loop {
        match pc {
            0x8317EBD8 => {
    //   block [0x8317EBD8..0x8317EBFC)
	// 8317EBD8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8317EBDC: 2F0BFFFD  cmpwi cr6, r11, -3
	ctx.cr[6].compare_i32(ctx.r[11].s32, -3, &mut ctx.xer);
	// 8317EBE0: 419A0034  beq cr6, 0x8317ec14
	if ctx.cr[6].eq {
		sub_8317EC14(ctx, base);
		return;
	}
	// 8317EBE4: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 8317EBE8: 419A001C  beq cr6, 0x8317ec04
	if ctx.cr[6].eq {
		sub_8317EC04(ctx, base);
		return;
	}
	// 8317EBEC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317EBF0: 419A000C  beq cr6, 0x8317ebfc
	if ctx.cr[6].eq {
		sub_8317EBFC(ctx, base);
		return;
	}
	// 8317EBF4: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 8317EBF8: 48008900  b 0x831874f8
	sub_831874F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317EBFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317EBFC size=8
    let mut pc: u32 = 0x8317EBFC;
    'dispatch: loop {
        match pc {
            0x8317EBFC => {
    //   block [0x8317EBFC..0x8317EC04)
	// 8317EBFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317EC00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317EC04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317EC04 size=16
    let mut pc: u32 = 0x8317EC04;
    'dispatch: loop {
        match pc {
            0x8317EC04 => {
    //   block [0x8317EC04..0x8317EC14)
	// 8317EC04: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8317EC08: 4199FFF4  bgt cr6, 0x8317ebfc
	if ctx.cr[6].gt {
		sub_8317EBFC(ctx, base);
		return;
	}
	// 8317EC0C: 3880FFFE  li r4, -2
	ctx.r[4].s64 = -2;
	// 8317EC10: 480088E8  b 0x831874f8
	sub_831874F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317EC14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317EC14 size=16
    let mut pc: u32 = 0x8317EC14;
    'dispatch: loop {
        match pc {
            0x8317EC14 => {
    //   block [0x8317EC14..0x8317EC24)
	// 8317EC14: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8317EC18: 4199FFE4  bgt cr6, 0x8317ebfc
	if ctx.cr[6].gt {
		sub_8317EBFC(ctx, base);
		return;
	}
	// 8317EC1C: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 8317EC20: 480088D8  b 0x831874f8
	sub_831874F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317EC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317EC28 size=36
    let mut pc: u32 = 0x8317EC28;
    'dispatch: loop {
        match pc {
            0x8317EC28 => {
    //   block [0x8317EC28..0x8317EC4C)
	// 8317EC28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317EC2C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8317EC30: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317EC34: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8317EC38: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8317EC3C: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 8317EC40: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8317EC44: 9143001C  stw r10, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8317EC48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317EC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317EC50 size=28
    let mut pc: u32 = 0x8317EC50;
    'dispatch: loop {
        match pc {
            0x8317EC50 => {
    //   block [0x8317EC50..0x8317EC6C)
	// 8317EC50: 3D40055D  lis r10, 0x55d
	ctx.r[10].s64 = 89980928;
	// 8317EC54: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 8317EC58: 614A4A80  ori r10, r10, 0x4a80
	ctx.r[10].u64 = ctx.r[10].u64 | 19072;
	// 8317EC5C: 7C8907B4  extsw r9, r4
	ctx.r[9].s64 = ctx.r[4].s32 as i64;
	// 8317EC60: 7D6B51D2  mulld r11, r11, r10
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[10].s64;
	// 8317EC64: 7C6B4BD2  divd r3, r11, r9
	ctx.r[3].s64 = ctx.r[11].s64 / ctx.r[9].s64;
	// 8317EC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317EC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317EC70 size=136
    let mut pc: u32 = 0x8317EC70;
    'dispatch: loop {
        match pc {
            0x8317EC70 => {
    //   block [0x8317EC70..0x8317ECF8)
	// 8317EC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317EC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317EC78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317EC7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317EC80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317EC84: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8317EC88: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317EC8C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317EC90: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8317EC94: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8317EC98: 3BCB0D88  addi r30, r11, 0xd88
	ctx.r[30].s64 = ctx.r[11].s64 + 3464;
	// 8317EC9C: 4BFFCD25  bl 0x8317b9c0
	ctx.lr = 0x8317ECA0;
	sub_8317B9C0(ctx, base);
	// 8317ECA0: 817E0060  lwz r11, 0x60(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 8317ECA4: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317ECA8: 389F0018  addi r4, r31, 0x18
	ctx.r[4].s64 = ctx.r[31].s64 + 24;
	// 8317ECAC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317ECB0: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 8317ECB4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8317ECB8: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 8317ECBC: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8317ECC0: 913F0018  stw r9, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 8317ECC4: 817E008C  lwz r11, 0x8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 8317ECC8: 815F003C  lwz r10, 0x3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8317ECCC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8317ECD0: 41990010  bgt cr6, 0x8317ece0
	if ctx.cr[6].gt {
	pc = 0x8317ECE0; continue 'dispatch;
	}
	// 8317ECD4: 387E0068  addi r3, r30, 0x68
	ctx.r[3].s64 = ctx.r[30].s64 + 104;
	// 8317ECD8: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 8317ECDC: 48029835  bl 0x831a8510
	ctx.lr = 0x8317ECE0;
	sub_831A8510(ctx, base);
	// 8317ECE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317ECE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317ECE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317ECEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317ECF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317ECF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317ECF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317ECF8 size=192
    let mut pc: u32 = 0x8317ECF8;
    'dispatch: loop {
        match pc {
            0x8317ECF8 => {
    //   block [0x8317ECF8..0x8317EDB8)
	// 8317ECF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317ECFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317ED00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317ED04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317ED08: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317ED0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317ED10: 397F0D88  addi r11, r31, 0xd88
	ctx.r[11].s64 = ctx.r[31].s64 + 3464;
	// 8317ED14: 3BCB0068  addi r30, r11, 0x68
	ctx.r[30].s64 = ctx.r[11].s64 + 104;
	// 8317ED18: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317ED1C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8317ED20: 419A007C  beq cr6, 0x8317ed9c
	if ctx.cr[6].eq {
	pc = 0x8317ED9C; continue 'dispatch;
	}
	// 8317ED24: 396B001C  addi r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 + 28;
	// 8317ED28: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8317ED2C: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8317ED30: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8317ED34: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317ED38: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8317ED3C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8317ED40: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8317ED44: 4200FFF0  bdnz 0x8317ed34
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8317ED34; continue 'dispatch;
	}
	// 8317ED48: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317ED4C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8317ED50: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8317ED54: 4BFFCC6D  bl 0x8317b9c0
	ctx.lr = 0x8317ED58;
	sub_8317B9C0(ctx, base);
	// 8317ED58: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317ED5C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8317ED60: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8317ED64: 4BFFCC5D  bl 0x8317b9c0
	ctx.lr = 0x8317ED68;
	sub_8317B9C0(ctx, base);
	// 8317ED68: 38800035  li r4, 0x35
	ctx.r[4].s64 = 53;
	// 8317ED6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317ED70: 4BFFA6A1  bl 0x83179410
	ctx.lr = 0x8317ED74;
	sub_83179410(ctx, base);
	// 8317ED74: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317ED78: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317ED7C: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8317ED80: 7D2349D6  mullw r9, r3, r9
	ctx.r[9].s64 = (ctx.r[3].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8317ED84: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8317ED88: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317ED8C: 40990014  ble cr6, 0x8317eda0
	if !ctx.cr[6].gt {
	pc = 0x8317EDA0; continue 'dispatch;
	}
	// 8317ED90: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 8317ED94: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8317ED98: 40980008  bge cr6, 0x8317eda0
	if !ctx.cr[6].lt {
	pc = 0x8317EDA0; continue 'dispatch;
	}
	// 8317ED9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317EDA0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8317EDA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317EDA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317EDAC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317EDB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317EDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317EDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317EDB8 size=480
    let mut pc: u32 = 0x8317EDB8;
    'dispatch: loop {
        match pc {
            0x8317EDB8 => {
    //   block [0x8317EDB8..0x8317EF98)
	// 8317EDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317EDBC: 480293A1  bl 0x831a815c
	ctx.lr = 0x8317EDC0;
	sub_831A8130(ctx, base);
	// 8317EDC0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317EDC4: 3D20821A  lis r9, -0x7de6
	ctx.r[9].s64 = -2112225280;
	// 8317EDC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317EDCC: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 8317EDD0: 3B69AEC8  addi r27, r9, -0x5138
	ctx.r[27].s64 = ctx.r[9].s64 + -20792;
	// 8317EDD4: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317EDD8: 394AACF0  addi r10, r10, -0x5310
	ctx.r[10].s64 = ctx.r[10].s64 + -21264;
	// 8317EDDC: 393BFFBC  addi r9, r27, -0x44
	ctx.r[9].s64 = ctx.r[27].s64 + -68;
	// 8317EDE0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8317EDE4: 3CA0055D  lis r5, 0x55d
	ctx.r[5].s64 = 89980928;
	// 8317EDE8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8317EDEC: 7F4B502E  lwzx r26, r11, r10
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8317EDF0: 60A54A80  ori r5, r5, 0x4a80
	ctx.r[5].u64 = ctx.r[5].u64 | 19072;
	// 8317EDF4: 7FEB482E  lwzx r31, r11, r9
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8317EDF8: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 8317EDFC: 574B083C  slwi r11, r26, 1
	ctx.r[11].u32 = ctx.r[26].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317EE00: 7D6407B4  extsw r4, r11
	ctx.r[4].s64 = ctx.r[11].s32 as i64;
	// 8317EE04: 48013A3D  bl 0x83192840
	ctx.lr = 0x8317EE08;
	sub_83192840(ctx, base);
	// 8317EE08: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 8317EE0C: 93D90000  stw r30, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8317EE10: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 8317EE14: 93990004  stw r28, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8317EE18: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8317EE1C: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 8317EE20: B159001E  sth r10, 0x1e(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(30 as u32), ctx.r[10].u16 ) };
	// 8317EE24: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317EE28: 41990008  bgt cr6, 0x8317ee30
	if ctx.cr[6].gt {
	pc = 0x8317EE30; continue 'dispatch;
	}
	// 8317EE2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317EE30: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8317EE34: 419A00EC  beq cr6, 0x8317ef20
	if ctx.cr[6].eq {
	pc = 0x8317EF20; continue 'dispatch;
	}
	// 8317EE38: 2F1A7512  cmpwi cr6, r26, 0x7512
	ctx.cr[6].compare_i32(ctx.r[26].s32, 29970, &mut ctx.xer);
	// 8317EE3C: 419A006C  beq cr6, 0x8317eea8
	if ctx.cr[6].eq {
	pc = 0x8317EEA8; continue 'dispatch;
	}
	// 8317EE40: 2B1AEA24  cmplwi cr6, r26, 0xea24
	ctx.cr[6].compare_u32(ctx.r[26].u32, 59940 as u32, &mut ctx.xer);
	// 8317EE44: 409A00DC  bne cr6, 0x8317ef20
	if !ctx.cr[6].eq {
	pc = 0x8317EF20; continue 'dispatch;
	}
	// 8317EE48: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8317EE4C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317EE50: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317EE54: 7CCB4BD6  divw r6, r11, r9
	ctx.r[6].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8317EE58: 80EA0008  lwz r7, 8(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317EE5C: 7CAB4BD6  divw r5, r11, r9
	ctx.r[5].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8317EE60: 7D2649D6  mullw r9, r6, r9
	ctx.r[9].s64 = (ctx.r[6].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8317EE64: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8317EE68: 7D2B43D6  divw r9, r11, r8
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[8].s32;
	// 8317EE6C: 7C8B43D6  divw r4, r11, r8
	ctx.r[4].s32 = ctx.r[11].s32 / ctx.r[8].s32;
	// 8317EE70: 7D2941D6  mullw r9, r9, r8
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[8].s32 as i64);
	// 8317EE74: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8317EE78: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 8317EE7C: 40980034  bge cr6, 0x8317eeb0
	if !ctx.cr[6].lt {
	pc = 0x8317EEB0; continue 'dispatch;
	}
	// 8317EE80: 810A0014  lwz r8, 0x14(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8317EE84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8317EE88: 7CEB43D6  divw r7, r11, r8
	ctx.r[7].s32 = ctx.r[11].s32 / ctx.r[8].s32;
	// 8317EE8C: 7D2B43D6  divw r9, r11, r8
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[8].s32;
	// 8317EE90: 7D0741D6  mullw r8, r7, r8
	ctx.r[8].s64 = (ctx.r[7].s32 as i64) * (ctx.r[8].s32 as i64);
	// 8317EE94: 7CE85850  subf r7, r8, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 8317EE98: 816A0018  lwz r11, 0x18(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 8317EE9C: 7D6B21D6  mullw r11, r11, r4
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[4].s32 as i64);
	// 8317EEA0: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 8317EEA4: 480000DC  b 0x8317ef80
	pc = 0x8317EF80; continue 'dispatch;
	// 8317EEA8: 395BFFE0  addi r10, r27, -0x20
	ctx.r[10].s64 = ctx.r[27].s64 + -32;
	// 8317EEAC: 4BFFFFA0  b 0x8317ee4c
	pc = 0x8317EE4C; continue 'dispatch;
	// 8317EEB0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317EEB4: 7D675850  subf r11, r7, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 8317EEB8: 80EA0010  lwz r7, 0x10(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8317EEBC: 7D0B4BD6  divw r8, r11, r9
	ctx.r[8].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8317EEC0: 7CCB4BD6  divw r6, r11, r9
	ctx.r[6].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8317EEC4: 7D2849D6  mullw r9, r8, r9
	ctx.r[9].s64 = (ctx.r[8].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8317EEC8: 7D095850  subf r8, r9, r11
	ctx.r[8].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8317EECC: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 8317EED0: 7F083800  cmpw cr6, r8, r7
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[7].s32, &mut ctx.xer);
	// 8317EED4: 40980020  bge cr6, 0x8317eef4
	if !ctx.cr[6].lt {
	pc = 0x8317EEF4; continue 'dispatch;
	}
	// 8317EED8: 816A001C  lwz r11, 0x1c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 8317EEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8317EEE0: 7CEB4214  add r7, r11, r8
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8317EEE4: 816A0018  lwz r11, 0x18(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 8317EEE8: 7D6B21D6  mullw r11, r11, r4
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[4].s32 as i64);
	// 8317EEEC: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 8317EEF0: 48000090  b 0x8317ef80
	pc = 0x8317EF80; continue 'dispatch;
	// 8317EEF4: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8317EEF8: 7D074050  subf r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 8317EEFC: 7CE85BD6  divw r7, r8, r11
	ctx.r[7].s32 = ctx.r[8].s32 / ctx.r[11].s32;
	// 8317EF00: 7D285BD6  divw r9, r8, r11
	ctx.r[9].s32 = ctx.r[8].s32 / ctx.r[11].s32;
	// 8317EF04: 7D6759D6  mullw r11, r7, r11
	ctx.r[11].s64 = (ctx.r[7].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8317EF08: 7CEB4050  subf r7, r11, r8
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 8317EF0C: 816A0018  lwz r11, 0x18(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 8317EF10: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8317EF14: 7D6B21D6  mullw r11, r11, r4
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[4].s32 as i64);
	// 8317EF18: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 8317EF1C: 48000064  b 0x8317ef80
	pc = 0x8317EF80; continue 'dispatch;
	// 8317EF20: 3D208888  lis r9, -0x7778
	ctx.r[9].s64 = -2004353024;
	// 8317EF24: 7D4BFBD6  divw r10, r11, r31
	ctx.r[10].s32 = ctx.r[11].s32 / ctx.r[31].s32;
	// 8317EF28: 61298889  ori r9, r9, 0x8889
	ctx.r[9].u64 = ctx.r[9].u64 | 34953;
	// 8317EF2C: 7CEBFBD6  divw r7, r11, r31
	ctx.r[7].s32 = ctx.r[11].s32 / ctx.r[31].s32;
	// 8317EF30: 7D264B78  mr r6, r9
	ctx.r[6].u64 = ctx.r[9].u64;
	// 8317EF34: 7D2A4896  mulhw r9, r10, r9
	ctx.r[9].s64 = ((ctx.r[10].s32 as i64 * ctx.r[9].s32 as i64) >> 32);
	// 8317EF38: 7CE7F9D6  mullw r7, r7, r31
	ctx.r[7].s64 = (ctx.r[7].s32 as i64) * (ctx.r[31].s32 as i64);
	// 8317EF3C: 3900003C  li r8, 0x3c
	ctx.r[8].s64 = 60;
	// 8317EF40: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 8317EF44: 7CE75850  subf r7, r7, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 8317EF48: 7D6A43D6  divw r11, r10, r8
	ctx.r[11].s32 = ctx.r[10].s32 / ctx.r[8].s32;
	// 8317EF4C: 7D292E70  srawi r9, r9, 5
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 5) as i64;
	// 8317EF50: 7CAB43D6  divw r5, r11, r8
	ctx.r[5].s32 = ctx.r[11].s32 / ctx.r[8].s32;
	// 8317EF54: 7D0B3096  mulhw r8, r11, r6
	ctx.r[8].s64 = ((ctx.r[11].s32 as i64 * ctx.r[6].s32 as i64) >> 32);
	// 8317EF58: 55260FFE  srwi r6, r9, 0x1f
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shr(31);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8317EF5C: 7D085A14  add r8, r8, r11
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8317EF60: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 8317EF64: 7D082E70  srawi r8, r8, 5
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 5) as i64;
	// 8317EF68: 1D29003C  mulli r9, r9, 0x3c
	ctx.r[9].s64 = ctx.r[9].s64 * 60;
	// 8317EF6C: 7D295050  subf r9, r9, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 8317EF70: 550A0FFE  srwi r10, r8, 0x1f
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8317EF74: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 8317EF78: 1D4A003C  mulli r10, r10, 0x3c
	ctx.r[10].s64 = ctx.r[10].s64 * 60;
	// 8317EF7C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8317EF80: 90B90008  stw r5, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 8317EF84: 9179000C  stw r11, 0xc(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8317EF88: 91390010  stw r9, 0x10(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 8317EF8C: 90F90014  stw r7, 0x14(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 8317EF90: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8317EF94: 48029218  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317EF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8317EF98 size=336
    let mut pc: u32 = 0x8317EF98;
    'dispatch: loop {
        match pc {
            0x8317EF98 => {
    //   block [0x8317EF98..0x8317F0E8)
	// 8317EF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317EF9C: 480291C5  bl 0x831a8160
	ctx.lr = 0x8317EFA0;
	sub_831A8130(ctx, base);
	// 8317EFA0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317EFA4: A143001E  lhz r10, 0x1e(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(30 as u32) ) } as u64;
	// 8317EFA8: A103001C  lhz r8, 0x1c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8317EFAC: 3BC0003C  li r30, 0x3c
	ctx.r[30].s64 = 60;
	// 8317EFB0: 392BAE84  addi r9, r11, -0x517c
	ctx.r[9].s64 = ctx.r[11].s64 + -20860;
	// 8317EFB4: 83E30018  lwz r31, 0x18(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8317EFB8: 7D4B0734  extsh r11, r10
	ctx.r[11].s64 = ctx.r[10].s16 as i64;
	// 8317EFBC: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317EFC0: 7D0A0734  extsh r10, r8
	ctx.r[10].s64 = ctx.r[8].s16 as i64;
	// 8317EFC4: 81030014  lwz r8, 0x14(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8317EFC8: 57BA103A  slwi r26, r29, 2
	ctx.r[26].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 8317EFCC: 80A30010  lwz r5, 0x10(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8317EFD0: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317EFD4: 80C3000C  lwz r6, 0xc(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317EFD8: 3D608888  lis r11, -0x7778
	ctx.r[11].s64 = -2004353024;
	// 8317EFDC: 80E30008  lwz r7, 8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317EFE0: 616B8889  ori r11, r11, 0x8889
	ctx.r[11].u64 = ctx.r[11].u64 | 34953;
	// 8317EFE4: 7D3A482E  lwzx r9, r26, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8317EFE8: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 8317EFEC: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 8317EFF0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317EFF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317EFF8: 7D4B0E70  srawi r11, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 8317EFFC: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8317F000: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8317F004: 7D5F0E70  srawi r31, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 8317F008: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8317F00C: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 8317F010: 7D684BD6  divw r11, r8, r9
	ctx.r[11].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8317F014: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 8317F018: 7CBF0194  addze r5, r31
	tmp.s64 = ctx.r[31].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[31].u32);
	ctx.r[5].s64 = tmp.s64;
	// 8317F01C: 7FE84BD6  divw r31, r8, r9
	ctx.r[31].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8317F020: 54A5083C  slwi r5, r5, 1
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8317F024: 7D3F49D6  mullw r9, r31, r9
	ctx.r[9].s64 = (ctx.r[31].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8317F028: 7FE55050  subf r31, r5, r10
	ctx.r[31].s64 = ctx.r[10].s64 - ctx.r[5].s64;
	// 8317F02C: 7D4BF3D6  divw r10, r11, r30
	ctx.r[10].s32 = ctx.r[11].s32 / ctx.r[30].s32;
	// 8317F030: 7CA94050  subf r5, r9, r8
	ctx.r[5].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 8317F034: 7D4A3214  add r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 8317F038: 7D2AF3D6  divw r9, r10, r30
	ctx.r[9].s32 = ctx.r[10].s32 / ctx.r[30].s32;
	// 8317F03C: 7D0AD896  mulhw r8, r10, r27
	ctx.r[8].s64 = ((ctx.r[10].s32 as i64 * ctx.r[27].s32 as i64) >> 32);
	// 8317F040: 7CC93A14  add r6, r9, r7
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 8317F044: 7D2BE096  mulhw r9, r11, r28
	ctx.r[9].s64 = ((ctx.r[11].s32 as i64 * ctx.r[28].s32 as i64) >> 32);
	// 8317F048: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8317F04C: 7D085214  add r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 8317F050: 7D292E70  srawi r9, r9, 5
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 5) as i64;
	// 8317F054: 7D082E70  srawi r8, r8, 5
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 5) as i64;
	// 8317F058: 55270FFE  srwi r7, r9, 0x1f
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shr(31);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8317F05C: 7D293A14  add r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 8317F060: 1D29003C  mulli r9, r9, 0x3c
	ctx.r[9].s64 = ctx.r[9].s64 * 60;
	// 8317F064: 7D295850  subf r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8317F068: 550B0FFE  srwi r11, r8, 0x1f
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(31);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317F06C: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8317F070: 1D6B003C  mulli r11, r11, 0x3c
	ctx.r[11].s64 = ctx.r[11].s64 * 60;
	// 8317F074: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8317F078: 419A004C  beq cr6, 0x8317f0c4
	if ctx.cr[6].eq {
	pc = 0x8317F0C4; continue 'dispatch;
	}
	// 8317F07C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8317F080: 409A0044  bne cr6, 0x8317f0c4
	if !ctx.cr[6].eq {
	pc = 0x8317F0C4; continue 'dispatch;
	}
	// 8317F084: 3D406666  lis r10, 0x6666
	ctx.r[10].s64 = 1717960704;
	// 8317F088: 614A6667  ori r10, r10, 0x6667
	ctx.r[10].u64 = ctx.r[10].u64 | 26215;
	// 8317F08C: 7D4B5096  mulhw r10, r11, r10
	ctx.r[10].s64 = ((ctx.r[11].s32 as i64 * ctx.r[10].s32 as i64) >> 32);
	// 8317F090: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 8317F094: 55480FFE  srwi r8, r10, 0x1f
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(31);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8317F098: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8317F09C: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8317F0A0: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8317F0A4: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8317F0A8: 7D4A5851  subf. r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8317F0AC: 41820018  beq 0x8317f0c4
	if ctx.cr[0].eq {
	pc = 0x8317F0C4; continue 'dispatch;
	}
	// 8317F0B0: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8317F0B4: 419A000C  beq cr6, 0x8317f0c0
	if ctx.cr[6].eq {
	pc = 0x8317F0C0; continue 'dispatch;
	}
	// 8317F0B8: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8317F0BC: 409A0008  bne cr6, 0x8317f0c4
	if !ctx.cr[6].eq {
	pc = 0x8317F0C4; continue 'dispatch;
	}
	// 8317F0C0: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8317F0C4: 93A40000  stw r29, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8317F0C8: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317F0CC: 90C40008  stw r6, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 8317F0D0: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8317F0D4: 91240010  stw r9, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 8317F0D8: 90A40014  stw r5, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 8317F0DC: 91040004  stw r8, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8317F0E0: B3E4001E  sth r31, 0x1e(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(30 as u32), ctx.r[31].u16 ) };
	// 8317F0E4: 480290CC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317F0E8 size=176
    let mut pc: u32 = 0x8317F0E8;
    'dispatch: loop {
        match pc {
            0x8317F0E8 => {
    //   block [0x8317F0E8..0x8317F198)
	// 8317F0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317F0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317F0F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317F0F4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317F0F8: 39630D88  addi r11, r3, 0xd88
	ctx.r[11].s64 = ctx.r[3].s64 + 3464;
	// 8317F0FC: 3BEB003C  addi r31, r11, 0x3c
	ctx.r[31].s64 = ctx.r[11].s64 + 60;
	// 8317F100: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F104: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8317F108: 409A007C  bne cr6, 0x8317f184
	if !ctx.cr[6].eq {
	pc = 0x8317F184; continue 'dispatch;
	}
	// 8317F10C: 396B001C  addi r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 + 28;
	// 8317F110: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8317F114: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8317F118: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8317F11C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F120: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8317F124: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8317F128: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8317F12C: 4200FFF0  bdnz 0x8317f11c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8317F11C; continue 'dispatch;
	}
	// 8317F130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317F134: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8317F138: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8317F13C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8317F140: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 8317F144: 4BFFC87D  bl 0x8317b9c0
	ctx.lr = 0x8317F148;
	sub_8317B9C0(ctx, base);
	// 8317F148: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 8317F14C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8317F150: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8317F154: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8317F158: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F15C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8317F160: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8317F164: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8317F168: 4200FFF0  bdnz 0x8317f158
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8317F158; continue 'dispatch;
	}
	// 8317F16C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317F170: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8317F174: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317F178: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8317F17C: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8317F180: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8317F184: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8317F188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317F18C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317F190: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317F194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317F198 size=184
    let mut pc: u32 = 0x8317F198;
    'dispatch: loop {
        match pc {
            0x8317F198 => {
    //   block [0x8317F198..0x8317F250)
	// 8317F198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317F19C: 48028FD1  bl 0x831a816c
	ctx.lr = 0x8317F1A0;
	sub_831A8130(ctx, base);
	// 8317F1A0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317F1A4: 3BE30D88  addi r31, r3, 0xd88
	ctx.r[31].s64 = ctx.r[3].s64 + 3464;
	// 8317F1A8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8317F1AC: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8317F1B0: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 8317F1B4: 3BBF0068  addi r29, r31, 0x68
	ctx.r[29].s64 = ctx.r[31].s64 + 104;
	// 8317F1B8: 3BDF00C0  addi r30, r31, 0xc0
	ctx.r[30].s64 = ctx.r[31].s64 + 192;
	// 8317F1BC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8317F1C0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F1C4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8317F1C8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8317F1CC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8317F1D0: 4200FFF0  bdnz 0x8317f1c0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8317F1C0; continue 'dispatch;
	}
	// 8317F1D4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317F1D8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8317F1DC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8317F1E0: 4BFFC7E1  bl 0x8317b9c0
	ctx.lr = 0x8317F1E4;
	sub_8317B9C0(ctx, base);
	// 8317F1E4: 395E0004  addi r10, r30, 4
	ctx.r[10].s64 = ctx.r[30].s64 + 4;
	// 8317F1E8: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8317F1EC: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8317F1F0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8317F1F4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F1F8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8317F1FC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8317F200: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8317F204: 4200FFF0  bdnz 0x8317f1f4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8317F1F4; continue 'dispatch;
	}
	// 8317F208: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8317F20C: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317F210: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317F214: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 8317F218: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8317F21C: 915E0028  stw r10, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8317F220: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8317F224: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8317F228: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317F22C: 815E0024  lwz r10, 0x24(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317F230: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8317F234: 41990014  bgt cr6, 0x8317f248
	if ctx.cr[6].gt {
	pc = 0x8317F248; continue 'dispatch;
	}
	// 8317F238: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 8317F23C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8317F240: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317F244: 480292CD  bl 0x831a8510
	ctx.lr = 0x8317F248;
	sub_831A8510(ctx, base);
	// 8317F248: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8317F24C: 48028F70  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F250 size=80
    let mut pc: u32 = 0x8317F250;
    'dispatch: loop {
        match pc {
            0x8317F250 => {
    //   block [0x8317F250..0x8317F2A0)
	// 8317F250: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8317F254: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8317F258: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317F25C: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8317F260: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8317F264: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8317F268: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8317F26C: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317F270: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8317F274: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8317F278: 91640010  stw r11, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8317F27C: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 8317F280: 91640014  stw r11, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8317F284: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8317F288: 91640018  stw r11, 0x18(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8317F28C: 89630054  lbz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317F290: B144001E  sth r10, 0x1e(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(30 as u32), ctx.r[10].u16 ) };
	// 8317F294: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8317F298: B164001C  sth r11, 0x1c(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(28 as u32), ctx.r[11].u16 ) };
	// 8317F29C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F2A0 size=52
    let mut pc: u32 = 0x8317F2A0;
    'dispatch: loop {
        match pc {
            0x8317F2A0 => {
    //   block [0x8317F2A0..0x8317F2D4)
	// 8317F2A0: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F2A4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317F2A8: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317F2AC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8317F2B0: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317F2B4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8317F2B8: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317F2BC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8317F2C0: 81650010  lwz r11, 0x10(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 8317F2C4: 90830010  stw r4, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 8317F2C8: 90C30020  stw r6, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 8317F2CC: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8317F2D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F2D8 size=28
    let mut pc: u32 = 0x8317F2D8;
    'dispatch: loop {
        match pc {
            0x8317F2D8 => {
    //   block [0x8317F2D8..0x8317F2F4)
	// 8317F2D8: 816317C8  lwz r11, 0x17c8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6088 as u32) ) } as u64;
	// 8317F2DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317F2E0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317F2E4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8317F2E8: 419A0010  beq cr6, 0x8317f2f8
	if ctx.cr[6].eq {
		sub_8317F2F8(ctx, base);
		return;
	}
	// 8317F2EC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8317F2F0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F2F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F2F4 size=4
    let mut pc: u32 = 0x8317F2F4;
    'dispatch: loop {
        match pc {
            0x8317F2F4 => {
    //   block [0x8317F2F4..0x8317F2F8)
	// 8317F2F4: 4800000C  b 0x8317f300
	sub_8317F2F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F2F8 size=16
    let mut pc: u32 = 0x8317F2F8;
    'dispatch: loop {
        match pc {
            0x8317F2F8 => {
    //   block [0x8317F2F8..0x8317F308)
	// 8317F2F8: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 8317F2FC: 419A000C  beq cr6, 0x8317f308
	if ctx.cr[6].eq {
		sub_8317F308(ctx, base);
		return;
	}
	// 8317F300: 2F040003  cmpwi cr6, r4, 3
	ctx.cr[6].compare_i32(ctx.r[4].s32, 3, &mut ctx.xer);
	// 8317F304: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F308 size=8
    let mut pc: u32 = 0x8317F308;
    'dispatch: loop {
        match pc {
            0x8317F308 => {
    //   block [0x8317F308..0x8317F310)
	// 8317F308: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317F30C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317F310 size=184
    let mut pc: u32 = 0x8317F310;
    'dispatch: loop {
        match pc {
            0x8317F310 => {
    //   block [0x8317F310..0x8317F3C8)
	// 8317F310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317F314: 48028E51  bl 0x831a8164
	ctx.lr = 0x8317F318;
	sub_831A8130(ctx, base);
	// 8317F318: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317F31C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317F320: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8317F324: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8317F328: 83FE17C8  lwz r31, 0x17c8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(6088 as u32) ) } as u64;
	// 8317F32C: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8317F330: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317F334: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317F338: 419A0058  beq cr6, 0x8317f390
	if ctx.cr[6].eq {
	pc = 0x8317F390; continue 'dispatch;
	}
	// 8317F33C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8317F340: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F344: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8317F348: 480136E9  bl 0x83192a30
	ctx.lr = 0x8317F34C;
	sub_83192A30(ctx, base);
	// 8317F34C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317F350: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317F354: 409A000C  bne cr6, 0x8317f360
	if !ctx.cr[6].eq {
	pc = 0x8317F360; continue 'dispatch;
	}
	// 8317F358: 3BA00005  li r29, 5
	ctx.r[29].s64 = 5;
	// 8317F35C: 48000034  b 0x8317f390
	pc = 0x8317F390; continue 'dispatch;
	// 8317F360: 817E0EA0  lwz r11, 0xea0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(3744 as u32) ) } as u64;
	// 8317F364: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317F368: 409A0018  bne cr6, 0x8317f380
	if !ctx.cr[6].eq {
	pc = 0x8317F380; continue 'dispatch;
	}
	// 8317F36C: 38800049  li r4, 0x49
	ctx.r[4].s64 = 73;
	// 8317F370: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317F374: 4BFFA09D  bl 0x83179410
	ctx.lr = 0x8317F378;
	sub_83179410(ctx, base);
	// 8317F378: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317F37C: 419A0010  beq cr6, 0x8317f38c
	if ctx.cr[6].eq {
	pc = 0x8317F38C; continue 'dispatch;
	}
	// 8317F380: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317F384: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317F388: 409A0008  bne cr6, 0x8317f390
	if !ctx.cr[6].eq {
	pc = 0x8317F390; continue 'dispatch;
	}
	// 8317F38C: 3BA00002  li r29, 2
	ctx.r[29].s64 = 2;
	// 8317F390: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 8317F394: 409A0028  bne cr6, 0x8317f3bc
	if !ctx.cr[6].eq {
	pc = 0x8317F3BC; continue 'dispatch;
	}
	// 8317F398: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 8317F39C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317F3A0: 419A000C  beq cr6, 0x8317f3ac
	if ctx.cr[6].eq {
	pc = 0x8317F3AC; continue 'dispatch;
	}
	// 8317F3A4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8317F3A8: 409A0014  bne cr6, 0x8317f3bc
	if !ctx.cr[6].eq {
	pc = 0x8317F3BC; continue 'dispatch;
	}
	// 8317F3AC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8317F3B0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8317F3B4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8317F3B8: 48028DFC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8317F3BC: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8317F3C0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8317F3C4: 48028DF0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F3C8 size=16
    let mut pc: u32 = 0x8317F3C8;
    'dispatch: loop {
        match pc {
            0x8317F3C8 => {
    //   block [0x8317F3C8..0x8317F3D8)
	// 8317F3C8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317F3CC: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 8317F3D0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8317F3D4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F3D8 size=8
    let mut pc: u32 = 0x8317F3D8;
    'dispatch: loop {
        match pc {
            0x8317F3D8 => {
    //   block [0x8317F3D8..0x8317F3E0)
	// 8317F3D8: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 8317F3DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317F3E0 size=104
    let mut pc: u32 = 0x8317F3E0;
    'dispatch: loop {
        match pc {
            0x8317F3E0 => {
    //   block [0x8317F3E0..0x8317F448)
	// 8317F3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317F3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317F3E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317F3EC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317F3F0: 394B0D88  addi r10, r11, 0xd88
	ctx.r[10].s64 = ctx.r[11].s64 + 3464;
	// 8317F3F4: 806B1E30  lwz r3, 0x1e30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7728 as u32) ) } as u64;
	// 8317F3F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317F3FC: 40980018  bge cr6, 0x8317f414
	if !ctx.cr[6].lt {
	pc = 0x8317F414; continue 'dispatch;
	}
	// 8317F400: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317F404: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317F408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317F40C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317F410: 4E800020  blr
	return;
	// 8317F414: 812A0118  lwz r9, 0x118(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(280 as u32) ) } as u64;
	// 8317F418: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8317F41C: 409AFFE4  bne cr6, 0x8317f400
	if !ctx.cr[6].eq {
	pc = 0x8317F400; continue 'dispatch;
	}
	// 8317F420: 80CA00E8  lwz r6, 0xe8(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(232 as u32) ) } as u64;
	// 8317F424: 80AA00E4  lwz r5, 0xe4(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(228 as u32) ) } as u64;
	// 8317F428: 808B1E34  lwz r4, 0x1e34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7732 as u32) ) } as u64;
	// 8317F42C: 480105DD  bl 0x8318fa08
	ctx.lr = 0x8317F430;
	sub_8318FA08(ctx, base);
	// 8317F430: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8317F434: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317F438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317F43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317F440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317F444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F448 size=32
    let mut pc: u32 = 0x8317F448;
    'dispatch: loop {
        match pc {
            0x8317F448 => {
    //   block [0x8317F448..0x8317F468)
	// 8317F448: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 8317F44C: 419A003C  beq cr6, 0x8317f488
	if ctx.cr[6].eq {
		sub_8317F488(ctx, base);
		return;
	}
	// 8317F450: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 8317F454: 419A0024  beq cr6, 0x8317f478
	if ctx.cr[6].eq {
		sub_8317F478(ctx, base);
		return;
	}
	// 8317F458: 2F040003  cmpwi cr6, r4, 3
	ctx.cr[6].compare_i32(ctx.r[4].s32, 3, &mut ctx.xer);
	// 8317F45C: 419A000C  beq cr6, 0x8317f468
	if ctx.cr[6].eq {
		sub_8317F468(ctx, base);
		return;
	}
	// 8317F460: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317F464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F468 size=16
    let mut pc: u32 = 0x8317F468;
    'dispatch: loop {
        match pc {
            0x8317F468 => {
    //   block [0x8317F468..0x8317F478)
	// 8317F468: 81630A1C  lwz r11, 0xa1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2588 as u32) ) } as u64;
	// 8317F46C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8317F470: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317F474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F478 size=16
    let mut pc: u32 = 0x8317F478;
    'dispatch: loop {
        match pc {
            0x8317F478 => {
    //   block [0x8317F478..0x8317F488)
	// 8317F478: 81630A18  lwz r11, 0xa18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2584 as u32) ) } as u64;
	// 8317F47C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8317F480: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317F484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F488 size=16
    let mut pc: u32 = 0x8317F488;
    'dispatch: loop {
        match pc {
            0x8317F488 => {
    //   block [0x8317F488..0x8317F498)
	// 8317F488: 81630A14  lwz r11, 0xa14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2580 as u32) ) } as u64;
	// 8317F48C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8317F490: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317F494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317F498 size=172
    let mut pc: u32 = 0x8317F498;
    'dispatch: loop {
        match pc {
            0x8317F498 => {
    //   block [0x8317F498..0x8317F544)
	// 8317F498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317F49C: 48028CD1  bl 0x831a816c
	ctx.lr = 0x8317F4A0;
	sub_831A8130(ctx, base);
	// 8317F4A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317F4A4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8317F4A8: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8317F4AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317F4B0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8317F4B4: 4BFF9F5D  bl 0x83179410
	ctx.lr = 0x8317F4B8;
	sub_83179410(ctx, base);
	// 8317F4B8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317F4BC: 409A007C  bne cr6, 0x8317f538
	if !ctx.cr[6].eq {
	pc = 0x8317F538; continue 'dispatch;
	}
	// 8317F4C0: 2F1D0003  cmpwi cr6, r29, 3
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3, &mut ctx.xer);
	// 8317F4C4: 409A0038  bne cr6, 0x8317f4fc
	if !ctx.cr[6].eq {
	pc = 0x8317F4FC; continue 'dispatch;
	}
	// 8317F4C8: 817F091C  lwz r11, 0x91c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2332 as u32) ) } as u64;
	// 8317F4CC: 815F0918  lwz r10, 0x918(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2328 as u32) ) } as u64;
	// 8317F4D0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317F4D4: 7CAB51D6  mullw r5, r11, r10
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8317F4D8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F4DC: 480135AD  bl 0x83192a88
	ctx.lr = 0x8317F4E0;
	sub_83192A88(ctx, base);
	// 8317F4E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317F4E4: 419A0058  beq cr6, 0x8317f53c
	if ctx.cr[6].eq {
	pc = 0x8317F53C; continue 'dispatch;
	}
	// 8317F4E8: 817F0960  lwz r11, 0x960(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2400 as u32) ) } as u64;
	// 8317F4EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8317F4F0: 917F0960  stw r11, 0x960(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2400 as u32), ctx.r[11].u32 ) };
	// 8317F4F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317F4F8: 48028CC4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317F4FC: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 8317F500: 409A0038  bne cr6, 0x8317f538
	if !ctx.cr[6].eq {
	pc = 0x8317F538; continue 'dispatch;
	}
	// 8317F504: 817F091C  lwz r11, 0x91c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2332 as u32) ) } as u64;
	// 8317F508: 815F0918  lwz r10, 0x918(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2328 as u32) ) } as u64;
	// 8317F50C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317F510: 7CAB51D6  mullw r5, r11, r10
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8317F514: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F518: 48013821  bl 0x83192d38
	ctx.lr = 0x8317F51C;
	sub_83192D38(ctx, base);
	// 8317F51C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317F520: 419A001C  beq cr6, 0x8317f53c
	if ctx.cr[6].eq {
	pc = 0x8317F53C; continue 'dispatch;
	}
	// 8317F524: 817F0964  lwz r11, 0x964(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2404 as u32) ) } as u64;
	// 8317F528: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8317F52C: 917F0964  stw r11, 0x964(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2404 as u32), ctx.r[11].u32 ) };
	// 8317F530: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317F534: 48028C88  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317F538: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317F53C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317F540: 48028C7C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F548 size=20
    let mut pc: u32 = 0x8317F548;
    'dispatch: loop {
        match pc {
            0x8317F548 => {
    //   block [0x8317F548..0x8317F55C)
	// 8317F548: 81630AB8  lwz r11, 0xab8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2744 as u32) ) } as u64;
	// 8317F54C: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317F550: 81630AB4  lwz r11, 0xab4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2740 as u32) ) } as u64;
	// 8317F554: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317F558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F560 size=24
    let mut pc: u32 = 0x8317F560;
    'dispatch: loop {
        match pc {
            0x8317F560 => {
    //   block [0x8317F560..0x8317F578)
	// 8317F560: 39630D88  addi r11, r3, 0xd88
	ctx.r[11].s64 = ctx.r[3].s64 + 3464;
	// 8317F564: 388B00C0  addi r4, r11, 0xc0
	ctx.r[4].s64 = ctx.r[11].s64 + 192;
	// 8317F568: 814B013C  lwz r10, 0x13c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(316 as u32) ) } as u64;
	// 8317F56C: 81240024  lwz r9, 0x24(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317F570: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8317F574: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F578 size=12
    let mut pc: u32 = 0x8317F578;
    'dispatch: loop {
        match pc {
            0x8317F578 => {
    //   block [0x8317F578..0x8317F584)
	// 8317F578: 386B00EC  addi r3, r11, 0xec
	ctx.r[3].s64 = ctx.r[11].s64 + 236;
	// 8317F57C: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 8317F580: 48028F90  b 0x831a8510
	sub_831A8510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F584(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F584 size=4
    let mut pc: u32 = 0x8317F584;
    'dispatch: loop {
        match pc {
            0x8317F584 => {
    //   block [0x8317F584..0x8317F588)
	// 8317F584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8317F588 size=720
    let mut pc: u32 = 0x8317F588;
    'dispatch: loop {
        match pc {
            0x8317F588 => {
    //   block [0x8317F588..0x8317F858)
	// 8317F588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317F58C: 48028BD9  bl 0x831a8164
	ctx.lr = 0x8317F590;
	sub_831A8130(ctx, base);
	// 8317F590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317F594: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8317F598: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8317F59C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8317F5A0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8317F5A4: 83DB17C8  lwz r30, 0x17c8(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(6088 as u32) ) } as u64;
	// 8317F5A8: 817E00F0  lwz r11, 0xf0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(240 as u32) ) } as u64;
	// 8317F5AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317F5B0: 419A0188  beq cr6, 0x8317f738
	if ctx.cr[6].eq {
	pc = 0x8317F738; continue 'dispatch;
	}
	// 8317F5B4: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317F5B8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F5BC: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 8317F5C0: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F5C4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8317F5C8: B16A000C  sth r11, 0xc(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
	// 8317F5CC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317F5D0: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F5D4: B16A000E  sth r11, 0xe(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(14 as u32), ctx.r[11].u16 ) };
	// 8317F5D8: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317F5DC: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F5E0: B16A0010  sth r11, 0x10(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 8317F5E4: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317F5E8: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F5EC: B16A0012  sth r11, 0x12(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(18 as u32), ctx.r[11].u16 ) };
	// 8317F5F0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F5F4: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8317F5F8: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8317F5FC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F600: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 8317F604: 48028F0D  bl 0x831a8510
	ctx.lr = 0x8317F608;
	sub_831A8510(ctx, base);
	// 8317F608: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F60C: E95E0100  ld r10, 0x100(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(256 as u32) ) };
	// 8317F610: F94B00E8  std r10, 0xe8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(232 as u32), ctx.r[10].u64 ) };
	// 8317F614: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F618: 815D0034  lwz r10, 0x34(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 8317F61C: 914B00F0  stw r10, 0xf0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(240 as u32), ctx.r[10].u32 ) };
	// 8317F620: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F624: 815D0030  lwz r10, 0x30(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 8317F628: 914B00F4  stw r10, 0xf4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(244 as u32), ctx.r[10].u32 ) };
	// 8317F62C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F630: 815D0068  lwz r10, 0x68(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8317F634: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 8317F638: 817B0038  lwz r11, 0x38(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(56 as u32) ) } as u64;
	// 8317F63C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8317F640: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8317F644: 409A014C  bne cr6, 0x8317f790
	if !ctx.cr[6].eq {
	pc = 0x8317F790; continue 'dispatch;
	}
	// 8317F648: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317F64C: 419A000C  beq cr6, 0x8317f658
	if ctx.cr[6].eq {
	pc = 0x8317F658; continue 'dispatch;
	}
	// 8317F650: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8317F654: 409A0028  bne cr6, 0x8317f67c
	if !ctx.cr[6].eq {
	pc = 0x8317F67C; continue 'dispatch;
	}
	// 8317F658: 817E00F0  lwz r11, 0xf0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(240 as u32) ) } as u64;
	// 8317F65C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317F660: 409A001C  bne cr6, 0x8317f67c
	if !ctx.cr[6].eq {
	pc = 0x8317F67C; continue 'dispatch;
	}
	// 8317F664: 807E00E8  lwz r3, 0xe8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(232 as u32) ) } as u64;
	// 8317F668: 48008929  bl 0x83187f90
	ctx.lr = 0x8317F66C;
	sub_83187F90(ctx, base);
	// 8317F66C: 817E00EC  lwz r11, 0xec(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(236 as u32) ) } as u64;
	// 8317F670: 917E00E8  stw r11, 0xe8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(232 as u32), ctx.r[11].u32 ) };
	// 8317F674: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F678: 917E00EC  stw r11, 0xec(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(236 as u32), ctx.r[11].u32 ) };
	// 8317F67C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F680: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317F684: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 8317F688: 392A000F  addi r9, r10, 0xf
	ctx.r[9].s64 = ctx.r[10].s64 + 15;
	// 8317F68C: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 8317F690: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8317F694: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317F698: 394B007F  addi r10, r11, 0x7f
	ctx.r[10].s64 = ctx.r[11].s64 + 127;
	// 8317F69C: 7D4A3E70  srawi r10, r10, 7
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 7) as i64;
	// 8317F6A0: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8317F6A4: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8317F6A8: 55483830  slwi r8, r10, 7
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(7);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8317F6AC: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8317F6B0: 7D080734  extsh r8, r8
	ctx.r[8].s64 = ctx.r[8].s16 as i64;
	// 8317F6B4: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 8317F6B8: 7D6B3E70  srawi r11, r11, 7
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 7) as i64;
	// 8317F6BC: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8317F6C0: B11F000E  sth r8, 0xe(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[8].u16 ) };
	// 8317F6C4: 7D292670  srawi r9, r9, 4
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 4) as i64;
	// 8317F6C8: 55673830  slwi r7, r11, 7
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8317F6CC: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 8317F6D0: 7CE70734  extsh r7, r7
	ctx.r[7].s64 = ctx.r[7].s16 as i64;
	// 8317F6D4: 7D4951D6  mullw r10, r9, r10
	ctx.r[10].s64 = (ctx.r[9].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8317F6D8: B0FF000C  sth r7, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u16 ) };
	// 8317F6DC: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8317F6E0: 554A5828  slwi r10, r10, 0xb
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(11);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8317F6E4: 7D290E70  srawi r9, r9, 1
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 1) as i64;
	// 8317F6E8: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 8317F6EC: 7D6959D6  mullw r11, r9, r11
	ctx.r[11].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8317F6F0: 813E00E8  lwz r9, 0xe8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(232 as u32) ) } as u64;
	// 8317F6F4: 81290008  lwz r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317F6F8: B11F001E  sth r8, 0x1e(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(30 as u32), ctx.r[8].u16 ) };
	// 8317F6FC: B0FF001C  sth r7, 0x1c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[7].u16 ) };
	// 8317F700: 7D0A4A14  add r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8317F704: 556B3830  slwi r11, r11, 7
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317F708: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8317F70C: 7D285A14  add r9, r8, r11
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8317F710: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8317F714: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8317F718: 813E00EC  lwz r9, 0xec(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(236 as u32) ) } as u64;
	// 8317F71C: 81290008  lwz r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317F720: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 8317F724: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8317F728: 913F0018  stw r9, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 8317F72C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8317F730: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8317F734: 480000EC  b 0x8317f820
	pc = 0x8317F820; continue 'dispatch;
	// 8317F738: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8317F73C: 4800874D  bl 0x83187e88
	ctx.lr = 0x8317F740;
	sub_83187E88(ctx, base);
	// 8317F740: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317F744: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8317F748: 409AFE70  bne cr6, 0x8317f5b8
	if !ctx.cr[6].eq {
	pc = 0x8317F5B8; continue 'dispatch;
	}
	// 8317F74C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8317F750: 917B097C  stw r11, 0x97c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(2428 as u32), ctx.r[11].u32 ) };
	// 8317F754: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8317F758: 806BA32C  lwz r3, -0x5cd4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23764 as u32) ) } as u64;
	// 8317F75C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317F760: 419A0024  beq cr6, 0x8317f784
	if ctx.cr[6].eq {
	pc = 0x8317F784; continue 'dispatch;
	}
	// 8317F764: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8317F768: 396BE6B8  addi r11, r11, -0x1948
	ctx.r[11].s64 = ctx.r[11].s64 + -6472;
	// 8317F76C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8317F770: 936B000C  stw r27, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 8317F774: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F778: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317F77C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317F780: 4E800421  bctrl
	ctx.lr = 0x8317F784;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317F784: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8317F788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317F78C: 48028A28  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8317F790: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317F794: 419A000C  beq cr6, 0x8317f7a0
	if ctx.cr[6].eq {
	pc = 0x8317F7A0; continue 'dispatch;
	}
	// 8317F798: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8317F79C: 409A0024  bne cr6, 0x8317f7c0
	if !ctx.cr[6].eq {
	pc = 0x8317F7C0; continue 'dispatch;
	}
	// 8317F7A0: 817E00C0  lwz r11, 0xc0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(192 as u32) ) } as u64;
	// 8317F7A4: 815E00C4  lwz r10, 0xc4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 8317F7A8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8317F7AC: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 8317F7B0: 917E00C0  stw r11, 0xc0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 8317F7B4: 915E00C4  stw r10, 0xc4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), ctx.r[10].u32 ) };
	// 8317F7B8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F7BC: 917E00EC  stw r11, 0xec(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(236 as u32), ctx.r[11].u32 ) };
	// 8317F7C0: 817E00C0  lwz r11, 0xc0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(192 as u32) ) } as u64;
	// 8317F7C4: 393E00C8  addi r9, r30, 0xc8
	ctx.r[9].s64 = ctx.r[30].s64 + 200;
	// 8317F7C8: 556A2036  slwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8317F7CC: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 8317F7D0: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8317F7D4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F7D8: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8317F7DC: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317F7E0: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8317F7E4: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317F7E8: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 8317F7EC: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317F7F0: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8317F7F4: 815E00C4  lwz r10, 0xc4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 8317F7F8: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8317F7FC: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8317F800: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F804: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8317F808: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317F80C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8317F810: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317F814: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8317F818: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317F81C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8317F820: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F824: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317F828: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317F82C: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317F830: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8317F834: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F838: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8317F83C: 394A0068  addi r10, r10, 0x68
	ctx.r[10].s64 = ctx.r[10].s64 + 104;
	// 8317F840: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8317F844: B17F0040  sth r11, 0x40(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u16 ) };
	// 8317F848: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 8317F84C: 917B097C  stw r11, 0x97c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(2428 as u32), ctx.r[11].u32 ) };
	// 8317F850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317F854: 48028960  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F858 size=84
    let mut pc: u32 = 0x8317F858;
    'dispatch: loop {
        match pc {
            0x8317F858 => {
    //   block [0x8317F858..0x8317F8AC)
	// 8317F858: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 8317F85C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8317F860: 409A0068  bne cr6, 0x8317f8c8
	if !ctx.cr[6].eq {
		sub_8317F8C8(ctx, base);
		return;
	}
	// 8317F864: 8164002C  lwz r11, 0x2c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 8317F868: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8317F86C: 419A0040  beq cr6, 0x8317f8ac
	if ctx.cr[6].eq {
		sub_8317F8AC(ctx, base);
		return;
	}
	// 8317F870: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8317F874: 409A0054  bne cr6, 0x8317f8c8
	if !ctx.cr[6].eq {
		sub_8317F8C8(ctx, base);
		return;
	}
	// 8317F878: 816400EC  lwz r11, 0xec(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(236 as u32) ) } as u64;
	// 8317F87C: 814400E8  lwz r10, 0xe8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(232 as u32) ) } as u64;
	// 8317F880: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 8317F884: 814A004C  lwz r10, 0x4c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 8317F888: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317F88C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317F890: 816400EC  lwz r11, 0xec(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(236 as u32) ) } as u64;
	// 8317F894: 814400E8  lwz r10, 0xe8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(232 as u32) ) } as u64;
	// 8317F898: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317F89C: 814A0050  lwz r10, 0x50(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317F8A0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317F8A4: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317F8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F8AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F8AC size=28
    let mut pc: u32 = 0x8317F8AC;
    'dispatch: loop {
        match pc {
            0x8317F8AC => {
    //   block [0x8317F8AC..0x8317F8C8)
	// 8317F8AC: 816400E8  lwz r11, 0xe8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(232 as u32) ) } as u64;
	// 8317F8B0: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 8317F8B4: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317F8B8: 816400E8  lwz r11, 0xe8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(232 as u32) ) } as u64;
	// 8317F8BC: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317F8C0: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317F8C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F8C8 size=16
    let mut pc: u32 = 0x8317F8C8;
    'dispatch: loop {
        match pc {
            0x8317F8C8 => {
    //   block [0x8317F8C8..0x8317F8D8)
	// 8317F8C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317F8CC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317F8D0: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317F8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317F8D8 size=236
    let mut pc: u32 = 0x8317F8D8;
    'dispatch: loop {
        match pc {
            0x8317F8D8 => {
    //   block [0x8317F8D8..0x8317F9C4)
	// 8317F8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317F8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317F8E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317F8E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317F8E8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317F8EC: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8317F8F0: 3BC80D88  addi r30, r8, 0xd88
	ctx.r[30].s64 = ctx.r[8].s64 + 3464;
	// 8317F8F4: 3BFE0118  addi r31, r30, 0x118
	ctx.r[31].s64 = ctx.r[30].s64 + 280;
	// 8317F8F8: 397E00C0  addi r11, r30, 0xc0
	ctx.r[11].s64 = ctx.r[30].s64 + 192;
	// 8317F8FC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F900: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8317F904: 409A00A8  bne cr6, 0x8317f9ac
	if !ctx.cr[6].eq {
	pc = 0x8317F9AC; continue 'dispatch;
	}
	// 8317F908: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8317F90C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8317F910: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8317F914: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8317F918: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F91C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8317F920: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8317F924: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8317F928: 4200FFF0  bdnz 0x8317f918
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8317F918; continue 'dispatch;
	}
	// 8317F92C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8317F930: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8317F934: 4BFFF9A5  bl 0x8317f2d8
	ctx.lr = 0x8317F938;
	sub_8317F2D8(ctx, base);
	// 8317F938: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317F93C: 409A001C  bne cr6, 0x8317f958
	if !ctx.cr[6].eq {
	pc = 0x8317F958; continue 'dispatch;
	}
	// 8317F940: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8317F944: 4BFFFB05  bl 0x8317f448
	ctx.lr = 0x8317F948;
	sub_8317F448(ctx, base);
	// 8317F948: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317F94C: 409A000C  bne cr6, 0x8317f958
	if !ctx.cr[6].eq {
	pc = 0x8317F958; continue 'dispatch;
	}
	// 8317F950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317F954: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 8317F958: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317F95C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8317F960: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8317F964: 4BFFC05D  bl 0x8317b9c0
	ctx.lr = 0x8317F968;
	sub_8317B9C0(ctx, base);
	// 8317F968: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 8317F96C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8317F970: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8317F974: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8317F978: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317F97C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8317F980: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8317F984: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8317F988: 4200FFF0  bdnz 0x8317f978
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8317F978; continue 'dispatch;
	}
	// 8317F98C: 817E0060  lwz r11, 0x60(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 8317F990: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317F994: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317F998: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 8317F99C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8317F9A0: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8317F9A4: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8317F9A8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8317F9AC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8317F9B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317F9B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317F9B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317F9BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317F9C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317F9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317F9C8 size=76
    let mut pc: u32 = 0x8317F9C8;
    'dispatch: loop {
        match pc {
            0x8317F9C8 => {
    //   block [0x8317F9C8..0x8317FA14)
	// 8317F9C8: 81440040  lwz r10, 0x40(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) } as u64;
	// 8317F9CC: 39630D88  addi r11, r3, 0xd88
	ctx.r[11].s64 = ctx.r[3].s64 + 3464;
	// 8317F9D0: 91440048  stw r10, 0x48(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 8317F9D4: 810B013C  lwz r8, 0x13c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(316 as u32) ) } as u64;
	// 8317F9D8: 814B0164  lwz r10, 0x164(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(356 as u32) ) } as u64;
	// 8317F9DC: 8124003C  lwz r9, 0x3c(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 8317F9E0: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 8317F9E4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8317F9E8: 91440044  stw r10, 0x44(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 8317F9EC: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8317F9F0: 8124003C  lwz r9, 0x3c(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 8317F9F4: 91240058  stw r9, 0x58(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 8317F9F8: 812B0164  lwz r9, 0x164(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(356 as u32) ) } as u64;
	// 8317F9FC: 8104003C  lwz r8, 0x3c(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 8317FA00: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8317FA04: 9124005C  stw r9, 0x5c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8317FA08: 812B0288  lwz r9, 0x288(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(648 as u32) ) } as u64;
	// 8317FA0C: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8317FA10: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FA14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317FA14 size=16
    let mut pc: u32 = 0x8317FA14;
    'dispatch: loop {
        match pc {
            0x8317FA14 => {
    //   block [0x8317FA14..0x8317FA24)
	// 8317FA14: 914B0288  stw r10, 0x288(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(648 as u32), ctx.r[10].u32 ) };
	// 8317FA18: 81440048  lwz r10, 0x48(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 8317FA1C: 914B028C  stw r10, 0x28c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(652 as u32), ctx.r[10].u32 ) };
	// 8317FA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317FA28 size=76
    let mut pc: u32 = 0x8317FA28;
    'dispatch: loop {
        match pc {
            0x8317FA28 => {
    //   block [0x8317FA28..0x8317FA74)
	// 8317FA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317FA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317FA30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317FA34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317FA38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317FA3C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8317FA40: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317FA44: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317FA48: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317FA4C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317FA50: 48028AC1  bl 0x831a8510
	ctx.lr = 0x8317FA54;
	sub_831A8510(ctx, base);
	// 8317FA54: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317FA58: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8317FA5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317FA60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317FA64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317FA68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317FA6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317FA70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317FA78 size=12
    let mut pc: u32 = 0x8317FA78;
    'dispatch: loop {
        match pc {
            0x8317FA78 => {
    //   block [0x8317FA78..0x8317FA84)
	// 8317FA78: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8317FA7C: 808317D0  lwz r4, 0x17d0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6096 as u32) ) } as u64;
	// 8317FA80: 4800EF10  b 0x8318e990
	sub_8318E990(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317FA88 size=92
    let mut pc: u32 = 0x8317FA88;
    'dispatch: loop {
        match pc {
            0x8317FA88 => {
    //   block [0x8317FA88..0x8317FAE4)
	// 8317FA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317FA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317FA90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317FA94: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317FA98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317FA9C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317FAA0: 809F17D0  lwz r4, 0x17d0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6096 as u32) ) } as u64;
	// 8317FAA4: 4800E3F5  bl 0x8318de98
	ctx.lr = 0x8317FAA8;
	sub_8318DE98(ctx, base);
	// 8317FAA8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317FAAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317FAB0: 419A0020  beq cr6, 0x8317fad0
	if ctx.cr[6].eq {
	pc = 0x8317FAD0; continue 'dispatch;
	}
	// 8317FAB4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8317FAB8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8317FABC: 4800E9ED  bl 0x8318e4a8
	ctx.lr = 0x8317FAC0;
	sub_8318E4A8(ctx, base);
	// 8317FAC0: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317FAC4: E87F09A0  ld r3, 0x9a0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2464 as u32) ) };
	// 8317FAC8: 4800E659  bl 0x8318e120
	ctx.lr = 0x8317FACC;
	sub_8318E120(ctx, base);
	// 8317FACC: F87F09A0  std r3, 0x9a0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2464 as u32), ctx.r[3].u64 ) };
	// 8317FAD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317FAD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317FAD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317FADC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317FAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317FAE8 size=12
    let mut pc: u32 = 0x8317FAE8;
    'dispatch: loop {
        match pc {
            0x8317FAE8 => {
    //   block [0x8317FAE8..0x8317FAF4)
	// 8317FAE8: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8317FAEC: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 8317FAF0: 48013598  b 0x83193088
	sub_83193088(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317FAF8 size=16
    let mut pc: u32 = 0x8317FAF8;
    'dispatch: loop {
        match pc {
            0x8317FAF8 => {
    //   block [0x8317FAF8..0x8317FB08)
	// 8317FAF8: 2F04FFFD  cmpwi cr6, r4, -3
	ctx.cr[6].compare_i32(ctx.r[4].s32, -3, &mut ctx.xer);
	// 8317FAFC: 41980014  blt cr6, 0x8317fb10
	if ctx.cr[6].lt {
		sub_8317FB10(ctx, base);
		return;
	}
	// 8317FB00: 2F04FFFE  cmpwi cr6, r4, -2
	ctx.cr[6].compare_i32(ctx.r[4].s32, -2, &mut ctx.xer);
	// 8317FB04: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317FB08 size=8
    let mut pc: u32 = 0x8317FB08;
    'dispatch: loop {
        match pc {
            0x8317FB08 => {
    //   block [0x8317FB08..0x8317FB10)
	// 8317FB08: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8317FB0C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317FB10 size=8
    let mut pc: u32 = 0x8317FB10;
    'dispatch: loop {
        match pc {
            0x8317FB10 => {
    //   block [0x8317FB10..0x8317FB18)
	// 8317FB10: 480079E8  b 0x831874f8
	sub_831874F8(ctx, base);
	return;
	// 8317FB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317FB18 size=4
    let mut pc: u32 = 0x8317FB18;
    'dispatch: loop {
        match pc {
            0x8317FB18 => {
    //   block [0x8317FB18..0x8317FB1C)
	// 8317FB18: 480104B0  b 0x8318ffc8
	sub_8318FFC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317FB20 size=52
    let mut pc: u32 = 0x8317FB20;
    'dispatch: loop {
        match pc {
            0x8317FB20 => {
    //   block [0x8317FB20..0x8317FB54)
	// 8317FB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317FB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317FB28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317FB2C: 816317C8  lwz r11, 0x17c8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6088 as u32) ) } as u64;
	// 8317FB30: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317FB34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317FB38: 419A0008  beq cr6, 0x8317fb40
	if ctx.cr[6].eq {
	pc = 0x8317FB40; continue 'dispatch;
	}
	// 8317FB3C: 480137BD  bl 0x831932f8
	ctx.lr = 0x8317FB40;
	sub_831932F8(ctx, base);
	// 8317FB40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317FB44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317FB48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317FB4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317FB50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317FB58 size=12
    let mut pc: u32 = 0x8317FB58;
    'dispatch: loop {
        match pc {
            0x8317FB58 => {
    //   block [0x8317FB58..0x8317FB64)
	// 8317FB58: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317FB5C: 60840F0D  ori r4, r4, 0xf0d
	ctx.r[4].u64 = ctx.r[4].u64 | 3853;
	// 8317FB60: 48007998  b 0x831874f8
	sub_831874F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317FB68 size=348
    let mut pc: u32 = 0x8317FB68;
    'dispatch: loop {
        match pc {
            0x8317FB68 => {
    //   block [0x8317FB68..0x8317FCC4)
	// 8317FB68: 81640068  lwz r11, 0x68(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(104 as u32) ) } as u64;
	// 8317FB6C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317FB70: 8164006C  lwz r11, 0x6c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(108 as u32) ) } as u64;
	// 8317FB74: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8317FB78: 81640070  lwz r11, 0x70(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(112 as u32) ) } as u64;
	// 8317FB7C: 91650008  stw r11, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8317FB80: 81640074  lwz r11, 0x74(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(116 as u32) ) } as u64;
	// 8317FB84: 9165000C  stw r11, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8317FB88: 81640080  lwz r11, 0x80(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(128 as u32) ) } as u64;
	// 8317FB8C: 91650010  stw r11, 0x10(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8317FB90: 81640044  lwz r11, 0x44(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 8317FB94: 91650014  stw r11, 0x14(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8317FB98: 81640048  lwz r11, 0x48(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 8317FB9C: 91650018  stw r11, 0x18(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8317FBA0: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 8317FBA4: 9165001C  stw r11, 0x1c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8317FBA8: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317FBAC: 91650020  stw r11, 0x20(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8317FBB0: 8164004C  lwz r11, 0x4c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 8317FBB4: 91650024  stw r11, 0x24(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8317FBB8: 81640050  lwz r11, 0x50(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317FBBC: 91650028  stw r11, 0x28(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8317FBC0: 81640054  lwz r11, 0x54(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317FBC4: 9165002C  stw r11, 0x2c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8317FBC8: 81640058  lwz r11, 0x58(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(88 as u32) ) } as u64;
	// 8317FBCC: 91650030  stw r11, 0x30(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8317FBD0: 8164005C  lwz r11, 0x5c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(92 as u32) ) } as u64;
	// 8317FBD4: 91650034  stw r11, 0x34(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8317FBD8: 81640060  lwz r11, 0x60(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(96 as u32) ) } as u64;
	// 8317FBDC: 91650038  stw r11, 0x38(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8317FBE0: 816400A8  lwz r11, 0xa8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(168 as u32) ) } as u64;
	// 8317FBE4: 9165003C  stw r11, 0x3c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8317FBE8: 816400AC  lwz r11, 0xac(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 8317FBEC: 91650040  stw r11, 0x40(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 8317FBF0: 816400A8  lwz r11, 0xa8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(168 as u32) ) } as u64;
	// 8317FBF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317FBF8: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8317FBFC: 419A0008  beq cr6, 0x8317fc04
	if ctx.cr[6].eq {
	pc = 0x8317FC04; continue 'dispatch;
	}
	// 8317FC00: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8317FC04: 91650048  stw r11, 0x48(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 8317FC08: E96400E8  ld r11, 0xe8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(232 as u32) ) };
	// 8317FC0C: F9650050  std r11, 0x50(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8317FC10: 816400A0  lwz r11, 0xa0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(160 as u32) ) } as u64;
	// 8317FC14: 91650058  stw r11, 0x58(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8317FC18: 816400A4  lwz r11, 0xa4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(164 as u32) ) } as u64;
	// 8317FC1C: 9165005C  stw r11, 0x5c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8317FC20: 816400B0  lwz r11, 0xb0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(176 as u32) ) } as u64;
	// 8317FC24: 91650060  stw r11, 0x60(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8317FC28: 816400B4  lwz r11, 0xb4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(180 as u32) ) } as u64;
	// 8317FC2C: 91650064  stw r11, 0x64(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8317FC30: A16400B8  lhz r11, 0xb8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(184 as u32) ) } as u64;
	// 8317FC34: B1650068  sth r11, 0x68(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(104 as u32), ctx.r[11].u16 ) };
	// 8317FC38: A16400BA  lhz r11, 0xba(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(186 as u32) ) } as u64;
	// 8317FC3C: B165006A  sth r11, 0x6a(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(106 as u32), ctx.r[11].u16 ) };
	// 8317FC40: 896400BD  lbz r11, 0xbd(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(189 as u32) ) } as u64;
	// 8317FC44: 9965006C  stb r11, 0x6c(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(108 as u32), ctx.r[11].u8 ) };
	// 8317FC48: 896400BE  lbz r11, 0xbe(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(190 as u32) ) } as u64;
	// 8317FC4C: 9965006D  stb r11, 0x6d(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(109 as u32), ctx.r[11].u8 ) };
	// 8317FC50: 896400BF  lbz r11, 0xbf(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(191 as u32) ) } as u64;
	// 8317FC54: 9965006E  stb r11, 0x6e(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(110 as u32), ctx.r[11].u8 ) };
	// 8317FC58: 896400C1  lbz r11, 0xc1(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(193 as u32) ) } as u64;
	// 8317FC5C: 9965006F  stb r11, 0x6f(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(111 as u32), ctx.r[11].u8 ) };
	// 8317FC60: 896400C2  lbz r11, 0xc2(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(194 as u32) ) } as u64;
	// 8317FC64: 99650070  stb r11, 0x70(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(112 as u32), ctx.r[11].u8 ) };
	// 8317FC68: 896400C3  lbz r11, 0xc3(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(195 as u32) ) } as u64;
	// 8317FC6C: 99650071  stb r11, 0x71(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(113 as u32), ctx.r[11].u8 ) };
	// 8317FC70: 896400C4  lbz r11, 0xc4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(196 as u32) ) } as u64;
	// 8317FC74: 99650072  stb r11, 0x72(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(114 as u32), ctx.r[11].u8 ) };
	// 8317FC78: 896400C5  lbz r11, 0xc5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(197 as u32) ) } as u64;
	// 8317FC7C: 99650073  stb r11, 0x73(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(115 as u32), ctx.r[11].u8 ) };
	// 8317FC80: 896400C6  lbz r11, 0xc6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(198 as u32) ) } as u64;
	// 8317FC84: 99650074  stb r11, 0x74(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(116 as u32), ctx.r[11].u8 ) };
	// 8317FC88: 896400C7  lbz r11, 0xc7(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(199 as u32) ) } as u64;
	// 8317FC8C: 99650075  stb r11, 0x75(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(117 as u32), ctx.r[11].u8 ) };
	// 8317FC90: 896400C8  lbz r11, 0xc8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(200 as u32) ) } as u64;
	// 8317FC94: 99650076  stb r11, 0x76(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(118 as u32), ctx.r[11].u8 ) };
	// 8317FC98: 896400C9  lbz r11, 0xc9(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(201 as u32) ) } as u64;
	// 8317FC9C: 99650077  stb r11, 0x77(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(119 as u32), ctx.r[11].u8 ) };
	// 8317FCA0: 896400CA  lbz r11, 0xca(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(202 as u32) ) } as u64;
	// 8317FCA4: 99650078  stb r11, 0x78(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(120 as u32), ctx.r[11].u8 ) };
	// 8317FCA8: 896400CB  lbz r11, 0xcb(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(203 as u32) ) } as u64;
	// 8317FCAC: 99650079  stb r11, 0x79(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(121 as u32), ctx.r[11].u8 ) };
	// 8317FCB0: 896400CC  lbz r11, 0xcc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(204 as u32) ) } as u64;
	// 8317FCB4: 9965007A  stb r11, 0x7a(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(122 as u32), ctx.r[11].u8 ) };
	// 8317FCB8: A16400FC  lhz r11, 0xfc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(252 as u32) ) } as u64;
	// 8317FCBC: B165007C  sth r11, 0x7c(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(124 as u32), ctx.r[11].u16 ) };
	// 8317FCC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317FCC8 size=4
    let mut pc: u32 = 0x8317FCC8;
    'dispatch: loop {
        match pc {
            0x8317FCC8 => {
    //   block [0x8317FCC8..0x8317FCCC)
	// 8317FCC8: 480088B8  b 0x83188580
	sub_83188580(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317FCD0 size=20
    let mut pc: u32 = 0x8317FCD0;
    'dispatch: loop {
        match pc {
            0x8317FCD0 => {
    //   block [0x8317FCD0..0x8317FCE4)
	// 8317FCD0: 81631E28  lwz r11, 0x1e28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7720 as u32) ) } as u64;
	// 8317FCD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317FCD8: 409A000C  bne cr6, 0x8317fce4
	if !ctx.cr[6].eq {
		sub_8317FCE4(ctx, base);
		return;
	}
	// 8317FCDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317FCE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FCE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317FCE4 size=20
    let mut pc: u32 = 0x8317FCE4;
    'dispatch: loop {
        match pc {
            0x8317FCE4 => {
    //   block [0x8317FCE4..0x8317FCF8)
	// 8317FCE4: 814317C8  lwz r10, 0x17c8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6088 as u32) ) } as u64;
	// 8317FCE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317FCEC: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8317FCF0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8317FCF4: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317FCF8 size=8
    let mut pc: u32 = 0x8317FCF8;
    'dispatch: loop {
        match pc {
            0x8317FCF8 => {
    //   block [0x8317FCF8..0x8317FD00)
	// 8317FCF8: 386B0AD0  addi r3, r11, 0xad0
	ctx.r[3].s64 = ctx.r[11].s64 + 2768;
	// 8317FCFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317FD00 size=224
    let mut pc: u32 = 0x8317FD00;
    'dispatch: loop {
        match pc {
            0x8317FD00 => {
    //   block [0x8317FD00..0x8317FDE0)
	// 8317FD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317FD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317FD08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317FD0C: 4BFFE77D  bl 0x8317e488
	ctx.lr = 0x8317FD10;
	sub_8317E488(ctx, base);
	// 8317FD10: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317FD14: 419A0008  beq cr6, 0x8317fd1c
	if ctx.cr[6].eq {
	pc = 0x8317FD1C; continue 'dispatch;
	}
	// 8317FD18: 48000000  b 0x8317fd18
	pc = 0x8317FD18; continue 'dispatch;
	// 8317FD1C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317FD20: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 8317FD24: 388BA5C8  addi r4, r11, -0x5a38
	ctx.r[4].s64 = ctx.r[11].s64 + -23096;
	// 8317FD28: 48010421  bl 0x83190148
	ctx.lr = 0x8317FD2C;
	sub_83190148(ctx, base);
	// 8317FD2C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317FD30: 419A0050  beq cr6, 0x8317fd80
	if ctx.cr[6].eq {
	pc = 0x8317FD80; continue 'dispatch;
	}
	// 8317FD34: 3D60FF03  lis r11, -0xfd
	ctx.r[11].s64 = -16580608;
	// 8317FD38: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317FD3C: 616BFF05  ori r11, r11, 0xff05
	ctx.r[11].u64 = ctx.r[11].u64 | 65285;
	// 8317FD40: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317FD44: 409A0020  bne cr6, 0x8317fd64
	if !ctx.cr[6].eq {
	pc = 0x8317FD64; continue 'dispatch;
	}
	// 8317FD48: 60840F13  ori r4, r4, 0xf13
	ctx.r[4].u64 = ctx.r[4].u64 | 3859;
	// 8317FD4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317FD50: 480077A9  bl 0x831874f8
	ctx.lr = 0x8317FD54;
	sub_831874F8(ctx, base);
	// 8317FD54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317FD58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317FD5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317FD60: 4E800020  blr
	return;
	// 8317FD64: 60840F01  ori r4, r4, 0xf01
	ctx.r[4].u64 = ctx.r[4].u64 | 3841;
	// 8317FD68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317FD6C: 4800778D  bl 0x831874f8
	ctx.lr = 0x8317FD70;
	sub_831874F8(ctx, base);
	// 8317FD70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317FD74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317FD78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317FD7C: 4E800020  blr
	return;
	// 8317FD80: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 8317FD84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8317FD88: 388BFAF8  addi r4, r11, -0x508
	ctx.r[4].s64 = ctx.r[11].s64 + -1288;
	// 8317FD8C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317FD90: 48013649  bl 0x831933d8
	ctx.lr = 0x8317FD94;
	sub_831933D8(ctx, base);
	// 8317FD94: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317FD98: 419A0024  beq cr6, 0x8317fdbc
	if ctx.cr[6].eq {
	pc = 0x8317FDBC; continue 'dispatch;
	}
	// 8317FD9C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317FDA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317FDA4: 60840F0B  ori r4, r4, 0xf0b
	ctx.r[4].u64 = ctx.r[4].u64 | 3851;
	// 8317FDA8: 48007751  bl 0x831874f8
	ctx.lr = 0x8317FDAC;
	sub_831874F8(ctx, base);
	// 8317FDAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317FDB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317FDB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317FDB8: 4E800020  blr
	return;
	// 8317FDBC: 48007B9D  bl 0x83187958
	ctx.lr = 0x8317FDC0;
	sub_83187958(ctx, base);
	// 8317FDC0: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 8317FDC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317FDC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317FDCC: 916AA350  stw r11, -0x5cb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-23728 as u32), ctx.r[11].u32 ) };
	// 8317FDD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317FDD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317FDD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317FDDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317FDE0 size=204
    let mut pc: u32 = 0x8317FDE0;
    'dispatch: loop {
        match pc {
            0x8317FDE0 => {
    //   block [0x8317FDE0..0x8317FEAC)
	// 8317FDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317FDE4: 48028389  bl 0x831a816c
	ctx.lr = 0x8317FDE8;
	sub_831A8130(ctx, base);
	// 8317FDE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317FDEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317FDF0: 83DF17C8  lwz r30, 0x17c8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6088 as u32) ) } as u64;
	// 8317FDF4: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317FDF8: 4BFFEDD9  bl 0x8317ebd0
	ctx.lr = 0x8317FDFC;
	sub_8317EBD0(ctx, base);
	// 8317FDFC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317FE00: 409A0010  bne cr6, 0x8317fe10
	if !ctx.cr[6].eq {
	pc = 0x8317FE10; continue 'dispatch;
	}
	// 8317FE04: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317FE08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317FE0C: 480283B0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317FE10: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 8317FE14: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317FE18: 419A0010  beq cr6, 0x8317fe28
	if ctx.cr[6].eq {
	pc = 0x8317FE28; continue 'dispatch;
	}
	// 8317FE1C: 817F00F8  lwz r11, 0xf8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(248 as u32) ) } as u64;
	// 8317FE20: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317FE24: 419AFFE0  beq cr6, 0x8317fe04
	if ctx.cr[6].eq {
	pc = 0x8317FE04; continue 'dispatch;
	}
	// 8317FE28: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8317FE2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317FE30: 48012B01  bl 0x83192930
	ctx.lr = 0x8317FE34;
	sub_83192930(ctx, base);
	// 8317FE34: 3D600003  lis r11, 3
	ctx.r[11].s64 = 196608;
	// 8317FE38: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317FE3C: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 8317FE40: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317FE44: 419AFFC0  beq cr6, 0x8317fe04
	if ctx.cr[6].eq {
	pc = 0x8317FE04; continue 'dispatch;
	}
	// 8317FE48: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8317FE4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317FE50: 4800DFD1  bl 0x8318de20
	ctx.lr = 0x8317FE54;
	sub_8318DE20(ctx, base);
	// 8317FE54: 817E009C  lwz r11, 0x9c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) } as u64;
	// 8317FE58: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317FE5C: 4098FFA8  bge cr6, 0x8317fe04
	if !ctx.cr[6].lt {
	pc = 0x8317FE04; continue 'dispatch;
	}
	// 8317FE60: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8317FE64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317FE68: 4800C001  bl 0x8318be68
	ctx.lr = 0x8317FE6C;
	sub_8318BE68(ctx, base);
	// 8317FE6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317FE70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317FE74: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8317FE78: 557EDFFE  rlwinm r30, r11, 0x1b, 0x1f, 0x1f
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317FE7C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8317FE80: 4800DFA1  bl 0x8318de20
	ctx.lr = 0x8317FE84;
	sub_8318DE20(ctx, base);
	// 8317FE84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8317FE88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8317FE8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317FE90: 4800DF71  bl 0x8318de00
	ctx.lr = 0x8317FE94;
	sub_8318DE00(ctx, base);
	// 8317FE94: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8317FE98: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317FE9C: 40980008  bge cr6, 0x8317fea4
	if !ctx.cr[6].lt {
	pc = 0x8317FEA4; continue 'dispatch;
	}
	// 8317FEA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317FEA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317FEA8: 48028314  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317FEB0 size=112
    let mut pc: u32 = 0x8317FEB0;
    'dispatch: loop {
        match pc {
            0x8317FEB0 => {
    //   block [0x8317FEB0..0x8317FF20)
	// 8317FEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317FEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317FEB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317FEBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317FEC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317FEC4: 48007F1D  bl 0x83187de0
	ctx.lr = 0x8317FEC8;
	sub_83187DE0(ctx, base);
	// 8317FEC8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8317FECC: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 8317FED0: 419A0014  beq cr6, 0x8317fee4
	if ctx.cr[6].eq {
	pc = 0x8317FEE4; continue 'dispatch;
	}
	// 8317FED4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317FED8: 4BFFE761  bl 0x8317e638
	ctx.lr = 0x8317FEDC;
	sub_8317E638(ctx, base);
	// 8317FEDC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317FEE0: 419A002C  beq cr6, 0x8317ff0c
	if ctx.cr[6].eq {
	pc = 0x8317FF0C; continue 'dispatch;
	}
	// 8317FEE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8317FEE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317FEEC: 4BFFE7CD  bl 0x8317e6b8
	ctx.lr = 0x8317FEF0;
	sub_8317E6B8(ctx, base);
	// 8317FEF0: 817F0950  lwz r11, 0x950(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2384 as u32) ) } as u64;
	// 8317FEF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317FEF8: 409A0014  bne cr6, 0x8317ff0c
	if !ctx.cr[6].eq {
	pc = 0x8317FF0C; continue 'dispatch;
	}
	// 8317FEFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8317FF00: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8317FF04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317FF08: 4BFF9551  bl 0x83179458
	ctx.lr = 0x8317FF0C;
	sub_83179458(ctx, base);
	// 8317FF0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317FF10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317FF14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317FF18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317FF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317FF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317FF20 size=864
    let mut pc: u32 = 0x8317FF20;
    'dispatch: loop {
        match pc {
            0x8317FF20 => {
    //   block [0x8317FF20..0x83180210)
	// 8317FF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317FF24: 48028245  bl 0x831a8168
	ctx.lr = 0x8317FF28;
	sub_831A8130(ctx, base);
	// 8317FF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317FF2C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8317FF30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8317FF34: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8317FF38: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8317FF3C: 419A0300  beq cr6, 0x8318023c
	if ctx.cr[6].eq {
	pc = 0x8318023C; continue 'dispatch;
	}
	// 8317FF40: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317FF44: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8317FF48: 419A02F4  beq cr6, 0x8318023c
	if ctx.cr[6].eq {
	pc = 0x8318023C; continue 'dispatch;
	}
	// 8317FF4C: 40990010  ble cr6, 0x8317ff5c
	if !ctx.cr[6].gt {
	pc = 0x8317FF5C; continue 'dispatch;
	}
	// 8317FF50: 7D4BF850  subf r10, r11, r31
	ctx.r[10].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 8317FF54: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 8317FF58: 409902E4  ble cr6, 0x8318023c
	if !ctx.cr[6].gt {
	pc = 0x8318023C; continue 'dispatch;
	}
	// 8317FF5C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8317FF60: 4198005C  blt cr6, 0x8317ffbc
	if ctx.cr[6].lt {
	pc = 0x8317FFBC; continue 'dispatch;
	}
	// 8317FF64: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317FF68: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8317FF6C: 7F1F4840  cmplw cr6, r31, r9
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8317FF70: 4098004C  bge cr6, 0x8317ffbc
	if !ctx.cr[6].lt {
	pc = 0x8317FFBC; continue 'dispatch;
	}
	// 8317FF74: 7D4AF850  subf r10, r10, r31
	ctx.r[10].s64 = ctx.r[31].s64 - ctx.r[10].s64;
	// 8317FF78: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8317FF7C: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 8317FF80: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8317FF84: 40990064  ble cr6, 0x8317ffe8
	if !ctx.cr[6].gt {
	pc = 0x8317FFE8; continue 'dispatch;
	}
	// 8317FF88: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317FF8C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317FF90: 419902AC  bgt cr6, 0x8318023c
	if ctx.cr[6].gt {
	pc = 0x8318023C; continue 'dispatch;
	}
	// 8317FF94: 20BE0004  subfic r5, r30, 4
	ctx.xer.ca = ctx.r[30].u32 <= 4 as u32;
	ctx.r[5].s64 = (4 as i64) - ctx.r[30].s64;
	// 8317FF98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8317FF9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8317FFA0: 48028571  bl 0x831a8510
	ctx.lr = 0x8317FFA4;
	sub_831A8510(ctx, base);
	// 8317FFA4: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 8317FFA8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8317FFAC: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317FFB0: 7C7E5850  subf r3, r30, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 8317FFB4: 4802855D  bl 0x831a8510
	ctx.lr = 0x8317FFB8;
	sub_831A8510(ctx, base);
	// 8317FFB8: 48000038  b 0x8317fff0
	pc = 0x8317FFF0; continue 'dispatch;
	// 8317FFBC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317FFC0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8317FFC4: 41980278  blt cr6, 0x8318023c
	if ctx.cr[6].lt {
	pc = 0x8318023C; continue 'dispatch;
	}
	// 8317FFC8: 815D000C  lwz r10, 0xc(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317FFCC: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317FFD0: 7F1F4840  cmplw cr6, r31, r9
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8317FFD4: 40980268  bge cr6, 0x8318023c
	if !ctx.cr[6].lt {
	pc = 0x8318023C; continue 'dispatch;
	}
	// 8317FFD8: 7D6BF850  subf r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 8317FFDC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8317FFE0: 356B0004  addic. r11, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317FFE4: 41810258  bgt 0x8318023c
	if ctx.cr[0].gt {
	pc = 0x8318023C; continue 'dispatch;
	}
	// 8317FFE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317FFEC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8317FFF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8317FFF4: 48012735  bl 0x83192728
	ctx.lr = 0x8317FFF8;
	sub_83192728(ctx, base);
	// 8317FFF8: 3963FFFC  addi r11, r3, -4
	ctx.r[11].s64 = ctx.r[3].s64 + -4;
	// 8317FFFC: 2B0B007C  cmplwi cr6, r11, 0x7c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 124 as u32, &mut ctx.xer);
	// 83180000: 4199023C  bgt cr6, 0x8318023c
	if ctx.cr[6].gt {
	pc = 0x8318023C; continue 'dispatch;
	}
	// 83180004: 3D808318  lis r12, -0x7ce8
	ctx.r[12].s64 = -2095579136;
	// 83180008: 398C001C  addi r12, r12, 0x1c
	ctx.r[12].s64 = ctx.r[12].s64 + 28;
	// 8318000C: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 83180010: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83180014: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 83180018: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x83180248; continue 'dispatch;
		},
		1 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		2 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		3 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		4 => {
	pc = 0x83180210; continue 'dispatch;
		},
		5 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		6 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		7 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		8 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		9 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		10 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		11 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		12 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		13 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		14 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		15 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		16 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		17 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		18 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		19 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		20 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		21 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		22 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		23 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		24 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		25 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		26 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		27 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		28 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		29 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		30 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		31 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		32 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		33 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		34 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		35 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		36 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		37 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		38 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		39 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		40 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		41 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		42 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		43 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		44 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		45 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		46 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		47 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		48 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		49 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		50 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		51 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		52 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		53 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		54 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		55 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		56 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		57 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		58 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		59 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		60 => {
	pc = 0x83180274; continue 'dispatch;
		},
		61 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		62 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		63 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		64 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		65 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		66 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		67 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		68 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		69 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		70 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		71 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		72 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		73 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		74 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		75 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		76 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		77 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		78 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		79 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		80 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		81 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		82 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		83 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		84 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		85 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		86 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		87 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		88 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		89 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		90 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		91 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		92 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		93 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		94 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		95 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		96 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		97 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		98 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		99 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		100 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		101 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		102 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		103 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		104 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		105 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		106 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		107 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		108 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		109 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		110 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		111 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		112 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		113 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		114 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		115 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		116 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		117 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		118 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		119 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		120 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		121 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		122 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		123 => {
	pc = 0x8318023C; continue 'dispatch;
		},
		124 => {
	pc = 0x83180274; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 8318001C: 83180248  lwz r24, 0x248(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(584 as u32) ) } as u64;
	// 83180020: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180024: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180028: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 8318002C: 83180210  lwz r24, 0x210(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(528 as u32) ) } as u64;
	// 83180030: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180034: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180038: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 8318003C: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180040: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180044: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180048: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 8318004C: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180050: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180054: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180058: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 8318005C: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180060: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180064: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180068: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 8318006C: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180070: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180074: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180078: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 8318007C: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180080: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180084: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180088: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 8318008C: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180090: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180094: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180098: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 8318009C: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800A0: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800A4: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800A8: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800AC: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800B0: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800B4: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800B8: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800BC: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800C0: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800C4: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800C8: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800CC: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800D0: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800D4: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800D8: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800DC: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800E0: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800E4: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800E8: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800EC: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800F0: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800F4: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800F8: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831800FC: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180100: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180104: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180108: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 8318010C: 83180274  lwz r24, 0x274(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(628 as u32) ) } as u64;
	// 83180110: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180114: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180118: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 8318011C: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180120: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180124: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180128: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 8318012C: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180130: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180134: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180138: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 8318013C: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180140: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180144: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180148: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 8318014C: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180150: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180154: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180158: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 8318015C: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180160: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180164: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180168: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 8318016C: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180170: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180174: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180178: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 8318017C: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180180: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180184: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180188: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 8318018C: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180190: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180194: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180198: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 8318019C: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801A0: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801A4: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801A8: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801AC: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801B0: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801B4: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801B8: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801BC: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801C0: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801C4: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801C8: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801CC: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801D0: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801D4: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801D8: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801DC: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801E0: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801E4: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801E8: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801EC: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801F0: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801F4: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801F8: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 831801FC: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180200: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180204: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 83180208: 8318023C  lwz r24, 0x23c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(572 as u32) ) } as u64;
	// 8318020C: 83180274  lwz r24, 0x274(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(628 as u32) ) } as u64;
            }
            0x83180210 => {
    //   block [0x83180210..0x8318023C)
	// 83180210: 578B0672  rlwinm r11, r28, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	// 83180214: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83180218: 419A005C  beq cr6, 0x83180274
	if ctx.cr[6].eq {
	pc = 0x83180274; continue 'dispatch;
	}
	// 8318021C: 38A1009C  addi r5, r1, 0x9c
	ctx.r[5].s64 = ctx.r[1].s64 + 156;
	// 83180220: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 83180224: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83180228: 4BFFE559  bl 0x8317e780
	ctx.lr = 0x8318022C;
	sub_8317E780(ctx, base);
	// 8318022C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83180230: 419A000C  beq cr6, 0x8318023c
	if ctx.cr[6].eq {
	pc = 0x8318023C; continue 'dispatch;
	}
	// 83180234: 7F03F840  cmplw cr6, r3, r31
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83180238: 409A003C  bne cr6, 0x83180274
	if !ctx.cr[6].eq {
	pc = 0x83180274; continue 'dispatch;
	}
	pc = 0x8318023C; continue 'dispatch;
            }
            0x8318023C => {
    //   block [0x8318023C..0x83180248)
	// 8318023C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83180240: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83180244: 48027F74  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            0x83180248 => {
    //   block [0x83180248..0x83180274)
	// 83180248: 738B0048  andi. r11, r28, 0x48
	ctx.r[11].u64 = ctx.r[28].u64 & 72;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318024C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83180250: 419A0024  beq cr6, 0x83180274
	if ctx.cr[6].eq {
	pc = 0x83180274; continue 'dispatch;
	}
	// 83180254: 38A1009C  addi r5, r1, 0x9c
	ctx.r[5].s64 = ctx.r[1].s64 + 156;
	// 83180258: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8318025C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83180260: 4BFFE521  bl 0x8317e780
	ctx.lr = 0x83180264;
	sub_8317E780(ctx, base);
	// 83180264: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83180268: 419AFFD4  beq cr6, 0x8318023c
	if ctx.cr[6].eq {
	pc = 0x8318023C; continue 'dispatch;
	}
	// 8318026C: 7F03F840  cmplw cr6, r3, r31
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83180270: 419AFFCC  beq cr6, 0x8318023c
	if ctx.cr[6].eq {
	pc = 0x8318023C; continue 'dispatch;
	}
	pc = 0x83180274; continue 'dispatch;
            }
            0x83180274 => {
    //   block [0x83180274..0x83180280)
	// 83180274: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83180278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318027C: 48027F3C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83180280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83180280 size=128
    let mut pc: u32 = 0x83180280;
    'dispatch: loop {
        match pc {
            0x83180280 => {
    //   block [0x83180280..0x83180300)
	// 83180280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83180284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83180288: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318028C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83180290: 3BE30D88  addi r31, r3, 0xd88
	ctx.r[31].s64 = ctx.r[3].s64 + 3464;
	// 83180294: 397F0068  addi r11, r31, 0x68
	ctx.r[11].s64 = ctx.r[31].s64 + 104;
	// 83180298: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318029C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831802A0: 409A001C  bne cr6, 0x831802bc
	if !ctx.cr[6].eq {
	pc = 0x831802BC; continue 'dispatch;
	}
	// 831802A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831802A8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831802AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831802B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831802B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831802B8: 4E800020  blr
	return;
	// 831802BC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 831802C0: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 831802C4: 4BFFECD5  bl 0x8317ef98
	ctx.lr = 0x831802C8;
	sub_8317EF98(ctx, base);
	// 831802C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831802CC: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 831802D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831802D4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 831802D8: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 831802DC: 4BFFB6E5  bl 0x8317b9c0
	ctx.lr = 0x831802E0;
	sub_8317B9C0(ctx, base);
	// 831802E0: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 831802E4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831802E8: 7C6B5050  subf r3, r11, r10
	ctx.r[3].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831802EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831802F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831802F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831802F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831802FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83180300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83180300 size=224
    let mut pc: u32 = 0x83180300;
    'dispatch: loop {
        match pc {
            0x83180300 => {
    //   block [0x83180300..0x831803E0)
	// 83180300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83180304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83180308: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318030C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83180310: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83180314: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83180318: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8318031C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83180320: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83180324: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83180328: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318032C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83180330: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83180334: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83180338: 4E800421  bctrl
	ctx.lr = 0x8318033C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318033C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83180340: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83180344: 409A0068  bne cr6, 0x831803ac
	if !ctx.cr[6].eq {
	pc = 0x831803AC; continue 'dispatch;
	}
	// 83180348: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8318034C: 480123DD  bl 0x83192728
	ctx.lr = 0x83180350;
	sub_83192728(ctx, base);
	// 83180350: 2F030080  cmpwi cr6, r3, 0x80
	ctx.cr[6].compare_i32(ctx.r[3].s32, 128, &mut ctx.xer);
	// 83180354: 409A0058  bne cr6, 0x831803ac
	if !ctx.cr[6].eq {
	pc = 0x831803AC; continue 'dispatch;
	}
	// 83180358: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318035C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83180360: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83180364: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83180368: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8318036C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83180370: 4E800421  bctrl
	ctx.lr = 0x83180374;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83180374: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83180378: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318037C: 4BFFE805  bl 0x8317eb80
	ctx.lr = 0x83180380;
	sub_8317EB80(ctx, base);
	// 83180380: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83180384: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83180388: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8318038C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83180390: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83180394: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83180398: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318039C: 4E800421  bctrl
	ctx.lr = 0x831803A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831803A0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831803A4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 831803A8: 419AFFA0  beq cr6, 0x83180348
	if ctx.cr[6].eq {
	pc = 0x83180348; continue 'dispatch;
	}
	// 831803AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831803B0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831803B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831803B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831803BC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831803C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831803C4: 4E800421  bctrl
	ctx.lr = 0x831803C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831803C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831803CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831803D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831803D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831803D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831803DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831803E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831803E0 size=84
    let mut pc: u32 = 0x831803E0;
    'dispatch: loop {
        match pc {
            0x831803E0 => {
    //   block [0x831803E0..0x83180434)
	// 831803E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831803E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831803E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831803EC: 2F050080  cmpwi cr6, r5, 0x80
	ctx.cr[6].compare_i32(ctx.r[5].s32, 128, &mut ctx.xer);
	// 831803F0: 409A0018  bne cr6, 0x83180408
	if !ctx.cr[6].eq {
	pc = 0x83180408; continue 'dispatch;
	}
	// 831803F4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831803F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831803FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83180400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83180404: 4E800020  blr
	return;
	// 83180408: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 8318040C: 41990014  bgt cr6, 0x83180420
	if ctx.cr[6].gt {
	pc = 0x83180420; continue 'dispatch;
	}
	// 83180410: 4BFFE7C1  bl 0x8317ebd0
	ctx.lr = 0x83180414;
	sub_8317EBD0(ctx, base);
	// 83180414: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83180418: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8318041C: 419A0008  beq cr6, 0x83180424
	if ctx.cr[6].eq {
	pc = 0x83180424; continue 'dispatch;
	}
	// 83180420: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83180424: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83180428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318042C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83180430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83180438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83180438 size=360
    let mut pc: u32 = 0x83180438;
    'dispatch: loop {
        match pc {
            0x83180438 => {
    //   block [0x83180438..0x831805A0)
	// 83180438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318043C: 48027D25  bl 0x831a8160
	ctx.lr = 0x83180440;
	sub_831A8130(ctx, base);
	// 83180440: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83180444: 81450010  lwz r10, 0x10(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 83180448: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8318044C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83180450: 83A50014  lwz r29, 0x14(r5)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) } as u64;
	// 83180454: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83180458: 396BACF0  addi r11, r11, -0x5310
	ctx.r[11].s64 = ctx.r[11].s64 + -21264;
	// 8318045C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83180460: E93E0150  ld r9, 0x150(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(336 as u32) ) };
	// 83180464: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83180468: 2F290000  cmpdi cr6, r9, 0
	ctx.cr[6].compare_i64(ctx.r[9].s64, 0, &mut ctx.xer);
	// 8318046C: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83180470: 40980024  bge cr6, 0x83180494
	if !ctx.cr[6].lt {
	pc = 0x83180494; continue 'dispatch;
	}
	// 83180474: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83180478: 4BFFE7D9  bl 0x8317ec50
	ctx.lr = 0x8318047C;
	sub_8317EC50(ctx, base);
	// 8318047C: E9660000  ld r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 83180480: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 83180484: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 83180488: 41990008  bgt cr6, 0x83180490
	if ctx.cr[6].gt {
	pc = 0x83180490; continue 'dispatch;
	}
	// 8318048C: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 83180490: F97E0150  std r11, 0x150(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(336 as u32), ctx.r[11].u64 ) };
	// 83180494: E9660000  ld r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 83180498: E95E0150  ld r10, 0x150(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(336 as u32) ) };
	// 8318049C: 7F8A5850  subf r28, r10, r11
	ctx.r[28].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831804A0: 2F3C0000  cmpdi cr6, r28, 0
	ctx.cr[6].compare_i64(ctx.r[28].s64, 0, &mut ctx.xer);
	// 831804A4: 41990008  bgt cr6, 0x831804ac
	if ctx.cr[6].gt {
	pc = 0x831804AC; continue 'dispatch;
	}
	// 831804A8: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 831804AC: 393F0010  addi r9, r31, 0x10
	ctx.r[9].s64 = ctx.r[31].s64 + 16;
	// 831804B0: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 831804B4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 831804B8: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 831804BC: 886B0000  lbz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831804C0: 8B4A0000  lbz r26, 0(r10)
	ctx.r[26].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831804C4: 7C7A1851  subf. r3, r26, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[26].s64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831804C8: 40820014  bne 0x831804dc
	if !ctx.cr[0].eq {
	pc = 0x831804DC; continue 'dispatch;
	}
	// 831804CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831804D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831804D4: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 831804D8: 409AFFE4  bne cr6, 0x831804bc
	if !ctx.cr[6].eq {
	pc = 0x831804BC; continue 'dispatch;
	}
	// 831804DC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831804E0: 419A005C  beq cr6, 0x8318053c
	if ctx.cr[6].eq {
	pc = 0x8318053C; continue 'dispatch;
	}
	// 831804E4: E9660000  ld r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 831804E8: F9690000  std r11, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 831804EC: E9660008  ld r11, 8(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 831804F0: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 831804F4: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 831804F8: F9690008  std r11, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 831804FC: 81650018  lwz r11, 0x18(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) } as u64;
	// 83180500: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83180504: 409A0020  bne cr6, 0x83180524
	if !ctx.cr[6].eq {
	pc = 0x83180524; continue 'dispatch;
	}
	// 83180508: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8318050C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83180510: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83180514: E9660000  ld r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 83180518: F9680000  std r11, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8318051C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83180520: 48027C90  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83180524: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 83180528: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8318052C: E9660000  ld r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 83180530: F9680000  std r11, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 83180534: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83180538: 48027C78  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 8318053C: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83180540: 419A0020  beq cr6, 0x83180560
	if ctx.cr[6].eq {
	pc = 0x83180560; continue 'dispatch;
	}
	// 83180544: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83180548: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318054C: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 83180550: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83180554: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 83180558: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318055C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83180560: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83180564: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83180568: 7D6BE850  subf r11, r11, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 8318056C: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83180570: 41990008  bgt cr6, 0x83180578
	if ctx.cr[6].gt {
	pc = 0x83180578; continue 'dispatch;
	}
	// 83180574: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 83180578: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318057C: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83180580: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83180584: 4BFFE6CD  bl 0x8317ec50
	ctx.lr = 0x83180588;
	sub_8317EC50(ctx, base);
	// 83180588: 7C63E214  add r3, r3, r28
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[28].u64;
	// 8318058C: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 83180590: 41990008  bgt cr6, 0x83180598
	if ctx.cr[6].gt {
	pc = 0x83180598; continue 'dispatch;
	}
	// 83180594: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83180598: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8318059C: 48027C14  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831805A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831805A0 size=508
    let mut pc: u32 = 0x831805A0;
    'dispatch: loop {
        match pc {
            0x831805A0 => {
    //   block [0x831805A0..0x8318079C)
	// 831805A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831805A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831805A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831805AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831805B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831805B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831805B8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 831805BC: 38DE0D88  addi r6, r30, 0xd88
	ctx.r[6].s64 = ctx.r[30].s64 + 3464;
	// 831805C0: 3886001C  addi r4, r6, 0x1c
	ctx.r[4].s64 = ctx.r[6].s64 + 28;
	// 831805C4: 83FE17C8  lwz r31, 0x17c8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(6088 as u32) ) } as u64;
	// 831805C8: 390604D0  addi r8, r6, 0x4d0
	ctx.r[8].s64 = ctx.r[6].s64 + 1232;
	// 831805CC: 4BFFEC85  bl 0x8317f250
	ctx.lr = 0x831805D0;
	sub_8317F250(ctx, base);
	// 831805D0: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 831805D4: 419A002C  beq cr6, 0x83180600
	if ctx.cr[6].eq {
	pc = 0x83180600; continue 'dispatch;
	}
	// 831805D8: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 831805DC: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 831805E0: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 831805E4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831805E8: B0EA0000  sth r7, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 831805EC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 831805F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831805F4: 409AFFF0  bne cr6, 0x831805e4
	if !ctx.cr[6].eq {
	pc = 0x831805E4; continue 'dispatch;
	}
	// 831805F8: B0E80002  sth r7, 2(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[7].u16 ) };
	// 831805FC: 48000068  b 0x83180664
	pc = 0x83180664; continue 'dispatch;
	// 83180600: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83180604: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83180608: 419A000C  beq cr6, 0x83180614
	if ctx.cr[6].eq {
	pc = 0x83180614; continue 'dispatch;
	}
	// 8318060C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83180610: 409A0054  bne cr6, 0x83180664
	if !ctx.cr[6].eq {
	pc = 0x83180664; continue 'dispatch;
	}
	// 83180614: 817F00EC  lwz r11, 0xec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83180618: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8318061C: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 83180620: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83180624: 40980008  bge cr6, 0x8318062c
	if !ctx.cr[6].lt {
	pc = 0x8318062C; continue 'dispatch;
	}
	// 83180628: 39290400  addi r9, r9, 0x400
	ctx.r[9].s64 = ctx.r[9].s64 + 1024;
	// 8318062C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83180630: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83180634: 40980030  bge cr6, 0x83180664
	if !ctx.cr[6].lt {
	pc = 0x83180664; continue 'dispatch;
	}
	// 83180638: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 8318063C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83180640: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83180644: 7D443670  srawi r4, r10, 6
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[10].s32 >> 6) as i64;
	// 83180648: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8318064C: 7C840194  addze r4, r4
	tmp.s64 = ctx.r[4].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[4].u32);
	ctx.r[4].s64 = tmp.s64;
	// 83180650: 54843032  slwi r4, r4, 6
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shl(6);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83180654: 7D445050  subf r10, r4, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[4].s64;
	// 83180658: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318065C: 7CEA432E  sthx r7, r10, r8
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[7].u16) };
	// 83180660: 4198FFDC  blt cr6, 0x8318063c
	if ctx.cr[6].lt {
	pc = 0x8318063C; continue 'dispatch;
	}
	// 83180664: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83180668: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8318066C: A1260038  lhz r9, 0x38(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(56 as u32) ) } as u64;
	// 83180670: 7D6A3670  srawi r10, r11, 6
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 6) as i64;
	// 83180674: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 83180678: 554A3032  slwi r10, r10, 6
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318067C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83180680: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83180684: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 83180688: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 8318068C: 419A000C  beq cr6, 0x83180698
	if ctx.cr[6].eq {
	pc = 0x83180698; continue 'dispatch;
	}
	// 83180690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83180694: 48000078  b 0x8318070c
	pc = 0x8318070C; continue 'dispatch;
	// 83180698: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8318069C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831806A0: 409A001C  bne cr6, 0x831806bc
	if !ctx.cr[6].eq {
	pc = 0x831806BC; continue 'dispatch;
	}
	// 831806A4: A1280002  lhz r9, 2(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(2 as u32) ) } as u64;
	// 831806A8: 2B09FFFF  cmplwi cr6, r9, 0xffff
	ctx.cr[6].compare_u32(ctx.r[9].u32, 65535 as u32, &mut ctx.xer);
	// 831806AC: 409A0010  bne cr6, 0x831806bc
	if !ctx.cr[6].eq {
	pc = 0x831806BC; continue 'dispatch;
	}
	// 831806B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831806B4: B1680002  sth r11, 2(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 831806B8: 48000058  b 0x83180710
	pc = 0x83180710; continue 'dispatch;
	// 831806BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831806C0: 396B003F  addi r11, r11, 0x3f
	ctx.r[11].s64 = ctx.r[11].s64 + 63;
	// 831806C4: 7D693670  srawi r9, r11, 6
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 6) as i64;
	// 831806C8: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 831806CC: 55293032  slwi r9, r9, 6
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(6);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831806D0: 7D295850  subf r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 831806D4: 5525103A  slwi r5, r9, 2
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831806D8: 7CA5422E  lhzx r5, r5, r8
	ctx.r[5].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 831806DC: 2B05FFFF  cmplwi cr6, r5, 0xffff
	ctx.cr[6].compare_u32(ctx.r[5].u32, 65535 as u32, &mut ctx.xer);
	// 831806E0: 409A0018  bne cr6, 0x831806f8
	if !ctx.cr[6].eq {
	pc = 0x831806F8; continue 'dispatch;
	}
	// 831806E4: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 831806E8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831806EC: 2F070040  cmpwi cr6, r7, 0x40
	ctx.cr[6].compare_i32(ctx.r[7].s32, 64, &mut ctx.xer);
	// 831806F0: 4198FFD4  blt cr6, 0x831806c4
	if ctx.cr[6].lt {
	pc = 0x831806C4; continue 'dispatch;
	}
	// 831806F4: 4800001C  b 0x83180710
	pc = 0x83180710; continue 'dispatch;
	// 831806F8: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831806FC: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 83180700: A12B0002  lhz r9, 2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83180704: A0EB0000  lhz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83180708: 7D693A14  add r11, r9, r7
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 8318070C: B16A0002  sth r11, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 83180710: A16A0002  lhz r11, 2(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 83180714: B166003A  sth r11, 0x3a(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(58 as u32), ctx.r[11].u16 ) };
	// 83180718: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8318071C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83180720: 409A0064  bne cr6, 0x83180784
	if !ctx.cr[6].eq {
	pc = 0x83180784; continue 'dispatch;
	}
	// 83180724: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83180728: 7D690734  extsh r9, r11
	ctx.r[9].s64 = ctx.r[11].s16 as i64;
	// 8318072C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83180730: 419A0054  beq cr6, 0x83180784
	if ctx.cr[6].eq {
	pc = 0x83180784; continue 'dispatch;
	}
	// 83180734: 80FF00EC  lwz r7, 0xec(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83180738: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318073C: A16A0002  lhz r11, 2(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 83180740: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 83180744: 8147007C  lwz r10, 0x7c(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(124 as u32) ) } as u64;
	// 83180748: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8318074C: 7D493670  srawi r9, r10, 6
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[10].s32 >> 6) as i64;
	// 83180750: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 83180754: 55293032  slwi r9, r9, 6
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(6);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83180758: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 8318075C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83180760: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 83180764: B16A0002  sth r11, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 83180768: 815F00EC  lwz r10, 0xec(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 8318076C: B16A003A  sth r11, 0x3a(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(58 as u32), ctx.r[11].u16 ) };
	// 83180770: 809F00EC  lwz r4, 0xec(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83180774: 4BFFE4FD  bl 0x8317ec70
	ctx.lr = 0x83180778;
	sub_8317EC70(ctx, base);
	// 83180778: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318077C: 809F00EC  lwz r4, 0xec(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83180780: 4BFFF249  bl 0x8317f9c8
	ctx.lr = 0x83180784;
	sub_8317F9C8(ctx, base);
	// 83180784: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83180788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318078C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83180790: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83180794: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83180798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831807A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831807A0 size=264
    let mut pc: u32 = 0x831807A0;
    'dispatch: loop {
        match pc {
            0x831807A0 => {
    //   block [0x831807A0..0x831808A8)
	// 831807A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831807A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831807A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831807AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831807B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831807B4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 831807B8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 831807BC: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 831807C0: 3BEA0D88  addi r31, r10, 0xd88
	ctx.r[31].s64 = ctx.r[10].s64 + 3464;
	// 831807C4: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831807C8: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 831807CC: 83CB0014  lwz r30, 0x14(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 831807D0: 419A0020  beq cr6, 0x831807f0
	if ctx.cr[6].eq {
	pc = 0x831807F0; continue 'dispatch;
	}
	// 831807D4: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 831807D8: 41980018  blt cr6, 0x831807f0
	if ctx.cr[6].lt {
	pc = 0x831807F0; continue 'dispatch;
	}
	// 831807DC: 38FF001C  addi r7, r31, 0x1c
	ctx.r[7].s64 = ctx.r[31].s64 + 28;
	// 831807E0: 80AB001C  lwz r5, 0x1c(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831807E4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 831807E8: 4BFFE5D1  bl 0x8317edb8
	ctx.lr = 0x831807EC;
	sub_8317EDB8(ctx, base);
	// 831807EC: 480000A4  b 0x83180890
	pc = 0x83180890; continue 'dispatch;
	// 831807F0: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 831807F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831807F8: 409A002C  bne cr6, 0x83180824
	if !ctx.cr[6].eq {
	pc = 0x83180824; continue 'dispatch;
	}
	// 831807FC: 816A1E28  lwz r11, 0x1e28(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(7720 as u32) ) } as u64;
	// 83180800: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83180804: 409A008C  bne cr6, 0x83180890
	if !ctx.cr[6].eq {
	pc = 0x83180890; continue 'dispatch;
	}
	// 83180808: 909F001C  stw r4, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 8318080C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83180810: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83180814: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83180818: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8318081C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83180820: 48000070  b 0x83180890
	pc = 0x83180890; continue 'dispatch;
	// 83180824: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 83180828: 419A0038  beq cr6, 0x83180860
	if ctx.cr[6].eq {
	pc = 0x83180860; continue 'dispatch;
	}
	// 8318082C: 389F001C  addi r4, r31, 0x1c
	ctx.r[4].s64 = ctx.r[31].s64 + 28;
	// 83180830: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 83180834: 4BFFE765  bl 0x8317ef98
	ctx.lr = 0x83180838;
	sub_8317EF98(ctx, base);
	// 83180838: 7FCA3670  srawi r10, r30, 6
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[30].s32 >> 6) as i64;
	// 8318083C: A17F003A  lhz r11, 0x3a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(58 as u32) ) } as u64;
	// 83180840: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 83180844: 554A3032  slwi r10, r10, 6
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83180848: 7D4AF050  subf r10, r10, r30
	ctx.r[10].s64 = ctx.r[30].s64 - ctx.r[10].s64;
	// 8318084C: B17F04D2  sth r11, 0x4d2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(1234 as u32), ctx.r[11].u16 ) };
	// 83180850: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83180854: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 83180858: B16A04D2  sth r11, 0x4d2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(1234 as u32), ctx.r[11].u16 ) };
	// 8318085C: 48000034  b 0x83180890
	pc = 0x83180890; continue 'dispatch;
	// 83180860: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 83180864: 815F0070  lwz r10, 0x70(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 83180868: 813F0074  lwz r9, 0x74(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 8318086C: 811F0078  lwz r8, 0x78(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83180870: 80FF007C  lwz r7, 0x7c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83180874: 80DF0080  lwz r6, 0x80(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83180878: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8318087C: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 83180880: 913F0024  stw r9, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 83180884: 911F0028  stw r8, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[8].u32 ) };
	// 83180888: 90FF002C  stw r7, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[7].u32 ) };
	// 8318088C: 90DF0030  stw r6, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[6].u32 ) };
	// 83180890: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83180894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83180898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318089C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831808A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831808A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831808A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831808A8 size=164
    let mut pc: u32 = 0x831808A8;
    'dispatch: loop {
        match pc {
            0x831808A8 => {
    //   block [0x831808A8..0x8318094C)
	// 831808A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831808AC: 480278BD  bl 0x831a8168
	ctx.lr = 0x831808B0;
	sub_831A8130(ctx, base);
	// 831808B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831808B4: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 831808B8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831808BC: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 831808C0: 4BFFF411  bl 0x8317fcd0
	ctx.lr = 0x831808C4;
	sub_8317FCD0(ctx, base);
	// 831808C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831808C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831808CC: 419A0078  beq cr6, 0x83180944
	if ctx.cr[6].eq {
	pc = 0x83180944; continue 'dispatch;
	}
	// 831808D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831808D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831808D8: 409A006C  bne cr6, 0x83180944
	if !ctx.cr[6].eq {
	pc = 0x83180944; continue 'dispatch;
	}
	// 831808DC: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 831808E0: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 831808E4: 3B890D88  addi r28, r9, 0xd88
	ctx.r[28].s64 = ctx.r[9].s64 + 3464;
	// 831808E8: 2F0B0200  cmpwi cr6, r11, 0x200
	ctx.cr[6].compare_i32(ctx.r[11].s32, 512, &mut ctx.xer);
	// 831808EC: 41980008  blt cr6, 0x831808f4
	if ctx.cr[6].lt {
	pc = 0x831808F4; continue 'dispatch;
	}
	// 831808F0: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 831808F4: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 831808F8: 91630200  stw r11, 0x200(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(512 as u32), ctx.r[11].u32 ) };
	// 831808FC: 80880000  lwz r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 83180900: 4800E099  bl 0x8318e998
	ctx.lr = 0x83180904;
	sub_8318E998(ctx, base);
	// 83180904: 3D600003  lis r11, 3
	ctx.r[11].s64 = 196608;
	// 83180908: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8318090C: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 83180910: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83180914: 409A0010  bne cr6, 0x83180924
	if !ctx.cr[6].eq {
	pc = 0x83180924; continue 'dispatch;
	}
	// 83180918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318091C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83180920: 4800000C  b 0x8318092c
	pc = 0x8318092C; continue 'dispatch;
	// 83180924: 1D7D0032  mulli r11, r29, 0x32
	ctx.r[11].s64 = ctx.r[29].s64 * 50;
	// 83180928: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8318092C: 389C003C  addi r4, r28, 0x3c
	ctx.r[4].s64 = ctx.r[28].s64 + 60;
	// 83180930: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83180934: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83180938: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 8318093C: 48027BD5  bl 0x831a8510
	ctx.lr = 0x83180940;
	sub_831A8510(ctx, base);
	// 83180940: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83180944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83180948: 48027870  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83180950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83180950 size=340
    let mut pc: u32 = 0x83180950;
    'dispatch: loop {
        match pc {
            0x83180950 => {
    //   block [0x83180950..0x83180AA4)
	// 83180950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83180954: 4802780D  bl 0x831a8160
	ctx.lr = 0x83180958;
	sub_831A8130(ctx, base);
	// 83180958: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318095C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83180960: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83180964: 3BFE0D88  addi r31, r30, 0xd88
	ctx.r[31].s64 = ctx.r[30].s64 + 3464;
	// 83180968: 835E17C8  lwz r26, 0x17c8(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(6088 as u32) ) } as u64;
	// 8318096C: 817F0118  lwz r11, 0x118(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(280 as u32) ) } as u64;
	// 83180970: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83180974: 409A000C  bne cr6, 0x83180980
	if !ctx.cr[6].eq {
	pc = 0x83180980; continue 'dispatch;
	}
	// 83180978: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8318097C: 48000018  b 0x83180994
	pc = 0x83180994; continue 'dispatch;
	// 83180980: 817F0164  lwz r11, 0x164(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) } as u64;
	// 83180984: 813F013C  lwz r9, 0x13c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) } as u64;
	// 83180988: 815F00E4  lwz r10, 0xe4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8318098C: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 83180990: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83180994: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83180998: 839F00E8  lwz r28, 0xe8(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 8318099C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831809A0: 419A0024  beq cr6, 0x831809c4
	if ctx.cr[6].eq {
	pc = 0x831809C4; continue 'dispatch;
	}
	// 831809A4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 831809A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831809AC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 831809B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831809B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831809B8: 4E800421  bctrl
	ctx.lr = 0x831809BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831809BC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831809C0: 480277F0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 831809C4: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 831809C8: 409A0014  bne cr6, 0x831809dc
	if !ctx.cr[6].eq {
	pc = 0x831809DC; continue 'dispatch;
	}
	// 831809CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831809D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831809D4: 4BFFAB1D  bl 0x8317b4f0
	ctx.lr = 0x831809D8;
	sub_8317B4F0(ctx, base);
	// 831809D8: 4800000C  b 0x831809e4
	pc = 0x831809E4; continue 'dispatch;
	// 831809DC: 2F1B0002  cmpwi cr6, r27, 2
	ctx.cr[6].compare_i32(ctx.r[27].s32, 2, &mut ctx.xer);
	// 831809E0: 409A0014  bne cr6, 0x831809f4
	if !ctx.cr[6].eq {
	pc = 0x831809F4; continue 'dispatch;
	}
	// 831809E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831809E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831809EC: 4BFFAB95  bl 0x8317b580
	ctx.lr = 0x831809F0;
	sub_8317B580(ctx, base);
	// 831809F0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831809F4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831809F8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 831809FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83180A00: 4BFFB771  bl 0x8317c170
	ctx.lr = 0x83180A04;
	sub_8317C170(ctx, base);
	// 83180A04: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83180A08: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83180A0C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83180A10: 41990020  bgt cr6, 0x83180a30
	if ctx.cr[6].gt {
	pc = 0x83180A30; continue 'dispatch;
	}
	// 83180A14: 817A000C  lwz r11, 0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 83180A18: 815E0AA4  lwz r10, 0xaa4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2724 as u32) ) } as u64;
	// 83180A1C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83180A20: 41980010  blt cr6, 0x83180a30
	if ctx.cr[6].lt {
	pc = 0x83180A30; continue 'dispatch;
	}
	// 83180A24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83180A28: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83180A2C: 48027784  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83180A30: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 83180A34: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83180A38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83180A3C: 4BFFADED  bl 0x8317b828
	ctx.lr = 0x83180A40;
	sub_8317B828(ctx, base);
	// 83180A40: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83180A44: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83180A48: 4198FFDC  blt cr6, 0x83180a24
	if ctx.cr[6].lt {
	pc = 0x83180A24; continue 'dispatch;
	}
	// 83180A4C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 83180A50: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 83180A54: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83180A58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83180A5C: 4BFFEAED  bl 0x8317f548
	ctx.lr = 0x83180A60;
	sub_8317F548(ctx, base);
	// 83180A60: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83180A64: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83180A68: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83180A6C: 7D6BE1D6  mullw r11, r11, r28
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[28].s32 as i64);
	// 83180A70: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83180A74: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 83180A78: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 83180A7C: 7CABE850  subf r5, r11, r29
	ctx.r[5].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 83180A80: 4800EF89  bl 0x8318fa08
	ctx.lr = 0x83180A84;
	sub_8318FA08(ctx, base);
	// 83180A84: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83180A88: 409AFF9C  bne cr6, 0x83180a24
	if !ctx.cr[6].eq {
	pc = 0x83180A24; continue 'dispatch;
	}
	// 83180A8C: 817A000C  lwz r11, 0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 83180A90: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83180A94: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83180A98: 917A000C  stw r11, 0xc(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83180A9C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83180AA0: 48027710  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83180AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83180AA8 size=196
    let mut pc: u32 = 0x83180AA8;
    'dispatch: loop {
        match pc {
            0x83180AA8 => {
    //   block [0x83180AA8..0x83180B6C)
	// 83180AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83180AAC: 480276B9  bl 0x831a8164
	ctx.lr = 0x83180AB0;
	sub_831A8130(ctx, base);
	// 83180AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83180AB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83180AB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83180ABC: 83BF17C8  lwz r29, 0x17c8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6088 as u32) ) } as u64;
	// 83180AC0: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83180AC4: 4BFFEA9D  bl 0x8317f560
	ctx.lr = 0x83180AC8;
	sub_8317F560(ctx, base);
	// 83180AC8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83180ACC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83180AD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83180AD4: 4BFB3C75  bl 0x83134748
	ctx.lr = 0x83180AD8;
	sub_83134748(ctx, base);
	// 83180AD8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83180ADC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83180AE0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83180AE4: 48012765  bl 0x83193248
	ctx.lr = 0x83180AE8;
	sub_83193248(ctx, base);
	// 83180AE8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83180AEC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83180AF0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83180AF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83180AF8: 4BFB3C51  bl 0x83134748
	ctx.lr = 0x83180AFC;
	sub_83134748(ctx, base);
	// 83180AFC: 7FDB1850  subf r30, r27, r3
	ctx.r[30].s64 = ctx.r[3].s64 - ctx.r[27].s64;
	// 83180B00: 3CC0FF00  lis r6, -0x100
	ctx.r[6].s64 = -16777216;
	// 83180B04: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83180B08: 60C60F07  ori r6, r6, 0xf07
	ctx.r[6].u64 = ctx.r[6].u64 | 3847;
	// 83180B0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83180B10: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83180B14: 4BFFE0C5  bl 0x8317ebd8
	ctx.lr = 0x83180B18;
	sub_8317EBD8(ctx, base);
	// 83180B18: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83180B1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83180B20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83180B24: 4BFFE05D  bl 0x8317eb80
	ctx.lr = 0x83180B28;
	sub_8317EB80(ctx, base);
	// 83180B28: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83180B2C: 419A0010  beq cr6, 0x83180b3c
	if ctx.cr[6].eq {
	pc = 0x83180B3C; continue 'dispatch;
	}
	// 83180B30: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83180B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83180B38: 4802767C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83180B3C: 897D006C  lbz r11, 0x6c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(108 as u32) ) } as u64;
	// 83180B40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83180B44: 409A000C  bne cr6, 0x83180b50
	if !ctx.cr[6].eq {
	pc = 0x83180B50; continue 'dispatch;
	}
	// 83180B48: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83180B4C: 917D00F4  stw r11, 0xf4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(244 as u32), ctx.r[11].u32 ) };
	// 83180B50: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83180B54: 80BD002C  lwz r5, 0x2c(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 83180B58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83180B5C: 480027ED  bl 0x83183348
	ctx.lr = 0x83180B60;
	sub_83183348(ctx, base);
	// 83180B60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83180B64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83180B68: 4802764C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83180B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83180B70 size=80
    let mut pc: u32 = 0x83180B70;
    'dispatch: loop {
        match pc {
            0x83180B70 => {
    //   block [0x83180B70..0x83180BC0)
	// 83180B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83180B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83180B78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83180B7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83180B80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83180B84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83180B88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83180B8C: 389F0E48  addi r4, r31, 0xe48
	ctx.r[4].s64 = ctx.r[31].s64 + 3656;
	// 83180B90: 387E0018  addi r3, r30, 0x18
	ctx.r[3].s64 = ctx.r[30].s64 + 24;
	// 83180B94: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 83180B98: 48027979  bl 0x831a8510
	ctx.lr = 0x83180B9C;
	sub_831A8510(ctx, base);
	// 83180B9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83180BA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83180BA4: 4BFFEE25  bl 0x8317f9c8
	ctx.lr = 0x83180BA8;
	sub_8317F9C8(ctx, base);
	// 83180BA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83180BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83180BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83180BB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83180BB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83180BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


