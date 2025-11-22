pub fn sub_823841A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823841A4 size=16
    let mut pc: u32 = 0x823841A4;
    'dispatch: loop {
        match pc {
            0x823841A4 => {
    //   block [0x823841A4..0x823841B4)
	// 823841A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823841A8: 419A000C  beq cr6, 0x823841b4
	if ctx.cr[6].eq {
		sub_823841B4(ctx, base);
		return;
	}
	// 823841AC: 908B000C  stw r4, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 823841B0: 4800000C  b 0x823841bc
	sub_823841B4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823841B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823841B4 size=28
    let mut pc: u32 = 0x823841B4;
    'dispatch: loop {
        match pc {
            0x823841B4 => {
    //   block [0x823841B4..0x823841D0)
	// 823841B4: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 823841B8: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 823841BC: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 823841C0: 9104000C  stw r8, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 823841C4: 90880008  stw r4, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 823841C8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 823841CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823841D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823841D0 size=204
    let mut pc: u32 = 0x823841D0;
    'dispatch: loop {
        match pc {
            0x823841D0 => {
    //   block [0x823841D0..0x8238429C)
	// 823841D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823841D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823841D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823841DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823841E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823841E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823841E8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 823841EC: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823841F0: 4198000C  blt cr6, 0x823841fc
	if ctx.cr[6].lt {
	pc = 0x823841FC; continue 'dispatch;
	}
	// 823841F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823841F8: 4800008C  b 0x82384284
	pc = 0x82384284; continue 'dispatch;
	// 823841FC: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82384200: 548B2036  slwi r11, r4, 4
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82384204: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82384208: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8238420C: 7FC44B96  divwu r30, r4, r9
	ctx.r[30].u32 = ctx.r[4].u32 / ctx.r[9].u32;
	// 82384210: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82384214: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82384218: 419A0010  beq cr6, 0x82384228
	if ctx.cr[6].eq {
	pc = 0x82384228; continue 'dispatch;
	}
	// 8238421C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82384220: 4BFFFE71  bl 0x82384090
	ctx.lr = 0x82384224;
	sub_82384090(ctx, base);
	// 82384224: 4800003C  b 0x82384260
	pc = 0x82384260; continue 'dispatch;
	// 82384228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8238422C: 9145000C  stw r10, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82384230: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82384234: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82384238: 40820014  bne 0x8238424c
	if !ctx.cr[0].eq {
	pc = 0x8238424C; continue 'dispatch;
	}
	// 8238423C: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82384240: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82384244: 91450008  stw r10, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82384248: 48000014  b 0x8238425c
	pc = 0x8238425C; continue 'dispatch;
	// 8238424C: 90AB000C  stw r5, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82384250: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82384254: 91650008  stw r11, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82384258: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 8238425C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82384260: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82384264: 419AFF90  beq cr6, 0x823841f4
	if ctx.cr[6].eq {
	pc = 0x823841F4; continue 'dispatch;
	}
	// 82384268: 7BCB0020  clrldi r11, r30, 0x20
	ctx.r[11].u64 = ctx.r[30].u64 & 0x00000000FFFFFFFFu64;
	// 8238426C: E95F0000  ld r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 82384270: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82384274: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82384278: 7D2B5836  sld r11, r9, r11
	if (ctx.r[11].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[9].u64) << ((ctx.r[11].u8 & 0x3F) as u32);
	}
	// 8238427C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82384280: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82384284: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82384288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238428C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82384290: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82384294: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82384298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823842A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823842A0 size=196
    let mut pc: u32 = 0x823842A0;
    'dispatch: loop {
        match pc {
            0x823842A0 => {
    //   block [0x823842A0..0x82384364)
	// 823842A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823842A4: 481B0E05  bl 0x825350a8
	ctx.lr = 0x823842A8;
	sub_82535080(ctx, base);
	// 823842A8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823842AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823842B0: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 823842B4: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 823842B8: EBBF0000  ld r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 823842BC: 835F001C  lwz r26, 0x1c(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 823842C0: 48000088  b 0x82384348
	pc = 0x82384348; continue 'dispatch;
	// 823842C4: 7BAB07E0  clrldi r11, r29, 0x3f
	ctx.r[11].u64 = ctx.r[29].u64 & 0x0000000000000001u64;
	// 823842C8: 2B2B0000  cmpldi cr6, r11, 0
	ctx.cr[6].compare_u64(ctx.r[11].u64, 0, &mut ctx.xer);
	// 823842CC: 419A006C  beq cr6, 0x82384338
	if ctx.cr[6].eq {
	pc = 0x82384338; continue 'dispatch;
	}
	// 823842D0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 823842D4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 823842D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823842DC: 4099005C  ble cr6, 0x82384338
	if !ctx.cr[6].gt {
	pc = 0x82384338; continue 'dispatch;
	}
	// 823842E0: 3B9A0008  addi r28, r26, 8
	ctx.r[28].s64 = ctx.r[26].s64 + 8;
	// 823842E4: 83DC0000  lwz r30, 0(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 823842E8: 48000034  b 0x8238431c
	pc = 0x8238431C; continue 'dispatch;
	// 823842EC: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 823842F0: 7D6BC039  and. r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[24].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823842F4: 41820024  beq 0x82384318
	if ctx.cr[0].eq {
	pc = 0x82384318; continue 'dispatch;
	}
	// 823842F8: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 823842FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82384300: 41820018  beq 0x82384318
	if ctx.cr[0].eq {
	pc = 0x82384318; continue 'dispatch;
	}
	// 82384304: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82384308: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238430C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82384310: 4E800421  bctrl
	ctx.lr = 0x82384314;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82384314: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82384318: 83DE000C  lwz r30, 0xc(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238431C: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82384320: 4082FFCC  bne 0x823842ec
	if !ctx.cr[0].eq {
	pc = 0x823842EC; continue 'dispatch;
	}
	// 82384324: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82384328: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8238432C: 3B9C0010  addi r28, r28, 0x10
	ctx.r[28].s64 = ctx.r[28].s64 + 16;
	// 82384330: 7F3B5840  cmpld cr6, r27, r11
	ctx.cr[6].compare_u64(ctx.r[27].u64, ctx.r[11].u64, &mut ctx.xer);
	// 82384334: 4198FFB0  blt cr6, 0x823842e4
	if ctx.cr[6].lt {
	pc = 0x823842E4; continue 'dispatch;
	}
	// 82384338: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238433C: 7BBDF842  rldicl r29, r29, 0x3f, 1
	ctx.r[29].u64 = ctx.r[29].u64 & 0x0000000000000001u64;
	// 82384340: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82384344: 7F4BD214  add r26, r11, r26
	ctx.r[26].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82384348: 2B3D0000  cmpldi cr6, r29, 0
	ctx.cr[6].compare_u64(ctx.r[29].u64, 0, &mut ctx.xer);
	// 8238434C: 409AFF78  bne cr6, 0x823842c4
	if !ctx.cr[6].eq {
	pc = 0x823842C4; continue 'dispatch;
	}
	// 82384350: 3D4082C0  lis r10, -0x7d40
	ctx.r[10].s64 = -2101346304;
	// 82384354: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82384358: 932ABFA4  stw r25, -0x405c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-16476 as u32), ctx.r[25].u32 ) };
	// 8238435C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82384360: 481B0D98  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82384368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82384368 size=204
    let mut pc: u32 = 0x82384368;
    'dispatch: loop {
        match pc {
            0x82384368 => {
    //   block [0x82384368..0x82384434)
	// 82384368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238436C: 481B0D45  bl 0x825350b0
	ctx.lr = 0x82384370;
	sub_82535080(ctx, base);
	// 82384370: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82384374: 3BC00064  li r30, 0x64
	ctx.r[30].s64 = 100;
	// 82384378: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238437C: 3BA00190  li r29, 0x190
	ctx.r[29].s64 = 400;
	// 82384380: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82384384: 83E4000C  lwz r31, 0xc(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82384388: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 8238438C: 80C40010  lwz r6, 0x10(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82384390: 794B0020  clrldi r11, r10, 0x20
	ctx.r[11].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82384394: 81040014  lwz r8, 0x14(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82384398: 792A0020  clrldi r10, r9, 0x20
	ctx.r[10].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 8238439C: 83840008  lwz r28, 8(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 823843A0: 7F4BF392  divdu r26, r11, r30
	ctx.r[26].u64 = ctx.r[11].u64 / ctx.r[30].u64;
	// 823843A4: 83630008  lwz r27, 8(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 823843A8: 7C8AF392  divdu r4, r10, r30
	ctx.r[4].u64 = ctx.r[10].u64 / ctx.r[30].u64;
	// 823843AC: 80A3000C  lwz r5, 0xc(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 823843B0: 80E30010  lwz r7, 0x10(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 823843B4: 7FDAD850  subf r30, r26, r27
	ctx.r[30].s64 = ctx.r[27].s64 - ctx.r[26].s64;
	// 823843B8: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 823843BC: 7C6AEB92  divdu r3, r10, r29
	ctx.r[3].u64 = ctx.r[10].u64 / ctx.r[29].u64;
	// 823843C0: 7C84E050  subf r4, r4, r28
	ctx.r[4].s64 = ctx.r[28].s64 - ctx.r[4].s64;
	// 823843C4: 7FABEB92  divdu r29, r11, r29
	ctx.r[29].u64 = ctx.r[11].u64 / ctx.r[29].u64;
	// 823843C8: 7C841A14  add r4, r4, r3
	ctx.r[4].u64 = ctx.r[4].u64 + ctx.r[3].u64;
	// 823843CC: 1C6A016D  mulli r3, r10, 0x16d
	ctx.r[3].s64 = ctx.r[10].s64 * 365;
	// 823843D0: 7FDEEA14  add r30, r30, r29
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 823843D4: 1FAB016D  mulli r29, r11, 0x16d
	ctx.r[29].s64 = ctx.r[11].s64 * 365;
	// 823843D8: 7C841A14  add r4, r4, r3
	ctx.r[4].u64 = ctx.r[4].u64 + ctx.r[3].u64;
	// 823843DC: 7943F082  rldicl r3, r10, 0x3e, 2
	ctx.r[3].u64 = ctx.r[10].u64 & 0x0000000000000003u64;
	// 823843E0: 796BF082  rldicl r11, r11, 0x3e, 2
	ctx.r[11].u64 = ctx.r[11].u64 & 0x0000000000000003u64;
	// 823843E4: 7D5EEA14  add r10, r30, r29
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 823843E8: 7C841A14  add r4, r4, r3
	ctx.r[4].u64 = ctx.r[4].u64 + ctx.r[3].u64;
	// 823843EC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823843F0: 1D440018  mulli r10, r4, 0x18
	ctx.r[10].s64 = ctx.r[4].s64 * 24;
	// 823843F4: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 823843F8: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 823843FC: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82384400: 1D4A003C  mulli r10, r10, 0x3c
	ctx.r[10].s64 = ctx.r[10].s64 * 60;
	// 82384404: 1D6B003C  mulli r11, r11, 0x3c
	ctx.r[11].s64 = ctx.r[11].s64 * 60;
	// 82384408: 7D4A3214  add r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 8238440C: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82384410: 1D4A003C  mulli r10, r10, 0x3c
	ctx.r[10].s64 = ctx.r[10].s64 * 60;
	// 82384414: 1D6B003C  mulli r11, r11, 0x3c
	ctx.r[11].s64 = ctx.r[11].s64 * 60;
	// 82384418: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8238441C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82384420: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82384424: 7F2B5040  cmpld cr6, r11, r10
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[10].u64, &mut ctx.xer);
	// 82384428: 41990008  bgt cr6, 0x82384430
	if ctx.cr[6].gt {
	pc = 0x82384430; continue 'dispatch;
	}
	// 8238442C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82384430: 481B0CD0  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82384438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82384438 size=280
    let mut pc: u32 = 0x82384438;
    'dispatch: loop {
        match pc {
            0x82384438 => {
    //   block [0x82384438..0x82384550)
	// 82384438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238443C: 481B0C7D  bl 0x825350b8
	ctx.lr = 0x82384440;
	sub_82535080(ctx, base);
	// 82384440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82384444: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82384448: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8238444C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82384450: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82384454: C03E0000  lfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82384458: 481AFDE1  bl 0x82534238
	ctx.lr = 0x8238445C;
	sub_82534238(ctx, base);
	// 8238445C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82384460: 418200E4  beq 0x82384544
	if ctx.cr[0].eq {
	pc = 0x82384544; continue 'dispatch;
	}
	// 82384464: C03E0004  lfs f1, 4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82384468: 481AFDD1  bl 0x82534238
	ctx.lr = 0x8238446C;
	sub_82534238(ctx, base);
	// 8238446C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82384470: 418200D4  beq 0x82384544
	if ctx.cr[0].eq {
	pc = 0x82384544; continue 'dispatch;
	}
	// 82384474: C03E0008  lfs f1, 8(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82384478: 481AFDC1  bl 0x82534238
	ctx.lr = 0x8238447C;
	sub_82534238(ctx, base);
	// 8238447C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82384480: 418200C4  beq 0x82384544
	if ctx.cr[0].eq {
	pc = 0x82384544; continue 'dispatch;
	}
	// 82384484: C03E000C  lfs f1, 0xc(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82384488: 481AFDB1  bl 0x82534238
	ctx.lr = 0x8238448C;
	sub_82534238(ctx, base);
	// 8238448C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82384490: 418200B4  beq 0x82384544
	if ctx.cr[0].eq {
	pc = 0x82384544; continue 'dispatch;
	}
	// 82384494: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 82384498: 394BB620  addi r10, r11, -0x49e0
	ctx.r[10].s64 = ctx.r[11].s64 + -18912;
	// 8238449C: 816A0030  lwz r11, 0x30(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 823844A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823844A4: 419A00A0  beq cr6, 0x82384544
	if ctx.cr[6].eq {
	pc = 0x82384544; continue 'dispatch;
	}
	// 823844A8: 57E6103A  slwi r6, r31, 2
	ctx.r[6].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 823844AC: 38AA0010  addi r5, r10, 0x10
	ctx.r[5].s64 = ctx.r[10].s64 + 16;
	// 823844B0: 7D26282E  lwzx r9, r6, r5
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 823844B4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823844B8: 4182000C  beq 0x823844c4
	if ctx.cr[0].eq {
	pc = 0x823844C4; continue 'dispatch;
	}
	// 823844BC: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823844C0: 48000010  b 0x823844d0
	pc = 0x823844D0; continue 'dispatch;
	// 823844C4: 57E9103A  slwi r9, r31, 2
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823844C8: 390A0004  addi r8, r10, 4
	ctx.r[8].s64 = ctx.r[10].s64 + 4;
	// 823844CC: 7D69412E  stwx r11, r9, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[11].u32) };
	// 823844D0: 7D66292E  stwx r11, r6, r5
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[5].u32), ctx.r[11].u32) };
	// 823844D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823844D8: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 823844DC: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 823844E0: 93AB000C  stw r29, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 823844E4: 57E8103A  slwi r8, r31, 2
	ctx.r[8].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823844E8: B38B0008  sth r28, 8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u16 ) };
	// 823844EC: 38EA0020  addi r7, r10, 0x20
	ctx.r[7].s64 = ctx.r[10].s64 + 32;
	// 823844F0: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 823844F4: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 823844F8: E8DE0000  ld r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 823844FC: F8C90000  std r6, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 82384500: E8DE0008  ld r6, 8(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 82384504: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 82384508: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 8238450C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82384510: 912A001C  stw r9, 0x1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 82384514: 7D28382E  lwzx r9, r8, r7
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82384518: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8238451C: 7D28392E  stwx r9, r8, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), ctx.r[9].u32) };
	// 82384520: 916A0030  stw r11, 0x30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82384524: 816A001C  lwz r11, 0x1c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82384528: 812A002C  lwz r9, 0x2c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 8238452C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82384530: 4198000C  blt cr6, 0x8238453c
	if ctx.cr[6].lt {
	pc = 0x8238453C; continue 'dispatch;
	}
	// 82384534: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82384538: 916A0030  stw r11, 0x30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8238453C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82384540: 48000008  b 0x82384548
	pc = 0x82384548; continue 'dispatch;
	// 82384544: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82384548: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8238454C: 481B0BBC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82384550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82384550 size=72
    let mut pc: u32 = 0x82384550;
    'dispatch: loop {
        match pc {
            0x82384550 => {
    //   block [0x82384550..0x82384598)
	// 82384550: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 82384554: 394BB620  addi r10, r11, -0x49e0
	ctx.r[10].s64 = ctx.r[11].s64 + -18912;
	// 82384558: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 8238455C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82384560: 419A0038  beq cr6, 0x82384598
	if ctx.cr[6].eq {
		sub_82384598(ctx, base);
		return;
	}
	// 82384564: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82384568: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238456C: 916A0034  stw r11, 0x34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82384570: 41820028  beq 0x82384598
	if ctx.cr[0].eq {
		sub_82384598(ctx, base);
		return;
	}
	// 82384574: A12B0008  lhz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82384578: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238457C: 41820014  beq 0x82384590
	if ctx.cr[0].eq {
	pc = 0x82384590; continue 'dispatch;
	}
	// 82384580: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82384584: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82384588: 916A0034  stw r11, 0x34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8238458C: 4082FFE8  bne 0x82384574
	if !ctx.cr[0].eq {
	pc = 0x82384574; continue 'dispatch;
	}
	// 82384590: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82384594: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82384598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82384598 size=12
    let mut pc: u32 = 0x82384598;
    'dispatch: loop {
        match pc {
            0x82384598 => {
    //   block [0x82384598..0x823845A4)
	// 82384598: 812A0038  lwz r9, 0x38(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 8238459C: 2F090003  cmpwi cr6, r9, 3
	ctx.cr[6].compare_i32(ctx.r[9].s32, 3, &mut ctx.xer);
	// 823845A0: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823845A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823845A4 size=72
    let mut pc: u32 = 0x823845A4;
    'dispatch: loop {
        match pc {
            0x823845A4 => {
    //   block [0x823845A4..0x823845EC)
	// 823845A4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 823845A8: 390A0004  addi r8, r10, 4
	ctx.r[8].s64 = ctx.r[10].s64 + 4;
	// 823845AC: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823845B0: 912A0038  stw r9, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[9].u32 ) };
	// 823845B4: 7D6B402E  lwzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 823845B8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823845BC: 916A0034  stw r11, 0x34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 823845C0: 4182FFDC  beq 0x8238459c
	if ctx.cr[0].eq {
		sub_82384598(ctx, base);
		return;
	}
	// 823845C4: A10B0008  lhz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 823845C8: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823845CC: 41820014  beq 0x823845e0
	if ctx.cr[0].eq {
	pc = 0x823845E0; continue 'dispatch;
	}
	// 823845D0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823845D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823845D8: 916A0034  stw r11, 0x34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 823845DC: 4082FFE8  bne 0x823845c4
	if !ctx.cr[0].eq {
	pc = 0x823845C4; continue 'dispatch;
	}
	// 823845E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823845E4: 419AFFB8  beq cr6, 0x8238459c
	if ctx.cr[6].eq {
		sub_82384598(ctx, base);
		return;
	}
	// 823845E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823845F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823845F0 size=232
    let mut pc: u32 = 0x823845F0;
    'dispatch: loop {
        match pc {
            0x823845F0 => {
    //   block [0x823845F0..0x823846D8)
	// 823845F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823845F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823845F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823845FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82384600: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82384604: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 82384608: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8238460C: 3BEB86B0  addi r31, r11, -0x7950
	ctx.r[31].s64 = ctx.r[11].s64 + -31056;
	// 82384610: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82384614: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82384618: 419A0014  beq cr6, 0x8238462c
	if ctx.cr[6].eq {
	pc = 0x8238462C; continue 'dispatch;
	}
	// 8238461C: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82384620: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82384624: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82384628: 419A0094  beq cr6, 0x823846bc
	if ctx.cr[6].eq {
	pc = 0x823846BC; continue 'dispatch;
	}
	// 8238462C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82384630: 38C0FC18  li r6, -0x3e8
	ctx.r[6].s64 = -1000;
	// 82384634: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82384638: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8238463C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82384640: 386004B0  li r3, 0x4b0
	ctx.r[3].s64 = 1200;
	// 82384644: 4BFE5E4D  bl 0x8236a490
	ctx.lr = 0x82384648;
	sub_8236A490(ctx, base);
	// 82384648: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238464C: 4182000C  beq 0x82384658
	if ctx.cr[0].eq {
	pc = 0x82384658; continue 'dispatch;
	}
	// 82384650: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82384654: 48000008  b 0x8238465c
	pc = 0x8238465C; continue 'dispatch;
	// 82384658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8238465C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82384660: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82384664: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82384668: 419A0048  beq cr6, 0x823846b0
	if ctx.cr[6].eq {
	pc = 0x823846B0; continue 'dispatch;
	}
	// 8238466C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82384670: 48000069  bl 0x823846d8
	ctx.lr = 0x82384674;
	sub_823846D8(ctx, base);
	// 82384674: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82384678: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8238467C: 419A0034  beq cr6, 0x823846b0
	if ctx.cr[6].eq {
	pc = 0x823846B0; continue 'dispatch;
	}
	// 82384680: 812B0030  lwz r9, 0x30(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82384684: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82384688: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8238468C: 409A0024  bne cr6, 0x823846b0
	if !ctx.cr[6].eq {
	pc = 0x823846B0; continue 'dispatch;
	}
	// 82384690: 896B0019  lbz r11, 0x19(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82384694: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82384698: 409A0018  bne cr6, 0x823846b0
	if !ctx.cr[6].eq {
	pc = 0x823846B0; continue 'dispatch;
	}
	// 8238469C: 3D408284  lis r10, -0x7d7c
	ctx.r[10].s64 = -2105278464;
	// 823846A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823846A4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 823846A8: 916AD54C  stw r11, -0x2ab4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10932 as u32), ctx.r[11].u32 ) };
	// 823846AC: 48000014  b 0x823846c0
	pc = 0x823846C0; continue 'dispatch;
	// 823846B0: 3D408284  lis r10, -0x7d7c
	ctx.r[10].s64 = -2105278464;
	// 823846B4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 823846B8: 916AD54C  stw r11, -0x2ab4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10932 as u32), ctx.r[11].u32 ) };
	// 823846BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823846C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823846C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823846C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823846CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823846D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823846D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823846D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823846D8 size=344
    let mut pc: u32 = 0x823846D8;
    'dispatch: loop {
        match pc {
            0x823846D8 => {
    //   block [0x823846D8..0x82384830)
	// 823846D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823846DC: 481B09D9  bl 0x825350b4
	ctx.lr = 0x823846E0;
	sub_82535080(ctx, base);
	// 823846E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823846E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823846E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823846EC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 823846F0: 394AFCD8  addi r10, r10, -0x328
	ctx.r[10].s64 = ctx.r[10].s64 + -808;
	// 823846F4: A13F0014  lhz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 823846F8: A17E0002  lhz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 823846FC: A11E0000  lhz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82384700: 61298000  ori r9, r9, 0x8000
	ctx.r[9].u64 = ctx.r[9].u64 | 32768;
	// 82384704: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82384708: 2B0B007F  cmplwi cr6, r11, 0x7f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 127 as u32, &mut ctx.xer);
	// 8238470C: 80DE000C  lwz r6, 0xc(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82384710: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82384714: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82384718: B13F0014  sth r9, 0x14(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u16 ) };
	// 8238471C: 911F0080  stw r8, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[8].u32 ) };
	// 82384720: 90FF0090  stw r7, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[7].u32 ) };
	// 82384724: 90DF0088  stw r6, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[6].u32 ) };
	// 82384728: 4099000C  ble cr6, 0x82384734
	if !ctx.cr[6].gt {
	pc = 0x82384734; continue 'dispatch;
	}
	// 8238472C: 3960007F  li r11, 0x7f
	ctx.r[11].s64 = 127;
	// 82384730: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82384734: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82384738: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8238473C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82384740: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82384744: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82384748: 4082002C  bne 0x82384774
	if !ctx.cr[0].eq {
	pc = 0x82384774; continue 'dispatch;
	}
	// 8238474C: 3D4082B5  lis r10, -0x7d4b
	ctx.r[10].s64 = -2102067200;
	// 82384750: 394A0C40  addi r10, r10, 0xc40
	ctx.r[10].s64 = ctx.r[10].s64 + 3136;
	// 82384754: 814A06D8  lwz r10, 0x6d8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1752 as u32) ) } as u64;
	// 82384758: 2B0A0007  cmplwi cr6, r10, 7
	ctx.cr[6].compare_u32(ctx.r[10].u32, 7 as u32, &mut ctx.xer);
	// 8238475C: 419A0014  beq cr6, 0x82384770
	if ctx.cr[6].eq {
	pc = 0x82384770; continue 'dispatch;
	}
	// 82384760: 2B0A0008  cmplwi cr6, r10, 8
	ctx.cr[6].compare_u32(ctx.r[10].u32, 8 as u32, &mut ctx.xer);
	// 82384764: 419A000C  beq cr6, 0x82384770
	if ctx.cr[6].eq {
	pc = 0x82384770; continue 'dispatch;
	}
	// 82384768: 2B0A0009  cmplwi cr6, r10, 9
	ctx.cr[6].compare_u32(ctx.r[10].u32, 9 as u32, &mut ctx.xer);
	// 8238476C: 409A0024  bne cr6, 0x82384790
	if !ctx.cr[6].eq {
	pc = 0x82384790; continue 'dispatch;
	}
	// 82384770: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82384774: 2B0B000B  cmplwi cr6, r11, 0xb
	ctx.cr[6].compare_u32(ctx.r[11].u32, 11 as u32, &mut ctx.xer);
	// 82384778: 419A0014  beq cr6, 0x8238478c
	if ctx.cr[6].eq {
	pc = 0x8238478C; continue 'dispatch;
	}
	// 8238477C: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 82384780: 419A000C  beq cr6, 0x8238478c
	if ctx.cr[6].eq {
	pc = 0x8238478C; continue 'dispatch;
	}
	// 82384784: 2B0B000F  cmplwi cr6, r11, 0xf
	ctx.cr[6].compare_u32(ctx.r[11].u32, 15 as u32, &mut ctx.xer);
	// 82384788: 409A0008  bne cr6, 0x82384790
	if !ctx.cr[6].eq {
	pc = 0x82384790; continue 'dispatch;
	}
	// 8238478C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82384790: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82384794: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82384798: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8238479C: 394AFC60  addi r10, r10, -0x3a0
	ctx.r[10].s64 = ctx.r[10].s64 + -928;
	// 823847A0: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 823847A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823847A8: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 823847AC: 656B2000  oris r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 536870912;
	// 823847B0: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 823847B4: 41820014  beq 0x823847c8
	if ctx.cr[0].eq {
	pc = 0x823847C8; continue 'dispatch;
	}
	// 823847B8: 389F0194  addi r4, r31, 0x194
	ctx.r[4].s64 = ctx.r[31].s64 + 404;
	// 823847BC: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 823847C0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 823847C4: 4BFF2BF5  bl 0x823773b8
	ctx.lr = 0x823847C8;
	sub_823773B8(ctx, base);
	// 823847C8: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 823847CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823847D0: 41820014  beq 0x823847e4
	if ctx.cr[0].eq {
	pc = 0x823847E4; continue 'dispatch;
	}
	// 823847D4: 389F0394  addi r4, r31, 0x394
	ctx.r[4].s64 = ctx.r[31].s64 + 916;
	// 823847D8: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 823847DC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 823847E0: 4BFF2BD9  bl 0x823773b8
	ctx.lr = 0x823847E4;
	sub_823773B8(ctx, base);
	// 823847E4: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 823847E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823847EC: 41820014  beq 0x82384800
	if ctx.cr[0].eq {
	pc = 0x82384800; continue 'dispatch;
	}
	// 823847F0: 389F0294  addi r4, r31, 0x294
	ctx.r[4].s64 = ctx.r[31].s64 + 660;
	// 823847F4: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 823847F8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 823847FC: 4BFF2BBD  bl 0x823773b8
	ctx.lr = 0x82384800;
	sub_823773B8(ctx, base);
	// 82384800: 395F0494  addi r10, r31, 0x494
	ctx.r[10].s64 = ctx.r[31].s64 + 1172;
	// 82384804: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82384808: 3920007F  li r9, 0x7f
	ctx.r[9].s64 = 127;
	// 8238480C: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82384810: 391F0094  addi r8, r31, 0x94
	ctx.r[8].s64 = ctx.r[31].s64 + 148;
	// 82384814: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82384818: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8238481C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82384820: 4803B3D1  bl 0x823bfbf0
	ctx.lr = 0x82384824;
	sub_823BFBF0(ctx, base);
	// 82384824: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82384828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8238482C: 481B08D8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82384830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82384830 size=164
    let mut pc: u32 = 0x82384830;
    'dispatch: loop {
        match pc {
            0x82384830 => {
    //   block [0x82384830..0x823848D4)
	// 82384830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82384834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82384838: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238483C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82384840: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82384844: 817F0494  lwz r11, 0x494(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1172 as u32) ) } as u64;
	// 82384848: 396BFC1B  addi r11, r11, -0x3e5
	ctx.r[11].s64 = ctx.r[11].s64 + -997;
	// 8238484C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82384850: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82384854: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82384858: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8238485C: 41820058  beq 0x823848b4
	if ctx.cr[0].eq {
	pc = 0x823848B4; continue 'dispatch;
	}
	// 82384860: A17F0094  lhz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82384864: 387F0094  addi r3, r31, 0x94
	ctx.r[3].s64 = ctx.r[31].s64 + 148;
	// 82384868: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238486C: 40820008  bne 0x82384874
	if !ctx.cr[0].eq {
	pc = 0x82384874; continue 'dispatch;
	}
	// 82384870: 387F0394  addi r3, r31, 0x394
	ctx.r[3].s64 = ctx.r[31].s64 + 916;
	// 82384874: 80DF008C  lwz r6, 0x8c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82384878: 80BF0088  lwz r5, 0x88(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 8238487C: 809F0090  lwz r4, 0x90(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82384880: 4BFF29C1  bl 0x82377240
	ctx.lr = 0x82384884;
	sub_82377240(ctx, base);
	// 82384884: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82384888: 3D408284  lis r10, -0x7d7c
	ctx.r[10].s64 = -2105278464;
	// 8238488C: 916AD54C  stw r11, -0x2ab4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10932 as u32), ctx.r[11].u32 ) };
	// 82384890: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82384894: 816BFAC0  lwz r11, -0x540(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82384898: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8238489C: 409A0024  bne cr6, 0x823848c0
	if !ctx.cr[6].eq {
	pc = 0x823848C0; continue 'dispatch;
	}
	// 823848A0: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 823848A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823848A8: 997F0019  stb r11, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 823848AC: 995F0025  stb r10, 0x25(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(37 as u32), ctx.r[10].u8 ) };
	// 823848B0: 48000010  b 0x823848c0
	pc = 0x823848C0; continue 'dispatch;
	// 823848B4: 3D408284  lis r10, -0x7d7c
	ctx.r[10].s64 = -2105278464;
	// 823848B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823848BC: 916AD54C  stw r11, -0x2ab4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10932 as u32), ctx.r[11].u32 ) };
	// 823848C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823848C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823848C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823848CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823848D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823848D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823848D8 size=432
    let mut pc: u32 = 0x823848D8;
    'dispatch: loop {
        match pc {
            0x823848D8 => {
    //   block [0x823848D8..0x82384A88)
	// 823848D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823848DC: 481B07DD  bl 0x825350b8
	ctx.lr = 0x823848E0;
	sub_82535080(ctx, base);
	// 823848E0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823848E4: B083000C  sth r4, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u16 ) };
	// 823848E8: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 823848EC: 39230040  addi r9, r3, 0x40
	ctx.r[9].s64 = ctx.r[3].s64 + 64;
	// 823848F0: 38C30050  addi r6, r3, 0x50
	ctx.r[6].s64 = ctx.r[3].s64 + 80;
	// 823848F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 823848F8: C18A2934  lfs f12, 0x2934(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(10548 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 823848FC: 3D408288  lis r10, -0x7d78
	ctx.r[10].s64 = -2105016320;
	// 82384900: 3FC0830F  lis r30, -0x7cf1
	ctx.r[30].s64 = -2096168960;
	// 82384904: 390A8C60  addi r8, r10, -0x73a0
	ctx.r[8].s64 = ctx.r[10].s64 + -29600;
	// 82384908: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8238490C: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82384910: C00BBA38  lfs f0, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82384914: 7D054378  mr r5, r8
	ctx.r[5].u64 = ctx.r[8].u64;
	// 82384918: 3BDE4A00  addi r30, r30, 0x4a00
	ctx.r[30].s64 = ctx.r[30].s64 + 18944;
	// 8238491C: 396300D0  addi r11, r3, 0xd0
	ctx.r[11].s64 = ctx.r[3].s64 + 208;
	// 82384920: C16ABFFC  lfs f11, -0x4004(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82384924: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82384928: 3BE80100  addi r31, r8, 0x100
	ctx.r[31].s64 = ctx.r[8].s64 + 256;
	// 8238492C: C1AA1FF8  lfs f13, 0x1ff8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82384930: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82384934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82384938: 7FA42030  slw r4, r29, r4
	if (ctx.r[4].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[29].u32) << ((ctx.r[4].u8 & 0x1F) as u32)) as u64;
	}
	// 8238493C: 99230010  stb r9, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u8 ) };
	// 82384940: 9083001C  stw r4, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82384944: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82384948: 99230011  stb r9, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[9].u8 ) };
	// 8238494C: EB870000  ld r28, 0(r7)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	// 82384950: FB8A0000  std r28, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[28].u64 ) };
	// 82384954: E8E70008  ld r7, 8(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) };
	// 82384958: F8EA0008  std r7, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[7].u64 ) };
	// 8238495C: 38E80100  addi r7, r8, 0x100
	ctx.r[7].s64 = ctx.r[8].s64 + 256;
	// 82384960: E9450000  ld r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82384964: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 82384968: F9460000  std r10, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 8238496C: E9450008  ld r10, 8(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82384970: D1830060  stfs f12, 0x60(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82384974: D1630064  stfs f11, 0x64(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82384978: B123000E  sth r9, 0xe(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[9].u16 ) };
	// 8238497C: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82384980: C1882100  lfs f12, 0x2100(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8448 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82384984: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82384988: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 8238498C: F9460008  std r10, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 82384990: D1A30080  stfs f13, 0x80(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82384994: D1A30084  stfs f13, 0x84(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82384998: D1A30088  stfs f13, 0x88(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 8238499C: D003008C  stfs f0, 0x8c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 823849A0: 93C30068  stw r30, 0x68(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 823849A4: 9123006C  stw r9, 0x6c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 823849A8: C1672090  lfs f11, 0x2090(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8336 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 823849AC: 91230070  stw r9, 0x70(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 823849B0: 9BA30012  stb r29, 0x12(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(18 as u32), ctx.r[29].u8 ) };
	// 823849B4: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 823849B8: F92B0008  std r9, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u64 ) };
	// 823849BC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823849C0: E95F0000  ld r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 823849C4: F9440000  std r10, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 823849C8: E95F0008  ld r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 823849CC: F9440008  std r10, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 823849D0: D1AB0040  stfs f13, 0x40(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 823849D4: D1AB0044  stfs f13, 0x44(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 823849D8: 39430290  addi r10, r3, 0x290
	ctx.r[10].s64 = ctx.r[3].s64 + 656;
	// 823849DC: D1AB0048  stfs f13, 0x48(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 823849E0: D00B004C  stfs f0, 0x4c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 823849E4: B12B0024  sth r9, 0x24(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[9].u16 ) };
	// 823849E8: D18B0014  stfs f12, 0x14(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 823849EC: B12B0026  sth r9, 0x26(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(38 as u32), ctx.r[9].u16 ) };
	// 823849F0: D1AB0060  stfs f13, 0x60(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 823849F4: F92B00A8  std r9, 0xa8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(168 as u32), ctx.r[9].u64 ) };
	// 823849F8: D1AB0064  stfs f13, 0x64(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 823849FC: D1AB0068  stfs f13, 0x68(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82384A00: D1AB006C  stfs f13, 0x6c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82384A04: D00B0018  stfs f0, 0x18(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82384A08: D00B001C  stfs f0, 0x1c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82384A0C: D16B0020  stfs f11, 0x20(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82384A10: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82384A14: D00A0008  stfs f0, 8(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82384A18: D00A0010  stfs f0, 0x10(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82384A1C: C18B2074  lfs f12, 0x2074(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8308 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82384A20: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82384A24: D00A0014  stfs f0, 0x14(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82384A28: B12A000C  sth r9, 0xc(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[9].u16 ) };
	// 82384A2C: D00A0018  stfs f0, 0x18(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82384A30: B12A000E  sth r9, 0xe(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(14 as u32), ctx.r[9].u16 ) };
	// 82384A34: 396A0040  addi r11, r10, 0x40
	ctx.r[11].s64 = ctx.r[10].s64 + 64;
	// 82384A38: E9280000  ld r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 82384A3C: F92A0020  std r9, 0x20(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[9].u64 ) };
	// 82384A40: E9280008  ld r9, 8(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) };
	// 82384A44: F92A0028  std r9, 0x28(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(40 as u32), ctx.r[9].u64 ) };
	// 82384A48: D1AA0030  stfs f13, 0x30(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82384A4C: D18A0034  stfs f12, 0x34(r10)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82384A50: D1AA0038  stfs f13, 0x38(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82384A54: D00A003C  stfs f0, 0x3c(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82384A58: E92A0020  ld r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) };
	// 82384A5C: E90A0028  ld r8, 0x28(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) };
	// 82384A60: E8EA0030  ld r7, 0x30(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) };
	// 82384A64: E94A0038  ld r10, 0x38(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) };
	// 82384A68: F92B0000  std r9, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82384A6C: F90B0008  std r8, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[8].u64 ) };
	// 82384A70: F8EB0010  std r7, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[7].u64 ) };
	// 82384A74: F94B0018  std r10, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u64 ) };
	// 82384A78: D0030360  stfs f0, 0x360(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(864 as u32), tmp.u32 ) };
	// 82384A7C: D0030364  stfs f0, 0x364(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(868 as u32), tmp.u32 ) };
	// 82384A80: D0030368  stfs f0, 0x368(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(872 as u32), tmp.u32 ) };
	// 82384A84: 481B0684  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82384A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82384A88 size=7252
    let mut pc: u32 = 0x82384A88;
    'dispatch: loop {
        match pc {
            0x82384A88 => {
    //   block [0x82384A88..0x823866DC)
	// 82384A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82384A8C: 481B05F5  bl 0x82535080
	ctx.lr = 0x82384A90;
	sub_82535080(ctx, base);
	// 82384A90: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82384A94: 481B151D  bl 0x82535fb0
	ctx.lr = 0x82384A98;
	sub_82535FB0(ctx, base);
	// 82384A98: 3981FED0  addi r12, r1, -0x130
	ctx.r[12].s64 = ctx.r[1].s64 + -304;
	// 82384A9C: 481B4909  bl 0x825393a4
	ctx.lr = 0x82384AA0;
	sub_82539130(ctx, base);
	// 82384AA0: 9421FD20  stwu r1, -0x2e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-736 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82384AA4: 7CAE2B78  mr r14, r5
	ctx.r[14].u64 = ctx.r[5].u64;
	// 82384AA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82384AAC: 3A6E00F0  addi r19, r14, 0xf0
	ctx.r[19].s64 = ctx.r[14].s64 + 240;
	// 82384AB0: C0330000  lfs f1, 0(r19)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82384AB4: 481AF785  bl 0x82534238
	ctx.lr = 0x82384AB8;
	sub_82534238(ctx, base);
	// 82384AB8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82384ABC: 41820034  beq 0x82384af0
	if ctx.cr[0].eq {
	pc = 0x82384AF0; continue 'dispatch;
	}
	// 82384AC0: C0330004  lfs f1, 4(r19)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82384AC4: 481AF775  bl 0x82534238
	ctx.lr = 0x82384AC8;
	sub_82534238(ctx, base);
	// 82384AC8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82384ACC: 41820024  beq 0x82384af0
	if ctx.cr[0].eq {
	pc = 0x82384AF0; continue 'dispatch;
	}
	// 82384AD0: C0330008  lfs f1, 8(r19)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82384AD4: 481AF765  bl 0x82534238
	ctx.lr = 0x82384AD8;
	sub_82534238(ctx, base);
	// 82384AD8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82384ADC: 41820014  beq 0x82384af0
	if ctx.cr[0].eq {
	pc = 0x82384AF0; continue 'dispatch;
	}
	// 82384AE0: C033000C  lfs f1, 0xc(r19)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(12 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82384AE4: 481AF755  bl 0x82534238
	ctx.lr = 0x82384AE8;
	sub_82534238(ctx, base);
	// 82384AE8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82384AEC: 40820018  bne 0x82384b04
	if !ctx.cr[0].eq {
	pc = 0x82384B04; continue 'dispatch;
	}
	// 82384AF0: 397F0080  addi r11, r31, 0x80
	ctx.r[11].s64 = ctx.r[31].s64 + 128;
	// 82384AF4: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82384AF8: F9530000  std r10, 0(r19)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[19].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82384AFC: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82384B00: F9730008  std r11, 8(r19)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[19].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82384B04: 3ACE0100  addi r22, r14, 0x100
	ctx.r[22].s64 = ctx.r[14].s64 + 256;
	// 82384B08: C0360000  lfs f1, 0(r22)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82384B0C: 481AF72D  bl 0x82534238
	ctx.lr = 0x82384B10;
	sub_82534238(ctx, base);
	// 82384B10: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82384B14: 41820034  beq 0x82384b48
	if ctx.cr[0].eq {
	pc = 0x82384B48; continue 'dispatch;
	}
	// 82384B18: C0360004  lfs f1, 4(r22)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82384B1C: 481AF71D  bl 0x82534238
	ctx.lr = 0x82384B20;
	sub_82534238(ctx, base);
	// 82384B20: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82384B24: 41820024  beq 0x82384b48
	if ctx.cr[0].eq {
	pc = 0x82384B48; continue 'dispatch;
	}
	// 82384B28: C0360008  lfs f1, 8(r22)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82384B2C: 481AF70D  bl 0x82534238
	ctx.lr = 0x82384B30;
	sub_82534238(ctx, base);
	// 82384B30: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82384B34: 41820014  beq 0x82384b48
	if ctx.cr[0].eq {
	pc = 0x82384B48; continue 'dispatch;
	}
	// 82384B38: C036000C  lfs f1, 0xc(r22)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(12 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82384B3C: 481AF6FD  bl 0x82534238
	ctx.lr = 0x82384B40;
	sub_82534238(ctx, base);
	// 82384B40: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82384B44: 40820020  bne 0x82384b64
	if !ctx.cr[0].eq {
	pc = 0x82384B64; continue 'dispatch;
	}
	// 82384B48: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82384B4C: 396B8C60  addi r11, r11, -0x73a0
	ctx.r[11].s64 = ctx.r[11].s64 + -29600;
	// 82384B50: 396B0100  addi r11, r11, 0x100
	ctx.r[11].s64 = ctx.r[11].s64 + 256;
	// 82384B54: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82384B58: F9560000  std r10, 0(r22)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82384B5C: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82384B60: F9760008  std r11, 8(r22)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[22].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82384B64: 3D60BBBB  lis r11, -0x4445
	ctx.r[11].s64 = -1145372672;
	// 82384B68: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82384B6C: 61701111  ori r16, r11, 0x1111
	ctx.r[16].u64 = ctx.r[11].u64 | 4369;
	// 82384B70: 3D602222  lis r11, 0x2222
	ctx.r[11].s64 = 572653568;
	// 82384B74: 39E00000  li r15, 0
	ctx.r[15].s64 = 0;
	// 82384B78: 7970000E  rldimi r16, r11, 0x20, 0
	ctx.r[16].u64 = ((ctx.r[11].u64).rotate_left(32) & 0xFFFFFFFF00000000) | (ctx.r[16].u64 & 0x00000000FFFFFFFF);
	// 82384B7C: E97F00D8  ld r11, 0xd8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) };
	// 82384B80: C2EABFFC  lfs f23, -0x4004(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 82384B84: 7DFE7B78  mr r30, r15
	ctx.r[30].u64 = ctx.r[15].u64;
	// 82384B88: D2E10090  stfs f23, 0x90(r1)
	tmp.f32 = (ctx.f[23].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 82384B8C: 7F2B8040  cmpld cr6, r11, r16
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[16].u64, &mut ctx.xer);
	// 82384B90: 419A001C  beq cr6, 0x82384bac
	if ctx.cr[6].eq {
	pc = 0x82384BAC; continue 'dispatch;
	}
	// 82384B94: 815F00D8  lwz r10, 0xd8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82384B98: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82384B9C: 41820010  beq 0x82384bac
	if ctx.cr[0].eq {
	pc = 0x82384BAC; continue 'dispatch;
	}
	// 82384BA0: E94A0038  ld r10, 0x38(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) };
	// 82384BA4: 7F2A5840  cmpld cr6, r10, r11
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[11].u64, &mut ctx.xer);
	// 82384BA8: 419A006C  beq cr6, 0x82384c14
	if ctx.cr[6].eq {
	pc = 0x82384C14; continue 'dispatch;
	}
	// 82384BAC: E97F0178  ld r11, 0x178(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(376 as u32) ) };
	// 82384BB0: 7F2B8040  cmpld cr6, r11, r16
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[16].u64, &mut ctx.xer);
	// 82384BB4: 409A0060  bne cr6, 0x82384c14
	if !ctx.cr[6].eq {
	pc = 0x82384C14; continue 'dispatch;
	}
	// 82384BB8: C01F0278  lfs f0, 0x278(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82384BBC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82384BC0: EC0005F2  fmuls f0, f0, f23
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[23].f64) as f32) as f64);
	// 82384BC4: FA1F00D8  std r16, 0xd8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[16].u64 ) };
	// 82384BC8: C1AB2620  lfs f13, 0x2620(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9760 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82384BCC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82384BD0: 40990044  ble cr6, 0x82384c14
	if !ctx.cr[6].gt {
	pc = 0x82384C14; continue 'dispatch;
	}
	// 82384BD4: 3BDF0140  addi r30, r31, 0x140
	ctx.r[30].s64 = ctx.r[31].s64 + 320;
	// 82384BD8: 3BBF01E0  addi r29, r31, 0x1e0
	ctx.r[29].s64 = ctx.r[31].s64 + 480;
	// 82384BDC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82384BE0: 91FE0014  stw r15, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[15].u32 ) };
	// 82384BE4: D01F0278  stfs f0, 0x278(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(632 as u32), tmp.u32 ) };
	// 82384BE8: 817F01E0  lwz r11, 0x1e0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(480 as u32) ) } as u64;
	// 82384BEC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82384BF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82384BF4: 4E800421  bctrl
	ctx.lr = 0x82384BF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82384BF8: 817F01E0  lwz r11, 0x1e0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(480 as u32) ) } as u64;
	// 82384BFC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82384C00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82384C04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82384C08: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82384C0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82384C10: 4E800421  bctrl
	ctx.lr = 0x82384C14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82384C14: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82384C18: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82384C1C: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82384C20: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82384C24: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 82384C28: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82384C2C: C1C92068  lfs f14, 0x2068(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8296 as u32) ) };
	ctx.f[14].f64 = (tmp.f32 as f64);
	// 82384C30: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82384C34: C22A2038  lfs f17, 0x2038(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8248 as u32) ) };
	ctx.f[17].f64 = (tmp.f32 as f64);
	// 82384C38: C24B1FF8  lfs f18, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[18].f64 = (tmp.f32 as f64);
	// 82384C3C: C3E8BA38  lfs f31, -0x45c8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82384C40: D2410060  stfs f18, 0x60(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82384C44: D2210158  stfs f17, 0x158(r1)
	tmp.f32 = (ctx.f[17].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(344 as u32), tmp.u32 ) };
	// 82384C48: D1C10064  stfs f14, 0x64(r1)
	tmp.f32 = (ctx.f[14].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82384C4C: 41820114  beq 0x82384d60
	if ctx.cr[0].eq {
	pc = 0x82384D60; continue 'dispatch;
	}
	// 82384C50: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 82384C54: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82384C58: C38A9F78  lfs f28, -0x6088(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24712 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82384C5C: C3AB2284  lfs f29, 0x2284(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8836 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82384C60: 897D0024  lbz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82384C64: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82384C68: 409A00E0  bne cr6, 0x82384d48
	if !ctx.cr[6].eq {
	pc = 0x82384D48; continue 'dispatch;
	}
	// 82384C6C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82384C70: 419A00CC  beq cr6, 0x82384d3c
	if ctx.cr[6].eq {
	pc = 0x82384D3C; continue 'dispatch;
	}
	// 82384C74: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82384C78: C3DD0030  lfs f30, 0x30(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82384C7C: 7DFB7B78  mr r27, r15
	ctx.r[27].u64 = ctx.r[15].u64;
	// 82384C80: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82384C84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82384C88: 409A007C  bne cr6, 0x82384d04
	if !ctx.cr[6].eq {
	pc = 0x82384D04; continue 'dispatch;
	}
	// 82384C8C: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82384C90: 896B0027  lbz r11, 0x27(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(39 as u32) ) } as u64;
	// 82384C94: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82384C98: 409A0030  bne cr6, 0x82384cc8
	if !ctx.cr[6].eq {
	pc = 0x82384CC8; continue 'dispatch;
	}
	// 82384C9C: FF1EF800  fcmpu cr6, f30, f31
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 82384CA0: 40980020  bge cr6, 0x82384cc0
	if !ctx.cr[6].lt {
	pc = 0x82384CC0; continue 'dispatch;
	}
	// 82384CA4: FF1E9000  fcmpu cr6, f30, f18
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[18].f64);
	// 82384CA8: 419A0030  beq cr6, 0x82384cd8
	if ctx.cr[6].eq {
	pc = 0x82384CD8; continue 'dispatch;
	}
	// 82384CAC: FC20F210  fabs f1, f30
	ctx.f[1].u64 = ctx.f[30].u64 & !0x8000_0000_0000_0000u64;
	// 82384CB0: 481AE329  bl 0x82532fd8
	ctx.lr = 0x82384CB4;
	sub_82532FD8(ctx, base);
	// 82384CB4: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82384CB8: EC200472  fmuls f1, f0, f17
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[17].f64) as f32) as f64);
	// 82384CBC: 48000034  b 0x82384cf0
	pc = 0x82384CF0; continue 'dispatch;
	// 82384CC0: EFDE03B2  fmuls f30, f30, f14
	ctx.f[30].f64 = (((ctx.f[30].f64 * ctx.f[14].f64) as f32) as f64);
	// 82384CC4: 48000040  b 0x82384d04
	pc = 0x82384D04; continue 'dispatch;
	// 82384CC8: FF1EF800  fcmpu cr6, f30, f31
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 82384CCC: 40980034  bge cr6, 0x82384d00
	if !ctx.cr[6].lt {
	pc = 0x82384D00; continue 'dispatch;
	}
	// 82384CD0: FF1E9000  fcmpu cr6, f30, f18
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[18].f64);
	// 82384CD4: 409A000C  bne cr6, 0x82384ce0
	if !ctx.cr[6].eq {
	pc = 0x82384CE0; continue 'dispatch;
	}
	// 82384CD8: FC009090  fmr f0, f18
	ctx.f[0].f64 = ctx.f[18].f64;
	// 82384CDC: 4800001C  b 0x82384cf8
	pc = 0x82384CF8; continue 'dispatch;
	// 82384CE0: FC20F210  fabs f1, f30
	ctx.f[1].u64 = ctx.f[30].u64 & !0x8000_0000_0000_0000u64;
	// 82384CE4: 481AE2F5  bl 0x82532fd8
	ctx.lr = 0x82384CE8;
	sub_82532FD8(ctx, base);
	// 82384CE8: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82384CEC: EC200772  fmuls f1, f0, f29
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[29].f64) as f32) as f64);
	// 82384CF0: 481AE7A9  bl 0x82533498
	ctx.lr = 0x82384CF4;
	sub_82533498(ctx, base);
	// 82384CF4: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82384CF8: EFC007B2  fmuls f30, f0, f30
	ctx.f[30].f64 = (((ctx.f[0].f64 * ctx.f[30].f64) as f32) as f64);
	// 82384CFC: 48000008  b 0x82384d04
	pc = 0x82384D04; continue 'dispatch;
	// 82384D00: EFDE0732  fmuls f30, f30, f28
	ctx.f[30].f64 = (((ctx.f[30].f64 * ctx.f[28].f64) as f32) as f64);
	// 82384D04: C01C0030  lfs f0, 0x30(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82384D08: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 82384D0C: 40980014  bge cr6, 0x82384d20
	if !ctx.cr[6].lt {
	pc = 0x82384D20; continue 'dispatch;
	}
	// 82384D10: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 82384D14: 839C0014  lwz r28, 0x14(r28)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82384D18: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82384D1C: 4082FFE8  bne 0x82384d04
	if !ctx.cr[0].eq {
	pc = 0x82384D04; continue 'dispatch;
	}
	// 82384D20: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82384D24: 409A000C  bne cr6, 0x82384d30
	if !ctx.cr[6].eq {
	pc = 0x82384D30; continue 'dispatch;
	}
	// 82384D28: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82384D2C: 48000008  b 0x82384d34
	pc = 0x82384D34; continue 'dispatch;
	// 82384D30: 93BB0014  stw r29, 0x14(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82384D34: 939D0014  stw r28, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82384D38: 48000014  b 0x82384d4c
	pc = 0x82384D4C; continue 'dispatch;
	// 82384D3C: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82384D40: 91FD0014  stw r15, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[15].u32 ) };
	// 82384D44: 48000008  b 0x82384d4c
	pc = 0x82384D4C; continue 'dispatch;
	// 82384D48: 41990018  bgt cr6, 0x82384d60
	if ctx.cr[6].gt {
	pc = 0x82384D60; continue 'dispatch;
	}
	// 82384D4C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82384D50: 83AB0004  lwz r29, 4(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82384D54: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82384D58: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82384D5C: 4082FF04  bne 0x82384c60
	if !ctx.cr[0].eq {
	pc = 0x82384C60; continue 'dispatch;
	}
	// 82384D60: 3EA082C0  lis r21, -0x7d40
	ctx.r[21].s64 = -2101346304;
	// 82384D64: D2410120  stfs f18, 0x120(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(288 as u32), tmp.u32 ) };
	// 82384D68: D2410124  stfs f18, 0x124(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(292 as u32), tmp.u32 ) };
	// 82384D6C: D2410128  stfs f18, 0x128(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(296 as u32), tmp.u32 ) };
	// 82384D70: D241012C  stfs f18, 0x12c(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(300 as u32), tmp.u32 ) };
	// 82384D74: D2410130  stfs f18, 0x130(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(304 as u32), tmp.u32 ) };
	// 82384D78: 8175BF90  lwz r11, -0x4070(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-16496 as u32) ) } as u64;
	// 82384D7C: D2410134  stfs f18, 0x134(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(308 as u32), tmp.u32 ) };
	// 82384D80: D2410138  stfs f18, 0x138(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(312 as u32), tmp.u32 ) };
	// 82384D84: D241013C  stfs f18, 0x13c(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(316 as u32), tmp.u32 ) };
	// 82384D88: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82384D8C: D24100A0  stfs f18, 0xa0(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 82384D90: D24100A4  stfs f18, 0xa4(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 82384D94: D24100A8  stfs f18, 0xa8(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 82384D98: D24100AC  stfs f18, 0xac(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 82384D9C: D2410110  stfs f18, 0x110(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(272 as u32), tmp.u32 ) };
	// 82384DA0: D2410114  stfs f18, 0x114(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(276 as u32), tmp.u32 ) };
	// 82384DA4: D2410118  stfs f18, 0x118(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(280 as u32), tmp.u32 ) };
	// 82384DA8: D241011C  stfs f18, 0x11c(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(284 as u32), tmp.u32 ) };
	// 82384DAC: 4082000C  bne 0x82384db8
	if !ctx.cr[0].eq {
	pc = 0x82384DB8; continue 'dispatch;
	}
	// 82384DB0: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82384DB4: 396B1E40  addi r11, r11, 0x1e40
	ctx.r[11].s64 = ctx.r[11].s64 + 7744;
	// 82384DB8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82384DBC: D2410100  stfs f18, 0x100(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), tmp.u32 ) };
	// 82384DC0: 396B0180  addi r11, r11, 0x180
	ctx.r[11].s64 = ctx.r[11].s64 + 384;
	// 82384DC4: D2410104  stfs f18, 0x104(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), tmp.u32 ) };
	// 82384DC8: 3B400020  li r26, 0x20
	ctx.r[26].s64 = 32;
	// 82384DCC: D241010C  stfs f18, 0x10c(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(268 as u32), tmp.u32 ) };
	// 82384DD0: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 82384DD4: 891F0011  lbz r8, 0x11(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(17 as u32) ) } as u64;
	// 82384DD8: 392E00C0  addi r9, r14, 0xc0
	ctx.r[9].s64 = ctx.r[14].s64 + 192;
	// 82384DDC: C14A2074  lfs f10, 0x2074(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8308 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82384DE0: 394100D0  addi r10, r1, 0xd0
	ctx.r[10].s64 = ctx.r[1].s64 + 208;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823866E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823866E0 size=364
    let mut pc: u32 = 0x823866E0;
    'dispatch: loop {
        match pc {
            0x823866E0 => {
    //   block [0x823866E0..0x8238684C)
	// 823866E0: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 823866E4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 823866E8: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 823866EC: 39640050  addi r11, r4, 0x50
	ctx.r[11].s64 = ctx.r[4].s64 + 80;
	// 823866F0: 39230050  addi r9, r3, 0x50
	ctx.r[9].s64 = ctx.r[3].s64 + 80;
	// 823866F4: 39030060  addi r8, r3, 0x60
	ctx.r[8].s64 = ctx.r[3].s64 + 96;
	// 823866F8: 38C40070  addi r6, r4, 0x70
	ctx.r[6].s64 = ctx.r[4].s64 + 112;
	// 823866FC: 38A30070  addi r5, r3, 0x70
	ctx.r[5].s64 = ctx.r[3].s64 + 112;
	// 82386700: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82386704: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82386708: 80E40008  lwz r7, 8(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238670C: 39240060  addi r9, r4, 0x60
	ctx.r[9].s64 = ctx.r[4].s64 + 96;
	// 82386710: 3BC30080  addi r30, r3, 0x80
	ctx.r[30].s64 = ctx.r[3].s64 + 128;
	// 82386714: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82386718: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8238671C: 83E4000C  lwz r31, 0xc(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82386720: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82386724: 38A40080  addi r5, r4, 0x80
	ctx.r[5].s64 = ctx.r[4].s64 + 128;
	// 82386728: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8238672C: 83E40010  lwz r31, 0x10(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82386730: 93E30010  stw r31, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 82386734: 83E40014  lwz r31, 0x14(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82386738: 93E30014  stw r31, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 8238673C: 8BE40018  lbz r31, 0x18(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82386740: 9BE30018  stb r31, 0x18(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[31].u8 ) };
	// 82386744: 8BE40019  lbz r31, 0x19(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(25 as u32) ) } as u64;
	// 82386748: 9BE30019  stb r31, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[31].u8 ) };
	// 8238674C: 8BE4001A  lbz r31, 0x1a(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(26 as u32) ) } as u64;
	// 82386750: 9BE3001A  stb r31, 0x1a(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(26 as u32), ctx.r[31].u8 ) };
	// 82386754: 8BE4001B  lbz r31, 0x1b(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(27 as u32) ) } as u64;
	// 82386758: 9BE3001B  stb r31, 0x1b(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(27 as u32), ctx.r[31].u8 ) };
	// 8238675C: 83E4001C  lwz r31, 0x1c(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 82386760: 93E3001C  stw r31, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[31].u32 ) };
	// 82386764: 83E40020  lwz r31, 0x20(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 82386768: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 8238676C: 83E40024  lwz r31, 0x24(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 82386770: 93E30024  stw r31, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 82386774: 8BE40024  lbz r31, 0x24(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 82386778: 9BE30024  stb r31, 0x24(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[31].u8 ) };
	// 8238677C: 8BE40025  lbz r31, 0x25(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(37 as u32) ) } as u64;
	// 82386780: 9BE30025  stb r31, 0x25(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(37 as u32), ctx.r[31].u8 ) };
	// 82386784: 8BE40026  lbz r31, 0x26(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(38 as u32) ) } as u64;
	// 82386788: 9BE30026  stb r31, 0x26(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(38 as u32), ctx.r[31].u8 ) };
	// 8238678C: 8BE40027  lbz r31, 0x27(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(39 as u32) ) } as u64;
	// 82386790: 9BE30027  stb r31, 0x27(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(39 as u32), ctx.r[31].u8 ) };
	// 82386794: 83E40028  lwz r31, 0x28(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82386798: 93E30028  stw r31, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[31].u32 ) };
	// 8238679C: C004002C  lfs f0, 0x2c(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823867A0: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 823867A4: C0040030  lfs f0, 0x30(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823867A8: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 823867AC: 83E40034  lwz r31, 0x34(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 823867B0: 93E30034  stw r31, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[31].u32 ) };
	// 823867B4: EBE40038  ld r31, 0x38(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) };
	// 823867B8: FBE30038  std r31, 0x38(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[31].u64 ) };
	// 823867BC: C0040040  lfs f0, 0x40(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823867C0: D0030040  stfs f0, 0x40(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 823867C4: C0040044  lfs f0, 0x44(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823867C8: D0030044  stfs f0, 0x44(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 823867CC: 83E40048  lwz r31, 0x48(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 823867D0: 93E30048  stw r31, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 823867D4: 83E4004C  lwz r31, 0x4c(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 823867D8: 93E3004C  stw r31, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[31].u32 ) };
	// 823867DC: EBEB0000  ld r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 823867E0: FBEA0000  std r31, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 823867E4: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 823867E8: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 823867EC: E9690000  ld r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 823867F0: F9680000  std r11, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 823867F4: E9690008  ld r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	// 823867F8: F9680008  std r11, 8(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 823867FC: E9670000  ld r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	// 82386800: F9660000  std r11, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82386804: E9670008  ld r11, 8(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) };
	// 82386808: F9660008  std r11, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8238680C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82386810: E9450000  ld r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82386814: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82386818: E9450008  ld r10, 8(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 8238681C: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 82386820: C0040090  lfs f0, 0x90(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(144 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82386824: D0030090  stfs f0, 0x90(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 82386828: C0040094  lfs f0, 0x94(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(148 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238682C: D0030094  stfs f0, 0x94(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82386830: C0040098  lfs f0, 0x98(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(152 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82386834: D0030098  stfs f0, 0x98(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 82386838: C004009C  lfs f0, 0x9c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(156 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238683C: D003009C  stfs f0, 0x9c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 82386840: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82386844: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82386848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82386850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82386850 size=2580
    let mut pc: u32 = 0x82386850;
    'dispatch: loop {
        match pc {
            0x82386850 => {
    //   block [0x82386850..0x82387264)
	// 82386850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82386854: 481AE859  bl 0x825350ac
	ctx.lr = 0x82386858;
	sub_82535080(ctx, base);
	// 82386858: 3981FFC0  addi r12, r1, -0x40
	ctx.r[12].s64 = ctx.r[1].s64 + -64;
	// 8238685C: 481AF775  bl 0x82535fd0
	ctx.lr = 0x82386860;
	sub_82535FB0(ctx, base);
	// 82386860: 3980FF50  li r12, -0xb0
	ctx.r[12].s64 = -176;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82387268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82387268 size=1056
    let mut pc: u32 = 0x82387268;
    'dispatch: loop {
        match pc {
            0x82387268 => {
    //   block [0x82387268..0x82387688)
	// 82387268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238726C: 481ADE31  bl 0x8253509c
	ctx.lr = 0x82387270;
	sub_82535080(ctx, base);
	// 82387270: 3981FFA0  addi r12, r1, -0x60
	ctx.r[12].s64 = ctx.r[1].s64 + -96;
	// 82387274: 481AED75  bl 0x82535fe8
	ctx.lr = 0x82387278;
	sub_82535FB0(ctx, base);
	// 82387278: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238727C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82387280: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82387284: 48000014  b 0x82387298
	pc = 0x82387298; continue 'dispatch;
	// 82387288: 895E0024  lbz r10, 0x24(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 8238728C: 2B0A0005  cmplwi cr6, r10, 5
	ctx.cr[6].compare_u32(ctx.r[10].u32, 5 as u32, &mut ctx.xer);
	// 82387290: 419A0018  beq cr6, 0x823872a8
	if ctx.cr[6].eq {
	pc = 0x823872A8; continue 'dispatch;
	}
	// 82387294: 83CB0004  lwz r30, 4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82387298: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238729C: 93DD0008  stw r30, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 823872A0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 823872A4: 4082FFE4  bne 0x82387288
	if !ctx.cr[0].eq {
	pc = 0x82387288; continue 'dispatch;
	}
	// 823872A8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 823872AC: 3BE50030  addi r31, r5, 0x30
	ctx.r[31].s64 = ctx.r[5].s64 + 48;
	// 823872B0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 823872B4: C3EBBA38  lfs f31, -0x45c8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 823872B8: 419A01F8  beq cr6, 0x823874b0
	if ctx.cr[6].eq {
	pc = 0x823874B0; continue 'dispatch;
	}
	// 823872BC: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 823872C0: 817E0040  lwz r11, 0x40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 823872C4: 39410090  addi r10, r1, 0x90
	ctx.r[10].s64 = ctx.r[1].s64 + 144;
	// 823872C8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 823872CC: 991E001B  stb r8, 0x1b(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(27 as u32), ctx.r[8].u8 ) };
	// 823872D0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 823872D4: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 823872D8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 823872DC: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 823872E0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823872E4: 4200FFF0  bdnz 0x823872d4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x823872D4; continue 'dispatch;
	}
	// 823872E8: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 823872EC: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 823872F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823872F4: 481AD85D  bl 0x82534b50
	ctx.lr = 0x823872F8;
	sub_82534B50(ctx, base);
	// 823872F8: 395E0050  addi r10, r30, 0x50
	ctx.r[10].s64 = ctx.r[30].s64 + 80;
	// 823872FC: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82387300: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82387304: 38FE0080  addi r7, r30, 0x80
	ctx.r[7].s64 = ctx.r[30].s64 + 128;
	// 82387308: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 8238730C: EAAA0000  ld r21, 0(r10)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82387310: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82387314: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 82387318: 389F0080  addi r4, r31, 0x80
	ctx.r[4].s64 = ctx.r[31].s64 + 128;
	// 8238731C: 3AE00060  li r23, 0x60
	ctx.r[23].s64 = 96;
	// 82387320: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82387324: 3B9F0040  addi r28, r31, 0x40
	ctx.r[28].s64 = ctx.r[31].s64 + 64;
	// 82387328: FAAB0000  std r21, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 8238732C: 3B7E0070  addi r27, r30, 0x70
	ctx.r[27].s64 = ctx.r[30].s64 + 112;
	// 82387330: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 82387334: 3B410060  addi r26, r1, 0x60
	ctx.r[26].s64 = ctx.r[1].s64 + 96;
	// 82387338: E9690000  ld r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 8238733C: 3AC00090  li r22, 0x90
	ctx.r[22].s64 = 144;
	// 82387340: E9490008  ld r10, 8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	// 82387344: 3B210060  addi r25, r1, 0x60
	ctx.r[25].s64 = ctx.r[1].s64 + 96;
	// 82387348: 3B1D02F0  addi r24, r29, 0x2f0
	ctx.r[24].s64 = ctx.r[29].s64 + 752;
	// 8238734C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82387350: F97F0090  std r11, 0x90(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u64 ) };
	// 82387354: F95F0098  std r10, 0x98(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[10].u64 ) };
	// 82387358: E9670000  ld r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	// 8238735C: E9470008  ld r10, 8(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) };
	// 82387360: F9680000  std r11, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82387364: F9480008  std r10, 8(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 82387368: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 8238736C: E9460008  ld r10, 8(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82387688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82387688 size=900
    let mut pc: u32 = 0x82387688;
    'dispatch: loop {
        match pc {
            0x82387688 => {
    //   block [0x82387688..0x82387A0C)
	// 82387688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238768C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82387690: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82387694: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82387698: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8238769C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823876A0: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 823876A4: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 823876A8: 3BC50120  addi r30, r5, 0x120
	ctx.r[30].s64 = ctx.r[5].s64 + 288;
	// 823876AC: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 823876B0: C00A1FF8  lfs f0, 0x1ff8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823876B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823876B8: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 823876BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823876C0: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 823876C4: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 823876C8: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 823876CC: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 823876D0: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 823876D4: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 823876D8: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 823876DC: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 823876E0: 418200DC  beq 0x823877bc
	if ctx.cr[0].eq {
	pc = 0x823877BC; continue 'dispatch;
	}
	// 823876E4: 890B0024  lbz r8, 0x24(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 823876E8: 2B080007  cmplwi cr6, r8, 7
	ctx.cr[6].compare_u32(ctx.r[8].u32, 7 as u32, &mut ctx.xer);
	// 823876EC: 409A00B0  bne cr6, 0x8238779c
	if !ctx.cr[6].eq {
	pc = 0x8238779C; continue 'dispatch;
	}
	// 823876F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 823876F4: C00B00A0  lfs f0, 0xa0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823876F8: 38E00050  li r7, 0x50
	ctx.r[7].s64 = 80;
	// 823876FC: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 82387700: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82387704: 990B001B  stb r8, 0x1b(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27 as u32), ctx.r[8].u8 ) };
	// 82387708: C1A10060  lfs f13, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238770C: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82387710: D1A10060  stfs f13, 0x60(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82387714: C1A10064  lfs f13, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82387718: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238771C: D1A10064  stfs f13, 0x64(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82387720: C1A10068  lfs f13, 0x68(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82387724: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82387728: D1A10068  stfs f13, 0x68(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 8238772C: C1A1006C  lfs f13, 0x6c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82387730: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82387734: D1A1006C  stfs f13, 0x6c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82387738: C1A10070  lfs f13, 0x70(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238773C: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82387740: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82387744: D1A10070  stfs f13, 0x70(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82387748: C1A10074  lfs f13, 0x74(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238774C: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82387750: D1A10074  stfs f13, 0x74(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82387754: C1A10078  lfs f13, 0x78(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82387758: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238775C: D1A10078  stfs f13, 0x78(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82387760: C1A1007C  lfs f13, 0x7c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82387764: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82387768: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82387A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82387A10 size=432
    let mut pc: u32 = 0x82387A10;
    'dispatch: loop {
        match pc {
            0x82387A10 => {
    //   block [0x82387A10..0x82387BC0)
	// 82387A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82387A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82387A18: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82387A1C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82387A20: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82387A24: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82387A28: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82387A2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82387A30: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82387A34: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82387A38: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82387A3C: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82387A40: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82387A44: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82387A48: 41820150  beq 0x82387b98
	if ctx.cr[0].eq {
	pc = 0x82387B98; continue 'dispatch;
	}
	// 82387A4C: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82387BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82387BC0 size=360
    let mut pc: u32 = 0x82387BC0;
    'dispatch: loop {
        match pc {
            0x82387BC0 => {
    //   block [0x82387BC0..0x82387D28)
	// 82387BC0: 81430078  lwz r10, 0x78(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) } as u64;
	// 82387BC4: 81630068  lwz r11, 0x68(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) } as u64;
	// 82387BC8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82387BCC: 41820048  beq 0x82387c14
	if ctx.cr[0].eq {
	pc = 0x82387C14; continue 'dispatch;
	}
	// 82387BD0: 814A001C  lwz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82387BD4: 8123007C  lwz r9, 0x7c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) } as u64;
	// 82387BD8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82387BDC: 409A0038  bne cr6, 0x82387c14
	if !ctx.cr[6].eq {
	pc = 0x82387C14; continue 'dispatch;
	}
	// 82387BE0: 81430078  lwz r10, 0x78(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) } as u64;
	// 82387BE4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82387BE8: 4182002C  beq 0x82387c14
	if ctx.cr[0].eq {
	pc = 0x82387C14; continue 'dispatch;
	}
	// 82387BEC: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82387BF0: 8103007C  lwz r8, 0x7c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) } as u64;
	// 82387BF4: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82387BF8: 409A001C  bne cr6, 0x82387c14
	if !ctx.cr[6].eq {
	pc = 0x82387C14; continue 'dispatch;
	}
	// 82387BFC: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82387C00: 912A0018  stw r9, 0x18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 82387C04: 552A003E  slwi r10, r9, 0
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82387C08: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82387C0C: 41820008  beq 0x82387c14
	if ctx.cr[0].eq {
	pc = 0x82387C14; continue 'dispatch;
	}
	// 82387C10: 396A0080  addi r11, r10, 0x80
	ctx.r[11].s64 = ctx.r[10].s64 + 128;
	// 82387C14: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82387D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82387D28 size=16
    let mut pc: u32 = 0x82387D28;
    'dispatch: loop {
        match pc {
            0x82387D28 => {
    //   block [0x82387D28..0x82387D38)
	// 82387D28: 3D608286  lis r11, -0x7d7a
	ctx.r[11].s64 = -2105147392;
	// 82387D2C: C00BD468  lfs f0, -0x2b98(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11160 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82387D30: D00501C0  stfs f0, 0x1c0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(448 as u32), tmp.u32 ) };
	// 82387D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82387D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82387D38 size=944
    let mut pc: u32 = 0x82387D38;
    'dispatch: loop {
        match pc {
            0x82387D38 => {
    //   block [0x82387D38..0x823880E8)
	// 82387D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82387D3C: 481AD365  bl 0x825350a0
	ctx.lr = 0x82387D40;
	sub_82535080(ctx, base);
	// 82387D40: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82387D44: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82387D48: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82387D4C: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 82387D50: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82387D54: 419A00B0  beq cr6, 0x82387e04
	if ctx.cr[6].eq {
	pc = 0x82387E04; continue 'dispatch;
	}
	// 82387D58: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 82387D5C: 419A00A8  beq cr6, 0x82387e04
	if ctx.cr[6].eq {
	pc = 0x82387E04; continue 'dispatch;
	}
	// 82387D60: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82387D64: 54B6043F  clrlwi. r22, r5, 0x10
	ctx.r[22].u64 = ctx.r[5].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82387D68: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82387D6C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82387D70: 61799F62  ori r25, r11, 0x9f62
	ctx.r[25].u64 = ctx.r[11].u64 | 40802;
	// 82387D74: 41820238  beq 0x82387fac
	if ctx.cr[0].eq {
	pc = 0x82387FAC; continue 'dispatch;
	}
	// 82387D78: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82387D7C: 617A9F60  ori r26, r11, 0x9f60
	ctx.r[26].u64 = ctx.r[11].u64 | 40800;
	// 82387D80: 889F004B  lbz r4, 0x4b(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(75 as u32) ) } as u64;
	// 82387D84: 2B040004  cmplwi cr6, r4, 4
	ctx.cr[6].compare_u32(ctx.r[4].u32, 4 as u32, &mut ctx.xer);
	// 82387D88: 409800F0  bge cr6, 0x82387e78
	if !ctx.cr[6].lt {
	pc = 0x82387E78; continue 'dispatch;
	}
	// 82387D8C: 7CBBD214  add r5, r27, r26
	ctx.r[5].u64 = ctx.r[27].u64 + ctx.r[26].u64;
	// 82387D90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82387D94: A1650000  lhz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82387D98: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82387D9C: 41820068  beq 0x82387e04
	if ctx.cr[0].eq {
	pc = 0x82387E04; continue 'dispatch;
	}
	// 82387DA0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82387DA4: 1D660050  mulli r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 * 80;
	// 82387DA8: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82387DAC: 894B004B  lbz r10, 0x4b(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(75 as u32) ) } as u64;
	// 82387DB0: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82387DB4: 409A0038  bne cr6, 0x82387dec
	if !ctx.cr[6].eq {
	pc = 0x82387DEC; continue 'dispatch;
	}
	// 82387DB8: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82387DBC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82387DC0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82387DC4: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82387DC8: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82387DCC: 7D084850  subf r8, r8, r9
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82387DD0: 41820014  beq 0x82387de4
	if ctx.cr[0].eq {
	pc = 0x82387DE4; continue 'dispatch;
	}
	// 82387DD4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82387DD8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82387DDC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82387DE0: 419AFFE0  beq cr6, 0x82387dc0
	if ctx.cr[6].eq {
	pc = 0x82387DC0; continue 'dispatch;
	}
	// 82387DE4: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82387DE8: 41820028  beq 0x82387e10
	if ctx.cr[0].eq {
	pc = 0x82387E10; continue 'dispatch;
	}
	// 82387DEC: 39660001  addi r11, r6, 1
	ctx.r[11].s64 = ctx.r[6].s64 + 1;
	// 82387DF0: A1450000  lhz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82387DF4: 5567043E  clrlwi r7, r11, 0x10
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82387DF8: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82387DFC: 7F065040  cmplw cr6, r6, r10
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82387E00: 4198FFA4  blt cr6, 0x82387da4
	if ctx.cr[6].lt {
	pc = 0x82387DA4; continue 'dispatch;
	}
	// 82387E04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82387E08: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82387E0C: 481AD2E4  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
	// 82387E10: 54EB043E  clrlwi r11, r7, 0x10
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x0000FFFFu64;
	// 82387E14: E95F0000  ld r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 82387E18: 57890BFC  rlwinm r9, r28, 1, 0xf, 0x1e
	ctx.r[9].u64 = ctx.r[28].u32 as u64 & 0x7FFFFFFFu64;
	// 82387E1C: 1D6B0050  mulli r11, r11, 0x50
	ctx.r[11].s64 = ctx.r[11].s64 * 80;
	// 82387E20: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82387E24: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82387E28: E95F0008  ld r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 82387E2C: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 82387E30: E95F0010  ld r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 82387E34: F94B0010  std r10, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82387E38: E95F0018  ld r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	// 82387E3C: F94B0018  std r10, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u64 ) };
	// 82387E40: C01F0020  lfs f0, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82387E44: D00B0020  stfs f0, 0x20(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82387E48: C01F0024  lfs f0, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82387E4C: D00B0024  stfs f0, 0x24(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82387E50: C01F0028  lfs f0, 0x28(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82387E54: D00B0028  stfs f0, 0x28(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82387E58: C01F0030  lfs f0, 0x30(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82387E5C: D00B0030  stfs f0, 0x30(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82387E60: C01F0034  lfs f0, 0x34(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82387E64: D00B0034  stfs f0, 0x34(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82387E68: C01F0038  lfs f0, 0x38(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82387E6C: D00B0038  stfs f0, 0x38(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82387E70: 7CE9BB2E  sthx r7, r9, r23
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[23].u32), ctx.r[7].u16) };
	// 82387E74: 4800011C  b 0x82387f90
	pc = 0x82387F90; continue 'dispatch;
	// 82387E78: 2B04000E  cmplwi cr6, r4, 0xe
	ctx.cr[6].compare_u32(ctx.r[4].u32, 14 as u32, &mut ctx.xer);
	// 82387E7C: 409A0010  bne cr6, 0x82387e8c
	if !ctx.cr[6].eq {
	pc = 0x82387E8C; continue 'dispatch;
	}
	// 82387E80: 578B0BFC  rlwinm r11, r28, 1, 0xf, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x7FFFFFFFu64;
	// 82387E84: 7F8BBB2E  sthx r28, r11, r23
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32), ctx.r[28].u16) };
	// 82387E88: 48000108  b 0x82387f90
	pc = 0x82387F90; continue 'dispatch;
	// 82387E8C: 7CFBD22E  lhzx r7, r27, r26
	ctx.r[7].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82387E90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82387E94: 7FDBCA14  add r30, r27, r25
	ctx.r[30].u64 = ctx.r[27].u64 + ctx.r[25].u64;
	// 82387E98: 48000054  b 0x82387eec
	pc = 0x82387EEC; continue 'dispatch;
	// 82387E9C: 1D4B0050  mulli r10, r11, 0x50
	ctx.r[10].s64 = ctx.r[11].s64 * 80;
	// 82387EA0: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 82387EA4: 892A004B  lbz r9, 0x4b(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(75 as u32) ) } as u64;
	// 82387EA8: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82387EAC: 409A0038  bne cr6, 0x82387ee4
	if !ctx.cr[6].eq {
	pc = 0x82387EE4; continue 'dispatch;
	}
	// 82387EB0: 813F002C  lwz r9, 0x2c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82387EB4: 814A002C  lwz r10, 0x2c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82387EB8: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82387EBC: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82387EC0: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82387EC4: 7CC64050  subf r6, r6, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[6].s64;
	// 82387EC8: 41820014  beq 0x82387edc
	if ctx.cr[0].eq {
	pc = 0x82387EDC; continue 'dispatch;
	}
	// 82387ECC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82387ED0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82387ED4: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82387ED8: 419AFFE0  beq cr6, 0x82387eb8
	if ctx.cr[6].eq {
	pc = 0x82387EB8; continue 'dispatch;
	}
	// 82387EDC: 2C060000  cmpwi r6, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82387EE0: 41820020  beq 0x82387f00
	if ctx.cr[0].eq {
	pc = 0x82387F00; continue 'dispatch;
	}
	// 82387EE4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82387EE8: 5567043E  clrlwi r7, r11, 0x10
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82387EEC: A15E0000  lhz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82387EF0: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82387EF4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82387EF8: 4198FFA4  blt cr6, 0x82387e9c
	if ctx.cr[6].lt {
	pc = 0x82387E9C; continue 'dispatch;
	}
	// 82387EFC: 48000010  b 0x82387f0c
	pc = 0x82387F0C; continue 'dispatch;
	// 82387F00: 578B0BFC  rlwinm r11, r28, 1, 0xf, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x7FFFFFFFu64;
	// 82387F04: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82387F08: 7CEBBB2E  sthx r7, r11, r23
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32), ctx.r[7].u16) };
	// 82387F0C: 54AB043F  clrlwi. r11, r5, 0x10
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82387F10: 40820080  bne 0x82387f90
	if !ctx.cr[0].eq {
	pc = 0x82387F90; continue 'dispatch;
	}
	// 82387F14: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82387F18: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82387F1C: 2F0A01FE  cmpwi cr6, r10, 0x1fe
	ctx.cr[6].compare_i32(ctx.r[10].s32, 510, &mut ctx.xer);
	// 82387F20: 4098FEE4  bge cr6, 0x82387e04
	if !ctx.cr[6].lt {
	pc = 0x82387E04; continue 'dispatch;
	}
	// 82387F24: 1D6B0050  mulli r11, r11, 0x50
	ctx.r[11].s64 = ctx.r[11].s64 * 80;
	// 82387F28: 7FABDA14  add r29, r11, r27
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82387F2C: 38A00050  li r5, 0x50
	ctx.r[5].s64 = 80;
	// 82387F30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82387F34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82387F38: 481ACC19  bl 0x82534b50
	ctx.lr = 0x82387F3C;
	sub_82534B50(ctx, base);
	// 82387F3C: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82387F40: B17D004E  sth r11, 0x4e(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(78 as u32), ctx.r[11].u16 ) };
	// 82387F44: A17F004C  lhz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82387F48: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 82387F4C: 409A0010  bne cr6, 0x82387f5c
	if !ctx.cr[6].eq {
	pc = 0x82387F5C; continue 'dispatch;
	}
	// 82387F50: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82387F54: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82387F58: 4800000C  b 0x82387f64
	pc = 0x82387F64; continue 'dispatch;
	// 82387F5C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82387F60: 7D6BBA2E  lhzx r11, r11, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82387F64: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82387F68: 570B043E  clrlwi r11, r24, 0x10
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0x0000FFFFu64;
	// 82387F6C: 57890BFC  rlwinm r9, r28, 1, 0xf, 0x1e
	ctx.r[9].u64 = ctx.r[28].u32 as u64 & 0x7FFFFFFFu64;
	// 82387F70: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82387F74: B15D004C  sth r10, 0x4c(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(76 as u32), ctx.r[10].u16 ) };
	// 82387F78: 5578043E  clrlwi r24, r11, 0x10
	ctx.r[24].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82387F7C: A15E0000  lhz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82387F80: 7D49BB2E  sthx r10, r9, r23
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[23].u32), ctx.r[10].u16) };
	// 82387F84: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82387F88: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82387F8C: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82387F90: 578B043E  clrlwi r11, r28, 0x10
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x0000FFFFu64;
	// 82387F94: 3BFF0050  addi r31, r31, 0x50
	ctx.r[31].s64 = ctx.r[31].s64 + 80;
	// 82387F98: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82387F9C: 557C043E  clrlwi r28, r11, 0x10
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82387FA0: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82387FA4: 7F0BB040  cmplw cr6, r11, r22
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[22].u32, &mut ctx.xer);
	// 82387FA8: 4198FDD8  blt cr6, 0x82387d80
	if ctx.cr[6].lt {
	pc = 0x82387D80; continue 'dispatch;
	}
	// 82387FAC: 7CFBCA14  add r7, r27, r25
	ctx.r[7].u64 = ctx.r[27].u64 + ctx.r[25].u64;
	// 82387FB0: 570A043E  clrlwi r10, r24, 0x10
	ctx.r[10].u64 = ctx.r[24].u32 as u64 & 0x0000FFFFu64;
	// 82387FB4: A1670000  lhz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82387FB8: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82387FBC: 5548043E  clrlwi r8, r10, 0x10
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82387FC0: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82387FC4: 4098011C  bge cr6, 0x823880e0
	if !ctx.cr[6].lt {
	pc = 0x823880E0; continue 'dispatch;
	}
	// 82387FC8: 1D680050  mulli r11, r8, 0x50
	ctx.r[11].s64 = ctx.r[8].s64 * 80;
	// 82387FCC: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82387FD0: 894B004B  lbz r10, 0x4b(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(75 as u32) ) } as u64;
	// 82387FD4: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 82387FD8: 2B0A0013  cmplwi cr6, r10, 0x13
	ctx.cr[6].compare_u32(ctx.r[10].u32, 19 as u32, &mut ctx.xer);
	// 82387FDC: 4199FE28  bgt cr6, 0x82387e04
	if ctx.cr[6].gt {
	pc = 0x82387E04; continue 'dispatch;
	}
	// 82387FE0: 3D80820C  lis r12, -0x7df4
	ctx.r[12].s64 = -2113142784;
	// 82387FE4: 398CDAA8  addi r12, r12, -0x2558
	ctx.r[12].s64 = ctx.r[12].s64 + -9560;
	// 82387FE8: 7C0C50AE  lbzx r0, r12, r10
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82387FEC: 5400103A  slwi r0, r0, 2
	ctx.r[0].u32 = ctx.r[0].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82387FF0: 3D808238  lis r12, -0x7dc8
	ctx.r[12].s64 = -2110259200;
	// 82387FF4: 398C7E04  addi r12, r12, 0x7e04
	ctx.r[12].s64 = ctx.r[12].s64 + 32260;
	// 82387FF8: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82387FFC: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82388000: 60000000  nop
	// 82388004: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 82388008: A14B0030  lhz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8238800C: 554A083E  rotlwi r10, r10, 1
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 82388010: 7D4ABA2E  lhzx r10, r10, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82388014: B14B0030  sth r10, 0x30(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u16 ) };
	// 82388018: 480000B4  b 0x823880cc
	pc = 0x823880CC; continue 'dispatch;
	// 8238801C: A14B0036  lhz r10, 0x36(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(54 as u32) ) } as u64;
	// 82388020: 554A083E  rotlwi r10, r10, 1
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 82388024: 7D4ABA2E  lhzx r10, r10, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82388028: B14B0036  sth r10, 0x36(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(54 as u32), ctx.r[10].u16 ) };
	// 8238802C: 480000A0  b 0x823880cc
	pc = 0x823880CC; continue 'dispatch;
	// 82388030: A14B004C  lhz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82388034: 48000094  b 0x823880c8
	pc = 0x823880C8; continue 'dispatch;
	// 82388038: 894B0041  lbz r10, 0x41(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(65 as u32) ) } as u64;
	// 8238803C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82388040: 41820030  beq 0x82388070
	if ctx.cr[0].eq {
	pc = 0x82388070; continue 'dispatch;
	}
	// 82388044: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82388048: 5549083C  slwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8238804C: 38CA0001  addi r6, r10, 1
	ctx.r[6].s64 = ctx.r[10].s64 + 1;
	// 82388050: 54CA063E  clrlwi r10, r6, 0x18
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82388054: 7CC95A2E  lhzx r6, r9, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82388058: 54C6083E  rotlwi r6, r6, 1
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(1)) as u64;
	// 8238805C: 7CC6BA2E  lhzx r6, r6, r23
	ctx.r[6].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82388060: 7CC95B2E  sthx r6, r9, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[6].u16) };
	// 82388064: 892B0041  lbz r9, 0x41(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(65 as u32) ) } as u64;
	// 82388068: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8238806C: 4198FFDC  blt cr6, 0x82388048
	if ctx.cr[6].lt {
	pc = 0x82388048; continue 'dispatch;
	}
	// 82388070: 894B0042  lbz r10, 0x42(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(66 as u32) ) } as u64;
	// 82388074: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82388078: 41820054  beq 0x823880cc
	if ctx.cr[0].eq {
	pc = 0x823880CC; continue 'dispatch;
	}
	// 8238807C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82388080: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 82388084: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 82388088: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8238808C: 54C9083C  slwi r9, r6, 1
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82388090: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82388094: 7CC95A2E  lhzx r6, r9, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82388098: 54C6083E  rotlwi r6, r6, 1
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(1)) as u64;
	// 8238809C: 7CC6BA2E  lhzx r6, r6, r23
	ctx.r[6].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 823880A0: 7CC95B2E  sthx r6, r9, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[6].u16) };
	// 823880A4: 892B0042  lbz r9, 0x42(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(66 as u32) ) } as u64;
	// 823880A8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 823880AC: 4198FFD4  blt cr6, 0x82388080
	if ctx.cr[6].lt {
	pc = 0x82388080; continue 'dispatch;
	}
	// 823880B0: 4800001C  b 0x823880cc
	pc = 0x823880CC; continue 'dispatch;
	// 823880B4: A14B0034  lhz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 823880B8: 2B0AFFFF  cmplwi cr6, r10, 0xffff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 65535 as u32, &mut ctx.xer);
	// 823880BC: 419A0010  beq cr6, 0x823880cc
	if ctx.cr[6].eq {
	pc = 0x823880CC; continue 'dispatch;
	}
	// 823880C0: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823880C4: 7D4ABA2E  lhzx r10, r10, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 823880C8: B14B0034  sth r10, 0x34(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[10].u16 ) };
	// 823880CC: 39680001  addi r11, r8, 1
	ctx.r[11].s64 = ctx.r[8].s64 + 1;
	// 823880D0: A1470000  lhz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 823880D4: 5568043E  clrlwi r8, r11, 0x10
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 823880D8: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823880DC: 4198FEEC  blt cr6, 0x82387fc8
	if ctx.cr[6].lt {
	pc = 0x82387FC8; continue 'dispatch;
	}
	// 823880E0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 823880E4: 4BFFFD24  b 0x82387e08
	pc = 0x82387E08; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823880E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823880E8 size=144
    let mut pc: u32 = 0x823880E8;
    'dispatch: loop {
        match pc {
            0x823880E8 => {
    //   block [0x823880E8..0x82388178)
	// 823880E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823880EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823880F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823880F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823880F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823880FC: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82388100: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82388104: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82388108: 915F0088  stw r10, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 8238810C: 41980058  blt cr6, 0x82388164
	if ctx.cr[6].lt {
	pc = 0x82388164; continue 'dispatch;
	}
	// 82388110: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82388114: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 82388118: 396B86B8  addi r11, r11, -0x7948
	ctx.r[11].s64 = ctx.r[11].s64 + -31048;
	// 8238811C: 1D430018  mulli r10, r3, 0x18
	ctx.r[10].s64 = ctx.r[3].s64 * 24;
	// 82388120: 7D2A58AE  lbzx r9, r10, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82388124: 2B090002  cmplwi cr6, r9, 2
	ctx.cr[6].compare_u32(ctx.r[9].u32, 2 as u32, &mut ctx.xer);
	// 82388128: 409A001C  bne cr6, 0x82388144
	if !ctx.cr[6].eq {
	pc = 0x82388144; continue 'dispatch;
	}
	// 8238812C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82388130: 813F0084  lwz r9, 0x84(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82388134: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82388138: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8238813C: 409A0008  bne cr6, 0x82388144
	if !ctx.cr[6].eq {
	pc = 0x82388144; continue 'dispatch;
	}
	// 82388140: 4BFF24D1  bl 0x8237a610
	ctx.lr = 0x82388144;
	sub_8237A610(ctx, base);
	// 82388144: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82388148: 816BFAC0  lwz r11, -0x540(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 8238814C: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82388150: 409A0014  bne cr6, 0x82388164
	if !ctx.cr[6].eq {
	pc = 0x82388164; continue 'dispatch;
	}
	// 82388154: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82388158: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8238815C: 997F0019  stb r11, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 82388160: 995F0025  stb r10, 0x25(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(37 as u32), ctx.r[10].u8 ) };
	// 82388164: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82388168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238816C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82388170: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82388174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82388178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82388178 size=124
    let mut pc: u32 = 0x82388178;
    'dispatch: loop {
        match pc {
            0x82388178 => {
    //   block [0x82388178..0x823881F4)
	// 82388178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238817C: 481ACF39  bl 0x825350b4
	ctx.lr = 0x82388180;
	sub_82535080(ctx, base);
	// 82388180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82388184: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 82388188: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8238818C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82388190: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82388194: 816BBFAC  lwz r11, -0x4054(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16468 as u32) ) } as u64;
	// 82388198: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8238819C: 409A0050  bne cr6, 0x823881ec
	if !ctx.cr[6].eq {
	pc = 0x823881EC; continue 'dispatch;
	}
	// 823881A0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823881A4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 823881A8: 419A003C  beq cr6, 0x823881e4
	if ctx.cr[6].eq {
	pc = 0x823881E4; continue 'dispatch;
	}
	// 823881AC: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 823881B0: 3BCB9580  addi r30, r11, -0x6a80
	ctx.r[30].s64 = ctx.r[11].s64 + -27264;
	// 823881B4: 7C9FEA14  add r4, r31, r29
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 823881B8: 387E0080  addi r3, r30, 0x80
	ctx.r[3].s64 = ctx.r[30].s64 + 128;
	// 823881BC: 48019985  bl 0x823a1b40
	ctx.lr = 0x823881C0;
	sub_823A1B40(ctx, base);
	// 823881C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823881C4: 41820014  beq 0x823881d8
	if ctx.cr[0].eq {
	pc = 0x823881D8; continue 'dispatch;
	}
	// 823881C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823881CC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 823881D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823881D4: 4E800421  bctrl
	ctx.lr = 0x823881D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823881D8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 823881DC: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 823881E0: 4198FFD4  blt cr6, 0x823881b4
	if ctx.cr[6].lt {
	pc = 0x823881B4; continue 'dispatch;
	}
	// 823881E4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 823881E8: 480126C1  bl 0x8239a8a8
	ctx.lr = 0x823881EC;
	sub_8239A8A8(ctx, base);
	// 823881EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823881F0: 481ACF14  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823881F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823881F8 size=100
    let mut pc: u32 = 0x823881F8;
    'dispatch: loop {
        match pc {
            0x823881F8 => {
    //   block [0x823881F8..0x8238825C)
	// 823881F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823881FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82388200: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82388204: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82388208: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238820C: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82388210: 556B0463  rlwinm. r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82388214: 41820034  beq 0x82388248
	if ctx.cr[0].eq {
	pc = 0x82388248; continue 'dispatch;
	}
	// 82388218: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238821C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82388220: 41820014  beq 0x82388234
	if ctx.cr[0].eq {
	pc = 0x82388234; continue 'dispatch;
	}
	// 82388224: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82388228: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238822C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82388230: 4E800421  bctrl
	ctx.lr = 0x82388234;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82388234: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82388238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8238823C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82388240: 716BBFFF  andi. r11, r11, 0xbfff
	ctx.r[11].u64 = ctx.r[11].u64 & 49151;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82388244: B17F0008  sth r11, 8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u16 ) };
	// 82388248: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8238824C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82388250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82388254: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82388258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82388260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82388260 size=132
    let mut pc: u32 = 0x82388260;
    'dispatch: loop {
        match pc {
            0x82388260 => {
    //   block [0x82388260..0x823882E4)
	// 82388260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82388264: 481ACE59  bl 0x825350bc
	ctx.lr = 0x82388268;
	sub_82535080(ctx, base);
	// 82388268: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238826C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82388270: 397D0020  addi r11, r29, 0x20
	ctx.r[11].s64 = ctx.r[29].s64 + 32;
	// 82388274: A15D000A  lhz r10, 0xa(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(10 as u32) ) } as u64;
	// 82388278: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238827C: 41820060  beq 0x823882dc
	if ctx.cr[0].eq {
	pc = 0x823882DC; continue 'dispatch;
	}
	// 82388280: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82388284: 3BEB000A  addi r31, r11, 0xa
	ctx.r[31].s64 = ctx.r[11].s64 + 10;
	// 82388288: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238828C: 556B0463  rlwinm. r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82388290: 41820034  beq 0x823882c4
	if ctx.cr[0].eq {
	pc = 0x823882C4; continue 'dispatch;
	}
	// 82388294: 807FFFFA  lwz r3, -6(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-6 as u32) ) } as u64;
	// 82388298: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238829C: 41820014  beq 0x823882b0
	if ctx.cr[0].eq {
	pc = 0x823882B0; continue 'dispatch;
	}
	// 823882A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823882A4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 823882A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823882AC: 4E800421  bctrl
	ctx.lr = 0x823882B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823882B0: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823882B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823882B8: 915FFFFA  stw r10, -6(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-6 as u32), ctx.r[10].u32 ) };
	// 823882BC: 716BBFFF  andi. r11, r11, 0xbfff
	ctx.r[11].u64 = ctx.r[11].u64 & 49151;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823882C0: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 823882C4: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 823882C8: A15D000A  lhz r10, 0xa(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(10 as u32) ) } as u64;
	// 823882CC: 3BFF0018  addi r31, r31, 0x18
	ctx.r[31].s64 = ctx.r[31].s64 + 24;
	// 823882D0: 557E043E  clrlwi r30, r11, 0x10
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 823882D4: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823882D8: 4198FFB0  blt cr6, 0x82388288
	if ctx.cr[6].lt {
	pc = 0x82388288; continue 'dispatch;
	}
	// 823882DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823882E0: 481ACE2C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823882E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823882E8 size=596
    let mut pc: u32 = 0x823882E8;
    'dispatch: loop {
        match pc {
            0x823882E8 => {
    //   block [0x823882E8..0x8238853C)
	// 823882E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823882EC: 481ACDC1  bl 0x825350ac
	ctx.lr = 0x823882F0;
	sub_82535080(ctx, base);
	// 823882F0: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 823882F4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823882F8: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 823882FC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82388300: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82388304: 3BAB2200  addi r29, r11, 0x2200
	ctx.r[29].s64 = ctx.r[11].s64 + 8704;
	// 82388308: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8238830C: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 82388310: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82388314: 48384F49  bl 0x8270d25c
	ctx.lr = 0x82388318;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82388318: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8238831C: 419A00A8  beq cr6, 0x823883c4
	if ctx.cr[6].eq {
	pc = 0x823883C4; continue 'dispatch;
	}
	// 82388320: 3960FFBE  li r11, -0x42
	ctx.r[11].s64 = -66;
	// 82388324: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82388328: 4098009C  bge cr6, 0x823883c4
	if !ctx.cr[6].lt {
	pc = 0x823883C4; continue 'dispatch;
	}
	// 8238832C: 397C003F  addi r11, r28, 0x3f
	ctx.r[11].s64 = ctx.r[28].s64 + 63;
	// 82388330: 387D0024  addi r3, r29, 0x24
	ctx.r[3].s64 = ctx.r[29].s64 + 36;
	// 82388334: 557E0032  rlwinm r30, r11, 0, 0, 0x19
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82388338: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8238833C: 48022E35  bl 0x823ab170
	ctx.lr = 0x82388340;
	sub_823AB170(ctx, base);
	// 82388340: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82388344: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82388348: 41820080  beq 0x823883c8
	if ctx.cr[0].eq {
	pc = 0x823883C8; continue 'dispatch;
	}
	// 8238834C: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82388350: 7D7E4850  subf r11, r30, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[30].s64;
	// 82388354: 2B0B0080  cmplwi cr6, r11, 0x80
	ctx.cr[6].compare_u32(ctx.r[11].u32, 128 as u32, &mut ctx.xer);
	// 82388358: 41980054  blt cr6, 0x823883ac
	if ctx.cr[6].lt {
	pc = 0x823883AC; continue 'dispatch;
	}
	// 8238835C: 396BFFC0  addi r11, r11, -0x40
	ctx.r[11].s64 = ctx.r[11].s64 + -64;
	// 82388360: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82388364: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82388368: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238836C: 7C89FA14  add r4, r9, r31
	ctx.r[4].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 82388370: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82388374: 41820008  beq 0x8238837c
	if ctx.cr[0].eq {
	pc = 0x8238837C; continue 'dispatch;
	}
	// 82388378: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 8238837C: 91640010  stw r11, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82388380: 387D0024  addi r3, r29, 0x24
	ctx.r[3].s64 = ctx.r[29].s64 + 36;
	// 82388384: 93E40004  stw r31, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82388388: 93640008  stw r27, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 8238838C: 9364000C  stw r27, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 82388390: 93640014  stw r27, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 82388394: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82388398: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238839C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 823883A0: 396BFFC0  addi r11, r11, -0x40
	ctx.r[11].s64 = ctx.r[11].s64 + -64;
	// 823883A4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 823883A8: 48022C41  bl 0x823aafe8
	ctx.lr = 0x823883AC;
	sub_823AAFE8(ctx, base);
	// 823883AC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 823883B0: 3BDF0040  addi r30, r31, 0x40
	ctx.r[30].s64 = ctx.r[31].s64 + 64;
	// 823883B4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 823883B8: 937F000C  stw r27, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 823883BC: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 823883C0: 4800000C  b 0x823883cc
	pc = 0x823883CC; continue 'dispatch;
	// 823883C4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 823883C8: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 823883CC: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 823883D0: 48384E9D  bl 0x8270d26c
	ctx.lr = 0x823883D4;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 823883D4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 823883D8: 409A000C  bne cr6, 0x823883e4
	if !ctx.cr[6].eq {
	pc = 0x823883E4; continue 'dispatch;
	}
	// 823883DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823883E0: 48000150  b 0x82388530
	pc = 0x82388530; continue 'dispatch;
	// 823883E4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 823883E8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 823883EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823883F0: 481AC761  bl 0x82534b50
	ctx.lr = 0x823883F4;
	sub_82534B50(ctx, base);
	// 823883F4: 897E0006  lbz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 823883F8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 823883FC: 419A000C  beq cr6, 0x82388408
	if ctx.cr[6].eq {
	pc = 0x82388408; continue 'dispatch;
	}
	// 82388400: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82388404: 4800000C  b 0x82388410
	pc = 0x82388410; continue 'dispatch;
	// 82388408: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8238840C: 4800F66D  bl 0x82397a78
	ctx.lr = 0x82388410;
	sub_82397A78(ctx, base);
	// 82388410: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82388414: 40820010  bne 0x82388424
	if !ctx.cr[0].eq {
	pc = 0x82388424; continue 'dispatch;
	}
	// 82388418: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8238841C: 48012C85  bl 0x8239b0a0
	ctx.lr = 0x82388420;
	sub_8239B0A0(ctx, base);
	// 82388420: 4BFFFFBC  b 0x823883dc
	pc = 0x823883DC; continue 'dispatch;
	// 82388424: 897E0007  lbz r11, 7(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(7 as u32) ) } as u64;
	// 82388428: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 8238842C: 40980014  bge cr6, 0x82388440
	if !ctx.cr[6].lt {
	pc = 0x82388440; continue 'dispatch;
	}
	// 82388430: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388434: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82388438: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238843C: 48000028  b 0x82388464
	pc = 0x82388464; continue 'dispatch;
	// 82388440: 897E0009  lbz r11, 9(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(9 as u32) ) } as u64;
	// 82388444: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82388448: 4098000C  bge cr6, 0x82388454
	if !ctx.cr[6].lt {
	pc = 0x82388454; continue 'dispatch;
	}
	// 8238844C: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82388450: 48000014  b 0x82388464
	pc = 0x82388464; continue 'dispatch;
	// 82388454: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388458: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8238845C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82388460: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82388464: 897A0007  lbz r11, 7(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(7 as u32) ) } as u64;
	// 82388468: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 8238846C: 40980014  bge cr6, 0x82388480
	if !ctx.cr[6].lt {
	pc = 0x82388480; continue 'dispatch;
	}
	// 82388470: 897A0004  lbz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388474: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82388478: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238847C: 48000028  b 0x823884a4
	pc = 0x823884A4; continue 'dispatch;
	// 82388480: 897A0009  lbz r11, 9(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(9 as u32) ) } as u64;
	// 82388484: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82388488: 4098000C  bge cr6, 0x82388494
	if !ctx.cr[6].lt {
	pc = 0x82388494; continue 'dispatch;
	}
	// 8238848C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82388490: 48000014  b 0x823884a4
	pc = 0x823884A4; continue 'dispatch;
	// 82388494: 897A0004  lbz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388498: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 8238849C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 823884A0: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823884A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823884A8: 419A0084  beq cr6, 0x8238852c
	if ctx.cr[6].eq {
	pc = 0x8238852C; continue 'dispatch;
	}
	// 823884AC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 823884B0: 419A007C  beq cr6, 0x8238852c
	if ctx.cr[6].eq {
	pc = 0x8238852C; continue 'dispatch;
	}
	// 823884B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823884B8: A09E000A  lhz r4, 0xa(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 823884BC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 823884C0: 48010EF1  bl 0x823993b0
	ctx.lr = 0x823884C4;
	sub_823993B0(ctx, base);
	// 823884C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823884C8: 4182FF50  beq 0x82388418
	if ctx.cr[0].eq {
	pc = 0x82388418; continue 'dispatch;
	}
	// 823884CC: 895E0004  lbz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 823884D0: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 823884D4: 897A0004  lbz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 823884D8: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 823884DC: A11E000A  lhz r8, 0xa(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 823884E0: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 823884E4: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823884E8: 808A000C  lwz r4, 0xc(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 823884EC: 80AB000C  lwz r5, 0xc(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823884F0: 4182003C  beq 0x8238852c
	if ctx.cr[0].eq {
	pc = 0x8238852C; continue 'dispatch;
	}
	// 823884F4: 391F004B  addi r8, r31, 0x4b
	ctx.r[8].s64 = ctx.r[31].s64 + 75;
	// 823884F8: 89680000  lbz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823884FC: 2B0B000B  cmplwi cr6, r11, 0xb
	ctx.cr[6].compare_u32(ctx.r[11].u32, 11 as u32, &mut ctx.xer);
	// 82388500: 409A0010  bne cr6, 0x82388510
	if !ctx.cr[6].eq {
	pc = 0x82388510; continue 'dispatch;
	}
	// 82388504: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82388508: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8238850C: 4BFDFDED  bl 0x823682f8
	ctx.lr = 0x82388510;
	sub_823682F8(ctx, base);
	// 82388510: A17E000A  lhz r11, 0xa(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82388514: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82388518: 38840040  addi r4, r4, 0x40
	ctx.r[4].s64 = ctx.r[4].s64 + 64;
	// 8238851C: 38A50040  addi r5, r5, 0x40
	ctx.r[5].s64 = ctx.r[5].s64 + 64;
	// 82388520: 39080050  addi r8, r8, 0x50
	ctx.r[8].s64 = ctx.r[8].s64 + 80;
	// 82388524: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82388528: 4198FFD0  blt cr6, 0x823884f8
	if ctx.cr[6].lt {
	pc = 0x823884F8; continue 'dispatch;
	}
	// 8238852C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82388530: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82388534: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 82388538: 481ACBC4  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82388540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82388540 size=908
    let mut pc: u32 = 0x82388540;
    'dispatch: loop {
        match pc {
            0x82388540 => {
    //   block [0x82388540..0x823888CC)
	// 82388540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82388544: 481ACB69  bl 0x825350ac
	ctx.lr = 0x82388548;
	sub_82535080(ctx, base);
	// 82388548: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238854C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82388550: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82388554: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82388558: 897C0006  lbz r11, 6(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(6 as u32) ) } as u64;
	// 8238855C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82388560: 419A000C  beq cr6, 0x8238856c
	if ctx.cr[6].eq {
	pc = 0x8238856C; continue 'dispatch;
	}
	// 82388564: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82388568: 4800000C  b 0x82388574
	pc = 0x82388574; continue 'dispatch;
	// 8238856C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82388570: 4800F509  bl 0x82397a78
	ctx.lr = 0x82388574;
	sub_82397A78(ctx, base);
	// 82388574: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82388578: 4082000C  bne 0x82388584
	if !ctx.cr[0].eq {
	pc = 0x82388584; continue 'dispatch;
	}
	// 8238857C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82388580: 48000344  b 0x823888c4
	pc = 0x823888C4; continue 'dispatch;
	// 82388584: 891C0007  lbz r8, 7(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(7 as u32) ) } as u64;
	// 82388588: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 8238858C: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82388590: 2B080009  cmplwi cr6, r8, 9
	ctx.cr[6].compare_u32(ctx.r[8].u32, 9 as u32, &mut ctx.xer);
	// 82388594: 40980018  bge cr6, 0x823885ac
	if !ctx.cr[6].lt {
	pc = 0x823885AC; continue 'dispatch;
	}
	// 82388598: A15C000A  lhz r10, 0xa(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(10 as u32) ) } as u64;
	// 8238859C: 214A0000  subfic r10, r10, 0
	ctx.xer.ca = ctx.r[10].u32 <= 0 as u32;
	ctx.r[10].s64 = (0 as i64) - ctx.r[10].s64;
	// 823885A0: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 823885A4: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 823885A8: 48000008  b 0x823885b0
	pc = 0x823885B0; continue 'dispatch;
	// 823885AC: 895C0009  lbz r10, 9(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(9 as u32) ) } as u64;
	// 823885B0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823885B4: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823885B8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823885BC: 40980048  bge cr6, 0x82388604
	if !ctx.cr[6].lt {
	pc = 0x82388604; continue 'dispatch;
	}
	// 823885C0: 2B080009  cmplwi cr6, r8, 9
	ctx.cr[6].compare_u32(ctx.r[8].u32, 9 as u32, &mut ctx.xer);
	// 823885C4: 4098000C  bge cr6, 0x823885d0
	if !ctx.cr[6].lt {
	pc = 0x823885D0; continue 'dispatch;
	}
	// 823885C8: A15C000A  lhz r10, 0xa(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(10 as u32) ) } as u64;
	// 823885CC: 48000024  b 0x823885f0
	pc = 0x823885F0; continue 'dispatch;
	// 823885D0: 895C0009  lbz r10, 9(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(9 as u32) ) } as u64;
	// 823885D4: 893C0004  lbz r9, 4(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 823885D8: 554A083E  rotlwi r10, r10, 1
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 823885DC: 7D29E214  add r9, r9, r28
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[28].u64;
	// 823885E0: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823885E4: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823885E8: 81290008  lwz r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 823885EC: 7D4A4A2E  lhzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 823885F0: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 823885F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 823885F8: 7FEAFA14  add r31, r10, r31
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 823885FC: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82388600: 4BFFFF90  b 0x82388590
	pc = 0x82388590; continue 'dispatch;
	// 82388604: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82388608: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8238860C: 4800F74D  bl 0x82397d58
	ctx.lr = 0x82388610;
	sub_82397D58(ctx, base);
	// 82388610: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82388614: 4182FF68  beq 0x8238857c
	if ctx.cr[0].eq {
	pc = 0x8238857C; continue 'dispatch;
	}
	// 82388618: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8238861C: 3BAB2200  addi r29, r11, 0x2200
	ctx.r[29].s64 = ctx.r[11].s64 + 8704;
	// 82388620: 1D7F0050  mulli r11, r31, 0x50
	ctx.r[11].s64 = ctx.r[31].s64 * 80;
	// 82388624: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 82388628: 7FEBDA14  add r31, r11, r27
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 8238862C: 48384C31  bl 0x8270d25c
	ctx.lr = 0x82388630;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82388630: 3B200002  li r25, 2
	ctx.r[25].s64 = 2;
	// 82388634: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82388638: 419A00A0  beq cr6, 0x823886d8
	if ctx.cr[6].eq {
	pc = 0x823886D8; continue 'dispatch;
	}
	// 8238863C: 3960FFBE  li r11, -0x42
	ctx.r[11].s64 = -66;
	// 82388640: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82388644: 40980094  bge cr6, 0x823886d8
	if !ctx.cr[6].lt {
	pc = 0x823886D8; continue 'dispatch;
	}
	// 82388648: 397F003F  addi r11, r31, 0x3f
	ctx.r[11].s64 = ctx.r[31].s64 + 63;
	// 8238864C: 387D0024  addi r3, r29, 0x24
	ctx.r[3].s64 = ctx.r[29].s64 + 36;
	// 82388650: 557E0032  rlwinm r30, r11, 0, 0, 0x19
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82388654: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82388658: 48022B19  bl 0x823ab170
	ctx.lr = 0x8238865C;
	sub_823AB170(ctx, base);
	// 8238865C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82388660: 41820078  beq 0x823886d8
	if ctx.cr[0].eq {
	pc = 0x823886D8; continue 'dispatch;
	}
	// 82388664: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82388668: 7D7E4850  subf r11, r30, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[30].s64;
	// 8238866C: 2B0B0080  cmplwi cr6, r11, 0x80
	ctx.cr[6].compare_u32(ctx.r[11].u32, 128 as u32, &mut ctx.xer);
	// 82388670: 41980054  blt cr6, 0x823886c4
	if ctx.cr[6].lt {
	pc = 0x823886C4; continue 'dispatch;
	}
	// 82388674: 396BFFC0  addi r11, r11, -0x40
	ctx.r[11].s64 = ctx.r[11].s64 + -64;
	// 82388678: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238867C: 7D0BF850  subf r8, r11, r31
	ctx.r[8].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82388680: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82388684: 7C884A14  add r4, r8, r9
	ctx.r[4].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82388688: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8238868C: 41820008  beq 0x82388694
	if ctx.cr[0].eq {
	pc = 0x82388694; continue 'dispatch;
	}
	// 82388690: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82388694: 91640010  stw r11, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82388698: 387D0024  addi r3, r29, 0x24
	ctx.r[3].s64 = ctx.r[29].s64 + 36;
	// 8238869C: 93E40004  stw r31, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 823886A0: 93440008  stw r26, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 823886A4: 9344000C  stw r26, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[26].u32 ) };
	// 823886A8: 93440014  stw r26, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 823886AC: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 823886B0: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 823886B4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 823886B8: 396BFFC0  addi r11, r11, -0x40
	ctx.r[11].s64 = ctx.r[11].s64 + -64;
	// 823886BC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 823886C0: 48022929  bl 0x823aafe8
	ctx.lr = 0x823886C4;
	sub_823AAFE8(ctx, base);
	// 823886C4: 933F0014  stw r25, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[25].u32 ) };
	// 823886C8: 3BDF0040  addi r30, r31, 0x40
	ctx.r[30].s64 = ctx.r[31].s64 + 64;
	// 823886CC: 935F000C  stw r26, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[26].u32 ) };
	// 823886D0: 935F0008  stw r26, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 823886D4: 48000008  b 0x823886dc
	pc = 0x823886DC; continue 'dispatch;
	// 823886D8: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 823886DC: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 823886E0: 48384B8D  bl 0x8270d26c
	ctx.lr = 0x823886E4;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 823886E4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 823886E8: 419AFE94  beq cr6, 0x8238857c
	if ctx.cr[6].eq {
	pc = 0x8238857C; continue 'dispatch;
	}
	// 823886EC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 823886F0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 823886F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823886F8: 7FBEDA14  add r29, r30, r27
	ctx.r[29].u64 = ctx.r[30].u64 + ctx.r[27].u64;
	// 823886FC: 481AC455  bl 0x82534b50
	ctx.lr = 0x82388700;
	sub_82534B50(ctx, base);
	// 82388700: 897E0006  lbz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82388704: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82388708: 419A000C  beq cr6, 0x82388714
	if ctx.cr[6].eq {
	pc = 0x82388714; continue 'dispatch;
	}
	// 8238870C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82388710: 4800000C  b 0x8238871c
	pc = 0x8238871C; continue 'dispatch;
	// 82388714: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82388718: 4800F361  bl 0x82397a78
	ctx.lr = 0x8238871C;
	sub_82397A78(ctx, base);
	// 8238871C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82388720: 40820010  bne 0x82388730
	if !ctx.cr[0].eq {
	pc = 0x82388730; continue 'dispatch;
	}
	// 82388724: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82388728: 48012979  bl 0x8239b0a0
	ctx.lr = 0x8238872C;
	sub_8239B0A0(ctx, base);
	// 8238872C: 4BFFFE50  b 0x8238857c
	pc = 0x8238857C; continue 'dispatch;
	// 82388730: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82388734: 893E0007  lbz r9, 7(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(7 as u32) ) } as u64;
	// 82388738: 2B090009  cmplwi cr6, r9, 9
	ctx.cr[6].compare_u32(ctx.r[9].u32, 9 as u32, &mut ctx.xer);
	// 8238873C: 40980018  bge cr6, 0x82388754
	if !ctx.cr[6].lt {
	pc = 0x82388754; continue 'dispatch;
	}
	// 82388740: A17E000A  lhz r11, 0xa(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82388744: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 82388748: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8238874C: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82388750: 48000008  b 0x82388758
	pc = 0x82388758; continue 'dispatch;
	// 82388754: 897E0009  lbz r11, 9(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(9 as u32) ) } as u64;
	// 82388758: 5546063E  clrlwi r6, r10, 0x18
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 8238875C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82388760: 7F065840  cmplw cr6, r6, r11
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82388764: 4098015C  bge cr6, 0x823888c0
	if !ctx.cr[6].lt {
	pc = 0x823888C0; continue 'dispatch;
	}
	// 82388768: 2B090009  cmplwi cr6, r9, 9
	ctx.cr[6].compare_u32(ctx.r[9].u32, 9 as u32, &mut ctx.xer);
	// 8238876C: 40980014  bge cr6, 0x82388780
	if !ctx.cr[6].lt {
	pc = 0x82388780; continue 'dispatch;
	}
	// 82388770: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388774: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82388778: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238877C: 48000028  b 0x823887a4
	pc = 0x823887A4; continue 'dispatch;
	// 82388780: 2B09000C  cmplwi cr6, r9, 0xc
	ctx.cr[6].compare_u32(ctx.r[9].u32, 12 as u32, &mut ctx.xer);
	// 82388784: 4098001C  bge cr6, 0x823887a0
	if !ctx.cr[6].lt {
	pc = 0x823887A0; continue 'dispatch;
	}
	// 82388788: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238878C: 54CA103A  slwi r10, r6, 2
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82388790: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82388794: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82388798: 7FEB502E  lwzx r31, r11, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8238879C: 48000008  b 0x823887a4
	pc = 0x823887A4; continue 'dispatch;
	// 823887A0: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 823887A4: 2B090009  cmplwi cr6, r9, 9
	ctx.cr[6].compare_u32(ctx.r[9].u32, 9 as u32, &mut ctx.xer);
	// 823887A8: 4098000C  bge cr6, 0x823887b4
	if !ctx.cr[6].lt {
	pc = 0x823887B4; continue 'dispatch;
	}
	// 823887AC: A17E000A  lhz r11, 0xa(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 823887B0: 48000024  b 0x823887d4
	pc = 0x823887D4; continue 'dispatch;
	// 823887B4: 897E0009  lbz r11, 9(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(9 as u32) ) } as u64;
	// 823887B8: 895E0004  lbz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 823887BC: 556B083E  rotlwi r11, r11, 1
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 823887C0: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 823887C4: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 823887C8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823887CC: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 823887D0: 7D6B522E  lhzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 823887D4: 5567043F  clrlwi. r7, r11, 0x10
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 823887D8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 823887DC: 41820028  beq 0x82388804
	if ctx.cr[0].eq {
	pc = 0x82388804; continue 'dispatch;
	}
	// 823887E0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823887E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823887E8: 48010E99  bl 0x82399680
	ctx.lr = 0x823887EC;
	sub_82399680(ctx, base);
	// 823887EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823887F0: 4182FF34  beq 0x82388724
	if ctx.cr[0].eq {
	pc = 0x82388724; continue 'dispatch;
	}
	// 823887F4: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 823887F8: 38A50040  addi r5, r5, 0x40
	ctx.r[5].s64 = ctx.r[5].s64 + 64;
	// 823887FC: 7F043840  cmplw cr6, r4, r7
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82388800: 4198FFE4  blt cr6, 0x823887e4
	if ctx.cr[6].lt {
	pc = 0x823887E4; continue 'dispatch;
	}
	// 82388804: 897E0007  lbz r11, 7(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(7 as u32) ) } as u64;
	// 82388808: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 8238880C: 4098002C  bge cr6, 0x82388838
	if !ctx.cr[6].lt {
	pc = 0x82388838; continue 'dispatch;
	}
	// 82388810: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 82388814: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388818: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8238881C: 4098000C  bge cr6, 0x82388828
	if !ctx.cr[6].lt {
	pc = 0x82388828; continue 'dispatch;
	}
	// 82388820: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82388824: 48000010  b 0x82388834
	pc = 0x82388834; continue 'dispatch;
	// 82388828: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238882C: 54CA103A  slwi r10, r6, 2
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82388830: 7FAB512E  stwx r29, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 82388834: 9B3E0008  stb r25, 8(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[25].u8 ) };
	// 82388838: 897E0007  lbz r11, 7(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(7 as u32) ) } as u64;
	// 8238883C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82388840: 4198006C  blt cr6, 0x823888ac
	if ctx.cr[6].lt {
	pc = 0x823888AC; continue 'dispatch;
	}
	// 82388844: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388848: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8238884C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82388850: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82388854: 41820058  beq 0x823888ac
	if ctx.cr[0].eq {
	pc = 0x823888AC; continue 'dispatch;
	}
	// 82388858: A15E000C  lhz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238885C: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 82388860: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82388864: 41820048  beq 0x823888ac
	if ctx.cr[0].eq {
	pc = 0x823888AC; continue 'dispatch;
	}
	// 82388868: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8238886C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82388870: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82388874: 2B080002  cmplwi cr6, r8, 2
	ctx.cr[6].compare_u32(ctx.r[8].u32, 2 as u32, &mut ctx.xer);
	// 82388878: 409A0020  bne cr6, 0x82388898
	if !ctx.cr[6].eq {
	pc = 0x82388898; continue 'dispatch;
	}
	// 8238887C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388880: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82388884: 41820014  beq 0x82388898
	if ctx.cr[0].eq {
	pc = 0x82388898; continue 'dispatch;
	}
	// 82388888: 810B0060  lwz r8, 0x60(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8238888C: 7F08F840  cmplw cr6, r8, r31
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82388890: 409A0008  bne cr6, 0x82388898
	if !ctx.cr[6].eq {
	pc = 0x82388898; continue 'dispatch;
	}
	// 82388894: 93AB0060  stw r29, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82388898: A17E000C  lhz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238889C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 823888A0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 823888A4: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823888A8: 4198FFC4  blt cr6, 0x8238886c
	if ctx.cr[6].lt {
	pc = 0x8238886C; continue 'dispatch;
	}
	// 823888AC: 1D670050  mulli r11, r7, 0x50
	ctx.r[11].s64 = ctx.r[7].s64 * 80;
	// 823888B0: 39460001  addi r10, r6, 1
	ctx.r[10].s64 = ctx.r[6].s64 + 1;
	// 823888B4: 7FABEA14  add r29, r11, r29
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 823888B8: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823888BC: 4BFFFE78  b 0x82388734
	pc = 0x82388734; continue 'dispatch;
	// 823888C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823888C4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 823888C8: 481AC834  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823888D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823888D0 size=1420
    let mut pc: u32 = 0x823888D0;
    'dispatch: loop {
        match pc {
            0x823888D0 => {
    //   block [0x823888D0..0x82388E5C)
	// 823888D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823888D4: 481AC7C9  bl 0x8253509c
	ctx.lr = 0x823888D8;
	sub_82535080(ctx, base);
	// 823888D8: DBE1FF98  stfd f31, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[31].u64 ) };
	// 823888DC: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823888E0: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 823888E4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823888E8: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 823888EC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 823888F0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 823888F4: 7EE8BB78  mr r8, r23
	ctx.r[8].u64 = ctx.r[23].u64;
	// 823888F8: 88F60007  lbz r7, 7(r22)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[22].u32.wrapping_add(7 as u32) ) } as u64;
	// 823888FC: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 82388900: 2B070009  cmplwi cr6, r7, 9
	ctx.cr[6].compare_u32(ctx.r[7].u32, 9 as u32, &mut ctx.xer);
	// 82388904: 40980018  bge cr6, 0x8238891c
	if !ctx.cr[6].lt {
	pc = 0x8238891C; continue 'dispatch;
	}
	// 82388908: A156000A  lhz r10, 0xa(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[22].u32.wrapping_add(10 as u32) ) } as u64;
	// 8238890C: 214A0000  subfic r10, r10, 0
	ctx.xer.ca = ctx.r[10].u32 <= 0 as u32;
	ctx.r[10].s64 = (0 as i64) - ctx.r[10].s64;
	// 82388910: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82388914: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82388918: 48000008  b 0x82388920
	pc = 0x82388920; continue 'dispatch;
	// 8238891C: 89560009  lbz r10, 9(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[22].u32.wrapping_add(9 as u32) ) } as u64;
	// 82388920: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82388924: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82388928: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8238892C: 40980048  bge cr6, 0x82388974
	if !ctx.cr[6].lt {
	pc = 0x82388974; continue 'dispatch;
	}
	// 82388930: 2B070009  cmplwi cr6, r7, 9
	ctx.cr[6].compare_u32(ctx.r[7].u32, 9 as u32, &mut ctx.xer);
	// 82388934: 4098000C  bge cr6, 0x82388940
	if !ctx.cr[6].lt {
	pc = 0x82388940; continue 'dispatch;
	}
	// 82388938: A156000A  lhz r10, 0xa(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[22].u32.wrapping_add(10 as u32) ) } as u64;
	// 8238893C: 48000024  b 0x82388960
	pc = 0x82388960; continue 'dispatch;
	// 82388940: 89560009  lbz r10, 9(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[22].u32.wrapping_add(9 as u32) ) } as u64;
	// 82388944: 89360004  lbz r9, 4(r22)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388948: 554A083E  rotlwi r10, r10, 1
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 8238894C: 7D29B214  add r9, r9, r22
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[22].u64;
	// 82388950: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82388954: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82388958: 81290008  lwz r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238895C: 7D4A4A2E  lhzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82388960: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82388964: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82388968: 7D0A4214  add r8, r10, r8
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8238896C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82388970: 4BFFFF90  b 0x82388900
	pc = 0x82388900; continue 'dispatch;
	// 82388974: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 82388978: 3B0B2200  addi r24, r11, 0x2200
	ctx.r[24].s64 = ctx.r[11].s64 + 8704;
	// 8238897C: 1D680050  mulli r11, r8, 0x50
	ctx.r[11].s64 = ctx.r[8].s64 * 80;
	// 82388980: 38780004  addi r3, r24, 4
	ctx.r[3].s64 = ctx.r[24].s64 + 4;
	// 82388984: 7FEBEA14  add r31, r11, r29
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82388988: 483848D5  bl 0x8270d25c
	ctx.lr = 0x8238898C;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 8238898C: 3AA00002  li r21, 2
	ctx.r[21].s64 = 2;
	// 82388990: 3B60FFBE  li r27, -0x42
	ctx.r[27].s64 = -66;
	// 82388994: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82388998: 419A009C  beq cr6, 0x82388a34
	if ctx.cr[6].eq {
	pc = 0x82388A34; continue 'dispatch;
	}
	// 8238899C: 7F1FD840  cmplw cr6, r31, r27
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[27].u32, &mut ctx.xer);
	// 823889A0: 40980094  bge cr6, 0x82388a34
	if !ctx.cr[6].lt {
	pc = 0x82388A34; continue 'dispatch;
	}
	// 823889A4: 397F003F  addi r11, r31, 0x3f
	ctx.r[11].s64 = ctx.r[31].s64 + 63;
	// 823889A8: 38780024  addi r3, r24, 0x24
	ctx.r[3].s64 = ctx.r[24].s64 + 36;
	// 823889AC: 557E0032  rlwinm r30, r11, 0, 0, 0x19
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823889B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823889B4: 480227BD  bl 0x823ab170
	ctx.lr = 0x823889B8;
	sub_823AB170(ctx, base);
	// 823889B8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823889BC: 41820078  beq 0x82388a34
	if ctx.cr[0].eq {
	pc = 0x82388A34; continue 'dispatch;
	}
	// 823889C0: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 823889C4: 7D7E4850  subf r11, r30, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[30].s64;
	// 823889C8: 2B0B0080  cmplwi cr6, r11, 0x80
	ctx.cr[6].compare_u32(ctx.r[11].u32, 128 as u32, &mut ctx.xer);
	// 823889CC: 41980054  blt cr6, 0x82388a20
	if ctx.cr[6].lt {
	pc = 0x82388A20; continue 'dispatch;
	}
	// 823889D0: 396BFFC0  addi r11, r11, -0x40
	ctx.r[11].s64 = ctx.r[11].s64 + -64;
	// 823889D4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823889D8: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 823889DC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823889E0: 7C89FA14  add r4, r9, r31
	ctx.r[4].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 823889E4: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823889E8: 41820008  beq 0x823889f0
	if ctx.cr[0].eq {
	pc = 0x823889F0; continue 'dispatch;
	}
	// 823889EC: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 823889F0: 91640010  stw r11, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 823889F4: 38780024  addi r3, r24, 0x24
	ctx.r[3].s64 = ctx.r[24].s64 + 36;
	// 823889F8: 93E40004  stw r31, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 823889FC: 92E40008  stw r23, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[23].u32 ) };
	// 82388A00: 92E4000C  stw r23, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[23].u32 ) };
	// 82388A04: 92E40014  stw r23, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[23].u32 ) };
	// 82388A08: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82388A0C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82388A10: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82388A14: 396BFFC0  addi r11, r11, -0x40
	ctx.r[11].s64 = ctx.r[11].s64 + -64;
	// 82388A18: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82388A1C: 480225CD  bl 0x823aafe8
	ctx.lr = 0x82388A20;
	sub_823AAFE8(ctx, base);
	// 82388A20: 92BF0014  stw r21, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[21].u32 ) };
	// 82388A24: 3BDF0040  addi r30, r31, 0x40
	ctx.r[30].s64 = ctx.r[31].s64 + 64;
	// 82388A28: 92FF000C  stw r23, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[23].u32 ) };
	// 82388A2C: 92FF0008  stw r23, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[23].u32 ) };
	// 82388A30: 48000008  b 0x82388a38
	pc = 0x82388A38; continue 'dispatch;
	// 82388A34: 7EFEBB78  mr r30, r23
	ctx.r[30].u64 = ctx.r[23].u64;
	// 82388A38: 38780004  addi r3, r24, 4
	ctx.r[3].s64 = ctx.r[24].s64 + 4;
	// 82388A3C: 48384831  bl 0x8270d26c
	ctx.lr = 0x82388A40;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82388A40: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82388A44: 409A000C  bne cr6, 0x82388a50
	if !ctx.cr[6].eq {
	pc = 0x82388A50; continue 'dispatch;
	}
	// 82388A48: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82388A4C: 48000404  b 0x82388e50
	pc = 0x82388E50; continue 'dispatch;
	// 82388A50: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82388A54: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82388A58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82388A5C: 481AC0F5  bl 0x82534b50
	ctx.lr = 0x82388A60;
	sub_82534B50(ctx, base);
	// 82388A60: 897E0006  lbz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82388A64: 7FBEEA14  add r29, r30, r29
	ctx.r[29].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82388A68: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82388A6C: 419A000C  beq cr6, 0x82388a78
	if ctx.cr[6].eq {
	pc = 0x82388A78; continue 'dispatch;
	}
	// 82388A70: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82388A74: 4800000C  b 0x82388a80
	pc = 0x82388A80; continue 'dispatch;
	// 82388A78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82388A7C: 4800EFFD  bl 0x82397a78
	ctx.lr = 0x82388A80;
	sub_82397A78(ctx, base);
	// 82388A80: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82388A84: 4182FFC4  beq 0x82388a48
	if ctx.cr[0].eq {
	pc = 0x82388A48; continue 'dispatch;
	}
	// 82388A88: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 82388A8C: 893E0007  lbz r9, 7(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(7 as u32) ) } as u64;
	// 82388A90: 2B090009  cmplwi cr6, r9, 9
	ctx.cr[6].compare_u32(ctx.r[9].u32, 9 as u32, &mut ctx.xer);
	// 82388A94: 40980018  bge cr6, 0x82388aac
	if !ctx.cr[6].lt {
	pc = 0x82388AAC; continue 'dispatch;
	}
	// 82388A98: A17E000A  lhz r11, 0xa(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82388A9C: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 82388AA0: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82388AA4: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82388AA8: 48000008  b 0x82388ab0
	pc = 0x82388AB0; continue 'dispatch;
	// 82388AAC: 897E0009  lbz r11, 9(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(9 as u32) ) } as u64;
	// 82388AB0: 5546063E  clrlwi r6, r10, 0x18
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82388AB4: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82388AB8: 7F065840  cmplw cr6, r6, r11
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82388ABC: 4098015C  bge cr6, 0x82388c18
	if !ctx.cr[6].lt {
	pc = 0x82388C18; continue 'dispatch;
	}
	// 82388AC0: 2B090009  cmplwi cr6, r9, 9
	ctx.cr[6].compare_u32(ctx.r[9].u32, 9 as u32, &mut ctx.xer);
	// 82388AC4: 40980014  bge cr6, 0x82388ad8
	if !ctx.cr[6].lt {
	pc = 0x82388AD8; continue 'dispatch;
	}
	// 82388AC8: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388ACC: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82388AD0: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82388AD4: 48000028  b 0x82388afc
	pc = 0x82388AFC; continue 'dispatch;
	// 82388AD8: 2B09000C  cmplwi cr6, r9, 0xc
	ctx.cr[6].compare_u32(ctx.r[9].u32, 12 as u32, &mut ctx.xer);
	// 82388ADC: 4098001C  bge cr6, 0x82388af8
	if !ctx.cr[6].lt {
	pc = 0x82388AF8; continue 'dispatch;
	}
	// 82388AE0: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388AE4: 54CA103A  slwi r10, r6, 2
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82388AE8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82388AEC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82388AF0: 7FEB502E  lwzx r31, r11, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82388AF4: 48000008  b 0x82388afc
	pc = 0x82388AFC; continue 'dispatch;
	// 82388AF8: 7EFFBB78  mr r31, r23
	ctx.r[31].u64 = ctx.r[23].u64;
	// 82388AFC: 2B090009  cmplwi cr6, r9, 9
	ctx.cr[6].compare_u32(ctx.r[9].u32, 9 as u32, &mut ctx.xer);
	// 82388B00: 4098000C  bge cr6, 0x82388b0c
	if !ctx.cr[6].lt {
	pc = 0x82388B0C; continue 'dispatch;
	}
	// 82388B04: A17E000A  lhz r11, 0xa(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82388B08: 48000024  b 0x82388b2c
	pc = 0x82388B2C; continue 'dispatch;
	// 82388B0C: 897E0009  lbz r11, 9(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(9 as u32) ) } as u64;
	// 82388B10: 895E0004  lbz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388B14: 556B083E  rotlwi r11, r11, 1
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 82388B18: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82388B1C: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82388B20: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82388B24: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82388B28: 7D6B522E  lhzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82388B2C: 5567043F  clrlwi. r7, r11, 0x10
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82388B30: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82388B34: 41820028  beq 0x82388b5c
	if ctx.cr[0].eq {
	pc = 0x82388B5C; continue 'dispatch;
	}
	// 82388B38: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82388B3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82388B40: 48010B41  bl 0x82399680
	ctx.lr = 0x82388B44;
	sub_82399680(ctx, base);
	// 82388B44: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82388B48: 41820290  beq 0x82388dd8
	if ctx.cr[0].eq {
	pc = 0x82388DD8; continue 'dispatch;
	}
	// 82388B4C: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82388B50: 38A50040  addi r5, r5, 0x40
	ctx.r[5].s64 = ctx.r[5].s64 + 64;
	// 82388B54: 7F043840  cmplw cr6, r4, r7
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82388B58: 4198FFE4  blt cr6, 0x82388b3c
	if ctx.cr[6].lt {
	pc = 0x82388B3C; continue 'dispatch;
	}
	// 82388B5C: 897E0007  lbz r11, 7(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(7 as u32) ) } as u64;
	// 82388B60: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 82388B64: 4098002C  bge cr6, 0x82388b90
	if !ctx.cr[6].lt {
	pc = 0x82388B90; continue 'dispatch;
	}
	// 82388B68: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 82388B6C: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388B70: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82388B74: 4098000C  bge cr6, 0x82388b80
	if !ctx.cr[6].lt {
	pc = 0x82388B80; continue 'dispatch;
	}
	// 82388B78: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82388B7C: 48000010  b 0x82388b8c
	pc = 0x82388B8C; continue 'dispatch;
	// 82388B80: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82388B84: 54CA103A  slwi r10, r6, 2
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82388B88: 7FAB512E  stwx r29, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 82388B8C: 9ABE0008  stb r21, 8(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[21].u8 ) };
	// 82388B90: 897E0007  lbz r11, 7(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(7 as u32) ) } as u64;
	// 82388B94: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82388B98: 4198006C  blt cr6, 0x82388c04
	if ctx.cr[6].lt {
	pc = 0x82388C04; continue 'dispatch;
	}
	// 82388B9C: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388BA0: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82388BA4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82388BA8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82388BAC: 41820058  beq 0x82388c04
	if ctx.cr[0].eq {
	pc = 0x82388C04; continue 'dispatch;
	}
	// 82388BB0: A15E000C  lhz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82388BB4: 7EE9BB78  mr r9, r23
	ctx.r[9].u64 = ctx.r[23].u64;
	// 82388BB8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82388BBC: 41820048  beq 0x82388c04
	if ctx.cr[0].eq {
	pc = 0x82388C04; continue 'dispatch;
	}
	// 82388BC0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82388BC4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82388BC8: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82388BCC: 2B080002  cmplwi cr6, r8, 2
	ctx.cr[6].compare_u32(ctx.r[8].u32, 2 as u32, &mut ctx.xer);
	// 82388BD0: 409A0020  bne cr6, 0x82388bf0
	if !ctx.cr[6].eq {
	pc = 0x82388BF0; continue 'dispatch;
	}
	// 82388BD4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388BD8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82388BDC: 41820014  beq 0x82388bf0
	if ctx.cr[0].eq {
	pc = 0x82388BF0; continue 'dispatch;
	}
	// 82388BE0: 810B0060  lwz r8, 0x60(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82388BE4: 7F08F840  cmplw cr6, r8, r31
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82388BE8: 409A0008  bne cr6, 0x82388bf0
	if !ctx.cr[6].eq {
	pc = 0x82388BF0; continue 'dispatch;
	}
	// 82388BEC: 93AB0060  stw r29, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82388BF0: A17E000C  lhz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82388BF4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82388BF8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82388BFC: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82388C00: 4198FFC4  blt cr6, 0x82388bc4
	if ctx.cr[6].lt {
	pc = 0x82388BC4; continue 'dispatch;
	}
	// 82388C04: 1D670050  mulli r11, r7, 0x50
	ctx.r[11].s64 = ctx.r[7].s64 * 80;
	// 82388C08: 39460001  addi r10, r6, 1
	ctx.r[10].s64 = ctx.r[6].s64 + 1;
	// 82388C0C: 7FABEA14  add r29, r11, r29
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82388C10: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82388C14: 4BFFFE78  b 0x82388a8c
	pc = 0x82388A8C; continue 'dispatch;
	// 82388C18: 897E0007  lbz r11, 7(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(7 as u32) ) } as u64;
	// 82388C1C: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 82388C20: 40980014  bge cr6, 0x82388c34
	if !ctx.cr[6].lt {
	pc = 0x82388C34; continue 'dispatch;
	}
	// 82388C24: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388C28: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82388C2C: 832B0008  lwz r25, 8(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82388C30: 48000028  b 0x82388c58
	pc = 0x82388C58; continue 'dispatch;
	// 82388C34: 897E0009  lbz r11, 9(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(9 as u32) ) } as u64;
	// 82388C38: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82388C3C: 4098000C  bge cr6, 0x82388c48
	if !ctx.cr[6].lt {
	pc = 0x82388C48; continue 'dispatch;
	}
	// 82388C40: 7EF9BB78  mr r25, r23
	ctx.r[25].u64 = ctx.r[23].u64;
	// 82388C44: 48000014  b 0x82388c58
	pc = 0x82388C58; continue 'dispatch;
	// 82388C48: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388C4C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82388C50: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82388C54: 832B0000  lwz r25, 0(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82388C58: 89760007  lbz r11, 7(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[22].u32.wrapping_add(7 as u32) ) } as u64;
	// 82388C5C: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 82388C60: 40980014  bge cr6, 0x82388c74
	if !ctx.cr[6].lt {
	pc = 0x82388C74; continue 'dispatch;
	}
	// 82388C64: 89760004  lbz r11, 4(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388C68: 7D6BB214  add r11, r11, r22
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[22].u64;
	// 82388C6C: 834B0008  lwz r26, 8(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82388C70: 48000024  b 0x82388c94
	pc = 0x82388C94; continue 'dispatch;
	// 82388C74: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 82388C78: 40980018  bge cr6, 0x82388c90
	if !ctx.cr[6].lt {
	pc = 0x82388C90; continue 'dispatch;
	}
	// 82388C7C: 89760004  lbz r11, 4(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388C80: 7D6BB214  add r11, r11, r22
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[22].u64;
	// 82388C84: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82388C88: 834B0000  lwz r26, 0(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82388C8C: 48000008  b 0x82388c94
	pc = 0x82388C94; continue 'dispatch;
	// 82388C90: 7EFABB78  mr r26, r23
	ctx.r[26].u64 = ctx.r[23].u64;
	// 82388C94: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82388C98: 419A01B4  beq cr6, 0x82388e4c
	if ctx.cr[6].eq {
	pc = 0x82388E4C; continue 'dispatch;
	}
	// 82388C9C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82388CA0: 419A01AC  beq cr6, 0x82388e4c
	if ctx.cr[6].eq {
	pc = 0x82388E4C; continue 'dispatch;
	}
	// 82388CA4: A176000A  lhz r11, 0xa(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[22].u32.wrapping_add(10 as u32) ) } as u64;
	// 82388CA8: 38780004  addi r3, r24, 4
	ctx.r[3].s64 = ctx.r[24].s64 + 4;
	// 82388CAC: 1FEB0050  mulli r31, r11, 0x50
	ctx.r[31].s64 = ctx.r[11].s64 * 80;
	// 82388CB0: 483845AD  bl 0x8270d25c
	ctx.lr = 0x82388CB4;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82388CB4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82388CB8: 419A009C  beq cr6, 0x82388d54
	if ctx.cr[6].eq {
	pc = 0x82388D54; continue 'dispatch;
	}
	// 82388CBC: 7F1FD840  cmplw cr6, r31, r27
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82388CC0: 40980094  bge cr6, 0x82388d54
	if !ctx.cr[6].lt {
	pc = 0x82388D54; continue 'dispatch;
	}
	// 82388CC4: 397F003F  addi r11, r31, 0x3f
	ctx.r[11].s64 = ctx.r[31].s64 + 63;
	// 82388CC8: 38780024  addi r3, r24, 0x24
	ctx.r[3].s64 = ctx.r[24].s64 + 36;
	// 82388CCC: 557D0032  rlwinm r29, r11, 0, 0, 0x19
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82388CD0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82388CD4: 4802249D  bl 0x823ab170
	ctx.lr = 0x82388CD8;
	sub_823AB170(ctx, base);
	// 82388CD8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82388CDC: 41820078  beq 0x82388d54
	if ctx.cr[0].eq {
	pc = 0x82388D54; continue 'dispatch;
	}
	// 82388CE0: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82388CE4: 7D7D4850  subf r11, r29, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[29].s64;
	// 82388CE8: 2B0B0080  cmplwi cr6, r11, 0x80
	ctx.cr[6].compare_u32(ctx.r[11].u32, 128 as u32, &mut ctx.xer);
	// 82388CEC: 41980054  blt cr6, 0x82388d40
	if ctx.cr[6].lt {
	pc = 0x82388D40; continue 'dispatch;
	}
	// 82388CF0: 396BFFC0  addi r11, r11, -0x40
	ctx.r[11].s64 = ctx.r[11].s64 + -64;
	// 82388CF4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82388CF8: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82388CFC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82388D00: 7C89FA14  add r4, r9, r31
	ctx.r[4].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 82388D04: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82388D08: 41820008  beq 0x82388d10
	if ctx.cr[0].eq {
	pc = 0x82388D10; continue 'dispatch;
	}
	// 82388D0C: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82388D10: 91640010  stw r11, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82388D14: 38780024  addi r3, r24, 0x24
	ctx.r[3].s64 = ctx.r[24].s64 + 36;
	// 82388D18: 93E40004  stw r31, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82388D1C: 92E40008  stw r23, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[23].u32 ) };
	// 82388D20: 92E4000C  stw r23, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[23].u32 ) };
	// 82388D24: 92E40014  stw r23, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[23].u32 ) };
	// 82388D28: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82388D2C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82388D30: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82388D34: 396BFFC0  addi r11, r11, -0x40
	ctx.r[11].s64 = ctx.r[11].s64 + -64;
	// 82388D38: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82388D3C: 480222AD  bl 0x823aafe8
	ctx.lr = 0x82388D40;
	sub_823AAFE8(ctx, base);
	// 82388D40: 92BF0014  stw r21, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[21].u32 ) };
	// 82388D44: 3B7F0040  addi r27, r31, 0x40
	ctx.r[27].s64 = ctx.r[31].s64 + 64;
	// 82388D48: 92FF000C  stw r23, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[23].u32 ) };
	// 82388D4C: 92FF0008  stw r23, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[23].u32 ) };
	// 82388D50: 48000008  b 0x82388d58
	pc = 0x82388D58; continue 'dispatch;
	// 82388D54: 7EFBBB78  mr r27, r23
	ctx.r[27].u64 = ctx.r[23].u64;
	// 82388D58: 38780004  addi r3, r24, 4
	ctx.r[3].s64 = ctx.r[24].s64 + 4;
	// 82388D5C: 48384511  bl 0x8270d26c
	ctx.lr = 0x82388D60;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82388D60: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82388D64: 419A0074  beq cr6, 0x82388dd8
	if ctx.cr[6].eq {
	pc = 0x82388DD8; continue 'dispatch;
	}
	// 82388D68: A39E000A  lhz r28, 0xa(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82388D6C: 7EFFBB78  mr r31, r23
	ctx.r[31].u64 = ctx.r[23].u64;
	// 82388D70: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82388D74: 41820040  beq 0x82388db4
	if ctx.cr[0].eq {
	pc = 0x82388DB4; continue 'dispatch;
	}
	// 82388D78: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82388D7C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82388D80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82388D84: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82388D88: 480108F9  bl 0x82399680
	ctx.lr = 0x82388D8C;
	sub_82399680(ctx, base);
	// 82388D8C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82388D90: 40820014  bne 0x82388da4
	if !ctx.cr[0].eq {
	pc = 0x82388DA4; continue 'dispatch;
	}
	// 82388D94: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82388D98: 48012309  bl 0x8239b0a0
	ctx.lr = 0x82388D9C;
	sub_8239B0A0(ctx, base);
	// 82388D9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82388DA0: 48012301  bl 0x8239b0a0
	ctx.lr = 0x82388DA4;
	sub_8239B0A0(ctx, base);
	// 82388DA4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82388DA8: 3BBD0040  addi r29, r29, 0x40
	ctx.r[29].s64 = ctx.r[29].s64 + 64;
	// 82388DAC: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82388DB0: 4198FFCC  blt cr6, 0x82388d7c
	if ctx.cr[6].lt {
	pc = 0x82388D7C; continue 'dispatch;
	}
	// 82388DB4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82388DB8: A09E000A  lhz r4, 0xa(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82388DBC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82388DC0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82388DC4: 480105ED  bl 0x823993b0
	ctx.lr = 0x82388DC8;
	sub_823993B0(ctx, base);
	// 82388DC8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82388DCC: 40820018  bne 0x82388de4
	if !ctx.cr[0].eq {
	pc = 0x82388DE4; continue 'dispatch;
	}
	// 82388DD0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82388DD4: 480122CD  bl 0x8239b0a0
	ctx.lr = 0x82388DD8;
	sub_8239B0A0(ctx, base);
	// 82388DD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82388DDC: 480122C5  bl 0x8239b0a0
	ctx.lr = 0x82388DE0;
	sub_8239B0A0(ctx, base);
	// 82388DE0: 4BFFFC68  b 0x82388a48
	pc = 0x82388A48; continue 'dispatch;
	// 82388DE4: 895E0004  lbz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388DE8: 7EE9BB78  mr r9, r23
	ctx.r[9].u64 = ctx.r[23].u64;
	// 82388DEC: 89760004  lbz r11, 4(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82388DF0: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82388DF4: A11E000A  lhz r8, 0xa(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82388DF8: 7D6BB214  add r11, r11, r22
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[22].u64;
	// 82388DFC: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82388E00: 808A000C  lwz r4, 0xc(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82388E04: 80AB000C  lwz r5, 0xc(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82388E08: 4182003C  beq 0x82388e44
	if ctx.cr[0].eq {
	pc = 0x82388E44; continue 'dispatch;
	}
	// 82388E0C: 3919004B  addi r8, r25, 0x4b
	ctx.r[8].s64 = ctx.r[25].s64 + 75;
	// 82388E10: 89680000  lbz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82388E14: 2B0B000B  cmplwi cr6, r11, 0xb
	ctx.cr[6].compare_u32(ctx.r[11].u32, 11 as u32, &mut ctx.xer);
	// 82388E18: 409A0010  bne cr6, 0x82388e28
	if !ctx.cr[6].eq {
	pc = 0x82388E28; continue 'dispatch;
	}
	// 82388E1C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82388E20: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82388E24: 4BFDF4D5  bl 0x823682f8
	ctx.lr = 0x82388E28;
	sub_823682F8(ctx, base);
	// 82388E28: A17E000A  lhz r11, 0xa(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82388E2C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82388E30: 38840040  addi r4, r4, 0x40
	ctx.r[4].s64 = ctx.r[4].s64 + 64;
	// 82388E34: 38A50040  addi r5, r5, 0x40
	ctx.r[5].s64 = ctx.r[5].s64 + 64;
	// 82388E38: 39080050  addi r8, r8, 0x50
	ctx.r[8].s64 = ctx.r[8].s64 + 80;
	// 82388E3C: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82388E40: 4198FFD0  blt cr6, 0x82388e10
	if ctx.cr[6].lt {
	pc = 0x82388E10; continue 'dispatch;
	}
	// 82388E44: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82388E48: 48012259  bl 0x8239b0a0
	ctx.lr = 0x82388E4C;
	sub_8239B0A0(ctx, base);
	// 82388E4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82388E50: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82388E54: CBE1FF98  lfd f31, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-104 as u32) ) };
	// 82388E58: 481AC294  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82388E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82388E60 size=60
    let mut pc: u32 = 0x82388E60;
    'dispatch: loop {
        match pc {
            0x82388E60 => {
    //   block [0x82388E60..0x82388E9C)
	// 82388E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82388E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82388E68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82388E6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82388E70: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82388E74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82388E78: 48054359  bl 0x823dd1d0
	ctx.lr = 0x82388E7C;
	sub_823DD1D0(ctx, base);
	// 82388E7C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82388E80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82388E84: 48053DF5  bl 0x823dcc78
	ctx.lr = 0x82388E88;
	sub_823DCC78(ctx, base);
	// 82388E88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82388E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82388E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82388E94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82388E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82388EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82388EA0 size=60
    let mut pc: u32 = 0x82388EA0;
    'dispatch: loop {
        match pc {
            0x82388EA0 => {
    //   block [0x82388EA0..0x82388EDC)
	// 82388EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82388EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82388EA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82388EAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82388EB0: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82388EB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82388EB8: 48054319  bl 0x823dd1d0
	ctx.lr = 0x82388EBC;
	sub_823DD1D0(ctx, base);
	// 82388EBC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82388EC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82388EC4: 48053DB5  bl 0x823dcc78
	ctx.lr = 0x82388EC8;
	sub_823DCC78(ctx, base);
	// 82388EC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82388ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82388ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82388ED4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82388ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82388EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82388EE0 size=308
    let mut pc: u32 = 0x82388EE0;
    'dispatch: loop {
        match pc {
            0x82388EE0 => {
    //   block [0x82388EE0..0x82389014)
	// 82388EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82388EE4: 481AC1B9  bl 0x8253509c
	ctx.lr = 0x82388EE8;
	sub_82535080(ctx, base);
	// 82388EE8: DBE1FF98  stfd f31, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[31].u64 ) };
	// 82388EEC: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82388EF0: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82388EF4: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 82388EF8: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82389018 size=324
    let mut pc: u32 = 0x82389018;
    'dispatch: loop {
        match pc {
            0x82389018 => {
    //   block [0x82389018..0x8238915C)
	// 82389018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238901C: 481AC07D  bl 0x82535098
	ctx.lr = 0x82389020;
	sub_82535080(ctx, base);
	// 82389020: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82389024: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82389028: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 8238902C: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82389160 size=460
    let mut pc: u32 = 0x82389160;
    'dispatch: loop {
        match pc {
            0x82389160 => {
    //   block [0x82389160..0x8238932C)
	// 82389160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82389164: 481ABF31  bl 0x82535094
	ctx.lr = 0x82389168;
	sub_82535080(ctx, base);
	// 82389168: DBE1FF88  stfd f31, -0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), ctx.f[31].u64 ) };
	// 8238916C: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82389170: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 82389174: 3D4082B6  lis r10, -0x7d4a
	ctx.r[10].s64 = -2102001664;
	// 82389178: 3CC082C0  lis r6, -0x7d40
	ctx.r[6].s64 = -2101346304;
	// 8238917C: 392ABA70  addi r9, r10, -0x4590
	ctx.r[9].s64 = ctx.r[10].s64 + -17808;
	// 82389180: 3CE082B6  lis r7, -0x7d4a
	ctx.r[7].s64 = -2102001664;
	// 82389184: 816BBF90  lwz r11, -0x4070(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16496 as u32) ) } as u64;
	// 82389188: 3E8082B6  lis r20, -0x7d4a
	ctx.r[20].s64 = -2102001664;
	// 8238918C: 38E7B660  addi r7, r7, -0x49a0
	ctx.r[7].s64 = ctx.r[7].s64 + -18848;
	// 82389190: 390B02E0  addi r8, r11, 0x2e0
	ctx.r[8].s64 = ctx.r[11].s64 + 736;
	// 82389194: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82389198: 390B02F0  addi r8, r11, 0x2f0
	ctx.r[8].s64 = ctx.r[11].s64 + 752;
	// 8238919C: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	// 823891A0: 9166BFB0  stw r11, -0x4050(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(-16464 as u32), ctx.r[11].u32 ) };
	// 823891A4: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 823891A8: E8CA0000  ld r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 823891AC: 80F4B65C  lwz r7, -0x49a4(r20)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-18852 as u32) ) } as u64;
	// 823891B0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823891B4: F8C90000  std r6, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 823891B8: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 823891BC: F9490008  std r10, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 823891C0: E9480000  ld r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 823891C4: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 823891C8: E9480008  ld r10, 8(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) };
	// 823891CC: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 823891D0: 419A000C  beq cr6, 0x823891dc
	if ctx.cr[6].eq {
	pc = 0x823891DC; continue 'dispatch;
	}
	// 823891D4: C3E70000  lfs f31, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 823891D8: 4800000C  b 0x823891e4
	pc = 0x823891E4; continue 'dispatch;
	// 823891DC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 823891E0: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 823891E4: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 823891E8: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 823891EC: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389330 size=16
    let mut pc: u32 = 0x82389330;
    'dispatch: loop {
        match pc {
            0x82389330 => {
    //   block [0x82389330..0x82389340)
	// 82389330: 3964FFFC  addi r11, r4, -4
	ctx.r[11].s64 = ctx.r[4].s64 + -4;
	// 82389334: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82389338: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8238933C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389340 size=16
    let mut pc: u32 = 0x82389340;
    'dispatch: loop {
        match pc {
            0x82389340 => {
    //   block [0x82389340..0x82389350)
	// 82389340: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82389344: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82389348: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8238934C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389350 size=8
    let mut pc: u32 = 0x82389350;
    'dispatch: loop {
        match pc {
            0x82389350 => {
    //   block [0x82389350..0x82389358)
	// 82389350: 548307FE  clrlwi r3, r4, 0x1f
	ctx.r[3].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82389354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389358 size=8
    let mut pc: u32 = 0x82389358;
    'dispatch: loop {
        match pc {
            0x82389358 => {
    //   block [0x82389358..0x82389360)
	// 82389358: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 8238935C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389360 size=12
    let mut pc: u32 = 0x82389360;
    'dispatch: loop {
        match pc {
            0x82389360 => {
    //   block [0x82389360..0x8238936C)
	// 82389360: 7C8B0034  cntlzw r11, r4
	ctx.r[11].u64 = if ctx.r[4].u32 == 0 { 32 } else { ctx.r[4].u32.leading_zeros() as u64 };
	// 82389364: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82389368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389370 size=16
    let mut pc: u32 = 0x82389370;
    'dispatch: loop {
        match pc {
            0x82389370 => {
    //   block [0x82389370..0x82389380)
	// 82389370: 3964FFF8  addi r11, r4, -8
	ctx.r[11].s64 = ctx.r[4].s64 + -8;
	// 82389374: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82389378: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8238937C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82389380 size=20
    let mut pc: u32 = 0x82389380;
    'dispatch: loop {
        match pc {
            0x82389380 => {
    //   block [0x82389380..0x82389394)
	// 82389380: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82389384: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82389388: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8238938C: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82389390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82389398 size=12
    let mut pc: u32 = 0x82389398;
    'dispatch: loop {
        match pc {
            0x82389398 => {
    //   block [0x82389398..0x823893A4)
	// 82389398: C0030030  lfs f0, 0x30(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238939C: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 823893A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823893A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823893A8 size=20
    let mut pc: u32 = 0x823893A8;
    'dispatch: loop {
        match pc {
            0x823893A8 => {
    //   block [0x823893A8..0x823893BC)
	// 823893A8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 823893AC: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823893B0: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 823893B4: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 823893B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823893C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823893C0 size=12
    let mut pc: u32 = 0x823893C0;
    'dispatch: loop {
        match pc {
            0x823893C0 => {
    //   block [0x823893C0..0x823893CC)
	// 823893C0: C0030034  lfs f0, 0x34(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823893C4: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 823893C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823893D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823893D0 size=8
    let mut pc: u32 = 0x823893D0;
    'dispatch: loop {
        match pc {
            0x823893D0 => {
    //   block [0x823893D0..0x823893D8)
	// 823893D0: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 823893D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823893D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823893D8 size=16
    let mut pc: u32 = 0x823893D8;
    'dispatch: loop {
        match pc {
            0x823893D8 => {
    //   block [0x823893D8..0x823893E8)
	// 823893D8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 823893DC: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823893E0: D0030094  stfs f0, 0x94(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 823893E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823893E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823893E8 size=20
    let mut pc: u32 = 0x823893E8;
    'dispatch: loop {
        match pc {
            0x823893E8 => {
    //   block [0x823893E8..0x823893FC)
	// 823893E8: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 823893EC: 419A0010  beq cr6, 0x823893fc
	if ctx.cr[6].eq {
		sub_823893FC(ctx, base);
		return;
	}
	// 823893F0: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 823893F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823893F8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823893FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823893FC size=8
    let mut pc: u32 = 0x823893FC;
    'dispatch: loop {
        match pc {
            0x823893FC => {
    //   block [0x823893FC..0x82389404)
	// 823893FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82389400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389408 size=8
    let mut pc: u32 = 0x82389408;
    'dispatch: loop {
        match pc {
            0x82389408 => {
    //   block [0x82389408..0x82389410)
	// 82389408: 386000B0  li r3, 0xb0
	ctx.r[3].s64 = 176;
	// 8238940C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389410 size=8
    let mut pc: u32 = 0x82389410;
    'dispatch: loop {
        match pc {
            0x82389410 => {
    //   block [0x82389410..0x82389418)
	// 82389410: 386000C0  li r3, 0xc0
	ctx.r[3].s64 = 192;
	// 82389414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389418 size=8
    let mut pc: u32 = 0x82389418;
    'dispatch: loop {
        match pc {
            0x82389418 => {
    //   block [0x82389418..0x82389420)
	// 82389418: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 8238941C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389420 size=16
    let mut pc: u32 = 0x82389420;
    'dispatch: loop {
        match pc {
            0x82389420 => {
    //   block [0x82389420..0x82389430)
	// 82389420: 3964FFF0  addi r11, r4, -0x10
	ctx.r[11].s64 = ctx.r[4].s64 + -16;
	// 82389424: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82389428: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8238942C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389430 size=16
    let mut pc: u32 = 0x82389430;
    'dispatch: loop {
        match pc {
            0x82389430 => {
    //   block [0x82389430..0x82389440)
	// 82389430: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82389434: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82389438: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238943C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389440 size=20
    let mut pc: u32 = 0x82389440;
    'dispatch: loop {
        match pc {
            0x82389440 => {
    //   block [0x82389440..0x82389454)
	// 82389440: 2F040010  cmpwi cr6, r4, 0x10
	ctx.cr[6].compare_i32(ctx.r[4].s32, 16, &mut ctx.xer);
	// 82389444: 419A0010  beq cr6, 0x82389454
	if ctx.cr[6].eq {
		sub_82389454(ctx, base);
		return;
	}
	// 82389448: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 8238944C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82389450: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389454(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389454 size=8
    let mut pc: u32 = 0x82389454;
    'dispatch: loop {
        match pc {
            0x82389454 => {
    //   block [0x82389454..0x8238945C)
	// 82389454: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82389458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389460 size=8
    let mut pc: u32 = 0x82389460;
    'dispatch: loop {
        match pc {
            0x82389460 => {
    //   block [0x82389460..0x82389468)
	// 82389460: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 82389464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389468 size=20
    let mut pc: u32 = 0x82389468;
    'dispatch: loop {
        match pc {
            0x82389468 => {
    //   block [0x82389468..0x8238947C)
	// 82389468: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 8238946C: 419A0010  beq cr6, 0x8238947c
	if ctx.cr[6].eq {
		sub_8238947C(ctx, base);
		return;
	}
	// 82389470: 2F040024  cmpwi cr6, r4, 0x24
	ctx.cr[6].compare_i32(ctx.r[4].s32, 36, &mut ctx.xer);
	// 82389474: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82389478: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238947C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238947C size=8
    let mut pc: u32 = 0x8238947C;
    'dispatch: loop {
        match pc {
            0x8238947C => {
    //   block [0x8238947C..0x82389484)
	// 8238947C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82389480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389488 size=8
    let mut pc: u32 = 0x82389488;
    'dispatch: loop {
        match pc {
            0x82389488 => {
    //   block [0x82389488..0x82389490)
	// 82389488: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 8238948C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389490 size=16
    let mut pc: u32 = 0x82389490;
    'dispatch: loop {
        match pc {
            0x82389490 => {
    //   block [0x82389490..0x823894A0)
	// 82389490: 3964FFC0  addi r11, r4, -0x40
	ctx.r[11].s64 = ctx.r[4].s64 + -64;
	// 82389494: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82389498: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8238949C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823894A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823894A0 size=120
    let mut pc: u32 = 0x823894A0;
    'dispatch: loop {
        match pc {
            0x823894A0 => {
    //   block [0x823894A0..0x82389518)
	// 823894A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823894A4: 481ABC19  bl 0x825350bc
	ctx.lr = 0x823894A8;
	sub_82535080(ctx, base);
	// 823894A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823894AC: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 823894B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823894B4: 48000048  b 0x823894fc
	pc = 0x823894FC; continue 'dispatch;
	// 823894B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823894BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823894C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823894C4: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 823894C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823894CC: 4E800421  bctrl
	ctx.lr = 0x823894D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823894D0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 823894D4: 41820024  beq 0x823894f8
	if ctx.cr[0].eq {
	pc = 0x823894F8; continue 'dispatch;
	}
	// 823894D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823894DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823894E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823894E4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 823894E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823894EC: 4E800421  bctrl
	ctx.lr = 0x823894F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823894F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823894F4: 4082001C  bne 0x82389510
	if !ctx.cr[0].eq {
	pc = 0x82389510; continue 'dispatch;
	}
	// 823894F8: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823894FC: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82389500: 4082FFB8  bne 0x823894b8
	if !ctx.cr[0].eq {
	pc = 0x823894B8; continue 'dispatch;
	}
	// 82389504: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82389508: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238950C: 481ABC00  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82389510: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82389514: 4BFFFFF4  b 0x82389508
	pc = 0x82389508; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389518 size=24
    let mut pc: u32 = 0x82389518;
    'dispatch: loop {
        match pc {
            0x82389518 => {
    //   block [0x82389518..0x82389530)
	// 82389518: 3D408311  lis r10, -0x7cef
	ctx.r[10].s64 = -2096037888;
	// 8238951C: 816A3C44  lwz r11, 0x3c44(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(15428 as u32) ) } as u64;
	// 82389520: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82389524: 3D208311  lis r9, -0x7cef
	ctx.r[9].s64 = -2096037888;
	// 82389528: 38693B70  addi r3, r9, 0x3b70
	ctx.r[3].s64 = ctx.r[9].s64 + 15216;
	// 8238952C: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389530 size=76
    let mut pc: u32 = 0x82389530;
    'dispatch: loop {
        match pc {
            0x82389530 => {
    //   block [0x82389530..0x8238957C)
	// 82389530: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82389534: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 82389538: 916A3C44  stw r11, 0x3c44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(15428 as u32), ctx.r[11].u32 ) };
	// 8238953C: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82389580 size=120
    let mut pc: u32 = 0x82389580;
    'dispatch: loop {
        match pc {
            0x82389580 => {
    //   block [0x82389580..0x823895F8)
	// 82389580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82389584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82389588: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238958C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82389590: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82389594: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82389598: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238959C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 823895A0: 394AFD28  addi r10, r10, -0x2d8
	ctx.r[10].s64 = ctx.r[10].s64 + -728;
	// 823895A4: C0091FF8  lfs f0, 0x1ff8(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823895A8: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 823895AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823895B0: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 823895B4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 823895B8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823895BC: C1A9BA38  lfs f13, -0x45c8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823895C0: D1BF000C  stfs f13, 0xc(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 823895C4: 4BFFFEDD  bl 0x823894a0
	ctx.lr = 0x823895C8;
	sub_823894A0(ctx, base);
	// 823895C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823895CC: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 823895D0: 40820010  bne 0x823895e0
	if !ctx.cr[0].eq {
	pc = 0x823895E0; continue 'dispatch;
	}
	// 823895D4: 3D608284  lis r11, -0x7d7c
	ctx.r[11].s64 = -2105278464;
	// 823895D8: 816BC8BC  lwz r11, -0x3744(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14148 as u32) ) } as u64;
	// 823895DC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 823895E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823895E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823895E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823895EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823895F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823895F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823895F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823895F8 size=24
    let mut pc: u32 = 0x823895F8;
    'dispatch: loop {
        match pc {
            0x823895F8 => {
    //   block [0x823895F8..0x82389610)
	// 823895F8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 823895FC: C003000C  lfs f0, 0xc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82389600: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82389604: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82389608: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8238960C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82389610 size=100
    let mut pc: u32 = 0x82389610;
    'dispatch: loop {
        match pc {
            0x82389610 => {
    //   block [0x82389610..0x82389674)
	// 82389610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82389614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82389618: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238961C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82389620: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82389624: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82389628: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8238962C: 394AFD54  addi r10, r10, -0x2ac
	ctx.r[10].s64 = ctx.r[10].s64 + -684;
	// 82389630: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82389634: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82389638: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8238963C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82389640: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82389644: 4BFFFE5D  bl 0x823894a0
	ctx.lr = 0x82389648;
	sub_823894A0(ctx, base);
	// 82389648: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238964C: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82389650: 4082000C  bne 0x8238965c
	if !ctx.cr[0].eq {
	pc = 0x8238965C; continue 'dispatch;
	}
	// 82389654: 4BFFFEC5  bl 0x82389518
	ctx.lr = 0x82389658;
	sub_82389518(ctx, base);
	// 82389658: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8238965C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82389660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82389664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82389668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238966C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82389670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389678 size=16
    let mut pc: u32 = 0x82389678;
    'dispatch: loop {
        match pc {
            0x82389678 => {
    //   block [0x82389678..0x82389688)
	// 82389678: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238967C: 38A30050  addi r5, r3, 0x50
	ctx.r[5].s64 = ctx.r[3].s64 + 80;
	// 82389680: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82389684: 4BFDE6AC  b 0x82367d30
	sub_82367D30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389688 size=16
    let mut pc: u32 = 0x82389688;
    'dispatch: loop {
        match pc {
            0x82389688 => {
    //   block [0x82389688..0x82389698)
	// 82389688: 38840010  addi r4, r4, 0x10
	ctx.r[4].s64 = ctx.r[4].s64 + 16;
	// 8238968C: 38630050  addi r3, r3, 0x50
	ctx.r[3].s64 = ctx.r[3].s64 + 80;
	// 82389690: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82389694: 481AB4BC  b 0x82534b50
	sub_82534B50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389698 size=44
    let mut pc: u32 = 0x82389698;
    'dispatch: loop {
        match pc {
            0x82389698 => {
    //   block [0x82389698..0x823896C4)
	// 82389698: A1430050  lhz r10, 0x50(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 8238969C: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 823896A0: A1230052  lhz r9, 0x52(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(82 as u32) ) } as u64;
	// 823896A4: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 823896A8: 5548103E  rotlwi r8, r10, 2
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 823896AC: 396BB670  addi r11, r11, -0x4990
	ctx.r[11].s64 = ctx.r[11].s64 + -18832;
	// 823896B0: 552A303E  rotlwi r10, r9, 6
	ctx.r[10].u64 = ((ctx.r[9].u32).rotate_left(6)) as u64;
	// 823896B4: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 823896B8: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 823896BC: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823896C0: 481AB490  b 0x82534b50
	sub_82534B50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823896C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823896C8 size=20
    let mut pc: u32 = 0x823896C8;
    'dispatch: loop {
        match pc {
            0x823896C8 => {
    //   block [0x823896C8..0x823896DC)
	// 823896C8: A1640012  lhz r11, 0x12(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(18 as u32) ) } as u64;
	// 823896CC: B1630050  sth r11, 0x50(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 823896D0: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 823896D4: B1630052  sth r11, 0x52(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(82 as u32), ctx.r[11].u16 ) };
	// 823896D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823896E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823896E0 size=60
    let mut pc: u32 = 0x823896E0;
    'dispatch: loop {
        match pc {
            0x823896E0 => {
    //   block [0x823896E0..0x8238971C)
	// 823896E0: A1430052  lhz r10, 0x52(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(82 as u32) ) } as u64;
	// 823896E4: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 823896E8: A1230050  lhz r9, 0x50(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 823896EC: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 823896F0: 5548083E  rotlwi r8, r10, 1
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 823896F4: 3D4082B6  lis r10, -0x7d4a
	ctx.r[10].s64 = -2102001664;
	// 823896F8: 396BB670  addi r11, r11, -0x4990
	ctx.r[11].s64 = ctx.r[11].s64 + -18832;
	// 823896FC: 5529103E  rotlwi r9, r9, 2
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(2)) as u64;
	// 82389700: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82389704: 814AB60C  lwz r10, -0x49f4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-18932 as u32) ) } as u64;
	// 82389708: 7D08522E  lhzx r8, r8, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8238970C: 7D49582E  lwzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82389710: 550B303E  rotlwi r11, r8, 6
	ctx.r[11].u64 = ((ctx.r[8].u32).rotate_left(6)) as u64;
	// 82389714: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82389718: 481AB438  b 0x82534b50
	sub_82534B50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82389720 size=316
    let mut pc: u32 = 0x82389720;
    'dispatch: loop {
        match pc {
            0x82389720 => {
    //   block [0x82389720..0x8238985C)
	// 82389720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82389724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82389728: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238972C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82389730: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82389734: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82389738: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238973C: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82389740: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82389744: FD800210  fabs f12, f0
	ctx.f[12].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82389748: C1AB2150  lfs f13, 0x2150(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8528 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238974C: FF0C6800  fcmpu cr6, f12, f13
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[13].f64);
	// 82389750: 419800F0  blt cr6, 0x82389840
	if ctx.cr[6].lt {
	pc = 0x82389840; continue 'dispatch;
	}
	// 82389754: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82389758: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8238975C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82389760: 419A00E0  beq cr6, 0x82389840
	if ctx.cr[6].eq {
	pc = 0x82389840; continue 'dispatch;
	}
	// 82389764: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82389768: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238976C: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 82389770: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82389774: 816BB65C  lwz r11, -0x49a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18852 as u32) ) } as u64;
	// 82389778: 4099003C  ble cr6, 0x823897b4
	if !ctx.cr[6].gt {
	pc = 0x823897B4; continue 'dispatch;
	}
	// 8238977C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82389780: 419A0008  beq cr6, 0x82389788
	if ctx.cr[6].eq {
	pc = 0x82389788; continue 'dispatch;
	}
	// 82389784: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82389788: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238978C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82389790: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82389794: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82389798: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389860 size=20
    let mut pc: u32 = 0x82389860;
    'dispatch: loop {
        match pc {
            0x82389860 => {
    //   block [0x82389860..0x82389874)
	// 82389860: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82389864: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82389868: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8238986C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82389870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389878 size=12
    let mut pc: u32 = 0x82389878;
    'dispatch: loop {
        match pc {
            0x82389878 => {
    //   block [0x82389878..0x82389884)
	// 82389878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8238987C: B1630016  sth r11, 0x16(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(22 as u32), ctx.r[11].u16 ) };
	// 82389880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82389888 size=456
    let mut pc: u32 = 0x82389888;
    'dispatch: loop {
        match pc {
            0x82389888 => {
    //   block [0x82389888..0x82389A50)
	// 82389888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238988C: 481AB82D  bl 0x825350b8
	ctx.lr = 0x82389890;
	sub_82535080(ctx, base);
	// 82389890: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82389894: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82389898: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238989C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 823898A0: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823898A4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 823898A8: FD800210  fabs f12, f0
	ctx.f[12].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 823898AC: C1AB2150  lfs f13, 0x2150(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8528 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823898B0: FF0C6800  fcmpu cr6, f12, f13
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[13].f64);
	// 823898B4: 41980190  blt cr6, 0x82389a44
	if ctx.cr[6].lt {
	pc = 0x82389A44; continue 'dispatch;
	}
	// 823898B8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 823898BC: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823898C0: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 823898C4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 823898C8: 816BB65C  lwz r11, -0x49a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18852 as u32) ) } as u64;
	// 823898CC: 40990074  ble cr6, 0x82389940
	if !ctx.cr[6].gt {
	pc = 0x82389940; continue 'dispatch;
	}
	// 823898D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823898D4: 419A0008  beq cr6, 0x823898dc
	if ctx.cr[6].eq {
	pc = 0x823898DC; continue 'dispatch;
	}
	// 823898D8: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 823898DC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 823898E0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 823898E4: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 823898E8: A11F0016  lhz r8, 0x16(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(22 as u32) ) } as u64;
	// 823898EC: 38E00030  li r7, 0x30
	ctx.r[7].s64 = 48;
	// 823898F0: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389A50 size=36
    let mut pc: u32 = 0x82389A50;
    'dispatch: loop {
        match pc {
            0x82389A50 => {
    //   block [0x82389A50..0x82389A74)
	// 82389A50: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82389A54: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82389A58: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82389A5C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82389A60: A1640016  lhz r11, 0x16(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(22 as u32) ) } as u64;
	// 82389A64: B1630014  sth r11, 0x14(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u16 ) };
	// 82389A68: 89640014  lbz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82389A6C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82389A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389A78 size=20
    let mut pc: u32 = 0x82389A78;
    'dispatch: loop {
        match pc {
            0x82389A78 => {
    //   block [0x82389A78..0x82389A8C)
	// 82389A78: A163000A  lhz r11, 0xa(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(10 as u32) ) } as u64;
	// 82389A7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82389A80: 39230020  addi r9, r3, 0x20
	ctx.r[9].s64 = ctx.r[3].s64 + 32;
	// 82389A84: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82389A88: 4C810020  blelr
	if !ctx.cr[0].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389A8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389A8C size=40
    let mut pc: u32 = 0x82389A8C;
    'dispatch: loop {
        match pc {
            0x82389A8C => {
    //   block [0x82389A8C..0x82389AB4)
	// 82389A8C: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82389A90: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82389A94: 7F075040  cmplw cr6, r7, r10
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82389A98: 409A0008  bne cr6, 0x82389aa0
	if !ctx.cr[6].eq {
	pc = 0x82389AA0; continue 'dispatch;
	}
	// 82389A9C: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82389AA0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82389AA4: 39290018  addi r9, r9, 0x18
	ctx.r[9].s64 = ctx.r[9].s64 + 24;
	// 82389AA8: 4082FFE8  bne 0x82389a90
	if !ctx.cr[0].eq {
	pc = 0x82389A90; continue 'dispatch;
	}
	// 82389AAC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82389AB0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389AB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389AB4 size=8
    let mut pc: u32 = 0x82389AB4;
    'dispatch: loop {
        match pc {
            0x82389AB4 => {
    //   block [0x82389AB4..0x82389ABC)
	// 82389AB4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82389AB8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389ABC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389ABC size=120
    let mut pc: u32 = 0x82389ABC;
    'dispatch: loop {
        match pc {
            0x82389ABC => {
    //   block [0x82389ABC..0x82389B34)
	// 82389ABC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82389AC0: 38ABFD20  addi r5, r11, -0x2e0
	ctx.r[5].s64 = ctx.r[11].s64 + -736;
	// 82389AC4: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82389AC8: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82389ACC: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82389AD0: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82389AD4: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82389AD8: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82389ADC: 7CE74050  subf r7, r7, r8
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 82389AE0: 41820014  beq 0x82389af4
	if ctx.cr[0].eq {
	pc = 0x82389AF4; continue 'dispatch;
	}
	// 82389AE4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82389AE8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82389AEC: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82389AF0: 419AFFE0  beq cr6, 0x82389ad0
	if ctx.cr[6].eq {
	pc = 0x82389AD0; continue 'dispatch;
	}
	// 82389AF4: 2C070000  cmpwi r7, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82389AF8: 4082009C  bne 0x82389b94
	if !ctx.cr[0].eq {
		sub_82389B34(ctx, base);
		return;
	}
	// 82389AFC: A1490008  lhz r10, 8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82389B00: 554B0631  rlwinm. r11, r10, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82389B04: 41820030  beq 0x82389b34
	if ctx.cr[0].eq {
		sub_82389B34(ctx, base);
		return;
	}
	// 82389B08: A1640008  lhz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82389B0C: 554A067E  clrlwi r10, r10, 0x19
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000007Fu64;
	// 82389B10: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82389B14: 409A0080  bne cr6, 0x82389b94
	if !ctx.cr[6].eq {
		sub_82389B34(ctx, base);
		return;
	}
	// 82389B18: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82389B1C: A14B0010  lhz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82389B20: A10B0012  lhz r8, 0x12(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 82389B24: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82389B28: 4098006C  bge cr6, 0x82389b94
	if !ctx.cr[6].lt {
		sub_82389B34(ctx, base);
		return;
	}
	// 82389B2C: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 82389B30: 48000048  b 0x82389b78
	sub_82389B34(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389B34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82389B34 size=124
    let mut pc: u32 = 0x82389B34;
    'dispatch: loop {
        match pc {
            0x82389B34 => {
    //   block [0x82389B34..0x82389BB0)
	// 82389B34: C0090018  lfs f0, 0x18(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82389B38: A0E40008  lhz r7, 8(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82389B3C: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82389B40: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 82389B44: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 82389B48: A161FFF6  lhz r11, -0xa(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-10 as u32) ) } as u64;
	// 82389B4C: 556B067E  clrlwi r11, r11, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 82389B50: 7F075840  cmplw cr6, r7, r11
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82389B54: 409A0040  bne cr6, 0x82389b94
	if !ctx.cr[6].eq {
	pc = 0x82389B94; continue 'dispatch;
	}
	// 82389B58: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82389B5C: 616B0080  ori r11, r11, 0x80
	ctx.r[11].u64 = ctx.r[11].u64 | 128;
	// 82389B60: B1690008  sth r11, 8(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u16 ) };
	// 82389B64: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82389B68: A14B0010  lhz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82389B6C: A0EB0012  lhz r7, 0x12(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 82389B70: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82389B74: 40980020  bge cr6, 0x82389b94
	if !ctx.cr[6].lt {
	pc = 0x82389B94; continue 'dispatch;
	}
	// 82389B78: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82389B7C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82389B80: 7D0A592E  stwx r8, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u32) };
	// 82389B84: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82389B88: A14B0010  lhz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82389B8C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82389B90: B14B0010  sth r10, 0x10(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u16 ) };
	// 82389B94: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 82389B98: 39650008  addi r11, r5, 8
	ctx.r[11].s64 = ctx.r[5].s64 + 8;
	// 82389B9C: 7F065800  cmpw cr6, r6, r11
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82389BA0: 4198FF28  blt cr6, 0x82389ac8
	if ctx.cr[6].lt {
		sub_82389ABC(ctx, base);
		return;
	}
	// 82389BA4: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82389BA8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82389BAC: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389BB0 size=12
    let mut pc: u32 = 0x82389BB0;
    'dispatch: loop {
        match pc {
            0x82389BB0 => {
    //   block [0x82389BB0..0x82389BBC)
	// 82389BB0: 7D2B4A15  add. r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82389BB4: 4082FF10  bne 0x82389ac4
	if !ctx.cr[0].eq {
		sub_82389ABC(ctx, base);
		return;
	}
	// 82389BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389BC0 size=12
    let mut pc: u32 = 0x82389BC0;
    'dispatch: loop {
        match pc {
            0x82389BC0 => {
    //   block [0x82389BC0..0x82389BCC)
	// 82389BC0: A1630010  lhz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82389BC4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82389BC8: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389BCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82389BCC size=68
    let mut pc: u32 = 0x82389BCC;
    'dispatch: loop {
        match pc {
            0x82389BCC => {
    //   block [0x82389BCC..0x82389C10)
	// 82389BCC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82389BD0: 81030014  lwz r8, 0x14(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82389BD4: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82389BD8: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82389BDC: C0030008  lfs f0, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82389BE0: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82389BE4: 7D2A402E  lwzx r9, r10, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82389BE8: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82389BEC: D0090000  stfs f0, 0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82389BF0: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82389BF4: C003000C  lfs f0, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82389BF8: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82389BFC: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82389C00: A1430010  lhz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82389C04: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82389C08: 4198FFC8  blt cr6, 0x82389bd0
	if ctx.cr[6].lt {
	pc = 0x82389BD0; continue 'dispatch;
	}
	// 82389C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389C10 size=16
    let mut pc: u32 = 0x82389C10;
    'dispatch: loop {
        match pc {
            0x82389C10 => {
    //   block [0x82389C10..0x82389C20)
	// 82389C10: A1630012  lhz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 82389C14: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82389C18: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82389C1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82389C20 size=148
    let mut pc: u32 = 0x82389C20;
    'dispatch: loop {
        match pc {
            0x82389C20 => {
    //   block [0x82389C20..0x82389CB4)
	// 82389C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82389C24: 481AB495  bl 0x825350b8
	ctx.lr = 0x82389C28;
	sub_82535080(ctx, base);
	// 82389C28: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82389C2C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82389C30: 83830014  lwz r28, 0x14(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82389C34: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82389C38: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82389C3C: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82389C40: A17F0016  lhz r11, 0x16(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(22 as u32) ) } as u64;
	// 82389C44: B1630012  sth r11, 0x12(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(18 as u32), ctx.r[11].u16 ) };
	// 82389C48: C01F0018  lfs f0, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82389C4C: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82389C50: C01F001C  lfs f0, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82389C54: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82389C58: A17F0012  lhz r11, 0x12(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 82389C5C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82389C60: A17F0014  lhz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82389C64: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82389C68: B1610058  sth r11, 0x58(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u16 ) };
	// 82389C6C: A17F0010  lhz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82389C70: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82389C74: 41820038  beq 0x82389cac
	if ctx.cr[0].eq {
	pc = 0x82389CAC; continue 'dispatch;
	}
	// 82389C78: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82389C7C: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82389C80: 3D408239  lis r10, -0x7dc7
	ctx.r[10].s64 = -2110193664;
	// 82389C84: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82389C88: 388A9A78  addi r4, r10, -0x6588
	ctx.r[4].s64 = ctx.r[10].s64 + -25992;
	// 82389C8C: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82389C90: 480126C1  bl 0x8239c350
	ctx.lr = 0x82389C94;
	sub_8239C350(ctx, base);
	// 82389C94: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 82389C98: A15F0010  lhz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82389C9C: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82389CA0: 557D043E  clrlwi r29, r11, 0x10
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82389CA4: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82389CA8: 4198FFD4  blt cr6, 0x82389c7c
	if ctx.cr[6].lt {
	pc = 0x82389C7C; continue 'dispatch;
	}
	// 82389CAC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82389CB0: 481AB458  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82389CB8 size=116
    let mut pc: u32 = 0x82389CB8;
    'dispatch: loop {
        match pc {
            0x82389CB8 => {
    //   block [0x82389CB8..0x82389D2C)
	// 82389CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82389CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82389CC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82389CC4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82389CC8: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 82389CCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82389CD0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82389CD4: C1BF0008  lfs f13, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82389CD8: C19F0018  lfs f12, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82389CDC: C17F000C  lfs f11, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82389CE0: C15F001C  lfs f10, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82389CE4: EC2C683A  fmadds f1, f12, f0, f13
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82389CE8: D03F0008  stfs f1, 8(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82389CEC: EC0A583A  fmadds f0, f10, f0, f11
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[11].f64) as f32) as f64);
	// 82389CF0: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82389CF4: 481AA3DD  bl 0x825340d0
	ctx.lr = 0x82389CF8;
	sub_825340D0(ctx, base);
	// 82389CF8: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82389CFC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82389D00: FDA00818  frsp f13, f1
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82389D04: D1BF0008  stfs f13, 8(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82389D08: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 82389D0C: 481AA3C5  bl 0x825340d0
	ctx.lr = 0x82389D10;
	sub_825340D0(ctx, base);
	// 82389D10: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82389D14: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82389D18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82389D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82389D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82389D24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82389D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389D30 size=16
    let mut pc: u32 = 0x82389D30;
    'dispatch: loop {
        match pc {
            0x82389D30 => {
    //   block [0x82389D30..0x82389D40)
	// 82389D30: A1630012  lhz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 82389D34: 396B000D  addi r11, r11, 0xd
	ctx.r[11].s64 = ctx.r[11].s64 + 13;
	// 82389D38: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82389D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82389D40 size=268
    let mut pc: u32 = 0x82389D40;
    'dispatch: loop {
        match pc {
            0x82389D40 => {
    //   block [0x82389D40..0x82389E4C)
	// 82389D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82389D44: 481AB375  bl 0x825350b8
	ctx.lr = 0x82389D48;
	sub_82535080(ctx, base);
	// 82389D48: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82389D4C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82389D50: 83830014  lwz r28, 0x14(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82389D54: 39430034  addi r10, r3, 0x34
	ctx.r[10].s64 = ctx.r[3].s64 + 52;
	// 82389D58: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82389D5C: 3BDF0024  addi r30, r31, 0x24
	ctx.r[30].s64 = ctx.r[31].s64 + 36;
	// 82389D60: A17F0016  lhz r11, 0x16(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(22 as u32) ) } as u64;
	// 82389D64: B1630012  sth r11, 0x12(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(18 as u32), ctx.r[11].u16 ) };
	// 82389D68: A17F0018  lhz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82389D6C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82389D70: A17F001A  lhz r11, 0x1a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(26 as u32) ) } as u64;
	// 82389D74: A13F0018  lhz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82389D78: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82389D7C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82389D80: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82389D84: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82389D88: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82389D8C: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82389D90: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82389D94: C01F0020  lfs f0, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82389D98: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82389D9C: A17F0018  lhz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82389DA0: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82389DA4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82389DA8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82389DAC: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82389DB0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82389DB4: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82389DB8: C00BBA38  lfs f0, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82389DBC: EDA06824  fdivs f13, f0, f13
	ctx.f[13].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82389DC0: D1A30020  stfs f13, 0x20(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82389DC4: A17F001A  lhz r11, 0x1a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(26 as u32) ) } as u64;
	// 82389DC8: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82389DCC: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82389DD0: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82389DD4: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82389DD8: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82389DDC: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82389DE0: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82389DE4: C01F001C  lfs f0, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82389DE8: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82389DEC: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82389DF0: A17F0012  lhz r11, 0x12(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 82389DF4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82389DF8: A17F0014  lhz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82389DFC: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82389E00: B1610060  sth r11, 0x60(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u16 ) };
	// 82389E04: A17F0010  lhz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82389E08: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82389E0C: 41820038  beq 0x82389e44
	if ctx.cr[0].eq {
	pc = 0x82389E44; continue 'dispatch;
	}
	// 82389E10: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82389E14: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82389E18: 3D408239  lis r10, -0x7dc7
	ctx.r[10].s64 = -2110193664;
	// 82389E1C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82389E20: 388A9A78  addi r4, r10, -0x6588
	ctx.r[4].s64 = ctx.r[10].s64 + -25992;
	// 82389E24: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82389E28: 48012529  bl 0x8239c350
	ctx.lr = 0x82389E2C;
	sub_8239C350(ctx, base);
	// 82389E2C: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 82389E30: A15F0010  lhz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82389E34: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82389E38: 557D043E  clrlwi r29, r11, 0x10
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82389E3C: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82389E40: 4198FFD4  blt cr6, 0x82389e14
	if ctx.cr[6].lt {
	pc = 0x82389E14; continue 'dispatch;
	}
	// 82389E44: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82389E48: 481AB2C0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82389E50 size=152
    let mut pc: u32 = 0x82389E50;
    'dispatch: loop {
        match pc {
            0x82389E50 => {
    //   block [0x82389E50..0x82389EE8)
	// 82389E50: C003002C  lfs f0, 0x2c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82389E54: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82389E58: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82389E5C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82389E60: EC0D007A  fmadds f0, f13, f1, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 82389E64: C183001C  lfs f12, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82389E68: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82389E6C: C1430024  lfs f10, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82389E70: C1630020  lfs f11, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82389E74: EDA06028  fsubs f13, f0, f12
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 82389E78: FD80065E  fctidz f12, f0
	ctx.f[12].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82389E7C: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 82389E80: FD806028  fsub f12, f0, f12
	ctx.f[12].f64 = ctx.f[0].f64 - ctx.f[12].f64;
	// 82389E84: FC0D032E  fsel f0, f13, f12, f0
	ctx.f[0].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[0].f64 };
	// 82389E88: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82389E8C: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82389E90: FDA0065E  fctidz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82389E94: 7DA057AE  stfiwx f13, 0, r10
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 82389E98: 8141FFF0  lwz r10, -0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82389E9C: 7D2A5B96  divwu r9, r10, r11
	ctx.r[9].u32 = ctx.r[10].u32 / ctx.r[11].u32;
	// 82389EA0: 7D0A5B96  divwu r8, r10, r11
	ctx.r[8].u32 = ctx.r[10].u32 / ctx.r[11].u32;
	// 82389EA4: 7D6959D6  mullw r11, r9, r11
	ctx.r[11].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82389EA8: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82389EAC: 790A0020  clrldi r10, r8, 0x20
	ctx.r[10].u64 = ctx.r[8].u64 & 0x00000000FFFFFFFFu64;
	// 82389EB0: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82389EB4: F941FFF0  std r10, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u64 ) };
	// 82389EB8: C801FFF0  lfd f0, -0x10(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82389EBC: F961FFF8  std r11, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[11].u64 ) };
	// 82389EC0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82389EC4: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82389EC8: EC0002B2  fmuls f0, f0, f10
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[10].f64) as f32) as f64);
	// 82389ECC: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82389ED0: C9A1FFF8  lfd f13, -8(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82389ED4: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82389ED8: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82389EDC: EC0D02F2  fmuls f0, f13, f11
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[11].f64) as f32) as f64);
	// 82389EE0: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82389EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82389EE8 size=16
    let mut pc: u32 = 0x82389EE8;
    'dispatch: loop {
        match pc {
            0x82389EE8 => {
    //   block [0x82389EE8..0x82389EF8)
	// 82389EE8: A1630012  lhz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 82389EEC: 396B000A  addi r11, r11, 0xa
	ctx.r[11].s64 = ctx.r[11].s64 + 10;
	// 82389EF0: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82389EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82389EF8 size=156
    let mut pc: u32 = 0x82389EF8;
    'dispatch: loop {
        match pc {
            0x82389EF8 => {
    //   block [0x82389EF8..0x82389F94)
	// 82389EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82389EFC: 481AB1BD  bl 0x825350b8
	ctx.lr = 0x82389F00;
	sub_82535080(ctx, base);
	// 82389F00: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82389F04: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82389F08: 83830014  lwz r28, 0x14(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82389F0C: 39430028  addi r10, r3, 0x28
	ctx.r[10].s64 = ctx.r[3].s64 + 40;
	// 82389F10: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82389F14: 3BDF0024  addi r30, r31, 0x24
	ctx.r[30].s64 = ctx.r[31].s64 + 36;
	// 82389F18: A17F0016  lhz r11, 0x16(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(22 as u32) ) } as u64;
	// 82389F1C: B1630012  sth r11, 0x12(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(18 as u32), ctx.r[11].u16 ) };
	// 82389F20: C01F0018  lfs f0, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82389F24: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82389F28: C01F001C  lfs f0, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82389F2C: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82389F30: C01F0020  lfs f0, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82389F34: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82389F38: A17F0012  lhz r11, 0x12(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 82389F3C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82389F40: A17F0014  lhz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82389F44: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82389F48: B1610058  sth r11, 0x58(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u16 ) };
	// 82389F4C: A17F0010  lhz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82389F50: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82389F54: 41820038  beq 0x82389f8c
	if ctx.cr[0].eq {
	pc = 0x82389F8C; continue 'dispatch;
	}
	// 82389F58: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82389F5C: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82389F60: 3D408239  lis r10, -0x7dc7
	ctx.r[10].s64 = -2110193664;
	// 82389F64: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82389F68: 388A9A78  addi r4, r10, -0x6588
	ctx.r[4].s64 = ctx.r[10].s64 + -25992;
	// 82389F6C: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82389F70: 480123E1  bl 0x8239c350
	ctx.lr = 0x82389F74;
	sub_8239C350(ctx, base);
	// 82389F74: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 82389F78: A15F0010  lhz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82389F7C: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82389F80: 557D043E  clrlwi r29, r11, 0x10
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82389F84: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82389F88: 4198FFD4  blt cr6, 0x82389f5c
	if ctx.cr[6].lt {
	pc = 0x82389F5C; continue 'dispatch;
	}
	// 82389F8C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82389F90: 481AB178  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82389F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82389F98 size=152
    let mut pc: u32 = 0x82389F98;
    'dispatch: loop {
        match pc {
            0x82389F98 => {
    //   block [0x82389F98..0x8238A030)
	// 82389F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82389F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82389FA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82389FA4: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82389FA8: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82389FAC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82389FB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82389FB4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82389FB8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82389FBC: C3DF001C  lfs f30, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82389FC0: C00B2490  lfs f0, 0x2490(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9360 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82389FC4: EC3E0032  fmuls f1, f30, f0
	ctx.f[1].f64 = (((ctx.f[30].f64 * ctx.f[0].f64) as f32) as f64);
	// 82389FC8: 4BD93501  bl 0x8211d4c8
	ctx.lr = 0x82389FCC;
	sub_8211D4C8(ctx, base);
	// 82389FCC: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82389FD0: C1BF0024  lfs f13, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82389FD4: C19F0018  lfs f12, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82389FD8: EC0D07FA  fmadds f0, f13, f31, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[31].f64 + ctx.f[0].f64) as f32) as f64);
	// 82389FDC: EDACF7FA  fmadds f13, f12, f31, f30
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[31].f64 + ctx.f[30].f64) as f32) as f64);
	// 82389FE0: C17F0020  lfs f11, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82389FE4: ED8102F2  fmuls f12, f1, f11
	ctx.f[12].f64 = (((ctx.f[1].f64 * ctx.f[11].f64) as f32) as f64);
	// 82389FE8: D19F0008  stfs f12, 8(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82389FEC: FD80065E  fctidz f12, f0
	ctx.f[12].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82389FF0: FD606E5E  fctidz f11, f13
	ctx.f[11].s64 = if ctx.f[13].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[13].f64.trunc() as i64 };
	// 82389FF4: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 82389FF8: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 82389FFC: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8238A000: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 8238A004: EC006028  fsubs f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 8238A008: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8238A00C: EC0D5828  fsubs f0, f13, f11
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[11].f64) as f32) as f64);
	// 8238A010: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8238A014: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238A018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238A01C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238A020: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8238A024: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238A028: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238A02C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238A030 size=16
    let mut pc: u32 = 0x8238A030;
    'dispatch: loop {
        match pc {
            0x8238A030 => {
    //   block [0x8238A030..0x8238A040)
	// 8238A030: A1630012  lhz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 8238A034: 396B000E  addi r11, r11, 0xe
	ctx.r[11].s64 = ctx.r[11].s64 + 14;
	// 8238A038: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8238A03C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238A040 size=220
    let mut pc: u32 = 0x8238A040;
    'dispatch: loop {
        match pc {
            0x8238A040 => {
    //   block [0x8238A040..0x8238A11C)
	// 8238A040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238A044: 481AB075  bl 0x825350b8
	ctx.lr = 0x8238A048;
	sub_82535080(ctx, base);
	// 8238A048: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238A04C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8238A050: A17F0016  lhz r11, 0x16(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(22 as u32) ) } as u64;
	// 8238A054: B1630012  sth r11, 0x12(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(18 as u32), ctx.r[11].u16 ) };
	// 8238A058: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238A05C: C01F0020  lfs f0, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A060: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8238A064: C01F0024  lfs f0, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A068: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8238A06C: C01F0018  lfs f0, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A070: C1AB2150  lfs f13, 0x2150(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8528 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A074: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8238A078: 41990008  bgt cr6, 0x8238a080
	if ctx.cr[6].gt {
	pc = 0x8238A080; continue 'dispatch;
	}
	// 8238A07C: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 8238A080: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238A084: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8238A088: 83830014  lwz r28, 0x14(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8238A08C: 3BDF002C  addi r30, r31, 0x2c
	ctx.r[30].s64 = ctx.r[31].s64 + 44;
	// 8238A090: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8238A094: C1ABBA38  lfs f13, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A098: 39630038  addi r11, r3, 0x38
	ctx.r[11].s64 = ctx.r[3].s64 + 56;
	// 8238A09C: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8238A0A0: D1A30020  stfs f13, 0x20(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8238A0A4: C1BF001C  lfs f13, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A0A8: D1A30028  stfs f13, 0x28(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8238A0AC: C1BF0028  lfs f13, 0x28(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A0B0: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238A0B4: D1A30034  stfs f13, 0x34(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 8238A0B8: D1A3002C  stfs f13, 0x2c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8238A0BC: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8238A0C0: A15F0012  lhz r10, 0x12(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 8238A0C4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8238A0C8: A15F0014  lhz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8238A0CC: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8238A0D0: A17F0010  lhz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238A0D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238A0D8: B1410058  sth r10, 0x58(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u16 ) };
	// 8238A0DC: 41820038  beq 0x8238a114
	if ctx.cr[0].eq {
	pc = 0x8238A114; continue 'dispatch;
	}
	// 8238A0E0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8238A0E4: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238A0E8: 3D408239  lis r10, -0x7dc7
	ctx.r[10].s64 = -2110193664;
	// 8238A0EC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8238A0F0: 388A9A78  addi r4, r10, -0x6588
	ctx.r[4].s64 = ctx.r[10].s64 + -25992;
	// 8238A0F4: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8238A0F8: 48012259  bl 0x8239c350
	ctx.lr = 0x8238A0FC;
	sub_8239C350(ctx, base);
	// 8238A0FC: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 8238A100: A15F0010  lhz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238A104: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 8238A108: 557D043E  clrlwi r29, r11, 0x10
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8238A10C: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8238A110: 4198FFD4  blt cr6, 0x8238a0e4
	if ctx.cr[6].lt {
	pc = 0x8238A0E4; continue 'dispatch;
	}
	// 8238A114: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8238A118: 481AAFF0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238A120 size=24
    let mut pc: u32 = 0x8238A120;
    'dispatch: loop {
        match pc {
            0x8238A120 => {
    //   block [0x8238A120..0x8238A138)
	// 8238A120: C0030034  lfs f0, 0x34(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A124: C1A30024  lfs f13, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A128: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8238A12C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238A130: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8238A134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238A138 size=192
    let mut pc: u32 = 0x8238A138;
    'dispatch: loop {
        match pc {
            0x8238A138 => {
    //   block [0x8238A138..0x8238A1F8)
	// 8238A138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238A13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238A140: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238A144: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238A148: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238A14C: C01F002C  lfs f0, 0x2c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A150: EC01002A  fadds f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8238A154: C1BF0020  lfs f13, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A158: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8238A15C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8238A160: 41990010  bgt cr6, 0x8238a170
	if ctx.cr[6].gt {
	pc = 0x8238A170; continue 'dispatch;
	}
	// 8238A164: C1BF0024  lfs f13, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A168: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238A16C: 48000028  b 0x8238a194
	pc = 0x8238A194; continue 'dispatch;
	// 8238A170: C19F0028  lfs f12, 0x28(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238A174: EDAC682A  fadds f13, f12, f13
	ctx.f[13].f64 = ((ctx.f[12].f64 + ctx.f[13].f64) as f32) as f64;
	// 8238A178: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8238A17C: 40990010  ble cr6, 0x8238a18c
	if !ctx.cr[6].gt {
	pc = 0x8238A18C; continue 'dispatch;
	}
	// 8238A180: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238A184: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8238A188: 4BFFFFDC  b 0x8238a164
	pc = 0x8238A164; continue 'dispatch;
	// 8238A18C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238A190: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A194: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 8238A198: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238A19C: D01F0030  stfs f0, 0x30(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8238A1A0: C00B2490  lfs f0, 0x2490(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9360 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A1A4: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238A1A8: 4BD93321  bl 0x8211d4c8
	ctx.lr = 0x8238A1AC;
	sub_8211D4C8(ctx, base);
	// 8238A1AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238A1B0: C1BF0018  lfs f13, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A1B4: C19F001C  lfs f12, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238A1B8: C00BBA38  lfs f0, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A1BC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238A1C0: ED41002A  fadds f10, f1, f0
	ctx.f[10].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8238A1C4: C16BBFFC  lfs f11, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8238A1C8: ED6A02F2  fmuls f11, f10, f11
	ctx.f[11].f64 = (((ctx.f[10].f64 * ctx.f[11].f64) as f32) as f64);
	// 8238A1CC: ED406828  fsubs f10, f0, f13
	ctx.f[10].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238A1D0: EC006028  fsubs f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 8238A1D4: EDAA6AFA  fmadds f13, f10, f11, f13
	ctx.f[13].f64 = (((ctx.f[10].f64 * ctx.f[11].f64 + ctx.f[13].f64) as f32) as f64);
	// 8238A1D8: D1BF0008  stfs f13, 8(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8238A1DC: EC0062FA  fmadds f0, f0, f11, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[11].f64 + ctx.f[12].f64) as f32) as f64);
	// 8238A1E0: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8238A1E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8238A1E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238A1EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238A1F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238A1F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238A1F8 size=16
    let mut pc: u32 = 0x8238A1F8;
    'dispatch: loop {
        match pc {
            0x8238A1F8 => {
    //   block [0x8238A1F8..0x8238A208)
	// 8238A1F8: A1630012  lhz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 8238A1FC: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 8238A200: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8238A204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238A208 size=300
    let mut pc: u32 = 0x8238A208;
    'dispatch: loop {
        match pc {
            0x8238A208 => {
    //   block [0x8238A208..0x8238A334)
	// 8238A208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238A20C: 481AAEAD  bl 0x825350b8
	ctx.lr = 0x8238A210;
	sub_82535080(ctx, base);
	// 8238A210: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238A214: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8238A218: 83830014  lwz r28, 0x14(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8238A21C: 39430040  addi r10, r3, 0x40
	ctx.r[10].s64 = ctx.r[3].s64 + 64;
	// 8238A220: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 8238A224: 3BDF002C  addi r30, r31, 0x2c
	ctx.r[30].s64 = ctx.r[31].s64 + 44;
	// 8238A228: A17F0016  lhz r11, 0x16(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(22 as u32) ) } as u64;
	// 8238A22C: B1630012  sth r11, 0x12(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(18 as u32), ctx.r[11].u16 ) };
	// 8238A230: A17F0018  lhz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8238A234: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8238A238: A17F001A  lhz r11, 0x1a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(26 as u32) ) } as u64;
	// 8238A23C: A13F0018  lhz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8238A240: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8238A244: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8238A248: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8238A24C: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8238A250: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8238A254: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8238A258: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8238A25C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8238A260: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8238A264: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8238A268: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8238A26C: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8238A270: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8238A274: C01F0024  lfs f0, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A278: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8238A27C: A17F0018  lhz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8238A280: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8238A284: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8238A288: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238A28C: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8238A290: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8238A294: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8238A298: C00BBA38  lfs f0, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A29C: EDA06824  fdivs f13, f0, f13
	ctx.f[13].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8238A2A0: D1A30024  stfs f13, 0x24(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8238A2A4: A17F001A  lhz r11, 0x1a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(26 as u32) ) } as u64;
	// 8238A2A8: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8238A2AC: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8238A2B0: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8238A2B4: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8238A2B8: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8238A2BC: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8238A2C0: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8238A2C4: C01F0020  lfs f0, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A2C8: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8238A2CC: D0030034  stfs f0, 0x34(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 8238A2D0: C01F0028  lfs f0, 0x28(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A2D4: D0030038  stfs f0, 0x38(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 8238A2D8: A17F0012  lhz r11, 0x12(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 8238A2DC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8238A2E0: A17F0014  lhz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8238A2E4: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8238A2E8: B1610060  sth r11, 0x60(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u16 ) };
	// 8238A2EC: A17F0010  lhz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238A2F0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238A2F4: 41820038  beq 0x8238a32c
	if ctx.cr[0].eq {
	pc = 0x8238A32C; continue 'dispatch;
	}
	// 8238A2F8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8238A2FC: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238A300: 3D408239  lis r10, -0x7dc7
	ctx.r[10].s64 = -2110193664;
	// 8238A304: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8238A308: 388A9A78  addi r4, r10, -0x6588
	ctx.r[4].s64 = ctx.r[10].s64 + -25992;
	// 8238A30C: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8238A310: 48012041  bl 0x8239c350
	ctx.lr = 0x8238A314;
	sub_8239C350(ctx, base);
	// 8238A314: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 8238A318: A15F0010  lhz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238A31C: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 8238A320: 557D043E  clrlwi r29, r11, 0x10
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8238A324: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8238A328: 4198FFD4  blt cr6, 0x8238a2fc
	if ctx.cr[6].lt {
	pc = 0x8238A2FC; continue 'dispatch;
	}
	// 8238A32C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8238A330: 481AADD8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238A338 size=260
    let mut pc: u32 = 0x8238A338;
    'dispatch: loop {
        match pc {
            0x8238A338 => {
    //   block [0x8238A338..0x8238A43C)
	// 8238A338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238A33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238A340: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238A344: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238A348: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238A34C: C01F0030  lfs f0, 0x30(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A350: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8238A354: C1BF002C  lfs f13, 0x2c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A358: EC0D007A  fmadds f0, f13, f1, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 8238A35C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8238A360: D01F0030  stfs f0, 0x30(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8238A364: 419A0030  beq cr6, 0x8238a394
	if ctx.cr[6].eq {
	pc = 0x8238A394; continue 'dispatch;
	}
	// 8238A368: C1BF0020  lfs f13, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A36C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8238A370: 41980050  blt cr6, 0x8238a3c0
	if ctx.cr[6].lt {
	pc = 0x8238A3C0; continue 'dispatch;
	}
	// 8238A374: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238A378: D01F0030  stfs f0, 0x30(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8238A37C: 4BFDD0CD  bl 0x82367448
	ctx.lr = 0x8238A380;
	sub_82367448(ctx, base);
	// 8238A380: C01F0038  lfs f0, 0x38(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A384: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8238A388: 40980038  bge cr6, 0x8238a3c0
	if !ctx.cr[6].lt {
	pc = 0x8238A3C0; continue 'dispatch;
	}
	// 8238A38C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8238A390: 4800002C  b 0x8238a3bc
	pc = 0x8238A3BC; continue 'dispatch;
	// 8238A394: C1BF001C  lfs f13, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A398: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8238A39C: 41980024  blt cr6, 0x8238a3c0
	if ctx.cr[6].lt {
	pc = 0x8238A3C0; continue 'dispatch;
	}
	// 8238A3A0: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238A3A4: D01F0030  stfs f0, 0x30(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8238A3A8: 4BFDD0A1  bl 0x82367448
	ctx.lr = 0x8238A3AC;
	sub_82367448(ctx, base);
	// 8238A3AC: C01F0038  lfs f0, 0x38(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A3B0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8238A3B4: 4198000C  blt cr6, 0x8238a3c0
	if ctx.cr[6].lt {
	pc = 0x8238A3C0; continue 'dispatch;
	}
	// 8238A3B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8238A3BC: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8238A3C0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8238A3C4: C01F0030  lfs f0, 0x30(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A3C8: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 8238A3CC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8238A3D0: C19F0028  lfs f12, 0x28(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238A3D4: C1BF0024  lfs f13, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A3D8: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 8238A3DC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8238A3E0: 7D2A5B96  divwu r9, r10, r11
	ctx.r[9].u32 = ctx.r[10].u32 / ctx.r[11].u32;
	// 8238A3E4: 7D0A5B96  divwu r8, r10, r11
	ctx.r[8].u32 = ctx.r[10].u32 / ctx.r[11].u32;
	// 8238A3E8: 7D6959D6  mullw r11, r9, r11
	ctx.r[11].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8238A3EC: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8238A3F0: 790A0020  clrldi r10, r8, 0x20
	ctx.r[10].u64 = ctx.r[8].u64 & 0x00000000FFFFFFFFu64;
	// 8238A3F4: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 8238A3F8: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 8238A3FC: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8238A400: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8238A404: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8238A408: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8238A40C: EC000332  fmuls f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 8238A410: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8238A414: C9610058  lfd f11, 0x58(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8238A418: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 8238A41C: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 8238A420: EC0B0372  fmuls f0, f11, f13
	ctx.f[0].f64 = (((ctx.f[11].f64 * ctx.f[13].f64) as f32) as f64);
	// 8238A424: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8238A428: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238A42C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238A430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238A434: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238A438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238A440 size=104
    let mut pc: u32 = 0x8238A440;
    'dispatch: loop {
        match pc {
            0x8238A440 => {
    //   block [0x8238A440..0x8238A4A8)
	// 8238A440: 8963001D  lbz r11, 0x1d(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(29 as u32) ) } as u64;
	// 8238A444: 8943001C  lbz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8238A448: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238A44C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238A450: 41820058  beq 0x8238a4a8
	if ctx.cr[0].eq {
		sub_8238A4A8(ctx, base);
		return;
	}
	// 8238A454: C00BBA38  lfs f0, -0x45c8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A458: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238A45C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8238A460: 418200D8  beq 0x8238a538
	if ctx.cr[0].eq {
		sub_8238A51C(ctx, base);
		return;
	}
	// 8238A464: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8238A468: 81030018  lwz r8, 0x18(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8238A46C: 3D4082B6  lis r10, -0x7d4a
	ctx.r[10].s64 = -2102001664;
	// 8238A470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8238A474: C0091FF8  lfs f0, 0x1ff8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A478: 814ABA80  lwz r10, -0x4580(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17792 as u32) ) } as u64;
	// 8238A47C: 7D2858AE  lbzx r9, r8, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8238A480: 5529103E  rotlwi r9, r9, 2
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(2)) as u64;
	// 8238A484: 7DA9542E  lfsx f13, r9, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A488: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8238A48C: 409800A8  bge cr6, 0x8238a534
	if !ctx.cr[6].lt {
		sub_8238A51C(ctx, base);
		return;
	}
	// 8238A490: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8238A494: 8923001C  lbz r9, 0x1c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8238A498: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8238A49C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8238A4A0: 4198FFDC  blt cr6, 0x8238a47c
	if ctx.cr[6].lt {
	pc = 0x8238A47C; continue 'dispatch;
	}
	// 8238A4A4: 48000094  b 0x8238a538
	sub_8238A51C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238A4A8 size=116
    let mut pc: u32 = 0x8238A4A8;
    'dispatch: loop {
        match pc {
            0x8238A4A8 => {
    //   block [0x8238A4A8..0x8238A51C)
	// 8238A4A8: C18BBA38  lfs f12, -0x45c8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238A4AC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238A4B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8238A4B4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238A4B8: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A4BC: 41820054  beq 0x8238a510
	if ctx.cr[0].eq {
	pc = 0x8238A510; continue 'dispatch;
	}
	// 8238A4C0: 3D4082B6  lis r10, -0x7d4a
	ctx.r[10].s64 = -2102001664;
	// 8238A4C4: 81230018  lwz r9, 0x18(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8238A4C8: 8903001C  lbz r8, 0x1c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8238A4CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8238A4D0: 814ABA80  lwz r10, -0x4580(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17792 as u32) ) } as u64;
	// 8238A4D4: 7CCB48AE  lbzx r6, r11, r9
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8238A4D8: 54C6103E  rotlwi r6, r6, 2
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(2)) as u64;
	// 8238A4DC: 7C06542E  lfsx f0, r6, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A4E0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8238A4E4: 41980014  blt cr6, 0x8238a4f8
	if ctx.cr[6].lt {
	pc = 0x8238A4F8; continue 'dispatch;
	}
	// 8238A4E8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8238A4EC: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 8238A4F0: 40990008  ble cr6, 0x8238a4f8
	if !ctx.cr[6].gt {
	pc = 0x8238A4F8; continue 'dispatch;
	}
	// 8238A4F4: FD800090  fmr f12, f0
	ctx.f[12].f64 = ctx.f[0].f64;
	// 8238A4F8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8238A4FC: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8238A500: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8238A504: 4198FFD0  blt cr6, 0x8238a4d4
	if ctx.cr[6].lt {
	pc = 0x8238A4D4; continue 'dispatch;
	}
	// 8238A508: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 8238A50C: 409A0010  bne cr6, 0x8238a51c
	if !ctx.cr[6].eq {
		sub_8238A51C(ctx, base);
		return;
	}
	// 8238A510: D1A3000C  stfs f13, 0xc(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8238A514: D1A30014  stfs f13, 0x14(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8238A518: 48000020  b 0x8238a538
	sub_8238A51C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A51C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238A51C size=52
    let mut pc: u32 = 0x8238A51C;
    'dispatch: loop {
        match pc {
            0x8238A51C => {
    //   block [0x8238A51C..0x8238A550)
	// 8238A51C: C0030014  lfs f0, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A520: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8238A524: ED8C0028  fsubs f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 8238A528: C1ABD560  lfs f13, -0x2aa0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10912 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A52C: EC0C037A  fmadds f0, f12, f13, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 8238A530: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8238A534: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8238A538: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238A53C: C003000C  lfs f0, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A540: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A544: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238A548: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8238A54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238A550 size=36
    let mut pc: u32 = 0x8238A550;
    'dispatch: loop {
        match pc {
            0x8238A550 => {
    //   block [0x8238A550..0x8238A574)
	// 8238A550: 39640014  addi r11, r4, 0x14
	ctx.r[11].s64 = ctx.r[4].s64 + 20;
	// 8238A554: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8238A558: 89640013  lbz r11, 0x13(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(19 as u32) ) } as u64;
	// 8238A55C: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8238A560: 89640011  lbz r11, 0x11(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(17 as u32) ) } as u64;
	// 8238A564: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 8238A568: 89640012  lbz r11, 0x12(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(18 as u32) ) } as u64;
	// 8238A56C: 9963001E  stb r11, 0x1e(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(30 as u32), ctx.r[11].u8 ) };
	// 8238A570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238A578 size=176
    let mut pc: u32 = 0x8238A578;
    'dispatch: loop {
        match pc {
            0x8238A578 => {
    //   block [0x8238A578..0x8238A628)
	// 8238A578: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 8238A57C: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8238A580: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 8238A584: C1630014  lfs f11, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8238A588: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238A628 size=44
    let mut pc: u32 = 0x8238A628;
    'dispatch: loop {
        match pc {
            0x8238A628 => {
    //   block [0x8238A628..0x8238A654)
	// 8238A628: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8238A62C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8238A630: 419AFFEC  beq cr6, 0x8238a61c
	if ctx.cr[6].eq {
		sub_8238A578(ctx, base);
		return;
	}
	// 8238A634: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238A638: C00BBA38  lfs f0, -0x45c8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A63C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238A640: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8238A644: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A648: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238A64C: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8238A650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238A658 size=52
    let mut pc: u32 = 0x8238A658;
    'dispatch: loop {
        match pc {
            0x8238A658 => {
    //   block [0x8238A658..0x8238A68C)
	// 8238A658: 89240011  lbz r9, 0x11(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(17 as u32) ) } as u64;
	// 8238A65C: 39640020  addi r11, r4, 0x20
	ctx.r[11].s64 = ctx.r[4].s64 + 32;
	// 8238A660: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 8238A664: 9123001C  stw r9, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 8238A668: C0040018  lfs f0, 0x18(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A66C: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8238A670: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A674: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8238A678: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8238A67C: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8238A680: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8238A684: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8238A688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238A690 size=156
    let mut pc: u32 = 0x8238A690;
    'dispatch: loop {
        match pc {
            0x8238A690 => {
    //   block [0x8238A690..0x8238A72C)
	// 8238A690: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8238A694: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8238A698: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 8238A69C: A0E3001E  lhz r7, 0x1e(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(30 as u32) ) } as u64;
	// 8238A6A0: 3D4082B6  lis r10, -0x7d4a
	ctx.r[10].s64 = -2102001664;
	// 8238A6A4: 28070000  cmplwi r7, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238A6A8: 394AB660  addi r10, r10, -0x49a0
	ctx.r[10].s64 = ctx.r[10].s64 + -18848;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A72C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238A72C size=24
    let mut pc: u32 = 0x8238A72C;
    'dispatch: loop {
        match pc {
            0x8238A72C => {
    //   block [0x8238A72C..0x8238A744)
	// 8238A72C: C1830018  lfs f12, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238A730: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8238A734: 40980010  bge cr6, 0x8238a744
	if !ctx.cr[6].lt {
		sub_8238A744(ctx, base);
		return;
	}
	// 8238A738: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238A73C: C00BBA38  lfs f0, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A740: 48000034  b 0x8238a774
	sub_8238A750(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A744(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238A744 size=12
    let mut pc: u32 = 0x8238A744;
    'dispatch: loop {
        match pc {
            0x8238A744 => {
    //   block [0x8238A744..0x8238A750)
	// 8238A744: EC0D0028  fsubs f0, f13, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8238A748: EDAD6028  fsubs f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 8238A74C: 48000024  b 0x8238a770
	sub_8238A750(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238A750 size=60
    let mut pc: u32 = 0x8238A750;
    'dispatch: loop {
        match pc {
            0x8238A750 => {
    //   block [0x8238A750..0x8238A78C)
	// 8238A750: C1830014  lfs f12, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238A754: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8238A758: 4098FFE0  bge cr6, 0x8238a738
	if !ctx.cr[6].lt {
		sub_8238A72C(ctx, base);
		return;
	}
	// 8238A75C: C1A30018  lfs f13, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A760: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8238A764: 4198FFBC  blt cr6, 0x8238a720
	if ctx.cr[6].lt {
		sub_8238A690(ctx, base);
		return;
	}
	// 8238A768: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238A76C: EDAC6828  fsubs f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238A770: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8238A774: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238A778: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8238A77C: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A780: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238A784: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8238A788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238A790 size=60
    let mut pc: u32 = 0x8238A790;
    'dispatch: loop {
        match pc {
            0x8238A790 => {
    //   block [0x8238A790..0x8238A7CC)
	// 8238A790: 89240011  lbz r9, 0x11(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(17 as u32) ) } as u64;
	// 8238A794: 39640020  addi r11, r4, 0x20
	ctx.r[11].s64 = ctx.r[4].s64 + 32;
	// 8238A798: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 8238A79C: B123001C  sth r9, 0x1c(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[9].u16 ) };
	// 8238A7A0: 89240012  lbz r9, 0x12(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(18 as u32) ) } as u64;
	// 8238A7A4: B123001E  sth r9, 0x1e(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(30 as u32), ctx.r[9].u16 ) };
	// 8238A7A8: C0040018  lfs f0, 0x18(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A7AC: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8238A7B0: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A7B4: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8238A7B8: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8238A7BC: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8238A7C0: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8238A7C4: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8238A7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238A7D0 size=44
    let mut pc: u32 = 0x8238A7D0;
    'dispatch: loop {
        match pc {
            0x8238A7D0 => {
    //   block [0x8238A7D0..0x8238A7FC)
	// 8238A7D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8238A7D4: 386B0050  addi r3, r11, 0x50
	ctx.r[3].s64 = ctx.r[11].s64 + 80;
	// 8238A7D8: C00B0094  lfs f0, 0x94(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(148 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A7DC: C1AB0090  lfs f13, 0x90(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A7E0: EC0D007A  fmadds f0, f13, f1, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 8238A7E4: FDA0065E  fctidz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 8238A7E8: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8238A7EC: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8238A7F0: EC206828  fsubs f1, f0, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238A7F4: D02B0094  stfs f1, 0x94(r11)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8238A7F8: 4BFDD7C0  b 0x82367fb8
	sub_82367FB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238A800 size=68
    let mut pc: u32 = 0x8238A800;
    'dispatch: loop {
        match pc {
            0x8238A800 => {
    //   block [0x8238A800..0x8238A844)
	// 8238A800: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 8238A804: 39630050  addi r11, r3, 0x50
	ctx.r[11].s64 = ctx.r[3].s64 + 80;
	// 8238A808: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 8238A80C: C0040010  lfs f0, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238A810: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 8238A814: D0030090  stfs f0, 0x90(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8238A818: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8238A81C: 3D403F80  lis r10, 0x3f80
	ctx.r[10].s64 = 1065353216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238A848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238A848 size=528
    let mut pc: u32 = 0x8238A848;
    'dispatch: loop {
        match pc {
            0x8238A848 => {
    //   block [0x8238A848..0x8238AA58)
	// 8238A848: 3D2082B6  lis r9, -0x7d4a
	ctx.r[9].s64 = -2102001664;
	// 8238A84C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238A850: 8141FFDC  lwz r10, -0x24(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-36 as u32) ) } as u64;
	// 8238A854: 11A1038C  vspltisw v13, 1
	for i in 0..4 {
		ctx.v[13].u32[i] = 1;
	}
	// 8238A858: 3929BA70  addi r9, r9, -0x4590
	ctx.r[9].s64 = ctx.r[9].s64 + -17808;
	// 8238A85C: C1AB0030  lfs f13, 0x30(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238A860: 10016B4A  vcfsx v0, v13, 1
	ctx.fpscr.enable_flush_mode_unconditional();
	let scale = f32::from_bits(((127u32 - (1 as u32)) << 23));
	for i in 0..4 {
		ctx.v[0].f32[i] = (ctx.v[13].s32[i] as f32) * scale;
	}
	// 8238A864: C18B0034  lfs f12, 0x34(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238A868: 11A06B4A  vcfsx v13, v13, 0
	ctx.fpscr.enable_flush_mode_unconditional();
	let scale = f32::from_bits(((127u32 - (0 as u32)) << 23));
	for i in 0..4 {
		ctx.v[13].f32[i] = (ctx.v[13].s32[i] as f32) * scale;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238AA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238AA58 size=156
    let mut pc: u32 = 0x8238AA58;
    'dispatch: loop {
        match pc {
            0x8238AA58 => {
    //   block [0x8238AA58..0x8238AAF4)
	// 8238AA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238AA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238AA60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238AA64: DBA1FFD8  stfd f29, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[29].u64 ) };
	// 8238AA68: DBC1FFE0  stfd f30, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 8238AA6C: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8238AA70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238AA74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238AA78: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238AA7C: C01F0014  lfs f0, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238AA80: EC01002A  fadds f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8238AA84: FDA0065E  fctidz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 8238AA88: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8238AA8C: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8238AA90: EFE06828  fsubs f31, f0, f13
	ctx.f[31].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238AA94: C00B2608  lfs f0, 0x2608(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9736 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238AA98: D3FF0014  stfs f31, 0x14(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8238AA9C: EC3F0032  fmuls f1, f31, f0
	ctx.f[1].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238AAA0: 4BD92A29  bl 0x8211d4c8
	ctx.lr = 0x8238AAA4;
	sub_8211D4C8(ctx, base);
	// 8238AAA4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238AAA8: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8238AAAC: C3BF0018  lfs f29, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8238AAB0: C00B2490  lfs f0, 0x2490(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9360 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238AAB4: EC3F0032  fmuls f1, f31, f0
	ctx.f[1].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238AAB8: 4BD92A11  bl 0x8211d4c8
	ctx.lr = 0x8238AABC;
	sub_8211D4C8(ctx, base);
	// 8238AABC: C01F001C  lfs f0, 0x1c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238AAC0: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238AAC4: C1BF0024  lfs f13, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238AAC8: EC1E077A  fmadds f0, f30, f29, f0
	ctx.f[0].f64 = (((ctx.f[30].f64 * ctx.f[29].f64 + ctx.f[0].f64) as f32) as f64);
	// 8238AACC: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 8238AAD0: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8238AAD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8238AAD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238AADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238AAE0: CBA1FFD8  lfd f29, -0x28(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8238AAE4: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8238AAE8: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238AAEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238AAF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238AAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238AAF8 size=12
    let mut pc: u32 = 0x8238AAF8;
    'dispatch: loop {
        match pc {
            0x8238AAF8 => {
    //   block [0x8238AAF8..0x8238AB04)
	// 8238AAF8: C0030020  lfs f0, 0x20(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238AAFC: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8238AB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238AB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238AB08 size=60
    let mut pc: u32 = 0x8238AB08;
    'dispatch: loop {
        match pc {
            0x8238AB08 => {
    //   block [0x8238AB08..0x8238AB44)
	// 8238AB08: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238AB0C: C0040014  lfs f0, 0x14(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238AB10: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8238AB14: C1830018  lfs f12, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238AB18: C0040018  lfs f0, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238AB1C: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8238AB20: C1ABBA38  lfs f13, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238AB24: EDAD6028  fsubs f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 8238AB28: C183001C  lfs f12, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238AB2C: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238AB30: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8238AB34: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8238AB38: EDAD6028  fsubs f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 8238AB3C: D1A30024  stfs f13, 0x24(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8238AB40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238AB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238AB48 size=68
    let mut pc: u32 = 0x8238AB48;
    'dispatch: loop {
        match pc {
            0x8238AB48 => {
    //   block [0x8238AB48..0x8238AB8C)
	// 8238AB48: C003008C  lfs f0, 0x8c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238AB4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238AB50: EC01002A  fadds f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8238AB54: C1830090  lfs f12, 0x90(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238AB58: D003008C  stfs f0, 0x8c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 8238AB5C: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238AB60: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8238AB64: 40990008  ble cr6, 0x8238ab6c
	if !ctx.cr[6].gt {
	pc = 0x8238AB6C; continue 'dispatch;
	}
	// 8238AB68: D1A3008C  stfs f13, 0x8c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 8238AB6C: C003008C  lfs f0, 0x8c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238AB70: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8238AB74: 41980038  blt cr6, 0x8238abac
	if ctx.cr[6].lt {
		sub_8238ABA4(ctx, base);
		return;
	}
	// 8238AB78: C1A30094  lfs f13, 0x94(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238AB7C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8238AB80: 4098000C  bge cr6, 0x8238ab8c
	if !ctx.cr[6].lt {
		sub_8238AB8C(ctx, base);
		return;
	}
	// 8238AB84: EDA06824  fdivs f13, f0, f13
	ctx.f[13].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8238AB88: 48000024  b 0x8238abac
	sub_8238ABA4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238AB8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238AB8C size=24
    let mut pc: u32 = 0x8238AB8C;
    'dispatch: loop {
        match pc {
            0x8238AB8C => {
    //   block [0x8238AB8C..0x8238ABA4)
	// 8238AB8C: ED6C6828  fsubs f11, f12, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238AB90: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 8238AB94: 40990010  ble cr6, 0x8238aba4
	if !ctx.cr[6].gt {
		sub_8238ABA4(ctx, base);
		return;
	}
	// 8238AB98: ED8C0028  fsubs f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 8238AB9C: EDAC6824  fdivs f13, f12, f13
	ctx.f[13].f64 = ((ctx.f[12].f64 / ctx.f[13].f64) as f32) as f64;
	// 8238ABA0: 4800000C  b 0x8238abac
	sub_8238ABA4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238ABA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238ABA4 size=88
    let mut pc: u32 = 0x8238ABA4;
    'dispatch: loop {
        match pc {
            0x8238ABA4 => {
    //   block [0x8238ABA4..0x8238ABFC)
	// 8238ABA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238ABA8: C1ABBA38  lfs f13, -0x45c8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238ABAC: 39630050  addi r11, r3, 0x50
	ctx.r[11].s64 = ctx.r[3].s64 + 80;
	// 8238ABB0: D1A30088  stfs f13, 0x88(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 8238ABB4: C1A30070  lfs f13, 0x70(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238ABB8: 39400060  li r10, 0x60
	ctx.r[10].s64 = 96;
	// 8238ABBC: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238ABC0: D1AB0000  stfs f13, 0(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8238ABC4: C1A30074  lfs f13, 0x74(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238ABC8: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238ABCC: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8238ABD0: C1A30078  lfs f13, 0x78(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238ABD4: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238ABD8: D1AB0008  stfs f13, 8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8238ABDC: C1A3007C  lfs f13, 0x7c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238ABE0: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238ABE4: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238AC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238AC00 size=156
    let mut pc: u32 = 0x8238AC00;
    'dispatch: loop {
        match pc {
            0x8238AC00 => {
    //   block [0x8238AC00..0x8238AC9C)
	// 8238AC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238AC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238AC08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238AC0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238AC10: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 8238AC14: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8238AC18: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 8238AC1C: 38A80010  addi r5, r8, 0x10
	ctx.r[5].s64 = ctx.r[8].s64 + 16;
	// 8238AC20: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8238AC24: C00BBA38  lfs f0, -0x45c8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238AC28: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8238AC2C: 3D603F80  lis r11, 0x3f80
	ctx.r[11].s64 = 1065353216;
	// 8238AC30: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238ACA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238ACA0 size=16
    let mut pc: u32 = 0x8238ACA0;
    'dispatch: loop {
        match pc {
            0x8238ACA0 => {
    //   block [0x8238ACA0..0x8238ACB0)
	// 8238ACA0: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 8238ACA4: 409A000C  bne cr6, 0x8238acb0
	if !ctx.cr[6].eq {
		sub_8238ACB0(ctx, base);
		return;
	}
	// 8238ACA8: 38630084  addi r3, r3, 0x84
	ctx.r[3].s64 = ctx.r[3].s64 + 132;
	// 8238ACAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238ACB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238ACB0 size=12
    let mut pc: u32 = 0x8238ACB0;
    'dispatch: loop {
        match pc {
            0x8238ACB0 => {
    //   block [0x8238ACB0..0x8238ACBC)
	// 8238ACB0: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 8238ACB4: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 8238ACB8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238ACBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238ACBC size=8
    let mut pc: u32 = 0x8238ACBC;
    'dispatch: loop {
        match pc {
            0x8238ACBC => {
    //   block [0x8238ACBC..0x8238ACC4)
	// 8238ACBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8238ACC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238ACC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238ACC8 size=32
    let mut pc: u32 = 0x8238ACC8;
    'dispatch: loop {
        match pc {
            0x8238ACC8 => {
    //   block [0x8238ACC8..0x8238ACE8)
	// 8238ACC8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238ACCC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238ACD0: C0030098  lfs f0, 0x98(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238ACD4: D003008C  stfs f0, 0x8c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 8238ACD8: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8238ACDC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238ACE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238ACE4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238ACE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238ACE8 size=80
    let mut pc: u32 = 0x8238ACE8;
    'dispatch: loop {
        match pc {
            0x8238ACE8 => {
    //   block [0x8238ACE8..0x8238AD38)
	// 8238ACE8: C0040018  lfs f0, 0x18(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238ACEC: 39640020  addi r11, r4, 0x20
	ctx.r[11].s64 = ctx.r[4].s64 + 32;
	// 8238ACF0: D0030094  stfs f0, 0x94(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8238ACF4: 39430060  addi r10, r3, 0x60
	ctx.r[10].s64 = ctx.r[3].s64 + 96;
	// 8238ACF8: C0040014  lfs f0, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238ACFC: 39240030  addi r9, r4, 0x30
	ctx.r[9].s64 = ctx.r[4].s64 + 48;
	// 8238AD00: D0030090  stfs f0, 0x90(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8238AD04: 39030070  addi r8, r3, 0x70
	ctx.r[8].s64 = ctx.r[3].s64 + 112;
	// 8238AD08: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238AD0C: D0030098  stfs f0, 0x98(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8238AD10: D003008C  stfs f0, 0x8c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 8238AD14: E8EB0000  ld r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8238AD18: F8EA0000  std r7, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u64 ) };
	// 8238AD1C: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8238AD20: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8238AD24: E9690000  ld r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 8238AD28: F9680000  std r11, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8238AD2C: E9690008  ld r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	// 8238AD30: F9680008  std r11, 8(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8238AD34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238AD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238AD38 size=388
    let mut pc: u32 = 0x8238AD38;
    'dispatch: loop {
        match pc {
            0x8238AD38 => {
    //   block [0x8238AD38..0x8238AEBC)
	// 8238AD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238AD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238AD40: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238AD44: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8238AD48: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8238AD4C: 80880020  lwz r4, 0x20(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 8238AD50: 4BFDCF39  bl 0x82367c88
	ctx.lr = 0x8238AD54;
	sub_82367C88(ctx, base);
	// 8238AD54: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 8238AD58: 81280014  lwz r9, 0x14(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 8238AD5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8238AD60: 396BBA70  addi r11, r11, -0x4590
	ctx.r[11].s64 = ctx.r[11].s64 + -17808;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238AEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238AEC0 size=40
    let mut pc: u32 = 0x8238AEC0;
    'dispatch: loop {
        match pc {
            0x8238AEC0 => {
    //   block [0x8238AEC0..0x8238AEE8)
	// 8238AEC0: 89640011  lbz r11, 0x11(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(17 as u32) ) } as u64;
	// 8238AEC4: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 8238AEC8: 89240012  lbz r9, 0x12(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(18 as u32) ) } as u64;
	// 8238AECC: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 8238AED0: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 8238AED4: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8238AED8: 89640013  lbz r11, 0x13(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(19 as u32) ) } as u64;
	// 8238AEDC: 9143001C  stw r10, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8238AEE0: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8238AEE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238AEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238AEE8 size=444
    let mut pc: u32 = 0x8238AEE8;
    'dispatch: loop {
        match pc {
            0x8238AEE8 => {
    //   block [0x8238AEE8..0x8238B0A4)
	// 8238AEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238AEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238AEF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238AEF4: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 8238AEF8: 481AB0F1  bl 0x82535fe8
	ctx.lr = 0x8238AEFC;
	sub_82535FB0(ctx, base);
	// 8238AEFC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238AF00: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238AF04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238AF08: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238AF0C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8238AF10: 4099015C  ble cr6, 0x8238b06c
	if !ctx.cr[6].gt {
	pc = 0x8238B06C; continue 'dispatch;
	}
	// 8238AF14: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238AF18: C15F00C0  lfs f10, 0xc0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8238AF1C: C17F00D4  lfs f11, 0xd4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8238AF20: C13F00C4  lfs f9, 0xc4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8238AF24: ECEB0072  fmuls f7, f11, f1
	ctx.f[7].f64 = (((ctx.f[11].f64 * ctx.f[1].f64) as f32) as f64);
	// 8238AF28: C11F00C8  lfs f8, 0xc8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 8238AF2C: C0DF00D8  lfs f6, 0xd8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 8238AF30: C00B2054  lfs f0, 0x2054(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8276 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238AF34: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238AF38: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238AF3C: C0BF00CC  lfs f5, 0xcc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 8238AF40: C07F00D0  lfs f3, 0xd0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8238AF44: ECC60072  fmuls f6, f6, f1
	ctx.f[6].f64 = (((ctx.f[6].f64 * ctx.f[1].f64) as f32) as f64);
	// 8238AF48: C09F00DC  lfs f4, 0xdc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 8238AF4C: C05F00E0  lfs f2, 0xe0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8238AF50: EC840072  fmuls f4, f4, f1
	ctx.f[4].f64 = (((ctx.f[4].f64 * ctx.f[1].f64) as f32) as f64);
	// 8238AF54: C18B20B0  lfs f12, 0x20b0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8368 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238AF58: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238AF5C: EC420072  fmuls f2, f2, f1
	ctx.f[2].f64 = (((ctx.f[2].f64 * ctx.f[1].f64) as f32) as f64);
	// 8238AF60: C3FF00B8  lfs f31, 0xb8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8238AF64: C03F00B4  lfs f1, 0xb4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8238AF68: C3DF00BC  lfs f30, 0xbc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8238AF6C: C1AB2238  lfs f13, 0x2238(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8760 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238AF70: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238AF74: EC0002B2  fmuls f0, f0, f10
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[10].f64) as f32) as f64);
	// 8238AF78: C15F00B0  lfs f10, 0xb0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8238AF7C: C16B21A4  lfs f11, 0x21a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8612 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8238AF80: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238AF84: ED290032  fmuls f9, f9, f0
	ctx.f[9].f64 = (((ctx.f[9].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238AF88: ED080032  fmuls f8, f8, f0
	ctx.f[8].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238AF8C: ECA50032  fmuls f5, f5, f0
	ctx.f[5].f64 = (((ctx.f[5].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238AF90: EC030032  fmuls f0, f3, f0
	ctx.f[0].f64 = (((ctx.f[3].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238AF94: ED893B3A  fmadds f12, f9, f12, f7
	ctx.f[12].f64 = (((ctx.f[9].f64 * ctx.f[12].f64 + ctx.f[7].f64) as f32) as f64);
	// 8238AF98: ED28337A  fmadds f9, f8, f13, f6
	ctx.f[9].f64 = (((ctx.f[8].f64 * ctx.f[13].f64 + ctx.f[6].f64) as f32) as f64);
	// 8238AF9C: ED05237A  fmadds f8, f5, f13, f4
	ctx.f[8].f64 = (((ctx.f[5].f64 * ctx.f[13].f64 + ctx.f[4].f64) as f32) as f64);
	// 8238AFA0: ED6012FA  fmadds f11, f0, f11, f2
	ctx.f[11].f64 = (((ctx.f[0].f64 * ctx.f[11].f64 + ctx.f[2].f64) as f32) as f64);
	// 8238AFA4: EC0C502A  fadds f0, f12, f10
	ctx.f[0].f64 = ((ctx.f[12].f64 + ctx.f[10].f64) as f32) as f64;
	// 8238AFA8: EDA9082A  fadds f13, f9, f1
	ctx.f[13].f64 = ((ctx.f[9].f64 + ctx.f[1].f64) as f32) as f64;
	// 8238AFAC: ED88F82A  fadds f12, f8, f31
	ctx.f[12].f64 = ((ctx.f[8].f64 + ctx.f[31].f64) as f32) as f64;
	// 8238AFB0: C3EB2490  lfs f31, 0x2490(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9360 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8238AFB4: ED6BF02A  fadds f11, f11, f30
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[30].f64) as f32) as f64;
	// 8238AFB8: FD40065E  fctidz f10, f0
	ctx.f[10].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 8238AFBC: FD206E5E  fctidz f9, f13
	ctx.f[9].s64 = if ctx.f[13].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[13].f64.trunc() as i64 };
	// 8238AFC0: FD00665E  fctidz f8, f12
	ctx.f[8].s64 = if ctx.f[12].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[12].f64.trunc() as i64 };
	// 8238AFC4: FCE05E5E  fctidz f7, f11
	ctx.f[7].s64 = if ctx.f[11].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[11].f64.trunc() as i64 };
	// 8238AFC8: FD40569C  fcfid f10, f10
	ctx.f[10].f64 = (ctx.f[10].s64 as f64);
	// 8238AFCC: FD204E9C  fcfid f9, f9
	ctx.f[9].f64 = (ctx.f[9].s64 as f64);
	// 8238AFD0: FD00469C  fcfid f8, f8
	ctx.f[8].f64 = (ctx.f[8].s64 as f64);
	// 8238AFD4: FCE03E9C  fcfid f7, f7
	ctx.f[7].f64 = (ctx.f[7].s64 as f64);
	// 8238AFD8: FD405018  frsp f10, f10
	ctx.f[10].f64 = (ctx.f[10].f64 as f32) as f64;
	// 8238AFDC: FD204818  frsp f9, f9
	ctx.f[9].f64 = (ctx.f[9].f64 as f32) as f64;
	// 8238AFE0: FD004018  frsp f8, f8
	ctx.f[8].f64 = (ctx.f[8].f64 as f32) as f64;
	// 8238AFE4: FCE03818  frsp f7, f7
	ctx.f[7].f64 = (ctx.f[7].f64 as f32) as f64;
	// 8238AFE8: EC005028  fsubs f0, f0, f10
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[10].f64) as f32) as f64);
	// 8238AFEC: D01F00B0  stfs f0, 0xb0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), tmp.u32 ) };
	// 8238AFF0: EFCD4828  fsubs f30, f13, f9
	ctx.f[30].f64 = (((ctx.f[13].f64 - ctx.f[9].f64) as f32) as f64);
	// 8238AFF4: D3DF00B4  stfs f30, 0xb4(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 8238AFF8: EFAC4028  fsubs f29, f12, f8
	ctx.f[29].f64 = (((ctx.f[12].f64 - ctx.f[8].f64) as f32) as f64);
	// 8238AFFC: D3BF00B8  stfs f29, 0xb8(r31)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), tmp.u32 ) };
	// 8238B000: EF8B3828  fsubs f28, f11, f7
	ctx.f[28].f64 = (((ctx.f[11].f64 - ctx.f[7].f64) as f32) as f64);
	// 8238B004: D39F00BC  stfs f28, 0xbc(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 8238B008: EC2007F2  fmuls f1, f0, f31
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 8238B00C: 4BD924BD  bl 0x8211d4c8
	ctx.lr = 0x8238B010;
	sub_8211D4C8(ctx, base);
	// 8238B010: FDA00890  fmr f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = ctx.f[1].f64;
	// 8238B014: 3D608286  lis r11, -0x7d7a
	ctx.r[11].s64 = -2105147392;
	// 8238B018: C19F0090  lfs f12, 0x90(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238B01C: EC3E07F2  fmuls f1, f30, f31
	ctx.f[1].f64 = (((ctx.f[30].f64 * ctx.f[31].f64) as f32) as f64);
	// 8238B020: C00BD5BC  lfs f0, -0x2a44(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10820 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B024: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238B028: EC000332  fmuls f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 8238B02C: D01F00A0  stfs f0, 0xa0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 8238B030: 4BD92499  bl 0x8211d4c8
	ctx.lr = 0x8238B034;
	sub_8211D4C8(ctx, base);
	// 8238B034: C01F0094  lfs f0, 0x94(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B038: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238B03C: D01F00A4  stfs f0, 0xa4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 8238B040: EC3D07F2  fmuls f1, f29, f31
	ctx.f[1].f64 = (((ctx.f[29].f64 * ctx.f[31].f64) as f32) as f64);
	// 8238B044: 4BD92485  bl 0x8211d4c8
	ctx.lr = 0x8238B048;
	sub_8211D4C8(ctx, base);
	// 8238B048: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 8238B04C: C1BF0098  lfs f13, 0x98(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B050: EC3C07F2  fmuls f1, f28, f31
	ctx.f[1].f64 = (((ctx.f[28].f64 * ctx.f[31].f64) as f32) as f64);
	// 8238B054: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8238B058: D01F00A8  stfs f0, 0xa8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 8238B05C: 4BD9246D  bl 0x8211d4c8
	ctx.lr = 0x8238B060;
	sub_8211D4C8(ctx, base);
	// 8238B060: C01F009C  lfs f0, 0x9c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B064: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238B068: D01F00AC  stfs f0, 0xac(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 8238B06C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8238B070: C07F00AC  lfs f3, 0xac(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8238B074: C05F00A8  lfs f2, 0xa8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8238B078: C03F00A4  lfs f1, 0xa4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8238B07C: 4BFDD19D  bl 0x82368218
	ctx.lr = 0x8238B080;
	sub_82368218(ctx, base);
	// 8238B080: C01F00A0  lfs f0, 0xa0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B084: D01F0084  stfs f0, 0x84(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 8238B088: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8238B08C: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 8238B090: 481AAFA5  bl 0x82536034
	ctx.lr = 0x8238B094;
	sub_82535FFC(ctx, base);
	// 8238B094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238B098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238B09C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238B0A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238B0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238B0A8 size=100
    let mut pc: u32 = 0x8238B0A8;
    'dispatch: loop {
        match pc {
            0x8238B0A8 => {
    //   block [0x8238B0A8..0x8238B10C)
	// 8238B0A8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8238B0AC: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 8238B0B0: 39630050  addi r11, r3, 0x50
	ctx.r[11].s64 = ctx.r[3].s64 + 80;
	// 8238B0B4: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 8238B0B8: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 8238B0BC: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8238B0C0: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B0C4: 3D403F80  lis r10, 0x3f80
	ctx.r[10].s64 = 1065353216;
	// 8238B0C8: D00300AC  stfs f0, 0xac(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 8238B0CC: D00300A8  stfs f0, 0xa8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 8238B0D0: D00300A4  stfs f0, 0xa4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 8238B0D4: D00300A0  stfs f0, 0xa0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 8238B0D8: D00300BC  stfs f0, 0xbc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 8238B0DC: D00300B8  stfs f0, 0xb8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(184 as u32), tmp.u32 ) };
	// 8238B0E0: D00300B4  stfs f0, 0xb4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 8238B0E4: D00300B0  stfs f0, 0xb0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238B110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238B110 size=248
    let mut pc: u32 = 0x8238B110;
    'dispatch: loop {
        match pc {
            0x8238B110 => {
    //   block [0x8238B110..0x8238B208)
	// 8238B110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238B114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238B118: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238B11C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238B120: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238B124: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8238B128: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238B12C: C01E0014  lfs f0, 0x14(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B130: D01F00C0  stfs f0, 0xc0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 8238B134: C01E0018  lfs f0, 0x18(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B138: D01F0090  stfs f0, 0x90(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8238B13C: C01E001C  lfs f0, 0x1c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B140: D01F0094  stfs f0, 0x94(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8238B144: C01E0020  lfs f0, 0x20(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B148: D01F0098  stfs f0, 0x98(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8238B14C: C01E0024  lfs f0, 0x24(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B150: D01F009C  stfs f0, 0x9c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 8238B154: 897E0010  lbz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238B158: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238B15C: 40820028  bne 0x8238b184
	if !ctx.cr[0].eq {
	pc = 0x8238B184; continue 'dispatch;
	}
	// 8238B160: 4BFDC2E9  bl 0x82367448
	ctx.lr = 0x8238B164;
	sub_82367448(ctx, base);
	// 8238B164: D03F00C4  stfs f1, 0xc4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 8238B168: 4BFDC2E1  bl 0x82367448
	ctx.lr = 0x8238B16C;
	sub_82367448(ctx, base);
	// 8238B16C: D03F00C8  stfs f1, 0xc8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), tmp.u32 ) };
	// 8238B170: 4BFDC2D9  bl 0x82367448
	ctx.lr = 0x8238B174;
	sub_82367448(ctx, base);
	// 8238B174: D03F00CC  stfs f1, 0xcc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 8238B178: 4BFDC2D1  bl 0x82367448
	ctx.lr = 0x8238B17C;
	sub_82367448(ctx, base);
	// 8238B17C: D03F00D0  stfs f1, 0xd0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), tmp.u32 ) };
	// 8238B180: 4800001C  b 0x8238b19c
	pc = 0x8238B19C; continue 'dispatch;
	// 8238B184: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B188: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B18C: D01F00C4  stfs f0, 0xc4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 8238B190: D01F00C8  stfs f0, 0xc8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), tmp.u32 ) };
	// 8238B194: D01F00CC  stfs f0, 0xcc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 8238B198: D01F00D0  stfs f0, 0xd0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), tmp.u32 ) };
	// 8238B19C: 897E0010  lbz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238B1A0: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238B1A4: 41820024  beq 0x8238b1c8
	if ctx.cr[0].eq {
	pc = 0x8238B1C8; continue 'dispatch;
	}
	// 8238B1A8: C01E0028  lfs f0, 0x28(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B1AC: D01F00D4  stfs f0, 0xd4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), tmp.u32 ) };
	// 8238B1B0: C01E002C  lfs f0, 0x2c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B1B4: D01F00D8  stfs f0, 0xd8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), tmp.u32 ) };
	// 8238B1B8: C01E0030  lfs f0, 0x30(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B1BC: D01F00DC  stfs f0, 0xdc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), tmp.u32 ) };
	// 8238B1C0: C01E0034  lfs f0, 0x34(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B1C4: 48000028  b 0x8238b1ec
	pc = 0x8238B1EC; continue 'dispatch;
	// 8238B1C8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B1CC: C00B2230  lfs f0, 0x2230(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8752 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B1D0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B1D4: D01F00D4  stfs f0, 0xd4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), tmp.u32 ) };
	// 8238B1D8: C00B222C  lfs f0, 0x222c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8748 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B1DC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B1E0: D01F00D8  stfs f0, 0xd8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), tmp.u32 ) };
	// 8238B1E4: C00B2228  lfs f0, 0x2228(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8744 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B1E8: D01F00DC  stfs f0, 0xdc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), tmp.u32 ) };
	// 8238B1EC: D01F00E0  stfs f0, 0xe0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 8238B1F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238B1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238B1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238B1FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238B200: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238B204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238B208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238B208 size=232
    let mut pc: u32 = 0x8238B208;
    'dispatch: loop {
        match pc {
            0x8238B208 => {
    //   block [0x8238B208..0x8238B2F0)
	// 8238B208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238B20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238B210: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238B214: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 8238B218: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8238B21C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238B220: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238B224: C01F00A4  lfs f0, 0xa4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B228: EDA1002A  fadds f13, f1, f0
	ctx.f[13].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8238B22C: C19F00A0  lfs f12, 0xa0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238B230: D1BF00A4  stfs f13, 0xa4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 8238B234: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 8238B238: 40990040  ble cr6, 0x8238b278
	if !ctx.cr[6].gt {
	pc = 0x8238B278; continue 'dispatch;
	}
	// 8238B23C: C01F009C  lfs f0, 0x9c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B240: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B244: ED80602A  fadds f12, f0, f12
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 8238B248: C15F00A8  lfs f10, 0xa8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8238B24C: EC010024  fdivs f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 / ctx.f[0].f64) as f32) as f64;
	// 8238B250: C16BBFFC  lfs f11, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8238B254: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 8238B258: EC0052FA  fmadds f0, f0, f11, f10
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[11].f64 + ctx.f[10].f64) as f32) as f64);
	// 8238B25C: D01F00A8  stfs f0, 0xa8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 8238B260: 40990018  ble cr6, 0x8238b278
	if !ctx.cr[6].gt {
	pc = 0x8238B278; continue 'dispatch;
	}
	// 8238B264: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B268: EDAD6028  fsubs f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 8238B26C: D1BF00A4  stfs f13, 0xa4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 8238B270: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B274: D01F00A8  stfs f0, 0xa8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 8238B278: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B27C: C1BF00A8  lfs f13, 0xa8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B280: C00B2490  lfs f0, 0x2490(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9360 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B284: EFED0032  fmuls f31, f13, f0
	ctx.f[31].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238B288: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8238B28C: 4BD9223D  bl 0x8211d4c8
	ctx.lr = 0x8238B290;
	sub_8211D4C8(ctx, base);
	// 8238B290: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 8238B294: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238B298: C1BF0090  lfs f13, 0x90(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B29C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8238B2A0: C3CBBA38  lfs f30, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8238B2A4: EC00F37A  fmadds f0, f0, f13, f30
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64 + ctx.f[30].f64) as f32) as f64);
	// 8238B2A8: D01F0050  stfs f0, 0x50(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8238B2AC: 4BD9221D  bl 0x8211d4c8
	ctx.lr = 0x8238B2B0;
	sub_8211D4C8(ctx, base);
	// 8238B2B0: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 8238B2B4: C1BF0094  lfs f13, 0x94(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B2B8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8238B2BC: EC00F37A  fmadds f0, f0, f13, f30
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64 + ctx.f[30].f64) as f32) as f64);
	// 8238B2C0: D01F0064  stfs f0, 0x64(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8238B2C4: 4BD92205  bl 0x8211d4c8
	ctx.lr = 0x8238B2C8;
	sub_8211D4C8(ctx, base);
	// 8238B2C8: C01F0098  lfs f0, 0x98(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B2CC: EC01F03A  fmadds f0, f1, f0, f30
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64 + ctx.f[30].f64) as f32) as f64);
	// 8238B2D0: D01F0078  stfs f0, 0x78(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 8238B2D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238B2D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238B2DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238B2E0: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8238B2E4: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238B2E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238B2EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238B2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238B2F0 size=80
    let mut pc: u32 = 0x8238B2F0;
    'dispatch: loop {
        match pc {
            0x8238B2F0 => {
    //   block [0x8238B2F0..0x8238B340)
	// 8238B2F0: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8238B2F4: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 8238B2F8: 39630050  addi r11, r3, 0x50
	ctx.r[11].s64 = ctx.r[3].s64 + 80;
	// 8238B2FC: C1A300AC  lfs f13, 0xac(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B300: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 8238B304: D1A300A4  stfs f13, 0xa4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 8238B308: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 8238B30C: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8238B310: C00A1FF8  lfs f0, 0x1ff8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B314: 3D403F80  lis r10, 0x3f80
	ctx.r[10].s64 = 1065353216;
	// 8238B318: D00300A8  stfs f0, 0xa8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238B340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238B340 size=56
    let mut pc: u32 = 0x8238B340;
    'dispatch: loop {
        match pc {
            0x8238B340 => {
    //   block [0x8238B340..0x8238B378)
	// 8238B340: C0040010  lfs f0, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B344: D003009C  stfs f0, 0x9c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 8238B348: C0040014  lfs f0, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B34C: D00300A0  stfs f0, 0xa0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 8238B350: C0040018  lfs f0, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B354: D00300AC  stfs f0, 0xac(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 8238B358: D00300A4  stfs f0, 0xa4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 8238B35C: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B360: D0030090  stfs f0, 0x90(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8238B364: C0040020  lfs f0, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B368: D0030094  stfs f0, 0x94(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8238B36C: C0040024  lfs f0, 0x24(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B370: D0030098  stfs f0, 0x98(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8238B374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238B378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238B378 size=152
    let mut pc: u32 = 0x8238B378;
    'dispatch: loop {
        match pc {
            0x8238B378 => {
    //   block [0x8238B378..0x8238B410)
	// 8238B378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238B37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238B380: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238B384: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238B388: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238B38C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238B390: 3BDF0050  addi r30, r31, 0x50
	ctx.r[30].s64 = ctx.r[31].s64 + 80;
	// 8238B394: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8238B398: C01F0090  lfs f0, 0x90(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B39C: C1BF009C  lfs f13, 0x9c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B3A0: EC0D007A  fmadds f0, f13, f1, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 8238B3A4: C07F0098  lfs f3, 0x98(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8238B3A8: C05F0094  lfs f2, 0x94(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8238B3AC: FDA0065E  fctidz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 8238B3B0: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8238B3B4: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8238B3B8: EC206828  fsubs f1, f0, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238B3BC: D03F0090  stfs f1, 0x90(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8238B3C0: 4BFDCE59  bl 0x82368218
	ctx.lr = 0x8238B3C4;
	sub_82368218(ctx, base);
	// 8238B3C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238B3C8: C1BF00A0  lfs f13, 0xa0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B3CC: 38BF00B0  addi r5, r31, 0xb0
	ctx.r[5].s64 = ctx.r[31].s64 + 176;
	// 8238B3D0: D1BE0030  stfs f13, 0x30(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8238B3D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8238B3D8: C1BF00A4  lfs f13, 0xa4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B3DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8238B3E0: D1BE0034  stfs f13, 0x34(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 8238B3E4: C1BF00A8  lfs f13, 0xa8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B3E8: C00BBA38  lfs f0, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B3EC: D1BE0038  stfs f13, 0x38(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 8238B3F0: D01E003C  stfs f0, 0x3c(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 8238B3F4: 4BFDC93D  bl 0x82367d30
	ctx.lr = 0x8238B3F8;
	sub_82367D30(ctx, base);
	// 8238B3F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238B3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238B400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238B404: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238B408: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238B40C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238B410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238B410 size=12
    let mut pc: u32 = 0x8238B410;
    'dispatch: loop {
        match pc {
            0x8238B410 => {
    //   block [0x8238B410..0x8238B41C)
	// 8238B410: C00300F0  lfs f0, 0xf0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(240 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B414: D0030090  stfs f0, 0x90(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8238B418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238B420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238B420 size=184
    let mut pc: u32 = 0x8238B420;
    'dispatch: loop {
        match pc {
            0x8238B420 => {
    //   block [0x8238B420..0x8238B4D8)
	// 8238B420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238B424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238B428: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238B42C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238B430: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238B434: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238B438: C0040014  lfs f0, 0x14(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B43C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B440: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 8238B444: 3BDF00B0  addi r30, r31, 0xb0
	ctx.r[30].s64 = ctx.r[31].s64 + 176;
	// 8238B448: D01F00F0  stfs f0, 0xf0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 8238B44C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8238B450: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8238B454: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8238B458: D01F0090  stfs f0, 0x90(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8238B45C: C0040018  lfs f0, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B460: D01F0094  stfs f0, 0x94(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8238B464: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B468: D01F0098  stfs f0, 0x98(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8238B46C: C0040010  lfs f0, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B470: D01F009C  stfs f0, 0x9c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 8238B474: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8238B478: C07F0098  lfs f3, 0x98(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8238B47C: C05F0094  lfs f2, 0x94(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8238B480: F95F00A0  std r10, 0xa0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[10].u64 ) };
	// 8238B484: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8238B488: F97F00A8  std r11, 0xa8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[11].u64 ) };
	// 8238B48C: 4BFDCD8D  bl 0x82368218
	ctx.lr = 0x8238B490;
	sub_82368218(ctx, base);
	// 8238B490: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238B494: C1BF00A0  lfs f13, 0xa0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B498: D1BE0030  stfs f13, 0x30(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8238B49C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8238B4A0: C1BF00A4  lfs f13, 0xa4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B4A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8238B4A8: D1BE0034  stfs f13, 0x34(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 8238B4AC: C1BF00A8  lfs f13, 0xa8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B4B0: C00BBA38  lfs f0, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B4B4: D1BE0038  stfs f13, 0x38(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 8238B4B8: D01E003C  stfs f0, 0x3c(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 8238B4BC: 4BFDC7CD  bl 0x82367c88
	ctx.lr = 0x8238B4C0;
	sub_82367C88(ctx, base);
	// 8238B4C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238B4C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238B4C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238B4CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238B4D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238B4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238B4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238B4D8 size=464
    let mut pc: u32 = 0x8238B4D8;
    'dispatch: loop {
        match pc {
            0x8238B4D8 => {
    //   block [0x8238B4D8..0x8238B6A8)
	// 8238B4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238B4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238B4E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238B4E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238B4E8: 3981FFE8  addi r12, r1, -0x18
	ctx.r[12].s64 = ctx.r[1].s64 + -24;
	// 8238B4EC: 481AAAF5  bl 0x82535fe0
	ctx.lr = 0x8238B4F0;
	sub_82535FB0(ctx, base);
	// 8238B4F0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238B4F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238B4F8: C01F0094  lfs f0, 0x94(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B4FC: EC01002A  fadds f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8238B500: C15F009C  lfs f10, 0x9c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8238B504: D01F0094  stfs f0, 0x94(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8238B508: FF0A0000  fcmpu cr6, f10, f0
	ctx.cr[6].compare_f64(ctx.f[10].f64, ctx.f[0].f64);
	// 8238B50C: 41990040  bgt cr6, 0x8238b54c
	if ctx.cr[6].gt {
	pc = 0x8238B54C; continue 'dispatch;
	}
	// 8238B510: C1BF0090  lfs f13, 0x90(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B514: C19F0098  lfs f12, 0x98(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238B518: EDAC682A  fadds f13, f12, f13
	ctx.f[13].f64 = ((ctx.f[12].f64 + ctx.f[13].f64) as f32) as f64;
	// 8238B51C: C19F00A0  lfs f12, 0xa0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238B520: ED8C0032  fmuls f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238B524: FD80665E  fctidz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[12].f64.trunc() as i64 };
	// 8238B528: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8238B52C: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8238B530: EC0C02BC  fnmsubs f0, f12, f10, f0
	ctx.f[0].f64 = -(((ctx.f[12].f64 * ctx.f[10].f64 - ctx.f[0].f64) as f32) as f64);
	// 8238B534: D01F0094  stfs f0, 0x94(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8238B538: FC006E5E  fctidz f0, f13
	ctx.f[0].s64 = if ctx.f[13].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[13].f64.trunc() as i64 };
	// 8238B53C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8238B540: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8238B544: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8238B548: D01F0090  stfs f0, 0x90(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8238B54C: 83DF00B0  lwz r30, 0xb0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 8238B550: C01F0094  lfs f0, 0x94(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B554: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238B558: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238B55C: C36BBA38  lfs f27, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 8238B560: 40820070  bne 0x8238b5d0
	if !ctx.cr[0].eq {
	pc = 0x8238B5D0; continue 'dispatch;
	}
	// 8238B564: C1BF00A4  lfs f13, 0xa4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B568: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8238B56C: 41980070  blt cr6, 0x8238b5dc
	if ctx.cr[6].lt {
	pc = 0x8238B5DC; continue 'dispatch;
	}
	// 8238B570: C17F00A8  lfs f11, 0xa8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8238B574: FF0D5800  fcmpu cr6, f13, f11
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[11].f64);
	// 8238B578: 419A00E8  beq cr6, 0x8238b660
	if ctx.cr[6].eq {
	pc = 0x8238B660; continue 'dispatch;
	}
	// 8238B57C: ED8B6828  fsubs f12, f11, f13
	ctx.f[12].f64 = (((ctx.f[11].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238B580: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8238B584: ED9B6024  fdivs f12, f27, f12
	ctx.f[12].f64 = ((ctx.f[27].f64 / ctx.f[12].f64) as f32) as f64;
	// 8238B588: 41990020  bgt cr6, 0x8238b5a8
	if ctx.cr[6].gt {
	pc = 0x8238B5A8; continue 'dispatch;
	}
	// 8238B58C: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 8238B590: 40980018  bge cr6, 0x8238b5a8
	if !ctx.cr[6].lt {
	pc = 0x8238B5A8; continue 'dispatch;
	}
	// 8238B594: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238B598: C1BF0098  lfs f13, 0x98(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B59C: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8238B5A0: EC000332  fmuls f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 8238B5A4: 480000C0  b 0x8238b664
	pc = 0x8238B664; continue 'dispatch;
	// 8238B5A8: C1BF00AC  lfs f13, 0xac(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B5AC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8238B5B0: C1BF0098  lfs f13, 0x98(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B5B4: 40980014  bge cr6, 0x8238b5c8
	if !ctx.cr[6].lt {
	pc = 0x8238B5C8; continue 'dispatch;
	}
	// 8238B5B8: EC005828  fsubs f0, f0, f11
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[11].f64) as f32) as f64);
	// 8238B5BC: EC00DB3A  fmadds f0, f0, f12, f27
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64 + ctx.f[27].f64) as f32) as f64);
	// 8238B5C0: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8238B5C4: 480000A0  b 0x8238b664
	pc = 0x8238B664; continue 'dispatch;
	// 8238B5C8: EC0A0028  fsubs f0, f10, f0
	ctx.f[0].f64 = (((ctx.f[10].f64 - ctx.f[0].f64) as f32) as f64);
	// 8238B5CC: 4BFFFFF0  b 0x8238b5bc
	pc = 0x8238B5BC; continue 'dispatch;
	// 8238B5D0: C3FF00A4  lfs f31, 0xa4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8238B5D4: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8238B5D8: 40980010  bge cr6, 0x8238b5e8
	if !ctx.cr[6].lt {
	pc = 0x8238B5E8; continue 'dispatch;
	}
	// 8238B5DC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B5E0: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B5E4: 48000080  b 0x8238b664
	pc = 0x8238B664; continue 'dispatch;
	// 8238B5E8: C35F00A8  lfs f26, 0xa8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 8238B5EC: FF1A5000  fcmpu cr6, f26, f10
	ctx.cr[6].compare_f64(ctx.f[26].f64, ctx.f[10].f64);
	// 8238B5F0: 419A0070  beq cr6, 0x8238b660
	if ctx.cr[6].eq {
	pc = 0x8238B660; continue 'dispatch;
	}
	// 8238B5F4: FF1AF800  fcmpu cr6, f26, f31
	ctx.cr[6].compare_f64(ctx.f[26].f64, ctx.f[31].f64);
	// 8238B5F8: 419A0068  beq cr6, 0x8238b660
	if ctx.cr[6].eq {
	pc = 0x8238B660; continue 'dispatch;
	}
	// 8238B5FC: C1BF00AC  lfs f13, 0xac(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B600: EC00F828  fsubs f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[31].f64) as f32) as f64);
	// 8238B604: EDADF828  fsubs f13, f13, f31
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[31].f64) as f32) as f64);
	// 8238B608: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B60C: C3CB24B4  lfs f30, 0x24b4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9396 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8238B610: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B614: EFBB6824  fdivs f29, f27, f13
	ctx.f[29].f64 = ((ctx.f[27].f64 / ctx.f[13].f64) as f32) as f64;
	// 8238B618: C38B2648  lfs f28, 0x2648(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9800 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 8238B61C: EC000772  fmuls f0, f0, f29
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[29].f64) as f32) as f64);
	// 8238B620: EC20E7BA  fmadds f1, f0, f30, f28
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[30].f64 + ctx.f[28].f64) as f32) as f64);
	// 8238B624: 4BD91EA5  bl 0x8211d4c8
	ctx.lr = 0x8238B628;
	sub_8211D4C8(ctx, base);
	// 8238B628: EC1AF828  fsubs f0, f26, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[26].f64 - ctx.f[31].f64) as f32) as f64);
	// 8238B62C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B630: EDA1D82A  fadds f13, f1, f27
	ctx.f[13].f64 = ((ctx.f[1].f64 + ctx.f[27].f64) as f32) as f64;
	// 8238B634: C3EBBFFC  lfs f31, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8238B638: EC000772  fmuls f0, f0, f29
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[29].f64) as f32) as f64);
	// 8238B63C: EFAD07F2  fmuls f29, f13, f31
	ctx.f[29].f64 = (((ctx.f[13].f64 * ctx.f[31].f64) as f32) as f64);
	// 8238B640: EC20E7BA  fmadds f1, f0, f30, f28
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[30].f64 + ctx.f[28].f64) as f32) as f64);
	// 8238B644: 4BD91E85  bl 0x8211d4c8
	ctx.lr = 0x8238B648;
	sub_8211D4C8(ctx, base);
	// 8238B648: EDA1D82A  fadds f13, f1, f27
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = ((ctx.f[1].f64 + ctx.f[27].f64) as f32) as f64;
	// 8238B64C: C01F0098  lfs f0, 0x98(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B650: EDAD07F2  fmuls f13, f13, f31
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[31].f64) as f32) as f64);
	// 8238B654: EDBD6824  fdivs f13, f29, f13
	ctx.f[13].f64 = ((ctx.f[29].f64 / ctx.f[13].f64) as f32) as f64;
	// 8238B658: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238B65C: 48000008  b 0x8238b664
	pc = 0x8238B664; continue 'dispatch;
	// 8238B660: C01F0098  lfs f0, 0x98(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B664: C1BF0090  lfs f13, 0x90(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B668: 57CB07BD  rlwinm. r11, r30, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238B66C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8238B670: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8238B674: 4182000C  beq 0x8238b680
	if ctx.cr[0].eq {
	pc = 0x8238B680; continue 'dispatch;
	}
	// 8238B678: EC20D82A  fadds f1, f0, f27
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[27].f64) as f32) as f64;
	// 8238B67C: 48000008  b 0x8238b684
	pc = 0x8238B684; continue 'dispatch;
	// 8238B680: EC3B0028  fsubs f1, f27, f0
	ctx.f[1].f64 = (((ctx.f[27].f64 - ctx.f[0].f64) as f32) as f64);
	// 8238B684: 4BFDC935  bl 0x82367fb8
	ctx.lr = 0x8238B688;
	sub_82367FB8(ctx, base);
	// 8238B688: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8238B68C: 3981FFE8  addi r12, r1, -0x18
	ctx.r[12].s64 = ctx.r[1].s64 + -24;
	// 8238B690: 481AA99D  bl 0x8253602c
	ctx.lr = 0x8238B694;
	sub_82535FFC(ctx, base);
	// 8238B694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238B698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238B69C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238B6A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238B6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238B6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238B6A8 size=76
    let mut pc: u32 = 0x8238B6A8;
    'dispatch: loop {
        match pc {
            0x8238B6A8 => {
    //   block [0x8238B6A8..0x8238B6F4)
	// 8238B6A8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8238B6AC: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 8238B6B0: 39630050  addi r11, r3, 0x50
	ctx.r[11].s64 = ctx.r[3].s64 + 80;
	// 8238B6B4: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 8238B6B8: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 8238B6BC: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8238B6C0: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B6C4: 3D403F80  lis r10, 0x3f80
	ctx.r[10].s64 = 1065353216;
	// 8238B6C8: D0030090  stfs f0, 0x90(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8238B6CC: D0030094  stfs f0, 0x94(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238B6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238B6F8 size=88
    let mut pc: u32 = 0x8238B6F8;
    'dispatch: loop {
        match pc {
            0x8238B6F8 => {
    //   block [0x8238B6F8..0x8238B750)
	// 8238B6F8: C0040014  lfs f0, 0x14(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B6FC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B700: D0030098  stfs f0, 0x98(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8238B704: C0040018  lfs f0, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B708: D003009C  stfs f0, 0x9c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 8238B70C: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B710: D00300A4  stfs f0, 0xa4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 8238B714: C0040020  lfs f0, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B718: C18BBFFC  lfs f12, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238B71C: D00300A8  stfs f0, 0xa8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 8238B720: 89640010  lbz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238B724: C003009C  lfs f0, 0x9c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B728: C1A300A8  lfs f13, 0xa8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(168 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B72C: 916300B0  stw r11, 0xb0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 8238B730: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238B734: C16BBA38  lfs f11, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8238B738: ED6B0024  fdivs f11, f11, f0
	ctx.f[11].f64 = ((ctx.f[11].f64 / ctx.f[0].f64) as f32) as f64;
	// 8238B73C: D16300A0  stfs f11, 0xa0(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 8238B740: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238B744: EC006B3A  fmadds f0, f0, f12, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64 + ctx.f[13].f64) as f32) as f64);
	// 8238B748: D00300AC  stfs f0, 0xac(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 8238B74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238B750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238B750 size=312
    let mut pc: u32 = 0x8238B750;
    'dispatch: loop {
        match pc {
            0x8238B750 => {
    //   block [0x8238B750..0x8238B888)
	// 8238B750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238B754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238B758: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238B75C: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 8238B760: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8238B764: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238B768: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238B76C: C01F0014  lfs f0, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B770: EC01002A  fadds f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8238B774: C1BF0018  lfs f13, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B778: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8238B77C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8238B780: 41980024  blt cr6, 0x8238b7a4
	if ctx.cr[6].lt {
	pc = 0x8238B7A4; continue 'dispatch;
	}
	// 8238B784: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8238B788: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8238B78C: 419A0010  beq cr6, 0x8238b79c
	if ctx.cr[6].eq {
	pc = 0x8238B79C; continue 'dispatch;
	}
	// 8238B790: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B794: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B798: 48000008  b 0x8238b7a0
	pc = 0x8238B7A0; continue 'dispatch;
	// 8238B79C: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238B7A0: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8238B7A4: C01F0014  lfs f0, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B7A8: C1BF0024  lfs f13, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B7AC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8238B7B0: 419800B4  blt cr6, 0x8238b864
	if ctx.cr[6].lt {
	pc = 0x8238B864; continue 'dispatch;
	}
	// 8238B7B4: C19F0028  lfs f12, 0x28(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238B7B8: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8238B7BC: 40980058  bge cr6, 0x8238b814
	if !ctx.cr[6].lt {
	pc = 0x8238B814; continue 'dispatch;
	}
	// 8238B7C0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B7C4: ED606828  fsubs f11, f0, f13
	ctx.f[11].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238B7C8: EDAC6828  fsubs f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238B7CC: C00B2490  lfs f0, 0x2490(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9360 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B7D0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B7D4: C3CBBFFC  lfs f30, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8238B7D8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B7DC: ED8B07B2  fmuls f12, f11, f30
	ctx.f[12].f64 = (((ctx.f[11].f64 * ctx.f[30].f64) as f32) as f64);
	// 8238B7E0: ED8C6824  fdivs f12, f12, f13
	ctx.f[12].f64 = ((ctx.f[12].f64 / ctx.f[13].f64) as f32) as f64;
	// 8238B7E4: C1AB2604  lfs f13, 0x2604(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9732 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B7E8: EC2C6838  fmsubs f1, f12, f0, f13
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238B7EC: C3FF001C  lfs f31, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8238B7F0: 4BD91CD9  bl 0x8211d4c8
	ctx.lr = 0x8238B7F4;
	sub_8211D4C8(ctx, base);
	// 8238B7F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238B7F8: C01F0020  lfs f0, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B7FC: EDA0F828  fsubs f13, f0, f31
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[31].f64) as f32) as f64);
	// 8238B800: C00BBA38  lfs f0, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B804: EC01002A  fadds f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8238B808: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8238B80C: EC00FFBA  fmadds f0, f0, f30, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64 + ctx.f[31].f64) as f32) as f64);
	// 8238B810: 48000058  b 0x8238b868
	pc = 0x8238B868; continue 'dispatch;
	// 8238B814: C1BF002C  lfs f13, 0x2c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B818: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8238B81C: 4098000C  bge cr6, 0x8238b828
	if !ctx.cr[6].lt {
	pc = 0x8238B828; continue 'dispatch;
	}
	// 8238B820: C01F0020  lfs f0, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B824: 48000044  b 0x8238b868
	pc = 0x8238B868; continue 'dispatch;
	// 8238B828: C19F0030  lfs f12, 0x30(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238B82C: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8238B830: 40980034  bge cr6, 0x8238b864
	if !ctx.cr[6].lt {
	pc = 0x8238B864; continue 'dispatch;
	}
	// 8238B834: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B838: ED606828  fsubs f11, f0, f13
	ctx.f[11].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238B83C: EDAC6828  fsubs f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238B840: C00B2490  lfs f0, 0x2490(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9360 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B844: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B848: C3CBBFFC  lfs f30, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8238B84C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238B850: ED8B07B2  fmuls f12, f11, f30
	ctx.f[12].f64 = (((ctx.f[11].f64 * ctx.f[30].f64) as f32) as f64);
	// 8238B854: ED8C6824  fdivs f12, f12, f13
	ctx.f[12].f64 = ((ctx.f[12].f64 / ctx.f[13].f64) as f32) as f64;
	// 8238B858: C1AB2604  lfs f13, 0x2604(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9732 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238B85C: EC2C683A  fmadds f1, f12, f0, f13
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 8238B860: 4BFFFF8C  b 0x8238b7ec
	pc = 0x8238B7EC; continue 'dispatch;
	// 8238B864: C01F001C  lfs f0, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B868: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8238B86C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238B870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238B874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238B878: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8238B87C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238B880: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238B884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238B888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238B888 size=12
    let mut pc: u32 = 0x8238B888;
    'dispatch: loop {
        match pc {
            0x8238B888 => {
    //   block [0x8238B888..0x8238B894)
	// 8238B888: C0030034  lfs f0, 0x34(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B88C: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8238B890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238B898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238B898 size=80
    let mut pc: u32 = 0x8238B898;
    'dispatch: loop {
        match pc {
            0x8238B898 => {
    //   block [0x8238B898..0x8238B8E8)
	// 8238B898: 89640011  lbz r11, 0x11(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(17 as u32) ) } as u64;
	// 8238B89C: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8238B8A0: C0040014  lfs f0, 0x14(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B8A4: D0030034  stfs f0, 0x34(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 8238B8A8: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8238B8AC: C0040018  lfs f0, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B8B0: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8238B8B4: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B8B8: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8238B8BC: C0040020  lfs f0, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B8C0: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8238B8C4: C0040024  lfs f0, 0x24(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B8C8: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8238B8CC: C0040028  lfs f0, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B8D0: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8238B8D4: C004002C  lfs f0, 0x2c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B8D8: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8238B8DC: C0040030  lfs f0, 0x30(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B8E0: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8238B8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238B8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238B8E8 size=384
    let mut pc: u32 = 0x8238B8E8;
    'dispatch: loop {
        match pc {
            0x8238B8E8 => {
    //   block [0x8238B8E8..0x8238BA68)
	// 8238B8E8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8238B8EC: 3D2082B6  lis r9, -0x7d4a
	ctx.r[9].s64 = -2102001664;
	// 8238B8F0: A1030026  lhz r8, 0x26(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(38 as u32) ) } as u64;
	// 8238B8F4: 3929BA70  addi r9, r9, -0x4590
	ctx.r[9].s64 = ctx.r[9].s64 + -17808;
	// 8238B8F8: 8141FFEC  lwz r10, -0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-20 as u32) ) } as u64;
	// 8238B8FC: C00B0030  lfs f0, 0x30(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B900: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 8238B904: C00B0034  lfs f0, 0x34(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B908: D001FFE4  stfs f0, -0x1c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), tmp.u32 ) };
	// 8238B90C: C00B0038  lfs f0, 0x38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238B910: D001FFE8  stfs f0, -0x18(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), tmp.u32 ) };
	// 8238B914: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238BA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238BA68 size=184
    let mut pc: u32 = 0x8238BA68;
    'dispatch: loop {
        match pc {
            0x8238BA68 => {
    //   block [0x8238BA68..0x8238BB20)
	// 8238BA68: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
	// 8238BA6C: 1181038C  vspltisw v12, 1
	for i in 0..4 {
		ctx.v[12].u32[i] = 1;
	}
	// 8238BA70: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 8238BA74: 1161634A  vcfsx v11, v12, 1
	ctx.fpscr.enable_flush_mode_unconditional();
	let scale = f32::from_bits(((127u32 - (1 as u32)) << 23));
	for i in 0..4 {
		ctx.v[11].f32[i] = (ctx.v[12].s32[i] as f32) * scale;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238BB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238BB20 size=8
    let mut pc: u32 = 0x8238BB20;
    'dispatch: loop {
        match pc {
            0x8238BB20 => {
    //   block [0x8238BB20..0x8238BB28)
	// 8238BB20: EC0D0028  fsubs f0, f13, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8238BB24: 48000020  b 0x8238bb44
	sub_8238BB40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238BB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238BB28 size=24
    let mut pc: u32 = 0x8238BB28;
    'dispatch: loop {
        match pc {
            0x8238BB28 => {
    //   block [0x8238BB28..0x8238BB40)
	// 8238BB28: 4098FFEC  bge cr6, 0x8238bb14
	if !ctx.cr[6].lt {
		sub_8238BA68(ctx, base);
		return;
	}
	// 8238BB2C: C1A30018  lfs f13, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238BB30: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8238BB34: 4098000C  bge cr6, 0x8238bb40
	if !ctx.cr[6].lt {
		sub_8238BB40(ctx, base);
		return;
	}
	// 8238BB38: D163000C  stfs f11, 0xc(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8238BB3C: 48000014  b 0x8238bb50
	sub_8238BB40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238BB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238BB40 size=40
    let mut pc: u32 = 0x8238BB40;
    'dispatch: loop {
        match pc {
            0x8238BB40 => {
    //   block [0x8238BB40..0x8238BB68)
	// 8238BB40: EC006828  fsubs f0, f0, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238BB44: C1A3001C  lfs f13, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238BB48: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8238BB4C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8238BB50: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238BB54: C003000C  lfs f0, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238BB58: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238BB5C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238BB60: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8238BB64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238BB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238BB68 size=60
    let mut pc: u32 = 0x8238BB68;
    'dispatch: loop {
        match pc {
            0x8238BB68 => {
    //   block [0x8238BB68..0x8238BBA4)
	// 8238BB68: 89640012  lbz r11, 0x12(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(18 as u32) ) } as u64;
	// 8238BB6C: B1630026  sth r11, 0x26(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(38 as u32), ctx.r[11].u16 ) };
	// 8238BB70: 89640011  lbz r11, 0x11(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(17 as u32) ) } as u64;
	// 8238BB74: B1630024  sth r11, 0x24(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u16 ) };
	// 8238BB78: C0040014  lfs f0, 0x14(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238BB7C: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8238BB80: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238BB84: C0040018  lfs f0, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238BB88: C1A30014  lfs f13, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238BB8C: EDAD0028  fsubs f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8238BB90: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8238BB94: C00BBA38  lfs f0, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238BB98: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8238BB9C: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8238BBA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238BBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238BBA8 size=124
    let mut pc: u32 = 0x8238BBA8;
    'dispatch: loop {
        match pc {
            0x8238BBA8 => {
    //   block [0x8238BBA8..0x8238BC24)
	// 8238BBA8: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 8238BBAC: C163001C  lfs f11, 0x1c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8238BBB0: C00B0034  lfs f0, 0x34(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238BBB4: C1AB0038  lfs f13, 0x38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238BBB8: C18B0030  lfs f12, 0x30(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238BBBC: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 8238BBC0: 396BBA70  addi r11, r11, -0x4590
	ctx.r[11].s64 = ctx.r[11].s64 + -17808;
	// 8238BBC4: C14B0004  lfs f10, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8238BBC8: EC005028  fsubs f0, f0, f10
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[10].f64) as f32) as f64);
	// 8238BBCC: C14B0008  lfs f10, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8238BBD0: EDAD5028  fsubs f13, f13, f10
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[10].f64) as f32) as f64);
	// 8238BBD4: C14B0000  lfs f10, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8238BBD8: ED8C5028  fsubs f12, f12, f10
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[10].f64) as f32) as f64);
	// 8238BBDC: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238BBE0: EC0D037A  fmadds f0, f13, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 8238BBE4: EC0C033A  fmadds f0, f12, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 8238BBE8: EC00002C  fsqrts f0, f0
	ctx.f[0].f64 = ((ctx.f[0].f64).sqrt() as f32) as f64;
	// 8238BBEC: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 8238BBF0: 41980060  blt cr6, 0x8238bc50
	if ctx.cr[6].lt {
		sub_8238BC50(ctx, base);
		return;
	}
	// 8238BBF4: C1830020  lfs f12, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238BBF8: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8238BBFC: 40980028  bge cr6, 0x8238bc24
	if !ctx.cr[6].lt {
		sub_8238BC24(ctx, base);
		return;
	}
	// 8238BC00: EC005828  fsubs f0, f0, f11
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[11].f64) as f32) as f64);
	// 8238BC04: ED8C5828  fsubs f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 8238BC08: C1A30014  lfs f13, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238BC0C: C1430018  lfs f10, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8238BC10: ED6A6828  fsubs f11, f10, f13
	ctx.f[11].f64 = (((ctx.f[10].f64 - ctx.f[13].f64) as f32) as f64);
	// 8238BC14: EC0B0032  fmuls f0, f11, f0
	ctx.f[0].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238BC18: EC006024  fdivs f0, f0, f12
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[12].f64) as f32) as f64;
	// 8238BC1C: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 8238BC20: 48000034  b 0x8238bc54
	sub_8238BC50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238BC24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238BC24 size=20
    let mut pc: u32 = 0x8238BC24;
    'dispatch: loop {
        match pc {
            0x8238BC24 => {
    //   block [0x8238BC24..0x8238BC38)
	// 8238BC24: C1630024  lfs f11, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8238BC28: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 8238BC2C: 4098000C  bge cr6, 0x8238bc38
	if !ctx.cr[6].lt {
		sub_8238BC38(ctx, base);
		return;
	}
	// 8238BC30: C0030018  lfs f0, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238BC34: 48000020  b 0x8238bc54
	sub_8238BC50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238BC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238BC38 size=24
    let mut pc: u32 = 0x8238BC38;
    'dispatch: loop {
        match pc {
            0x8238BC38 => {
    //   block [0x8238BC38..0x8238BC50)
	// 8238BC38: C1830028  lfs f12, 0x28(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238BC3C: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8238BC40: 40980010  bge cr6, 0x8238bc50
	if !ctx.cr[6].lt {
		sub_8238BC50(ctx, base);
		return;
	}
	// 8238BC44: EC006028  fsubs f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 8238BC48: ED8B6028  fsubs f12, f11, f12
	ctx.f[12].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 8238BC4C: 4BFFFFBC  b 0x8238bc08
	sub_8238BBA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238BC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238BC50 size=32
    let mut pc: u32 = 0x8238BC50;
    'dispatch: loop {
        match pc {
            0x8238BC50 => {
    //   block [0x8238BC50..0x8238BC70)
	// 8238BC50: C0030014  lfs f0, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238BC54: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238BC58: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8238BC5C: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238BC60: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8238BC64: FC0C682E  fsel f0, f12, f0, f13
	ctx.f[0].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 8238BC68: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8238BC6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238BC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238BC70 size=28
    let mut pc: u32 = 0x8238BC70;
    'dispatch: loop {
        match pc {
            0x8238BC70 => {
    //   block [0x8238BC70..0x8238BC8C)
	// 8238BC70: 89640011  lbz r11, 0x11(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(17 as u32) ) } as u64;
	// 8238BC74: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238BC78: 40820014  bne 0x8238bc8c
	if !ctx.cr[0].eq {
		sub_8238BC8C(ctx, base);
		return;
	}
	// 8238BC7C: C0040018  lfs f0, 0x18(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238BC80: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8238BC84: C0040014  lfs f0, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238BC88: 48000010  b 0x8238bc98
	sub_8238BC8C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238BC8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238BC8C size=52
    let mut pc: u32 = 0x8238BC8C;
    'dispatch: loop {
        match pc {
            0x8238BC8C => {
    //   block [0x8238BC8C..0x8238BCC0)
	// 8238BC8C: C0040014  lfs f0, 0x14(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238BC90: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8238BC94: C0040018  lfs f0, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238BC98: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8238BC9C: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238BCA0: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8238BCA4: C0040020  lfs f0, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238BCA8: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8238BCAC: C0040024  lfs f0, 0x24(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238BCB0: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8238BCB4: C0040028  lfs f0, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238BCB8: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8238BCBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238BCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238BCC0 size=16
    let mut pc: u32 = 0x8238BCC0;
    'dispatch: loop {
        match pc {
            0x8238BCC0 => {
    //   block [0x8238BCC0..0x8238BCD0)
	// 8238BCC0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238BCC4: 396B0005  addi r11, r11, 5
	ctx.r[11].s64 = ctx.r[11].s64 + 5;
	// 8238BCC8: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8238BCCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238BCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238BCD0 size=16
    let mut pc: u32 = 0x8238BCD0;
    'dispatch: loop {
        match pc {
            0x8238BCD0 => {
    //   block [0x8238BCD0..0x8238BCE0)
	// 8238BCD0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238BCD4: 3D4082B6  lis r10, -0x7d4a
	ctx.r[10].s64 = -2102001664;
	// 8238BCD8: 916ABA80  stw r11, -0x4580(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17792 as u32), ctx.r[11].u32 ) };
	// 8238BCDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238BCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238BCE0 size=84
    let mut pc: u32 = 0x8238BCE0;
    'dispatch: loop {
        match pc {
            0x8238BCE0 => {
    //   block [0x8238BCE0..0x8238BD34)
	// 8238BCE0: 81440010  lwz r10, 0x10(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238BCE4: 39230014  addi r9, r3, 0x14
	ctx.r[9].s64 = ctx.r[3].s64 + 20;
	// 8238BCE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8238BCEC: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8238BCF0: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8238BCF4: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8238BCF8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8238BCFC: 4099002C  ble cr6, 0x8238bd28
	if !ctx.cr[6].gt {
	pc = 0x8238BD28; continue 'dispatch;
	}
	// 8238BD00: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8238BD04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8238BD08: C0091FF8  lfs f0, 0x1ff8(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238BD0C: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238BD10: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8238BD14: 7C09552E  stfsx f0, r9, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), tmp.u32) };
	// 8238BD18: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238BD1C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8238BD20: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8238BD24: 4198FFE8  blt cr6, 0x8238bd0c
	if ctx.cr[6].lt {
	pc = 0x8238BD0C; continue 'dispatch;
	}
	// 8238BD28: 39640014  addi r11, r4, 0x14
	ctx.r[11].s64 = ctx.r[4].s64 + 20;
	// 8238BD2C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8238BD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238BD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238BD38 size=68
    let mut pc: u32 = 0x8238BD38;
    'dispatch: loop {
        match pc {
            0x8238BD38 => {
    //   block [0x8238BD38..0x8238BD7C)
	// 8238BD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238BD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238BD40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238BD44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238BD48: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8238BD4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238BD50: 396B3BF8  addi r11, r11, 0x3bf8
	ctx.r[11].s64 = ctx.r[11].s64 + 15352;
	// 8238BD54: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8238BD58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8238BD5C: 41820008  beq 0x8238bd64
	if ctx.cr[0].eq {
	pc = 0x8238BD64; continue 'dispatch;
	}
	// 8238BD60: 481A6E59  bl 0x82532bb8
	ctx.lr = 0x8238BD64;
	sub_82532BB8(ctx, base);
	// 8238BD64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8238BD68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8238BD6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238BD70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238BD74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238BD78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238BD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238BD80 size=1008
    let mut pc: u32 = 0x8238BD80;
    'dispatch: loop {
        match pc {
            0x8238BD80 => {
    //   block [0x8238BD80..0x8238C170)
	// 8238BD80: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 8238BD84: 11A1038C  vspltisw v13, 1
	for i in 0..4 {
		ctx.v[13].u32[i] = 1;
	}
	// 8238BD88: 10016B4A  vcfsx v0, v13, 1
	ctx.fpscr.enable_flush_mode_unconditional();
	let scale = f32::from_bits(((127u32 - (1 as u32)) << 23));
	for i in 0..4 {
		ctx.v[0].f32[i] = (ctx.v[13].s32[i] as f32) * scale;
	}
	// 8238BD8C: 814BBF90  lwz r10, -0x4070(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16496 as u32) ) } as u64;
	// 8238BD90: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238BD94: 11A06B4A  vcfsx v13, v13, 0
	let scale = f32::from_bits(((127u32 - (0 as u32)) << 23));
	for i in 0..4 {
		ctx.v[13].f32[i] = (ctx.v[13].s32[i] as f32) * scale;
	}
	// 8238BD98: C1AA00C0  lfs f13, 0xc0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(192 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238BD9C: C18BBA38  lfs f12, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238BDA0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238BDA4: ED2C6824  fdivs f9, f12, f13
	ctx.f[9].f64 = ((ctx.f[12].f64 / ctx.f[13].f64) as f32) as f64;
	// 8238BDA8: C16A00EC  lfs f11, 0xec(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(236 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8238BDAC: C14A00E0  lfs f10, 0xe0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(224 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8238BDB0: C10A00E4  lfs f8, 0xe4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(228 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 8238BDB4: ECAA5828  fsubs f5, f10, f11
	ctx.f[5].f64 = (((ctx.f[10].f64 - ctx.f[11].f64) as f32) as f64);
	// 8238BDB8: C1AA00D4  lfs f13, 0xd4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238BDBC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8238BDC0: ECEC6824  fdivs f7, f12, f13
	ctx.f[7].f64 = ((ctx.f[12].f64 / ctx.f[13].f64) as f32) as f64;
	// 8238BDC4: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238BDC8: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 8238BDCC: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 8238BDD0: D181FFF0  stfs f12, -0x10(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8238BDD4: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 8238BDD8: C1AA2074  lfs f13, 0x2074(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8308 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238BDDC: ECCB0372  fmuls f6, f11, f13
	ctx.f[6].f64 = (((ctx.f[11].f64 * ctx.f[13].f64) as f32) as f64);
	// 8238BDE0: ED685828  fsubs f11, f8, f11
	ctx.f[11].f64 = (((ctx.f[8].f64 - ctx.f[11].f64) as f32) as f64);
	// 8238BDE4: 816BBFB0  lwz r11, -0x4050(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16464 as u32) ) } as u64;
	// 8238BDE8: ECA50272  fmuls f5, f5, f9
	ctx.f[5].f64 = (((ctx.f[5].f64 * ctx.f[9].f64) as f32) as f64);
	// 8238BDEC: C06B0008  lfs f3, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8238BDF0: C04B0000  lfs f2, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8238BDF4: C02B0004  lfs f1, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8238BDF8: EC8B01F2  fmuls f4, f11, f7
	ctx.f[4].f64 = (((ctx.f[11].f64 * ctx.f[7].f64) as f32) as f64);
	// 8238BDFC: ED6A3028  fsubs f11, f10, f6
	ctx.f[11].f64 = (((ctx.f[10].f64 - ctx.f[6].f64) as f32) as f64);
	// 8238BE00: ED483028  fsubs f10, f8, f6
	ctx.f[10].f64 = (((ctx.f[8].f64 - ctx.f[6].f64) as f32) as f64);
	// 8238BE04: ED6B0272  fmuls f11, f11, f9
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[9].f64) as f32) as f64);
	// 8238BE08: D161FFF8  stfs f11, -8(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 8238BE0C: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 8238BE10: ED0A01F2  fmuls f8, f10, f7
	ctx.f[8].f64 = (((ctx.f[10].f64 * ctx.f[7].f64) as f32) as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238C170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238C170 size=108
    let mut pc: u32 = 0x8238C170;
    'dispatch: loop {
        match pc {
            0x8238C170 => {
    //   block [0x8238C170..0x8238C1DC)
	// 8238C170: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8238C174: C0040008  lfs f0, 8(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238C178: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8238C17C: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238C180: C1840000  lfs f12, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238C184: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8238C188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8238C18C: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 8238C190: C1691FF8  lfs f11, 0x1ff8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8238C194: C14A0004  lfs f10, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8238C198: ED4A0032  fmuls f10, f10, f0
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238C19C: C12AFFFC  lfs f9, -4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8238C1A0: C10A0000  lfs f8, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 8238C1A4: C0EA0008  lfs f7, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 8238C1A8: ED49533A  fmadds f10, f9, f12, f10
	ctx.f[10].f64 = (((ctx.f[9].f64 * ctx.f[12].f64 + ctx.f[10].f64) as f32) as f64);
	// 8238C1AC: ED48537A  fmadds f10, f8, f13, f10
	ctx.f[10].f64 = (((ctx.f[8].f64 * ctx.f[13].f64 + ctx.f[10].f64) as f32) as f64);
	// 8238C1B0: ED4A382A  fadds f10, f10, f7
	ctx.f[10].f64 = ((ctx.f[10].f64 + ctx.f[7].f64) as f32) as f64;
	// 8238C1B4: FF0A5800  fcmpu cr6, f10, f11
	ctx.cr[6].compare_f64(ctx.f[10].f64, ctx.f[11].f64);
	// 8238C1B8: 40980010  bge cr6, 0x8238c1c8
	if !ctx.cr[6].lt {
	pc = 0x8238C1C8; continue 'dispatch;
	}
	// 8238C1BC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8238C1C0: 7D295830  slw r9, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 8238C1C4: 7D231B78  or r3, r9, r3
	ctx.r[3].u64 = ctx.r[9].u64 | ctx.r[3].u64;
	// 8238C1C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8238C1CC: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 8238C1D0: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 8238C1D4: 4198FFC0  blt cr6, 0x8238c194
	if ctx.cr[6].lt {
	pc = 0x8238C194; continue 'dispatch;
	}
	// 8238C1D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238C1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238C1E0 size=1364
    let mut pc: u32 = 0x8238C1E0;
    'dispatch: loop {
        match pc {
            0x8238C1E0 => {
    //   block [0x8238C1E0..0x8238C734)
	// 8238C1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238C1E4: 481A8EB5  bl 0x82535098
	ctx.lr = 0x8238C1E8;
	sub_82535080(ctx, base);
	// 8238C1E8: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238C1EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8238C1F0: 4BFFFB91  bl 0x8238bd80
	ctx.lr = 0x8238C1F4;
	sub_8238BD80(ctx, base);
	// 8238C1F4: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238C1F8: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 8238C1FC: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 8238C200: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8238C204: 40990528  ble cr6, 0x8238c72c
	if !ctx.cr[6].gt {
	pc = 0x8238C72C; continue 'dispatch;
	}
	// 8238C208: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 8238C20C: 3CA0820A  lis r5, -0x7df6
	ctx.r[5].s64 = -2113273856;
	// 8238C210: 3B6BBA70  addi r27, r11, -0x4590
	ctx.r[27].s64 = ctx.r[11].s64 + -17808;
	// 8238C214: 3D6082B5  lis r11, -0x7d4b
	ctx.r[11].s64 = -2102067200;
	// 8238C218: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 8238C21C: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 8238C220: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 8238C224: C085BA38  lfs f4, -0x45c8(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 8238C228: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8238C22C: 3D4082B6  lis r10, -0x7d4a
	ctx.r[10].s64 = -2102001664;
	// 8238C230: 3B4B5C40  addi r26, r11, 0x5c40
	ctx.r[26].s64 = ctx.r[11].s64 + 23616;
	// 8238C234: C0462048  lfs f2, 0x2048(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8264 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8238C238: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 8238C23C: C0A7BFFC  lfs f5, -0x4004(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 8238C240: C0C81FF8  lfs f6, 0x1ff8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8184 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 8238C244: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 8238C248: C0292074  lfs f1, 0x2074(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8308 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8238C24C: 3BEBB620  addi r31, r11, -0x49e0
	ctx.r[31].s64 = ctx.r[11].s64 + -18912;
	// 8238C250: 832ABA80  lwz r25, -0x4580(r10)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17792 as u32) ) } as u64;
	// 8238C254: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 8238C258: 3AE00030  li r23, 0x30
	ctx.r[23].s64 = 48;
	// 8238C25C: 3B000020  li r24, 0x20
	ctx.r[24].s64 = 32;
	// 8238C260: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238C264: FC600890  fmr f3, f1
	ctx.f[3].f64 = ctx.f[1].f64;
	// 8238C268: 7FD65A14  add r30, r22, r11
	ctx.r[30].u64 = ctx.r[22].u64 + ctx.r[11].u64;
	// 8238C26C: 897E0040  lbz r11, 0x40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 8238C270: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238C274: 40820334  bne 0x8238c5a8
	if !ctx.cr[0].eq {
	pc = 0x8238C5A8; continue 'dispatch;
	}
	// 8238C278: 897E0041  lbz r11, 0x41(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(65 as u32) ) } as u64;
	// 8238C27C: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 8238C280: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238C284: 41820030  beq 0x8238c2b4
	if ctx.cr[0].eq {
	pc = 0x8238C2B4; continue 'dispatch;
	}
	// 8238C288: 2B140000  cmplwi cr6, r20, 0
	ctx.cr[6].compare_u32(ctx.r[20].u32, 0 as u32, &mut ctx.xer);
	// 8238C28C: 409A0014  bne cr6, 0x8238c2a0
	if !ctx.cr[6].eq {
	pc = 0x8238C2A0; continue 'dispatch;
	}
	// 8238C290: 38610120  addi r3, r1, 0x120
	ctx.r[3].s64 = ctx.r[1].s64 + 288;
	// 8238C294: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8238C298: 4BFDB9F1  bl 0x82367c88
	ctx.lr = 0x8238C29C;
	sub_82367C88(ctx, base);
	// 8238C29C: 3A810120  addi r20, r1, 0x120
	ctx.r[20].s64 = ctx.r[1].s64 + 288;
	// 8238C2A0: 7E85A378  mr r5, r20
	ctx.r[5].u64 = ctx.r[20].u64;
	// 8238C2A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8238C2A8: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 8238C2AC: 4BFDBA85  bl 0x82367d30
	ctx.lr = 0x8238C2B0;
	sub_82367D30(ctx, base);
	// 8238C2B0: 392100E0  addi r9, r1, 0xe0
	ctx.r[9].s64 = ctx.r[1].s64 + 224;
	// 8238C2B4: 38890030  addi r4, r9, 0x30
	ctx.r[4].s64 = ctx.r[9].s64 + 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238C738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238C738 size=20
    let mut pc: u32 = 0x8238C738;
    'dispatch: loop {
        match pc {
            0x8238C738 => {
    //   block [0x8238C738..0x8238C74C)
	// 8238C738: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238C73C: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 8238C740: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8238C744: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8238C748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238C750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238C750 size=40
    let mut pc: u32 = 0x8238C750;
    'dispatch: loop {
        match pc {
            0x8238C750 => {
    //   block [0x8238C750..0x8238C778)
	// 8238C750: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238C754: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238C758: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8238C75C: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 8238C760: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238C764: 816BBA80  lwz r11, -0x4580(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17792 as u32) ) } as u64;
	// 8238C768: 7DA95C2E  lfsx f13, r9, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238C76C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238C770: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8238C774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238C778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238C778 size=12
    let mut pc: u32 = 0x8238C778;
    'dispatch: loop {
        match pc {
            0x8238C778 => {
    //   block [0x8238C778..0x8238C784)
	// 8238C778: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238C77C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8238C780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238C788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238C788 size=36
    let mut pc: u32 = 0x8238C788;
    'dispatch: loop {
        match pc {
            0x8238C788 => {
    //   block [0x8238C788..0x8238C7AC)
	// 8238C788: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8238C78C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8238C790: 38630050  addi r3, r3, 0x50
	ctx.r[3].s64 = ctx.r[3].s64 + 80;
	// 8238C794: 1D4B0070  mulli r10, r11, 0x70
	ctx.r[10].s64 = ctx.r[11].s64 * 112;
	// 8238C798: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 8238C79C: 816BBA88  lwz r11, -0x4578(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17784 as u32) ) } as u64;
	// 8238C7A0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8238C7A4: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 8238C7A8: 481A83A8  b 0x82534b50
	sub_82534B50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238C7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238C7B0 size=152
    let mut pc: u32 = 0x8238C7B0;
    'dispatch: loop {
        match pc {
            0x8238C7B0 => {
    //   block [0x8238C7B0..0x8238C848)
	// 8238C7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238C7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238C7B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238C7BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238C7C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238C7C4: 89640010  lbz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238C7C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238C7CC: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 8238C7D0: 89640011  lbz r11, 0x11(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(17 as u32) ) } as u64;
	// 8238C7D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238C7D8: 40820010  bne 0x8238c7e8
	if !ctx.cr[0].eq {
	pc = 0x8238C7E8; continue 'dispatch;
	}
	// 8238C7DC: 4BFFCD3D  bl 0x82389518
	ctx.lr = 0x8238C7E0;
	sub_82389518(ctx, base);
	// 8238C7E0: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8238C7E4: 4800004C  b 0x8238c830
	pc = 0x8238C830; continue 'dispatch;
	// 8238C7E8: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8238C7EC: 409A0044  bne cr6, 0x8238c830
	if !ctx.cr[6].eq {
	pc = 0x8238C830; continue 'dispatch;
	}
	// 8238C7F0: 89640001  lbz r11, 1(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 8238C7F4: 2B0B000E  cmplwi cr6, r11, 0xe
	ctx.cr[6].compare_u32(ctx.r[11].u32, 14 as u32, &mut ctx.xer);
	// 8238C7F8: 41980038  blt cr6, 0x8238c830
	if ctx.cr[6].lt {
	pc = 0x8238C830; continue 'dispatch;
	}
	// 8238C7FC: 3BDF00A0  addi r30, r31, 0xa0
	ctx.r[30].s64 = ctx.r[31].s64 + 160;
	// 8238C800: C0240014  lfs f1, 0x14(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8238C804: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8238C808: 4BFDB7B1  bl 0x82367fb8
	ctx.lr = 0x8238C80C;
	sub_82367FB8(ctx, base);
	// 8238C80C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238C810: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8238C814: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238C818: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238C81C: D01F00D8  stfs f0, 0xd8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), tmp.u32 ) };
	// 8238C820: D01F00D4  stfs f0, 0xd4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), tmp.u32 ) };
	// 8238C824: D01F00D0  stfs f0, 0xd0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), tmp.u32 ) };
	// 8238C828: C00BBA38  lfs f0, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238C82C: D01F00DC  stfs f0, 0xdc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), tmp.u32 ) };
	// 8238C830: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238C834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238C838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238C83C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238C840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238C844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238C848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238C848 size=64
    let mut pc: u32 = 0x8238C848;
    'dispatch: loop {
        match pc {
            0x8238C848 => {
    //   block [0x8238C848..0x8238C888)
	// 8238C848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238C84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238C850: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238C854: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238C858: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238C85C: 4800002D  bl 0x8238c888
	ctx.lr = 0x8238C860;
	sub_8238C888(ctx, base);
	// 8238C860: 548B07FF  clrlwi. r11, r4, 0x1f
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238C864: 4182000C  beq 0x8238c870
	if ctx.cr[0].eq {
	pc = 0x8238C870; continue 'dispatch;
	}
	// 8238C868: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8238C86C: 481A634D  bl 0x82532bb8
	ctx.lr = 0x8238C870;
	sub_82532BB8(ctx, base);
	// 8238C870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8238C874: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8238C878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238C87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238C880: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238C884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238C888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238C888 size=96
    let mut pc: u32 = 0x8238C888;
    'dispatch: loop {
        match pc {
            0x8238C888 => {
    //   block [0x8238C888..0x8238C8E8)
	// 8238C888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238C88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238C890: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238C894: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238C898: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8238C89C: 396B0250  addi r11, r11, 0x250
	ctx.r[11].s64 = ctx.r[11].s64 + 592;
	// 8238C8A0: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8238C8A4: 4BFFCC75  bl 0x82389518
	ctx.lr = 0x8238C8A8;
	sub_82389518(ctx, base);
	// 8238C8A8: 3D6082B5  lis r11, -0x7d4b
	ctx.r[11].s64 = -2102067200;
	// 8238C8AC: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 8238C8B0: 396B5C40  addi r11, r11, 0x5c40
	ctx.r[11].s64 = ctx.r[11].s64 + 23616;
	// 8238C8B4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8238C8B8: E9430000  ld r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8238C8BC: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 8238C8C0: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 8238C8C4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8238C8C8: 4200FFF0  bdnz 0x8238c8b8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8238C8B8; continue 'dispatch;
	}
	// 8238C8CC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8238C8D0: 396B3BF8  addi r11, r11, 0x3bf8
	ctx.r[11].s64 = ctx.r[11].s64 + 15352;
	// 8238C8D4: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8238C8D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8238C8DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238C8E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238C8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238C8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238C8E8 size=84
    let mut pc: u32 = 0x8238C8E8;
    'dispatch: loop {
        match pc {
            0x8238C8E8 => {
    //   block [0x8238C8E8..0x8238C93C)
	// 8238C8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238C8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238C8F0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238C8F4: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238C8F8: 38A30010  addi r5, r3, 0x10
	ctx.r[5].s64 = ctx.r[3].s64 + 16;
	// 8238C8FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8238C900: 4BFDB431  bl 0x82367d30
	ctx.lr = 0x8238C904;
	sub_82367D30(ctx, base);
	// 8238C904: 3D6082B5  lis r11, -0x7d4b
	ctx.r[11].s64 = -2102067200;
	// 8238C908: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8238C90C: 394B5C40  addi r10, r11, 0x5c40
	ctx.r[10].s64 = ctx.r[11].s64 + 23616;
	// 8238C910: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8238C914: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8238C918: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8238C91C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8238C920: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8238C924: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8238C928: 4200FFF0  bdnz 0x8238c918
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8238C918; continue 'dispatch;
	}
	// 8238C92C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8238C930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238C934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238C938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238C940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238C940 size=28
    let mut pc: u32 = 0x8238C940;
    'dispatch: loop {
        match pc {
            0x8238C940 => {
    //   block [0x8238C940..0x8238C95C)
	// 8238C940: 89640001  lbz r11, 1(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 8238C944: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 8238C948: 41980014  blt cr6, 0x8238c95c
	if ctx.cr[6].lt {
		sub_8238C95C(ctx, base);
		return;
	}
	// 8238C94C: 38840010  addi r4, r4, 0x10
	ctx.r[4].s64 = ctx.r[4].s64 + 16;
	// 8238C950: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 8238C954: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8238C958: 481A81F8  b 0x82534b50
	sub_82534B50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238C95C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238C95C size=60
    let mut pc: u32 = 0x8238C95C;
    'dispatch: loop {
        match pc {
            0x8238C95C => {
    //   block [0x8238C95C..0x8238C998)
	// 8238C95C: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 8238C960: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 8238C964: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 8238C968: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 8238C96C: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8238C970: 3D403F80  lis r10, 0x3f80
	ctx.r[10].s64 = 1065353216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238C998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238C998 size=92
    let mut pc: u32 = 0x8238C998;
    'dispatch: loop {
        match pc {
            0x8238C998 => {
    //   block [0x8238C998..0x8238C9F4)
	// 8238C998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238C99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238C9A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238C9A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238C9A8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8238C9AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238C9B0: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8238C9B4: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8238C9B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8238C9BC: 41820020  beq 0x8238c9dc
	if ctx.cr[0].eq {
	pc = 0x8238C9DC; continue 'dispatch;
	}
	// 8238C9C0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238C9C4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8238C9C8: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 8238C9CC: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238C9D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8238C9D4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8238C9D8: 480D76E1  bl 0x824640b8
	ctx.lr = 0x8238C9DC;
	sub_824640B8(ctx, base);
	// 8238C9DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8238C9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8238C9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238C9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238C9EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238C9F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238C9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238C9F8 size=80
    let mut pc: u32 = 0x8238C9F8;
    'dispatch: loop {
        match pc {
            0x8238C9F8 => {
    //   block [0x8238C9F8..0x8238CA48)
	// 8238C9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238C9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238CA00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238CA04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238CA08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238CA0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238CA10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8238CA14: 481017ED  bl 0x8248e200
	ctx.lr = 0x8238CA18;
	sub_8248E200(ctx, base);
	// 8238CA18: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 8238CA1C: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 8238CA20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8238CA24: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 8238CA28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238CA2C: 4E800421  bctrl
	ctx.lr = 0x8238CA30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238CA30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238CA34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238CA38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238CA3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238CA40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238CA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238CA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238CA48 size=80
    let mut pc: u32 = 0x8238CA48;
    'dispatch: loop {
        match pc {
            0x8238CA48 => {
    //   block [0x8238CA48..0x8238CA98)
	// 8238CA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238CA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238CA50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238CA54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238CA58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238CA5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238CA60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8238CA64: 4810179D  bl 0x8248e200
	ctx.lr = 0x8238CA68;
	sub_8248E200(ctx, base);
	// 8238CA68: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 8238CA6C: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 8238CA70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8238CA74: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8238CA78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238CA7C: 4E800421  bctrl
	ctx.lr = 0x8238CA80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238CA80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238CA84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238CA88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238CA8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238CA90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238CA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238CA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238CA98 size=80
    let mut pc: u32 = 0x8238CA98;
    'dispatch: loop {
        match pc {
            0x8238CA98 => {
    //   block [0x8238CA98..0x8238CAE8)
	// 8238CA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238CA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238CAA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238CAA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238CAA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238CAAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238CAB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8238CAB4: 4810174D  bl 0x8248e200
	ctx.lr = 0x8238CAB8;
	sub_8248E200(ctx, base);
	// 8238CAB8: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 8238CABC: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 8238CAC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8238CAC4: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 8238CAC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238CACC: 4E800421  bctrl
	ctx.lr = 0x8238CAD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238CAD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238CAD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238CAD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238CADC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238CAE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238CAE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238CAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238CAE8 size=76
    let mut pc: u32 = 0x8238CAE8;
    'dispatch: loop {
        match pc {
            0x8238CAE8 => {
    //   block [0x8238CAE8..0x8238CB34)
	// 8238CAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238CAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238CAF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238CAF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238CAF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238CAFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238CB00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8238CB04: 48001F45  bl 0x8238ea48
	ctx.lr = 0x8238CB08;
	sub_8238EA48(ctx, base);
	// 8238CB08: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238CB0C: 4182000C  beq 0x8238cb18
	if ctx.cr[0].eq {
	pc = 0x8238CB18; continue 'dispatch;
	}
	// 8238CB10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8238CB14: 481A60A5  bl 0x82532bb8
	ctx.lr = 0x8238CB18;
	sub_82532BB8(ctx, base);
	// 8238CB18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8238CB1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238CB20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238CB24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238CB28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238CB2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238CB30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238CB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238CB38 size=324
    let mut pc: u32 = 0x8238CB38;
    'dispatch: loop {
        match pc {
            0x8238CB38 => {
    //   block [0x8238CB38..0x8238CC7C)
	// 8238CB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238CB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238CB40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238CB44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238CB48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238CB4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238CB50: 3FC08273  lis r30, -0x7d8d
	ctx.r[30].s64 = -2106392576;
	// 8238CB54: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 8238CB58: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8238CB5C: 917E49AC  stw r11, 0x49ac(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(18860 as u32), ctx.r[11].u32 ) };
	// 8238CB60: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238CB64: 38600D80  li r3, 0xd80
	ctx.r[3].s64 = 3456;
	// 8238CB68: 916A49B0  stw r11, 0x49b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(18864 as u32), ctx.r[11].u32 ) };
	// 8238CB6C: 817E49AC  lwz r11, 0x49ac(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(18860 as u32) ) } as u64;
	// 8238CB70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238CB74: 4E800421  bctrl
	ctx.lr = 0x8238CB78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238CB78: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238CB7C: 41820018  beq 0x8238cb94
	if ctx.cr[0].eq {
	pc = 0x8238CB94; continue 'dispatch;
	}
	// 8238CB80: 480D8EB9  bl 0x82465a38
	ctx.lr = 0x8238CB84;
	sub_82465A38(ctx, base);
	// 8238CB84: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 8238CB88: 3BEBBFBC  addi r31, r11, -0x4044
	ctx.r[31].s64 = ctx.r[11].s64 + -16452;
	// 8238CB8C: 907FFFF8  stw r3, -8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-8 as u32), ctx.r[3].u32 ) };
	// 8238CB90: 48000014  b 0x8238cba4
	pc = 0x8238CBA4; continue 'dispatch;
	// 8238CB94: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 8238CB98: 3BEBBFBC  addi r31, r11, -0x4044
	ctx.r[31].s64 = ctx.r[11].s64 + -16452;
	// 8238CB9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8238CBA0: 917FFFF8  stw r11, -8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-8 as u32), ctx.r[11].u32 ) };
	// 8238CBA4: 817E49AC  lwz r11, 0x49ac(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(18860 as u32) ) } as u64;
	// 8238CBA8: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8238CBAC: 38600330  li r3, 0x330
	ctx.r[3].s64 = 816;
	// 8238CBB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238CBB4: 4E800421  bctrl
	ctx.lr = 0x8238CBB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238CBB8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238CBBC: 41820018  beq 0x8238cbd4
	if ctx.cr[0].eq {
	pc = 0x8238CBD4; continue 'dispatch;
	}
	// 8238CBC0: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 8238CBC4: 809FFFF8  lwz r4, -8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238CBC8: 480D6F29  bl 0x82463af0
	ctx.lr = 0x8238CBCC;
	sub_82463AF0(ctx, base);
	// 8238CBCC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8238CBD0: 48000008  b 0x8238cbd8
	pc = 0x8238CBD8; continue 'dispatch;
	// 8238CBD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8238CBD8: 807FFFF8  lwz r3, -8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238CBDC: 3D400060  lis r10, 0x60
	ctx.r[10].s64 = 6291456;
	// 8238CBE0: 3D60822B  lis r11, -0x7dd5
	ctx.r[11].s64 = -2111111168;
	// 8238CBE4: 909FFFFC  stw r4, -4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[4].u32 ) };
	// 8238CBE8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8238CBEC: 38AB2F80  addi r5, r11, 0x2f80
	ctx.r[5].s64 = ctx.r[11].s64 + 12160;
	// 8238CBF0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8238CBF4: 480DAA25  bl 0x82467618
	ctx.lr = 0x8238CBF8;
	sub_82467618(ctx, base);
	// 8238CBF8: 807FFFF8  lwz r3, -8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238CBFC: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238CC00: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238CC04: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8238CC08: 40820018  bne 0x8238cc20
	if !ctx.cr[0].eq {
	pc = 0x8238CC20; continue 'dispatch;
	}
	// 8238CC0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238CC10: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8238CC14: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238CC18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238CC1C: 4E800421  bctrl
	ctx.lr = 0x8238CC20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238CC20: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8238CC24: 480D6CFD  bl 0x82463920
	ctx.lr = 0x8238CC28;
	sub_82463920(ctx, base);
	// 8238CC28: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238CC2C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8238CC30: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8238CC34: 3C800020  lis r4, 0x20
	ctx.r[4].s64 = 2097152;
	// 8238CC38: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8238CC3C: 480D762D  bl 0x82464268
	ctx.lr = 0x8238CC40;
	sub_82464268(ctx, base);
	// 8238CC40: 817FFFFC  lwz r11, -4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8238CC44: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8238CC48: 3CA00020  lis r5, 0x20
	ctx.r[5].s64 = 2097152;
	// 8238CC4C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8238CC50: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238CC54: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8238CC58: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238CC5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238CC60: 4E800421  bctrl
	ctx.lr = 0x8238CC64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238CC64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238CC68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238CC6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238CC70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238CC74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238CC78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238CC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238CC80 size=228
    let mut pc: u32 = 0x8238CC80;
    'dispatch: loop {
        match pc {
            0x8238CC80 => {
    //   block [0x8238CC80..0x8238CD64)
	// 8238CC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238CC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238CC88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238CC8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238CC90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238CC94: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 8238CC98: 3BEBBFBC  addi r31, r11, -0x4044
	ctx.r[31].s64 = ctx.r[11].s64 + -16452;
	// 8238CC9C: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8238CCA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238CCA4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238CCA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238CCAC: 4E800421  bctrl
	ctx.lr = 0x8238CCB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238CCB0: 807FFFF8  lwz r3, -8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238CCB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238CCB8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8238CCBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238CCC0: 4E800421  bctrl
	ctx.lr = 0x8238CCC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238CCC4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238CCC8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8238CCCC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238CCD0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8238CCD4: 480D75ED  bl 0x824642c0
	ctx.lr = 0x8238CCD8;
	sub_824642C0(ctx, base);
	// 8238CCD8: 480DAA79  bl 0x82467750
	ctx.lr = 0x8238CCDC;
	sub_82467750(ctx, base);
	// 8238CCDC: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8238CCE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8238CCE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238CCE8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238CCEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238CCF0: 4E800421  bctrl
	ctx.lr = 0x8238CCF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238CCF4: 807FFFF8  lwz r3, -8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238CCF8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8238CCFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238CD00: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238CD04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238CD08: 4E800421  bctrl
	ctx.lr = 0x8238CD0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238CD0C: 3FC08273  lis r30, -0x7d8d
	ctx.r[30].s64 = -2106392576;
	// 8238CD10: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8238CD14: 817E49B0  lwz r11, 0x49b0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(18864 as u32) ) } as u64;
	// 8238CD18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238CD1C: 4E800421  bctrl
	ctx.lr = 0x8238CD20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238CD20: 807FFFF8  lwz r3, -8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238CD24: 817E49B0  lwz r11, 0x49b0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(18864 as u32) ) } as u64;
	// 8238CD28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238CD2C: 4E800421  bctrl
	ctx.lr = 0x8238CD30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238CD30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8238CD34: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 8238CD38: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8238CD3C: 917FFFFC  stw r11, -4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 8238CD40: 917FFFF8  stw r11, -8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-8 as u32), ctx.r[11].u32 ) };
	// 8238CD44: 916A49AC  stw r11, 0x49ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(18860 as u32), ctx.r[11].u32 ) };
	// 8238CD48: 917E49B0  stw r11, 0x49b0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(18864 as u32), ctx.r[11].u32 ) };
	// 8238CD4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238CD50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238CD54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238CD58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238CD5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238CD60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238CD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238CD68 size=272
    let mut pc: u32 = 0x8238CD68;
    'dispatch: loop {
        match pc {
            0x8238CD68 => {
    //   block [0x8238CD68..0x8238CE78)
	// 8238CD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238CD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238CD70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238CD74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238CD78: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238CD7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8238CD80: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8238CD84: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 8238CD88: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238CD8C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238CD90: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238CD94: 419A0070  beq cr6, 0x8238ce04
	if ctx.cr[6].eq {
	pc = 0x8238CE04; continue 'dispatch;
	}
	// 8238CD98: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8238CD9C: 816B49AC  lwz r11, 0x49ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18860 as u32) ) } as u64;
	// 8238CDA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238CDA4: 4E800421  bctrl
	ctx.lr = 0x8238CDA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238CDA8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8238CDAC: 418200B0  beq 0x8238ce5c
	if ctx.cr[0].eq {
	pc = 0x8238CE5C; continue 'dispatch;
	}
	// 8238CDB0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238CDB4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238CDB8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8238CDBC: 396B0290  addi r11, r11, 0x290
	ctx.r[11].s64 = ctx.r[11].s64 + 656;
	// 8238CDC0: 38A0002F  li r5, 0x2f
	ctx.r[5].s64 = 47;
	// 8238CDC4: 38800310  li r4, 0x310
	ctx.r[4].s64 = 784;
	// 8238CDC8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8238CDCC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8238CDD0: 83DE000C  lwz r30, 0xc(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238CDD4: 480D7265  bl 0x82464038
	ctx.lr = 0x8238CDD8;
	sub_82464038(ctx, base);
	// 8238CDD8: 39600310  li r11, 0x310
	ctx.r[11].s64 = 784;
	// 8238CDDC: 3CA00000  lis r5, 0
	ctx.r[5].s64 = 0;
	// 8238CDE0: 60A5C544  ori r5, r5, 0xc544
	ctx.r[5].u64 = ctx.r[5].u64 | 50500;
	// 8238CDE4: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8238CDE8: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238CDEC: 48108EA5  bl 0x82495c90
	ctx.lr = 0x8238CDF0;
	sub_82495C90(ctx, base);
	// 8238CDF0: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8238CDF4: 80630074  lwz r3, 0x74(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) } as u64;
	// 8238CDF8: 4816FD81  bl 0x824fcb78
	ctx.lr = 0x8238CDFC;
	sub_824FCB78(ctx, base);
	// 8238CDFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8238CE00: 48000060  b 0x8238ce60
	pc = 0x8238CE60; continue 'dispatch;
	// 8238CE04: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238CE08: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 8238CE0C: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238CE10: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238CE14: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8238CE18: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8238CE1C: C1AB2224  lfs f13, 0x2224(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8740 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238CE20: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238CE24: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8238CE28: C00BBA38  lfs f0, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238CE2C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238CE30: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8238CE34: C00B204C  lfs f0, 0x204c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8268 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238CE38: 816A49AC  lwz r11, 0x49ac(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(18860 as u32) ) } as u64;
	// 8238CE3C: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8238CE40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238CE44: 4E800421  bctrl
	ctx.lr = 0x8238CE48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238CE48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238CE4C: 41820010  beq 0x8238ce5c
	if ctx.cr[0].eq {
	pc = 0x8238CE5C; continue 'dispatch;
	}
	// 8238CE50: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8238CE54: 4800085D  bl 0x8238d6b0
	ctx.lr = 0x8238CE58;
	sub_8238D6B0(ctx, base);
	// 8238CE58: 48000008  b 0x8238ce60
	pc = 0x8238CE60; continue 'dispatch;
	// 8238CE5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8238CE60: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8238CE64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238CE68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238CE6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238CE70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238CE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238CE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238CE78 size=244
    let mut pc: u32 = 0x8238CE78;
    'dispatch: loop {
        match pc {
            0x8238CE78 => {
    //   block [0x8238CE78..0x8238CF6C)
	// 8238CE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238CE7C: 481A8241  bl 0x825350bc
	ctx.lr = 0x8238CE80;
	sub_82535080(ctx, base);
	// 8238CE80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238CE84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8238CE88: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8238CE8C: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8238CE90: 817D0054  lwz r11, 0x54(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 8238CE94: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8238CE98: 41980068  blt cr6, 0x8238cf00
	if ctx.cr[6].lt {
	pc = 0x8238CF00; continue 'dispatch;
	}
	// 8238CE9C: 419A0038  beq cr6, 0x8238ced4
	if ctx.cr[6].eq {
	pc = 0x8238CED4; continue 'dispatch;
	}
	// 8238CEA0: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8238CEA4: 409800BC  bge cr6, 0x8238cf60
	if !ctx.cr[6].lt {
	pc = 0x8238CF60; continue 'dispatch;
	}
	// 8238CEA8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8238CEAC: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8238CEB0: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8238CEB4: 816B49AC  lwz r11, 0x49ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18860 as u32) ) } as u64;
	// 8238CEB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238CEBC: 4E800421  bctrl
	ctx.lr = 0x8238CEC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238CEC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238CEC4: 41820078  beq 0x8238cf3c
	if ctx.cr[0].eq {
	pc = 0x8238CF3C; continue 'dispatch;
	}
	// 8238CEC8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238CECC: 396B02BC  addi r11, r11, 0x2bc
	ctx.r[11].s64 = ctx.r[11].s64 + 700;
	// 8238CED0: 48000058  b 0x8238cf28
	pc = 0x8238CF28; continue 'dispatch;
	// 8238CED4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8238CED8: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8238CEDC: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8238CEE0: 816B49AC  lwz r11, 0x49ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18860 as u32) ) } as u64;
	// 8238CEE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238CEE8: 4E800421  bctrl
	ctx.lr = 0x8238CEEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238CEEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238CEF0: 4182004C  beq 0x8238cf3c
	if ctx.cr[0].eq {
	pc = 0x8238CF3C; continue 'dispatch;
	}
	// 8238CEF4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238CEF8: 396B02A8  addi r11, r11, 0x2a8
	ctx.r[11].s64 = ctx.r[11].s64 + 680;
	// 8238CEFC: 4800002C  b 0x8238cf28
	pc = 0x8238CF28; continue 'dispatch;
	// 8238CF00: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8238CF04: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8238CF08: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8238CF0C: 816B49AC  lwz r11, 0x49ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18860 as u32) ) } as u64;
	// 8238CF10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238CF14: 4E800421  bctrl
	ctx.lr = 0x8238CF18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238CF18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238CF1C: 41820020  beq 0x8238cf3c
	if ctx.cr[0].eq {
	pc = 0x8238CF3C; continue 'dispatch;
	}
	// 8238CF20: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238CF24: 396B0294  addi r11, r11, 0x294
	ctx.r[11].s64 = ctx.r[11].s64 + 660;
	// 8238CF28: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8238CF2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238CF30: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8238CF34: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8238CF38: 48000008  b 0x8238cf40
	pc = 0x8238CF40; continue 'dispatch;
	// 8238CF3C: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8238CF40: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8238CF44: 419A001C  beq cr6, 0x8238cf60
	if ctx.cr[6].eq {
	pc = 0x8238CF60; continue 'dispatch;
	}
	// 8238CF48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238CF4C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8238CF50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8238CF54: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238CF58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238CF5C: 4E800421  bctrl
	ctx.lr = 0x8238CF60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238CF60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8238CF64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238CF68: 481A81A4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238CF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238CF70 size=76
    let mut pc: u32 = 0x8238CF70;
    'dispatch: loop {
        match pc {
            0x8238CF70 => {
    //   block [0x8238CF70..0x8238CFBC)
	// 8238CF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238CF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238CF78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238CF7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238CF80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238CF84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238CF88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8238CF8C: 480009C5  bl 0x8238d950
	ctx.lr = 0x8238CF90;
	sub_8238D950(ctx, base);
	// 8238CF90: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238CF94: 4182000C  beq 0x8238cfa0
	if ctx.cr[0].eq {
	pc = 0x8238CFA0; continue 'dispatch;
	}
	// 8238CF98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8238CF9C: 481A5C1D  bl 0x82532bb8
	ctx.lr = 0x8238CFA0;
	sub_82532BB8(ctx, base);
	// 8238CFA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8238CFA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238CFA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238CFAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238CFB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238CFB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238CFB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238CFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238CFC0 size=412
    let mut pc: u32 = 0x8238CFC0;
    'dispatch: loop {
        match pc {
            0x8238CFC0 => {
    //   block [0x8238CFC0..0x8238D15C)
	// 8238CFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238CFC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238CFC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238CFCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238CFD0: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238CFD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238CFD8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8238CFDC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238CFE0: 916100BC  stw r11, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[11].u32 ) };
	// 8238CFE4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238CFE8: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238CFEC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8238CFF0: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8238CFF4: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8238CFF8: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 8238CFFC: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8238D000: 916100AC  stw r11, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 8238D004: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 8238D008: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8238D00C: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 8238D010: D0010090  stfs f0, 0x90(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8238D014: D0010094  stfs f0, 0x94(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8238D018: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 8238D01C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8238D020: D0010098  stfs f0, 0x98(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8238D024: 916100B8  stw r11, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 8238D028: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238D02C: C1ABBA38  lfs f13, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238D030: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8238D034: D1A1006C  stfs f13, 0x6c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8238D038: D1A1007C  stfs f13, 0x7c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8238D03C: D1A10080  stfs f13, 0x80(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 8238D040: D1A10084  stfs f13, 0x84(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 8238D044: 916100B4  stw r11, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 8238D048: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238D04C: D1A10088  stfs f13, 0x88(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 8238D050: D1A1008C  stfs f13, 0x8c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 8238D054: D1A1009C  stfs f13, 0x9c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 8238D058: D1A100A8  stfs f13, 0xa8(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 8238D05C: C00BBFFC  lfs f0, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238D060: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8238D064: D00100A4  stfs f0, 0xa4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 8238D068: C00BD218  lfs f0, -0x2de8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11752 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238D06C: D00100A0  stfs f0, 0xa0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 8238D070: 4BFFFE09  bl 0x8238ce78
	ctx.lr = 0x8238D074;
	sub_8238CE78(ctx, base);
	// 8238D074: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8238D078: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238D07C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238D080: C00B01A0  lfs f0, 0x1a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(416 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238D084: C1AB01A4  lfs f13, 0x1a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(420 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238D088: C18B01A8  lfs f12, 0x1a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(424 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238D08C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8238D090: C16B01AC  lfs f11, 0x1ac(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(428 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8238D094: 419A0030  beq cr6, 0x8238d0c4
	if ctx.cr[6].eq {
	pc = 0x8238D0C4; continue 'dispatch;
	}
	// 8238D098: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8238D09C: 396100C0  addi r11, r1, 0xc0
	ctx.r[11].s64 = ctx.r[1].s64 + 192;
	// 8238D0A0: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8238D0A4: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 8238D0A8: D1810058  stfs f12, 0x58(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8238D0AC: D161005C  stfs f11, 0x5c(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8238D0B0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238D160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238D160 size=80
    let mut pc: u32 = 0x8238D160;
    'dispatch: loop {
        match pc {
            0x8238D160 => {
    //   block [0x8238D160..0x8238D1B0)
	// 8238D160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238D164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238D168: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238D16C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238D170: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238D174: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8238D178: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238D17C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238D180: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238D184: 4E800421  bctrl
	ctx.lr = 0x8238D188;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238D188: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8238D18C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8238D190: 816B49B0  lwz r11, 0x49b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18864 as u32) ) } as u64;
	// 8238D194: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238D198: 4E800421  bctrl
	ctx.lr = 0x8238D19C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238D19C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8238D1A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238D1A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238D1A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238D1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238D1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238D1B0 size=168
    let mut pc: u32 = 0x8238D1B0;
    'dispatch: loop {
        match pc {
            0x8238D1B0 => {
    //   block [0x8238D1B0..0x8238D258)
	// 8238D1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238D1B4: 481A7F05  bl 0x825350b8
	ctx.lr = 0x8238D1B8;
	sub_82535080(ctx, base);
	// 8238D1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238D1BC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8238D1C0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8238D1C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8238D1C8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8238D1CC: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8238D1D0: 816B49AC  lwz r11, 0x49ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18860 as u32) ) } as u64;
	// 8238D1D4: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 8238D1D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238D1DC: 4E800421  bctrl
	ctx.lr = 0x8238D1E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238D1E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8238D1E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238D1E8: 41820018  beq 0x8238d200
	if ctx.cr[0].eq {
	pc = 0x8238D200; continue 'dispatch;
	}
	// 8238D1EC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238D1F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238D1F4: 396B0290  addi r11, r11, 0x290
	ctx.r[11].s64 = ctx.r[11].s64 + 656;
	// 8238D1F8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8238D1FC: 48000008  b 0x8238d204
	pc = 0x8238D204; continue 'dispatch;
	// 8238D200: 7D5F5378  mr r31, r10
	ctx.r[31].u64 = ctx.r[10].u64;
	// 8238D204: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 8238D208: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8238D20C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8238D210: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8238D214: 419A000C  beq cr6, 0x8238d220
	if ctx.cr[6].eq {
	pc = 0x8238D220; continue 'dispatch;
	}
	// 8238D218: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238D21C: 48000008  b 0x8238d224
	pc = 0x8238D224; continue 'dispatch;
	// 8238D220: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8238D224: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8238D228: 419A000C  beq cr6, 0x8238d234
	if ctx.cr[6].eq {
	pc = 0x8238D234; continue 'dispatch;
	}
	// 8238D22C: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238D230: 48000008  b 0x8238d238
	pc = 0x8238D238; continue 'dispatch;
	// 8238D234: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8238D238: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8238D23C: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238D240: 48104FA1  bl 0x824921e0
	ctx.lr = 0x8238D244;
	sub_824921E0(ctx, base);
	// 8238D244: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8238D248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8238D24C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8238D250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8238D254: 481A7EB4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238D258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238D258 size=144
    let mut pc: u32 = 0x8238D258;
    'dispatch: loop {
        match pc {
            0x8238D258 => {
    //   block [0x8238D258..0x8238D2E8)
	// 8238D258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238D25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238D260: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238D264: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238D268: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238D26C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8238D270: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8238D274: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8238D278: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8238D27C: 816B49AC  lwz r11, 0x49ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18860 as u32) ) } as u64;
	// 8238D280: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238D284: 4E800421  bctrl
	ctx.lr = 0x8238D288;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238D288: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238D28C: 41820028  beq 0x8238d2b4
	if ctx.cr[0].eq {
	pc = 0x8238D2B4; continue 'dispatch;
	}
	// 8238D290: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238D294: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238D298: 394B02D0  addi r10, r11, 0x2d0
	ctx.r[10].s64 = ctx.r[11].s64 + 720;
	// 8238D29C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8238D2A0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8238D2A4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8238D2A8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8238D2AC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8238D2B0: 48000008  b 0x8238d2b8
	pc = 0x8238D2B8; continue 'dispatch;
	// 8238D2B4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8238D2B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8238D2BC: 419A0010  beq cr6, 0x8238d2cc
	if ctx.cr[6].eq {
	pc = 0x8238D2CC; continue 'dispatch;
	}
	// 8238D2C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8238D2C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8238D2C8: 48001221  bl 0x8238e4e8
	ctx.lr = 0x8238D2CC;
	sub_8238E4E8(ctx, base);
	// 8238D2CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8238D2D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238D2D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238D2D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238D2DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238D2E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238D2E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238D2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238D2E8 size=276
    let mut pc: u32 = 0x8238D2E8;
    'dispatch: loop {
        match pc {
            0x8238D2E8 => {
    //   block [0x8238D2E8..0x8238D3FC)
	// 8238D2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238D2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238D2F0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238D2F4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8238D2F8: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 8238D2FC: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8238D300: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8238D304: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8238D308: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8238D30C: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8238D310: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8238D314: 4200FFF0  bdnz 0x8238d304
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8238D304; continue 'dispatch;
	}
	// 8238D318: C0010070  lfs f0, 0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238D31C: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 8238D320: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8238D324: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8238D328: C0010074  lfs f0, 0x74(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238D32C: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8238D330: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8238D334: C0010078  lfs f0, 0x78(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238D338: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8238D33C: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 8238D340: C0010080  lfs f0, 0x80(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238D344: 816BBFA0  lwz r11, -0x4060(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16480 as u32) ) } as u64;
	// 8238D348: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8238D34C: D0010080  stfs f0, 0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 8238D350: C0010084  lfs f0, 0x84(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238D354: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8238D358: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8238D35C: D0010084  stfs f0, 0x84(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 8238D360: C0010088  lfs f0, 0x88(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238D364: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8238D368: D0010088  stfs f0, 0x88(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 8238D36C: C0010090  lfs f0, 0x90(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238D370: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8238D374: D0010090  stfs f0, 0x90(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8238D378: C0010094  lfs f0, 0x94(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238D37C: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8238D380: D0010094  stfs f0, 0x94(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8238D384: C0010098  lfs f0, 0x98(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238D388: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8238D38C: D0010098  stfs f0, 0x98(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8238D390: 419A005C  beq cr6, 0x8238d3ec
	if ctx.cr[6].eq {
	pc = 0x8238D3EC; continue 'dispatch;
	}
	// 8238D394: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 8238D398: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8238D39C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8238D3A0: 396BBF80  addi r11, r11, -0x4080
	ctx.r[11].s64 = ctx.r[11].s64 + -16512;
	// 8238D3A4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8238D3A8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238D400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238D400 size=688
    let mut pc: u32 = 0x8238D400;
    'dispatch: loop {
        match pc {
            0x8238D400 => {
    //   block [0x8238D400..0x8238D6B0)
	// 8238D400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238D404: 481A7CB9  bl 0x825350bc
	ctx.lr = 0x8238D408;
	sub_82535080(ctx, base);
	// 8238D408: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8238D40C: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238D410: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8238D414: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8238D418: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8238D41C: 394100E0  addi r10, r1, 0xe0
	ctx.r[10].s64 = ctx.r[1].s64 + 224;
	// 8238D420: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 8238D424: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238D428: E9630000  ld r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8238D42C: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 8238D430: F96A0000  std r11, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8238D434: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8238D438: 4200FFF0  bdnz 0x8238d428
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8238D428; continue 'dispatch;
	}
	// 8238D43C: E9040000  ld r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 8238D440: 396100D0  addi r11, r1, 0xd0
	ctx.r[11].s64 = ctx.r[1].s64 + 208;
	// 8238D444: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 8238D448: E8E40008  ld r7, 8(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 8238D44C: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8238D450: F90B0000  std r8, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 8238D454: C00ABA38  lfs f0, -0x45c8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238D458: F8EB0008  std r7, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[7].u64 ) };
	// 8238D45C: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8238D460: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8238D464: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8238D468: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8238D46C: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 8238D470: D001009C  stfs f0, 0x9c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 8238D474: D00100AC  stfs f0, 0xac(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 8238D478: D00100BC  stfs f0, 0xbc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 8238D47C: D00100CC  stfs f0, 0xcc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 8238D480: C00100D0  lfs f0, 0xd0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238D484: C18100D4  lfs f12, 0xd4(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238D488: FDA00050  fneg f13, f0
	ctx.f[13].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 8238D48C: C14100D8  lfs f10, 0xd8(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(216 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8238D490: FD606050  fneg f11, f12
	ctx.f[11].u64 = ctx.f[12].u64 ^ 0x8000_0000_0000_0000u64;
	// 8238D494: FD205050  fneg f9, f10
	ctx.f[9].u64 = ctx.f[10].u64 ^ 0x8000_0000_0000_0000u64;
	// 8238D498: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8238D49C: D1810074  stfs f12, 0x74(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8238D4A0: D0010080  stfs f0, 0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 8238D4A4: D1810084  stfs f12, 0x84(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 8238D4A8: D1410098  stfs f10, 0x98(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8238D4AC: D00100A0  stfs f0, 0xa0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 8238D4B0: D14100A8  stfs f10, 0xa8(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 8238D4B4: D18100B4  stfs f12, 0xb4(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 8238D4B8: D14100B8  stfs f10, 0xb8(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), tmp.u32 ) };
	// 8238D4BC: D00100C0  stfs f0, 0xc0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 8238D4C0: D18100C4  stfs f12, 0xc4(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 8238D4C4: D14100C8  stfs f10, 0xc8(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), tmp.u32 ) };
	// 8238D4C8: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8238D4CC: D1610054  stfs f11, 0x54(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8238D4D0: D1210058  stfs f9, 0x58(r1)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8238D4D4: D1610064  stfs f11, 0x64(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8238D4D8: D1210068  stfs f9, 0x68(r1)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 8238D4DC: D1A10070  stfs f13, 0x70(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8238D4E0: D1210078  stfs f9, 0x78(r1)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 8238D4E4: D1210088  stfs f9, 0x88(r1)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 8238D4E8: D1A10090  stfs f13, 0x90(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8238D4EC: D1610094  stfs f11, 0x94(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8238D4F0: D16100A4  stfs f11, 0xa4(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 8238D4F4: D1A100B0  stfs f13, 0xb0(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), tmp.u32 ) };
	// 8238D4F8: 39610110  addi r11, r1, 0x110
	ctx.r[11].s64 = ctx.r[1].s64 + 272;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238D6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238D6B0 size=236
    let mut pc: u32 = 0x8238D6B0;
    'dispatch: loop {
        match pc {
            0x8238D6B0 => {
    //   block [0x8238D6B0..0x8238D79C)
	// 8238D6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238D6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238D6B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238D6BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238D6C0: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238D6C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8238D6C8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238D6CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238D6D0: 396B0290  addi r11, r11, 0x290
	ctx.r[11].s64 = ctx.r[11].s64 + 656;
	// 8238D6D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8238D6D8: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238D6DC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8238D6E0: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8238D6E4: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238D6E8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8238D6EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8238D6F0: C01E0008  lfs f0, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238D6F4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8238D6F8: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8238D6FC: C01E000C  lfs f0, 0xc(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238D700: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8238D704: 4810D98D  bl 0x8249b090
	ctx.lr = 0x8238D708;
	sub_8249B090(ctx, base);
	// 8238D708: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8238D70C: C03E0010  lfs f1, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8238D710: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8238D714: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8238D718: 9961011D  stb r11, 0x11d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(285 as u32), ctx.r[11].u8 ) };
	// 8238D71C: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238D7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238D7A0 size=284
    let mut pc: u32 = 0x8238D7A0;
    'dispatch: loop {
        match pc {
            0x8238D7A0 => {
    //   block [0x8238D7A0..0x8238D8BC)
	// 8238D7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238D7A4: 481A790D  bl 0x825350b0
	ctx.lr = 0x8238D7A8;
	sub_82535080(ctx, base);
	// 8238D7A8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238D7AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238D7B0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8238D7B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8238D7B8: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 8238D7BC: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 8238D7C0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8238D7C4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8238D7C8: 480D6871  bl 0x82464038
	ctx.lr = 0x8238D7CC;
	sub_82464038(ctx, base);
	// 8238D7CC: 39600100  li r11, 0x100
	ctx.r[11].s64 = 256;
	// 8238D7D0: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8238D7D4: 4816FFA5  bl 0x824fd778
	ctx.lr = 0x8238D7D8;
	sub_824FD778(ctx, base);
	// 8238D7D8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238D7DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8238D7E0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8238D7E4: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8238D7E8: 40810070  ble 0x8238d858
	if !ctx.cr[0].gt {
	pc = 0x8238D858; continue 'dispatch;
	}
	// 8238D7EC: 3B7E0004  addi r27, r30, 4
	ctx.r[27].s64 = ctx.r[30].s64 + 4;
	// 8238D7F0: 3B9E0048  addi r28, r30, 0x48
	ctx.r[28].s64 = ctx.r[30].s64 + 72;
	// 8238D7F4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8238D7F8: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8238D7FC: 40980008  bge cr6, 0x8238d804
	if !ctx.cr[6].lt {
	pc = 0x8238D804; continue 'dispatch;
	}
	// 8238D800: E95C0000  ld r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	// 8238D804: 79490022  rldicl r9, r10, 0x20, 0x20
	ctx.r[9].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 8238D808: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8238D80C: 5545003E  slwi r5, r10, 0
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8238D810: 5524003E  slwi r4, r9, 0
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8238D814: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8238D818: 40980008  bge cr6, 0x8238d820
	if !ctx.cr[6].lt {
	pc = 0x8238D820; continue 'dispatch;
	}
	// 8238D81C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238D820: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8238D824: 41980014  blt cr6, 0x8238d838
	if ctx.cr[6].lt {
	pc = 0x8238D838; continue 'dispatch;
	}
	// 8238D828: 409A0018  bne cr6, 0x8238d840
	if !ctx.cr[6].eq {
	pc = 0x8238D840; continue 'dispatch;
	}
	// 8238D82C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8238D830: 4816FEF1  bl 0x824fd720
	ctx.lr = 0x8238D834;
	sub_824FD720(ctx, base);
	// 8238D834: 4800000C  b 0x8238d840
	pc = 0x8238D840; continue 'dispatch;
	// 8238D838: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8238D83C: 4816FE1D  bl 0x824fd658
	ctx.lr = 0x8238D840;
	sub_824FD658(ctx, base);
	// 8238D840: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238D844: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8238D848: 3B9C0008  addi r28, r28, 8
	ctx.r[28].s64 = ctx.r[28].s64 + 8;
	// 8238D84C: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 8238D850: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8238D854: 4198FFA0  blt cr6, 0x8238d7f4
	if ctx.cr[6].lt {
	pc = 0x8238D7F4; continue 'dispatch;
	}
	// 8238D858: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8238D85C: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238D860: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8238D864: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8238D868: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8238D86C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8238D870: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 8238D874: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238D878: 4810D179  bl 0x8249a9f0
	ctx.lr = 0x8238D87C;
	sub_8249A9F0(ctx, base);
	// 8238D87C: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238D880: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238D884: 41820030  beq 0x8238d8b4
	if ctx.cr[0].eq {
	pc = 0x8238D8B4; continue 'dispatch;
	}
	// 8238D888: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 8238D88C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238D890: 7D6B0735  extsh. r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238D894: B17D0006  sth r11, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8238D898: 4082001C  bne 0x8238d8b4
	if !ctx.cr[0].eq {
	pc = 0x8238D8B4; continue 'dispatch;
	}
	// 8238D89C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238D8A0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8238D8A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8238D8A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238D8AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238D8B0: 4E800421  bctrl
	ctx.lr = 0x8238D8B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238D8B4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8238D8B8: 481A7848  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238D8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238D8C0 size=140
    let mut pc: u32 = 0x8238D8C0;
    'dispatch: loop {
        match pc {
            0x8238D8C0 => {
    //   block [0x8238D8C0..0x8238D94C)
	// 8238D8C0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238D8C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8238D8C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8238D8CC: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8238D8D0: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238D8D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238D8D8: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8238D8DC: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8238D8E0: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8238D8E4: C1ABBA38  lfs f13, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238D8E8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238D8EC: D1A3000C  stfs f13, 0xc(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8238D8F0: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8238D8F4: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8238D8F8: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8238D8FC: C18BBFFC  lfs f12, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238D900: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8238D904: D1A3001C  stfs f13, 0x1c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8238D908: D1A30020  stfs f13, 0x20(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8238D90C: D1A30024  stfs f13, 0x24(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8238D910: D1A30028  stfs f13, 0x28(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8238D914: D1A3002C  stfs f13, 0x2c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8238D918: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8238D91C: D0030034  stfs f0, 0x34(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 8238D920: D0030038  stfs f0, 0x38(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 8238D924: D1A3003C  stfs f13, 0x3c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 8238D928: 9123004C  stw r9, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[9].u32 ) };
	// 8238D92C: C16BD218  lfs f11, -0x2de8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11752 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8238D930: 91030050  stw r8, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 8238D934: D1A30048  stfs f13, 0x48(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 8238D938: 91430054  stw r10, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8238D93C: D1830044  stfs f12, 0x44(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 8238D940: 91430058  stw r10, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8238D944: D1630040  stfs f11, 0x40(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 8238D948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238D950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238D950 size=152
    let mut pc: u32 = 0x8238D950;
    'dispatch: loop {
        match pc {
            0x8238D950 => {
    //   block [0x8238D950..0x8238D9E8)
	// 8238D950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238D954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238D958: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238D95C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238D960: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238D964: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238D968: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238D96C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8238D970: 396B027C  addi r11, r11, 0x27c
	ctx.r[11].s64 = ctx.r[11].s64 + 636;
	// 8238D974: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238D978: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238D97C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8238D980: 4182000C  beq 0x8238d98c
	if ctx.cr[0].eq {
	pc = 0x8238D98C; continue 'dispatch;
	}
	// 8238D984: 4810E0CD  bl 0x8249ba50
	ctx.lr = 0x8238D988;
	sub_8249BA50(ctx, base);
	// 8238D988: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8238D98C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238D990: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238D994: 4182003C  beq 0x8238d9d0
	if ctx.cr[0].eq {
	pc = 0x8238D9D0; continue 'dispatch;
	}
	// 8238D998: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238D99C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238D9A0: 4182002C  beq 0x8238d9cc
	if ctx.cr[0].eq {
	pc = 0x8238D9CC; continue 'dispatch;
	}
	// 8238D9A4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8238D9A8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238D9AC: 7D6B0735  extsh. r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238D9B0: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8238D9B4: 40820018  bne 0x8238d9cc
	if !ctx.cr[0].eq {
	pc = 0x8238D9CC; continue 'dispatch;
	}
	// 8238D9B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238D9BC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8238D9C0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238D9C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238D9C8: 4E800421  bctrl
	ctx.lr = 0x8238D9CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238D9CC: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8238D9D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238D9D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238D9D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238D9DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238D9E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238D9E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238D9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238D9E8 size=100
    let mut pc: u32 = 0x8238D9E8;
    'dispatch: loop {
        match pc {
            0x8238D9E8 => {
    //   block [0x8238D9E8..0x8238DA4C)
	// 8238D9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238D9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238D9F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238D9F4: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238D9F8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8238D9FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238DA00: 4182003C  beq 0x8238da3c
	if ctx.cr[0].eq {
	pc = 0x8238DA3C; continue 'dispatch;
	}
	// 8238DA04: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DA08: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8238DA0C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8238DA10: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DA14: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8238DA18: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DA1C: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8238DA20: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DA24: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8238DA28: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8238DA2C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238DA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238DA50 size=160
    let mut pc: u32 = 0x8238DA50;
    'dispatch: loop {
        match pc {
            0x8238DA50 => {
    //   block [0x8238DA50..0x8238DAF0)
	// 8238DA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238DA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238DA58: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238DA5C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238DA60: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238DA64: 4182007C  beq 0x8238dae0
	if ctx.cr[0].eq {
	pc = 0x8238DAE0; continue 'dispatch;
	}
	// 8238DA68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8238DA6C: C1240030  lfs f9, 0x30(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8238DA70: C1040034  lfs f8, 0x34(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 8238DA74: C0E40038  lfs f7, 0x38(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 8238DA78: 4BFE6929  bl 0x823743a0
	ctx.lr = 0x8238DA7C;
	sub_823743A0(ctx, base);
	// 8238DA7C: C001008C  lfs f0, 0x8c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DA80: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 8238DA84: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8238DA88: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 8238DA8C: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DA90: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8238DA94: D1210060  stfs f9, 0x60(r1)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8238DA98: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 8238DA9C: D1010064  stfs f8, 0x64(r1)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8238DAA0: D0E10068  stfs f7, 0x68(r1)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 8238DAA4: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8238DAA8: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8238DAAC: C0010054  lfs f0, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DAB0: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8238DAB4: C0010058  lfs f0, 0x58(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DAB8: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 8238DABC: C001005C  lfs f0, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238DAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238DAF0 size=100
    let mut pc: u32 = 0x8238DAF0;
    'dispatch: loop {
        match pc {
            0x8238DAF0 => {
    //   block [0x8238DAF0..0x8238DB54)
	// 8238DAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238DAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238DAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238DAFC: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238DB00: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8238DB04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238DB08: 4182003C  beq 0x8238db44
	if ctx.cr[0].eq {
	pc = 0x8238DB44; continue 'dispatch;
	}
	// 8238DB0C: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DB10: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8238DB14: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8238DB18: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DB1C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8238DB20: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DB24: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8238DB28: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DB2C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8238DB30: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8238DB34: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238DB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238DB58 size=100
    let mut pc: u32 = 0x8238DB58;
    'dispatch: loop {
        match pc {
            0x8238DB58 => {
    //   block [0x8238DB58..0x8238DBBC)
	// 8238DB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238DB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238DB60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238DB64: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238DB68: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8238DB6C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238DB70: 4182003C  beq 0x8238dbac
	if ctx.cr[0].eq {
	pc = 0x8238DBAC; continue 'dispatch;
	}
	// 8238DB74: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DB78: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8238DB7C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8238DB80: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DB84: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8238DB88: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DB8C: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8238DB90: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DB94: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8238DB98: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8238DB9C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238DBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238DBC0 size=104
    let mut pc: u32 = 0x8238DBC0;
    'dispatch: loop {
        match pc {
            0x8238DBC0 => {
    //   block [0x8238DBC0..0x8238DC28)
	// 8238DBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238DBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238DBC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238DBCC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238DBD0: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238DBD4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8238DBD8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238DBDC: 41820038  beq 0x8238dc14
	if ctx.cr[0].eq {
	pc = 0x8238DC14; continue 'dispatch;
	}
	// 8238DBE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238DBE4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8238DBE8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8238DBEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238DBF0: 4E800421  bctrl
	ctx.lr = 0x8238DBF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238DBF4: C0010050  lfs f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DBF8: C1A10054  lfs f13, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238DBFC: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8238DC00: D1BF0004  stfs f13, 4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8238DC04: C0010058  lfs f0, 0x58(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DC08: C1A1005C  lfs f13, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238DC0C: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8238DC10: D1BF000C  stfs f13, 0xc(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8238DC14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238DC18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238DC1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238DC20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238DC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238DC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238DC28 size=100
    let mut pc: u32 = 0x8238DC28;
    'dispatch: loop {
        match pc {
            0x8238DC28 => {
    //   block [0x8238DC28..0x8238DC8C)
	// 8238DC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238DC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238DC30: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238DC34: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238DC38: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8238DC3C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8238DC40: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8238DC44: C00B0150  lfs f0, 0x150(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DC48: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8238DC4C: C00B0154  lfs f0, 0x154(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DC50: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8238DC54: C00B0158  lfs f0, 0x158(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(344 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DC58: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8238DC5C: C00B015C  lfs f0, 0x15c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(348 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DC60: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8238DC64: 4BFDA8D5  bl 0x82368538
	ctx.lr = 0x8238DC68;
	sub_82368538(ctx, base);
	// 8238DC68: 38CA0008  addi r6, r10, 8
	ctx.r[6].s64 = ctx.r[10].s64 + 8;
	// 8238DC6C: 38AA0004  addi r5, r10, 4
	ctx.r[5].s64 = ctx.r[10].s64 + 4;
	// 8238DC70: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 8238DC74: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8238DC78: 4BFDA999  bl 0x82368610
	ctx.lr = 0x8238DC7C;
	sub_82368610(ctx, base);
	// 8238DC7C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8238DC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238DC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238DC88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238DC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238DC90 size=120
    let mut pc: u32 = 0x8238DC90;
    'dispatch: loop {
        match pc {
            0x8238DC90 => {
    //   block [0x8238DC90..0x8238DD08)
	// 8238DC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238DC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238DC98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238DC9C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8238DCA0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8238DCA4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8238DCA8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238DCAC: C00B0150  lfs f0, 0x150(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DCB0: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8238DCB4: C00B0154  lfs f0, 0x154(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DCB8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8238DCBC: C00B0158  lfs f0, 0x158(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(344 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DCC0: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8238DCC4: C00B015C  lfs f0, 0x15c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(348 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DCC8: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8238DCCC: 4BFDA86D  bl 0x82368538
	ctx.lr = 0x8238DCD0;
	sub_82368538(ctx, base);
	// 8238DCD0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238DCD4: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 8238DCD8: C1AB0110  lfs f13, 0x110(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(272 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238DCDC: C18B0114  lfs f12, 0x114(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(276 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238DCE0: C16B0118  lfs f11, 0x118(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(280 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8238DCE4: C00ABA38  lfs f0, -0x45c8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DCE8: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 8238DCEC: D1A30030  stfs f13, 0x30(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8238DCF0: D1830034  stfs f12, 0x34(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 8238DCF4: D1630038  stfs f11, 0x38(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 8238DCF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238DCFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238DD00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238DD04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238DD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238DD08 size=576
    let mut pc: u32 = 0x8238DD08;
    'dispatch: loop {
        match pc {
            0x8238DD08 => {
    //   block [0x8238DD08..0x8238DF48)
	// 8238DD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238DD0C: 481A73AD  bl 0x825350b8
	ctx.lr = 0x8238DD10;
	sub_82535080(ctx, base);
	// 8238DD10: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8238DD14: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238DD18: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8238DD1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8238DD20: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 8238DD24: C07F0018  lfs f3, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8238DD28: C05F0014  lfs f2, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8238DD2C: C03F0010  lfs f1, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8238DD30: 4BFDA4E9  bl 0x82368218
	ctx.lr = 0x8238DD34;
	sub_82368218(ctx, base);
	// 8238DD34: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 8238DD38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8238DD3C: 4BFE6665  bl 0x823743a0
	ctx.lr = 0x8238DD40;
	sub_823743A0(ctx, base);
	// 8238DD40: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DD44: D0010080  stfs f0, 0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 8238DD48: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 8238DD4C: C01F0004  lfs f0, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DD50: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238DD54: D0010084  stfs f0, 0x84(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 8238DD58: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 8238DD5C: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DD60: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 8238DD64: D0010088  stfs f0, 0x88(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 8238DD68: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 8238DD6C: C01F000C  lfs f0, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DD70: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 8238DD74: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DD78: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8238DD7C: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8238DD80: C0010054  lfs f0, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DD84: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8238DD88: C0010058  lfs f0, 0x58(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DD8C: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 8238DD90: C001005C  lfs f0, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DD94: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8238DD98: C01F0020  lfs f0, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DD9C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8238DDA0: C01F0024  lfs f0, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DDA4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8238DDA8: C01F0028  lfs f0, 0x28(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DDAC: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8238DDB0: C01F002C  lfs f0, 0x2c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DDB4: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8238DDB8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238DF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238DF48 size=440
    let mut pc: u32 = 0x8238DF48;
    'dispatch: loop {
        match pc {
            0x8238DF48 => {
    //   block [0x8238DF48..0x8238E100)
	// 8238DF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238DF4C: 481A716D  bl 0x825350b8
	ctx.lr = 0x8238DF50;
	sub_82535080(ctx, base);
	// 8238DF50: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8238DF54: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238DF58: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8238DF5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8238DF60: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8238DF64: C07F0018  lfs f3, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8238DF68: C05F0014  lfs f2, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8238DF6C: C03F0010  lfs f1, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8238DF70: 4BFDA2A9  bl 0x82368218
	ctx.lr = 0x8238DF74;
	sub_82368218(ctx, base);
	// 8238DF74: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 8238DF78: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8238DF7C: 4BFE6425  bl 0x823743a0
	ctx.lr = 0x8238DF80;
	sub_823743A0(ctx, base);
	// 8238DF80: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DF84: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8238DF88: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238DF8C: C01F0004  lfs f0, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DF90: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 8238DF94: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8238DF98: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 8238DF9C: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DFA0: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8238DFA4: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 8238DFA8: C01F000C  lfs f0, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DFAC: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8238DFB0: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8238DFB4: C0010060  lfs f0, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DFB8: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8238DFBC: C0010064  lfs f0, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DFC0: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8238DFC4: C0010068  lfs f0, 0x68(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DFC8: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8238DFCC: C001006C  lfs f0, 0x6c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238DFD0: C3FF0020  lfs f31, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8238DFD4: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8238DFD8: 480D6061  bl 0x82464038
	ctx.lr = 0x8238DFDC;
	sub_82464038(ctx, base);
	// 8238DFDC: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 8238DFE0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8238DFE4: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8238DFE8: 4816CB89  bl 0x824fab70
	ctx.lr = 0x8238DFEC;
	sub_824FAB70(ctx, base);
	// 8238DFEC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8238DFF0: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 8238DFF4: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8238DFF8: 48103F01  bl 0x82491ef8
	ctx.lr = 0x8238DFFC;
	sub_82491EF8(ctx, base);
	// 8238DFFC: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 8238E000: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8238E004: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8238E008: C01F0044  lfs f0, 0x44(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238E00C: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 8238E010: D001016C  stfs f0, 0x16c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(364 as u32), tmp.u32 ) };
	// 8238E014: C01F0040  lfs f0, 0x40(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238E018: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 8238E01C: C05F0048  lfs f2, 0x48(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8238E020: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238E100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238E100 size=12
    let mut pc: u32 = 0x8238E100;
    'dispatch: loop {
        match pc {
            0x8238E100 => {
    //   block [0x8238E100..0x8238E10C)
	// 8238E100: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238E104: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238E108: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238E10C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238E10C size=32
    let mut pc: u32 = 0x8238E10C;
    'dispatch: loop {
        match pc {
            0x8238E10C => {
    //   block [0x8238E10C..0x8238E12C)
	// 8238E10C: C00B0010  lfs f0, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238E110: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8238E114: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8238E118: D0040004  stfs f0, 4(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8238E11C: D0040008  stfs f0, 8(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8238E120: C1ABBA38  lfs f13, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238E124: D1A4000C  stfs f13, 0xc(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8238E128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238E130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238E130 size=136
    let mut pc: u32 = 0x8238E130;
    'dispatch: loop {
        match pc {
            0x8238E130 => {
    //   block [0x8238E130..0x8238E1B8)
	// 8238E130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238E134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238E138: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238E13C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238E140: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238E144: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238E148: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8238E14C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8238E150: 48103DA9  bl 0x82491ef8
	ctx.lr = 0x8238E154;
	sub_82491EF8(ctx, base);
	// 8238E154: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8238E158: 807E005C  lwz r3, 0x5c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 8238E15C: 48102B6D  bl 0x82490cc8
	ctx.lr = 0x8238E160;
	sub_82490CC8(ctx, base);
	// 8238E160: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238E164: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8238E168: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 8238E16C: 38800200  li r4, 0x200
	ctx.r[4].s64 = 512;
	// 8238E170: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8238E174: 480D5EC5  bl 0x82464038
	ctx.lr = 0x8238E178;
	sub_82464038(ctx, base);
	// 8238E178: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 8238E17C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8238E180: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8238E184: 48101E95  bl 0x82490018
	ctx.lr = 0x8238E188;
	sub_82490018(ctx, base);
	// 8238E188: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8238E18C: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8238E190: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8238E194: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 8238E198: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 8238E19C: 91630070  stw r11, 0x70(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8238E1A0: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 8238E1A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238E1A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238E1AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238E1B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238E1B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238E1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238E1B8 size=556
    let mut pc: u32 = 0x8238E1B8;
    'dispatch: loop {
        match pc {
            0x8238E1B8 => {
    //   block [0x8238E1B8..0x8238E3E4)
	// 8238E1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238E1BC: 481A6F01  bl 0x825350bc
	ctx.lr = 0x8238E1C0;
	sub_82535080(ctx, base);
	// 8238E1C0: DBA1FFC8  stfd f29, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[29].u64 ) };
	// 8238E1C4: DBC1FFD0  stfd f30, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 8238E1C8: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8238E1CC: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238E1D0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8238E1D4: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 8238E1D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8238E1DC: 4BFFFAB5  bl 0x8238dc90
	ctx.lr = 0x8238E1E0;
	sub_8238DC90(ctx, base);
	// 8238E1E0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238E1E4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8238E1E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8238E1EC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238E1F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238E1F4: 4E800421  bctrl
	ctx.lr = 0x8238E1F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238E1F8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238E1FC: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238E200: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238E204: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8238E208: 419A01B8  beq cr6, 0x8238e3c0
	if ctx.cr[6].eq {
	pc = 0x8238E3C0; continue 'dispatch;
	}
	// 8238E20C: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 8238E210: 419A0160  beq cr6, 0x8238e370
	if ctx.cr[6].eq {
	pc = 0x8238E370; continue 'dispatch;
	}
	// 8238E214: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 8238E218: 419A0020  beq cr6, 0x8238e238
	if ctx.cr[6].eq {
	pc = 0x8238E238; continue 'dispatch;
	}
	// 8238E21C: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8238E220: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8238E224: C00BD4DC  lfs f0, -0x2b24(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11044 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238E228: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8238E22C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8238E230: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8238E234: 48000174  b 0x8238e3a8
	pc = 0x8238E3A8; continue 'dispatch;
	// 8238E238: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8238E23C: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 8238E240: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8238E244: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8238E248: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 8238E24C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8238E250: 4816DD79  bl 0x824fbfc8
	ctx.lr = 0x8238E254;
	sub_824FBFC8(ctx, base);
	// 8238E254: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8238E258: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 8238E25C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238E260: 409900E4  ble cr6, 0x8238e344
	if !ctx.cr[6].gt {
	pc = 0x8238E344; continue 'dispatch;
	}
	// 8238E264: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 8238E268: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8238E26C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238E270: C3E9BA38  lfs f31, -0x45c8(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8238E274: C3AA1FF8  lfs f29, 0x1ff8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8238E278: C3CB2090  lfs f30, 0x2090(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8336 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8238E27C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8238E280: D3E1005C  stfs f31, 0x5c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8238E284: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8238E288: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8238E28C: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 8238E290: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 8238E294: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8238E298: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8238E29C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8238E2A0: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238E2A4: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 8238E2A8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8238E2AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8238E2B0: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238E2B4: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8238E2B8: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238E2BC: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8238E2C0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238E3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238E3E8 size=76
    let mut pc: u32 = 0x8238E3E8;
    'dispatch: loop {
        match pc {
            0x8238E3E8 => {
    //   block [0x8238E3E8..0x8238E434)
	// 8238E3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238E3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238E3F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238E3F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238E3F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238E3FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238E400: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8238E404: 48000035  bl 0x8238e438
	ctx.lr = 0x8238E408;
	sub_8238E438(ctx, base);
	// 8238E408: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238E40C: 4182000C  beq 0x8238e418
	if ctx.cr[0].eq {
	pc = 0x8238E418; continue 'dispatch;
	}
	// 8238E410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8238E414: 481A47A5  bl 0x82532bb8
	ctx.lr = 0x8238E418;
	sub_82532BB8(ctx, base);
	// 8238E418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8238E41C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238E420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238E424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238E428: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238E42C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238E430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238E438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238E438 size=172
    let mut pc: u32 = 0x8238E438;
    'dispatch: loop {
        match pc {
            0x8238E438 => {
    //   block [0x8238E438..0x8238E4E4)
	// 8238E438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238E43C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238E440: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238E444: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238E448: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238E44C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238E450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8238E454: 396B02D0  addi r11, r11, 0x2d0
	ctx.r[11].s64 = ctx.r[11].s64 + 720;
	// 8238E458: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238E45C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8238E460: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8238E464: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238E468: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238E46C: 4182002C  beq 0x8238e498
	if ctx.cr[0].eq {
	pc = 0x8238E498; continue 'dispatch;
	}
	// 8238E470: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8238E474: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238E478: 7D6B0735  extsh. r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238E47C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8238E480: 40820018  bne 0x8238e498
	if !ctx.cr[0].eq {
	pc = 0x8238E498; continue 'dispatch;
	}
	// 8238E484: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238E488: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8238E48C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238E490: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238E494: 4E800421  bctrl
	ctx.lr = 0x8238E498;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238E498: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238E49C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238E4A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238E4A4: 4182002C  beq 0x8238e4d0
	if ctx.cr[0].eq {
	pc = 0x8238E4D0; continue 'dispatch;
	}
	// 8238E4A8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8238E4AC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238E4B0: 7D6B0735  extsh. r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238E4B4: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8238E4B8: 40820018  bne 0x8238e4d0
	if !ctx.cr[0].eq {
	pc = 0x8238E4D0; continue 'dispatch;
	}
	// 8238E4BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238E4C0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8238E4C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238E4C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238E4CC: 4E800421  bctrl
	ctx.lr = 0x8238E4D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238E4D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8238E4D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238E4D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238E4DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238E4E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238E4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238E4E8 size=204
    let mut pc: u32 = 0x8238E4E8;
    'dispatch: loop {
        match pc {
            0x8238E4E8 => {
    //   block [0x8238E4E8..0x8238E5B4)
	// 8238E4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238E4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238E4F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238E4F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238E4F8: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238E4FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238E500: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8238E504: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8238E508: 480E3C11  bl 0x82472118
	ctx.lr = 0x8238E50C;
	sub_82472118(ctx, base);
	// 8238E50C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8238E510: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238E514: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238E518: 480E5639  bl 0x82473b50
	ctx.lr = 0x8238E51C;
	sub_82473B50(ctx, base);
	// 8238E51C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8238E520: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8238E524: 808B92F0  lwz r4, -0x6d10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27920 as u32) ) } as u64;
	// 8238E528: 480E9239  bl 0x82477760
	ctx.lr = 0x8238E52C;
	sub_82477760(ctx, base);
	// 8238E52C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8238E530: 480E2F11  bl 0x82471440
	ctx.lr = 0x8238E534;
	sub_82471440(ctx, base);
	// 8238E534: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8238E538: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238E53C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238E540: 41820010  beq 0x8238e550
	if ctx.cr[0].eq {
	pc = 0x8238E550; continue 'dispatch;
	}
	// 8238E544: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8238E548: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8238E54C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8238E550: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8238E554: 386B9244  addi r3, r11, -0x6dbc
	ctx.r[3].s64 = ctx.r[11].s64 + -28092;
	// 8238E558: 480D8301  bl 0x82466858
	ctx.lr = 0x8238E55C;
	sub_82466858(ctx, base);
	// 8238E55C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8238E560: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8238E564: 480E6335  bl 0x82474898
	ctx.lr = 0x8238E568;
	sub_82474898(ctx, base);
	// 8238E568: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8238E56C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 8238E570: 386B14F8  addi r3, r11, 0x14f8
	ctx.r[3].s64 = ctx.r[11].s64 + 5368;
	// 8238E574: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8238E578: 480D82E1  bl 0x82466858
	ctx.lr = 0x8238E57C;
	sub_82466858(ctx, base);
	// 8238E57C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8238E580: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238E584: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8238E588: 480EC1F1  bl 0x8247a778
	ctx.lr = 0x8238E58C;
	sub_8247A778(ctx, base);
	// 8238E58C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8238E590: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8238E594: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8238E598: 480E3C41  bl 0x824721d8
	ctx.lr = 0x8238E59C;
	sub_824721D8(ctx, base);
	// 8238E59C: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 8238E5A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238E5A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238E5A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238E5AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238E5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238E5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238E5B8 size=224
    let mut pc: u32 = 0x8238E5B8;
    'dispatch: loop {
        match pc {
            0x8238E5B8 => {
    //   block [0x8238E5B8..0x8238E698)
	// 8238E5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238E5BC: 481A6AE1  bl 0x8253509c
	ctx.lr = 0x8238E5C0;
	sub_82535080(ctx, base);
	// 8238E5C0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238E5C4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8238E5C8: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 8238E5CC: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 8238E5D0: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8238E5D4: 8379000C  lwz r27, 0xc(r25)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238E5D8: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238E5DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238E5E0: 409900AC  ble cr6, 0x8238e68c
	if !ctx.cr[6].gt {
	pc = 0x8238E68C; continue 'dispatch;
	}
	// 8238E5E4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8238E5E8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8238E5EC: 3AEB9684  addi r23, r11, -0x697c
	ctx.r[23].s64 = ctx.r[11].s64 + -27004;
	// 8238E5F0: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238E5F4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8238E5F8: 7FDA582E  lwzx r30, r26, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8238E5FC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238E600: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238E604: 40990074  ble cr6, 0x8238e678
	if !ctx.cr[6].gt {
	pc = 0x8238E678; continue 'dispatch;
	}
	// 8238E608: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8238E60C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238E610: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8238E614: 83EB0070  lwz r31, 0x70(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 8238E618: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238E61C: 41820028  beq 0x8238e644
	if ctx.cr[0].eq {
	pc = 0x8238E644; continue 'dispatch;
	}
	// 8238E620: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8238E624: 480D8235  bl 0x82466858
	ctx.lr = 0x8238E628;
	sub_82466858(ctx, base);
	// 8238E628: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8238E62C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8238E630: 80790008  lwz r3, 8(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238E634: 480EC145  bl 0x8247a778
	ctx.lr = 0x8238E638;
	sub_8247A778(ctx, base);
	// 8238E638: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8238E63C: 480F1FED  bl 0x82480628
	ctx.lr = 0x8238E640;
	sub_82480628(ctx, base);
	// 8238E640: 48000008  b 0x8238e648
	pc = 0x8238E648; continue 'dispatch;
	// 8238E644: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8238E648: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8238E64C: 419A0018  beq cr6, 0x8238e664
	if ctx.cr[6].eq {
	pc = 0x8238E664; continue 'dispatch;
	}
	// 8238E650: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 8238E654: 480F1D0D  bl 0x82480360
	ctx.lr = 0x8238E658;
	sub_82480360(ctx, base);
	// 8238E658: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238E65C: 41820008  beq 0x8238e664
	if ctx.cr[0].eq {
	pc = 0x8238E664; continue 'dispatch;
	}
	// 8238E660: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 8238E664: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238E668: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8238E66C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8238E670: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8238E674: 4198FF98  blt cr6, 0x8238e60c
	if ctx.cr[6].lt {
	pc = 0x8238E60C; continue 'dispatch;
	}
	// 8238E678: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238E67C: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 8238E680: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 8238E684: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8238E688: 4198FF68  blt cr6, 0x8238e5f0
	if ctx.cr[6].lt {
	pc = 0x8238E5F0; continue 'dispatch;
	}
	// 8238E68C: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8238E690: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8238E694: 481A6A58  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238E698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238E698 size=412
    let mut pc: u32 = 0x8238E698;
    'dispatch: loop {
        match pc {
            0x8238E698 => {
    //   block [0x8238E698..0x8238E834)
	// 8238E698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238E69C: 481A69F5  bl 0x82535090
	ctx.lr = 0x8238E6A0;
	sub_82535080(ctx, base);
	// 8238E6A0: 3981FF88  addi r12, r1, -0x78
	ctx.r[12].s64 = ctx.r[1].s64 + -120;
	// 8238E6A4: 481A7945  bl 0x82535fe8
	ctx.lr = 0x8238E6A8;
	sub_82535FB0(ctx, base);
	// 8238E6A8: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238E6AC: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 8238E6B0: 7C922378  mr r18, r4
	ctx.r[18].u64 = ctx.r[4].u64;
	// 8238E6B4: 7CB32B78  mr r19, r5
	ctx.r[19].u64 = ctx.r[5].u64;
	// 8238E6B8: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8238E6BC: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 8238E6C0: 8316000C  lwz r24, 0xc(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238E6C4: 81780010  lwz r11, 0x10(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238E6C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238E6CC: 40990154  ble cr6, 0x8238e820
	if !ctx.cr[6].gt {
	pc = 0x8238E820; continue 'dispatch;
	}
	// 8238E6D0: 3CE0820A  lis r7, -0x7df6
	ctx.r[7].s64 = -2113273856;
	// 8238E6D4: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 8238E6D8: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8238E6DC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8238E6E0: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8238E6E4: C3C7BA38  lfs f30, -0x45c8(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8238E6E8: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 8238E6EC: C388D218  lfs f28, -0x2de8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-11752 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 8238E6F0: 3AEB9684  addi r23, r11, -0x697c
	ctx.r[23].s64 = ctx.r[11].s64 + -27004;
	// 8238E6F4: C3A9BFFC  lfs f29, -0x4004(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8238E6F8: C3EA1FF8  lfs f31, 0x1ff8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8238E6FC: 8178000C  lwz r11, 0xc(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238E700: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8238E704: 7F75582E  lwzx r27, r21, r11
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8238E708: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238E70C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238E710: 409900FC  ble cr6, 0x8238e80c
	if !ctx.cr[6].gt {
	pc = 0x8238E80C; continue 'dispatch;
	}
	// 8238E714: 572B103A  slwi r11, r25, 2
	ctx.r[11].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8238E718: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8238E71C: 7F8B9A14  add r28, r11, r19
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[19].u64;
	// 8238E720: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238E724: 7FABF02E  lwzx r29, r11, r30
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8238E728: 83FD0070  lwz r31, 0x70(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(112 as u32) ) } as u64;
	// 8238E72C: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238E730: 41820028  beq 0x8238e758
	if ctx.cr[0].eq {
	pc = 0x8238E758; continue 'dispatch;
	}
	// 8238E734: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8238E738: 480D8121  bl 0x82466858
	ctx.lr = 0x8238E73C;
	sub_82466858(ctx, base);
	// 8238E73C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8238E740: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8238E744: 80760008  lwz r3, 8(r22)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238E748: 480EC031  bl 0x8247a778
	ctx.lr = 0x8238E74C;
	sub_8247A778(ctx, base);
	// 8238E74C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8238E750: 480F1ED9  bl 0x82480628
	ctx.lr = 0x8238E754;
	sub_82480628(ctx, base);
	// 8238E754: 48000008  b 0x8238e75c
	pc = 0x8238E75C; continue 'dispatch;
	// 8238E758: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8238E75C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8238E760: 419A0098  beq cr6, 0x8238e7f8
	if ctx.cr[6].eq {
	pc = 0x8238E7F8; continue 'dispatch;
	}
	// 8238E764: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 8238E768: 480F1BF9  bl 0x82480360
	ctx.lr = 0x8238E76C;
	sub_82480360(ctx, base);
	// 8238E76C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238E770: 41820088  beq 0x8238e7f8
	if ctx.cr[0].eq {
	pc = 0x8238E7F8; continue 'dispatch;
	}
	// 8238E774: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8238E778: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8238E77C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8238E780: D3E10054  stfs f31, 0x54(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8238E784: D3E10058  stfs f31, 0x58(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8238E788: 93A100AC  stw r29, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[29].u32 ) };
	// 8238E78C: D3C1005C  stfs f30, 0x5c(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8238E790: D3E10060  stfs f31, 0x60(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8238E794: 9161009C  stw r11, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 8238E798: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 8238E79C: D3E10064  stfs f31, 0x64(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8238E7A0: D3E10068  stfs f31, 0x68(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 8238E7A4: D3C1006C  stfs f30, 0x6c(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8238E7A8: D3C10070  stfs f30, 0x70(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8238E7AC: 916100A0  stw r11, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 8238E7B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8238E7B4: D3C10074  stfs f30, 0x74(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8238E7B8: D3C10078  stfs f30, 0x78(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 8238E7BC: D3C1007C  stfs f30, 0x7c(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8238E7C0: D3E10080  stfs f31, 0x80(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 8238E7C4: 916100A8  stw r11, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 8238E7C8: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8238E7CC: D3E10084  stfs f31, 0x84(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 8238E7D0: D3E10088  stfs f31, 0x88(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 8238E7D4: D3C1008C  stfs f30, 0x8c(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 8238E7D8: D3C10098  stfs f30, 0x98(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8238E7DC: D3A10094  stfs f29, 0x94(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8238E7E0: 916100A4  stw r11, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 8238E7E4: D3810090  stfs f28, 0x90(r1)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8238E7E8: 4BFFE691  bl 0x8238ce78
	ctx.lr = 0x8238E7EC;
	sub_8238CE78(ctx, base);
	// 8238E7EC: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8238E7F0: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 8238E7F4: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 8238E7F8: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238E7FC: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 8238E800: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8238E804: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8238E808: 4198FF18  blt cr6, 0x8238e720
	if ctx.cr[6].lt {
	pc = 0x8238E720; continue 'dispatch;
	}
	// 8238E80C: 81780010  lwz r11, 0x10(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238E810: 3A940001  addi r20, r20, 1
	ctx.r[20].s64 = ctx.r[20].s64 + 1;
	// 8238E814: 3AB50004  addi r21, r21, 4
	ctx.r[21].s64 = ctx.r[21].s64 + 4;
	// 8238E818: 7F145800  cmpw cr6, r20, r11
	ctx.cr[6].compare_i32(ctx.r[20].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8238E81C: 4198FEE0  blt cr6, 0x8238e6fc
	if ctx.cr[6].lt {
	pc = 0x8238E6FC; continue 'dispatch;
	}
	// 8238E820: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8238E824: 38210150  addi r1, r1, 0x150
	ctx.r[1].s64 = ctx.r[1].s64 + 336;
	// 8238E828: 3981FF88  addi r12, r1, -0x78
	ctx.r[12].s64 = ctx.r[1].s64 + -120;
	// 8238E82C: 481A7809  bl 0x82536034
	ctx.lr = 0x8238E830;
	sub_82535FFC(ctx, base);
	// 8238E830: 481A68B0  b 0x825350e0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238E838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238E838 size=224
    let mut pc: u32 = 0x8238E838;
    'dispatch: loop {
        match pc {
            0x8238E838 => {
    //   block [0x8238E838..0x8238E918)
	// 8238E838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238E83C: 481A6861  bl 0x8253509c
	ctx.lr = 0x8238E840;
	sub_82535080(ctx, base);
	// 8238E840: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238E844: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8238E848: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 8238E84C: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 8238E850: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8238E854: 8379000C  lwz r27, 0xc(r25)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238E858: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238E85C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238E860: 409900AC  ble cr6, 0x8238e90c
	if !ctx.cr[6].gt {
	pc = 0x8238E90C; continue 'dispatch;
	}
	// 8238E864: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8238E868: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8238E86C: 3AEB9684  addi r23, r11, -0x697c
	ctx.r[23].s64 = ctx.r[11].s64 + -27004;
	// 8238E870: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238E874: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8238E878: 7FDA582E  lwzx r30, r26, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8238E87C: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8238E880: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238E884: 40990074  ble cr6, 0x8238e8f8
	if !ctx.cr[6].gt {
	pc = 0x8238E8F8; continue 'dispatch;
	}
	// 8238E888: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8238E88C: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8238E890: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8238E894: 83EB0020  lwz r31, 0x20(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8238E898: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238E89C: 41820028  beq 0x8238e8c4
	if ctx.cr[0].eq {
	pc = 0x8238E8C4; continue 'dispatch;
	}
	// 8238E8A0: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8238E8A4: 480D7FB5  bl 0x82466858
	ctx.lr = 0x8238E8A8;
	sub_82466858(ctx, base);
	// 8238E8A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8238E8AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8238E8B0: 80790008  lwz r3, 8(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238E8B4: 480EBEC5  bl 0x8247a778
	ctx.lr = 0x8238E8B8;
	sub_8247A778(ctx, base);
	// 8238E8B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8238E8BC: 480F1D6D  bl 0x82480628
	ctx.lr = 0x8238E8C0;
	sub_82480628(ctx, base);
	// 8238E8C0: 48000008  b 0x8238e8c8
	pc = 0x8238E8C8; continue 'dispatch;
	// 8238E8C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8238E8C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8238E8CC: 419A0018  beq cr6, 0x8238e8e4
	if ctx.cr[6].eq {
	pc = 0x8238E8E4; continue 'dispatch;
	}
	// 8238E8D0: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 8238E8D4: 480F1A8D  bl 0x82480360
	ctx.lr = 0x8238E8D8;
	sub_82480360(ctx, base);
	// 8238E8D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238E8DC: 41820008  beq 0x8238e8e4
	if ctx.cr[0].eq {
	pc = 0x8238E8E4; continue 'dispatch;
	}
	// 8238E8E0: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 8238E8E4: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8238E8E8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8238E8EC: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8238E8F0: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8238E8F4: 4198FF98  blt cr6, 0x8238e88c
	if ctx.cr[6].lt {
	pc = 0x8238E88C; continue 'dispatch;
	}
	// 8238E8F8: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238E8FC: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 8238E900: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 8238E904: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8238E908: 4198FF68  blt cr6, 0x8238e870
	if ctx.cr[6].lt {
	pc = 0x8238E870; continue 'dispatch;
	}
	// 8238E90C: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8238E910: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8238E914: 481A67D8  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238E918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238E918 size=300
    let mut pc: u32 = 0x8238E918;
    'dispatch: loop {
        match pc {
            0x8238E918 => {
    //   block [0x8238E918..0x8238EA44)
	// 8238E918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238E91C: 481A676D  bl 0x82535088
	ctx.lr = 0x8238E920;
	sub_82535080(ctx, base);
	// 8238E920: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238E924: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 8238E928: 7C902378  mr r16, r4
	ctx.r[16].u64 = ctx.r[4].u64;
	// 8238E92C: 7CB32B78  mr r19, r5
	ctx.r[19].u64 = ctx.r[5].u64;
	// 8238E930: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8238E934: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 8238E938: 8317000C  lwz r24, 0xc(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238E93C: 81780010  lwz r11, 0x10(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238E940: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238E944: 409900F4  ble cr6, 0x8238ea38
	if !ctx.cr[6].gt {
	pc = 0x8238EA38; continue 'dispatch;
	}
	// 8238E948: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238E94C: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 8238E950: 3A4B0290  addi r18, r11, 0x290
	ctx.r[18].s64 = ctx.r[11].s64 + 656;
	// 8238E954: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8238E958: 3E208273  lis r17, -0x7d8d
	ctx.r[17].s64 = -2106392576;
	// 8238E95C: 3AAB9684  addi r21, r11, -0x697c
	ctx.r[21].s64 = ctx.r[11].s64 + -27004;
	// 8238E960: 8178000C  lwz r11, 0xc(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238E964: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8238E968: 7F76582E  lwzx r27, r22, r11
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8238E96C: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 8238E970: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238E974: 409900B0  ble cr6, 0x8238ea24
	if !ctx.cr[6].gt {
	pc = 0x8238EA24; continue 'dispatch;
	}
	// 8238E978: 572B103A  slwi r11, r25, 2
	ctx.r[11].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8238E97C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8238E980: 7F8B9A14  add r28, r11, r19
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[19].u64;
	// 8238E984: 817B0014  lwz r11, 0x14(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 8238E988: 7FABF02E  lwzx r29, r11, r30
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8238E98C: 83FD0020  lwz r31, 0x20(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 8238E990: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238E994: 41820028  beq 0x8238e9bc
	if ctx.cr[0].eq {
	pc = 0x8238E9BC; continue 'dispatch;
	}
	// 8238E998: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8238E99C: 480D7EBD  bl 0x82466858
	ctx.lr = 0x8238E9A0;
	sub_82466858(ctx, base);
	// 8238E9A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8238E9A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8238E9A8: 80770008  lwz r3, 8(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238E9AC: 480EBDCD  bl 0x8247a778
	ctx.lr = 0x8238E9B0;
	sub_8247A778(ctx, base);
	// 8238E9B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8238E9B4: 480F1C75  bl 0x82480628
	ctx.lr = 0x8238E9B8;
	sub_82480628(ctx, base);
	// 8238E9B8: 48000008  b 0x8238e9c0
	pc = 0x8238E9C0; continue 'dispatch;
	// 8238E9BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8238E9C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8238E9C4: 419A004C  beq cr6, 0x8238ea10
	if ctx.cr[6].eq {
	pc = 0x8238EA10; continue 'dispatch;
	}
	// 8238E9C8: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 8238E9CC: 480F1995  bl 0x82480360
	ctx.lr = 0x8238E9D0;
	sub_82480360(ctx, base);
	// 8238E9D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238E9D4: 4182003C  beq 0x8238ea10
	if ctx.cr[0].eq {
	pc = 0x8238EA10; continue 'dispatch;
	}
	// 8238E9D8: 817149AC  lwz r11, 0x49ac(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(18860 as u32) ) } as u64;
	// 8238E9DC: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8238E9E0: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 8238E9E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238E9E8: 4E800421  bctrl
	ctx.lr = 0x8238E9EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238E9EC: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238E9F0: 4182000C  beq 0x8238e9fc
	if ctx.cr[0].eq {
	pc = 0x8238E9FC; continue 'dispatch;
	}
	// 8238E9F4: 924B0000  stw r18, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[18].u32 ) };
	// 8238E9F8: 48000008  b 0x8238ea00
	pc = 0x8238EA00; continue 'dispatch;
	// 8238E9FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8238EA00: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8238EA04: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 8238EA08: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8238EA0C: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 8238EA10: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 8238EA14: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 8238EA18: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8238EA1C: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8238EA20: 4198FF64  blt cr6, 0x8238e984
	if ctx.cr[6].lt {
	pc = 0x8238E984; continue 'dispatch;
	}
	// 8238EA24: 81780010  lwz r11, 0x10(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16 as u32) ) } as u64;
	// 8238EA28: 3A940001  addi r20, r20, 1
	ctx.r[20].s64 = ctx.r[20].s64 + 1;
	// 8238EA2C: 3AD60004  addi r22, r22, 4
	ctx.r[22].s64 = ctx.r[22].s64 + 4;
	// 8238EA30: 7F145800  cmpw cr6, r20, r11
	ctx.cr[6].compare_i32(ctx.r[20].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8238EA34: 4198FF2C  blt cr6, 0x8238e960
	if ctx.cr[6].lt {
	pc = 0x8238E960; continue 'dispatch;
	}
	// 8238EA38: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8238EA3C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8238EA40: 481A6698  b 0x825350d8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238EA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238EA48 size=124
    let mut pc: u32 = 0x8238EA48;
    'dispatch: loop {
        match pc {
            0x8238EA48 => {
    //   block [0x8238EA48..0x8238EAC4)
	// 8238EA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238EA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238EA50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238EA54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238EA58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8238EA5C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238EA60: 396B0290  addi r11, r11, 0x290
	ctx.r[11].s64 = ctx.r[11].s64 + 656;
	// 8238EA64: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238EA68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238EA6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8238EA70: 41820040  beq 0x8238eab0
	if ctx.cr[0].eq {
	pc = 0x8238EAB0; continue 'dispatch;
	}
	// 8238EA74: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238EA78: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238EA7C: 4182002C  beq 0x8238eaa8
	if ctx.cr[0].eq {
	pc = 0x8238EAA8; continue 'dispatch;
	}
	// 8238EA80: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8238EA84: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238EA88: 7D6B0735  extsh. r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238EA8C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8238EA90: 40820018  bne 0x8238eaa8
	if !ctx.cr[0].eq {
	pc = 0x8238EAA8; continue 'dispatch;
	}
	// 8238EA94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238EA98: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8238EA9C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238EAA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238EAA4: 4E800421  bctrl
	ctx.lr = 0x8238EAA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238EAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8238EAAC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8238EAB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8238EAB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238EAB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238EABC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238EAC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238EAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238EAC8 size=188
    let mut pc: u32 = 0x8238EAC8;
    'dispatch: loop {
        match pc {
            0x8238EAC8 => {
    //   block [0x8238EAC8..0x8238EB84)
	// 8238EAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238EACC: 481A65E9  bl 0x825350b4
	ctx.lr = 0x8238EAD0;
	sub_82535080(ctx, base);
	// 8238EAD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238EAD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8238EAD8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8238EADC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8238EAE0: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8238EAE4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8238EAE8: 419A0090  beq cr6, 0x8238eb78
	if ctx.cr[6].eq {
	pc = 0x8238EB78; continue 'dispatch;
	}
	// 8238EAEC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8238EAF0: 419A0088  beq cr6, 0x8238eb78
	if ctx.cr[6].eq {
	pc = 0x8238EB78; continue 'dispatch;
	}
	// 8238EAF4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8238EAF8: 419A0080  beq cr6, 0x8238eb78
	if ctx.cr[6].eq {
	pc = 0x8238EB78; continue 'dispatch;
	}
	// 8238EAFC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8238EB00: 419A0078  beq cr6, 0x8238eb78
	if ctx.cr[6].eq {
	pc = 0x8238EB78; continue 'dispatch;
	}
	// 8238EB04: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238EB08: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238EB0C: 41820038  beq 0x8238eb44
	if ctx.cr[0].eq {
	pc = 0x8238EB44; continue 'dispatch;
	}
	// 8238EB10: 83EB0070  lwz r31, 0x70(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 8238EB14: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238EB18: 4182002C  beq 0x8238eb44
	if ctx.cr[0].eq {
	pc = 0x8238EB44; continue 'dispatch;
	}
	// 8238EB1C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8238EB20: 386B9684  addi r3, r11, -0x697c
	ctx.r[3].s64 = ctx.r[11].s64 + -27004;
	// 8238EB24: 480D7D35  bl 0x82466858
	ctx.lr = 0x8238EB28;
	sub_82466858(ctx, base);
	// 8238EB28: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8238EB2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8238EB30: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238EB34: 480EBC45  bl 0x8247a778
	ctx.lr = 0x8238EB38;
	sub_8247A778(ctx, base);
	// 8238EB38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8238EB3C: 480F1AED  bl 0x82480628
	ctx.lr = 0x8238EB40;
	sub_82480628(ctx, base);
	// 8238EB40: 48000008  b 0x8238eb48
	pc = 0x8238EB48; continue 'dispatch;
	// 8238EB44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8238EB48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8238EB4C: 419A002C  beq cr6, 0x8238eb78
	if ctx.cr[6].eq {
	pc = 0x8238EB78; continue 'dispatch;
	}
	// 8238EB50: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8238EB54: 480F180D  bl 0x82480360
	ctx.lr = 0x8238EB58;
	sub_82480360(ctx, base);
	// 8238EB58: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238EB5C: 4182001C  beq 0x8238eb78
	if ctx.cr[0].eq {
	pc = 0x8238EB78; continue 'dispatch;
	}
	// 8238EB60: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8238EB64: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8238EB68: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8238EB6C: 480F1165  bl 0x8247fcd0
	ctx.lr = 0x8238EB70;
	sub_8247FCD0(ctx, base);
	// 8238EB70: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8238EB74: 48000008  b 0x8238eb7c
	pc = 0x8238EB7C; continue 'dispatch;
	// 8238EB78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8238EB7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8238EB80: 481A6584  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238EB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238EB88 size=188
    let mut pc: u32 = 0x8238EB88;
    'dispatch: loop {
        match pc {
            0x8238EB88 => {
    //   block [0x8238EB88..0x8238EC44)
	// 8238EB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238EB8C: 481A6529  bl 0x825350b4
	ctx.lr = 0x8238EB90;
	sub_82535080(ctx, base);
	// 8238EB90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238EB94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8238EB98: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8238EB9C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8238EBA0: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8238EBA4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8238EBA8: 419A0090  beq cr6, 0x8238ec38
	if ctx.cr[6].eq {
	pc = 0x8238EC38; continue 'dispatch;
	}
	// 8238EBAC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8238EBB0: 419A0088  beq cr6, 0x8238ec38
	if ctx.cr[6].eq {
	pc = 0x8238EC38; continue 'dispatch;
	}
	// 8238EBB4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8238EBB8: 419A0080  beq cr6, 0x8238ec38
	if ctx.cr[6].eq {
	pc = 0x8238EC38; continue 'dispatch;
	}
	// 8238EBBC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8238EBC0: 419A0078  beq cr6, 0x8238ec38
	if ctx.cr[6].eq {
	pc = 0x8238EC38; continue 'dispatch;
	}
	// 8238EBC4: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238EBC8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238EBCC: 41820038  beq 0x8238ec04
	if ctx.cr[0].eq {
	pc = 0x8238EC04; continue 'dispatch;
	}
	// 8238EBD0: 83EB0020  lwz r31, 0x20(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8238EBD4: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238EBD8: 4182002C  beq 0x8238ec04
	if ctx.cr[0].eq {
	pc = 0x8238EC04; continue 'dispatch;
	}
	// 8238EBDC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8238EBE0: 386B9684  addi r3, r11, -0x697c
	ctx.r[3].s64 = ctx.r[11].s64 + -27004;
	// 8238EBE4: 480D7C75  bl 0x82466858
	ctx.lr = 0x8238EBE8;
	sub_82466858(ctx, base);
	// 8238EBE8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8238EBEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8238EBF0: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238EBF4: 480EBB85  bl 0x8247a778
	ctx.lr = 0x8238EBF8;
	sub_8247A778(ctx, base);
	// 8238EBF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8238EBFC: 480F1A2D  bl 0x82480628
	ctx.lr = 0x8238EC00;
	sub_82480628(ctx, base);
	// 8238EC00: 48000008  b 0x8238ec08
	pc = 0x8238EC08; continue 'dispatch;
	// 8238EC04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8238EC08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8238EC0C: 419A002C  beq cr6, 0x8238ec38
	if ctx.cr[6].eq {
	pc = 0x8238EC38; continue 'dispatch;
	}
	// 8238EC10: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8238EC14: 480F174D  bl 0x82480360
	ctx.lr = 0x8238EC18;
	sub_82480360(ctx, base);
	// 8238EC18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238EC1C: 4182001C  beq 0x8238ec38
	if ctx.cr[0].eq {
	pc = 0x8238EC38; continue 'dispatch;
	}
	// 8238EC20: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8238EC24: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8238EC28: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8238EC2C: 480F10A5  bl 0x8247fcd0
	ctx.lr = 0x8238EC30;
	sub_8247FCD0(ctx, base);
	// 8238EC30: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8238EC34: 48000008  b 0x8238ec3c
	pc = 0x8238EC3C; continue 'dispatch;
	// 8238EC38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8238EC3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8238EC40: 481A64C4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238EC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238EC48 size=156
    let mut pc: u32 = 0x8238EC48;
    'dispatch: loop {
        match pc {
            0x8238EC48 => {
    //   block [0x8238EC48..0x8238ECE4)
	// 8238EC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238EC4C: 481A646D  bl 0x825350b8
	ctx.lr = 0x8238EC50;
	sub_82535080(ctx, base);
	// 8238EC50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238EC54: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238EC58: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8238EC5C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8238EC60: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8238EC64: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238EC68: 41820038  beq 0x8238eca0
	if ctx.cr[0].eq {
	pc = 0x8238ECA0; continue 'dispatch;
	}
	// 8238EC6C: 83EB0070  lwz r31, 0x70(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 8238EC70: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238EC74: 4182002C  beq 0x8238eca0
	if ctx.cr[0].eq {
	pc = 0x8238ECA0; continue 'dispatch;
	}
	// 8238EC78: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8238EC7C: 386B9684  addi r3, r11, -0x697c
	ctx.r[3].s64 = ctx.r[11].s64 + -27004;
	// 8238EC80: 480D7BD9  bl 0x82466858
	ctx.lr = 0x8238EC84;
	sub_82466858(ctx, base);
	// 8238EC84: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8238EC88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8238EC8C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238EC90: 480EBAE9  bl 0x8247a778
	ctx.lr = 0x8238EC94;
	sub_8247A778(ctx, base);
	// 8238EC94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8238EC98: 480F1991  bl 0x82480628
	ctx.lr = 0x8238EC9C;
	sub_82480628(ctx, base);
	// 8238EC9C: 48000008  b 0x8238eca4
	pc = 0x8238ECA4; continue 'dispatch;
	// 8238ECA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8238ECA4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8238ECA8: 409A000C  bne cr6, 0x8238ecb4
	if !ctx.cr[6].eq {
	pc = 0x8238ECB4; continue 'dispatch;
	}
	// 8238ECAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8238ECB0: 4800002C  b 0x8238ecdc
	pc = 0x8238ECDC; continue 'dispatch;
	// 8238ECB4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8238ECB8: 480F16A9  bl 0x82480360
	ctx.lr = 0x8238ECBC;
	sub_82480360(ctx, base);
	// 8238ECBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238ECC0: 4182FFEC  beq 0x8238ecac
	if ctx.cr[0].eq {
	pc = 0x8238ECAC; continue 'dispatch;
	}
	// 8238ECC4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238ECC8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8238ECCC: 388BC4D4  addi r4, r11, -0x3b2c
	ctx.r[4].s64 = ctx.r[11].s64 + -15148;
	// 8238ECD0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8238ECD4: 480F1285  bl 0x8247ff58
	ctx.lr = 0x8238ECD8;
	sub_8247FF58(ctx, base);
	// 8238ECD8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8238ECDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8238ECE0: 481A6428  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238ECE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238ECE8 size=116
    let mut pc: u32 = 0x8238ECE8;
    'dispatch: loop {
        match pc {
            0x8238ECE8 => {
    //   block [0x8238ECE8..0x8238ED5C)
	// 8238ECE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238ECEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238ECF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238ECF4: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238ECF8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8238ECFC: 81040004  lwz r8, 4(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8238ED00: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8238ED04: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238ED08: 386AFFF8  addi r3, r10, -8
	ctx.r[3].s64 = ctx.r[10].s64 + -8;
	// 8238ED0C: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238ED10: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238ED14: 814AFFF8  lwz r10, -8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238ED18: C00B001C  lfs f0, 0x1c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238ED1C: C1AB0018  lfs f13, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238ED20: C18B0014  lfs f12, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238ED24: 80890008  lwz r4, 8(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238ED28: C16B0010  lfs f11, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8238ED2C: 80A80008  lwz r5, 8(r8)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 8238ED30: D1610050  stfs f11, 0x50(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8238ED34: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8238ED38: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8238ED3C: D1A10058  stfs f13, 0x58(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8238ED40: D1810054  stfs f12, 0x54(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8238ED44: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8238ED48: 4E800421  bctrl
	ctx.lr = 0x8238ED4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238ED4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8238ED50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238ED54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238ED58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238ED60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238ED60 size=60
    let mut pc: u32 = 0x8238ED60;
    'dispatch: loop {
        match pc {
            0x8238ED60 => {
    //   block [0x8238ED60..0x8238ED9C)
	// 8238ED60: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8238ED64: 54C807FE  clrlwi r8, r6, 0x1f
	ctx.r[8].u64 = ctx.r[6].u32 as u64 & 0x00000001u64;
	// 8238ED68: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8238ED6C: 409A0020  bne cr6, 0x8238ed8c
	if !ctx.cr[6].eq {
	pc = 0x8238ED8C; continue 'dispatch;
	}
	// 8238ED70: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8238ED74: 2B2A0000  cmpldi cr6, r10, 0
	ctx.cr[6].compare_u64(ctx.r[10].u64, 0, &mut ctx.xer);
	// 8238ED78: 40990014  ble cr6, 0x8238ed8c
	if !ctx.cr[6].gt {
	pc = 0x8238ED8C; continue 'dispatch;
	}
	// 8238ED7C: 796BF842  rldicl r11, r11, 0x3f, 1
	ctx.r[11].u64 = ctx.r[11].u64 & 0x0000000000000001u64;
	// 8238ED80: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 8238ED84: 2B2B0000  cmpldi cr6, r11, 0
	ctx.cr[6].compare_u64(ctx.r[11].u64, 0, &mut ctx.xer);
	// 8238ED88: 4199FFF4  bgt cr6, 0x8238ed7c
	if ctx.cr[6].gt {
	pc = 0x8238ED7C; continue 'dispatch;
	}
	// 8238ED8C: 7D632A14  add r11, r3, r5
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[5].u64;
	// 8238ED90: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8238ED94: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8238ED98: 4800001C  b 0x8238edb4
	sub_8238ED9C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238ED9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238ED9C size=72
    let mut pc: u32 = 0x8238ED9C;
    'dispatch: loop {
        match pc {
            0x8238ED9C => {
    //   block [0x8238ED9C..0x8238EDE4)
	// 8238ED9C: 2F2A0000  cmpdi cr6, r10, 0
	ctx.cr[6].compare_i64(ctx.r[10].s64, 0, &mut ctx.xer);
	// 8238EDA0: 419A002C  beq cr6, 0x8238edcc
	if ctx.cr[6].eq {
	pc = 0x8238EDCC; continue 'dispatch;
	}
	// 8238EDA4: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 8238EDA8: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 8238EDAC: 7D4A0E74  sradi r10, r10, 1
	ctx.xer.ca = (ctx.r[10].s64 < 0) && ((ctx.r[10].u64 & ((1u64 << 1) - 1)) != 0);
	ctx.r[10].s64 = ctx.r[10].s64 >> 1;
	// 8238EDB0: 39290030  addi r9, r9, 0x30
	ctx.r[9].s64 = ctx.r[9].s64 + 48;
	// 8238EDB4: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8238EDB8: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8238EDBC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238EDC0: 4199FFDC  bgt cr6, 0x8238ed9c
	if ctx.cr[6].gt {
	pc = 0x8238ED9C; continue 'dispatch;
	}
	// 8238EDC4: 2F2A0000  cmpdi cr6, r10, 0
	ctx.cr[6].compare_i64(ctx.r[10].s64, 0, &mut ctx.xer);
	// 8238EDC8: 409A0030  bne cr6, 0x8238edf8
	if !ctx.cr[6].eq {
		sub_8238EDF8(ctx, base);
		return;
	}
	// 8238EDCC: 21480000  subfic r10, r8, 0
	ctx.xer.ca = ctx.r[8].u32 <= 0 as u32;
	ctx.r[10].s64 = (0 as i64) - ctx.r[8].s64;
	// 8238EDD0: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8238EDD4: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8238EDD8: 554A06F6  rlwinm r10, r10, 0, 0x1b, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8238EDDC: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 8238EDE0: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238EDE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238EDE4 size=20
    let mut pc: u32 = 0x8238EDE4;
    'dispatch: loop {
        match pc {
            0x8238EDE4 => {
    //   block [0x8238EDE4..0x8238EDF8)
	// 8238EDE4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8238EDE8: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8238EDEC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238EDF0: 4181FFF4  bgt 0x8238ede4
	if ctx.cr[0].gt {
	pc = 0x8238EDE4; continue 'dispatch;
	}
	// 8238EDF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238EDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238EDF8 size=28
    let mut pc: u32 = 0x8238EDF8;
    'dispatch: loop {
        match pc {
            0x8238EDF8 => {
    //   block [0x8238EDF8..0x8238EE14)
	// 8238EDF8: 7D490E74  sradi r9, r10, 1
	ctx.xer.ca = (ctx.r[10].s64 < 0) && ((ctx.r[10].u64 & ((1u64 << 1) - 1)) != 0);
	ctx.r[9].s64 = ctx.r[10].s64 >> 1;
	// 8238EDFC: 2F290000  cmpdi cr6, r9, 0
	ctx.cr[6].compare_i64(ctx.r[9].s64, 0, &mut ctx.xer);
	// 8238EE00: 419A0014  beq cr6, 0x8238ee14
	if ctx.cr[6].eq {
		sub_8238EE14(ctx, base);
		return;
	}
	// 8238EE04: 2F29FFFF  cmpdi cr6, r9, -1
	ctx.cr[6].compare_i64(ctx.r[9].s64, -1, &mut ctx.xer);
	// 8238EE08: 419A000C  beq cr6, 0x8238ee14
	if ctx.cr[6].eq {
		sub_8238EE14(ctx, base);
		return;
	}
	// 8238EE0C: 39400056  li r10, 0x56
	ctx.r[10].s64 = 86;
	// 8238EE10: 4800000C  b 0x8238ee1c
	sub_8238EE14(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238EE14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238EE14 size=16
    let mut pc: u32 = 0x8238EE14;
    'dispatch: loop {
        match pc {
            0x8238EE14 => {
    //   block [0x8238EE14..0x8238EE24)
	// 8238EE14: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 8238EE18: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	// 8238EE1C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8238EE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238EE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238EE28 size=72
    let mut pc: u32 = 0x8238EE28;
    'dispatch: loop {
        match pc {
            0x8238EE28 => {
    //   block [0x8238EE28..0x8238EE70)
	// 8238EE28: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8238EE2C: 54C907FE  clrlwi r9, r6, 0x1f
	ctx.r[9].u64 = ctx.r[6].u32 as u64 & 0x00000001u64;
	// 8238EE30: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8238EE34: 409A002C  bne cr6, 0x8238ee60
	if !ctx.cr[6].eq {
	pc = 0x8238EE60; continue 'dispatch;
	}
	// 8238EE38: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8238EE3C: 2B2A0000  cmpldi cr6, r10, 0
	ctx.cr[6].compare_u64(ctx.r[10].u64, 0, &mut ctx.xer);
	// 8238EE40: 4099001C  ble cr6, 0x8238ee5c
	if !ctx.cr[6].gt {
	pc = 0x8238EE5C; continue 'dispatch;
	}
	// 8238EE44: 796BE102  rldicl r11, r11, 0x3c, 4
	ctx.r[11].u64 = ctx.r[11].u64 & 0x000000000000000Fu64;
	// 8238EE48: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 8238EE4C: 2B2B0000  cmpldi cr6, r11, 0
	ctx.cr[6].compare_u64(ctx.r[11].u64, 0, &mut ctx.xer);
	// 8238EE50: 4199FFF4  bgt cr6, 0x8238ee44
	if ctx.cr[6].gt {
	pc = 0x8238EE44; continue 'dispatch;
	}
	// 8238EE54: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8238EE58: 409A0008  bne cr6, 0x8238ee60
	if !ctx.cr[6].eq {
	pc = 0x8238EE60; continue 'dispatch;
	}
	// 8238EE5C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8238EE60: 7D632A14  add r11, r3, r5
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[5].u64;
	// 8238EE64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8238EE68: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8238EE6C: 4800001C  b 0x8238ee88
	sub_8238EE70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238EE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238EE70 size=72
    let mut pc: u32 = 0x8238EE70;
    'dispatch: loop {
        match pc {
            0x8238EE70 => {
    //   block [0x8238EE70..0x8238EEB8)
	// 8238EE70: 2B2A0000  cmpldi cr6, r10, 0
	ctx.cr[6].compare_u64(ctx.r[10].u64, 0, &mut ctx.xer);
	// 8238EE74: 419A002C  beq cr6, 0x8238eea0
	if ctx.cr[6].eq {
	pc = 0x8238EEA0; continue 'dispatch;
	}
	// 8238EE78: 5548073E  clrlwi r8, r10, 0x1c
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 8238EE7C: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 8238EE80: 794AE102  rldicl r10, r10, 0x3c, 4
	ctx.r[10].u64 = ctx.r[10].u64 & 0x000000000000000Fu64;
	// 8238EE84: 7D0838AE  lbzx r8, r8, r7
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8238EE88: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 8238EE8C: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8238EE90: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238EE94: 4199FFDC  bgt cr6, 0x8238ee70
	if ctx.cr[6].gt {
	pc = 0x8238EE70; continue 'dispatch;
	}
	// 8238EE98: 2B2A0000  cmpldi cr6, r10, 0
	ctx.cr[6].compare_u64(ctx.r[10].u64, 0, &mut ctx.xer);
	// 8238EE9C: 409A0030  bne cr6, 0x8238eecc
	if !ctx.cr[6].eq {
		sub_8238EECC(ctx, base);
		return;
	}
	// 8238EEA0: 21490000  subfic r10, r9, 0
	ctx.xer.ca = ctx.r[9].u32 <= 0 as u32;
	ctx.r[10].s64 = (0 as i64) - ctx.r[9].s64;
	// 8238EEA4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8238EEA8: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8238EEAC: 554A06F6  rlwinm r10, r10, 0, 0x1b, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8238EEB0: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 8238EEB4: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238EEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238EEB8 size=20
    let mut pc: u32 = 0x8238EEB8;
    'dispatch: loop {
        match pc {
            0x8238EEB8 => {
    //   block [0x8238EEB8..0x8238EECC)
	// 8238EEB8: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8238EEBC: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8238EEC0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238EEC4: 4181FFF4  bgt 0x8238eeb8
	if ctx.cr[0].gt {
	pc = 0x8238EEB8; continue 'dispatch;
	}
	// 8238EEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238EECC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238EECC size=28
    let mut pc: u32 = 0x8238EECC;
    'dispatch: loop {
        match pc {
            0x8238EECC => {
    //   block [0x8238EECC..0x8238EEE8)
	// 8238EECC: 7949E102  rldicl r9, r10, 0x3c, 4
	ctx.r[9].u64 = ctx.r[10].u64 & 0x000000000000000Fu64;
	// 8238EED0: 2F290000  cmpdi cr6, r9, 0
	ctx.cr[6].compare_i64(ctx.r[9].s64, 0, &mut ctx.xer);
	// 8238EED4: 419A0014  beq cr6, 0x8238eee8
	if ctx.cr[6].eq {
		sub_8238EEE8(ctx, base);
		return;
	}
	// 8238EED8: 2F29FFFF  cmpdi cr6, r9, -1
	ctx.cr[6].compare_i64(ctx.r[9].s64, -1, &mut ctx.xer);
	// 8238EEDC: 419A000C  beq cr6, 0x8238eee8
	if ctx.cr[6].eq {
		sub_8238EEE8(ctx, base);
		return;
	}
	// 8238EEE0: 39400056  li r10, 0x56
	ctx.r[10].s64 = 86;
	// 8238EEE4: 4800000C  b 0x8238eef0
	sub_8238EEE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238EEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238EEE8 size=16
    let mut pc: u32 = 0x8238EEE8;
    'dispatch: loop {
        match pc {
            0x8238EEE8 => {
    //   block [0x8238EEE8..0x8238EEF8)
	// 8238EEE8: 554A073E  clrlwi r10, r10, 0x1c
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 8238EEEC: 7D4A38AE  lbzx r10, r10, r7
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8238EEF0: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8238EEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238EEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238EEF8 size=36
    let mut pc: u32 = 0x8238EEF8;
    'dispatch: loop {
        match pc {
            0x8238EEF8 => {
    //   block [0x8238EEF8..0x8238EF1C)
	// 8238EEF8: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8238EEFC: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 8238EF00: 409A001C  bne cr6, 0x8238ef1c
	if !ctx.cr[6].eq {
		sub_8238EF1C(ctx, base);
		return;
	}
	// 8238EF04: 2F2A0000  cmpdi cr6, r10, 0
	ctx.cr[6].compare_i64(ctx.r[10].s64, 0, &mut ctx.xer);
	// 8238EF08: 40980014  bge cr6, 0x8238ef1c
	if !ctx.cr[6].lt {
		sub_8238EF1C(ctx, base);
		return;
	}
	// 8238EF0C: 7D4A00D0  neg r10, r10
	ctx.r[10].s64 = -ctx.r[10].s64;
	// 8238EF10: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8238EF14: 3880002D  li r4, 0x2d
	ctx.r[4].s64 = 45;
	// 8238EF18: 48000020  b 0x8238ef38
	sub_8238EF30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238EF1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238EF1C size=20
    let mut pc: u32 = 0x8238EF1C;
    'dispatch: loop {
        match pc {
            0x8238EF1C => {
    //   block [0x8238EF1C..0x8238EF30)
	// 8238EF1C: 54CB07BD  rlwinm. r11, r6, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238EF20: 41820010  beq 0x8238ef30
	if ctx.cr[0].eq {
		sub_8238EF30(ctx, base);
		return;
	}
	// 8238EF24: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8238EF28: 3880002B  li r4, 0x2b
	ctx.r[4].s64 = 43;
	// 8238EF2C: 4800000C  b 0x8238ef38
	sub_8238EF30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238EF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238EF30 size=72
    let mut pc: u32 = 0x8238EF30;
    'dispatch: loop {
        match pc {
            0x8238EF30 => {
    //   block [0x8238EF30..0x8238EF78)
	// 8238EF30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8238EF34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8238EF38: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8238EF3C: 54C607FE  clrlwi r6, r6, 0x1f
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0x00000001u64;
	// 8238EF40: 79690020  clrldi r9, r11, 0x20
	ctx.r[9].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 8238EF44: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 8238EF48: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8238EF4C: 409A0058  bne cr6, 0x8238efa4
	if !ctx.cr[6].eq {
		sub_8238EFA4(ctx, base);
		return;
	}
	// 8238EF50: 7F2A4840  cmpld cr6, r10, r9
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[9].u64, &mut ctx.xer);
	// 8238EF54: 41990024  bgt cr6, 0x8238ef78
	if ctx.cr[6].gt {
		sub_8238EF78(ctx, base);
		return;
	}
	// 8238EF58: 554B003F  rotlwi. r11, r10, 0
	ctx.r[11].u64 = ((ctx.r[10].u32).rotate_left(0)) as u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238EF5C: 41820040  beq 0x8238ef9c
	if ctx.cr[0].eq {
		sub_8238EF78(ctx, base);
		return;
	}
	// 8238EF60: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 8238EF64: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 8238EF68: 7D6B3BD2  divd r11, r11, r7
	ctx.r[11].s64 = ctx.r[11].s64 / ctx.r[7].s64;
	// 8238EF6C: 556B003F  rotlwi. r11, r11, 0
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(0)) as u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238EF70: 4082FFF0  bne 0x8238ef60
	if !ctx.cr[0].eq {
	pc = 0x8238EF60; continue 'dispatch;
	}
	// 8238EF74: 48000020  b 0x8238ef94
	sub_8238EF78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238EF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238EF78 size=44
    let mut pc: u32 = 0x8238EF78;
    'dispatch: loop {
        match pc {
            0x8238EF78 => {
    //   block [0x8238EF78..0x8238EFA4)
	// 8238EF78: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8238EF7C: 2B2A0000  cmpldi cr6, r10, 0
	ctx.cr[6].compare_u64(ctx.r[10].u64, 0, &mut ctx.xer);
	// 8238EF80: 4099001C  ble cr6, 0x8238ef9c
	if !ctx.cr[6].gt {
	pc = 0x8238EF9C; continue 'dispatch;
	}
	// 8238EF84: 7D6B3B92  divdu r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 / ctx.r[7].u64;
	// 8238EF88: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 8238EF8C: 2B2B0000  cmpldi cr6, r11, 0
	ctx.cr[6].compare_u64(ctx.r[11].u64, 0, &mut ctx.xer);
	// 8238EF90: 4199FFF4  bgt cr6, 0x8238ef84
	if ctx.cr[6].gt {
	pc = 0x8238EF84; continue 'dispatch;
	}
	// 8238EF94: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8238EF98: 409A0010  bne cr6, 0x8238efa8
	if !ctx.cr[6].eq {
		sub_8238EFA4(ctx, base);
		return;
	}
	// 8238EF9C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8238EFA0: 48000008  b 0x8238efa8
	sub_8238EFA4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238EFA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238EFA4 size=44
    let mut pc: u32 = 0x8238EFA4;
    'dispatch: loop {
        match pc {
            0x8238EFA4 => {
    //   block [0x8238EFA4..0x8238EFD0)
	// 8238EFA4: 7CA82850  subf r5, r8, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[8].s64;
	// 8238EFA8: 7D681A14  add r11, r8, r3
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[3].u64;
	// 8238EFAC: 7F2A4840  cmpld cr6, r10, r9
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[9].u64, &mut ctx.xer);
	// 8238EFB0: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 8238EFB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8238EFB8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8238EFBC: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8238EFC0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238EFC4: 4199006C  bgt cr6, 0x8238f030
	if ctx.cr[6].gt {
		sub_8238F008(ctx, base);
		return;
	}
	// 8238EFC8: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8238EFCC: 4800002C  b 0x8238eff8
	sub_8238EFD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238EFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238EFD0 size=56
    let mut pc: u32 = 0x8238EFD0;
    'dispatch: loop {
        match pc {
            0x8238EFD0 => {
    //   block [0x8238EFD0..0x8238F008)
	// 8238EFD0: 2B0A000A  cmplwi cr6, r10, 0xa
	ctx.cr[6].compare_u32(ctx.r[10].u32, 10 as u32, &mut ctx.xer);
	// 8238EFD4: 4198002C  blt cr6, 0x8238f000
	if ctx.cr[6].lt {
	pc = 0x8238F000; continue 'dispatch;
	}
	// 8238EFD8: 7D2A3B96  divwu r9, r10, r7
	ctx.r[9].u32 = ctx.r[10].u32 / ctx.r[7].u32;
	// 8238EFDC: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 8238EFE0: 1D29000A  mulli r9, r9, 0xa
	ctx.r[9].s64 = ctx.r[9].s64 * 10;
	// 8238EFE4: 7D295050  subf r9, r9, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 8238EFE8: 7D4A3B96  divwu r10, r10, r7
	ctx.r[10].u32 = ctx.r[10].u32 / ctx.r[7].u32;
	// 8238EFEC: 39290030  addi r9, r9, 0x30
	ctx.r[9].s64 = ctx.r[9].s64 + 48;
	// 8238EFF0: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8238EFF4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238EFF8: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 8238EFFC: 4199FFD4  bgt cr6, 0x8238efd0
	if ctx.cr[6].gt {
	pc = 0x8238EFD0; continue 'dispatch;
	}
	// 8238F000: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 8238F004: 48000034  b 0x8238f038
	sub_8238F008(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238F008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238F008 size=64
    let mut pc: u32 = 0x8238F008;
    'dispatch: loop {
        match pc {
            0x8238F008 => {
    //   block [0x8238F008..0x8238F048)
	// 8238F008: 2B2A000A  cmpldi cr6, r10, 0xa
	ctx.cr[6].compare_u64(ctx.r[10].u64, 10, &mut ctx.xer);
	// 8238F00C: 4198002C  blt cr6, 0x8238f038
	if ctx.cr[6].lt {
	pc = 0x8238F038; continue 'dispatch;
	}
	// 8238F010: 7D2A3B92  divdu r9, r10, r7
	ctx.r[9].u64 = ctx.r[10].u64 / ctx.r[7].u64;
	// 8238F014: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 8238F018: 1D29000A  mulli r9, r9, 0xa
	ctx.r[9].s64 = ctx.r[9].s64 * 10;
	// 8238F01C: 7D295050  subf r9, r9, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 8238F020: 7D4A3B92  divdu r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 / ctx.r[7].u64;
	// 8238F024: 39290030  addi r9, r9, 0x30
	ctx.r[9].s64 = ctx.r[9].s64 + 48;
	// 8238F028: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8238F02C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238F030: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 8238F034: 4199FFD4  bgt cr6, 0x8238f008
	if ctx.cr[6].gt {
	pc = 0x8238F008; continue 'dispatch;
	}
	// 8238F038: 2F2A0009  cmpdi cr6, r10, 9
	ctx.cr[6].compare_i64(ctx.r[10].s64, 9, &mut ctx.xer);
	// 8238F03C: 4099000C  ble cr6, 0x8238f048
	if !ctx.cr[6].gt {
		sub_8238F048(ctx, base);
		return;
	}
	// 8238F040: 39400056  li r10, 0x56
	ctx.r[10].s64 = 86;
	// 8238F044: 48000008  b 0x8238f04c
	sub_8238F048(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238F048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238F048 size=88
    let mut pc: u32 = 0x8238F048;
    'dispatch: loop {
        match pc {
            0x8238F048 => {
    //   block [0x8238F048..0x8238F0A0)
	// 8238F048: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	// 8238F04C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8238F050: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 8238F054: 3965FFFF  addi r11, r5, -1
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	// 8238F058: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8238F05C: 409A0014  bne cr6, 0x8238f070
	if !ctx.cr[6].eq {
	pc = 0x8238F070; continue 'dispatch;
	}
	// 8238F060: 5489063F  clrlwi. r9, r4, 0x18
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8238F064: 4182000C  beq 0x8238f070
	if ctx.cr[0].eq {
	pc = 0x8238F070; continue 'dispatch;
	}
	// 8238F068: 988A0000  stb r4, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 8238F06C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8238F070: 21260000  subfic r9, r6, 0
	ctx.xer.ca = ctx.r[6].u32 <= 0 as u32;
	ctx.r[9].s64 = (0 as i64) - ctx.r[6].s64;
	// 8238F074: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8238F078: 7D294910  subfe r9, r9, r9
	let x = (!ctx.r[9].u32);
	let y = ctx.r[9].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[9].u32 = res;
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8238F07C: 552906F6  rlwinm r9, r9, 0, 0x1b, 0x1b
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 8238F080: 39290020  addi r9, r9, 0x20
	ctx.r[9].s64 = ctx.r[9].s64 + 32;
	// 8238F084: 419A0014  beq cr6, 0x8238f098
	if ctx.cr[6].eq {
	pc = 0x8238F098; continue 'dispatch;
	}
	// 8238F088: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8238F08C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238F090: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8238F094: 4082FFF4  bne 0x8238f088
	if !ctx.cr[0].eq {
	pc = 0x8238F088; continue 'dispatch;
	}
	// 8238F098: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8238F09C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238F0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238F0A0 size=8
    let mut pc: u32 = 0x8238F0A0;
    'dispatch: loop {
        match pc {
            0x8238F0A0 => {
    //   block [0x8238F0A0..0x8238F0A8)
	// 8238F0A0: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238F0A4: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238F0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238F0A8 size=8
    let mut pc: u32 = 0x8238F0A8;
    'dispatch: loop {
        match pc {
            0x8238F0A8 => {
    //   block [0x8238F0A8..0x8238F0B0)
	// 8238F0A8: 988A0000  stb r4, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 8238F0AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238F0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238F0B0 size=580
    let mut pc: u32 = 0x8238F0B0;
    'dispatch: loop {
        match pc {
            0x8238F0B0 => {
    //   block [0x8238F0B0..0x8238F2F4)
	// 8238F0B0: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 8238F0B4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8238F0B8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238F0BC: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238F0C0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8238F0C4: 40980014  bge cr6, 0x8238f0d8
	if !ctx.cr[6].lt {
	pc = 0x8238F0D8; continue 'dispatch;
	}
	// 8238F0C8: FC200850  fneg f1, f1
	ctx.f[1].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 8238F0CC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8238F0D0: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8238F0D4: 48000010  b 0x8238f0e4
	pc = 0x8238F0E4; continue 'dispatch;
	// 8238F0D8: 54EBFFFE  rlwinm r11, r7, 0x1f, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x00000001u64;
	// 8238F0DC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8238F0E0: 616A0002  ori r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u64 | 2;
	// 8238F0E4: 54FE07FE  clrlwi r30, r7, 0x1f
	ctx.r[30].u64 = ctx.r[7].u32 as u64 & 0x00000001u64;
	// 8238F0E8: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8238F0EC: 409A000C  bne cr6, 0x8238f0f8
	if !ctx.cr[6].eq {
	pc = 0x8238F0F8; continue 'dispatch;
	}
	// 8238F0F0: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8238F0F4: 48000024  b 0x8238f118
	pc = 0x8238F118; continue 'dispatch;
	// 8238F0F8: 2F060009  cmpwi cr6, r6, 9
	ctx.cr[6].compare_i32(ctx.r[6].s32, 9, &mut ctx.xer);
	// 8238F0FC: 40990008  ble cr6, 0x8238f104
	if !ctx.cr[6].gt {
	pc = 0x8238F104; continue 'dispatch;
	}
	// 8238F100: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 8238F104: 7D662850  subf r11, r6, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	// 8238F108: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8238F10C: 4098000C  bge cr6, 0x8238f118
	if !ctx.cr[6].lt {
	pc = 0x8238F118; continue 'dispatch;
	}
	// 8238F110: 7D6A2850  subf r11, r10, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[10].s64;
	// 8238F114: 38CBFFFF  addi r6, r11, -1
	ctx.r[6].s64 = ctx.r[11].s64 + -1;
	// 8238F118: 3D608284  lis r11, -0x7d7c
	ctx.r[11].s64 = -2105278464;
	// 8238F11C: FC00081E  fctiwz f0, f1
	ctx.f[0].s64 = if ctx.f[1].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[1].f64.trunc() as i32 as i64 };
	// 8238F120: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
	// 8238F124: 396BD570  addi r11, r11, -0x2a90
	ctx.r[11].s64 = ctx.r[11].s64 + -10896;
	// 8238F128: 54C8103A  slwi r8, r6, 2
	ctx.r[8].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8238F12C: 3881FFE0  addi r4, r1, -0x20
	ctx.r[4].s64 = ctx.r[1].s64 + -32;
	// 8238F130: 38EAFFFF  addi r7, r10, -1
	ctx.r[7].s64 = ctx.r[10].s64 + -1;
	// 8238F134: 7C004FAE  stfiwx f0, 0, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 8238F138: 3921FFE8  addi r9, r1, -0x18
	ctx.r[9].s64 = ctx.r[1].s64 + -24;
	// 8238F13C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8238F140: 7DA85C2E  lfsx f13, r8, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238F144: E961FFE2  lwa r11, -0x20(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) } as i32) as i64;
	// 8238F148: F961FFE8  std r11, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[11].u64 ) };
	// 8238F14C: C801FFE8  lfd f0, -0x18(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238F150: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8238F154: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238F158: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8238F15C: ED810028  fsubs f12, f1, f0
	ctx.f[12].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 8238F160: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8238F164: 7C0027AE  stfiwx f0, 0, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32, tmp.u32) };
	// 8238F168: C00BBFFC  lfs f0, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238F16C: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 8238F170: EC0C037A  fmadds f0, f12, f13, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 8238F174: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8238F178: 7C004FAE  stfiwx f0, 0, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 8238F17C: 8121FFE0  lwz r9, -0x20(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) } as u64;
	// 8238F180: 409A0030  bne cr6, 0x8238f1b0
	if !ctx.cr[6].eq {
	pc = 0x8238F1B0; continue 'dispatch;
	}
	// 8238F184: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8238F188: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8238F18C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8238F190: 40990018  ble cr6, 0x8238f1a8
	if !ctx.cr[6].gt {
	pc = 0x8238F1A8; continue 'dispatch;
	}
	// 8238F194: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8238F198: 7D6B23D7  divw. r11, r11, r4
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[4].s32;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238F19C: 4181FFF8  bgt 0x8238f194
	if ctx.cr[0].gt {
	pc = 0x8238F194; continue 'dispatch;
	}
	// 8238F1A0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8238F1A4: 409A0014  bne cr6, 0x8238f1b8
	if !ctx.cr[6].eq {
	pc = 0x8238F1B8; continue 'dispatch;
	}
	// 8238F1A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8238F1AC: 4800000C  b 0x8238f1b8
	pc = 0x8238F1B8; continue 'dispatch;
	// 8238F1B0: 7D672850  subf r11, r7, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[7].s64;
	// 8238F1B4: 7D465850  subf r10, r6, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 8238F1B8: 7D6A3A14  add r11, r10, r7
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 8238F1BC: 3900002E  li r8, 0x2e
	ctx.r[8].s64 = 46;
	// 8238F1C0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8238F1C4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8238F1C8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238F1CC: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 8238F1D0: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 8238F1D4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238F1D8: 419A004C  beq cr6, 0x8238f224
	if ctx.cr[6].eq {
	pc = 0x8238F224; continue 'dispatch;
	}
	// 8238F1DC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8238F1E0: 419A0030  beq cr6, 0x8238f210
	if ctx.cr[6].eq {
	pc = 0x8238F210; continue 'dispatch;
	}
	// 8238F1E4: 7D0923D6  divw r8, r9, r4
	ctx.r[8].s32 = ctx.r[9].s32 / ctx.r[4].s32;
	// 8238F1E8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8238F1EC: 1D08000A  mulli r8, r8, 0xa
	ctx.r[8].s64 = ctx.r[8].s64 * 10;
	// 8238F1F0: 7D084850  subf r8, r8, r9
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 8238F1F4: 7D2923D7  divw. r9, r9, r4
	ctx.r[9].s32 = ctx.r[9].s32 / ctx.r[4].s32;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8238F1F8: 39080030  addi r8, r8, 0x30
	ctx.r[8].s64 = ctx.r[8].s64 + 48;
	// 8238F1FC: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 8238F200: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238F204: 4082FFD8  bne 0x8238f1dc
	if !ctx.cr[0].eq {
	pc = 0x8238F1DC; continue 'dispatch;
	}
	// 8238F208: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8238F20C: 409A0028  bne cr6, 0x8238f234
	if !ctx.cr[6].eq {
	pc = 0x8238F234; continue 'dispatch;
	}
	// 8238F210: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8238F214: 419A0020  beq cr6, 0x8238f234
	if ctx.cr[6].eq {
	pc = 0x8238F234; continue 'dispatch;
	}
	// 8238F218: 39200056  li r9, 0x56
	ctx.r[9].s64 = 86;
	// 8238F21C: 992B0001  stb r9, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[9].u8 ) };
	// 8238F220: 48000014  b 0x8238f234
	pc = 0x8238F234; continue 'dispatch;
	// 8238F224: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 8238F228: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8238F22C: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8238F230: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238F234: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8238F238: 409A0028  bne cr6, 0x8238f260
	if !ctx.cr[6].eq {
	pc = 0x8238F260; continue 'dispatch;
	}
	// 8238F23C: 2F070001  cmpwi cr6, r7, 1
	ctx.cr[6].compare_i32(ctx.r[7].s32, 1, &mut ctx.xer);
	// 8238F240: 40990020  ble cr6, 0x8238f260
	if !ctx.cr[6].gt {
	pc = 0x8238F260; continue 'dispatch;
	}
	// 8238F244: 213F0000  subfic r9, r31, 0
	ctx.xer.ca = ctx.r[31].u32 <= 0 as u32;
	ctx.r[9].s64 = (0 as i64) - ctx.r[31].s64;
	// 8238F248: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8238F24C: 7D294910  subfe r9, r9, r9
	let x = (!ctx.r[9].u32);
	let y = ctx.r[9].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[9].u32 = res;
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8238F250: 552907BC  rlwinm r9, r9, 0, 0x1e, 0x1e
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 8238F254: 3929002B  addi r9, r9, 0x2b
	ctx.r[9].s64 = ctx.r[9].s64 + 43;
	// 8238F258: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8238F25C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238F260: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8238F264: 419A0020  beq cr6, 0x8238f284
	if ctx.cr[6].eq {
	pc = 0x8238F284; continue 'dispatch;
	}
	// 8238F268: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8238F26C: 53C926F6  rlwimi r9, r30, 4, 0x1b, 0x1b
	ctx.r[9].u64 = (((ctx.r[30].u32).rotate_left(4) as u64) & 0x0000000000000010) | (ctx.r[9].u64 & 0xFFFFFFFFFFFFFFEF);
	// 8238F270: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 8238F274: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8238F278: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8238F27C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238F280: 4082FFF4  bne 0x8238f274
	if !ctx.cr[0].eq {
	pc = 0x8238F274; continue 'dispatch;
	}
	// 8238F284: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8238F288: 419A0018  beq cr6, 0x8238f2a0
	if ctx.cr[6].eq {
	pc = 0x8238F2A0; continue 'dispatch;
	}
	// 8238F28C: 215F0000  subfic r10, r31, 0
	ctx.xer.ca = ctx.r[31].u32 <= 0 as u32;
	ctx.r[10].s64 = (0 as i64) - ctx.r[31].s64;
	// 8238F290: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8238F294: 554A07BC  rlwinm r10, r10, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8238F298: 394A002B  addi r10, r10, 0x2b
	ctx.r[10].s64 = ctx.r[10].s64 + 43;
	// 8238F29C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8238F2A0: 7D653214  add r11, r5, r6
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 8238F2A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8238F2A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8238F2AC: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8238F2B0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8238F2B4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8238F2B8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238F2BC: 4099002C  ble cr6, 0x8238f2e8
	if !ctx.cr[6].gt {
	pc = 0x8238F2E8; continue 'dispatch;
	}
	// 8238F2C0: 8141FFE8  lwz r10, -0x18(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) } as u64;
	// 8238F2C4: 7D2A23D6  divw r9, r10, r4
	ctx.r[9].s32 = ctx.r[10].s32 / ctx.r[4].s32;
	// 8238F2C8: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8238F2CC: 1D29000A  mulli r9, r9, 0xa
	ctx.r[9].s64 = ctx.r[9].s64 * 10;
	// 8238F2D0: 7D295050  subf r9, r9, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 8238F2D4: 7D4A23D6  divw r10, r10, r4
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[4].s32;
	// 8238F2D8: 39290030  addi r9, r9, 0x30
	ctx.r[9].s64 = ctx.r[9].s64 + 48;
	// 8238F2DC: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8238F2E0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238F2E4: 4181FFE0  bgt 0x8238f2c4
	if ctx.cr[0].gt {
	pc = 0x8238F2C4; continue 'dispatch;
	}
	// 8238F2E8: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238F2EC: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8238F2F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238F2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238F2F8 size=116
    let mut pc: u32 = 0x8238F2F8;
    'dispatch: loop {
        match pc {
            0x8238F2F8 => {
    //   block [0x8238F2F8..0x8238F36C)
	// 8238F2F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8238F2FC: 54CA07BD  rlwinm. r10, r6, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8238F300: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 8238F304: 40820068  bne 0x8238f36c
	if !ctx.cr[0].eq {
		sub_8238F36C(ctx, base);
		return;
	}
	// 8238F308: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8238F30C: 40990050  ble cr6, 0x8238f35c
	if !ctx.cr[6].gt {
	pc = 0x8238F35C; continue 'dispatch;
	}
	// 8238F310: 7D2B2050  subf r9, r11, r4
	ctx.r[9].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 8238F314: 7D4958AE  lbzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8238F318: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238F31C: 41820014  beq 0x8238f330
	if ctx.cr[0].eq {
	pc = 0x8238F330; continue 'dispatch;
	}
	// 8238F320: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8238F324: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8238F328: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8238F32C: 4181FFE8  bgt 0x8238f314
	if ctx.cr[0].gt {
	pc = 0x8238F314; continue 'dispatch;
	}
	// 8238F330: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8238F334: 40990028  ble cr6, 0x8238f35c
	if !ctx.cr[6].gt {
	pc = 0x8238F35C; continue 'dispatch;
	}
	// 8238F338: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8238F33C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8238F340: 28050000  cmplwi r5, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238F344: 41820014  beq 0x8238f358
	if ctx.cr[0].eq {
	pc = 0x8238F358; continue 'dispatch;
	}
	// 8238F348: 7CA903A6  mtctr r5
	ctx.ctr.u64 = ctx.r[5].u64;
	// 8238F34C: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8238F350: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8238F354: 4200FFF8  bdnz 0x8238f34c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8238F34C; continue 'dispatch;
	}
	// 8238F358: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 8238F35C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8238F360: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8238F364: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8238F368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238F36C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238F36C size=28
    let mut pc: u32 = 0x8238F36C;
    'dispatch: loop {
        match pc {
            0x8238F36C => {
    //   block [0x8238F36C..0x8238F388)
	// 8238F36C: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 8238F370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8238F374: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8238F378: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8238F37C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238F380: 89040000  lbz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238F384: 48000010  b 0x8238f394
	sub_8238F388(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238F388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238F388 size=32
    let mut pc: u32 = 0x8238F388;
    'dispatch: loop {
        match pc {
            0x8238F388 => {
    //   block [0x8238F388..0x8238F3A8)
	// 8238F388: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8238F38C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8238F390: 89090000  lbz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238F394: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238F398: 4082FFF0  bne 0x8238f388
	if !ctx.cr[0].eq {
	pc = 0x8238F388; continue 'dispatch;
	}
	// 8238F39C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8238F3A0: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8238F3A4: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238F3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238F3A8 size=44
    let mut pc: u32 = 0x8238F3A8;
    'dispatch: loop {
        match pc {
            0x8238F3A8 => {
    //   block [0x8238F3A8..0x8238F3D4)
	// 8238F3A8: 2B2A0000  cmpldi cr6, r10, 0
	ctx.cr[6].compare_u64(ctx.r[10].u64, 0, &mut ctx.xer);
	// 8238F3AC: 40990020  ble cr6, 0x8238f3cc
	if !ctx.cr[6].gt {
	pc = 0x8238F3CC; continue 'dispatch;
	}
	// 8238F3B0: 89090000  lbz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238F3B4: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8238F3B8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8238F3BC: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8238F3C0: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 8238F3C4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238F3C8: 4181FFE0  bgt 0x8238f3a8
	if ctx.cr[0].gt {
	pc = 0x8238F3A8; continue 'dispatch;
	}
	// 8238F3CC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8238F3D0: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238F3D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238F3D4 size=24
    let mut pc: u32 = 0x8238F3D4;
    'dispatch: loop {
        match pc {
            0x8238F3D4 => {
    //   block [0x8238F3D4..0x8238F3EC)
	// 8238F3D4: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 8238F3D8: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8238F3DC: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8238F3E0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238F3E4: 4181FFF0  bgt 0x8238f3d4
	if ctx.cr[0].gt {
	pc = 0x8238F3D4; continue 'dispatch;
	}
	// 8238F3E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238F3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8238F3F0 size=272
    let mut pc: u32 = 0x8238F3F0;
    'dispatch: loop {
        match pc {
            0x8238F3F0 => {
    //   block [0x8238F3F0..0x8238F500)
	// 8238F3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238F3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238F3F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8238F3FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8238F400: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238F404: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8238F408: 90A102A4  stw r5, 0x2a4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(676 as u32), ctx.r[5].u32 ) };
	// 8238F40C: 54EB07BD  rlwinm. r11, r7, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8238F410: 90810050  stw r4, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 8238F414: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8238F418: 817E007C  lwz r11, 0x7c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 8238F41C: 40820044  bne 0x8238f460
	if !ctx.cr[0].eq {
	pc = 0x8238F460; continue 'dispatch;
	}
	// 8238F420: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8238F424: 388102A4  addi r4, r1, 0x2a4
	ctx.r[4].s64 = ctx.r[1].s64 + 676;
	// 8238F428: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8238F42C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238F430: 4E800421  bctrl
	ctx.lr = 0x8238F434;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238F434: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8238F438: 408100A0  ble 0x8238f4d8
	if !ctx.cr[0].gt {
	pc = 0x8238F4D8; continue 'dispatch;
	}
	// 8238F43C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8238F440: 39400009  li r10, 9
	ctx.r[10].s64 = 9;
	// 8238F444: 3463FFFF  addic. r3, r3, -1
	ctx.xer.ca = (ctx.r[3].u32 > (!(-1 as u32)));
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8238F448: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8238F44C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8238F450: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8238F454: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8238F458: 4181FFE4  bgt 0x8238f43c
	if ctx.cr[0].gt {
	pc = 0x8238F43C; continue 'dispatch;
	}
	// 8238F45C: 4800007C  b 0x8238f4d8
	pc = 0x8238F4D8; continue 'dispatch;
	// 8238F460: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8238F464: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 8238F468: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 8238F46C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8238F470: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8238F474: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8238F478: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238F47C: 4E800421  bctrl
	ctx.lr = 0x8238F480;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238F480: 20A30080  subfic r5, r3, 0x80
	ctx.xer.ca = ctx.r[3].u32 <= 128 as u32;
	ctx.r[5].s64 = (128 as i64) - ctx.r[3].s64;
	// 8238F484: 7F05F840  cmplw cr6, r5, r31
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8238F488: 40980038  bge cr6, 0x8238f4c0
	if !ctx.cr[6].lt {
	pc = 0x8238F4C0; continue 'dispatch;
	}
	// 8238F48C: 7D65F850  subf r11, r5, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[5].s64;
	// 8238F490: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 8238F494: 48000020  b 0x8238f4b4
	pc = 0x8238F4B4; continue 'dispatch;
	// 8238F498: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8238F49C: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8238F4A0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8238F4A4: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8238F4A8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8238F4AC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8238F4B0: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8238F4B4: 2B2B0000  cmpldi cr6, r11, 0
	ctx.cr[6].compare_u64(ctx.r[11].u64, 0, &mut ctx.xer);
	// 8238F4B8: 4199FFE0  bgt cr6, 0x8238f498
	if ctx.cr[6].gt {
	pc = 0x8238F498; continue 'dispatch;
	}
	// 8238F4BC: 48000008  b 0x8238f4c4
	pc = 0x8238F4C4; continue 'dispatch;
	// 8238F4C0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8238F4C4: 817E007C  lwz r11, 0x7c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 8238F4C8: 388102A4  addi r4, r1, 0x2a4
	ctx.r[4].s64 = ctx.r[1].s64 + 676;
	// 8238F4CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8238F4D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8238F4D4: 4E800421  bctrl
	ctx.lr = 0x8238F4D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8238F4D8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8238F4DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8238F4E0: 996A0000  stb r11, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8238F4E4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8238F4E8: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 8238F4EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238F4F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238F4F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8238F4F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8238F4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238F500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238F500 size=80
    let mut pc: u32 = 0x8238F500;
    'dispatch: loop {
        match pc {
            0x8238F500 => {
    //   block [0x8238F500..0x8238F550)
	// 8238F500: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8238F504: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8238F508: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8238F50C: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238F510: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238F514: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8238F518: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238F51C: 41820010  beq 0x8238f52c
	if ctx.cr[0].eq {
	pc = 0x8238F52C; continue 'dispatch;
	}
	// 8238F520: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 8238F524: 2B090020  cmplwi cr6, r9, 0x20
	ctx.cr[6].compare_u32(ctx.r[9].u32, 32 as u32, &mut ctx.xer);
	// 8238F528: 4198FFE8  blt cr6, 0x8238f510
	if ctx.cr[6].lt {
	pc = 0x8238F510; continue 'dispatch;
	}
	// 8238F52C: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 8238F530: 419A0020  beq cr6, 0x8238f550
	if ctx.cr[6].eq {
		sub_8238F550(ctx, base);
		return;
	}
	// 8238F534: 392BFFD0  addi r9, r11, -0x30
	ctx.r[9].s64 = ctx.r[11].s64 + -48;
	// 8238F538: 2B090009  cmplwi cr6, r9, 9
	ctx.cr[6].compare_u32(ctx.r[9].u32, 9 as u32, &mut ctx.xer);
	// 8238F53C: 41990024  bgt cr6, 0x8238f560
	if ctx.cr[6].gt {
		sub_8238F560(ctx, base);
		return;
	}
	// 8238F540: 1D23000A  mulli r9, r3, 0xa
	ctx.r[9].s64 = ctx.r[3].s64 * 10;
	// 8238F544: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8238F548: 386BFFD0  addi r3, r11, -0x30
	ctx.r[3].s64 = ctx.r[11].s64 + -48;
	// 8238F54C: 48000008  b 0x8238f554
	sub_8238F550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238F550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238F550 size=16
    let mut pc: u32 = 0x8238F550;
    'dispatch: loop {
        match pc {
            0x8238F550 => {
    //   block [0x8238F550..0x8238F560)
	// 8238F550: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8238F554: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238F558: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8238F55C: 4BFFFFD0  b 0x8238f52c
	sub_8238F500(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238F560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8238F560 size=32
    let mut pc: u32 = 0x8238F560;
    'dispatch: loop {
        match pc {
            0x8238F560 => {
    //   block [0x8238F560..0x8238F580)
	// 8238F560: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 8238F564: 419A0008  beq cr6, 0x8238f56c
	if ctx.cr[6].eq {
	pc = 0x8238F56C; continue 'dispatch;
	}
	// 8238F568: 7C6300D0  neg r3, r3
	ctx.r[3].s64 = -ctx.r[3].s64;
	// 8238F56C: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 8238F570: 419A0008  beq cr6, 0x8238f578
	if ctx.cr[6].eq {
	pc = 0x8238F578; continue 'dispatch;
	}
	// 8238F574: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8238F578: 91480000  stw r10, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8238F57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238F580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8238F580 size=284
    let mut pc: u32 = 0x8238F580;
    'dispatch: loop {
        match pc {
            0x8238F580 => {
    //   block [0x8238F580..0x8238F69C)
	// 8238F580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8238F584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8238F588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8238F58C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8238F590: 4BFFFF71  bl 0x8238f500
	ctx.lr = 0x8238F594;
	sub_8238F500(ctx, base);
	// 8238F594: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 8238F598: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238F59C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8238F5A0: C1AABA38  lfs f13, -0x45c8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8238F5A4: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238F5A8: 2B0A002E  cmplwi cr6, r10, 0x2e
	ctx.cr[6].compare_u32(ctx.r[10].u32, 46 as u32, &mut ctx.xer);
	// 8238F5AC: 409A008C  bne cr6, 0x8238f638
	if !ctx.cr[6].eq {
	pc = 0x8238F638; continue 'dispatch;
	}
	// 8238F5B0: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 8238F5B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8238F5B8: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238F5BC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8238F5C0: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 8238F5C4: 41980064  blt cr6, 0x8238f628
	if ctx.cr[6].lt {
	pc = 0x8238F628; continue 'dispatch;
	}
	// 8238F5C8: 3D008288  lis r8, -0x7d78
	ctx.r[8].s64 = -2105016320;
	// 8238F5CC: C008D4DC  lfs f0, -0x2b24(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-11044 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8238F5D0: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 8238F5D4: 41990030  bgt cr6, 0x8238f604
	if ctx.cr[6].gt {
	pc = 0x8238F604; continue 'dispatch;
	}
	// 8238F5D8: 2B090007  cmplwi cr6, r9, 7
	ctx.cr[6].compare_u32(ctx.r[9].u32, 7 as u32, &mut ctx.xer);
	// 8238F5DC: 40980044  bge cr6, 0x8238f620
	if !ctx.cr[6].lt {
	pc = 0x8238F620; continue 'dispatch;
	}
	// 8238F5E0: 1D07000A  mulli r8, r7, 0xa
	ctx.r[8].s64 = ctx.r[7].s64 * 10;
	// 8238F5E4: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8238F5E8: 7D085A14  add r8, r8, r11
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8238F5EC: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238F5F0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8238F5F4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8238F5F8: 38E8FFD0  addi r7, r8, -0x30
	ctx.r[7].s64 = ctx.r[8].s64 + -48;
	// 8238F5FC: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 8238F600: 4098FFD0  bge cr6, 0x8238f5d0
	if !ctx.cr[6].lt {
	pc = 0x8238F5D0; continue 'dispatch;
	}
	// 8238F604: 2B090007  cmplwi cr6, r9, 7
	ctx.cr[6].compare_u32(ctx.r[9].u32, 7 as u32, &mut ctx.xer);
	// 8238F608: 41980020  blt cr6, 0x8238f628
	if ctx.cr[6].lt {
	pc = 0x8238F628; continue 'dispatch;
	}
	// 8238F60C: 48000014  b 0x8238f620
	pc = 0x8238F620; continue 'dispatch;
	// 8238F610: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 8238F614: 41990014  bgt cr6, 0x8238f628
	if ctx.cr[6].gt {
	pc = 0x8238F628; continue 'dispatch;
	}
	// 8238F618: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238F61C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8238F620: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 8238F624: 4098FFEC  bge cr6, 0x8238f610
	if !ctx.cr[6].lt {
	pc = 0x8238F610; continue 'dispatch;
	}
	// 8238F628: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 8238F62C: 419A0008  beq cr6, 0x8238f634
	if ctx.cr[6].eq {
	pc = 0x8238F634; continue 'dispatch;
	}
	// 8238F630: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8238F634: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8238F638: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 8238F63C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8238F640: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8238F644: C18B1FF8  lfs f12, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8238F648: 7CEB07B4  extsw r11, r7
	ctx.r[11].s64 = ctx.r[7].s32 as i64;
	// 8238F64C: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8238F650: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8238F654: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8238F658: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8238F65C: 4098001C  bge cr6, 0x8238f678
	if !ctx.cr[6].lt {
	pc = 0x8238F678; continue 'dispatch;
	}
	// 8238F660: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8238F664: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8238F668: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8238F66C: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8238F670: EC2C037C  fnmsubs f1, f12, f13, f0
	ctx.f[1].f64 = -(((ctx.f[12].f64 * ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8238F674: 48000018  b 0x8238f68c
	pc = 0x8238F68C; continue 'dispatch;
	// 8238F678: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8238F67C: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8238F680: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8238F684: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8238F688: EC2C037A  fmadds f1, f12, f13, f0
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 8238F68C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8238F690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8238F694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8238F698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8238F6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8238F6A0 size=152
    let mut pc: u32 = 0x8238F6A0;
    'dispatch: loop {
        match pc {
            0x8238F6A0 => {
    //   block [0x8238F6A0..0x8238F738)
	// 8238F6A0: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8238F6A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8238F6A8: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238F6AC: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238F6B0: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 8238F6B4: 409A0014  bne cr6, 0x8238f6c8
	if !ctx.cr[6].eq {
	pc = 0x8238F6C8; continue 'dispatch;
	}
	// 8238F6B8: 896A0001  lbz r11, 1(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 8238F6BC: 2B0B0078  cmplwi cr6, r11, 0x78
	ctx.cr[6].compare_u32(ctx.r[11].u32, 120 as u32, &mut ctx.xer);
	// 8238F6C0: 409A0008  bne cr6, 0x8238f6c8
	if !ctx.cr[6].eq {
	pc = 0x8238F6C8; continue 'dispatch;
	}
	// 8238F6C4: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8238F6C8: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238F6CC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8238F6D0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8238F6D4: 41820010  beq 0x8238f6e4
	if ctx.cr[0].eq {
	pc = 0x8238F6E4; continue 'dispatch;
	}
	// 8238F6D8: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 8238F6DC: 2B090020  cmplwi cr6, r9, 0x20
	ctx.cr[6].compare_u32(ctx.r[9].u32, 32 as u32, &mut ctx.xer);
	// 8238F6E0: 4198FFE8  blt cr6, 0x8238f6c8
	if ctx.cr[6].lt {
	pc = 0x8238F6C8; continue 'dispatch;
	}
	// 8238F6E4: 2B0B0061  cmplwi cr6, r11, 0x61
	ctx.cr[6].compare_u32(ctx.r[11].u32, 97 as u32, &mut ctx.xer);
	// 8238F6E8: 41980008  blt cr6, 0x8238f6f0
	if ctx.cr[6].lt {
	pc = 0x8238F6F0; continue 'dispatch;
	}
	// 8238F6EC: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 8238F6F0: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 8238F6F4: 4198000C  blt cr6, 0x8238f700
	if ctx.cr[6].lt {
	pc = 0x8238F700; continue 'dispatch;
	}
	// 8238F6F8: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 8238F6FC: 40990010  ble cr6, 0x8238f70c
	if !ctx.cr[6].gt {
	pc = 0x8238F70C; continue 'dispatch;
	}
	// 8238F700: 392BFFBF  addi r9, r11, -0x41
	ctx.r[9].s64 = ctx.r[11].s64 + -65;
	// 8238F704: 2B090005  cmplwi cr6, r9, 5
	ctx.cr[6].compare_u32(ctx.r[9].u32, 5 as u32, &mut ctx.xer);
	// 8238F708: 41990030  bgt cr6, 0x8238f738
	if ctx.cr[6].gt {
		sub_8238F738(ctx, base);
		return;
	}
	// 8238F70C: 39200041  li r9, 0x41
	ctx.r[9].s64 = 65;
	// 8238F710: 54672036  slwi r7, r3, 4
	ctx.r[7].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8238F714: 7D295810  subfc r9, r9, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[9].u32;
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8238F718: 7D294910  subfe r9, r9, r9
	let x = (!ctx.r[9].u32);
	let y = ctx.r[9].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[9].u32 = res;
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8238F71C: 552907F8  rlwinm r9, r9, 0, 0x1f, 0x1c
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 8238F720: 39290037  addi r9, r9, 0x37
	ctx.r[9].s64 = ctx.r[9].s64 + 55;
	// 8238F724: 7D295850  subf r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8238F728: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8238F72C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8238F730: 7D233B78  or r3, r9, r7
	ctx.r[3].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 8238F734: 4BFFFFB0  b 0x8238f6e4
	pc = 0x8238F6E4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


