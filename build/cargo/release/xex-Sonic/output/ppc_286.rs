pub fn sub_831A80A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A80A0 size=88
    let mut pc: u32 = 0x831A80A0;
    'dispatch: loop {
        match pc {
            0x831A80A0 => {
    //   block [0x831A80A0..0x831A80F8)
	// 831A80A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A80A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A80A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831A80AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831A80B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A80B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831A80B8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831A80BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831A80C0: 396BC36C  addi r11, r11, -0x3c94
	ctx.r[11].s64 = ctx.r[11].s64 + -15508;
	// 831A80C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831A80C8: 4800AF21  bl 0x831b2fe8
	ctx.lr = 0x831A80CC;
	sub_831B2FE8(ctx, base);
	// 831A80CC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831A80D0: 4182000C  beq 0x831a80dc
	if ctx.cr[0].eq {
	pc = 0x831A80DC; continue 'dispatch;
	}
	// 831A80D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831A80D8: 4B118191  bl 0x822c0268
	ctx.lr = 0x831A80DC;
	sub_822C0268(ctx, base);
	// 831A80DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831A80E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831A80E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A80E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A80EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831A80F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831A80F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A80F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A80F8 size=56
    let mut pc: u32 = 0x831A80F8;
    'dispatch: loop {
        match pc {
            0x831A80F8 => {
    //   block [0x831A80F8..0x831A8130)
	// 831A80F8: 39430009  addi r10, r3, 9
	ctx.r[10].s64 = ctx.r[3].s64 + 9;
	// 831A80FC: 39640009  addi r11, r4, 9
	ctx.r[11].s64 = ctx.r[4].s64 + 9;
	// 831A8100: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A8104: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A8108: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A810C: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 831A8110: 41820014  beq 0x831a8124
	if ctx.cr[0].eq {
	pc = 0x831A8124; continue 'dispatch;
	}
	// 831A8114: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831A8118: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831A811C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831A8120: 419AFFE0  beq cr6, 0x831a8100
	if ctx.cr[6].eq {
	pc = 0x831A8100; continue 'dispatch;
	}
	// 831A8124: 7D2B0034  cntlzw r11, r9
	ctx.r[11].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 831A8128: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 831A812C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A8130 size=80
    let mut pc: u32 = 0x831A8130;
    'dispatch: loop {
        match pc {
            0x831A8130 => {
    //   block [0x831A8130..0x831A8180)
	// 831A8130: F9C1FF68  std r14, -0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), ctx.r[14].u64 ) };
	// 831A8134: F9E1FF70  std r15, -0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.r[15].u64 ) };
	// 831A8138: FA01FF78  std r16, -0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.r[16].u64 ) };
	// 831A813C: FA21FF80  std r17, -0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.r[17].u64 ) };
	// 831A8140: FA41FF88  std r18, -0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), ctx.r[18].u64 ) };
	// 831A8144: FA61FF90  std r19, -0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), ctx.r[19].u64 ) };
	// 831A8148: FA81FF98  std r20, -0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.r[20].u64 ) };
	// 831A814C: FAA1FFA0  std r21, -0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.r[21].u64 ) };
	// 831A8150: FAC1FFA8  std r22, -0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.r[22].u64 ) };
	// 831A8154: FAE1FFB0  std r23, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.r[23].u64 ) };
	// 831A8158: FB01FFB8  std r24, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.r[24].u64 ) };
	// 831A815C: FB21FFC0  std r25, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[25].u64 ) };
	// 831A8160: FB41FFC8  std r26, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.r[26].u64 ) };
	// 831A8164: FB61FFD0  std r27, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.r[27].u64 ) };
	// 831A8168: FB81FFD8  std r28, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.r[28].u64 ) };
	// 831A816C: FBA1FFE0  std r29, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[29].u64 ) };
	// 831A8170: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831A8174: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831A8178: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A817C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A8180 size=84
    let mut pc: u32 = 0x831A8180;
    'dispatch: loop {
        match pc {
            0x831A8180 => {
    //   block [0x831A8180..0x831A81D4)
	// 831A8180: E9C1FF68  ld r14, -0x98(r1)
	ctx.r[14].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-152 as u32) ) };
	// 831A8184: E9E1FF70  ld r15, -0x90(r1)
	ctx.r[15].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-144 as u32) ) };
	// 831A8188: EA01FF78  ld r16, -0x88(r1)
	ctx.r[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-136 as u32) ) };
	// 831A818C: EA21FF80  ld r17, -0x80(r1)
	ctx.r[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-128 as u32) ) };
	// 831A8190: EA41FF88  ld r18, -0x78(r1)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-120 as u32) ) };
	// 831A8194: EA61FF90  ld r19, -0x70(r1)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-112 as u32) ) };
	// 831A8198: EA81FF98  ld r20, -0x68(r1)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-104 as u32) ) };
	// 831A819C: EAA1FFA0  ld r21, -0x60(r1)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-96 as u32) ) };
	// 831A81A0: EAC1FFA8  ld r22, -0x58(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 831A81A4: EAE1FFB0  ld r23, -0x50(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 831A81A8: EB01FFB8  ld r24, -0x48(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 831A81AC: EB21FFC0  ld r25, -0x40(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 831A81B0: EB41FFC8  ld r26, -0x38(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 831A81B4: EB61FFD0  ld r27, -0x30(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 831A81B8: EB81FFD8  ld r28, -0x28(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 831A81BC: EBA1FFE0  ld r29, -0x20(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 831A81C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831A81C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831A81C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A81CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A81D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A81D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A81D4 size=12
    let mut pc: u32 = 0x831A81D4;
    'dispatch: loop {
        match pc {
            0x831A81D4 => {
    //   block [0x831A81D4..0x831A81E0)
	// 831A81D4: 7C8903A6  mtctr r4
	ctx.ctr.u64 = ctx.r[4].u64;
	// 831A81D8: 7CAC2B78  mr r12, r5
	ctx.r[12].u64 = ctx.r[5].u64;
	// 831A81DC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A81E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A81E0 size=16
    let mut pc: u32 = 0x831A81E0;
    'dispatch: loop {
        match pc {
            0x831A81E0 => {
    //   block [0x831A81E0..0x831A81F0)
	// 831A81E0: 38050001  addi r0, r5, 1
	ctx.r[0].s64 = ctx.r[5].s64 + 1;
	// 831A81E4: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831A81E8: 60660000  ori r6, r3, 0
	ctx.r[6].u64 = ctx.r[3].u64 | 0;
	// 831A81EC: 48000010  b 0x831a81fc
	sub_831A81F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A81F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A81F0 size=120
    let mut pc: u32 = 0x831A81F0;
    'dispatch: loop {
        match pc {
            0x831A81F0 => {
    //   block [0x831A81F0..0x831A8268)
	// 831A81F0: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 831A81F4: 98860000  stb r4, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 831A81F8: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 831A81FC: 70C00003  andi. r0, r6, 3
	ctx.r[0].u64 = ctx.r[6].u64 & 3;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A8200: 4002FFF0  bdnzf eq, 0x831a81f0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 && !ctx.cr[0].eq {
	pc = 0x831A81F0; continue 'dispatch;
	}
	// 831A8204: 5084442E  rlwimi r4, r4, 8, 0x10, 0x17
	ctx.r[4].u64 = (((ctx.r[4].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[4].u64 & 0xFFFFFFFFFFFF00FF);
	// 831A8208: 54A0E13F  rlwinm. r0, r5, 0x1c, 4, 0x1f
	ctx.r[0].u64 = ctx.r[5].u32 as u64 & 0x0000000Fu64;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A820C: 5084801E  rlwimi r4, r4, 0x10, 0, 0xf
	ctx.r[4].u64 = (((ctx.r[4].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[4].u64 & 0xFFFFFFFF0000FFFF);
	// 831A8210: 41E20020  beq+ 0x831a8230
	if ctx.cr[0].eq {
	pc = 0x831A8230; continue 'dispatch;
	}
	// 831A8214: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831A8218: 90860000  stw r4, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 831A821C: 90860004  stw r4, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 831A8220: 90860008  stw r4, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 831A8224: 9086000C  stw r4, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 831A8228: 38C60010  addi r6, r6, 0x10
	ctx.r[6].s64 = ctx.r[6].s64 + 16;
	// 831A822C: 4320FFEC  bdnz+ 0x831a8218
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831A8218; continue 'dispatch;
	}
	// 831A8230: 54A0F7BF  rlwinm. r0, r5, 0x1e, 0x1e, 0x1f
	ctx.r[0].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A8234: 41C20028  beq- 0x831a825c
	if ctx.cr[0].eq {
	pc = 0x831A825C; continue 'dispatch;
	}
	// 831A8238: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831A823C: 90860000  stw r4, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 831A8240: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 831A8244: 43400018  bdz- 0x831a825c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 == 0 {
	pc = 0x831A825C; continue 'dispatch;
	}
	// 831A8248: 90860000  stw r4, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 831A824C: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 831A8250: 4340000C  bdz- 0x831a825c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 == 0 {
	pc = 0x831A825C; continue 'dispatch;
	}
	// 831A8254: 90860000  stw r4, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 831A8258: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 831A825C: 70A00003  andi. r0, r5, 3
	ctx.r[0].u64 = ctx.r[5].u64 & 3;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A8260: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831A8264: 4DE20020  beqlr+
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A8268 size=8
    let mut pc: u32 = 0x831A8268;
    'dispatch: loop {
        match pc {
            0x831A8268 => {
    //   block [0x831A8268..0x831A8270)
	// 831A8268: 98860000  stb r4, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 831A826C: 4F400020  bdzlr-
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 == 0 { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A8270 size=8
    let mut pc: u32 = 0x831A8270;
    'dispatch: loop {
        match pc {
            0x831A8270 => {
    //   block [0x831A8270..0x831A8278)
	// 831A8270: 98860001  stb r4, 1(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 831A8274: 4F400020  bdzlr-
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 == 0 { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A8278 size=8
    let mut pc: u32 = 0x831A8278;
    'dispatch: loop {
        match pc {
            0x831A8278 => {
    //   block [0x831A8278..0x831A8280)
	// 831A8278: 98860002  stb r4, 2(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(2 as u32), ctx.r[4].u8 ) };
	// 831A827C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A8280 size=76
    let mut pc: u32 = 0x831A8280;
    'dispatch: loop {
        match pc {
            0x831A8280 => {
    //   block [0x831A8280..0x831A82CC)
	// 831A8280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A8284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A8288: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A828C: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 831A8290: 816BBEF4  lwz r11, -0x410c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16652 as u32) ) } as u64;
	// 831A8294: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831A8298: 4182000C  beq 0x831a82a4
	if ctx.cr[0].eq {
	pc = 0x831A82A4; continue 'dispatch;
	}
	// 831A829C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831A82A0: 4E800421  bctrl
	ctx.lr = 0x831A82A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831A82A4: 38600019  li r3, 0x19
	ctx.r[3].s64 = 25;
	// 831A82A8: 4800AE11  bl 0x831b30b8
	ctx.lr = 0x831A82AC;
	sub_831B30B8(ctx, base);
	// 831A82AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831A82B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A82B4: 48003945  bl 0x831abbf8
	ctx.lr = 0x831A82B8;
	sub_831ABBF8(ctx, base);
	// 831A82B8: 48003891  bl 0x831abb48
	ctx.lr = 0x831A82BC;
	sub_831ABB48(ctx, base);
	// 831A82BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831A82C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A82C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A82C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A82D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A82D0 size=148
    let mut pc: u32 = 0x831A82D0;
    'dispatch: loop {
        match pc {
            0x831A82D0 => {
    //   block [0x831A82D0..0x831A8364)
	// 831A82D0: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 831A82D4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 831A82D8: 3D008335  lis r8, -0x7ccb
	ctx.r[8].s64 = -2093678592;
	// 831A82DC: 3D60831B  lis r11, -0x7ce5
	ctx.r[11].s64 = -2095382528;
	// 831A82E0: 3BC8F440  addi r30, r8, -0xbc0
	ctx.r[30].s64 = ctx.r[8].s64 + -3008;
	// 831A82E4: 396B3F58  addi r11, r11, 0x3f58
	ctx.r[11].s64 = ctx.r[11].s64 + 16216;
	// 831A82E8: 3D40831B  lis r10, -0x7ce5
	ctx.r[10].s64 = -2095382528;
	// 831A82EC: 3D20831B  lis r9, -0x7ce5
	ctx.r[9].s64 = -2095382528;
	// 831A82F0: 9168F440  stw r11, -0xbc0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-3008 as u32), ctx.r[11].u32 ) };
	// 831A82F4: 3FE0831B  lis r31, -0x7ce5
	ctx.r[31].s64 = -2095382528;
	// 831A82F8: 394A3328  addi r10, r10, 0x3328
	ctx.r[10].s64 = ctx.r[10].s64 + 13096;
	// 831A82FC: 39293318  addi r9, r9, 0x3318
	ctx.r[9].s64 = ctx.r[9].s64 + 13080;
	// 831A8300: 397F3320  addi r11, r31, 0x3320
	ctx.r[11].s64 = ctx.r[31].s64 + 13088;
	// 831A8304: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 831A8308: 3C60831B  lis r3, -0x7ce5
	ctx.r[3].s64 = -2095382528;
	// 831A830C: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 831A8310: 3C80831B  lis r4, -0x7ce5
	ctx.r[4].s64 = -2095382528;
	// 831A8314: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831A8318: 3CA0831B  lis r5, -0x7ce5
	ctx.r[5].s64 = -2095382528;
	// 831A831C: 39433298  addi r10, r3, 0x3298
	ctx.r[10].s64 = ctx.r[3].s64 + 12952;
	// 831A8320: 39243F58  addi r9, r4, 0x3f58
	ctx.r[9].s64 = ctx.r[4].s64 + 16216;
	// 831A8324: 39653F08  addi r11, r5, 0x3f08
	ctx.r[11].s64 = ctx.r[5].s64 + 16136;
	// 831A8328: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 831A832C: 3CC0831B  lis r6, -0x7ce5
	ctx.r[6].s64 = -2095382528;
	// 831A8330: 913E0014  stw r9, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 831A8334: 3CE0831B  lis r7, -0x7ce5
	ctx.r[7].s64 = -2095382528;
	// 831A8338: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 831A833C: 3D00831B  lis r8, -0x7ce5
	ctx.r[8].s64 = -2095382528;
	// 831A8340: 394632B8  addi r10, r6, 0x32b8
	ctx.r[10].s64 = ctx.r[6].s64 + 12984;
	// 831A8344: 392731E0  addi r9, r7, 0x31e0
	ctx.r[9].s64 = ctx.r[7].s64 + 12768;
	// 831A8348: 39683140  addi r11, r8, 0x3140
	ctx.r[11].s64 = ctx.r[8].s64 + 12608;
	// 831A834C: 915E001C  stw r10, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 831A8350: 913E0020  stw r9, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 831A8354: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 831A8358: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831A835C: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 831A8360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A8368 size=4
    let mut pc: u32 = 0x831A8368;
    'dispatch: loop {
        match pc {
            0x831A8368 => {
    //   block [0x831A8368..0x831A836C)
	// 831A8368: 4BFFFF68  b 0x831a82d0
	sub_831A82D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A8370 size=84
    let mut pc: u32 = 0x831A8370;
    'dispatch: loop {
        match pc {
            0x831A8370 => {
    //   block [0x831A8370..0x831A83C4)
	// 831A8370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A8374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A8378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A837C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 831A8380: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 831A8384: 48007BA5  bl 0x831aff28
	ctx.lr = 0x831A8388;
	sub_831AFF28(ctx, base);
	// 831A8388: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 831A838C: 3D208345  lis r9, -0x7cbb
	ctx.r[9].s64 = -2092630016;
	// 831A8390: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831A8394: 916ACF14  stw r11, -0x30ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12524 as u32), ctx.r[11].u32 ) };
	// 831A8398: 9169CF10  stw r11, -0x30f0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-12528 as u32), ctx.r[11].u32 ) };
	// 831A839C: 4082000C  bne 0x831a83a8
	if !ctx.cr[0].eq {
	pc = 0x831A83A8; continue 'dispatch;
	}
	// 831A83A0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 831A83A4: 48000010  b 0x831a83b4
	pc = 0x831A83B4; continue 'dispatch;
	// 831A83A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831A83AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A83B0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831A83B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831A83B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A83BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A83C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A83C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A83C8 size=8
    let mut pc: u32 = 0x831A83C8;
    'dispatch: loop {
        match pc {
            0x831A83C8 => {
    //   block [0x831A83C8..0x831A83D0)
	// 831A83C8: 8324280C  lwz r25, 0x280c(r4)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(10252 as u32) ) } as u64;
	// 831A83CC: 8224BF10  lwz r17, -0x40f0(r4)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-16624 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A83D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A83D0 size=228
    let mut pc: u32 = 0x831A83D0;
    'dispatch: loop {
        match pc {
            0x831A83D0 => {
    //   block [0x831A83D0..0x831A84B4)
	// 831A83D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A83D4: 4BFFFD81  bl 0x831a8154
	ctx.lr = 0x831A83D8;
	sub_831A8130(ctx, base);
	// 831A83D8: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 831A83DC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A83E0: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 831A83E4: 480045CD  bl 0x831ac9b0
	ctx.lr = 0x831A83E8;
	sub_831AC9B0(ctx, base);
	// 831A83E8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831A83EC: 3F008345  lis r24, -0x7cbb
	ctx.r[24].s64 = -2092630016;
	// 831A83F0: 3F208345  lis r25, -0x7cbb
	ctx.r[25].s64 = -2092630016;
	// 831A83F4: 83D8CF10  lwz r30, -0x30f0(r24)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-12528 as u32) ) } as u64;
	// 831A83F8: 8399CF14  lwz r28, -0x30ec(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-12524 as u32) ) } as u64;
	// 831A83FC: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 831A8400: 41980094  blt cr6, 0x831a8494
	if ctx.cr[6].lt {
	pc = 0x831A8494; continue 'dispatch;
	}
	// 831A8404: 7F5CF050  subf r26, r28, r30
	ctx.r[26].s64 = ctx.r[30].s64 - ctx.r[28].s64;
	// 831A8408: 3B7A0004  addi r27, r26, 4
	ctx.r[27].s64 = ctx.r[26].s64 + 4;
	// 831A840C: 2B1B0004  cmplwi cr6, r27, 4
	ctx.cr[6].compare_u32(ctx.r[27].u32, 4 as u32, &mut ctx.xer);
	// 831A8410: 41980084  blt cr6, 0x831a8494
	if ctx.cr[6].lt {
	pc = 0x831A8494; continue 'dispatch;
	}
	// 831A8414: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831A8418: 4800BB99  bl 0x831b3fb0
	ctx.lr = 0x831A841C;
	sub_831B3FB0(ctx, base);
	// 831A841C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831A8420: 7F1DD840  cmplw cr6, r29, r27
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[27].u32, &mut ctx.xer);
	// 831A8424: 4098005C  bge cr6, 0x831a8480
	if !ctx.cr[6].lt {
	pc = 0x831A8480; continue 'dispatch;
	}
	// 831A8428: 2B1D0800  cmplwi cr6, r29, 0x800
	ctx.cr[6].compare_u32(ctx.r[29].u32, 2048 as u32, &mut ctx.xer);
	// 831A842C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 831A8430: 41980008  blt cr6, 0x831a8438
	if ctx.cr[6].lt {
	pc = 0x831A8438; continue 'dispatch;
	}
	// 831A8434: 39600800  li r11, 0x800
	ctx.r[11].s64 = 2048;
	// 831A8438: 7C8BEA14  add r4, r11, r29
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 831A843C: 7F04E840  cmplw cr6, r4, r29
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[29].u32, &mut ctx.xer);
	// 831A8440: 41980014  blt cr6, 0x831a8454
	if ctx.cr[6].lt {
	pc = 0x831A8454; continue 'dispatch;
	}
	// 831A8444: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831A8448: 48008B11  bl 0x831b0f58
	ctx.lr = 0x831A844C;
	sub_831B0F58(ctx, base);
	// 831A844C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831A8450: 40820020  bne 0x831a8470
	if !ctx.cr[0].eq {
	pc = 0x831A8470; continue 'dispatch;
	}
	// 831A8454: 389D0010  addi r4, r29, 0x10
	ctx.r[4].s64 = ctx.r[29].s64 + 16;
	// 831A8458: 7F04E840  cmplw cr6, r4, r29
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[29].u32, &mut ctx.xer);
	// 831A845C: 41980038  blt cr6, 0x831a8494
	if ctx.cr[6].lt {
	pc = 0x831A8494; continue 'dispatch;
	}
	// 831A8460: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831A8464: 48008AF5  bl 0x831b0f58
	ctx.lr = 0x831A8468;
	sub_831B0F58(ctx, base);
	// 831A8468: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831A846C: 41820028  beq 0x831a8494
	if ctx.cr[0].eq {
	pc = 0x831A8494; continue 'dispatch;
	}
	// 831A8470: 7F4B1670  srawi r11, r26, 2
	ctx.xer.ca = (ctx.r[26].s32 < 0) && ((ctx.r[26].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[26].s32 >> 2) as i64;
	// 831A8474: 9079CF14  stw r3, -0x30ec(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(-12524 as u32), ctx.r[3].u32 ) };
	// 831A8478: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831A847C: 7FCB1A14  add r30, r11, r3
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 831A8480: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 831A8484: 92FE0000  stw r23, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 831A8488: 92FF0050  stw r23, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[23].u32 ) };
	// 831A848C: 9178CF10  stw r11, -0x30f0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(-12528 as u32), ctx.r[11].u32 ) };
	// 831A8490: 4800000C  b 0x831a849c
	pc = 0x831A849C; continue 'dispatch;
	// 831A8494: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831A8498: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 831A849C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831A84A0: 399F00B0  addi r12, r31, 0xb0
	ctx.r[12].s64 = ctx.r[31].s64 + 176;
	// 831A84A4: 48000011  bl 0x831a84b4
	ctx.lr = 0x831A84A8;
	sub_831A84B4(ctx, base);
	// 831A84A8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 831A84AC: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 831A84B0: 4BFFFCF4  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A84B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A84B4 size=32
    let mut pc: u32 = 0x831A84B4;
    'dispatch: loop {
        match pc {
            0x831A84B4 => {
    //   block [0x831A84B4..0x831A84D4)
	// 831A84B4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A84B8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A84BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A84C0: 480044F9  bl 0x831ac9b8
	ctx.lr = 0x831A84C4;
	sub_831AC9B8(ctx, base);
	// 831A84C4: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A84C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A84CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A84D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A84D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A84D8 size=44
    let mut pc: u32 = 0x831A84D8;
    'dispatch: loop {
        match pc {
            0x831A84D8 => {
    //   block [0x831A84D8..0x831A8504)
	// 831A84D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A84DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A84E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A84E4: 4BFFFEED  bl 0x831a83d0
	ctx.lr = 0x831A84E8;
	sub_831A83D0(ctx, base);
	// 831A84E8: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 831A84EC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 831A84F0: 7C6B00D0  neg r3, r11
	ctx.r[3].s64 = -ctx.r[11].s64;
	// 831A84F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831A84F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A84FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A8500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A8510 size=80
    let mut pc: u32 = 0x831A8510;
    'dispatch: loop {
        match pc {
            0x831A8510 => {
    //   block [0x831A8510..0x831A8560)
	// 831A8510: F861FFF8  std r3, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[3].u64 ) };
	// 831A8514: 5466077E  clrlwi r6, r3, 0x1d
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 831A8518: 7C00222C  dcbt 0, r4
	// 831A851C: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831A8520: 20C60008  subfic r6, r6, 8
	ctx.xer.ca = ctx.r[6].u32 <= 8 as u32;
	ctx.r[6].s64 = (8 as i64) - ctx.r[6].s64;
	// 831A8524: 41820050  beq 0x831a8574
	if ctx.cr[0].eq {
		sub_831A8560(ctx, base);
		return;
	}
	// 831A8528: 7C053040  cmplw r5, r6
	ctx.cr[0].compare_u32(ctx.r[6].u32, ctx.r[0].u32, &mut ctx.xer);
	// 831A852C: 40810064  ble 0x831a8590
	if !ctx.cr[0].gt {
		sub_831A8560(ctx, base);
		return;
	}
	// 831A8530: 28060004  cmplwi r6, 4
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831A8534: 4182002C  beq 0x831a8560
	if ctx.cr[0].eq {
		sub_831A8560(ctx, base);
		return;
	}
	// 831A8538: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 831A853C: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 831A8540: 7CA62850  subf r5, r6, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	// 831A8544: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 831A8548: 8CC40001  lbzu r6, 1(r4)
	ea = ctx.r[4].u32.wrapping_add(1 as u32);
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[4].u32 = ea;
	// 831A854C: 9CC30001  stbu r6, 1(r3)
	ea = ctx.r[3].u32.wrapping_add(1 as u32);
	unsafe { crate::rt::store_u8(base as *mut u8, ea, ctx.r[6].u8) };
	ctx.r[3].u32 = ea;
	// 831A8550: 4200FFF8  bdnz 0x831a8548
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831A8548; continue 'dispatch;
	}
	// 831A8554: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 831A8558: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 831A855C: 48000018  b 0x831a8574
	sub_831A8560(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A8560 size=140
    let mut pc: u32 = 0x831A8560;
    'dispatch: loop {
        match pc {
            0x831A8560 => {
    //   block [0x831A8560..0x831A85EC)
	// 831A8560: 7CA62850  subf r5, r6, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	// 831A8564: 80C40000  lwz r6, 0(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A8568: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 831A856C: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 831A8570: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 831A8574: 5486077E  clrlwi r6, r4, 0x1d
	ctx.r[6].u64 = ctx.r[4].u32 as u64 & 0x00000007u64;
	// 831A8578: 2B060004  cmplwi cr6, r6, 4
	ctx.cr[6].compare_u32(ctx.r[6].u32, 4 as u32, &mut ctx.xer);
	// 831A857C: 28860000  cmplwi cr1, r6, 0
	ctx.cr[1].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 831A8580: 2B850080  cmplwi cr7, r5, 0x80
	ctx.cr[7].compare_u32(ctx.r[5].u32, 128 as u32, &mut ctx.xer);
	// 831A8584: 419A01D4  beq cr6, 0x831a8758
	if ctx.cr[6].eq {
		sub_831A8758(ctx, base);
		return;
	}
	// 831A8588: 40860300  bne cr1, 0x831a8888
	if !ctx.cr[1].eq {
		sub_831A8888(ctx, base);
		return;
	}
	// 831A858C: 409C00A0  bge cr7, 0x831a862c
	if !ctx.cr[7].lt {
		sub_831A862C(ctx, base);
		return;
	}
	// 831A8590: 7C0019EC  dcbtst 0, r3
	// 831A8594: 3884FFF8  addi r4, r4, -8
	ctx.r[4].s64 = ctx.r[4].s64 + -8;
	// 831A8598: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 831A859C: 54A7EF3E  rlwinm r7, r5, 0x1d, 0x1c, 0x1f
	ctx.r[7].u64 = ctx.r[5].u32 as u64 & 0x00000007u64;
	// 831A85A0: 54A6077E  clrlwi r6, r5, 0x1d
	ctx.r[6].u64 = ctx.r[5].u32 as u64 & 0x00000007u64;
	// 831A85A4: 28870000  cmplwi cr1, r7, 0
	ctx.cr[1].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 831A85A8: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 831A85AC: 41860014  beq cr1, 0x831a85c0
	if ctx.cr[1].eq {
	pc = 0x831A85C0; continue 'dispatch;
	}
	// 831A85B0: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 831A85B4: E8E40009  ldu r7, 8(r4)
	ea = ctx.r[4].u32.wrapping_add(8 as u32);
	ctx.r[7].u64 = unsafe { crate::rt::load_u64(base as *const u8, ea) };
	ctx.r[4].u32 = ea;
	// 831A85B8: F8E30009  stdu r7, 8(r3)
	ea = ctx.r[3].u32.wrapping_add(8 as u32);
	unsafe { crate::rt::store_u64(base as *mut u8, ea, ctx.r[7].u64) };
	ctx.r[3].u32 = ea;
	// 831A85BC: 4200FFF8  bdnz 0x831a85b4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831A85B4; continue 'dispatch;
	}
	// 831A85C0: 28860004  cmplwi cr1, r6, 4
	ctx.cr[1].compare_u32(ctx.r[6].u32, 4 as u32, &mut ctx.xer);
	// 831A85C4: 419A0020  beq cr6, 0x831a85e4
	if ctx.cr[6].eq {
	pc = 0x831A85E4; continue 'dispatch;
	}
	// 831A85C8: 41860024  beq cr1, 0x831a85ec
	if ctx.cr[1].eq {
		sub_831A85EC(ctx, base);
		return;
	}
	// 831A85CC: 38630007  addi r3, r3, 7
	ctx.r[3].s64 = ctx.r[3].s64 + 7;
	// 831A85D0: 38840007  addi r4, r4, 7
	ctx.r[4].s64 = ctx.r[4].s64 + 7;
	// 831A85D4: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 831A85D8: 8CE40001  lbzu r7, 1(r4)
	ea = ctx.r[4].u32.wrapping_add(1 as u32);
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[4].u32 = ea;
	// 831A85DC: 9CE30001  stbu r7, 1(r3)
	ea = ctx.r[3].u32.wrapping_add(1 as u32);
	unsafe { crate::rt::store_u8(base as *mut u8, ea, ctx.r[7].u8) };
	ctx.r[3].u32 = ea;
	// 831A85E0: 4200FFF8  bdnz 0x831a85d8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831A85D8; continue 'dispatch;
	}
	// 831A85E4: E861FFF8  ld r3, -8(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 831A85E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A85EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A85EC size=28
    let mut pc: u32 = 0x831A85EC;
    'dispatch: loop {
        match pc {
            0x831A85EC => {
    //   block [0x831A85EC..0x831A8608)
	// 831A85EC: 546607BE  clrlwi r6, r3, 0x1e
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0x00000003u64;
	// 831A85F0: 80A40008  lwz r5, 8(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 831A85F4: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831A85F8: 40820010  bne 0x831a8608
	if !ctx.cr[0].eq {
		sub_831A8608(ctx, base);
		return;
	}
	// 831A85FC: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 831A8600: E861FFF8  ld r3, -8(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 831A8604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A8608 size=36
    let mut pc: u32 = 0x831A8608;
    'dispatch: loop {
        match pc {
            0x831A8608 => {
    //   block [0x831A8608..0x831A862C)
	// 831A8608: 89040008  lbz r8, 8(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 831A860C: 88E40009  lbz r7, 9(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(9 as u32) ) } as u64;
	// 831A8610: 88C4000A  lbz r6, 0xa(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(10 as u32) ) } as u64;
	// 831A8614: 99030008  stb r8, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u8 ) };
	// 831A8618: 98E30009  stb r7, 9(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(9 as u32), ctx.r[7].u8 ) };
	// 831A861C: 98C3000A  stb r6, 0xa(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10 as u32), ctx.r[6].u8 ) };
	// 831A8620: 98A3000B  stb r5, 0xb(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(11 as u32), ctx.r[5].u8 ) };
	// 831A8624: E861FFF8  ld r3, -8(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 831A8628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A862C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A862C size=276
    let mut pc: u32 = 0x831A862C;
    'dispatch: loop {
        match pc {
            0x831A862C => {
    //   block [0x831A862C..0x831A8740)
	// 831A862C: 5466067E  clrlwi r6, r3, 0x19
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0x0000007Fu64;
	// 831A8630: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 831A8634: 3884FFF8  addi r4, r4, -8
	ctx.r[4].s64 = ctx.r[4].s64 + -8;
	// 831A8638: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831A863C: 20C60080  subfic r6, r6, 0x80
	ctx.xer.ca = ctx.r[6].u32 <= 128 as u32;
	ctx.r[6].s64 = (128 as i64) - ctx.r[6].s64;
	// 831A8640: 4182001C  beq 0x831a865c
	if ctx.cr[0].eq {
	pc = 0x831A865C; continue 'dispatch;
	}
	// 831A8644: 54C7E8FE  srwi r7, r6, 3
	ctx.r[7].u32 = ctx.r[6].u32.wrapping_shr(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831A8648: 7CA62850  subf r5, r6, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	// 831A864C: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 831A8650: E8E40009  ldu r7, 8(r4)
	ea = ctx.r[4].u32.wrapping_add(8 as u32);
	ctx.r[7].u64 = unsafe { crate::rt::load_u64(base as *const u8, ea) };
	ctx.r[4].u32 = ea;
	// 831A8654: F8E30009  stdu r7, 8(r3)
	ea = ctx.r[3].u32.wrapping_add(8 as u32);
	unsafe { crate::rt::store_u64(base as *mut u8, ea, ctx.r[7].u64) };
	ctx.r[3].u32 = ea;
	// 831A8658: 4200FFF8  bdnz 0x831a8650
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831A8650; continue 'dispatch;
	}
	// 831A865C: 54A6C9FE  srwi r6, r5, 7
	ctx.r[6].u32 = ctx.r[5].u32.wrapping_shr(7);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831A8660: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831A8664: 4182FF38  beq 0x831a859c
	if ctx.cr[0].eq {
		sub_831A8560(ctx, base);
		return;
	}
	// 831A8668: 3945007F  addi r10, r5, 0x7f
	ctx.r[10].s64 = ctx.r[5].s64 + 127;
	// 831A866C: 54A8067E  clrlwi r8, r5, 0x19
	ctx.r[8].u64 = ctx.r[5].u32 as u64 & 0x0000007Fu64;
	// 831A8670: 554AC9FE  srwi r10, r10, 7
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(7);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831A8674: 28880000  cmplwi cr1, r8, 0
	ctx.cr[1].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 831A8678: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 831A867C: 554A077E  clrlwi r10, r10, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 831A8680: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831A8684: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 831A8688: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831A868C: 7C09222C  dcbt r9, r4
	// 831A8690: 39290080  addi r9, r9, 0x80
	ctx.r[9].s64 = ctx.r[9].s64 + 128;
	// 831A8694: 4200FFF8  bdnz 0x831a868c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831A868C; continue 'dispatch;
	}
	// 831A8698: 7D842A14  add r12, r4, r5
	ctx.r[12].u64 = ctx.r[4].u64 + ctx.r[5].u64;
	// 831A869C: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 831A86A0: 7D696050  subf r11, r9, r12
	ctx.r[11].s64 = ctx.r[12].s64 - ctx.r[9].s64;
	// 831A86A4: 7D832A14  add r12, r3, r5
	ctx.r[12].u64 = ctx.r[3].u64 + ctx.r[5].u64;
	// 831A86A8: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 831A86AC: E8C40008  ld r6, 8(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 831A86B0: E8E40010  ld r7, 0x10(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	// 831A86B4: E9040018  ld r8, 0x18(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	// 831A86B8: F8C30008  std r6, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 831A86BC: E8C40020  ld r6, 0x20(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	// 831A86C0: F8E30010  std r7, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u64 ) };
	// 831A86C4: E8E40028  ld r7, 0x28(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	// 831A86C8: F9030018  std r8, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[8].u64 ) };
	// 831A86CC: E9040030  ld r8, 0x30(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) };
	// 831A86D0: F8C30020  std r6, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[6].u64 ) };
	// 831A86D4: E8C40038  ld r6, 0x38(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) };
	// 831A86D8: F8E30028  std r7, 0x28(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[7].u64 ) };
	// 831A86DC: E8E40040  ld r7, 0x40(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) };
	// 831A86E0: F9030030  std r8, 0x30(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[8].u64 ) };
	// 831A86E4: E9040048  ld r8, 0x48(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) };
	// 831A86E8: F8C30038  std r6, 0x38(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[6].u64 ) };
	// 831A86EC: E8C40050  ld r6, 0x50(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(80 as u32) ) };
	// 831A86F0: F8E30040  std r7, 0x40(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[7].u64 ) };
	// 831A86F4: E8E40058  ld r7, 0x58(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(88 as u32) ) };
	// 831A86F8: F9030048  std r8, 0x48(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[8].u64 ) };
	// 831A86FC: E9040060  ld r8, 0x60(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(96 as u32) ) };
	// 831A8700: F8C30050  std r6, 0x50(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[6].u64 ) };
	// 831A8704: E8C40068  ld r6, 0x68(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(104 as u32) ) };
	// 831A8708: F8E30058  std r7, 0x58(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[7].u64 ) };
	// 831A870C: E8E40070  ld r7, 0x70(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(112 as u32) ) };
	// 831A8710: F9030060  std r8, 0x60(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[8].u64 ) };
	// 831A8714: E9040078  ld r8, 0x78(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(120 as u32) ) };
	// 831A8718: F8C30068  std r6, 0x68(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[6].u64 ) };
	// 831A871C: E8C40081  ldu r6, 0x80(r4)
	ea = ctx.r[4].u32.wrapping_add(128 as u32);
	ctx.r[6].u64 = unsafe { crate::rt::load_u64(base as *const u8, ea) };
	ctx.r[4].u32 = ea;
	// 831A8720: F8E30070  std r7, 0x70(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), ctx.r[7].u64 ) };
	// 831A8724: F9030078  std r8, 0x78(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), ctx.r[8].u64 ) };
	// 831A8728: F8C30081  stdu r6, 0x80(r3)
	ea = ctx.r[3].u32.wrapping_add(128 as u32);
	unsafe { crate::rt::store_u64(base as *mut u8, ea, ctx.r[6].u64) };
	ctx.r[3].u32 = ea;
	// 831A872C: 7C045840  cmplw r4, r11
	ctx.cr[0].compare_u32(ctx.r[11].u32, ctx.r[0].u32, &mut ctx.xer);
	// 831A8730: 40800010  bge 0x831a8740
	if !ctx.cr[0].lt {
		sub_831A8740(ctx, base);
		return;
	}
	// 831A8734: 7C09222C  dcbt r9, r4
	// 831A8738: 4200FF74  bdnz 0x831a86ac
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831A86AC; continue 'dispatch;
	}
	// 831A873C: 4BFFFE60  b 0x831a859c
	sub_831A8560(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A8740 size=24
    let mut pc: u32 = 0x831A8740;
    'dispatch: loop {
        match pc {
            0x831A8740 => {
    //   block [0x831A8740..0x831A8758)
	// 831A8740: 41860010  beq cr1, 0x831a8750
	if ctx.cr[1].eq {
	pc = 0x831A8750; continue 'dispatch;
	}
	// 831A8744: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 831A8748: 7C0861EC  dcbtst r8, r12
	// 831A874C: 28880000  cmplwi cr1, r8, 0
	ctx.cr[1].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 831A8750: 4200FF5C  bdnz 0x831a86ac
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			sub_831A862C(ctx, base);
		return;
	}
	// 831A8754: 4BFFFE48  b 0x831a859c
	sub_831A8560(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A8758 size=88
    let mut pc: u32 = 0x831A8758;
    'dispatch: loop {
        match pc {
            0x831A8758 => {
    //   block [0x831A8758..0x831A87B0)
	// 831A8758: 3884FFFC  addi r4, r4, -4
	ctx.r[4].s64 = ctx.r[4].s64 + -4;
	// 831A875C: 409C0054  bge cr7, 0x831a87b0
	if !ctx.cr[7].lt {
		sub_831A87B0(ctx, base);
		return;
	}
	// 831A8760: 7C0019EC  dcbtst 0, r3
	// 831A8764: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 831A8768: 54A7F6FE  rlwinm r7, r5, 0x1e, 0x1b, 0x1f
	ctx.r[7].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	// 831A876C: 54A607BE  clrlwi r6, r5, 0x1e
	ctx.r[6].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	// 831A8770: 28870000  cmplwi cr1, r7, 0
	ctx.cr[1].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 831A8774: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 831A8778: 41860014  beq cr1, 0x831a878c
	if ctx.cr[1].eq {
	pc = 0x831A878C; continue 'dispatch;
	}
	// 831A877C: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 831A8780: 84E40004  lwzu r7, 4(r4)
	ea = ctx.r[4].u32.wrapping_add(4 as u32);
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ea) } as u64;
	ctx.r[4].u32 = ea;
	// 831A8784: 94E30004  stwu r7, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[3].u32 = ea;
	// 831A8788: 4200FFF8  bdnz 0x831a8780
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831A8780; continue 'dispatch;
	}
	// 831A878C: 419A001C  beq cr6, 0x831a87a8
	if ctx.cr[6].eq {
	pc = 0x831A87A8; continue 'dispatch;
	}
	// 831A8790: 38630003  addi r3, r3, 3
	ctx.r[3].s64 = ctx.r[3].s64 + 3;
	// 831A8794: 38840003  addi r4, r4, 3
	ctx.r[4].s64 = ctx.r[4].s64 + 3;
	// 831A8798: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 831A879C: 8CE40001  lbzu r7, 1(r4)
	ea = ctx.r[4].u32.wrapping_add(1 as u32);
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[4].u32 = ea;
	// 831A87A0: 9CE30001  stbu r7, 1(r3)
	ea = ctx.r[3].u32.wrapping_add(1 as u32);
	unsafe { crate::rt::store_u8(base as *mut u8, ea, ctx.r[7].u8) };
	ctx.r[3].u32 = ea;
	// 831A87A4: 4200FFF8  bdnz 0x831a879c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831A879C; continue 'dispatch;
	}
	// 831A87A8: E861FFF8  ld r3, -8(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 831A87AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A87B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A87B0 size=192
    let mut pc: u32 = 0x831A87B0;
    'dispatch: loop {
        match pc {
            0x831A87B0 => {
    //   block [0x831A87B0..0x831A8870)
	// 831A87B0: 5466067E  clrlwi r6, r3, 0x19
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0x0000007Fu64;
	// 831A87B4: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 831A87B8: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831A87BC: 20C60080  subfic r6, r6, 0x80
	ctx.xer.ca = ctx.r[6].u32 <= 128 as u32;
	ctx.r[6].s64 = (128 as i64) - ctx.r[6].s64;
	// 831A87C0: 4182001C  beq 0x831a87dc
	if ctx.cr[0].eq {
	pc = 0x831A87DC; continue 'dispatch;
	}
	// 831A87C4: 54C7F0BE  srwi r7, r6, 2
	ctx.r[7].u32 = ctx.r[6].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831A87C8: 7CA62850  subf r5, r6, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	// 831A87CC: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 831A87D0: 84E40004  lwzu r7, 4(r4)
	ea = ctx.r[4].u32.wrapping_add(4 as u32);
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ea) } as u64;
	ctx.r[4].u32 = ea;
	// 831A87D4: 94E30004  stwu r7, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[3].u32 = ea;
	// 831A87D8: 4200FFF8  bdnz 0x831a87d0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831A87D0; continue 'dispatch;
	}
	// 831A87DC: 54A6C9FE  srwi r6, r5, 7
	ctx.r[6].u32 = ctx.r[5].u32.wrapping_shr(7);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831A87E0: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831A87E4: 4182FF84  beq 0x831a8768
	if ctx.cr[0].eq {
		sub_831A8758(ctx, base);
		return;
	}
	// 831A87E8: 3945007F  addi r10, r5, 0x7f
	ctx.r[10].s64 = ctx.r[5].s64 + 127;
	// 831A87EC: 54A8067E  clrlwi r8, r5, 0x19
	ctx.r[8].u64 = ctx.r[5].u32 as u64 & 0x0000007Fu64;
	// 831A87F0: 554AC9FE  srwi r10, r10, 7
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(7);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831A87F4: 28880000  cmplwi cr1, r8, 0
	ctx.cr[1].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 831A87F8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 831A87FC: 554A077E  clrlwi r10, r10, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 831A8800: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831A8804: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 831A8808: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831A880C: 7C09222C  dcbt r9, r4
	// 831A8810: 39290080  addi r9, r9, 0x80
	ctx.r[9].s64 = ctx.r[9].s64 + 128;
	// 831A8814: 4200FFF8  bdnz 0x831a880c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831A880C; continue 'dispatch;
	}
	// 831A8818: 7D842A14  add r12, r4, r5
	ctx.r[12].u64 = ctx.r[4].u64 + ctx.r[5].u64;
	// 831A881C: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 831A8820: 7D696050  subf r11, r9, r12
	ctx.r[11].s64 = ctx.r[12].s64 - ctx.r[9].s64;
	// 831A8824: 7D832A14  add r12, r3, r5
	ctx.r[12].u64 = ctx.r[3].u64 + ctx.r[5].u64;
	// 831A8828: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 831A882C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 831A8830: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 831A8834: 80040004  lwz r0, 4(r4)
	ctx.r[0].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 831A8838: 80E40008  lwz r7, 8(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 831A883C: 8104000C  lwz r8, 0xc(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 831A8840: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831A8844: 90030004  stw r0, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 831A8848: 84040010  lwzu r0, 0x10(r4)
	ea = ctx.r[4].u32.wrapping_add(16 as u32);
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ea) } as u64;
	ctx.r[4].u32 = ea;
	// 831A884C: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 831A8850: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 831A8854: 94030010  stwu r0, 0x10(r3)
	ea = ctx.r[3].u32.wrapping_add(16 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[0].u32) };
	ctx.r[3].u32 = ea;
	// 831A8858: 4082FFD8  bne 0x831a8830
	if !ctx.cr[0].eq {
	pc = 0x831A8830; continue 'dispatch;
	}
	// 831A885C: 7C045840  cmplw r4, r11
	ctx.cr[0].compare_u32(ctx.r[11].u32, ctx.r[0].u32, &mut ctx.xer);
	// 831A8860: 40800010  bge 0x831a8870
	if !ctx.cr[0].lt {
		sub_831A8870(ctx, base);
		return;
	}
	// 831A8864: 7C09222C  dcbt r9, r4
	// 831A8868: 4200FFC4  bdnz 0x831a882c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831A882C; continue 'dispatch;
	}
	// 831A886C: 4BFFFEFC  b 0x831a8768
	sub_831A8758(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A8870 size=24
    let mut pc: u32 = 0x831A8870;
    'dispatch: loop {
        match pc {
            0x831A8870 => {
    //   block [0x831A8870..0x831A8888)
	// 831A8870: 41860010  beq cr1, 0x831a8880
	if ctx.cr[1].eq {
	pc = 0x831A8880; continue 'dispatch;
	}
	// 831A8874: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 831A8878: 7C0861EC  dcbtst r8, r12
	// 831A887C: 28880000  cmplwi cr1, r8, 0
	ctx.cr[1].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 831A8880: 4200FFAC  bdnz 0x831a882c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			sub_831A87B0(ctx, base);
		return;
	}
	// 831A8884: 4BFFFEE4  b 0x831a8768
	sub_831A8758(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A8888 size=52
    let mut pc: u32 = 0x831A8888;
    'dispatch: loop {
        match pc {
            0x831A8888 => {
    //   block [0x831A8888..0x831A88BC)
	// 831A8888: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 831A888C: 409C0030  bge cr7, 0x831a88bc
	if !ctx.cr[7].lt {
		sub_831A88BC(ctx, base);
		return;
	}
	// 831A8890: 7C0019EC  dcbtst 0, r3
	// 831A8894: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 831A8898: 54A6067E  clrlwi r6, r5, 0x19
	ctx.r[6].u64 = ctx.r[5].u32 as u64 & 0x0000007Fu64;
	// 831A889C: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831A88A0: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 831A88A4: 41820010  beq 0x831a88b4
	if ctx.cr[0].eq {
	pc = 0x831A88B4; continue 'dispatch;
	}
	// 831A88A8: 8CC40001  lbzu r6, 1(r4)
	ea = ctx.r[4].u32.wrapping_add(1 as u32);
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[4].u32 = ea;
	// 831A88AC: 9CC30001  stbu r6, 1(r3)
	ea = ctx.r[3].u32.wrapping_add(1 as u32);
	unsafe { crate::rt::store_u8(base as *mut u8, ea, ctx.r[6].u8) };
	ctx.r[3].u32 = ea;
	// 831A88B0: 4200FFF8  bdnz 0x831a88a8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831A88A8; continue 'dispatch;
	}
	// 831A88B4: E861FFF8  ld r3, -8(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 831A88B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A88BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A88BC size=196
    let mut pc: u32 = 0x831A88BC;
    'dispatch: loop {
        match pc {
            0x831A88BC => {
    //   block [0x831A88BC..0x831A8980)
	// 831A88BC: 5466067E  clrlwi r6, r3, 0x19
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0x0000007Fu64;
	// 831A88C0: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 831A88C4: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831A88C8: 20C60080  subfic r6, r6, 0x80
	ctx.xer.ca = ctx.r[6].u32 <= 128 as u32;
	ctx.r[6].s64 = (128 as i64) - ctx.r[6].s64;
	// 831A88CC: 41820018  beq 0x831a88e4
	if ctx.cr[0].eq {
	pc = 0x831A88E4; continue 'dispatch;
	}
	// 831A88D0: 7CA62850  subf r5, r6, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	// 831A88D4: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 831A88D8: 8CC40001  lbzu r6, 1(r4)
	ea = ctx.r[4].u32.wrapping_add(1 as u32);
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[4].u32 = ea;
	// 831A88DC: 9CC30001  stbu r6, 1(r3)
	ea = ctx.r[3].u32.wrapping_add(1 as u32);
	unsafe { crate::rt::store_u8(base as *mut u8, ea, ctx.r[6].u8) };
	ctx.r[3].u32 = ea;
	// 831A88E0: 4200FFF8  bdnz 0x831a88d8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831A88D8; continue 'dispatch;
	}
	// 831A88E4: 54A6C9FE  srwi r6, r5, 7
	ctx.r[6].u32 = ctx.r[5].u32.wrapping_shr(7);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831A88E8: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831A88EC: 4182FFAC  beq 0x831a8898
	if ctx.cr[0].eq {
		sub_831A8888(ctx, base);
		return;
	}
	// 831A88F0: 3945007F  addi r10, r5, 0x7f
	ctx.r[10].s64 = ctx.r[5].s64 + 127;
	// 831A88F4: 54A8067E  clrlwi r8, r5, 0x19
	ctx.r[8].u64 = ctx.r[5].u32 as u64 & 0x0000007Fu64;
	// 831A88F8: 554AC9FE  srwi r10, r10, 7
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(7);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831A88FC: 28880000  cmplwi cr1, r8, 0
	ctx.cr[1].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 831A8900: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 831A8904: 554A077E  clrlwi r10, r10, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 831A8908: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831A890C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 831A8910: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831A8914: 7C09222C  dcbt r9, r4
	// 831A8918: 39290080  addi r9, r9, 0x80
	ctx.r[9].s64 = ctx.r[9].s64 + 128;
	// 831A891C: 4200FFF8  bdnz 0x831a8914
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831A8914; continue 'dispatch;
	}
	// 831A8920: 7D842A14  add r12, r4, r5
	ctx.r[12].u64 = ctx.r[4].u64 + ctx.r[5].u64;
	// 831A8924: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 831A8928: 7D696050  subf r11, r9, r12
	ctx.r[11].s64 = ctx.r[12].s64 - ctx.r[9].s64;
	// 831A892C: 7D832A14  add r12, r3, r5
	ctx.r[12].u64 = ctx.r[3].u64 + ctx.r[5].u64;
	// 831A8930: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 831A8934: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 831A8938: 88E40004  lbz r7, 4(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 831A893C: 89040003  lbz r8, 3(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(3 as u32) ) } as u64;
	// 831A8940: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 831A8944: 5107442E  rlwimi r7, r8, 8, 0x10, 0x17
	ctx.r[7].u64 = (((ctx.r[8].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[7].u64 & 0xFFFFFFFFFFFF00FF);
	// 831A8948: 89240002  lbz r9, 2(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 831A894C: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831A8950: 5127821E  rlwimi r7, r9, 0x10, 8, 0xf
	ctx.r[7].u64 = (((ctx.r[9].u32).rotate_left(16) as u64) & 0x0000000000FF0000) | (ctx.r[7].u64 & 0xFFFFFFFFFF00FFFF);
	// 831A8954: 89440001  lbz r10, 1(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 831A8958: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 831A895C: 5147C00E  rlwimi r7, r10, 0x18, 0, 7
	ctx.r[7].u64 = (((ctx.r[10].u32).rotate_left(24) as u64) & 0x00000000FF000000) | (ctx.r[7].u64 & 0xFFFFFFFF00FFFFFF);
	// 831A8960: 90E30001  stw r7, 1(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1 as u32), ctx.r[7].u32 ) };
	// 831A8964: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 831A8968: 4082FFD0  bne 0x831a8938
	if !ctx.cr[0].eq {
	pc = 0x831A8938; continue 'dispatch;
	}
	// 831A896C: 7C045840  cmplw r4, r11
	ctx.cr[0].compare_u32(ctx.r[11].u32, ctx.r[0].u32, &mut ctx.xer);
	// 831A8970: 40800010  bge 0x831a8980
	if !ctx.cr[0].lt {
		sub_831A8980(ctx, base);
		return;
	}
	// 831A8974: 7C09222C  dcbt r9, r4
	// 831A8978: 4200FFBC  bdnz 0x831a8934
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831A8934; continue 'dispatch;
	}
	// 831A897C: 4BFFFF1C  b 0x831a8898
	sub_831A8888(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A8980 size=24
    let mut pc: u32 = 0x831A8980;
    'dispatch: loop {
        match pc {
            0x831A8980 => {
    //   block [0x831A8980..0x831A8998)
	// 831A8980: 41860010  beq cr1, 0x831a8990
	if ctx.cr[1].eq {
	pc = 0x831A8990; continue 'dispatch;
	}
	// 831A8984: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 831A8988: 7C0861EC  dcbtst r8, r12
	// 831A898C: 28880000  cmplwi cr1, r8, 0
	ctx.cr[1].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 831A8990: 4200FFA4  bdnz 0x831a8934
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			sub_831A88BC(ctx, base);
		return;
	}
	// 831A8994: 4BFFFF04  b 0x831a8898
	sub_831A8888(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A8998 size=16
    let mut pc: u32 = 0x831A8998;
    'dispatch: loop {
        match pc {
            0x831A8998 => {
    //   block [0x831A8998..0x831A89A8)
	// 831A8998: 38050001  addi r0, r5, 1
	ctx.r[0].s64 = ctx.r[5].s64 + 1;
	// 831A899C: 60660000  ori r6, r3, 0
	ctx.r[6].u64 = ctx.r[3].u64 | 0;
	// 831A89A0: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831A89A4: 48000018  b 0x831a89bc
	sub_831A89A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A89A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A89A8 size=80
    let mut pc: u32 = 0x831A89A8;
    'dispatch: loop {
        match pc {
            0x831A89A8 => {
    //   block [0x831A89A8..0x831A89F8)
	// 831A89A8: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 831A89AC: 88040000  lbz r0, 0(r4)
	ctx.r[0].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A89B0: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 831A89B4: 98060000  stb r0, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[0].u8 ) };
	// 831A89B8: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 831A89BC: 70C00003  andi. r0, r6, 3
	ctx.r[0].u64 = ctx.r[6].u64 & 3;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A89C0: 4002FFE8  bdnzf eq, 0x831a89a8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 && !ctx.cr[0].eq {
	pc = 0x831A89A8; continue 'dispatch;
	}
	// 831A89C4: 54A0F0BF  rlwinm. r0, r5, 0x1e, 2, 0x1f
	ctx.r[0].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A89C8: 41C20024  beq- 0x831a89ec
	if ctx.cr[0].eq {
	pc = 0x831A89EC; continue 'dispatch;
	}
	// 831A89CC: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831A89D0: 70800003  andi. r0, r4, 3
	ctx.r[0].u64 = ctx.r[4].u64 & 3;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A89D4: 40C2003C  bne- 0x831a8a10
	if !ctx.cr[0].eq {
		sub_831A8A10(ctx, base);
		return;
	}
	// 831A89D8: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A89DC: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 831A89E0: 90E60000  stw r7, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 831A89E4: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 831A89E8: 4320FFF0  bdnz+ 0x831a89d8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831A89D8; continue 'dispatch;
	}
	// 831A89EC: 70A00003  andi. r0, r5, 3
	ctx.r[0].u64 = ctx.r[5].u64 & 3;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A89F0: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831A89F4: 4DE20020  beqlr+
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A89F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A89F8 size=24
    let mut pc: u32 = 0x831A89F8;
    'dispatch: loop {
        match pc {
            0x831A89F8 => {
    //   block [0x831A89F8..0x831A8A10)
	// 831A89F8: 88040000  lbz r0, 0(r4)
	ctx.r[0].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A89FC: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 831A8A00: 98060000  stb r0, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[0].u8 ) };
	// 831A8A04: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 831A8A08: 4320FFF0  bdnz+ 0x831a89f8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831A89F8; continue 'dispatch;
	}
	// 831A8A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A8A10 size=48
    let mut pc: u32 = 0x831A8A10;
    'dispatch: loop {
        match pc {
            0x831A8A10 => {
    //   block [0x831A8A10..0x831A8A40)
	// 831A8A10: 88E40003  lbz r7, 3(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(3 as u32) ) } as u64;
	// 831A8A14: 89040002  lbz r8, 2(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 831A8A18: 5107442E  rlwimi r7, r8, 8, 0x10, 0x17
	ctx.r[7].u64 = (((ctx.r[8].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[7].u64 & 0xFFFFFFFFFFFF00FF);
	// 831A8A1C: 89240001  lbz r9, 1(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 831A8A20: 5127821E  rlwimi r7, r9, 0x10, 8, 0xf
	ctx.r[7].u64 = (((ctx.r[9].u32).rotate_left(16) as u64) & 0x0000000000FF0000) | (ctx.r[7].u64 & 0xFFFFFFFFFF00FFFF);
	// 831A8A24: 89440000  lbz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A8A28: 5147C00E  rlwimi r7, r10, 0x18, 0, 7
	ctx.r[7].u64 = (((ctx.r[10].u32).rotate_left(24) as u64) & 0x00000000FF000000) | (ctx.r[7].u64 & 0xFFFFFFFF00FFFFFF);
	// 831A8A2C: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 831A8A30: 90E60000  stw r7, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 831A8A34: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 831A8A38: 4200FFD8  bdnz 0x831a8a10
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831A8A10; continue 'dispatch;
	}
	// 831A8A3C: 4BFFFFB0  b 0x831a89ec
	sub_831A89A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A8A40 size=76
    let mut pc: u32 = 0x831A8A40;
    'dispatch: loop {
        match pc {
            0x831A8A40 => {
    //   block [0x831A8A40..0x831A8A8C)
	// 831A8A40: D9CCFF70  stfd f14, -0x90(r12)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-144 as u32), ctx.f[14].u64 ) };
	// 831A8A44: D9ECFF78  stfd f15, -0x88(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-136 as u32), ctx.f[15].u64 ) };
	// 831A8A48: DA0CFF80  stfd f16, -0x80(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-128 as u32), ctx.f[16].u64 ) };
	// 831A8A4C: DA2CFF88  stfd f17, -0x78(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-120 as u32), ctx.f[17].u64 ) };
	// 831A8A50: DA4CFF90  stfd f18, -0x70(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-112 as u32), ctx.f[18].u64 ) };
	// 831A8A54: DA6CFF98  stfd f19, -0x68(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-104 as u32), ctx.f[19].u64 ) };
	// 831A8A58: DA8CFFA0  stfd f20, -0x60(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-96 as u32), ctx.f[20].u64 ) };
	// 831A8A5C: DAACFFA8  stfd f21, -0x58(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-88 as u32), ctx.f[21].u64 ) };
	// 831A8A60: DACCFFB0  stfd f22, -0x50(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-80 as u32), ctx.f[22].u64 ) };
	// 831A8A64: DAECFFB8  stfd f23, -0x48(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-72 as u32), ctx.f[23].u64 ) };
	// 831A8A68: DB0CFFC0  stfd f24, -0x40(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-64 as u32), ctx.f[24].u64 ) };
	// 831A8A6C: DB2CFFC8  stfd f25, -0x38(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-56 as u32), ctx.f[25].u64 ) };
	// 831A8A70: DB4CFFD0  stfd f26, -0x30(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-48 as u32), ctx.f[26].u64 ) };
	// 831A8A74: DB6CFFD8  stfd f27, -0x28(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-40 as u32), ctx.f[27].u64 ) };
	// 831A8A78: DB8CFFE0  stfd f28, -0x20(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-32 as u32), ctx.f[28].u64 ) };
	// 831A8A7C: DBACFFE8  stfd f29, -0x18(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-24 as u32), ctx.f[29].u64 ) };
	// 831A8A80: DBCCFFF0  stfd f30, -0x10(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-16 as u32), ctx.f[30].u64 ) };
	// 831A8A84: DBECFFF8  stfd f31, -8(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-8 as u32), ctx.f[31].u64 ) };
	// 831A8A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8A8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A8A8C size=76
    let mut pc: u32 = 0x831A8A8C;
    'dispatch: loop {
        match pc {
            0x831A8A8C => {
    //   block [0x831A8A8C..0x831A8AD8)
	// 831A8A8C: C9CCFF70  lfd f14, -0x90(r12)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[14].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-144 as u32) ) };
	// 831A8A90: C9ECFF78  lfd f15, -0x88(r12)
	ctx.f[15].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-136 as u32) ) };
	// 831A8A94: CA0CFF80  lfd f16, -0x80(r12)
	ctx.f[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-128 as u32) ) };
	// 831A8A98: CA2CFF88  lfd f17, -0x78(r12)
	ctx.f[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-120 as u32) ) };
	// 831A8A9C: CA4CFF90  lfd f18, -0x70(r12)
	ctx.f[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-112 as u32) ) };
	// 831A8AA0: CA6CFF98  lfd f19, -0x68(r12)
	ctx.f[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-104 as u32) ) };
	// 831A8AA4: CA8CFFA0  lfd f20, -0x60(r12)
	ctx.f[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-96 as u32) ) };
	// 831A8AA8: CAACFFA8  lfd f21, -0x58(r12)
	ctx.f[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-88 as u32) ) };
	// 831A8AAC: CACCFFB0  lfd f22, -0x50(r12)
	ctx.f[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-80 as u32) ) };
	// 831A8AB0: CAECFFB8  lfd f23, -0x48(r12)
	ctx.f[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-72 as u32) ) };
	// 831A8AB4: CB0CFFC0  lfd f24, -0x40(r12)
	ctx.f[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-64 as u32) ) };
	// 831A8AB8: CB2CFFC8  lfd f25, -0x38(r12)
	ctx.f[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-56 as u32) ) };
	// 831A8ABC: CB4CFFD0  lfd f26, -0x30(r12)
	ctx.f[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-48 as u32) ) };
	// 831A8AC0: CB6CFFD8  lfd f27, -0x28(r12)
	ctx.f[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-40 as u32) ) };
	// 831A8AC4: CB8CFFE0  lfd f28, -0x20(r12)
	ctx.f[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-32 as u32) ) };
	// 831A8AC8: CBACFFE8  lfd f29, -0x18(r12)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-24 as u32) ) };
	// 831A8ACC: CBCCFFF0  lfd f30, -0x10(r12)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-16 as u32) ) };
	// 831A8AD0: CBECFFF8  lfd f31, -8(r12)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-8 as u32) ) };
	// 831A8AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A8AD8 size=228
    let mut pc: u32 = 0x831A8AD8;
    'dispatch: loop {
        match pc {
            0x831A8AD8 => {
    //   block [0x831A8AD8..0x831A8BBC)
	// 831A8AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A8ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A8AE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831A8AE4: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 831A8AE8: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 831A8AEC: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 831A8AF0: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 831A8AF4: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 831A8AF8: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 831A8AFC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A8B00: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831A8B04: 409A0030  bne cr6, 0x831a8b34
	if !ctx.cr[6].eq {
	pc = 0x831A8B34; continue 'dispatch;
	}
	// 831A8B08: 48008259  bl 0x831b0d60
	ctx.lr = 0x831A8B0C;
	sub_831B0D60(ctx, base);
	// 831A8B0C: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 831A8B10: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831A8B14: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831A8B18: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831A8B1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831A8B20: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831A8B24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A8B28: 4800C2E9  bl 0x831b4e10
	ctx.lr = 0x831A8B2C;
	sub_831B4E10(ctx, base);
	// 831A8B2C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831A8B30: 48000078  b 0x831a8ba8
	pc = 0x831A8BA8; continue 'dispatch;
	// 831A8B34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831A8B38: 419AFFD0  beq cr6, 0x831a8b08
	if ctx.cr[6].eq {
	pc = 0x831A8B08; continue 'dispatch;
	}
	// 831A8B3C: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 831A8B40: 90610068  stw r3, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 831A8B44: 39400042  li r10, 0x42
	ctx.r[10].s64 = 66;
	// 831A8B48: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 831A8B4C: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 831A8B50: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 831A8B54: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 831A8B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 831A8B5C: 396100B0  addi r11, r1, 0xb0
	ctx.r[11].s64 = ctx.r[1].s64 + 176;
	// 831A8B60: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831A8B64: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 831A8B68: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831A8B6C: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831A8B70: 4800B719  bl 0x831b4288
	ctx.lr = 0x831A8B74;
	sub_831B4288(ctx, base);
	// 831A8B74: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 831A8B78: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831A8B7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831A8B80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 831A8B84: 41800014  blt 0x831a8b98
	if ctx.cr[0].lt {
	pc = 0x831A8B98; continue 'dispatch;
	}
	// 831A8B88: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 831A8B8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831A8B90: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 831A8B94: 48000010  b 0x831a8ba4
	pc = 0x831A8BA4; continue 'dispatch;
	// 831A8B98: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 831A8B9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A8BA0: 4800B481  bl 0x831b4020
	ctx.lr = 0x831A8BA4;
	sub_831B4020(ctx, base);
	// 831A8BA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831A8BA8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831A8BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A8BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A8BB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831A8BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A8BC0 size=72
    let mut pc: u32 = 0x831A8BC0;
    'dispatch: loop {
        match pc {
            0x831A8BC0 => {
    //   block [0x831A8BC0..0x831A8C08)
	// 831A8BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A8BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A8BC8: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 831A8BCC: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 831A8BD0: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 831A8BD4: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 831A8BD8: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 831A8BDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A8BE0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 831A8BE4: 39410088  addi r10, r1, 0x88
	ctx.r[10].s64 = ctx.r[1].s64 + 136;
	// 831A8BE8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831A8BEC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831A8BF0: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831A8BF4: 48009F1D  bl 0x831b2b10
	ctx.lr = 0x831A8BF8;
	sub_831B2B10(ctx, base);
	// 831A8BF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831A8BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A8C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A8C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A8C08 size=4
    let mut pc: u32 = 0x831A8C08;
    'dispatch: loop {
        match pc {
            0x831A8C08 => {
    //   block [0x831A8C08..0x831A8C0C)
	// 831A8C08: 4B117660  b 0x822c0268
	sub_822C0268(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A8C10 size=32
    let mut pc: u32 = 0x831A8C10;
    'dispatch: loop {
        match pc {
            0x831A8C10 => {
    //   block [0x831A8C10..0x831A8C30)
	// 831A8C10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831A8C14: A1440000  lhz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A8C18: 38840002  addi r4, r4, 2
	ctx.r[4].s64 = ctx.r[4].s64 + 2;
	// 831A8C1C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831A8C20: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 831A8C24: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831A8C28: 4082FFEC  bne 0x831a8c14
	if !ctx.cr[0].eq {
	pc = 0x831A8C14; continue 'dispatch;
	}
	// 831A8C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A8C30 size=212
    let mut pc: u32 = 0x831A8C30;
    'dispatch: loop {
        match pc {
            0x831A8C30 => {
    //   block [0x831A8C30..0x831A8D04)
	// 831A8C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A8C34: 4BFFF539  bl 0x831a816c
	ctx.lr = 0x831A8C38;
	sub_831A8130(ctx, base);
	// 831A8C38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A8C3C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 831A8C40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831A8C44: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831A8C48: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831A8C4C: 409A000C  bne cr6, 0x831a8c58
	if !ctx.cr[6].eq {
	pc = 0x831A8C58; continue 'dispatch;
	}
	// 831A8C50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A8C54: 48000034  b 0x831a8c88
	pc = 0x831A8C88; continue 'dispatch;
	// 831A8C58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831A8C5C: 409A0034  bne cr6, 0x831a8c90
	if !ctx.cr[6].eq {
	pc = 0x831A8C90; continue 'dispatch;
	}
	// 831A8C60: 48008101  bl 0x831b0d60
	ctx.lr = 0x831A8C64;
	sub_831B0D60(ctx, base);
	// 831A8C64: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 831A8C68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831A8C6C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831A8C70: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831A8C74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831A8C78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831A8C7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A8C80: 4800C191  bl 0x831b4e10
	ctx.lr = 0x831A8C84;
	sub_831B4E10(ctx, base);
	// 831A8C84: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 831A8C88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831A8C8C: 4BFFF530  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831A8C90: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 831A8C94: 419A001C  beq cr6, 0x831a8cb0
	if ctx.cr[6].eq {
	pc = 0x831A8CB0; continue 'dispatch;
	}
	// 831A8C98: 7F1EF840  cmplw cr6, r30, r31
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[31].u32, &mut ctx.xer);
	// 831A8C9C: 41980014  blt cr6, 0x831a8cb0
	if ctx.cr[6].lt {
	pc = 0x831A8CB0; continue 'dispatch;
	}
	// 831A8CA0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831A8CA4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831A8CA8: 4BFFF869  bl 0x831a8510
	ctx.lr = 0x831A8CAC;
	sub_831A8510(ctx, base);
	// 831A8CAC: 4BFFFFA4  b 0x831a8c50
	pc = 0x831A8C50; continue 'dispatch;
	// 831A8CB0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 831A8CB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831A8CB8: 4BFFF529  bl 0x831a81e0
	ctx.lr = 0x831A8CBC;
	sub_831A81E0(ctx, base);
	// 831A8CBC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 831A8CC0: 409A0030  bne cr6, 0x831a8cf0
	if !ctx.cr[6].eq {
	pc = 0x831A8CF0; continue 'dispatch;
	}
	// 831A8CC4: 4800809D  bl 0x831b0d60
	ctx.lr = 0x831A8CC8;
	sub_831B0D60(ctx, base);
	// 831A8CC8: 3BE00016  li r31, 0x16
	ctx.r[31].s64 = 22;
	// 831A8CCC: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 831A8CD0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831A8CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831A8CD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831A8CDC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831A8CE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A8CE4: 4800C12D  bl 0x831b4e10
	ctx.lr = 0x831A8CE8;
	sub_831B4E10(ctx, base);
	// 831A8CE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831A8CEC: 4BFFFF9C  b 0x831a8c88
	pc = 0x831A8C88; continue 'dispatch;
	// 831A8CF0: 7F1EF840  cmplw cr6, r30, r31
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[31].u32, &mut ctx.xer);
	// 831A8CF4: 4098FF90  bge cr6, 0x831a8c84
	if !ctx.cr[6].lt {
	pc = 0x831A8C84; continue 'dispatch;
	}
	// 831A8CF8: 48008069  bl 0x831b0d60
	ctx.lr = 0x831A8CFC;
	sub_831B0D60(ctx, base);
	// 831A8CFC: 3BE00022  li r31, 0x22
	ctx.r[31].s64 = 34;
	// 831A8D00: 4BFFFFCC  b 0x831a8ccc
	pc = 0x831A8CCC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A8D08 size=188
    let mut pc: u32 = 0x831A8D08;
    'dispatch: loop {
        match pc {
            0x831A8D08 => {
    //   block [0x831A8D08..0x831A8DC4)
	// 831A8D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A8D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A8D10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831A8D14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A8D18: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 831A8D1C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 831A8D20: 419A008C  beq cr6, 0x831a8dac
	if ctx.cr[6].eq {
	pc = 0x831A8DAC; continue 'dispatch;
	}
	// 831A8D24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831A8D28: 409A0030  bne cr6, 0x831a8d58
	if !ctx.cr[6].eq {
	pc = 0x831A8D58; continue 'dispatch;
	}
	// 831A8D2C: 48008035  bl 0x831b0d60
	ctx.lr = 0x831A8D30;
	sub_831B0D60(ctx, base);
	// 831A8D30: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 831A8D34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831A8D38: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831A8D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831A8D40: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831A8D44: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831A8D48: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A8D4C: 4800C0C5  bl 0x831b4e10
	ctx.lr = 0x831A8D50;
	sub_831B4E10(ctx, base);
	// 831A8D50: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 831A8D54: 4800005C  b 0x831a8db0
	pc = 0x831A8DB0; continue 'dispatch;
	// 831A8D58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831A8D5C: 409A0030  bne cr6, 0x831a8d8c
	if !ctx.cr[6].eq {
	pc = 0x831A8D8C; continue 'dispatch;
	}
	// 831A8D60: 48008001  bl 0x831b0d60
	ctx.lr = 0x831A8D64;
	sub_831B0D60(ctx, base);
	// 831A8D64: 3BE00016  li r31, 0x16
	ctx.r[31].s64 = 22;
	// 831A8D68: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 831A8D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831A8D70: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831A8D74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831A8D78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831A8D7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A8D80: 4800C091  bl 0x831b4e10
	ctx.lr = 0x831A8D84;
	sub_831B4E10(ctx, base);
	// 831A8D84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831A8D88: 48000028  b 0x831a8db0
	pc = 0x831A8DB0; continue 'dispatch;
	// 831A8D8C: 7F043040  cmplw cr6, r4, r6
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[6].u32, &mut ctx.xer);
	// 831A8D90: 40980010  bge cr6, 0x831a8da0
	if !ctx.cr[6].lt {
	pc = 0x831A8DA0; continue 'dispatch;
	}
	// 831A8D94: 48007FCD  bl 0x831b0d60
	ctx.lr = 0x831A8D98;
	sub_831B0D60(ctx, base);
	// 831A8D98: 3BE00022  li r31, 0x22
	ctx.r[31].s64 = 34;
	// 831A8D9C: 4BFFFFCC  b 0x831a8d68
	pc = 0x831A8D68; continue 'dispatch;
	// 831A8DA0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 831A8DA4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 831A8DA8: 480045A9  bl 0x831ad350
	ctx.lr = 0x831A8DAC;
	sub_831AD350(ctx, base);
	// 831A8DAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A8DB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831A8DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A8DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A8DBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831A8DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831A8DC8 size=224
    let mut pc: u32 = 0x831A8DC8;
    'dispatch: loop {
        match pc {
            0x831A8DC8 => {
    //   block [0x831A8DC8..0x831A8EA8)
	// 831A8DC8: DBC1FFF0  stfd f30, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[30].u64 ) };
	// 831A8DCC: DBE1FFF8  stfd f31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.f[31].u64 ) };
	// 831A8DD0: FC000A10  fabs f0, f1
	ctx.f[0].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 831A8DD4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831A8DD8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 831A8DDC: D801FFE8  stfd f0, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[0].u64 ) };
	// 831A8DE0: 396BEC80  addi r11, r11, -0x1380
	ctx.r[11].s64 = ctx.r[11].s64 + -4992;
	// 831A8DE4: CBCAE3A0  lfd f30, -0x1c60(r10)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-7264 as u32) ) };
	// 831A8DE8: C96B0008  lfd f11, 8(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 831A8DEC: C18B0020  lfs f12, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831A8DF0: C94B0028  lfd f10, 0x28(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 831A8DF4: FDAB0032  fmul f13, f11, f0
	ctx.f[13].f64 = ctx.f[11].f64 * ctx.f[0].f64;
	// 831A8DF8: C92B0030  lfd f9, 0x30(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 831A8DFC: C90B0070  lfd f8, 0x70(r11)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) };
	// 831A8E00: C8EB0068  lfd f7, 0x68(r11)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) };
	// 831A8E04: C8CB0060  lfd f6, 0x60(r11)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	// 831A8E08: C8AB0058  lfd f5, 0x58(r11)
	ctx.f[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	// 831A8E0C: C88B0050  lfd f4, 0x50(r11)
	ctx.f[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 831A8E10: C86B0048  lfd f3, 0x48(r11)
	ctx.f[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) };
	// 831A8E14: C84B0040  lfd f2, 0x40(r11)
	ctx.f[2].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 831A8E18: CBEB0038  lfd f31, 0x38(r11)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 831A8E1C: FD606E5C  fctid f11, f13
	ctx.f[11].s64 = if ctx.f[13].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[13].f64 as i64 };
	// 831A8E20: C1AB001C  lfs f13, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831A8E24: FD81636E  fsel f12, f1, f13, f12
	ctx.f[12].f64 = if ctx.f[1].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[12].f64 };
	// 831A8E28: FDA05E9C  fcfid f13, f11
	ctx.f[13].f64 = (ctx.f[11].s64 as f64);
	// 831A8E2C: FD6A037C  fnmsub f11, f10, f13, f0
	ctx.f[11].f64 = -(ctx.f[10].f64 * ctx.f[13].f64 - ctx.f[0].f64);
	// 831A8E30: FD406E5E  fctidz f10, f13
	ctx.f[10].s64 = if ctx.f[13].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[13].f64.trunc() as i64 };
	// 831A8E34: D941FFE0  stfd f10, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[10].u64 ) };
	// 831A8E38: E921FFE0  ld r9, -0x20(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 831A8E3C: 792807E0  clrldi r8, r9, 0x3f
	ctx.r[8].u64 = ctx.r[9].u64 & 0x0000000000000001u64;
	// 831A8E40: FD295B7C  fnmsub f9, f9, f13, f11
	ctx.f[9].f64 = -(ctx.f[9].f64 * ctx.f[13].f64 - ctx.f[11].f64);
	// 831A8E44: 2F280000  cmpdi cr6, r8, 0
	ctx.cr[6].compare_i64(ctx.r[8].s64, 0, &mut ctx.xer);
	// 831A8E48: FDA90272  fmul f13, f9, f9
	ctx.f[13].f64 = ctx.f[9].f64 * ctx.f[9].f64;
	// 831A8E4C: FD683B7A  fmadd f11, f8, f13, f7
	ctx.f[11].f64 = ctx.f[8].f64 * ctx.f[13].f64 + ctx.f[7].f64;
	// 831A8E50: FD4B337A  fmadd f10, f11, f13, f6
	ctx.f[10].f64 = ctx.f[11].f64 * ctx.f[13].f64 + ctx.f[6].f64;
	// 831A8E54: FD0A2B7A  fmadd f8, f10, f13, f5
	ctx.f[8].f64 = ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[5].f64;
	// 831A8E58: FCE8237A  fmadd f7, f8, f13, f4
	ctx.f[7].f64 = ctx.f[8].f64 * ctx.f[13].f64 + ctx.f[4].f64;
	// 831A8E5C: FCC71B7A  fmadd f6, f7, f13, f3
	ctx.f[6].f64 = ctx.f[7].f64 * ctx.f[13].f64 + ctx.f[3].f64;
	// 831A8E60: FCA6137A  fmadd f5, f6, f13, f2
	ctx.f[5].f64 = ctx.f[6].f64 * ctx.f[13].f64 + ctx.f[2].f64;
	// 831A8E64: FC85FB7A  fmadd f4, f5, f13, f31
	ctx.f[4].f64 = ctx.f[5].f64 * ctx.f[13].f64 + ctx.f[31].f64;
	// 831A8E68: FC64F37A  fmadd f3, f4, f13, f30
	ctx.f[3].f64 = ctx.f[4].f64 * ctx.f[13].f64 + ctx.f[30].f64;
	// 831A8E6C: FDA30272  fmul f13, f3, f9
	ctx.f[13].f64 = ctx.f[3].f64 * ctx.f[9].f64;
	// 831A8E70: 419A0008  beq cr6, 0x831a8e78
	if ctx.cr[6].eq {
	pc = 0x831A8E78; continue 'dispatch;
	}
	// 831A8E74: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 831A8E78: E941FFE8  ld r10, -0x18(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831A8E7C: FD8D0332  fmul f12, f13, f12
	ctx.f[12].f64 = ctx.f[13].f64 * ctx.f[12].f64;
	// 831A8E80: 2F2A0000  cmpdi cr6, r10, 0
	ctx.cr[6].compare_i64(ctx.r[10].s64, 0, &mut ctx.xer);
	// 831A8E84: 419A0018  beq cr6, 0x831a8e9c
	if ctx.cr[6].eq {
	pc = 0x831A8E9C; continue 'dispatch;
	}
	// 831A8E88: C9AB0010  lfd f13, 0x10(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 831A8E8C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831A8E90: FDA06828  fsub f13, f0, f13
	ctx.f[13].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 831A8E94: C80BF470  lfd f0, -0xb90(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-2960 as u32) ) };
	// 831A8E98: FC2D602E  fsel f1, f13, f0, f12
	ctx.f[1].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[12].f64 };
	// 831A8E9C: CBC1FFF0  lfd f30, -0x10(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831A8EA0: CBE1FFF8  lfd f31, -8(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 831A8EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831A8EA8 size=224
    let mut pc: u32 = 0x831A8EA8;
    'dispatch: loop {
        match pc {
            0x831A8EA8 => {
    //   block [0x831A8EA8..0x831A8F88)
	// 831A8EA8: DBE1FFF8  stfd f31, -8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.f[31].u64 ) };
	// 831A8EAC: FC000A10  fabs f0, f1
	ctx.f[0].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 831A8EB0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831A8EB4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 831A8EB8: 396BEC80  addi r11, r11, -0x1380
	ctx.r[11].s64 = ctx.r[11].s64 + -4992;
	// 831A8EBC: CBEAE3A0  lfd f31, -0x1c60(r10)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-7264 as u32) ) };
	// 831A8EC0: C9AB0000  lfd f13, 0(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 831A8EC4: C16B0024  lfs f11, 0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 831A8EC8: C94B0028  lfd f10, 0x28(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 831A8ECC: FD8D002A  fadd f12, f13, f0
	ctx.f[12].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	// 831A8ED0: C9AB0008  lfd f13, 8(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 831A8ED4: C92B0030  lfd f9, 0x30(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 831A8ED8: C90B0070  lfd f8, 0x70(r11)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) };
	// 831A8EDC: C8EB0068  lfd f7, 0x68(r11)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) };
	// 831A8EE0: C8CB0060  lfd f6, 0x60(r11)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	// 831A8EE4: C8AB0058  lfd f5, 0x58(r11)
	ctx.f[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	// 831A8EE8: C88B0050  lfd f4, 0x50(r11)
	ctx.f[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 831A8EEC: C86B0048  lfd f3, 0x48(r11)
	ctx.f[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) };
	// 831A8EF0: C84B0040  lfd f2, 0x40(r11)
	ctx.f[2].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 831A8EF4: C82B0038  lfd f1, 0x38(r11)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 831A8EF8: FDAD0332  fmul f13, f13, f12
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[12].f64;
	// 831A8EFC: FDA06E5C  fctid f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[13].f64 as i64 };
	// 831A8F00: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 831A8F04: FD6D5828  fsub f11, f13, f11
	ctx.f[11].f64 = ctx.f[13].f64 - ctx.f[11].f64;
	// 831A8F08: FDA06E5E  fctidz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[13].f64.trunc() as i64 };
	// 831A8F0C: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 831A8F10: E921FFF0  ld r9, -0x10(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831A8F14: 792807E0  clrldi r8, r9, 0x3f
	ctx.r[8].u64 = ctx.r[9].u64 & 0x0000000000000001u64;
	// 831A8F18: FD4A02FC  fnmsub f10, f10, f11, f0
	ctx.f[10].f64 = -(ctx.f[10].f64 * ctx.f[11].f64 - ctx.f[0].f64);
	// 831A8F1C: 2F280000  cmpdi cr6, r8, 0
	ctx.cr[6].compare_i64(ctx.r[8].s64, 0, &mut ctx.xer);
	// 831A8F20: FD2952FC  fnmsub f9, f9, f11, f10
	ctx.f[9].f64 = -(ctx.f[9].f64 * ctx.f[11].f64 - ctx.f[10].f64);
	// 831A8F24: FDA90272  fmul f13, f9, f9
	ctx.f[13].f64 = ctx.f[9].f64 * ctx.f[9].f64;
	// 831A8F28: FD683B7A  fmadd f11, f8, f13, f7
	ctx.f[11].f64 = ctx.f[8].f64 * ctx.f[13].f64 + ctx.f[7].f64;
	// 831A8F2C: FD4B337A  fmadd f10, f11, f13, f6
	ctx.f[10].f64 = ctx.f[11].f64 * ctx.f[13].f64 + ctx.f[6].f64;
	// 831A8F30: FD0A2B7A  fmadd f8, f10, f13, f5
	ctx.f[8].f64 = ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[5].f64;
	// 831A8F34: FCE8237A  fmadd f7, f8, f13, f4
	ctx.f[7].f64 = ctx.f[8].f64 * ctx.f[13].f64 + ctx.f[4].f64;
	// 831A8F38: FCC71B7A  fmadd f6, f7, f13, f3
	ctx.f[6].f64 = ctx.f[7].f64 * ctx.f[13].f64 + ctx.f[3].f64;
	// 831A8F3C: FCA6137A  fmadd f5, f6, f13, f2
	ctx.f[5].f64 = ctx.f[6].f64 * ctx.f[13].f64 + ctx.f[2].f64;
	// 831A8F40: FC850B7A  fmadd f4, f5, f13, f1
	ctx.f[4].f64 = ctx.f[5].f64 * ctx.f[13].f64 + ctx.f[1].f64;
	// 831A8F44: FC64FB7A  fmadd f3, f4, f13, f31
	ctx.f[3].f64 = ctx.f[4].f64 * ctx.f[13].f64 + ctx.f[31].f64;
	// 831A8F48: FDA30272  fmul f13, f3, f9
	ctx.f[13].f64 = ctx.f[3].f64 * ctx.f[9].f64;
	// 831A8F4C: 419A0008  beq cr6, 0x831a8f54
	if ctx.cr[6].eq {
	pc = 0x831A8F54; continue 'dispatch;
	}
	// 831A8F50: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 831A8F54: C16B0018  lfs f11, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 831A8F58: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 831A8F5C: 409A0010  bne cr6, 0x831a8f6c
	if !ctx.cr[6].eq {
	pc = 0x831A8F6C; continue 'dispatch;
	}
	// 831A8F60: C02B001C  lfs f1, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 831A8F64: CBE1FFF8  lfd f31, -8(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 831A8F68: 4E800020  blr
	return;
	// 831A8F6C: C80B0010  lfd f0, 0x10(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 831A8F70: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831A8F74: FD8C0028  fsub f12, f12, f0
	ctx.f[12].f64 = ctx.f[12].f64 - ctx.f[0].f64;
	// 831A8F78: C80BF470  lfd f0, -0xb90(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-2960 as u32) ) };
	// 831A8F7C: FC2C682E  fsel f1, f12, f0, f13
	ctx.f[1].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 831A8F80: CBE1FFF8  lfd f31, -8(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 831A8F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A8F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A8F88 size=304
    let mut pc: u32 = 0x831A8F88;
    'dispatch: loop {
        match pc {
            0x831A8F88 => {
    //   block [0x831A8F88..0x831A90B8)
	// 831A8F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A8F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A8F90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831A8F94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831A8F98: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A8F9C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 831A8FA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831A8FA4: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 831A8FA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831A8FAC: 409A0030  bne cr6, 0x831a8fdc
	if !ctx.cr[6].eq {
	pc = 0x831A8FDC; continue 'dispatch;
	}
	// 831A8FB0: 48007DB1  bl 0x831b0d60
	ctx.lr = 0x831A8FB4;
	sub_831B0D60(ctx, base);
	// 831A8FB4: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 831A8FB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831A8FBC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831A8FC0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831A8FC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831A8FC8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831A8FCC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A8FD0: 4800BE41  bl 0x831b4e10
	ctx.lr = 0x831A8FD4;
	sub_831B4E10(ctx, base);
	// 831A8FD4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831A8FD8: 480000C8  b 0x831a90a0
	pc = 0x831A90A0; continue 'dispatch;
	// 831A8FDC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831A8FE0: 419A000C  beq cr6, 0x831a8fec
	if ctx.cr[6].eq {
	pc = 0x831A8FEC; continue 'dispatch;
	}
	// 831A8FE4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831A8FE8: 419AFFC8  beq cr6, 0x831a8fb0
	if ctx.cr[6].eq {
	pc = 0x831A8FB0; continue 'dispatch;
	}
	// 831A8FEC: 3D403FFF  lis r10, 0x3fff
	ctx.r[10].s64 = 1073676288;
	// 831A8FF0: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 831A8FF4: 39200042  li r9, 0x42
	ctx.r[9].s64 = 66;
	// 831A8FF8: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 831A8FFC: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 831A9000: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 831A9004: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831A9008: 40990010  ble cr6, 0x831a9018
	if !ctx.cr[6].gt {
	pc = 0x831A9018; continue 'dispatch;
	}
	// 831A900C: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 831A9010: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 831A9014: 48000008  b 0x831a901c
	pc = 0x831A901C; continue 'dispatch;
	// 831A9018: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831A901C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 831A9020: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 831A9024: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 831A9028: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831A902C: 4800C0AD  bl 0x831b50d8
	ctx.lr = 0x831A9030;
	sub_831B50D8(ctx, base);
	// 831A9030: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831A9034: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831A9038: 419A0064  beq cr6, 0x831a909c
	if ctx.cr[6].eq {
	pc = 0x831A909C; continue 'dispatch;
	}
	// 831A903C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831A9040: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831A9044: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831A9048: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831A904C: 4180001C  blt 0x831a9068
	if ctx.cr[0].lt {
	pc = 0x831A9068; continue 'dispatch;
	}
	// 831A9050: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831A9054: 9BEB0000  stb r31, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u8 ) };
	// 831A9058: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831A905C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831A9060: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 831A9064: 48000010  b 0x831a9074
	pc = 0x831A9074; continue 'dispatch;
	// 831A9068: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831A906C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A9070: 4800AFB1  bl 0x831b4020
	ctx.lr = 0x831A9074;
	sub_831B4020(ctx, base);
	// 831A9074: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831A9078: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831A907C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831A9080: 41800010  blt 0x831a9090
	if ctx.cr[0].lt {
	pc = 0x831A9090; continue 'dispatch;
	}
	// 831A9084: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831A9088: 9BEB0000  stb r31, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u8 ) };
	// 831A908C: 48000010  b 0x831a909c
	pc = 0x831A909C; continue 'dispatch;
	// 831A9090: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831A9094: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A9098: 4800AF89  bl 0x831b4020
	ctx.lr = 0x831A909C;
	sub_831B4020(ctx, base);
	// 831A909C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831A90A0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831A90A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A90A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A90AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831A90B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831A90B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A90B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A90B8 size=12
    let mut pc: u32 = 0x831A90B8;
    'dispatch: loop {
        match pc {
            0x831A90B8 => {
    //   block [0x831A90B8..0x831A90C4)
	// 831A90B8: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 831A90BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831A90C0: 4BFFFEC8  b 0x831a8f88
	sub_831A8F88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A90C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A90C8 size=36
    let mut pc: u32 = 0x831A90C8;
    'dispatch: loop {
        match pc {
            0x831A90C8 => {
    //   block [0x831A90C8..0x831A90EC)
	// 831A90C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831A90CC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A90D0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831A90D4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831A90D8: 4082FFF4  bne 0x831a90cc
	if !ctx.cr[0].eq {
	pc = 0x831A90CC; continue 'dispatch;
	}
	// 831A90DC: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 831A90E0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 831A90E4: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 831A90E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A90F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A90F0 size=244
    let mut pc: u32 = 0x831A90F0;
    'dispatch: loop {
        match pc {
            0x831A90F0 => {
    //   block [0x831A90F0..0x831A91E4)
	// 831A90F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A90F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A90F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831A90FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831A9100: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A9104: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831A9108: 409A0030  bne cr6, 0x831a9138
	if !ctx.cr[6].eq {
	pc = 0x831A9138; continue 'dispatch;
	}
	// 831A910C: 48007C55  bl 0x831b0d60
	ctx.lr = 0x831A9110;
	sub_831B0D60(ctx, base);
	// 831A9110: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 831A9114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831A9118: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831A911C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831A9120: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831A9124: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831A9128: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A912C: 4800BCE5  bl 0x831b4e10
	ctx.lr = 0x831A9130;
	sub_831B4E10(ctx, base);
	// 831A9130: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831A9134: 48000098  b 0x831a91cc
	pc = 0x831A91CC; continue 'dispatch;
	// 831A9138: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831A913C: 419AFFD0  beq cr6, 0x831a910c
	if ctx.cr[6].eq {
	pc = 0x831A910C; continue 'dispatch;
	}
	// 831A9140: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 831A9144: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 831A9148: 39400042  li r10, 0x42
	ctx.r[10].s64 = 66;
	// 831A914C: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 831A9150: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 831A9154: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 831A9158: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831A915C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831A9160: 4800BF79  bl 0x831b50d8
	ctx.lr = 0x831A9164;
	sub_831B50D8(ctx, base);
	// 831A9164: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831A9168: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831A916C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831A9170: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831A9174: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831A9178: 4180001C  blt 0x831a9194
	if ctx.cr[0].lt {
	pc = 0x831A9194; continue 'dispatch;
	}
	// 831A917C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831A9180: 9BEB0000  stb r31, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u8 ) };
	// 831A9184: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831A9188: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831A918C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 831A9190: 48000010  b 0x831a91a0
	pc = 0x831A91A0; continue 'dispatch;
	// 831A9194: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831A9198: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A919C: 4800AE85  bl 0x831b4020
	ctx.lr = 0x831A91A0;
	sub_831B4020(ctx, base);
	// 831A91A0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831A91A4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831A91A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831A91AC: 41800010  blt 0x831a91bc
	if ctx.cr[0].lt {
	pc = 0x831A91BC; continue 'dispatch;
	}
	// 831A91B0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831A91B4: 9BEB0000  stb r31, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u8 ) };
	// 831A91B8: 48000010  b 0x831a91c8
	pc = 0x831A91C8; continue 'dispatch;
	// 831A91BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831A91C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A91C4: 4800AE5D  bl 0x831b4020
	ctx.lr = 0x831A91C8;
	sub_831B4020(ctx, base);
	// 831A91C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831A91CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831A91D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A91D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A91D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831A91DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831A91E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A91E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A91E8 size=12
    let mut pc: u32 = 0x831A91E8;
    'dispatch: loop {
        match pc {
            0x831A91E8 => {
    //   block [0x831A91E8..0x831A91F4)
	// 831A91E8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 831A91EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831A91F0: 4BFFFF00  b 0x831a90f0
	sub_831A90F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A91F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A91F8 size=8
    let mut pc: u32 = 0x831A91F8;
    'dispatch: loop {
        match pc {
            0x831A91F8 => {
    //   block [0x831A91F8..0x831A9200)
	// 831A91F8: 2F030041  cmpwi cr6, r3, 0x41
	ctx.cr[6].compare_i32(ctx.r[3].s32, 65, &mut ctx.xer);
	// 831A91FC: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A9200 size=8
    let mut pc: u32 = 0x831A9200;
    'dispatch: loop {
        match pc {
            0x831A9200 => {
    //   block [0x831A9200..0x831A9208)
	// 831A9200: 2F03005A  cmpwi cr6, r3, 0x5a
	ctx.cr[6].compare_i32(ctx.r[3].s32, 90, &mut ctx.xer);
	// 831A9204: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A9208 size=8
    let mut pc: u32 = 0x831A9208;
    'dispatch: loop {
        match pc {
            0x831A9208 => {
    //   block [0x831A9208..0x831A9210)
	// 831A9208: 38630020  addi r3, r3, 0x20
	ctx.r[3].s64 = ctx.r[3].s64 + 32;
	// 831A920C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A9210 size=48
    let mut pc: u32 = 0x831A9210;
    'dispatch: loop {
        match pc {
            0x831A9210 => {
    //   block [0x831A9210..0x831A9240)
	// 831A9210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A9214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A9218: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831A921C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A9220: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831A9224: 48003E75  bl 0x831ad098
	ctx.lr = 0x831A9228;
	sub_831AD098(ctx, base);
	// 831A9228: 93E30090  stw r31, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 831A922C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831A9230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A9234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A9238: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831A923C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A9240 size=36
    let mut pc: u32 = 0x831A9240;
    'dispatch: loop {
        match pc {
            0x831A9240 => {
    //   block [0x831A9240..0x831A9264)
	// 831A9240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A9244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A9248: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A924C: 48003E4D  bl 0x831ad098
	ctx.lr = 0x831A9250;
	sub_831AD098(ctx, base);
	// 831A9250: 80630090  lwz r3, 0x90(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 831A9254: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831A9258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A925C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A9260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A9268 size=12
    let mut pc: u32 = 0x831A9268;
    'dispatch: loop {
        match pc {
            0x831A9268 => {
    //   block [0x831A9268..0x831A9274)
	// 831A9268: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 831A926C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831A9270: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9274(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A9274 size=12
    let mut pc: u32 = 0x831A9274;
    'dispatch: loop {
        match pc {
            0x831A9274 => {
    //   block [0x831A9274..0x831A9280)
	// 831A9274: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831A9278: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831A927C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A9280 size=8
    let mut pc: u32 = 0x831A9280;
    'dispatch: loop {
        match pc {
            0x831A9280 => {
    //   block [0x831A9280..0x831A9288)
	// 831A9280: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831A9284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A9288 size=8
    let mut pc: u32 = 0x831A9288;
    'dispatch: loop {
        match pc {
            0x831A9288 => {
    //   block [0x831A9288..0x831A9290)
	// 831A9288: 38A00A40  li r5, 0xa40
	ctx.r[5].s64 = 2624;
	// 831A928C: 480040C4  b 0x831ad350
	sub_831AD350(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A9290 size=148
    let mut pc: u32 = 0x831A9290;
    'dispatch: loop {
        match pc {
            0x831A9290 => {
    //   block [0x831A9290..0x831A9324)
	// 831A9290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A9294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A9298: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831A929C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A92A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831A92A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831A92A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831A92AC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A92B0: 4800C979  bl 0x831b5c28
	ctx.lr = 0x831A92B4;
	sub_831B5C28(ctx, base);
	// 831A92B4: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831A92B8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831A92BC: 419A003C  beq cr6, 0x831a92f8
	if ctx.cr[6].eq {
	pc = 0x831A92F8; continue 'dispatch;
	}
	// 831A92C0: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831A92C4: 1D6A0014  mulli r11, r10, 0x14
	ctx.r[11].s64 = ctx.r[10].s64 * 20;
	// 831A92C8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 831A92CC: 396BFFF4  addi r11, r11, -0xc
	ctx.r[11].s64 = ctx.r[11].s64 + -12;
	// 831A92D0: 810BFFFC  lwz r8, -4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831A92D4: 392BFFF8  addi r9, r11, -8
	ctx.r[9].s64 = ctx.r[11].s64 + -8;
	// 831A92D8: 7F034000  cmpw cr6, r3, r8
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[8].s32, &mut ctx.xer);
	// 831A92DC: 40990010  ble cr6, 0x831a92ec
	if !ctx.cr[6].gt {
	pc = 0x831A92EC; continue 'dispatch;
	}
	// 831A92E0: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A92E4: 7F034000  cmpw cr6, r3, r8
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[8].s32, &mut ctx.xer);
	// 831A92E8: 40990034  ble cr6, 0x831a931c
	if !ctx.cr[6].gt {
	pc = 0x831A931C; continue 'dispatch;
	}
	// 831A92EC: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831A92F0: 396BFFEC  addi r11, r11, -0x14
	ctx.r[11].s64 = ctx.r[11].s64 + -20;
	// 831A92F4: 4082FFDC  bne 0x831a92d0
	if !ctx.cr[0].eq {
	pc = 0x831A92D0; continue 'dispatch;
	}
	// 831A92F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831A92FC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 831A9300: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 831A9304: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 831A9308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831A930C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A9310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A9314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831A9318: 4E800020  blr
	return;
	// 831A931C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 831A9320: 4BFFFFDC  b 0x831a92fc
	pc = 0x831A92FC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A9328 size=196
    let mut pc: u32 = 0x831A9328;
    'dispatch: loop {
        match pc {
            0x831A9328 => {
    //   block [0x831A9328..0x831A93EC)
	// 831A9328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A932C: 4BFFEE41  bl 0x831a816c
	ctx.lr = 0x831A9330;
	sub_831A8130(ctx, base);
	// 831A9330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A9334: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 831A9338: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831A933C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831A9340: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831A9344: 419A0018  beq cr6, 0x831a935c
	if ctx.cr[6].eq {
	pc = 0x831A935C; continue 'dispatch;
	}
	// 831A9348: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831A934C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831A9350: 419A000C  beq cr6, 0x831a935c
	if ctx.cr[6].eq {
	pc = 0x831A935C; continue 'dispatch;
	}
	// 831A9354: 83AB0004  lwz r29, 4(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831A9358: 48000008  b 0x831a9360
	pc = 0x831A9360; continue 'dispatch;
	// 831A935C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831A9360: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831A9364: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9368: 4800C8C1  bl 0x831b5c28
	ctx.lr = 0x831A936C;
	sub_831B5C28(ctx, base);
	// 831A936C: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831A9370: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831A9374: 419A003C  beq cr6, 0x831a93b0
	if ctx.cr[6].eq {
	pc = 0x831A93B0; continue 'dispatch;
	}
	// 831A9378: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831A937C: 1D6A0014  mulli r11, r10, 0x14
	ctx.r[11].s64 = ctx.r[10].s64 * 20;
	// 831A9380: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 831A9384: 396BFFF4  addi r11, r11, -0xc
	ctx.r[11].s64 = ctx.r[11].s64 + -12;
	// 831A9388: 810BFFFC  lwz r8, -4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831A938C: 392BFFF8  addi r9, r11, -8
	ctx.r[9].s64 = ctx.r[11].s64 + -8;
	// 831A9390: 7F034000  cmpw cr6, r3, r8
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[8].s32, &mut ctx.xer);
	// 831A9394: 40990010  ble cr6, 0x831a93a4
	if !ctx.cr[6].gt {
	pc = 0x831A93A4; continue 'dispatch;
	}
	// 831A9398: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A939C: 7F034000  cmpw cr6, r3, r8
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[8].s32, &mut ctx.xer);
	// 831A93A0: 40990024  ble cr6, 0x831a93c4
	if !ctx.cr[6].gt {
	pc = 0x831A93C4; continue 'dispatch;
	}
	// 831A93A4: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831A93A8: 396BFFEC  addi r11, r11, -0x14
	ctx.r[11].s64 = ctx.r[11].s64 + -20;
	// 831A93AC: 4082FFDC  bne 0x831a9388
	if !ctx.cr[0].eq {
	pc = 0x831A9388; continue 'dispatch;
	}
	// 831A93B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831A93B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831A93B8: 409A0014  bne cr6, 0x831a93cc
	if !ctx.cr[6].eq {
	pc = 0x831A93CC; continue 'dispatch;
	}
	// 831A93BC: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 831A93C0: 48000014  b 0x831a93d4
	pc = 0x831A93D4; continue 'dispatch;
	// 831A93C4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 831A93C8: 4BFFFFEC  b 0x831a93b4
	pc = 0x831A93B4; continue 'dispatch;
	// 831A93CC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831A93D0: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 831A93D4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831A93D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831A93DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831A93E0: 4800CA99  bl 0x831b5e78
	ctx.lr = 0x831A93E4;
	sub_831B5E78(ctx, base);
	// 831A93E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831A93E8: 4BFFEDD4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A93F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A93F0 size=56
    let mut pc: u32 = 0x831A93F0;
    'dispatch: loop {
        match pc {
            0x831A93F0 => {
    //   block [0x831A93F0..0x831A9428)
	// 831A93F0: 81660004  lwz r11, 4(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 831A93F4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A93F8: 80EBFFFC  lwz r7, -4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831A93FC: 8167000C  lwz r11, 0xc(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 831A9400: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831A9404: 419A0014  beq cr6, 0x831a9418
	if ctx.cr[6].eq {
	pc = 0x831A9418; continue 'dispatch;
	}
	// 831A9408: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 831A940C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831A9410: 419A0008  beq cr6, 0x831a9418
	if ctx.cr[6].eq {
	pc = 0x831A9418; continue 'dispatch;
	}
	// 831A9414: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831A9418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831A941C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831A9420: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 831A9424: 4800D734  b 0x831b6b58
	sub_831B6B58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A9428 size=116
    let mut pc: u32 = 0x831A9428;
    'dispatch: loop {
        match pc {
            0x831A9428 => {
    //   block [0x831A9428..0x831A949C)
	// 831A9428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A942C: 4BFFED31  bl 0x831a815c
	ctx.lr = 0x831A9430;
	sub_831A8130(ctx, base);
	// 831A9430: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A9434: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831A9438: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831A943C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831A9440: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 831A9444: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 831A9448: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 831A944C: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 831A9450: 48003C49  bl 0x831ad098
	ctx.lr = 0x831A9454;
	sub_831AD098(ctx, base);
	// 831A9454: 93C300B0  stw r30, 0xb0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), ctx.r[30].u32 ) };
	// 831A9458: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 831A945C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831A9460: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 831A9468: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 831A946C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 831A9470: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 831A9474: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831A9478: 4800D6E1  bl 0x831b6b58
	ctx.lr = 0x831A947C;
	sub_831B6B58(ctx, base);
	// 831A947C: 48003C1D  bl 0x831ad098
	ctx.lr = 0x831A9480;
	sub_831AD098(ctx, base);
	// 831A9480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831A9484: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 831A9488: 916300B0  stw r11, 0xb0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 831A948C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831A9490: 91590000  stw r10, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831A9494: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831A9498: 4BFFED14  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A94A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A94A0 size=8
    let mut pc: u32 = 0x831A94A0;
    'dispatch: loop {
        match pc {
            0x831A94A0 => {
    //   block [0x831A94A0..0x831A94A8)
	// 831A94A0: 8324280C  lwz r25, 0x280c(r4)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(10252 as u32) ) } as u64;
	// 831A94A4: 8224BF28  lwz r17, -0x40d8(r4)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-16600 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A94A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A94A8 size=120
    let mut pc: u32 = 0x831A94A8;
    'dispatch: loop {
        match pc {
            0x831A94A8 => {
    //   block [0x831A94A8..0x831A9520)
	// 831A94A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A94AC: 4BFFECC1  bl 0x831a816c
	ctx.lr = 0x831A94B0;
	sub_831A8130(ctx, base);
	// 831A94B0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 831A94B4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A94B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831A94BC: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 831A94C0: 90BF00A4  stw r5, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 831A94C4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 831A94C8: 909F009C  stw r4, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[4].u32 ) };
	// 831A94CC: 90DF00AC  stw r6, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[6].u32 ) };
	// 831A94D0: 90FF00B4  stw r7, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[7].u32 ) };
	// 831A94D4: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 831A94D8: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 831A94DC: 90BF005C  stw r5, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 831A94E0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831A94E4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831A94E8: 48003BB1  bl 0x831ad098
	ctx.lr = 0x831A94EC;
	sub_831AD098(ctx, base);
	// 831A94EC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831A94F0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A94F4: 389F0058  addi r4, r31, 0x58
	ctx.r[4].s64 = ctx.r[31].s64 + 88;
	// 831A94F8: 816B0074  lwz r11, 0x74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 831A94FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831A9500: 4E800421  bctrl
	ctx.lr = 0x831A9504;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831A9504: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 831A9508: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831A950C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831A9510: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831A9514: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 831A9518: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 831A951C: 4BFFECA0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A9520 size=72
    let mut pc: u32 = 0x831A9520;
    'dispatch: loop {
        match pc {
            0x831A9520 => {
    //   block [0x831A9520..0x831A9568)
	// 831A9520: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 831A9524: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 831A9528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A952C: 9181FFF0  stw r12, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[12].u32 ) };
	// 831A9530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A9534: 393F0050  addi r9, r31, 0x50
	ctx.r[9].s64 = ctx.r[31].s64 + 80;
	// 831A9538: 811F00B4  lwz r8, 0xb4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 831A953C: 80FF00AC  lwz r7, 0xac(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 831A9540: 80DF00A4  lwz r6, 0xa4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 831A9544: 80BF009C  lwz r5, 0x9c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 831A9548: 809F0094  lwz r4, 0x94(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 831A954C: 4BFFFEDD  bl 0x831a9428
	ctx.lr = 0x831A9550;
	sub_831A9428(ctx, base);
	// 831A9550: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831A9554: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9558: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 831A955C: 8181FFF0  lwz r12, -0x10(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 831A9560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A9564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A9568 size=344
    let mut pc: u32 = 0x831A9568;
    'dispatch: loop {
        match pc {
            0x831A9568 => {
    //   block [0x831A9568..0x831A96C0)
	// 831A9568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A956C: 4BFFEBED  bl 0x831a8158
	ctx.lr = 0x831A9570;
	sub_831A8130(ctx, base);
	// 831A9570: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A9574: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831A9578: 80880000  lwz r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A957C: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 831A9580: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831A9584: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 831A9588: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 831A958C: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831A9590: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 831A9594: 7F1CC378  mr r28, r24
	ctx.r[28].u64 = ctx.r[24].u64;
	// 831A9598: 4800C691  bl 0x831b5c28
	ctx.lr = 0x831A959C;
	sub_831B5C28(ctx, base);
	// 831A959C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831A95A0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 831A95A4: 409A0008  bne cr6, 0x831a95ac
	if !ctx.cr[6].eq {
	pc = 0x831A95AC; continue 'dispatch;
	}
	// 831A95A8: 4800D831  bl 0x831b6dd8
	ctx.lr = 0x831A95AC;
	sub_831B6DD8(ctx, base);
	// 831A95AC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831A95B0: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 831A95B4: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831A95B8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 831A95BC: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831A95C0: 419A0038  beq cr6, 0x831a95f8
	if ctx.cr[6].eq {
	pc = 0x831A95F8; continue 'dispatch;
	}
	// 831A95C4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831A95C8: 1D7E0014  mulli r11, r30, 0x14
	ctx.r[11].s64 = ctx.r[30].s64 * 20;
	// 831A95CC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831A95D0: 396BFFF4  addi r11, r11, -0xc
	ctx.r[11].s64 = ctx.r[11].s64 + -12;
	// 831A95D4: 814BFFFC  lwz r10, -4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831A95D8: 7F1D5000  cmpw cr6, r29, r10
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[10].s32, &mut ctx.xer);
	// 831A95DC: 40990010  ble cr6, 0x831a95ec
	if !ctx.cr[6].gt {
	pc = 0x831A95EC; continue 'dispatch;
	}
	// 831A95E0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A95E4: 7F1D5000  cmpw cr6, r29, r10
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[10].s32, &mut ctx.xer);
	// 831A95E8: 40990010  ble cr6, 0x831a95f8
	if !ctx.cr[6].gt {
	pc = 0x831A95F8; continue 'dispatch;
	}
	// 831A95EC: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831A95F0: 396BFFEC  addi r11, r11, -0x14
	ctx.r[11].s64 = ctx.r[11].s64 + -20;
	// 831A95F4: 4082FFE0  bne 0x831a95d4
	if !ctx.cr[0].eq {
	pc = 0x831A95D4; continue 'dispatch;
	}
	// 831A95F8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 831A95FC: 419A0014  beq cr6, 0x831a9610
	if ctx.cr[6].eq {
	pc = 0x831A9610; continue 'dispatch;
	}
	// 831A9600: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831A9604: 1D690014  mulli r11, r9, 0x14
	ctx.r[11].s64 = ctx.r[9].s64 * 20;
	// 831A9608: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831A960C: 3B8BFFEC  addi r28, r11, -0x14
	ctx.r[28].s64 = ctx.r[11].s64 + -20;
	// 831A9610: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 831A9614: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 831A9618: 419A0084  beq cr6, 0x831a969c
	if ctx.cr[6].eq {
	pc = 0x831A969C; continue 'dispatch;
	}
	// 831A961C: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 831A9620: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831A9624: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 831A9628: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 831A962C: 419A0024  beq cr6, 0x831a9650
	if ctx.cr[6].eq {
	pc = 0x831A9650; continue 'dispatch;
	}
	// 831A9630: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9634: 80FC0004  lwz r7, 4(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 831A9638: 7F083800  cmpw cr6, r8, r7
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[7].s32, &mut ctx.xer);
	// 831A963C: 40990044  ble cr6, 0x831a9680
	if !ctx.cr[6].gt {
	pc = 0x831A9680; continue 'dispatch;
	}
	// 831A9640: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831A9644: 80FC0008  lwz r7, 8(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 831A9648: 7F083800  cmpw cr6, r8, r7
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[7].s32, &mut ctx.xer);
	// 831A964C: 41990034  bgt cr6, 0x831a9680
	if ctx.cr[6].gt {
	pc = 0x831A9680; continue 'dispatch;
	}
	// 831A9650: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9654: 7F194000  cmpw cr6, r25, r8
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[8].s32, &mut ctx.xer);
	// 831A9658: 41980028  blt cr6, 0x831a9680
	if ctx.cr[6].lt {
	pc = 0x831A9680; continue 'dispatch;
	}
	// 831A965C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831A9660: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831A9664: 4199001C  bgt cr6, 0x831a9680
	if ctx.cr[6].gt {
	pc = 0x831A9680; continue 'dispatch;
	}
	// 831A9668: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A966C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 831A9670: 409A0008  bne cr6, 0x831a9678
	if !ctx.cr[6].eq {
	pc = 0x831A9678; continue 'dispatch;
	}
	// 831A9674: 915B0000  stw r10, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831A9678: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 831A967C: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831A9680: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831A9684: 39290014  addi r9, r9, 0x14
	ctx.r[9].s64 = ctx.r[9].s64 + 20;
	// 831A9688: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 831A968C: 4198FF94  blt cr6, 0x831a9620
	if ctx.cr[6].lt {
	pc = 0x831A9620; continue 'dispatch;
	}
	// 831A9690: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9694: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 831A9698: 409A0014  bne cr6, 0x831a96ac
	if !ctx.cr[6].eq {
	pc = 0x831A96AC; continue 'dispatch;
	}
	// 831A969C: 931B0000  stw r24, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 831A96A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A96A4: 931A0000  stw r24, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 831A96A8: 48000010  b 0x831a96b8
	pc = 0x831A96B8; continue 'dispatch;
	// 831A96AC: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831A96B0: 1D6B0014  mulli r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 * 20;
	// 831A96B4: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831A96B8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831A96BC: 4BFFEAEC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A96C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A96C0 size=180
    let mut pc: u32 = 0x831A96C0;
    'dispatch: loop {
        match pc {
            0x831A96C0 => {
    //   block [0x831A96C0..0x831A9774)
	// 831A96C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A96C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A96C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831A96CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A96D0: 3D60E06D  lis r11, -0x1f93
	ctx.r[11].s64 = -529727488;
	// 831A96D4: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A96D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831A96DC: 616B7363  ori r11, r11, 0x7363
	ctx.r[11].u64 = ctx.r[11].u64 | 29539;
	// 831A96E0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831A96E4: 409A0058  bne cr6, 0x831a973c
	if !ctx.cr[6].eq {
	pc = 0x831A973C; continue 'dispatch;
	}
	// 831A96E8: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 831A96EC: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 831A96F0: 409A004C  bne cr6, 0x831a973c
	if !ctx.cr[6].eq {
	pc = 0x831A973C; continue 'dispatch;
	}
	// 831A96F4: 3D401993  lis r10, 0x1993
	ctx.r[10].s64 = 429064192;
	// 831A96F8: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 831A96FC: 614A0520  ori r10, r10, 0x520
	ctx.r[10].u64 = ctx.r[10].u64 | 1312;
	// 831A9700: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831A9704: 419A0024  beq cr6, 0x831a9728
	if ctx.cr[6].eq {
	pc = 0x831A9728; continue 'dispatch;
	}
	// 831A9708: 3D401993  lis r10, 0x1993
	ctx.r[10].s64 = 429064192;
	// 831A970C: 614A0521  ori r10, r10, 0x521
	ctx.r[10].u64 = ctx.r[10].u64 | 1313;
	// 831A9710: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831A9714: 419A0014  beq cr6, 0x831a9728
	if ctx.cr[6].eq {
	pc = 0x831A9728; continue 'dispatch;
	}
	// 831A9718: 3D401993  lis r10, 0x1993
	ctx.r[10].s64 = 429064192;
	// 831A971C: 614A0522  ori r10, r10, 0x522
	ctx.r[10].u64 = ctx.r[10].u64 | 1314;
	// 831A9720: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831A9724: 409A0018  bne cr6, 0x831a973c
	if !ctx.cr[6].eq {
	pc = 0x831A973C; continue 'dispatch;
	}
	// 831A9728: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 831A972C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831A9730: 8164001C  lwz r11, 0x1c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 831A9734: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831A9738: 4800000C  b 0x831a9744
	pc = 0x831A9744; continue 'dispatch;
	// 831A973C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831A9740: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831A9744: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831A9748: 48003951  bl 0x831ad098
	ctx.lr = 0x831A974C;
	sub_831AD098(ctx, base);
	// 831A974C: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 831A9750: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 831A9754: 48003945  bl 0x831ad098
	ctx.lr = 0x831A9758;
	sub_831AD098(ctx, base);
	// 831A9758: 93E30094  stw r31, 0x94(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), ctx.r[31].u32 ) };
	// 831A975C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831A9760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831A9764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A9768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A976C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831A9770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A9778 size=88
    let mut pc: u32 = 0x831A9778;
    'dispatch: loop {
        match pc {
            0x831A9778 => {
    //   block [0x831A9778..0x831A97D0)
	// 831A9778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A977C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A9780: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831A9784: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A9788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831A978C: 4800390D  bl 0x831ad098
	ctx.lr = 0x831A9790;
	sub_831AD098(ctx, base);
	// 831A9790: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 831A9794: 48000014  b 0x831a97a8
	pc = 0x831A97A8; continue 'dispatch;
	// 831A9798: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A979C: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 831A97A0: 419A0028  beq cr6, 0x831a97c8
	if ctx.cr[6].eq {
	pc = 0x831A97C8; continue 'dispatch;
	}
	// 831A97A4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831A97A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831A97AC: 409AFFEC  bne cr6, 0x831a9798
	if !ctx.cr[6].eq {
	pc = 0x831A9798; continue 'dispatch;
	}
	// 831A97B0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831A97B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831A97B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A97BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A97C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831A97C4: 4E800020  blr
	return;
	// 831A97C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A97CC: 4BFFFFE8  b 0x831a97b4
	pc = 0x831A97B4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A97D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A97D0 size=172
    let mut pc: u32 = 0x831A97D0;
    'dispatch: loop {
        match pc {
            0x831A97D0 => {
    //   block [0x831A97D0..0x831A987C)
	// 831A97D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A97D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A97D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831A97DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A97E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831A97E4: 480038B5  bl 0x831ad098
	ctx.lr = 0x831A97E8;
	sub_831AD098(ctx, base);
	// 831A97E8: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 831A97EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831A97F0: 419A000C  beq cr6, 0x831a97fc
	if ctx.cr[6].eq {
	pc = 0x831A97FC; continue 'dispatch;
	}
	// 831A97F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831A97F8: 409A0008  bne cr6, 0x831a9800
	if !ctx.cr[6].eq {
	pc = 0x831A9800; continue 'dispatch;
	}
	// 831A97FC: 4800D5DD  bl 0x831b6dd8
	ctx.lr = 0x831A9800;
	sub_831B6DD8(ctx, base);
	// 831A9800: 48003899  bl 0x831ad098
	ctx.lr = 0x831A9804;
	sub_831AD098(ctx, base);
	// 831A9804: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 831A9808: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831A980C: 409A0014  bne cr6, 0x831a9820
	if !ctx.cr[6].eq {
	pc = 0x831A9820; continue 'dispatch;
	}
	// 831A9810: 48003889  bl 0x831ad098
	ctx.lr = 0x831A9814;
	sub_831AD098(ctx, base);
	// 831A9814: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831A9818: 91630094  stw r11, 0x94(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 831A981C: 48000040  b 0x831a985c
	pc = 0x831A985C; continue 'dispatch;
	// 831A9820: 48003879  bl 0x831ad098
	ctx.lr = 0x831A9824;
	sub_831AD098(ctx, base);
	// 831A9824: 81230094  lwz r9, 0x94(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 831A9828: 39690008  addi r11, r9, 8
	ctx.r[11].s64 = ctx.r[9].s64 + 8;
	// 831A982C: 81490008  lwz r10, 8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 831A9830: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831A9834: 419A0024  beq cr6, 0x831a9858
	if ctx.cr[6].eq {
	pc = 0x831A9858; continue 'dispatch;
	}
	// 831A9838: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A983C: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831A9840: 419A0030  beq cr6, 0x831a9870
	if ctx.cr[6].eq {
	pc = 0x831A9870; continue 'dispatch;
	}
	// 831A9844: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 831A9848: 396A0008  addi r11, r10, 8
	ctx.r[11].s64 = ctx.r[10].s64 + 8;
	// 831A984C: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 831A9850: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 831A9854: 409AFFE4  bne cr6, 0x831a9838
	if !ctx.cr[6].eq {
	pc = 0x831A9838; continue 'dispatch;
	}
	// 831A9858: 4800D581  bl 0x831b6dd8
	ctx.lr = 0x831A985C;
	sub_831B6DD8(ctx, base);
	// 831A985C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831A9860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A9864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A9868: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831A986C: 4E800020  blr
	return;
	// 831A9870: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831A9874: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 831A9878: 4BFFFFE4  b 0x831a985c
	pc = 0x831A985C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A9880 size=72
    let mut pc: u32 = 0x831A9880;
    'dispatch: loop {
        match pc {
            0x831A9880 => {
    //   block [0x831A9880..0x831A98C8)
	// 831A9880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A9884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A9888: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 831A988C: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 831A9890: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 831A9894: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 831A9898: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 831A989C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A98A0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 831A98A4: 39410088  addi r10, r1, 0x88
	ctx.r[10].s64 = ctx.r[1].s64 + 136;
	// 831A98A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831A98AC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831A98B0: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831A98B4: 48002F6D  bl 0x831ac820
	ctx.lr = 0x831A98B8;
	sub_831AC820(ctx, base);
	// 831A98B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831A98BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A98C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A98C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A98C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A98C8 size=68
    let mut pc: u32 = 0x831A98C8;
    'dispatch: loop {
        match pc {
            0x831A98C8 => {
    //   block [0x831A98C8..0x831A990C)
	// 831A98C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A98CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A98D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831A98D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A98D8: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 831A98DC: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 831A98E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831A98E4: 4BC46CA5  bl 0x82df0588
	ctx.lr = 0x831A98E8;
	sub_82DF0588(ctx, base);
	// 831A98E8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831A98EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831A98F0: 396BC37C  addi r11, r11, -0x3c84
	ctx.r[11].s64 = ctx.r[11].s64 + -15492;
	// 831A98F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831A98F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831A98FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A9900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A9904: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831A9908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A9910 size=24
    let mut pc: u32 = 0x831A9910;
    'dispatch: loop {
        match pc {
            0x831A9910 => {
    //   block [0x831A9910..0x831A9928)
	// 831A9910: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831A9914: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831A9918: 396BC37C  addi r11, r11, -0x3c84
	ctx.r[11].s64 = ctx.r[11].s64 + -15492;
	// 831A991C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831A9920: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831A9924: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A9928 size=8
    let mut pc: u32 = 0x831A9928;
    'dispatch: loop {
        match pc {
            0x831A9928 => {
    //   block [0x831A9928..0x831A9930)
	// 831A9928: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831A992C: 480021AC  b 0x831abad8
	sub_831ABAD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A9930 size=4
    let mut pc: u32 = 0x831A9930;
    'dispatch: loop {
        match pc {
            0x831A9930 => {
    //   block [0x831A9930..0x831A9934)
	// 831A9930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A9938 size=68
    let mut pc: u32 = 0x831A9938;
    'dispatch: loop {
        match pc {
            0x831A9938 => {
    //   block [0x831A9938..0x831A997C)
	// 831A9938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A993C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A9940: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831A9944: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A9948: 90810050  stw r4, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 831A994C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831A9950: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831A9954: 4BC46C35  bl 0x82df0588
	ctx.lr = 0x831A9958;
	sub_82DF0588(ctx, base);
	// 831A9958: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831A995C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831A9960: 396BC37C  addi r11, r11, -0x3c84
	ctx.r[11].s64 = ctx.r[11].s64 + -15492;
	// 831A9964: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831A9968: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831A996C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A9970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A9974: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831A9978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A9980 size=8
    let mut pc: u32 = 0x831A9980;
    'dispatch: loop {
        match pc {
            0x831A9980 => {
    //   block [0x831A9980..0x831A9988)
	// 831A9980: 8324280C  lwz r25, 0x280c(r4)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(10252 as u32) ) } as u64;
	// 831A9984: 8224BF40  lwz r17, -0x40c0(r4)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-16576 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A9988 size=188
    let mut pc: u32 = 0x831A9988;
    'dispatch: loop {
        match pc {
            0x831A9988 => {
    //   block [0x831A9988..0x831A9A44)
	// 831A9988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A998C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831A9990: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831A9994: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 831A9998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A999C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831A99A0: 419A0070  beq cr6, 0x831a9a10
	if ctx.cr[6].eq {
	pc = 0x831A9A10; continue 'dispatch;
	}
	// 831A99A4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831A99A8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831A99AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A99B0: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831A99B4: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 831A99B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831A99BC: 419A0010  beq cr6, 0x831a99cc
	if ctx.cr[6].eq {
	pc = 0x831A99CC; continue 'dispatch;
	}
	// 831A99C0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 831A99C4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831A99C8: 48000068  b 0x831a9a30
	pc = 0x831A9A30; continue 'dispatch;
	// 831A99CC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831A99D0: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 831A99D4: 388BC3CC  addi r4, r11, -0x3c34
	ctx.r[4].s64 = ctx.r[11].s64 + -15412;
	// 831A99D8: 4BFFFF61  bl 0x831a9938
	ctx.lr = 0x831A99DC;
	sub_831A9938(ctx, base);
	// 831A99DC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 831A99E0: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 831A99E4: 388BCAF8  addi r4, r11, -0x3508
	ctx.r[4].s64 = ctx.r[11].s64 + -13576;
	// 831A99E8: 48007241  bl 0x831b0c28
	ctx.lr = 0x831A99EC;
	sub_831B0C28(ctx, base);
	// 831A99EC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831A99F0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831A99F4: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 831A99F8: 388BC3A8  addi r4, r11, -0x3c58
	ctx.r[4].s64 = ctx.r[11].s64 + -15448;
	// 831A99FC: 4BFFFF3D  bl 0x831a9938
	ctx.lr = 0x831A9A00;
	sub_831A9938(ctx, base);
	// 831A9A00: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 831A9A04: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 831A9A08: 388BCAF8  addi r4, r11, -0x3508
	ctx.r[4].s64 = ctx.r[11].s64 + -13576;
	// 831A9A0C: 4800721D  bl 0x831b0c28
	ctx.lr = 0x831A9A10;
	sub_831B0C28(ctx, base);
	// 831A9A10: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831A9A14: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 831A9A18: 388BC384  addi r4, r11, -0x3c7c
	ctx.r[4].s64 = ctx.r[11].s64 + -15484;
	// 831A9A1C: 4BFFFEAD  bl 0x831a98c8
	ctx.lr = 0x831A9A20;
	sub_831A98C8(ctx, base);
	// 831A9A20: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 831A9A24: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 831A9A28: 388BCAE8  addi r4, r11, -0x3518
	ctx.r[4].s64 = ctx.r[11].s64 + -13592;
	// 831A9A2C: 480071FD  bl 0x831b0c28
	ctx.lr = 0x831A9A30;
	sub_831B0C28(ctx, base);
	// 831A9A30: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 831A9A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831A9A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831A9A3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831A9A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9A44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A9A44 size=36
    let mut pc: u32 = 0x831A9A44;
    'dispatch: loop {
        match pc {
            0x831A9A44 => {
    //   block [0x831A9A44..0x831A9A68)
	// 831A9A44: 3D60C000  lis r11, -0x4000
	ctx.r[11].s64 = -1073741824;
	// 831A9A48: 616B0005  ori r11, r11, 5
	ctx.r[11].u64 = ctx.r[11].u64 | 5;
	// 831A9A4C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9A50: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9A54: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831A9A58: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 831A9A5C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 831A9A60: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831A9A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A9A68 size=256
    let mut pc: u32 = 0x831A9A68;
    'dispatch: loop {
        match pc {
            0x831A9A68 => {
    //   block [0x831A9A68..0x831A9B68)
	// 831A9A68: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 831A9A6C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 831A9A70: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 831A9A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831A9A78: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831A9A7C: 80CB000C  lwz r6, 0xc(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 831A9A80: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831A9A84: 419A00D4  beq cr6, 0x831a9b58
	if ctx.cr[6].eq {
	pc = 0x831A9B58; continue 'dispatch;
	}
	// 831A9A88: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 831A9A8C: 80680000  lwz r3, 0(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9A90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9A94: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 831A9A98: 419A004C  beq cr6, 0x831a9ae4
	if ctx.cr[6].eq {
	pc = 0x831A9AE4; continue 'dispatch;
	}
	// 831A9A9C: 39450008  addi r10, r5, 8
	ctx.r[10].s64 = ctx.r[5].s64 + 8;
	// 831A9AA0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831A9AA4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9AA8: 8BCA0000  lbz r30, 0(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9AAC: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A9AB0: 7D3E4850  subf r9, r30, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[30].s64;
	// 831A9AB4: 41820014  beq 0x831a9ac8
	if ctx.cr[0].eq {
	pc = 0x831A9AC8; continue 'dispatch;
	}
	// 831A9AB8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831A9ABC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831A9AC0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831A9AC4: 419AFFE0  beq cr6, 0x831a9aa4
	if ctx.cr[6].eq {
	pc = 0x831A9AA4; continue 'dispatch;
	}
	// 831A9AC8: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A9ACC: 41820018  beq 0x831a9ae4
	if ctx.cr[0].eq {
	pc = 0x831A9AE4; continue 'dispatch;
	}
	// 831A9AD0: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 831A9AD4: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 831A9AD8: 7F07F840  cmplw cr6, r7, r31
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[31].u32, &mut ctx.xer);
	// 831A9ADC: 4198FFB0  blt cr6, 0x831a9a8c
	if ctx.cr[6].lt {
	pc = 0x831A9A8C; continue 'dispatch;
	}
	// 831A9AE0: 48000078  b 0x831a9b58
	pc = 0x831A9B58; continue 'dispatch;
	// 831A9AE4: 39070001  addi r8, r7, 1
	ctx.r[8].s64 = ctx.r[7].s64 + 1;
	// 831A9AE8: 7F08F840  cmplw cr6, r8, r31
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[31].u32, &mut ctx.xer);
	// 831A9AEC: 4098006C  bge cr6, 0x831a9b58
	if !ctx.cr[6].lt {
	pc = 0x831A9B58; continue 'dispatch;
	}
	// 831A9AF0: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831A9AF4: 7CEB3214  add r7, r11, r6
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 831A9AF8: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9AFC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 831A9B00: 554A077B  rlwinm. r10, r10, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831A9B04: 40820054  bne 0x831a9b58
	if !ctx.cr[0].eq {
	pc = 0x831A9B58; continue 'dispatch;
	}
	// 831A9B08: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9B0C: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 831A9B10: 419A004C  beq cr6, 0x831a9b5c
	if ctx.cr[6].eq {
	pc = 0x831A9B5C; continue 'dispatch;
	}
	// 831A9B14: 39440008  addi r10, r4, 8
	ctx.r[10].s64 = ctx.r[4].s64 + 8;
	// 831A9B18: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831A9B1C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9B20: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9B24: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A9B28: 7D264850  subf r9, r6, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[6].s64;
	// 831A9B2C: 41820014  beq 0x831a9b40
	if ctx.cr[0].eq {
	pc = 0x831A9B40; continue 'dispatch;
	}
	// 831A9B30: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831A9B34: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831A9B38: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831A9B3C: 419AFFE0  beq cr6, 0x831a9b1c
	if ctx.cr[6].eq {
	pc = 0x831A9B1C; continue 'dispatch;
	}
	// 831A9B40: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A9B44: 41820018  beq 0x831a9b5c
	if ctx.cr[0].eq {
	pc = 0x831A9B5C; continue 'dispatch;
	}
	// 831A9B48: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 831A9B4C: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 831A9B50: 7F08F840  cmplw cr6, r8, r31
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[31].u32, &mut ctx.xer);
	// 831A9B54: 4198FFA4  blt cr6, 0x831a9af8
	if ctx.cr[6].lt {
	pc = 0x831A9AF8; continue 'dispatch;
	}
	// 831A9B58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A9B5C: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831A9B60: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 831A9B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A9B68 size=424
    let mut pc: u32 = 0x831A9B68;
    'dispatch: loop {
        match pc {
            0x831A9B68 => {
    //   block [0x831A9B68..0x831A9D10)
	// 831A9B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A9B6C: 4BFFE5F1  bl 0x831a815c
	ctx.lr = 0x831A9B70;
	sub_831A8130(ctx, base);
	// 831A9B70: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 831A9B74: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 831A9B78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A9B7C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 831A9B80: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831A9B84: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 831A9B88: 83AB0008  lwz r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831A9B8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831A9B90: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 831A9B94: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 831A9B98: 419A0170  beq cr6, 0x831a9d08
	if ctx.cr[6].eq {
	pc = 0x831A9D08; continue 'dispatch;
	}
	// 831A9B9C: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 831A9BA0: 7D7E2050  subf r11, r30, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[30].s64;
	// 831A9BA4: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9BA8: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 831A9BAC: 40990058  ble cr6, 0x831a9c04
	if !ctx.cr[6].gt {
	pc = 0x831A9C04; continue 'dispatch;
	}
	// 831A9BB0: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9BB4: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 831A9BB8: 419A0038  beq cr6, 0x831a9bf0
	if ctx.cr[6].eq {
	pc = 0x831A9BF0; continue 'dispatch;
	}
	// 831A9BBC: 39470008  addi r10, r7, 8
	ctx.r[10].s64 = ctx.r[7].s64 + 8;
	// 831A9BC0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831A9BC4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9BC8: 8B2A0000  lbz r25, 0(r10)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9BCC: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A9BD0: 7D394850  subf r9, r25, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[25].s64;
	// 831A9BD4: 41820014  beq 0x831a9be8
	if ctx.cr[0].eq {
	pc = 0x831A9BE8; continue 'dispatch;
	}
	// 831A9BD8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831A9BDC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831A9BE0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831A9BE4: 419AFFE0  beq cr6, 0x831a9bc4
	if ctx.cr[6].eq {
	pc = 0x831A9BC4; continue 'dispatch;
	}
	// 831A9BE8: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A9BEC: 40820018  bne 0x831a9c04
	if !ctx.cr[0].eq {
	pc = 0x831A9C04; continue 'dispatch;
	}
	// 831A9BF0: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 831A9BF4: 409A00A0  bne cr6, 0x831a9c94
	if !ctx.cr[6].eq {
	pc = 0x831A9C94; continue 'dispatch;
	}
	// 831A9BF8: 83E80004  lwz r31, 4(r8)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 831A9BFC: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 831A9C00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831A9C04: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9C08: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 831A9C0C: 419A0038  beq cr6, 0x831a9c44
	if ctx.cr[6].eq {
	pc = 0x831A9C44; continue 'dispatch;
	}
	// 831A9C10: 39450008  addi r10, r5, 8
	ctx.r[10].s64 = ctx.r[5].s64 + 8;
	// 831A9C14: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831A9C18: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9C1C: 8B2A0000  lbz r25, 0(r10)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9C20: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A9C24: 7D394850  subf r9, r25, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[25].s64;
	// 831A9C28: 41820014  beq 0x831a9c3c
	if ctx.cr[0].eq {
	pc = 0x831A9C3C; continue 'dispatch;
	}
	// 831A9C2C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831A9C30: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831A9C34: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831A9C38: 419AFFE0  beq cr6, 0x831a9c18
	if ctx.cr[6].eq {
	pc = 0x831A9C18; continue 'dispatch;
	}
	// 831A9C3C: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A9C40: 40820040  bne 0x831a9c80
	if !ctx.cr[0].eq {
	pc = 0x831A9C80; continue 'dispatch;
	}
	// 831A9C44: 8168000C  lwz r11, 0xc(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 831A9C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831A9C4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831A9C50: 41980014  blt cr6, 0x831a9c64
	if ctx.cr[6].lt {
	pc = 0x831A9C64; continue 'dispatch;
	}
	// 831A9C54: 7D4BD82E  lwzx r10, r11, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 831A9C58: 81280010  lwz r9, 0x10(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 831A9C5C: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 831A9C60: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831A9C64: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 831A9C68: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831A9C6C: 7F0B3000  cmpw cr6, r11, r6
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[6].s32, &mut ctx.xer);
	// 831A9C70: 409A0010  bne cr6, 0x831a9c80
	if !ctx.cr[6].eq {
	pc = 0x831A9C80; continue 'dispatch;
	}
	// 831A9C74: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831A9C78: 409A003C  bne cr6, 0x831a9cb4
	if !ctx.cr[6].eq {
	pc = 0x831A9CB4; continue 'dispatch;
	}
	// 831A9C7C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 831A9C80: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 831A9C84: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 831A9C88: 7F04E840  cmplw cr6, r4, r29
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[29].u32, &mut ctx.xer);
	// 831A9C8C: 4198FF14  blt cr6, 0x831a9ba0
	if ctx.cr[6].lt {
	pc = 0x831A9BA0; continue 'dispatch;
	}
	// 831A9C90: 48000078  b 0x831a9d08
	pc = 0x831A9D08; continue 'dispatch;
	// 831A9C94: 81680014  lwz r11, 0x14(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 831A9C98: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831A9C9C: 4082006C  bne 0x831a9d08
	if !ctx.cr[0].eq {
	pc = 0x831A9D08; continue 'dispatch;
	}
	// 831A9CA0: 817A0014  lwz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 831A9CA4: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831A9CA8: 40820060  bne 0x831a9d08
	if !ctx.cr[0].eq {
	pc = 0x831A9D08; continue 'dispatch;
	}
	// 831A9CAC: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 831A9CB0: 4800005C  b 0x831a9d0c
	pc = 0x831A9D0C; continue 'dispatch;
	// 831A9CB4: 7D7E2050  subf r11, r30, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[30].s64;
	// 831A9CB8: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 831A9CBC: 41990040  bgt cr6, 0x831a9cfc
	if ctx.cr[6].gt {
	pc = 0x831A9CFC; continue 'dispatch;
	}
	// 831A9CC0: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831A9CC4: 554A0673  rlwinm. r10, r10, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831A9CC8: 4082001C  bne 0x831a9ce4
	if !ctx.cr[0].eq {
	pc = 0x831A9CE4; continue 'dispatch;
	}
	// 831A9CCC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 831A9CD0: 409A003C  bne cr6, 0x831a9d0c
	if !ctx.cr[6].eq {
	pc = 0x831A9D0C; continue 'dispatch;
	}
	// 831A9CD4: 81680014  lwz r11, 0x14(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 831A9CD8: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831A9CDC: 4082002C  bne 0x831a9d08
	if !ctx.cr[0].eq {
	pc = 0x831A9D08; continue 'dispatch;
	}
	// 831A9CE0: 4800002C  b 0x831a9d0c
	pc = 0x831A9D0C; continue 'dispatch;
	// 831A9CE4: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831A9CE8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831A9CEC: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 831A9CF0: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831A9CF4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 831A9CF8: 4BFFFFE0  b 0x831a9cd8
	pc = 0x831A9CD8; continue 'dispatch;
	// 831A9CFC: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831A9D00: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831A9D04: 4182FFD0  beq 0x831a9cd4
	if ctx.cr[0].eq {
	pc = 0x831A9CD4; continue 'dispatch;
	}
	// 831A9D08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A9D0C: 4BFFE4A0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A9D10 size=560
    let mut pc: u32 = 0x831A9D10;
    'dispatch: loop {
        match pc {
            0x831A9D10 => {
    //   block [0x831A9D10..0x831A9F40)
	// 831A9D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A9D14: 4BFFE439  bl 0x831a814c
	ctx.lr = 0x831A9D18;
	sub_831A8130(ctx, base);
	// 831A9D18: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 831A9D1C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 831A9D20: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 831A9D24: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 831A9D28: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831A9D2C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 831A9D30: 836B0008  lwz r27, 8(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831A9D34: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 831A9D38: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 831A9D3C: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 831A9D40: 3B00FFFF  li r24, -1
	ctx.r[24].s64 = -1;
	// 831A9D44: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831A9D48: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 831A9D4C: 419A01C0  beq cr6, 0x831a9f0c
	if ctx.cr[6].eq {
	pc = 0x831A9F0C; continue 'dispatch;
	}
	// 831A9D50: 7D5DF050  subf r10, r29, r30
	ctx.r[10].s64 = ctx.r[30].s64 - ctx.r[29].s64;
	// 831A9D54: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9D58: 7F0AE040  cmplw cr6, r10, r28
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[28].u32, &mut ctx.xer);
	// 831A9D5C: 40990060  ble cr6, 0x831a9dbc
	if !ctx.cr[6].gt {
	pc = 0x831A9DBC; continue 'dispatch;
	}
	// 831A9D60: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9D64: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 831A9D68: 419A0038  beq cr6, 0x831a9da0
	if ctx.cr[6].eq {
	pc = 0x831A9DA0; continue 'dispatch;
	}
	// 831A9D6C: 39270008  addi r9, r7, 8
	ctx.r[9].s64 = ctx.r[7].s64 + 8;
	// 831A9D70: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 831A9D74: 888A0000  lbz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9D78: 8AA90000  lbz r21, 0(r9)
	ctx.r[21].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9D7C: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A9D80: 7C952050  subf r4, r21, r4
	ctx.r[4].s64 = ctx.r[4].s64 - ctx.r[21].s64;
	// 831A9D84: 41820014  beq 0x831a9d98
	if ctx.cr[0].eq {
	pc = 0x831A9D98; continue 'dispatch;
	}
	// 831A9D88: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831A9D8C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831A9D90: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831A9D94: 419AFFE0  beq cr6, 0x831a9d74
	if ctx.cr[6].eq {
	pc = 0x831A9D74; continue 'dispatch;
	}
	// 831A9D98: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A9D9C: 40820020  bne 0x831a9dbc
	if !ctx.cr[0].eq {
	pc = 0x831A9DBC; continue 'dispatch;
	}
	// 831A9DA0: 81480014  lwz r10, 0x14(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 831A9DA4: 554A07BF  clrlwi. r10, r10, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831A9DA8: 40820008  bne 0x831a9db0
	if !ctx.cr[0].eq {
	pc = 0x831A9DB0; continue 'dispatch;
	}
	// 831A9DAC: 7D164378  mr r22, r8
	ctx.r[22].u64 = ctx.r[8].u64;
	// 831A9DB0: 83880004  lwz r28, 4(r8)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 831A9DB4: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 831A9DB8: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 831A9DBC: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9DC0: 7F0A2840  cmplw cr6, r10, r5
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[5].u32, &mut ctx.xer);
	// 831A9DC4: 419A0038  beq cr6, 0x831a9dfc
	if ctx.cr[6].eq {
	pc = 0x831A9DFC; continue 'dispatch;
	}
	// 831A9DC8: 39250008  addi r9, r5, 8
	ctx.r[9].s64 = ctx.r[5].s64 + 8;
	// 831A9DCC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 831A9DD0: 888A0000  lbz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9DD4: 8AA90000  lbz r21, 0(r9)
	ctx.r[21].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9DD8: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A9DDC: 7C952050  subf r4, r21, r4
	ctx.r[4].s64 = ctx.r[4].s64 - ctx.r[21].s64;
	// 831A9DE0: 41820014  beq 0x831a9df4
	if ctx.cr[0].eq {
	pc = 0x831A9DF4; continue 'dispatch;
	}
	// 831A9DE4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831A9DE8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831A9DEC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831A9DF0: 419AFFE0  beq cr6, 0x831a9dd0
	if ctx.cr[6].eq {
	pc = 0x831A9DD0; continue 'dispatch;
	}
	// 831A9DF4: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831A9DF8: 40820104  bne 0x831a9efc
	if !ctx.cr[0].eq {
	pc = 0x831A9EFC; continue 'dispatch;
	}
	// 831A9DFC: 8148000C  lwz r10, 0xc(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 831A9E00: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831A9E04: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831A9E08: 41980014  blt cr6, 0x831a9e1c
	if ctx.cr[6].lt {
	pc = 0x831A9E1C; continue 'dispatch;
	}
	// 831A9E0C: 7D2A182E  lwzx r9, r10, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 831A9E10: 80880010  lwz r4, 0x10(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 831A9E14: 7D29202E  lwzx r9, r9, r4
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 831A9E18: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 831A9E1C: 81480008  lwz r10, 8(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 831A9E20: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 831A9E24: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 831A9E28: 409A00D4  bne cr6, 0x831a9efc
	if !ctx.cr[6].eq {
	pc = 0x831A9EFC; continue 'dispatch;
	}
	// 831A9E2C: 7D5DF050  subf r10, r29, r30
	ctx.r[10].s64 = ctx.r[30].s64 - ctx.r[29].s64;
	// 831A9E30: 7F0AE040  cmplw cr6, r10, r28
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[28].u32, &mut ctx.xer);
	// 831A9E34: 419900B4  bgt cr6, 0x831a9ee8
	if ctx.cr[6].gt {
	pc = 0x831A9EE8; continue 'dispatch;
	}
	// 831A9E38: 5729063F  clrlwi. r9, r25, 0x18
	ctx.r[9].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831A9E3C: 418200C0  beq 0x831a9efc
	if ctx.cr[0].eq {
	pc = 0x831A9EFC; continue 'dispatch;
	}
	// 831A9E40: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831A9E44: 55290673  rlwinm. r9, r9, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831A9E48: 40820024  bne 0x831a9e6c
	if !ctx.cr[0].eq {
	pc = 0x831A9E6C; continue 'dispatch;
	}
	// 831A9E4C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 831A9E50: 409A0014  bne cr6, 0x831a9e64
	if !ctx.cr[6].eq {
	pc = 0x831A9E64; continue 'dispatch;
	}
	// 831A9E54: 81480014  lwz r10, 0x14(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 831A9E58: 554A07FF  clrlwi. r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831A9E5C: 41820008  beq 0x831a9e64
	if ctx.cr[0].eq {
	pc = 0x831A9E64; continue 'dispatch;
	}
	// 831A9E60: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 831A9E64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 831A9E68: 4800002C  b 0x831a9e94
	pc = 0x831A9E94; continue 'dispatch;
	// 831A9E6C: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 831A9E70: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831A9E74: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 831A9E78: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 831A9E7C: 814A0014  lwz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 831A9E80: 554907FF  clrlwi. r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831A9E84: 41820008  beq 0x831a9e8c
	if ctx.cr[0].eq {
	pc = 0x831A9E8C; continue 'dispatch;
	}
	// 831A9E88: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 831A9E8C: 7D4A50F8  nor r10, r10, r10
	ctx.r[10].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 831A9E90: 554AF7FE  rlwinm r10, r10, 0x1e, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 831A9E94: 5729063F  clrlwi. r9, r25, 0x18
	ctx.r[9].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831A9E98: 41820064  beq 0x831a9efc
	if ctx.cr[0].eq {
	pc = 0x831A9EFC; continue 'dispatch;
	}
	// 831A9E9C: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831A9EA0: 4182005C  beq 0x831a9efc
	if ctx.cr[0].eq {
	pc = 0x831A9EFC; continue 'dispatch;
	}
	// 831A9EA4: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831A9EA8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831A9EAC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831A9EB0: 41980014  blt cr6, 0x831a9ec4
	if ctx.cr[6].lt {
	pc = 0x831A9EC4; continue 'dispatch;
	}
	// 831A9EB4: 7D2A182E  lwzx r9, r10, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 831A9EB8: 811F0010  lwz r8, 0x10(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831A9EBC: 7D29402E  lwzx r9, r9, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 831A9EC0: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 831A9EC4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831A9EC8: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 831A9ECC: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 831A9ED0: 419A000C  beq cr6, 0x831a9edc
	if ctx.cr[6].eq {
	pc = 0x831A9EDC; continue 'dispatch;
	}
	// 831A9ED4: 7F185000  cmpw cr6, r24, r10
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[10].s32, &mut ctx.xer);
	// 831A9ED8: 409A0060  bne cr6, 0x831a9f38
	if !ctx.cr[6].eq {
	pc = 0x831A9F38; continue 'dispatch;
	}
	// 831A9EDC: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 831A9EE0: 7D585378  mr r24, r10
	ctx.r[24].u64 = ctx.r[10].u64;
	// 831A9EE4: 48000018  b 0x831a9efc
	pc = 0x831A9EFC; continue 'dispatch;
	// 831A9EE8: 81480014  lwz r10, 0x14(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 831A9EEC: 714A0005  andi. r10, r10, 5
	ctx.r[10].u64 = ctx.r[10].u64 & 5;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831A9EF0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831A9EF4: 40820008  bne 0x831a9efc
	if !ctx.cr[0].eq {
	pc = 0x831A9EFC; continue 'dispatch;
	}
	// 831A9EF8: 7D174378  mr r23, r8
	ctx.r[23].u64 = ctx.r[8].u64;
	// 831A9EFC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 831A9F00: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831A9F04: 7F1ED840  cmplw cr6, r30, r27
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[27].u32, &mut ctx.xer);
	// 831A9F08: 4198FE48  blt cr6, 0x831a9d50
	if ctx.cr[6].lt {
	pc = 0x831A9D50; continue 'dispatch;
	}
	// 831A9F0C: 572B063F  clrlwi. r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831A9F10: 41820014  beq 0x831a9f24
	if ctx.cr[0].eq {
	pc = 0x831A9F24; continue 'dispatch;
	}
	// 831A9F14: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 831A9F18: 419A000C  beq cr6, 0x831a9f24
	if ctx.cr[6].eq {
	pc = 0x831A9F24; continue 'dispatch;
	}
	// 831A9F1C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 831A9F20: 4800001C  b 0x831a9f3c
	pc = 0x831A9F3C; continue 'dispatch;
	// 831A9F24: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 831A9F28: 419A0010  beq cr6, 0x831a9f38
	if ctx.cr[6].eq {
	pc = 0x831A9F38; continue 'dispatch;
	}
	// 831A9F2C: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 831A9F30: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 831A9F34: 409A0008  bne cr6, 0x831a9f3c
	if !ctx.cr[6].eq {
	pc = 0x831A9F3C; continue 'dispatch;
	}
	// 831A9F38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831A9F3C: 4BFFE260  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831A9F40 size=8
    let mut pc: u32 = 0x831A9F40;
    'dispatch: loop {
        match pc {
            0x831A9F40 => {
    //   block [0x831A9F40..0x831A9F48)
	// 831A9F40: 8324280C  lwz r25, 0x280c(r4)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(10252 as u32) ) } as u64;
	// 831A9F44: 8224BF58  lwz r17, -0x40a8(r4)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-16552 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831A9F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831A9F48 size=360
    let mut pc: u32 = 0x831A9F48;
    'dispatch: loop {
        match pc {
            0x831A9F48 => {
    //   block [0x831A9F48..0x831AA0B0)
	// 831A9F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831A9F4C: 4BFFE221  bl 0x831a816c
	ctx.lr = 0x831A9F50;
	sub_831A8130(ctx, base);
	// 831A9F50: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 831A9F54: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831A9F58: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 831A9F5C: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 831A9F60: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 831A9F64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831A9F68: 419A0140  beq cr6, 0x831aa0a8
	if ctx.cr[6].eq {
	pc = 0x831AA0A8; continue 'dispatch;
	}
	// 831A9F6C: 60000000  nop
	// 831A9F70: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831A9F74: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831A9F78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9F7C: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831A9F80: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831A9F84: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831A9F88: 7FC81850  subf r30, r8, r3
	ctx.r[30].s64 = ctx.r[3].s64 - ctx.r[8].s64;
	// 831A9F8C: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 831A9F90: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831A9F94: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 831A9F98: 419A0014  beq cr6, 0x831a9fac
	if ctx.cr[6].eq {
	pc = 0x831A9FAC; continue 'dispatch;
	}
	// 831A9F9C: 7D4A1850  subf r10, r10, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[10].s64;
	// 831A9FA0: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831A9FA4: 7FCAF050  subf r30, r10, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[10].s64;
	// 831A9FA8: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 831A9FAC: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 831A9FB0: 7D041850  subf r8, r4, r3
	ctx.r[8].s64 = ctx.r[3].s64 - ctx.r[4].s64;
	// 831A9FB4: 7CDE4050  subf r6, r30, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[30].s64;
	// 831A9FB8: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 831A9FBC: 554807FF  clrlwi. r8, r10, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 831A9FC0: 40820014  bne 0x831a9fd4
	if !ctx.cr[0].eq {
	pc = 0x831A9FD4; continue 'dispatch;
	}
	// 831A9FC4: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 831A9FC8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 831A9FCC: 4BFFFA9D  bl 0x831a9a68
	ctx.lr = 0x831A9FD0;
	sub_831A9A68(ctx, base);
	// 831A9FD0: 48000028  b 0x831a9ff8
	pc = 0x831A9FF8; continue 'dispatch;
	// 831A9FD4: 554A07BD  rlwinm. r10, r10, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831A9FD8: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 831A9FDC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 831A9FE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831A9FE4: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 831A9FE8: 4082000C  bne 0x831a9ff4
	if !ctx.cr[0].eq {
	pc = 0x831A9FF4; continue 'dispatch;
	}
	// 831A9FEC: 4BFFFB7D  bl 0x831a9b68
	ctx.lr = 0x831A9FF0;
	sub_831A9B68(ctx, base);
	// 831A9FF0: 48000008  b 0x831a9ff8
	pc = 0x831A9FF8; continue 'dispatch;
	// 831A9FF4: 4BFFFD1D  bl 0x831a9d10
	ctx.lr = 0x831A9FF8;
	sub_831A9D10(ctx, base);
	// 831A9FF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831A9FFC: 419A003C  beq cr6, 0x831aa038
	if ctx.cr[6].eq {
	pc = 0x831AA038; continue 'dispatch;
	}
	// 831AA000: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 831AA004: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831AA008: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AA00C: 41980018  blt cr6, 0x831aa024
	if ctx.cr[6].lt {
	pc = 0x831AA024; continue 'dispatch;
	}
	// 831AA010: 7D4BF02E  lwzx r10, r11, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 831AA014: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 831AA018: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 831AA01C: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 831AA020: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831AA024: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831AA028: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831AA02C: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 831AA030: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 831AA034: 48000038  b 0x831aa06c
	pc = 0x831AA06C; continue 'dispatch;
	// 831AA038: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831AA03C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 831AA040: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 831AA044: 419A0028  beq cr6, 0x831aa06c
	if ctx.cr[6].eq {
	pc = 0x831AA06C; continue 'dispatch;
	}
	// 831AA048: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831AA04C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 831AA050: 388BC3F0  addi r4, r11, -0x3c10
	ctx.r[4].s64 = ctx.r[11].s64 + -15376;
	// 831AA054: 4BFFF875  bl 0x831a98c8
	ctx.lr = 0x831AA058;
	sub_831A98C8(ctx, base);
	// 831AA058: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 831AA05C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 831AA060: 388BCB5C  addi r4, r11, -0x34a4
	ctx.r[4].s64 = ctx.r[11].s64 + -13476;
	// 831AA064: 48006BC5  bl 0x831b0c28
	ctx.lr = 0x831AA068;
	sub_831B0C28(ctx, base);
	// 831AA068: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831AA06C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831AA070: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831AA074: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831AA078: 4800002C  b 0x831aa0a4
	pc = 0x831AA0A4; continue 'dispatch;
	// 831AA07C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831AA080: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831AA084: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 831AA088: 388BC3A8  addi r4, r11, -0x3c58
	ctx.r[4].s64 = ctx.r[11].s64 + -15448;
	// 831AA08C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 831AA090: 4BFFF8A9  bl 0x831a9938
	ctx.lr = 0x831AA094;
	sub_831A9938(ctx, base);
	// 831AA094: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 831AA098: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 831AA09C: 388BCAF8  addi r4, r11, -0x3508
	ctx.r[4].s64 = ctx.r[11].s64 + -13576;
	// 831AA0A0: 48006B89  bl 0x831b0c28
	ctx.lr = 0x831AA0A4;
	sub_831B0C28(ctx, base);
	// 831AA0A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831AA0A8: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 831AA0AC: 4BFFE110  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA0B0 size=36
    let mut pc: u32 = 0x831AA0B0;
    'dispatch: loop {
        match pc {
            0x831AA0B0 => {
    //   block [0x831AA0B0..0x831AA0D4)
	// 831AA0B0: 3D60C000  lis r11, -0x4000
	ctx.r[11].s64 = -1073741824;
	// 831AA0B4: 616B0005  ori r11, r11, 5
	ctx.r[11].u64 = ctx.r[11].u64 | 5;
	// 831AA0B8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AA0BC: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AA0C0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831AA0C4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 831AA0C8: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 831AA0CC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831AA0D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AA0D8 size=60
    let mut pc: u32 = 0x831AA0D8;
    'dispatch: loop {
        match pc {
            0x831AA0D8 => {
    //   block [0x831AA0D8..0x831AA114)
	// 831AA0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AA0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831AA0E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831AA0E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AA0E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831AA0EC: 4BC465AD  bl 0x82df0698
	ctx.lr = 0x831AA0F0;
	sub_82DF0698(ctx, base);
	// 831AA0F0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831AA0F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831AA0F8: 396BC37C  addi r11, r11, -0x3c84
	ctx.r[11].s64 = ctx.r[11].s64 + -15492;
	// 831AA0FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831AA100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831AA104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831AA108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831AA10C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831AA110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AA120 size=60
    let mut pc: u32 = 0x831AA120;
    'dispatch: loop {
        match pc {
            0x831AA120 => {
    //   block [0x831AA120..0x831AA15C)
	// 831AA120: 2F850000  cmpwi cr7, r5, 0
	ctx.cr[7].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 831AA124: 2C850001  cmpwi cr1, r5, 1
	ctx.cr[1].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 831AA128: 419E002C  beq cr7, 0x831aa154
	if ctx.cr[7].eq {
	pc = 0x831AA154; continue 'dispatch;
	}
	// 831AA12C: 88C30000  lbz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AA130: 7F062000  cmpw cr6, r6, r4
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[4].s32, &mut ctx.xer);
	// 831AA134: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 831AA138: 419A0020  beq cr6, 0x831aa158
	if ctx.cr[6].eq {
	pc = 0x831AA158; continue 'dispatch;
	}
	// 831AA13C: 41860018  beq cr1, 0x831aa154
	if ctx.cr[1].eq {
	pc = 0x831AA154; continue 'dispatch;
	}
	// 831AA140: 7CA903A6  mtctr r5
	ctx.ctr.u64 = ctx.r[5].u64;
	// 831AA144: 8CC30001  lbzu r6, 1(r3)
	ea = ctx.r[3].u32.wrapping_add(1 as u32);
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[3].u32 = ea;
	// 831AA148: 7C062000  cmpw r6, r4
	ctx.cr[0].compare_i32(ctx.r[4].s32, ctx.r[0].s32, &mut ctx.xer);
	// 831AA14C: 4002FFF8  bdnzf eq, 0x831aa144
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 && !ctx.cr[0].eq {
	pc = 0x831AA144; continue 'dispatch;
	}
	// 831AA150: 41820008  beq 0x831aa158
	if ctx.cr[0].eq {
	pc = 0x831AA158; continue 'dispatch;
	}
	// 831AA154: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AA158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA160 size=28
    let mut pc: u32 = 0x831AA160;
    'dispatch: loop {
        match pc {
            0x831AA160 => {
    //   block [0x831AA160..0x831AA17C)
	// 831AA160: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AA164: 546A083C  slwi r10, r3, 1
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831AA168: 396BF4A0  addi r11, r11, -0xb60
	ctx.r[11].s64 = ctx.r[11].s64 + -2912;
	// 831AA16C: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 831AA170: 7D6A5A2E  lhzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831AA174: 71630103  andi. r3, r11, 0x103
	ctx.r[3].u64 = ctx.r[11].u64 & 259;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831AA178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA180 size=28
    let mut pc: u32 = 0x831AA180;
    'dispatch: loop {
        match pc {
            0x831AA180 => {
    //   block [0x831AA180..0x831AA19C)
	// 831AA180: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AA184: 546A083C  slwi r10, r3, 1
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831AA188: 396BF4A0  addi r11, r11, -0xb60
	ctx.r[11].s64 = ctx.r[11].s64 + -2912;
	// 831AA18C: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 831AA190: 7D6A5A2E  lhzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831AA194: 5563077A  rlwinm r3, r11, 0, 0x1d, 0x1d
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AA198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA1A0 size=28
    let mut pc: u32 = 0x831AA1A0;
    'dispatch: loop {
        match pc {
            0x831AA1A0 => {
    //   block [0x831AA1A0..0x831AA1BC)
	// 831AA1A0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AA1A4: 546A083C  slwi r10, r3, 1
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831AA1A8: 396BF4A0  addi r11, r11, -0xb60
	ctx.r[11].s64 = ctx.r[11].s64 + -2912;
	// 831AA1AC: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 831AA1B0: 7D6A5A2E  lhzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831AA1B4: 55630630  rlwinm r3, r11, 0, 0x18, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AA1B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA1C0 size=28
    let mut pc: u32 = 0x831AA1C0;
    'dispatch: loop {
        match pc {
            0x831AA1C0 => {
    //   block [0x831AA1C0..0x831AA1DC)
	// 831AA1C0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AA1C4: 546A083C  slwi r10, r3, 1
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831AA1C8: 396BF4A0  addi r11, r11, -0xb60
	ctx.r[11].s64 = ctx.r[11].s64 + -2912;
	// 831AA1CC: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 831AA1D0: 7D6A5A2E  lhzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831AA1D4: 55630738  rlwinm r3, r11, 0, 0x1c, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AA1D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA1E0 size=28
    let mut pc: u32 = 0x831AA1E0;
    'dispatch: loop {
        match pc {
            0x831AA1E0 => {
    //   block [0x831AA1E0..0x831AA1FC)
	// 831AA1E0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AA1E4: 546A083C  slwi r10, r3, 1
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831AA1E8: 396BF4A0  addi r11, r11, -0xb60
	ctx.r[11].s64 = ctx.r[11].s64 + -2912;
	// 831AA1EC: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 831AA1F0: 7D6A5A2E  lhzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831AA1F4: 556306F6  rlwinm r3, r11, 0, 0x1b, 0x1b
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AA1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA200 size=28
    let mut pc: u32 = 0x831AA200;
    'dispatch: loop {
        match pc {
            0x831AA200 => {
    //   block [0x831AA200..0x831AA21C)
	// 831AA200: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AA204: 546A083C  slwi r10, r3, 1
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831AA208: 396BF4A0  addi r11, r11, -0xb60
	ctx.r[11].s64 = ctx.r[11].s64 + -2912;
	// 831AA20C: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 831AA210: 7D6A5A2E  lhzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831AA214: 71630107  andi. r3, r11, 0x107
	ctx.r[3].u64 = ctx.r[11].u64 & 263;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831AA218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AA220 size=752
    let mut pc: u32 = 0x831AA220;
    'dispatch: loop {
        match pc {
            0x831AA220 => {
    //   block [0x831AA220..0x831AA510)
	// 831AA220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AA224: 4BFFDF31  bl 0x831a8154
	ctx.lr = 0x831AA228;
	sub_831A8130(ctx, base);
	// 831AA228: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AA22C: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 831AA230: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 831AA234: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 831AA238: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 831AA23C: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 831AA240: 419A0008  beq cr6, 0x831aa248
	if ctx.cr[6].eq {
	pc = 0x831AA248; continue 'dispatch;
	}
	// 831AA244: 93370000  stw r25, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 831AA248: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 831AA24C: 409A002C  bne cr6, 0x831aa278
	if !ctx.cr[6].eq {
	pc = 0x831AA278; continue 'dispatch;
	}
	// 831AA250: 48006B11  bl 0x831b0d60
	ctx.lr = 0x831AA254;
	sub_831B0D60(ctx, base);
	// 831AA254: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 831AA258: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831AA25C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831AA260: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831AA264: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831AA268: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831AA26C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AA270: 4800ABA1  bl 0x831b4e10
	ctx.lr = 0x831AA274;
	sub_831B4E10(ctx, base);
	// 831AA274: 48000290  b 0x831aa504
	pc = 0x831AA504; continue 'dispatch;
	// 831AA278: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 831AA27C: 419A0014  beq cr6, 0x831aa290
	if ctx.cr[6].eq {
	pc = 0x831AA290; continue 'dispatch;
	}
	// 831AA280: 2F1C0002  cmpwi cr6, r28, 2
	ctx.cr[6].compare_i32(ctx.r[28].s32, 2, &mut ctx.xer);
	// 831AA284: 4198FFCC  blt cr6, 0x831aa250
	if ctx.cr[6].lt {
	pc = 0x831AA250; continue 'dispatch;
	}
	// 831AA288: 2F1C0024  cmpwi cr6, r28, 0x24
	ctx.cr[6].compare_i32(ctx.r[28].s32, 36, &mut ctx.xer);
	// 831AA28C: 4199FFC4  bgt cr6, 0x831aa250
	if ctx.cr[6].gt {
	pc = 0x831AA250; continue 'dispatch;
	}
	// 831AA290: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AA294: 8BF90000  lbz r31, 0(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AA298: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 831AA29C: 3BB90001  addi r29, r25, 1
	ctx.r[29].s64 = ctx.r[25].s64 + 1;
	// 831AA2A0: 3BCBF580  addi r30, r11, -0xa80
	ctx.r[30].s64 = ctx.r[11].s64 + -2688;
	// 831AA2A4: 814BF580  lwz r10, -0xa80(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2688 as u32) ) } as u64;
	// 831AA2A8: 816A00AC  lwz r11, 0xac(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(172 as u32) ) } as u64;
	// 831AA2AC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831AA2B0: 4099001C  ble cr6, 0x831aa2cc
	if !ctx.cr[6].gt {
	pc = 0x831AA2CC; continue 'dispatch;
	}
	// 831AA2B4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 831AA2B8: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 831AA2BC: 57E3063E  clrlwi r3, r31, 0x18
	ctx.r[3].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 831AA2C0: 48006BC1  bl 0x831b0e80
	ctx.lr = 0x831AA2C4;
	sub_831B0E80(ctx, base);
	// 831AA2C4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AA2C8: 48000014  b 0x831aa2dc
	pc = 0x831AA2DC; continue 'dispatch;
	// 831AA2CC: 816A00C8  lwz r11, 0xc8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(200 as u32) ) } as u64;
	// 831AA2D0: 57E90DFC  rlwinm r9, r31, 1, 0x17, 0x1e
	ctx.r[9].u64 = ctx.r[31].u32 as u64 & 0x7FFFFFFFu64;
	// 831AA2D4: 7D695A2E  lhzx r11, r9, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831AA2D8: 55630738  rlwinm r3, r11, 0, 0x1c, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AA2DC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831AA2E0: 419A0010  beq cr6, 0x831aa2f0
	if ctx.cr[6].eq {
	pc = 0x831AA2F0; continue 'dispatch;
	}
	// 831AA2E4: 8BFD0000  lbz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AA2E8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 831AA2EC: 4BFFFFBC  b 0x831aa2a8
	pc = 0x831AA2A8; continue 'dispatch;
	// 831AA2F0: 7FEB0774  extsb r11, r31
	ctx.r[11].s64 = ctx.r[31].s8 as i64;
	// 831AA2F4: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 831AA2F8: 409A000C  bne cr6, 0x831aa304
	if !ctx.cr[6].eq {
	pc = 0x831AA304; continue 'dispatch;
	}
	// 831AA2FC: 63180002  ori r24, r24, 2
	ctx.r[24].u64 = ctx.r[24].u64 | 2;
	// 831AA300: 4800000C  b 0x831aa30c
	pc = 0x831AA30C; continue 'dispatch;
	// 831AA304: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 831AA308: 409A000C  bne cr6, 0x831aa314
	if !ctx.cr[6].eq {
	pc = 0x831AA314; continue 'dispatch;
	}
	// 831AA30C: 8BFD0000  lbz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AA310: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 831AA314: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 831AA318: 419801E0  blt cr6, 0x831aa4f8
	if ctx.cr[6].lt {
	pc = 0x831AA4F8; continue 'dispatch;
	}
	// 831AA31C: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 831AA320: 419A01D8  beq cr6, 0x831aa4f8
	if ctx.cr[6].eq {
	pc = 0x831AA4F8; continue 'dispatch;
	}
	// 831AA324: 2F1C0024  cmpwi cr6, r28, 0x24
	ctx.cr[6].compare_i32(ctx.r[28].s32, 36, &mut ctx.xer);
	// 831AA328: 419901D0  bgt cr6, 0x831aa4f8
	if ctx.cr[6].gt {
	pc = 0x831AA4F8; continue 'dispatch;
	}
	// 831AA32C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 831AA330: 409A0040  bne cr6, 0x831aa370
	if !ctx.cr[6].eq {
	pc = 0x831AA370; continue 'dispatch;
	}
	// 831AA334: 7FEB0774  extsb r11, r31
	ctx.r[11].s64 = ctx.r[31].s8 as i64;
	// 831AA338: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 831AA33C: 419A000C  beq cr6, 0x831aa348
	if ctx.cr[6].eq {
	pc = 0x831AA348; continue 'dispatch;
	}
	// 831AA340: 3B80000A  li r28, 0xa
	ctx.r[28].s64 = 10;
	// 831AA344: 48000064  b 0x831aa3a8
	pc = 0x831AA3A8; continue 'dispatch;
	// 831AA348: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AA34C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 831AA350: 2F0B0078  cmpwi cr6, r11, 0x78
	ctx.cr[6].compare_i32(ctx.r[11].s32, 120, &mut ctx.xer);
	// 831AA354: 419A0014  beq cr6, 0x831aa368
	if ctx.cr[6].eq {
	pc = 0x831AA368; continue 'dispatch;
	}
	// 831AA358: 2F0B0058  cmpwi cr6, r11, 0x58
	ctx.cr[6].compare_i32(ctx.r[11].s32, 88, &mut ctx.xer);
	// 831AA35C: 419A000C  beq cr6, 0x831aa368
	if ctx.cr[6].eq {
	pc = 0x831AA368; continue 'dispatch;
	}
	// 831AA360: 3B800008  li r28, 8
	ctx.r[28].s64 = 8;
	// 831AA364: 48000044  b 0x831aa3a8
	pc = 0x831AA3A8; continue 'dispatch;
	// 831AA368: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 831AA36C: 4800000C  b 0x831aa378
	pc = 0x831AA378; continue 'dispatch;
	// 831AA370: 2F1C0010  cmpwi cr6, r28, 0x10
	ctx.cr[6].compare_i32(ctx.r[28].s32, 16, &mut ctx.xer);
	// 831AA374: 409A0034  bne cr6, 0x831aa3a8
	if !ctx.cr[6].eq {
	pc = 0x831AA3A8; continue 'dispatch;
	}
	// 831AA378: 7FEB0774  extsb r11, r31
	ctx.r[11].s64 = ctx.r[31].s8 as i64;
	// 831AA37C: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 831AA380: 409A0028  bne cr6, 0x831aa3a8
	if !ctx.cr[6].eq {
	pc = 0x831AA3A8; continue 'dispatch;
	}
	// 831AA384: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AA388: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 831AA38C: 2F0B0078  cmpwi cr6, r11, 0x78
	ctx.cr[6].compare_i32(ctx.r[11].s32, 120, &mut ctx.xer);
	// 831AA390: 419A000C  beq cr6, 0x831aa39c
	if ctx.cr[6].eq {
	pc = 0x831AA39C; continue 'dispatch;
	}
	// 831AA394: 2F0B0058  cmpwi cr6, r11, 0x58
	ctx.cr[6].compare_i32(ctx.r[11].s32, 88, &mut ctx.xer);
	// 831AA398: 409A0010  bne cr6, 0x831aa3a8
	if !ctx.cr[6].eq {
	pc = 0x831AA3A8; continue 'dispatch;
	}
	// 831AA39C: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 831AA3A0: 8BFD0001  lbz r31, 1(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(1 as u32) ) } as u64;
	// 831AA3A4: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 831AA3A8: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 831AA3AC: 810A00C8  lwz r8, 0xc8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(200 as u32) ) } as u64;
	// 831AA3B0: 0CDC0000  twi 6, r28, 0
	// 831AA3B4: 7D3BE396  divwu r9, r27, r28
	ctx.r[9].u32 = ctx.r[27].u32 / ctx.r[28].u32;
	// 831AA3B8: 57EB0DFC  rlwinm r11, r31, 1, 0x17, 0x1e
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x7FFFFFFFu64;
	// 831AA3BC: 7D6B422E  lhzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 831AA3C0: 556A077B  rlwinm. r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AA3C4: 41820010  beq 0x831aa3d4
	if ctx.cr[0].eq {
	pc = 0x831AA3D4; continue 'dispatch;
	}
	// 831AA3C8: 7FEB0774  extsb r11, r31
	ctx.r[11].s64 = ctx.r[31].s8 as i64;
	// 831AA3CC: 396BFFD0  addi r11, r11, -0x30
	ctx.r[11].s64 = ctx.r[11].s64 + -48;
	// 831AA3D0: 4800002C  b 0x831aa3fc
	pc = 0x831AA3FC; continue 'dispatch;
	// 831AA3D4: 716B0103  andi. r11, r11, 0x103
	ctx.r[11].u64 = ctx.r[11].u64 & 259;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AA3D8: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831AA3DC: 41820060  beq 0x831aa43c
	if ctx.cr[0].eq {
	pc = 0x831AA43C; continue 'dispatch;
	}
	// 831AA3E0: 7FEB0774  extsb r11, r31
	ctx.r[11].s64 = ctx.r[31].s8 as i64;
	// 831AA3E4: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 831AA3E8: 41980010  blt cr6, 0x831aa3f8
	if ctx.cr[6].lt {
	pc = 0x831AA3F8; continue 'dispatch;
	}
	// 831AA3EC: 2F0B007A  cmpwi cr6, r11, 0x7a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 122, &mut ctx.xer);
	// 831AA3F0: 41990008  bgt cr6, 0x831aa3f8
	if ctx.cr[6].gt {
	pc = 0x831AA3F8; continue 'dispatch;
	}
	// 831AA3F4: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 831AA3F8: 396BFFC9  addi r11, r11, -0x37
	ctx.r[11].s64 = ctx.r[11].s64 + -55;
	// 831AA3FC: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 831AA400: 4098003C  bge cr6, 0x831aa43c
	if !ctx.cr[6].lt {
	pc = 0x831AA43C; continue 'dispatch;
	}
	// 831AA404: 63180008  ori r24, r24, 8
	ctx.r[24].u64 = ctx.r[24].u64 | 8;
	// 831AA408: 7F1A4840  cmplw cr6, r26, r9
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[9].u32, &mut ctx.xer);
	// 831AA40C: 41980050  blt cr6, 0x831aa45c
	if ctx.cr[6].lt {
	pc = 0x831AA45C; continue 'dispatch;
	}
	// 831AA410: 409A0020  bne cr6, 0x831aa430
	if !ctx.cr[6].eq {
	pc = 0x831AA430; continue 'dispatch;
	}
	// 831AA414: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 831AA418: 0CDC0000  twi 6, r28, 0
	// 831AA41C: 7CEAE396  divwu r7, r10, r28
	ctx.r[7].u32 = ctx.r[10].u32 / ctx.r[28].u32;
	// 831AA420: 7CE7E1D6  mullw r7, r7, r28
	ctx.r[7].s64 = (ctx.r[7].s32 as i64) * (ctx.r[28].s32 as i64);
	// 831AA424: 7D475050  subf r10, r7, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 831AA428: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831AA42C: 40990030  ble cr6, 0x831aa45c
	if !ctx.cr[6].gt {
	pc = 0x831AA45C; continue 'dispatch;
	}
	// 831AA430: 63180004  ori r24, r24, 4
	ctx.r[24].u64 = ctx.r[24].u64 | 4;
	// 831AA434: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 831AA438: 409A002C  bne cr6, 0x831aa464
	if !ctx.cr[6].eq {
	pc = 0x831AA464; continue 'dispatch;
	}
	// 831AA43C: 570B0739  rlwinm. r11, r24, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AA440: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 831AA444: 4082002C  bne 0x831aa470
	if !ctx.cr[0].eq {
	pc = 0x831AA470; continue 'dispatch;
	}
	// 831AA448: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 831AA44C: 419A0008  beq cr6, 0x831aa454
	if ctx.cr[6].eq {
	pc = 0x831AA454; continue 'dispatch;
	}
	// 831AA450: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 831AA454: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 831AA458: 48000080  b 0x831aa4d8
	pc = 0x831AA4D8; continue 'dispatch;
	// 831AA45C: 7D5AE1D6  mullw r10, r26, r28
	ctx.r[10].s64 = (ctx.r[26].s32 as i64) * (ctx.r[28].s32 as i64);
	// 831AA460: 7F4A5A14  add r26, r10, r11
	ctx.r[26].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831AA464: 8BFD0000  lbz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AA468: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 831AA46C: 4BFFFF4C  b 0x831aa3b8
	pc = 0x831AA3B8; continue 'dispatch;
	// 831AA470: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 831AA474: 570B077B  rlwinm. r11, r24, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AA478: 3FC08000  lis r30, -0x8000
	ctx.r[30].s64 = -2147483648;
	// 831AA47C: 615FFFFF  ori r31, r10, 0xffff
	ctx.r[31].u64 = ctx.r[10].u64 | 65535;
	// 831AA480: 4082002C  bne 0x831aa4ac
	if !ctx.cr[0].eq {
	pc = 0x831AA4AC; continue 'dispatch;
	}
	// 831AA484: 570B07FF  clrlwi. r11, r24, 0x1f
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AA488: 40820050  bne 0x831aa4d8
	if !ctx.cr[0].eq {
	pc = 0x831AA4D8; continue 'dispatch;
	}
	// 831AA48C: 570B07BD  rlwinm. r11, r24, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AA490: 4182000C  beq 0x831aa49c
	if ctx.cr[0].eq {
	pc = 0x831AA49C; continue 'dispatch;
	}
	// 831AA494: 7F1AF040  cmplw cr6, r26, r30
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[30].u32, &mut ctx.xer);
	// 831AA498: 41990014  bgt cr6, 0x831aa4ac
	if ctx.cr[6].gt {
	pc = 0x831AA4AC; continue 'dispatch;
	}
	// 831AA49C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AA4A0: 409A0038  bne cr6, 0x831aa4d8
	if !ctx.cr[6].eq {
	pc = 0x831AA4D8; continue 'dispatch;
	}
	// 831AA4A4: 7F1AF840  cmplw cr6, r26, r31
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[31].u32, &mut ctx.xer);
	// 831AA4A8: 40990030  ble cr6, 0x831aa4d8
	if !ctx.cr[6].gt {
	pc = 0x831AA4D8; continue 'dispatch;
	}
	// 831AA4AC: 480068B5  bl 0x831b0d60
	ctx.lr = 0x831AA4B0;
	sub_831B0D60(ctx, base);
	// 831AA4B0: 39600022  li r11, 0x22
	ctx.r[11].s64 = 34;
	// 831AA4B4: 570A07FF  clrlwi. r10, r24, 0x1f
	ctx.r[10].u64 = ctx.r[24].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AA4B8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831AA4BC: 4182000C  beq 0x831aa4c8
	if ctx.cr[0].eq {
	pc = 0x831AA4C8; continue 'dispatch;
	}
	// 831AA4C0: 7F7ADB78  mr r26, r27
	ctx.r[26].u64 = ctx.r[27].u64;
	// 831AA4C4: 48000014  b 0x831aa4d8
	pc = 0x831AA4D8; continue 'dispatch;
	// 831AA4C8: 570B07BD  rlwinm. r11, r24, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AA4CC: 7FDAF378  mr r26, r30
	ctx.r[26].u64 = ctx.r[30].u64;
	// 831AA4D0: 40820008  bne 0x831aa4d8
	if !ctx.cr[0].eq {
	pc = 0x831AA4D8; continue 'dispatch;
	}
	// 831AA4D4: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 831AA4D8: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 831AA4DC: 419A0008  beq cr6, 0x831aa4e4
	if ctx.cr[6].eq {
	pc = 0x831AA4E4; continue 'dispatch;
	}
	// 831AA4E0: 93B70000  stw r29, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 831AA4E4: 570B07BD  rlwinm. r11, r24, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AA4E8: 41820008  beq 0x831aa4f0
	if ctx.cr[0].eq {
	pc = 0x831AA4F0; continue 'dispatch;
	}
	// 831AA4EC: 7F5A00D0  neg r26, r26
	ctx.r[26].s64 = -ctx.r[26].s64;
	// 831AA4F0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 831AA4F4: 48000014  b 0x831aa508
	pc = 0x831AA508; continue 'dispatch;
	// 831AA4F8: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 831AA4FC: 419A0008  beq cr6, 0x831aa504
	if ctx.cr[6].eq {
	pc = 0x831AA504; continue 'dispatch;
	}
	// 831AA500: 93370000  stw r25, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 831AA504: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AA508: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831AA50C: 4BFFDC98  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA510 size=32
    let mut pc: u32 = 0x831AA510;
    'dispatch: loop {
        match pc {
            0x831AA510 => {
    //   block [0x831AA510..0x831AA530)
	// 831AA510: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831AA514: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 831AA518: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 831AA51C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 831AA520: 386AF580  addi r3, r10, -0xa80
	ctx.r[3].s64 = ctx.r[10].s64 + -2688;
	// 831AA524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831AA528: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 831AA52C: 4BFFFCF4  b 0x831aa220
	sub_831AA220(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA530 size=32
    let mut pc: u32 = 0x831AA530;
    'dispatch: loop {
        match pc {
            0x831AA530 => {
    //   block [0x831AA530..0x831AA550)
	// 831AA530: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831AA534: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 831AA538: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 831AA53C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 831AA540: 386AF580  addi r3, r10, -0xa80
	ctx.r[3].s64 = ctx.r[10].s64 + -2688;
	// 831AA544: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 831AA548: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 831AA54C: 4BFFFCD4  b 0x831aa220
	sub_831AA220(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AA550 size=408
    let mut pc: u32 = 0x831AA550;
    'dispatch: loop {
        match pc {
            0x831AA550 => {
    //   block [0x831AA550..0x831AA6E8)
	// 831AA550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AA554: 4BFFDC15  bl 0x831a8168
	ctx.lr = 0x831AA558;
	sub_831A8130(ctx, base);
	// 831AA558: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 831AA55C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AA560: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 831AA564: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831AA568: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 831AA56C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 831AA570: 419A0008  beq cr6, 0x831aa578
	if ctx.cr[6].eq {
	pc = 0x831AA578; continue 'dispatch;
	}
	// 831AA574: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 831AA578: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 831AA57C: 409A0034  bne cr6, 0x831aa5b0
	if !ctx.cr[6].eq {
	pc = 0x831AA5B0; continue 'dispatch;
	}
	// 831AA580: 480067E1  bl 0x831b0d60
	ctx.lr = 0x831AA584;
	sub_831B0D60(ctx, base);
	// 831AA584: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 831AA588: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831AA58C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831AA590: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831AA594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831AA598: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831AA59C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AA5A0: 4800A871  bl 0x831b4e10
	ctx.lr = 0x831AA5A4;
	sub_831B4E10(ctx, base);
	// 831AA5A4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 831AA5A8: C82BD228  lfd f1, -0x2dd8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11736 as u32) ) };
	// 831AA5AC: 48000130  b 0x831aa6dc
	pc = 0x831AA6DC; continue 'dispatch;
	// 831AA5B0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AA5B4: 3BCBF580  addi r30, r11, -0xa80
	ctx.r[30].s64 = ctx.r[11].s64 + -2688;
	// 831AA5B8: 816BF580  lwz r11, -0xa80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2688 as u32) ) } as u64;
	// 831AA5BC: 814B00AC  lwz r10, 0xac(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 831AA5C0: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 831AA5C4: 4099001C  ble cr6, 0x831aa5e0
	if !ctx.cr[6].gt {
	pc = 0x831AA5E0; continue 'dispatch;
	}
	// 831AA5C8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 831AA5CC: 887F0000  lbz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AA5D0: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 831AA5D4: 480068AD  bl 0x831b0e80
	ctx.lr = 0x831AA5D8;
	sub_831B0E80(ctx, base);
	// 831AA5D8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AA5DC: 48000018  b 0x831aa5f4
	pc = 0x831AA5F4; continue 'dispatch;
	// 831AA5E0: 895F0000  lbz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AA5E4: 812B00C8  lwz r9, 0xc8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 831AA5E8: 554A083E  rotlwi r10, r10, 1
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 831AA5EC: 7D4A4A2E  lhzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 831AA5F0: 55430738  rlwinm r3, r10, 0, 0x1c, 0x1c
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 831AA5F4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831AA5F8: 419A000C  beq cr6, 0x831aa604
	if ctx.cr[6].eq {
	pc = 0x831AA604; continue 'dispatch;
	}
	// 831AA5FC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 831AA600: 4BFFFFBC  b 0x831aa5bc
	pc = 0x831AA5BC; continue 'dispatch;
	// 831AA604: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 831AA608: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AA60C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831AA610: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831AA614: 409AFFF4  bne cr6, 0x831aa608
	if !ctx.cr[6].eq {
	pc = 0x831AA608; continue 'dispatch;
	}
	// 831AA618: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 831AA61C: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 831AA620: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831AA624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831AA628: 5565003E  slwi r5, r11, 0
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831AA62C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831AA630: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831AA634: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831AA638: 4800C809  bl 0x831b6e40
	ctx.lr = 0x831AA63C;
	sub_831B6E40(ctx, base);
	// 831AA63C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 831AA640: 419A0010  beq cr6, 0x831aa650
	if ctx.cr[6].eq {
	pc = 0x831AA650; continue 'dispatch;
	}
	// 831AA644: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831AA648: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 831AA64C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831AA650: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AA654: 716A0240  andi. r10, r11, 0x240
	ctx.r[10].u64 = ctx.r[11].u64 & 576;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AA658: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831AA65C: 4182001C  beq 0x831aa678
	if ctx.cr[0].eq {
	pc = 0x831AA678; continue 'dispatch;
	}
	// 831AA660: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 831AA664: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 831AA668: CBEBD228  lfd f31, -0x2dd8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11736 as u32) ) };
	// 831AA66C: 419A006C  beq cr6, 0x831aa6d8
	if ctx.cr[6].eq {
	pc = 0x831AA6D8; continue 'dispatch;
	}
	// 831AA670: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 831AA674: 48000064  b 0x831aa6d8
	pc = 0x831AA6D8; continue 'dispatch;
	// 831AA678: 716A0081  andi. r10, r11, 0x81
	ctx.r[10].u64 = ctx.r[11].u64 & 129;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AA67C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831AA680: 41820028  beq 0x831aa6a8
	if ctx.cr[0].eq {
	pc = 0x831AA6A8; continue 'dispatch;
	}
	// 831AA684: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AA688: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 831AA68C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831AA690: 409A0010  bne cr6, 0x831aa6a0
	if !ctx.cr[6].eq {
	pc = 0x831AA6A0; continue 'dispatch;
	}
	// 831AA694: C80BD700  lfd f0, -0x2900(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-10496 as u32) ) };
	// 831AA698: FFE00050  fneg f31, f0
	ctx.f[31].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 831AA69C: 48000028  b 0x831aa6c4
	pc = 0x831AA6C4; continue 'dispatch;
	// 831AA6A0: CBEBD700  lfd f31, -0x2900(r11)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-10496 as u32) ) };
	// 831AA6A4: 48000020  b 0x831aa6c4
	pc = 0x831AA6C4; continue 'dispatch;
	// 831AA6A8: 556B05EF  rlwinm. r11, r11, 0, 0x17, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AA6AC: 41820028  beq 0x831aa6d4
	if ctx.cr[0].eq {
	pc = 0x831AA6D4; continue 'dispatch;
	}
	// 831AA6B0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 831AA6B4: C8030010  lfd f0, 0x10(r3)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 831AA6B8: CBEBD228  lfd f31, -0x2dd8(r11)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11736 as u32) ) };
	// 831AA6BC: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 831AA6C0: 409A0014  bne cr6, 0x831aa6d4
	if !ctx.cr[6].eq {
	pc = 0x831AA6D4; continue 'dispatch;
	}
	// 831AA6C4: 4800669D  bl 0x831b0d60
	ctx.lr = 0x831AA6C8;
	sub_831B0D60(ctx, base);
	// 831AA6C8: 39600022  li r11, 0x22
	ctx.r[11].s64 = 34;
	// 831AA6CC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831AA6D0: 48000008  b 0x831aa6d8
	pc = 0x831AA6D8; continue 'dispatch;
	// 831AA6D4: CBE30010  lfd f31, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 831AA6D8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 831AA6DC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831AA6E0: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 831AA6E4: 4BFFDAD4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA6E8 size=8
    let mut pc: u32 = 0x831AA6E8;
    'dispatch: loop {
        match pc {
            0x831AA6E8 => {
    //   block [0x831AA6E8..0x831AA6F0)
	// 831AA6E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831AA6EC: 4BFFFE64  b 0x831aa550
	sub_831AA550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AA6F0 size=192
    let mut pc: u32 = 0x831AA6F0;
    'dispatch: loop {
        match pc {
            0x831AA6F0 => {
    //   block [0x831AA6F0..0x831AA7B0)
	// 831AA6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AA6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831AA6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AA6FC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 831AA700: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 831AA704: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 831AA708: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 831AA70C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 831AA710: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AA714: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831AA718: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 831AA71C: 409AFFF4  bne cr6, 0x831aa710
	if !ctx.cr[6].eq {
	pc = 0x831AA710; continue 'dispatch;
	}
	// 831AA720: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831AA724: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831AA728: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 831AA72C: 5549003E  slwi r9, r10, 0
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831AA730: 409A0030  bne cr6, 0x831aa760
	if !ctx.cr[6].eq {
	pc = 0x831AA760; continue 'dispatch;
	}
	// 831AA734: 4800662D  bl 0x831b0d60
	ctx.lr = 0x831AA738;
	sub_831B0D60(ctx, base);
	// 831AA738: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 831AA73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831AA740: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831AA744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831AA748: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831AA74C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831AA750: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AA754: 4800A6BD  bl 0x831b4e10
	ctx.lr = 0x831AA758;
	sub_831B4E10(ctx, base);
	// 831AA758: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831AA75C: 48000044  b 0x831aa7a0
	pc = 0x831AA7A0; continue 'dispatch;
	// 831AA760: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831AA764: 419AFFD0  beq cr6, 0x831aa734
	if ctx.cr[6].eq {
	pc = 0x831AA734; continue 'dispatch;
	}
	// 831AA768: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 831AA76C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 831AA770: 38C00049  li r6, 0x49
	ctx.r[6].s64 = 73;
	// 831AA774: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 831AA778: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 831AA77C: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 831AA780: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831AA784: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 831AA788: 41990008  bgt cr6, 0x831aa790
	if ctx.cr[6].gt {
	pc = 0x831AA790; continue 'dispatch;
	}
	// 831AA78C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 831AA790: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 831AA794: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831AA798: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 831AA79C: 4E800421  bctrl
	ctx.lr = 0x831AA7A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831AA7A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831AA7A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831AA7A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831AA7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AA7B0 size=92
    let mut pc: u32 = 0x831AA7B0;
    'dispatch: loop {
        match pc {
            0x831AA7B0 => {
    //   block [0x831AA7B0..0x831AA80C)
	// 831AA7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AA7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831AA7B8: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 831AA7BC: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 831AA7C0: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 831AA7C4: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 831AA7C8: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 831AA7CC: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 831AA7D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AA7D4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 831AA7D8: 3D40831B  lis r10, -0x7ce5
	ctx.r[10].s64 = -2095382528;
	// 831AA7DC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 831AA7E0: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 831AA7E4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 831AA7E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831AA7EC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831AA7F0: 386A6F98  addi r3, r10, 0x6f98
	ctx.r[3].s64 = ctx.r[10].s64 + 28568;
	// 831AA7F4: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831AA7F8: 4BFFFEF9  bl 0x831aa6f0
	ctx.lr = 0x831AA7FC;
	sub_831AA6F0(ctx, base);
	// 831AA7FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831AA800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831AA804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831AA808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA810 size=36
    let mut pc: u32 = 0x831AA810;
    'dispatch: loop {
        match pc {
            0x831AA810 => {
    //   block [0x831AA810..0x831AA834)
	// 831AA810: D8210010  stfd f1, 0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.f[1].u64 ) };
	// 831AA814: A1610010  lhz r11, 0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 831AA818: 556B0476  rlwinm r11, r11, 0, 0x11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AA81C: D8410018  stfd f2, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[2].u64 ) };
	// 831AA820: 2B0B7FF0  cmplwi cr6, r11, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32752 as u32, &mut ctx.xer);
	// 831AA824: 409A0010  bne cr6, 0x831aa834
	if !ctx.cr[6].eq {
		sub_831AA834(ctx, base);
		return;
	}
	// 831AA828: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AA82C: C82BF470  lfd f1, -0xb90(r11)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-2960 as u32) ) };
	// 831AA830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA834(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA834 size=44
    let mut pc: u32 = 0x831AA834;
    'dispatch: loop {
        match pc {
            0x831AA834 => {
    //   block [0x831AA834..0x831AA860)
	// 831AA834: A1610018  lhz r11, 0x18(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) } as u64;
	// 831AA838: 556B0476  rlwinm r11, r11, 0, 0x11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AA83C: 2B0B7FF0  cmplwi cr6, r11, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32752 as u32, &mut ctx.xer);
	// 831AA840: 409A0038  bne cr6, 0x831aa878
	if !ctx.cr[6].eq {
		sub_831AA878(ctx, base);
		return;
	}
	// 831AA844: 81610018  lwz r11, 0x18(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) } as u64;
	// 831AA848: 3D207FF0  lis r9, 0x7ff0
	ctx.r[9].s64 = 2146435072;
	// 831AA84C: 8141001C  lwz r10, 0x1c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 831AA850: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 831AA854: 409A000C  bne cr6, 0x831aa860
	if !ctx.cr[6].eq {
		sub_831AA860(ctx, base);
		return;
	}
	// 831AA858: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831AA85C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA860 size=20
    let mut pc: u32 = 0x831AA860;
    'dispatch: loop {
        match pc {
            0x831AA860 => {
    //   block [0x831AA860..0x831AA874)
	// 831AA860: 3D20FFF0  lis r9, -0x10
	ctx.r[9].s64 = -1048576;
	// 831AA864: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 831AA868: 409AFFC0  bne cr6, 0x831aa828
	if !ctx.cr[6].eq {
		sub_831AA810(ctx, base);
		return;
	}
	// 831AA86C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831AA870: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA874(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA874 size=4
    let mut pc: u32 = 0x831AA874;
    'dispatch: loop {
        match pc {
            0x831AA874 => {
    //   block [0x831AA874..0x831AA878)
	// 831AA874: 4BFFFFB4  b 0x831aa828
	sub_831AA810(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA878 size=24
    let mut pc: u32 = 0x831AA878;
    'dispatch: loop {
        match pc {
            0x831AA878 => {
    //   block [0x831AA878..0x831AA890)
	// 831AA878: E9410018  ld r10, 0x18(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 831AA87C: E9210010  ld r9, 0x10(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) };
	// 831AA880: 794B0040  clrldi r11, r10, 1
	ctx.r[11].u64 = ctx.r[10].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 831AA884: 79280040  clrldi r8, r9, 1
	ctx.r[8].u64 = ctx.r[9].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 831AA888: 7F285840  cmpld cr6, r8, r11
	ctx.cr[6].compare_u64(ctx.r[8].u64, ctx.r[11].u64, &mut ctx.xer);
	// 831AA88C: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA890 size=68
    let mut pc: u32 = 0x831AA890;
    'dispatch: loop {
        match pc {
            0x831AA890 => {
    //   block [0x831AA890..0x831AA8D4)
	// 831AA890: 79486560  rldicl r8, r10, 0xc, 0x35
	ctx.r[8].u64 = ctx.r[10].u64 & 0x000FFFFFFFFFFFFFu64;
	// 831AA894: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831AA898: 79250004  rldicr r5, r9, 0, 0
	ctx.r[5].u64 = (ctx.r[9].u64).rotate_left(0) & 0x8000000000000000;
	// 831AA89C: 794A5840  rldicl r10, r10, 0xb, 1
	ctx.r[10].u64 = ctx.r[10].u64 & 0x001FFFFFFFFFFFFFu64;
	// 831AA8A0: 7967FFE6  rldicr r7, r11, 0x3f, 0x3f
	ctx.r[7].u64 = (ctx.r[11].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 831AA8A4: F8A1FFF0  std r5, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[5].u64 ) };
	// 831AA8A8: 792B6560  rldicl r11, r9, 0xc, 0x35
	ctx.r[11].u64 = ctx.r[9].u64 & 0x000FFFFFFFFFFFFFu64;
	// 831AA8AC: 794A0524  rldicr r10, r10, 0, 0x34
	ctx.r[10].u64 = (ctx.r[10].u64).rotate_left(0) & 0xFFFFFFFFFFFFF800;
	// 831AA8B0: 2F280000  cmpdi cr6, r8, 0
	ctx.cr[6].compare_i64(ctx.r[8].s64, 0, &mut ctx.xer);
	// 831AA8B4: 409A0020  bne cr6, 0x831aa8d4
	if !ctx.cr[6].eq {
		sub_831AA8D4(ctx, base);
		return;
	}
	// 831AA8B8: 2B2A0000  cmpldi cr6, r10, 0
	ctx.cr[6].compare_u64(ctx.r[10].u64, 0, &mut ctx.xer);
	// 831AA8BC: 419AFF6C  beq cr6, 0x831aa828
	if ctx.cr[6].eq {
		sub_831AA810(ctx, base);
		return;
	}
	// 831AA8C0: 7D480074  cntlzd r8, r10
	ctx.r[8].u64 = if ctx.r[10].u64 == 0 { 64 } else { ctx.r[10].u64.leading_zeros() as u64 };
	// 831AA8C4: 7D0807B4  extsw r8, r8
	ctx.r[8].s64 = ctx.r[8].s32 as i64;
	// 831AA8C8: 7D464036  sld r6, r10, r8
	if (ctx.r[8].u8 & 0x40) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = (ctx.r[10].u64) << ((ctx.r[8].u8 & 0x3F) as u32);
	}
	// 831AA8CC: 21080001  subfic r8, r8, 1
	ctx.xer.ca = ctx.r[8].u32 <= 1 as u32;
	ctx.r[8].s64 = (1 as i64) - ctx.r[8].s64;
	// 831AA8D0: 48000008  b 0x831aa8d8
	sub_831AA8D4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA8D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA8D4 size=28
    let mut pc: u32 = 0x831AA8D4;
    'dispatch: loop {
        match pc {
            0x831AA8D4 => {
    //   block [0x831AA8D4..0x831AA8F0)
	// 831AA8D4: 7D463B78  or r6, r10, r7
	ctx.r[6].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 831AA8D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831AA8DC: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 831AA8E0: 792A584C  rldimi r10, r9, 0xb, 1
	ctx.r[10].u64 = ((ctx.r[9].u64).rotate_left(11) & 0x7FFFFFFFFFFFF800) | (ctx.r[10].u64 & 0x80000000000007FF);
	// 831AA8E4: 409A0020  bne cr6, 0x831aa904
	if !ctx.cr[6].eq {
		sub_831AA904(ctx, base);
		return;
	}
	// 831AA8E8: 2B2A0000  cmpldi cr6, r10, 0
	ctx.cr[6].compare_u64(ctx.r[10].u64, 0, &mut ctx.xer);
	// 831AA8EC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA8F0 size=20
    let mut pc: u32 = 0x831AA8F0;
    'dispatch: loop {
        match pc {
            0x831AA8F0 => {
    //   block [0x831AA8F0..0x831AA904)
	// 831AA8F0: 7D4B0074  cntlzd r11, r10
	ctx.r[11].u64 = if ctx.r[10].u64 == 0 { 64 } else { ctx.r[10].u64.leading_zeros() as u64 };
	// 831AA8F4: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 831AA8F8: 7D4A5836  sld r10, r10, r11
	if (ctx.r[11].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[10].u64) << ((ctx.r[11].u8 & 0x3F) as u32);
	}
	// 831AA8FC: 216B0001  subfic r11, r11, 1
	ctx.xer.ca = ctx.r[11].u32 <= 1 as u32;
	ctx.r[11].s64 = (1 as i64) - ctx.r[11].s64;
	// 831AA900: 48000008  b 0x831aa908
	sub_831AA904(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA904(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA904 size=12
    let mut pc: u32 = 0x831AA904;
    'dispatch: loop {
        match pc {
            0x831AA904 => {
    //   block [0x831AA904..0x831AA910)
	// 831AA904: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 831AA908: 78C7F842  rldicl r7, r6, 0x3f, 1
	ctx.r[7].u64 = ctx.r[6].u64 & 0x0000000000000001u64;
	// 831AA90C: 48000028  b 0x831aa934
	sub_831AA910(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA910 size=132
    let mut pc: u32 = 0x831AA910;
    'dispatch: loop {
        match pc {
            0x831AA910 => {
    //   block [0x831AA910..0x831AA994)
	// 831AA910: 7F2A3040  cmpld cr6, r10, r6
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[6].u64, &mut ctx.xer);
	// 831AA914: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 831AA918: 40980008  bge cr6, 0x831aa920
	if !ctx.cr[6].lt {
	pc = 0x831AA920; continue 'dispatch;
	}
	// 831AA91C: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 831AA920: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 831AA924: 7D490074  cntlzd r9, r10
	ctx.r[9].u64 = if ctx.r[10].u64 == 0 { 64 } else { ctx.r[10].u64.leading_zeros() as u64 };
	// 831AA928: 7D2907B4  extsw r9, r9
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 831AA92C: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 831AA930: 7D4A4836  sld r10, r10, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[10].u64) << ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 831AA934: 7F2B4000  cmpd cr6, r11, r8
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[8].s64, &mut ctx.xer);
	// 831AA938: 4199FFD8  bgt cr6, 0x831aa910
	if ctx.cr[6].gt {
	pc = 0x831AA910; continue 'dispatch;
	}
	// 831AA93C: 409A0020  bne cr6, 0x831aa95c
	if !ctx.cr[6].eq {
	pc = 0x831AA95C; continue 'dispatch;
	}
	// 831AA940: 7F2A3040  cmpld cr6, r10, r6
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[6].u64, &mut ctx.xer);
	// 831AA944: 41980018  blt cr6, 0x831aa95c
	if ctx.cr[6].lt {
	pc = 0x831AA95C; continue 'dispatch;
	}
	// 831AA948: 7D465050  subf r10, r6, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[6].s64;
	// 831AA94C: 7D490074  cntlzd r9, r10
	ctx.r[9].u64 = if ctx.r[10].u64 == 0 { 64 } else { ctx.r[10].u64.leading_zeros() as u64 };
	// 831AA950: 7D2907B4  extsw r9, r9
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 831AA954: 7D4A4836  sld r10, r10, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[10].u64) << ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 831AA958: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 831AA95C: 2B2A0000  cmpldi cr6, r10, 0
	ctx.cr[6].compare_u64(ctx.r[10].u64, 0, &mut ctx.xer);
	// 831AA960: 419A002C  beq cr6, 0x831aa98c
	if ctx.cr[6].eq {
	pc = 0x831AA98C; continue 'dispatch;
	}
	// 831AA964: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 831AA968: 41990010  bgt cr6, 0x831aa978
	if ctx.cr[6].gt {
	pc = 0x831AA978; continue 'dispatch;
	}
	// 831AA96C: 212B0001  subfic r9, r11, 1
	ctx.xer.ca = ctx.r[11].u32 <= 1 as u32;
	ctx.r[9].s64 = (1 as i64) - ctx.r[11].s64;
	// 831AA970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831AA974: 7D4A4C36  srd r10, r10, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[10].u64) >> ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 831AA978: 796BA2C6  sldi r11, r11, 0x34
	ctx.r[11].u64 = ctx.r[11].u64.wrapping_shl(52);
	ctx.r[11].u32 = ctx.r[11].u64 as u32;
	// 831AA97C: 794AAB02  rldicl r10, r10, 0x35, 0xc
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000000007FFu64;
	// 831AA980: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831AA984: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 831AA988: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 831AA98C: C821FFF0  lfd f1, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831AA990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AA9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AA9A0 size=664
    let mut pc: u32 = 0x831AA9A0;
    'dispatch: loop {
        match pc {
            0x831AA9A0 => {
    //   block [0x831AA9A0..0x831AAC38)
	// 831AA9A0: 3960FEE0  li r11, -0x120
	ctx.r[11].s64 = -288;
	// 831AA9A4: 7DCB61CE  stvx v14, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[14].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[14].u8[first + i]); }
	}
	// 831AA9A8: 3960FEF0  li r11, -0x110
	ctx.r[11].s64 = -272;
	// 831AA9AC: 7DEB61CE  stvx v15, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[15].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[15].u8[first + i]); }
	}
	// 831AA9B0: 3960FF00  li r11, -0x100
	ctx.r[11].s64 = -256;
	// 831AA9B4: 7E0B61CE  stvx v16, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[16].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[16].u8[first + i]); }
	}
	// 831AA9B8: 3960FF10  li r11, -0xf0
	ctx.r[11].s64 = -240;
	// 831AA9BC: 7E2B61CE  stvx v17, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[17].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[17].u8[first + i]); }
	}
	// 831AA9C0: 3960FF20  li r11, -0xe0
	ctx.r[11].s64 = -224;
	// 831AA9C4: 7E4B61CE  stvx v18, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[18].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[18].u8[first + i]); }
	}
	// 831AA9C8: 3960FF30  li r11, -0xd0
	ctx.r[11].s64 = -208;
	// 831AA9CC: 7E6B61CE  stvx v19, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[19].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[19].u8[first + i]); }
	}
	// 831AA9D0: 3960FF40  li r11, -0xc0
	ctx.r[11].s64 = -192;
	// 831AA9D4: 7E8B61CE  stvx v20, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[20].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[20].u8[first + i]); }
	}
	// 831AA9D8: 3960FF50  li r11, -0xb0
	ctx.r[11].s64 = -176;
	// 831AA9DC: 7EAB61CE  stvx v21, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[21].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[21].u8[first + i]); }
	}
	// 831AA9E0: 3960FF60  li r11, -0xa0
	ctx.r[11].s64 = -160;
	// 831AA9E4: 7ECB61CE  stvx v22, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[22].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[22].u8[first + i]); }
	}
	// 831AA9E8: 3960FF70  li r11, -0x90
	ctx.r[11].s64 = -144;
	// 831AA9EC: 7EEB61CE  stvx v23, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[23].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[23].u8[first + i]); }
	}
	// 831AA9F0: 3960FF80  li r11, -0x80
	ctx.r[11].s64 = -128;
	// 831AA9F4: 7F0B61CE  stvx v24, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[24].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[24].u8[first + i]); }
	}
	// 831AA9F8: 3960FF90  li r11, -0x70
	ctx.r[11].s64 = -112;
	// 831AA9FC: 7F2B61CE  stvx v25, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[25].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[25].u8[first + i]); }
	}
	// 831AAA00: 3960FFA0  li r11, -0x60
	ctx.r[11].s64 = -96;
	// 831AAA04: 7F4B61CE  stvx v26, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[26].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[26].u8[first + i]); }
	}
	// 831AAA08: 3960FFB0  li r11, -0x50
	ctx.r[11].s64 = -80;
	// 831AAA0C: 7F6B61CE  stvx v27, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[27].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[27].u8[first + i]); }
	}
	// 831AAA10: 3960FFC0  li r11, -0x40
	ctx.r[11].s64 = -64;
	// 831AAA14: 7F8B61CE  stvx v28, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[28].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[28].u8[first + i]); }
	}
	// 831AAA18: 3960FFD0  li r11, -0x30
	ctx.r[11].s64 = -48;
	// 831AAA1C: 7FAB61CE  stvx v29, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[29].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[29].u8[first + i]); }
	}
	// 831AAA20: 3960FFE0  li r11, -0x20
	ctx.r[11].s64 = -32;
	// 831AAA24: 7FCB61CE  stvx v30, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[30].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[30].u8[first + i]); }
	}
	// 831AAA28: 3960FFF0  li r11, -0x10
	ctx.r[11].s64 = -16;
	// 831AAA2C: 7FEB61CE  stvx v31, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[31].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[31].u8[first + i]); }
	}
	// 831AAA30: 4E800020  blr
	return;
	// 831AAA34: 3960FC00  li r11, -0x400
	ctx.r[11].s64 = -1024;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AAC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831AAC38 size=664
    let mut pc: u32 = 0x831AAC38;
    'dispatch: loop {
        match pc {
            0x831AAC38 => {
    //   block [0x831AAC38..0x831AAED0)
	// 831AAC38: 3960FEE0  li r11, -0x120
	ctx.r[11].s64 = -288;
	// 831AAC3C: 7DCB60CE  lvx v14, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[14] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831AAC40: 3960FEF0  li r11, -0x110
	ctx.r[11].s64 = -272;
	// 831AAC44: 7DEB60CE  lvx v15, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[15] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831AAC48: 3960FF00  li r11, -0x100
	ctx.r[11].s64 = -256;
	// 831AAC4C: 7E0B60CE  lvx v16, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[16] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831AAC50: 3960FF10  li r11, -0xf0
	ctx.r[11].s64 = -240;
	// 831AAC54: 7E2B60CE  lvx v17, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[17] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831AAC58: 3960FF20  li r11, -0xe0
	ctx.r[11].s64 = -224;
	// 831AAC5C: 7E4B60CE  lvx v18, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[18] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831AAC60: 3960FF30  li r11, -0xd0
	ctx.r[11].s64 = -208;
	// 831AAC64: 7E6B60CE  lvx v19, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[19] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831AAC68: 3960FF40  li r11, -0xc0
	ctx.r[11].s64 = -192;
	// 831AAC6C: 7E8B60CE  lvx v20, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[20] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831AAC70: 3960FF50  li r11, -0xb0
	ctx.r[11].s64 = -176;
	// 831AAC74: 7EAB60CE  lvx v21, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[21] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831AAC78: 3960FF60  li r11, -0xa0
	ctx.r[11].s64 = -160;
	// 831AAC7C: 7ECB60CE  lvx v22, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[22] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831AAC80: 3960FF70  li r11, -0x90
	ctx.r[11].s64 = -144;
	// 831AAC84: 7EEB60CE  lvx v23, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[23] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831AAC88: 3960FF80  li r11, -0x80
	ctx.r[11].s64 = -128;
	// 831AAC8C: 7F0B60CE  lvx v24, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[24] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831AAC90: 3960FF90  li r11, -0x70
	ctx.r[11].s64 = -112;
	// 831AAC94: 7F2B60CE  lvx v25, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[25] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831AAC98: 3960FFA0  li r11, -0x60
	ctx.r[11].s64 = -96;
	// 831AAC9C: 7F4B60CE  lvx v26, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[26] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831AACA0: 3960FFB0  li r11, -0x50
	ctx.r[11].s64 = -80;
	// 831AACA4: 7F6B60CE  lvx v27, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[27] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831AACA8: 3960FFC0  li r11, -0x40
	ctx.r[11].s64 = -64;
	// 831AACAC: 7F8B60CE  lvx v28, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[28] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831AACB0: 3960FFD0  li r11, -0x30
	ctx.r[11].s64 = -48;
	// 831AACB4: 7FAB60CE  lvx v29, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[29] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831AACB8: 3960FFE0  li r11, -0x20
	ctx.r[11].s64 = -32;
	// 831AACBC: 7FCB60CE  lvx v30, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[30] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831AACC0: 3960FFF0  li r11, -0x10
	ctx.r[11].s64 = -16;
	// 831AACC4: 7FEB60CE  lvx v31, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[31] using VectorMaskL[(tmp.u32 & 0xF)]
	// 831AACC8: 4E800020  blr
	return;
	// 831AACCC: 3960FC00  li r11, -0x400
	ctx.r[11].s64 = -1024;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AAED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831AAED0 size=48
    let mut pc: u32 = 0x831AAED0;
    'dispatch: loop {
        match pc {
            0x831AAED0 => {
    //   block [0x831AAED0..0x831AAF00)
	// 831AAED0: FC000A10  fabs f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 831AAED4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AAED8: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 831AAEDC: 396BED60  addi r11, r11, -0x12a0
	ctx.r[11].s64 = ctx.r[11].s64 + -4768;
	// 831AAEE0: C1AB00B0  lfs f13, 0xb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831AAEE4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 831AAEE8: 40990024  ble cr6, 0x831aaf0c
	if !ctx.cr[6].gt {
		sub_831AAF0C(ctx, base);
		return;
	}
	// 831AAEEC: C98B0030  lfd f12, 0x30(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 831AAEF0: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 831AAEF4: 4099000C  ble cr6, 0x831aaf00
	if !ctx.cr[6].gt {
		sub_831AAF00(ctx, base);
		return;
	}
	// 831AAEF8: C80B0008  lfd f0, 8(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 831AAEFC: 480000A0  b 0x831aaf9c
	sub_831AAF0C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AAF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AAF00 size=12
    let mut pc: u32 = 0x831AAF00;
    'dispatch: loop {
        match pc {
            0x831AAF00 => {
    //   block [0x831AAF00..0x831AAF0C)
	// 831AAF00: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 831AAF04: FC0D0024  fdiv f0, f13, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[13].f64 / ctx.f[0].f64;
	// 831AAF08: 48000008  b 0x831aaf10
	sub_831AAF0C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AAF0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AAF0C size=156
    let mut pc: u32 = 0x831AAF0C;
    'dispatch: loop {
        match pc {
            0x831AAF0C => {
    //   block [0x831AAF0C..0x831AAFA8)
	// 831AAF0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831AAF10: C98B0018  lfd f12, 0x18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 831AAF14: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 831AAF18: 40990018  ble cr6, 0x831aaf30
	if !ctx.cr[6].gt {
	pc = 0x831AAF30; continue 'dispatch;
	}
	// 831AAF1C: C98B0028  lfd f12, 0x28(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 831AAF20: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831AAF24: FD6C002A  fadd f11, f12, f0
	ctx.f[11].f64 = ctx.f[12].f64 + ctx.f[0].f64;
	// 831AAF28: FC0C6838  fmsub f0, f12, f0, f13
	ctx.f[0].f64 = ctx.f[12].f64 * ctx.f[0].f64 - ctx.f[13].f64;
	// 831AAF2C: FC005824  fdiv f0, f0, f11
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[11].f64;
	// 831AAF30: FCA00032  fmul f5, f0, f0
	ctx.f[5].f64 = ctx.f[0].f64 * ctx.f[0].f64;
	// 831AAF34: C98B0048  lfd f12, 0x48(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) };
	// 831AAF38: C9AB0050  lfd f13, 0x50(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 831AAF3C: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 831AAF40: C96B0070  lfd f11, 0x70(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) };
	// 831AAF44: C94B0040  lfd f10, 0x40(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 831AAF48: C92B0068  lfd f9, 0x68(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) };
	// 831AAF4C: C90B0038  lfd f8, 0x38(r11)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 831AAF50: C8EB0060  lfd f7, 0x60(r11)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	// 831AAF54: C8CB0058  lfd f6, 0x58(r11)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	// 831AAF58: FDAD617A  fmadd f13, f13, f5, f12
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[5].f64 + ctx.f[12].f64;
	// 831AAF5C: FD8B282A  fadd f12, f11, f5
	ctx.f[12].f64 = ctx.f[11].f64 + ctx.f[5].f64;
	// 831AAF60: FDAD517A  fmadd f13, f13, f5, f10
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[5].f64 + ctx.f[10].f64;
	// 831AAF64: FD8C497A  fmadd f12, f12, f5, f9
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[5].f64 + ctx.f[9].f64;
	// 831AAF68: FDAD417A  fmadd f13, f13, f5, f8
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[5].f64 + ctx.f[8].f64;
	// 831AAF6C: FD8C397A  fmadd f12, f12, f5, f7
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[5].f64 + ctx.f[7].f64;
	// 831AAF70: FDAD0172  fmul f13, f13, f5
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[5].f64;
	// 831AAF74: FD8C317A  fmadd f12, f12, f5, f6
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[5].f64 + ctx.f[6].f64;
	// 831AAF78: FDAD0032  fmul f13, f13, f0
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 831AAF7C: FDAD6024  fdiv f13, f13, f12
	ctx.f[13].f64 = ctx.f[13].f64 / ctx.f[12].f64;
	// 831AAF80: FC0D002A  fadd f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	// 831AAF84: 40990008  ble cr6, 0x831aaf8c
	if !ctx.cr[6].gt {
	pc = 0x831AAF8C; continue 'dispatch;
	}
	// 831AAF88: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 831AAF8C: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831AAF90: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	// 831AAF94: 7DAA5CAE  lfdx f13, r10, r11
	ctx.f[13].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) };
	// 831AAF98: FC0D002A  fadd f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	// 831AAF9C: E961FFF0  ld r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831AAFA0: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 831AAFA4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AAFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AAFA8 size=12
    let mut pc: u32 = 0x831AAFA8;
    'dispatch: loop {
        match pc {
            0x831AAFA8 => {
    //   block [0x831AAFA8..0x831AAFB4)
	// 831AAFA8: FDA00050  fneg f13, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 831AAFAC: FC21682E  fsel f1, f1, f0, f13
	ctx.f[1].f64 = if ctx.f[1].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 831AAFB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AAFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831AAFB8 size=48
    let mut pc: u32 = 0x831AAFB8;
    'dispatch: loop {
        match pc {
            0x831AAFB8 => {
    //   block [0x831AAFB8..0x831AAFE8)
	// 831AAFB8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AAFBC: D8210010  stfd f1, 0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.f[1].u64 ) };
	// 831AAFC0: D8410018  stfd f2, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[2].u64 ) };
	// 831AAFC4: 396BED60  addi r11, r11, -0x12a0
	ctx.r[11].s64 = ctx.r[11].s64 + -4768;
	// 831AAFC8: C00B00A8  lfs f0, 0xa8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831AAFCC: FF020000  fcmpu cr6, f2, f0
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[0].f64);
	// 831AAFD0: 409A0040  bne cr6, 0x831ab010
	if !ctx.cr[6].eq {
		sub_831AB010(ctx, base);
		return;
	}
	// 831AAFD4: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 831AAFD8: 409A0030  bne cr6, 0x831ab008
	if !ctx.cr[6].eq {
		sub_831AB008(ctx, base);
		return;
	}
	// 831AAFDC: 81410018  lwz r10, 0x18(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) } as u64;
	// 831AAFE0: 554A0001  rlwinm. r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AAFE4: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AAFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AAFE8 size=24
    let mut pc: u32 = 0x831AAFE8;
    'dispatch: loop {
        match pc {
            0x831AAFE8 => {
    //   block [0x831AAFE8..0x831AB000)
	// 831AAFE8: 81410010  lwz r10, 0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 831AAFEC: 554A0001  rlwinm. r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AAFF0: 41820010  beq 0x831ab000
	if ctx.cr[0].eq {
		sub_831AB000(ctx, base);
		return;
	}
	// 831AAFF4: C80B0010  lfd f0, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 831AAFF8: FC200050  fneg f1, f0
	ctx.f[1].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 831AAFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AB000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AB000 size=8
    let mut pc: u32 = 0x831AB000;
    'dispatch: loop {
        match pc {
            0x831AB000 => {
    //   block [0x831AB000..0x831AB008)
	// 831AB000: C82B0010  lfd f1, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 831AB004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AB008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AB008 size=8
    let mut pc: u32 = 0x831AB008;
    'dispatch: loop {
        match pc {
            0x831AB008 => {
    //   block [0x831AB008..0x831AB010)
	// 831AB008: C80B0008  lfd f0, 8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 831AB00C: 480000BC  b 0x831ab0c8
	sub_831AB010(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AB010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831AB010 size=208
    let mut pc: u32 = 0x831AB010;
    'dispatch: loop {
        match pc {
            0x831AB010 => {
    //   block [0x831AB010..0x831AB0E0)
	// 831AB010: FDA01210  fabs f13, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = ctx.f[2].u64 & !0x8000_0000_0000_0000u64;
	// 831AB014: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831AB018: FC000A10  fabs f0, f1
	ctx.f[0].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 831AB01C: FD806890  fmr f12, f13
	ctx.f[12].f64 = ctx.f[13].f64;
	// 831AB020: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 831AB024: 40990010  ble cr6, 0x831ab034
	if !ctx.cr[6].gt {
	pc = 0x831AB034; continue 'dispatch;
	}
	// 831AB028: FD800090  fmr f12, f0
	ctx.f[12].f64 = ctx.f[0].f64;
	// 831AB02C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 831AB030: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 831AB034: FC006024  fdiv f0, f0, f12
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[12].f64;
	// 831AB038: C9AB0018  lfd f13, 0x18(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 831AB03C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 831AB040: 4099001C  ble cr6, 0x831ab05c
	if !ctx.cr[6].gt {
	pc = 0x831AB05C; continue 'dispatch;
	}
	// 831AB044: C9AB0028  lfd f13, 0x28(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 831AB048: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831AB04C: C18B00B0  lfs f12, 0xb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831AB050: FD6D002A  fadd f11, f13, f0
	ctx.f[11].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	// 831AB054: FC0D6038  fmsub f0, f13, f0, f12
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[12].f64;
	// 831AB058: FC005824  fdiv f0, f0, f11
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[11].f64;
	// 831AB05C: FCA00032  fmul f5, f0, f0
	ctx.f[5].f64 = ctx.f[0].f64 * ctx.f[0].f64;
	// 831AB060: C98B0048  lfd f12, 0x48(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) };
	// 831AB064: C9AB0050  lfd f13, 0x50(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 831AB068: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 831AB06C: C96B0070  lfd f11, 0x70(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) };
	// 831AB070: C94B0040  lfd f10, 0x40(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 831AB074: C92B0068  lfd f9, 0x68(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) };
	// 831AB078: C90B0038  lfd f8, 0x38(r11)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 831AB07C: C8EB0060  lfd f7, 0x60(r11)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	// 831AB080: C8CB0058  lfd f6, 0x58(r11)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	// 831AB084: FDAD617A  fmadd f13, f13, f5, f12
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[5].f64 + ctx.f[12].f64;
	// 831AB088: FD8B282A  fadd f12, f11, f5
	ctx.f[12].f64 = ctx.f[11].f64 + ctx.f[5].f64;
	// 831AB08C: FDAD517A  fmadd f13, f13, f5, f10
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[5].f64 + ctx.f[10].f64;
	// 831AB090: FD8C497A  fmadd f12, f12, f5, f9
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[5].f64 + ctx.f[9].f64;
	// 831AB094: FDAD417A  fmadd f13, f13, f5, f8
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[5].f64 + ctx.f[8].f64;
	// 831AB098: FD8C397A  fmadd f12, f12, f5, f7
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[5].f64 + ctx.f[7].f64;
	// 831AB09C: FDAD0172  fmul f13, f13, f5
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[5].f64;
	// 831AB0A0: FD8C317A  fmadd f12, f12, f5, f6
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[5].f64 + ctx.f[6].f64;
	// 831AB0A4: FDAD0032  fmul f13, f13, f0
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 831AB0A8: FDAD6024  fdiv f13, f13, f12
	ctx.f[13].f64 = ctx.f[13].f64 / ctx.f[12].f64;
	// 831AB0AC: FC0D002A  fadd f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	// 831AB0B0: 40990008  ble cr6, 0x831ab0b8
	if !ctx.cr[6].gt {
	pc = 0x831AB0B8; continue 'dispatch;
	}
	// 831AB0B4: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 831AB0B8: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831AB0BC: 392B0080  addi r9, r11, 0x80
	ctx.r[9].s64 = ctx.r[11].s64 + 128;
	// 831AB0C0: 7DAA4CAE  lfdx f13, r10, r9
	ctx.f[13].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 831AB0C4: FC0D002A  fadd f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	// 831AB0C8: C9AB0010  lfd f13, 0x10(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 831AB0CC: 81610010  lwz r11, 0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 831AB0D0: FDAD0028  fsub f13, f13, f0
	ctx.f[13].f64 = ctx.f[13].f64 - ctx.f[0].f64;
	// 831AB0D4: 556B0001  rlwinm. r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AB0D8: FC22682E  fsel f1, f2, f0, f13
	ctx.f[1].f64 = if ctx.f[2].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 831AB0DC: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AB0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AB0E0 size=8
    let mut pc: u32 = 0x831AB0E0;
    'dispatch: loop {
        match pc {
            0x831AB0E0 => {
    //   block [0x831AB0E0..0x831AB0E8)
	// 831AB0E0: FC200850  fneg f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 831AB0E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AB0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831AB0E8 size=60
    let mut pc: u32 = 0x831AB0E8;
    'dispatch: loop {
        match pc {
            0x831AB0E8 => {
    //   block [0x831AB0E8..0x831AB124)
	// 831AB0E8: FDA00A10  fabs f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 831AB0EC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AB0F0: 396BEE18  addi r11, r11, -0x11e8
	ctx.r[11].s64 = ctx.r[11].s64 + -4584;
	// 831AB0F4: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831AB0F8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831AB0FC: 40990028  ble cr6, 0x831ab124
	if !ctx.cr[6].gt {
		sub_831AB124(ctx, base);
		return;
	}
	// 831AB100: C18B0004  lfs f12, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831AB104: 21440001  subfic r10, r4, 1
	ctx.xer.ca = ctx.r[4].u32 <= 1 as u32;
	ctx.r[10].s64 = (1 as i64) - ctx.r[4].s64;
	// 831AB108: FD8C6828  fsub f12, f12, f13
	ctx.f[12].f64 = ctx.f[12].f64 - ctx.f[13].f64;
	// 831AB10C: C1AB0008  lfs f13, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831AB110: FC0C0032  fmul f0, f12, f0
	ctx.f[0].f64 = ctx.f[12].f64 * ctx.f[0].f64;
	// 831AB114: FD80002C  fsqrt f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64).sqrt();
	// 831AB118: FDAC0372  fmul f13, f12, f13
	ctx.f[13].f64 = ctx.f[12].f64 * ctx.f[13].f64;
	// 831AB11C: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 831AB120: 48000020  b 0x831ab140
	sub_831AB140(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AB124(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831AB124 size=28
    let mut pc: u32 = 0x831AB124;
    'dispatch: loop {
        match pc {
            0x831AB124 => {
    //   block [0x831AB124..0x831AB140)
	// 831AB124: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 831AB128: FC0D0372  fmul f0, f13, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[13].f64;
	// 831AB12C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831AB130: 409A0010  bne cr6, 0x831ab140
	if !ctx.cr[6].eq {
		sub_831AB140(ctx, base);
		return;
	}
	// 831AB134: C18B0000  lfs f12, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831AB138: FF016000  fcmpu cr6, f1, f12
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[12].f64);
	// 831AB13C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AB140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AB140 size=128
    let mut pc: u32 = 0x831AB140;
    'dispatch: loop {
        match pc {
            0x831AB140 => {
    //   block [0x831AB140..0x831AB1C0)
	// 831AB140: C98B0050  lfd f12, 0x50(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 831AB144: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831AB148: C96B0048  lfd f11, 0x48(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) };
	// 831AB14C: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831AB150: FCAC583A  fmadd f5, f12, f0, f11
	ctx.f[5].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 831AB154: C98B0078  lfd f12, 0x78(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) };
	// 831AB158: C96B0040  lfd f11, 0x40(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 831AB15C: FC8C002A  fadd f4, f12, f0
	ctx.f[4].f64 = ctx.f[12].f64 + ctx.f[0].f64;
	// 831AB160: C98B0070  lfd f12, 0x70(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) };
	// 831AB164: C94B0038  lfd f10, 0x38(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 831AB168: C92B0068  lfd f9, 0x68(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) };
	// 831AB16C: C90B0030  lfd f8, 0x30(r11)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 831AB170: C8EB0060  lfd f7, 0x60(r11)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	// 831AB174: C8CB0058  lfd f6, 0x58(r11)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	// 831AB178: FD65583A  fmadd f11, f5, f0, f11
	ctx.f[11].f64 = ctx.f[5].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 831AB17C: FD84603A  fmadd f12, f4, f0, f12
	ctx.f[12].f64 = ctx.f[4].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 831AB180: FD6B503A  fmadd f11, f11, f0, f10
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[10].f64;
	// 831AB184: FD8C483A  fmadd f12, f12, f0, f9
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[9].f64;
	// 831AB188: FD6B403A  fmadd f11, f11, f0, f8
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[8].f64;
	// 831AB18C: FD8C383A  fmadd f12, f12, f0, f7
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[7].f64;
	// 831AB190: FD6B0032  fmul f11, f11, f0
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64;
	// 831AB194: FC0C303A  fmadd f0, f12, f0, f6
	ctx.f[0].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[6].f64;
	// 831AB198: FD8B0372  fmul f12, f11, f13
	ctx.f[12].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	// 831AB19C: FC0C0024  fdiv f0, f12, f0
	ctx.f[0].f64 = ctx.f[12].f64 / ctx.f[0].f64;
	// 831AB1A0: FC00682A  fadd f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 + ctx.f[13].f64;
	// 831AB1A4: 409A001C  bne cr6, 0x831ab1c0
	if !ctx.cr[6].eq {
		sub_831AB1C0(ctx, base);
		return;
	}
	// 831AB1A8: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 831AB1AC: 7DAA5CAE  lfdx f13, r10, r11
	ctx.f[13].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) };
	// 831AB1B0: FC0D002A  fadd f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	// 831AB1B4: FDA00050  fneg f13, f0
	ctx.f[13].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 831AB1B8: FC21682E  fsel f1, f1, f0, f13
	ctx.f[1].f64 = if ctx.f[1].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 831AB1BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AB1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AB1C0 size=32
    let mut pc: u32 = 0x831AB1C0;
    'dispatch: loop {
        match pc {
            0x831AB1C0 => {
    //   block [0x831AB1C0..0x831AB1E0)
	// 831AB1C0: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 831AB1C4: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 831AB1C8: 7DAA4CAE  lfdx f13, r10, r9
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 831AB1CC: 7D8A5CAE  lfdx f12, r10, r11
	ctx.f[12].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) };
	// 831AB1D0: FDAD0028  fsub f13, f13, f0
	ctx.f[13].f64 = ctx.f[13].f64 - ctx.f[0].f64;
	// 831AB1D4: FC0C002A  fadd f0, f12, f0
	ctx.f[0].f64 = ctx.f[12].f64 + ctx.f[0].f64;
	// 831AB1D8: FC21036E  fsel f1, f1, f13, f0
	ctx.f[1].f64 = if ctx.f[1].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 831AB1DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AB1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831AB1E0 size=60
    let mut pc: u32 = 0x831AB1E0;
    'dispatch: loop {
        match pc {
            0x831AB1E0 => {
    //   block [0x831AB1E0..0x831AB21C)
	// 831AB1E0: FDA00A10  fabs f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 831AB1E4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AB1E8: 396BEE18  addi r11, r11, -0x11e8
	ctx.r[11].s64 = ctx.r[11].s64 + -4584;
	// 831AB1EC: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831AB1F0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831AB1F4: 40990028  ble cr6, 0x831ab21c
	if !ctx.cr[6].gt {
		sub_831AB21C(ctx, base);
		return;
	}
	// 831AB1F8: C18B0004  lfs f12, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831AB1FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 831AB200: FD8C6828  fsub f12, f12, f13
	ctx.f[12].f64 = ctx.f[12].f64 - ctx.f[13].f64;
	// 831AB204: C1AB0008  lfs f13, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831AB208: FC0C0032  fmul f0, f12, f0
	ctx.f[0].f64 = ctx.f[12].f64 * ctx.f[0].f64;
	// 831AB20C: FD80002C  fsqrt f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64).sqrt();
	// 831AB210: FDAC0372  fmul f13, f12, f13
	ctx.f[13].f64 = ctx.f[12].f64 * ctx.f[13].f64;
	// 831AB214: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 831AB218: 48000018  b 0x831ab230
	sub_831AB230(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AB21C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831AB21C size=20
    let mut pc: u32 = 0x831AB21C;
    'dispatch: loop {
        match pc {
            0x831AB21C => {
    //   block [0x831AB21C..0x831AB230)
	// 831AB21C: C18B0000  lfs f12, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831AB220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831AB224: FC0D0372  fmul f0, f13, f13
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[13].f64;
	// 831AB228: FF016000  fcmpu cr6, f1, f12
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[12].f64);
	// 831AB22C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AB230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AB230 size=120
    let mut pc: u32 = 0x831AB230;
    'dispatch: loop {
        match pc {
            0x831AB230 => {
    //   block [0x831AB230..0x831AB2A8)
	// 831AB230: C98B0050  lfd f12, 0x50(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 831AB234: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831AB238: C96B0048  lfd f11, 0x48(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) };
	// 831AB23C: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 831AB240: FCAC583A  fmadd f5, f12, f0, f11
	ctx.f[5].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 831AB244: C98B0078  lfd f12, 0x78(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) };
	// 831AB248: C96B0040  lfd f11, 0x40(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 831AB24C: FC8C002A  fadd f4, f12, f0
	ctx.f[4].f64 = ctx.f[12].f64 + ctx.f[0].f64;
	// 831AB250: C98B0070  lfd f12, 0x70(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) };
	// 831AB254: C94B0038  lfd f10, 0x38(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 831AB258: C92B0068  lfd f9, 0x68(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) };
	// 831AB25C: C90B0030  lfd f8, 0x30(r11)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 831AB260: C8EB0060  lfd f7, 0x60(r11)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	// 831AB264: C8CB0058  lfd f6, 0x58(r11)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	// 831AB268: 7C6A4CAE  lfdx f3, r10, r9
	ctx.f[3].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 831AB26C: FD65583A  fmadd f11, f5, f0, f11
	ctx.f[11].f64 = ctx.f[5].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 831AB270: FD84603A  fmadd f12, f4, f0, f12
	ctx.f[12].f64 = ctx.f[4].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 831AB274: FD6B503A  fmadd f11, f11, f0, f10
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[10].f64;
	// 831AB278: FD8C483A  fmadd f12, f12, f0, f9
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[9].f64;
	// 831AB27C: FD6B403A  fmadd f11, f11, f0, f8
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[8].f64;
	// 831AB280: FD8C383A  fmadd f12, f12, f0, f7
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[7].f64;
	// 831AB284: FD6B0032  fmul f11, f11, f0
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64;
	// 831AB288: FC0C303A  fmadd f0, f12, f0, f6
	ctx.f[0].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[6].f64;
	// 831AB28C: FD8B0372  fmul f12, f11, f13
	ctx.f[12].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	// 831AB290: FC0C0024  fdiv f0, f12, f0
	ctx.f[0].f64 = ctx.f[12].f64 / ctx.f[0].f64;
	// 831AB294: FC00682A  fadd f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 + ctx.f[13].f64;
	// 831AB298: FC00182A  fadd f0, f0, f3
	ctx.f[0].f64 = ctx.f[0].f64 + ctx.f[3].f64;
	// 831AB29C: FDA00050  fneg f13, f0
	ctx.f[13].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 831AB2A0: FC21682E  fsel f1, f1, f0, f13
	ctx.f[1].f64 = if ctx.f[1].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 831AB2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AB2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AB2A8 size=8
    let mut pc: u32 = 0x831AB2A8;
    'dispatch: loop {
        match pc {
            0x831AB2A8 => {
    //   block [0x831AB2A8..0x831AB2B0)
	// 831AB2A8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831AB2AC: 4BFFFE3C  b 0x831ab0e8
	sub_831AB0E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AB2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AB2B0 size=92
    let mut pc: u32 = 0x831AB2B0;
    'dispatch: loop {
        match pc {
            0x831AB2B0 => {
    //   block [0x831AB2B0..0x831AB30C)
	// 831AB2B0: D8210010  stfd f1, 0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.f[1].u64 ) };
	// 831AB2B4: A1610010  lhz r11, 0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 831AB2B8: 556B0477  rlwinm. r11, r11, 0, 0x11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AB2BC: 4082001C  bne 0x831ab2d8
	if !ctx.cr[0].eq {
	pc = 0x831AB2D8; continue 'dispatch;
	}
	// 831AB2C0: 81610010  lwz r11, 0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 831AB2C4: 556B033F  clrlwi. r11, r11, 0xc
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AB2C8: 4082004C  bne 0x831ab314
	if !ctx.cr[0].eq {
		sub_831AB314(ctx, base);
		return;
	}
	// 831AB2CC: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 831AB2D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831AB2D4: 409A0040  bne cr6, 0x831ab314
	if !ctx.cr[6].eq {
		sub_831AB314(ctx, base);
		return;
	}
	// 831AB2D8: FC000E5C  fctid f0, f1
	ctx.f[0].s64 = if ctx.f[1].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[1].f64 as i64 };
	// 831AB2DC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 831AB2E0: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 831AB2E4: 409A0030  bne cr6, 0x831ab314
	if !ctx.cr[6].eq {
		sub_831AB314(ctx, base);
		return;
	}
	// 831AB2E8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 831AB2EC: C80BD760  lfd f0, -0x28a0(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-10400 as u32) ) };
	// 831AB2F0: FC010032  fmul f0, f1, f0
	ctx.f[0].f64 = ctx.f[1].f64 * ctx.f[0].f64;
	// 831AB2F4: FDA0065C  fctid f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64 as i64 };
	// 831AB2F8: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 831AB2FC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831AB300: 409A000C  bne cr6, 0x831ab30c
	if !ctx.cr[6].eq {
		sub_831AB30C(ctx, base);
		return;
	}
	// 831AB304: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 831AB308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AB30C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AB30C size=8
    let mut pc: u32 = 0x831AB30C;
    'dispatch: loop {
        match pc {
            0x831AB30C => {
    //   block [0x831AB30C..0x831AB314)
	// 831AB30C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831AB310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AB314(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AB314 size=8
    let mut pc: u32 = 0x831AB314;
    'dispatch: loop {
        match pc {
            0x831AB314 => {
    //   block [0x831AB314..0x831AB31C)
	// 831AB314: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AB318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AB320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AB320 size=392
    let mut pc: u32 = 0x831AB320;
    'dispatch: loop {
        match pc {
            0x831AB320 => {
    //   block [0x831AB320..0x831AB4A8)
	// 831AB320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AB324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831AB328: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831AB32C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831AB330: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 831AB334: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AB338: FFE01090  fmr f31, f2
	ctx.f[31].f64 = ctx.f[2].f64;
	// 831AB33C: 3D207FF0  lis r9, 0x7ff0
	ctx.r[9].s64 = 2146435072;
	// 831AB340: DBE10088  stfd f31, 0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.f[31].u64 ) };
	// 831AB344: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 831AB348: D8210080  stfd f1, 0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.f[1].u64 ) };
	// 831AB34C: 3D40FFF0  lis r10, -0x10
	ctx.r[10].s64 = -1048576;
	// 831AB350: FC000A10  fabs f0, f1
	ctx.f[0].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 831AB354: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 831AB358: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831AB35C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 831AB360: 409A0048  bne cr6, 0x831ab3a8
	if !ctx.cr[6].eq {
	pc = 0x831AB3A8; continue 'dispatch;
	}
	// 831AB364: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 831AB368: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831AB36C: 409A0078  bne cr6, 0x831ab3e4
	if !ctx.cr[6].eq {
	pc = 0x831AB3E4; continue 'dispatch;
	}
	// 831AB370: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 831AB374: C9ABE3A0  lfd f13, -0x1c60(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-7264 as u32) ) };
	// 831AB378: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 831AB37C: 40990010  ble cr6, 0x831ab38c
	if !ctx.cr[6].gt {
	pc = 0x831AB38C; continue 'dispatch;
	}
	// 831AB380: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AB384: C80BF468  lfd f0, -0xb98(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-2968 as u32) ) };
	// 831AB388: 480000FC  b 0x831ab484
	pc = 0x831AB484; continue 'dispatch;
	// 831AB38C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 831AB390: 40980010  bge cr6, 0x831ab3a0
	if !ctx.cr[6].lt {
	pc = 0x831AB3A0; continue 'dispatch;
	}
	// 831AB394: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 831AB398: C80BD228  lfd f0, -0x2dd8(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11736 as u32) ) };
	// 831AB39C: 480000E8  b 0x831ab484
	pc = 0x831AB484; continue 'dispatch;
	// 831AB3A0: D9BF0000  stfd f13, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.f[13].u64 ) };
	// 831AB3A4: 480000E4  b 0x831ab488
	pc = 0x831AB488; continue 'dispatch;
	// 831AB3A8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831AB3AC: 409A0038  bne cr6, 0x831ab3e4
	if !ctx.cr[6].eq {
	pc = 0x831AB3E4; continue 'dispatch;
	}
	// 831AB3B0: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 831AB3B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831AB3B8: 409A002C  bne cr6, 0x831ab3e4
	if !ctx.cr[6].eq {
	pc = 0x831AB3E4; continue 'dispatch;
	}
	// 831AB3BC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 831AB3C0: C9ABE3A0  lfd f13, -0x1c60(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-7264 as u32) ) };
	// 831AB3C4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 831AB3C8: 4199FFCC  bgt cr6, 0x831ab394
	if ctx.cr[6].gt {
	pc = 0x831AB394; continue 'dispatch;
	}
	// 831AB3CC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 831AB3D0: 4198FFB0  blt cr6, 0x831ab380
	if ctx.cr[6].lt {
	pc = 0x831AB380; continue 'dispatch;
	}
	// 831AB3D4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AB3D8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 831AB3DC: C80BF470  lfd f0, -0xb90(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-2960 as u32) ) };
	// 831AB3E0: 480000A4  b 0x831ab484
	pc = 0x831AB484; continue 'dispatch;
	// 831AB3E4: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 831AB3E8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 831AB3EC: 409A0030  bne cr6, 0x831ab41c
	if !ctx.cr[6].eq {
	pc = 0x831AB41C; continue 'dispatch;
	}
	// 831AB3F0: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 831AB3F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831AB3F8: 409A0090  bne cr6, 0x831ab488
	if !ctx.cr[6].eq {
	pc = 0x831AB488; continue 'dispatch;
	}
	// 831AB3FC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 831AB400: C80BD228  lfd f0, -0x2dd8(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11736 as u32) ) };
	// 831AB404: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 831AB408: 4199FF78  bgt cr6, 0x831ab380
	if ctx.cr[6].gt {
	pc = 0x831AB380; continue 'dispatch;
	}
	// 831AB40C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 831AB410: C9ABE3A0  lfd f13, -0x1c60(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-7264 as u32) ) };
	// 831AB414: FC1F036E  fsel f0, f31, f13, f0
	ctx.f[0].f64 = if ctx.f[31].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 831AB418: 4800006C  b 0x831ab484
	pc = 0x831AB484; continue 'dispatch;
	// 831AB41C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831AB420: 409A0068  bne cr6, 0x831ab488
	if !ctx.cr[6].eq {
	pc = 0x831AB488; continue 'dispatch;
	}
	// 831AB424: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 831AB428: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831AB42C: 409A005C  bne cr6, 0x831ab488
	if !ctx.cr[6].eq {
	pc = 0x831AB488; continue 'dispatch;
	}
	// 831AB430: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 831AB434: 4BFFFE7D  bl 0x831ab2b0
	ctx.lr = 0x831AB438;
	sub_831AB2B0(ctx, base);
	// 831AB438: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 831AB43C: C80BD228  lfd f0, -0x2dd8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11736 as u32) ) };
	// 831AB440: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 831AB444: 4099001C  ble cr6, 0x831ab460
	if !ctx.cr[6].gt {
	pc = 0x831AB460; continue 'dispatch;
	}
	// 831AB448: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AB44C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831AB450: C80BF468  lfd f0, -0xb98(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-2968 as u32) ) };
	// 831AB454: 409A0030  bne cr6, 0x831ab484
	if !ctx.cr[6].eq {
	pc = 0x831AB484; continue 'dispatch;
	}
	// 831AB458: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 831AB45C: 48000028  b 0x831ab484
	pc = 0x831AB484; continue 'dispatch;
	// 831AB460: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 831AB464: 40980018  bge cr6, 0x831ab47c
	if !ctx.cr[6].lt {
	pc = 0x831AB47C; continue 'dispatch;
	}
	// 831AB468: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831AB46C: 409A0018  bne cr6, 0x831ab484
	if !ctx.cr[6].eq {
	pc = 0x831AB484; continue 'dispatch;
	}
	// 831AB470: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AB474: C80BF488  lfd f0, -0xb78(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-2936 as u32) ) };
	// 831AB478: 4800000C  b 0x831ab484
	pc = 0x831AB484; continue 'dispatch;
	// 831AB47C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 831AB480: C80BE3A0  lfd f0, -0x1c60(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-7264 as u32) ) };
	// 831AB484: D81F0000  stfd f0, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.f[0].u64 ) };
	// 831AB488: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831AB48C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831AB490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831AB494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831AB498: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 831AB49C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831AB4A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831AB4A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AB4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831AB4A8 size=1260
    let mut pc: u32 = 0x831AB4A8;
    'dispatch: loop {
        match pc {
            0x831AB4A8 => {
    //   block [0x831AB4A8..0x831AB994)
	// 831AB4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AB4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831AB4B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831AB4B4: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 831AB4B8: 4BFFD5B5  bl 0x831a8a6c
	ctx.lr = 0x831AB4BC;
	sub_831A8A40(ctx, base);
	// 831AB4BC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AB4C0: FFA01090  fmr f29, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[2].f64;
	// 831AB4C4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 831AB4C8: FF800890  fmr f28, f1
	ctx.f[28].f64 = ctx.f[1].f64;
	// 831AB4CC: DB8100C0  stfd f28, 0xc0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.f[28].u64 ) };
	// 831AB4D0: DBA100C8  stfd f29, 0xc8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.f[29].u64 ) };
	// 831AB4D4: CB6BD228  lfd f27, -0x2dd8(r11)
	ctx.f[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11736 as u32) ) };
	// 831AB4D8: FF1DD800  fcmpu cr6, f29, f27
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[27].f64);
	// 831AB4DC: 409A0010  bne cr6, 0x831ab4ec
	if !ctx.cr[6].eq {
	pc = 0x831AB4EC; continue 'dispatch;
	}
	// 831AB4E0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 831AB4E4: C82BE3A0  lfd f1, -0x1c60(r11)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-7264 as u32) ) };
	// 831AB4E8: 48000490  b 0x831ab978
	pc = 0x831AB978; continue 'dispatch;
	// 831AB4EC: FF1CD800  fcmpu cr6, f28, f27
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[27].f64);
	// 831AB4F0: 409A0050  bne cr6, 0x831ab540
	if !ctx.cr[6].eq {
	pc = 0x831AB540; continue 'dispatch;
	}
	// 831AB4F4: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 831AB4F8: 4BFFFDB9  bl 0x831ab2b0
	ctx.lr = 0x831AB4FC;
	sub_831AB2B0(ctx, base);
	// 831AB4FC: FF1DD800  fcmpu cr6, f29, f27
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[27].f64);
	// 831AB500: 40980020  bge cr6, 0x831ab520
	if !ctx.cr[6].lt {
	pc = 0x831AB520; continue 'dispatch;
	}
	// 831AB504: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AB508: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831AB50C: C82BF468  lfd f1, -0xb98(r11)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-2968 as u32) ) };
	// 831AB510: 409A0468  bne cr6, 0x831ab978
	if !ctx.cr[6].eq {
	pc = 0x831AB978; continue 'dispatch;
	}
	// 831AB514: FC40E090  fmr f2, f28
	ctx.f[2].f64 = ctx.f[28].f64;
	// 831AB518: 48003031  bl 0x831ae548
	ctx.lr = 0x831AB51C;
	sub_831AE548(ctx, base);
	// 831AB51C: 4800045C  b 0x831ab978
	pc = 0x831AB978; continue 'dispatch;
	// 831AB520: FF1DD800  fcmpu cr6, f29, f27
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[27].f64);
	// 831AB524: 4099001C  ble cr6, 0x831ab540
	if !ctx.cr[6].gt {
	pc = 0x831AB540; continue 'dispatch;
	}
	// 831AB528: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831AB52C: 409A000C  bne cr6, 0x831ab538
	if !ctx.cr[6].eq {
	pc = 0x831AB538; continue 'dispatch;
	}
	// 831AB530: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 831AB534: 48000444  b 0x831ab978
	pc = 0x831AB978; continue 'dispatch;
	// 831AB538: FC20D890  fmr f1, f27
	ctx.f[1].f64 = ctx.f[27].f64;
	// 831AB53C: 4800043C  b 0x831ab978
	pc = 0x831AB978; continue 'dispatch;
	// 831AB540: A16100C0  lhz r11, 0xc0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(192 as u32) ) } as u64;
	// 831AB544: A12100C8  lhz r9, 0xc8(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(200 as u32) ) } as u64;
	// 831AB548: 556A0476  rlwinm r10, r11, 0, 0x11, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AB54C: 2B0A7FF0  cmplwi cr6, r10, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32752 as u32, &mut ctx.xer);
	// 831AB550: 419A03B4  beq cr6, 0x831ab904
	if ctx.cr[6].eq {
	pc = 0x831AB904; continue 'dispatch;
	}
	// 831AB554: 552A0476  rlwinm r10, r9, 0, 0x11, 0x1b
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 831AB558: 2B0A7FF0  cmplwi cr6, r10, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32752 as u32, &mut ctx.xer);
	// 831AB55C: 419A03A8  beq cr6, 0x831ab904
	if ctx.cr[6].eq {
	pc = 0x831AB904; continue 'dispatch;
	}
	// 831AB560: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 831AB564: FF1CD800  fcmpu cr6, f28, f27
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[27].f64);
	// 831AB568: CB4BE3A0  lfd f26, -0x1c60(r11)
	ctx.f[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-7264 as u32) ) };
	// 831AB56C: FF20D090  fmr f25, f26
	ctx.f[25].f64 = ctx.f[26].f64;
	// 831AB570: 40980034  bge cr6, 0x831ab5a4
	if !ctx.cr[6].lt {
	pc = 0x831AB5A4; continue 'dispatch;
	}
	// 831AB574: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 831AB578: 4BFFFD39  bl 0x831ab2b0
	ctx.lr = 0x831AB57C;
	sub_831AB2B0(ctx, base);
	// 831AB57C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831AB580: 419A0018  beq cr6, 0x831ab598
	if ctx.cr[6].eq {
	pc = 0x831AB598; continue 'dispatch;
	}
	// 831AB584: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 831AB588: 419A0018  beq cr6, 0x831ab5a0
	if ctx.cr[6].eq {
	pc = 0x831AB5A0; continue 'dispatch;
	}
	// 831AB58C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AB590: C82BF470  lfd f1, -0xb90(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-2960 as u32) ) };
	// 831AB594: 480003E4  b 0x831ab978
	pc = 0x831AB978; continue 'dispatch;
	// 831AB598: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 831AB59C: CB2BA3F8  lfd f25, -0x5c08(r11)
	ctx.f[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-23560 as u32) ) };
	// 831AB5A0: FF80E050  fneg f28, f28
	ctx.f[28].u64 = ctx.f[28].u64 ^ 0x8000_0000_0000_0000u64;
	// 831AB5A4: FDA0EA10  fabs f13, f29
	ctx.f[13].u64 = ctx.f[29].u64 & !0x8000_0000_0000_0000u64;
	// 831AB5A8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AB5AC: C80BEE98  lfd f0, -0x1168(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-4456 as u32) ) };
	// 831AB5B0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831AB5B4: 40990040  ble cr6, 0x831ab5f4
	if !ctx.cr[6].gt {
	pc = 0x831AB5F4; continue 'dispatch;
	}
	// 831AB5B8: FF1DD800  fcmpu cr6, f29, f27
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[27].f64);
	// 831AB5BC: 40980008  bge cr6, 0x831ab5c4
	if !ctx.cr[6].lt {
	pc = 0x831AB5C4; continue 'dispatch;
	}
	// 831AB5C0: FF9AE024  fdiv f28, f26, f28
	ctx.f[28].f64 = ctx.f[26].f64 / ctx.f[28].f64;
	// 831AB5C4: FF1CD000  fcmpu cr6, f28, f26
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[26].f64);
	// 831AB5C8: 40990014  ble cr6, 0x831ab5dc
	if !ctx.cr[6].gt {
	pc = 0x831AB5DC; continue 'dispatch;
	}
	// 831AB5CC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AB5D0: C80BF468  lfd f0, -0xb98(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-2968 as u32) ) };
	// 831AB5D4: FC200672  fmul f1, f0, f25
	ctx.f[1].f64 = ctx.f[0].f64 * ctx.f[25].f64;
	// 831AB5D8: 480003A0  b 0x831ab978
	pc = 0x831AB978; continue 'dispatch;
	// 831AB5DC: FF1CD000  fcmpu cr6, f28, f26
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[26].f64);
	// 831AB5E0: 4098000C  bge cr6, 0x831ab5ec
	if !ctx.cr[6].lt {
	pc = 0x831AB5EC; continue 'dispatch;
	}
	// 831AB5E4: FC3906F2  fmul f1, f25, f27
	ctx.f[1].f64 = ctx.f[25].f64 * ctx.f[27].f64;
	// 831AB5E8: 48000390  b 0x831ab978
	pc = 0x831AB978; continue 'dispatch;
	// 831AB5EC: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 831AB5F0: 48000388  b 0x831ab978
	pc = 0x831AB978; continue 'dispatch;
	// 831AB5F4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831AB5F8: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 831AB5FC: 480099E5  bl 0x831b4fe0
	ctx.lr = 0x831AB600;
	sub_831B4FE0(ctx, base);
	// 831AB600: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831AB604: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 831AB608: C80BBE88  lfd f0, -0x4178(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-16760 as u32) ) };
	// 831AB60C: FF1D0000  fcmpu cr6, f29, f0
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[0].f64);
	// 831AB610: 4199009C  bgt cr6, 0x831ab6ac
	if ctx.cr[6].gt {
	pc = 0x831AB6AC; continue 'dispatch;
	}
	// 831AB614: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 831AB618: 4BFFFC99  bl 0x831ab2b0
	ctx.lr = 0x831AB61C;
	sub_831AB2B0(ctx, base);
	// 831AB61C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831AB620: 4182008C  beq 0x831ab6ac
	if ctx.cr[0].eq {
	pc = 0x831AB6AC; continue 'dispatch;
	}
	// 831AB624: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 831AB628: 4BFFFC89  bl 0x831ab2b0
	ctx.lr = 0x831AB62C;
	sub_831AB2B0(ctx, base);
	// 831AB62C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831AB630: 4182007C  beq 0x831ab6ac
	if ctx.cr[0].eq {
	pc = 0x831AB6AC; continue 'dispatch;
	}
	// 831AB634: FF1DD800  fcmpu cr6, f29, f27
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[27].f64);
	// 831AB638: 40990074  ble cr6, 0x831ab6ac
	if !ctx.cr[6].gt {
	pc = 0x831AB6AC; continue 'dispatch;
	}
	// 831AB63C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831AB640: FC00E81E  fctiwz f0, f29
	ctx.f[0].s64 = if ctx.f[29].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[29].f64.trunc() as i32 as i64 };
	// 831AB644: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 831AB648: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 831AB64C: FFE0D090  fmr f31, f26
	ctx.f[31].f64 = ctx.f[26].f64;
	// 831AB650: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AB654: 7FEB51D6  mullw r31, r11, r10
	ctx.r[31].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 831AB658: 419A001C  beq cr6, 0x831ab674
	if ctx.cr[6].eq {
	pc = 0x831AB674; continue 'dispatch;
	}
	// 831AB65C: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AB660: 41820008  beq 0x831ab668
	if ctx.cr[0].eq {
	pc = 0x831AB668; continue 'dispatch;
	}
	// 831AB664: FFFF07B2  fmul f31, f31, f30
	ctx.f[31].f64 = ctx.f[31].f64 * ctx.f[30].f64;
	// 831AB668: 7D6B0E71  srawi. r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AB66C: FFDE07B2  fmul f30, f30, f30
	ctx.f[30].f64 = ctx.f[30].f64 * ctx.f[30].f64;
	// 831AB670: 4082FFEC  bne 0x831ab65c
	if !ctx.cr[0].eq {
	pc = 0x831AB65C; continue 'dispatch;
	}
	// 831AB674: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 831AB678: 48009891  bl 0x831b4f08
	ctx.lr = 0x831AB67C;
	sub_831B4F08(ctx, base);
	// 831AB67C: 7C83FA14  add r4, r3, r31
	ctx.r[4].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 831AB680: 2F040A00  cmpwi cr6, r4, 0xa00
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2560, &mut ctx.xer);
	// 831AB684: 40990014  ble cr6, 0x831ab698
	if !ctx.cr[6].gt {
	pc = 0x831AB698; continue 'dispatch;
	}
	// 831AB688: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AB68C: C80BF468  lfd f0, -0xb98(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-2968 as u32) ) };
	// 831AB690: FC0007F2  fmul f0, f0, f31
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[31].f64;
	// 831AB694: 4BFFFF40  b 0x831ab5d4
	pc = 0x831AB5D4; continue 'dispatch;
	// 831AB698: 2F04F603  cmpwi cr6, r4, -0x9fd
	ctx.cr[6].compare_i32(ctx.r[4].s32, -2557, &mut ctx.xer);
	// 831AB69C: 40980248  bge cr6, 0x831ab8e4
	if !ctx.cr[6].lt {
	pc = 0x831AB8E4; continue 'dispatch;
	}
	// 831AB6A0: FC1F0672  fmul f0, f31, f25
	ctx.f[0].f64 = ctx.f[31].f64 * ctx.f[25].f64;
	// 831AB6A4: FC2006F2  fmul f1, f0, f27
	ctx.f[1].f64 = ctx.f[0].f64 * ctx.f[27].f64;
	// 831AB6A8: 480002D0  b 0x831ab978
	pc = 0x831AB978; continue 'dispatch;
	// 831AB6AC: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 831AB6B0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831AB6B4: 3BEAC408  addi r31, r10, -0x3bf8
	ctx.r[31].s64 = ctx.r[10].s64 + -15352;
	// 831AB6B8: C81F0048  lfd f0, 0x48(r31)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) };
	// 831AB6BC: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 831AB6C0: 41990008  bgt cr6, 0x831ab6c8
	if ctx.cr[6].gt {
	pc = 0x831AB6C8; continue 'dispatch;
	}
	// 831AB6C4: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 831AB6C8: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831AB6CC: 393F0020  addi r9, r31, 0x20
	ctx.r[9].s64 = ctx.r[31].s64 + 32;
	// 831AB6D0: 7C0A4CAE  lfdx f0, r10, r9
	ctx.f[0].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 831AB6D4: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 831AB6D8: 41990008  bgt cr6, 0x831ab6e0
	if ctx.cr[6].gt {
	pc = 0x831AB6E0; continue 'dispatch;
	}
	// 831AB6DC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831AB6E0: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831AB6E4: 393F0010  addi r9, r31, 0x10
	ctx.r[9].s64 = ctx.r[31].s64 + 16;
	// 831AB6E8: 7C0A4CAE  lfdx f0, r10, r9
	ctx.f[0].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 831AB6EC: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 831AB6F0: 41990008  bgt cr6, 0x831ab6f8
	if ctx.cr[6].gt {
	pc = 0x831AB6F8; continue 'dispatch;
	}
	// 831AB6F4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831AB6F8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831AB6FC: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831AB700: 7D2B5051  subf. r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831AB704: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831AB708: 41800008  blt 0x831ab710
	if ctx.cr[0].lt {
	pc = 0x831AB710; continue 'dispatch;
	}
	// 831AB70C: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 831AB710: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 831AB714: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 831AB718: CBE80888  lfd f31, 0x888(r8)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(2184 as u32) ) };
	// 831AB71C: 409A001C  bne cr6, 0x831ab738
	if !ctx.cr[6].eq {
	pc = 0x831AB738; continue 'dispatch;
	}
	// 831AB720: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 831AB724: 48002BAD  bl 0x831ae2d0
	ctx.lr = 0x831AB728;
	sub_831AE2D0(ctx, base);
	// 831AB728: C81F00D8  lfd f0, 0xd8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) };
	// 831AB72C: FDA0D890  fmr f13, f27
	ctx.f[13].f64 = ctx.f[27].f64;
	// 831AB730: FC010032  fmul f0, f1, f0
	ctx.f[0].f64 = ctx.f[1].f64 * ctx.f[0].f64;
	// 831AB734: 48000094  b 0x831ab7c8
	pc = 0x831AB7C8; continue 'dispatch;
	// 831AB738: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831AB73C: C97F0100  lfd f11, 0x100(r31)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) };
	// 831AB740: 391F0008  addi r8, r31, 8
	ctx.r[8].s64 = ctx.r[31].s64 + 8;
	// 831AB744: C95F00F8  lfd f10, 0xf8(r31)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(248 as u32) ) };
	// 831AB748: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831AB74C: C93F00F0  lfd f9, 0xf0(r31)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(240 as u32) ) };
	// 831AB750: 38FF0090  addi r7, r31, 0x90
	ctx.r[7].s64 = ctx.r[31].s64 + 144;
	// 831AB754: C91F00E8  lfd f8, 0xe8(r31)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) };
	// 831AB758: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 831AB75C: C99F00E0  lfd f12, 0xe0(r31)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) };
	// 831AB760: 3CC08205  lis r6, -0x7dfb
	ctx.r[6].s64 = -2113601536;
	// 831AB764: C8FF00D8  lfd f7, 0xd8(r31)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) };
	// 831AB768: 7C0A44AE  lfdx f0, r10, r8
	ctx.f[0].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) };
	// 831AB76C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 831AB770: FCDE0028  fsub f6, f30, f0
	ctx.f[6].f64 = ctx.f[30].f64 - ctx.f[0].f64;
	// 831AB774: 7D2A07B4  extsw r10, r9
	ctx.r[10].s64 = ctx.r[9].s32 as i64;
	// 831AB778: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831AB77C: FCA0F02A  fadd f5, f0, f30
	ctx.f[5].f64 = ctx.f[0].f64 + ctx.f[30].f64;
	// 831AB780: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 831AB784: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831AB788: C9A6AA10  lfd f13, -0x55f0(r6)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(-22000 as u32) ) };
	// 831AB78C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 831AB790: 7C8B3CAE  lfdx f4, r11, r7
	ctx.f[4].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) };
	// 831AB794: FC0007F2  fmul f0, f0, f31
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[31].f64;
	// 831AB798: FCC62028  fsub f6, f6, f4
	ctx.f[6].f64 = ctx.f[6].f64 - ctx.f[4].f64;
	// 831AB79C: FCC62824  fdiv f6, f6, f5
	ctx.f[6].f64 = ctx.f[6].f64 / ctx.f[5].f64;
	// 831AB7A0: FDA60372  fmul f13, f6, f13
	ctx.f[13].f64 = ctx.f[6].f64 * ctx.f[13].f64;
	// 831AB7A4: FCCD0372  fmul f6, f13, f13
	ctx.f[6].f64 = ctx.f[13].f64 * ctx.f[13].f64;
	// 831AB7A8: FD8D0332  fmul f12, f13, f12
	ctx.f[12].f64 = ctx.f[13].f64 * ctx.f[12].f64;
	// 831AB7AC: FD6652FA  fmadd f11, f6, f11, f10
	ctx.f[11].f64 = ctx.f[6].f64 * ctx.f[11].f64 + ctx.f[10].f64;
	// 831AB7B0: FD6B49BA  fmadd f11, f11, f6, f9
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[6].f64 + ctx.f[9].f64;
	// 831AB7B4: FD6B41BA  fmadd f11, f11, f6, f8
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[6].f64 + ctx.f[8].f64;
	// 831AB7B8: FD6B01B2  fmul f11, f11, f6
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[6].f64;
	// 831AB7BC: FD6B0372  fmul f11, f11, f13
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	// 831AB7C0: FD8B61FA  fmadd f12, f11, f7, f12
	ctx.f[12].f64 = ctx.f[11].f64 * ctx.f[7].f64 + ctx.f[12].f64;
	// 831AB7C4: FDAC682A  fadd f13, f12, f13
	ctx.f[13].f64 = ctx.f[12].f64 + ctx.f[13].f64;
	// 831AB7C8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 831AB7CC: FD6D0772  fmul f11, f13, f29
	ctx.f[11].f64 = ctx.f[13].f64 * ctx.f[29].f64;
	// 831AB7D0: C99F0140  lfd f12, 0x140(r31)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(320 as u32) ) };
	// 831AB7D4: C9ABE5A0  lfd f13, -0x1a60(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-6752 as u32) ) };
	// 831AB7D8: FD5D0372  fmul f10, f29, f13
	ctx.f[10].f64 = ctx.f[29].f64 * ctx.f[13].f64;
	// 831AB7DC: FD40565C  fctid f10, f10
	ctx.f[10].s64 = if ctx.f[10].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[10].f64 as i64 };
	// 831AB7E0: FD40569C  fcfid f10, f10
	ctx.f[10].f64 = (ctx.f[10].s64 as f64);
	// 831AB7E4: FD4A07F2  fmul f10, f10, f31
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[31].f64;
	// 831AB7E8: FD3D5028  fsub f9, f29, f10
	ctx.f[9].f64 = ctx.f[29].f64 - ctx.f[10].f64;
	// 831AB7EC: FD69583A  fmadd f11, f9, f0, f11
	ctx.f[11].f64 = ctx.f[9].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 831AB7F0: FD2B0372  fmul f9, f11, f13
	ctx.f[9].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	// 831AB7F4: FD204E5C  fctid f9, f9
	ctx.f[9].s64 = if ctx.f[9].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[9].f64 as i64 };
	// 831AB7F8: FD204E9C  fcfid f9, f9
	ctx.f[9].f64 = (ctx.f[9].s64 as f64);
	// 831AB7FC: FD2907F2  fmul f9, f9, f31
	ctx.f[9].f64 = ctx.f[9].f64 * ctx.f[31].f64;
	// 831AB800: FC0A483A  fmadd f0, f10, f0, f9
	ctx.f[0].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[9].f64;
	// 831AB804: FD6B4828  fsub f11, f11, f9
	ctx.f[11].f64 = ctx.f[11].f64 - ctx.f[9].f64;
	// 831AB808: FD400372  fmul f10, f0, f13
	ctx.f[10].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 831AB80C: FD40565C  fctid f10, f10
	ctx.f[10].s64 = if ctx.f[10].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[10].f64 as i64 };
	// 831AB810: FD40569C  fcfid f10, f10
	ctx.f[10].f64 = (ctx.f[10].s64 as f64);
	// 831AB814: FD4A07F2  fmul f10, f10, f31
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[31].f64;
	// 831AB818: FC005028  fsub f0, f0, f10
	ctx.f[0].f64 = ctx.f[0].f64 - ctx.f[10].f64;
	// 831AB81C: FC00582A  fadd f0, f0, f11
	ctx.f[0].f64 = ctx.f[0].f64 + ctx.f[11].f64;
	// 831AB820: FD600372  fmul f11, f0, f13
	ctx.f[11].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 831AB824: FD605E5C  fctid f11, f11
	ctx.f[11].s64 = if ctx.f[11].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[11].f64 as i64 };
	// 831AB828: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 831AB82C: FD6B07F2  fmul f11, f11, f31
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[31].f64;
	// 831AB830: FD4A582A  fadd f10, f10, f11
	ctx.f[10].f64 = ctx.f[10].f64 + ctx.f[11].f64;
	// 831AB834: FC005828  fsub f0, f0, f11
	ctx.f[0].f64 = ctx.f[0].f64 - ctx.f[11].f64;
	// 831AB838: FDAA0372  fmul f13, f10, f13
	ctx.f[13].f64 = ctx.f[10].f64 * ctx.f[13].f64;
	// 831AB83C: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 831AB840: 4199FD8C  bgt cr6, 0x831ab5cc
	if ctx.cr[6].gt {
	pc = 0x831AB5CC; continue 'dispatch;
	}
	// 831AB844: C99F0148  lfd f12, 0x148(r31)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) };
	// 831AB848: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 831AB84C: 4198FD98  blt cr6, 0x831ab5e4
	if ctx.cr[6].lt {
	pc = 0x831AB5E4; continue 'dispatch;
	}
	// 831AB850: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 831AB854: D9A10058  stfd f13, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[13].u64 ) };
	// 831AB858: FF00D800  fcmpu cr6, f0, f27
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[27].f64);
	// 831AB85C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 831AB860: 4099000C  ble cr6, 0x831ab86c
	if !ctx.cr[6].gt {
	pc = 0x831AB86C; continue 'dispatch;
	}
	// 831AB864: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831AB868: FC00F828  fsub f0, f0, f31
	ctx.f[0].f64 = ctx.f[0].f64 - ctx.f[31].f64;
	// 831AB86C: C9BF0138  lfd f13, 0x138(r31)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(312 as u32) ) };
	// 831AB870: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 831AB874: C99F0130  lfd f12, 0x130(r31)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) };
	// 831AB878: 7D692670  srawi r9, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 831AB87C: FD00637A  fmadd f8, f0, f13, f12
	ctx.f[8].f64 = ctx.f[0].f64 * ctx.f[13].f64 + ctx.f[12].f64;
	// 831AB880: C9BF0128  lfd f13, 0x128(r31)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(296 as u32) ) };
	// 831AB884: C99F0120  lfd f12, 0x120(r31)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) };
	// 831AB888: 7D480034  cntlzw r8, r10
	ctx.r[8].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 831AB88C: C97F0118  lfd f11, 0x118(r31)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(280 as u32) ) };
	// 831AB890: 7D490194  addze r10, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[10].s64 = tmp.s64;
	// 831AB894: C95F0110  lfd f10, 0x110(r31)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) };
	// 831AB898: 5509DFFE  rlwinm r9, r8, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 831AB89C: C93F0108  lfd f9, 0x108(r31)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) };
	// 831AB8A0: 391F0008  addi r8, r31, 8
	ctx.r[8].s64 = ctx.r[31].s64 + 8;
	// 831AB8A4: 69290001  xori r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u64 ^ 1;
	// 831AB8A8: 7FE95214  add r31, r9, r10
	ctx.r[31].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 831AB8AC: 57EA2036  slwi r10, r31, 4
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831AB8B0: FDA8683A  fmadd f13, f8, f0, f13
	ctx.f[13].f64 = ctx.f[8].f64 * ctx.f[0].f64 + ctx.f[13].f64;
	// 831AB8B4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831AB8B8: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831AB8BC: 7D0B44AE  lfdx f8, r11, r8
	ctx.f[8].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) };
	// 831AB8C0: FDAD603A  fmadd f13, f13, f0, f12
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 831AB8C4: FDAD583A  fmadd f13, f13, f0, f11
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 831AB8C8: FDAD503A  fmadd f13, f13, f0, f10
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[10].f64;
	// 831AB8CC: FDAD483A  fmadd f13, f13, f0, f9
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[9].f64;
	// 831AB8D0: FC0DD03A  fmadd f0, f13, f0, f26
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[26].f64;
	// 831AB8D4: FFE00232  fmul f31, f0, f8
	ctx.f[31].f64 = ctx.f[0].f64 * ctx.f[8].f64;
	// 831AB8D8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 831AB8DC: 4800962D  bl 0x831b4f08
	ctx.lr = 0x831AB8E0;
	sub_831B4F08(ctx, base);
	// 831AB8E0: 7C83FA14  add r4, r3, r31
	ctx.r[4].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 831AB8E4: 2F040400  cmpwi cr6, r4, 0x400
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1024, &mut ctx.xer);
	// 831AB8E8: 4199FCE4  bgt cr6, 0x831ab5cc
	if ctx.cr[6].gt {
	pc = 0x831AB5CC; continue 'dispatch;
	}
	// 831AB8EC: 2F04FC03  cmpwi cr6, r4, -0x3fd
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1021, &mut ctx.xer);
	// 831AB8F0: 4198FCF4  blt cr6, 0x831ab5e4
	if ctx.cr[6].lt {
	pc = 0x831AB5E4; continue 'dispatch;
	}
	// 831AB8F4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 831AB8F8: 480095E9  bl 0x831b4ee0
	ctx.lr = 0x831AB8FC;
	sub_831B4EE0(ctx, base);
	// 831AB8FC: FC210672  fmul f1, f1, f25
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[1].f64 * ctx.f[25].f64;
	// 831AB900: 48000078  b 0x831ab978
	pc = 0x831AB978; continue 'dispatch;
	// 831AB904: 556A0478  rlwinm r10, r11, 0, 0x11, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AB908: 2B0A7FF0  cmplwi cr6, r10, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32752 as u32, &mut ctx.xer);
	// 831AB90C: 409A001C  bne cr6, 0x831ab928
	if !ctx.cr[6].eq {
	pc = 0x831AB928; continue 'dispatch;
	}
	// 831AB910: 816100C0  lwz r11, 0xc0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(192 as u32) ) } as u64;
	// 831AB914: 556B037F  clrlwi. r11, r11, 0xd
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0007FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AB918: 4082005C  bne 0x831ab974
	if !ctx.cr[0].eq {
	pc = 0x831AB974; continue 'dispatch;
	}
	// 831AB91C: 816100C4  lwz r11, 0xc4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(196 as u32) ) } as u64;
	// 831AB920: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831AB924: 409A0050  bne cr6, 0x831ab974
	if !ctx.cr[6].eq {
	pc = 0x831AB974; continue 'dispatch;
	}
	// 831AB928: 552B0478  rlwinm r11, r9, 0, 0x11, 0x1c
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 831AB92C: 2B0B7FF0  cmplwi cr6, r11, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32752 as u32, &mut ctx.xer);
	// 831AB930: 409A001C  bne cr6, 0x831ab94c
	if !ctx.cr[6].eq {
	pc = 0x831AB94C; continue 'dispatch;
	}
	// 831AB934: 812100C8  lwz r9, 0xc8(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(200 as u32) ) } as u64;
	// 831AB938: 5529037F  clrlwi. r9, r9, 0xd
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0007FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831AB93C: 40820038  bne 0x831ab974
	if !ctx.cr[0].eq {
	pc = 0x831AB974; continue 'dispatch;
	}
	// 831AB940: 812100CC  lwz r9, 0xcc(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(204 as u32) ) } as u64;
	// 831AB944: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 831AB948: 409A002C  bne cr6, 0x831ab974
	if !ctx.cr[6].eq {
	pc = 0x831AB974; continue 'dispatch;
	}
	// 831AB94C: 2B0A7FF8  cmplwi cr6, r10, 0x7ff8
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32760 as u32, &mut ctx.xer);
	// 831AB950: 419A0024  beq cr6, 0x831ab974
	if ctx.cr[6].eq {
	pc = 0x831AB974; continue 'dispatch;
	}
	// 831AB954: 2B0B7FF8  cmplwi cr6, r11, 0x7ff8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32760 as u32, &mut ctx.xer);
	// 831AB958: 419A001C  beq cr6, 0x831ab974
	if ctx.cr[6].eq {
	pc = 0x831AB974; continue 'dispatch;
	}
	// 831AB95C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 831AB960: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 831AB964: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 831AB968: 4BFFF9B9  bl 0x831ab320
	ctx.lr = 0x831AB96C;
	sub_831AB320(ctx, base);
	// 831AB96C: C8210058  lfd f1, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831AB970: 48000008  b 0x831ab978
	pc = 0x831AB978; continue 'dispatch;
	// 831AB974: FC3CE82A  fadd f1, f28, f29
	ctx.f[1].f64 = ctx.f[28].f64 + ctx.f[29].f64;
	// 831AB978: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 831AB97C: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 831AB980: 4BFFD139  bl 0x831a8ab8
	ctx.lr = 0x831AB984;
	sub_831A8A8C(ctx, base);
	// 831AB984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831AB988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831AB98C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831AB990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AB998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AB998 size=16
    let mut pc: u32 = 0x831AB998;
    'dispatch: loop {
        match pc {
            0x831AB998 => {
    //   block [0x831AB998..0x831AB9A8)
	// 831AB998: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 831AB99C: C80BD228  lfd f0, -0x2dd8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11736 as u32) ) };
	// 831AB9A0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 831AB9A4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AB9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AB9A8 size=164
    let mut pc: u32 = 0x831AB9A8;
    'dispatch: loop {
        match pc {
            0x831AB9A8 => {
    //   block [0x831AB9A8..0x831ABA4C)
	// 831AB9A8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831AB9AC: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 831AB9B0: 396BC558  addi r11, r11, -0x3aa8
	ctx.r[11].s64 = ctx.r[11].s64 + -15016;
	// 831AB9B4: 392AEEA0  addi r9, r10, -0x1160
	ctx.r[9].s64 = ctx.r[10].s64 + -4448;
	// 831AB9B8: 3D00821A  lis r8, -0x7de6
	ctx.r[8].s64 = -2112225280;
	// 831AB9BC: 3CE0821A  lis r7, -0x7de6
	ctx.r[7].s64 = -2112225280;
	// 831AB9C0: C9AAEEA0  lfd f13, -0x1160(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-4448 as u32) ) };
	// 831AB9C4: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 831AB9C8: C80B0000  lfd f0, 0(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 831AB9CC: 3CC0821A  lis r6, -0x7de6
	ctx.r[6].s64 = -2112225280;
	// 831AB9D0: FC810032  fmul f4, f1, f0
	ctx.f[4].f64 = ctx.f[1].f64 * ctx.f[0].f64;
	// 831AB9D4: C9890008  lfd f12, 8(r9)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	// 831AB9D8: C94B0020  lfd f10, 0x20(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 831AB9DC: C968C5C8  lfd f11, -0x3a38(r8)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(-14904 as u32) ) };
	// 831AB9E0: C92B0050  lfd f9, 0x50(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 831AB9E4: C907C5C0  lfd f8, -0x3a40(r7)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(-14912 as u32) ) };
	// 831AB9E8: C8EAC5B8  lfd f7, -0x3a48(r10)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-14920 as u32) ) };
	// 831AB9EC: C8CB0040  lfd f6, 0x40(r11)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 831AB9F0: C80B0030  lfd f0, 0x30(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 831AB9F4: C8A6C5B0  lfd f5, -0x3a50(r6)
	ctx.f[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(-14928 as u32) ) };
	// 831AB9F8: FC80265C  fctid f4, f4
	ctx.f[4].s64 = if ctx.f[4].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[4].f64 as i64 };
	// 831AB9FC: D881FFF0  stfd f4, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[4].u64 ) };
	// 831ABA00: E941FFF0  ld r10, -0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831ABA04: 794A07E0  clrldi r10, r10, 0x3f
	ctx.r[10].u64 = ctx.r[10].u64 & 0x0000000000000001u64;
	// 831ABA08: FC80269C  fcfid f4, f4
	ctx.f[4].f64 = (ctx.f[4].s64 as f64);
	// 831ABA0C: 2F2A0000  cmpdi cr6, r10, 0
	ctx.cr[6].compare_i64(ctx.r[10].s64, 0, &mut ctx.xer);
	// 831ABA10: FDAD093C  fnmsub f13, f13, f4, f1
	ctx.f[13].f64 = -(ctx.f[13].f64 * ctx.f[4].f64 - ctx.f[1].f64);
	// 831ABA14: FDAC693C  fnmsub f13, f12, f4, f13
	ctx.f[13].f64 = -(ctx.f[12].f64 * ctx.f[4].f64 - ctx.f[13].f64);
	// 831ABA18: FD8D0372  fmul f12, f13, f13
	ctx.f[12].f64 = ctx.f[13].f64 * ctx.f[13].f64;
	// 831ABA1C: FD6C52FC  fnmsub f11, f12, f11, f10
	ctx.f[11].f64 = -(ctx.f[12].f64 * ctx.f[11].f64 - ctx.f[10].f64);
	// 831ABA20: FD4C4278  fmsub f10, f12, f9, f8
	ctx.f[10].f64 = ctx.f[12].f64 * ctx.f[9].f64 - ctx.f[8].f64;
	// 831ABA24: FD6B3B38  fmsub f11, f11, f12, f7
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[12].f64 - ctx.f[7].f64;
	// 831ABA28: FD4A333A  fmadd f10, f10, f12, f6
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[12].f64 + ctx.f[6].f64;
	// 831ABA2C: FD6B033A  fmadd f11, f11, f12, f0
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[12].f64 + ctx.f[0].f64;
	// 831ABA30: FD4A2B38  fmsub f10, f10, f12, f5
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[12].f64 - ctx.f[5].f64;
	// 831ABA34: FDAB0372  fmul f13, f11, f13
	ctx.f[13].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	// 831ABA38: FC0A033A  fmadd f0, f10, f12, f0
	ctx.f[0].f64 = ctx.f[10].f64 * ctx.f[12].f64 + ctx.f[0].f64;
	// 831ABA3C: 419A0010  beq cr6, 0x831aba4c
	if ctx.cr[6].eq {
		sub_831ABA4C(ctx, base);
		return;
	}
	// 831ABA40: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 831ABA44: FC006824  fdiv f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[13].f64;
	// 831ABA48: 48000008  b 0x831aba50
	sub_831ABA4C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ABA4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ABA4C size=32
    let mut pc: u32 = 0x831ABA4C;
    'dispatch: loop {
        match pc {
            0x831ABA4C => {
    //   block [0x831ABA4C..0x831ABA6C)
	// 831ABA4C: FC0D0024  fdiv f0, f13, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[13].f64 / ctx.f[0].f64;
	// 831ABA50: FD600A10  fabs f11, f1
	ctx.f[11].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 831ABA54: C9AB0010  lfd f13, 0x10(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 831ABA58: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831ABA5C: C98BF470  lfd f12, -0xb90(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-2960 as u32) ) };
	// 831ABA60: FDAB6828  fsub f13, f11, f13
	ctx.f[13].f64 = ctx.f[11].f64 - ctx.f[13].f64;
	// 831ABA64: FC2D032E  fsel f1, f13, f12, f0
	ctx.f[1].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[0].f64 };
	// 831ABA68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ABA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ABA70 size=4
    let mut pc: u32 = 0x831ABA70;
    'dispatch: loop {
        match pc {
            0x831ABA70 => {
    //   block [0x831ABA70..0x831ABA74)
	// 831ABA70: 7D8300D0  neg r12, r3
	ctx.r[12].s64 = -ctx.r[3].s64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ABA74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ABA74 size=16
    let mut pc: u32 = 0x831ABA74;
    'dispatch: loop {
        match pc {
            0x831ABA74 => {
    //   block [0x831ABA74..0x831ABA84)
	// 831ABA74: 7D6C00D0  neg r11, r12
	ctx.r[11].s64 = -ctx.r[12].s64;
	// 831ABA78: 380B0FFF  addi r0, r11, 0xfff
	ctx.r[0].s64 = ctx.r[11].s64 + 4095;
	// 831ABA7C: 7C006671  srawi. r0, r0, 0xc
	ctx.xer.ca = (ctx.r[0].s32 < 0) && ((ctx.r[0].u32 & ((1u32 << 12) - 1)) != 0);
	ctx.r[0].s64 = (ctx.r[0].s32 >> 12) as i64;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831ABA80: 4C810020  blelr
	if !ctx.cr[0].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ABA84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ABA84 size=20
    let mut pc: u32 = 0x831ABA84;
    'dispatch: loop {
        match pc {
            0x831ABA84 => {
    //   block [0x831ABA84..0x831ABA98)
	// 831ABA84: 7C2B0B78  mr r11, r1
	ctx.r[11].u64 = ctx.r[1].u64;
	// 831ABA88: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831ABA8C: 840BF000  lwzu r0, -0x1000(r11)
	ea = ctx.r[11].u32.wrapping_add(-4096 as u32);
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ea) } as u64;
	ctx.r[11].u32 = ea;
	// 831ABA90: 4200FFFC  bdnz 0x831aba8c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831ABA8C; continue 'dispatch;
	}
	// 831ABA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ABA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ABA98 size=60
    let mut pc: u32 = 0x831ABA98;
    'dispatch: loop {
        match pc {
            0x831ABA98 => {
    //   block [0x831ABA98..0x831ABAD4)
	// 831ABA98: FD800E5E  fctidz f12, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].s64 = if ctx.f[1].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[1].f64.trunc() as i64 };
	// 831ABA9C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 831ABAA0: FD600A10  fabs f11, f1
	ctx.f[11].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 831ABAA4: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 831ABAA8: C9ABE3A0  lfd f13, -0x1c60(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-7264 as u32) ) };
	// 831ABAAC: C80AC5D0  lfd f0, -0x3a30(r10)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-14896 as u32) ) };
	// 831ABAB0: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 831ABAB4: FC005828  fsub f0, f0, f11
	ctx.f[0].f64 = ctx.f[0].f64 - ctx.f[11].f64;
	// 831ABAB8: FD605850  fneg f11, f11
	ctx.f[11].u64 = ctx.f[11].u64 ^ 0x8000_0000_0000_0000u64;
	// 831ABABC: FD4C0828  fsub f10, f12, f1
	ctx.f[10].f64 = ctx.f[12].f64 - ctx.f[1].f64;
	// 831ABAC0: FDAC682A  fadd f13, f12, f13
	ctx.f[13].f64 = ctx.f[12].f64 + ctx.f[13].f64;
	// 831ABAC4: FDAA6B2E  fsel f13, f10, f12, f13
	ctx.f[13].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[13].f64 };
	// 831ABAC8: FC000B6E  fsel f0, f0, f13, f1
	ctx.f[0].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[1].f64 };
	// 831ABACC: FC2B006E  fsel f1, f11, f1, f0
	ctx.f[1].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[1].f64 } else { ctx.f[0].f64 };
	// 831ABAD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ABAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ABAD8 size=92
    let mut pc: u32 = 0x831ABAD8;
    'dispatch: loop {
        match pc {
            0x831ABAD8 => {
    //   block [0x831ABAD8..0x831ABB34)
	// 831ABAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831ABADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831ABAE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831ABAE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831ABAE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831ABAEC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831ABAF0: 419A0030  beq cr6, 0x831abb20
	if ctx.cr[6].eq {
	pc = 0x831ABB20; continue 'dispatch;
	}
	// 831ABAF4: 4BA25DE5  bl 0x82bd18d8
	ctx.lr = 0x831ABAF8;
	sub_82BD18D8(ctx, base);
	// 831ABAF8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831ABAFC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831ABB00: 4BA250F1  bl 0x82bd0bf0
	ctx.lr = 0x831ABB04;
	sub_82BD0BF0(ctx, base);
	// 831ABB04: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831ABB08: 40820018  bne 0x831abb20
	if !ctx.cr[0].eq {
	pc = 0x831ABB20; continue 'dispatch;
	}
	// 831ABB0C: 48005255  bl 0x831b0d60
	ctx.lr = 0x831ABB10;
	sub_831B0D60(ctx, base);
	// 831ABB10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831ABB14: 4BA221E5  bl 0x82bcdcf8
	ctx.lr = 0x831ABB18;
	sub_82BCDCF8(ctx, base);
	// 831ABB18: 480051D9  bl 0x831b0cf0
	ctx.lr = 0x831ABB1C;
	sub_831B0CF0(ctx, base);
	// 831ABB1C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 831ABB20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831ABB24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831ABB28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831ABB2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831ABB30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ABB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ABB38 size=12
    let mut pc: u32 = 0x831ABB38;
    'dispatch: loop {
        match pc {
            0x831ABB38 => {
    //   block [0x831ABB38..0x831ABB44)
	// 831ABB38: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831ABB3C: 806BEEE4  lwz r3, -0x111c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4380 as u32) ) } as u64;
	// 831ABB40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ABB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ABB48 size=176
    let mut pc: u32 = 0x831ABB48;
    'dispatch: loop {
        match pc {
            0x831ABB48 => {
    //   block [0x831ABB48..0x831ABBF8)
	// 831ABB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831ABB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831ABB50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831ABB54: 9421F500  stwu r1, -0xb00(r1)
	ea = ctx.r[1].u32.wrapping_add(-2816 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831ABB58: 3FE08335  lis r31, -0x7ccb
	ctx.r[31].s64 = -2093678592;
	// 831ABB5C: 817FEEE8  lwz r11, -0x1118(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4376 as u32) ) } as u64;
	// 831ABB60: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831ABB64: 4182000C  beq 0x831abb70
	if ctx.cr[0].eq {
	pc = 0x831ABB70; continue 'dispatch;
	}
	// 831ABB68: 3860000A  li r3, 0xa
	ctx.r[3].s64 = 10;
	// 831ABB6C: 4800754D  bl 0x831b30b8
	ctx.lr = 0x831ABB70;
	sub_831B30B8(ctx, base);
	// 831ABB70: 4800C651  bl 0x831b81c0
	ctx.lr = 0x831ABB74;
	sub_831B81C0(ctx, base);
	// 831ABB74: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831ABB78: 4182000C  beq 0x831abb84
	if ctx.cr[0].eq {
	pc = 0x831ABB84; continue 'dispatch;
	}
	// 831ABB7C: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 831ABB80: 4800C659  bl 0x831b81d8
	ctx.lr = 0x831ABB84;
	sub_831B81D8(ctx, base);
	// 831ABB84: 817FEEE8  lwz r11, -0x1118(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4376 as u32) ) } as u64;
	// 831ABB88: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831ABB8C: 41820064  beq 0x831abbf0
	if ctx.cr[0].eq {
	pc = 0x831ABBF0; continue 'dispatch;
	}
	// 831ABB90: 38A00A40  li r5, 0xa40
	ctx.r[5].s64 = 2624;
	// 831ABB94: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831ABB98: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 831ABB9C: 4BFFC645  bl 0x831a81e0
	ctx.lr = 0x831ABBA0;
	sub_831A81E0(ctx, base);
	// 831ABBA0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 831ABBA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831ABBA8: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 831ABBAC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831ABBB0: F92B0000  std r9, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 831ABBB4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831ABBB8: 4200FFF8  bdnz 0x831abbb0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831ABBB0; continue 'dispatch;
	}
	// 831ABBBC: 81410AF8  lwz r10, 0xaf8(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2808 as u32) ) } as u64;
	// 831ABBC0: 3D604000  lis r11, 0x4000
	ctx.r[11].s64 = 1073741824;
	// 831ABBC4: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 831ABBC8: 616B0015  ori r11, r11, 0x15
	ctx.r[11].u64 = ctx.r[11].u64 | 21;
	// 831ABBCC: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 831ABBD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ABBD4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 831ABBD8: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 831ABBDC: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 831ABBE0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 831ABBE4: 4BA222ED  bl 0x82bcded0
	ctx.lr = 0x831ABBE8;
	sub_82BCDED0(ctx, base);
	// 831ABBE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831ABBEC: 4BA223CD  bl 0x82bcdfb8
	ctx.lr = 0x831ABBF0;
	sub_82BCDFB8(ctx, base);
	// 831ABBF0: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 831ABBF4: 48000F9D  bl 0x831acb90
	ctx.lr = 0x831ABBF8;
	sub_831ACB90(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ABBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ABBF8 size=28
    let mut pc: u32 = 0x831ABBF8;
    'dispatch: loop {
        match pc {
            0x831ABBF8 => {
    //   block [0x831ABBF8..0x831ABC14)
	// 831ABBF8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 831ABBFC: 7C6B2038  and r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 & ctx.r[4].u64;
	// 831ABC00: 806AEEE8  lwz r3, -0x1118(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4376 as u32) ) } as u64;
	// 831ABC04: 7C692078  andc r9, r3, r4
	ctx.r[9].u64 = ctx.r[3].u64 & !ctx.r[4].u64;
	// 831ABC08: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 831ABC0C: 916AEEE8  stw r11, -0x1118(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4376 as u32), ctx.r[11].u32 ) };
	// 831ABC10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ABC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ABC18 size=40
    let mut pc: u32 = 0x831ABC18;
    'dispatch: loop {
        match pc {
            0x831ABC18 => {
    //   block [0x831ABC18..0x831ABC40)
	// 831ABC18: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 831ABC1C: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
	// 831ABC20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 831ABC24: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 831ABC28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831ABC2C: 99090000  stb r8, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 831ABC30: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831ABC34: 4200FFF8  bdnz 0x831abc2c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831ABC2C; continue 'dispatch;
	}
	// 831ABC38: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 831ABC3C: 48000020  b 0x831abc5c
	sub_831ABC40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ABC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ABC40 size=132
    let mut pc: u32 = 0x831ABC40;
    'dispatch: loop {
        match pc {
            0x831ABC40 => {
    //   block [0x831ABC40..0x831ABCC4)
	// 831ABC40: 5568E8FE  srwi r8, r11, 3
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831ABC44: 556B077E  clrlwi r11, r11, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 831ABC48: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831ABC4C: 7CE65830  slw r6, r7, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 831ABC50: 7CA848AE  lbzx r5, r8, r9
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 831ABC54: 7CC62B78  or r6, r6, r5
	ctx.r[6].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 831ABC58: 7CC849AE  stbx r6, r8, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32), ctx.r[6].u8) };
	// 831ABC5C: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831ABC60: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
	// 831ABC64: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831ABC68: 4082FFD8  bne 0x831abc40
	if !ctx.cr[0].eq {
	pc = 0x831ABC40; continue 'dispatch;
	}
	// 831ABC6C: 8941FFE0  lbz r10, -0x20(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) } as u64;
	// 831ABC70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831ABC74: 89030000  lbz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831ABC78: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 831ABC7C: 5506E8FE  srwi r6, r8, 3
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shr(3);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831ABC80: 9941FFE0  stb r10, -0x20(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[10].u8 ) };
	// 831ABC84: 5508077E  clrlwi r8, r8, 0x1d
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x00000007u64;
	// 831ABC88: 7CEA4030  slw r10, r7, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[7].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 831ABC8C: 7D2648AE  lbzx r9, r6, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 831ABC90: 7D4A4839  and. r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[9].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831ABC94: 40820028  bne 0x831abcbc
	if !ctx.cr[0].eq {
	pc = 0x831ABCBC; continue 'dispatch;
	}
	// 831ABC98: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831ABC9C: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 831ABCA0: 7D2B18AE  lbzx r9, r11, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 831ABCA4: 5528E8FE  srwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shr(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831ABCA8: 5529077E  clrlwi r9, r9, 0x1d
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	// 831ABCAC: 7CE94830  slw r9, r7, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[7].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 831ABCB0: 7D4850AE  lbzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 831ABCB4: 7D2A5039  and. r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831ABCB8: 4182FFE0  beq 0x831abc98
	if ctx.cr[0].eq {
	pc = 0x831ABC98; continue 'dispatch;
	}
	// 831ABCBC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 831ABCC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ABCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ABCC8 size=64
    let mut pc: u32 = 0x831ABCC8;
    'dispatch: loop {
        match pc {
            0x831ABCC8 => {
    //   block [0x831ABCC8..0x831ABD08)
	// 831ABCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831ABCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831ABCD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831ABCD4: 480013C5  bl 0x831ad098
	ctx.lr = 0x831ABCD8;
	sub_831AD098(ctx, base);
	// 831ABCD8: 3D600003  lis r11, 3
	ctx.r[11].s64 = 196608;
	// 831ABCDC: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831ABCE0: 616B43FD  ori r11, r11, 0x43fd
	ctx.r[11].u64 = ctx.r[11].u64 | 17405;
	// 831ABCE4: 7D6959D6  mullw r11, r9, r11
	ctx.r[11].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 831ABCE8: 3D6B0027  addis r11, r11, 0x27
	ctx.r[11].s64 = ctx.r[11].s64 + 2555904;
	// 831ABCEC: 396B9EC3  addi r11, r11, -0x613d
	ctx.r[11].s64 = ctx.r[11].s64 + -24893;
	// 831ABCF0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 831ABCF4: 5563847E  rlwinm r3, r11, 0x10, 0x11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 831ABCF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831ABCFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831ABD00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831ABD04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ABD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ABD08 size=60
    let mut pc: u32 = 0x831ABD08;
    'dispatch: loop {
        match pc {
            0x831ABD08 => {
    //   block [0x831ABD08..0x831ABD44)
	// 831ABD08: FD800E5E  fctidz f12, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].s64 = if ctx.f[1].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[1].f64.trunc() as i64 };
	// 831ABD0C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 831ABD10: FD600A10  fabs f11, f1
	ctx.f[11].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 831ABD14: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 831ABD18: C9ABE3A0  lfd f13, -0x1c60(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-7264 as u32) ) };
	// 831ABD1C: C80AC5D0  lfd f0, -0x3a30(r10)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-14896 as u32) ) };
	// 831ABD20: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 831ABD24: FC005828  fsub f0, f0, f11
	ctx.f[0].f64 = ctx.f[0].f64 - ctx.f[11].f64;
	// 831ABD28: FD605850  fneg f11, f11
	ctx.f[11].u64 = ctx.f[11].u64 ^ 0x8000_0000_0000_0000u64;
	// 831ABD2C: FD416028  fsub f10, f1, f12
	ctx.f[10].f64 = ctx.f[1].f64 - ctx.f[12].f64;
	// 831ABD30: FDAC6828  fsub f13, f12, f13
	ctx.f[13].f64 = ctx.f[12].f64 - ctx.f[13].f64;
	// 831ABD34: FDAA6B2E  fsel f13, f10, f12, f13
	ctx.f[13].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[13].f64 };
	// 831ABD38: FC000B6E  fsel f0, f0, f13, f1
	ctx.f[0].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[1].f64 };
	// 831ABD3C: FC2B006E  fsel f1, f11, f1, f0
	ctx.f[1].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[1].f64 } else { ctx.f[0].f64 };
	// 831ABD40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ABD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ABD48 size=184
    let mut pc: u32 = 0x831ABD48;
    'dispatch: loop {
        match pc {
            0x831ABD48 => {
    //   block [0x831ABD48..0x831ABE00)
	// 831ABD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831ABD4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831ABD50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831ABD54: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831ABD58: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831ABD5C: 409A0030  bne cr6, 0x831abd8c
	if !ctx.cr[6].eq {
	pc = 0x831ABD8C; continue 'dispatch;
	}
	// 831ABD60: 48005001  bl 0x831b0d60
	ctx.lr = 0x831ABD64;
	sub_831B0D60(ctx, base);
	// 831ABD64: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 831ABD68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831ABD6C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831ABD70: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831ABD74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831ABD78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831ABD7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ABD80: 48009091  bl 0x831b4e10
	ctx.lr = 0x831ABD84;
	sub_831B4E10(ctx, base);
	// 831ABD84: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831ABD88: 48000064  b 0x831abdec
	pc = 0x831ABDEC; continue 'dispatch;
	// 831ABD8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831ABD90: 419AFFD0  beq cr6, 0x831abd60
	if ctx.cr[6].eq {
	pc = 0x831ABD60; continue 'dispatch;
	}
	// 831ABD94: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 831ABD98: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 831ABD9C: 39400042  li r10, 0x42
	ctx.r[10].s64 = 66;
	// 831ABDA0: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 831ABDA4: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 831ABDA8: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 831ABDAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831ABDB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831ABDB4: 480084D5  bl 0x831b4288
	ctx.lr = 0x831ABDB8;
	sub_831B4288(ctx, base);
	// 831ABDB8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831ABDBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831ABDC0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831ABDC4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831ABDC8: 41800014  blt 0x831abddc
	if ctx.cr[0].lt {
	pc = 0x831ABDDC; continue 'dispatch;
	}
	// 831ABDCC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831ABDD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831ABDD4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 831ABDD8: 48000010  b 0x831abde8
	pc = 0x831ABDE8; continue 'dispatch;
	// 831ABDDC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831ABDE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ABDE4: 4800823D  bl 0x831b4020
	ctx.lr = 0x831ABDE8;
	sub_831B4020(ctx, base);
	// 831ABDE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831ABDEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831ABDF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831ABDF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831ABDF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831ABDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ABE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ABE00 size=12
    let mut pc: u32 = 0x831ABE00;
    'dispatch: loop {
        match pc {
            0x831ABE00 => {
    //   block [0x831ABE00..0x831ABE0C)
	// 831ABE00: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 831ABE04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831ABE08: 4BFFFF40  b 0x831abd48
	sub_831ABD48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ABE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ABE10 size=548
    let mut pc: u32 = 0x831ABE10;
    'dispatch: loop {
        match pc {
            0x831ABE10 => {
    //   block [0x831ABE10..0x831AC034)
	// 831ABE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831ABE14: 4BFFC359  bl 0x831a816c
	ctx.lr = 0x831ABE18;
	sub_831A8130(ctx, base);
	// 831ABE18: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 831ABE1C: 4BFFCC5D  bl 0x831a8a78
	ctx.lr = 0x831ABE20;
	sub_831A8A40(ctx, base);
	// 831ABE20: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831ABE24: 3D60C007  lis r11, -0x3ff9
	ctx.r[11].s64 = -1073283072;
	// 831ABE28: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 831ABE2C: 386000F8  li r3, 0xf8
	ctx.r[3].s64 = 248;
	// 831ABE30: FFE01090  fmr f31, f2
	ctx.f[31].f64 = ctx.f[2].f64;
	// 831ABE34: 617DFEFF  ori r29, r11, 0xfeff
	ctx.r[29].u64 = ctx.r[11].u64 | 65279;
	// 831ABE38: DBC100B0  stfd f30, 0xb0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.f[30].u64 ) };
	// 831ABE3C: DBE100B8  stfd f31, 0xb8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.f[31].u64 ) };
	// 831ABE40: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 831ABE44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831ABE48: 4800D159  bl 0x831b8fa0
	ctx.lr = 0x831ABE4C;
	sub_831B8FA0(ctx, base);
	// 831ABE4C: A16100B0  lhz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 831ABE50: A12100B8  lhz r9, 0xb8(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) } as u64;
	// 831ABE54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831ABE58: 556A0476  rlwinm r10, r11, 0, 0x11, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831ABE5C: 2B0A7FF0  cmplwi cr6, r10, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32752 as u32, &mut ctx.xer);
	// 831ABE60: 419A0120  beq cr6, 0x831abf80
	if ctx.cr[6].eq {
	pc = 0x831ABF80; continue 'dispatch;
	}
	// 831ABE64: 552A0476  rlwinm r10, r9, 0, 0x11, 0x1b
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 831ABE68: 2B0A7FF0  cmplwi cr6, r10, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32752 as u32, &mut ctx.xer);
	// 831ABE6C: 419A0114  beq cr6, 0x831abf80
	if ctx.cr[6].eq {
	pc = 0x831ABF80; continue 'dispatch;
	}
	// 831ABE70: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 831ABE74: CB8BD228  lfd f28, -0x2dd8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11736 as u32) ) };
	// 831ABE78: FF1EE000  fcmpu cr6, f30, f28
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[28].f64);
	// 831ABE7C: 40980008  bge cr6, 0x831abe84
	if !ctx.cr[6].lt {
	pc = 0x831ABE84; continue 'dispatch;
	}
	// 831ABE80: FFC0F050  fneg f30, f30
	ctx.f[30].u64 = ctx.f[30].u64 ^ 0x8000_0000_0000_0000u64;
	// 831ABE84: FF1FE000  fcmpu cr6, f31, f28
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[28].f64);
	// 831ABE88: 40980008  bge cr6, 0x831abe90
	if !ctx.cr[6].lt {
	pc = 0x831ABE90; continue 'dispatch;
	}
	// 831ABE8C: FFE0F850  fneg f31, f31
	ctx.f[31].u64 = ctx.f[31].u64 ^ 0x8000_0000_0000_0000u64;
	// 831ABE90: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 831ABE94: 4099000C  ble cr6, 0x831abea0
	if !ctx.cr[6].gt {
	pc = 0x831ABEA0; continue 'dispatch;
	}
	// 831ABE98: FFA0F890  fmr f29, f31
	ctx.f[29].f64 = ctx.f[31].f64;
	// 831ABE9C: 48000008  b 0x831abea4
	pc = 0x831ABEA4; continue 'dispatch;
	// 831ABEA0: FFA0F090  fmr f29, f30
	ctx.f[29].f64 = ctx.f[30].f64;
	// 831ABEA4: FF1DE000  fcmpu cr6, f29, f28
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[28].f64);
	// 831ABEA8: 409A0018  bne cr6, 0x831abec0
	if !ctx.cr[6].eq {
	pc = 0x831ABEC0; continue 'dispatch;
	}
	// 831ABEAC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831ABEB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831ABEB4: 4800D0ED  bl 0x831b8fa0
	ctx.lr = 0x831ABEB8;
	sub_831B8FA0(ctx, base);
	// 831ABEB8: FC20E090  fmr f1, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[28].f64;
	// 831ABEBC: 48000168  b 0x831ac024
	pc = 0x831AC024; continue 'dispatch;
	// 831ABEC0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 831ABEC4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 831ABEC8: C80BE3A0  lfd f0, -0x1c60(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-7264 as u32) ) };
	// 831ABECC: FC00E824  fdiv f0, f0, f29
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[29].f64;
	// 831ABED0: FFE007F2  fmul f31, f0, f31
	ctx.f[31].f64 = ctx.f[0].f64 * ctx.f[31].f64;
	// 831ABED4: FFC007B2  fmul f30, f0, f30
	ctx.f[30].f64 = ctx.f[0].f64 * ctx.f[30].f64;
	// 831ABED8: FC1F07F2  fmul f0, f31, f31
	ctx.f[0].f64 = ctx.f[31].f64 * ctx.f[31].f64;
	// 831ABEDC: FC1E07BA  fmadd f0, f30, f30, f0
	ctx.f[0].f64 = ctx.f[30].f64 * ctx.f[30].f64 + ctx.f[0].f64;
	// 831ABEE0: FC20002C  fsqrt f1, f0
	ctx.f[1].f64 = (ctx.f[0].f64).sqrt();
	// 831ABEE4: 480090FD  bl 0x831b4fe0
	ctx.lr = 0x831ABEE8;
	sub_831B4FE0(ctx, base);
	// 831ABEE8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831ABEEC: FF800890  fmr f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[1].f64;
	// 831ABEF0: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 831ABEF4: 480090ED  bl 0x831b4fe0
	ctx.lr = 0x831ABEF8;
	sub_831B4FE0(ctx, base);
	// 831ABEF8: FFBC0072  fmul f29, f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[28].f64 * ctx.f[1].f64;
	// 831ABEFC: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 831ABF00: 48009009  bl 0x831b4f08
	ctx.lr = 0x831ABF04;
	sub_831B4F08(ctx, base);
	// 831ABF04: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831ABF08: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 831ABF0C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831ABF10: 7D435214  add r10, r3, r10
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[10].u64;
	// 831ABF14: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831ABF18: 2F040400  cmpwi cr6, r4, 0x400
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1024, &mut ctx.xer);
	// 831ABF1C: 40990018  ble cr6, 0x831abf34
	if !ctx.cr[6].gt {
	pc = 0x831ABF34; continue 'dispatch;
	}
	// 831ABF20: 3884FA00  addi r4, r4, -0x600
	ctx.r[4].s64 = ctx.r[4].s64 + -1536;
	// 831ABF24: 48008FBD  bl 0x831b4ee0
	ctx.lr = 0x831ABF28;
	sub_831B4EE0(ctx, base);
	// 831ABF28: FC600890  fmr f3, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[1].f64;
	// 831ABF2C: 38600011  li r3, 0x11
	ctx.r[3].s64 = 17;
	// 831ABF30: 480000A0  b 0x831abfd0
	pc = 0x831ABFD0; continue 'dispatch;
	// 831ABF34: 2F04FC03  cmpwi cr6, r4, -0x3fd
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1021, &mut ctx.xer);
	// 831ABF38: 40980018  bge cr6, 0x831abf50
	if !ctx.cr[6].lt {
	pc = 0x831ABF50; continue 'dispatch;
	}
	// 831ABF3C: 38840600  addi r4, r4, 0x600
	ctx.r[4].s64 = ctx.r[4].s64 + 1536;
	// 831ABF40: 48008FA1  bl 0x831b4ee0
	ctx.lr = 0x831ABF44;
	sub_831B4EE0(ctx, base);
	// 831ABF44: FC600890  fmr f3, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[1].f64;
	// 831ABF48: 38600012  li r3, 0x12
	ctx.r[3].s64 = 18;
	// 831ABF4C: 48000084  b 0x831abfd0
	pc = 0x831ABFD0; continue 'dispatch;
	// 831ABF50: 48008F91  bl 0x831b4ee0
	ctx.lr = 0x831ABF54;
	sub_831B4EE0(ctx, base);
	// 831ABF54: 57EB0739  rlwinm. r11, r31, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831ABF58: FFA00890  fmr f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[1].f64;
	// 831ABF5C: 41820018  beq 0x831abf74
	if ctx.cr[0].eq {
	pc = 0x831ABF74; continue 'dispatch;
	}
	// 831ABF60: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831ABF64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831ABF68: 4800D039  bl 0x831b8fa0
	ctx.lr = 0x831ABF6C;
	sub_831B8FA0(ctx, base);
	// 831ABF6C: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 831ABF70: 480000B4  b 0x831ac024
	pc = 0x831AC024; continue 'dispatch;
	// 831ABF74: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 831ABF78: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 831ABF7C: 48000054  b 0x831abfd0
	pc = 0x831ABFD0; continue 'dispatch;
	// 831ABF80: 556A0478  rlwinm r10, r11, 0, 0x11, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831ABF84: 2B0A7FF0  cmplwi cr6, r10, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32752 as u32, &mut ctx.xer);
	// 831ABF88: 409A001C  bne cr6, 0x831abfa4
	if !ctx.cr[6].eq {
	pc = 0x831ABFA4; continue 'dispatch;
	}
	// 831ABF8C: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 831ABF90: 556B037F  clrlwi. r11, r11, 0xd
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0007FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831ABF94: 40820034  bne 0x831abfc8
	if !ctx.cr[0].eq {
	pc = 0x831ABFC8; continue 'dispatch;
	}
	// 831ABF98: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 831ABF9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831ABFA0: 409A0028  bne cr6, 0x831abfc8
	if !ctx.cr[6].eq {
	pc = 0x831ABFC8; continue 'dispatch;
	}
	// 831ABFA4: 552B0478  rlwinm r11, r9, 0, 0x11, 0x1c
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 831ABFA8: 2B0B7FF0  cmplwi cr6, r11, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32752 as u32, &mut ctx.xer);
	// 831ABFAC: 409A003C  bne cr6, 0x831abfe8
	if !ctx.cr[6].eq {
	pc = 0x831ABFE8; continue 'dispatch;
	}
	// 831ABFB0: 812100B8  lwz r9, 0xb8(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) } as u64;
	// 831ABFB4: 5529037F  clrlwi. r9, r9, 0xd
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0007FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831ABFB8: 40820010  bne 0x831abfc8
	if !ctx.cr[0].eq {
	pc = 0x831ABFC8; continue 'dispatch;
	}
	// 831ABFBC: 812100BC  lwz r9, 0xbc(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 831ABFC0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 831ABFC4: 419A0024  beq cr6, 0x831abfe8
	if ctx.cr[6].eq {
	pc = 0x831ABFE8; continue 'dispatch;
	}
	// 831ABFC8: FC7EF82A  fadd f3, f30, f31
	ctx.f[3].f64 = ctx.f[30].f64 + ctx.f[31].f64;
	// 831ABFCC: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 831ABFD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831ABFD4: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 831ABFD8: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 831ABFDC: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 831ABFE0: 4800CED1  bl 0x831b8eb0
	ctx.lr = 0x831ABFE4;
	sub_831B8EB0(ctx, base);
	// 831ABFE4: 48000040  b 0x831ac024
	pc = 0x831AC024; continue 'dispatch;
	// 831ABFE8: 2B0A7FF8  cmplwi cr6, r10, 0x7ff8
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32760 as u32, &mut ctx.xer);
	// 831ABFEC: 419A0024  beq cr6, 0x831ac010
	if ctx.cr[6].eq {
	pc = 0x831AC010; continue 'dispatch;
	}
	// 831ABFF0: 2B0B7FF8  cmplwi cr6, r11, 0x7ff8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32760 as u32, &mut ctx.xer);
	// 831ABFF4: 419A001C  beq cr6, 0x831ac010
	if ctx.cr[6].eq {
	pc = 0x831AC010; continue 'dispatch;
	}
	// 831ABFF8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831ABFFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831AC000: 4800CFA1  bl 0x831b8fa0
	ctx.lr = 0x831AC004;
	sub_831B8FA0(ctx, base);
	// 831AC004: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AC008: C82BF468  lfd f1, -0xb98(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-2968 as u32) ) };
	// 831AC00C: 48000018  b 0x831ac024
	pc = 0x831AC024; continue 'dispatch;
	// 831AC010: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 831AC014: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 831AC018: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831AC01C: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 831AC020: 4800CD39  bl 0x831b8d58
	ctx.lr = 0x831AC024;
	sub_831B8D58(ctx, base);
	// 831AC024: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831AC028: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 831AC02C: 4BFFCA99  bl 0x831a8ac4
	ctx.lr = 0x831AC030;
	sub_831A8A8C(ctx, base);
	// 831AC030: 4BFFC18C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC038 size=8
    let mut pc: u32 = 0x831AC038;
    'dispatch: loop {
        match pc {
            0x831AC038 => {
    //   block [0x831AC038..0x831AC040)
	// 831AC038: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 831AC03C: 4BFFFDD4  b 0x831abe10
	sub_831ABE10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AC040 size=304
    let mut pc: u32 = 0x831AC040;
    'dispatch: loop {
        match pc {
            0x831AC040 => {
    //   block [0x831AC040..0x831AC170)
	// 831AC040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AC044: 4BFFC125  bl 0x831a8168
	ctx.lr = 0x831AC048;
	sub_831A8130(ctx, base);
	// 831AC048: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 831AC04C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AC050: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831AC054: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 831AC058: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831AC05C: DBE100A0  stfd f31, 0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.f[31].u64 ) };
	// 831AC060: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AC064: 4800CF3D  bl 0x831b8fa0
	ctx.lr = 0x831AC068;
	sub_831B8FA0(ctx, base);
	// 831AC068: 3D40C007  lis r10, -0x3ff9
	ctx.r[10].s64 = -1073283072;
	// 831AC06C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AC070: 615CFEFF  ori r28, r10, 0xfeff
	ctx.r[28].u64 = ctx.r[10].u64 | 65279;
	// 831AC074: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831AC078: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831AC07C: 806BEEEC  lwz r3, -0x1114(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4372 as u32) ) } as u64;
	// 831AC080: 4800CF21  bl 0x831b8fa0
	ctx.lr = 0x831AC084;
	sub_831B8FA0(ctx, base);
	// 831AC084: A16100A0  lhz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 831AC088: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 831AC08C: 556A0476  rlwinm r10, r11, 0, 0x11, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AC090: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 831AC094: 2B0A7FF0  cmplwi cr6, r10, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32752 as u32, &mut ctx.xer);
	// 831AC098: 409A0088  bne cr6, 0x831ac120
	if !ctx.cr[6].eq {
	pc = 0x831AC120; continue 'dispatch;
	}
	// 831AC09C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AC0A0: C80BF470  lfd f0, -0xb90(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-2960 as u32) ) };
	// 831AC0A4: D81F0000  stfd f0, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.f[0].u64 ) };
	// 831AC0A8: 48008EB1  bl 0x831b4f58
	ctx.lr = 0x831AC0AC;
	sub_831B4F58(ctx, base);
	// 831AC0AC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831AC0B0: 40810048  ble 0x831ac0f8
	if !ctx.cr[0].gt {
	pc = 0x831AC0F8; continue 'dispatch;
	}
	// 831AC0B4: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 831AC0B8: 40990024  ble cr6, 0x831ac0dc
	if !ctx.cr[6].gt {
	pc = 0x831AC0DC; continue 'dispatch;
	}
	// 831AC0BC: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 831AC0C0: 409A0038  bne cr6, 0x831ac0f8
	if !ctx.cr[6].eq {
	pc = 0x831AC0F8; continue 'dispatch;
	}
	// 831AC0C4: DBFF0000  stfd f31, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.f[31].u64 ) };
	// 831AC0C8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 831AC0CC: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 831AC0D0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 831AC0D4: 4800CBFD  bl 0x831b8cd0
	ctx.lr = 0x831AC0D8;
	sub_831B8CD0(ctx, base);
	// 831AC0D8: 4800008C  b 0x831ac164
	pc = 0x831AC164; continue 'dispatch;
	// 831AC0DC: DBFF0000  stfd f31, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.f[31].u64 ) };
	// 831AC0E0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 831AC0E4: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 831AC0E8: C82BD228  lfd f1, -0x2dd8(r11)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11736 as u32) ) };
	// 831AC0EC: 4800245D  bl 0x831ae548
	ctx.lr = 0x831AC0F0;
	sub_831AE548(ctx, base);
	// 831AC0F0: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 831AC0F4: 48000060  b 0x831ac154
	pc = 0x831AC154; continue 'dispatch;
	// 831AC0F8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 831AC0FC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 831AC100: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 831AC104: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 831AC108: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 831AC10C: C80BE3A0  lfd f0, -0x1c60(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-7264 as u32) ) };
	// 831AC110: FC5F002A  fadd f2, f31, f0
	ctx.f[2].f64 = ctx.f[31].f64 + ctx.f[0].f64;
	// 831AC114: D85F0000  stfd f2, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.f[2].u64 ) };
	// 831AC118: 4800CCC1  bl 0x831b8dd8
	ctx.lr = 0x831AC11C;
	sub_831B8DD8(ctx, base);
	// 831AC11C: 48000048  b 0x831ac164
	pc = 0x831AC164; continue 'dispatch;
	// 831AC120: 4800CF31  bl 0x831b9050
	ctx.lr = 0x831AC124;
	sub_831B9050(ctx, base);
	// 831AC124: FFFF0828  fsub f31, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[31].f64 - ctx.f[1].f64;
	// 831AC128: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 831AC12C: D83F0000  stfd f1, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.f[1].u64 ) };
	// 831AC130: DBE10050  stfd f31, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[31].u64 ) };
	// 831AC134: C80BD228  lfd f0, -0x2dd8(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11736 as u32) ) };
	// 831AC138: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 831AC13C: 409A0018  bne cr6, 0x831ac154
	if !ctx.cr[6].eq {
	pc = 0x831AC154; continue 'dispatch;
	}
	// 831AC140: A1610050  lhz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831AC144: 57AA0420  rlwinm r10, r29, 0, 0x10, 0x10
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	// 831AC148: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 831AC14C: B1610050  sth r11, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 831AC150: CBE10050  lfd f31, 0x50(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831AC154: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831AC158: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831AC15C: 4800CE45  bl 0x831b8fa0
	ctx.lr = 0x831AC160;
	sub_831B8FA0(ctx, base);
	// 831AC160: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 831AC164: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831AC168: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 831AC16C: 4BFFC04C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AC170 size=100
    let mut pc: u32 = 0x831AC170;
    'dispatch: loop {
        match pc {
            0x831AC170 => {
    //   block [0x831AC170..0x831AC1D4)
	// 831AC170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AC174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831AC178: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831AC17C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AC180: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831AC184: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831AC188: 48019DD9  bl 0x831c5f60
	ctx.lr = 0x831AC18C;
	sub_831C5F60(ctx, base);
	// 831AC18C: 3D80FE62  lis r12, -0x19e
	ctx.r[12].s64 = -27131904;
	// 831AC190: E9410050  ld r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831AC194: 3D600098  lis r11, 0x98
	ctx.r[11].s64 = 9961472;
	// 831AC198: 618C4E21  ori r12, r12, 0x4e21
	ctx.r[12].u64 = ctx.r[12].u64 | 20001;
	// 831AC19C: 616B9680  ori r11, r11, 0x9680
	ctx.r[11].u64 = ctx.r[11].u64 | 38528;
	// 831AC1A0: 798C07C6  sldi r12, r12, 0x20
	ctx.r[12].u64 = ctx.r[12].u64.wrapping_shl(32);
	ctx.r[12].u32 = ctx.r[12].u64 as u32;
	// 831AC1A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831AC1A8: 658C2AC1  oris r12, r12, 0x2ac1
	ctx.r[12].u64 = ctx.r[12].u64 | 717291520;
	// 831AC1AC: 618C8000  ori r12, r12, 0x8000
	ctx.r[12].u64 = ctx.r[12].u64 | 32768;
	// 831AC1B0: 7D4A6214  add r10, r10, r12
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[12].u64;
	// 831AC1B4: 7C6A5B92  divdu r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 / ctx.r[11].u64;
	// 831AC1B8: 419A0008  beq cr6, 0x831ac1c0
	if ctx.cr[6].eq {
	pc = 0x831AC1C0; continue 'dispatch;
	}
	// 831AC1BC: F87F0000  std r3, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 831AC1C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831AC1C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831AC1C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831AC1CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831AC1D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AC1D8 size=724
    let mut pc: u32 = 0x831AC1D8;
    'dispatch: loop {
        match pc {
            0x831AC1D8 => {
    //   block [0x831AC1D8..0x831AC4AC)
	// 831AC1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AC1DC: 4BFFBF7D  bl 0x831a8158
	ctx.lr = 0x831AC1E0;
	sub_831A8130(ctx, base);
	// 831AC1E0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AC1E4: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 831AC1E8: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 831AC1EC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 831AC1F0: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 831AC1F4: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 831AC1F8: 419A0008  beq cr6, 0x831ac200
	if ctx.cr[6].eq {
	pc = 0x831AC200; continue 'dispatch;
	}
	// 831AC1FC: 93580000  stw r26, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 831AC200: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 831AC204: 409A0030  bne cr6, 0x831ac234
	if !ctx.cr[6].eq {
	pc = 0x831AC234; continue 'dispatch;
	}
	// 831AC208: 48004B59  bl 0x831b0d60
	ctx.lr = 0x831AC20C;
	sub_831B0D60(ctx, base);
	// 831AC20C: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 831AC210: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831AC214: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831AC218: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831AC21C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831AC220: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831AC224: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AC228: 48008BE9  bl 0x831b4e10
	ctx.lr = 0x831AC22C;
	sub_831B4E10(ctx, base);
	// 831AC22C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AC230: 48000274  b 0x831ac4a4
	pc = 0x831AC4A4; continue 'dispatch;
	// 831AC234: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 831AC238: 419A0014  beq cr6, 0x831ac24c
	if ctx.cr[6].eq {
	pc = 0x831AC24C; continue 'dispatch;
	}
	// 831AC23C: 2F1B0002  cmpwi cr6, r27, 2
	ctx.cr[6].compare_i32(ctx.r[27].s32, 2, &mut ctx.xer);
	// 831AC240: 4198FFC8  blt cr6, 0x831ac208
	if ctx.cr[6].lt {
	pc = 0x831AC208; continue 'dispatch;
	}
	// 831AC244: 2F1B0024  cmpwi cr6, r27, 0x24
	ctx.cr[6].compare_i32(ctx.r[27].s32, 36, &mut ctx.xer);
	// 831AC248: 4199FFC0  bgt cr6, 0x831ac208
	if ctx.cr[6].gt {
	pc = 0x831AC208; continue 'dispatch;
	}
	// 831AC24C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AC250: 8BFA0000  lbz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AC254: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 831AC258: 3BBA0001  addi r29, r26, 1
	ctx.r[29].s64 = ctx.r[26].s64 + 1;
	// 831AC25C: 3BCBF580  addi r30, r11, -0xa80
	ctx.r[30].s64 = ctx.r[11].s64 + -2688;
	// 831AC260: 814BF580  lwz r10, -0xa80(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2688 as u32) ) } as u64;
	// 831AC264: 816A00AC  lwz r11, 0xac(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(172 as u32) ) } as u64;
	// 831AC268: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831AC26C: 4099001C  ble cr6, 0x831ac288
	if !ctx.cr[6].gt {
	pc = 0x831AC288; continue 'dispatch;
	}
	// 831AC270: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 831AC274: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 831AC278: 57E3063E  clrlwi r3, r31, 0x18
	ctx.r[3].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 831AC27C: 48004C05  bl 0x831b0e80
	ctx.lr = 0x831AC280;
	sub_831B0E80(ctx, base);
	// 831AC280: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AC284: 48000014  b 0x831ac298
	pc = 0x831AC298; continue 'dispatch;
	// 831AC288: 816A00C8  lwz r11, 0xc8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(200 as u32) ) } as u64;
	// 831AC28C: 57E90DFC  rlwinm r9, r31, 1, 0x17, 0x1e
	ctx.r[9].u64 = ctx.r[31].u32 as u64 & 0x7FFFFFFFu64;
	// 831AC290: 7D695A2E  lhzx r11, r9, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831AC294: 55630738  rlwinm r3, r11, 0, 0x1c, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AC298: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831AC29C: 419A0010  beq cr6, 0x831ac2ac
	if ctx.cr[6].eq {
	pc = 0x831AC2AC; continue 'dispatch;
	}
	// 831AC2A0: 8BFD0000  lbz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AC2A4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 831AC2A8: 4BFFFFBC  b 0x831ac264
	pc = 0x831AC264; continue 'dispatch;
	// 831AC2AC: 7FEB0774  extsb r11, r31
	ctx.r[11].s64 = ctx.r[31].s8 as i64;
	// 831AC2B0: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 831AC2B4: 409A000C  bne cr6, 0x831ac2c0
	if !ctx.cr[6].eq {
	pc = 0x831AC2C0; continue 'dispatch;
	}
	// 831AC2B8: 63390002  ori r25, r25, 2
	ctx.r[25].u64 = ctx.r[25].u64 | 2;
	// 831AC2BC: 4800000C  b 0x831ac2c8
	pc = 0x831AC2C8; continue 'dispatch;
	// 831AC2C0: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 831AC2C4: 409A000C  bne cr6, 0x831ac2d0
	if !ctx.cr[6].eq {
	pc = 0x831AC2D0; continue 'dispatch;
	}
	// 831AC2C8: 8BFD0000  lbz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AC2CC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 831AC2D0: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 831AC2D4: 409A003C  bne cr6, 0x831ac310
	if !ctx.cr[6].eq {
	pc = 0x831AC310; continue 'dispatch;
	}
	// 831AC2D8: 7FEB0774  extsb r11, r31
	ctx.r[11].s64 = ctx.r[31].s8 as i64;
	// 831AC2DC: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 831AC2E0: 419A000C  beq cr6, 0x831ac2ec
	if ctx.cr[6].eq {
	pc = 0x831AC2EC; continue 'dispatch;
	}
	// 831AC2E4: 3B60000A  li r27, 0xa
	ctx.r[27].s64 = 10;
	// 831AC2E8: 48000060  b 0x831ac348
	pc = 0x831AC348; continue 'dispatch;
	// 831AC2EC: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AC2F0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 831AC2F4: 2F0B0078  cmpwi cr6, r11, 0x78
	ctx.cr[6].compare_i32(ctx.r[11].s32, 120, &mut ctx.xer);
	// 831AC2F8: 419A0014  beq cr6, 0x831ac30c
	if ctx.cr[6].eq {
	pc = 0x831AC30C; continue 'dispatch;
	}
	// 831AC2FC: 2F0B0058  cmpwi cr6, r11, 0x58
	ctx.cr[6].compare_i32(ctx.r[11].s32, 88, &mut ctx.xer);
	// 831AC300: 419A000C  beq cr6, 0x831ac30c
	if ctx.cr[6].eq {
	pc = 0x831AC30C; continue 'dispatch;
	}
	// 831AC304: 3B600008  li r27, 8
	ctx.r[27].s64 = 8;
	// 831AC308: 48000040  b 0x831ac348
	pc = 0x831AC348; continue 'dispatch;
	// 831AC30C: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 831AC310: 2F1B0010  cmpwi cr6, r27, 0x10
	ctx.cr[6].compare_i32(ctx.r[27].s32, 16, &mut ctx.xer);
	// 831AC314: 409A0034  bne cr6, 0x831ac348
	if !ctx.cr[6].eq {
	pc = 0x831AC348; continue 'dispatch;
	}
	// 831AC318: 7FEB0774  extsb r11, r31
	ctx.r[11].s64 = ctx.r[31].s8 as i64;
	// 831AC31C: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 831AC320: 409A0028  bne cr6, 0x831ac348
	if !ctx.cr[6].eq {
	pc = 0x831AC348; continue 'dispatch;
	}
	// 831AC324: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AC328: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 831AC32C: 2F0B0078  cmpwi cr6, r11, 0x78
	ctx.cr[6].compare_i32(ctx.r[11].s32, 120, &mut ctx.xer);
	// 831AC330: 419A000C  beq cr6, 0x831ac33c
	if ctx.cr[6].eq {
	pc = 0x831AC33C; continue 'dispatch;
	}
	// 831AC334: 2F0B0058  cmpwi cr6, r11, 0x58
	ctx.cr[6].compare_i32(ctx.r[11].s32, 88, &mut ctx.xer);
	// 831AC338: 409A0010  bne cr6, 0x831ac348
	if !ctx.cr[6].eq {
	pc = 0x831AC348; continue 'dispatch;
	}
	// 831AC33C: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 831AC340: 8BFD0001  lbz r31, 1(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(1 as u32) ) } as u64;
	// 831AC344: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 831AC348: 7F6907B4  extsw r9, r27
	ctx.r[9].s64 = ctx.r[27].s32 as i64;
	// 831AC34C: 80EA00C8  lwz r7, 0xc8(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(200 as u32) ) } as u64;
	// 831AC350: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831AC354: 08C90000  tdi 6, r9, 0
	// tdi: trap doubleword immediate — TODO: implement trap semantics
	// 831AC358: 7D0B4B92  divdu r8, r11, r9
	ctx.r[8].u64 = ctx.r[11].u64 / ctx.r[9].u64;
	// 831AC35C: 57EB0DFC  rlwinm r11, r31, 1, 0x17, 0x1e
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x7FFFFFFFu64;
	// 831AC360: 7D6B3A2E  lhzx r11, r11, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 831AC364: 556A077B  rlwinm. r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AC368: 41820010  beq 0x831ac378
	if ctx.cr[0].eq {
	pc = 0x831AC378; continue 'dispatch;
	}
	// 831AC36C: 7FEB0774  extsb r11, r31
	ctx.r[11].s64 = ctx.r[31].s8 as i64;
	// 831AC370: 396BFFD0  addi r11, r11, -0x30
	ctx.r[11].s64 = ctx.r[11].s64 + -48;
	// 831AC374: 4800002C  b 0x831ac3a0
	pc = 0x831AC3A0; continue 'dispatch;
	// 831AC378: 716B0103  andi. r11, r11, 0x103
	ctx.r[11].u64 = ctx.r[11].u64 & 259;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AC37C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831AC380: 41820064  beq 0x831ac3e4
	if ctx.cr[0].eq {
	pc = 0x831AC3E4; continue 'dispatch;
	}
	// 831AC384: 7FEB0774  extsb r11, r31
	ctx.r[11].s64 = ctx.r[31].s8 as i64;
	// 831AC388: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 831AC38C: 41980010  blt cr6, 0x831ac39c
	if ctx.cr[6].lt {
	pc = 0x831AC39C; continue 'dispatch;
	}
	// 831AC390: 2F0B007A  cmpwi cr6, r11, 0x7a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 122, &mut ctx.xer);
	// 831AC394: 41990008  bgt cr6, 0x831ac39c
	if ctx.cr[6].gt {
	pc = 0x831AC39C; continue 'dispatch;
	}
	// 831AC398: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 831AC39C: 396BFFC9  addi r11, r11, -0x37
	ctx.r[11].s64 = ctx.r[11].s64 + -55;
	// 831AC3A0: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 831AC3A4: 40980040  bge cr6, 0x831ac3e4
	if !ctx.cr[6].lt {
	pc = 0x831AC3E4; continue 'dispatch;
	}
	// 831AC3A8: 63390008  ori r25, r25, 8
	ctx.r[25].u64 = ctx.r[25].u64 | 8;
	// 831AC3AC: 7F3C4040  cmpld cr6, r28, r8
	ctx.cr[6].compare_u64(ctx.r[28].u64, ctx.r[8].u64, &mut ctx.xer);
	// 831AC3B0: 41980054  blt cr6, 0x831ac404
	if ctx.cr[6].lt {
	pc = 0x831AC404; continue 'dispatch;
	}
	// 831AC3B4: 409A0024  bne cr6, 0x831ac3d8
	if !ctx.cr[6].eq {
	pc = 0x831AC3D8; continue 'dispatch;
	}
	// 831AC3B8: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 831AC3BC: 79660020  clrldi r6, r11, 0x20
	ctx.r[6].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 831AC3C0: 7CAA4B92  divdu r5, r10, r9
	ctx.r[5].u64 = ctx.r[10].u64 / ctx.r[9].u64;
	// 831AC3C4: 08C90000  tdi 6, r9, 0
	// tdi: trap doubleword immediate — TODO: implement trap semantics
	// 831AC3C8: 7CA549D2  mulld r5, r5, r9
	ctx.r[5].s64 = ctx.r[5].s64 * ctx.r[9].s64;
	// 831AC3CC: 7D455050  subf r10, r5, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[5].s64;
	// 831AC3D0: 7F265040  cmpld cr6, r6, r10
	ctx.cr[6].compare_u64(ctx.r[6].u64, ctx.r[10].u64, &mut ctx.xer);
	// 831AC3D4: 40990030  ble cr6, 0x831ac404
	if !ctx.cr[6].gt {
	pc = 0x831AC404; continue 'dispatch;
	}
	// 831AC3D8: 63390004  ori r25, r25, 4
	ctx.r[25].u64 = ctx.r[25].u64 | 4;
	// 831AC3DC: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 831AC3E0: 409A0030  bne cr6, 0x831ac410
	if !ctx.cr[6].eq {
	pc = 0x831AC410; continue 'dispatch;
	}
	// 831AC3E4: 572B0739  rlwinm. r11, r25, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AC3E8: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 831AC3EC: 40820030  bne 0x831ac41c
	if !ctx.cr[0].eq {
	pc = 0x831AC41C; continue 'dispatch;
	}
	// 831AC3F0: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 831AC3F4: 419A0008  beq cr6, 0x831ac3fc
	if ctx.cr[6].eq {
	pc = 0x831AC3FC; continue 'dispatch;
	}
	// 831AC3F8: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 831AC3FC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 831AC400: 48000088  b 0x831ac488
	pc = 0x831AC488; continue 'dispatch;
	// 831AC404: 7D49E1D2  mulld r10, r9, r28
	ctx.r[10].s64 = ctx.r[9].s64 * ctx.r[28].s64;
	// 831AC408: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 831AC40C: 7F8A5A14  add r28, r10, r11
	ctx.r[28].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831AC410: 8BFD0000  lbz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AC414: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 831AC418: 4BFFFF44  b 0x831ac35c
	pc = 0x831AC35C; continue 'dispatch;
	// 831AC41C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 831AC420: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 831AC424: 572B077B  rlwinm. r11, r25, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AC428: 795F0040  clrldi r31, r10, 1
	ctx.r[31].u64 = ctx.r[10].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 831AC42C: 793EFFE6  rldicr r30, r9, 0x3f, 0x3f
	ctx.r[30].u64 = (ctx.r[9].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 831AC430: 4082002C  bne 0x831ac45c
	if !ctx.cr[0].eq {
	pc = 0x831AC45C; continue 'dispatch;
	}
	// 831AC434: 572B07FF  clrlwi. r11, r25, 0x1f
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AC438: 40820050  bne 0x831ac488
	if !ctx.cr[0].eq {
	pc = 0x831AC488; continue 'dispatch;
	}
	// 831AC43C: 572B07BD  rlwinm. r11, r25, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AC440: 4182000C  beq 0x831ac44c
	if ctx.cr[0].eq {
	pc = 0x831AC44C; continue 'dispatch;
	}
	// 831AC444: 7F3CF040  cmpld cr6, r28, r30
	ctx.cr[6].compare_u64(ctx.r[28].u64, ctx.r[30].u64, &mut ctx.xer);
	// 831AC448: 41990014  bgt cr6, 0x831ac45c
	if ctx.cr[6].gt {
	pc = 0x831AC45C; continue 'dispatch;
	}
	// 831AC44C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AC450: 409A0038  bne cr6, 0x831ac488
	if !ctx.cr[6].eq {
	pc = 0x831AC488; continue 'dispatch;
	}
	// 831AC454: 7F3CF840  cmpld cr6, r28, r31
	ctx.cr[6].compare_u64(ctx.r[28].u64, ctx.r[31].u64, &mut ctx.xer);
	// 831AC458: 40990030  ble cr6, 0x831ac488
	if !ctx.cr[6].gt {
	pc = 0x831AC488; continue 'dispatch;
	}
	// 831AC45C: 48004905  bl 0x831b0d60
	ctx.lr = 0x831AC460;
	sub_831B0D60(ctx, base);
	// 831AC460: 39600022  li r11, 0x22
	ctx.r[11].s64 = 34;
	// 831AC464: 572A07FF  clrlwi. r10, r25, 0x1f
	ctx.r[10].u64 = ctx.r[25].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AC468: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831AC46C: 4182000C  beq 0x831ac478
	if ctx.cr[0].eq {
	pc = 0x831AC478; continue 'dispatch;
	}
	// 831AC470: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 831AC474: 48000014  b 0x831ac488
	pc = 0x831AC488; continue 'dispatch;
	// 831AC478: 572B07BD  rlwinm. r11, r25, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AC47C: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 831AC480: 40820008  bne 0x831ac488
	if !ctx.cr[0].eq {
	pc = 0x831AC488; continue 'dispatch;
	}
	// 831AC484: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 831AC488: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 831AC48C: 419A0008  beq cr6, 0x831ac494
	if ctx.cr[6].eq {
	pc = 0x831AC494; continue 'dispatch;
	}
	// 831AC490: 93B80000  stw r29, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 831AC494: 572B07BD  rlwinm. r11, r25, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AC498: 41820008  beq 0x831ac4a0
	if ctx.cr[0].eq {
	pc = 0x831AC4A0; continue 'dispatch;
	}
	// 831AC49C: 7F9C00D0  neg r28, r28
	ctx.r[28].s64 = -ctx.r[28].s64;
	// 831AC4A0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831AC4A4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831AC4A8: 4BFFBD00  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC4B0 size=32
    let mut pc: u32 = 0x831AC4B0;
    'dispatch: loop {
        match pc {
            0x831AC4B0 => {
    //   block [0x831AC4B0..0x831AC4D0)
	// 831AC4B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831AC4B4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 831AC4B8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 831AC4BC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 831AC4C0: 386AF580  addi r3, r10, -0xa80
	ctx.r[3].s64 = ctx.r[10].s64 + -2688;
	// 831AC4C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831AC4C8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 831AC4CC: 4BFFFD0C  b 0x831ac1d8
	sub_831AC1D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC4D0 size=32
    let mut pc: u32 = 0x831AC4D0;
    'dispatch: loop {
        match pc {
            0x831AC4D0 => {
    //   block [0x831AC4D0..0x831AC4F0)
	// 831AC4D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831AC4D4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 831AC4D8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 831AC4DC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 831AC4E0: 386AF580  addi r3, r10, -0xa80
	ctx.r[3].s64 = ctx.r[10].s64 + -2688;
	// 831AC4E4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 831AC4E8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 831AC4EC: 4BFFFCEC  b 0x831ac1d8
	sub_831AC1D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC4F0 size=12
    let mut pc: u32 = 0x831AC4F0;
    'dispatch: loop {
        match pc {
            0x831AC4F0 => {
    //   block [0x831AC4F0..0x831AC4FC)
	// 831AC4F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831AC4F4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 831AC4F8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC4FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC4FC size=40
    let mut pc: u32 = 0x831AC4FC;
    'dispatch: loop {
        match pc {
            0x831AC4FC => {
    //   block [0x831AC4FC..0x831AC524)
	// 831AC4FC: 89440000  lbz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AC500: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 831AC504: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831AC508: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 831AC50C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831AC510: 4182000C  beq 0x831ac51c
	if ctx.cr[0].eq {
	pc = 0x831AC51C; continue 'dispatch;
	}
	// 831AC514: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 831AC518: 4082FFE4  bne 0x831ac4fc
	if !ctx.cr[0].eq {
	pc = 0x831AC4FC; continue 'dispatch;
	}
	// 831AC51C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 831AC520: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC524(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC524 size=8
    let mut pc: u32 = 0x831AC524;
    'dispatch: loop {
        match pc {
            0x831AC524 => {
    //   block [0x831AC524..0x831AC52C)
	// 831AC524: 3545FFFF  addic. r10, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AC528: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC52C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC52C size=12
    let mut pc: u32 = 0x831AC52C;
    'dispatch: loop {
        match pc {
            0x831AC52C => {
    //   block [0x831AC52C..0x831AC538)
	// 831AC52C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831AC530: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831AC534: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC538 size=20
    let mut pc: u32 = 0x831AC538;
    'dispatch: loop {
        match pc {
            0x831AC538 => {
    //   block [0x831AC538..0x831AC54C)
	// 831AC538: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831AC53C: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 831AC540: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831AC544: 4200FFF8  bdnz 0x831ac53c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831AC53C; continue 'dispatch;
	}
	// 831AC548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AC550 size=248
    let mut pc: u32 = 0x831AC550;
    'dispatch: loop {
        match pc {
            0x831AC550 => {
    //   block [0x831AC550..0x831AC648)
	// 831AC550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AC554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831AC558: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831AC55C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831AC560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AC564: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831AC568: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831AC56C: 409A0034  bne cr6, 0x831ac5a0
	if !ctx.cr[6].eq {
	pc = 0x831AC5A0; continue 'dispatch;
	}
	// 831AC570: 480047F1  bl 0x831b0d60
	ctx.lr = 0x831AC574;
	sub_831B0D60(ctx, base);
	// 831AC574: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 831AC578: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831AC57C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831AC580: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831AC584: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831AC588: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831AC58C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AC590: 48008881  bl 0x831b4e10
	ctx.lr = 0x831AC594;
	sub_831B4E10(ctx, base);
	// 831AC594: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 831AC598: C82BD228  lfd f1, -0x2dd8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11736 as u32) ) };
	// 831AC59C: 48000094  b 0x831ac630
	pc = 0x831AC630; continue 'dispatch;
	// 831AC5A0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AC5A4: 3BCBF580  addi r30, r11, -0xa80
	ctx.r[30].s64 = ctx.r[11].s64 + -2688;
	// 831AC5A8: 816BF580  lwz r11, -0xa80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2688 as u32) ) } as u64;
	// 831AC5AC: 814B00AC  lwz r10, 0xac(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 831AC5B0: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 831AC5B4: 4099001C  ble cr6, 0x831ac5d0
	if !ctx.cr[6].gt {
	pc = 0x831AC5D0; continue 'dispatch;
	}
	// 831AC5B8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 831AC5BC: 887F0000  lbz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AC5C0: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 831AC5C4: 480048BD  bl 0x831b0e80
	ctx.lr = 0x831AC5C8;
	sub_831B0E80(ctx, base);
	// 831AC5C8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AC5CC: 48000018  b 0x831ac5e4
	pc = 0x831AC5E4; continue 'dispatch;
	// 831AC5D0: 895F0000  lbz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AC5D4: 812B00C8  lwz r9, 0xc8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 831AC5D8: 554A083E  rotlwi r10, r10, 1
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 831AC5DC: 7D4A4A2E  lhzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 831AC5E0: 55430738  rlwinm r3, r10, 0, 0x1c, 0x1c
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 831AC5E4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831AC5E8: 419A000C  beq cr6, 0x831ac5f4
	if ctx.cr[6].eq {
	pc = 0x831AC5F4; continue 'dispatch;
	}
	// 831AC5EC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 831AC5F0: 4BFFFFBC  b 0x831ac5ac
	pc = 0x831AC5AC; continue 'dispatch;
	// 831AC5F4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 831AC5F8: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AC5FC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831AC600: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831AC604: 409AFFF4  bne cr6, 0x831ac5f8
	if !ctx.cr[6].eq {
	pc = 0x831AC5F8; continue 'dispatch;
	}
	// 831AC608: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 831AC60C: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 831AC610: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831AC614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831AC618: 5565003E  slwi r5, r11, 0
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831AC61C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831AC620: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831AC624: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831AC628: 4800A819  bl 0x831b6e40
	ctx.lr = 0x831AC62C;
	sub_831B6E40(ctx, base);
	// 831AC62C: C8230010  lfd f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 831AC630: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831AC634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831AC638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831AC63C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831AC640: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831AC644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC648 size=8
    let mut pc: u32 = 0x831AC648;
    'dispatch: loop {
        match pc {
            0x831AC648 => {
    //   block [0x831AC648..0x831AC650)
	// 831AC648: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831AC64C: 4BFFFF04  b 0x831ac550
	sub_831AC550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC650 size=12
    let mut pc: u32 = 0x831AC650;
    'dispatch: loop {
        match pc {
            0x831AC650 => {
    //   block [0x831AC650..0x831AC65C)
	// 831AC650: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 831AC654: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831AC658: 4BFFDEB8  b 0x831aa510
	sub_831AA510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC660 size=8
    let mut pc: u32 = 0x831AC660;
    'dispatch: loop {
        match pc {
            0x831AC660 => {
    //   block [0x831AC660..0x831AC668)
	// 831AC660: 2F030061  cmpwi cr6, r3, 0x61
	ctx.cr[6].compare_i32(ctx.r[3].s32, 97, &mut ctx.xer);
	// 831AC664: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC668 size=8
    let mut pc: u32 = 0x831AC668;
    'dispatch: loop {
        match pc {
            0x831AC668 => {
    //   block [0x831AC668..0x831AC670)
	// 831AC668: 2F03007A  cmpwi cr6, r3, 0x7a
	ctx.cr[6].compare_i32(ctx.r[3].s32, 122, &mut ctx.xer);
	// 831AC66C: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC670 size=8
    let mut pc: u32 = 0x831AC670;
    'dispatch: loop {
        match pc {
            0x831AC670 => {
    //   block [0x831AC670..0x831AC678)
	// 831AC670: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 831AC674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AC678 size=352
    let mut pc: u32 = 0x831AC678;
    'dispatch: loop {
        match pc {
            0x831AC678 => {
    //   block [0x831AC678..0x831AC7D8)
	// 831AC678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AC67C: 4BFFBAED  bl 0x831a8168
	ctx.lr = 0x831AC680;
	sub_831A8130(ctx, base);
	// 831AC680: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AC684: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831AC688: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 831AC68C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831AC690: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 831AC694: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831AC698: 409A0030  bne cr6, 0x831ac6c8
	if !ctx.cr[6].eq {
	pc = 0x831AC6C8; continue 'dispatch;
	}
	// 831AC69C: 480046C5  bl 0x831b0d60
	ctx.lr = 0x831AC6A0;
	sub_831B0D60(ctx, base);
	// 831AC6A0: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 831AC6A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831AC6A8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831AC6AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831AC6B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831AC6B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831AC6B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AC6BC: 48008755  bl 0x831b4e10
	ctx.lr = 0x831AC6C0;
	sub_831B4E10(ctx, base);
	// 831AC6C0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831AC6C4: 4800010C  b 0x831ac7d0
	pc = 0x831AC7D0; continue 'dispatch;
	// 831AC6C8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 831AC6CC: 419A000C  beq cr6, 0x831ac6d8
	if ctx.cr[6].eq {
	pc = 0x831AC6D8; continue 'dispatch;
	}
	// 831AC6D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831AC6D4: 419AFFC8  beq cr6, 0x831ac69c
	if ctx.cr[6].eq {
	pc = 0x831AC69C; continue 'dispatch;
	}
	// 831AC6D8: 3D403FFF  lis r10, 0x3fff
	ctx.r[10].s64 = 1073676288;
	// 831AC6DC: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 831AC6E0: 39200042  li r9, 0x42
	ctx.r[9].s64 = 66;
	// 831AC6E4: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 831AC6E8: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 831AC6EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 831AC6F0: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831AC6F4: 40990010  ble cr6, 0x831ac704
	if !ctx.cr[6].gt {
	pc = 0x831AC704; continue 'dispatch;
	}
	// 831AC6F8: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 831AC6FC: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 831AC700: 48000008  b 0x831ac708
	pc = 0x831AC708; continue 'dispatch;
	// 831AC704: 578A083C  slwi r10, r28, 1
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831AC708: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 831AC70C: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 831AC710: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 831AC714: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831AC718: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831AC71C: 4E800421  bctrl
	ctx.lr = 0x831AC720;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831AC720: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831AC724: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831AC728: 409A000C  bne cr6, 0x831ac734
	if !ctx.cr[6].eq {
	pc = 0x831AC734; continue 'dispatch;
	}
	// 831AC72C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831AC730: 480000A0  b 0x831ac7d0
	pc = 0x831AC7D0; continue 'dispatch;
	// 831AC734: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831AC738: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 831AC73C: 41980070  blt cr6, 0x831ac7ac
	if ctx.cr[6].lt {
	pc = 0x831AC7AC; continue 'dispatch;
	}
	// 831AC740: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831AC744: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AC748: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831AC74C: 4180001C  blt 0x831ac768
	if ctx.cr[0].lt {
	pc = 0x831AC768; continue 'dispatch;
	}
	// 831AC750: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831AC754: 9BCB0000  stb r30, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 831AC758: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831AC75C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831AC760: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 831AC764: 48000018  b 0x831ac77c
	pc = 0x831AC77C; continue 'dispatch;
	// 831AC768: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831AC76C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AC770: 480078B1  bl 0x831b4020
	ctx.lr = 0x831AC774;
	sub_831B4020(ctx, base);
	// 831AC774: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 831AC778: 419A0034  beq cr6, 0x831ac7ac
	if ctx.cr[6].eq {
	pc = 0x831AC7AC; continue 'dispatch;
	}
	// 831AC77C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831AC780: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831AC784: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831AC788: 41800010  blt 0x831ac798
	if ctx.cr[0].lt {
	pc = 0x831AC798; continue 'dispatch;
	}
	// 831AC78C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831AC790: 9BCB0000  stb r30, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 831AC794: 4BFFFF98  b 0x831ac72c
	pc = 0x831AC72C; continue 'dispatch;
	// 831AC798: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831AC79C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AC7A0: 48007881  bl 0x831b4020
	ctx.lr = 0x831AC7A4;
	sub_831B4020(ctx, base);
	// 831AC7A4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 831AC7A8: 409AFF84  bne cr6, 0x831ac72c
	if !ctx.cr[6].eq {
	pc = 0x831AC72C; continue 'dispatch;
	}
	// 831AC7AC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831AC7B0: 578B083C  slwi r11, r28, 1
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831AC7B4: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 831AC7B8: 7D2BFA14  add r9, r11, r31
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 831AC7BC: 7D4B0034  cntlzw r11, r10
	ctx.r[11].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 831AC7C0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 831AC7C4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 831AC7C8: B3C9FFFE  sth r30, -2(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(-2 as u32), ctx.r[30].u16 ) };
	// 831AC7CC: 386BFFFE  addi r3, r11, -2
	ctx.r[3].s64 = ctx.r[11].s64 + -2;
	// 831AC7D0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831AC7D4: 4BFFB9E4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AC7D8 size=72
    let mut pc: u32 = 0x831AC7D8;
    'dispatch: loop {
        match pc {
            0x831AC7D8 => {
    //   block [0x831AC7D8..0x831AC820)
	// 831AC7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AC7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831AC7E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AC7E4: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 831AC7E8: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 831AC7EC: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 831AC7F0: 3D60831B  lis r11, -0x7ce5
	ctx.r[11].s64 = -2095382528;
	// 831AC7F4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 831AC7F8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 831AC7FC: 386B50D8  addi r3, r11, 0x50d8
	ctx.r[3].s64 = ctx.r[11].s64 + 20696;
	// 831AC800: 4BFFFE79  bl 0x831ac678
	ctx.lr = 0x831AC804;
	sub_831AC678(ctx, base);
	// 831AC804: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831AC808: 40800008  bge 0x831ac810
	if !ctx.cr[0].lt {
	pc = 0x831AC810; continue 'dispatch;
	}
	// 831AC80C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831AC810: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831AC814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831AC818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831AC81C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AC820 size=172
    let mut pc: u32 = 0x831AC820;
    'dispatch: loop {
        match pc {
            0x831AC820 => {
    //   block [0x831AC820..0x831AC8CC)
	// 831AC820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AC824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831AC828: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831AC82C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AC830: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831AC834: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 831AC838: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 831AC83C: 409A0010  bne cr6, 0x831ac84c
	if !ctx.cr[6].eq {
	pc = 0x831AC84C; continue 'dispatch;
	}
	// 831AC840: 48004521  bl 0x831b0d60
	ctx.lr = 0x831AC844;
	sub_831B0D60(ctx, base);
	// 831AC844: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 831AC848: 48000050  b 0x831ac898
	pc = 0x831AC898; continue 'dispatch;
	// 831AC84C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831AC850: 419AFFF0  beq cr6, 0x831ac840
	if ctx.cr[6].eq {
	pc = 0x831AC840; continue 'dispatch;
	}
	// 831AC854: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831AC858: 419AFFE8  beq cr6, 0x831ac840
	if ctx.cr[6].eq {
	pc = 0x831AC840; continue 'dispatch;
	}
	// 831AC85C: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 831AC860: 3D60831C  lis r11, -0x7ce4
	ctx.r[11].s64 = -2095316992;
	// 831AC864: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 831AC868: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 831AC86C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831AC870: 386B9A88  addi r3, r11, -0x6578
	ctx.r[3].s64 = ctx.r[11].s64 + -25976;
	// 831AC874: 4BFFFE05  bl 0x831ac678
	ctx.lr = 0x831AC878;
	sub_831AC678(ctx, base);
	// 831AC878: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831AC87C: 4080000C  bge 0x831ac888
	if !ctx.cr[0].lt {
	pc = 0x831AC888; continue 'dispatch;
	}
	// 831AC880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831AC884: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 831AC888: 2F03FFFE  cmpwi cr6, r3, -2
	ctx.cr[6].compare_i32(ctx.r[3].s32, -2, &mut ctx.xer);
	// 831AC88C: 409A002C  bne cr6, 0x831ac8b8
	if !ctx.cr[6].eq {
	pc = 0x831AC8B8; continue 'dispatch;
	}
	// 831AC890: 480044D1  bl 0x831b0d60
	ctx.lr = 0x831AC894;
	sub_831B0D60(ctx, base);
	// 831AC894: 39600022  li r11, 0x22
	ctx.r[11].s64 = 34;
	// 831AC898: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831AC89C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831AC8A0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831AC8A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831AC8A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831AC8AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AC8B0: 48008561  bl 0x831b4e10
	ctx.lr = 0x831AC8B4;
	sub_831B4E10(ctx, base);
	// 831AC8B4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831AC8B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831AC8BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831AC8C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831AC8C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831AC8C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC8D0 size=12
    let mut pc: u32 = 0x831AC8D0;
    'dispatch: loop {
        match pc {
            0x831AC8D0 => {
    //   block [0x831AC8D0..0x831AC8DC)
	// 831AC8D0: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 831AC8D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831AC8D8: 4BFFFF48  b 0x831ac820
	sub_831AC820(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC8E0 size=12
    let mut pc: u32 = 0x831AC8E0;
    'dispatch: loop {
        match pc {
            0x831AC8E0 => {
    //   block [0x831AC8E0..0x831AC8EC)
	// 831AC8E0: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AC8E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831AC8E8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC8EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC8EC size=84
    let mut pc: u32 = 0x831AC8EC;
    'dispatch: loop {
        match pc {
            0x831AC8EC => {
    //   block [0x831AC8EC..0x831AC940)
	// 831AC8EC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AC8F0: 7D6A0775  extsb. r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AC8F4: 41820060  beq 0x831ac954
	if ctx.cr[0].eq {
		sub_831AC940(ctx, base);
		return;
	}
	// 831AC8F8: 7D241850  subf r9, r4, r3
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[4].s64;
	// 831AC8FC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 831AC900: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AC904: 419A0030  beq cr6, 0x831ac934
	if ctx.cr[6].eq {
	pc = 0x831AC934; continue 'dispatch;
	}
	// 831AC908: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AC90C: 7D4A0775  extsb. r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AC910: 41820024  beq 0x831ac934
	if ctx.cr[0].eq {
	pc = 0x831AC934; continue 'dispatch;
	}
	// 831AC914: 7D0958AE  lbzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831AC918: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 831AC91C: 7D4A4051  subf. r10, r10, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AC920: 40820014  bne 0x831ac934
	if !ctx.cr[0].eq {
	pc = 0x831AC934; continue 'dispatch;
	}
	// 831AC924: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831AC928: 7D4958AE  lbzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831AC92C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831AC930: 409AFFD8  bne cr6, 0x831ac908
	if !ctx.cr[6].eq {
	pc = 0x831AC908; continue 'dispatch;
	}
	// 831AC934: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AC938: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831AC93C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC940 size=28
    let mut pc: u32 = 0x831AC940;
    'dispatch: loop {
        match pc {
            0x831AC940 => {
    //   block [0x831AC940..0x831AC95C)
	// 831AC940: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 831AC944: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831AC948: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AC94C: 7D6A0775  extsb. r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AC950: 4082FFAC  bne 0x831ac8fc
	if !ctx.cr[0].eq {
		sub_831AC8EC(ctx, base);
		return;
	}
	// 831AC954: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AC958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AC960 size=72
    let mut pc: u32 = 0x831AC960;
    'dispatch: loop {
        match pc {
            0x831AC960 => {
    //   block [0x831AC960..0x831AC9A8)
	// 831AC960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AC964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831AC968: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831AC96C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AC970: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831AC974: 4800678D  bl 0x831b3100
	ctx.lr = 0x831AC978;
	sub_831B3100(ctx, base);
	// 831AC978: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831AC97C: 4800673D  bl 0x831b30b8
	ctx.lr = 0x831AC980;
	sub_831B30B8(ctx, base);
	// 831AC980: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AC984: 386000FF  li r3, 0xff
	ctx.r[3].s64 = 255;
	// 831AC988: 816BEEF0  lwz r11, -0x1110(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4368 as u32) ) } as u64;
	// 831AC98C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831AC990: 4E800421  bctrl
	ctx.lr = 0x831AC994;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831AC994: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831AC998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831AC99C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831AC9A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831AC9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC9A8 size=8
    let mut pc: u32 = 0x831AC9A8;
    'dispatch: loop {
        match pc {
            0x831AC9A8 => {
    //   block [0x831AC9A8..0x831AC9B0)
	// 831AC9A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AC9AC: 480962A0  b 0x83242c4c
	sub_83242C4C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC9B0 size=8
    let mut pc: u32 = 0x831AC9B0;
    'dispatch: loop {
        match pc {
            0x831AC9B0 => {
    //   block [0x831AC9B0..0x831AC9B8)
	// 831AC9B0: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 831AC9B4: 4800DE84  b 0x831ba838
	sub_831BA838(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AC9B8 size=8
    let mut pc: u32 = 0x831AC9B8;
    'dispatch: loop {
        match pc {
            0x831AC9B8 => {
    //   block [0x831AC9B8..0x831AC9C0)
	// 831AC9B8: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 831AC9BC: 4800DD1C  b 0x831ba6d8
	sub_831BA6D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AC9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AC9C0 size=88
    let mut pc: u32 = 0x831AC9C0;
    'dispatch: loop {
        match pc {
            0x831AC9C0 => {
    //   block [0x831AC9C0..0x831ACA18)
	// 831AC9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AC9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831AC9C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831AC9CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831AC9D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AC9D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831AC9D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831AC9DC: 4800001C  b 0x831ac9f8
	pc = 0x831AC9F8; continue 'dispatch;
	// 831AC9E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AC9E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831AC9E8: 419A000C  beq cr6, 0x831ac9f4
	if ctx.cr[6].eq {
	pc = 0x831AC9F4; continue 'dispatch;
	}
	// 831AC9EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831AC9F0: 4E800421  bctrl
	ctx.lr = 0x831AC9F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831AC9F4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 831AC9F8: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 831AC9FC: 4198FFE4  blt cr6, 0x831ac9e0
	if ctx.cr[6].lt {
	pc = 0x831AC9E0; continue 'dispatch;
	}
	// 831ACA00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831ACA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831ACA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831ACA0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831ACA10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831ACA14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ACA18 size=8
    let mut pc: u32 = 0x831ACA18;
    'dispatch: loop {
        match pc {
            0x831ACA18 => {
    //   block [0x831ACA18..0x831ACA20)
	// 831ACA18: 8324280C  lwz r25, 0x280c(r4)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(10252 as u32) ) } as u64;
	// 831ACA1C: 8224BF70  lwz r17, -0x4090(r4)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-16528 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ACA20 size=256
    let mut pc: u32 = 0x831ACA20;
    'dispatch: loop {
        match pc {
            0x831ACA20 => {
    //   block [0x831ACA20..0x831ACB20)
	// 831ACA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831ACA24: 4BFFB745  bl 0x831a8168
	ctx.lr = 0x831ACA28;
	sub_831A8130(ctx, base);
	// 831ACA28: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 831ACA2C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831ACA30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831ACA34: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 831ACA38: 939F00A4  stw r28, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[28].u32 ) };
	// 831ACA3C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 831ACA40: 4800DDF9  bl 0x831ba838
	ctx.lr = 0x831ACA44;
	sub_831BA838(ctx, base);
	// 831ACA44: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831ACA48: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 831ACA4C: 816BBE8C  lwz r11, -0x4174(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16756 as u32) ) } as u64;
	// 831ACA50: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831ACA54: 419A00A4  beq cr6, 0x831acaf8
	if ctx.cr[6].eq {
	pc = 0x831ACAF8; continue 'dispatch;
	}
	// 831ACA58: 3D208343  lis r9, -0x7cbd
	ctx.r[9].s64 = -2092761088;
	// 831ACA5C: 8169BE88  lwz r11, -0x4178(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16760 as u32) ) } as u64;
	// 831ACA60: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831ACA64: 409A000C  bne cr6, 0x831aca70
	if !ctx.cr[6].eq {
	pc = 0x831ACA70; continue 'dispatch;
	}
	// 831ACA68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ACA6C: 480961E1  bl 0x83242c4c
	ctx.lr = 0x831ACA70;
	// extern call 0x83242C4C → crate::xboxkrnl::KeBugCheck
	crate::xboxkrnl::KeBugCheck(ctx, base);
	// 831ACA70: 3D008343  lis r8, -0x7cbd
	ctx.r[8].s64 = -2092761088;
	// 831ACA74: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831ACA78: 9169BE88  stw r11, -0x4178(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-16760 as u32), ctx.r[11].u32 ) };
	// 831ACA7C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 831ACA80: 9B88BE84  stb r28, -0x417c(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(-16764 as u32), ctx.r[28].u8 ) };
	// 831ACA84: 409A0060  bne cr6, 0x831acae4
	if !ctx.cr[6].eq {
	pc = 0x831ACAE4; continue 'dispatch;
	}
	// 831ACA88: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 831ACA8C: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 831ACA90: 83CBCF10  lwz r30, -0x30f0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12528 as u32) ) } as u64;
	// 831ACA94: 83AACF14  lwz r29, -0x30ec(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-12524 as u32) ) } as u64;
	// 831ACA98: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831ACA9C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 831ACAA0: 41820030  beq 0x831acad0
	if ctx.cr[0].eq {
	pc = 0x831ACAD0; continue 'dispatch;
	}
	// 831ACAA4: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 831ACAA8: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 831ACAAC: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 831ACAB0: 41980020  blt cr6, 0x831acad0
	if ctx.cr[6].lt {
	pc = 0x831ACAD0; continue 'dispatch;
	}
	// 831ACAB4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831ACAB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831ACABC: 419A000C  beq cr6, 0x831acac8
	if ctx.cr[6].eq {
	pc = 0x831ACAC8; continue 'dispatch;
	}
	// 831ACAC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831ACAC4: 4E800421  bctrl
	ctx.lr = 0x831ACAC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831ACAC8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831ACACC: 4BFFFFD8  b 0x831acaa4
	pc = 0x831ACAA4; continue 'dispatch;
	// 831ACAD0: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 831ACAD4: 3D408325  lis r10, -0x7cdb
	ctx.r[10].s64 = -2094727168;
	// 831ACAD8: 388B1394  addi r4, r11, 0x1394
	ctx.r[4].s64 = ctx.r[11].s64 + 5012;
	// 831ACADC: 386A1388  addi r3, r10, 0x1388
	ctx.r[3].s64 = ctx.r[10].s64 + 5000;
	// 831ACAE0: 4BFFFEE1  bl 0x831ac9c0
	ctx.lr = 0x831ACAE4;
	sub_831AC9C0(ctx, base);
	// 831ACAE4: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 831ACAE8: 3D408325  lis r10, -0x7cdb
	ctx.r[10].s64 = -2094727168;
	// 831ACAEC: 388B139C  addi r4, r11, 0x139c
	ctx.r[4].s64 = ctx.r[11].s64 + 5020;
	// 831ACAF0: 386A1398  addi r3, r10, 0x1398
	ctx.r[3].s64 = ctx.r[10].s64 + 5016;
	// 831ACAF4: 4BFFFECD  bl 0x831ac9c0
	ctx.lr = 0x831ACAF8;
	sub_831AC9C0(ctx, base);
	// 831ACAF8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831ACAFC: 399F0080  addi r12, r31, 0x80
	ctx.r[12].s64 = ctx.r[31].s64 + 128;
	// 831ACB00: 48000041  bl 0x831acb40
	ctx.lr = 0x831ACB04;
	sub_831ACB20(ctx, base);
	// 831ACB04: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 831ACB08: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831ACB0C: 409A000C  bne cr6, 0x831acb18
	if !ctx.cr[6].eq {
	pc = 0x831ACB18; continue 'dispatch;
	}
	// 831ACB10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ACB14: 48096139  bl 0x83242c4c
	ctx.lr = 0x831ACB18;
	// extern call 0x83242C4C → crate::xboxkrnl::KeBugCheck
	crate::xboxkrnl::KeBugCheck(ctx, base);
	// 831ACB18: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 831ACB1C: 4BFFB69C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ACB20 size=96
    let mut pc: u32 = 0x831ACB20;
    'dispatch: loop {
        match pc {
            0x831ACB20 => {
    //   block [0x831ACB20..0x831ACB80)
	// 831ACB20: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 831ACB24: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 831ACB28: FB81FFF0  std r28, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[28].u64 ) };
	// 831ACB2C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831ACB30: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 831ACB34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831ACB38: 839F00A4  lwz r28, 0xa4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 831ACB3C: 4800001C  b 0x831acb58
	pc = 0x831ACB58; continue 'dispatch;
	// 831ACB40: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 831ACB44: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 831ACB48: FB81FFF0  std r28, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[28].u64 ) };
	// 831ACB4C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831ACB50: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 831ACB54: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831ACB58: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 831ACB5C: 419A000C  beq cr6, 0x831acb68
	if ctx.cr[6].eq {
	pc = 0x831ACB68; continue 'dispatch;
	}
	// 831ACB60: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 831ACB64: 4800DB75  bl 0x831ba6d8
	ctx.lr = 0x831ACB68;
	sub_831BA6D8(ctx, base);
	// 831ACB68: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 831ACB6C: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 831ACB70: EB81FFF0  ld r28, -0x10(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831ACB74: 8181FFE8  lwz r12, -0x18(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) } as u64;
	// 831ACB78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831ACB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ACB80 size=12
    let mut pc: u32 = 0x831ACB80;
    'dispatch: loop {
        match pc {
            0x831ACB80 => {
    //   block [0x831ACB80..0x831ACB8C)
	// 831ACB80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831ACB84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831ACB88: 4BFFFE98  b 0x831aca20
	sub_831ACA20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ACB90 size=12
    let mut pc: u32 = 0x831ACB90;
    'dispatch: loop {
        match pc {
            0x831ACB90 => {
    //   block [0x831ACB90..0x831ACB9C)
	// 831ACB90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831ACB94: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831ACB98: 4BFFFE88  b 0x831aca20
	sub_831ACA20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ACBA0 size=16
    let mut pc: u32 = 0x831ACBA0;
    'dispatch: loop {
        match pc {
            0x831ACBA0 => {
    //   block [0x831ACBA0..0x831ACBB0)
	// 831ACBA0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 831ACBA4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831ACBA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ACBAC: 4BFFFE74  b 0x831aca20
	sub_831ACA20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ACBB0 size=92
    let mut pc: u32 = 0x831ACBB0;
    'dispatch: loop {
        match pc {
            0x831ACBB0 => {
    //   block [0x831ACBB0..0x831ACC0C)
	// 831ACBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831ACBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831ACBB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831ACBBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ACBC0: 4800DDC9  bl 0x831ba988
	ctx.lr = 0x831ACBC4;
	sub_831BA988(ctx, base);
	// 831ACBC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ACBC8: 4800DCD1  bl 0x831ba898
	ctx.lr = 0x831ACBCC;
	sub_831BA898(ctx, base);
	// 831ACBCC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ACBD0: 48008231  bl 0x831b4e00
	ctx.lr = 0x831ACBD4;
	sub_831B4E00(ctx, base);
	// 831ACBD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ACBD8: 48006559  bl 0x831b3130
	ctx.lr = 0x831ACBDC;
	sub_831B3130(ctx, base);
	// 831ACBDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ACBE0: 4B113421  bl 0x822c0000
	ctx.lr = 0x831ACBE4;
	sub_822C0000(ctx, base);
	// 831ACBE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ACBE8: 4800A241  bl 0x831b6e28
	ctx.lr = 0x831ACBEC;
	sub_831B6E28(ctx, base);
	// 831ACBEC: 3D60831B  lis r11, -0x7ce5
	ctx.r[11].s64 = -2095382528;
	// 831ACBF0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 831ACBF4: 396BCB90  addi r11, r11, -0x3470
	ctx.r[11].s64 = ctx.r[11].s64 + -13424;
	// 831ACBF8: 916AEEF0  stw r11, -0x1110(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4368 as u32), ctx.r[11].u32 ) };
	// 831ACBFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831ACC00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831ACC04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831ACC08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831ACC10 size=220
    let mut pc: u32 = 0x831ACC10;
    'dispatch: loop {
        match pc {
            0x831ACC10 => {
    //   block [0x831ACC10..0x831ACCEC)
	// 831ACC10: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831ACC14: 396BEEF8  addi r11, r11, -0x1108
	ctx.r[11].s64 = ctx.r[11].s64 + -4360;
	// 831ACC18: C80B0020  lfd f0, 0x20(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 831ACC1C: FCA00072  fmul f5, f0, f1
	ctx.f[5].f64 = ctx.f[0].f64 * ctx.f[1].f64;
	// 831ACC20: C9AB0028  lfd f13, 0x28(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 831ACC24: C98B0030  lfd f12, 0x30(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 831ACC28: C94B0040  lfd f10, 0x40(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 831ACC2C: C96B0048  lfd f11, 0x48(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) };
	// 831ACC30: C92B0060  lfd f9, 0x60(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	// 831ACC34: C90B0058  lfd f8, 0x58(r11)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	// 831ACC38: C8EB0038  lfd f7, 0x38(r11)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 831ACC3C: C8CB0050  lfd f6, 0x50(r11)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 831ACC40: C00B006C  lfs f0, 0x6c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831ACC44: FCA02E5C  fctid f5, f5
	ctx.f[5].s64 = if ctx.f[5].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[5].f64 as i64 };
	// 831ACC48: FCA02E9C  fcfid f5, f5
	ctx.f[5].f64 = (ctx.f[5].s64 as f64);
	// 831ACC4C: FDAD097C  fnmsub f13, f13, f5, f1
	ctx.f[13].f64 = -(ctx.f[13].f64 * ctx.f[5].f64 - ctx.f[1].f64);
	// 831ACC50: FC80281E  fctiwz f4, f5
	ctx.f[4].s64 = if ctx.f[5].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[5].f64.trunc() as i32 as i64 };
	// 831ACC54: D881FFF0  stfd f4, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[4].u64 ) };
	// 831ACC58: 8141FFF4  lwz r10, -0xc(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 831ACC5C: 354A0001  addic. r10, r10, 1
	ctx.xer.ca = (ctx.r[10].u32 > (!(1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831ACC60: FDAC697C  fnmsub f13, f12, f5, f13
	ctx.f[13].f64 = -(ctx.f[12].f64 * ctx.f[5].f64 - ctx.f[13].f64);
	// 831ACC64: FD8D0372  fmul f12, f13, f13
	ctx.f[12].f64 = ctx.f[13].f64 * ctx.f[13].f64;
	// 831ACC68: FD6B533A  fmadd f11, f11, f12, f10
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[12].f64 + ctx.f[10].f64;
	// 831ACC6C: FD49433A  fmadd f10, f9, f12, f8
	ctx.f[10].f64 = ctx.f[9].f64 * ctx.f[12].f64 + ctx.f[8].f64;
	// 831ACC70: FD6B3B3A  fmadd f11, f11, f12, f7
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[12].f64 + ctx.f[7].f64;
	// 831ACC74: FD8A333A  fmadd f12, f10, f12, f6
	ctx.f[12].f64 = ctx.f[10].f64 * ctx.f[12].f64 + ctx.f[6].f64;
	// 831ACC78: FDAB0372  fmul f13, f11, f13
	ctx.f[13].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	// 831ACC7C: FD8C6828  fsub f12, f12, f13
	ctx.f[12].f64 = ctx.f[12].f64 - ctx.f[13].f64;
	// 831ACC80: FDAD6024  fdiv f13, f13, f12
	ctx.f[13].f64 = ctx.f[13].f64 / ctx.f[12].f64;
	// 831ACC84: FC0D002A  fadd f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	// 831ACC88: 41820040  beq 0x831accc8
	if ctx.cr[0].eq {
	pc = 0x831ACCC8; continue 'dispatch;
	}
	// 831ACC8C: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 831ACC90: A101FFF0  lhz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 831ACC94: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 831ACC98: A121FFF0  lhz r9, -0x10(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 831ACC9C: 7108800F  andi. r8, r8, 0x800f
	ctx.r[8].u64 = ctx.r[8].u64 & 32783;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 831ACCA0: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 831ACCA4: 5529E57E  rlwinm r9, r9, 0x1c, 0x15, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000000Fu64;
	// 831ACCA8: 3929FC02  addi r9, r9, -0x3fe
	ctx.r[9].s64 = ctx.r[9].s64 + -1022;
	// 831ACCAC: 7D290734  extsh r9, r9
	ctx.r[9].s64 = ctx.r[9].s16 as i64;
	// 831ACCB0: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 831ACCB4: 394A03FE  addi r10, r10, 0x3fe
	ctx.r[10].s64 = ctx.r[10].s64 + 1022;
	// 831ACCB8: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831ACCBC: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 831ACCC0: B141FFF0  sth r10, -0x10(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u16 ) };
	// 831ACCC4: C801FFF0  lfd f0, -0x10(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831ACCC8: C9AB0000  lfd f13, 0(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 831ACCCC: FD616828  fsub f11, f1, f13
	ctx.f[11].f64 = ctx.f[1].f64 - ctx.f[13].f64;
	// 831ACCD0: C9AB0008  lfd f13, 8(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 831ACCD4: C98B0010  lfd f12, 0x10(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 831ACCD8: FD4D0828  fsub f10, f13, f1
	ctx.f[10].f64 = ctx.f[13].f64 - ctx.f[1].f64;
	// 831ACCDC: C1AB0068  lfs f13, 0x68(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831ACCE0: FC0B032E  fsel f0, f11, f12, f0
	ctx.f[0].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[0].f64 };
	// 831ACCE4: FC2A036E  fsel f1, f10, f13, f0
	ctx.f[1].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 831ACCE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ACCF0 size=212
    let mut pc: u32 = 0x831ACCF0;
    'dispatch: loop {
        match pc {
            0x831ACCF0 => {
    //   block [0x831ACCF0..0x831ACDC4)
	// 831ACCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831ACCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831ACCF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831ACCFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831ACD00: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 831ACD04: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 831ACD08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831ACD0C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 831ACD10: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831ACD14: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831ACD18: 3BCBEEF8  addi r30, r11, -0x1108
	ctx.r[30].s64 = ctx.r[11].s64 + -4360;
	// 831ACD1C: C81E0020  lfd f0, 0x20(r30)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) };
	// 831ACD20: FC2007F2  fmul f1, f0, f31
	ctx.f[1].f64 = ctx.f[0].f64 * ctx.f[31].f64;
	// 831ACD24: 4800C32D  bl 0x831b9050
	ctx.lr = 0x831ACD28;
	sub_831B9050(ctx, base);
	// 831ACD28: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 831ACD2C: C81E0028  lfd f0, 0x28(r30)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) };
	// 831ACD30: C9BE0030  lfd f13, 0x30(r30)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) };
	// 831ACD34: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 831ACD38: C97E0040  lfd f11, 0x40(r30)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) };
	// 831ACD3C: C99E0048  lfd f12, 0x48(r30)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) };
	// 831ACD40: C95E0060  lfd f10, 0x60(r30)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) };
	// 831ACD44: C93E0058  lfd f9, 0x58(r30)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) };
	// 831ACD48: C91E0038  lfd f8, 0x38(r30)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) };
	// 831ACD4C: C8FE0050  lfd f7, 0x50(r30)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) };
	// 831ACD50: C8CBD760  lfd f6, -0x28a0(r11)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-10400 as u32) ) };
	// 831ACD54: FC00FFBC  fnmsub f0, f0, f30, f31
	ctx.f[0].f64 = -(ctx.f[0].f64 * ctx.f[30].f64 - ctx.f[31].f64);
	// 831ACD58: FC0D07BC  fnmsub f0, f13, f30, f0
	ctx.f[0].f64 = -(ctx.f[13].f64 * ctx.f[30].f64 - ctx.f[0].f64);
	// 831ACD5C: FDA00032  fmul f13, f0, f0
	ctx.f[13].f64 = ctx.f[0].f64 * ctx.f[0].f64;
	// 831ACD60: FD8C5B7A  fmadd f12, f12, f13, f11
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[13].f64 + ctx.f[11].f64;
	// 831ACD64: FD6A4B7A  fmadd f11, f10, f13, f9
	ctx.f[11].f64 = ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[9].f64;
	// 831ACD68: FD8C437A  fmadd f12, f12, f13, f8
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[13].f64 + ctx.f[8].f64;
	// 831ACD6C: FDAB3B7A  fmadd f13, f11, f13, f7
	ctx.f[13].f64 = ctx.f[11].f64 * ctx.f[13].f64 + ctx.f[7].f64;
	// 831ACD70: FC0C0032  fmul f0, f12, f0
	ctx.f[0].f64 = ctx.f[12].f64 * ctx.f[0].f64;
	// 831ACD74: FDAD0028  fsub f13, f13, f0
	ctx.f[13].f64 = ctx.f[13].f64 - ctx.f[0].f64;
	// 831ACD78: FC006824  fdiv f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[13].f64;
	// 831ACD7C: FFE0302A  fadd f31, f0, f6
	ctx.f[31].f64 = ctx.f[0].f64 + ctx.f[6].f64;
	// 831ACD80: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 831ACD84: 48008185  bl 0x831b4f08
	ctx.lr = 0x831ACD88;
	sub_831B4F08(ctx, base);
	// 831ACD88: FC00F01E  fctiwz f0, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[30].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[30].f64.trunc() as i32 as i64 };
	// 831ACD8C: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 831ACD90: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831ACD94: 7D635A14  add r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 831ACD98: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831ACD9C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 831ACDA0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831ACDA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831ACDA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831ACDAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831ACDB0: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 831ACDB4: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 831ACDB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831ACDBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831ACDC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ACDD0 size=40
    let mut pc: u32 = 0x831ACDD0;
    'dispatch: loop {
        match pc {
            0x831ACDD0 => {
    //   block [0x831ACDD0..0x831ACDF8)
	// 831ACDD0: 7CA02B79  or. r0, r5, r5
	ctx.r[0].u64 = ctx.r[5].u64 | ctx.r[5].u64;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831ACDD4: 7CA903A6  mtctr r5
	ctx.ctr.u64 = ctx.r[5].u64;
	// 831ACDD8: 4081003C  ble 0x831ace14
	if !ctx.cr[0].gt {
		sub_831ACE14(ctx, base);
		return;
	}
	// 831ACDDC: 89030000  lbz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831ACDE0: 88E40000  lbz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831ACDE4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 831ACDE8: 2C880000  cmpwi cr1, r8, 0
	ctx.cr[1].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 831ACDEC: 7C674011  subfc. r3, r7, r8
	ctx.xer.ca = ctx.r[8].u32 >= ctx.r[7].u32;
	ctx.r[3].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831ACDF0: 40060008  bdnzf 4*cr1+eq, 0x831acdf8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 && !ctx.cr[0].eq {
	pc = 0x831ACDF8; continue 'dispatch;
	}
	// 831ACDF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ACDF8 size=4
    let mut pc: u32 = 0x831ACDF8;
    'dispatch: loop {
        match pc {
            0x831ACDF8 => {
    //   block [0x831ACDF8..0x831ACDFC)
	// 831ACDF8: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACDFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ACDFC size=24
    let mut pc: u32 = 0x831ACDFC;
    'dispatch: loop {
        match pc {
            0x831ACDFC => {
    //   block [0x831ACDFC..0x831ACE14)
	// 831ACDFC: 8D0A0001  lbzu r8, 1(r10)
	ea = ctx.r[10].u32.wrapping_add(1 as u32);
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[10].u32 = ea;
	// 831ACE00: 8CE40001  lbzu r7, 1(r4)
	ea = ctx.r[4].u32.wrapping_add(1 as u32);
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[4].u32 = ea;
	// 831ACE04: 2C880000  cmpwi cr1, r8, 0
	ctx.cr[1].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 831ACE08: 7C674011  subfc. r3, r7, r8
	ctx.xer.ca = ctx.r[8].u32 >= ctx.r[7].u32;
	ctx.r[3].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831ACE0C: 4006FFEC  bdnzf 4*cr1+eq, 0x831acdf8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 && !ctx.cr[0].eq {
	pc = 0x831ACDF8; continue 'dispatch;
	}
	// 831ACE10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACE14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ACE14 size=8
    let mut pc: u32 = 0x831ACE14;
    'dispatch: loop {
        match pc {
            0x831ACE14 => {
    //   block [0x831ACE14..0x831ACE1C)
	// 831ACE14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ACE18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ACE20 size=36
    let mut pc: u32 = 0x831ACE20;
    'dispatch: loop {
        match pc {
            0x831ACE20 => {
    //   block [0x831ACE20..0x831ACE44)
	// 831ACE20: 88C30000  lbz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831ACE24: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831ACE28: 4182001C  beq 0x831ace44
	if ctx.cr[0].eq {
		sub_831ACE44(ctx, base);
		return;
	}
	// 831ACE2C: 2C860000  cmpwi cr1, r6, 0
	ctx.cr[1].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831ACE30: 7C062000  cmpw r6, r4
	ctx.cr[0].compare_i32(ctx.r[4].s32, ctx.r[0].s32, &mut ctx.xer);
	// 831ACE34: 41860020  beq cr1, 0x831ace54
	if ctx.cr[1].eq {
		sub_831ACE54(ctx, base);
		return;
	}
	// 831ACE38: 41820020  beq 0x831ace58
	if ctx.cr[0].eq {
		sub_831ACE54(ctx, base);
		return;
	}
	// 831ACE3C: 8CC30001  lbzu r6, 1(r3)
	ea = ctx.r[3].u32.wrapping_add(1 as u32);
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[3].u32 = ea;
	// 831ACE40: 4BFFFFEC  b 0x831ace2c
	pc = 0x831ACE2C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACE44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ACE44 size=16
    let mut pc: u32 = 0x831ACE44;
    'dispatch: loop {
        match pc {
            0x831ACE44 => {
    //   block [0x831ACE44..0x831ACE54)
	// 831ACE44: 2C060000  cmpwi r6, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831ACE48: 41820010  beq 0x831ace58
	if ctx.cr[0].eq {
		sub_831ACE54(ctx, base);
		return;
	}
	// 831ACE4C: 8CC30001  lbzu r6, 1(r3)
	ea = ctx.r[3].u32.wrapping_add(1 as u32);
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[3].u32 = ea;
	// 831ACE50: 4BFFFFF4  b 0x831ace44
	pc = 0x831ACE44; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACE54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ACE54 size=8
    let mut pc: u32 = 0x831ACE54;
    'dispatch: loop {
        match pc {
            0x831ACE54 => {
    //   block [0x831ACE54..0x831ACE5C)
	// 831ACE54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ACE58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ACE60 size=8
    let mut pc: u32 = 0x831ACE60;
    'dispatch: loop {
        match pc {
            0x831ACE60 => {
    //   block [0x831ACE60..0x831ACE68)
	// 831ACE60: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 831ACE64: 8219C60C  lwz r16, -0x39f4(r25)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-14836 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ACE68 size=64
    let mut pc: u32 = 0x831ACE68;
    'dispatch: loop {
        match pc {
            0x831ACE68 => {
    //   block [0x831ACE68..0x831ACEA8)
	// 831ACE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831ACE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831ACE70: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 831ACE74: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 831ACE78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831ACE7C: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 831ACE80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831ACE84: 4B113AB5  bl 0x822c0938
	ctx.lr = 0x831ACE88;
	sub_822C0938(ctx, base);
	// 831ACE88: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 831ACE8C: 48000008  b 0x831ace94
	pc = 0x831ACE94; continue 'dispatch;
	// 831ACE90: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 831ACE94: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 831ACE98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831ACE9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831ACEA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831ACEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ACEA8 size=8
    let mut pc: u32 = 0x831ACEA8;
    'dispatch: loop {
        match pc {
            0x831ACEA8 => {
    //   block [0x831ACEA8..0x831ACEB0)
	// 831ACEA8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 831ACEAC: 8219C60C  lwz r16, -0x39f4(r25)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-14836 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ACEB0 size=32
    let mut pc: u32 = 0x831ACEB0;
    'dispatch: loop {
        match pc {
            0x831ACEB0 => {
    //   block [0x831ACEB0..0x831ACED0)
	// 831ACEB0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 831ACEB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831ACEB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831ACEBC: 3C60831B  lis r3, -0x7ce5
	ctx.r[3].s64 = -2095382528;
	// 831ACEC0: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 831ACEC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831ACEC8: 3863CE90  addi r3, r3, -0x3170
	ctx.r[3].s64 = ctx.r[3].s64 + -12656;
	// 831ACECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ACED0 size=4
    let mut pc: u32 = 0x831ACED0;
    'dispatch: loop {
        match pc {
            0x831ACED0 => {
    //   block [0x831ACED0..0x831ACED4)
	// 831ACED0: 48095DFC  b 0x83242ccc
	sub_83242CCC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ACED8 size=88
    let mut pc: u32 = 0x831ACED8;
    'dispatch: loop {
        match pc {
            0x831ACED8 => {
    //   block [0x831ACED8..0x831ACF30)
	// 831ACED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831ACEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831ACEE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831ACEE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831ACEE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831ACEEC: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 831ACEF0: 807EEF78  lwz r3, -0x1088(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4232 as u32) ) } as u64;
	// 831ACEF4: 48095DB9  bl 0x83242cac
	ctx.lr = 0x831ACEF8;
	// extern call 0x83242CAC → crate::xboxkrnl::KeTlsGetValue
	crate::xboxkrnl::KeTlsGetValue(ctx, base);
	// 831ACEF8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831ACEFC: 40820018  bne 0x831acf14
	if !ctx.cr[0].eq {
	pc = 0x831ACF14; continue 'dispatch;
	}
	// 831ACF00: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 831ACF04: 807EEF78  lwz r3, -0x1088(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4232 as u32) ) } as u64;
	// 831ACF08: 808BBEA0  lwz r4, -0x4160(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16736 as u32) ) } as u64;
	// 831ACF0C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831ACF10: 48095DAD  bl 0x83242cbc
	ctx.lr = 0x831ACF14;
	// extern call 0x83242CBC → crate::xboxkrnl::KeTlsSetValue
	crate::xboxkrnl::KeTlsSetValue(ctx, base);
	// 831ACF14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831ACF18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831ACF1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831ACF20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831ACF24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831ACF28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831ACF2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ACF30 size=120
    let mut pc: u32 = 0x831ACF30;
    'dispatch: loop {
        match pc {
            0x831ACF30 => {
    //   block [0x831ACF30..0x831ACFA8)
	// 831ACF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831ACF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831ACF38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831ACF3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831ACF40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831ACF44: 3FE08335  lis r31, -0x7ccb
	ctx.r[31].s64 = -2093678592;
	// 831ACF48: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 831ACF4C: 807FEF74  lwz r3, -0x108c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4236 as u32) ) } as u64;
	// 831ACF50: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 831ACF54: 419A001C  beq cr6, 0x831acf70
	if ctx.cr[6].eq {
	pc = 0x831ACF70; continue 'dispatch;
	}
	// 831ACF58: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 831ACF5C: 816BBEA8  lwz r11, -0x4158(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16728 as u32) ) } as u64;
	// 831ACF60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831ACF64: 4E800421  bctrl
	ctx.lr = 0x831ACF68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831ACF68: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 831ACF6C: 917FEF74  stw r11, -0x108c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4236 as u32), ctx.r[11].u32 ) };
	// 831ACF70: 3FE08335  lis r31, -0x7ccb
	ctx.r[31].s64 = -2093678592;
	// 831ACF74: 807FEF78  lwz r3, -0x1088(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4232 as u32) ) } as u64;
	// 831ACF78: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 831ACF7C: 419A0010  beq cr6, 0x831acf8c
	if ctx.cr[6].eq {
	pc = 0x831ACF8C; continue 'dispatch;
	}
	// 831ACF80: 48095D5D  bl 0x83242cdc
	ctx.lr = 0x831ACF84;
	// extern call 0x83242CDC → crate::xboxkrnl::KeTlsFree
	crate::xboxkrnl::KeTlsFree(ctx, base);
	// 831ACF84: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 831ACF88: 917FEF78  stw r11, -0x1088(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4232 as u32), ctx.r[11].u32 ) };
	// 831ACF8C: 4800D6DD  bl 0x831ba668
	ctx.lr = 0x831ACF90;
	sub_831BA668(ctx, base);
	// 831ACF90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831ACF94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831ACF98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831ACF9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831ACFA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831ACFA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ACFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ACFA8 size=236
    let mut pc: u32 = 0x831ACFA8;
    'dispatch: loop {
        match pc {
            0x831ACFA8 => {
    //   block [0x831ACFA8..0x831AD094)
	// 831ACFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831ACFAC: 4BFFB1BD  bl 0x831a8168
	ctx.lr = 0x831ACFB0;
	sub_831A8130(ctx, base);
	// 831ACFB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831ACFB4: 4BA20D45  bl 0x82bcdcf8
	ctx.lr = 0x831ACFB8;
	sub_82BCDCF8(ctx, base);
	// 831ACFB8: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 831ACFBC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 831ACFC0: 83FEEF74  lwz r31, -0x108c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4236 as u32) ) } as u64;
	// 831ACFC4: 4BFFFF15  bl 0x831aced8
	ctx.lr = 0x831ACFC8;
	sub_831ACED8(ctx, base);
	// 831ACFC8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831ACFCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831ACFD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831ACFD4: 4E800421  bctrl
	ctx.lr = 0x831ACFD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831ACFD8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831ACFDC: 40820098  bne 0x831ad074
	if !ctx.cr[0].eq {
	pc = 0x831AD074; continue 'dispatch;
	}
	// 831ACFE0: 3FA08343  lis r29, -0x7cbd
	ctx.r[29].s64 = -2092761088;
	// 831ACFE4: 807EEF74  lwz r3, -0x108c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4236 as u32) ) } as u64;
	// 831ACFE8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831ACFEC: 817DBEA4  lwz r11, -0x415c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-16732 as u32) ) } as u64;
	// 831ACFF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831ACFF4: 4E800421  bctrl
	ctx.lr = 0x831ACFF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831ACFF8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831ACFFC: 41820084  beq 0x831ad080
	if ctx.cr[0].eq {
	pc = 0x831AD080; continue 'dispatch;
	}
	// 831AD000: 388000C4  li r4, 0xc4
	ctx.r[4].s64 = 196;
	// 831AD004: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831AD008: 48002F21  bl 0x831aff28
	ctx.lr = 0x831AD00C;
	sub_831AFF28(ctx, base);
	// 831AD00C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831AD010: 807EEF74  lwz r3, -0x108c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4236 as u32) ) } as u64;
	// 831AD014: 817DBEA4  lwz r11, -0x415c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-16732 as u32) ) } as u64;
	// 831AD018: 4182004C  beq 0x831ad064
	if ctx.cr[0].eq {
	pc = 0x831AD064; continue 'dispatch;
	}
	// 831AD01C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831AD020: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831AD024: 4E800421  bctrl
	ctx.lr = 0x831AD028;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831AD028: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831AD02C: 4182002C  beq 0x831ad058
	if ctx.cr[0].eq {
	pc = 0x831AD058; continue 'dispatch;
	}
	// 831AD030: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AD034: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 831AD038: 396BF7A0  addi r11, r11, -0x860
	ctx.r[11].s64 = ctx.r[11].s64 + -2144;
	// 831AD03C: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 831AD040: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 831AD044: 48016FDD  bl 0x831c4020
	ctx.lr = 0x831AD048;
	sub_831C4020(ctx, base);
	// 831AD048: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831AD04C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 831AD050: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831AD054: 4800002C  b 0x831ad080
	pc = 0x831AD080; continue 'dispatch;
	// 831AD058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831AD05C: 4BFFEA7D  bl 0x831abad8
	ctx.lr = 0x831AD060;
	sub_831ABAD8(ctx, base);
	// 831AD060: 4800001C  b 0x831ad07c
	pc = 0x831AD07C; continue 'dispatch;
	// 831AD064: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831AD068: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831AD06C: 4E800421  bctrl
	ctx.lr = 0x831AD070;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831AD070: 48000010  b 0x831ad080
	pc = 0x831AD080; continue 'dispatch;
	// 831AD074: 2B1F0001  cmplwi cr6, r31, 1
	ctx.cr[6].compare_u32(ctx.r[31].u32, 1 as u32, &mut ctx.xer);
	// 831AD078: 409A0008  bne cr6, 0x831ad080
	if !ctx.cr[6].eq {
	pc = 0x831AD080; continue 'dispatch;
	}
	// 831AD07C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831AD080: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831AD084: 4BA20B6D  bl 0x82bcdbf0
	ctx.lr = 0x831AD088;
	sub_82BCDBF0(ctx, base);
	// 831AD088: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831AD08C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831AD090: 4BFFB128  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AD098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AD098 size=60
    let mut pc: u32 = 0x831AD098;
    'dispatch: loop {
        match pc {
            0x831AD098 => {
    //   block [0x831AD098..0x831AD0D4)
	// 831AD098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AD09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831AD0A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831AD0A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AD0A8: 4BFFFF01  bl 0x831acfa8
	ctx.lr = 0x831AD0AC;
	sub_831ACFA8(ctx, base);
	// 831AD0AC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831AD0B0: 4082000C  bne 0x831ad0bc
	if !ctx.cr[0].eq {
	pc = 0x831AD0BC; continue 'dispatch;
	}
	// 831AD0B4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 831AD0B8: 4BFFF8A9  bl 0x831ac960
	ctx.lr = 0x831AD0BC;
	sub_831AC960(ctx, base);
	// 831AD0BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831AD0C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831AD0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831AD0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831AD0CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831AD0D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AD0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AD0D8 size=176
    let mut pc: u32 = 0x831AD0D8;
    'dispatch: loop {
        match pc {
            0x831AD0D8 => {
    //   block [0x831AD0D8..0x831AD188)
	// 831AD0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AD0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831AD0E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831AD0E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AD0E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831AD0EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831AD0F0: 419A0084  beq cr6, 0x831ad174
	if ctx.cr[6].eq {
	pc = 0x831AD174; continue 'dispatch;
	}
	// 831AD0F4: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 831AD0F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831AD0FC: 419A0008  beq cr6, 0x831ad104
	if ctx.cr[6].eq {
	pc = 0x831AD104; continue 'dispatch;
	}
	// 831AD100: 4BFFE9D9  bl 0x831abad8
	ctx.lr = 0x831AD104;
	sub_831ABAD8(ctx, base);
	// 831AD104: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 831AD108: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831AD10C: 419A0008  beq cr6, 0x831ad114
	if ctx.cr[6].eq {
	pc = 0x831AD114; continue 'dispatch;
	}
	// 831AD110: 4BFFE9C9  bl 0x831abad8
	ctx.lr = 0x831AD114;
	sub_831ABAD8(ctx, base);
	// 831AD114: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 831AD118: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831AD11C: 419A0008  beq cr6, 0x831ad124
	if ctx.cr[6].eq {
	pc = 0x831AD124; continue 'dispatch;
	}
	// 831AD120: 4BFFE9B9  bl 0x831abad8
	ctx.lr = 0x831AD124;
	sub_831ABAD8(ctx, base);
	// 831AD124: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 831AD128: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831AD12C: 419A0008  beq cr6, 0x831ad134
	if ctx.cr[6].eq {
	pc = 0x831AD134; continue 'dispatch;
	}
	// 831AD130: 4BFFE9A9  bl 0x831abad8
	ctx.lr = 0x831AD134;
	sub_831ABAD8(ctx, base);
	// 831AD134: 807F0044  lwz r3, 0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 831AD138: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831AD13C: 419A0008  beq cr6, 0x831ad144
	if ctx.cr[6].eq {
	pc = 0x831AD144; continue 'dispatch;
	}
	// 831AD140: 4BFFE999  bl 0x831abad8
	ctx.lr = 0x831AD144;
	sub_831ABAD8(ctx, base);
	// 831AD144: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 831AD148: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831AD14C: 419A0008  beq cr6, 0x831ad154
	if ctx.cr[6].eq {
	pc = 0x831AD154; continue 'dispatch;
	}
	// 831AD150: 4BFFE989  bl 0x831abad8
	ctx.lr = 0x831AD154;
	sub_831ABAD8(ctx, base);
	// 831AD154: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AD158: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 831AD15C: 396BF7A0  addi r11, r11, -0x860
	ctx.r[11].s64 = ctx.r[11].s64 + -2144;
	// 831AD160: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831AD164: 419A0008  beq cr6, 0x831ad16c
	if ctx.cr[6].eq {
	pc = 0x831AD16C; continue 'dispatch;
	}
	// 831AD168: 4BFFE971  bl 0x831abad8
	ctx.lr = 0x831AD16C;
	sub_831ABAD8(ctx, base);
	// 831AD16C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831AD170: 4BFFE969  bl 0x831abad8
	ctx.lr = 0x831AD174;
	sub_831ABAD8(ctx, base);
	// 831AD174: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831AD178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831AD17C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831AD180: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831AD184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AD188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AD188 size=152
    let mut pc: u32 = 0x831AD188;
    'dispatch: loop {
        match pc {
            0x831AD188 => {
    //   block [0x831AD188..0x831AD220)
	// 831AD188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AD18C: 4BFFAFE1  bl 0x831a816c
	ctx.lr = 0x831AD190;
	sub_831A8130(ctx, base);
	// 831AD190: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AD194: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 831AD198: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831AD19C: 3FA08335  lis r29, -0x7ccb
	ctx.r[29].s64 = -2093678592;
	// 831AD1A0: 817EEF74  lwz r11, -0x108c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4236 as u32) ) } as u64;
	// 831AD1A4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 831AD1A8: 419A005C  beq cr6, 0x831ad204
	if ctx.cr[6].eq {
	pc = 0x831AD204; continue 'dispatch;
	}
	// 831AD1AC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831AD1B0: 409A0034  bne cr6, 0x831ad1e4
	if !ctx.cr[6].eq {
	pc = 0x831AD1E4; continue 'dispatch;
	}
	// 831AD1B4: 807DEF78  lwz r3, -0x1088(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-4232 as u32) ) } as u64;
	// 831AD1B8: 48095AF5  bl 0x83242cac
	ctx.lr = 0x831AD1BC;
	// extern call 0x83242CAC → crate::xboxkrnl::KeTlsGetValue
	crate::xboxkrnl::KeTlsGetValue(ctx, base);
	// 831AD1BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831AD1C0: 41820024  beq 0x831ad1e4
	if ctx.cr[0].eq {
	pc = 0x831AD1E4; continue 'dispatch;
	}
	// 831AD1C4: 807DEF78  lwz r3, -0x1088(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-4232 as u32) ) } as u64;
	// 831AD1C8: 83FEEF74  lwz r31, -0x108c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4236 as u32) ) } as u64;
	// 831AD1CC: 48095AE1  bl 0x83242cac
	ctx.lr = 0x831AD1D0;
	// extern call 0x83242CAC → crate::xboxkrnl::KeTlsGetValue
	crate::xboxkrnl::KeTlsGetValue(ctx, base);
	// 831AD1D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831AD1D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831AD1D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831AD1DC: 4E800421  bctrl
	ctx.lr = 0x831AD1E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831AD1E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831AD1E4: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 831AD1E8: 807EEF74  lwz r3, -0x108c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4236 as u32) ) } as u64;
	// 831AD1EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831AD1F0: 816BBEA4  lwz r11, -0x415c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16732 as u32) ) } as u64;
	// 831AD1F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831AD1F8: 4E800421  bctrl
	ctx.lr = 0x831AD1FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831AD1FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831AD200: 4BFFFED9  bl 0x831ad0d8
	ctx.lr = 0x831AD204;
	sub_831AD0D8(ctx, base);
	// 831AD204: 807DEF78  lwz r3, -0x1088(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-4232 as u32) ) } as u64;
	// 831AD208: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 831AD20C: 419A000C  beq cr6, 0x831ad218
	if ctx.cr[6].eq {
	pc = 0x831AD218; continue 'dispatch;
	}
	// 831AD210: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831AD214: 48095AA9  bl 0x83242cbc
	ctx.lr = 0x831AD218;
	// extern call 0x83242CBC → crate::xboxkrnl::KeTlsSetValue
	crate::xboxkrnl::KeTlsSetValue(ctx, base);
	// 831AD218: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831AD21C: 4BFFAFA0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AD220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AD220 size=304
    let mut pc: u32 = 0x831AD220;
    'dispatch: loop {
        match pc {
            0x831AD220 => {
    //   block [0x831AD220..0x831AD350)
	// 831AD220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AD224: 4BFFAF49  bl 0x831a816c
	ctx.lr = 0x831AD228;
	sub_831A8130(ctx, base);
	// 831AD228: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AD22C: 3FC08343  lis r30, -0x7cbd
	ctx.r[30].s64 = -2092761088;
	// 831AD230: 3D60831B  lis r11, -0x7ce5
	ctx.r[11].s64 = -2095382528;
	// 831AD234: 3D008324  lis r8, -0x7cdc
	ctx.r[8].s64 = -2094792704;
	// 831AD238: 396BCED0  addi r11, r11, -0x3130
	ctx.r[11].s64 = ctx.r[11].s64 + -12592;
	// 831AD23C: 3CE08343  lis r7, -0x7cbd
	ctx.r[7].s64 = -2092761088;
	// 831AD240: 3FE08343  lis r31, -0x7cbd
	ctx.r[31].s64 = -2092761088;
	// 831AD244: 917EBE9C  stw r11, -0x4164(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-16740 as u32), ctx.r[11].u32 ) };
	// 831AD248: 3FA08343  lis r29, -0x7cbd
	ctx.r[29].s64 = -2092761088;
	// 831AD24C: 3D408324  lis r10, -0x7cdc
	ctx.r[10].s64 = -2094792704;
	// 831AD250: 3D208324  lis r9, -0x7cdc
	ctx.r[9].s64 = -2094792704;
	// 831AD254: 394A2CAC  addi r10, r10, 0x2cac
	ctx.r[10].s64 = ctx.r[10].s64 + 11436;
	// 831AD258: 39292CBC  addi r9, r9, 0x2cbc
	ctx.r[9].s64 = ctx.r[9].s64 + 11452;
	// 831AD25C: 39682CDC  addi r11, r8, 0x2cdc
	ctx.r[11].s64 = ctx.r[8].s64 + 11484;
	// 831AD260: 915FBEA0  stw r10, -0x4160(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-16736 as u32), ctx.r[10].u32 ) };
	// 831AD264: 913DBEA4  stw r9, -0x415c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-16732 as u32), ctx.r[9].u32 ) };
	// 831AD268: 9167BEA8  stw r11, -0x4158(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(-16728 as u32), ctx.r[11].u32 ) };
	// 831AD26C: 48095A61  bl 0x83242ccc
	ctx.lr = 0x831AD270;
	// extern call 0x83242CCC → crate::xboxkrnl::KeTlsAlloc
	crate::xboxkrnl::KeTlsAlloc(ctx, base);
	// 831AD270: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AD274: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 831AD278: 906BEF78  stw r3, -0x1088(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4232 as u32), ctx.r[3].u32 ) };
	// 831AD27C: 419A00C8  beq cr6, 0x831ad344
	if ctx.cr[6].eq {
	pc = 0x831AD344; continue 'dispatch;
	}
	// 831AD280: 809FBEA0  lwz r4, -0x4160(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-16736 as u32) ) } as u64;
	// 831AD284: 48095A39  bl 0x83242cbc
	ctx.lr = 0x831AD288;
	// extern call 0x83242CBC → crate::xboxkrnl::KeTlsSetValue
	crate::xboxkrnl::KeTlsSetValue(ctx, base);
	// 831AD288: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831AD28C: 418200B8  beq 0x831ad344
	if ctx.cr[0].eq {
	pc = 0x831AD344; continue 'dispatch;
	}
	// 831AD290: 4BFFF921  bl 0x831acbb0
	ctx.lr = 0x831AD294;
	sub_831ACBB0(ctx, base);
	// 831AD294: 4800D355  bl 0x831ba5e8
	ctx.lr = 0x831AD298;
	sub_831BA5E8(ctx, base);
	// 831AD298: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831AD29C: 418200A4  beq 0x831ad340
	if ctx.cr[0].eq {
	pc = 0x831AD340; continue 'dispatch;
	}
	// 831AD2A0: 3D60831B  lis r11, -0x7ce5
	ctx.r[11].s64 = -2095382528;
	// 831AD2A4: 815EBE9C  lwz r10, -0x4164(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16740 as u32) ) } as u64;
	// 831AD2A8: 386BD0D8  addi r3, r11, -0x2f28
	ctx.r[3].s64 = ctx.r[11].s64 + -12072;
	// 831AD2AC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831AD2B0: 4E800421  bctrl
	ctx.lr = 0x831AD2B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831AD2B4: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 831AD2B8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 831AD2BC: 907EEF74  stw r3, -0x108c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-4236 as u32), ctx.r[3].u32 ) };
	// 831AD2C0: 419A0080  beq cr6, 0x831ad340
	if ctx.cr[6].eq {
	pc = 0x831AD340; continue 'dispatch;
	}
	// 831AD2C4: 388000C4  li r4, 0xc4
	ctx.r[4].s64 = 196;
	// 831AD2C8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831AD2CC: 48002C5D  bl 0x831aff28
	ctx.lr = 0x831AD2D0;
	sub_831AFF28(ctx, base);
	// 831AD2D0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831AD2D4: 4182006C  beq 0x831ad340
	if ctx.cr[0].eq {
	pc = 0x831AD340; continue 'dispatch;
	}
	// 831AD2D8: 807EEF74  lwz r3, -0x108c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4236 as u32) ) } as u64;
	// 831AD2DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831AD2E0: 817DBEA4  lwz r11, -0x415c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-16732 as u32) ) } as u64;
	// 831AD2E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831AD2E8: 4E800421  bctrl
	ctx.lr = 0x831AD2EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831AD2EC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831AD2F0: 41820050  beq 0x831ad340
	if ctx.cr[0].eq {
	pc = 0x831AD340; continue 'dispatch;
	}
	// 831AD2F4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AD2F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 831AD2FC: 396BF7A0  addi r11, r11, -0x860
	ctx.r[11].s64 = ctx.r[11].s64 + -2144;
	// 831AD300: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 831AD304: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 831AD308: 48016D19  bl 0x831c4020
	ctx.lr = 0x831AD30C;
	sub_831C4020(ctx, base);
	// 831AD30C: 3D408343  lis r10, -0x7cbd
	ctx.r[10].s64 = -2092761088;
	// 831AD310: 3D60831C  lis r11, -0x7ce4
	ctx.r[11].s64 = -2095316992;
	// 831AD314: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 831AD318: 394ABE90  addi r10, r10, -0x4170
	ctx.r[10].s64 = ctx.r[10].s64 + -16752;
	// 831AD31C: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 831AD320: 396BA9E0  addi r11, r11, -0x5620
	ctx.r[11].s64 = ctx.r[11].s64 + -22048;
	// 831AD324: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 831AD328: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 831AD32C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831AD330: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 831AD334: 4BA20EE5  bl 0x82bce218
	ctx.lr = 0x831AD338;
	sub_82BCE218(ctx, base);
	// 831AD338: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831AD33C: 4800000C  b 0x831ad348
	pc = 0x831AD348; continue 'dispatch;
	// 831AD340: 4BFFFBF1  bl 0x831acf30
	ctx.lr = 0x831AD344;
	sub_831ACF30(ctx, base);
	// 831AD344: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AD348: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831AD34C: 4BFFAE70  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AD350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AD350 size=8
    let mut pc: u32 = 0x831AD350;
    'dispatch: loop {
        match pc {
            0x831AD350 => {
    //   block [0x831AD350..0x831AD358)
	// 831AD350: 7C032000  cmpw r3, r4
	ctx.cr[0].compare_i32(ctx.r[4].s32, ctx.r[0].s32, &mut ctx.xer);
	// 831AD354: 4DC20020  beqlr-
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AD358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AD358 size=8
    let mut pc: u32 = 0x831AD358;
    'dispatch: loop {
        match pc {
            0x831AD358 => {
    //   block [0x831AD358..0x831AD360)
	// 831AD358: 40E00008  bge+ 0x831ad360
	if !ctx.cr[0].lt {
		sub_831AD360(ctx, base);
		return;
	}
	// 831AD35C: 4BFFB1B4  b 0x831a8510
	sub_831A8510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AD360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AD360 size=20
    let mut pc: u32 = 0x831AD360;
    'dispatch: loop {
        match pc {
            0x831AD360 => {
    //   block [0x831AD360..0x831AD374)
	// 831AD360: 38050001  addi r0, r5, 1
	ctx.r[0].s64 = ctx.r[5].s64 + 1;
	// 831AD364: 7C632A14  add r3, r3, r5
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[5].u64;
	// 831AD368: 7C842A14  add r4, r4, r5
	ctx.r[4].u64 = ctx.r[4].u64 + ctx.r[5].u64;
	// 831AD36C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831AD370: 48000018  b 0x831ad388
	sub_831AD374(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AD374(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AD374 size=80
    let mut pc: u32 = 0x831AD374;
    'dispatch: loop {
        match pc {
            0x831AD374 => {
    //   block [0x831AD374..0x831AD3C4)
	// 831AD374: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 831AD378: 8804FFFF  lbz r0, -1(r4)
	ctx.r[0].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(-1 as u32) ) } as u64;
	// 831AD37C: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 831AD380: 9803FFFF  stb r0, -1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(-1 as u32), ctx.r[0].u8 ) };
	// 831AD384: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 831AD388: 70600003  andi. r0, r3, 3
	ctx.r[0].u64 = ctx.r[3].u64 & 3;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831AD38C: 4002FFE8  bdnzf eq, 0x831ad374
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 && !ctx.cr[0].eq {
	pc = 0x831AD374; continue 'dispatch;
	}
	// 831AD390: 54A0F0BF  rlwinm. r0, r5, 0x1e, 2, 0x1f
	ctx.r[0].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831AD394: 41C20024  beq- 0x831ad3b8
	if ctx.cr[0].eq {
	pc = 0x831AD3B8; continue 'dispatch;
	}
	// 831AD398: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831AD39C: 70800003  andi. r0, r4, 3
	ctx.r[0].u64 = ctx.r[4].u64 & 3;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831AD3A0: 40C2003C  bne- 0x831ad3dc
	if !ctx.cr[0].eq {
		sub_831AD3DC(ctx, base);
		return;
	}
	// 831AD3A4: 80E4FFFC  lwz r7, -4(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831AD3A8: 3884FFFC  addi r4, r4, -4
	ctx.r[4].s64 = ctx.r[4].s64 + -4;
	// 831AD3AC: 90E3FFFC  stw r7, -4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(-4 as u32), ctx.r[7].u32 ) };
	// 831AD3B0: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 831AD3B4: 4320FFF0  bdnz+ 0x831ad3a4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831AD3A4; continue 'dispatch;
	}
	// 831AD3B8: 70A00003  andi. r0, r5, 3
	ctx.r[0].u64 = ctx.r[5].u64 & 3;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831AD3BC: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831AD3C0: 4DE20020  beqlr+
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AD3C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AD3C4 size=24
    let mut pc: u32 = 0x831AD3C4;
    'dispatch: loop {
        match pc {
            0x831AD3C4 => {
    //   block [0x831AD3C4..0x831AD3DC)
	// 831AD3C4: 8804FFFF  lbz r0, -1(r4)
	ctx.r[0].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(-1 as u32) ) } as u64;
	// 831AD3C8: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 831AD3CC: 9803FFFF  stb r0, -1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(-1 as u32), ctx.r[0].u8 ) };
	// 831AD3D0: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 831AD3D4: 4320FFF0  bdnz+ 0x831ad3c4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831AD3C4; continue 'dispatch;
	}
	// 831AD3D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AD3DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AD3DC size=48
    let mut pc: u32 = 0x831AD3DC;
    'dispatch: loop {
        match pc {
            0x831AD3DC => {
    //   block [0x831AD3DC..0x831AD40C)
	// 831AD3DC: 88E4FFFF  lbz r7, -1(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(-1 as u32) ) } as u64;
	// 831AD3E0: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 831AD3E4: 8904FFFE  lbz r8, -2(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(-2 as u32) ) } as u64;
	// 831AD3E8: 5107442E  rlwimi r7, r8, 8, 0x10, 0x17
	ctx.r[7].u64 = (((ctx.r[8].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[7].u64 & 0xFFFFFFFFFFFF00FF);
	// 831AD3EC: 8924FFFD  lbz r9, -3(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(-3 as u32) ) } as u64;
	// 831AD3F0: 5127821E  rlwimi r7, r9, 0x10, 8, 0xf
	ctx.r[7].u64 = (((ctx.r[9].u32).rotate_left(16) as u64) & 0x0000000000FF0000) | (ctx.r[7].u64 & 0xFFFFFFFFFF00FFFF);
	// 831AD3F4: 8944FFFC  lbz r10, -4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831AD3F8: 5147C00E  rlwimi r7, r10, 0x18, 0, 7
	ctx.r[7].u64 = (((ctx.r[10].u32).rotate_left(24) as u64) & 0x00000000FF000000) | (ctx.r[7].u64 & 0xFFFFFFFF00FFFFFF);
	// 831AD3FC: 3884FFFC  addi r4, r4, -4
	ctx.r[4].s64 = ctx.r[4].s64 + -4;
	// 831AD400: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 831AD404: 4200FFD8  bdnz 0x831ad3dc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831AD3DC; continue 'dispatch;
	}
	// 831AD408: 4BFFFFB0  b 0x831ad3b8
	sub_831AD374(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AD410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AD410 size=940
    let mut pc: u32 = 0x831AD410;
    'dispatch: loop {
        match pc {
            0x831AD410 => {
    //   block [0x831AD410..0x831AD7BC)
	// 831AD410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AD414: 4BFFAD3D  bl 0x831a8150
	ctx.lr = 0x831AD418;
	sub_831A8130(ctx, base);
	// 831AD418: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AD41C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 831AD420: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 831AD424: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831AD428: 409A0038  bne cr6, 0x831ad460
	if !ctx.cr[6].eq {
	pc = 0x831AD460; continue 'dispatch;
	}
	// 831AD42C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831AD430: 419A0030  beq cr6, 0x831ad460
	if ctx.cr[6].eq {
	pc = 0x831AD460; continue 'dispatch;
	}
	// 831AD434: 4800392D  bl 0x831b0d60
	ctx.lr = 0x831AD438;
	sub_831B0D60(ctx, base);
	// 831AD438: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 831AD43C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831AD440: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831AD444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831AD448: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831AD44C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831AD450: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AD454: 480079BD  bl 0x831b4e10
	ctx.lr = 0x831AD458;
	sub_831B4E10(ctx, base);
	// 831AD458: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 831AD45C: 4BFFAD44  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
	// 831AD460: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 831AD464: 419AFFD0  beq cr6, 0x831ad434
	if ctx.cr[6].eq {
	pc = 0x831AD434; continue 'dispatch;
	}
	// 831AD468: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 831AD46C: 419AFFC8  beq cr6, 0x831ad434
	if ctx.cr[6].eq {
	pc = 0x831AD434; continue 'dispatch;
	}
	// 831AD470: 2B040002  cmplwi cr6, r4, 2
	ctx.cr[6].compare_u32(ctx.r[4].u32, 2 as u32, &mut ctx.xer);
	// 831AD474: 4198FFE4  blt cr6, 0x831ad458
	if ctx.cr[6].lt {
	pc = 0x831AD458; continue 'dispatch;
	}
	// 831AD478: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 831AD47C: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 831AD480: 7D6BD9D6  mullw r11, r11, r27
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[27].s32 as i64);
	// 831AD484: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 831AD488: 7F8B1A14  add r28, r11, r3
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 831AD48C: 3AE10050  addi r23, r1, 0x50
	ctx.r[23].s64 = ctx.r[1].s64 + 80;
	// 831AD490: 3B0100D0  addi r24, r1, 0xd0
	ctx.r[24].s64 = ctx.r[1].s64 + 208;
	// 831AD494: 7D7AE050  subf r11, r26, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[26].s64;
	// 831AD498: 0CDB0000  twi 6, r27, 0
	// 831AD49C: 7D6BDB96  divwu r11, r11, r27
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[27].u32;
	// 831AD4A0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831AD4A4: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 831AD4A8: 419900A4  bgt cr6, 0x831ad54c
	if ctx.cr[6].gt {
	pc = 0x831AD54C; continue 'dispatch;
	}
	// 831AD4AC: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 831AD4B0: 7F1CD040  cmplw cr6, r28, r26
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[26].u32, &mut ctx.xer);
	// 831AD4B4: 4099007C  ble cr6, 0x831ad530
	if !ctx.cr[6].gt {
	pc = 0x831AD530; continue 'dispatch;
	}
	// 831AD4B8: 7F9ADA14  add r28, r26, r27
	ctx.r[28].u64 = ctx.r[26].u64 + ctx.r[27].u64;
	// 831AD4BC: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 831AD4C0: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 831AD4C4: 7F1CF040  cmplw cr6, r28, r30
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[30].u32, &mut ctx.xer);
	// 831AD4C8: 4199002C  bgt cr6, 0x831ad4f4
	if ctx.cr[6].gt {
	pc = 0x831AD4F4; continue 'dispatch;
	}
	// 831AD4CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831AD4D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831AD4D4: 7F2903A6  mtctr r25
	ctx.ctr.u64 = ctx.r[25].u64;
	// 831AD4D8: 4E800421  bctrl
	ctx.lr = 0x831AD4DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831AD4DC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831AD4E0: 40990008  ble cr6, 0x831ad4e8
	if !ctx.cr[6].gt {
	pc = 0x831AD4E8; continue 'dispatch;
	}
	// 831AD4E4: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 831AD4E8: 7FFFDA14  add r31, r31, r27
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[27].u64;
	// 831AD4EC: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 831AD4F0: 4099FFDC  ble cr6, 0x831ad4cc
	if !ctx.cr[6].gt {
	pc = 0x831AD4CC; continue 'dispatch;
	}
	// 831AD4F4: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 831AD4F8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 831AD4FC: 7F1DF040  cmplw cr6, r29, r30
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[30].u32, &mut ctx.xer);
	// 831AD500: 419A0024  beq cr6, 0x831ad524
	if ctx.cr[6].eq {
	pc = 0x831AD524; continue 'dispatch;
	}
	// 831AD504: 7D3EE850  subf r9, r30, r29
	ctx.r[9].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 831AD508: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AD50C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AD510: 7CE958AE  lbzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831AD514: 7D0959AE  stbx r8, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u8) };
	// 831AD518: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 831AD51C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831AD520: 4082FFE8  bne 0x831ad508
	if !ctx.cr[0].eq {
	pc = 0x831AD508; continue 'dispatch;
	}
	// 831AD524: 7FDBF050  subf r30, r27, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[27].s64;
	// 831AD528: 7F1ED040  cmplw cr6, r30, r26
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[26].u32, &mut ctx.xer);
	// 831AD52C: 4199FF90  bgt cr6, 0x831ad4bc
	if ctx.cr[6].gt {
	pc = 0x831AD4BC; continue 'dispatch;
	}
	// 831AD530: 36D6FFFF  addic. r22, r22, -1
	ctx.xer.ca = (ctx.r[22].u32 > (!(-1 as u32)));
	ctx.r[22].s64 = ctx.r[22].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 831AD534: 3B18FFFC  addi r24, r24, -4
	ctx.r[24].s64 = ctx.r[24].s64 + -4;
	// 831AD538: 3AF7FFFC  addi r23, r23, -4
	ctx.r[23].s64 = ctx.r[23].s64 + -4;
	// 831AD53C: 4180FF1C  blt 0x831ad458
	if ctx.cr[0].lt {
	pc = 0x831AD458; continue 'dispatch;
	}
	// 831AD540: 83580000  lwz r26, 0(r24)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AD544: 83970000  lwz r28, 0(r23)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AD548: 4BFFFF4C  b 0x831ad494
	pc = 0x831AD494; continue 'dispatch;
	// 831AD54C: 556BF87E  srwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831AD550: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 831AD554: 7D6BD9D6  mullw r11, r11, r27
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[27].s32 as i64);
	// 831AD558: 7FABD214  add r29, r11, r26
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 831AD55C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831AD560: 7F2903A6  mtctr r25
	ctx.ctr.u64 = ctx.r[25].u64;
	// 831AD564: 4E800421  bctrl
	ctx.lr = 0x831AD568;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831AD568: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831AD56C: 40990034  ble cr6, 0x831ad5a0
	if !ctx.cr[6].gt {
	pc = 0x831AD5A0; continue 'dispatch;
	}
	// 831AD570: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 831AD574: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 831AD578: 7F1AE840  cmplw cr6, r26, r29
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[29].u32, &mut ctx.xer);
	// 831AD57C: 419A0024  beq cr6, 0x831ad5a0
	if ctx.cr[6].eq {
	pc = 0x831AD5A0; continue 'dispatch;
	}
	// 831AD580: 7D3DD050  subf r9, r29, r26
	ctx.r[9].s64 = ctx.r[26].s64 - ctx.r[29].s64;
	// 831AD584: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AD588: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AD58C: 7CE958AE  lbzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831AD590: 7D0959AE  stbx r8, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u8) };
	// 831AD594: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 831AD598: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831AD59C: 4082FFE8  bne 0x831ad584
	if !ctx.cr[0].eq {
	pc = 0x831AD584; continue 'dispatch;
	}
	// 831AD5A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831AD5A4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 831AD5A8: 7F2903A6  mtctr r25
	ctx.ctr.u64 = ctx.r[25].u64;
	// 831AD5AC: 4E800421  bctrl
	ctx.lr = 0x831AD5B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831AD5B0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831AD5B4: 40990034  ble cr6, 0x831ad5e8
	if !ctx.cr[6].gt {
	pc = 0x831AD5E8; continue 'dispatch;
	}
	// 831AD5B8: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 831AD5BC: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 831AD5C0: 7F1AE040  cmplw cr6, r26, r28
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[28].u32, &mut ctx.xer);
	// 831AD5C4: 419A0024  beq cr6, 0x831ad5e8
	if ctx.cr[6].eq {
	pc = 0x831AD5E8; continue 'dispatch;
	}
	// 831AD5C8: 7D3CD050  subf r9, r28, r26
	ctx.r[9].s64 = ctx.r[26].s64 - ctx.r[28].s64;
	// 831AD5CC: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AD5D0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AD5D4: 7CE958AE  lbzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831AD5D8: 7D0959AE  stbx r8, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u8) };
	// 831AD5DC: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 831AD5E0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831AD5E4: 4082FFE8  bne 0x831ad5cc
	if !ctx.cr[0].eq {
	pc = 0x831AD5CC; continue 'dispatch;
	}
	// 831AD5E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831AD5EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831AD5F0: 7F2903A6  mtctr r25
	ctx.ctr.u64 = ctx.r[25].u64;
	// 831AD5F4: 4E800421  bctrl
	ctx.lr = 0x831AD5F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831AD5F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831AD5FC: 40990034  ble cr6, 0x831ad630
	if !ctx.cr[6].gt {
	pc = 0x831AD630; continue 'dispatch;
	}
	// 831AD600: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 831AD604: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 831AD608: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 831AD60C: 419A0024  beq cr6, 0x831ad630
	if ctx.cr[6].eq {
	pc = 0x831AD630; continue 'dispatch;
	}
	// 831AD610: 7D3CE850  subf r9, r28, r29
	ctx.r[9].s64 = ctx.r[29].s64 - ctx.r[28].s64;
	// 831AD614: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AD618: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AD61C: 7CE958AE  lbzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831AD620: 7D0959AE  stbx r8, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u8) };
	// 831AD624: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 831AD628: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831AD62C: 4082FFE8  bne 0x831ad614
	if !ctx.cr[0].eq {
	pc = 0x831AD614; continue 'dispatch;
	}
	// 831AD630: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 831AD634: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 831AD638: 7F1DF840  cmplw cr6, r29, r31
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[31].u32, &mut ctx.xer);
	// 831AD63C: 40990030  ble cr6, 0x831ad66c
	if !ctx.cr[6].gt {
	pc = 0x831AD66C; continue 'dispatch;
	}
	// 831AD640: 7FFFDA14  add r31, r31, r27
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[27].u64;
	// 831AD644: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 831AD648: 40980024  bge cr6, 0x831ad66c
	if !ctx.cr[6].lt {
	pc = 0x831AD66C; continue 'dispatch;
	}
	// 831AD64C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831AD650: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831AD654: 7F2903A6  mtctr r25
	ctx.ctr.u64 = ctx.r[25].u64;
	// 831AD658: 4E800421  bctrl
	ctx.lr = 0x831AD65C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831AD65C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831AD660: 4099FFE0  ble cr6, 0x831ad640
	if !ctx.cr[6].gt {
	pc = 0x831AD640; continue 'dispatch;
	}
	// 831AD664: 7F1DF840  cmplw cr6, r29, r31
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[31].u32, &mut ctx.xer);
	// 831AD668: 41990028  bgt cr6, 0x831ad690
	if ctx.cr[6].gt {
	pc = 0x831AD690; continue 'dispatch;
	}
	// 831AD66C: 7FFFDA14  add r31, r31, r27
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[27].u64;
	// 831AD670: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 831AD674: 4199001C  bgt cr6, 0x831ad690
	if ctx.cr[6].gt {
	pc = 0x831AD690; continue 'dispatch;
	}
	// 831AD678: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831AD67C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831AD680: 7F2903A6  mtctr r25
	ctx.ctr.u64 = ctx.r[25].u64;
	// 831AD684: 4E800421  bctrl
	ctx.lr = 0x831AD688;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831AD688: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831AD68C: 4099FFE0  ble cr6, 0x831ad66c
	if !ctx.cr[6].gt {
	pc = 0x831AD66C; continue 'dispatch;
	}
	// 831AD690: 7FDBF050  subf r30, r27, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[27].s64;
	// 831AD694: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 831AD698: 4099001C  ble cr6, 0x831ad6b4
	if !ctx.cr[6].gt {
	pc = 0x831AD6B4; continue 'dispatch;
	}
	// 831AD69C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831AD6A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831AD6A4: 7F2903A6  mtctr r25
	ctx.ctr.u64 = ctx.r[25].u64;
	// 831AD6A8: 4E800421  bctrl
	ctx.lr = 0x831AD6AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831AD6AC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831AD6B0: 4199FFE0  bgt cr6, 0x831ad690
	if ctx.cr[6].gt {
	pc = 0x831AD690; continue 'dispatch;
	}
	// 831AD6B4: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 831AD6B8: 41990040  bgt cr6, 0x831ad6f8
	if ctx.cr[6].gt {
	pc = 0x831AD6F8; continue 'dispatch;
	}
	// 831AD6BC: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 831AD6C0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 831AD6C4: 419A0024  beq cr6, 0x831ad6e8
	if ctx.cr[6].eq {
	pc = 0x831AD6E8; continue 'dispatch;
	}
	// 831AD6C8: 7D3EF850  subf r9, r30, r31
	ctx.r[9].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 831AD6CC: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AD6D0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AD6D4: 7CE958AE  lbzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831AD6D8: 7D0959AE  stbx r8, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u8) };
	// 831AD6DC: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 831AD6E0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831AD6E4: 4082FFE8  bne 0x831ad6cc
	if !ctx.cr[0].eq {
	pc = 0x831AD6CC; continue 'dispatch;
	}
	// 831AD6E8: 7F1DF040  cmplw cr6, r29, r30
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[30].u32, &mut ctx.xer);
	// 831AD6EC: 409AFF4C  bne cr6, 0x831ad638
	if !ctx.cr[6].eq {
	pc = 0x831AD638; continue 'dispatch;
	}
	// 831AD6F0: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 831AD6F4: 4BFFFF44  b 0x831ad638
	pc = 0x831AD638; continue 'dispatch;
	// 831AD6F8: 7FDEDA14  add r30, r30, r27
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[27].u64;
	// 831AD6FC: 7F1DF040  cmplw cr6, r29, r30
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[30].u32, &mut ctx.xer);
	// 831AD700: 40980030  bge cr6, 0x831ad730
	if !ctx.cr[6].lt {
	pc = 0x831AD730; continue 'dispatch;
	}
	// 831AD704: 7FDBF050  subf r30, r27, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[27].s64;
	// 831AD708: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 831AD70C: 40990024  ble cr6, 0x831ad730
	if !ctx.cr[6].gt {
	pc = 0x831AD730; continue 'dispatch;
	}
	// 831AD710: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831AD714: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831AD718: 7F2903A6  mtctr r25
	ctx.ctr.u64 = ctx.r[25].u64;
	// 831AD71C: 4E800421  bctrl
	ctx.lr = 0x831AD720;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831AD720: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831AD724: 419AFFE0  beq cr6, 0x831ad704
	if ctx.cr[6].eq {
	pc = 0x831AD704; continue 'dispatch;
	}
	// 831AD728: 7F1DF040  cmplw cr6, r29, r30
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[30].u32, &mut ctx.xer);
	// 831AD72C: 41980028  blt cr6, 0x831ad754
	if ctx.cr[6].lt {
	pc = 0x831AD754; continue 'dispatch;
	}
	// 831AD730: 7FDBF050  subf r30, r27, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[27].s64;
	// 831AD734: 7F1ED040  cmplw cr6, r30, r26
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[26].u32, &mut ctx.xer);
	// 831AD738: 4099001C  ble cr6, 0x831ad754
	if !ctx.cr[6].gt {
	pc = 0x831AD754; continue 'dispatch;
	}
	// 831AD73C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831AD740: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831AD744: 7F2903A6  mtctr r25
	ctx.ctr.u64 = ctx.r[25].u64;
	// 831AD748: 4E800421  bctrl
	ctx.lr = 0x831AD74C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831AD74C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831AD750: 419AFFE0  beq cr6, 0x831ad730
	if ctx.cr[6].eq {
	pc = 0x831AD730; continue 'dispatch;
	}
	// 831AD754: 7D7FE050  subf r11, r31, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[31].s64;
	// 831AD758: 7D5AF050  subf r10, r26, r30
	ctx.r[10].s64 = ctx.r[30].s64 - ctx.r[26].s64;
	// 831AD75C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831AD760: 41980030  blt cr6, 0x831ad790
	if ctx.cr[6].lt {
	pc = 0x831AD790; continue 'dispatch;
	}
	// 831AD764: 7F1AF040  cmplw cr6, r26, r30
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[30].u32, &mut ctx.xer);
	// 831AD768: 40980018  bge cr6, 0x831ad780
	if !ctx.cr[6].lt {
	pc = 0x831AD780; continue 'dispatch;
	}
	// 831AD76C: 93580000  stw r26, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 831AD770: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 831AD774: 93D70000  stw r30, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 831AD778: 3B180004  addi r24, r24, 4
	ctx.r[24].s64 = ctx.r[24].s64 + 4;
	// 831AD77C: 3AF70004  addi r23, r23, 4
	ctx.r[23].s64 = ctx.r[23].s64 + 4;
	// 831AD780: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 831AD784: 4098FDAC  bge cr6, 0x831ad530
	if !ctx.cr[6].lt {
	pc = 0x831AD530; continue 'dispatch;
	}
	// 831AD788: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 831AD78C: 4BFFFD08  b 0x831ad494
	pc = 0x831AD494; continue 'dispatch;
	// 831AD790: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 831AD794: 40980018  bge cr6, 0x831ad7ac
	if !ctx.cr[6].lt {
	pc = 0x831AD7AC; continue 'dispatch;
	}
	// 831AD798: 93F80000  stw r31, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 831AD79C: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 831AD7A0: 93970000  stw r28, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 831AD7A4: 3B180004  addi r24, r24, 4
	ctx.r[24].s64 = ctx.r[24].s64 + 4;
	// 831AD7A8: 3AF70004  addi r23, r23, 4
	ctx.r[23].s64 = ctx.r[23].s64 + 4;
	// 831AD7AC: 7F1AF040  cmplw cr6, r26, r30
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[30].u32, &mut ctx.xer);
	// 831AD7B0: 4098FD80  bge cr6, 0x831ad530
	if !ctx.cr[6].lt {
	pc = 0x831AD530; continue 'dispatch;
	}
	// 831AD7B4: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 831AD7B8: 4BFFFCDC  b 0x831ad494
	pc = 0x831AD494; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AD7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AD7C0 size=40
    let mut pc: u32 = 0x831AD7C0;
    'dispatch: loop {
        match pc {
            0x831AD7C0 => {
    //   block [0x831AD7C0..0x831AD7E8)
	// 831AD7C0: 7C650074  cntlzd r5, r3
	ctx.r[5].u64 = if ctx.r[3].u64 == 0 { 64 } else { ctx.r[3].u64.leading_zeros() as u64 };
	// 831AD7C4: 7C632836  sld r3, r3, r5
	if (ctx.r[5].u8 & 0x40) != 0 {
		ctx.r[3].u64 = 0;
	} else {
		ctx.r[3].u64 = (ctx.r[3].u64) << ((ctx.r[5].u8 & 0x3F) as u32);
	}
	// 831AD7C8: 2C230000  cmpdi r3, 0
	ctx.cr[0].compare_i64(ctx.r[0].s64, 0, &mut ctx.xer);
	// 831AD7CC: 41820010  beq 0x831ad7dc
	if ctx.cr[0].eq {
	pc = 0x831AD7DC; continue 'dispatch;
	}
	// 831AD7D0: 20A5043E  subfic r5, r5, 0x43e
	ctx.xer.ca = ctx.r[5].u32 <= 1086 as u32;
	ctx.r[5].s64 = (1086 as i64) - ctx.r[5].s64;
	// 831AD7D4: 7863AB02  rldicl r3, r3, 0x35, 0xc
	ctx.r[3].u64 = ctx.r[3].u64 & 0x00000000000007FFu64;
	// 831AD7D8: 78A3A04E  rldimi r3, r5, 0x34, 1
	ctx.r[3].u64 = ((ctx.r[5].u64).rotate_left(52) & 0x7FF0000000000000) | (ctx.r[3].u64 & 0x800FFFFFFFFFFFFF);
	// 831AD7DC: F861FFF8  std r3, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[3].u64 ) };
	// 831AD7E0: C821FFF8  lfd f1, -8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 831AD7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AD7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AD7E8 size=468
    let mut pc: u32 = 0x831AD7E8;
    'dispatch: loop {
        match pc {
            0x831AD7E8 => {
    //   block [0x831AD7E8..0x831AD9BC)
	// 831AD7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AD7EC: 4BFFA981  bl 0x831a816c
	ctx.lr = 0x831AD7F0;
	sub_831A8130(ctx, base);
	// 831AD7F0: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 831AD7F4: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 831AD7F8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AD7FC: 3D60C007  lis r11, -0x3ff9
	ctx.r[11].s64 = -1073283072;
	// 831AD800: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 831AD804: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831AD808: DBE100A0  stfd f31, 0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.f[31].u64 ) };
	// 831AD80C: 617DFEFF  ori r29, r11, 0xfeff
	ctx.r[29].u64 = ctx.r[11].u64 | 65279;
	// 831AD810: 386000F8  li r3, 0xf8
	ctx.r[3].s64 = 248;
	// 831AD814: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831AD818: 4800B789  bl 0x831b8fa0
	ctx.lr = 0x831AD81C;
	sub_831B8FA0(ctx, base);
	// 831AD81C: A16100A0  lhz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 831AD820: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831AD824: 556B0476  rlwinm r11, r11, 0, 0x11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AD828: 2B0B7FF0  cmplwi cr6, r11, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32752 as u32, &mut ctx.xer);
	// 831AD82C: 409A007C  bne cr6, 0x831ad8a8
	if !ctx.cr[6].eq {
	pc = 0x831AD8A8; continue 'dispatch;
	}
	// 831AD830: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 831AD834: 48007725  bl 0x831b4f58
	ctx.lr = 0x831AD838;
	sub_831B4F58(ctx, base);
	// 831AD838: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831AD83C: 40810038  ble 0x831ad874
	if !ctx.cr[0].gt {
	pc = 0x831AD874; continue 'dispatch;
	}
	// 831AD840: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 831AD844: 40990158  ble cr6, 0x831ad99c
	if !ctx.cr[6].gt {
	pc = 0x831AD99C; continue 'dispatch;
	}
	// 831AD848: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 831AD84C: 409A0028  bne cr6, 0x831ad874
	if !ctx.cr[6].eq {
	pc = 0x831AD874; continue 'dispatch;
	}
	// 831AD850: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 831AD854: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 831AD858: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 831AD85C: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 831AD860: 38600019  li r3, 0x19
	ctx.r[3].s64 = 25;
	// 831AD864: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831AD868: FC40069C  fcfid f2, f0
	ctx.f[2].f64 = (ctx.f[0].s64 as f64);
	// 831AD86C: 4800B4ED  bl 0x831b8d58
	ctx.lr = 0x831AD870;
	sub_831B8D58(ctx, base);
	// 831AD870: 4800013C  b 0x831ad9ac
	pc = 0x831AD9AC; continue 'dispatch;
	// 831AD874: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 831AD878: 7FEA07B4  extsw r10, r31
	ctx.r[10].s64 = ctx.r[31].s32 as i64;
	// 831AD87C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 831AD880: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 831AD884: C9A10058  lfd f13, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831AD888: FC406E9C  fcfid f2, f13
	ctx.f[2].f64 = (ctx.f[13].s64 as f64);
	// 831AD88C: C80BE3A0  lfd f0, -0x1c60(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-7264 as u32) ) };
	// 831AD890: FC7F002A  fadd f3, f31, f0
	ctx.f[3].f64 = ctx.f[31].f64 + ctx.f[0].f64;
	// 831AD894: 38800019  li r4, 0x19
	ctx.r[4].s64 = 25;
	// 831AD898: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 831AD89C: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 831AD8A0: 4800B611  bl 0x831b8eb0
	ctx.lr = 0x831AD8A4;
	sub_831B8EB0(ctx, base);
	// 831AD8A4: 48000108  b 0x831ad9ac
	pc = 0x831AD9AC; continue 'dispatch;
	// 831AD8A8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 831AD8AC: CBCBD228  lfd f30, -0x2dd8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11736 as u32) ) };
	// 831AD8B0: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 831AD8B4: 419A00E8  beq cr6, 0x831ad99c
	if ctx.cr[6].eq {
	pc = 0x831AD99C; continue 'dispatch;
	}
	// 831AD8B8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831AD8BC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 831AD8C0: 48007721  bl 0x831b4fe0
	ctx.lr = 0x831AD8C4;
	sub_831B4FE0(ctx, base);
	// 831AD8C4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831AD8C8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831AD8CC: 40980018  bge cr6, 0x831ad8e4
	if !ctx.cr[6].lt {
	pc = 0x831AD8E4; continue 'dispatch;
	}
	// 831AD8D0: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 831AD8D4: 7D5F5050  subf r10, r31, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[31].s64;
	// 831AD8D8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 831AD8DC: 4198007C  blt cr6, 0x831ad958
	if ctx.cr[6].lt {
	pc = 0x831AD958; continue 'dispatch;
	}
	// 831AD8E0: 48000018  b 0x831ad8f8
	pc = 0x831AD8F8; continue 'dispatch;
	// 831AD8E4: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 831AD8E8: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 831AD8EC: 7D5F5050  subf r10, r31, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[31].s64;
	// 831AD8F0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 831AD8F4: 41990010  bgt cr6, 0x831ad904
	if ctx.cr[6].gt {
	pc = 0x831AD904; continue 'dispatch;
	}
	// 831AD8F8: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 831AD8FC: 2F040A00  cmpwi cr6, r4, 0xa00
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2560, &mut ctx.xer);
	// 831AD900: 40990030  ble cr6, 0x831ad930
	if !ctx.cr[6].gt {
	pc = 0x831AD930; continue 'dispatch;
	}
	// 831AD904: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AD908: FC400890  fmr f2, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[1].f64;
	// 831AD90C: C82BF468  lfd f1, -0xb98(r11)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-2968 as u32) ) };
	// 831AD910: 48000C39  bl 0x831ae548
	ctx.lr = 0x831AD914;
	sub_831AE548(ctx, base);
	// 831AD914: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 831AD918: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 831AD91C: C8010058  lfd f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831AD920: 38600011  li r3, 0x11
	ctx.r[3].s64 = 17;
	// 831AD924: FC600890  fmr f3, f1
	ctx.f[3].f64 = ctx.f[1].f64;
	// 831AD928: FC40069C  fcfid f2, f0
	ctx.f[2].f64 = (ctx.f[0].s64 as f64);
	// 831AD92C: 4BFFFF68  b 0x831ad894
	pc = 0x831AD894; continue 'dispatch;
	// 831AD930: 2F040400  cmpwi cr6, r4, 0x400
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1024, &mut ctx.xer);
	// 831AD934: 4099001C  ble cr6, 0x831ad950
	if !ctx.cr[6].gt {
	pc = 0x831AD950; continue 'dispatch;
	}
	// 831AD938: 3884FA00  addi r4, r4, -0x600
	ctx.r[4].s64 = ctx.r[4].s64 + -1536;
	// 831AD93C: 480075A5  bl 0x831b4ee0
	ctx.lr = 0x831AD940;
	sub_831B4EE0(ctx, base);
	// 831AD940: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 831AD944: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 831AD948: C8010058  lfd f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831AD94C: 4BFFFFD4  b 0x831ad920
	pc = 0x831AD920; continue 'dispatch;
	// 831AD950: 2F04F603  cmpwi cr6, r4, -0x9fd
	ctx.cr[6].compare_i32(ctx.r[4].s32, -2557, &mut ctx.xer);
	// 831AD954: 4098001C  bge cr6, 0x831ad970
	if !ctx.cr[6].lt {
	pc = 0x831AD970; continue 'dispatch;
	}
	// 831AD958: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 831AD95C: FC6107B2  fmul f3, f1, f30
	ctx.f[3].f64 = ctx.f[1].f64 * ctx.f[30].f64;
	// 831AD960: 38600012  li r3, 0x12
	ctx.r[3].s64 = 18;
	// 831AD964: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 831AD968: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831AD96C: 4BFFFFBC  b 0x831ad928
	pc = 0x831AD928; continue 'dispatch;
	// 831AD970: 2F04FC03  cmpwi cr6, r4, -0x3fd
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1021, &mut ctx.xer);
	// 831AD974: 40980020  bge cr6, 0x831ad994
	if !ctx.cr[6].lt {
	pc = 0x831AD994; continue 'dispatch;
	}
	// 831AD978: 38840600  addi r4, r4, 0x600
	ctx.r[4].s64 = ctx.r[4].s64 + 1536;
	// 831AD97C: 48007565  bl 0x831b4ee0
	ctx.lr = 0x831AD980;
	sub_831B4EE0(ctx, base);
	// 831AD980: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 831AD984: 38600012  li r3, 0x12
	ctx.r[3].s64 = 18;
	// 831AD988: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 831AD98C: C8010058  lfd f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831AD990: 4BFFFF94  b 0x831ad924
	pc = 0x831AD924; continue 'dispatch;
	// 831AD994: 4800754D  bl 0x831b4ee0
	ctx.lr = 0x831AD998;
	sub_831B4EE0(ctx, base);
	// 831AD998: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 831AD99C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831AD9A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831AD9A4: 4800B5FD  bl 0x831b8fa0
	ctx.lr = 0x831AD9A8;
	sub_831B8FA0(ctx, base);
	// 831AD9A8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 831AD9AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831AD9B0: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 831AD9B4: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 831AD9B8: 4BFFA804  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AD9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AD9C0 size=28
    let mut pc: u32 = 0x831AD9C0;
    'dispatch: loop {
        match pc {
            0x831AD9C0 => {
    //   block [0x831AD9C0..0x831AD9DC)
	// 831AD9C0: 546A07BF  clrlwi. r10, r3, 0x1e
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AD9C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831AD9C8: 41820030  beq 0x831ad9f8
	if ctx.cr[0].eq {
		sub_831AD9EC(ctx, base);
		return;
	}
	// 831AD9CC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 831AD9D0: 409A000C  bne cr6, 0x831ad9dc
	if !ctx.cr[6].eq {
		sub_831AD9DC(ctx, base);
		return;
	}
	// 831AD9D4: 39600300  li r11, 0x300
	ctx.r[11].s64 = 768;
	// 831AD9D8: 48000020  b 0x831ad9f8
	sub_831AD9EC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AD9DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AD9DC size=16
    let mut pc: u32 = 0x831AD9DC;
    'dispatch: loop {
        match pc {
            0x831AD9DC => {
    //   block [0x831AD9DC..0x831AD9EC)
	// 831AD9DC: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 831AD9E0: 409A000C  bne cr6, 0x831ad9ec
	if !ctx.cr[6].eq {
		sub_831AD9EC(ctx, base);
		return;
	}
	// 831AD9E4: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 831AD9E8: 48000010  b 0x831ad9f8
	sub_831AD9EC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AD9EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AD9EC size=80
    let mut pc: u32 = 0x831AD9EC;
    'dispatch: loop {
        match pc {
            0x831AD9EC => {
    //   block [0x831AD9EC..0x831ADA3C)
	// 831AD9EC: 2B0A0003  cmplwi cr6, r10, 3
	ctx.cr[6].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	// 831AD9F0: 409A0008  bne cr6, 0x831ad9f8
	if !ctx.cr[6].eq {
	pc = 0x831AD9F8; continue 'dispatch;
	}
	// 831AD9F4: 39600100  li r11, 0x100
	ctx.r[11].s64 = 256;
	// 831AD9F8: 546A0739  rlwinm. r10, r3, 0, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AD9FC: 41820008  beq 0x831ada04
	if ctx.cr[0].eq {
	pc = 0x831ADA04; continue 'dispatch;
	}
	// 831ADA00: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 831ADA04: 546A06B5  rlwinm. r10, r3, 0, 0x1a, 0x1a
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831ADA08: 41820008  beq 0x831ada10
	if ctx.cr[0].eq {
	pc = 0x831ADA10; continue 'dispatch;
	}
	// 831ADA0C: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 831ADA10: 546A0673  rlwinm. r10, r3, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831ADA14: 41820008  beq 0x831ada1c
	if ctx.cr[0].eq {
	pc = 0x831ADA1C; continue 'dispatch;
	}
	// 831ADA18: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 831ADA1C: 546A06F7  rlwinm. r10, r3, 0, 0x1b, 0x1b
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831ADA20: 41820008  beq 0x831ada28
	if ctx.cr[0].eq {
	pc = 0x831ADA28; continue 'dispatch;
	}
	// 831ADA24: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 831ADA28: 546A0631  rlwinm. r10, r3, 0, 0x18, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831ADA2C: 41820008  beq 0x831ada34
	if ctx.cr[0].eq {
	pc = 0x831ADA34; continue 'dispatch;
	}
	// 831ADA30: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 831ADA34: 65630008  oris r3, r11, 8
	ctx.r[3].u64 = ctx.r[11].u64 | 524288;
	// 831ADA38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ADA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ADA40 size=356
    let mut pc: u32 = 0x831ADA40;
    'dispatch: loop {
        match pc {
            0x831ADA40 => {
    //   block [0x831ADA40..0x831ADBA4)
	// 831ADA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831ADA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831ADA48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831ADA4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831ADA50: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 831ADA54: 548805AE  rlwinm r8, r4, 0, 0x16, 0x17
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 831ADA58: 554B05AF  rlwinm. r11, r10, 0, 0x16, 0x17
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831ADA5C: 5489062A  rlwinm r9, r4, 0, 0x18, 0x15
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 831ADA60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ADA64: 41820030  beq 0x831ada94
	if ctx.cr[0].eq {
	pc = 0x831ADA94; continue 'dispatch;
	}
	// 831ADA68: 2B0B0300  cmplwi cr6, r11, 0x300
	ctx.cr[6].compare_u32(ctx.r[11].u32, 768 as u32, &mut ctx.xer);
	// 831ADA6C: 409A000C  bne cr6, 0x831ada78
	if !ctx.cr[6].eq {
	pc = 0x831ADA78; continue 'dispatch;
	}
	// 831ADA70: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831ADA74: 48000020  b 0x831ada94
	pc = 0x831ADA94; continue 'dispatch;
	// 831ADA78: 2B0B0200  cmplwi cr6, r11, 0x200
	ctx.cr[6].compare_u32(ctx.r[11].u32, 512 as u32, &mut ctx.xer);
	// 831ADA7C: 409A000C  bne cr6, 0x831ada88
	if !ctx.cr[6].eq {
	pc = 0x831ADA88; continue 'dispatch;
	}
	// 831ADA80: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 831ADA84: 48000010  b 0x831ada94
	pc = 0x831ADA94; continue 'dispatch;
	// 831ADA88: 2B0B0100  cmplwi cr6, r11, 0x100
	ctx.cr[6].compare_u32(ctx.r[11].u32, 256 as u32, &mut ctx.xer);
	// 831ADA8C: 409A0008  bne cr6, 0x831ada94
	if !ctx.cr[6].eq {
	pc = 0x831ADA94; continue 'dispatch;
	}
	// 831ADA90: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 831ADA94: 554B07FF  clrlwi. r11, r10, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831ADA98: 41820008  beq 0x831adaa0
	if ctx.cr[0].eq {
	pc = 0x831ADAA0; continue 'dispatch;
	}
	// 831ADA9C: 60630008  ori r3, r3, 8
	ctx.r[3].u64 = ctx.r[3].u64 | 8;
	// 831ADAA0: 554B07BD  rlwinm. r11, r10, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831ADAA4: 41820008  beq 0x831adaac
	if ctx.cr[0].eq {
	pc = 0x831ADAAC; continue 'dispatch;
	}
	// 831ADAA8: 60630020  ori r3, r3, 0x20
	ctx.r[3].u64 = ctx.r[3].u64 | 32;
	// 831ADAAC: 554B077B  rlwinm. r11, r10, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831ADAB0: 41820008  beq 0x831adab8
	if ctx.cr[0].eq {
	pc = 0x831ADAB8; continue 'dispatch;
	}
	// 831ADAB4: 60630040  ori r3, r3, 0x40
	ctx.r[3].u64 = ctx.r[3].u64 | 64;
	// 831ADAB8: 554B0739  rlwinm. r11, r10, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831ADABC: 41820008  beq 0x831adac4
	if ctx.cr[0].eq {
	pc = 0x831ADAC4; continue 'dispatch;
	}
	// 831ADAC0: 60630010  ori r3, r3, 0x10
	ctx.r[3].u64 = ctx.r[3].u64 | 16;
	// 831ADAC4: 554B06F7  rlwinm. r11, r10, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831ADAC8: 41820008  beq 0x831adad0
	if ctx.cr[0].eq {
	pc = 0x831ADAD0; continue 'dispatch;
	}
	// 831ADACC: 60630080  ori r3, r3, 0x80
	ctx.r[3].u64 = ctx.r[3].u64 | 128;
	// 831ADAD0: 552A05AF  rlwinm. r10, r9, 0, 0x16, 0x17
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831ADAD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831ADAD8: 41820030  beq 0x831adb08
	if ctx.cr[0].eq {
	pc = 0x831ADB08; continue 'dispatch;
	}
	// 831ADADC: 2B0A0300  cmplwi cr6, r10, 0x300
	ctx.cr[6].compare_u32(ctx.r[10].u32, 768 as u32, &mut ctx.xer);
	// 831ADAE0: 409A000C  bne cr6, 0x831adaec
	if !ctx.cr[6].eq {
	pc = 0x831ADAEC; continue 'dispatch;
	}
	// 831ADAE4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831ADAE8: 48000020  b 0x831adb08
	pc = 0x831ADB08; continue 'dispatch;
	// 831ADAEC: 2B0A0200  cmplwi cr6, r10, 0x200
	ctx.cr[6].compare_u32(ctx.r[10].u32, 512 as u32, &mut ctx.xer);
	// 831ADAF0: 409A000C  bne cr6, 0x831adafc
	if !ctx.cr[6].eq {
	pc = 0x831ADAFC; continue 'dispatch;
	}
	// 831ADAF4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 831ADAF8: 48000010  b 0x831adb08
	pc = 0x831ADB08; continue 'dispatch;
	// 831ADAFC: 2B0A0100  cmplwi cr6, r10, 0x100
	ctx.cr[6].compare_u32(ctx.r[10].u32, 256 as u32, &mut ctx.xer);
	// 831ADB00: 409A0008  bne cr6, 0x831adb08
	if !ctx.cr[6].eq {
	pc = 0x831ADB08; continue 'dispatch;
	}
	// 831ADB04: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 831ADB08: 552A07FF  clrlwi. r10, r9, 0x1f
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831ADB0C: 41820008  beq 0x831adb14
	if ctx.cr[0].eq {
	pc = 0x831ADB14; continue 'dispatch;
	}
	// 831ADB10: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 831ADB14: 552A07BD  rlwinm. r10, r9, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831ADB18: 41820008  beq 0x831adb20
	if ctx.cr[0].eq {
	pc = 0x831ADB20; continue 'dispatch;
	}
	// 831ADB1C: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 831ADB20: 552A077B  rlwinm. r10, r9, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831ADB24: 41820008  beq 0x831adb2c
	if ctx.cr[0].eq {
	pc = 0x831ADB2C; continue 'dispatch;
	}
	// 831ADB28: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 831ADB2C: 552A0739  rlwinm. r10, r9, 0, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831ADB30: 41820008  beq 0x831adb38
	if ctx.cr[0].eq {
	pc = 0x831ADB38; continue 'dispatch;
	}
	// 831ADB34: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 831ADB38: 552A06F7  rlwinm. r10, r9, 0, 0x1b, 0x1b
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831ADB3C: 41820008  beq 0x831adb44
	if ctx.cr[0].eq {
	pc = 0x831ADB44; continue 'dispatch;
	}
	// 831ADB40: 616B0080  ori r11, r11, 0x80
	ctx.r[11].u64 = ctx.r[11].u64 | 128;
	// 831ADB44: 550AC23E  srwi r10, r8, 8
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shr(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831ADB48: 7D445B78  or r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 831ADB4C: 4800B455  bl 0x831b8fa0
	ctx.lr = 0x831ADB50;
	sub_831B8FA0(ctx, base);
	// 831ADB50: 4BFFFE71  bl 0x831ad9c0
	ctx.lr = 0x831ADB54;
	sub_831AD9C0(ctx, base);
	// 831ADB54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831ADB58: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831ADB5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ADB60: 4800B441  bl 0x831b8fa0
	ctx.lr = 0x831ADB64;
	sub_831B8FA0(ctx, base);
	// 831ADB64: 4BFFFE5D  bl 0x831ad9c0
	ctx.lr = 0x831ADB68;
	sub_831AD9C0(ctx, base);
	// 831ADB68: 546A033E  clrlwi r10, r3, 0xc
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000FFFFFu64;
	// 831ADB6C: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 831ADB70: 554A06D8  rlwinm r10, r10, 0, 0x1b, 0xc
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 831ADB74: 616B001F  ori r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u64 | 31;
	// 831ADB78: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831ADB7C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831ADB80: 409A0008  bne cr6, 0x831adb88
	if !ctx.cr[6].eq {
	pc = 0x831ADB88; continue 'dispatch;
	}
	// 831ADB84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ADB88: 48095355  bl 0x83242edc
	ctx.lr = 0x831ADB8C;
	// extern call 0x83242EDC → crate::xboxkrnl::KeEnableFpuExceptions
	crate::xboxkrnl::KeEnableFpuExceptions(ctx, base);
	// 831ADB8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831ADB90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831ADB94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831ADB98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831ADB9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831ADBA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ADBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ADBA8 size=48
    let mut pc: u32 = 0x831ADBA8;
    'dispatch: loop {
        match pc {
            0x831ADBA8 => {
    //   block [0x831ADBA8..0x831ADBD8)
	// 831ADBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831ADBAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831ADBB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831ADBB4: 3C80FFF8  lis r4, -8
	ctx.r[4].s64 = -524288;
	// 831ADBB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ADBBC: 60840700  ori r4, r4, 0x700
	ctx.r[4].u64 = ctx.r[4].u64 | 1792;
	// 831ADBC0: 4800B3E1  bl 0x831b8fa0
	ctx.lr = 0x831ADBC4;
	sub_831B8FA0(ctx, base);
	// 831ADBC4: 4BFFFDFD  bl 0x831ad9c0
	ctx.lr = 0x831ADBC8;
	sub_831AD9C0(ctx, base);
	// 831ADBC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831ADBCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831ADBD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831ADBD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ADBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ADBD8 size=240
    let mut pc: u32 = 0x831ADBD8;
    'dispatch: loop {
        match pc {
            0x831ADBD8 => {
    //   block [0x831ADBD8..0x831ADCC8)
	// 831ADBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831ADBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831ADBE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831ADBE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831ADBE8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831ADBEC: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 831ADBF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831ADBF4: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 831ADBF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831ADBFC: 409A0030  bne cr6, 0x831adc2c
	if !ctx.cr[6].eq {
	pc = 0x831ADC2C; continue 'dispatch;
	}
	// 831ADC00: 48003161  bl 0x831b0d60
	ctx.lr = 0x831ADC04;
	sub_831B0D60(ctx, base);
	// 831ADC04: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 831ADC08: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831ADC0C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831ADC10: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831ADC14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831ADC18: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831ADC1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ADC20: 480071F1  bl 0x831b4e10
	ctx.lr = 0x831ADC24;
	sub_831B4E10(ctx, base);
	// 831ADC24: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831ADC28: 48000088  b 0x831adcb0
	pc = 0x831ADCB0; continue 'dispatch;
	// 831ADC2C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831ADC30: 419A000C  beq cr6, 0x831adc3c
	if ctx.cr[6].eq {
	pc = 0x831ADC3C; continue 'dispatch;
	}
	// 831ADC34: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831ADC38: 419AFFC8  beq cr6, 0x831adc00
	if ctx.cr[6].eq {
	pc = 0x831ADC00; continue 'dispatch;
	}
	// 831ADC3C: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 831ADC40: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 831ADC44: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831ADC48: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 831ADC4C: 41990008  bgt cr6, 0x831adc54
	if ctx.cr[6].gt {
	pc = 0x831ADC54; continue 'dispatch;
	}
	// 831ADC50: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 831ADC54: 39400042  li r10, 0x42
	ctx.r[10].s64 = 66;
	// 831ADC58: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 831ADC5C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 831ADC60: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 831ADC64: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 831ADC68: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 831ADC6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831ADC70: 48006619  bl 0x831b4288
	ctx.lr = 0x831ADC74;
	sub_831B4288(ctx, base);
	// 831ADC74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831ADC78: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831ADC7C: 419A0030  beq cr6, 0x831adcac
	if ctx.cr[6].eq {
	pc = 0x831ADCAC; continue 'dispatch;
	}
	// 831ADC80: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831ADC84: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831ADC88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831ADC8C: 41800014  blt 0x831adca0
	if ctx.cr[0].lt {
	pc = 0x831ADCA0; continue 'dispatch;
	}
	// 831ADC90: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831ADC94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831ADC98: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 831ADC9C: 48000010  b 0x831adcac
	pc = 0x831ADCAC; continue 'dispatch;
	// 831ADCA0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831ADCA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831ADCA8: 48006379  bl 0x831b4020
	ctx.lr = 0x831ADCAC;
	sub_831B4020(ctx, base);
	// 831ADCAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831ADCB0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831ADCB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831ADCB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831ADCBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831ADCC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831ADCC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ADCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831ADCC8 size=12
    let mut pc: u32 = 0x831ADCC8;
    'dispatch: loop {
        match pc {
            0x831ADCC8 => {
    //   block [0x831ADCC8..0x831ADCD4)
	// 831ADCC8: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 831ADCCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831ADCD0: 4BFFFF08  b 0x831adbd8
	sub_831ADBD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831ADCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831ADCE0 size=788
    let mut pc: u32 = 0x831ADCE0;
    'dispatch: loop {
        match pc {
            0x831ADCE0 => {
    //   block [0x831ADCE0..0x831ADFF4)
	// 831ADCE0: 7C0802A6  mflr r0
	ctx.r[0].u64 = ctx.lr;
	// 831ADCE4: 9421FFB0  stwu r1, -0x50(r1)
	ea = ctx.r[1].u32.wrapping_add(-80 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831ADCE8: 90010008  stw r0, 8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(8 as u32), ctx.r[0].u32 ) };
	// 831ADCEC: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 831ADCF0: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831ADCF4: 80030138  lwz r0, 0x138(r3)
	ctx.r[0].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(312 as u32) ) } as u64;
	// 831ADCF8: 2C800000  cmpwi cr1, r0, 0
	ctx.cr[1].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831ADCFC: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 831ADD00: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831ADD04: 40820008  bne 0x831add0c
	if !ctx.cr[0].eq {
	pc = 0x831ADD0C; continue 'dispatch;
	}
	// 831ADD08: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 831ADD0C: 408602C0  bne cr1, 0x831adfcc
	if !ctx.cr[1].eq {
	pc = 0x831ADFCC; continue 'dispatch;
	}
	// 831ADD10: 80670134  lwz r3, 0x134(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(308 as u32) ) } as u64;
	// 831ADD14: 80870090  lwz r4, 0x90(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(144 as u32) ) } as u64;
	// 831ADD18: 4800CD39  bl 0x831baa50
	ctx.lr = 0x831ADD1C;
	sub_831BAA50(ctx, base);
	// 831ADD1C: C9C70000  lfd f14, 0(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[14].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	// 831ADD20: C9E70008  lfd f15, 8(r7)
	ctx.f[15].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) };
	// 831ADD24: CA070010  lfd f16, 0x10(r7)
	ctx.f[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(16 as u32) ) };
	// 831ADD28: CA270018  lfd f17, 0x18(r7)
	ctx.f[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) };
	// 831ADD2C: CA470020  lfd f18, 0x20(r7)
	ctx.f[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(32 as u32) ) };
	// 831ADD30: CA670028  lfd f19, 0x28(r7)
	ctx.f[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(40 as u32) ) };
	// 831ADD34: CA870030  lfd f20, 0x30(r7)
	ctx.f[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) };
	// 831ADD38: CAA70038  lfd f21, 0x38(r7)
	ctx.f[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(56 as u32) ) };
	// 831ADD3C: CAC70040  lfd f22, 0x40(r7)
	ctx.f[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(64 as u32) ) };
	// 831ADD40: CAE70048  lfd f23, 0x48(r7)
	ctx.f[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(72 as u32) ) };
	// 831ADD44: CB070050  lfd f24, 0x50(r7)
	ctx.f[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(80 as u32) ) };
	// 831ADD48: CB270058  lfd f25, 0x58(r7)
	ctx.f[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(88 as u32) ) };
	// 831ADD4C: CB470060  lfd f26, 0x60(r7)
	ctx.f[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(96 as u32) ) };
	// 831ADD50: CB670068  lfd f27, 0x68(r7)
	ctx.f[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(104 as u32) ) };
	// 831ADD54: CB870070  lfd f28, 0x70(r7)
	ctx.f[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(112 as u32) ) };
	// 831ADD58: CBA70078  lfd f29, 0x78(r7)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(120 as u32) ) };
	// 831ADD5C: CBC70080  lfd f30, 0x80(r7)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(128 as u32) ) };
	// 831ADD60: CBE70088  lfd f31, 0x88(r7)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(136 as u32) ) };
	// 831ADD64: E9A70098  ld r13, 0x98(r7)
	ctx.r[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(152 as u32) ) };
	// 831ADD68: E9C700A0  ld r14, 0xa0(r7)
	ctx.r[14].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(160 as u32) ) };
	// 831ADD6C: E9E700A8  ld r15, 0xa8(r7)
	ctx.r[15].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(168 as u32) ) };
	// 831ADD70: EA0700B0  ld r16, 0xb0(r7)
	ctx.r[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(176 as u32) ) };
	// 831ADD74: EA2700B8  ld r17, 0xb8(r7)
	ctx.r[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(184 as u32) ) };
	// 831ADD78: EA4700C0  ld r18, 0xc0(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(192 as u32) ) };
	// 831ADD7C: EA6700C8  ld r19, 0xc8(r7)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(200 as u32) ) };
	// 831ADD80: EA8700D0  ld r20, 0xd0(r7)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(208 as u32) ) };
	// 831ADD84: EAA700D8  ld r21, 0xd8(r7)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(216 as u32) ) };
	// 831ADD88: EAC700E0  ld r22, 0xe0(r7)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(224 as u32) ) };
	// 831ADD8C: EAE700E8  ld r23, 0xe8(r7)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(232 as u32) ) };
	// 831ADD90: EB0700F0  ld r24, 0xf0(r7)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(240 as u32) ) };
	// 831ADD94: EB2700F8  ld r25, 0xf8(r7)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(248 as u32) ) };
	// 831ADD98: EB470100  ld r26, 0x100(r7)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(256 as u32) ) };
	// 831ADD9C: EB670108  ld r27, 0x108(r7)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(264 as u32) ) };
	// 831ADDA0: EB870110  ld r28, 0x110(r7)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(272 as u32) ) };
	// 831ADDA4: EBA70118  ld r29, 0x118(r7)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(280 as u32) ) };
	// 831ADDA8: EBC70120  ld r30, 0x120(r7)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(288 as u32) ) };
	// 831ADDAC: EBE70128  ld r31, 0x128(r7)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(296 as u32) ) };
	// 831ADDB0: 38600140  li r3, 0x140
	ctx.r[3].s64 = 320;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AE000 size=20
    let mut pc: u32 = 0x831AE000;
    'dispatch: loop {
        match pc {
            0x831AE000 => {
    //   block [0x831AE000..0x831AE014)
	// 831AE000: 3C808345  lis r4, -0x7cbb
	ctx.r[4].s64 = -2092630016;
	// 831AE004: 8004CF0C  lwz r0, -0x30f4(r4)
	ctx.r[0].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-12532 as u32) ) } as u64;
	// 831AE008: 2C000000  cmpwi r0, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831AE00C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831AE010: 4C820420  bnectr
	if !ctx.cr[0].eq {
		crate::rt::call_indirect(ctx.ctr.u32);
		return;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE014(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AE014 size=696
    let mut pc: u32 = 0x831AE014;
    'dispatch: loop {
        match pc {
            0x831AE014 => {
    //   block [0x831AE014..0x831AE2CC)
	// 831AE014: 7C0802A6  mflr r0
	ctx.r[0].u64 = ctx.lr;
	// 831AE018: 7C800026  mfcr r4
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
	// 831AE01C: D9C30000  stfd f14, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.f[14].u64 ) };
	// 831AE020: D9E30008  stfd f15, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.f[15].u64 ) };
	// 831AE024: DA030010  stfd f16, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.f[16].u64 ) };
	// 831AE028: DA230018  stfd f17, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.f[17].u64 ) };
	// 831AE02C: DA430020  stfd f18, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.f[18].u64 ) };
	// 831AE030: DA630028  stfd f19, 0x28(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.f[19].u64 ) };
	// 831AE034: DA830030  stfd f20, 0x30(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.f[20].u64 ) };
	// 831AE038: DAA30038  stfd f21, 0x38(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.f[21].u64 ) };
	// 831AE03C: DAC30040  stfd f22, 0x40(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.f[22].u64 ) };
	// 831AE040: DAE30048  stfd f23, 0x48(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.f[23].u64 ) };
	// 831AE044: DB030050  stfd f24, 0x50(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.f[24].u64 ) };
	// 831AE048: DB230058  stfd f25, 0x58(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.f[25].u64 ) };
	// 831AE04C: DB430060  stfd f26, 0x60(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.f[26].u64 ) };
	// 831AE050: DB630068  stfd f27, 0x68(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.f[27].u64 ) };
	// 831AE054: DB830070  stfd f28, 0x70(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), ctx.f[28].u64 ) };
	// 831AE058: DBA30078  stfd f29, 0x78(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), ctx.f[29].u64 ) };
	// 831AE05C: DBC30080  stfd f30, 0x80(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.f[30].u64 ) };
	// 831AE060: DBE30088  stfd f31, 0x88(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.f[31].u64 ) };
	// 831AE064: F9A30098  std r13, 0x98(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[13].u64 ) };
	// 831AE068: F9C300A0  std r14, 0xa0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), ctx.r[14].u64 ) };
	// 831AE06C: F9E300A8  std r15, 0xa8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), ctx.r[15].u64 ) };
	// 831AE070: FA0300B0  std r16, 0xb0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), ctx.r[16].u64 ) };
	// 831AE074: FA2300B8  std r17, 0xb8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(184 as u32), ctx.r[17].u64 ) };
	// 831AE078: FA4300C0  std r18, 0xc0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(192 as u32), ctx.r[18].u64 ) };
	// 831AE07C: FA6300C8  std r19, 0xc8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(200 as u32), ctx.r[19].u64 ) };
	// 831AE080: FA8300D0  std r20, 0xd0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(208 as u32), ctx.r[20].u64 ) };
	// 831AE084: FAA300D8  std r21, 0xd8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(216 as u32), ctx.r[21].u64 ) };
	// 831AE088: FAC300E0  std r22, 0xe0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(224 as u32), ctx.r[22].u64 ) };
	// 831AE08C: FAE300E8  std r23, 0xe8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(232 as u32), ctx.r[23].u64 ) };
	// 831AE090: FB0300F0  std r24, 0xf0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(240 as u32), ctx.r[24].u64 ) };
	// 831AE094: FB2300F8  std r25, 0xf8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(248 as u32), ctx.r[25].u64 ) };
	// 831AE098: FB430100  std r26, 0x100(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(256 as u32), ctx.r[26].u64 ) };
	// 831AE09C: FB630108  std r27, 0x108(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(264 as u32), ctx.r[27].u64 ) };
	// 831AE0A0: FB830110  std r28, 0x110(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(272 as u32), ctx.r[28].u64 ) };
	// 831AE0A4: FBA30118  std r29, 0x118(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(280 as u32), ctx.r[29].u64 ) };
	// 831AE0A8: FBC30120  std r30, 0x120(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(288 as u32), ctx.r[30].u64 ) };
	// 831AE0AC: FBE30128  std r31, 0x128(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(296 as u32), ctx.r[31].u64 ) };
	// 831AE0B0: 38A00140  li r5, 0x140
	ctx.r[5].s64 = 320;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AE2D0 size=32
    let mut pc: u32 = 0x831AE2D0;
    'dispatch: loop {
        match pc {
            0x831AE2D0 => {
    //   block [0x831AE2D0..0x831AE2F0)
	// 831AE2D0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 831AE2D4: D8210010  stfd f1, 0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.f[1].u64 ) };
	// 831AE2D8: C8CBE3A0  lfd f6, -0x1c60(r11)
	ctx.f[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-7264 as u32) ) };
	// 831AE2DC: FF013000  fcmpu cr6, f1, f6
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[6].f64);
	// 831AE2E0: 409A0010  bne cr6, 0x831ae2f0
	if !ctx.cr[6].eq {
		sub_831AE2F0(ctx, base);
		return;
	}
	// 831AE2E4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 831AE2E8: C82BD228  lfd f1, -0x2dd8(r11)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11736 as u32) ) };
	// 831AE2EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AE2F0 size=36
    let mut pc: u32 = 0x831AE2F0;
    'dispatch: loop {
        match pc {
            0x831AE2F0 => {
    //   block [0x831AE2F0..0x831AE314)
	// 831AE2F0: A1410010  lhz r10, 0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 831AE2F4: 554B0476  rlwinm r11, r10, 0, 0x11, 0x1b
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 831AE2F8: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 831AE2FC: 2B0B7FF0  cmplwi cr6, r11, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32752 as u32, &mut ctx.xer);
	// 831AE300: 409A0024  bne cr6, 0x831ae324
	if !ctx.cr[6].eq {
		sub_831AE324(ctx, base);
		return;
	}
	// 831AE304: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831AE308: C80BC6B8  lfd f0, -0x3948(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-14664 as u32) ) };
	// 831AE30C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 831AE310: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE314(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AE314 size=16
    let mut pc: u32 = 0x831AE314;
    'dispatch: loop {
        match pc {
            0x831AE314 => {
    //   block [0x831AE314..0x831AE324)
	// 831AE314: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AE318: C80BF470  lfd f0, -0xb90(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-2960 as u32) ) };
	// 831AE31C: FC200050  fneg f1, f0
	ctx.f[1].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 831AE320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE324(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AE324 size=36
    let mut pc: u32 = 0x831AE324;
    'dispatch: loop {
        match pc {
            0x831AE324 => {
    //   block [0x831AE324..0x831AE348)
	// 831AE324: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 831AE328: C80AD228  lfd f0, -0x2dd8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-11736 as u32) ) };
	// 831AE32C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 831AE330: 41990018  bgt cr6, 0x831ae348
	if ctx.cr[6].gt {
		sub_831AE348(ctx, base);
		return;
	}
	// 831AE334: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 831AE338: 409AFFDC  bne cr6, 0x831ae314
	if !ctx.cr[6].eq {
		sub_831AE314(ctx, base);
		return;
	}
	// 831AE33C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831AE340: C80BF468  lfd f0, -0xb98(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-2968 as u32) ) };
	// 831AE344: 4BFFFFD8  b 0x831ae31c
	sub_831AE314(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AE348 size=56
    let mut pc: u32 = 0x831AE348;
    'dispatch: loop {
        match pc {
            0x831AE348 => {
    //   block [0x831AE348..0x831AE380)
	// 831AE348: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 831AE34C: C80AC650  lfd f0, -0x39b0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-14768 as u32) ) };
	// 831AE350: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 831AE354: 4098002C  bge cr6, 0x831ae380
	if !ctx.cr[6].lt {
		sub_831AE380(ctx, base);
		return;
	}
	// 831AE358: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831AE35C: C80BC6B0  lfd f0, -0x3950(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-14672 as u32) ) };
	// 831AE360: FC210032  fmul f1, f1, f0
	ctx.f[1].f64 = ctx.f[1].f64 * ctx.f[0].f64;
	// 831AE364: D8210010  stfd f1, 0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.f[1].u64 ) };
	// 831AE368: A1610010  lhz r11, 0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 831AE36C: 556AE57E  rlwinm r10, r11, 0x1c, 0x15, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 831AE370: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 831AE374: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 831AE378: 394BFBCD  addi r10, r11, -0x433
	ctx.r[10].s64 = ctx.r[11].s64 + -1075;
	// 831AE37C: 4800000C  b 0x831ae388
	sub_831AE380(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AE380 size=76
    let mut pc: u32 = 0x831AE380;
    'dispatch: loop {
        match pc {
            0x831AE380 => {
    //   block [0x831AE380..0x831AE3CC)
	// 831AE380: 556BE53E  rlwinm r11, r11, 0x1c, 0x14, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 831AE384: 394BFC02  addi r10, r11, -0x3fe
	ctx.r[10].s64 = ctx.r[11].s64 + -1022;
	// 831AE388: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831AE38C: D821FFF0  stfd f1, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[1].u64 ) };
	// 831AE390: 7129800F  andi. r9, r9, 0x800f
	ctx.r[9].u64 = ctx.r[9].u64 & 32783;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831AE394: 396BC638  addi r11, r11, -0x39c8
	ctx.r[11].s64 = ctx.r[11].s64 + -14792;
	// 831AE398: 61293FE0  ori r9, r9, 0x3fe0
	ctx.r[9].u64 = ctx.r[9].u64 | 16352;
	// 831AE39C: B121FFF0  sth r9, -0x10(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[9].u16 ) };
	// 831AE3A0: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831AE3A4: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 831AE3A8: C80B0000  lfd f0, 0(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 831AE3AC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 831AE3B0: 4099001C  ble cr6, 0x831ae3cc
	if !ctx.cr[6].gt {
		sub_831AE3CC(ctx, base);
		return;
	}
	// 831AE3B4: C809D760  lfd f0, -0x28a0(r9)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(-10400 as u32) ) };
	// 831AE3B8: FD8D302A  fadd f12, f13, f6
	ctx.f[12].f64 = ctx.f[13].f64 + ctx.f[6].f64;
	// 831AE3BC: FD6D0028  fsub f11, f13, f0
	ctx.f[11].f64 = ctx.f[13].f64 - ctx.f[0].f64;
	// 831AE3C0: FDAC0032  fmul f13, f12, f0
	ctx.f[13].f64 = ctx.f[12].f64 * ctx.f[0].f64;
	// 831AE3C4: FC0B0028  fsub f0, f11, f0
	ctx.f[0].f64 = ctx.f[11].f64 - ctx.f[0].f64;
	// 831AE3C8: 48000018  b 0x831ae3e0
	sub_831AE3CC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE3CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AE3CC size=144
    let mut pc: u32 = 0x831AE3CC;
    'dispatch: loop {
        match pc {
            0x831AE3CC => {
    //   block [0x831AE3CC..0x831AE45C)
	// 831AE3CC: C989D760  lfd f12, -0x28a0(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(-10400 as u32) ) };
	// 831AE3D0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 831AE3D4: FC0D6028  fsub f0, f13, f12
	ctx.f[0].f64 = ctx.f[13].f64 - ctx.f[12].f64;
	// 831AE3D8: FDA0302A  fadd f13, f0, f6
	ctx.f[13].f64 = ctx.f[0].f64 + ctx.f[6].f64;
	// 831AE3DC: FDAD0332  fmul f13, f13, f12
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[12].f64;
	// 831AE3E0: FCA06824  fdiv f5, f0, f13
	ctx.f[5].f64 = ctx.f[0].f64 / ctx.f[13].f64;
	// 831AE3E4: 3D20821A  lis r9, -0x7de6
	ctx.r[9].s64 = -2112225280;
	// 831AE3E8: 3D00821A  lis r8, -0x7de6
	ctx.r[8].s64 = -2112225280;
	// 831AE3EC: C98B0028  lfd f12, 0x28(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 831AE3F0: 3CE0821A  lis r7, -0x7de6
	ctx.r[7].s64 = -2112225280;
	// 831AE3F4: C92B0040  lfd f9, 0x40(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 831AE3F8: 3CC0821A  lis r6, -0x7de6
	ctx.r[6].s64 = -2112225280;
	// 831AE3FC: C8EB0008  lfd f7, 8(r11)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 831AE400: 7D4B07B4  extsw r11, r10
	ctx.r[11].s64 = ctx.r[10].s32 as i64;
	// 831AE404: C9A9C6A8  lfd f13, -0x3958(r9)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(-14680 as u32) ) };
	// 831AE408: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 831AE40C: C968C6A0  lfd f11, -0x3960(r8)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(-14688 as u32) ) };
	// 831AE410: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 831AE414: C947C698  lfd f10, -0x3968(r7)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(-14696 as u32) ) };
	// 831AE418: C906C690  lfd f8, -0x3970(r6)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(-14704 as u32) ) };
	// 831AE41C: C801FFF0  lfd f0, -0x10(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831AE420: FC80069C  fcfid f4, f0
	ctx.f[4].f64 = (ctx.f[0].s64 as f64);
	// 831AE424: FC650172  fmul f3, f5, f5
	ctx.f[3].f64 = ctx.f[5].f64 * ctx.f[5].f64;
	// 831AE428: C80AC688  lfd f0, -0x3978(r10)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-14712 as u32) ) };
	// 831AE42C: FC040032  fmul f0, f4, f0
	ctx.f[0].f64 = ctx.f[4].f64 * ctx.f[0].f64;
	// 831AE430: FDA3637C  fnmsub f13, f3, f13, f12
	ctx.f[13].f64 = -(ctx.f[3].f64 * ctx.f[13].f64 - ctx.f[12].f64);
	// 831AE434: FD835828  fsub f12, f3, f11
	ctx.f[12].f64 = ctx.f[3].f64 - ctx.f[11].f64;
	// 831AE438: FDAD50F8  fmsub f13, f13, f3, f10
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[3].f64 - ctx.f[10].f64;
	// 831AE43C: FD8C48FA  fmadd f12, f12, f3, f9
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[3].f64 + ctx.f[9].f64;
	// 831AE440: FDAD00F2  fmul f13, f13, f3
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[3].f64;
	// 831AE444: FD8C40F8  fmsub f12, f12, f3, f8
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[3].f64 - ctx.f[8].f64;
	// 831AE448: FDAD6024  fdiv f13, f13, f12
	ctx.f[13].f64 = ctx.f[13].f64 / ctx.f[12].f64;
	// 831AE44C: FDAD302A  fadd f13, f13, f6
	ctx.f[13].f64 = ctx.f[13].f64 + ctx.f[6].f64;
	// 831AE450: FC0D0178  fmsub f0, f13, f5, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[5].f64 - ctx.f[0].f64;
	// 831AE454: FC2401FA  fmadd f1, f4, f7, f0
	ctx.f[1].f64 = ctx.f[4].f64 * ctx.f[7].f64 + ctx.f[0].f64;
	// 831AE458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AE460 size=44
    let mut pc: u32 = 0x831AE460;
    'dispatch: loop {
        match pc {
            0x831AE460 => {
    //   block [0x831AE460..0x831AE48C)
	// 831AE460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AE464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831AE468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AE46C: 4BFFFE65  bl 0x831ae2d0
	ctx.lr = 0x831AE470;
	sub_831AE2D0(ctx, base);
	// 831AE470: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831AE474: C80BC650  lfd f0, -0x39b0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-14768 as u32) ) };
	// 831AE478: FC210032  fmul f1, f1, f0
	ctx.f[1].f64 = ctx.f[1].f64 * ctx.f[0].f64;
	// 831AE47C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831AE480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831AE484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831AE488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AE490 size=68
    let mut pc: u32 = 0x831AE490;
    'dispatch: loop {
        match pc {
            0x831AE490 => {
    //   block [0x831AE490..0x831AE4D4)
	// 831AE490: 88A30000  lbz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AE494: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 831AE498: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831AE49C: 7C052000  cmpw r5, r4
	ctx.cr[0].compare_i32(ctx.r[4].s32, ctx.r[0].s32, &mut ctx.xer);
	// 831AE4A0: 419A0034  beq cr6, 0x831ae4d4
	if ctx.cr[6].eq {
		sub_831AE4D4(ctx, base);
		return;
	}
	// 831AE4A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AE4A8: 41820018  beq 0x831ae4c0
	if ctx.cr[0].eq {
	pc = 0x831AE4C0; continue 'dispatch;
	}
	// 831AE4AC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 831AE4B0: 419A0034  beq cr6, 0x831ae4e4
	if ctx.cr[6].eq {
		sub_831AE4E4(ctx, base);
		return;
	}
	// 831AE4B4: 8CA90001  lbzu r5, 1(r9)
	ea = ctx.r[9].u32.wrapping_add(1 as u32);
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[9].u32 = ea;
	// 831AE4B8: 7C042800  cmpw r4, r5
	ctx.cr[0].compare_i32(ctx.r[5].s32, ctx.r[0].s32, &mut ctx.xer);
	// 831AE4BC: 4082FFF0  bne 0x831ae4ac
	if !ctx.cr[0].eq {
	pc = 0x831AE4AC; continue 'dispatch;
	}
	// 831AE4C0: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 831AE4C4: 8CA90001  lbzu r5, 1(r9)
	ea = ctx.r[9].u32.wrapping_add(1 as u32);
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[9].u32 = ea;
	// 831AE4C8: 7C042800  cmpw r4, r5
	ctx.cr[0].compare_i32(ctx.r[5].s32, ctx.r[0].s32, &mut ctx.xer);
	// 831AE4CC: 4182FFF4  beq 0x831ae4c0
	if ctx.cr[0].eq {
	pc = 0x831AE4C0; continue 'dispatch;
	}
	// 831AE4D0: 4BFFFFDC  b 0x831ae4ac
	pc = 0x831AE4AC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE4D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AE4D4 size=16
    let mut pc: u32 = 0x831AE4D4;
    'dispatch: loop {
        match pc {
            0x831AE4D4 => {
    //   block [0x831AE4D4..0x831AE4E4)
	// 831AE4D4: 41820010  beq 0x831ae4e4
	if ctx.cr[0].eq {
		sub_831AE4E4(ctx, base);
		return;
	}
	// 831AE4D8: 8CA30001  lbzu r5, 1(r3)
	ea = ctx.r[3].u32.wrapping_add(1 as u32);
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[3].u32 = ea;
	// 831AE4DC: 2C050000  cmpwi r5, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831AE4E0: 4BFFFFF4  b 0x831ae4d4
	pc = 0x831AE4D4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE4E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AE4E4 size=4
    let mut pc: u32 = 0x831AE4E4;
    'dispatch: loop {
        match pc {
            0x831AE4E4 => {
    //   block [0x831AE4E4..0x831AE4E8)
	// 831AE4E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AE4F0 size=84
    let mut pc: u32 = 0x831AE4F0;
    'dispatch: loop {
        match pc {
            0x831AE4F0 => {
    //   block [0x831AE4F0..0x831AE544)
	// 831AE4F0: 3923FFFF  addi r9, r3, -1
	ctx.r[9].s64 = ctx.r[3].s64 + -1;
	// 831AE4F4: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 831AE4F8: 8CC40001  lbzu r6, 1(r4)
	ea = ctx.r[4].u32.wrapping_add(1 as u32);
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[4].u32 = ea;
	// 831AE4FC: 8CA90001  lbzu r5, 1(r9)
	ea = ctx.r[9].u32.wrapping_add(1 as u32);
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[9].u32 = ea;
	// 831AE500: 2F860000  cmpwi cr7, r6, 0
	ctx.cr[7].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831AE504: 7C662851  subf. r3, r6, r5
	ctx.r[3].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831AE508: 419E0038  beq cr7, 0x831ae540
	if ctx.cr[7].eq {
	pc = 0x831AE540; continue 'dispatch;
	}
	// 831AE50C: 4182FFEC  beq 0x831ae4f8
	if ctx.cr[0].eq {
	pc = 0x831AE4F8; continue 'dispatch;
	}
	// 831AE510: 2E860041  cmpwi cr5, r6, 0x41
	ctx.cr[5].compare_i32(ctx.r[6].s32, 65, &mut ctx.xer);
	// 831AE514: 2F06005A  cmpwi cr6, r6, 0x5a
	ctx.cr[6].compare_i32(ctx.r[6].s32, 90, &mut ctx.xer);
	// 831AE518: 4194000C  blt cr5, 0x831ae524
	if ctx.cr[5].lt {
	pc = 0x831AE524; continue 'dispatch;
	}
	// 831AE51C: 41990008  bgt cr6, 0x831ae524
	if ctx.cr[6].gt {
	pc = 0x831AE524; continue 'dispatch;
	}
	// 831AE520: 60C60020  ori r6, r6, 0x20
	ctx.r[6].u64 = ctx.r[6].u64 | 32;
	// 831AE524: 2C050041  cmpwi r5, 0x41
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831AE528: 2C85005A  cmpwi cr1, r5, 0x5a
	ctx.cr[1].compare_i32(ctx.r[5].s32, 90, &mut ctx.xer);
	// 831AE52C: 4180000C  blt 0x831ae538
	if ctx.cr[0].lt {
	pc = 0x831AE538; continue 'dispatch;
	}
	// 831AE530: 41850008  bgt cr1, 0x831ae538
	if ctx.cr[1].gt {
	pc = 0x831AE538; continue 'dispatch;
	}
	// 831AE534: 60A50020  ori r5, r5, 0x20
	ctx.r[5].u64 = ctx.r[5].u64 | 32;
	// 831AE538: 7C662851  subf. r3, r6, r5
	ctx.r[3].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831AE53C: 4182FFBC  beq 0x831ae4f8
	if ctx.cr[0].eq {
	pc = 0x831AE4F8; continue 'dispatch;
	}
	// 831AE540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AE548 size=52
    let mut pc: u32 = 0x831AE548;
    'dispatch: loop {
        match pc {
            0x831AE548 => {
    //   block [0x831AE548..0x831AE57C)
	// 831AE548: D8410018  stfd f2, 0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[2].u64 ) };
	// 831AE54C: 81610018  lwz r11, 0x18(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) } as u64;
	// 831AE550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 831AE554: D8210010  stfd f1, 0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.f[1].u64 ) };
	// 831AE558: 81210010  lwz r9, 0x10(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 831AE55C: 81010014  lwz r8, 0x14(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 831AE560: C80AD228  lfd f0, -0x2dd8(r10)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-11736 as u32) ) };
	// 831AE564: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 831AE568: 9101FFF4  stw r8, -0xc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[8].u32 ) };
	// 831AE56C: 512B007E  rlwimi r11, r9, 0, 1, 0x1f
	ctx.r[11].u64 = (((ctx.r[9].u32).rotate_left(0) as u64) & 0x000000007FFFFFFF) | (ctx.r[11].u64 & 0xFFFFFFFF80000000);
	// 831AE570: 9161FFF0  stw r11, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u32 ) };
	// 831AE574: C821FFF0  lfd f1, -0x10(r1)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831AE578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AE580 size=48
    let mut pc: u32 = 0x831AE580;
    'dispatch: loop {
        match pc {
            0x831AE580 => {
    //   block [0x831AE580..0x831AE5B0)
	// 831AE580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 831AE584: D8210010  stfd f1, 0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.f[1].u64 ) };
	// 831AE588: 81610010  lwz r11, 0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 831AE58C: 7D6858F8  nor r8, r11, r11
	ctx.r[8].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 831AE590: 81210014  lwz r9, 0x14(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 831AE594: C80AD228  lfd f0, -0x2dd8(r10)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-11736 as u32) ) };
	// 831AE598: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 831AE59C: 9121FFF4  stw r9, -0xc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[9].u32 ) };
	// 831AE5A0: 5168007E  rlwimi r8, r11, 0, 1, 0x1f
	ctx.r[8].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x000000007FFFFFFF) | (ctx.r[8].u64 & 0xFFFFFFFF80000000);
	// 831AE5A4: 9101FFF0  stw r8, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[8].u32 ) };
	// 831AE5A8: C821FFF0  lfd f1, -0x10(r1)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831AE5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AE5B0 size=32
    let mut pc: u32 = 0x831AE5B0;
    'dispatch: loop {
        match pc {
            0x831AE5B0 => {
    //   block [0x831AE5B0..0x831AE5D0)
	// 831AE5B0: D8210010  stfd f1, 0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.f[1].u64 ) };
	// 831AE5B4: A1610010  lhz r11, 0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 831AE5B8: 556B0476  rlwinm r11, r11, 0, 0x11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AE5BC: 396B8010  addi r11, r11, -0x7ff0
	ctx.r[11].s64 = ctx.r[11].s64 + -32752;
	// 831AE5C0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 831AE5C4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 831AE5C8: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 831AE5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AE5D0 size=60
    let mut pc: u32 = 0x831AE5D0;
    'dispatch: loop {
        match pc {
            0x831AE5D0 => {
    //   block [0x831AE5D0..0x831AE60C)
	// 831AE5D0: D8210010  stfd f1, 0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.f[1].u64 ) };
	// 831AE5D4: A1610010  lhz r11, 0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 831AE5D8: 556B0478  rlwinm r11, r11, 0, 0x11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AE5DC: 2B0B7FF0  cmplwi cr6, r11, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32752 as u32, &mut ctx.xer);
	// 831AE5E0: 409A001C  bne cr6, 0x831ae5fc
	if !ctx.cr[6].eq {
	pc = 0x831AE5FC; continue 'dispatch;
	}
	// 831AE5E4: 81410010  lwz r10, 0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 831AE5E8: 554A037F  clrlwi. r10, r10, 0xd
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0007FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AE5EC: 40820018  bne 0x831ae604
	if !ctx.cr[0].eq {
	pc = 0x831AE604; continue 'dispatch;
	}
	// 831AE5F0: 81410014  lwz r10, 0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 831AE5F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831AE5F8: 409A000C  bne cr6, 0x831ae604
	if !ctx.cr[6].eq {
	pc = 0x831AE604; continue 'dispatch;
	}
	// 831AE5FC: 2B0B7FF8  cmplwi cr6, r11, 0x7ff8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32760 as u32, &mut ctx.xer);
	// 831AE600: 409A000C  bne cr6, 0x831ae60c
	if !ctx.cr[6].eq {
		sub_831AE60C(ctx, base);
		return;
	}
	// 831AE604: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831AE608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE60C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AE60C size=8
    let mut pc: u32 = 0x831AE60C;
    'dispatch: loop {
        match pc {
            0x831AE60C => {
    //   block [0x831AE60C..0x831AE614)
	// 831AE60C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831AE610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831AE618 size=212
    let mut pc: u32 = 0x831AE618;
    'dispatch: loop {
        match pc {
            0x831AE618 => {
    //   block [0x831AE618..0x831AE6EC)
	// 831AE618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AE61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831AE620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AE624: D8210070  stfd f1, 0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.f[1].u64 ) };
	// 831AE628: A1610070  lhz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 831AE62C: 556A0476  rlwinm r10, r11, 0, 0x11, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AE630: 2B0A7FF0  cmplwi cr6, r10, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32752 as u32, &mut ctx.xer);
	// 831AE634: 409A003C  bne cr6, 0x831ae670
	if !ctx.cr[6].eq {
	pc = 0x831AE670; continue 'dispatch;
	}
	// 831AE638: 48006921  bl 0x831b4f58
	ctx.lr = 0x831AE63C;
	sub_831B4F58(ctx, base);
	// 831AE63C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831AE640: 419A0028  beq cr6, 0x831ae668
	if ctx.cr[6].eq {
	pc = 0x831AE668; continue 'dispatch;
	}
	// 831AE644: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 831AE648: 419A0018  beq cr6, 0x831ae660
	if ctx.cr[6].eq {
	pc = 0x831AE660; continue 'dispatch;
	}
	// 831AE64C: 3963FFFD  addi r11, r3, -3
	ctx.r[11].s64 = ctx.r[3].s64 + -3;
	// 831AE650: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 831AE654: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 831AE658: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 831AE65C: 48000080  b 0x831ae6dc
	pc = 0x831AE6DC; continue 'dispatch;
	// 831AE660: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 831AE664: 48000078  b 0x831ae6dc
	pc = 0x831AE6DC; continue 'dispatch;
	// 831AE668: 38600200  li r3, 0x200
	ctx.r[3].s64 = 512;
	// 831AE66C: 48000070  b 0x831ae6dc
	pc = 0x831AE6DC; continue 'dispatch;
	// 831AE670: 556B0420  rlwinm r11, r11, 0, 0x10, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AE674: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831AE678: 409A0034  bne cr6, 0x831ae6ac
	if !ctx.cr[6].eq {
	pc = 0x831AE6AC; continue 'dispatch;
	}
	// 831AE67C: 81410070  lwz r10, 0x70(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 831AE680: 554A033F  clrlwi. r10, r10, 0xc
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000FFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831AE684: 40820010  bne 0x831ae694
	if !ctx.cr[0].eq {
	pc = 0x831AE694; continue 'dispatch;
	}
	// 831AE688: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 831AE68C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831AE690: 419A001C  beq cr6, 0x831ae6ac
	if ctx.cr[6].eq {
	pc = 0x831AE6AC; continue 'dispatch;
	}
	// 831AE694: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 831AE698: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 831AE69C: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AE6A0: 556B06F0  rlwinm r11, r11, 0, 0x1b, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AE6A4: 386B0080  addi r3, r11, 0x80
	ctx.r[3].s64 = ctx.r[11].s64 + 128;
	// 831AE6A8: 48000034  b 0x831ae6dc
	pc = 0x831AE6DC; continue 'dispatch;
	// 831AE6AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 831AE6B0: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 831AE6B4: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 831AE6B8: C80AD228  lfd f0, -0x2dd8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-11736 as u32) ) };
	// 831AE6BC: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 831AE6C0: 409A0010  bne cr6, 0x831ae6d0
	if !ctx.cr[6].eq {
	pc = 0x831AE6D0; continue 'dispatch;
	}
	// 831AE6C4: 556B0034  rlwinm r11, r11, 0, 0, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AE6C8: 386B0040  addi r3, r11, 0x40
	ctx.r[3].s64 = ctx.r[11].s64 + 64;
	// 831AE6CC: 48000010  b 0x831ae6dc
	pc = 0x831AE6DC; continue 'dispatch;
	// 831AE6D0: 556B0038  rlwinm r11, r11, 0, 0, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AE6D4: 556B072E  rlwinm r11, r11, 0, 0x1c, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831AE6D8: 386B0100  addi r3, r11, 0x100
	ctx.r[3].s64 = ctx.r[11].s64 + 256;
	// 831AE6DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831AE6E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831AE6E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831AE6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AE6F0 size=72
    let mut pc: u32 = 0x831AE6F0;
    'dispatch: loop {
        match pc {
            0x831AE6F0 => {
    //   block [0x831AE6F0..0x831AE738)
	// 831AE6F0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 831AE6F4: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831AE6F8: 419A0014  beq cr6, 0x831ae70c
	if ctx.cr[6].eq {
	pc = 0x831AE70C; continue 'dispatch;
	}
	// 831AE6FC: 3940002D  li r10, 0x2d
	ctx.r[10].s64 = 45;
	// 831AE700: 39640001  addi r11, r4, 1
	ctx.r[11].s64 = ctx.r[4].s64 + 1;
	// 831AE704: 99440000  stb r10, 0(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 831AE708: 7C6300D0  neg r3, r3
	ctx.r[3].s64 = -ctx.r[3].s64;
	// 831AE70C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 831AE710: 7D432B96  divwu r10, r3, r5
	ctx.r[10].u32 = ctx.r[3].u32 / ctx.r[5].u32;
	// 831AE714: 0CC50000  twi 6, r5, 0
	// 831AE718: 7D4A29D6  mullw r10, r10, r5
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[5].s32 as i64);
	// 831AE71C: 7D4A1850  subf r10, r10, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[10].s64;
	// 831AE720: 7C632B96  divwu r3, r3, r5
	ctx.r[3].u32 = ctx.r[3].u32 / ctx.r[5].u32;
	// 831AE724: 0CC50000  twi 6, r5, 0
	// 831AE728: 2B0A0009  cmplwi cr6, r10, 9
	ctx.cr[6].compare_u32(ctx.r[10].u32, 9 as u32, &mut ctx.xer);
	// 831AE72C: 4099000C  ble cr6, 0x831ae738
	if !ctx.cr[6].gt {
		sub_831AE738(ctx, base);
		return;
	}
	// 831AE730: 394A0057  addi r10, r10, 0x57
	ctx.r[10].s64 = ctx.r[10].s64 + 87;
	// 831AE734: 48000008  b 0x831ae73c
	sub_831AE738(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831AE738 size=68
    let mut pc: u32 = 0x831AE738;
    'dispatch: loop {
        match pc {
            0x831AE738 => {
    //   block [0x831AE738..0x831AE77C)
	// 831AE738: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	// 831AE73C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 831AE740: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831AE744: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831AE748: 409AFFC8  bne cr6, 0x831ae710
	if !ctx.cr[6].eq {
		sub_831AE6F0(ctx, base);
		return;
	}
	// 831AE74C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831AE750: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 831AE754: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831AE758: 89490000  lbz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AE75C: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831AE760: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 831AE764: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831AE768: 99090000  stb r8, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 831AE76C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831AE770: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831AE774: 4198FFE4  blt cr6, 0x831ae758
	if ctx.cr[6].lt {
	pc = 0x831AE758; continue 'dispatch;
	}
	// 831AE778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AE780 size=44
    let mut pc: u32 = 0x831AE780;
    'dispatch: loop {
        match pc {
            0x831AE780 => {
    //   block [0x831AE780..0x831AE7AC)
	// 831AE780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AE784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831AE788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AE78C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831AE790: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 831AE794: 4BFFFF5D  bl 0x831ae6f0
	ctx.lr = 0x831AE798;
	sub_831AE6F0(ctx, base);
	// 831AE798: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 831AE79C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831AE7A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831AE7A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831AE7A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831AE7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831AE7B0 size=256
    let mut pc: u32 = 0x831AE7B0;
    'dispatch: loop {
        match pc {
            0x831AE7B0 => {
    //   block [0x831AE7B0..0x831AE8B0)
	// 831AE7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831AE7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831AE7B8: DBC1FFE8  stfd f30, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[30].u64 ) };
	// 831AE7BC: DBE1FFF0  stfd f31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 831AE7C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831AE7C4: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 831AE7C8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 831AE7CC: C9ABD228  lfd f13, -0x2dd8(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11736 as u32) ) };
	// 831AE7D0: FC00F210  fabs f0, f30
	ctx.f[0].u64 = ctx.f[30].u64 & !0x8000_0000_0000_0000u64;
	// 831AE7D4: FF1E6800  fcmpu cr6, f30, f13
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[13].f64);
	// 831AE7D8: 409A0008  bne cr6, 0x831ae7e0
	if !ctx.cr[6].eq {
	pc = 0x831AE7E0; continue 'dispatch;
	}
	// 831AE7DC: 480000BC  b 0x831ae898
	pc = 0x831AE898; continue 'dispatch;
	// 831AE7E0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831AE7E4: 396BC6F0  addi r11, r11, -0x3910
	ctx.r[11].s64 = ctx.r[11].s64 + -14608;
	// 831AE7E8: C9ABFFE0  lfd f13, -0x20(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-32 as u32) ) };
	// 831AE7EC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 831AE7F0: 40990050  ble cr6, 0x831ae840
	if !ctx.cr[6].gt {
	pc = 0x831AE840; continue 'dispatch;
	}
	// 831AE7F4: C9ABFFD8  lfd f13, -0x28(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-40 as u32) ) };
	// 831AE7F8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 831AE7FC: 40990010  ble cr6, 0x831ae80c
	if !ctx.cr[6].gt {
	pc = 0x831AE80C; continue 'dispatch;
	}
	// 831AE800: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 831AE804: C80BE3A0  lfd f0, -0x1c60(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-7264 as u32) ) };
	// 831AE808: 48000088  b 0x831ae890
	pc = 0x831AE890; continue 'dispatch;
	// 831AE80C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 831AE810: CBEBAA10  lfd f31, -0x55f0(r11)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-22000 as u32) ) };
	// 831AE814: FC2007F2  fmul f1, f0, f31
	ctx.f[1].f64 = ctx.f[0].f64 * ctx.f[31].f64;
	// 831AE818: 4BFFE3F9  bl 0x831acc10
	ctx.lr = 0x831AE81C;
	sub_831ACC10(ctx, base);
	// 831AE81C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 831AE820: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 831AE824: C80BE3A0  lfd f0, -0x1c60(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-7264 as u32) ) };
	// 831AE828: FD81002A  fadd f12, f1, f0
	ctx.f[12].f64 = ctx.f[1].f64 + ctx.f[0].f64;
	// 831AE82C: C9AAD760  lfd f13, -0x28a0(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-10400 as u32) ) };
	// 831AE830: FC006024  fdiv f0, f0, f12
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[12].f64;
	// 831AE834: FC0D0028  fsub f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 - ctx.f[0].f64;
	// 831AE838: FC0007F2  fmul f0, f0, f31
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[31].f64;
	// 831AE83C: 48000054  b 0x831ae890
	pc = 0x831AE890; continue 'dispatch;
	// 831AE840: FCC00032  fmul f6, f0, f0
	ctx.f[6].f64 = ctx.f[0].f64 * ctx.f[0].f64;
	// 831AE844: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 831AE848: C98BFFF0  lfd f12, -0x10(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-16 as u32) ) };
	// 831AE84C: 3D20821A  lis r9, -0x7de6
	ctx.r[9].s64 = -2112225280;
	// 831AE850: C96B0010  lfd f11, 0x10(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 831AE854: 3D008205  lis r8, -0x7dfb
	ctx.r[8].s64 = -2113601536;
	// 831AE858: C92B0008  lfd f9, 8(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 831AE85C: C90B0000  lfd f8, 0(r11)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 831AE860: C9AAC718  lfd f13, -0x38e8(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-14568 as u32) ) };
	// 831AE864: C949C710  lfd f10, -0x38f0(r9)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(-14576 as u32) ) };
	// 831AE868: C8E8E3A0  lfd f7, -0x1c60(r8)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(-7264 as u32) ) };
	// 831AE86C: FDA6637C  fnmsub f13, f6, f13, f12
	ctx.f[13].f64 = -(ctx.f[6].f64 * ctx.f[13].f64 - ctx.f[12].f64);
	// 831AE870: FD86582A  fadd f12, f6, f11
	ctx.f[12].f64 = ctx.f[6].f64 + ctx.f[11].f64;
	// 831AE874: FDAD51B8  fmsub f13, f13, f6, f10
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[6].f64 - ctx.f[10].f64;
	// 831AE878: FD8C49BA  fmadd f12, f12, f6, f9
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[6].f64 + ctx.f[9].f64;
	// 831AE87C: FDAD01B2  fmul f13, f13, f6
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[6].f64;
	// 831AE880: FD8C41BA  fmadd f12, f12, f6, f8
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[6].f64 + ctx.f[8].f64;
	// 831AE884: FDAD6024  fdiv f13, f13, f12
	ctx.f[13].f64 = ctx.f[13].f64 / ctx.f[12].f64;
	// 831AE888: FDAD382A  fadd f13, f13, f7
	ctx.f[13].f64 = ctx.f[13].f64 + ctx.f[7].f64;
	// 831AE88C: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 831AE890: FDA00050  fneg f13, f0
	ctx.f[13].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 831AE894: FC3E682E  fsel f1, f30, f0, f13
	ctx.f[1].f64 = if ctx.f[30].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 831AE898: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831AE89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831AE8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831AE8A4: CBC1FFE8  lfd f30, -0x18(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831AE8A8: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831AE8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


