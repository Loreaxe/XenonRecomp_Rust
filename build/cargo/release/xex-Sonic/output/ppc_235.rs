pub fn sub_830004F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830004F8 size=312
    let mut pc: u32 = 0x830004F8;
    'dispatch: loop {
        match pc {
            0x830004F8 => {
    //   block [0x830004F8..0x83000630)
	// 830004F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830004FC: 481A7C71  bl 0x831a816c
	ctx.lr = 0x83000500;
	sub_831A8130(ctx, base);
	// 83000500: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000504: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83000508: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300050C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83000510: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000514: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000518: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300051C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000520: 4E800421  bctrl
	ctx.lr = 0x83000524;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000524: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83000528: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8300052C: 419A0080  beq cr6, 0x830005ac
	if ctx.cr[6].eq {
	pc = 0x830005AC; continue 'dispatch;
	}
	// 83000530: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83000534: 419A0078  beq cr6, 0x830005ac
	if ctx.cr[6].eq {
	pc = 0x830005AC; continue 'dispatch;
	}
	// 83000538: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 8300053C: 419A0070  beq cr6, 0x830005ac
	if ctx.cr[6].eq {
	pc = 0x830005AC; continue 'dispatch;
	}
	// 83000540: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83000544: 419A0068  beq cr6, 0x830005ac
	if ctx.cr[6].eq {
	pc = 0x830005AC; continue 'dispatch;
	}
	// 83000548: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300054C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000550: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83000554: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000558: 4E800421  bctrl
	ctx.lr = 0x8300055C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300055C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83000560: 48000018  b 0x83000578
	pc = 0x83000578; continue 'dispatch;
	// 83000564: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000568: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300056C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000570: 4E800421  bctrl
	ctx.lr = 0x83000574;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000574: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83000578: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300057C: 4082FFE8  bne 0x83000564
	if !ctx.cr[0].eq {
	pc = 0x83000564; continue 'dispatch;
	}
	// 83000580: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83000584: 409800A4  bge cr6, 0x83000628
	if !ctx.cr[6].lt {
	pc = 0x83000628; continue 'dispatch;
	}
	// 83000588: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300058C: 80DD0024  lwz r6, 0x24(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 83000590: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83000594: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83000598: 4BFF9939  bl 0x82ff9ed0
	ctx.lr = 0x8300059C;
	sub_82FF9ED0(ctx, base);
	// 8300059C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830005A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830005A4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830005A8: 481B0681  bl 0x831b0c28
	ctx.lr = 0x830005AC;
	sub_831B0C28(ctx, base);
	// 830005AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830005B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830005B4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830005B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830005BC: 4E800421  bctrl
	ctx.lr = 0x830005C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830005C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830005C4: 41820034  beq 0x830005f8
	if ctx.cr[0].eq {
	pc = 0x830005F8; continue 'dispatch;
	}
	// 830005C8: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830005CC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830005D0: 41820028  beq 0x830005f8
	if ctx.cr[0].eq {
	pc = 0x830005F8; continue 'dispatch;
	}
	// 830005D4: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 830005D8: 48000008  b 0x830005e0
	pc = 0x830005E0; continue 'dispatch;
	// 830005DC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830005E0: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830005E4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830005E8: 4082FFF4  bne 0x830005dc
	if !ctx.cr[0].eq {
	pc = 0x830005DC; continue 'dispatch;
	}
	// 830005EC: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 830005F0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830005F4: 48000008  b 0x830005fc
	pc = 0x830005FC; continue 'dispatch;
	// 830005F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830005FC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000600: 40990028  ble cr6, 0x83000628
	if !ctx.cr[6].gt {
	pc = 0x83000628; continue 'dispatch;
	}
	// 83000604: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83000608: 80DD0024  lwz r6, 0x24(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300060C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83000610: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83000614: 4BFF98BD  bl 0x82ff9ed0
	ctx.lr = 0x83000618;
	sub_82FF9ED0(ctx, base);
	// 83000618: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300061C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83000620: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83000624: 481B0605  bl 0x831b0c28
	ctx.lr = 0x83000628;
	sub_831B0C28(ctx, base);
	// 83000628: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8300062C: 481A7B90  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000630 size=188
    let mut pc: u32 = 0x83000630;
    'dispatch: loop {
        match pc {
            0x83000630 => {
    //   block [0x83000630..0x830006EC)
	// 83000630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83000638: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300063C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83000640: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000644: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83000648: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300064C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83000650: 419A0080  beq cr6, 0x830006d0
	if ctx.cr[6].eq {
	pc = 0x830006D0; continue 'dispatch;
	}
	// 83000654: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83000658: 41820040  beq 0x83000698
	if ctx.cr[0].eq {
	pc = 0x83000698; continue 'dispatch;
	}
	// 8300065C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000660: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000664: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83000668: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300066C: 4E800421  bctrl
	ctx.lr = 0x83000670;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000670: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83000674: 40820060  bne 0x830006d4
	if !ctx.cr[0].eq {
	pc = 0x830006D4; continue 'dispatch;
	}
	// 83000678: 48000020  b 0x83000698
	pc = 0x83000698; continue 'dispatch;
	// 8300067C: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83000680: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83000684: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 83000688: 40820008  bne 0x83000690
	if !ctx.cr[0].eq {
	pc = 0x83000690; continue 'dispatch;
	}
	// 8300068C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83000690: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000694: 419A003C  beq cr6, 0x830006d0
	if ctx.cr[6].eq {
	pc = 0x830006D0; continue 'dispatch;
	}
	// 83000698: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300069C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830006A0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 830006A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830006A8: 4E800421  bctrl
	ctx.lr = 0x830006AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830006AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830006B0: 40820024  bne 0x830006d4
	if !ctx.cr[0].eq {
	pc = 0x830006D4; continue 'dispatch;
	}
	// 830006B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830006B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830006BC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830006C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830006C4: 4E800421  bctrl
	ctx.lr = 0x830006C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830006C8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830006CC: 4082FFB0  bne 0x8300067c
	if !ctx.cr[0].eq {
	pc = 0x8300067C; continue 'dispatch;
	}
	// 830006D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830006D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830006D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830006DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830006E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830006E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830006E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830006F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830006F0 size=144
    let mut pc: u32 = 0x830006F0;
    'dispatch: loop {
        match pc {
            0x830006F0 => {
    //   block [0x830006F0..0x83000780)
	// 830006F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830006F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830006F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830006FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000700: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83000704: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 83000708: 419A0060  beq cr6, 0x83000768
	if ctx.cr[6].eq {
	pc = 0x83000768; continue 'dispatch;
	}
	// 8300070C: 2F050002  cmpwi cr6, r5, 2
	ctx.cr[6].compare_i32(ctx.r[5].s32, 2, &mut ctx.xer);
	// 83000710: 419A003C  beq cr6, 0x8300074c
	if ctx.cr[6].eq {
	pc = 0x8300074C; continue 'dispatch;
	}
	// 83000714: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 83000718: 409A002C  bne cr6, 0x83000744
	if !ctx.cr[6].eq {
	pc = 0x83000744; continue 'dispatch;
	}
	// 8300071C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000724: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000728: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300072C: 4E800421  bctrl
	ctx.lr = 0x83000730;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000730: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000734: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83000738: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8300073C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000740: 4E800421  bctrl
	ctx.lr = 0x83000744;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000744: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83000748: 48000024  b 0x8300076c
	pc = 0x8300076C; continue 'dispatch;
	// 8300074C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000750: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83000754: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000758: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8300075C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000760: 4E800421  bctrl
	ctx.lr = 0x83000764;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000764: 48000008  b 0x8300076c
	pc = 0x8300076C; continue 'dispatch;
	// 83000768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300076C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83000770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83000774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83000778: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300077C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000780 size=192
    let mut pc: u32 = 0x83000780;
    'dispatch: loop {
        match pc {
            0x83000780 => {
    //   block [0x83000780..0x83000840)
	// 83000780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83000788: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300078C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83000790: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000794: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83000798: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8300079C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830007A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830007A4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830007A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830007AC: 4E800421  bctrl
	ctx.lr = 0x830007B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830007B0: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 830007B4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830007B8: 419A006C  beq cr6, 0x83000824
	if ctx.cr[6].eq {
	pc = 0x83000824; continue 'dispatch;
	}
	// 830007BC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830007C0: 419A0064  beq cr6, 0x83000824
	if ctx.cr[6].eq {
	pc = 0x83000824; continue 'dispatch;
	}
	// 830007C4: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 830007C8: 419A005C  beq cr6, 0x83000824
	if ctx.cr[6].eq {
	pc = 0x83000824; continue 'dispatch;
	}
	// 830007CC: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 830007D0: 419A0054  beq cr6, 0x83000824
	if ctx.cr[6].eq {
	pc = 0x83000824; continue 'dispatch;
	}
	// 830007D4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830007D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830007DC: 4198004C  blt cr6, 0x83000828
	if ctx.cr[6].lt {
	pc = 0x83000828; continue 'dispatch;
	}
	// 830007E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830007E4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830007E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830007EC: 4E800421  bctrl
	ctx.lr = 0x830007F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830007F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830007F4: 41820030  beq 0x83000824
	if ctx.cr[0].eq {
	pc = 0x83000824; continue 'dispatch;
	}
	// 830007F8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830007FC: 40990020  ble cr6, 0x8300081c
	if !ctx.cr[6].gt {
	pc = 0x8300081C; continue 'dispatch;
	}
	// 83000800: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000804: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 83000808: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300080C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000810: 4E800421  bctrl
	ctx.lr = 0x83000814;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000814: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83000818: 4082FFE0  bne 0x830007f8
	if !ctx.cr[0].eq {
	pc = 0x830007F8; continue 'dispatch;
	}
	// 8300081C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83000820: 409A0008  bne cr6, 0x83000828
	if !ctx.cr[6].eq {
	pc = 0x83000828; continue 'dispatch;
	}
	// 83000824: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000828: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300082C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83000830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83000834: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83000838: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300083C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000840 size=284
    let mut pc: u32 = 0x83000840;
    'dispatch: loop {
        match pc {
            0x83000840 => {
    //   block [0x83000840..0x8300095C)
	// 83000840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000844: 481A7925  bl 0x831a8168
	ctx.lr = 0x83000848;
	sub_831A8130(ctx, base);
	// 83000848: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300084C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83000850: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83000854: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83000858: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8300085C: 419A00B0  beq cr6, 0x8300090c
	if ctx.cr[6].eq {
	pc = 0x8300090C; continue 'dispatch;
	}
	// 83000860: 3F808214  lis r28, -0x7dec
	ctx.r[28].s64 = -2112618496;
	// 83000864: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83000868: 419A00A4  beq cr6, 0x8300090c
	if ctx.cr[6].eq {
	pc = 0x8300090C; continue 'dispatch;
	}
	// 8300086C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000874: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000878: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300087C: 4E800421  bctrl
	ctx.lr = 0x83000880;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000880: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83000884: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 83000888: 419A008C  beq cr6, 0x83000914
	if ctx.cr[6].eq {
	pc = 0x83000914; continue 'dispatch;
	}
	// 8300088C: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83000890: A17CA698  lhz r11, -0x5968(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83000894: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83000898: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300089C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830008A0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830008A4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830008A8: 40820090  bne 0x83000938
	if !ctx.cr[0].eq {
	pc = 0x83000938; continue 'dispatch;
	}
	// 830008AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830008B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830008B4: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 830008B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830008BC: 4E800421  bctrl
	ctx.lr = 0x830008C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830008C0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830008C4: 4182002C  beq 0x830008f0
	if ctx.cr[0].eq {
	pc = 0x830008F0; continue 'dispatch;
	}
	// 830008C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830008CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830008D0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830008D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830008D8: 4E800421  bctrl
	ctx.lr = 0x830008DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830008DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830008E0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830008E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830008E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830008EC: 4BFFFF55  bl 0x83000840
	ctx.lr = 0x830008F0;
	sub_83000840(ctx, base);
	// 830008F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830008F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830008F8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 830008FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000900: 4E800421  bctrl
	ctx.lr = 0x83000904;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000904: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83000908: 4082FF5C  bne 0x83000864
	if !ctx.cr[0].eq {
	pc = 0x83000864; continue 'dispatch;
	}
	// 8300090C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83000910: 481A78A8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83000914: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83000918: 80DE0024  lwz r6, 0x24(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300091C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83000920: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83000924: 4BFF95AD  bl 0x82ff9ed0
	ctx.lr = 0x83000928;
	sub_82FF9ED0(ctx, base);
	// 83000928: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300092C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83000930: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83000934: 481B02F5  bl 0x831b0c28
	ctx.lr = 0x83000938;
	sub_831B0C28(ctx, base);
	// 83000938: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300093C: 80DE0024  lwz r6, 0x24(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83000940: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83000944: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83000948: 4BFF9589  bl 0x82ff9ed0
	ctx.lr = 0x8300094C;
	sub_82FF9ED0(ctx, base);
	// 8300094C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83000950: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83000954: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83000958: 481B02D1  bl 0x831b0c28
	ctx.lr = 0x8300095C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000960 size=184
    let mut pc: u32 = 0x83000960;
    'dispatch: loop {
        match pc {
            0x83000960 => {
    //   block [0x83000960..0x83000A18)
	// 83000960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000964: 481A7809  bl 0x831a816c
	ctx.lr = 0x83000968;
	sub_831A8130(ctx, base);
	// 83000968: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300096C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83000970: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83000974: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83000978: 419A0098  beq cr6, 0x83000a10
	if ctx.cr[6].eq {
	pc = 0x83000A10; continue 'dispatch;
	}
	// 8300097C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83000980: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000984: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000988: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300098C: 4E800421  bctrl
	ctx.lr = 0x83000990;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000990: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83000994: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83000998: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300099C: 409A002C  bne cr6, 0x830009c8
	if !ctx.cr[6].eq {
	pc = 0x830009C8; continue 'dispatch;
	}
	// 830009A0: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 830009A4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830009A8: 419A001C  beq cr6, 0x830009c4
	if ctx.cr[6].eq {
	pc = 0x830009C4; continue 'dispatch;
	}
	// 830009AC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830009B0: 419A0014  beq cr6, 0x830009c4
	if ctx.cr[6].eq {
	pc = 0x830009C4; continue 'dispatch;
	}
	// 830009B4: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 830009B8: 419A000C  beq cr6, 0x830009c4
	if ctx.cr[6].eq {
	pc = 0x830009C4; continue 'dispatch;
	}
	// 830009BC: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 830009C0: 409A0008  bne cr6, 0x830009c8
	if !ctx.cr[6].eq {
	pc = 0x830009C8; continue 'dispatch;
	}
	// 830009C4: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830009C8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830009CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830009D0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830009D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830009D8: 4E800421  bctrl
	ctx.lr = 0x830009DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830009DC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830009E0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830009E4: 409A002C  bne cr6, 0x83000a10
	if !ctx.cr[6].eq {
	pc = 0x83000A10; continue 'dispatch;
	}
	// 830009E8: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 830009EC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830009F0: 419A001C  beq cr6, 0x83000a0c
	if ctx.cr[6].eq {
	pc = 0x83000A0C; continue 'dispatch;
	}
	// 830009F4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830009F8: 419A0014  beq cr6, 0x83000a0c
	if ctx.cr[6].eq {
	pc = 0x83000A0C; continue 'dispatch;
	}
	// 830009FC: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 83000A00: 419A000C  beq cr6, 0x83000a0c
	if ctx.cr[6].eq {
	pc = 0x83000A0C; continue 'dispatch;
	}
	// 83000A04: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83000A08: 409A0008  bne cr6, 0x83000a10
	if !ctx.cr[6].eq {
	pc = 0x83000A10; continue 'dispatch;
	}
	// 83000A0C: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 83000A10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83000A14: 481A77A8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000A18 size=260
    let mut pc: u32 = 0x83000A18;
    'dispatch: loop {
        match pc {
            0x83000A18 => {
    //   block [0x83000A18..0x83000B1C)
	// 83000A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000A1C: 481A774D  bl 0x831a8168
	ctx.lr = 0x83000A20;
	sub_831A8130(ctx, base);
	// 83000A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000A24: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83000A28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83000A2C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83000A30: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83000A34: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83000A38: 419A00DC  beq cr6, 0x83000b14
	if ctx.cr[6].eq {
	pc = 0x83000B14; continue 'dispatch;
	}
	// 83000A3C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83000A40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000A44: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000A48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000A4C: 4E800421  bctrl
	ctx.lr = 0x83000A50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000A50: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83000A54: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000A58: 409A0050  bne cr6, 0x83000aa8
	if !ctx.cr[6].eq {
	pc = 0x83000AA8; continue 'dispatch;
	}
	// 83000A5C: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83000A60: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83000A64: 419A001C  beq cr6, 0x83000a80
	if ctx.cr[6].eq {
	pc = 0x83000A80; continue 'dispatch;
	}
	// 83000A68: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83000A6C: 419A0014  beq cr6, 0x83000a80
	if ctx.cr[6].eq {
	pc = 0x83000A80; continue 'dispatch;
	}
	// 83000A70: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 83000A74: 419A000C  beq cr6, 0x83000a80
	if ctx.cr[6].eq {
	pc = 0x83000A80; continue 'dispatch;
	}
	// 83000A78: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83000A7C: 409A002C  bne cr6, 0x83000aa8
	if !ctx.cr[6].eq {
	pc = 0x83000AA8; continue 'dispatch;
	}
	// 83000A80: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83000A84: 7D5DE214  add r10, r29, r28
	ctx.r[10].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 83000A88: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83000A8C: 40990010  ble cr6, 0x83000a9c
	if !ctx.cr[6].gt {
	pc = 0x83000A9C; continue 'dispatch;
	}
	// 83000A90: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 83000A94: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83000A98: 48000010  b 0x83000aa8
	pc = 0x83000AA8; continue 'dispatch;
	// 83000A9C: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83000AA0: 40990008  ble cr6, 0x83000aa8
	if !ctx.cr[6].gt {
	pc = 0x83000AA8; continue 'dispatch;
	}
	// 83000AA4: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83000AA8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000AAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000AB0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000AB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000AB8: 4E800421  bctrl
	ctx.lr = 0x83000ABC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000ABC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000AC0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000AC4: 409A0050  bne cr6, 0x83000b14
	if !ctx.cr[6].eq {
	pc = 0x83000B14; continue 'dispatch;
	}
	// 83000AC8: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83000ACC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83000AD0: 419A001C  beq cr6, 0x83000aec
	if ctx.cr[6].eq {
	pc = 0x83000AEC; continue 'dispatch;
	}
	// 83000AD4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83000AD8: 419A0014  beq cr6, 0x83000aec
	if ctx.cr[6].eq {
	pc = 0x83000AEC; continue 'dispatch;
	}
	// 83000ADC: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 83000AE0: 419A000C  beq cr6, 0x83000aec
	if ctx.cr[6].eq {
	pc = 0x83000AEC; continue 'dispatch;
	}
	// 83000AE4: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83000AE8: 409A002C  bne cr6, 0x83000b14
	if !ctx.cr[6].eq {
	pc = 0x83000B14; continue 'dispatch;
	}
	// 83000AEC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000AF0: 7D5DE214  add r10, r29, r28
	ctx.r[10].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 83000AF4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83000AF8: 40990010  ble cr6, 0x83000b08
	if !ctx.cr[6].gt {
	pc = 0x83000B08; continue 'dispatch;
	}
	// 83000AFC: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 83000B00: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83000B04: 48000010  b 0x83000b14
	pc = 0x83000B14; continue 'dispatch;
	// 83000B08: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83000B0C: 40990008  ble cr6, 0x83000b14
	if !ctx.cr[6].gt {
	pc = 0x83000B14; continue 'dispatch;
	}
	// 83000B10: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 83000B14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83000B18: 481A76A0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000B20 size=216
    let mut pc: u32 = 0x83000B20;
    'dispatch: loop {
        match pc {
            0x83000B20 => {
    //   block [0x83000B20..0x83000BF8)
	// 83000B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000B24: 481A7645  bl 0x831a8168
	ctx.lr = 0x83000B28;
	sub_831A8130(ctx, base);
	// 83000B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000B2C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83000B30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83000B34: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83000B38: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83000B3C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83000B40: 419A00B0  beq cr6, 0x83000bf0
	if ctx.cr[6].eq {
	pc = 0x83000BF0; continue 'dispatch;
	}
	// 83000B44: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83000B48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000B4C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000B50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000B54: 4E800421  bctrl
	ctx.lr = 0x83000B58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000B58: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83000B5C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000B60: 409A0038  bne cr6, 0x83000b98
	if !ctx.cr[6].eq {
	pc = 0x83000B98; continue 'dispatch;
	}
	// 83000B64: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83000B68: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83000B6C: 419A001C  beq cr6, 0x83000b88
	if ctx.cr[6].eq {
	pc = 0x83000B88; continue 'dispatch;
	}
	// 83000B70: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83000B74: 419A0014  beq cr6, 0x83000b88
	if ctx.cr[6].eq {
	pc = 0x83000B88; continue 'dispatch;
	}
	// 83000B78: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 83000B7C: 419A000C  beq cr6, 0x83000b88
	if ctx.cr[6].eq {
	pc = 0x83000B88; continue 'dispatch;
	}
	// 83000B80: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83000B84: 409A0014  bne cr6, 0x83000b98
	if !ctx.cr[6].eq {
	pc = 0x83000B98; continue 'dispatch;
	}
	// 83000B88: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83000B8C: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83000B90: 40990008  ble cr6, 0x83000b98
	if !ctx.cr[6].gt {
	pc = 0x83000B98; continue 'dispatch;
	}
	// 83000B94: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83000B98: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000B9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000BA0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000BA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000BA8: 4E800421  bctrl
	ctx.lr = 0x83000BAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000BAC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000BB0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000BB4: 409A003C  bne cr6, 0x83000bf0
	if !ctx.cr[6].eq {
	pc = 0x83000BF0; continue 'dispatch;
	}
	// 83000BB8: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83000BBC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83000BC0: 419A001C  beq cr6, 0x83000bdc
	if ctx.cr[6].eq {
	pc = 0x83000BDC; continue 'dispatch;
	}
	// 83000BC4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83000BC8: 419A0014  beq cr6, 0x83000bdc
	if ctx.cr[6].eq {
	pc = 0x83000BDC; continue 'dispatch;
	}
	// 83000BCC: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 83000BD0: 419A000C  beq cr6, 0x83000bdc
	if ctx.cr[6].eq {
	pc = 0x83000BDC; continue 'dispatch;
	}
	// 83000BD4: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83000BD8: 409A0018  bne cr6, 0x83000bf0
	if !ctx.cr[6].eq {
	pc = 0x83000BF0; continue 'dispatch;
	}
	// 83000BDC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000BE0: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83000BE4: 4099000C  ble cr6, 0x83000bf0
	if !ctx.cr[6].gt {
	pc = 0x83000BF0; continue 'dispatch;
	}
	// 83000BE8: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 83000BEC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83000BF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83000BF4: 481A75C4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000BF8 size=400
    let mut pc: u32 = 0x83000BF8;
    'dispatch: loop {
        match pc {
            0x83000BF8 => {
    //   block [0x83000BF8..0x83000D88)
	// 83000BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000BFC: 481A7571  bl 0x831a816c
	ctx.lr = 0x83000C00;
	sub_831A8130(ctx, base);
	// 83000C00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000C04: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83000C08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83000C0C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83000C10: 419A0170  beq cr6, 0x83000d80
	if ctx.cr[6].eq {
	pc = 0x83000D80; continue 'dispatch;
	}
	// 83000C14: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83000C18: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83000C1C: 419A0164  beq cr6, 0x83000d80
	if ctx.cr[6].eq {
	pc = 0x83000D80; continue 'dispatch;
	}
	// 83000C20: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000C24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83000C28: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83000C2C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000C30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000C34: 4E800421  bctrl
	ctx.lr = 0x83000C38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000C38: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83000C3C: 409A0028  bne cr6, 0x83000c64
	if !ctx.cr[6].eq {
	pc = 0x83000C64; continue 'dispatch;
	}
	// 83000C40: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83000C44: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83000C48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000C4C: 4BFFF7AD  bl 0x830003f8
	ctx.lr = 0x83000C50;
	sub_830003F8(ctx, base);
	// 83000C50: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83000C54: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83000C58: 4099000C  ble cr6, 0x83000c64
	if !ctx.cr[6].gt {
	pc = 0x83000C64; continue 'dispatch;
	}
	// 83000C5C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83000C60: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83000C64: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000C68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83000C6C: 83BF000C  lwz r29, 0xc(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000C70: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000C74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000C78: 4E800421  bctrl
	ctx.lr = 0x83000C7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000C7C: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83000C80: 409A0028  bne cr6, 0x83000ca8
	if !ctx.cr[6].eq {
	pc = 0x83000CA8; continue 'dispatch;
	}
	// 83000C84: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83000C88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83000C8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000C90: 4BFFF769  bl 0x830003f8
	ctx.lr = 0x83000C94;
	sub_830003F8(ctx, base);
	// 83000C94: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000C98: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83000C9C: 4099000C  ble cr6, 0x83000ca8
	if !ctx.cr[6].gt {
	pc = 0x83000CA8; continue 'dispatch;
	}
	// 83000CA0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83000CA4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83000CA8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000CAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83000CB0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000CB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000CB8: 4E800421  bctrl
	ctx.lr = 0x83000CBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000CBC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83000CC0: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000CC4: 409A0024  bne cr6, 0x83000ce8
	if !ctx.cr[6].eq {
	pc = 0x83000CE8; continue 'dispatch;
	}
	// 83000CC8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000CCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83000CD0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000CD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000CD8: 4E800421  bctrl
	ctx.lr = 0x83000CDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000CDC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000CE0: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000CE4: 419A009C  beq cr6, 0x83000d80
	if ctx.cr[6].eq {
	pc = 0x83000D80; continue 'dispatch;
	}
	// 83000CE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83000CEC: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83000CF0: 4BFFF591  bl 0x83000280
	ctx.lr = 0x83000CF4;
	sub_83000280(ctx, base);
	// 83000CF4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83000CF8: 4182003C  beq 0x83000d34
	if ctx.cr[0].eq {
	pc = 0x83000D34; continue 'dispatch;
	}
	// 83000CFC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000D00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83000D04: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000D08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000D0C: 4E800421  bctrl
	ctx.lr = 0x83000D10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000D10: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83000D14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000D18: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83000D1C: 4BFFF265  bl 0x82ffff80
	ctx.lr = 0x83000D20;
	sub_82FFFF80(ctx, base);
	// 83000D20: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83000D24: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83000D28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000D2C: 4BFFF6CD  bl 0x830003f8
	ctx.lr = 0x83000D30;
	sub_830003F8(ctx, base);
	// 83000D30: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83000D34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83000D38: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000D3C: 4BFFF545  bl 0x83000280
	ctx.lr = 0x83000D40;
	sub_83000280(ctx, base);
	// 83000D40: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83000D44: 4182003C  beq 0x83000d80
	if ctx.cr[0].eq {
	pc = 0x83000D80; continue 'dispatch;
	}
	// 83000D48: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000D4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83000D50: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000D54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000D58: 4E800421  bctrl
	ctx.lr = 0x83000D5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000D5C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83000D60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000D64: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83000D68: 4BFFF269  bl 0x82ffffd0
	ctx.lr = 0x83000D6C;
	sub_82FFFFD0(ctx, base);
	// 83000D6C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83000D70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83000D74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000D78: 4BFFF681  bl 0x830003f8
	ctx.lr = 0x83000D7C;
	sub_830003F8(ctx, base);
	// 83000D7C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83000D80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83000D84: 481A7438  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000D88 size=172
    let mut pc: u32 = 0x83000D88;
    'dispatch: loop {
        match pc {
            0x83000D88 => {
    //   block [0x83000D88..0x83000E34)
	// 83000D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000D8C: 481A73E1  bl 0x831a816c
	ctx.lr = 0x83000D90;
	sub_831A8130(ctx, base);
	// 83000D90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000D94: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83000D98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83000D9C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83000DA0: 419A008C  beq cr6, 0x83000e2c
	if ctx.cr[6].eq {
	pc = 0x83000E2C; continue 'dispatch;
	}
	// 83000DA4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000DA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83000DAC: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83000DB0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000DB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000DB8: 4E800421  bctrl
	ctx.lr = 0x83000DBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000DBC: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83000DC0: 409A0028  bne cr6, 0x83000de8
	if !ctx.cr[6].eq {
	pc = 0x83000DE8; continue 'dispatch;
	}
	// 83000DC4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83000DC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83000DCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000DD0: 4BFFF629  bl 0x830003f8
	ctx.lr = 0x83000DD4;
	sub_830003F8(ctx, base);
	// 83000DD4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83000DD8: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000DDC: 4098000C  bge cr6, 0x83000de8
	if !ctx.cr[6].lt {
	pc = 0x83000DE8; continue 'dispatch;
	}
	// 83000DE0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83000DE4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83000DE8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000DEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83000DF0: 83BF000C  lwz r29, 0xc(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000DF4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000DF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000DFC: 4E800421  bctrl
	ctx.lr = 0x83000E00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000E00: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83000E04: 409A0028  bne cr6, 0x83000e2c
	if !ctx.cr[6].eq {
	pc = 0x83000E2C; continue 'dispatch;
	}
	// 83000E08: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83000E0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83000E10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000E14: 4BFFF5E5  bl 0x830003f8
	ctx.lr = 0x83000E18;
	sub_830003F8(ctx, base);
	// 83000E18: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000E1C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000E20: 4098000C  bge cr6, 0x83000e2c
	if !ctx.cr[6].lt {
	pc = 0x83000E2C; continue 'dispatch;
	}
	// 83000E24: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83000E28: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83000E2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83000E30: 481A738C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000E38 size=228
    let mut pc: u32 = 0x83000E38;
    'dispatch: loop {
        match pc {
            0x83000E38 => {
    //   block [0x83000E38..0x83000F1C)
	// 83000E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000E3C: 481A732D  bl 0x831a8168
	ctx.lr = 0x83000E40;
	sub_831A8130(ctx, base);
	// 83000E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000E44: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83000E48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83000E4C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83000E50: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83000E54: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83000E58: 419A00BC  beq cr6, 0x83000f14
	if ctx.cr[6].eq {
	pc = 0x83000F14; continue 'dispatch;
	}
	// 83000E5C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83000E60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000E64: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000E68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000E6C: 4E800421  bctrl
	ctx.lr = 0x83000E70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000E70: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83000E74: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000E78: 409A0040  bne cr6, 0x83000eb8
	if !ctx.cr[6].eq {
	pc = 0x83000EB8; continue 'dispatch;
	}
	// 83000E7C: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83000E80: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83000E84: 419A001C  beq cr6, 0x83000ea0
	if ctx.cr[6].eq {
	pc = 0x83000EA0; continue 'dispatch;
	}
	// 83000E88: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83000E8C: 419A0014  beq cr6, 0x83000ea0
	if ctx.cr[6].eq {
	pc = 0x83000EA0; continue 'dispatch;
	}
	// 83000E90: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 83000E94: 419A000C  beq cr6, 0x83000ea0
	if ctx.cr[6].eq {
	pc = 0x83000EA0; continue 'dispatch;
	}
	// 83000E98: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83000E9C: 409A001C  bne cr6, 0x83000eb8
	if !ctx.cr[6].eq {
	pc = 0x83000EB8; continue 'dispatch;
	}
	// 83000EA0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83000EA4: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83000EA8: 40990010  ble cr6, 0x83000eb8
	if !ctx.cr[6].gt {
	pc = 0x83000EB8; continue 'dispatch;
	}
	// 83000EAC: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 83000EB0: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 83000EB4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83000EB8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000EBC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000EC0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000EC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000EC8: 4E800421  bctrl
	ctx.lr = 0x83000ECC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000ECC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000ED0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83000ED4: 409A0040  bne cr6, 0x83000f14
	if !ctx.cr[6].eq {
	pc = 0x83000F14; continue 'dispatch;
	}
	// 83000ED8: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83000EDC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83000EE0: 419A001C  beq cr6, 0x83000efc
	if ctx.cr[6].eq {
	pc = 0x83000EFC; continue 'dispatch;
	}
	// 83000EE4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83000EE8: 419A0014  beq cr6, 0x83000efc
	if ctx.cr[6].eq {
	pc = 0x83000EFC; continue 'dispatch;
	}
	// 83000EEC: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 83000EF0: 419A000C  beq cr6, 0x83000efc
	if ctx.cr[6].eq {
	pc = 0x83000EFC; continue 'dispatch;
	}
	// 83000EF4: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83000EF8: 409A001C  bne cr6, 0x83000f14
	if !ctx.cr[6].eq {
	pc = 0x83000F14; continue 'dispatch;
	}
	// 83000EFC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000F00: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83000F04: 40990010  ble cr6, 0x83000f14
	if !ctx.cr[6].gt {
	pc = 0x83000F14; continue 'dispatch;
	}
	// 83000F08: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 83000F0C: 939F000C  stw r28, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 83000F10: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83000F14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83000F18: 481A72A0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000F20 size=364
    let mut pc: u32 = 0x83000F20;
    'dispatch: loop {
        match pc {
            0x83000F20 => {
    //   block [0x83000F20..0x8300108C)
	// 83000F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000F24: 481A7249  bl 0x831a816c
	ctx.lr = 0x83000F28;
	sub_831A8130(ctx, base);
	// 83000F28: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000F2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83000F30: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83000F34: 4BFFF535  bl 0x83000468
	ctx.lr = 0x83000F38;
	sub_83000468(ctx, base);
	// 83000F38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83000F3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83000F40: 4BFFF449  bl 0x83000388
	ctx.lr = 0x83000F44;
	sub_83000388(ctx, base);
	// 83000F44: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83000F48: 40820028  bne 0x83000f70
	if !ctx.cr[0].eq {
	pc = 0x83000F70; continue 'dispatch;
	}
	// 83000F4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83000F50: 80DE0024  lwz r6, 0x24(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83000F54: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83000F58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83000F5C: 48037E2D  bl 0x83038d88
	ctx.lr = 0x83000F60;
	sub_83038D88(ctx, base);
	// 83000F60: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83000F64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83000F68: 388BC878  addi r4, r11, -0x3788
	ctx.r[4].s64 = ctx.r[11].s64 + -14216;
	// 83000F6C: 481AFCBD  bl 0x831b0c28
	ctx.lr = 0x83000F70;
	sub_831B0C28(ctx, base);
	// 83000F70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000F74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000F78: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000F7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000F80: 4E800421  bctrl
	ctx.lr = 0x83000F84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000F84: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83000F88: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83000F8C: 419A0080  beq cr6, 0x8300100c
	if ctx.cr[6].eq {
	pc = 0x8300100C; continue 'dispatch;
	}
	// 83000F90: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83000F94: 419A0078  beq cr6, 0x8300100c
	if ctx.cr[6].eq {
	pc = 0x8300100C; continue 'dispatch;
	}
	// 83000F98: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 83000F9C: 419A0070  beq cr6, 0x8300100c
	if ctx.cr[6].eq {
	pc = 0x8300100C; continue 'dispatch;
	}
	// 83000FA0: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83000FA4: 419A0068  beq cr6, 0x8300100c
	if ctx.cr[6].eq {
	pc = 0x8300100C; continue 'dispatch;
	}
	// 83000FA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000FAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000FB0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000FB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000FB8: 4E800421  bctrl
	ctx.lr = 0x83000FBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000FBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83000FC0: 418200C4  beq 0x83001084
	if ctx.cr[0].eq {
	pc = 0x83001084; continue 'dispatch;
	}
	// 83000FC4: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83000FC8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83000FCC: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83000FD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000FD4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83000FD8: 48000018  b 0x83000ff0
	pc = 0x83000FF0; continue 'dispatch;
	// 83000FDC: 7F03F840  cmplw cr6, r3, r31
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83000FE0: 419A0020  beq cr6, 0x83001000
	if ctx.cr[6].eq {
	pc = 0x83001000; continue 'dispatch;
	}
	// 83000FE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000FE8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83000FEC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83000FF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000FF4: 4E800421  bctrl
	ctx.lr = 0x83000FF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000FF8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83000FFC: 4082FFE0  bne 0x83000fdc
	if !ctx.cr[0].eq {
	pc = 0x83000FDC; continue 'dispatch;
	}
	// 83001000: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 83001004: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83001008: 4800006C  b 0x83001074
	pc = 0x83001074; continue 'dispatch;
	// 8300100C: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83001010: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 83001014: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83001018: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8300101C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83001020: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83001024: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001028: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 8300102C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001030: 409A004C  bne cr6, 0x8300107c
	if !ctx.cr[6].eq {
	pc = 0x8300107C; continue 'dispatch;
	}
	// 83001034: 4E800421  bctrl
	ctx.lr = 0x83001038;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001038: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300103C: 41820034  beq 0x83001070
	if ctx.cr[0].eq {
	pc = 0x83001070; continue 'dispatch;
	}
	// 83001040: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001044: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001048: 41820028  beq 0x83001070
	if ctx.cr[0].eq {
	pc = 0x83001070; continue 'dispatch;
	}
	// 8300104C: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 83001050: 48000008  b 0x83001058
	pc = 0x83001058; continue 'dispatch;
	// 83001054: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83001058: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300105C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001060: 4082FFF4  bne 0x83001054
	if !ctx.cr[0].eq {
	pc = 0x83001054; continue 'dispatch;
	}
	// 83001064: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 83001068: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8300106C: 48000008  b 0x83001074
	pc = 0x83001074; continue 'dispatch;
	// 83001070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83001074: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83001078: 4800000C  b 0x83001084
	pc = 0x83001084; continue 'dispatch;
	// 8300107C: 4E800421  bctrl
	ctx.lr = 0x83001080;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001080: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83001084: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83001088: 481A7134  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83001090 size=284
    let mut pc: u32 = 0x83001090;
    'dispatch: loop {
        match pc {
            0x83001090 => {
    //   block [0x83001090..0x830011AC)
	// 83001090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83001094: 481A70D9  bl 0x831a816c
	ctx.lr = 0x83001098;
	sub_831A8130(ctx, base);
	// 83001098: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300109C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830010A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830010A4: 4BFFF3C5  bl 0x83000468
	ctx.lr = 0x830010A8;
	sub_83000468(ctx, base);
	// 830010A8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830010AC: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 830010B0: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 830010B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830010B8: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830010BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830010C0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830010C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830010C8: 4E800421  bctrl
	ctx.lr = 0x830010CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830010CC: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 830010D0: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830010D4: 419A00B8  beq cr6, 0x8300118c
	if ctx.cr[6].eq {
	pc = 0x8300118C; continue 'dispatch;
	}
	// 830010D8: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830010DC: 419A00B0  beq cr6, 0x8300118c
	if ctx.cr[6].eq {
	pc = 0x8300118C; continue 'dispatch;
	}
	// 830010E0: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 830010E4: 419A00A8  beq cr6, 0x8300118c
	if ctx.cr[6].eq {
	pc = 0x8300118C; continue 'dispatch;
	}
	// 830010E8: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 830010EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830010F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830010F4: 409A0054  bne cr6, 0x83001148
	if !ctx.cr[6].eq {
	pc = 0x83001148; continue 'dispatch;
	}
	// 830010F8: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 830010FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001100: 4E800421  bctrl
	ctx.lr = 0x83001104;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001104: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001108: 41820034  beq 0x8300113c
	if ctx.cr[0].eq {
	pc = 0x8300113C; continue 'dispatch;
	}
	// 8300110C: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001110: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001114: 41820028  beq 0x8300113c
	if ctx.cr[0].eq {
	pc = 0x8300113C; continue 'dispatch;
	}
	// 83001118: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 8300111C: 48000008  b 0x83001124
	pc = 0x83001124; continue 'dispatch;
	// 83001120: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83001124: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001128: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300112C: 4082FFF4  bne 0x83001120
	if !ctx.cr[0].eq {
	pc = 0x83001120; continue 'dispatch;
	}
	// 83001130: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 83001134: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83001138: 48000008  b 0x83001140
	pc = 0x83001140; continue 'dispatch;
	// 8300113C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83001140: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83001144: 48000060  b 0x830011a4
	pc = 0x830011A4; continue 'dispatch;
	// 83001148: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8300114C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001150: 4E800421  bctrl
	ctx.lr = 0x83001154;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001154: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001158: 4082000C  bne 0x83001164
	if !ctx.cr[0].eq {
	pc = 0x83001164; continue 'dispatch;
	}
	// 8300115C: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 83001160: 48000044  b 0x830011a4
	pc = 0x830011A4; continue 'dispatch;
	// 83001164: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 83001168: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300116C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83001170: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83001174: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001178: 4E800421  bctrl
	ctx.lr = 0x8300117C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300117C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001180: 4082FFE8  bne 0x83001168
	if !ctx.cr[0].eq {
	pc = 0x83001168; continue 'dispatch;
	}
	// 83001184: 93FE0010  stw r31, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 83001188: 4800001C  b 0x830011a4
	pc = 0x830011A4; continue 'dispatch;
	// 8300118C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001190: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83001194: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 83001198: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300119C: 4E800421  bctrl
	ctx.lr = 0x830011A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830011A0: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 830011A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830011A8: 481A7014  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830011B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830011B0 size=552
    let mut pc: u32 = 0x830011B0;
    'dispatch: loop {
        match pc {
            0x830011B0 => {
    //   block [0x830011B0..0x830013D8)
	// 830011B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830011B4: 481A6FB5  bl 0x831a8168
	ctx.lr = 0x830011B8;
	sub_831A8130(ctx, base);
	// 830011B8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830011BC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830011C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830011C4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830011C8: 419A01E4  beq cr6, 0x830013ac
	if ctx.cr[6].eq {
	pc = 0x830013AC; continue 'dispatch;
	}
	// 830011CC: 897F001C  lbz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830011D0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830011D4: 41820028  beq 0x830011fc
	if ctx.cr[0].eq {
	pc = 0x830011FC; continue 'dispatch;
	}
	// 830011D8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830011DC: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830011E0: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 830011E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830011E8: 4BFF8CE9  bl 0x82ff9ed0
	ctx.lr = 0x830011EC;
	sub_82FF9ED0(ctx, base);
	// 830011EC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830011F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830011F4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830011F8: 481AFA31  bl 0x831b0c28
	ctx.lr = 0x830011FC;
	sub_831B0C28(ctx, base);
	// 830011FC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001200: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83001204: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83001208: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300120C: 4E800421  bctrl
	ctx.lr = 0x83001210;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001210: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83001214: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83001218: 419A0028  beq cr6, 0x83001240
	if ctx.cr[6].eq {
	pc = 0x83001240; continue 'dispatch;
	}
	// 8300121C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83001220: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83001224: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83001228: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300122C: 4BFF8CA5  bl 0x82ff9ed0
	ctx.lr = 0x83001230;
	sub_82FF9ED0(ctx, base);
	// 83001230: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83001234: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83001238: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300123C: 481AF9ED  bl 0x831b0c28
	ctx.lr = 0x83001240;
	sub_831B0C28(ctx, base);
	// 83001240: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001244: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83001248: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300124C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001250: 4E800421  bctrl
	ctx.lr = 0x83001254;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001254: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83001258: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300125C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83001260: 7D7E0734  extsh r30, r11
	ctx.r[30].s64 = ctx.r[11].s16 as i64;
	// 83001264: 4BFFF125  bl 0x83000388
	ctx.lr = 0x83001268;
	sub_83000388(ctx, base);
	// 83001268: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300126C: 41820148  beq 0x830013b4
	if ctx.cr[0].eq {
	pc = 0x830013B4; continue 'dispatch;
	}
	// 83001270: 2F1E000A  cmpwi cr6, r30, 0xa
	ctx.cr[6].compare_i32(ctx.r[30].s32, 10, &mut ctx.xer);
	// 83001274: 419A0140  beq cr6, 0x830013b4
	if ctx.cr[6].eq {
	pc = 0x830013B4; continue 'dispatch;
	}
	// 83001278: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300127C: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83001280: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83001284: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001288: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300128C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001290: 4E800421  bctrl
	ctx.lr = 0x83001294;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001294: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83001298: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8300129C: 419A001C  beq cr6, 0x830012b8
	if ctx.cr[6].eq {
	pc = 0x830012B8; continue 'dispatch;
	}
	// 830012A0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830012A4: 419A0014  beq cr6, 0x830012b8
	if ctx.cr[6].eq {
	pc = 0x830012B8; continue 'dispatch;
	}
	// 830012A8: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 830012AC: 419A000C  beq cr6, 0x830012b8
	if ctx.cr[6].eq {
	pc = 0x830012B8; continue 'dispatch;
	}
	// 830012B0: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 830012B4: 409A001C  bne cr6, 0x830012d0
	if !ctx.cr[6].eq {
	pc = 0x830012D0; continue 'dispatch;
	}
	// 830012B8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830012BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830012C0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830012C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830012C8: 4E800421  bctrl
	ctx.lr = 0x830012CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830012CC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830012D0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830012D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830012D8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830012DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830012E0: 4E800421  bctrl
	ctx.lr = 0x830012E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830012E4: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 830012E8: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830012EC: 419A001C  beq cr6, 0x83001308
	if ctx.cr[6].eq {
	pc = 0x83001308; continue 'dispatch;
	}
	// 830012F0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830012F4: 419A0014  beq cr6, 0x83001308
	if ctx.cr[6].eq {
	pc = 0x83001308; continue 'dispatch;
	}
	// 830012F8: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 830012FC: 419A000C  beq cr6, 0x83001308
	if ctx.cr[6].eq {
	pc = 0x83001308; continue 'dispatch;
	}
	// 83001300: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83001304: 409A001C  bne cr6, 0x83001320
	if !ctx.cr[6].eq {
	pc = 0x83001320; continue 'dispatch;
	}
	// 83001308: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300130C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001310: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83001314: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001318: 4E800421  bctrl
	ctx.lr = 0x8300131C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300131C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83001320: 7F1CF040  cmplw cr6, r28, r30
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83001324: 419A0028  beq cr6, 0x8300134c
	if ctx.cr[6].eq {
	pc = 0x8300134C; continue 'dispatch;
	}
	// 83001328: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300132C: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83001330: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83001334: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83001338: 48037A51  bl 0x83038d88
	ctx.lr = 0x8300133C;
	sub_83038D88(ctx, base);
	// 8300133C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83001340: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83001344: 388BC878  addi r4, r11, -0x3788
	ctx.r[4].s64 = ctx.r[11].s64 + -14216;
	// 83001348: 481AF8E1  bl 0x831b0c28
	ctx.lr = 0x8300134C;
	sub_831B0C28(ctx, base);
	// 8300134C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001350: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83001354: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 83001358: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300135C: 4E800421  bctrl
	ctx.lr = 0x83001360;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001360: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001364: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83001368: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300136C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83001370: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 83001374: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001378: 4E800421  bctrl
	ctx.lr = 0x8300137C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300137C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001380: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83001384: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83001388: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8300138C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001390: 4E800421  bctrl
	ctx.lr = 0x83001394;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001394: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001398: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300139C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830013A0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830013A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830013A8: 4E800421  bctrl
	ctx.lr = 0x830013AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830013AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830013B0: 481A6E08  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 830013B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830013B8: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830013BC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 830013C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830013C4: 480379C5  bl 0x83038d88
	ctx.lr = 0x830013C8;
	sub_83038D88(ctx, base);
	// 830013C8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830013CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830013D0: 388BC878  addi r4, r11, -0x3788
	ctx.r[4].s64 = ctx.r[11].s64 + -14216;
	// 830013D4: 481AF855  bl 0x831b0c28
	ctx.lr = 0x830013D8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830013D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830013D8 size=872
    let mut pc: u32 = 0x830013D8;
    'dispatch: loop {
        match pc {
            0x830013D8 => {
    //   block [0x830013D8..0x83001740)
	// 830013D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830013DC: 481A6D85  bl 0x831a8160
	ctx.lr = 0x830013E0;
	sub_831A8130(ctx, base);
	// 830013E0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830013E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830013E8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830013EC: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830013F0: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830013F4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830013F8: 419A0028  beq cr6, 0x83001420
	if ctx.cr[6].eq {
	pc = 0x83001420; continue 'dispatch;
	}
	// 830013FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83001400: 80DE0024  lwz r6, 0x24(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83001404: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83001408: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8300140C: 4BFF8AC5  bl 0x82ff9ed0
	ctx.lr = 0x83001410;
	sub_82FF9ED0(ctx, base);
	// 83001410: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83001414: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83001418: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300141C: 481AF80D  bl 0x831b0c28
	ctx.lr = 0x83001420;
	sub_831B0C28(ctx, base);
	// 83001420: 897E001C  lbz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83001424: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001428: 41820028  beq 0x83001450
	if ctx.cr[0].eq {
	pc = 0x83001450; continue 'dispatch;
	}
	// 8300142C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83001430: 80DE0024  lwz r6, 0x24(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83001434: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 83001438: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8300143C: 4BFF8A95  bl 0x82ff9ed0
	ctx.lr = 0x83001440;
	sub_82FF9ED0(ctx, base);
	// 83001440: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83001444: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83001448: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300144C: 481AF7DD  bl 0x831b0c28
	ctx.lr = 0x83001450;
	sub_831B0C28(ctx, base);
	// 83001450: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 83001454: 419800A8  blt cr6, 0x830014fc
	if ctx.cr[6].lt {
	pc = 0x830014FC; continue 'dispatch;
	}
	// 83001458: 419A0068  beq cr6, 0x830014c0
	if ctx.cr[6].eq {
	pc = 0x830014C0; continue 'dispatch;
	}
	// 8300145C: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 83001460: 41980040  blt cr6, 0x830014a0
	if ctx.cr[6].lt {
	pc = 0x830014A0; continue 'dispatch;
	}
	// 83001464: 409A00B8  bne cr6, 0x8300151c
	if !ctx.cr[6].eq {
	pc = 0x8300151C; continue 'dispatch;
	}
	// 83001468: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300146C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83001470: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83001474: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001478: 4E800421  bctrl
	ctx.lr = 0x8300147C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300147C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001480: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83001484: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83001488: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300148C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83001490: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001494: 4E800421  bctrl
	ctx.lr = 0x83001498;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001498: 83FE0008  lwz r31, 8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300149C: 48000058  b 0x830014f4
	pc = 0x830014F4; continue 'dispatch;
	// 830014A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830014A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830014A8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830014AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830014B0: 4E800421  bctrl
	ctx.lr = 0x830014B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830014B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830014B8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830014BC: 48000020  b 0x830014dc
	pc = 0x830014DC; continue 'dispatch;
	// 830014C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830014C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830014C8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830014CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830014D0: 4E800421  bctrl
	ctx.lr = 0x830014D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830014D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830014D8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830014DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830014E0: 839E000C  lwz r28, 0xc(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830014E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830014E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830014EC: 4E800421  bctrl
	ctx.lr = 0x830014F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830014F0: 83FE0010  lwz r31, 0x10(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830014F4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830014F8: 48000034  b 0x8300152c
	pc = 0x8300152C; continue 'dispatch;
	// 830014FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83001504: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83001508: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300150C: 4E800421  bctrl
	ctx.lr = 0x83001510;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001510: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001514: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001518: 4BFFFF6C  b 0x83001484
	pc = 0x83001484; continue 'dispatch;
	// 8300151C: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83001520: 83810050  lwz r28, 0x50(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83001524: 83610050  lwz r27, 0x50(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83001528: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300152C: 7F1CE840  cmplw cr6, r28, r29
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83001530: 409A0020  bne cr6, 0x83001550
	if !ctx.cr[6].eq {
	pc = 0x83001550; continue 'dispatch;
	}
	// 83001534: 7F1FD800  cmpw cr6, r31, r27
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[27].s32, &mut ctx.xer);
	// 83001538: 419801FC  blt cr6, 0x83001734
	if ctx.cr[6].lt {
	pc = 0x83001734; continue 'dispatch;
	}
	// 8300153C: 7D7FD850  subf r11, r31, r27
	ctx.r[11].s64 = ctx.r[27].s64 - ctx.r[31].s64;
	// 83001540: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83001544: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83001548: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8300154C: 480001EC  b 0x83001738
	pc = 0x83001738; continue 'dispatch;
	// 83001550: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001554: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83001558: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8300155C: 48000024  b 0x83001580
	pc = 0x83001580; continue 'dispatch;
	// 83001560: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83001564: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83001568: 4BFFED19  bl 0x83000280
	ctx.lr = 0x8300156C;
	sub_83000280(ctx, base);
	// 8300156C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83001570: 40820030  bne 0x830015a0
	if !ctx.cr[0].eq {
	pc = 0x830015A0; continue 'dispatch;
	}
	// 83001574: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001578: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8300157C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83001580: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001584: 4E800421  bctrl
	ctx.lr = 0x83001588;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001588: 7C7A1B79  or. r26, r3, r3
	ctx.r[26].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8300158C: 4082FFD4  bne 0x83001560
	if !ctx.cr[0].eq {
	pc = 0x83001560; continue 'dispatch;
	}
	// 83001590: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001594: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83001598: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8300159C: 48000044  b 0x830015e0
	pc = 0x830015E0; continue 'dispatch;
	// 830015A0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830015A4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830015A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830015AC: 4BFFEE4D  bl 0x830003f8
	ctx.lr = 0x830015B0;
	sub_830003F8(ctx, base);
	// 830015B0: 7F1F1800  cmpw cr6, r31, r3
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[3].s32, &mut ctx.xer);
	// 830015B4: 40990180  ble cr6, 0x83001734
	if !ctx.cr[6].gt {
	pc = 0x83001734; continue 'dispatch;
	}
	// 830015B8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830015BC: 4800017C  b 0x83001738
	pc = 0x83001738; continue 'dispatch;
	// 830015C0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830015C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830015C8: 4BFFECB9  bl 0x83000280
	ctx.lr = 0x830015CC;
	sub_83000280(ctx, base);
	// 830015CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830015D0: 408200DC  bne 0x830016ac
	if !ctx.cr[0].eq {
	pc = 0x830016AC; continue 'dispatch;
	}
	// 830015D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830015D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830015DC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 830015E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830015E4: 4E800421  bctrl
	ctx.lr = 0x830015E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830015E8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830015EC: 4082FFD4  bne 0x830015c0
	if !ctx.cr[0].eq {
	pc = 0x830015C0; continue 'dispatch;
	}
	// 830015F0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830015F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830015F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830015FC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83001600: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83001604: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001608: 4E800421  bctrl
	ctx.lr = 0x8300160C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300160C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001610: 4082FFE8  bne 0x830015f8
	if !ctx.cr[0].eq {
	pc = 0x830015F8; continue 'dispatch;
	}
	// 83001614: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83001618: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300161C: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 83001620: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83001624: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001628: 4E800421  bctrl
	ctx.lr = 0x8300162C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300162C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001630: 4082FFE8  bne 0x83001618
	if !ctx.cr[0].eq {
	pc = 0x83001618; continue 'dispatch;
	}
	// 83001634: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83001638: 40990028  ble cr6, 0x83001660
	if !ctx.cr[6].gt {
	pc = 0x83001660; continue 'dispatch;
	}
	// 8300163C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001640: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83001644: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83001648: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300164C: 4E800421  bctrl
	ctx.lr = 0x83001650;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001650: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83001654: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83001658: 4181FFE4  bgt 0x8300163c
	if ctx.cr[0].gt {
	pc = 0x8300163C; continue 'dispatch;
	}
	// 8300165C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83001660: 40980028  bge cr6, 0x83001688
	if !ctx.cr[6].lt {
	pc = 0x83001688; continue 'dispatch;
	}
	// 83001664: 7FFF00D0  neg r31, r31
	ctx.r[31].s64 = -ctx.r[31].s64;
	// 83001668: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300166C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83001670: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83001674: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001678: 4E800421  bctrl
	ctx.lr = 0x8300167C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300167C: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83001680: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83001684: 4082FFE4  bne 0x83001668
	if !ctx.cr[0].eq {
	pc = 0x83001668; continue 'dispatch;
	}
	// 83001688: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300168C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83001690: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83001694: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001698: 4E800421  bctrl
	ctx.lr = 0x8300169C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300169C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830016A0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830016A4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830016A8: 48000048  b 0x830016f0
	pc = 0x830016F0; continue 'dispatch;
	// 830016AC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830016B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830016B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830016B8: 4BFFED41  bl 0x830003f8
	ctx.lr = 0x830016BC;
	sub_830003F8(ctx, base);
	// 830016BC: 7F03D800  cmpw cr6, r3, r27
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[27].s32, &mut ctx.xer);
	// 830016C0: 41980074  blt cr6, 0x83001734
	if ctx.cr[6].lt {
	pc = 0x83001734; continue 'dispatch;
	}
	// 830016C4: 4BFFFEF4  b 0x830015b8
	pc = 0x830015B8; continue 'dispatch;
	// 830016C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830016CC: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 830016D0: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 830016D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830016D8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830016DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830016E0: 4E800421  bctrl
	ctx.lr = 0x830016E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830016E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830016E8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830016EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830016F0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830016F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830016F8: 4E800421  bctrl
	ctx.lr = 0x830016FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830016FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83001700: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83001704: 409AFFC4  bne cr6, 0x830016c8
	if !ctx.cr[6].eq {
	pc = 0x830016C8; continue 'dispatch;
	}
	// 83001708: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300170C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83001710: 48000010  b 0x83001720
	pc = 0x83001720; continue 'dispatch;
	// 83001714: 7F03E040  cmplw cr6, r3, r28
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83001718: 419AFEA0  beq cr6, 0x830015b8
	if ctx.cr[6].eq {
	pc = 0x830015B8; continue 'dispatch;
	}
	// 8300171C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001720: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83001724: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001728: 4E800421  bctrl
	ctx.lr = 0x8300172C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300172C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001730: 4082FFE4  bne 0x83001714
	if !ctx.cr[0].eq {
	pc = 0x83001714; continue 'dispatch;
	}
	// 83001734: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83001738: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8300173C: 481A6A74  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83001740 size=732
    let mut pc: u32 = 0x83001740;
    'dispatch: loop {
        match pc {
            0x83001740 => {
    //   block [0x83001740..0x83001A1C)
	// 83001740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83001744: 481A6A25  bl 0x831a8168
	ctx.lr = 0x83001748;
	sub_831A8130(ctx, base);
	// 83001748: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300174C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83001750: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83001754: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83001758: 419A0284  beq cr6, 0x830019dc
	if ctx.cr[6].eq {
	pc = 0x830019DC; continue 'dispatch;
	}
	// 8300175C: 897F001C  lbz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83001760: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001764: 41820028  beq 0x8300178c
	if ctx.cr[0].eq {
	pc = 0x8300178C; continue 'dispatch;
	}
	// 83001768: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300176C: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83001770: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 83001774: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83001778: 4BFF8759  bl 0x82ff9ed0
	ctx.lr = 0x8300177C;
	sub_82FF9ED0(ctx, base);
	// 8300177C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83001780: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83001784: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83001788: 481AF4A1  bl 0x831b0c28
	ctx.lr = 0x8300178C;
	sub_831B0C28(ctx, base);
	// 8300178C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001790: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83001794: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83001798: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300179C: 4E800421  bctrl
	ctx.lr = 0x830017A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830017A0: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 830017A4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 830017A8: 419A0250  beq cr6, 0x830019f8
	if ctx.cr[6].eq {
	pc = 0x830019F8; continue 'dispatch;
	}
	// 830017AC: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 830017B0: 419A0248  beq cr6, 0x830019f8
	if ctx.cr[6].eq {
	pc = 0x830019F8; continue 'dispatch;
	}
	// 830017B4: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 830017B8: 419A0240  beq cr6, 0x830019f8
	if ctx.cr[6].eq {
	pc = 0x830019F8; continue 'dispatch;
	}
	// 830017BC: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 830017C0: 419A0238  beq cr6, 0x830019f8
	if ctx.cr[6].eq {
	pc = 0x830019F8; continue 'dispatch;
	}
	// 830017C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830017C8: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830017CC: 4BFFEAB5  bl 0x83000280
	ctx.lr = 0x830017D0;
	sub_83000280(ctx, base);
	// 830017D0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830017D4: 41820028  beq 0x830017fc
	if ctx.cr[0].eq {
	pc = 0x830017FC; continue 'dispatch;
	}
	// 830017D8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830017DC: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830017E0: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 830017E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830017E8: 4BFF86E9  bl 0x82ff9ed0
	ctx.lr = 0x830017EC;
	sub_82FF9ED0(ctx, base);
	// 830017EC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830017F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830017F4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830017F8: 481AF431  bl 0x831b0c28
	ctx.lr = 0x830017FC;
	sub_831B0C28(ctx, base);
	// 830017FC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83001800: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001804: 41820040  beq 0x83001844
	if ctx.cr[0].eq {
	pc = 0x83001844; continue 'dispatch;
	}
	// 83001808: 3FC08214  lis r30, -0x7dec
	ctx.r[30].s64 = -2112618496;
	// 8300180C: A15C0008  lhz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001810: A17EA698  lhz r11, -0x5968(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83001814: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83001818: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300181C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83001820: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83001824: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001828: 40820060  bne 0x83001888
	if !ctx.cr[0].eq {
	pc = 0x83001888; continue 'dispatch;
	}
	// 8300182C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001830: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83001834: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001838: 4E800421  bctrl
	ctx.lr = 0x8300183C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300183C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001840: 4082FFCC  bne 0x8300180c
	if !ctx.cr[0].eq {
	pc = 0x8300180C; continue 'dispatch;
	}
	// 83001844: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001848: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300184C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83001850: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001854: 4E800421  bctrl
	ctx.lr = 0x83001858;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001858: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8300185C: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83001860: 419A004C  beq cr6, 0x830018ac
	if ctx.cr[6].eq {
	pc = 0x830018AC; continue 'dispatch;
	}
	// 83001864: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83001868: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300186C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83001870: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83001874: 4BFF865D  bl 0x82ff9ed0
	ctx.lr = 0x83001878;
	sub_82FF9ED0(ctx, base);
	// 83001878: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300187C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83001880: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83001884: 481AF3A5  bl 0x831b0c28
	ctx.lr = 0x83001888;
	sub_831B0C28(ctx, base);
	// 83001888: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300188C: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83001890: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83001894: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83001898: 4BFF8639  bl 0x82ff9ed0
	ctx.lr = 0x8300189C;
	sub_82FF9ED0(ctx, base);
	// 8300189C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830018A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830018A4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830018A8: 481AF381  bl 0x831b0c28
	ctx.lr = 0x830018AC;
	sub_831B0C28(ctx, base);
	// 830018AC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830018B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830018B4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830018B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830018BC: 4E800421  bctrl
	ctx.lr = 0x830018C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830018C0: 7C7E0734  extsh r30, r3
	ctx.r[30].s64 = ctx.r[3].s16 as i64;
	// 830018C4: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 830018C8: 419A0070  beq cr6, 0x83001938
	if ctx.cr[6].eq {
	pc = 0x83001938; continue 'dispatch;
	}
	// 830018CC: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 830018D0: 419A0068  beq cr6, 0x83001938
	if ctx.cr[6].eq {
	pc = 0x83001938; continue 'dispatch;
	}
	// 830018D4: 2F1E0008  cmpwi cr6, r30, 8
	ctx.cr[6].compare_i32(ctx.r[30].s32, 8, &mut ctx.xer);
	// 830018D8: 419A0060  beq cr6, 0x83001938
	if ctx.cr[6].eq {
	pc = 0x83001938; continue 'dispatch;
	}
	// 830018DC: 2F1E0007  cmpwi cr6, r30, 7
	ctx.cr[6].compare_i32(ctx.r[30].s32, 7, &mut ctx.xer);
	// 830018E0: 419A0058  beq cr6, 0x83001938
	if ctx.cr[6].eq {
	pc = 0x83001938; continue 'dispatch;
	}
	// 830018E4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830018E8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830018EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830018F0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830018F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830018F8: 4E800421  bctrl
	ctx.lr = 0x830018FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830018FC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001900: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83001904: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83001908: 409900A8  ble cr6, 0x830019b0
	if !ctx.cr[6].gt {
	pc = 0x830019B0; continue 'dispatch;
	}
	// 8300190C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83001910: 419A00A0  beq cr6, 0x830019b0
	if ctx.cr[6].eq {
	pc = 0x830019B0; continue 'dispatch;
	}
	// 83001914: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001918: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300191C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001920: 4E800421  bctrl
	ctx.lr = 0x83001924;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001924: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001928: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8300192C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83001930: 4198FFDC  blt cr6, 0x8300190c
	if ctx.cr[6].lt {
	pc = 0x8300190C; continue 'dispatch;
	}
	// 83001934: 4800007C  b 0x830019b0
	pc = 0x830019B0; continue 'dispatch;
	// 83001938: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300193C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001940: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83001944: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001948: 4E800421  bctrl
	ctx.lr = 0x8300194C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300194C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001950: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83001954: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001958: 41820038  beq 0x83001990
	if ctx.cr[0].eq {
	pc = 0x83001990; continue 'dispatch;
	}
	// 8300195C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83001960: 2F1E0008  cmpwi cr6, r30, 8
	ctx.cr[6].compare_i32(ctx.r[30].s32, 8, &mut ctx.xer);
	// 83001964: 409A000C  bne cr6, 0x83001970
	if !ctx.cr[6].eq {
	pc = 0x83001970; continue 'dispatch;
	}
	// 83001968: 480047F9  bl 0x83006160
	ctx.lr = 0x8300196C;
	sub_83006160(ctx, base);
	// 8300196C: 48000024  b 0x83001990
	pc = 0x83001990; continue 'dispatch;
	// 83001970: 2F1E0007  cmpwi cr6, r30, 7
	ctx.cr[6].compare_i32(ctx.r[30].s32, 7, &mut ctx.xer);
	// 83001974: 409A000C  bne cr6, 0x83001980
	if !ctx.cr[6].eq {
	pc = 0x83001980; continue 'dispatch;
	}
	// 83001978: 48009421  bl 0x8300ad98
	ctx.lr = 0x8300197C;
	sub_8300AD98(ctx, base);
	// 8300197C: 48000014  b 0x83001990
	pc = 0x83001990; continue 'dispatch;
	// 83001980: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001984: 816B00BC  lwz r11, 0xbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(188 as u32) ) } as u64;
	// 83001988: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300198C: 4E800421  bctrl
	ctx.lr = 0x83001990;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001990: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001994: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83001998: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300199C: 419A0014  beq cr6, 0x830019b0
	if ctx.cr[6].eq {
	pc = 0x830019B0; continue 'dispatch;
	}
	// 830019A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830019A4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 830019A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830019AC: 4E800421  bctrl
	ctx.lr = 0x830019B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830019B0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830019B4: 419A0028  beq cr6, 0x830019dc
	if ctx.cr[6].eq {
	pc = 0x830019DC; continue 'dispatch;
	}
	// 830019B8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830019BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830019C0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830019C4: 419A0020  beq cr6, 0x830019e4
	if ctx.cr[6].eq {
	pc = 0x830019E4; continue 'dispatch;
	}
	// 830019C8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830019CC: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830019D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830019D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830019D8: 4E800421  bctrl
	ctx.lr = 0x830019DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830019DC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830019E0: 481A67D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 830019E4: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 830019E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830019EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830019F0: 4E800421  bctrl
	ctx.lr = 0x830019F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830019F4: 4BFFFFE8  b 0x830019dc
	pc = 0x830019DC; continue 'dispatch;
	// 830019F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830019FC: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83001A00: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83001A04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83001A08: 48037381  bl 0x83038d88
	ctx.lr = 0x83001A0C;
	sub_83038D88(ctx, base);
	// 83001A0C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83001A10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83001A14: 388BC878  addi r4, r11, -0x3788
	ctx.r[4].s64 = ctx.r[11].s64 + -14216;
	// 83001A18: 481AF211  bl 0x831b0c28
	ctx.lr = 0x83001A1C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83001A20 size=8
    let mut pc: u32 = 0x83001A20;
    'dispatch: loop {
        match pc {
            0x83001A20 => {
    //   block [0x83001A20..0x83001A28)
	// 83001A20: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83001A24: 82140530  lwz r16, 0x530(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(1328 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83001A28 size=1396
    let mut pc: u32 = 0x83001A28;
    'dispatch: loop {
        match pc {
            0x83001A28 => {
    //   block [0x83001A28..0x83001F9C)
	// 83001A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83001A2C: 481A672D  bl 0x831a8158
	ctx.lr = 0x83001A30;
	sub_831A8130(ctx, base);
	// 83001A30: 3BE1DFE0  addi r31, r1, -0x2020
	ctx.r[31].s64 = ctx.r[1].s64 + -8224;
	// 83001A34: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 83001A38: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 83001A3C: 9421DFE0  stwu r1, -0x2020(r1)
	ea = ctx.r[1].u32.wrapping_add(-8224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83001A40: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83001A44: 897E001C  lbz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83001A48: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001A4C: 41820028  beq 0x83001a74
	if ctx.cr[0].eq {
	pc = 0x83001A74; continue 'dispatch;
	}
	// 83001A50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83001A54: 80DE0024  lwz r6, 0x24(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83001A58: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 83001A5C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83001A60: 4BFF8471  bl 0x82ff9ed0
	ctx.lr = 0x83001A64;
	sub_82FF9ED0(ctx, base);
	// 83001A64: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83001A68: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83001A6C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83001A70: 481AF1B9  bl 0x831b0c28
	ctx.lr = 0x83001A74;
	sub_831B0C28(ctx, base);
	// 83001A74: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83001A78: 835E0004  lwz r26, 4(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83001A7C: 7F1A5840  cmplw cr6, r26, r11
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83001A80: 409A0020  bne cr6, 0x83001aa0
	if !ctx.cr[6].eq {
	pc = 0x83001AA0; continue 'dispatch;
	}
	// 83001A84: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83001A88: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001A8C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83001A90: 409A0010  bne cr6, 0x83001aa0
	if !ctx.cr[6].eq {
	pc = 0x83001AA0; continue 'dispatch;
	}
	// 83001A94: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83001A98: 386B8158  addi r3, r11, -0x7ea8
	ctx.r[3].s64 = ctx.r[11].s64 + -32424;
	// 83001A9C: 480004F8  b 0x83001f94
	pc = 0x83001F94; continue 'dispatch;
	// 83001AA0: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 83001AA4: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83001AA8: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 83001AAC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83001AB0: 80AB0090  lwz r5, 0x90(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 83001AB4: 4BFDD3A5  bl 0x82fdee58
	ctx.lr = 0x83001AB8;
	sub_82FDEE58(ctx, base);
	// 83001AB8: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83001ABC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001AC0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83001AC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001AC8: 4E800421  bctrl
	ctx.lr = 0x83001ACC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001ACC: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83001AD0: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83001AD4: 419A0090  beq cr6, 0x83001b64
	if ctx.cr[6].eq {
	pc = 0x83001B64; continue 'dispatch;
	}
	// 83001AD8: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83001ADC: 419A0088  beq cr6, 0x83001b64
	if ctx.cr[6].eq {
	pc = 0x83001B64; continue 'dispatch;
	}
	// 83001AE0: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 83001AE4: 419A0080  beq cr6, 0x83001b64
	if ctx.cr[6].eq {
	pc = 0x83001B64; continue 'dispatch;
	}
	// 83001AE8: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83001AEC: 419A0078  beq cr6, 0x83001b64
	if ctx.cr[6].eq {
	pc = 0x83001B64; continue 'dispatch;
	}
	// 83001AF0: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001AF4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83001AF8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83001AFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001B00: 4E800421  bctrl
	ctx.lr = 0x83001B04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001B04: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001B08: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83001B0C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001B10: 41820040  beq 0x83001b50
	if ctx.cr[0].eq {
	pc = 0x83001B50; continue 'dispatch;
	}
	// 83001B14: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83001B18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83001B1C: 419A0034  beq cr6, 0x83001b50
	if ctx.cr[6].eq {
	pc = 0x83001B50; continue 'dispatch;
	}
	// 83001B20: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83001B24: 419A0034  beq cr6, 0x83001b58
	if ctx.cr[6].eq {
	pc = 0x83001B58; continue 'dispatch;
	}
	// 83001B28: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001B2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83001B30: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83001B34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001B38: 4E800421  bctrl
	ctx.lr = 0x83001B3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001B3C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001B40: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83001B44: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83001B48: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83001B4C: 4198FFD4  blt cr6, 0x83001b20
	if ctx.cr[6].lt {
	pc = 0x83001B20; continue 'dispatch;
	}
	// 83001B50: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83001B54: 409A0208  bne cr6, 0x83001d5c
	if !ctx.cr[6].eq {
	pc = 0x83001D5C; continue 'dispatch;
	}
	// 83001B58: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83001B5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83001B60: 480001F0  b 0x83001d50
	pc = 0x83001D50; continue 'dispatch;
	// 83001B64: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83001B68: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83001B6C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83001B70: 409A00D8  bne cr6, 0x83001c48
	if !ctx.cr[6].eq {
	pc = 0x83001C48; continue 'dispatch;
	}
	// 83001B74: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83001B78: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001B7C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83001B80: 2B0B0F9F  cmplwi cr6, r11, 0xf9f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3999 as u32, &mut ctx.xer);
	// 83001B84: 41980028  blt cr6, 0x83001bac
	if ctx.cr[6].lt {
	pc = 0x83001BAC; continue 'dispatch;
	}
	// 83001B88: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83001B8C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83001B90: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83001B94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001B98: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83001B9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001BA0: 4E800421  bctrl
	ctx.lr = 0x83001BA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001BA4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83001BA8: 48000008  b 0x83001bb0
	pc = 0x83001BB0; continue 'dispatch;
	// 83001BAC: 3BBF0090  addi r29, r31, 0x90
	ctx.r[29].s64 = ctx.r[31].s64 + 144;
	// 83001BB0: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83001BB4: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83001BB8: 837E0010  lwz r27, 0x10(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83001BBC: 835E0008  lwz r26, 8(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001BC0: 838B0090  lwz r28, 0x90(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 83001BC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001BC8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001BCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001BD0: 4E800421  bctrl
	ctx.lr = 0x83001BD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001BD4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83001BD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83001BDC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83001BE0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83001BE4: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 83001BE8: 4BFD04A1  bl 0x82fd2088
	ctx.lr = 0x83001BEC;
	sub_82FD2088(ctx, base);
	// 83001BEC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83001BF0: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83001BF4: 4BFDFDE5  bl 0x82fe19d8
	ctx.lr = 0x83001BF8;
	sub_82FE19D8(ctx, base);
	// 83001BF8: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83001BFC: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001C00: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83001C04: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83001C08: 2B0B0F9F  cmplwi cr6, r11, 0xf9f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3999 as u32, &mut ctx.xer);
	// 83001C0C: 4198001C  blt cr6, 0x83001c28
	if ctx.cr[6].lt {
	pc = 0x83001C28; continue 'dispatch;
	}
	// 83001C10: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83001C14: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83001C18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001C1C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001C20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001C24: 4E800421  bctrl
	ctx.lr = 0x83001C28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001C28: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83001C2C: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83001C30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001C34: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001C38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001C3C: 4E800421  bctrl
	ctx.lr = 0x83001C40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001C40: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83001C44: 48000350  b 0x83001f94
	pc = 0x83001F94; continue 'dispatch;
	// 83001C48: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83001C4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001C50: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001C54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001C58: 4E800421  bctrl
	ctx.lr = 0x83001C5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001C5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001C60: 41820034  beq 0x83001c94
	if ctx.cr[0].eq {
	pc = 0x83001C94; continue 'dispatch;
	}
	// 83001C64: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001C68: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001C6C: 41820028  beq 0x83001c94
	if ctx.cr[0].eq {
	pc = 0x83001C94; continue 'dispatch;
	}
	// 83001C70: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 83001C74: 48000008  b 0x83001c7c
	pc = 0x83001C7C; continue 'dispatch;
	// 83001C78: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83001C7C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001C80: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001C84: 4082FFF4  bne 0x83001c78
	if !ctx.cr[0].eq {
	pc = 0x83001C78; continue 'dispatch;
	}
	// 83001C88: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 83001C8C: 7D7C0E70  srawi r28, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83001C90: 48000008  b 0x83001c98
	pc = 0x83001C98; continue 'dispatch;
	// 83001C94: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83001C98: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001C9C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83001CA0: 419A00A8  beq cr6, 0x83001d48
	if ctx.cr[6].eq {
	pc = 0x83001D48; continue 'dispatch;
	}
	// 83001CA4: 7D6BE050  subf r11, r11, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	// 83001CA8: 2B0B0F9F  cmplwi cr6, r11, 0xf9f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3999 as u32, &mut ctx.xer);
	// 83001CAC: 41980028  blt cr6, 0x83001cd4
	if ctx.cr[6].lt {
	pc = 0x83001CD4; continue 'dispatch;
	}
	// 83001CB0: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83001CB4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83001CB8: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83001CBC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001CC0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83001CC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001CC8: 4E800421  bctrl
	ctx.lr = 0x83001CCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001CCC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83001CD0: 48000008  b 0x83001cd8
	pc = 0x83001CD8; continue 'dispatch;
	// 83001CD4: 3BBF0090  addi r29, r31, 0x90
	ctx.r[29].s64 = ctx.r[31].s64 + 144;
	// 83001CD8: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83001CDC: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83001CE0: 831E0008  lwz r24, 8(r30)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001CE4: 836B0090  lwz r27, 0x90(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 83001CE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001CEC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001CF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001CF4: 4E800421  bctrl
	ctx.lr = 0x83001CF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001CF8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83001CFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83001D00: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 83001D04: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83001D08: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 83001D0C: 4BFD037D  bl 0x82fd2088
	ctx.lr = 0x83001D10;
	sub_82FD2088(ctx, base);
	// 83001D10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83001D14: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83001D18: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83001D1C: 4BFD7855  bl 0x82fd9570
	ctx.lr = 0x83001D20;
	sub_82FD9570(ctx, base);
	// 83001D20: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001D24: 7D6BE050  subf r11, r11, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	// 83001D28: 2B0B0F9F  cmplwi cr6, r11, 0xf9f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3999 as u32, &mut ctx.xer);
	// 83001D2C: 4198001C  blt cr6, 0x83001d48
	if ctx.cr[6].lt {
	pc = 0x83001D48; continue 'dispatch;
	}
	// 83001D30: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83001D34: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83001D38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001D3C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001D40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001D44: 4E800421  bctrl
	ctx.lr = 0x83001D48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001D48: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83001D4C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83001D50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83001D54: 4BFFE8DD  bl 0x83000630
	ctx.lr = 0x83001D58;
	sub_83000630(ctx, base);
	// 83001D58: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83001D5C: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83001D60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001D64: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83001D68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001D6C: 4E800421  bctrl
	ctx.lr = 0x83001D70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001D70: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83001D74: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83001D78: 419A00F4  beq cr6, 0x83001e6c
	if ctx.cr[6].eq {
	pc = 0x83001E6C; continue 'dispatch;
	}
	// 83001D7C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83001D80: 419A00EC  beq cr6, 0x83001e6c
	if ctx.cr[6].eq {
	pc = 0x83001E6C; continue 'dispatch;
	}
	// 83001D84: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 83001D88: 419A00E4  beq cr6, 0x83001e6c
	if ctx.cr[6].eq {
	pc = 0x83001E6C; continue 'dispatch;
	}
	// 83001D8C: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83001D90: 419A00DC  beq cr6, 0x83001e6c
	if ctx.cr[6].eq {
	pc = 0x83001E6C; continue 'dispatch;
	}
	// 83001D94: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83001D98: 839E0010  lwz r28, 0x10(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83001D9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001DA0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83001DA4: 4800001C  b 0x83001dc0
	pc = 0x83001DC0; continue 'dispatch;
	// 83001DA8: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 83001DAC: 419A0030  beq cr6, 0x83001ddc
	if ctx.cr[6].eq {
	pc = 0x83001DDC; continue 'dispatch;
	}
	// 83001DB0: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001DB4: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 83001DB8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83001DBC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83001DC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001DC4: 4E800421  bctrl
	ctx.lr = 0x83001DC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001DC8: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83001DCC: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 83001DD0: 4199FFD8  bgt cr6, 0x83001da8
	if ctx.cr[6].gt {
	pc = 0x83001DA8; continue 'dispatch;
	}
	// 83001DD4: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 83001DD8: 409A0094  bne cr6, 0x83001e6c
	if !ctx.cr[6].eq {
	pc = 0x83001E6C; continue 'dispatch;
	}
	// 83001DDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83001DE0: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83001DE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83001DE8: 4BFFE849  bl 0x83000630
	ctx.lr = 0x83001DEC;
	sub_83000630(ctx, base);
	// 83001DEC: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 83001DF0: 4800007C  b 0x83001e6c
	pc = 0x83001E6C; continue 'dispatch;
	// 83001DF4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83001DF8: 419A007C  beq cr6, 0x83001e74
	if ctx.cr[6].eq {
	pc = 0x83001E74; continue 'dispatch;
	}
	// 83001DFC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001E00: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83001E04: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83001E08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001E0C: 4E800421  bctrl
	ctx.lr = 0x83001E10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001E10: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83001E14: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83001E18: 419A001C  beq cr6, 0x83001e34
	if ctx.cr[6].eq {
	pc = 0x83001E34; continue 'dispatch;
	}
	// 83001E1C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83001E20: 419A0014  beq cr6, 0x83001e34
	if ctx.cr[6].eq {
	pc = 0x83001E34; continue 'dispatch;
	}
	// 83001E24: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 83001E28: 419A000C  beq cr6, 0x83001e34
	if ctx.cr[6].eq {
	pc = 0x83001E34; continue 'dispatch;
	}
	// 83001E2C: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83001E30: 409A0028  bne cr6, 0x83001e58
	if !ctx.cr[6].eq {
	pc = 0x83001E58; continue 'dispatch;
	}
	// 83001E34: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001E38: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83001E3C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001E40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001E44: 4E800421  bctrl
	ctx.lr = 0x83001E48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001E48: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83001E4C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83001E50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83001E54: 4BFD771D  bl 0x82fd9570
	ctx.lr = 0x83001E58;
	sub_82FD9570(ctx, base);
	// 83001E58: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83001E5C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83001E60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83001E64: 4BFFE7CD  bl 0x83000630
	ctx.lr = 0x83001E68;
	sub_83000630(ctx, base);
	// 83001E68: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83001E6C: 7F1DC840  cmplw cr6, r29, r25
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[25].u32, &mut ctx.xer);
	// 83001E70: 409AFF84  bne cr6, 0x83001df4
	if !ctx.cr[6].eq {
	pc = 0x83001DF4; continue 'dispatch;
	}
	// 83001E74: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83001E78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001E7C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83001E80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001E84: 4E800421  bctrl
	ctx.lr = 0x83001E88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001E88: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83001E8C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83001E90: 419A001C  beq cr6, 0x83001eac
	if ctx.cr[6].eq {
	pc = 0x83001EAC; continue 'dispatch;
	}
	// 83001E94: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83001E98: 419A0014  beq cr6, 0x83001eac
	if ctx.cr[6].eq {
	pc = 0x83001EAC; continue 'dispatch;
	}
	// 83001E9C: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 83001EA0: 419A000C  beq cr6, 0x83001eac
	if ctx.cr[6].eq {
	pc = 0x83001EAC; continue 'dispatch;
	}
	// 83001EA4: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83001EA8: 409A00AC  bne cr6, 0x83001f54
	if !ctx.cr[6].eq {
	pc = 0x83001F54; continue 'dispatch;
	}
	// 83001EAC: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83001EB0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83001EB4: 418200A0  beq 0x83001f54
	if ctx.cr[0].eq {
	pc = 0x83001F54; continue 'dispatch;
	}
	// 83001EB8: 2B0B0F9F  cmplwi cr6, r11, 0xf9f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3999 as u32, &mut ctx.xer);
	// 83001EBC: 41980028  blt cr6, 0x83001ee4
	if ctx.cr[6].lt {
	pc = 0x83001EE4; continue 'dispatch;
	}
	// 83001EC0: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83001EC4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83001EC8: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83001ECC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001ED0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83001ED4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001ED8: 4E800421  bctrl
	ctx.lr = 0x83001EDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001EDC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83001EE0: 48000008  b 0x83001ee8
	pc = 0x83001EE8; continue 'dispatch;
	// 83001EE4: 3BBF0090  addi r29, r31, 0x90
	ctx.r[29].s64 = ctx.r[31].s64 + 144;
	// 83001EE8: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83001EEC: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83001EF0: 837E0010  lwz r27, 0x10(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83001EF4: 838B0090  lwz r28, 0x90(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 83001EF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001EFC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001F00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001F04: 4E800421  bctrl
	ctx.lr = 0x83001F08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001F08: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83001F0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83001F10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83001F14: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83001F18: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 83001F1C: 4BFD016D  bl 0x82fd2088
	ctx.lr = 0x83001F20;
	sub_82FD2088(ctx, base);
	// 83001F20: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83001F24: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83001F28: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83001F2C: 4BFD7645  bl 0x82fd9570
	ctx.lr = 0x83001F30;
	sub_82FD9570(ctx, base);
	// 83001F30: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83001F34: 2B0B0F9F  cmplwi cr6, r11, 0xf9f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3999 as u32, &mut ctx.xer);
	// 83001F38: 4198001C  blt cr6, 0x83001f54
	if ctx.cr[6].lt {
	pc = 0x83001F54; continue 'dispatch;
	}
	// 83001F3C: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83001F40: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83001F44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001F48: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001F4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001F50: 4E800421  bctrl
	ctx.lr = 0x83001F54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001F54: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83001F58: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83001F5C: 815F0068  lwz r10, 0x68(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83001F60: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83001F64: 7D2B532E  sthx r9, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u16) };
	// 83001F68: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83001F6C: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83001F70: 4BFDFA69  bl 0x82fe19d8
	ctx.lr = 0x83001F74;
	sub_82FE19D8(ctx, base);
	// 83001F74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83001F78: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83001F7C: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83001F80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001F84: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83001F88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001F8C: 4E800421  bctrl
	ctx.lr = 0x83001F90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83001F90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83001F94: 383F2020  addi r1, r31, 0x2020
	ctx.r[1].s64 = ctx.r[31].s64 + 8224;
	// 83001F98: 481A6210  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001F9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83001F9C size=40
    let mut pc: u32 = 0x83001F9C;
    'dispatch: loop {
        match pc {
            0x83001F9C => {
    //   block [0x83001F9C..0x83001FC4)
	// 83001F9C: 3BECDFE0  addi r31, r12, -0x2020
	ctx.r[31].s64 = ctx.r[12].s64 + -8224;
	// 83001FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83001FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83001FA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83001FAC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83001FB0: 4BFDCF29  bl 0x82fdeed8
	ctx.lr = 0x83001FB4;
	sub_82FDEED8(ctx, base);
	// 83001FB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83001FB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83001FBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83001FC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83001FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83001FC8 size=640
    let mut pc: u32 = 0x83001FC8;
    'dispatch: loop {
        match pc {
            0x83001FC8 => {
    //   block [0x83001FC8..0x83002248)
	// 83001FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83001FCC: 481A6189  bl 0x831a8154
	ctx.lr = 0x83001FD0;
	sub_831A8130(ctx, base);
	// 83001FD0: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 83001FD4: 9421E020  stwu r1, -0x1fe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-8160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83001FD8: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 83001FDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83001FE0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83001FE4: 2F190003  cmpwi cr6, r25, 3
	ctx.cr[6].compare_i32(ctx.r[25].s32, 3, &mut ctx.xer);
	// 83001FE8: 419A001C  beq cr6, 0x83002004
	if ctx.cr[6].eq {
	pc = 0x83002004; continue 'dispatch;
	}
	// 83001FEC: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83001FF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83001FF4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83001FF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83001FFC: 4E800421  bctrl
	ctx.lr = 0x83002000;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002000: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83002004: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83002008: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300200C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83002010: 419A022C  beq cr6, 0x8300223c
	if ctx.cr[6].eq {
	pc = 0x8300223C; continue 'dispatch;
	}
	// 83002014: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002018: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300201C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83002020: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002024: 4E800421  bctrl
	ctx.lr = 0x83002028;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002028: 7C7B0734  extsh r27, r3
	ctx.r[27].s64 = ctx.r[3].s16 as i64;
	// 8300202C: 2F1B0003  cmpwi cr6, r27, 3
	ctx.cr[6].compare_i32(ctx.r[27].s32, 3, &mut ctx.xer);
	// 83002030: 419A00A0  beq cr6, 0x830020d0
	if ctx.cr[6].eq {
	pc = 0x830020D0; continue 'dispatch;
	}
	// 83002034: 2F1B0004  cmpwi cr6, r27, 4
	ctx.cr[6].compare_i32(ctx.r[27].s32, 4, &mut ctx.xer);
	// 83002038: 419A0098  beq cr6, 0x830020d0
	if ctx.cr[6].eq {
	pc = 0x830020D0; continue 'dispatch;
	}
	// 8300203C: 2F1B0008  cmpwi cr6, r27, 8
	ctx.cr[6].compare_i32(ctx.r[27].s32, 8, &mut ctx.xer);
	// 83002040: 419A0090  beq cr6, 0x830020d0
	if ctx.cr[6].eq {
	pc = 0x830020D0; continue 'dispatch;
	}
	// 83002044: 2F1B0007  cmpwi cr6, r27, 7
	ctx.cr[6].compare_i32(ctx.r[27].s32, 7, &mut ctx.xer);
	// 83002048: 419A0088  beq cr6, 0x830020d0
	if ctx.cr[6].eq {
	pc = 0x830020D0; continue 'dispatch;
	}
	// 8300204C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002050: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002054: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002058: 4BFFE729  bl 0x83000780
	ctx.lr = 0x8300205C;
	sub_83000780(ctx, base);
	// 8300205C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83002060: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002064: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83002068: 7F8A5851  subf. r28, r10, r11
	ctx.r[28].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8300206C: 408101B0  ble 0x8300221c
	if !ctx.cr[0].gt {
	pc = 0x8300221C; continue 'dispatch;
	}
	// 83002070: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83002074: 419A01A8  beq cr6, 0x8300221c
	if ctx.cr[6].eq {
	pc = 0x8300221C; continue 'dispatch;
	}
	// 83002078: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300207C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83002080: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002084: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002088: 4E800421  bctrl
	ctx.lr = 0x8300208C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300208C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83002090: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 83002094: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83002098: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300209C: 4BFFE655  bl 0x830006f0
	ctx.lr = 0x830020A0;
	sub_830006F0(ctx, base);
	// 830020A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830020A4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 830020A8: 419A0018  beq cr6, 0x830020c0
	if ctx.cr[6].eq {
	pc = 0x830020C0; continue 'dispatch;
	}
	// 830020AC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830020B0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830020B4: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 830020B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830020BC: 4E800421  bctrl
	ctx.lr = 0x830020C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830020C0: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830020C4: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 830020C8: 4181FFA8  bgt 0x83002070
	if ctx.cr[0].gt {
	pc = 0x83002070; continue 'dispatch;
	}
	// 830020CC: 48000150  b 0x8300221c
	pc = 0x8300221C; continue 'dispatch;
	// 830020D0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830020D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830020D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830020DC: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830020E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830020E4: 4E800421  bctrl
	ctx.lr = 0x830020E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830020E8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830020EC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830020F0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830020F4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830020F8: 409A0018  bne cr6, 0x83002110
	if !ctx.cr[6].eq {
	pc = 0x83002110; continue 'dispatch;
	}
	// 830020FC: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002100: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83002104: 388B8158  addi r4, r11, -0x7ea8
	ctx.r[4].s64 = ctx.r[11].s64 + -32424;
	// 83002108: 816A0048  lwz r11, 0x48(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 8300210C: 480000B0  b 0x830021bc
	pc = 0x830021BC; continue 'dispatch;
	// 83002110: 2B0B0F9F  cmplwi cr6, r11, 0xf9f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3999 as u32, &mut ctx.xer);
	// 83002114: 41980028  blt cr6, 0x8300213c
	if ctx.cr[6].lt {
	pc = 0x8300213C; continue 'dispatch;
	}
	// 83002118: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300211C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83002120: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83002124: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002128: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300212C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002130: 4E800421  bctrl
	ctx.lr = 0x83002134;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002134: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83002138: 48000008  b 0x83002140
	pc = 0x83002140; continue 'dispatch;
	// 8300213C: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 83002140: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002144: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83002148: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8300214C: 831F0010  lwz r24, 0x10(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83002150: 82FF0008  lwz r23, 8(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002154: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002158: 838A0090  lwz r28, 0x90(r10)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300215C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002160: 4E800421  bctrl
	ctx.lr = 0x83002164;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002164: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83002168: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300216C: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 83002170: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 83002174: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 83002178: 4BFCFF11  bl 0x82fd2088
	ctx.lr = 0x8300217C;
	sub_82FD2088(ctx, base);
	// 8300217C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83002180: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002184: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83002188: 4BFDF851  bl 0x82fe19d8
	ctx.lr = 0x8300218C;
	sub_82FE19D8(ctx, base);
	// 8300218C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83002190: 817C0048  lwz r11, 0x48(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(72 as u32) ) } as u64;
	// 83002194: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83002198: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300219C: 4E800421  bctrl
	ctx.lr = 0x830021A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830021A0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830021A4: 2B0B0F9F  cmplwi cr6, r11, 0xf9f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3999 as u32, &mut ctx.xer);
	// 830021A8: 4198001C  blt cr6, 0x830021c4
	if ctx.cr[6].lt {
	pc = 0x830021C4; continue 'dispatch;
	}
	// 830021AC: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830021B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830021B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830021B8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830021BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830021C0: 4E800421  bctrl
	ctx.lr = 0x830021C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830021C4: 2F190002  cmpwi cr6, r25, 2
	ctx.cr[6].compare_i32(ctx.r[25].s32, 2, &mut ctx.xer);
	// 830021C8: 419A0034  beq cr6, 0x830021fc
	if ctx.cr[6].eq {
	pc = 0x830021FC; continue 'dispatch;
	}
	// 830021CC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830021D0: 2F1B0007  cmpwi cr6, r27, 7
	ctx.cr[6].compare_i32(ctx.r[27].s32, 7, &mut ctx.xer);
	// 830021D4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830021D8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830021DC: 7CA45850  subf r5, r4, r11
	ctx.r[5].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 830021E0: 409A000C  bne cr6, 0x830021ec
	if !ctx.cr[6].eq {
	pc = 0x830021EC; continue 'dispatch;
	}
	// 830021E4: 48008B4D  bl 0x8300ad30
	ctx.lr = 0x830021E8;
	sub_8300AD30(ctx, base);
	// 830021E8: 48000014  b 0x830021fc
	pc = 0x830021FC; continue 'dispatch;
	// 830021EC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830021F0: 816A00B0  lwz r11, 0xb0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(176 as u32) ) } as u64;
	// 830021F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830021F8: 4E800421  bctrl
	ctx.lr = 0x830021FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830021FC: 2F190003  cmpwi cr6, r25, 3
	ctx.cr[6].compare_i32(ctx.r[25].s32, 3, &mut ctx.xer);
	// 83002200: 419A001C  beq cr6, 0x8300221c
	if ctx.cr[6].eq {
	pc = 0x8300221C; continue 'dispatch;
	}
	// 83002204: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002208: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300220C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83002210: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83002214: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002218: 4E800421  bctrl
	ctx.lr = 0x8300221C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300221C: 2F190002  cmpwi cr6, r25, 2
	ctx.cr[6].compare_i32(ctx.r[25].s32, 2, &mut ctx.xer);
	// 83002220: 419A001C  beq cr6, 0x8300223c
	if ctx.cr[6].eq {
	pc = 0x8300223C; continue 'dispatch;
	}
	// 83002224: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002228: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8300222C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002230: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83002234: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002238: 4E800421  bctrl
	ctx.lr = 0x8300223C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300223C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83002240: 38211FE0  addi r1, r1, 0x1fe0
	ctx.r[1].s64 = ctx.r[1].s64 + 8160;
	// 83002244: 481A5F60  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83002248 size=488
    let mut pc: u32 = 0x83002248;
    'dispatch: loop {
        match pc {
            0x83002248 => {
    //   block [0x83002248..0x83002430)
	// 83002248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300224C: 481A5F19  bl 0x831a8164
	ctx.lr = 0x83002250;
	sub_831A8130(ctx, base);
	// 83002250: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83002254: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83002258: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8300225C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83002260: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83002264: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 83002268: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8300226C: 419A01BC  beq cr6, 0x83002428
	if ctx.cr[6].eq {
	pc = 0x83002428; continue 'dispatch;
	}
	// 83002270: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83002274: 419A01B4  beq cr6, 0x83002428
	if ctx.cr[6].eq {
	pc = 0x83002428; continue 'dispatch;
	}
	// 83002278: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300227C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002280: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83002284: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002288: 4E800421  bctrl
	ctx.lr = 0x8300228C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300228C: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83002290: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 83002294: 409A0028  bne cr6, 0x830022bc
	if !ctx.cr[6].eq {
	pc = 0x830022BC; continue 'dispatch;
	}
	// 83002298: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300229C: 80DB0024  lwz r6, 0x24(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 830022A0: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 830022A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830022A8: 4BFF7C29  bl 0x82ff9ed0
	ctx.lr = 0x830022AC;
	sub_82FF9ED0(ctx, base);
	// 830022AC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830022B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830022B4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830022B8: 481AE971  bl 0x831b0c28
	ctx.lr = 0x830022BC;
	sub_831B0C28(ctx, base);
	// 830022BC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830022C0: 419A0060  beq cr6, 0x83002320
	if ctx.cr[6].eq {
	pc = 0x83002320; continue 'dispatch;
	}
	// 830022C4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830022C8: 419A0058  beq cr6, 0x83002320
	if ctx.cr[6].eq {
	pc = 0x83002320; continue 'dispatch;
	}
	// 830022CC: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 830022D0: 419A0050  beq cr6, 0x83002320
	if ctx.cr[6].eq {
	pc = 0x83002320; continue 'dispatch;
	}
	// 830022D4: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 830022D8: 419A0048  beq cr6, 0x83002320
	if ctx.cr[6].eq {
	pc = 0x83002320; continue 'dispatch;
	}
	// 830022DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830022E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830022E4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830022E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830022EC: 4E800421  bctrl
	ctx.lr = 0x830022F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830022F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830022F4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830022F8: 419A0078  beq cr6, 0x83002370
	if ctx.cr[6].eq {
	pc = 0x83002370; continue 'dispatch;
	}
	// 830022FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002300: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002304: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002308: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300230C: 4E800421  bctrl
	ctx.lr = 0x83002310;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002310: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83002314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83002318: 4082FFE4  bne 0x830022fc
	if !ctx.cr[0].eq {
	pc = 0x830022FC; continue 'dispatch;
	}
	// 8300231C: 48000054  b 0x83002370
	pc = 0x83002370; continue 'dispatch;
	// 83002320: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83002324: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002328: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 8300232C: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83002330: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83002334: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83002338: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8300233C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83002340: 41820028  beq 0x83002368
	if ctx.cr[0].eq {
	pc = 0x83002368; continue 'dispatch;
	}
	// 83002344: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83002348: 80DB0024  lwz r6, 0x24(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300234C: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83002350: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83002354: 4BFF7B7D  bl 0x82ff9ed0
	ctx.lr = 0x83002358;
	sub_82FF9ED0(ctx, base);
	// 83002358: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300235C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83002360: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83002364: 481AE8C5  bl 0x831b0c28
	ctx.lr = 0x83002368;
	sub_831B0C28(ctx, base);
	// 83002368: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8300236C: 419A00BC  beq cr6, 0x83002428
	if ctx.cr[6].eq {
	pc = 0x83002428; continue 'dispatch;
	}
	// 83002370: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002374: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83002378: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300237C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002380: 4E800421  bctrl
	ctx.lr = 0x83002384;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002384: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83002388: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 8300238C: 409A0028  bne cr6, 0x830023b4
	if !ctx.cr[6].eq {
	pc = 0x830023B4; continue 'dispatch;
	}
	// 83002390: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83002394: 80DB0024  lwz r6, 0x24(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002398: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8300239C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830023A0: 4BFF7B31  bl 0x82ff9ed0
	ctx.lr = 0x830023A4;
	sub_82FF9ED0(ctx, base);
	// 830023A4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830023A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830023AC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830023B0: 481AE879  bl 0x831b0c28
	ctx.lr = 0x830023B4;
	sub_831B0C28(ctx, base);
	// 830023B4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830023B8: 419A005C  beq cr6, 0x83002414
	if ctx.cr[6].eq {
	pc = 0x83002414; continue 'dispatch;
	}
	// 830023BC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830023C0: 419A0054  beq cr6, 0x83002414
	if ctx.cr[6].eq {
	pc = 0x83002414; continue 'dispatch;
	}
	// 830023C4: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 830023C8: 419A004C  beq cr6, 0x83002414
	if ctx.cr[6].eq {
	pc = 0x83002414; continue 'dispatch;
	}
	// 830023CC: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 830023D0: 419A0044  beq cr6, 0x83002414
	if ctx.cr[6].eq {
	pc = 0x83002414; continue 'dispatch;
	}
	// 830023D4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830023D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830023DC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830023E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830023E4: 4E800421  bctrl
	ctx.lr = 0x830023E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830023E8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830023EC: 419A002C  beq cr6, 0x83002418
	if ctx.cr[6].eq {
	pc = 0x83002418; continue 'dispatch;
	}
	// 830023F0: 37DCFFFF  addic. r30, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830023F4: 41820024  beq 0x83002418
	if ctx.cr[0].eq {
	pc = 0x83002418; continue 'dispatch;
	}
	// 830023F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830023FC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002400: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002404: 4E800421  bctrl
	ctx.lr = 0x83002408;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002408: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8300240C: 4082FFEC  bne 0x830023f8
	if !ctx.cr[0].eq {
	pc = 0x830023F8; continue 'dispatch;
	}
	// 83002410: 48000008  b 0x83002418
	pc = 0x83002418; continue 'dispatch;
	// 83002414: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83002418: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8300241C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83002420: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83002424: 4BFFE41D  bl 0x83000840
	ctx.lr = 0x83002428;
	sub_83000840(ctx, base);
	// 83002428: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8300242C: 481A5D88  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83002430 size=8
    let mut pc: u32 = 0x83002430;
    'dispatch: loop {
        match pc {
            0x83002430 => {
    //   block [0x83002430..0x83002438)
	// 83002430: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83002434: 82140590  lwz r16, 0x590(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(1424 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83002438 size=380
    let mut pc: u32 = 0x83002438;
    'dispatch: loop {
        match pc {
            0x83002438 => {
    //   block [0x83002438..0x830025B4)
	// 83002438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300243C: 481A5D29  bl 0x831a8164
	ctx.lr = 0x83002440;
	sub_831A8130(ctx, base);
	// 83002440: 3BE1FF20  addi r31, r1, -0xe0
	ctx.r[31].s64 = ctx.r[1].s64 + -224;
	// 83002444: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83002448: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300244C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83002450: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83002454: 897D001C  lbz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 83002458: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300245C: 41820028  beq 0x83002484
	if ctx.cr[0].eq {
	pc = 0x83002484; continue 'dispatch;
	}
	// 83002460: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83002464: 80DD0024  lwz r6, 0x24(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002468: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 8300246C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83002470: 4BFF7A61  bl 0x82ff9ed0
	ctx.lr = 0x83002474;
	sub_82FF9ED0(ctx, base);
	// 83002474: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83002478: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8300247C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83002480: 481AE7A9  bl 0x831b0c28
	ctx.lr = 0x83002484;
	sub_831B0C28(ctx, base);
	// 83002484: 7F1ED840  cmplw cr6, r30, r27
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[27].u32, &mut ctx.xer);
	// 83002488: 409A000C  bne cr6, 0x83002494
	if !ctx.cr[6].eq {
	pc = 0x83002494; continue 'dispatch;
	}
	// 8300248C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83002490: 4800011C  b 0x830025ac
	pc = 0x830025AC; continue 'dispatch;
	// 83002494: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83002498: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300249C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830024A0: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 830024A4: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 830024A8: 4804A349  bl 0x8304c7f0
	ctx.lr = 0x830024AC;
	sub_8304C7F0(ctx, base);
	// 830024AC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830024B0: 3B8B2990  addi r28, r11, 0x2990
	ctx.r[28].s64 = ctx.r[11].s64 + 10640;
	// 830024B4: 939F0070  stw r28, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[28].u32 ) };
	// 830024B8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830024BC: 419A002C  beq cr6, 0x830024e8
	if ctx.cr[6].eq {
	pc = 0x830024E8; continue 'dispatch;
	}
	// 830024C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830024C4: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 830024C8: 48038C89  bl 0x8303b150
	ctx.lr = 0x830024CC;
	sub_8303B150(ctx, base);
	// 830024CC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830024D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830024D4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830024D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830024DC: 4E800421  bctrl
	ctx.lr = 0x830024E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830024E0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830024E4: 4082FFDC  bne 0x830024c0
	if !ctx.cr[0].eq {
	pc = 0x830024C0; continue 'dispatch;
	}
	// 830024E8: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830024EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830024F0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830024F4: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 830024F8: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 830024FC: 4804A2F5  bl 0x8304c7f0
	ctx.lr = 0x83002500;
	sub_8304C7F0(ctx, base);
	// 83002500: 939F0090  stw r28, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[28].u32 ) };
	// 83002504: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 83002508: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8300250C: 419A002C  beq cr6, 0x83002538
	if ctx.cr[6].eq {
	pc = 0x83002538; continue 'dispatch;
	}
	// 83002510: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83002514: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83002518: 48038C39  bl 0x8303b150
	ctx.lr = 0x8300251C;
	sub_8303B150(ctx, base);
	// 8300251C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002520: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83002524: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83002528: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300252C: 4E800421  bctrl
	ctx.lr = 0x83002530;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002530: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83002534: 4082FFDC  bne 0x83002510
	if !ctx.cr[0].eq {
	pc = 0x83002510; continue 'dispatch;
	}
	// 83002538: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 8300253C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83002540: 37CBFFFF  addic. r30, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83002544: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 83002548: 3BABFFFF  addi r29, r11, -1
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	// 8300254C: 4180004C  blt 0x83002598
	if ctx.cr[0].lt {
	pc = 0x83002598; continue 'dispatch;
	}
	// 83002550: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83002554: 41980044  blt cr6, 0x83002598
	if ctx.cr[6].lt {
	pc = 0x83002598; continue 'dispatch;
	}
	// 83002558: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300255C: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83002560: 4802A311  bl 0x8302c870
	ctx.lr = 0x83002564;
	sub_8302C870(ctx, base);
	// 83002564: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83002568: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300256C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83002570: 4802A301  bl 0x8302c870
	ctx.lr = 0x83002574;
	sub_8302C870(ctx, base);
	// 83002574: 7F03D840  cmplw cr6, r3, r27
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[27].u32, &mut ctx.xer);
	// 83002578: 409A0020  bne cr6, 0x83002598
	if !ctx.cr[6].eq {
	pc = 0x83002598; continue 'dispatch;
	}
	// 8300257C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83002580: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83002584: 4802A2ED  bl 0x8302c870
	ctx.lr = 0x83002588;
	sub_8302C870(ctx, base);
	// 83002588: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8300258C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83002590: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 83002594: 4080FFBC  bge 0x83002550
	if !ctx.cr[0].lt {
	pc = 0x83002550; continue 'dispatch;
	}
	// 83002598: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 8300259C: 4804A855  bl 0x8304cdf0
	ctx.lr = 0x830025A0;
	sub_8304CDF0(ctx, base);
	// 830025A0: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 830025A4: 4804A84D  bl 0x8304cdf0
	ctx.lr = 0x830025A8;
	sub_8304CDF0(ctx, base);
	// 830025A8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830025AC: 383F00E0  addi r1, r31, 0xe0
	ctx.r[1].s64 = ctx.r[31].s64 + 224;
	// 830025B0: 481A5C04  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830025B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830025B4 size=40
    let mut pc: u32 = 0x830025B4;
    'dispatch: loop {
        match pc {
            0x830025B4 => {
    //   block [0x830025B4..0x830025DC)
	// 830025B4: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 830025B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830025BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830025C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830025C4: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 830025C8: 4804A829  bl 0x8304cdf0
	ctx.lr = 0x830025CC;
	sub_8304CDF0(ctx, base);
	// 830025CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830025D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830025D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830025D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830025DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830025DC size=40
    let mut pc: u32 = 0x830025DC;
    'dispatch: loop {
        match pc {
            0x830025DC => {
    //   block [0x830025DC..0x83002604)
	// 830025DC: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 830025E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830025E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830025E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830025EC: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 830025F0: 4804A801  bl 0x8304cdf0
	ctx.lr = 0x830025F4;
	sub_8304CDF0(ctx, base);
	// 830025F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830025F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830025FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83002600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83002608 size=8
    let mut pc: u32 = 0x83002608;
    'dispatch: loop {
        match pc {
            0x83002608 => {
    //   block [0x83002608..0x83002610)
	// 83002608: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300260C: 821405E8  lwz r16, 0x5e8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(1512 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83002610 size=1140
    let mut pc: u32 = 0x83002610;
    'dispatch: loop {
        match pc {
            0x83002610 => {
    //   block [0x83002610..0x83002A84)
	// 83002610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83002614: 481A5B3D  bl 0x831a8150
	ctx.lr = 0x83002618;
	sub_831A8130(ctx, base);
	// 83002618: 3BE1E000  addi r31, r1, -0x2000
	ctx.r[31].s64 = ctx.r[1].s64 + -8192;
	// 8300261C: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 83002620: 9421E000  stwu r1, -0x2000(r1)
	ea = ctx.r[1].u32.wrapping_add(-8192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83002624: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 83002628: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300262C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83002630: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 83002634: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83002638: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300263C: 839E0024  lwz r28, 0x24(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002640: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002644: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002648: 4E800421  bctrl
	ctx.lr = 0x8300264C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300264C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83002650: 4BFCE531  bl 0x82fd0b80
	ctx.lr = 0x83002654;
	sub_82FD0B80(ctx, base);
	// 83002654: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002658: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 8300265C: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83002660: 92DF0050  stw r22, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[22].u32 ) };
	// 83002664: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83002668: 41820218  beq 0x83002880
	if ctx.cr[0].eq {
	pc = 0x83002880; continue 'dispatch;
	}
	// 8300266C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002670: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002674: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002678: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300267C: 4E800421  bctrl
	ctx.lr = 0x83002680;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002680: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83002684: 41820034  beq 0x830026b8
	if ctx.cr[0].eq {
	pc = 0x830026B8; continue 'dispatch;
	}
	// 83002688: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300268C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83002690: 41820028  beq 0x830026b8
	if ctx.cr[0].eq {
	pc = 0x830026B8; continue 'dispatch;
	}
	// 83002694: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 83002698: 48000008  b 0x830026a0
	pc = 0x830026A0; continue 'dispatch;
	// 8300269C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830026A0: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830026A4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830026A8: 4082FFF4  bne 0x8300269c
	if !ctx.cr[0].eq {
	pc = 0x8300269C; continue 'dispatch;
	}
	// 830026AC: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 830026B0: 7D790E70  srawi r25, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[25].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830026B4: 48000008  b 0x830026bc
	pc = 0x830026BC; continue 'dispatch;
	// 830026B8: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 830026BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830026C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830026C4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830026C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830026CC: 4E800421  bctrl
	ctx.lr = 0x830026D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830026D0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830026D4: 3B0B8158  addi r24, r11, -0x7ea8
	ctx.r[24].s64 = ctx.r[11].s64 + -32424;
	// 830026D8: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830026DC: 40820020  bne 0x830026fc
	if !ctx.cr[0].eq {
	pc = 0x830026FC; continue 'dispatch;
	}
	// 830026E0: 2F170002  cmpwi cr6, r23, 2
	ctx.cr[6].compare_i32(ctx.r[23].s32, 2, &mut ctx.xer);
	// 830026E4: 419A00B0  beq cr6, 0x83002794
	if ctx.cr[6].eq {
	pc = 0x83002794; continue 'dispatch;
	}
	// 830026E8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830026EC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830026F0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830026F4: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 830026F8: 48000094  b 0x8300278c
	pc = 0x8300278C; continue 'dispatch;
	// 830026FC: 2F1D0F9F  cmpwi cr6, r29, 0xf9f
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3999, &mut ctx.xer);
	// 83002700: 41980028  blt cr6, 0x83002728
	if ctx.cr[6].lt {
	pc = 0x83002728; continue 'dispatch;
	}
	// 83002704: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002708: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 8300270C: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83002710: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002714: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002718: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300271C: 4E800421  bctrl
	ctx.lr = 0x83002720;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002720: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83002724: 48000008  b 0x8300272c
	pc = 0x8300272C; continue 'dispatch;
	// 83002728: 3B9F0060  addi r28, r31, 0x60
	ctx.r[28].s64 = ctx.r[31].s64 + 96;
	// 8300272C: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83002730: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83002734: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83002738: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8300273C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83002740: 80EB0090  lwz r7, 0x90(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 83002744: 4BFCF945  bl 0x82fd2088
	ctx.lr = 0x83002748;
	sub_82FD2088(ctx, base);
	// 83002748: 2F170002  cmpwi cr6, r23, 2
	ctx.cr[6].compare_i32(ctx.r[23].s32, 2, &mut ctx.xer);
	// 8300274C: 419A0028  beq cr6, 0x83002774
	if ctx.cr[6].eq {
	pc = 0x83002774; continue 'dispatch;
	}
	// 83002750: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83002754: 837A0000  lwz r27, 0(r26)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002758: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8300275C: 4BFDF27D  bl 0x82fe19d8
	ctx.lr = 0x83002760;
	sub_82FE19D8(ctx, base);
	// 83002760: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83002764: 817B0048  lwz r11, 0x48(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 83002768: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8300276C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002770: 4E800421  bctrl
	ctx.lr = 0x83002774;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002774: 2F1D0F9F  cmpwi cr6, r29, 0xf9f
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3999, &mut ctx.xer);
	// 83002778: 4198001C  blt cr6, 0x83002794
	if ctx.cr[6].lt {
	pc = 0x83002794; continue 'dispatch;
	}
	// 8300277C: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002780: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83002784: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002788: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300278C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002790: 4E800421  bctrl
	ctx.lr = 0x83002794;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002794: 2F170003  cmpwi cr6, r23, 3
	ctx.cr[6].compare_i32(ctx.r[23].s32, 3, &mut ctx.xer);
	// 83002798: 409A0020  bne cr6, 0x830027b8
	if !ctx.cr[6].eq {
	pc = 0x830027B8; continue 'dispatch;
	}
	// 8300279C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 830027A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830027A4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830027A8: 4BFD0319  bl 0x82fd2ac0
	ctx.lr = 0x830027AC;
	sub_82FD2AC0(ctx, base);
	// 830027AC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830027B0: 383F2000  addi r1, r31, 0x2000
	ctx.r[1].s64 = ctx.r[31].s64 + 8192;
	// 830027B4: 481A59EC  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
	// 830027B8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830027BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830027C0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830027C4: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830027C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830027CC: 4E800421  bctrl
	ctx.lr = 0x830027D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830027D0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830027D4: 7F19E800  cmpw cr6, r25, r29
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[29].s32, &mut ctx.xer);
	// 830027D8: 409A0014  bne cr6, 0x830027ec
	if !ctx.cr[6].eq {
	pc = 0x830027EC; continue 'dispatch;
	}
	// 830027DC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830027E0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830027E4: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 830027E8: 4800008C  b 0x83002874
	pc = 0x83002874; continue 'dispatch;
	// 830027EC: 2F1D0F9F  cmpwi cr6, r29, 0xf9f
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3999, &mut ctx.xer);
	// 830027F0: 41980028  blt cr6, 0x83002818
	if ctx.cr[6].lt {
	pc = 0x83002818; continue 'dispatch;
	}
	// 830027F4: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830027F8: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 830027FC: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83002800: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002804: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002808: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300280C: 4E800421  bctrl
	ctx.lr = 0x83002810;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002810: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83002814: 48000008  b 0x8300281c
	pc = 0x8300281C; continue 'dispatch;
	// 83002818: 3B9F0060  addi r28, r31, 0x60
	ctx.r[28].s64 = ctx.r[31].s64 + 96;
	// 8300281C: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83002820: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 83002824: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83002828: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8300282C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83002830: 80EB0090  lwz r7, 0x90(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 83002834: 4BFCF855  bl 0x82fd2088
	ctx.lr = 0x83002838;
	sub_82FD2088(ctx, base);
	// 83002838: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300283C: 835B0000  lwz r26, 0(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002840: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83002844: 4BFDF195  bl 0x82fe19d8
	ctx.lr = 0x83002848;
	sub_82FE19D8(ctx, base);
	// 83002848: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300284C: 817A0048  lwz r11, 0x48(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(72 as u32) ) } as u64;
	// 83002850: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83002854: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002858: 4E800421  bctrl
	ctx.lr = 0x8300285C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300285C: 2F1D0F9F  cmpwi cr6, r29, 0xf9f
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3999, &mut ctx.xer);
	// 83002860: 4198001C  blt cr6, 0x8300287c
	if ctx.cr[6].lt {
	pc = 0x8300287C; continue 'dispatch;
	}
	// 83002864: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002868: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300286C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002870: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002874: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002878: 4E800421  bctrl
	ctx.lr = 0x8300287C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300287C: 4BFFFF24  b 0x830027a0
	pc = 0x830027A0; continue 'dispatch;
	// 83002880: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83002884: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002888: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300288C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002890: 4E800421  bctrl
	ctx.lr = 0x83002894;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002894: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83002898: 41820034  beq 0x830028cc
	if ctx.cr[0].eq {
	pc = 0x830028CC; continue 'dispatch;
	}
	// 8300289C: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830028A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830028A4: 41820028  beq 0x830028cc
	if ctx.cr[0].eq {
	pc = 0x830028CC; continue 'dispatch;
	}
	// 830028A8: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 830028AC: 48000008  b 0x830028b4
	pc = 0x830028B4; continue 'dispatch;
	// 830028B0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830028B4: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830028B8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830028BC: 4082FFF4  bne 0x830028b0
	if !ctx.cr[0].eq {
	pc = 0x830028B0; continue 'dispatch;
	}
	// 830028C0: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 830028C4: 7D7B0E70  srawi r27, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830028C8: 48000008  b 0x830028d0
	pc = 0x830028D0; continue 'dispatch;
	// 830028CC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 830028D0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830028D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830028D8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830028DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830028E0: 4E800421  bctrl
	ctx.lr = 0x830028E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830028E4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830028E8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830028EC: 7F1BE800  cmpw cr6, r27, r29
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[29].s32, &mut ctx.xer);
	// 830028F0: 3B2B8158  addi r25, r11, -0x7ea8
	ctx.r[25].s64 = ctx.r[11].s64 + -32424;
	// 830028F4: 409A0020  bne cr6, 0x83002914
	if !ctx.cr[6].eq {
	pc = 0x83002914; continue 'dispatch;
	}
	// 830028F8: 2F170002  cmpwi cr6, r23, 2
	ctx.cr[6].compare_i32(ctx.r[23].s32, 2, &mut ctx.xer);
	// 830028FC: 419A00B0  beq cr6, 0x830029ac
	if ctx.cr[6].eq {
	pc = 0x830029AC; continue 'dispatch;
	}
	// 83002900: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002904: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83002908: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8300290C: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 83002910: 48000094  b 0x830029a4
	pc = 0x830029A4; continue 'dispatch;
	// 83002914: 2F1D0F9F  cmpwi cr6, r29, 0xf9f
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3999, &mut ctx.xer);
	// 83002918: 41980028  blt cr6, 0x83002940
	if ctx.cr[6].lt {
	pc = 0x83002940; continue 'dispatch;
	}
	// 8300291C: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002920: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 83002924: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83002928: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300292C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002930: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002934: 4E800421  bctrl
	ctx.lr = 0x83002938;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002938: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8300293C: 48000008  b 0x83002944
	pc = 0x83002944; continue 'dispatch;
	// 83002940: 3B9F0060  addi r28, r31, 0x60
	ctx.r[28].s64 = ctx.r[31].s64 + 96;
	// 83002944: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83002948: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8300294C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83002950: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 83002954: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83002958: 80EB0090  lwz r7, 0x90(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300295C: 4BFCF72D  bl 0x82fd2088
	ctx.lr = 0x83002960;
	sub_82FD2088(ctx, base);
	// 83002960: 2F170002  cmpwi cr6, r23, 2
	ctx.cr[6].compare_i32(ctx.r[23].s32, 2, &mut ctx.xer);
	// 83002964: 419A0028  beq cr6, 0x8300298c
	if ctx.cr[6].eq {
	pc = 0x8300298C; continue 'dispatch;
	}
	// 83002968: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300296C: 837A0000  lwz r27, 0(r26)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002970: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83002974: 4BFDF065  bl 0x82fe19d8
	ctx.lr = 0x83002978;
	sub_82FE19D8(ctx, base);
	// 83002978: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300297C: 817B0048  lwz r11, 0x48(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 83002980: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83002984: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002988: 4E800421  bctrl
	ctx.lr = 0x8300298C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300298C: 2F1D0F9F  cmpwi cr6, r29, 0xf9f
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3999, &mut ctx.xer);
	// 83002990: 4198001C  blt cr6, 0x830029ac
	if ctx.cr[6].lt {
	pc = 0x830029AC; continue 'dispatch;
	}
	// 83002994: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002998: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300299C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830029A0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830029A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830029A8: 4E800421  bctrl
	ctx.lr = 0x830029AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830029AC: 2F170003  cmpwi cr6, r23, 3
	ctx.cr[6].compare_i32(ctx.r[23].s32, 3, &mut ctx.xer);
	// 830029B0: 409A0008  bne cr6, 0x830029b8
	if !ctx.cr[6].eq {
	pc = 0x830029B8; continue 'dispatch;
	}
	// 830029B4: 4BFFFDE8  b 0x8300279c
	pc = 0x8300279C; continue 'dispatch;
	// 830029B8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830029BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830029C0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830029C4: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830029C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830029CC: 4E800421  bctrl
	ctx.lr = 0x830029D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830029D0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830029D4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830029D8: 409A0014  bne cr6, 0x830029ec
	if !ctx.cr[6].eq {
	pc = 0x830029EC; continue 'dispatch;
	}
	// 830029DC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830029E0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830029E4: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 830029E8: 4800008C  b 0x83002a74
	pc = 0x83002A74; continue 'dispatch;
	// 830029EC: 2F1D0F9F  cmpwi cr6, r29, 0xf9f
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3999, &mut ctx.xer);
	// 830029F0: 41980028  blt cr6, 0x83002a18
	if ctx.cr[6].lt {
	pc = 0x83002A18; continue 'dispatch;
	}
	// 830029F4: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830029F8: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 830029FC: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83002A00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002A04: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002A08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002A0C: 4E800421  bctrl
	ctx.lr = 0x83002A10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002A10: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83002A14: 48000008  b 0x83002a1c
	pc = 0x83002A1C; continue 'dispatch;
	// 83002A18: 3B9F0060  addi r28, r31, 0x60
	ctx.r[28].s64 = ctx.r[31].s64 + 96;
	// 83002A1C: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83002A20: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83002A24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83002A28: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 83002A2C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83002A30: 80EB0090  lwz r7, 0x90(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 83002A34: 4BFCF655  bl 0x82fd2088
	ctx.lr = 0x83002A38;
	sub_82FD2088(ctx, base);
	// 83002A38: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83002A3C: 837A0000  lwz r27, 0(r26)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002A40: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83002A44: 4BFDEF95  bl 0x82fe19d8
	ctx.lr = 0x83002A48;
	sub_82FE19D8(ctx, base);
	// 83002A48: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83002A4C: 817B0048  lwz r11, 0x48(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 83002A50: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83002A54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002A58: 4E800421  bctrl
	ctx.lr = 0x83002A5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002A5C: 2F1D0F9F  cmpwi cr6, r29, 0xf9f
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3999, &mut ctx.xer);
	// 83002A60: 4198001C  blt cr6, 0x83002a7c
	if ctx.cr[6].lt {
	pc = 0x83002A7C; continue 'dispatch;
	}
	// 83002A64: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002A68: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83002A6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002A70: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83002A74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002A78: 4E800421  bctrl
	ctx.lr = 0x83002A7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002A7C: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 83002A80: 4BFFFD20  b 0x830027a0
	pc = 0x830027A0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002A84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83002A84 size=40
    let mut pc: u32 = 0x83002A84;
    'dispatch: loop {
        match pc {
            0x83002A84 => {
    //   block [0x83002A84..0x83002AAC)
	// 83002A84: 3BECE000  addi r31, r12, -0x2000
	ctx.r[31].s64 = ctx.r[12].s64 + -8192;
	// 83002A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83002A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83002A90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83002A94: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83002A98: 4BFD03C1  bl 0x82fd2e58
	ctx.lr = 0x83002A9C;
	sub_82FD2E58(ctx, base);
	// 83002A9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83002AA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83002AA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83002AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83002AB0 size=308
    let mut pc: u32 = 0x83002AB0;
    'dispatch: loop {
        match pc {
            0x83002AB0 => {
    //   block [0x83002AB0..0x83002BE4)
	// 83002AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83002AB4: 481A56B5  bl 0x831a8168
	ctx.lr = 0x83002AB8;
	sub_831A8130(ctx, base);
	// 83002AB8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83002ABC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83002AC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83002AC4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83002AC8: 4BFFD9A1  bl 0x83000468
	ctx.lr = 0x83002ACC;
	sub_83000468(ctx, base);
	// 83002ACC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83002AD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83002AD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002AD8: 4BFFDA21  bl 0x830004f8
	ctx.lr = 0x83002ADC;
	sub_830004F8(ctx, base);
	// 83002ADC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002AE0: 83BF0018  lwz r29, 0x18(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83002AE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83002AE8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83002AEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002AF0: 4E800421  bctrl
	ctx.lr = 0x83002AF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002AF4: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83002AF8: 419A0060  beq cr6, 0x83002b58
	if ctx.cr[6].eq {
	pc = 0x83002B58; continue 'dispatch;
	}
	// 83002AFC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83002B00: 397D000C  addi r11, r29, 0xc
	ctx.r[11].s64 = ctx.r[29].s64 + 12;
	// 83002B04: 409A0008  bne cr6, 0x83002b0c
	if !ctx.cr[6].eq {
	pc = 0x83002B0C; continue 'dispatch;
	}
	// 83002B08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83002B0C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83002B10: 419A0048  beq cr6, 0x83002b58
	if ctx.cr[6].eq {
	pc = 0x83002B58; continue 'dispatch;
	}
	// 83002B14: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002B18: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83002B1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002B20: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83002B24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002B28: 4E800421  bctrl
	ctx.lr = 0x83002B2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002B2C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83002B30: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83002B34: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002B38: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83002B3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83002B40: 997F0014  stb r11, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83002B44: 4BFF738D  bl 0x82ff9ed0
	ctx.lr = 0x83002B48;
	sub_82FF9ED0(ctx, base);
	// 83002B48: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83002B4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83002B50: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83002B54: 481AE0D5  bl 0x831b0c28
	ctx.lr = 0x83002B58;
	sub_831B0C28(ctx, base);
	// 83002B58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83002B5C: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83002B60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002B64: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 83002B68: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 83002B6C: 4BFFF8CD  bl 0x83002438
	ctx.lr = 0x83002B70;
	sub_83002438(ctx, base);
	// 83002B70: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83002B74: 4082001C  bne 0x83002b90
	if !ctx.cr[0].eq {
	pc = 0x83002B90; continue 'dispatch;
	}
	// 83002B78: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002B7C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83002B80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002B84: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83002B88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002B8C: 4E800421  bctrl
	ctx.lr = 0x83002B90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002B90: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002B94: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83002B98: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83002B9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002BA0: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83002BA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002BA8: 4E800421  bctrl
	ctx.lr = 0x83002BAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002BAC: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83002BB0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83002BB4: 409A0020  bne cr6, 0x83002bd4
	if !ctx.cr[6].eq {
	pc = 0x83002BD4; continue 'dispatch;
	}
	// 83002BB8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002BBC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83002BC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002BC4: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83002BC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002BCC: 4E800421  bctrl
	ctx.lr = 0x83002BD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002BD0: 4800000C  b 0x83002bdc
	pc = 0x83002BDC; continue 'dispatch;
	// 83002BD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83002BD8: 997F0014  stb r11, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83002BDC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83002BE0: 481A55D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83002BE8 size=308
    let mut pc: u32 = 0x83002BE8;
    'dispatch: loop {
        match pc {
            0x83002BE8 => {
    //   block [0x83002BE8..0x83002D1C)
	// 83002BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83002BEC: 481A557D  bl 0x831a8168
	ctx.lr = 0x83002BF0;
	sub_831A8130(ctx, base);
	// 83002BF0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83002BF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83002BF8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83002BFC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83002C00: 4BFFD869  bl 0x83000468
	ctx.lr = 0x83002C04;
	sub_83000468(ctx, base);
	// 83002C04: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83002C08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83002C0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002C10: 4BFFD8E9  bl 0x830004f8
	ctx.lr = 0x83002C14;
	sub_830004F8(ctx, base);
	// 83002C14: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002C18: 83BF0018  lwz r29, 0x18(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83002C1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83002C20: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83002C24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002C28: 4E800421  bctrl
	ctx.lr = 0x83002C2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002C2C: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83002C30: 419A0060  beq cr6, 0x83002c90
	if ctx.cr[6].eq {
	pc = 0x83002C90; continue 'dispatch;
	}
	// 83002C34: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83002C38: 397D000C  addi r11, r29, 0xc
	ctx.r[11].s64 = ctx.r[29].s64 + 12;
	// 83002C3C: 409A0008  bne cr6, 0x83002c44
	if !ctx.cr[6].eq {
	pc = 0x83002C44; continue 'dispatch;
	}
	// 83002C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83002C44: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83002C48: 419A0048  beq cr6, 0x83002c90
	if ctx.cr[6].eq {
	pc = 0x83002C90; continue 'dispatch;
	}
	// 83002C4C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002C50: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83002C54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002C58: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83002C5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002C60: 4E800421  bctrl
	ctx.lr = 0x83002C64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002C64: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83002C68: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83002C6C: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002C70: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83002C74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83002C78: 997F0014  stb r11, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83002C7C: 4BFF7255  bl 0x82ff9ed0
	ctx.lr = 0x83002C80;
	sub_82FF9ED0(ctx, base);
	// 83002C80: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83002C84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83002C88: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83002C8C: 481ADF9D  bl 0x831b0c28
	ctx.lr = 0x83002C90;
	sub_831B0C28(ctx, base);
	// 83002C90: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83002C94: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83002C98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002C9C: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83002CA0: 939F0010  stw r28, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 83002CA4: 4BFFF795  bl 0x83002438
	ctx.lr = 0x83002CA8;
	sub_83002438(ctx, base);
	// 83002CA8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83002CAC: 4082001C  bne 0x83002cc8
	if !ctx.cr[0].eq {
	pc = 0x83002CC8; continue 'dispatch;
	}
	// 83002CB0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002CB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83002CB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002CBC: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83002CC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002CC4: 4E800421  bctrl
	ctx.lr = 0x83002CC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002CC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002CCC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83002CD0: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83002CD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002CD8: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83002CDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002CE0: 4E800421  bctrl
	ctx.lr = 0x83002CE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002CE4: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83002CE8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83002CEC: 409A0020  bne cr6, 0x83002d0c
	if !ctx.cr[6].eq {
	pc = 0x83002D0C; continue 'dispatch;
	}
	// 83002CF0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002CF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83002CF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002CFC: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83002D00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002D04: 4E800421  bctrl
	ctx.lr = 0x83002D08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002D08: 4800000C  b 0x83002d14
	pc = 0x83002D14; continue 'dispatch;
	// 83002D0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83002D10: 997F0014  stb r11, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83002D14: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83002D18: 481A54A0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83002D20 size=488
    let mut pc: u32 = 0x83002D20;
    'dispatch: loop {
        match pc {
            0x83002D20 => {
    //   block [0x83002D20..0x83002F08)
	// 83002D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83002D24: 481A5445  bl 0x831a8168
	ctx.lr = 0x83002D28;
	sub_831A8130(ctx, base);
	// 83002D28: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83002D2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83002D30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83002D34: 897F001C  lbz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83002D38: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83002D3C: 41820028  beq 0x83002d64
	if ctx.cr[0].eq {
	pc = 0x83002D64; continue 'dispatch;
	}
	// 83002D40: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83002D44: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002D48: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 83002D4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83002D50: 4BFF7181  bl 0x82ff9ed0
	ctx.lr = 0x83002D54;
	sub_82FF9ED0(ctx, base);
	// 83002D54: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83002D58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83002D5C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83002D60: 481ADEC9  bl 0x831b0c28
	ctx.lr = 0x83002D64;
	sub_831B0C28(ctx, base);
	// 83002D64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83002D68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002D6C: 4BFFD575  bl 0x830002e0
	ctx.lr = 0x83002D70;
	sub_830002E0(ctx, base);
	// 83002D70: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83002D74: 41820170  beq 0x83002ee4
	if ctx.cr[0].eq {
	pc = 0x83002EE4; continue 'dispatch;
	}
	// 83002D78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83002D7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002D80: 4BFFD609  bl 0x83000388
	ctx.lr = 0x83002D84;
	sub_83000388(ctx, base);
	// 83002D84: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83002D88: 4182015C  beq 0x83002ee4
	if ctx.cr[0].eq {
	pc = 0x83002EE4; continue 'dispatch;
	}
	// 83002D8C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002D90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83002D94: 83BF0018  lwz r29, 0x18(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83002D98: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83002D9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002DA0: 4E800421  bctrl
	ctx.lr = 0x83002DA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002DA4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83002DA8: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83002DAC: 419A0060  beq cr6, 0x83002e0c
	if ctx.cr[6].eq {
	pc = 0x83002E0C; continue 'dispatch;
	}
	// 83002DB0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83002DB4: 397D000C  addi r11, r29, 0xc
	ctx.r[11].s64 = ctx.r[29].s64 + 12;
	// 83002DB8: 409A0008  bne cr6, 0x83002dc0
	if !ctx.cr[6].eq {
	pc = 0x83002DC0; continue 'dispatch;
	}
	// 83002DBC: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 83002DC0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83002DC4: 419A0048  beq cr6, 0x83002e0c
	if ctx.cr[6].eq {
	pc = 0x83002E0C; continue 'dispatch;
	}
	// 83002DC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002DCC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83002DD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002DD4: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83002DD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002DDC: 4E800421  bctrl
	ctx.lr = 0x83002DE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002DE0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83002DE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83002DE8: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002DEC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83002DF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83002DF4: 997F0014  stb r11, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83002DF8: 4BFF70D9  bl 0x82ff9ed0
	ctx.lr = 0x83002DFC;
	sub_82FF9ED0(ctx, base);
	// 83002DFC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83002E00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83002E04: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83002E08: 481ADE21  bl 0x831b0c28
	ctx.lr = 0x83002E0C;
	sub_831B0C28(ctx, base);
	// 83002E0C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002E10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83002E14: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83002E18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002E1C: 4E800421  bctrl
	ctx.lr = 0x83002E20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002E20: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 83002E24: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83002E28: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83002E2C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002E30: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83002E34: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83002E38: 816A0020  lwz r11, 0x20(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 83002E3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002E40: 4E800421  bctrl
	ctx.lr = 0x83002E44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002E44: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83002E48: 4082FFE4  bne 0x83002e2c
	if !ctx.cr[0].eq {
	pc = 0x83002E2C; continue 'dispatch;
	}
	// 83002E4C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83002E50: 409A000C  bne cr6, 0x83002e5c
	if !ctx.cr[6].eq {
	pc = 0x83002E5C; continue 'dispatch;
	}
	// 83002E54: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 83002E58: 4800000C  b 0x83002e64
	pc = 0x83002E64; continue 'dispatch;
	// 83002E5C: 397DFFFF  addi r11, r29, -1
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	// 83002E60: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83002E64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83002E68: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83002E6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002E70: 4BFFF5C9  bl 0x83002438
	ctx.lr = 0x83002E74;
	sub_83002438(ctx, base);
	// 83002E74: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83002E78: 4082001C  bne 0x83002e94
	if !ctx.cr[0].eq {
	pc = 0x83002E94; continue 'dispatch;
	}
	// 83002E7C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002E80: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83002E84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002E88: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83002E8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002E90: 4E800421  bctrl
	ctx.lr = 0x83002E94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002E94: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002E98: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83002E9C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83002EA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002EA4: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83002EA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002EAC: 4E800421  bctrl
	ctx.lr = 0x83002EB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002EB0: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83002EB4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83002EB8: 409A0024  bne cr6, 0x83002edc
	if !ctx.cr[6].eq {
	pc = 0x83002EDC; continue 'dispatch;
	}
	// 83002EBC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002EC0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83002EC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002EC8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83002ECC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002ED0: 4E800421  bctrl
	ctx.lr = 0x83002ED4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002ED4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83002ED8: 481A52E0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83002EDC: 9B9F0014  stb r28, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[28].u8 ) };
	// 83002EE0: 4BFFFFF4  b 0x83002ed4
	pc = 0x83002ED4; continue 'dispatch;
	// 83002EE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83002EE8: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002EEC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83002EF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83002EF4: 48035E95  bl 0x83038d88
	ctx.lr = 0x83002EF8;
	sub_83038D88(ctx, base);
	// 83002EF8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83002EFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83002F00: 388BC878  addi r4, r11, -0x3788
	ctx.r[4].s64 = ctx.r[11].s64 + -14216;
	// 83002F04: 481ADD25  bl 0x831b0c28
	ctx.lr = 0x83002F08;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83002F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83002F08 size=468
    let mut pc: u32 = 0x83002F08;
    'dispatch: loop {
        match pc {
            0x83002F08 => {
    //   block [0x83002F08..0x830030DC)
	// 83002F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83002F0C: 481A5261  bl 0x831a816c
	ctx.lr = 0x83002F10;
	sub_831A8130(ctx, base);
	// 83002F10: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83002F14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83002F18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83002F1C: 897F001C  lbz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83002F20: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83002F24: 41820028  beq 0x83002f4c
	if ctx.cr[0].eq {
	pc = 0x83002F4C; continue 'dispatch;
	}
	// 83002F28: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83002F2C: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002F30: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 83002F34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83002F38: 4BFF6F99  bl 0x82ff9ed0
	ctx.lr = 0x83002F3C;
	sub_82FF9ED0(ctx, base);
	// 83002F3C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83002F40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83002F44: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83002F48: 481ADCE1  bl 0x831b0c28
	ctx.lr = 0x83002F4C;
	sub_831B0C28(ctx, base);
	// 83002F4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83002F50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002F54: 4BFFD38D  bl 0x830002e0
	ctx.lr = 0x83002F58;
	sub_830002E0(ctx, base);
	// 83002F58: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83002F5C: 4182015C  beq 0x830030b8
	if ctx.cr[0].eq {
	pc = 0x830030B8; continue 'dispatch;
	}
	// 83002F60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83002F64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002F68: 4BFFD421  bl 0x83000388
	ctx.lr = 0x83002F6C;
	sub_83000388(ctx, base);
	// 83002F6C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83002F70: 41820148  beq 0x830030b8
	if ctx.cr[0].eq {
	pc = 0x830030B8; continue 'dispatch;
	}
	// 83002F74: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002F78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83002F7C: 83BF0018  lwz r29, 0x18(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83002F80: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83002F84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002F88: 4E800421  bctrl
	ctx.lr = 0x83002F8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002F8C: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83002F90: 419A0060  beq cr6, 0x83002ff0
	if ctx.cr[6].eq {
	pc = 0x83002FF0; continue 'dispatch;
	}
	// 83002F94: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83002F98: 397D000C  addi r11, r29, 0xc
	ctx.r[11].s64 = ctx.r[29].s64 + 12;
	// 83002F9C: 409A0008  bne cr6, 0x83002fa4
	if !ctx.cr[6].eq {
	pc = 0x83002FA4; continue 'dispatch;
	}
	// 83002FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83002FA4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83002FA8: 419A0048  beq cr6, 0x83002ff0
	if ctx.cr[6].eq {
	pc = 0x83002FF0; continue 'dispatch;
	}
	// 83002FAC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002FB0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83002FB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83002FB8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83002FBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83002FC0: 4E800421  bctrl
	ctx.lr = 0x83002FC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83002FC4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83002FC8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83002FCC: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83002FD0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83002FD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83002FD8: 997F0014  stb r11, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83002FDC: 4BFF6EF5  bl 0x82ff9ed0
	ctx.lr = 0x83002FE0;
	sub_82FF9ED0(ctx, base);
	// 83002FE0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83002FE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83002FE8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83002FEC: 481ADC3D  bl 0x831b0c28
	ctx.lr = 0x83002FF0;
	sub_831B0C28(ctx, base);
	// 83002FF0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83002FF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83002FF8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83002FFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003000: 4E800421  bctrl
	ctx.lr = 0x83003004;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003004: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83003008: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8300300C: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83003010: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003014: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83003018: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8300301C: 816A0020  lwz r11, 0x20(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 83003020: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003024: 4E800421  bctrl
	ctx.lr = 0x83003028;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003028: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300302C: 4082FFE4  bne 0x83003010
	if !ctx.cr[0].eq {
	pc = 0x83003010; continue 'dispatch;
	}
	// 83003030: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83003034: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83003038: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300303C: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83003040: 4BFFF3F9  bl 0x83002438
	ctx.lr = 0x83003044;
	sub_83002438(ctx, base);
	// 83003044: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83003048: 4082001C  bne 0x83003064
	if !ctx.cr[0].eq {
	pc = 0x83003064; continue 'dispatch;
	}
	// 8300304C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003050: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83003054: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003058: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8300305C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003060: 4E800421  bctrl
	ctx.lr = 0x83003064;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003064: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003068: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8300306C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83003070: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003074: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83003078: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300307C: 4E800421  bctrl
	ctx.lr = 0x83003080;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003080: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83003084: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83003088: 409A0024  bne cr6, 0x830030ac
	if !ctx.cr[6].eq {
	pc = 0x830030AC; continue 'dispatch;
	}
	// 8300308C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003090: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83003094: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003098: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8300309C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830030A0: 4E800421  bctrl
	ctx.lr = 0x830030A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830030A4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830030A8: 481A5114  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 830030AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830030B0: 997F0014  stb r11, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 830030B4: 4BFFFFF0  b 0x830030a4
	pc = 0x830030A4; continue 'dispatch;
	// 830030B8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830030BC: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830030C0: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 830030C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830030C8: 48035CC1  bl 0x83038d88
	ctx.lr = 0x830030CC;
	sub_83038D88(ctx, base);
	// 830030CC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830030D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830030D4: 388BC878  addi r4, r11, -0x3788
	ctx.r[4].s64 = ctx.r[11].s64 + -14216;
	// 830030D8: 481ADB51  bl 0x831b0c28
	ctx.lr = 0x830030DC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830030E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830030E0 size=488
    let mut pc: u32 = 0x830030E0;
    'dispatch: loop {
        match pc {
            0x830030E0 => {
    //   block [0x830030E0..0x830032C8)
	// 830030E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830030E4: 481A5085  bl 0x831a8168
	ctx.lr = 0x830030E8;
	sub_831A8130(ctx, base);
	// 830030E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830030EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830030F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830030F4: 897F001C  lbz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830030F8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830030FC: 41820028  beq 0x83003124
	if ctx.cr[0].eq {
	pc = 0x83003124; continue 'dispatch;
	}
	// 83003100: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83003104: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83003108: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 8300310C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83003110: 4BFF6DC1  bl 0x82ff9ed0
	ctx.lr = 0x83003114;
	sub_82FF9ED0(ctx, base);
	// 83003114: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83003118: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300311C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83003120: 481ADB09  bl 0x831b0c28
	ctx.lr = 0x83003124;
	sub_831B0C28(ctx, base);
	// 83003124: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83003128: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300312C: 4BFFD1B5  bl 0x830002e0
	ctx.lr = 0x83003130;
	sub_830002E0(ctx, base);
	// 83003130: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83003134: 41820170  beq 0x830032a4
	if ctx.cr[0].eq {
	pc = 0x830032A4; continue 'dispatch;
	}
	// 83003138: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300313C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003140: 4BFFD249  bl 0x83000388
	ctx.lr = 0x83003144;
	sub_83000388(ctx, base);
	// 83003144: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83003148: 4182015C  beq 0x830032a4
	if ctx.cr[0].eq {
	pc = 0x830032A4; continue 'dispatch;
	}
	// 8300314C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003150: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83003154: 83BF0018  lwz r29, 0x18(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83003158: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300315C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003160: 4E800421  bctrl
	ctx.lr = 0x83003164;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003164: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83003168: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 8300316C: 419A0060  beq cr6, 0x830031cc
	if ctx.cr[6].eq {
	pc = 0x830031CC; continue 'dispatch;
	}
	// 83003170: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83003174: 397D000C  addi r11, r29, 0xc
	ctx.r[11].s64 = ctx.r[29].s64 + 12;
	// 83003178: 409A0008  bne cr6, 0x83003180
	if !ctx.cr[6].eq {
	pc = 0x83003180; continue 'dispatch;
	}
	// 8300317C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 83003180: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83003184: 419A0048  beq cr6, 0x830031cc
	if ctx.cr[6].eq {
	pc = 0x830031CC; continue 'dispatch;
	}
	// 83003188: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300318C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83003190: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003194: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83003198: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300319C: 4E800421  bctrl
	ctx.lr = 0x830031A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830031A0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830031A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830031A8: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830031AC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 830031B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830031B4: 997F0014  stb r11, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 830031B8: 4BFF6D19  bl 0x82ff9ed0
	ctx.lr = 0x830031BC;
	sub_82FF9ED0(ctx, base);
	// 830031BC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830031C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830031C4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830031C8: 481ADA61  bl 0x831b0c28
	ctx.lr = 0x830031CC;
	sub_831B0C28(ctx, base);
	// 830031CC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830031D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830031D4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830031D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830031DC: 4E800421  bctrl
	ctx.lr = 0x830031E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830031E0: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 830031E4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830031E8: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 830031EC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830031F0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830031F4: 816A0020  lwz r11, 0x20(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 830031F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830031FC: 4E800421  bctrl
	ctx.lr = 0x83003200;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003200: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83003204: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83003208: 4082FFE4  bne 0x830031ec
	if !ctx.cr[0].eq {
	pc = 0x830031EC; continue 'dispatch;
	}
	// 8300320C: 2B1D0001  cmplwi cr6, r29, 1
	ctx.cr[6].compare_u32(ctx.r[29].u32, 1 as u32, &mut ctx.xer);
	// 83003210: 4098000C  bge cr6, 0x8300321c
	if !ctx.cr[6].lt {
	pc = 0x8300321C; continue 'dispatch;
	}
	// 83003214: 939F0010  stw r28, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 83003218: 4800000C  b 0x83003224
	pc = 0x83003224; continue 'dispatch;
	// 8300321C: 397DFFFF  addi r11, r29, -1
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	// 83003220: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83003224: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83003228: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300322C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003230: 4BFFF209  bl 0x83002438
	ctx.lr = 0x83003234;
	sub_83002438(ctx, base);
	// 83003234: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83003238: 4082001C  bne 0x83003254
	if !ctx.cr[0].eq {
	pc = 0x83003254; continue 'dispatch;
	}
	// 8300323C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003240: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83003244: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003248: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8300324C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003250: 4E800421  bctrl
	ctx.lr = 0x83003254;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003254: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003258: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8300325C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83003260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003264: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83003268: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300326C: 4E800421  bctrl
	ctx.lr = 0x83003270;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003270: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83003274: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83003278: 409A0024  bne cr6, 0x8300329c
	if !ctx.cr[6].eq {
	pc = 0x8300329C; continue 'dispatch;
	}
	// 8300327C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003280: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83003284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003288: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8300328C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003290: 4E800421  bctrl
	ctx.lr = 0x83003294;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003294: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83003298: 481A4F20  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8300329C: 9B9F0014  stb r28, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[28].u8 ) };
	// 830032A0: 4BFFFFF4  b 0x83003294
	pc = 0x83003294; continue 'dispatch;
	// 830032A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830032A8: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830032AC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 830032B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830032B4: 48035AD5  bl 0x83038d88
	ctx.lr = 0x830032B8;
	sub_83038D88(ctx, base);
	// 830032B8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830032BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830032C0: 388BC878  addi r4, r11, -0x3788
	ctx.r[4].s64 = ctx.r[11].s64 + -14216;
	// 830032C4: 481AD965  bl 0x831b0c28
	ctx.lr = 0x830032C8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830032C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830032C8 size=484
    let mut pc: u32 = 0x830032C8;
    'dispatch: loop {
        match pc {
            0x830032C8 => {
    //   block [0x830032C8..0x830034AC)
	// 830032C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830032CC: 481A4E9D  bl 0x831a8168
	ctx.lr = 0x830032D0;
	sub_831A8130(ctx, base);
	// 830032D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830032D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830032D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830032DC: 897F001C  lbz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830032E0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830032E4: 41820028  beq 0x8300330c
	if ctx.cr[0].eq {
	pc = 0x8300330C; continue 'dispatch;
	}
	// 830032E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830032EC: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830032F0: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 830032F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830032F8: 4BFF6BD9  bl 0x82ff9ed0
	ctx.lr = 0x830032FC;
	sub_82FF9ED0(ctx, base);
	// 830032FC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83003300: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83003304: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83003308: 481AD921  bl 0x831b0c28
	ctx.lr = 0x8300330C;
	sub_831B0C28(ctx, base);
	// 8300330C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83003310: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003314: 4BFFCFCD  bl 0x830002e0
	ctx.lr = 0x83003318;
	sub_830002E0(ctx, base);
	// 83003318: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300331C: 4182016C  beq 0x83003488
	if ctx.cr[0].eq {
	pc = 0x83003488; continue 'dispatch;
	}
	// 83003320: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83003324: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003328: 4BFFD061  bl 0x83000388
	ctx.lr = 0x8300332C;
	sub_83000388(ctx, base);
	// 8300332C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83003330: 41820158  beq 0x83003488
	if ctx.cr[0].eq {
	pc = 0x83003488; continue 'dispatch;
	}
	// 83003334: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003338: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300333C: 83BF0018  lwz r29, 0x18(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83003340: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83003344: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003348: 4E800421  bctrl
	ctx.lr = 0x8300334C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300334C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83003350: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83003354: 419A0060  beq cr6, 0x830033b4
	if ctx.cr[6].eq {
	pc = 0x830033B4; continue 'dispatch;
	}
	// 83003358: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8300335C: 397D000C  addi r11, r29, 0xc
	ctx.r[11].s64 = ctx.r[29].s64 + 12;
	// 83003360: 409A0008  bne cr6, 0x83003368
	if !ctx.cr[6].eq {
	pc = 0x83003368; continue 'dispatch;
	}
	// 83003364: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 83003368: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300336C: 419A0048  beq cr6, 0x830033b4
	if ctx.cr[6].eq {
	pc = 0x830033B4; continue 'dispatch;
	}
	// 83003370: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003374: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83003378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300337C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83003380: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003384: 4E800421  bctrl
	ctx.lr = 0x83003388;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003388: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8300338C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83003390: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83003394: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83003398: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300339C: 997F0014  stb r11, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 830033A0: 4BFF6B31  bl 0x82ff9ed0
	ctx.lr = 0x830033A4;
	sub_82FF9ED0(ctx, base);
	// 830033A4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830033A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830033AC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830033B0: 481AD879  bl 0x831b0c28
	ctx.lr = 0x830033B4;
	sub_831B0C28(ctx, base);
	// 830033B4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830033B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830033BC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830033C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830033C4: 4E800421  bctrl
	ctx.lr = 0x830033C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830033C8: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 830033CC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830033D0: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 830033D4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830033D8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830033DC: 816A0020  lwz r11, 0x20(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 830033E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830033E4: 4E800421  bctrl
	ctx.lr = 0x830033E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830033E8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830033EC: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830033F0: 4082FFE4  bne 0x830033d4
	if !ctx.cr[0].eq {
	pc = 0x830033D4; continue 'dispatch;
	}
	// 830033F4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830033F8: 409A000C  bne cr6, 0x83003404
	if !ctx.cr[6].eq {
	pc = 0x83003404; continue 'dispatch;
	}
	// 830033FC: 939F0010  stw r28, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 83003400: 48000008  b 0x83003408
	pc = 0x83003408; continue 'dispatch;
	// 83003404: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 83003408: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300340C: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83003410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003414: 4BFFF025  bl 0x83002438
	ctx.lr = 0x83003418;
	sub_83002438(ctx, base);
	// 83003418: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300341C: 4082001C  bne 0x83003438
	if !ctx.cr[0].eq {
	pc = 0x83003438; continue 'dispatch;
	}
	// 83003420: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003424: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83003428: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300342C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83003430: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003434: 4E800421  bctrl
	ctx.lr = 0x83003438;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003438: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300343C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83003440: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83003444: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003448: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8300344C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003450: 4E800421  bctrl
	ctx.lr = 0x83003454;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003454: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83003458: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8300345C: 409A0024  bne cr6, 0x83003480
	if !ctx.cr[6].eq {
	pc = 0x83003480; continue 'dispatch;
	}
	// 83003460: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003464: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83003468: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300346C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83003470: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003474: 4E800421  bctrl
	ctx.lr = 0x83003478;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003478: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8300347C: 481A4D3C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83003480: 9B9F0014  stb r28, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[28].u8 ) };
	// 83003484: 4BFFFFF4  b 0x83003478
	pc = 0x83003478; continue 'dispatch;
	// 83003488: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300348C: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83003490: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83003494: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83003498: 480358F1  bl 0x83038d88
	ctx.lr = 0x8300349C;
	sub_83038D88(ctx, base);
	// 8300349C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830034A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830034A4: 388BC878  addi r4, r11, -0x3788
	ctx.r[4].s64 = ctx.r[11].s64 + -14216;
	// 830034A8: 481AD781  bl 0x831b0c28
	ctx.lr = 0x830034AC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830034B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830034B0 size=12
    let mut pc: u32 = 0x830034B0;
    'dispatch: loop {
        match pc {
            0x830034B0 => {
    //   block [0x830034B0..0x830034BC)
	// 830034B0: 80A3000C  lwz r5, 0xc(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830034B4: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830034B8: 4BFFEF80  b 0x83002438
	sub_83002438(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830034C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830034C0 size=184
    let mut pc: u32 = 0x830034C0;
    'dispatch: loop {
        match pc {
            0x830034C0 => {
    //   block [0x830034C0..0x83003578)
	// 830034C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830034C4: 481A4CA5  bl 0x831a8168
	ctx.lr = 0x830034C8;
	sub_831A8130(ctx, base);
	// 830034C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830034CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830034D0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830034D4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830034D8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 830034DC: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830034E0: 41820010  beq 0x830034f0
	if ctx.cr[0].eq {
	pc = 0x830034F0; continue 'dispatch;
	}
	// 830034E4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830034E8: 4BFFD209  bl 0x830006f0
	ctx.lr = 0x830034EC;
	sub_830006F0(ctx, base);
	// 830034EC: 48000084  b 0x83003570
	pc = 0x83003570; continue 'dispatch;
	// 830034F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830034F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830034F8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830034FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003500: 4E800421  bctrl
	ctx.lr = 0x83003504;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003504: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83003508: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8300350C: 419A0050  beq cr6, 0x8300355c
	if ctx.cr[6].eq {
	pc = 0x8300355C; continue 'dispatch;
	}
	// 83003510: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83003514: 419A0048  beq cr6, 0x8300355c
	if ctx.cr[6].eq {
	pc = 0x8300355C; continue 'dispatch;
	}
	// 83003518: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 8300351C: 419A0040  beq cr6, 0x8300355c
	if ctx.cr[6].eq {
	pc = 0x8300355C; continue 'dispatch;
	}
	// 83003520: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83003524: 419A0038  beq cr6, 0x8300355c
	if ctx.cr[6].eq {
	pc = 0x8300355C; continue 'dispatch;
	}
	// 83003528: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8300352C: 40990028  ble cr6, 0x83003554
	if !ctx.cr[6].gt {
	pc = 0x83003554; continue 'dispatch;
	}
	// 83003530: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 83003534: 41990020  bgt cr6, 0x83003554
	if ctx.cr[6].gt {
	pc = 0x83003554; continue 'dispatch;
	}
	// 83003538: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300353C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83003540: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003544: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 83003548: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300354C: 4E800421  bctrl
	ctx.lr = 0x83003550;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003550: 48000020  b 0x83003570
	pc = 0x83003570; continue 'dispatch;
	// 83003554: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83003558: 48000018  b 0x83003570
	pc = 0x83003570; continue 'dispatch;
	// 8300355C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83003560: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83003564: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83003568: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300356C: 4BFFF0A5  bl 0x83002610
	ctx.lr = 0x83003570;
	sub_83002610(ctx, base);
	// 83003570: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83003574: 481A4C44  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83003578 size=432
    let mut pc: u32 = 0x83003578;
    'dispatch: loop {
        match pc {
            0x83003578 => {
    //   block [0x83003578..0x83003728)
	// 83003578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300357C: 481A4BDD  bl 0x831a8158
	ctx.lr = 0x83003580;
	sub_831A8130(ctx, base);
	// 83003580: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83003584: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83003588: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8300358C: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 83003590: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83003594: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83003598: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 8300359C: 4BFFD1E5  bl 0x83000780
	ctx.lr = 0x830035A0;
	sub_83000780(ctx, base);
	// 830035A0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830035A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830035A8: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 830035AC: 7F1FC040  cmplw cr6, r31, r24
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[24].u32, &mut ctx.xer);
	// 830035B0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830035B4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830035B8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830035BC: 697A0001  xori r26, r11, 1
	ctx.r[26].u64 = ctx.r[11].u64 ^ 1;
	// 830035C0: 409A0020  bne cr6, 0x830035e0
	if !ctx.cr[6].eq {
	pc = 0x830035E0; continue 'dispatch;
	}
	// 830035C4: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 830035C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830035CC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 830035D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830035D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830035D8: 4BFFFEE9  bl 0x830034c0
	ctx.lr = 0x830035DC;
	sub_830034C0(ctx, base);
	// 830035DC: 4800013C  b 0x83003718
	pc = 0x83003718; continue 'dispatch;
	// 830035E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830035E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830035E8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830035EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830035F0: 4E800421  bctrl
	ctx.lr = 0x830035F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830035F4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830035F8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 830035FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83003600: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83003604: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83003608: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300360C: 4BFFFEB5  bl 0x830034c0
	ctx.lr = 0x83003610;
	sub_830034C0(ctx, base);
	// 83003610: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 83003614: 480000F8  b 0x8300370c
	pc = 0x8300370C; continue 'dispatch;
	// 83003618: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8300361C: 419A007C  beq cr6, 0x83003698
	if ctx.cr[6].eq {
	pc = 0x83003698; continue 'dispatch;
	}
	// 83003620: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003624: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003628: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8300362C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003630: 4E800421  bctrl
	ctx.lr = 0x83003634;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003634: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83003638: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8300363C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83003640: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83003644: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83003648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300364C: 4BFFFE75  bl 0x830034c0
	ctx.lr = 0x83003650;
	sub_830034C0(ctx, base);
	// 83003650: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83003654: 2F1C0003  cmpwi cr6, r28, 3
	ctx.cr[6].compare_i32(ctx.r[28].s32, 3, &mut ctx.xer);
	// 83003658: 419A0030  beq cr6, 0x83003688
	if ctx.cr[6].eq {
	pc = 0x83003688; continue 'dispatch;
	}
	// 8300365C: 83F90000  lwz r31, 0(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003660: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83003664: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83003668: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300366C: 4E800421  bctrl
	ctx.lr = 0x83003670;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003670: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83003674: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83003678: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8300367C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83003680: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003684: 4E800421  bctrl
	ctx.lr = 0x83003688;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003688: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8300368C: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 83003690: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83003694: 409AFF8C  bne cr6, 0x83003620
	if !ctx.cr[6].eq {
	pc = 0x83003620; continue 'dispatch;
	}
	// 83003698: 7F1DC040  cmplw cr6, r29, r24
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[24].u32, &mut ctx.xer);
	// 8300369C: 419A0084  beq cr6, 0x83003720
	if ctx.cr[6].eq {
	pc = 0x83003720; continue 'dispatch;
	}
	// 830036A0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830036A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830036A8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830036AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830036B0: 4E800421  bctrl
	ctx.lr = 0x830036B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830036B4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830036B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830036BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830036C0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830036C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830036C8: 4E800421  bctrl
	ctx.lr = 0x830036CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830036CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830036D0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 830036D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830036D8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830036DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830036E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830036E4: 4BFFFDDD  bl 0x830034c0
	ctx.lr = 0x830036E8;
	sub_830034C0(ctx, base);
	// 830036E8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830036EC: 2F1C0003  cmpwi cr6, r28, 3
	ctx.cr[6].compare_i32(ctx.r[28].s32, 3, &mut ctx.xer);
	// 830036F0: 419A0018  beq cr6, 0x83003708
	if ctx.cr[6].eq {
	pc = 0x83003708; continue 'dispatch;
	}
	// 830036F4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830036F8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830036FC: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83003700: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003704: 4E800421  bctrl
	ctx.lr = 0x83003708;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003708: 7F79DB78  mr r25, r27
	ctx.r[25].u64 = ctx.r[27].u64;
	// 8300370C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83003710: 409AFF08  bne cr6, 0x83003618
	if !ctx.cr[6].eq {
	pc = 0x83003618; continue 'dispatch;
	}
	// 83003714: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83003718: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8300371C: 481A4A8C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83003720: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83003724: 4BFFFFF4  b 0x83003718
	pc = 0x83003718; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83003728 size=464
    let mut pc: u32 = 0x83003728;
    'dispatch: loop {
        match pc {
            0x83003728 => {
    //   block [0x83003728..0x830038F8)
	// 83003728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300372C: 481A4A2D  bl 0x831a8158
	ctx.lr = 0x83003730;
	sub_831A8130(ctx, base);
	// 83003730: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83003734: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83003738: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 8300373C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83003740: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003744: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83003748: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300374C: 4E800421  bctrl
	ctx.lr = 0x83003750;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003750: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003754: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83003758: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300375C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83003760: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003764: 4E800421  bctrl
	ctx.lr = 0x83003768;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003768: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300376C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83003770: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83003774: 4BFFD00D  bl 0x83000780
	ctx.lr = 0x83003778;
	sub_83000780(ctx, base);
	// 83003778: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300377C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83003780: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83003784: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83003788: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300378C: 4E800421  bctrl
	ctx.lr = 0x83003790;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003790: 7D7F1850  subf r11, r31, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 83003794: 7F1FC040  cmplw cr6, r31, r24
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[24].u32, &mut ctx.xer);
	// 83003798: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300379C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830037A0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830037A4: 697A0001  xori r26, r11, 1
	ctx.r[26].u64 = ctx.r[11].u64 ^ 1;
	// 830037A8: 409A0020  bne cr6, 0x830037c8
	if !ctx.cr[6].eq {
	pc = 0x830037C8; continue 'dispatch;
	}
	// 830037AC: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 830037B0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 830037B4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 830037B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830037BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830037C0: 4BFFFD01  bl 0x830034c0
	ctx.lr = 0x830037C4;
	sub_830034C0(ctx, base);
	// 830037C4: 48000124  b 0x830038e8
	pc = 0x830038E8; continue 'dispatch;
	// 830037C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830037CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830037D0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830037D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830037D8: 4E800421  bctrl
	ctx.lr = 0x830037DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830037DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830037E0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 830037E4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 830037E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830037EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830037F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830037F4: 4BFFFCCD  bl 0x830034c0
	ctx.lr = 0x830037F8;
	sub_830034C0(ctx, base);
	// 830037F8: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 830037FC: 480000E0  b 0x830038dc
	pc = 0x830038DC; continue 'dispatch;
	// 83003800: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83003804: 419A0064  beq cr6, 0x83003868
	if ctx.cr[6].eq {
	pc = 0x83003868; continue 'dispatch;
	}
	// 83003808: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300380C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003810: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83003814: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003818: 4E800421  bctrl
	ctx.lr = 0x8300381C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300381C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83003820: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 83003824: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83003828: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8300382C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83003830: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83003834: 4BFFFC8D  bl 0x830034c0
	ctx.lr = 0x83003838;
	sub_830034C0(ctx, base);
	// 83003838: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300383C: 2F1C0003  cmpwi cr6, r28, 3
	ctx.cr[6].compare_i32(ctx.r[28].s32, 3, &mut ctx.xer);
	// 83003840: 419A0018  beq cr6, 0x83003858
	if ctx.cr[6].eq {
	pc = 0x83003858; continue 'dispatch;
	}
	// 83003844: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003848: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8300384C: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83003850: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003854: 4E800421  bctrl
	ctx.lr = 0x83003858;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003858: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8300385C: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 83003860: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83003864: 409AFFA4  bne cr6, 0x83003808
	if !ctx.cr[6].eq {
	pc = 0x83003808; continue 'dispatch;
	}
	// 83003868: 7F1DC040  cmplw cr6, r29, r24
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[24].u32, &mut ctx.xer);
	// 8300386C: 419A0084  beq cr6, 0x830038f0
	if ctx.cr[6].eq {
	pc = 0x830038F0; continue 'dispatch;
	}
	// 83003870: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003874: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83003878: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300387C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003880: 4E800421  bctrl
	ctx.lr = 0x83003884;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003884: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003888: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300388C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83003890: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83003894: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003898: 4E800421  bctrl
	ctx.lr = 0x8300389C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300389C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830038A0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 830038A4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 830038A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830038AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830038B0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830038B4: 4BFFFC0D  bl 0x830034c0
	ctx.lr = 0x830038B8;
	sub_830034C0(ctx, base);
	// 830038B8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830038BC: 2F1C0003  cmpwi cr6, r28, 3
	ctx.cr[6].compare_i32(ctx.r[28].s32, 3, &mut ctx.xer);
	// 830038C0: 419A0018  beq cr6, 0x830038d8
	if ctx.cr[6].eq {
	pc = 0x830038D8; continue 'dispatch;
	}
	// 830038C4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830038C8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830038CC: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 830038D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830038D4: 4E800421  bctrl
	ctx.lr = 0x830038D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830038D8: 7F79DB78  mr r25, r27
	ctx.r[25].u64 = ctx.r[27].u64;
	// 830038DC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830038E0: 409AFF20  bne cr6, 0x83003800
	if !ctx.cr[6].eq {
	pc = 0x83003800; continue 'dispatch;
	}
	// 830038E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830038E8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 830038EC: 481A48BC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 830038F0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830038F4: 4BFFFFF4  b 0x830038e8
	pc = 0x830038E8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830038F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830038F8 size=336
    let mut pc: u32 = 0x830038F8;
    'dispatch: loop {
        match pc {
            0x830038F8 => {
    //   block [0x830038F8..0x83003A48)
	// 830038F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830038FC: 481A485D  bl 0x831a8158
	ctx.lr = 0x83003900;
	sub_831A8130(ctx, base);
	// 83003900: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83003904: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 83003908: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300390C: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 83003910: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83003914: 2F190003  cmpwi cr6, r25, 3
	ctx.cr[6].compare_i32(ctx.r[25].s32, 3, &mut ctx.xer);
	// 83003918: 419A001C  beq cr6, 0x83003934
	if ctx.cr[6].eq {
	pc = 0x83003934; continue 'dispatch;
	}
	// 8300391C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83003920: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003924: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83003928: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300392C: 4E800421  bctrl
	ctx.lr = 0x83003930;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003930: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83003934: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 83003938: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8300393C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003940: 4BFFFC39  bl 0x83003578
	ctx.lr = 0x83003944;
	sub_83003578(ctx, base);
	// 83003944: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83003948: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8300394C: 419A0018  beq cr6, 0x83003964
	if ctx.cr[6].eq {
	pc = 0x83003964; continue 'dispatch;
	}
	// 83003950: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003954: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83003958: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8300395C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003960: 4E800421  bctrl
	ctx.lr = 0x83003964;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003964: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83003968: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300396C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003970: 4BFFCA89  bl 0x830003f8
	ctx.lr = 0x83003974;
	sub_830003F8(ctx, base);
	// 83003974: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83003978: 7F6B1851  subf. r27, r11, r3
	ctx.r[27].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8300397C: 40810088  ble 0x83003a04
	if !ctx.cr[0].gt {
	pc = 0x83003A04; continue 'dispatch;
	}
	// 83003980: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003984: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83003988: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8300398C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003990: 4E800421  bctrl
	ctx.lr = 0x83003994;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003994: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83003998: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300399C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830039A0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830039A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830039A8: 4E800421  bctrl
	ctx.lr = 0x830039AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830039AC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830039B0: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 830039B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830039B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830039BC: 4BFFCD35  bl 0x830006f0
	ctx.lr = 0x830039C0;
	sub_830006F0(ctx, base);
	// 830039C0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830039C4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830039C8: 419A0030  beq cr6, 0x830039f8
	if ctx.cr[6].eq {
	pc = 0x830039F8; continue 'dispatch;
	}
	// 830039CC: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830039D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830039D4: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830039D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830039DC: 4E800421  bctrl
	ctx.lr = 0x830039E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830039E0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830039E4: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 830039E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830039EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830039F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830039F4: 4E800421  bctrl
	ctx.lr = 0x830039F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830039F8: 377BFFFF  addic. r27, r27, -1
	ctx.xer.ca = (ctx.r[27].u32 > (!(-1 as u32)));
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 830039FC: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 83003A00: 4181FF98  bgt 0x83003998
	if ctx.cr[0].gt {
	pc = 0x83003998; continue 'dispatch;
	}
	// 83003A04: 2F190002  cmpwi cr6, r25, 2
	ctx.cr[6].compare_i32(ctx.r[25].s32, 2, &mut ctx.xer);
	// 83003A08: 419A0034  beq cr6, 0x83003a3c
	if ctx.cr[6].eq {
	pc = 0x83003A3C; continue 'dispatch;
	}
	// 83003A0C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003A10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003A14: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83003A18: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83003A1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003A20: 4E800421  bctrl
	ctx.lr = 0x83003A24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003A24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003A28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003A2C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83003A30: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83003A34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003A38: 4E800421  bctrl
	ctx.lr = 0x83003A3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003A3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83003A40: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83003A44: 481A4764  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83003A48 size=324
    let mut pc: u32 = 0x83003A48;
    'dispatch: loop {
        match pc {
            0x83003A48 => {
    //   block [0x83003A48..0x83003B8C)
	// 83003A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83003A4C: 481A4711  bl 0x831a815c
	ctx.lr = 0x83003A50;
	sub_831A8130(ctx, base);
	// 83003A50: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83003A54: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 83003A58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83003A5C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83003A60: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83003A64: 2F190003  cmpwi cr6, r25, 3
	ctx.cr[6].compare_i32(ctx.r[25].s32, 3, &mut ctx.xer);
	// 83003A68: 419A001C  beq cr6, 0x83003a84
	if ctx.cr[6].eq {
	pc = 0x83003A84; continue 'dispatch;
	}
	// 83003A6C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83003A70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003A74: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83003A78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003A7C: 4E800421  bctrl
	ctx.lr = 0x83003A80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003A80: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83003A84: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 83003A88: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83003A8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003A90: 4BFFFC99  bl 0x83003728
	ctx.lr = 0x83003A94;
	sub_83003728(ctx, base);
	// 83003A94: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83003A98: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83003A9C: 419A0018  beq cr6, 0x83003ab4
	if ctx.cr[6].eq {
	pc = 0x83003AB4; continue 'dispatch;
	}
	// 83003AA0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003AA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83003AA8: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83003AAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003AB0: 4E800421  bctrl
	ctx.lr = 0x83003AB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003AB4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83003AB8: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83003ABC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003AC0: 4BFFC939  bl 0x830003f8
	ctx.lr = 0x83003AC4;
	sub_830003F8(ctx, base);
	// 83003AC4: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003AC8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83003ACC: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83003AD0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83003AD4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83003AD8: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 83003ADC: 7F8B4850  subf r28, r11, r9
	ctx.r[28].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 83003AE0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83003AE4: 4E800421  bctrl
	ctx.lr = 0x83003AE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003AE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83003AEC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83003AF0: 40990058  ble cr6, 0x83003b48
	if !ctx.cr[6].gt {
	pc = 0x83003B48; continue 'dispatch;
	}
	// 83003AF4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003AF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83003AFC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83003B00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003B04: 4E800421  bctrl
	ctx.lr = 0x83003B08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003B08: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83003B0C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 83003B10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83003B14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003B18: 4BFFCBD9  bl 0x830006f0
	ctx.lr = 0x83003B1C;
	sub_830006F0(ctx, base);
	// 83003B1C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83003B20: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83003B24: 419A0018  beq cr6, 0x83003b3c
	if ctx.cr[6].eq {
	pc = 0x83003B3C; continue 'dispatch;
	}
	// 83003B28: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003B2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83003B30: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83003B34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003B38: 4E800421  bctrl
	ctx.lr = 0x83003B3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003B3C: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83003B40: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 83003B44: 4181FFB0  bgt 0x83003af4
	if ctx.cr[0].gt {
	pc = 0x83003AF4; continue 'dispatch;
	}
	// 83003B48: 2F190002  cmpwi cr6, r25, 2
	ctx.cr[6].compare_i32(ctx.r[25].s32, 2, &mut ctx.xer);
	// 83003B4C: 419A0034  beq cr6, 0x83003b80
	if ctx.cr[6].eq {
	pc = 0x83003B80; continue 'dispatch;
	}
	// 83003B50: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003B54: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83003B58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003B5C: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83003B60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003B64: 4E800421  bctrl
	ctx.lr = 0x83003B68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003B68: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003B6C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83003B70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003B74: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83003B78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003B7C: 4E800421  bctrl
	ctx.lr = 0x83003B80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003B80: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83003B84: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83003B88: 481A4624  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83003B90 size=416
    let mut pc: u32 = 0x83003B90;
    'dispatch: loop {
        match pc {
            0x83003B90 => {
    //   block [0x83003B90..0x83003D30)
	// 83003B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83003B94: 481A45C5  bl 0x831a8158
	ctx.lr = 0x83003B98;
	sub_831A8130(ctx, base);
	// 83003B98: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83003B9C: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 83003BA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83003BA4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83003BA8: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 83003BAC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83003BB0: 2F190003  cmpwi cr6, r25, 3
	ctx.cr[6].compare_i32(ctx.r[25].s32, 3, &mut ctx.xer);
	// 83003BB4: 419A001C  beq cr6, 0x83003bd0
	if ctx.cr[6].eq {
	pc = 0x83003BD0; continue 'dispatch;
	}
	// 83003BB8: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83003BBC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003BC0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83003BC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003BC8: 4E800421  bctrl
	ctx.lr = 0x83003BCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003BCC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83003BD0: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 83003BD4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83003BD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003BDC: 4BFFFB4D  bl 0x83003728
	ctx.lr = 0x83003BE0;
	sub_83003728(ctx, base);
	// 83003BE0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83003BE4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83003BE8: 419A0018  beq cr6, 0x83003c00
	if ctx.cr[6].eq {
	pc = 0x83003C00; continue 'dispatch;
	}
	// 83003BEC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003BF0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83003BF4: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83003BF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003BFC: 4E800421  bctrl
	ctx.lr = 0x83003C00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003C00: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003C04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83003C08: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83003C0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003C10: 4E800421  bctrl
	ctx.lr = 0x83003C14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003C14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83003C18: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83003C1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003C20: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83003C24: 4BFFC7D5  bl 0x830003f8
	ctx.lr = 0x83003C28;
	sub_830003F8(ctx, base);
	// 83003C28: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83003C2C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83003C30: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83003C34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003C38: 4BFFC7C1  bl 0x830003f8
	ctx.lr = 0x83003C3C;
	sub_830003F8(ctx, base);
	// 83003C3C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003C40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83003C44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83003C48: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 83003C4C: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 83003C50: 3B6BFFFF  addi r27, r11, -1
	ctx.r[27].s64 = ctx.r[11].s64 + -1;
	// 83003C54: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83003C58: 4E800421  bctrl
	ctx.lr = 0x83003C5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003C5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83003C60: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83003C64: 40990058  ble cr6, 0x83003cbc
	if !ctx.cr[6].gt {
	pc = 0x83003CBC; continue 'dispatch;
	}
	// 83003C68: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003C6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83003C70: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83003C74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003C78: 4E800421  bctrl
	ctx.lr = 0x83003C7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003C7C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83003C80: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 83003C84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83003C88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003C8C: 4BFFCA65  bl 0x830006f0
	ctx.lr = 0x83003C90;
	sub_830006F0(ctx, base);
	// 83003C90: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83003C94: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83003C98: 419A0018  beq cr6, 0x83003cb0
	if ctx.cr[6].eq {
	pc = 0x83003CB0; continue 'dispatch;
	}
	// 83003C9C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003CA0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83003CA4: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83003CA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003CAC: 4E800421  bctrl
	ctx.lr = 0x83003CB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003CB0: 377BFFFF  addic. r27, r27, -1
	ctx.xer.ca = (ctx.r[27].u32 > (!(-1 as u32)));
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83003CB4: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 83003CB8: 4181FFB0  bgt 0x83003c68
	if ctx.cr[0].gt {
	pc = 0x83003C68; continue 'dispatch;
	}
	// 83003CBC: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 83003CC0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83003CC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003CC8: 4BFFF8B1  bl 0x83003578
	ctx.lr = 0x83003CCC;
	sub_83003578(ctx, base);
	// 83003CCC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83003CD0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83003CD4: 419A0018  beq cr6, 0x83003cec
	if ctx.cr[6].eq {
	pc = 0x83003CEC; continue 'dispatch;
	}
	// 83003CD8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003CDC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83003CE0: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83003CE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003CE8: 4E800421  bctrl
	ctx.lr = 0x83003CEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003CEC: 2F190002  cmpwi cr6, r25, 2
	ctx.cr[6].compare_i32(ctx.r[25].s32, 2, &mut ctx.xer);
	// 83003CF0: 419A0034  beq cr6, 0x83003d24
	if ctx.cr[6].eq {
	pc = 0x83003D24; continue 'dispatch;
	}
	// 83003CF4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003CF8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83003CFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003D00: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83003D04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003D08: 4E800421  bctrl
	ctx.lr = 0x83003D0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003D0C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003D10: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83003D14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003D18: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83003D1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003D20: 4E800421  bctrl
	ctx.lr = 0x83003D24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003D24: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83003D28: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83003D2C: 481A447C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83003D30 size=508
    let mut pc: u32 = 0x83003D30;
    'dispatch: loop {
        match pc {
            0x83003D30 => {
    //   block [0x83003D30..0x83003F2C)
	// 83003D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83003D34: 481A442D  bl 0x831a8160
	ctx.lr = 0x83003D38;
	sub_831A8130(ctx, base);
	// 83003D38: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83003D3C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83003D40: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 83003D44: 897B001C  lbz r11, 0x1c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 83003D48: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83003D4C: 41820028  beq 0x83003d74
	if ctx.cr[0].eq {
	pc = 0x83003D74; continue 'dispatch;
	}
	// 83003D50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83003D54: 80DB0024  lwz r6, 0x24(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 83003D58: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 83003D5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83003D60: 4BFF6171  bl 0x82ff9ed0
	ctx.lr = 0x83003D64;
	sub_82FF9ED0(ctx, base);
	// 83003D64: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83003D68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83003D6C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83003D70: 481ACEB9  bl 0x831b0c28
	ctx.lr = 0x83003D74;
	sub_831B0C28(ctx, base);
	// 83003D74: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 83003D78: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83003D7C: 418201A4  beq 0x83003f20
	if ctx.cr[0].eq {
	pc = 0x83003F20; continue 'dispatch;
	}
	// 83003D80: 83FB000C  lwz r31, 0xc(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 83003D84: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83003D88: 41820198  beq 0x83003f20
	if ctx.cr[0].eq {
	pc = 0x83003F20; continue 'dispatch;
	}
	// 83003D8C: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83003D90: 409A0014  bne cr6, 0x83003da4
	if !ctx.cr[6].eq {
	pc = 0x83003DA4; continue 'dispatch;
	}
	// 83003D94: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83003D98: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83003D9C: 4BFFE22D  bl 0x83001fc8
	ctx.lr = 0x83003DA0;
	sub_83001FC8(ctx, base);
	// 83003DA0: 48000184  b 0x83003f24
	pc = 0x83003F24; continue 'dispatch;
	// 83003DA4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003DA8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83003DAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003DB0: 4800001C  b 0x83003dcc
	pc = 0x83003DCC; continue 'dispatch;
	// 83003DB4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 83003DB8: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83003DBC: 419A0038  beq cr6, 0x83003df4
	if ctx.cr[6].eq {
	pc = 0x83003DF4; continue 'dispatch;
	}
	// 83003DC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003DC4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83003DC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83003DCC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83003DD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003DD4: 4E800421  bctrl
	ctx.lr = 0x83003DD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003DD8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83003DDC: 4082FFD8  bne 0x83003db4
	if !ctx.cr[0].eq {
	pc = 0x83003DB4; continue 'dispatch;
	}
	// 83003DE0: 83FB0004  lwz r31, 4(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 83003DE4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83003DE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003DEC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003DF0: 48000030  b 0x83003e20
	pc = 0x83003E20; continue 'dispatch;
	// 83003DF4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83003DF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83003DFC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83003E00: 4BFFFAF9  bl 0x830038f8
	ctx.lr = 0x83003E04;
	sub_830038F8(ctx, base);
	// 83003E04: 48000120  b 0x83003f24
	pc = 0x83003F24; continue 'dispatch;
	// 83003E08: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 83003E0C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83003E10: 419A00A4  beq cr6, 0x83003eb4
	if ctx.cr[6].eq {
	pc = 0x83003EB4; continue 'dispatch;
	}
	// 83003E14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003E18: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83003E1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83003E20: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83003E24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003E28: 4E800421  bctrl
	ctx.lr = 0x83003E2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003E2C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83003E30: 4082FFD8  bne 0x83003e08
	if !ctx.cr[0].eq {
	pc = 0x83003E08; continue 'dispatch;
	}
	// 83003E34: 839B0004  lwz r28, 4(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 83003E38: 7FFDF051  subf. r31, r29, r30
	ctx.r[31].s64 = ctx.r[30].s64 - ctx.r[29].s64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83003E3C: 40810024  ble 0x83003e60
	if !ctx.cr[0].gt {
	pc = 0x83003E60; continue 'dispatch;
	}
	// 83003E40: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003E44: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83003E48: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83003E4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003E50: 4E800421  bctrl
	ctx.lr = 0x83003E54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003E54: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83003E58: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83003E5C: 4181FFE4  bgt 0x83003e40
	if ctx.cr[0].gt {
	pc = 0x83003E40; continue 'dispatch;
	}
	// 83003E60: 83BB000C  lwz r29, 0xc(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 83003E64: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83003E68: 40980028  bge cr6, 0x83003e90
	if !ctx.cr[6].lt {
	pc = 0x83003E90; continue 'dispatch;
	}
	// 83003E6C: 7FFF00D0  neg r31, r31
	ctx.r[31].s64 = -ctx.r[31].s64;
	// 83003E70: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003E74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83003E78: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83003E7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003E80: 4E800421  bctrl
	ctx.lr = 0x83003E84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003E84: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83003E88: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83003E8C: 4082FFE4  bne 0x83003e70
	if !ctx.cr[0].eq {
	pc = 0x83003E70; continue 'dispatch;
	}
	// 83003E90: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003E94: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83003E98: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83003E9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003EA0: 4E800421  bctrl
	ctx.lr = 0x83003EA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003EA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83003EA8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003EAC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83003EB0: 48000040  b 0x83003ef0
	pc = 0x83003EF0; continue 'dispatch;
	// 83003EB4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83003EB8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83003EBC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83003EC0: 4BFFFB89  bl 0x83003a48
	ctx.lr = 0x83003EC4;
	sub_83003A48(ctx, base);
	// 83003EC4: 48000060  b 0x83003f24
	pc = 0x83003F24; continue 'dispatch;
	// 83003EC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003ECC: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 83003ED0: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 83003ED4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003ED8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83003EDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003EE0: 4E800421  bctrl
	ctx.lr = 0x83003EE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003EE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83003EE8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83003EEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83003EF0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83003EF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83003EF8: 4E800421  bctrl
	ctx.lr = 0x83003EFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83003EFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83003F00: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83003F04: 409AFFC4  bne cr6, 0x83003ec8
	if !ctx.cr[6].eq {
	pc = 0x83003EC8; continue 'dispatch;
	}
	// 83003F08: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 83003F0C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83003F10: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83003F14: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83003F18: 4BFFFC79  bl 0x83003b90
	ctx.lr = 0x83003F1C;
	sub_83003B90(ctx, base);
	// 83003F1C: 48000008  b 0x83003f24
	pc = 0x83003F24; continue 'dispatch;
	// 83003F20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83003F24: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83003F28: 481A4288  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83003F30 size=8
    let mut pc: u32 = 0x83003F30;
    'dispatch: loop {
        match pc {
            0x83003F30 => {
    //   block [0x83003F30..0x83003F38)
	// 83003F30: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83003F34: 4BFFFDFC  b 0x83003d30
	sub_83003D30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83003F38 size=72
    let mut pc: u32 = 0x83003F38;
    'dispatch: loop {
        match pc {
            0x83003F38 => {
    //   block [0x83003F38..0x83003F80)
	// 83003F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83003F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83003F40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83003F44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83003F48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83003F4C: 80FF0010  lwz r7, 0x10(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83003F50: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83003F54: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83003F58: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83003F5C: 4BFFE2ED  bl 0x83002248
	ctx.lr = 0x83003F60;
	sub_83002248(ctx, base);
	// 83003F60: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83003F64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83003F68: 4BFFFDC9  bl 0x83003d30
	ctx.lr = 0x83003F6C;
	sub_83003D30(ctx, base);
	// 83003F6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83003F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83003F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83003F78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83003F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83003F80 size=8
    let mut pc: u32 = 0x83003F80;
    'dispatch: loop {
        match pc {
            0x83003F80 => {
    //   block [0x83003F80..0x83003F88)
	// 83003F80: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83003F84: 4BFFFDAC  b 0x83003d30
	sub_83003D30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83003F88 size=8
    let mut pc: u32 = 0x83003F88;
    'dispatch: loop {
        match pc {
            0x83003F88 => {
    //   block [0x83003F88..0x83003F90)
	// 83003F88: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83003F8C: 82140650  lwz r16, 0x650(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(1616 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83003F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83003F90 size=120
    let mut pc: u32 = 0x83003F90;
    'dispatch: loop {
        match pc {
            0x83003F90 => {
    //   block [0x83003F90..0x83004008)
	// 83003F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83003F94: 481A41D9  bl 0x831a816c
	ctx.lr = 0x83003F98;
	sub_831A8130(ctx, base);
	// 83003F98: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83003F9C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83003FA0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83003FA4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83003FA8: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83003FAC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83003FB0: 38640004  addi r3, r4, 4
	ctx.r[3].s64 = ctx.r[4].s64 + 4;
	// 83003FB4: 909E0004  stw r4, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 83003FB8: 394B063C  addi r10, r11, 0x63c
	ctx.r[10].s64 = ctx.r[11].s64 + 1596;
	// 83003FBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83003FC0: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83003FC4: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83003FC8: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83003FCC: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83003FD0: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83003FD4: 997E0020  stb r11, 0x20(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 83003FD8: 997E0021  stb r11, 0x21(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	// 83003FDC: 4BFDB24D  bl 0x82fdf228
	ctx.lr = 0x83003FE0;
	sub_82FDF228(ctx, base);
	// 83003FE0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83003FE4: 4BFDD9F5  bl 0x82fe19d8
	ctx.lr = 0x83003FE8;
	sub_82FE19D8(ctx, base);
	// 83003FE8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83003FEC: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83003FF0: 388B0638  addi r4, r11, 0x638
	ctx.r[4].s64 = ctx.r[11].s64 + 1592;
	// 83003FF4: 4BFCFC4D  bl 0x82fd3c40
	ctx.lr = 0x83003FF8;
	sub_82FD3C40(ctx, base);
	// 83003FF8: 987E000C  stb r3, 0xc(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u8 ) };
	// 83003FFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83004000: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83004004: 481A41B8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004008 size=40
    let mut pc: u32 = 0x83004008;
    'dispatch: loop {
        match pc {
            0x83004008 => {
    //   block [0x83004008..0x83004030)
	// 83004008: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8300400C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004010: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004014: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004018: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300401C: 4BFF5FED  bl 0x82ffa008
	ctx.lr = 0x83004020;
	sub_82FFA008(ctx, base);
	// 83004020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83004024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300402C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004030 size=8
    let mut pc: u32 = 0x83004030;
    'dispatch: loop {
        match pc {
            0x83004030 => {
    //   block [0x83004030..0x83004038)
	// 83004030: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83004034: 82140688  lwz r16, 0x688(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(1672 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004038 size=176
    let mut pc: u32 = 0x83004038;
    'dispatch: loop {
        match pc {
            0x83004038 => {
    //   block [0x83004038..0x830040E8)
	// 83004038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300403C: 481A4129  bl 0x831a8164
	ctx.lr = 0x83004040;
	sub_831A8130(ctx, base);
	// 83004040: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83004044: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004048: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300404C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83004050: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83004054: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83004058: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300405C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83004060: 909E0004  stw r4, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 83004064: 394B063C  addi r10, r11, 0x63c
	ctx.r[10].s64 = ctx.r[11].s64 + 1596;
	// 83004068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300406C: 3B840004  addi r28, r4, 4
	ctx.r[28].s64 = ctx.r[4].s64 + 4;
	// 83004070: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83004074: 993E0021  stb r9, 0x21(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(33 as u32), ctx.r[9].u8 ) };
	// 83004078: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8300407C: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83004080: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83004084: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83004088: 997E0020  stb r11, 0x20(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 8300408C: 4BFDB19D  bl 0x82fdf228
	ctx.lr = 0x83004090;
	sub_82FDF228(ctx, base);
	// 83004090: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83004094: 4BFDD945  bl 0x82fe19d8
	ctx.lr = 0x83004098;
	sub_82FE19D8(ctx, base);
	// 83004098: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300409C: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 830040A0: 3B6B0638  addi r27, r11, 0x638
	ctx.r[27].s64 = ctx.r[11].s64 + 1592;
	// 830040A4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830040A8: 4BFCFB99  bl 0x82fd3c40
	ctx.lr = 0x830040AC;
	sub_82FD3C40(ctx, base);
	// 830040AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830040B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830040B4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830040B8: 997E000C  stb r11, 0xc(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 830040BC: 4BFCFB85  bl 0x82fd3c40
	ctx.lr = 0x830040C0;
	sub_82FD3C40(ctx, base);
	// 830040C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830040C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830040C8: 997E0020  stb r11, 0x20(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 830040CC: 4BFDB15D  bl 0x82fdf228
	ctx.lr = 0x830040D0;
	sub_82FDF228(ctx, base);
	// 830040D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830040D4: 4BFDD905  bl 0x82fe19d8
	ctx.lr = 0x830040D8;
	sub_82FE19D8(ctx, base);
	// 830040D8: 907E001C  stw r3, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 830040DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830040E0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830040E4: 481A40D0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830040E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830040E8 size=40
    let mut pc: u32 = 0x830040E8;
    'dispatch: loop {
        match pc {
            0x830040E8 => {
    //   block [0x830040E8..0x83004110)
	// 830040E8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830040EC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830040F0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830040F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830040F8: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830040FC: 4BFF5F0D  bl 0x82ffa008
	ctx.lr = 0x83004100;
	sub_82FFA008(ctx, base);
	// 83004100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83004104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300410C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004110 size=92
    let mut pc: u32 = 0x83004110;
    'dispatch: loop {
        match pc {
            0x83004110 => {
    //   block [0x83004110..0x8300416C)
	// 83004110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004118: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300411C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004120: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83004124: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83004128: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300412C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83004130: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004134: 4E800421  bctrl
	ctx.lr = 0x83004138;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004138: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300413C: 3C807FFF  lis r4, 0x7fff
	ctx.r[4].s64 = 2147418112;
	// 83004140: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83004144: 6084FFFF  ori r4, r4, 0xffff
	ctx.r[4].u64 = ctx.r[4].u64 | 65535;
	// 83004148: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300414C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004150: 4E800421  bctrl
	ctx.lr = 0x83004154;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004154: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83004158: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300415C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004164: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83004168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004170 size=416
    let mut pc: u32 = 0x83004170;
    'dispatch: loop {
        match pc {
            0x83004170 => {
    //   block [0x83004170..0x83004310)
	// 83004170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004174: 481A3FF9  bl 0x831a816c
	ctx.lr = 0x83004178;
	sub_831A8130(ctx, base);
	// 83004178: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300417C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83004180: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83004184: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83004188: 419A017C  beq cr6, 0x83004304
	if ctx.cr[6].eq {
	pc = 0x83004304; continue 'dispatch;
	}
	// 8300418C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004190: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83004194: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 83004198: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300419C: 4E800421  bctrl
	ctx.lr = 0x830041A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830041A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830041A4: 4182001C  beq 0x830041c0
	if ctx.cr[0].eq {
	pc = 0x830041C0; continue 'dispatch;
	}
	// 830041A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830041AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830041B0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830041B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830041B8: 4E800421  bctrl
	ctx.lr = 0x830041BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830041BC: 4800002C  b 0x830041e8
	pc = 0x830041E8; continue 'dispatch;
	// 830041C0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830041C4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830041C8: 419A0028  beq cr6, 0x830041f0
	if ctx.cr[6].eq {
	pc = 0x830041F0; continue 'dispatch;
	}
	// 830041CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830041D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830041D4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 830041D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830041DC: 4E800421  bctrl
	ctx.lr = 0x830041E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830041E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830041E4: 4182000C  beq 0x830041f0
	if ctx.cr[0].eq {
	pc = 0x830041F0; continue 'dispatch;
	}
	// 830041E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830041EC: 48000050  b 0x8300423c
	pc = 0x8300423C; continue 'dispatch;
	// 830041F0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830041F4: 48000038  b 0x8300422c
	pc = 0x8300422C; continue 'dispatch;
	// 830041F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830041FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83004200: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83004204: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004208: 4E800421  bctrl
	ctx.lr = 0x8300420C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300420C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83004210: 40820028  bne 0x83004238
	if !ctx.cr[0].eq {
	pc = 0x83004238; continue 'dispatch;
	}
	// 83004214: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004218: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300421C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83004220: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004224: 4E800421  bctrl
	ctx.lr = 0x83004228;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004228: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300422C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83004230: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83004234: 409AFFC4  bne cr6, 0x830041f8
	if !ctx.cr[6].eq {
	pc = 0x830041F8; continue 'dispatch;
	}
	// 83004238: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 8300423C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83004240: 419A00C4  beq cr6, 0x83004304
	if ctx.cr[6].eq {
	pc = 0x83004304; continue 'dispatch;
	}
	// 83004244: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83004248: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300424C: 419AFF40  beq cr6, 0x8300418c
	if ctx.cr[6].eq {
	pc = 0x8300418C; continue 'dispatch;
	}
	// 83004250: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004254: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83004258: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300425C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004260: 4E800421  bctrl
	ctx.lr = 0x83004264;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004264: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83004268: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8300426C: 409AFF20  bne cr6, 0x8300418c
	if !ctx.cr[6].eq {
	pc = 0x8300418C; continue 'dispatch;
	}
	// 83004270: 897E0021  lbz r11, 0x21(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(33 as u32) ) } as u64;
	// 83004274: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83004278: 4082001C  bne 0x83004294
	if !ctx.cr[0].eq {
	pc = 0x83004294; continue 'dispatch;
	}
	// 8300427C: 897E000C  lbz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83004280: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83004284: 40820078  bne 0x830042fc
	if !ctx.cr[0].eq {
	pc = 0x830042FC; continue 'dispatch;
	}
	// 83004288: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300428C: 816B009C  lwz r11, 0x9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 83004290: 4800004C  b 0x830042dc
	pc = 0x830042DC; continue 'dispatch;
	// 83004294: 897E0020  lbz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83004298: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300429C: 4082002C  bne 0x830042c8
	if !ctx.cr[0].eq {
	pc = 0x830042C8; continue 'dispatch;
	}
	// 830042A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830042A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830042A8: 83BE001C  lwz r29, 0x1c(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830042AC: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 830042B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830042B4: 4E800421  bctrl
	ctx.lr = 0x830042B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830042B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830042BC: 4BFCF985  bl 0x82fd3c40
	ctx.lr = 0x830042C0;
	sub_82FD3C40(ctx, base);
	// 830042C0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830042C4: 4182FEC8  beq 0x8300418c
	if ctx.cr[0].eq {
	pc = 0x8300418C; continue 'dispatch;
	}
	// 830042C8: 897E000C  lbz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830042CC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830042D0: 4082002C  bne 0x830042fc
	if !ctx.cr[0].eq {
	pc = 0x830042FC; continue 'dispatch;
	}
	// 830042D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830042D8: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 830042DC: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830042E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830042E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830042E8: 4E800421  bctrl
	ctx.lr = 0x830042EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830042EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830042F0: 4BFCF951  bl 0x82fd3c40
	ctx.lr = 0x830042F4;
	sub_82FD3C40(ctx, base);
	// 830042F4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830042F8: 4182FE94  beq 0x8300418c
	if ctx.cr[0].eq {
	pc = 0x8300418C; continue 'dispatch;
	}
	// 830042FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83004300: 48000008  b 0x83004308
	pc = 0x83004308; continue 'dispatch;
	// 83004304: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83004308: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300430C: 481A3EB0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004310 size=196
    let mut pc: u32 = 0x83004310;
    'dispatch: loop {
        match pc {
            0x83004310 => {
    //   block [0x83004310..0x830043D4)
	// 83004310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004314: 481A3E55  bl 0x831a8168
	ctx.lr = 0x83004318;
	sub_831A8130(ctx, base);
	// 83004318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300431C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83004320: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83004324: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83004328: 83DF0018  lwz r30, 0x18(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8300432C: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 83004330: 83BF0014  lwz r29, 0x14(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83004334: 4BFF83F5  bl 0x82ffc728
	ctx.lr = 0x83004338;
	sub_82FFC728(ctx, base);
	// 83004338: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300433C: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83004340: 419A0024  beq cr6, 0x83004364
	if ctx.cr[6].eq {
	pc = 0x83004364; continue 'dispatch;
	}
	// 83004344: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83004348: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8300434C: 387D000C  addi r3, r29, 0xc
	ctx.r[3].s64 = ctx.r[29].s64 + 12;
	// 83004350: 4BFF83D9  bl 0x82ffc728
	ctx.lr = 0x83004354;
	sub_82FFC728(ctx, base);
	// 83004354: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83004358: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8300435C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83004360: 4800004C  b 0x830043ac
	pc = 0x830043AC; continue 'dispatch;
	// 83004364: 397C0001  addi r11, r28, 1
	ctx.r[11].s64 = ctx.r[28].s64 + 1;
	// 83004368: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8300436C: 40980010  bge cr6, 0x8300437c
	if !ctx.cr[6].lt {
	pc = 0x8300437C; continue 'dispatch;
	}
	// 83004370: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83004374: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83004378: 4BFFFFE0  b 0x83004358
	pc = 0x83004358; continue 'dispatch;
	// 8300437C: 409AFFDC  bne cr6, 0x83004358
	if !ctx.cr[6].eq {
	pc = 0x83004358; continue 'dispatch;
	}
	// 83004380: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83004384: 48000048  b 0x830043cc
	pc = 0x830043CC; continue 'dispatch;
	// 83004388: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8300438C: 419A0028  beq cr6, 0x830043b4
	if ctx.cr[6].eq {
	pc = 0x830043B4; continue 'dispatch;
	}
	// 83004390: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83004394: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83004398: 4BFFFDD9  bl 0x83004170
	ctx.lr = 0x8300439C;
	sub_83004170(ctx, base);
	// 8300439C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830043A0: 41820014  beq 0x830043b4
	if ctx.cr[0].eq {
	pc = 0x830043B4; continue 'dispatch;
	}
	// 830043A4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 830043A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830043AC: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830043B0: 4198FFD8  blt cr6, 0x83004388
	if ctx.cr[6].lt {
	pc = 0x83004388; continue 'dispatch;
	}
	// 830043B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830043B8: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 830043BC: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 830043C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830043C4: 409A0008  bne cr6, 0x830043cc
	if !ctx.cr[6].eq {
	pc = 0x830043CC; continue 'dispatch;
	}
	// 830043C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830043CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830043D0: 481A3DE8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830043D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830043D8 size=4
    let mut pc: u32 = 0x830043D8;
    'dispatch: loop {
        match pc {
            0x830043D8 => {
    //   block [0x830043D8..0x830043DC)
	// 830043D8: 4BFFFF38  b 0x83004310
	sub_83004310(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830043E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830043E0 size=8
    let mut pc: u32 = 0x830043E0;
    'dispatch: loop {
        match pc {
            0x830043E0 => {
    //   block [0x830043E0..0x830043E8)
	// 830043E0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830043E4: 82140798  lwz r16, 0x798(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(1944 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830043E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830043E8 size=100
    let mut pc: u32 = 0x830043E8;
    'dispatch: loop {
        match pc {
            0x830043E8 => {
    //   block [0x830043E8..0x8300444C)
	// 830043E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830043EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830043F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830043F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830043F8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830043FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004400: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83004404: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83004408: 396B06B8  addi r11, r11, 0x6b8
	ctx.r[11].s64 = ctx.r[11].s64 + 1720;
	// 8300440C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83004410: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83004414: 397E000C  addi r11, r30, 0xc
	ctx.r[11].s64 = ctx.r[30].s64 + 12;
	// 83004418: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 8300441C: 4BFF5BED  bl 0x82ffa008
	ctx.lr = 0x83004420;
	sub_82FFA008(ctx, base);
	// 83004420: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 83004424: 480C36BD  bl 0x830c7ae0
	ctx.lr = 0x83004428;
	sub_830C7AE0(ctx, base);
	// 83004428: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300442C: 396BA8A0  addi r11, r11, -0x5760
	ctx.r[11].s64 = ctx.r[11].s64 + -22368;
	// 83004430: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83004434: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83004438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300443C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004440: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83004444: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83004448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300444C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300444C size=40
    let mut pc: u32 = 0x8300444C;
    'dispatch: loop {
        match pc {
            0x8300444C => {
    //   block [0x8300444C..0x83004474)
	// 8300444C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83004450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300445C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83004460: 48001721  bl 0x83005b80
	ctx.lr = 0x83004464;
	sub_83005B80(ctx, base);
	// 83004464: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83004468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300446C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004474(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004474 size=44
    let mut pc: u32 = 0x83004474;
    'dispatch: loop {
        match pc {
            0x83004474 => {
    //   block [0x83004474..0x830044A0)
	// 83004474: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83004478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300447C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004480: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004484: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83004488: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8300448C: 480C3655  bl 0x830c7ae0
	ctx.lr = 0x83004490;
	sub_830C7AE0(ctx, base);
	// 83004490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83004494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300449C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830044A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830044A0 size=132
    let mut pc: u32 = 0x830044A0;
    'dispatch: loop {
        match pc {
            0x830044A0 => {
    //   block [0x830044A0..0x83004524)
	// 830044A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830044A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830044A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830044AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830044B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830044B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830044B8: 3FC08214  lis r30, -0x7dec
	ctx.r[30].s64 = -2112618496;
	// 830044BC: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830044C0: A17EA6B8  lhz r11, -0x5948(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(-22856 as u32) ) } as u64;
	// 830044C4: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 830044C8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830044CC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830044D0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830044D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830044D8: 41820034  beq 0x8300450c
	if ctx.cr[0].eq {
	pc = 0x8300450C; continue 'dispatch;
	}
	// 830044DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830044E0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830044E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830044E8: 4E800421  bctrl
	ctx.lr = 0x830044EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830044EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830044F0: 80630028  lwz r3, 0x28(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 830044F4: 4BFED3F5  bl 0x82ff18e8
	ctx.lr = 0x830044F8;
	sub_82FF18E8(ctx, base);
	// 830044F8: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 830044FC: A15EA6B8  lhz r10, -0x5948(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(-22856 as u32) ) } as u64;
	// 83004500: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83004504: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 83004508: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 8300450C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83004510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004518: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300451C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83004520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004528 size=8
    let mut pc: u32 = 0x83004528;
    'dispatch: loop {
        match pc {
            0x83004528 => {
    //   block [0x83004528..0x83004530)
	// 83004528: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300452C: 821407D8  lwz r16, 0x7d8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(2008 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004530 size=160
    let mut pc: u32 = 0x83004530;
    'dispatch: loop {
        match pc {
            0x83004530 => {
    //   block [0x83004530..0x830045D0)
	// 83004530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004534: 481A3C39  bl 0x831a816c
	ctx.lr = 0x83004538;
	sub_831A8130(ctx, base);
	// 83004538: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8300453C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004540: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83004544: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83004548: A14BA6B8  lhz r10, -0x5948(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22856 as u32) ) } as u64;
	// 8300454C: A17D0008  lhz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83004550: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 83004554: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83004558: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8300455C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83004560: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83004564: 40820064  bne 0x830045c8
	if !ctx.cr[0].eq {
	pc = 0x830045C8; continue 'dispatch;
	}
	// 83004568: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 8300456C: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83004570: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83004574: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 83004578: 83DD000C  lwz r30, 0xc(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300457C: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83004580: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83004584: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83004588: 409A0034  bne cr6, 0x830045bc
	if !ctx.cr[6].eq {
	pc = 0x830045BC; continue 'dispatch;
	}
	// 8300458C: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 83004590: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83004594: 4BFDD45D  bl 0x82fe19f0
	ctx.lr = 0x83004598;
	sub_82FE19F0(ctx, base);
	// 83004598: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8300459C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830045A0: 41820014  beq 0x830045b4
	if ctx.cr[0].eq {
	pc = 0x830045B4; continue 'dispatch;
	}
	// 830045A4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830045A8: 388001F4  li r4, 0x1f4
	ctx.r[4].s64 = 500;
	// 830045AC: 4BFED11D  bl 0x82ff16c8
	ctx.lr = 0x830045B0;
	sub_82FF16C8(ctx, base);
	// 830045B0: 48000008  b 0x830045b8
	pc = 0x830045B8; continue 'dispatch;
	// 830045B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830045B8: 907E0028  stw r3, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 830045BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830045C0: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830045C4: 4BFED24D  bl 0x82ff1810
	ctx.lr = 0x830045C8;
	sub_82FF1810(ctx, base);
	// 830045C8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830045CC: 481A3BF0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830045D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830045D0 size=44
    let mut pc: u32 = 0x830045D0;
    'dispatch: loop {
        match pc {
            0x830045D0 => {
    //   block [0x830045D0..0x830045FC)
	// 830045D0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830045D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830045D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830045DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830045E0: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830045E4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830045E8: 480C34F9  bl 0x830c7ae0
	ctx.lr = 0x830045EC;
	sub_830C7AE0(ctx, base);
	// 830045EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830045F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830045F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830045F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004600 size=8
    let mut pc: u32 = 0x83004600;
    'dispatch: loop {
        match pc {
            0x83004600 => {
    //   block [0x83004600..0x83004608)
	// 83004600: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83004604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004608 size=16
    let mut pc: u32 = 0x83004608;
    'dispatch: loop {
        match pc {
            0x83004608 => {
    //   block [0x83004608..0x83004618)
	// 83004608: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300460C: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 83004610: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004614: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004618 size=36
    let mut pc: u32 = 0x83004618;
    'dispatch: loop {
        match pc {
            0x83004618 => {
    //   block [0x83004618..0x8300463C)
	// 83004618: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300461C: A1430008  lhz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83004620: A16BA6AC  lhz r11, -0x5954(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22868 as u32) ) } as u64;
	// 83004624: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83004628: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300462C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83004630: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83004634: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 83004638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004640 size=228
    let mut pc: u32 = 0x83004640;
    'dispatch: loop {
        match pc {
            0x83004640 => {
    //   block [0x83004640..0x83004724)
	// 83004640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004644: 481A3B25  bl 0x831a8168
	ctx.lr = 0x83004648;
	sub_831A8130(ctx, base);
	// 83004648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300464C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83004650: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83004654: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83004658: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300465C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004660: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83004664: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004668: 4E800421  bctrl
	ctx.lr = 0x8300466C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300466C: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83004670: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83004674: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83004678: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300467C: 409A0024  bne cr6, 0x830046a0
	if !ctx.cr[6].eq {
	pc = 0x830046A0; continue 'dispatch;
	}
	// 83004680: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83004684: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004688: 4E800421  bctrl
	ctx.lr = 0x8300468C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300468C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83004690: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83004694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83004698: 4BFD4ED9  bl 0x82fd9570
	ctx.lr = 0x8300469C;
	sub_82FD9570(ctx, base);
	// 8300469C: 48000080  b 0x8300471c
	pc = 0x8300471C; continue 'dispatch;
	// 830046A0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830046A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830046A8: 4E800421  bctrl
	ctx.lr = 0x830046AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830046AC: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 830046B0: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 830046B4: 409A0068  bne cr6, 0x8300471c
	if !ctx.cr[6].eq {
	pc = 0x8300471C; continue 'dispatch;
	}
	// 830046B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830046BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830046C0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830046C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830046C8: 4E800421  bctrl
	ctx.lr = 0x830046CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830046CC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830046D0: 4182004C  beq 0x8300471c
	if ctx.cr[0].eq {
	pc = 0x8300471C; continue 'dispatch;
	}
	// 830046D4: 3FA08214  lis r29, -0x7dec
	ctx.r[29].s64 = -2112618496;
	// 830046D8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830046DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830046E0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830046E4: 4BFFFF5D  bl 0x83004640
	ctx.lr = 0x830046E8;
	sub_83004640(ctx, base);
	// 830046E8: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830046EC: A17DA6C0  lhz r11, -0x5940(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 830046F0: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 830046F4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830046F8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830046FC: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83004700: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83004704: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 83004708: 40820008  bne 0x83004710
	if !ctx.cr[0].eq {
	pc = 0x83004710; continue 'dispatch;
	}
	// 8300470C: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 83004710: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83004714: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83004718: 4082FFC0  bne 0x830046d8
	if !ctx.cr[0].eq {
	pc = 0x830046D8; continue 'dispatch;
	}
	// 8300471C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83004720: 481A3A98  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004728 size=16
    let mut pc: u32 = 0x83004728;
    'dispatch: loop {
        match pc {
            0x83004728 => {
    //   block [0x83004728..0x83004738)
	// 83004728: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300472C: 816B00A8  lwz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 83004730: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004734: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004738 size=32
    let mut pc: u32 = 0x83004738;
    'dispatch: loop {
        match pc {
            0x83004738 => {
    //   block [0x83004738..0x83004758)
	// 83004738: 548A063F  clrlwi. r10, r4, 0x18
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8300473C: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 83004740: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 83004744: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83004748: A14AA6AC  lhz r10, -0x5954(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22868 as u32) ) } as u64;
	// 8300474C: 4182000C  beq 0x83004758
	if ctx.cr[0].eq {
		sub_83004758(ctx, base);
		return;
	}
	// 83004750: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83004754: 48000008  b 0x8300475c
	sub_83004758(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004758 size=12
    let mut pc: u32 = 0x83004758;
    'dispatch: loop {
        match pc {
            0x83004758 => {
    //   block [0x83004758..0x83004764)
	// 83004758: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 8300475C: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 83004760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004768 size=404
    let mut pc: u32 = 0x83004768;
    'dispatch: loop {
        match pc {
            0x83004768 => {
    //   block [0x83004768..0x830048FC)
	// 83004768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300476C: 481A39F9  bl 0x831a8164
	ctx.lr = 0x83004770;
	sub_831A8130(ctx, base);
	// 83004770: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004774: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83004778: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300477C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83004780: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83004784: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83004788: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 8300478C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83004790: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83004794: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83004798: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300479C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830047A0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830047A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830047A8: 41820054  beq 0x830047fc
	if ctx.cr[0].eq {
	pc = 0x830047FC; continue 'dispatch;
	}
	// 830047AC: 4E800421  bctrl
	ctx.lr = 0x830047B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830047B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830047B4: 41820020  beq 0x830047d4
	if ctx.cr[0].eq {
	pc = 0x830047D4; continue 'dispatch;
	}
	// 830047B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830047BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830047C0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830047C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830047C8: 4E800421  bctrl
	ctx.lr = 0x830047CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830047CC: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 830047D0: 4800000C  b 0x830047dc
	pc = 0x830047DC; continue 'dispatch;
	// 830047D4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830047D8: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830047DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830047E0: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 830047E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830047E8: 4BFF56E9  bl 0x82ff9ed0
	ctx.lr = 0x830047EC;
	sub_82FF9ED0(ctx, base);
	// 830047EC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830047F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830047F4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830047F8: 481AC431  bl 0x831b0c28
	ctx.lr = 0x830047FC;
	sub_831B0C28(ctx, base);
	// 830047FC: 4E800421  bctrl
	ctx.lr = 0x83004800;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004800: 3F808214  lis r28, -0x7dec
	ctx.r[28].s64 = -2112618496;
	// 83004804: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83004808: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300480C: A17CA6B8  lhz r11, -0x5948(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(-22856 as u32) ) } as u64;
	// 83004810: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 83004814: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83004818: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8300481C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83004820: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83004824: 41820040  beq 0x83004864
	if ctx.cr[0].eq {
	pc = 0x83004864; continue 'dispatch;
	}
	// 83004828: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300482C: 807D0028  lwz r3, 0x28(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 83004830: 4BFED0B9  bl 0x82ff18e8
	ctx.lr = 0x83004834;
	sub_82FF18E8(ctx, base);
	// 83004834: 48000030  b 0x83004864
	pc = 0x83004864; continue 'dispatch;
	// 83004838: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300483C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83004840: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 83004844: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004848: 4E800421  bctrl
	ctx.lr = 0x8300484C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300484C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83004850: 41820014  beq 0x83004864
	if ctx.cr[0].eq {
	pc = 0x83004864; continue 'dispatch;
	}
	// 83004854: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004858: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 8300485C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004860: 4E800421  bctrl
	ctx.lr = 0x83004864;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004864: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83004868: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300486C: 4082FFCC  bne 0x83004838
	if !ctx.cr[0].eq {
	pc = 0x83004838; continue 'dispatch;
	}
	// 83004870: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83004874: 419A0034  beq cr6, 0x830048a8
	if ctx.cr[6].eq {
	pc = 0x830048A8; continue 'dispatch;
	}
	// 83004878: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300487C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83004880: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004884: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83004888: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300488C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004890: 4E800421  bctrl
	ctx.lr = 0x83004894;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004894: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83004898: 817E0040  lwz r11, 0x40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 8300489C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830048A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830048A4: 4E800421  bctrl
	ctx.lr = 0x830048A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830048A8: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 830048AC: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 830048B0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 830048B4: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830048B8: A14AA6AC  lhz r10, -0x5954(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22868 as u32) ) } as u64;
	// 830048BC: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 830048C0: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 830048C4: 4BFF7E4D  bl 0x82ffc710
	ctx.lr = 0x830048C8;
	sub_82FFC710(ctx, base);
	// 830048C8: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830048CC: A17CA6B8  lhz r11, -0x5948(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(-22856 as u32) ) } as u64;
	// 830048D0: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 830048D4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830048D8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830048DC: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830048E0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830048E4: 41820010  beq 0x830048f4
	if ctx.cr[0].eq {
	pc = 0x830048F4; continue 'dispatch;
	}
	// 830048E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830048EC: 807D0028  lwz r3, 0x28(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 830048F0: 4BFECF21  bl 0x82ff1810
	ctx.lr = 0x830048F4;
	sub_82FF1810(ctx, base);
	// 830048F4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 830048F8: 481A38BC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004900 size=44
    let mut pc: u32 = 0x83004900;
    'dispatch: loop {
        match pc {
            0x83004900 => {
    //   block [0x83004900..0x8300492C)
	// 83004900: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83004904: A1430008  lhz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83004908: A16BA6A4  lhz r11, -0x595c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 8300490C: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83004910: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83004914: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83004918: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8300491C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83004920: 4182000C  beq 0x8300492c
	if ctx.cr[0].eq {
		sub_8300492C(ctx, base);
		return;
	}
	// 83004924: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83004928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300492C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300492C size=8
    let mut pc: u32 = 0x8300492C;
    'dispatch: loop {
        match pc {
            0x8300492C => {
    //   block [0x8300492C..0x83004934)
	// 8300492C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83004930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004938 size=376
    let mut pc: u32 = 0x83004938;
    'dispatch: loop {
        match pc {
            0x83004938 => {
    //   block [0x83004938..0x83004AB0)
	// 83004938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300493C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004940: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83004944: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83004948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300494C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83004950: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 83004954: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83004958: A14AA6A4  lhz r10, -0x595c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 8300495C: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83004960: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 83004964: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 83004968: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 8300496C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83004970: 41820080  beq 0x830049f0
	if ctx.cr[0].eq {
	pc = 0x830049F0; continue 'dispatch;
	}
	// 83004974: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 83004978: A14AA6C8  lhz r10, -0x5938(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22840 as u32) ) } as u64;
	// 8300497C: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83004980: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83004984: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83004988: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8300498C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83004990: 40820060  bne 0x830049f0
	if !ctx.cr[0].eq {
	pc = 0x830049F0; continue 'dispatch;
	}
	// 83004994: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004998: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300499C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830049A0: 4E800421  bctrl
	ctx.lr = 0x830049A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830049A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830049A8: 41820020  beq 0x830049c8
	if ctx.cr[0].eq {
	pc = 0x830049C8; continue 'dispatch;
	}
	// 830049AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830049B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830049B4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830049B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830049BC: 4E800421  bctrl
	ctx.lr = 0x830049C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830049C0: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 830049C4: 4800000C  b 0x830049d0
	pc = 0x830049D0; continue 'dispatch;
	// 830049C8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830049CC: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830049D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830049D4: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 830049D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830049DC: 4BFF54F5  bl 0x82ff9ed0
	ctx.lr = 0x830049E0;
	sub_82FF9ED0(ctx, base);
	// 830049E0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830049E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830049E8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830049EC: 481AC23D  bl 0x831b0c28
	ctx.lr = 0x830049F0;
	sub_831B0C28(ctx, base);
	// 830049F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830049F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830049F8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830049FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004A00: 4E800421  bctrl
	ctx.lr = 0x83004A04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004A04: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83004A08: 41820048  beq 0x83004a50
	if ctx.cr[0].eq {
	pc = 0x83004A50; continue 'dispatch;
	}
	// 83004A0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83004A10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83004A14: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83004A18: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83004A1C: 4BFDAD5D  bl 0x82fdf778
	ctx.lr = 0x83004A20;
	sub_82FDF778(ctx, base);
	// 83004A20: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83004A24: 4BFF7C65  bl 0x82ffc688
	ctx.lr = 0x83004A28;
	sub_82FFC688(ctx, base);
	// 83004A28: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83004A2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83004A30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83004A34: 4BFE0CAD  bl 0x82fe56e0
	ctx.lr = 0x83004A38;
	sub_82FE56E0(ctx, base);
	// 83004A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83004A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004A44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83004A48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83004A4C: 4E800020  blr
	return;
	// 83004A50: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004A54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83004A58: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83004A5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004A60: 4E800421  bctrl
	ctx.lr = 0x83004A64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004A64: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83004A68: 41820020  beq 0x83004a88
	if ctx.cr[0].eq {
	pc = 0x83004A88; continue 'dispatch;
	}
	// 83004A6C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004A70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83004A74: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83004A78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004A7C: 4E800421  bctrl
	ctx.lr = 0x83004A80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004A80: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83004A84: 4800000C  b 0x83004a90
	pc = 0x83004A90; continue 'dispatch;
	// 83004A88: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83004A8C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83004A90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83004A94: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83004A98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83004A9C: 4BFF5435  bl 0x82ff9ed0
	ctx.lr = 0x83004AA0;
	sub_82FF9ED0(ctx, base);
	// 83004AA0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83004AA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83004AA8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83004AAC: 481AC17D  bl 0x831b0c28
	ctx.lr = 0x83004AB0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004AB0 size=36
    let mut pc: u32 = 0x83004AB0;
    'dispatch: loop {
        match pc {
            0x83004AB0 => {
    //   block [0x83004AB0..0x83004AD4)
	// 83004AB0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83004AB4: A1430008  lhz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83004AB8: A16BA6B8  lhz r11, -0x5948(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22856 as u32) ) } as u64;
	// 83004ABC: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83004AC0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83004AC4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83004AC8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83004ACC: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 83004AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004AD8 size=372
    let mut pc: u32 = 0x83004AD8;
    'dispatch: loop {
        match pc {
            0x83004AD8 => {
    //   block [0x83004AD8..0x83004C4C)
	// 83004AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004ADC: 481A3689  bl 0x831a8164
	ctx.lr = 0x83004AE0;
	sub_831A8130(ctx, base);
	// 83004AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004AE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83004AE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83004AEC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83004AF0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004AF4: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 83004AF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004AFC: 4E800421  bctrl
	ctx.lr = 0x83004B00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004B00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004B04: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83004B08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83004B0C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83004B10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004B14: 4E800421  bctrl
	ctx.lr = 0x83004B18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004B18: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83004B1C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83004B20: 419A001C  beq cr6, 0x83004b3c
	if ctx.cr[6].eq {
	pc = 0x83004B3C; continue 'dispatch;
	}
	// 83004B24: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004B28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83004B2C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83004B30: 816B00B4  lwz r11, 0xb4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(180 as u32) ) } as u64;
	// 83004B34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004B38: 4E800421  bctrl
	ctx.lr = 0x83004B3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004B3C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83004B40: 419A00D0  beq cr6, 0x83004c10
	if ctx.cr[6].eq {
	pc = 0x83004C10; continue 'dispatch;
	}
	// 83004B44: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004B48: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83004B4C: 418200C4  beq 0x83004c10
	if ctx.cr[0].eq {
	pc = 0x83004C10; continue 'dispatch;
	}
	// 83004B50: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004B54: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83004B58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83004B5C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83004B60: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 83004B64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004B68: 4E800421  bctrl
	ctx.lr = 0x83004B6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004B6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83004B70: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 83004B74: 3B7E0004  addi r27, r30, 4
	ctx.r[27].s64 = ctx.r[30].s64 + 4;
	// 83004B78: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83004B7C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83004B80: 4BFE1959  bl 0x82fe64d8
	ctx.lr = 0x83004B84;
	sub_82FE64D8(ctx, base);
	// 83004B84: 48000034  b 0x83004bb8
	pc = 0x83004BB8; continue 'dispatch;
	// 83004B88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004B8C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83004B90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83004B94: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 83004B98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004B9C: 4E800421  bctrl
	ctx.lr = 0x83004BA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004BA0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004BA4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83004BA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83004BAC: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83004BB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004BB4: 4E800421  bctrl
	ctx.lr = 0x83004BB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004BB8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004BBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83004BC0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83004BC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004BC8: 4E800421  bctrl
	ctx.lr = 0x83004BCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004BCC: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83004BD0: 4082FFB8  bne 0x83004b88
	if !ctx.cr[0].eq {
	pc = 0x83004B88; continue 'dispatch;
	}
	// 83004BD4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83004BD8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83004BDC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83004BE0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83004BE4: 4BFDAB95  bl 0x82fdf778
	ctx.lr = 0x83004BE8;
	sub_82FDF778(ctx, base);
	// 83004BE8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83004BEC: 419A001C  beq cr6, 0x83004c08
	if ctx.cr[6].eq {
	pc = 0x83004C08; continue 'dispatch;
	}
	// 83004BF0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004BF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83004BF8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83004BFC: 816B00CC  lwz r11, 0xcc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(204 as u32) ) } as u64;
	// 83004C00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004C04: 4E800421  bctrl
	ctx.lr = 0x83004C08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004C08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83004C0C: 48000038  b 0x83004c44
	pc = 0x83004C44; continue 'dispatch;
	// 83004C10: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83004C14: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83004C18: 4BFDCDC1  bl 0x82fe19d8
	ctx.lr = 0x83004C1C;
	sub_82FE19D8(ctx, base);
	// 83004C1C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83004C20: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 83004C24: 419A001C  beq cr6, 0x83004c40
	if ctx.cr[6].eq {
	pc = 0x83004C40; continue 'dispatch;
	}
	// 83004C28: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004C2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83004C30: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83004C34: 816B00B0  lwz r11, 0xb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 83004C38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004C3C: 4E800421  bctrl
	ctx.lr = 0x83004C40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004C40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83004C44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83004C48: 481A356C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004C50 size=12
    let mut pc: u32 = 0x83004C50;
    'dispatch: loop {
        match pc {
            0x83004C50 => {
    //   block [0x83004C50..0x83004C5C)
	// 83004C50: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83004C54: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83004C58: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004C5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004C5C size=12
    let mut pc: u32 = 0x83004C5C;
    'dispatch: loop {
        match pc {
            0x83004C5C => {
    //   block [0x83004C5C..0x83004C68)
	// 83004C5C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83004C60: 386B2F84  addi r3, r11, 0x2f84
	ctx.r[3].s64 = ctx.r[11].s64 + 12164;
	// 83004C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004C68 size=112
    let mut pc: u32 = 0x83004C68;
    'dispatch: loop {
        match pc {
            0x83004C68 => {
    //   block [0x83004C68..0x83004CD8)
	// 83004C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004C70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83004C74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83004C78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004C7C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83004C80: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83004C84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83004C88: 388B9468  addi r4, r11, -0x6b98
	ctx.r[4].s64 = ctx.r[11].s64 + -27544;
	// 83004C8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83004C90: 4BFCEFB1  bl 0x82fd3c40
	ctx.lr = 0x83004C94;
	sub_82FD3C40(ctx, base);
	// 83004C94: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83004C98: 4182001C  beq 0x83004cb4
	if ctx.cr[0].eq {
	pc = 0x83004CB4; continue 'dispatch;
	}
	// 83004C9C: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83004CA0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83004CA4: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83004CA8: 40820018  bne 0x83004cc0
	if !ctx.cr[0].eq {
	pc = 0x83004CC0; continue 'dispatch;
	}
	// 83004CAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83004CB0: 48000010  b 0x83004cc0
	pc = 0x83004CC0; continue 'dispatch;
	// 83004CB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83004CB8: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 83004CBC: 4BFDB85D  bl 0x82fe0518
	ctx.lr = 0x83004CC0;
	sub_82FE0518(ctx, base);
	// 83004CC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83004CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004CCC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83004CD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83004CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004CD8 size=8
    let mut pc: u32 = 0x83004CD8;
    'dispatch: loop {
        match pc {
            0x83004CD8 => {
    //   block [0x83004CD8..0x83004CE0)
	// 83004CD8: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 83004CDC: 4BFF785C  b 0x82ffc538
	sub_82FFC538(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004CE0 size=8
    let mut pc: u32 = 0x83004CE0;
    'dispatch: loop {
        match pc {
            0x83004CE0 => {
    //   block [0x83004CE0..0x83004CE8)
	// 83004CE0: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 83004CE4: 4804DA64  b 0x83052748
	sub_83052748(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004CE8 size=8
    let mut pc: u32 = 0x83004CE8;
    'dispatch: loop {
        match pc {
            0x83004CE8 => {
    //   block [0x83004CE8..0x83004CF0)
	// 83004CE8: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 83004CEC: 4BFF7A54  b 0x82ffc740
	sub_82FFC740(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004CF0 size=8
    let mut pc: u32 = 0x83004CF0;
    'dispatch: loop {
        match pc {
            0x83004CF0 => {
    //   block [0x83004CF0..0x83004CF8)
	// 83004CF0: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 83004CF4: 4BFF8534  b 0x82ffd228
	sub_82FFD228(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004CF8 size=8
    let mut pc: u32 = 0x83004CF8;
    'dispatch: loop {
        match pc {
            0x83004CF8 => {
    //   block [0x83004CF8..0x83004D00)
	// 83004CF8: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 83004CFC: 4BFF78AC  b 0x82ffc5a8
	sub_82FFC5A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004D00 size=8
    let mut pc: u32 = 0x83004D00;
    'dispatch: loop {
        match pc {
            0x83004D00 => {
    //   block [0x83004D00..0x83004D08)
	// 83004D00: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83004D04: 82140830  lwz r16, 0x830(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(2096 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004D08 size=140
    let mut pc: u32 = 0x83004D08;
    'dispatch: loop {
        match pc {
            0x83004D08 => {
    //   block [0x83004D08..0x83004D94)
	// 83004D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004D0C: 481A3459  bl 0x831a8164
	ctx.lr = 0x83004D10;
	sub_831A8130(ctx, base);
	// 83004D10: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83004D14: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004D18: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83004D1C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83004D20: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83004D24: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83004D28: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83004D2C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83004D30: 396B06B8  addi r11, r11, 0x6b8
	ctx.r[11].s64 = ctx.r[11].s64 + 1720;
	// 83004D34: 389C000C  addi r4, r28, 0xc
	ctx.r[4].s64 = ctx.r[28].s64 + 12;
	// 83004D38: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83004D3C: 409A0008  bne cr6, 0x83004d44
	if !ctx.cr[6].eq {
	pc = 0x83004D44; continue 'dispatch;
	}
	// 83004D40: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83004D44: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 83004D48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83004D4C: 4BFDA4CD  bl 0x82fdf218
	ctx.lr = 0x83004D50;
	sub_82FDF218(ctx, base);
	// 83004D50: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83004D54: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 83004D58: 4BFF7741  bl 0x82ffc498
	ctx.lr = 0x83004D5C;
	sub_82FFC498(ctx, base);
	// 83004D5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83004D60: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83004D64: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83004D68: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83004D6C: 4BFDCC6D  bl 0x82fe19d8
	ctx.lr = 0x83004D70;
	sub_82FE19D8(ctx, base);
	// 83004D70: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83004D74: 907E001C  stw r3, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 83004D78: A15D0004  lhz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83004D7C: A16BA6AC  lhz r11, -0x5954(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22868 as u32) ) } as u64;
	// 83004D80: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 83004D84: B17D0004  sth r11, 4(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 83004D88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83004D8C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83004D90: 481A3424  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004D94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004D94 size=40
    let mut pc: u32 = 0x83004D94;
    'dispatch: loop {
        match pc {
            0x83004D94 => {
    //   block [0x83004D94..0x83004DBC)
	// 83004D94: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83004D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004DA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004DA4: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83004DA8: 48000DD9  bl 0x83005b80
	ctx.lr = 0x83004DAC;
	sub_83005B80(ctx, base);
	// 83004DAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83004DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004DBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004DBC size=44
    let mut pc: u32 = 0x83004DBC;
    'dispatch: loop {
        match pc {
            0x83004DBC => {
    //   block [0x83004DBC..0x83004DE8)
	// 83004DBC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83004DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004DC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004DCC: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83004DD0: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83004DD4: 480C2D0D  bl 0x830c7ae0
	ctx.lr = 0x83004DD8;
	sub_830C7AE0(ctx, base);
	// 83004DD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83004DDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004DE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004DE8 size=44
    let mut pc: u32 = 0x83004DE8;
    'dispatch: loop {
        match pc {
            0x83004DE8 => {
    //   block [0x83004DE8..0x83004E14)
	// 83004DE8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83004DEC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004DF0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004DF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004DF8: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83004DFC: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 83004E00: 4BFDC429  bl 0x82fe1228
	ctx.lr = 0x83004E04;
	sub_82FE1228(ctx, base);
	// 83004E04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83004E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004E18 size=8
    let mut pc: u32 = 0x83004E18;
    'dispatch: loop {
        match pc {
            0x83004E18 => {
    //   block [0x83004E18..0x83004E20)
	// 83004E18: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83004E1C: 82140890  lwz r16, 0x890(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(2192 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004E20 size=232
    let mut pc: u32 = 0x83004E20;
    'dispatch: loop {
        match pc {
            0x83004E20 => {
    //   block [0x83004E20..0x83004F08)
	// 83004E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004E24: 481A3341  bl 0x831a8164
	ctx.lr = 0x83004E28;
	sub_831A8130(ctx, base);
	// 83004E28: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83004E2C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004E30: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83004E34: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83004E38: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83004E3C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83004E40: 3B9E0004  addi r28, r30, 4
	ctx.r[28].s64 = ctx.r[30].s64 + 4;
	// 83004E44: 396B06B8  addi r11, r11, 0x6b8
	ctx.r[11].s64 = ctx.r[11].s64 + 1720;
	// 83004E48: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 83004E4C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83004E50: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83004E54: 4BFDB7A5  bl 0x82fe05f8
	ctx.lr = 0x83004E58;
	sub_82FE05F8(ctx, base);
	// 83004E58: 3B7E000C  addi r27, r30, 0xc
	ctx.r[27].s64 = ctx.r[30].s64 + 12;
	// 83004E5C: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 83004E60: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83004E64: 4BFF767D  bl 0x82ffc4e0
	ctx.lr = 0x83004E68;
	sub_82FFC4E0(ctx, base);
	// 83004E68: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 83004E6C: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83004E70: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 83004E74: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83004E78: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83004E7C: A15D0008  lhz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83004E80: A16BA6AC  lhz r11, -0x5954(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22868 as u32) ) } as u64;
	// 83004E84: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83004E88: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 83004E8C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 83004E90: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 83004E94: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83004E98: A15C0004  lhz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83004E9C: 4182000C  beq 0x83004ea8
	if ctx.cr[0].eq {
	pc = 0x83004EA8; continue 'dispatch;
	}
	// 83004EA0: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 83004EA4: 48000008  b 0x83004eac
	pc = 0x83004EAC; continue 'dispatch;
	// 83004EA8: 7D4B5878  andc r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & !ctx.r[11].u64;
	// 83004EAC: B17C0004  sth r11, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 83004EB0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83004EB4: A15D0008  lhz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83004EB8: A16BA6B8  lhz r11, -0x5948(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22856 as u32) ) } as u64;
	// 83004EBC: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83004EC0: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 83004EC4: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 83004EC8: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 83004ECC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83004ED0: 41820020  beq 0x83004ef0
	if ctx.cr[0].eq {
	pc = 0x83004EF0; continue 'dispatch;
	}
	// 83004ED4: A15C0004  lhz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83004ED8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83004EDC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 83004EE0: B17C0004  sth r11, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 83004EE4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004EE8: 806B0028  lwz r3, 0x28(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83004EEC: 4BFEC925  bl 0x82ff1810
	ctx.lr = 0x83004EF0;
	sub_82FF1810(ctx, base);
	// 83004EF0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83004EF4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83004EF8: 4BFF84A1  bl 0x82ffd398
	ctx.lr = 0x83004EFC;
	sub_82FFD398(ctx, base);
	// 83004EFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83004F00: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83004F04: 481A32B0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004F08 size=40
    let mut pc: u32 = 0x83004F08;
    'dispatch: loop {
        match pc {
            0x83004F08 => {
    //   block [0x83004F08..0x83004F30)
	// 83004F08: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83004F0C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004F10: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004F14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004F18: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83004F1C: 48000C65  bl 0x83005b80
	ctx.lr = 0x83004F20;
	sub_83005B80(ctx, base);
	// 83004F20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83004F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004F30 size=44
    let mut pc: u32 = 0x83004F30;
    'dispatch: loop {
        match pc {
            0x83004F30 => {
    //   block [0x83004F30..0x83004F5C)
	// 83004F30: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83004F34: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004F38: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004F3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004F40: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83004F44: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83004F48: 480C2B99  bl 0x830c7ae0
	ctx.lr = 0x83004F4C;
	sub_830C7AE0(ctx, base);
	// 83004F4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83004F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004F5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004F5C size=44
    let mut pc: u32 = 0x83004F5C;
    'dispatch: loop {
        match pc {
            0x83004F5C => {
    //   block [0x83004F5C..0x83004F88)
	// 83004F5C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83004F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004F68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004F6C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83004F70: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 83004F74: 4BFDC2B5  bl 0x82fe1228
	ctx.lr = 0x83004F78;
	sub_82FE1228(ctx, base);
	// 83004F78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83004F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83004F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83004F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83004F88 size=8
    let mut pc: u32 = 0x83004F88;
    'dispatch: loop {
        match pc {
            0x83004F88 => {
    //   block [0x83004F88..0x83004F90)
	// 83004F88: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83004F8C: 821408E0  lwz r16, 0x8e0(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(2272 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83004F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004F90 size=124
    let mut pc: u32 = 0x83004F90;
    'dispatch: loop {
        match pc {
            0x83004F90 => {
    //   block [0x83004F90..0x8300500C)
	// 83004F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004F94: 481A31D9  bl 0x831a816c
	ctx.lr = 0x83004F98;
	sub_831A8130(ctx, base);
	// 83004F98: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83004F9C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004FA0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83004FA4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83004FA8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83004FAC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83004FB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83004FB4: 4E800421  bctrl
	ctx.lr = 0x83004FB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83004FB8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83004FBC: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 83004FC0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83004FC4: 4BFDF8F5  bl 0x82fe48b8
	ctx.lr = 0x83004FC8;
	sub_82FE48B8(ctx, base);
	// 83004FC8: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83004FCC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83004FD0: 41820018  beq 0x83004fe8
	if ctx.cr[0].eq {
	pc = 0x83004FE8; continue 'dispatch;
	}
	// 83004FD4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83004FD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83004FDC: 4BFFFE45  bl 0x83004e20
	ctx.lr = 0x83004FE0;
	sub_83004E20(ctx, base);
	// 83004FE0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83004FE4: 48000008  b 0x83004fec
	pc = 0x83004FEC; continue 'dispatch;
	// 83004FE8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83004FEC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83004FF0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83004FF4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83004FF8: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 83004FFC: 4BFDA77D  bl 0x82fdf778
	ctx.lr = 0x83005000;
	sub_82FDF778(ctx, base);
	// 83005000: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83005004: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83005008: 481A31B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300500C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300500C size=48
    let mut pc: u32 = 0x8300500C;
    'dispatch: loop {
        match pc {
            0x8300500C => {
    //   block [0x8300500C..0x8300503C)
	// 8300500C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83005010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005018: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300501C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83005020: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005024: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83005028: 480C2AB9  bl 0x830c7ae0
	ctx.lr = 0x8300502C;
	sub_830C7AE0(ctx, base);
	// 8300502C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005040 size=8
    let mut pc: u32 = 0x83005040;
    'dispatch: loop {
        match pc {
            0x83005040 => {
    //   block [0x83005040..0x83005048)
	// 83005040: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83005044: 82140928  lwz r16, 0x928(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(2344 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005048 size=308
    let mut pc: u32 = 0x83005048;
    'dispatch: loop {
        match pc {
            0x83005048 => {
    //   block [0x83005048..0x8300517C)
	// 83005048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300504C: 481A3121  bl 0x831a816c
	ctx.lr = 0x83005050;
	sub_831A8130(ctx, base);
	// 83005050: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83005054: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005058: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300505C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83005060: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83005064: 40820010  bne 0x83005074
	if !ctx.cr[0].eq {
	pc = 0x83005074; continue 'dispatch;
	}
	// 83005068: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300506C: 386B8158  addi r3, r11, -0x7ea8
	ctx.r[3].s64 = ctx.r[11].s64 + -32424;
	// 83005070: 48000104  b 0x83005174
	pc = 0x83005174; continue 'dispatch;
	// 83005074: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 83005078: A12B0008  lhz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300507C: A14AA6C0  lhz r10, -0x5940(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 83005080: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 83005084: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 83005088: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8300508C: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 83005090: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83005094: 4182000C  beq 0x830050a0
	if ctx.cr[0].eq {
	pc = 0x830050A0; continue 'dispatch;
	}
	// 83005098: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 8300509C: 48000008  b 0x830050a4
	pc = 0x830050A4; continue 'dispatch;
	// 830050A0: 396B001C  addi r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 + 28;
	// 830050A4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830050A8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830050AC: 4082003C  bne 0x830050e8
	if !ctx.cr[0].eq {
	pc = 0x830050E8; continue 'dispatch;
	}
	// 830050B0: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830050B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830050B8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830050BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830050C0: 4E800421  bctrl
	ctx.lr = 0x830050C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830050C4: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 830050C8: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830050CC: 409A001C  bne cr6, 0x830050e8
	if !ctx.cr[6].eq {
	pc = 0x830050E8; continue 'dispatch;
	}
	// 830050D0: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830050D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830050D8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830050DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830050E0: 4E800421  bctrl
	ctx.lr = 0x830050E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830050E4: 48000090  b 0x83005174
	pc = 0x83005174; continue 'dispatch;
	// 830050E8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830050EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830050F0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830050F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830050F8: 4E800421  bctrl
	ctx.lr = 0x830050FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830050FC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83005100: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 83005104: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83005108: 80AB0090  lwz r5, 0x90(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300510C: 4BFD9D4D  bl 0x82fdee58
	ctx.lr = 0x83005110;
	sub_82FDEE58(ctx, base);
	// 83005110: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 83005114: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83005118: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300511C: 4BFFF525  bl 0x83004640
	ctx.lr = 0x83005120;
	sub_83004640(ctx, base);
	// 83005120: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83005124: 813F0068  lwz r9, 0x68(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83005128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8300512C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83005130: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83005134: 7D4B4B2E  sthx r10, r11, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u16) };
	// 83005138: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300513C: 83BF0068  lwz r29, 0x68(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83005140: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83005144: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83005148: 4E800421  bctrl
	ctx.lr = 0x8300514C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300514C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83005150: 4BFDC889  bl 0x82fe19d8
	ctx.lr = 0x83005154;
	sub_82FE19D8(ctx, base);
	// 83005154: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83005158: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8300515C: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83005160: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005164: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83005168: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300516C: 4E800421  bctrl
	ctx.lr = 0x83005170;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005170: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83005174: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83005178: 481A3044  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300517C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300517C size=40
    let mut pc: u32 = 0x8300517C;
    'dispatch: loop {
        match pc {
            0x8300517C => {
    //   block [0x8300517C..0x830051A4)
	// 8300517C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83005180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005188: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300518C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83005190: 4BFD9D49  bl 0x82fdeed8
	ctx.lr = 0x83005194;
	sub_82FDEED8(ctx, base);
	// 83005194: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300519C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830051A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830051A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830051A8 size=8
    let mut pc: u32 = 0x830051A8;
    'dispatch: loop {
        match pc {
            0x830051A8 => {
    //   block [0x830051A8..0x830051B0)
	// 830051A8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830051AC: 82140A58  lwz r16, 0xa58(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(2648 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830051B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830051B0 size=144
    let mut pc: u32 = 0x830051B0;
    'dispatch: loop {
        match pc {
            0x830051B0 => {
    //   block [0x830051B0..0x83005240)
	// 830051B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830051B4: 481A2FB5  bl 0x831a8168
	ctx.lr = 0x830051B8;
	sub_831A8130(ctx, base);
	// 830051B8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830051BC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830051C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830051C4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830051C8: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830051CC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830051D0: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 830051D4: 396B0968  addi r11, r11, 0x968
	ctx.r[11].s64 = ctx.r[11].s64 + 2408;
	// 830051D8: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 830051DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830051E0: 4BFDB419  bl 0x82fe05f8
	ctx.lr = 0x830051E4;
	sub_82FE05F8(ctx, base);
	// 830051E4: 3B9D000C  addi r28, r29, 0xc
	ctx.r[28].s64 = ctx.r[29].s64 + 12;
	// 830051E8: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 830051EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830051F0: 4BFF72F1  bl 0x82ffc4e0
	ctx.lr = 0x830051F4;
	sub_82FFC4E0(ctx, base);
	// 830051F4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830051F8: A15D0008  lhz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 830051FC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83005200: A16BA6C0  lhz r11, -0x5940(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 83005204: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83005208: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300520C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83005210: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83005214: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83005218: 40820008  bne 0x83005220
	if !ctx.cr[0].eq {
	pc = 0x83005220; continue 'dispatch;
	}
	// 8300521C: 389D001C  addi r4, r29, 0x1c
	ctx.r[4].s64 = ctx.r[29].s64 + 28;
	// 83005220: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 83005224: 4818898D  bl 0x8318dbb0
	ctx.lr = 0x83005228;
	sub_8318DBB0(ctx, base);
	// 83005228: 389D0024  addi r4, r29, 0x24
	ctx.r[4].s64 = ctx.r[29].s64 + 36;
	// 8300522C: 387E0024  addi r3, r30, 0x24
	ctx.r[3].s64 = ctx.r[30].s64 + 36;
	// 83005230: 48033D29  bl 0x83038f58
	ctx.lr = 0x83005234;
	sub_83038F58(ctx, base);
	// 83005234: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83005238: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8300523C: 481A2F7C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005240 size=40
    let mut pc: u32 = 0x83005240;
    'dispatch: loop {
        match pc {
            0x83005240 => {
    //   block [0x83005240..0x83005268)
	// 83005240: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83005244: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005248: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300524C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005250: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83005254: 4800092D  bl 0x83005b80
	ctx.lr = 0x83005258;
	sub_83005B80(ctx, base);
	// 83005258: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300525C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005268 size=44
    let mut pc: u32 = 0x83005268;
    'dispatch: loop {
        match pc {
            0x83005268 => {
    //   block [0x83005268..0x83005294)
	// 83005268: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300526C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005270: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005274: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005278: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300527C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83005280: 480C2861  bl 0x830c7ae0
	ctx.lr = 0x83005284;
	sub_830C7AE0(ctx, base);
	// 83005284: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300528C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005294(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005294 size=44
    let mut pc: u32 = 0x83005294;
    'dispatch: loop {
        match pc {
            0x83005294 => {
    //   block [0x83005294..0x830052C0)
	// 83005294: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83005298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300529C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830052A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830052A4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830052A8: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 830052AC: 4BFDBF7D  bl 0x82fe1228
	ctx.lr = 0x830052B0;
	sub_82FE1228(ctx, base);
	// 830052B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830052B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830052B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830052BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830052C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830052C0 size=44
    let mut pc: u32 = 0x830052C0;
    'dispatch: loop {
        match pc {
            0x830052C0 => {
    //   block [0x830052C0..0x830052EC)
	// 830052C0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830052C4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830052C8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830052CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830052D0: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830052D4: 386B001C  addi r3, r11, 0x1c
	ctx.r[3].s64 = ctx.r[11].s64 + 28;
	// 830052D8: 480C2809  bl 0x830c7ae0
	ctx.lr = 0x830052DC;
	sub_830C7AE0(ctx, base);
	// 830052DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830052E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830052E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830052E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830052F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830052F0 size=8
    let mut pc: u32 = 0x830052F0;
    'dispatch: loop {
        match pc {
            0x830052F0 => {
    //   block [0x830052F0..0x830052F8)
	// 830052F0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830052F4: 82140AC8  lwz r16, 0xac8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(2760 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830052F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830052F8 size=116
    let mut pc: u32 = 0x830052F8;
    'dispatch: loop {
        match pc {
            0x830052F8 => {
    //   block [0x830052F8..0x8300536C)
	// 830052F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830052FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005300: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83005304: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83005308: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8300530C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005310: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83005314: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83005318: 396B0968  addi r11, r11, 0x968
	ctx.r[11].s64 = ctx.r[11].s64 + 2408;
	// 8300531C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83005320: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005324: 387E0024  addi r3, r30, 0x24
	ctx.r[3].s64 = ctx.r[30].s64 + 36;
	// 83005328: 480C27B9  bl 0x830c7ae0
	ctx.lr = 0x8300532C;
	sub_830C7AE0(ctx, base);
	// 8300532C: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 83005330: 480C27B1  bl 0x830c7ae0
	ctx.lr = 0x83005334;
	sub_830C7AE0(ctx, base);
	// 83005334: 397E000C  addi r11, r30, 0xc
	ctx.r[11].s64 = ctx.r[30].s64 + 12;
	// 83005338: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 8300533C: 4BFF4CCD  bl 0x82ffa008
	ctx.lr = 0x83005340;
	sub_82FFA008(ctx, base);
	// 83005340: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 83005344: 480C279D  bl 0x830c7ae0
	ctx.lr = 0x83005348;
	sub_830C7AE0(ctx, base);
	// 83005348: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300534C: 396BA8A0  addi r11, r11, -0x5760
	ctx.r[11].s64 = ctx.r[11].s64 + -22368;
	// 83005350: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005354: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83005358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300535C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005360: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83005364: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83005368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300536C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300536C size=40
    let mut pc: u32 = 0x8300536C;
    'dispatch: loop {
        match pc {
            0x8300536C => {
    //   block [0x8300536C..0x83005394)
	// 8300536C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83005370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300537C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005380: 48000801  bl 0x83005b80
	ctx.lr = 0x83005384;
	sub_83005B80(ctx, base);
	// 83005384: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300538C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005394(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005394 size=44
    let mut pc: u32 = 0x83005394;
    'dispatch: loop {
        match pc {
            0x83005394 => {
    //   block [0x83005394..0x830053C0)
	// 83005394: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83005398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300539C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830053A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830053A4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830053A8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830053AC: 480C2735  bl 0x830c7ae0
	ctx.lr = 0x830053B0;
	sub_830C7AE0(ctx, base);
	// 830053B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830053B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830053B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830053BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830053C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830053C0 size=44
    let mut pc: u32 = 0x830053C0;
    'dispatch: loop {
        match pc {
            0x830053C0 => {
    //   block [0x830053C0..0x830053EC)
	// 830053C0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830053C4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830053C8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830053CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830053D0: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830053D4: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 830053D8: 4BFDBE51  bl 0x82fe1228
	ctx.lr = 0x830053DC;
	sub_82FE1228(ctx, base);
	// 830053DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830053E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830053E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830053E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830053EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830053EC size=44
    let mut pc: u32 = 0x830053EC;
    'dispatch: loop {
        match pc {
            0x830053EC => {
    //   block [0x830053EC..0x83005418)
	// 830053EC: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830053F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830053F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830053F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830053FC: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005400: 386B001C  addi r3, r11, 0x1c
	ctx.r[3].s64 = ctx.r[11].s64 + 28;
	// 83005404: 480C26DD  bl 0x830c7ae0
	ctx.lr = 0x83005408;
	sub_830C7AE0(ctx, base);
	// 83005408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300540C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005418 size=8
    let mut pc: u32 = 0x83005418;
    'dispatch: loop {
        match pc {
            0x83005418 => {
    //   block [0x83005418..0x83005420)
	// 83005418: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300541C: 82140B18  lwz r16, 0xb18(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(2840 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005420 size=124
    let mut pc: u32 = 0x83005420;
    'dispatch: loop {
        match pc {
            0x83005420 => {
    //   block [0x83005420..0x8300549C)
	// 83005420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005424: 481A2D49  bl 0x831a816c
	ctx.lr = 0x83005428;
	sub_831A8130(ctx, base);
	// 83005428: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8300542C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005430: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83005434: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83005438: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300543C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83005440: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83005444: 4E800421  bctrl
	ctx.lr = 0x83005448;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005448: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8300544C: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 83005450: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83005454: 4BFDF465  bl 0x82fe48b8
	ctx.lr = 0x83005458;
	sub_82FE48B8(ctx, base);
	// 83005458: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8300545C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83005460: 41820018  beq 0x83005478
	if ctx.cr[0].eq {
	pc = 0x83005478; continue 'dispatch;
	}
	// 83005464: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83005468: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300546C: 4BFFFD45  bl 0x830051b0
	ctx.lr = 0x83005470;
	sub_830051B0(ctx, base);
	// 83005470: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83005474: 48000008  b 0x8300547c
	pc = 0x8300547C; continue 'dispatch;
	// 83005478: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8300547C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83005480: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83005484: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83005488: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8300548C: 4BFDA2ED  bl 0x82fdf778
	ctx.lr = 0x83005490;
	sub_82FDF778(ctx, base);
	// 83005490: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83005494: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83005498: 481A2D24  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300549C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300549C size=48
    let mut pc: u32 = 0x8300549C;
    'dispatch: loop {
        match pc {
            0x8300549C => {
    //   block [0x8300549C..0x830054CC)
	// 8300549C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830054A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830054A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830054A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830054AC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 830054B0: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830054B4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830054B8: 480C2629  bl 0x830c7ae0
	ctx.lr = 0x830054BC;
	sub_830C7AE0(ctx, base);
	// 830054BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830054C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830054C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830054C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830054D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830054D0 size=12
    let mut pc: u32 = 0x830054D0;
    'dispatch: loop {
        match pc {
            0x830054D0 => {
    //   block [0x830054D0..0x830054DC)
	// 830054D0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830054D4: 386B0B58  addi r3, r11, 0xb58
	ctx.r[3].s64 = ctx.r[11].s64 + 2904;
	// 830054D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830054E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830054E0 size=36
    let mut pc: u32 = 0x830054E0;
    'dispatch: loop {
        match pc {
            0x830054E0 => {
    //   block [0x830054E0..0x83005504)
	// 830054E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830054E4: A1430008  lhz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830054E8: A16BA6B0  lhz r11, -0x5950(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22864 as u32) ) } as u64;
	// 830054EC: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 830054F0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830054F4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830054F8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830054FC: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 83005500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005508 size=120
    let mut pc: u32 = 0x83005508;
    'dispatch: loop {
        match pc {
            0x83005508 => {
    //   block [0x83005508..0x83005580)
	// 83005508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300550C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005510: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83005514: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005518: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300551C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83005520: 419A0038  beq cr6, 0x83005558
	if ctx.cr[6].eq {
	pc = 0x83005558; continue 'dispatch;
	}
	// 83005524: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005528: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300552C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83005530: 4E800421  bctrl
	ctx.lr = 0x83005534;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005534: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83005538: 41820020  beq 0x83005558
	if ctx.cr[0].eq {
	pc = 0x83005558; continue 'dispatch;
	}
	// 8300553C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005540: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83005544: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83005548: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300554C: 4E800421  bctrl
	ctx.lr = 0x83005550;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005550: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83005554: 4800000C  b 0x83005560
	pc = 0x83005560; continue 'dispatch;
	// 83005558: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300555C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83005560: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83005564: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 83005568: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300556C: 4BFF4965  bl 0x82ff9ed0
	ctx.lr = 0x83005570;
	sub_82FF9ED0(ctx, base);
	// 83005570: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83005574: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83005578: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300557C: 481AB6AD  bl 0x831b0c28
	ctx.lr = 0x83005580;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005580 size=384
    let mut pc: u32 = 0x83005580;
    'dispatch: loop {
        match pc {
            0x83005580 => {
    //   block [0x83005580..0x83005700)
	// 83005580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005588: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300558C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83005590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005594: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83005598: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8300559C: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830055A0: A14AA6A4  lhz r10, -0x595c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 830055A4: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 830055A8: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 830055AC: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 830055B0: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 830055B4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830055B8: 41820080  beq 0x83005638
	if ctx.cr[0].eq {
	pc = 0x83005638; continue 'dispatch;
	}
	// 830055BC: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 830055C0: A14AA6C8  lhz r10, -0x5938(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22840 as u32) ) } as u64;
	// 830055C4: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 830055C8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830055CC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830055D0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830055D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830055D8: 40820060  bne 0x83005638
	if !ctx.cr[0].eq {
	pc = 0x83005638; continue 'dispatch;
	}
	// 830055DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830055E0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830055E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830055E8: 4E800421  bctrl
	ctx.lr = 0x830055EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830055EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830055F0: 41820020  beq 0x83005610
	if ctx.cr[0].eq {
	pc = 0x83005610; continue 'dispatch;
	}
	// 830055F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830055F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830055FC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83005600: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83005604: 4E800421  bctrl
	ctx.lr = 0x83005608;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005608: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300560C: 4800000C  b 0x83005618
	pc = 0x83005618; continue 'dispatch;
	// 83005610: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83005614: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83005618: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300561C: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83005620: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83005624: 4BFF48AD  bl 0x82ff9ed0
	ctx.lr = 0x83005628;
	sub_82FF9ED0(ctx, base);
	// 83005628: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300562C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83005630: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83005634: 481AB5F5  bl 0x831b0c28
	ctx.lr = 0x83005638;
	sub_831B0C28(ctx, base);
	// 83005638: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300563C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83005640: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83005644: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83005648: 4E800421  bctrl
	ctx.lr = 0x8300564C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300564C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83005650: 41820050  beq 0x830056a0
	if ctx.cr[0].eq {
	pc = 0x830056A0; continue 'dispatch;
	}
	// 83005654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83005658: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300565C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83005660: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83005664: 4BFDA115  bl 0x82fdf778
	ctx.lr = 0x83005668;
	sub_82FDF778(ctx, base);
	// 83005668: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8300566C: 4BFF701D  bl 0x82ffc688
	ctx.lr = 0x83005670;
	sub_82FFC688(ctx, base);
	// 83005670: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83005674: 48033BD5  bl 0x83039248
	ctx.lr = 0x83005678;
	sub_83039248(ctx, base);
	// 83005678: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8300567C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83005680: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83005684: 4BFE005D  bl 0x82fe56e0
	ctx.lr = 0x83005688;
	sub_82FE56E0(ctx, base);
	// 83005688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8300568C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005694: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83005698: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300569C: 4E800020  blr
	return;
	// 830056A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830056A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830056A8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830056AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830056B0: 4E800421  bctrl
	ctx.lr = 0x830056B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830056B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830056B8: 41820020  beq 0x830056d8
	if ctx.cr[0].eq {
	pc = 0x830056D8; continue 'dispatch;
	}
	// 830056BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830056C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830056C4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830056C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830056CC: 4E800421  bctrl
	ctx.lr = 0x830056D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830056D0: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 830056D4: 4800000C  b 0x830056e0
	pc = 0x830056E0; continue 'dispatch;
	// 830056D8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830056DC: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830056E0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830056E4: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 830056E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830056EC: 4BFF47E5  bl 0x82ff9ed0
	ctx.lr = 0x830056F0;
	sub_82FF9ED0(ctx, base);
	// 830056F0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830056F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830056F8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830056FC: 481AB52D  bl 0x831b0c28
	ctx.lr = 0x83005700;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005700 size=8
    let mut pc: u32 = 0x83005700;
    'dispatch: loop {
        match pc {
            0x83005700 => {
    //   block [0x83005700..0x83005708)
	// 83005700: 38630024  addi r3, r3, 0x24
	ctx.r[3].s64 = ctx.r[3].s64 + 36;
	// 83005704: 480339CC  b 0x830390d0
	sub_830390D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005708 size=12
    let mut pc: u32 = 0x83005708;
    'dispatch: loop {
        match pc {
            0x83005708 => {
    //   block [0x83005708..0x83005714)
	// 83005708: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300570C: 3864001C  addi r3, r4, 0x1c
	ctx.r[3].s64 = ctx.r[4].s64 + 28;
	// 83005710: 4802F200  b 0x83034910
	sub_83034910(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005718 size=8
    let mut pc: u32 = 0x83005718;
    'dispatch: loop {
        match pc {
            0x83005718 => {
    //   block [0x83005718..0x83005720)
	// 83005718: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 8300571C: 4BFF6E74  b 0x82ffc590
	sub_82FFC590(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005720 size=8
    let mut pc: u32 = 0x83005720;
    'dispatch: loop {
        match pc {
            0x83005720 => {
    //   block [0x83005720..0x83005728)
	// 83005720: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 83005724: 4BFF7024  b 0x82ffc748
	sub_82FFC748(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005728 size=8
    let mut pc: u32 = 0x83005728;
    'dispatch: loop {
        match pc {
            0x83005728 => {
    //   block [0x83005728..0x83005730)
	// 83005728: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 8300572C: 4BFDA81C  b 0x82fdff48
	sub_82FDFF48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005730 size=8
    let mut pc: u32 = 0x83005730;
    'dispatch: loop {
        match pc {
            0x83005730 => {
    //   block [0x83005730..0x83005738)
	// 83005730: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 83005734: 4BFDB0CC  b 0x82fe0800
	sub_82FE0800(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005738 size=8
    let mut pc: u32 = 0x83005738;
    'dispatch: loop {
        match pc {
            0x83005738 => {
    //   block [0x83005738..0x83005740)
	// 83005738: 38630024  addi r3, r3, 0x24
	ctx.r[3].s64 = ctx.r[3].s64 + 36;
	// 8300573C: 480339B4  b 0x830390f0
	sub_830390F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005740 size=20
    let mut pc: u32 = 0x83005740;
    'dispatch: loop {
        match pc {
            0x83005740 => {
    //   block [0x83005740..0x83005754)
	// 83005740: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 83005744: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83005748: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300574C: 38630024  addi r3, r3, 0x24
	ctx.r[3].s64 = ctx.r[3].s64 + 36;
	// 83005750: 480339B0  b 0x83039100
	sub_83039100(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005758 size=16
    let mut pc: u32 = 0x83005758;
    'dispatch: loop {
        match pc {
            0x83005758 => {
    //   block [0x83005758..0x83005768)
	// 83005758: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8300575C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83005760: 38630024  addi r3, r3, 0x24
	ctx.r[3].s64 = ctx.r[3].s64 + 36;
	// 83005764: 480338DC  b 0x83039040
	sub_83039040(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005768 size=20
    let mut pc: u32 = 0x83005768;
    'dispatch: loop {
        match pc {
            0x83005768 => {
    //   block [0x83005768..0x8300577C)
	// 83005768: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8300576C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83005770: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83005774: 38630024  addi r3, r3, 0x24
	ctx.r[3].s64 = ctx.r[3].s64 + 36;
	// 83005778: 48033E20  b 0x83039598
	sub_83039598(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005780 size=20
    let mut pc: u32 = 0x83005780;
    'dispatch: loop {
        match pc {
            0x83005780 => {
    //   block [0x83005780..0x83005794)
	// 83005780: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 83005784: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83005788: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300578C: 38630024  addi r3, r3, 0x24
	ctx.r[3].s64 = ctx.r[3].s64 + 36;
	// 83005790: 48033BD0  b 0x83039360
	sub_83039360(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005798 size=24
    let mut pc: u32 = 0x83005798;
    'dispatch: loop {
        match pc {
            0x83005798 => {
    //   block [0x83005798..0x830057B0)
	// 83005798: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8300579C: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 830057A0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 830057A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830057A8: 38630024  addi r3, r3, 0x24
	ctx.r[3].s64 = ctx.r[3].s64 + 36;
	// 830057AC: 4803405C  b 0x83039808
	sub_83039808(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830057B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830057B0 size=16
    let mut pc: u32 = 0x830057B0;
    'dispatch: loop {
        match pc {
            0x830057B0 => {
    //   block [0x830057B0..0x830057C0)
	// 830057B0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 830057B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830057B8: 38630024  addi r3, r3, 0x24
	ctx.r[3].s64 = ctx.r[3].s64 + 36;
	// 830057BC: 480340F4  b 0x830398b0
	sub_830398B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830057C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830057C0 size=16
    let mut pc: u32 = 0x830057C0;
    'dispatch: loop {
        match pc {
            0x830057C0 => {
    //   block [0x830057C0..0x830057D0)
	// 830057C0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 830057C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830057C8: 38630024  addi r3, r3, 0x24
	ctx.r[3].s64 = ctx.r[3].s64 + 36;
	// 830057CC: 48033A8C  b 0x83039258
	sub_83039258(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830057D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830057D0 size=8
    let mut pc: u32 = 0x830057D0;
    'dispatch: loop {
        match pc {
            0x830057D0 => {
    //   block [0x830057D0..0x830057D8)
	// 830057D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830057D4: 82140B98  lwz r16, 0xb98(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(2968 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830057D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830057D8 size=116
    let mut pc: u32 = 0x830057D8;
    'dispatch: loop {
        match pc {
            0x830057D8 => {
    //   block [0x830057D8..0x8300584C)
	// 830057D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830057DC: 481A298D  bl 0x831a8168
	ctx.lr = 0x830057E0;
	sub_831A8130(ctx, base);
	// 830057E0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830057E4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830057E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830057EC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830057F0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830057F4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830057F8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830057FC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83005800: 396B0968  addi r11, r11, 0x968
	ctx.r[11].s64 = ctx.r[11].s64 + 2408;
	// 83005804: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 83005808: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300580C: 409A0008  bne cr6, 0x83005814
	if !ctx.cr[6].eq {
	pc = 0x83005814; continue 'dispatch;
	}
	// 83005810: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83005814: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 83005818: 4BFD9A01  bl 0x82fdf218
	ctx.lr = 0x8300581C;
	sub_82FDF218(ctx, base);
	// 8300581C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83005820: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 83005824: 4BFF6C75  bl 0x82ffc498
	ctx.lr = 0x83005828;
	sub_82FFC498(ctx, base);
	// 83005828: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 8300582C: 48188385  bl 0x8318dbb0
	ctx.lr = 0x83005830;
	sub_8318DBB0(ctx, base);
	// 83005830: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83005834: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83005838: 387E0024  addi r3, r30, 0x24
	ctx.r[3].s64 = ctx.r[30].s64 + 36;
	// 8300583C: 4803365D  bl 0x83038e98
	ctx.lr = 0x83005840;
	sub_83038E98(ctx, base);
	// 83005840: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83005844: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83005848: 481A2970  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300584C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300584C size=40
    let mut pc: u32 = 0x8300584C;
    'dispatch: loop {
        match pc {
            0x8300584C => {
    //   block [0x8300584C..0x83005874)
	// 8300584C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83005850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005858: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300585C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83005860: 48000321  bl 0x83005b80
	ctx.lr = 0x83005864;
	sub_83005B80(ctx, base);
	// 83005864: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300586C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005874(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005874 size=44
    let mut pc: u32 = 0x83005874;
    'dispatch: loop {
        match pc {
            0x83005874 => {
    //   block [0x83005874..0x830058A0)
	// 83005874: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83005878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300587C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005884: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83005888: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8300588C: 480C2255  bl 0x830c7ae0
	ctx.lr = 0x83005890;
	sub_830C7AE0(ctx, base);
	// 83005890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300589C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830058A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830058A0 size=44
    let mut pc: u32 = 0x830058A0;
    'dispatch: loop {
        match pc {
            0x830058A0 => {
    //   block [0x830058A0..0x830058CC)
	// 830058A0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830058A4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830058A8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830058AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830058B0: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830058B4: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 830058B8: 4BFDB971  bl 0x82fe1228
	ctx.lr = 0x830058BC;
	sub_82FE1228(ctx, base);
	// 830058BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830058C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830058C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830058C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830058CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830058CC size=44
    let mut pc: u32 = 0x830058CC;
    'dispatch: loop {
        match pc {
            0x830058CC => {
    //   block [0x830058CC..0x830058F8)
	// 830058CC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830058D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830058D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830058D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830058DC: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830058E0: 386B001C  addi r3, r11, 0x1c
	ctx.r[3].s64 = ctx.r[11].s64 + 28;
	// 830058E4: 480C21FD  bl 0x830c7ae0
	ctx.lr = 0x830058E8;
	sub_830C7AE0(ctx, base);
	// 830058E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830058EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830058F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830058F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830058F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830058F8 size=76
    let mut pc: u32 = 0x830058F8;
    'dispatch: loop {
        match pc {
            0x830058F8 => {
    //   block [0x830058F8..0x83005944)
	// 830058F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830058FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005900: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83005904: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83005908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300590C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83005910: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83005914: 4BFFF9E5  bl 0x830052f8
	ctx.lr = 0x83005918;
	sub_830052F8(ctx, base);
	// 83005918: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300591C: 4182000C  beq 0x83005928
	if ctx.cr[0].eq {
	pc = 0x83005928; continue 'dispatch;
	}
	// 83005920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83005924: 4B2BA945  bl 0x822c0268
	ctx.lr = 0x83005928;
	sub_822C0268(ctx, base);
	// 83005928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300592C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83005930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005938: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300593C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83005940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005948 size=564
    let mut pc: u32 = 0x83005948;
    'dispatch: loop {
        match pc {
            0x83005948 => {
    //   block [0x83005948..0x83005B7C)
	// 83005948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300594C: 481A2815  bl 0x831a8160
	ctx.lr = 0x83005950;
	sub_831A8130(ctx, base);
	// 83005950: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005954: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83005958: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300595C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83005960: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83005964: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83005968: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8300596C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83005970: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83005974: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83005978: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300597C: 41820060  beq 0x830059dc
	if ctx.cr[0].eq {
	pc = 0x830059DC; continue 'dispatch;
	}
	// 83005980: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005984: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83005988: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300598C: 4E800421  bctrl
	ctx.lr = 0x83005990;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005990: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83005994: 41820020  beq 0x830059b4
	if ctx.cr[0].eq {
	pc = 0x830059B4; continue 'dispatch;
	}
	// 83005998: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300599C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830059A0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830059A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830059A8: 4E800421  bctrl
	ctx.lr = 0x830059AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830059AC: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 830059B0: 4800000C  b 0x830059bc
	pc = 0x830059BC; continue 'dispatch;
	// 830059B4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830059B8: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830059BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830059C0: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 830059C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830059C8: 4BFF4509  bl 0x82ff9ed0
	ctx.lr = 0x830059CC;
	sub_82FF9ED0(ctx, base);
	// 830059CC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830059D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830059D4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830059D8: 481AB251  bl 0x831b0c28
	ctx.lr = 0x830059DC;
	sub_831B0C28(ctx, base);
	// 830059DC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830059E0: 838B0004  lwz r28, 4(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830059E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830059E8: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830059EC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830059F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830059F4: 41990138  bgt cr6, 0x83005b2c
	if ctx.cr[6].gt {
	pc = 0x83005B2C; continue 'dispatch;
	}
	// 830059F8: 4E800421  bctrl
	ctx.lr = 0x830059FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830059FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005A00: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83005A04: 7CBDE050  subf r5, r29, r28
	ctx.r[5].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 83005A08: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83005A0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83005A10: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 83005A14: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005A18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83005A1C: 4E800421  bctrl
	ctx.lr = 0x83005A20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005A20: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83005A24: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83005A28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83005A2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83005A30: 4E800421  bctrl
	ctx.lr = 0x83005A34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005A34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005A38: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83005A3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83005A40: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83005A44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83005A48: 4E800421  bctrl
	ctx.lr = 0x83005A4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005A4C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83005A50: 41820034  beq 0x83005a84
	if ctx.cr[0].eq {
	pc = 0x83005A84; continue 'dispatch;
	}
	// 83005A54: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005A58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83005A5C: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005A60: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83005A64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83005A68: 4E800421  bctrl
	ctx.lr = 0x83005A6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005A6C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83005A70: 817C0034  lwz r11, 0x34(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(52 as u32) ) } as u64;
	// 83005A74: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83005A78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83005A7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83005A80: 4E800421  bctrl
	ctx.lr = 0x83005A84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005A84: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83005A88: 57AA083C  slwi r10, r29, 1
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83005A8C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83005A90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83005A94: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005A98: 7FCA4B2E  sthx r30, r10, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u16) };
	// 83005A9C: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 83005AA0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005AA4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83005AA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83005AAC: 4E800421  bctrl
	ctx.lr = 0x83005AB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005AB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83005AB4: 4182006C  beq 0x83005b20
	if ctx.cr[0].eq {
	pc = 0x83005B20; continue 'dispatch;
	}
	// 83005AB8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005ABC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83005AC0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83005AC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83005AC8: 4E800421  bctrl
	ctx.lr = 0x83005ACC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005ACC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005AD0: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 83005AD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83005AD8: 4E800421  bctrl
	ctx.lr = 0x83005ADC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005ADC: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83005AE0: 41820040  beq 0x83005b20
	if ctx.cr[0].eq {
	pc = 0x83005B20; continue 'dispatch;
	}
	// 83005AE4: 839B0008  lwz r28, 8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 83005AE8: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83005AEC: 41820034  beq 0x83005b20
	if ctx.cr[0].eq {
	pc = 0x83005B20; continue 'dispatch;
	}
	// 83005AF0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83005AF4: 419A002C  beq cr6, 0x83005b20
	if ctx.cr[6].eq {
	pc = 0x83005B20; continue 'dispatch;
	}
	// 83005AF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83005AFC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83005B00: 48026D71  bl 0x8302c870
	ctx.lr = 0x83005B04;
	sub_8302C870(ctx, base);
	// 83005B04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83005B08: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83005B0C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83005B10: 4BFFB329  bl 0x83000e38
	ctx.lr = 0x83005B14;
	sub_83000E38(ctx, base);
	// 83005B14: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83005B18: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83005B1C: 4198FFDC  blt cr6, 0x83005af8
	if ctx.cr[6].lt {
	pc = 0x83005AF8; continue 'dispatch;
	}
	// 83005B20: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83005B24: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83005B28: 481A2688  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83005B2C: 4E800421  bctrl
	ctx.lr = 0x83005B30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005B30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83005B34: 41820020  beq 0x83005b54
	if ctx.cr[0].eq {
	pc = 0x83005B54; continue 'dispatch;
	}
	// 83005B38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005B3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83005B40: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83005B44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83005B48: 4E800421  bctrl
	ctx.lr = 0x83005B4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005B4C: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83005B50: 4800000C  b 0x83005b5c
	pc = 0x83005B5C; continue 'dispatch;
	// 83005B54: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83005B58: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83005B5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83005B60: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83005B64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83005B68: 4BFF4369  bl 0x82ff9ed0
	ctx.lr = 0x83005B6C;
	sub_82FF9ED0(ctx, base);
	// 83005B6C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83005B70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83005B74: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83005B78: 481AB0B1  bl 0x831b0c28
	ctx.lr = 0x83005B7C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005B80 size=16
    let mut pc: u32 = 0x83005B80;
    'dispatch: loop {
        match pc {
            0x83005B80 => {
    //   block [0x83005B80..0x83005B90)
	// 83005B80: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83005B84: 396BA8A0  addi r11, r11, -0x5760
	ctx.r[11].s64 = ctx.r[11].s64 + -22368;
	// 83005B88: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005B8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005B90 size=8
    let mut pc: u32 = 0x83005B90;
    'dispatch: loop {
        match pc {
            0x83005B90 => {
    //   block [0x83005B90..0x83005B98)
	// 83005B90: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83005B94: 82140CC0  lwz r16, 0xcc0(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(3264 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005B98 size=128
    let mut pc: u32 = 0x83005B98;
    'dispatch: loop {
        match pc {
            0x83005B98 => {
    //   block [0x83005B98..0x83005C18)
	// 83005B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005B9C: 481A25C9  bl 0x831a8164
	ctx.lr = 0x83005BA0;
	sub_831A8130(ctx, base);
	// 83005BA0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83005BA4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005BA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83005BAC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83005BB0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83005BB4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83005BB8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83005BBC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83005BC0: 396B0BE8  addi r11, r11, 0xbe8
	ctx.r[11].s64 = ctx.r[11].s64 + 3048;
	// 83005BC4: 389C000C  addi r4, r28, 0xc
	ctx.r[4].s64 = ctx.r[28].s64 + 12;
	// 83005BC8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005BCC: 409A0008  bne cr6, 0x83005bd4
	if !ctx.cr[6].eq {
	pc = 0x83005BD4; continue 'dispatch;
	}
	// 83005BD0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83005BD4: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 83005BD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83005BDC: 4BFD963D  bl 0x82fdf218
	ctx.lr = 0x83005BE0;
	sub_82FDF218(ctx, base);
	// 83005BE0: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 83005BE4: 48187FCD  bl 0x8318dbb0
	ctx.lr = 0x83005BE8;
	sub_8318DBB0(ctx, base);
	// 83005BE8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83005BEC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83005BF0: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 83005BF4: 480332A5  bl 0x83038e98
	ctx.lr = 0x83005BF8;
	sub_83038E98(ctx, base);
	// 83005BF8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83005BFC: A15D0004  lhz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83005C00: A16BA6C0  lhz r11, -0x5940(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 83005C04: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 83005C08: B17D0004  sth r11, 4(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 83005C0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83005C10: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83005C14: 481A25A0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005C18 size=40
    let mut pc: u32 = 0x83005C18;
    'dispatch: loop {
        match pc {
            0x83005C18 => {
    //   block [0x83005C18..0x83005C40)
	// 83005C18: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83005C1C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005C20: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005C24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005C28: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83005C2C: 4BFFFF55  bl 0x83005b80
	ctx.lr = 0x83005C30;
	sub_83005B80(ctx, base);
	// 83005C30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005C40 size=44
    let mut pc: u32 = 0x83005C40;
    'dispatch: loop {
        match pc {
            0x83005C40 => {
    //   block [0x83005C40..0x83005C6C)
	// 83005C40: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83005C44: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005C48: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005C4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005C50: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83005C54: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83005C58: 480C1E89  bl 0x830c7ae0
	ctx.lr = 0x83005C5C;
	sub_830C7AE0(ctx, base);
	// 83005C5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005C6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005C6C size=44
    let mut pc: u32 = 0x83005C6C;
    'dispatch: loop {
        match pc {
            0x83005C6C => {
    //   block [0x83005C6C..0x83005C98)
	// 83005C6C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83005C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005C78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005C7C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83005C80: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 83005C84: 480C1E5D  bl 0x830c7ae0
	ctx.lr = 0x83005C88;
	sub_830C7AE0(ctx, base);
	// 83005C88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005C98 size=8
    let mut pc: u32 = 0x83005C98;
    'dispatch: loop {
        match pc {
            0x83005C98 => {
    //   block [0x83005C98..0x83005CA0)
	// 83005C98: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83005C9C: 82140D20  lwz r16, 0xd20(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(3360 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005CA0 size=112
    let mut pc: u32 = 0x83005CA0;
    'dispatch: loop {
        match pc {
            0x83005CA0 => {
    //   block [0x83005CA0..0x83005D10)
	// 83005CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005CA4: 481A24C5  bl 0x831a8168
	ctx.lr = 0x83005CA8;
	sub_831A8130(ctx, base);
	// 83005CA8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83005CAC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005CB0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83005CB4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83005CB8: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83005CBC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83005CC0: 3B9E0004  addi r28, r30, 4
	ctx.r[28].s64 = ctx.r[30].s64 + 4;
	// 83005CC4: 396B0BE8  addi r11, r11, 0xbe8
	ctx.r[11].s64 = ctx.r[11].s64 + 3048;
	// 83005CC8: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 83005CCC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83005CD0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005CD4: 4BFDA925  bl 0x82fe05f8
	ctx.lr = 0x83005CD8;
	sub_82FE05F8(ctx, base);
	// 83005CD8: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 83005CDC: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 83005CE0: 48187ED1  bl 0x8318dbb0
	ctx.lr = 0x83005CE4;
	sub_8318DBB0(ctx, base);
	// 83005CE4: 389D0014  addi r4, r29, 0x14
	ctx.r[4].s64 = ctx.r[29].s64 + 20;
	// 83005CE8: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 83005CEC: 4803326D  bl 0x83038f58
	ctx.lr = 0x83005CF0;
	sub_83038F58(ctx, base);
	// 83005CF0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83005CF4: A15C0004  lhz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83005CF8: A16BA6C0  lhz r11, -0x5940(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 83005CFC: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 83005D00: B17C0004  sth r11, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 83005D04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83005D08: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83005D0C: 481A24AC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005D10 size=40
    let mut pc: u32 = 0x83005D10;
    'dispatch: loop {
        match pc {
            0x83005D10 => {
    //   block [0x83005D10..0x83005D38)
	// 83005D10: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83005D14: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005D18: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005D1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005D20: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83005D24: 4BFFFE5D  bl 0x83005b80
	ctx.lr = 0x83005D28;
	sub_83005B80(ctx, base);
	// 83005D28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005D38 size=44
    let mut pc: u32 = 0x83005D38;
    'dispatch: loop {
        match pc {
            0x83005D38 => {
    //   block [0x83005D38..0x83005D64)
	// 83005D38: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83005D3C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005D40: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005D44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005D48: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83005D4C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83005D50: 480C1D91  bl 0x830c7ae0
	ctx.lr = 0x83005D54;
	sub_830C7AE0(ctx, base);
	// 83005D54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005D64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005D64 size=44
    let mut pc: u32 = 0x83005D64;
    'dispatch: loop {
        match pc {
            0x83005D64 => {
    //   block [0x83005D64..0x83005D90)
	// 83005D64: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83005D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005D70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005D74: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83005D78: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 83005D7C: 480C1D65  bl 0x830c7ae0
	ctx.lr = 0x83005D80;
	sub_830C7AE0(ctx, base);
	// 83005D80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005D90 size=8
    let mut pc: u32 = 0x83005D90;
    'dispatch: loop {
        match pc {
            0x83005D90 => {
    //   block [0x83005D90..0x83005D98)
	// 83005D90: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83005D94: 82140D80  lwz r16, 0xd80(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(3456 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005D98 size=104
    let mut pc: u32 = 0x83005D98;
    'dispatch: loop {
        match pc {
            0x83005D98 => {
    //   block [0x83005D98..0x83005E00)
	// 83005D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005DA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83005DA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83005DA8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83005DAC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005DB0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83005DB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83005DB8: 396B0BE8  addi r11, r11, 0xbe8
	ctx.r[11].s64 = ctx.r[11].s64 + 3048;
	// 83005DBC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83005DC0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005DC4: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 83005DC8: 480C1D19  bl 0x830c7ae0
	ctx.lr = 0x83005DCC;
	sub_830C7AE0(ctx, base);
	// 83005DCC: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 83005DD0: 480C1D11  bl 0x830c7ae0
	ctx.lr = 0x83005DD4;
	sub_830C7AE0(ctx, base);
	// 83005DD4: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 83005DD8: 480C1D09  bl 0x830c7ae0
	ctx.lr = 0x83005DDC;
	sub_830C7AE0(ctx, base);
	// 83005DDC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83005DE0: 396BA8A0  addi r11, r11, -0x5760
	ctx.r[11].s64 = ctx.r[11].s64 + -22368;
	// 83005DE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005DE8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83005DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005DF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83005DF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83005DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005E00 size=40
    let mut pc: u32 = 0x83005E00;
    'dispatch: loop {
        match pc {
            0x83005E00 => {
    //   block [0x83005E00..0x83005E28)
	// 83005E00: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83005E04: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005E08: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005E0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005E10: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005E14: 4BFFFD6D  bl 0x83005b80
	ctx.lr = 0x83005E18;
	sub_83005B80(ctx, base);
	// 83005E18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005E28 size=44
    let mut pc: u32 = 0x83005E28;
    'dispatch: loop {
        match pc {
            0x83005E28 => {
    //   block [0x83005E28..0x83005E54)
	// 83005E28: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83005E2C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005E30: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005E34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005E38: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005E3C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83005E40: 480C1CA1  bl 0x830c7ae0
	ctx.lr = 0x83005E44;
	sub_830C7AE0(ctx, base);
	// 83005E44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005E54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005E54 size=44
    let mut pc: u32 = 0x83005E54;
    'dispatch: loop {
        match pc {
            0x83005E54 => {
    //   block [0x83005E54..0x83005E80)
	// 83005E54: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83005E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005E60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005E64: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005E68: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 83005E6C: 480C1C75  bl 0x830c7ae0
	ctx.lr = 0x83005E70;
	sub_830C7AE0(ctx, base);
	// 83005E70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005E80 size=8
    let mut pc: u32 = 0x83005E80;
    'dispatch: loop {
        match pc {
            0x83005E80 => {
    //   block [0x83005E80..0x83005E88)
	// 83005E80: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83005E84: 82140DC8  lwz r16, 0xdc8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(3528 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005E88 size=124
    let mut pc: u32 = 0x83005E88;
    'dispatch: loop {
        match pc {
            0x83005E88 => {
    //   block [0x83005E88..0x83005F04)
	// 83005E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005E8C: 481A22E1  bl 0x831a816c
	ctx.lr = 0x83005E90;
	sub_831A8130(ctx, base);
	// 83005E90: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83005E94: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005E98: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83005E9C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83005EA0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005EA4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83005EA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83005EAC: 4E800421  bctrl
	ctx.lr = 0x83005EB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005EB0: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 83005EB4: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 83005EB8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83005EBC: 4BFDE9FD  bl 0x82fe48b8
	ctx.lr = 0x83005EC0;
	sub_82FE48B8(ctx, base);
	// 83005EC0: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83005EC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83005EC8: 41820018  beq 0x83005ee0
	if ctx.cr[0].eq {
	pc = 0x83005EE0; continue 'dispatch;
	}
	// 83005ECC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83005ED0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83005ED4: 4BFFFDCD  bl 0x83005ca0
	ctx.lr = 0x83005ED8;
	sub_83005CA0(ctx, base);
	// 83005ED8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83005EDC: 48000008  b 0x83005ee4
	pc = 0x83005EE4; continue 'dispatch;
	// 83005EE0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83005EE4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83005EE8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83005EEC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83005EF0: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 83005EF4: 4BFD9885  bl 0x82fdf778
	ctx.lr = 0x83005EF8;
	sub_82FDF778(ctx, base);
	// 83005EF8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83005EFC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83005F00: 481A22BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005F04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005F04 size=48
    let mut pc: u32 = 0x83005F04;
    'dispatch: loop {
        match pc {
            0x83005F04 => {
    //   block [0x83005F04..0x83005F34)
	// 83005F04: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83005F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005F10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005F14: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 83005F18: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005F1C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83005F20: 480C1BC1  bl 0x830c7ae0
	ctx.lr = 0x83005F24;
	sub_830C7AE0(ctx, base);
	// 83005F24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005F38 size=12
    let mut pc: u32 = 0x83005F38;
    'dispatch: loop {
        match pc {
            0x83005F38 => {
    //   block [0x83005F38..0x83005F44)
	// 83005F38: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83005F3C: 386B0E08  addi r3, r11, 0xe08
	ctx.r[3].s64 = ctx.r[11].s64 + 3592;
	// 83005F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83005F48 size=8
    let mut pc: u32 = 0x83005F48;
    'dispatch: loop {
        match pc {
            0x83005F48 => {
    //   block [0x83005F48..0x83005F50)
	// 83005F48: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 83005F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83005F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005F50 size=376
    let mut pc: u32 = 0x83005F50;
    'dispatch: loop {
        match pc {
            0x83005F50 => {
    //   block [0x83005F50..0x830060C8)
	// 83005F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005F58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83005F5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83005F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005F64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83005F68: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 83005F6C: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83005F70: A14AA6A4  lhz r10, -0x595c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 83005F74: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83005F78: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 83005F7C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 83005F80: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 83005F84: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83005F88: 41820080  beq 0x83006008
	if ctx.cr[0].eq {
	pc = 0x83006008; continue 'dispatch;
	}
	// 83005F8C: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 83005F90: A14AA6C8  lhz r10, -0x5938(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22840 as u32) ) } as u64;
	// 83005F94: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83005F98: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83005F9C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83005FA0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83005FA4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83005FA8: 40820060  bne 0x83006008
	if !ctx.cr[0].eq {
	pc = 0x83006008; continue 'dispatch;
	}
	// 83005FAC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005FB0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83005FB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83005FB8: 4E800421  bctrl
	ctx.lr = 0x83005FBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005FBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83005FC0: 41820020  beq 0x83005fe0
	if ctx.cr[0].eq {
	pc = 0x83005FE0; continue 'dispatch;
	}
	// 83005FC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005FC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83005FCC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83005FD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83005FD4: 4E800421  bctrl
	ctx.lr = 0x83005FD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005FD8: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83005FDC: 4800000C  b 0x83005fe8
	pc = 0x83005FE8; continue 'dispatch;
	// 83005FE0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83005FE4: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83005FE8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83005FEC: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83005FF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83005FF4: 4BFF3EDD  bl 0x82ff9ed0
	ctx.lr = 0x83005FF8;
	sub_82FF9ED0(ctx, base);
	// 83005FF8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83005FFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006000: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83006004: 481AAC25  bl 0x831b0c28
	ctx.lr = 0x83006008;
	sub_831B0C28(ctx, base);
	// 83006008: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300600C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006010: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006014: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006018: 4E800421  bctrl
	ctx.lr = 0x8300601C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300601C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83006020: 41820048  beq 0x83006068
	if ctx.cr[0].eq {
	pc = 0x83006068; continue 'dispatch;
	}
	// 83006024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83006028: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300602C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83006030: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83006034: 4BFD9745  bl 0x82fdf778
	ctx.lr = 0x83006038;
	sub_82FDF778(ctx, base);
	// 83006038: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8300603C: 4803320D  bl 0x83039248
	ctx.lr = 0x83006040;
	sub_83039248(ctx, base);
	// 83006040: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 83006044: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83006048: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300604C: 4BFDF695  bl 0x82fe56e0
	ctx.lr = 0x83006050;
	sub_82FE56E0(ctx, base);
	// 83006050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83006054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83006058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300605C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83006060: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83006064: 4E800020  blr
	return;
	// 83006068: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300606C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006070: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006074: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006078: 4E800421  bctrl
	ctx.lr = 0x8300607C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300607C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83006080: 41820020  beq 0x830060a0
	if ctx.cr[0].eq {
	pc = 0x830060A0; continue 'dispatch;
	}
	// 83006084: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006088: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300608C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006090: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006094: 4E800421  bctrl
	ctx.lr = 0x83006098;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006098: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300609C: 4800000C  b 0x830060a8
	pc = 0x830060A8; continue 'dispatch;
	// 830060A0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830060A4: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830060A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830060AC: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 830060B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830060B4: 4BFF3E1D  bl 0x82ff9ed0
	ctx.lr = 0x830060B8;
	sub_82FF9ED0(ctx, base);
	// 830060B8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830060BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830060C0: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830060C4: 481AAB65  bl 0x831b0c28
	ctx.lr = 0x830060C8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830060C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830060C8 size=8
    let mut pc: u32 = 0x830060C8;
    'dispatch: loop {
        match pc {
            0x830060C8 => {
    //   block [0x830060C8..0x830060D0)
	// 830060C8: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 830060CC: 4BFD915C  b 0x82fdf228
	sub_82FDF228(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830060D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830060D0 size=8
    let mut pc: u32 = 0x830060D0;
    'dispatch: loop {
        match pc {
            0x830060D0 => {
    //   block [0x830060D0..0x830060D8)
	// 830060D0: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 830060D4: 4BFD91FC  b 0x82fdf2d0
	sub_82FDF2D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830060D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830060D8 size=20
    let mut pc: u32 = 0x830060D8;
    'dispatch: loop {
        match pc {
            0x830060D8 => {
    //   block [0x830060D8..0x830060EC)
	// 830060D8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 830060DC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 830060E0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830060E4: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 830060E8: 48033018  b 0x83039100
	sub_83039100(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830060F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830060F0 size=16
    let mut pc: u32 = 0x830060F0;
    'dispatch: loop {
        match pc {
            0x830060F0 => {
    //   block [0x830060F0..0x83006100)
	// 830060F0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 830060F4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830060F8: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 830060FC: 48032F44  b 0x83039040
	sub_83039040(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006100 size=16
    let mut pc: u32 = 0x83006100;
    'dispatch: loop {
        match pc {
            0x83006100 => {
    //   block [0x83006100..0x83006110)
	// 83006100: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83006104: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83006108: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 8300610C: 480337A4  b 0x830398b0
	sub_830398B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006110 size=76
    let mut pc: u32 = 0x83006110;
    'dispatch: loop {
        match pc {
            0x83006110 => {
    //   block [0x83006110..0x8300615C)
	// 83006110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006118: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300611C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83006120: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006124: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83006128: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300612C: 4BFFFC6D  bl 0x83005d98
	ctx.lr = 0x83006130;
	sub_83005D98(ctx, base);
	// 83006130: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83006134: 4182000C  beq 0x83006140
	if ctx.cr[0].eq {
	pc = 0x83006140; continue 'dispatch;
	}
	// 83006138: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300613C: 4B2BA12D  bl 0x822c0268
	ctx.lr = 0x83006140;
	sub_822C0268(ctx, base);
	// 83006140: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006144: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83006148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300614C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83006150: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83006154: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83006158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006160 size=564
    let mut pc: u32 = 0x83006160;
    'dispatch: loop {
        match pc {
            0x83006160 => {
    //   block [0x83006160..0x83006394)
	// 83006160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006164: 481A1FFD  bl 0x831a8160
	ctx.lr = 0x83006168;
	sub_831A8130(ctx, base);
	// 83006168: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300616C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83006170: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83006174: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83006178: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300617C: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83006180: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83006184: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83006188: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8300618C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83006190: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83006194: 41820060  beq 0x830061f4
	if ctx.cr[0].eq {
	pc = 0x830061F4; continue 'dispatch;
	}
	// 83006198: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300619C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830061A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830061A4: 4E800421  bctrl
	ctx.lr = 0x830061A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830061A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830061AC: 41820020  beq 0x830061cc
	if ctx.cr[0].eq {
	pc = 0x830061CC; continue 'dispatch;
	}
	// 830061B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830061B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830061B8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830061BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830061C0: 4E800421  bctrl
	ctx.lr = 0x830061C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830061C4: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 830061C8: 4800000C  b 0x830061d4
	pc = 0x830061D4; continue 'dispatch;
	// 830061CC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830061D0: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830061D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830061D8: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 830061DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830061E0: 4BFF3CF1  bl 0x82ff9ed0
	ctx.lr = 0x830061E4;
	sub_82FF9ED0(ctx, base);
	// 830061E4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830061E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830061EC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830061F0: 481AAA39  bl 0x831b0c28
	ctx.lr = 0x830061F4;
	sub_831B0C28(ctx, base);
	// 830061F4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830061F8: 838B0004  lwz r28, 4(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830061FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006200: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83006204: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006208: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300620C: 41990138  bgt cr6, 0x83006344
	if ctx.cr[6].gt {
	pc = 0x83006344; continue 'dispatch;
	}
	// 83006210: 4E800421  bctrl
	ctx.lr = 0x83006214;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006214: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006218: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300621C: 7CBDE050  subf r5, r29, r28
	ctx.r[5].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 83006220: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83006224: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006228: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300622C: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006230: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006234: 4E800421  bctrl
	ctx.lr = 0x83006238;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006238: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300623C: 817C0014  lwz r11, 0x14(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 83006240: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83006244: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006248: 4E800421  bctrl
	ctx.lr = 0x8300624C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300624C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006250: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83006254: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006258: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300625C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006260: 4E800421  bctrl
	ctx.lr = 0x83006264;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006264: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83006268: 41820034  beq 0x8300629c
	if ctx.cr[0].eq {
	pc = 0x8300629C; continue 'dispatch;
	}
	// 8300626C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006274: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006278: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300627C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006280: 4E800421  bctrl
	ctx.lr = 0x83006284;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006284: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83006288: 817C0034  lwz r11, 0x34(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(52 as u32) ) } as u64;
	// 8300628C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83006290: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83006294: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006298: 4E800421  bctrl
	ctx.lr = 0x8300629C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300629C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830062A0: 57AA083C  slwi r10, r29, 1
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830062A4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830062A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830062AC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830062B0: 7FCA4B2E  sthx r30, r10, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u16) };
	// 830062B4: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 830062B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830062BC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830062C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830062C4: 4E800421  bctrl
	ctx.lr = 0x830062C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830062C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830062CC: 4182006C  beq 0x83006338
	if ctx.cr[0].eq {
	pc = 0x83006338; continue 'dispatch;
	}
	// 830062D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830062D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830062D8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830062DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830062E0: 4E800421  bctrl
	ctx.lr = 0x830062E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830062E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830062E8: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 830062EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830062F0: 4E800421  bctrl
	ctx.lr = 0x830062F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830062F4: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 830062F8: 41820040  beq 0x83006338
	if ctx.cr[0].eq {
	pc = 0x83006338; continue 'dispatch;
	}
	// 830062FC: 839B0008  lwz r28, 8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 83006300: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83006304: 41820034  beq 0x83006338
	if ctx.cr[0].eq {
	pc = 0x83006338; continue 'dispatch;
	}
	// 83006308: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8300630C: 419A002C  beq cr6, 0x83006338
	if ctx.cr[6].eq {
	pc = 0x83006338; continue 'dispatch;
	}
	// 83006310: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83006314: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83006318: 48026559  bl 0x8302c870
	ctx.lr = 0x8300631C;
	sub_8302C870(ctx, base);
	// 8300631C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83006320: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83006324: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83006328: 4BFFAB11  bl 0x83000e38
	ctx.lr = 0x8300632C;
	sub_83000E38(ctx, base);
	// 8300632C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83006330: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83006334: 4198FFDC  blt cr6, 0x83006310
	if ctx.cr[6].lt {
	pc = 0x83006310; continue 'dispatch;
	}
	// 83006338: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8300633C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83006340: 481A1E70  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83006344: 4E800421  bctrl
	ctx.lr = 0x83006348;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006348: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300634C: 41820020  beq 0x8300636c
	if ctx.cr[0].eq {
	pc = 0x8300636C; continue 'dispatch;
	}
	// 83006350: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006354: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006358: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300635C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006360: 4E800421  bctrl
	ctx.lr = 0x83006364;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006364: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83006368: 4800000C  b 0x83006374
	pc = 0x83006374; continue 'dispatch;
	// 8300636C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83006370: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83006374: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83006378: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8300637C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006380: 4BFF3B51  bl 0x82ff9ed0
	ctx.lr = 0x83006384;
	sub_82FF9ED0(ctx, base);
	// 83006384: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83006388: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300638C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83006390: 481AA899  bl 0x831b0c28
	ctx.lr = 0x83006394;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006398 size=8
    let mut pc: u32 = 0x83006398;
    'dispatch: loop {
        match pc {
            0x83006398 => {
    //   block [0x83006398..0x830063A0)
	// 83006398: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300639C: 82140ED8  lwz r16, 0xed8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(3800 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830063A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830063A0 size=104
    let mut pc: u32 = 0x830063A0;
    'dispatch: loop {
        match pc {
            0x830063A0 => {
    //   block [0x830063A0..0x83006408)
	// 830063A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830063A4: 481A1DC1  bl 0x831a8164
	ctx.lr = 0x830063A8;
	sub_831A8130(ctx, base);
	// 830063A8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830063AC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830063B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830063B4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830063B8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830063BC: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830063C0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830063C4: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 830063C8: 396B0E20  addi r11, r11, 0xe20
	ctx.r[11].s64 = ctx.r[11].s64 + 3616;
	// 830063CC: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 830063D0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830063D4: 4BFDA225  bl 0x82fe05f8
	ctx.lr = 0x830063D8;
	sub_82FE05F8(ctx, base);
	// 830063D8: 3B9E000C  addi r28, r30, 0xc
	ctx.r[28].s64 = ctx.r[30].s64 + 12;
	// 830063DC: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 830063E0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830063E4: 4BFF60FD  bl 0x82ffc4e0
	ctx.lr = 0x830063E8;
	sub_82FFC4E0(ctx, base);
	// 830063E8: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830063EC: 41820010  beq 0x830063fc
	if ctx.cr[0].eq {
	pc = 0x830063FC; continue 'dispatch;
	}
	// 830063F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830063F4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830063F8: 4BFF6FA1  bl 0x82ffd398
	ctx.lr = 0x830063FC;
	sub_82FFD398(ctx, base);
	// 830063FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83006400: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83006404: 481A1DB0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006408 size=40
    let mut pc: u32 = 0x83006408;
    'dispatch: loop {
        match pc {
            0x83006408 => {
    //   block [0x83006408..0x83006430)
	// 83006408: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300640C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006410: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006414: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006418: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300641C: 4BFFF765  bl 0x83005b80
	ctx.lr = 0x83006420;
	sub_83005B80(ctx, base);
	// 83006420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83006424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83006428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300642C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006430 size=44
    let mut pc: u32 = 0x83006430;
    'dispatch: loop {
        match pc {
            0x83006430 => {
    //   block [0x83006430..0x8300645C)
	// 83006430: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83006434: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006438: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300643C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006440: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83006444: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83006448: 480C1699  bl 0x830c7ae0
	ctx.lr = 0x8300644C;
	sub_830C7AE0(ctx, base);
	// 8300644C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83006450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83006454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83006458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300645C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300645C size=44
    let mut pc: u32 = 0x8300645C;
    'dispatch: loop {
        match pc {
            0x8300645C => {
    //   block [0x8300645C..0x83006488)
	// 8300645C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83006460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300646C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83006470: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 83006474: 4BFDADB5  bl 0x82fe1228
	ctx.lr = 0x83006478;
	sub_82FE1228(ctx, base);
	// 83006478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300647C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83006480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83006484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006488 size=8
    let mut pc: u32 = 0x83006488;
    'dispatch: loop {
        match pc {
            0x83006488 => {
    //   block [0x83006488..0x83006490)
	// 83006488: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300648C: 82140F30  lwz r16, 0xf30(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(3888 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006490 size=100
    let mut pc: u32 = 0x83006490;
    'dispatch: loop {
        match pc {
            0x83006490 => {
    //   block [0x83006490..0x830064F4)
	// 83006490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006498: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300649C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830064A0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830064A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830064A8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830064AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830064B0: 396B0E20  addi r11, r11, 0xe20
	ctx.r[11].s64 = ctx.r[11].s64 + 3616;
	// 830064B4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830064B8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830064BC: 397E000C  addi r11, r30, 0xc
	ctx.r[11].s64 = ctx.r[30].s64 + 12;
	// 830064C0: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 830064C4: 4BFF3B45  bl 0x82ffa008
	ctx.lr = 0x830064C8;
	sub_82FFA008(ctx, base);
	// 830064C8: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 830064CC: 480C1615  bl 0x830c7ae0
	ctx.lr = 0x830064D0;
	sub_830C7AE0(ctx, base);
	// 830064D0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830064D4: 396BA8A0  addi r11, r11, -0x5760
	ctx.r[11].s64 = ctx.r[11].s64 + -22368;
	// 830064D8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830064DC: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830064E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830064E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830064E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830064EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830064F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830064F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830064F4 size=40
    let mut pc: u32 = 0x830064F4;
    'dispatch: loop {
        match pc {
            0x830064F4 => {
    //   block [0x830064F4..0x8300651C)
	// 830064F4: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830064F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830064FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006500: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006504: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83006508: 4BFFF679  bl 0x83005b80
	ctx.lr = 0x8300650C;
	sub_83005B80(ctx, base);
	// 8300650C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83006510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83006514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83006518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300651C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300651C size=44
    let mut pc: u32 = 0x8300651C;
    'dispatch: loop {
        match pc {
            0x8300651C => {
    //   block [0x8300651C..0x83006548)
	// 8300651C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83006520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300652C: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83006530: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83006534: 480C15AD  bl 0x830c7ae0
	ctx.lr = 0x83006538;
	sub_830C7AE0(ctx, base);
	// 83006538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300653C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83006540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83006544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006548 size=8
    let mut pc: u32 = 0x83006548;
    'dispatch: loop {
        match pc {
            0x83006548 => {
    //   block [0x83006548..0x83006550)
	// 83006548: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300654C: 82140F70  lwz r16, 0xf70(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(3952 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006550 size=120
    let mut pc: u32 = 0x83006550;
    'dispatch: loop {
        match pc {
            0x83006550 => {
    //   block [0x83006550..0x830065C8)
	// 83006550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006554: 481A1C15  bl 0x831a8168
	ctx.lr = 0x83006558;
	sub_831A8130(ctx, base);
	// 83006558: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8300655C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006560: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83006564: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83006568: 3B9E0004  addi r28, r30, 4
	ctx.r[28].s64 = ctx.r[30].s64 + 4;
	// 8300656C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83006570: 4BFD8CB9  bl 0x82fdf228
	ctx.lr = 0x83006574;
	sub_82FDF228(ctx, base);
	// 83006574: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83006578: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 8300657C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83006580: 4BFDE339  bl 0x82fe48b8
	ctx.lr = 0x83006584;
	sub_82FE48B8(ctx, base);
	// 83006584: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83006588: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300658C: 41820018  beq 0x830065a4
	if ctx.cr[0].eq {
	pc = 0x830065A4; continue 'dispatch;
	}
	// 83006590: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83006594: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83006598: 4BFFFE09  bl 0x830063a0
	ctx.lr = 0x8300659C;
	sub_830063A0(ctx, base);
	// 8300659C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830065A0: 48000008  b 0x830065a8
	pc = 0x830065A8; continue 'dispatch;
	// 830065A4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830065A8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830065AC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830065B0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830065B4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830065B8: 4BFD91C1  bl 0x82fdf778
	ctx.lr = 0x830065BC;
	sub_82FDF778(ctx, base);
	// 830065BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830065C0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830065C4: 481A1BF4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830065C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830065C8 size=48
    let mut pc: u32 = 0x830065C8;
    'dispatch: loop {
        match pc {
            0x830065C8 => {
    //   block [0x830065C8..0x830065F8)
	// 830065C8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830065CC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830065D0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830065D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830065D8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830065DC: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830065E0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830065E4: 480C14FD  bl 0x830c7ae0
	ctx.lr = 0x830065E8;
	sub_830C7AE0(ctx, base);
	// 830065E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830065EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830065F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830065F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830065F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830065F8 size=12
    let mut pc: u32 = 0x830065F8;
    'dispatch: loop {
        match pc {
            0x830065F8 => {
    //   block [0x830065F8..0x83006604)
	// 830065F8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830065FC: 386B0FB0  addi r3, r11, 0xfb0
	ctx.r[3].s64 = ctx.r[11].s64 + 4016;
	// 83006600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006608 size=8
    let mut pc: u32 = 0x83006608;
    'dispatch: loop {
        match pc {
            0x83006608 => {
    //   block [0x83006608..0x83006610)
	// 83006608: 3860000B  li r3, 0xb
	ctx.r[3].s64 = 11;
	// 8300660C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006610 size=376
    let mut pc: u32 = 0x83006610;
    'dispatch: loop {
        match pc {
            0x83006610 => {
    //   block [0x83006610..0x83006788)
	// 83006610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006618: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300661C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83006620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006624: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83006628: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8300662C: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83006630: A14AA6A4  lhz r10, -0x595c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 83006634: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83006638: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8300663C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 83006640: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 83006644: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83006648: 41820080  beq 0x830066c8
	if ctx.cr[0].eq {
	pc = 0x830066C8; continue 'dispatch;
	}
	// 8300664C: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 83006650: A14AA6C8  lhz r10, -0x5938(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22840 as u32) ) } as u64;
	// 83006654: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83006658: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300665C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83006660: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83006664: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83006668: 40820060  bne 0x830066c8
	if !ctx.cr[0].eq {
	pc = 0x830066C8; continue 'dispatch;
	}
	// 8300666C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006670: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006674: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006678: 4E800421  bctrl
	ctx.lr = 0x8300667C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300667C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83006680: 41820020  beq 0x830066a0
	if ctx.cr[0].eq {
	pc = 0x830066A0; continue 'dispatch;
	}
	// 83006684: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006688: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300668C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006690: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006694: 4E800421  bctrl
	ctx.lr = 0x83006698;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006698: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300669C: 4800000C  b 0x830066a8
	pc = 0x830066A8; continue 'dispatch;
	// 830066A0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830066A4: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830066A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830066AC: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 830066B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830066B4: 4BFF381D  bl 0x82ff9ed0
	ctx.lr = 0x830066B8;
	sub_82FF9ED0(ctx, base);
	// 830066B8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830066BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830066C0: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830066C4: 481AA565  bl 0x831b0c28
	ctx.lr = 0x830066C8;
	sub_831B0C28(ctx, base);
	// 830066C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830066CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830066D0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830066D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830066D8: 4E800421  bctrl
	ctx.lr = 0x830066DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830066DC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830066E0: 41820048  beq 0x83006728
	if ctx.cr[0].eq {
	pc = 0x83006728; continue 'dispatch;
	}
	// 830066E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830066E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830066EC: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 830066F0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 830066F4: 4BFD9085  bl 0x82fdf778
	ctx.lr = 0x830066F8;
	sub_82FDF778(ctx, base);
	// 830066F8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 830066FC: 4BFF5F8D  bl 0x82ffc688
	ctx.lr = 0x83006700;
	sub_82FFC688(ctx, base);
	// 83006700: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83006704: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83006708: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300670C: 4BFDEFD5  bl 0x82fe56e0
	ctx.lr = 0x83006710;
	sub_82FE56E0(ctx, base);
	// 83006710: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83006714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83006718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300671C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83006720: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83006724: 4E800020  blr
	return;
	// 83006728: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300672C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006730: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006734: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006738: 4E800421  bctrl
	ctx.lr = 0x8300673C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300673C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83006740: 41820020  beq 0x83006760
	if ctx.cr[0].eq {
	pc = 0x83006760; continue 'dispatch;
	}
	// 83006744: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006748: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300674C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006750: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006754: 4E800421  bctrl
	ctx.lr = 0x83006758;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006758: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300675C: 4800000C  b 0x83006768
	pc = 0x83006768; continue 'dispatch;
	// 83006760: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83006764: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83006768: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300676C: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83006770: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006774: 4BFF375D  bl 0x82ff9ed0
	ctx.lr = 0x83006778;
	sub_82FF9ED0(ctx, base);
	// 83006778: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300677C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006780: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83006784: 481AA4A5  bl 0x831b0c28
	ctx.lr = 0x83006788;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006788 size=8
    let mut pc: u32 = 0x83006788;
    'dispatch: loop {
        match pc {
            0x83006788 => {
    //   block [0x83006788..0x83006790)
	// 83006788: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 8300678C: 4BFF6C04  b 0x82ffd390
	sub_82FFD390(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006790 size=8
    let mut pc: u32 = 0x83006790;
    'dispatch: loop {
        match pc {
            0x83006790 => {
    //   block [0x83006790..0x83006798)
	// 83006790: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 83006794: 48177CF4  b 0x8317e488
	sub_8317E488(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006798 size=8
    let mut pc: u32 = 0x83006798;
    'dispatch: loop {
        match pc {
            0x83006798 => {
    //   block [0x83006798..0x830067A0)
	// 83006798: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 8300679C: 4BFD8CE4  b 0x82fdf480
	sub_82FDF480(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830067A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830067A0 size=8
    let mut pc: u32 = 0x830067A0;
    'dispatch: loop {
        match pc {
            0x830067A0 => {
    //   block [0x830067A0..0x830067A8)
	// 830067A0: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 830067A4: 4BFD8D2C  b 0x82fdf4d0
	sub_82FDF4D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830067A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830067A8 size=8
    let mut pc: u32 = 0x830067A8;
    'dispatch: loop {
        match pc {
            0x830067A8 => {
    //   block [0x830067A8..0x830067B0)
	// 830067A8: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 830067AC: 4BFDA154  b 0x82fe0900
	sub_82FE0900(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830067B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830067B0 size=8
    let mut pc: u32 = 0x830067B0;
    'dispatch: loop {
        match pc {
            0x830067B0 => {
    //   block [0x830067B0..0x830067B8)
	// 830067B0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830067B4: 82140FE8  lwz r16, 0xfe8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(4072 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830067B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830067B8 size=88
    let mut pc: u32 = 0x830067B8;
    'dispatch: loop {
        match pc {
            0x830067B8 => {
    //   block [0x830067B8..0x83006810)
	// 830067B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830067BC: 481A19B1  bl 0x831a816c
	ctx.lr = 0x830067C0;
	sub_831A8130(ctx, base);
	// 830067C0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830067C4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830067C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830067CC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830067D0: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830067D4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830067D8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830067DC: 396B0E20  addi r11, r11, 0xe20
	ctx.r[11].s64 = ctx.r[11].s64 + 3616;
	// 830067E0: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 830067E4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830067E8: 409A0008  bne cr6, 0x830067f0
	if !ctx.cr[6].eq {
	pc = 0x830067F0; continue 'dispatch;
	}
	// 830067EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830067F0: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 830067F4: 4BFD8A25  bl 0x82fdf218
	ctx.lr = 0x830067F8;
	sub_82FDF218(ctx, base);
	// 830067F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830067FC: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 83006800: 4BFF5C99  bl 0x82ffc498
	ctx.lr = 0x83006804;
	sub_82FFC498(ctx, base);
	// 83006804: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83006808: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8300680C: 481A19B0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006810 size=40
    let mut pc: u32 = 0x83006810;
    'dispatch: loop {
        match pc {
            0x83006810 => {
    //   block [0x83006810..0x83006838)
	// 83006810: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83006814: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006818: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300681C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006820: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83006824: 4BFFF35D  bl 0x83005b80
	ctx.lr = 0x83006828;
	sub_83005B80(ctx, base);
	// 83006828: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300682C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83006830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83006834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006838 size=44
    let mut pc: u32 = 0x83006838;
    'dispatch: loop {
        match pc {
            0x83006838 => {
    //   block [0x83006838..0x83006864)
	// 83006838: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8300683C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006840: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006844: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006848: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300684C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83006850: 480C1291  bl 0x830c7ae0
	ctx.lr = 0x83006854;
	sub_830C7AE0(ctx, base);
	// 83006854: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83006858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300685C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83006860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006868 size=76
    let mut pc: u32 = 0x83006868;
    'dispatch: loop {
        match pc {
            0x83006868 => {
    //   block [0x83006868..0x830068B4)
	// 83006868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300686C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006870: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83006874: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83006878: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300687C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83006880: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83006884: 4BFFFC0D  bl 0x83006490
	ctx.lr = 0x83006888;
	sub_83006490(ctx, base);
	// 83006888: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300688C: 4182000C  beq 0x83006898
	if ctx.cr[0].eq {
	pc = 0x83006898; continue 'dispatch;
	}
	// 83006890: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006894: 4B2B99D5  bl 0x822c0268
	ctx.lr = 0x83006898;
	sub_822C0268(ctx, base);
	// 83006898: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300689C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830068A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830068A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830068A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830068AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830068B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830068B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830068B8 size=8
    let mut pc: u32 = 0x830068B8;
    'dispatch: loop {
        match pc {
            0x830068B8 => {
    //   block [0x830068B8..0x830068C0)
	// 830068B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830068BC: 82141160  lwz r16, 0x1160(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(4448 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830068C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830068C0 size=108
    let mut pc: u32 = 0x830068C0;
    'dispatch: loop {
        match pc {
            0x830068C0 => {
    //   block [0x830068C0..0x8300692C)
	// 830068C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830068C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830068C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830068CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830068D0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830068D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830068D8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830068DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830068E0: 396B1040  addi r11, r11, 0x1040
	ctx.r[11].s64 = ctx.r[11].s64 + 4160;
	// 830068E4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830068E8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830068EC: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 830068F0: 480C11F1  bl 0x830c7ae0
	ctx.lr = 0x830068F4;
	sub_830C7AE0(ctx, base);
	// 830068F4: 397E000C  addi r11, r30, 0xc
	ctx.r[11].s64 = ctx.r[30].s64 + 12;
	// 830068F8: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 830068FC: 4BFF370D  bl 0x82ffa008
	ctx.lr = 0x83006900;
	sub_82FFA008(ctx, base);
	// 83006900: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 83006904: 480C11DD  bl 0x830c7ae0
	ctx.lr = 0x83006908;
	sub_830C7AE0(ctx, base);
	// 83006908: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300690C: 396BA8A0  addi r11, r11, -0x5760
	ctx.r[11].s64 = ctx.r[11].s64 + -22368;
	// 83006910: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83006914: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83006918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300691C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83006920: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83006924: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83006928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300692C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300692C size=40
    let mut pc: u32 = 0x8300692C;
    'dispatch: loop {
        match pc {
            0x8300692C => {
    //   block [0x8300692C..0x83006954)
	// 8300692C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83006930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300693C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83006940: 4BFFF241  bl 0x83005b80
	ctx.lr = 0x83006944;
	sub_83005B80(ctx, base);
	// 83006944: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83006948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300694C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83006950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006954(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006954 size=44
    let mut pc: u32 = 0x83006954;
    'dispatch: loop {
        match pc {
            0x83006954 => {
    //   block [0x83006954..0x83006980)
	// 83006954: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83006958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300695C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006960: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006964: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83006968: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8300696C: 480C1175  bl 0x830c7ae0
	ctx.lr = 0x83006970;
	sub_830C7AE0(ctx, base);
	// 83006970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83006974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83006978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300697C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006980 size=44
    let mut pc: u32 = 0x83006980;
    'dispatch: loop {
        match pc {
            0x83006980 => {
    //   block [0x83006980..0x830069AC)
	// 83006980: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83006984: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006988: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300698C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006990: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83006994: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 83006998: 4BFDA891  bl 0x82fe1228
	ctx.lr = 0x8300699C;
	sub_82FE1228(ctx, base);
	// 8300699C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830069A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830069A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830069A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830069B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830069B0 size=84
    let mut pc: u32 = 0x830069B0;
    'dispatch: loop {
        match pc {
            0x830069B0 => {
    //   block [0x830069B0..0x83006A04)
	// 830069B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830069B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830069B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830069BC: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 830069C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830069C4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830069C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830069CC: 4E800421  bctrl
	ctx.lr = 0x830069D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830069D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830069D4: 41820018  beq 0x830069ec
	if ctx.cr[0].eq {
	pc = 0x830069EC; continue 'dispatch;
	}
	// 830069D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830069DC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830069E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830069E4: 4E800421  bctrl
	ctx.lr = 0x830069E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830069E8: 4800000C  b 0x830069f4
	pc = 0x830069F4; continue 'dispatch;
	// 830069EC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830069F0: 386B8158  addi r3, r11, -0x7ea8
	ctx.r[3].s64 = ctx.r[11].s64 + -32424;
	// 830069F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830069F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830069FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83006A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006A08 size=20
    let mut pc: u32 = 0x83006A08;
    'dispatch: loop {
        match pc {
            0x83006A08 => {
    //   block [0x83006A08..0x83006A1C)
	// 83006A08: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 83006A0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006A10: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83006A14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006A18: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006A20 size=80
    let mut pc: u32 = 0x83006A20;
    'dispatch: loop {
        match pc {
            0x83006A20 => {
    //   block [0x83006A20..0x83006A70)
	// 83006A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006A28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83006A2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83006A30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006A34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83006A38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83006A3C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006A40: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006A44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006A48: 4E800421  bctrl
	ctx.lr = 0x83006A4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006A4C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83006A50: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83006A54: 4BFDDB2D  bl 0x82fe4580
	ctx.lr = 0x83006A58;
	sub_82FE4580(ctx, base);
	// 83006A58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83006A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83006A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83006A64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83006A68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83006A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006A70 size=260
    let mut pc: u32 = 0x83006A70;
    'dispatch: loop {
        match pc {
            0x83006A70 => {
    //   block [0x83006A70..0x83006B74)
	// 83006A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006A78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83006A7C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006A80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83006A84: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83006A88: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83006A8C: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83006A90: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83006A94: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83006A98: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83006A9C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83006AA0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83006AA4: 41820060  beq 0x83006b04
	if ctx.cr[0].eq {
	pc = 0x83006B04; continue 'dispatch;
	}
	// 83006AA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006AAC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006AB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006AB4: 4E800421  bctrl
	ctx.lr = 0x83006AB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006AB8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83006ABC: 41820020  beq 0x83006adc
	if ctx.cr[0].eq {
	pc = 0x83006ADC; continue 'dispatch;
	}
	// 83006AC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006AC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006AC8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006ACC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006AD0: 4E800421  bctrl
	ctx.lr = 0x83006AD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006AD4: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83006AD8: 4800000C  b 0x83006ae4
	pc = 0x83006AE4; continue 'dispatch;
	// 83006ADC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83006AE0: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83006AE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83006AE8: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83006AEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006AF0: 4BFF33E1  bl 0x82ff9ed0
	ctx.lr = 0x83006AF4;
	sub_82FF9ED0(ctx, base);
	// 83006AF4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83006AF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006AFC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83006B00: 481AA129  bl 0x831b0c28
	ctx.lr = 0x83006B04;
	sub_831B0C28(ctx, base);
	// 83006B04: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83006B08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006B0C: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 83006B10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006B14: 4E800421  bctrl
	ctx.lr = 0x83006B18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006B18: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83006B1C: 41800044  blt 0x83006b60
	if ctx.cr[0].lt {
	pc = 0x83006B60; continue 'dispatch;
	}
	// 83006B20: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83006B24: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83006B28: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006B2C: 816A003C  lwz r11, 0x3c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 83006B30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006B34: 4E800421  bctrl
	ctx.lr = 0x83006B38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006B38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83006B3C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006B40: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 83006B44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006B48: 4E800421  bctrl
	ctx.lr = 0x83006B4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006B4C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006B50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006B54: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 83006B58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006B5C: 4E800421  bctrl
	ctx.lr = 0x83006B60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83006B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83006B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83006B6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83006B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006B78 size=560
    let mut pc: u32 = 0x83006B78;
    'dispatch: loop {
        match pc {
            0x83006B78 => {
    //   block [0x83006B78..0x83006DA8)
	// 83006B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006B7C: 481A15ED  bl 0x831a8168
	ctx.lr = 0x83006B80;
	sub_831A8130(ctx, base);
	// 83006B80: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006B84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83006B88: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83006B8C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83006B90: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83006B94: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83006B98: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83006B9C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83006BA0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83006BA4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83006BA8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83006BAC: 41820060  beq 0x83006c0c
	if ctx.cr[0].eq {
	pc = 0x83006C0C; continue 'dispatch;
	}
	// 83006BB0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006BB4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006BB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006BBC: 4E800421  bctrl
	ctx.lr = 0x83006BC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006BC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83006BC4: 41820020  beq 0x83006be4
	if ctx.cr[0].eq {
	pc = 0x83006BE4; continue 'dispatch;
	}
	// 83006BC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006BCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006BD0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006BD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006BD8: 4E800421  bctrl
	ctx.lr = 0x83006BDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006BDC: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83006BE0: 4800000C  b 0x83006bec
	pc = 0x83006BEC; continue 'dispatch;
	// 83006BE4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83006BE8: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83006BEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83006BF0: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83006BF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006BF8: 4BFF32D9  bl 0x82ff9ed0
	ctx.lr = 0x83006BFC;
	sub_82FF9ED0(ctx, base);
	// 83006BFC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83006C00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006C04: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83006C08: 481AA021  bl 0x831b0c28
	ctx.lr = 0x83006C0C;
	sub_831B0C28(ctx, base);
	// 83006C0C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006C10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83006C14: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 83006C18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006C1C: 4E800421  bctrl
	ctx.lr = 0x83006C20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006C20: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83006C24: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006C28: 83AA0000  lwz r29, 0(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006C2C: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83006C30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83006C34: 4182002C  beq 0x83006c60
	if ctx.cr[0].eq {
	pc = 0x83006C60; continue 'dispatch;
	}
	// 83006C38: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 83006C3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006C40: 4E800421  bctrl
	ctx.lr = 0x83006C44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006C44: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83006C48: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83006C4C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83006C50: 817D0034  lwz r11, 0x34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 83006C54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006C58: 4E800421  bctrl
	ctx.lr = 0x83006C5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006C5C: 48000024  b 0x83006c80
	pc = 0x83006C80; continue 'dispatch;
	// 83006C60: 816B009C  lwz r11, 0x9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 83006C64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006C68: 4E800421  bctrl
	ctx.lr = 0x83006C6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006C6C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83006C70: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83006C74: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 83006C78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006C7C: 4E800421  bctrl
	ctx.lr = 0x83006C80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006C80: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83006C84: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83006C88: 419800C0  blt cr6, 0x83006d48
	if ctx.cr[6].lt {
	pc = 0x83006D48; continue 'dispatch;
	}
	// 83006C8C: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83006C90: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83006C94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006C98: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83006C9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006CA0: 4E800421  bctrl
	ctx.lr = 0x83006CA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006CA4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83006CA8: 7F1CF040  cmplw cr6, r28, r30
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83006CAC: 409A003C  bne cr6, 0x83006ce8
	if !ctx.cr[6].eq {
	pc = 0x83006CE8; continue 'dispatch;
	}
	// 83006CB0: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83006CB4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83006CB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006CBC: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 83006CC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006CC4: 4E800421  bctrl
	ctx.lr = 0x83006CC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006CC8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006CCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83006CD0: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 83006CD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006CD8: 4E800421  bctrl
	ctx.lr = 0x83006CDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006CDC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83006CE0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83006CE4: 481A14D4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83006CE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006CEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006CF0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006CF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006CF8: 4E800421  bctrl
	ctx.lr = 0x83006CFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006CFC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83006D00: 41820020  beq 0x83006d20
	if ctx.cr[0].eq {
	pc = 0x83006D20; continue 'dispatch;
	}
	// 83006D04: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006D08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006D0C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006D10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006D14: 4E800421  bctrl
	ctx.lr = 0x83006D18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006D18: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83006D1C: 4800000C  b 0x83006d28
	pc = 0x83006D28; continue 'dispatch;
	// 83006D20: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83006D24: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83006D28: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83006D2C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 83006D30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006D34: 4BFF319D  bl 0x82ff9ed0
	ctx.lr = 0x83006D38;
	sub_82FF9ED0(ctx, base);
	// 83006D38: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83006D3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006D40: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83006D44: 481A9EE5  bl 0x831b0c28
	ctx.lr = 0x83006D48;
	sub_831B0C28(ctx, base);
	// 83006D48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006D4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006D50: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006D54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006D58: 4E800421  bctrl
	ctx.lr = 0x83006D5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006D5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83006D60: 41820020  beq 0x83006d80
	if ctx.cr[0].eq {
	pc = 0x83006D80; continue 'dispatch;
	}
	// 83006D64: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006D68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006D6C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006D70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006D74: 4E800421  bctrl
	ctx.lr = 0x83006D78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006D78: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83006D7C: 4800000C  b 0x83006d88
	pc = 0x83006D88; continue 'dispatch;
	// 83006D80: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83006D84: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83006D88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83006D8C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 83006D90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006D94: 4BFF313D  bl 0x82ff9ed0
	ctx.lr = 0x83006D98;
	sub_82FF9ED0(ctx, base);
	// 83006D98: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83006D9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006DA0: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83006DA4: 481A9E85  bl 0x831b0c28
	ctx.lr = 0x83006DA8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006DA8 size=268
    let mut pc: u32 = 0x83006DA8;
    'dispatch: loop {
        match pc {
            0x83006DA8 => {
    //   block [0x83006DA8..0x83006EB4)
	// 83006DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006DAC: 481A13BD  bl 0x831a8168
	ctx.lr = 0x83006DB0;
	sub_831A8130(ctx, base);
	// 83006DB0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006DB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83006DB8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83006DBC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83006DC0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83006DC4: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83006DC8: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83006DCC: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83006DD0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83006DD4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83006DD8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83006DDC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83006DE0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006DE4: 4182005C  beq 0x83006e40
	if ctx.cr[0].eq {
	pc = 0x83006E40; continue 'dispatch;
	}
	// 83006DE8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006DEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006DF0: 4E800421  bctrl
	ctx.lr = 0x83006DF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006DF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83006DF8: 41820020  beq 0x83006e18
	if ctx.cr[0].eq {
	pc = 0x83006E18; continue 'dispatch;
	}
	// 83006DFC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006E00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006E04: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006E08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006E0C: 4E800421  bctrl
	ctx.lr = 0x83006E10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006E10: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83006E14: 4800000C  b 0x83006e20
	pc = 0x83006E20; continue 'dispatch;
	// 83006E18: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83006E1C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83006E20: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83006E24: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83006E28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006E2C: 4BFF30A5  bl 0x82ff9ed0
	ctx.lr = 0x83006E30;
	sub_82FF9ED0(ctx, base);
	// 83006E30: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83006E34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006E38: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83006E3C: 481A9DED  bl 0x831b0c28
	ctx.lr = 0x83006E40;
	sub_831B0C28(ctx, base);
	// 83006E40: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 83006E44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83006E48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006E4C: 4E800421  bctrl
	ctx.lr = 0x83006E50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006E50: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83006E54: 40820040  bne 0x83006e94
	if !ctx.cr[0].eq {
	pc = 0x83006E94; continue 'dispatch;
	}
	// 83006E58: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83006E5C: 4BFD83CD  bl 0x82fdf228
	ctx.lr = 0x83006E60;
	sub_82FDF228(ctx, base);
	// 83006E60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006E64: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83006E68: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83006E6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006E70: 4E800421  bctrl
	ctx.lr = 0x83006E74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006E74: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83006E78: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83006E7C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83006E80: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83006E84: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006E88: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83006E8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006E90: 4E800421  bctrl
	ctx.lr = 0x83006E94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006E94: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006E98: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83006E9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83006EA0: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 83006EA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006EA8: 4E800421  bctrl
	ctx.lr = 0x83006EAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006EAC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83006EB0: 481A1308  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006EB8 size=300
    let mut pc: u32 = 0x83006EB8;
    'dispatch: loop {
        match pc {
            0x83006EB8 => {
    //   block [0x83006EB8..0x83006FE4)
	// 83006EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006EC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83006EC4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006EC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83006ECC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83006ED0: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83006ED4: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83006ED8: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83006EDC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83006EE0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83006EE4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83006EE8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83006EEC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006EF0: 4182005C  beq 0x83006f4c
	if ctx.cr[0].eq {
	pc = 0x83006F4C; continue 'dispatch;
	}
	// 83006EF4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006EF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006EFC: 4E800421  bctrl
	ctx.lr = 0x83006F00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006F00: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83006F04: 41820020  beq 0x83006f24
	if ctx.cr[0].eq {
	pc = 0x83006F24; continue 'dispatch;
	}
	// 83006F08: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006F0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006F10: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006F14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006F18: 4E800421  bctrl
	ctx.lr = 0x83006F1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006F1C: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83006F20: 4800000C  b 0x83006f2c
	pc = 0x83006F2C; continue 'dispatch;
	// 83006F24: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83006F28: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83006F2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83006F30: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83006F34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006F38: 4BFF2F99  bl 0x82ff9ed0
	ctx.lr = 0x83006F3C;
	sub_82FF9ED0(ctx, base);
	// 83006F3C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83006F40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006F44: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83006F48: 481A9CE1  bl 0x831b0c28
	ctx.lr = 0x83006F4C;
	sub_831B0C28(ctx, base);
	// 83006F4C: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 83006F50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006F54: 4E800421  bctrl
	ctx.lr = 0x83006F58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006F58: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83006F5C: 40820064  bne 0x83006fc0
	if !ctx.cr[0].eq {
	pc = 0x83006FC0; continue 'dispatch;
	}
	// 83006F60: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006F64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006F68: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006F6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006F70: 4E800421  bctrl
	ctx.lr = 0x83006F74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006F74: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83006F78: 41820020  beq 0x83006f98
	if ctx.cr[0].eq {
	pc = 0x83006F98; continue 'dispatch;
	}
	// 83006F7C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006F80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006F84: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83006F88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006F8C: 4E800421  bctrl
	ctx.lr = 0x83006F90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006F90: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83006F94: 4800000C  b 0x83006fa0
	pc = 0x83006FA0; continue 'dispatch;
	// 83006F98: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83006F9C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83006FA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83006FA4: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 83006FA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006FAC: 4BFF2F25  bl 0x82ff9ed0
	ctx.lr = 0x83006FB0;
	sub_82FF9ED0(ctx, base);
	// 83006FB0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83006FB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006FB8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83006FBC: 481A9C6D  bl 0x831b0c28
	ctx.lr = 0x83006FC0;
	sub_831B0C28(ctx, base);
	// 83006FC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006FC4: 816B00C4  lwz r11, 0xc4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(196 as u32) ) } as u64;
	// 83006FC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83006FCC: 4E800421  bctrl
	ctx.lr = 0x83006FD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006FD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83006FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83006FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83006FDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83006FE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83006FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006FE8 size=300
    let mut pc: u32 = 0x83006FE8;
    'dispatch: loop {
        match pc {
            0x83006FE8 => {
    //   block [0x83006FE8..0x83007114)
	// 83006FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006FF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83006FF4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006FF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83006FFC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83007000: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83007004: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83007008: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8300700C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83007010: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83007014: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83007018: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300701C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007020: 4182005C  beq 0x8300707c
	if ctx.cr[0].eq {
	pc = 0x8300707C; continue 'dispatch;
	}
	// 83007024: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007028: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300702C: 4E800421  bctrl
	ctx.lr = 0x83007030;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007030: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007034: 41820020  beq 0x83007054
	if ctx.cr[0].eq {
	pc = 0x83007054; continue 'dispatch;
	}
	// 83007038: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300703C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007040: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007044: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007048: 4E800421  bctrl
	ctx.lr = 0x8300704C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300704C: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83007050: 4800000C  b 0x8300705c
	pc = 0x8300705C; continue 'dispatch;
	// 83007054: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83007058: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300705C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007060: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83007064: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007068: 4BFF2E69  bl 0x82ff9ed0
	ctx.lr = 0x8300706C;
	sub_82FF9ED0(ctx, base);
	// 8300706C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83007070: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007074: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83007078: 481A9BB1  bl 0x831b0c28
	ctx.lr = 0x8300707C;
	sub_831B0C28(ctx, base);
	// 8300707C: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 83007080: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007084: 4E800421  bctrl
	ctx.lr = 0x83007088;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007088: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300708C: 40820064  bne 0x830070f0
	if !ctx.cr[0].eq {
	pc = 0x830070F0; continue 'dispatch;
	}
	// 83007090: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007094: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007098: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300709C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830070A0: 4E800421  bctrl
	ctx.lr = 0x830070A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830070A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830070A8: 41820020  beq 0x830070c8
	if ctx.cr[0].eq {
	pc = 0x830070C8; continue 'dispatch;
	}
	// 830070AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830070B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830070B4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830070B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830070BC: 4E800421  bctrl
	ctx.lr = 0x830070C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830070C0: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 830070C4: 4800000C  b 0x830070d0
	pc = 0x830070D0; continue 'dispatch;
	// 830070C8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830070CC: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830070D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830070D4: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 830070D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830070DC: 4BFF2DF5  bl 0x82ff9ed0
	ctx.lr = 0x830070E0;
	sub_82FF9ED0(ctx, base);
	// 830070E0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830070E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830070E8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830070EC: 481A9B3D  bl 0x831b0c28
	ctx.lr = 0x830070F0;
	sub_831B0C28(ctx, base);
	// 830070F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830070F4: 816B00C4  lwz r11, 0xc4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(196 as u32) ) } as u64;
	// 830070F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830070FC: 4E800421  bctrl
	ctx.lr = 0x83007100;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83007104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83007108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300710C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83007110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83007118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007118 size=416
    let mut pc: u32 = 0x83007118;
    'dispatch: loop {
        match pc {
            0x83007118 => {
    //   block [0x83007118..0x830072B8)
	// 83007118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300711C: 481A104D  bl 0x831a8168
	ctx.lr = 0x83007120;
	sub_831A8130(ctx, base);
	// 83007120: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007124: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83007128: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300712C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83007130: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83007134: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83007138: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8300713C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83007140: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83007144: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83007148: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300714C: 41820060  beq 0x830071ac
	if ctx.cr[0].eq {
	pc = 0x830071AC; continue 'dispatch;
	}
	// 83007150: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007154: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007158: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300715C: 4E800421  bctrl
	ctx.lr = 0x83007160;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007160: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007164: 41820020  beq 0x83007184
	if ctx.cr[0].eq {
	pc = 0x83007184; continue 'dispatch;
	}
	// 83007168: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300716C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007170: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007174: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007178: 4E800421  bctrl
	ctx.lr = 0x8300717C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300717C: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83007180: 4800000C  b 0x8300718c
	pc = 0x8300718C; continue 'dispatch;
	// 83007184: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83007188: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300718C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007190: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83007194: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007198: 4BFF2D39  bl 0x82ff9ed0
	ctx.lr = 0x8300719C;
	sub_82FF9ED0(ctx, base);
	// 8300719C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830071A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830071A4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830071A8: 481A9A81  bl 0x831b0c28
	ctx.lr = 0x830071AC;
	sub_831B0C28(ctx, base);
	// 830071AC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830071B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830071B4: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 830071B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830071BC: 4E800421  bctrl
	ctx.lr = 0x830071C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830071C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830071C4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830071C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830071CC: 41820048  beq 0x83007214
	if ctx.cr[0].eq {
	pc = 0x83007214; continue 'dispatch;
	}
	// 830071D0: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830071D4: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 830071D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830071DC: 4E800421  bctrl
	ctx.lr = 0x830071E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830071E0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830071E4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830071E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830071EC: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 830071F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830071F4: 4E800421  bctrl
	ctx.lr = 0x830071F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830071F8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830071FC: 817D00C8  lwz r11, 0xc8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(200 as u32) ) } as u64;
	// 83007200: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007204: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83007208: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300720C: 4E800421  bctrl
	ctx.lr = 0x83007210;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007210: 48000028  b 0x83007238
	pc = 0x83007238; continue 'dispatch;
	// 83007214: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007218: 816B009C  lwz r11, 0x9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 8300721C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007220: 4E800421  bctrl
	ctx.lr = 0x83007224;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007224: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83007228: 817E00A4  lwz r11, 0xa4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300722C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007230: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007234: 4E800421  bctrl
	ctx.lr = 0x83007238;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007238: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8300723C: 409A0064  bne cr6, 0x830072a0
	if !ctx.cr[6].eq {
	pc = 0x830072A0; continue 'dispatch;
	}
	// 83007240: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007244: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007248: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300724C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007250: 4E800421  bctrl
	ctx.lr = 0x83007254;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007254: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007258: 41820020  beq 0x83007278
	if ctx.cr[0].eq {
	pc = 0x83007278; continue 'dispatch;
	}
	// 8300725C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007264: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007268: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300726C: 4E800421  bctrl
	ctx.lr = 0x83007270;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007270: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83007274: 4800000C  b 0x83007280
	pc = 0x83007280; continue 'dispatch;
	// 83007278: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300727C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83007280: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007284: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 83007288: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300728C: 4BFF2C45  bl 0x82ff9ed0
	ctx.lr = 0x83007290;
	sub_82FF9ED0(ctx, base);
	// 83007290: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83007294: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007298: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300729C: 481A998D  bl 0x831b0c28
	ctx.lr = 0x830072A0;
	sub_831B0C28(ctx, base);
	// 830072A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830072A4: 816B00C4  lwz r11, 0xc4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(196 as u32) ) } as u64;
	// 830072A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830072AC: 4E800421  bctrl
	ctx.lr = 0x830072B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830072B0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830072B4: 481A0F04  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830072B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830072B8 size=332
    let mut pc: u32 = 0x830072B8;
    'dispatch: loop {
        match pc {
            0x830072B8 => {
    //   block [0x830072B8..0x83007404)
	// 830072B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830072BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830072C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830072C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830072C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830072CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830072D0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830072D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830072D8: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830072DC: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 830072E0: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 830072E4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830072E8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830072EC: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830072F0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830072F4: 41820060  beq 0x83007354
	if ctx.cr[0].eq {
	pc = 0x83007354; continue 'dispatch;
	}
	// 830072F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830072FC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007300: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007304: 4E800421  bctrl
	ctx.lr = 0x83007308;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007308: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300730C: 41820020  beq 0x8300732c
	if ctx.cr[0].eq {
	pc = 0x8300732C; continue 'dispatch;
	}
	// 83007310: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007318: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300731C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007320: 4E800421  bctrl
	ctx.lr = 0x83007324;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007324: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83007328: 4800000C  b 0x83007334
	pc = 0x83007334; continue 'dispatch;
	// 8300732C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83007330: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83007334: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007338: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8300733C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007340: 4BFF2B91  bl 0x82ff9ed0
	ctx.lr = 0x83007344;
	sub_82FF9ED0(ctx, base);
	// 83007344: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83007348: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300734C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83007350: 481A98D9  bl 0x831b0c28
	ctx.lr = 0x83007354;
	sub_831B0C28(ctx, base);
	// 83007354: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007358: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300735C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83007360: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007364: 4E800421  bctrl
	ctx.lr = 0x83007368;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007368: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 8300736C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83007370: 419A0064  beq cr6, 0x830073d4
	if ctx.cr[6].eq {
	pc = 0x830073D4; continue 'dispatch;
	}
	// 83007374: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300737C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007380: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007384: 4E800421  bctrl
	ctx.lr = 0x83007388;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007388: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300738C: 41820020  beq 0x830073ac
	if ctx.cr[0].eq {
	pc = 0x830073AC; continue 'dispatch;
	}
	// 83007390: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007394: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007398: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300739C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830073A0: 4E800421  bctrl
	ctx.lr = 0x830073A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830073A4: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 830073A8: 4800000C  b 0x830073b4
	pc = 0x830073B4; continue 'dispatch;
	// 830073AC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830073B0: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830073B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830073B8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 830073BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830073C0: 4BFF2B11  bl 0x82ff9ed0
	ctx.lr = 0x830073C4;
	sub_82FF9ED0(ctx, base);
	// 830073C4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830073C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830073CC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830073D0: 481A9859  bl 0x831b0c28
	ctx.lr = 0x830073D4;
	sub_831B0C28(ctx, base);
	// 830073D4: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830073D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830073DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830073E0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830073E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830073E8: 4E800421  bctrl
	ctx.lr = 0x830073EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830073EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830073F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830073F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830073F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830073FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83007400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83007408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007408 size=88
    let mut pc: u32 = 0x83007408;
    'dispatch: loop {
        match pc {
            0x83007408 => {
    //   block [0x83007408..0x83007460)
	// 83007408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300740C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83007410: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83007414: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83007418: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300741C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83007420: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83007424: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83007428: 4BFD7F69  bl 0x82fdf390
	ctx.lr = 0x8300742C;
	sub_82FDF390(ctx, base);
	// 8300742C: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83007430: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83007434: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83007438: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300743C: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83007440: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007444: 4E800421  bctrl
	ctx.lr = 0x83007448;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007448: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300744C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83007450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83007454: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83007458: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300745C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83007460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007460 size=84
    let mut pc: u32 = 0x83007460;
    'dispatch: loop {
        match pc {
            0x83007460 => {
    //   block [0x83007460..0x830074B4)
	// 83007460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83007468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300746C: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 83007470: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007474: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83007478: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300747C: 4E800421  bctrl
	ctx.lr = 0x83007480;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007480: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007484: 40820010  bne 0x83007494
	if !ctx.cr[0].eq {
	pc = 0x83007494; continue 'dispatch;
	}
	// 83007488: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300748C: 386B8158  addi r3, r11, -0x7ea8
	ctx.r[3].s64 = ctx.r[11].s64 + -32424;
	// 83007490: 48000014  b 0x830074a4
	pc = 0x830074A4; continue 'dispatch;
	// 83007494: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007498: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300749C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830074A0: 4E800421  bctrl
	ctx.lr = 0x830074A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830074A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830074A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830074AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830074B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830074B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830074B8 size=280
    let mut pc: u32 = 0x830074B8;
    'dispatch: loop {
        match pc {
            0x830074B8 => {
    //   block [0x830074B8..0x830075D0)
	// 830074B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830074BC: 481A0CA9  bl 0x831a8164
	ctx.lr = 0x830074C0;
	sub_831A8130(ctx, base);
	// 830074C0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830074C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830074C8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830074CC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830074D0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830074D4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 830074D8: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830074DC: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 830074E0: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 830074E4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830074E8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830074EC: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830074F0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830074F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830074F8: 4182005C  beq 0x83007554
	if ctx.cr[0].eq {
	pc = 0x83007554; continue 'dispatch;
	}
	// 830074FC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007500: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007504: 4E800421  bctrl
	ctx.lr = 0x83007508;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007508: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300750C: 41820020  beq 0x8300752c
	if ctx.cr[0].eq {
	pc = 0x8300752C; continue 'dispatch;
	}
	// 83007510: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007514: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007518: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300751C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007520: 4E800421  bctrl
	ctx.lr = 0x83007524;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007524: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83007528: 4800000C  b 0x83007534
	pc = 0x83007534; continue 'dispatch;
	// 8300752C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83007530: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83007534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007538: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8300753C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007540: 4BFF2991  bl 0x82ff9ed0
	ctx.lr = 0x83007544;
	sub_82FF9ED0(ctx, base);
	// 83007544: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83007548: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300754C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83007550: 481A96D9  bl 0x831b0c28
	ctx.lr = 0x83007554;
	sub_831B0C28(ctx, base);
	// 83007554: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 83007558: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8300755C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83007560: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007564: 4E800421  bctrl
	ctx.lr = 0x83007568;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007568: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8300756C: 40820044  bne 0x830075b0
	if !ctx.cr[0].eq {
	pc = 0x830075B0; continue 'dispatch;
	}
	// 83007570: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83007574: 4BFD7CB5  bl 0x82fdf228
	ctx.lr = 0x83007578;
	sub_82FDF228(ctx, base);
	// 83007578: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300757C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83007580: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83007584: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 83007588: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300758C: 4E800421  bctrl
	ctx.lr = 0x83007590;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007590: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83007594: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83007598: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8300759C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830075A0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830075A4: 816A001C  lwz r11, 0x1c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 830075A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830075AC: 4E800421  bctrl
	ctx.lr = 0x830075B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830075B0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830075B4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830075B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830075BC: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 830075C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830075C4: 4E800421  bctrl
	ctx.lr = 0x830075C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830075C8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 830075CC: 481A0BE8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830075D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830075D0 size=232
    let mut pc: u32 = 0x830075D0;
    'dispatch: loop {
        match pc {
            0x830075D0 => {
    //   block [0x830075D0..0x830076B8)
	// 830075D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830075D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830075D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830075DC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830075E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830075E4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830075E8: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830075EC: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 830075F0: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 830075F4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830075F8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830075FC: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83007600: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007604: 41820060  beq 0x83007664
	if ctx.cr[0].eq {
	pc = 0x83007664; continue 'dispatch;
	}
	// 83007608: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300760C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007610: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007614: 4E800421  bctrl
	ctx.lr = 0x83007618;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007618: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300761C: 41820020  beq 0x8300763c
	if ctx.cr[0].eq {
	pc = 0x8300763C; continue 'dispatch;
	}
	// 83007620: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007624: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007628: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300762C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007630: 4E800421  bctrl
	ctx.lr = 0x83007634;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007634: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83007638: 4800000C  b 0x83007644
	pc = 0x83007644; continue 'dispatch;
	// 8300763C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83007640: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83007644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007648: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8300764C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007650: 4BFF2881  bl 0x82ff9ed0
	ctx.lr = 0x83007654;
	sub_82FF9ED0(ctx, base);
	// 83007654: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83007658: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300765C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83007660: 481A95C9  bl 0x831b0c28
	ctx.lr = 0x83007664;
	sub_831B0C28(ctx, base);
	// 83007664: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83007668: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300766C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83007670: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007674: 4E800421  bctrl
	ctx.lr = 0x83007678;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007678: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8300767C: 41800028  blt 0x830076a4
	if ctx.cr[0].lt {
	pc = 0x830076A4; continue 'dispatch;
	}
	// 83007680: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83007684: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007688: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8300768C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007690: 4E800421  bctrl
	ctx.lr = 0x83007694;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007694: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007698: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 8300769C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830076A0: 4E800421  bctrl
	ctx.lr = 0x830076A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830076A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830076A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830076AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830076B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830076B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830076B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830076B8 size=20
    let mut pc: u32 = 0x830076B8;
    'dispatch: loop {
        match pc {
            0x830076B8 => {
    //   block [0x830076B8..0x830076CC)
	// 830076B8: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 830076BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830076C0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830076C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830076C8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830076D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830076D0 size=328
    let mut pc: u32 = 0x830076D0;
    'dispatch: loop {
        match pc {
            0x830076D0 => {
    //   block [0x830076D0..0x83007818)
	// 830076D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830076D4: 481A0A99  bl 0x831a816c
	ctx.lr = 0x830076D8;
	sub_831A8130(ctx, base);
	// 830076D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830076DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830076E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830076E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830076E8: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830076EC: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 830076F0: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 830076F4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830076F8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830076FC: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83007700: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007704: 41820060  beq 0x83007764
	if ctx.cr[0].eq {
	pc = 0x83007764; continue 'dispatch;
	}
	// 83007708: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300770C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007710: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007714: 4E800421  bctrl
	ctx.lr = 0x83007718;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007718: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300771C: 41820020  beq 0x8300773c
	if ctx.cr[0].eq {
	pc = 0x8300773C; continue 'dispatch;
	}
	// 83007720: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007728: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300772C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007730: 4E800421  bctrl
	ctx.lr = 0x83007734;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007734: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83007738: 4800000C  b 0x83007744
	pc = 0x83007744; continue 'dispatch;
	// 8300773C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83007740: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83007744: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007748: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8300774C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007750: 4BFF2781  bl 0x82ff9ed0
	ctx.lr = 0x83007754;
	sub_82FF9ED0(ctx, base);
	// 83007754: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83007758: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300775C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83007760: 481A94C9  bl 0x831b0c28
	ctx.lr = 0x83007764;
	sub_831B0C28(ctx, base);
	// 83007764: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007768: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300776C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007770: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007774: 4E800421  bctrl
	ctx.lr = 0x83007778;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007778: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300777C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83007780: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007784: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007788: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300778C: 4E800421  bctrl
	ctx.lr = 0x83007790;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007790: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83007794: 419A0064  beq cr6, 0x830077f8
	if ctx.cr[6].eq {
	pc = 0x830077F8; continue 'dispatch;
	}
	// 83007798: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300779C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830077A0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830077A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830077A8: 4E800421  bctrl
	ctx.lr = 0x830077AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830077AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830077B0: 41820020  beq 0x830077d0
	if ctx.cr[0].eq {
	pc = 0x830077D0; continue 'dispatch;
	}
	// 830077B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830077B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830077BC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830077C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830077C4: 4E800421  bctrl
	ctx.lr = 0x830077C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830077C8: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 830077CC: 4800000C  b 0x830077d8
	pc = 0x830077D8; continue 'dispatch;
	// 830077D0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830077D4: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830077D8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830077DC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 830077E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830077E4: 4BFF26ED  bl 0x82ff9ed0
	ctx.lr = 0x830077E8;
	sub_82FF9ED0(ctx, base);
	// 830077E8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830077EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830077F0: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830077F4: 481A9435  bl 0x831b0c28
	ctx.lr = 0x830077F8;
	sub_831B0C28(ctx, base);
	// 830077F8: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830077FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83007800: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007804: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83007808: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300780C: 4E800421  bctrl
	ctx.lr = 0x83007810;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007810: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83007814: 481A09A8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83007818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007818 size=64
    let mut pc: u32 = 0x83007818;
    'dispatch: loop {
        match pc {
            0x83007818 => {
    //   block [0x83007818..0x83007858)
	// 83007818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300781C: 481A0951  bl 0x831a816c
	ctx.lr = 0x83007820;
	sub_831A8130(ctx, base);
	// 83007820: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007824: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83007828: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300782C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83007830: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007834: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007838: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300783C: 4E800421  bctrl
	ctx.lr = 0x83007840;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007840: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83007844: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83007848: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300784C: 4BFDCE75  bl 0x82fe46c0
	ctx.lr = 0x83007850;
	sub_82FE46C0(ctx, base);
	// 83007850: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83007854: 481A0968  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83007858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007858 size=80
    let mut pc: u32 = 0x83007858;
    'dispatch: loop {
        match pc {
            0x83007858 => {
    //   block [0x83007858..0x830078A8)
	// 83007858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300785C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83007860: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007864: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 83007868: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300786C: 419A0024  beq cr6, 0x83007890
	if ctx.cr[6].eq {
	pc = 0x83007890; continue 'dispatch;
	}
	// 83007870: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83007874: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007878: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300787C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007880: 4E800421  bctrl
	ctx.lr = 0x83007884;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007884: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007888: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8300788C: 40820008  bne 0x83007894
	if !ctx.cr[0].eq {
	pc = 0x83007894; continue 'dispatch;
	}
	// 83007890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83007894: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83007898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300789C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830078A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830078A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830078A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830078A8 size=60
    let mut pc: u32 = 0x830078A8;
    'dispatch: loop {
        match pc {
            0x830078A8 => {
    //   block [0x830078A8..0x830078E4)
	// 830078A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830078AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830078B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830078B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830078B8: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 830078BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830078C0: 4E800421  bctrl
	ctx.lr = 0x830078C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830078C4: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 830078C8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830078CC: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830078D0: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 830078D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830078D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830078DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830078E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830078E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830078E8 size=60
    let mut pc: u32 = 0x830078E8;
    'dispatch: loop {
        match pc {
            0x830078E8 => {
    //   block [0x830078E8..0x83007924)
	// 830078E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830078EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830078F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830078F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830078F8: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 830078FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007900: 4E800421  bctrl
	ctx.lr = 0x83007904;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007904: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 83007908: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8300790C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83007910: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 83007914: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83007918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300791C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83007920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83007928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83007928 size=8
    let mut pc: u32 = 0x83007928;
    'dispatch: loop {
        match pc {
            0x83007928 => {
    //   block [0x83007928..0x83007930)
	// 83007928: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300792C: 821411A8  lwz r16, 0x11a8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(4520 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83007930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007930 size=276
    let mut pc: u32 = 0x83007930;
    'dispatch: loop {
        match pc {
            0x83007930 => {
    //   block [0x83007930..0x83007A44)
	// 83007930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007934: 481A0835  bl 0x831a8168
	ctx.lr = 0x83007938;
	sub_831A8130(ctx, base);
	// 83007938: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8300793C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007940: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83007944: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007948: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300794C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007950: 4E800421  bctrl
	ctx.lr = 0x83007954;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007954: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83007958: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300795C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83007960: 419A00DC  beq cr6, 0x83007a3c
	if ctx.cr[6].eq {
	pc = 0x83007A3C; continue 'dispatch;
	}
	// 83007964: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83007968: 419A00D4  beq cr6, 0x83007a3c
	if ctx.cr[6].eq {
	pc = 0x83007A3C; continue 'dispatch;
	}
	// 8300796C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007970: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83007974: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007978: 4E800421  bctrl
	ctx.lr = 0x8300797C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300797C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007980: 418200BC  beq 0x83007a3c
	if ctx.cr[0].eq {
	pc = 0x83007A3C; continue 'dispatch;
	}
	// 83007984: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007988: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300798C: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83007990: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007994: 4E800421  bctrl
	ctx.lr = 0x83007998;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007998: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300799C: 816B00C4  lwz r11, 0xc4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(196 as u32) ) } as u64;
	// 830079A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830079A4: 4E800421  bctrl
	ctx.lr = 0x830079A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830079A8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830079AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830079B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830079B4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830079B8: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830079BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830079C0: 4E800421  bctrl
	ctx.lr = 0x830079C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830079C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830079C8: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 830079CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830079D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830079D4: 4E800421  bctrl
	ctx.lr = 0x830079D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830079D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830079DC: 41820060  beq 0x83007a3c
	if ctx.cr[0].eq {
	pc = 0x83007A3C; continue 'dispatch;
	}
	// 830079E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830079E4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 830079E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830079EC: 4E800421  bctrl
	ctx.lr = 0x830079F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830079F0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830079F4: 41820048  beq 0x83007a3c
	if ctx.cr[0].eq {
	pc = 0x83007A3C; continue 'dispatch;
	}
	// 830079F8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830079FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007A00: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007A04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007A08: 4E800421  bctrl
	ctx.lr = 0x83007A0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007A0C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 83007A10: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83007A14: 4BFD9FDD  bl 0x82fe19f0
	ctx.lr = 0x83007A18;
	sub_82FE19F0(ctx, base);
	// 83007A18: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83007A1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007A20: 41820014  beq 0x83007a34
	if ctx.cr[0].eq {
	pc = 0x83007A34; continue 'dispatch;
	}
	// 83007A24: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83007A28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83007A2C: 48033485  bl 0x8303aeb0
	ctx.lr = 0x83007A30;
	sub_8303AEB0(ctx, base);
	// 83007A30: 48000008  b 0x83007a38
	pc = 0x83007A38; continue 'dispatch;
	// 83007A34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83007A38: 907E0028  stw r3, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 83007A3C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83007A40: 481A0778  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83007A44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007A44 size=44
    let mut pc: u32 = 0x83007A44;
    'dispatch: loop {
        match pc {
            0x83007A44 => {
    //   block [0x83007A44..0x83007A70)
	// 83007A44: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83007A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83007A50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007A54: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83007A58: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83007A5C: 480C0085  bl 0x830c7ae0
	ctx.lr = 0x83007A60;
	sub_830C7AE0(ctx, base);
	// 83007A60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83007A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83007A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83007A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83007A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007A70 size=368
    let mut pc: u32 = 0x83007A70;
    'dispatch: loop {
        match pc {
            0x83007A70 => {
    //   block [0x83007A70..0x83007BE0)
	// 83007A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83007A78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83007A7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83007A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007A84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83007A88: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83007A8C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83007A90: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83007A94: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83007A98: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83007A9C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83007AA0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83007AA4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83007AA8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007AAC: 41820060  beq 0x83007b0c
	if ctx.cr[0].eq {
	pc = 0x83007B0C; continue 'dispatch;
	}
	// 83007AB0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007AB4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007AB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007ABC: 4E800421  bctrl
	ctx.lr = 0x83007AC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007AC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007AC4: 41820020  beq 0x83007ae4
	if ctx.cr[0].eq {
	pc = 0x83007AE4; continue 'dispatch;
	}
	// 83007AC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007ACC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007AD0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007AD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007AD8: 4E800421  bctrl
	ctx.lr = 0x83007ADC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007ADC: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83007AE0: 4800000C  b 0x83007aec
	pc = 0x83007AEC; continue 'dispatch;
	// 83007AE4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83007AE8: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83007AEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007AF0: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83007AF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007AF8: 4BFF23D9  bl 0x82ff9ed0
	ctx.lr = 0x83007AFC;
	sub_82FF9ED0(ctx, base);
	// 83007AFC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83007B00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007B04: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83007B08: 481A9121  bl 0x831b0c28
	ctx.lr = 0x83007B0C;
	sub_831B0C28(ctx, base);
	// 83007B0C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007B10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007B14: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83007B18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007B1C: 4E800421  bctrl
	ctx.lr = 0x83007B20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007B20: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83007B24: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83007B28: 419A0064  beq cr6, 0x83007b8c
	if ctx.cr[6].eq {
	pc = 0x83007B8C; continue 'dispatch;
	}
	// 83007B2C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007B30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007B34: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007B38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007B3C: 4E800421  bctrl
	ctx.lr = 0x83007B40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007B40: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007B44: 41820020  beq 0x83007b64
	if ctx.cr[0].eq {
	pc = 0x83007B64; continue 'dispatch;
	}
	// 83007B48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007B4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007B50: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007B54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007B58: 4E800421  bctrl
	ctx.lr = 0x83007B5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007B5C: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83007B60: 4800000C  b 0x83007b6c
	pc = 0x83007B6C; continue 'dispatch;
	// 83007B64: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83007B68: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83007B6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007B70: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83007B74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007B78: 4BFF2359  bl 0x82ff9ed0
	ctx.lr = 0x83007B7C;
	sub_82FF9ED0(ctx, base);
	// 83007B7C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83007B80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007B84: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83007B88: 481A90A1  bl 0x831b0c28
	ctx.lr = 0x83007B8C;
	sub_831B0C28(ctx, base);
	// 83007B8C: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83007B90: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83007B94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007B98: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83007B9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007BA0: 4E800421  bctrl
	ctx.lr = 0x83007BA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007BA4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83007BA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83007BAC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83007BB0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83007BB4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007BB8: 816A002C  lwz r11, 0x2c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007BBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007BC0: 4E800421  bctrl
	ctx.lr = 0x83007BC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007BC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007BC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83007BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83007BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83007BD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83007BD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83007BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83007BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007BE0 size=364
    let mut pc: u32 = 0x83007BE0;
    'dispatch: loop {
        match pc {
            0x83007BE0 => {
    //   block [0x83007BE0..0x83007D4C)
	// 83007BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007BE4: 481A0589  bl 0x831a816c
	ctx.lr = 0x83007BE8;
	sub_831A8130(ctx, base);
	// 83007BE8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007BEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83007BF0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83007BF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83007BF8: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83007BFC: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83007C00: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83007C04: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83007C08: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83007C0C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83007C10: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007C14: 41820060  beq 0x83007c74
	if ctx.cr[0].eq {
	pc = 0x83007C74; continue 'dispatch;
	}
	// 83007C18: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007C1C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007C20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007C24: 4E800421  bctrl
	ctx.lr = 0x83007C28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007C28: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007C2C: 41820020  beq 0x83007c4c
	if ctx.cr[0].eq {
	pc = 0x83007C4C; continue 'dispatch;
	}
	// 83007C30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007C34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007C38: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007C3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007C40: 4E800421  bctrl
	ctx.lr = 0x83007C44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007C44: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83007C48: 4800000C  b 0x83007c54
	pc = 0x83007C54; continue 'dispatch;
	// 83007C4C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83007C50: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83007C54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007C58: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83007C5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007C60: 4BFF2271  bl 0x82ff9ed0
	ctx.lr = 0x83007C64;
	sub_82FF9ED0(ctx, base);
	// 83007C64: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83007C68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007C6C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83007C70: 481A8FB9  bl 0x831b0c28
	ctx.lr = 0x83007C74;
	sub_831B0C28(ctx, base);
	// 83007C74: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007C78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007C7C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007C80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007C84: 4E800421  bctrl
	ctx.lr = 0x83007C88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007C88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007C8C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83007C90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007C94: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007C98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007C9C: 4E800421  bctrl
	ctx.lr = 0x83007CA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007CA0: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83007CA4: 419A0064  beq cr6, 0x83007d08
	if ctx.cr[6].eq {
	pc = 0x83007D08; continue 'dispatch;
	}
	// 83007CA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007CAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007CB0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007CB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007CB8: 4E800421  bctrl
	ctx.lr = 0x83007CBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007CBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007CC0: 41820020  beq 0x83007ce0
	if ctx.cr[0].eq {
	pc = 0x83007CE0; continue 'dispatch;
	}
	// 83007CC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007CC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007CCC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007CD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007CD4: 4E800421  bctrl
	ctx.lr = 0x83007CD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007CD8: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83007CDC: 4800000C  b 0x83007ce8
	pc = 0x83007CE8; continue 'dispatch;
	// 83007CE0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83007CE4: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83007CE8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007CEC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83007CF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007CF4: 4BFF21DD  bl 0x82ff9ed0
	ctx.lr = 0x83007CF8;
	sub_82FF9ED0(ctx, base);
	// 83007CF8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83007CFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007D00: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83007D04: 481A8F25  bl 0x831b0c28
	ctx.lr = 0x83007D08;
	sub_831B0C28(ctx, base);
	// 83007D08: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83007D0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83007D10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007D14: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83007D18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007D1C: 4E800421  bctrl
	ctx.lr = 0x83007D20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007D20: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83007D24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83007D28: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83007D2C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83007D30: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007D34: 816A002C  lwz r11, 0x2c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007D38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007D3C: 4E800421  bctrl
	ctx.lr = 0x83007D40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007D40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007D44: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83007D48: 481A0474  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83007D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007D50 size=376
    let mut pc: u32 = 0x83007D50;
    'dispatch: loop {
        match pc {
            0x83007D50 => {
    //   block [0x83007D50..0x83007EC8)
	// 83007D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83007D58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83007D5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83007D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007D64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83007D68: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 83007D6C: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83007D70: A14AA6A4  lhz r10, -0x595c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 83007D74: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83007D78: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 83007D7C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 83007D80: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 83007D84: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007D88: 41820080  beq 0x83007e08
	if ctx.cr[0].eq {
	pc = 0x83007E08; continue 'dispatch;
	}
	// 83007D8C: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 83007D90: A14AA6C8  lhz r10, -0x5938(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22840 as u32) ) } as u64;
	// 83007D94: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83007D98: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83007D9C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83007DA0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83007DA4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007DA8: 40820060  bne 0x83007e08
	if !ctx.cr[0].eq {
	pc = 0x83007E08; continue 'dispatch;
	}
	// 83007DAC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007DB0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007DB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007DB8: 4E800421  bctrl
	ctx.lr = 0x83007DBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007DBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007DC0: 41820020  beq 0x83007de0
	if ctx.cr[0].eq {
	pc = 0x83007DE0; continue 'dispatch;
	}
	// 83007DC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007DC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007DCC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007DD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007DD4: 4E800421  bctrl
	ctx.lr = 0x83007DD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007DD8: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83007DDC: 4800000C  b 0x83007de8
	pc = 0x83007DE8; continue 'dispatch;
	// 83007DE0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83007DE4: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83007DE8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007DEC: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83007DF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007DF4: 4BFF20DD  bl 0x82ff9ed0
	ctx.lr = 0x83007DF8;
	sub_82FF9ED0(ctx, base);
	// 83007DF8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83007DFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007E00: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83007E04: 481A8E25  bl 0x831b0c28
	ctx.lr = 0x83007E08;
	sub_831B0C28(ctx, base);
	// 83007E08: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007E0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007E10: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007E14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007E18: 4E800421  bctrl
	ctx.lr = 0x83007E1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007E1C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83007E20: 41820048  beq 0x83007e68
	if ctx.cr[0].eq {
	pc = 0x83007E68; continue 'dispatch;
	}
	// 83007E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83007E28: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007E2C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83007E30: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83007E34: 4BFD7945  bl 0x82fdf778
	ctx.lr = 0x83007E38;
	sub_82FDF778(ctx, base);
	// 83007E38: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83007E3C: 4BFF484D  bl 0x82ffc688
	ctx.lr = 0x83007E40;
	sub_82FFC688(ctx, base);
	// 83007E40: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 83007E44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83007E48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007E4C: 4BFDD895  bl 0x82fe56e0
	ctx.lr = 0x83007E50;
	sub_82FE56E0(ctx, base);
	// 83007E50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83007E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83007E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83007E5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83007E60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83007E64: 4E800020  blr
	return;
	// 83007E68: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007E6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007E70: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007E74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007E78: 4E800421  bctrl
	ctx.lr = 0x83007E7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007E7C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007E80: 41820020  beq 0x83007ea0
	if ctx.cr[0].eq {
	pc = 0x83007EA0; continue 'dispatch;
	}
	// 83007E84: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007E88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007E8C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007E90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007E94: 4E800421  bctrl
	ctx.lr = 0x83007E98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007E98: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83007E9C: 4800000C  b 0x83007ea8
	pc = 0x83007EA8; continue 'dispatch;
	// 83007EA0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83007EA4: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83007EA8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007EAC: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83007EB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007EB4: 4BFF201D  bl 0x82ff9ed0
	ctx.lr = 0x83007EB8;
	sub_82FF9ED0(ctx, base);
	// 83007EB8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83007EBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007EC0: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83007EC4: 481A8D65  bl 0x831b0c28
	ctx.lr = 0x83007EC8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83007EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83007EC8 size=8
    let mut pc: u32 = 0x83007EC8;
    'dispatch: loop {
        match pc {
            0x83007EC8 => {
    //   block [0x83007EC8..0x83007ED0)
	// 83007EC8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83007ECC: 8214122C  lwz r16, 0x122c(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(4652 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83007ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007ED0 size=452
    let mut pc: u32 = 0x83007ED0;
    'dispatch: loop {
        match pc {
            0x83007ED0 => {
    //   block [0x83007ED0..0x83008094)
	// 83007ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007ED4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83007ED8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83007EDC: 481A028D  bl 0x831a8168
	ctx.lr = 0x83007EE0;
	sub_831A8130(ctx, base);
	// 83007EE0: 3BE1FF00  addi r31, r1, -0x100
	ctx.r[31].s64 = ctx.r[1].s64 + -256;
	// 83007EE4: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007EE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83007EEC: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83007EF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007EF4: 816B0078  lwz r11, 0x78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 83007EF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007EFC: 4E800421  bctrl
	ctx.lr = 0x83007F00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007F00: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83007F04: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83007F08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83007F0C: 419A017C  beq cr6, 0x83008088
	if ctx.cr[6].eq {
	pc = 0x83008088; continue 'dispatch;
	}
	// 83007F10: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83007F14: 39600078  li r11, 0x78
	ctx.r[11].s64 = 120;
	// 83007F18: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 83007F1C: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 83007F20: 3960006D  li r11, 0x6d
	ctx.r[11].s64 = 109;
	// 83007F24: B17F0052  sth r11, 0x52(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(82 as u32), ctx.r[11].u16 ) };
	// 83007F28: 3960006C  li r11, 0x6c
	ctx.r[11].s64 = 108;
	// 83007F2C: B17F0054  sth r11, 0x54(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u16 ) };
	// 83007F30: 3960003A  li r11, 0x3a
	ctx.r[11].s64 = 58;
	// 83007F34: B17F0056  sth r11, 0x56(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(86 as u32), ctx.r[11].u16 ) };
	// 83007F38: 39600062  li r11, 0x62
	ctx.r[11].s64 = 98;
	// 83007F3C: B17F0058  sth r11, 0x58(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u16 ) };
	// 83007F40: 39600061  li r11, 0x61
	ctx.r[11].s64 = 97;
	// 83007F44: B17F005A  sth r11, 0x5a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(90 as u32), ctx.r[11].u16 ) };
	// 83007F48: 39600073  li r11, 0x73
	ctx.r[11].s64 = 115;
	// 83007F4C: B17F005C  sth r11, 0x5c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u16 ) };
	// 83007F50: 39600065  li r11, 0x65
	ctx.r[11].s64 = 101;
	// 83007F54: B17F005E  sth r11, 0x5e(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(94 as u32), ctx.r[11].u16 ) };
	// 83007F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83007F5C: B17F0060  sth r11, 0x60(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u16 ) };
	// 83007F60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007F64: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83007F68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007F6C: 4E800421  bctrl
	ctx.lr = 0x83007F70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007F70: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007F74: 41820114  beq 0x83008088
	if ctx.cr[0].eq {
	pc = 0x83008088; continue 'dispatch;
	}
	// 83007F78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007F7C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83007F80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007F84: 4E800421  bctrl
	ctx.lr = 0x83007F88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007F88: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83007F8C: 418200FC  beq 0x83008088
	if ctx.cr[0].eq {
	pc = 0x83008088; continue 'dispatch;
	}
	// 83007F90: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007F94: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007F98: 418200F0  beq 0x83008088
	if ctx.cr[0].eq {
	pc = 0x83008088; continue 'dispatch;
	}
	// 83007F9C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007FA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007FA4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007FA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007FAC: 4E800421  bctrl
	ctx.lr = 0x83007FB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007FB0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83007FB4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83007FB8: 80A30090  lwz r5, 0x90(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83007FBC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83007FC0: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 83007FC4: 4BFD5AAD  bl 0x82fdda70
	ctx.lr = 0x83007FC8;
	sub_82FDDA70(ctx, base);
	// 83007FC8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83007FCC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83007FD0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007FD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007FD8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007FDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83007FE0: 4E800421  bctrl
	ctx.lr = 0x83007FE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007FE4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83007FE8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83007FEC: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83007FF0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83007FF4: 389F00A0  addi r4, r31, 0xa0
	ctx.r[4].s64 = ctx.r[31].s64 + 160;
	// 83007FF8: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83007FFC: 4BFD5B85  bl 0x82fddb80
	ctx.lr = 0x83008000;
	sub_82FDDB80(ctx, base);
	// 83008000: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83008004: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83008008: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300800C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83008010: 409A0014  bne cr6, 0x83008024
	if !ctx.cr[6].eq {
	pc = 0x83008024; continue 'dispatch;
	}
	// 83008014: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83008018: 4BFD2DD9  bl 0x82fdadf0
	ctx.lr = 0x8300801C;
	sub_82FDADF0(ctx, base);
	// 8300801C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83008020: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83008024: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008028: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300802C: 83BF0094  lwz r29, 0x94(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83008030: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83008034: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83008038: 4E800421  bctrl
	ctx.lr = 0x8300803C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300803C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83008040: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83008044: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83008048: 4BFDABB9  bl 0x82fe2c00
	ctx.lr = 0x8300804C;
	sub_82FE2C00(ctx, base);
	// 8300804C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83008050: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83008054: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83008058: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8300805C: 4BFD37DD  bl 0x82fdb838
	ctx.lr = 0x83008060;
	sub_82FDB838(ctx, base);
	// 83008060: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83008064: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83008068: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 8300806C: 4BFD37CD  bl 0x82fdb838
	ctx.lr = 0x83008070;
	sub_82FDB838(ctx, base);
	// 83008070: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83008074: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83008078: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300807C: 48000010  b 0x8300808c
	pc = 0x8300808C; continue 'dispatch;
	// 83008080: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83008084: 48000008  b 0x8300808c
	pc = 0x8300808C; continue 'dispatch;
	// 83008088: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300808C: 383F0100  addi r1, r31, 0x100
	ctx.r[1].s64 = ctx.r[31].s64 + 256;
	// 83008090: 481A0128  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008094(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83008094 size=8
    let mut pc: u32 = 0x83008094;
    'dispatch: loop {
        match pc {
            0x83008094 => {
    //   block [0x83008094..0x8300809C)
	// 83008094: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83008098: 8214122C  lwz r16, 0x122c(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(4652 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300809C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300809C size=24
    let mut pc: u32 = 0x8300809C;
    'dispatch: loop {
        match pc {
            0x8300809C => {
    //   block [0x8300809C..0x830080B4)
	// 8300809C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830080A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830080A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830080A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830080AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830080B0: 481A8B79  bl 0x831b0c28
	ctx.lr = 0x830080B4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830080BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830080BC size=20
    let mut pc: u32 = 0x830080BC;
    'dispatch: loop {
        match pc {
            0x830080BC => {
    //   block [0x830080BC..0x830080D0)
	// 830080BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830080C0: 3C608301  lis r3, -0x7cff
	ctx.r[3].s64 = -2097086464;
	// 830080C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830080C8: 38638080  addi r3, r3, -0x7f80
	ctx.r[3].s64 = ctx.r[3].s64 + -32640;
	// 830080CC: 481A00EC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830080D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830080D0 size=40
    let mut pc: u32 = 0x830080D0;
    'dispatch: loop {
        match pc {
            0x830080D0 => {
    //   block [0x830080D0..0x830080F8)
	// 830080D0: 3BECFF00  addi r31, r12, -0x100
	ctx.r[31].s64 = ctx.r[12].s64 + -256;
	// 830080D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830080D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830080DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830080E0: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 830080E4: 4BFD3755  bl 0x82fdb838
	ctx.lr = 0x830080E8;
	sub_82FDB838(ctx, base);
	// 830080E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830080EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830080F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830080F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830080F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830080F8 size=40
    let mut pc: u32 = 0x830080F8;
    'dispatch: loop {
        match pc {
            0x830080F8 => {
    //   block [0x830080F8..0x83008120)
	// 830080F8: 3BECFF00  addi r31, r12, -0x100
	ctx.r[31].s64 = ctx.r[12].s64 + -256;
	// 830080FC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008100: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008104: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008108: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8300810C: 4BFD372D  bl 0x82fdb838
	ctx.lr = 0x83008110;
	sub_82FDB838(ctx, base);
	// 83008110: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83008114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300811C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83008120 size=8
    let mut pc: u32 = 0x83008120;
    'dispatch: loop {
        match pc {
            0x83008120 => {
    //   block [0x83008120..0x83008128)
	// 83008120: 3863001C  addi r3, r3, 0x1c
	ctx.r[3].s64 = ctx.r[3].s64 + 28;
	// 83008124: 4804A624  b 0x83052748
	sub_83052748(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83008128 size=8
    let mut pc: u32 = 0x83008128;
    'dispatch: loop {
        match pc {
            0x83008128 => {
    //   block [0x83008128..0x83008130)
	// 83008128: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300812C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83008130 size=8
    let mut pc: u32 = 0x83008130;
    'dispatch: loop {
        match pc {
            0x83008130 => {
    //   block [0x83008130..0x83008138)
	// 83008130: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 83008134: 4BFF4D14  b 0x82ffce48
	sub_82FFCE48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83008138 size=8
    let mut pc: u32 = 0x83008138;
    'dispatch: loop {
        match pc {
            0x83008138 => {
    //   block [0x83008138..0x83008140)
	// 83008138: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 8300813C: 4BFD8B7C  b 0x82fe0cb8
	sub_82FE0CB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008140 size=488
    let mut pc: u32 = 0x83008140;
    'dispatch: loop {
        match pc {
            0x83008140 => {
    //   block [0x83008140..0x83008328)
	// 83008140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008144: 481A0011  bl 0x831a8154
	ctx.lr = 0x83008148;
	sub_831A8130(ctx, base);
	// 83008148: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300814C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83008150: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83008154: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008158: 816B0068  lwz r11, 0x68(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8300815C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83008160: 4E800421  bctrl
	ctx.lr = 0x83008164;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008164: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83008168: 4182000C  beq 0x83008174
	if ctx.cr[0].eq {
	pc = 0x83008174; continue 'dispatch;
	}
	// 8300816C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83008170: 480001B0  b 0x83008320
	pc = 0x83008320; continue 'dispatch;
	// 83008174: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83008178: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 8300817C: 4BFD765D  bl 0x82fdf7d8
	ctx.lr = 0x83008180;
	sub_82FDF7D8(ctx, base);
	// 83008180: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83008184: 4082000C  bne 0x83008190
	if !ctx.cr[0].eq {
	pc = 0x83008190; continue 'dispatch;
	}
	// 83008188: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300818C: 48000194  b 0x83008320
	pc = 0x83008320; continue 'dispatch;
	// 83008190: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008194: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83008198: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8300819C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830081A0: 4E800421  bctrl
	ctx.lr = 0x830081A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830081A4: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830081A8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830081AC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830081B0: 557F063E  clrlwi r31, r11, 0x18
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830081B4: 814A0064  lwz r10, 0x64(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(100 as u32) ) } as u64;
	// 830081B8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 830081BC: 4E800421  bctrl
	ctx.lr = 0x830081C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830081C0: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 830081C4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830081C8: 409AFFC0  bne cr6, 0x83008188
	if !ctx.cr[6].eq {
	pc = 0x83008188; continue 'dispatch;
	}
	// 830081CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830081D0: 419A0144  beq cr6, 0x83008314
	if ctx.cr[6].eq {
	pc = 0x83008314; continue 'dispatch;
	}
	// 830081D4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830081D8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830081DC: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 830081E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830081E4: 4E800421  bctrl
	ctx.lr = 0x830081E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830081E8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830081EC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830081F0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830081F4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 830081F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830081FC: 4E800421  bctrl
	ctx.lr = 0x83008200;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008200: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008204: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83008208: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8300820C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83008210: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83008214: 4E800421  bctrl
	ctx.lr = 0x83008218;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008218: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300821C: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 83008220: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83008224: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83008228: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300822C: 4E800421  bctrl
	ctx.lr = 0x83008230;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008230: 7F181840  cmplw cr6, r24, r3
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83008234: 409AFF54  bne cr6, 0x83008188
	if !ctx.cr[6].eq {
	pc = 0x83008188; continue 'dispatch;
	}
	// 83008238: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8300823C: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 83008240: 419A00D4  beq cr6, 0x83008314
	if ctx.cr[6].eq {
	pc = 0x83008314; continue 'dispatch;
	}
	// 83008244: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008248: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8300824C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83008250: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83008254: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83008258: 4E800421  bctrl
	ctx.lr = 0x8300825C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300825C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83008260: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008264: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 83008268: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300826C: 4E800421  bctrl
	ctx.lr = 0x83008270;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008270: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83008274: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008278: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300827C: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008280: 40820028  bne 0x830082a8
	if !ctx.cr[0].eq {
	pc = 0x830082A8; continue 'dispatch;
	}
	// 83008284: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83008288: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300828C: 4E800421  bctrl
	ctx.lr = 0x83008290;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008290: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83008294: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83008298: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300829C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830082A0: 4E800421  bctrl
	ctx.lr = 0x830082A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830082A4: 48000040  b 0x830082e4
	pc = 0x830082E4; continue 'dispatch;
	// 830082A8: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 830082AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830082B0: 4E800421  bctrl
	ctx.lr = 0x830082B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830082B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830082B8: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 830082BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830082C0: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 830082C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830082C8: 4E800421  bctrl
	ctx.lr = 0x830082CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830082CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830082D0: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830082D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830082D8: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 830082DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830082E0: 4E800421  bctrl
	ctx.lr = 0x830082E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830082E4: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 830082E8: 4182FEA0  beq 0x83008188
	if ctx.cr[0].eq {
	pc = 0x83008188; continue 'dispatch;
	}
	// 830082EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830082F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830082F4: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 830082F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830082FC: 4E800421  bctrl
	ctx.lr = 0x83008300;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008300: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83008304: 4182FE84  beq 0x83008188
	if ctx.cr[0].eq {
	pc = 0x83008188; continue 'dispatch;
	}
	// 83008308: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 8300830C: 7F19C040  cmplw cr6, r25, r24
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[24].u32, &mut ctx.xer);
	// 83008310: 4198FF34  blt cr6, 0x83008244
	if ctx.cr[6].lt {
	pc = 0x83008244; continue 'dispatch;
	}
	// 83008314: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83008318: 387C000C  addi r3, r28, 0xc
	ctx.r[3].s64 = ctx.r[28].s64 + 12;
	// 8300831C: 4BFF428D  bl 0x82ffc5a8
	ctx.lr = 0x83008320;
	sub_82FFC5A8(ctx, base);
	// 83008320: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83008324: 4819FE80  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008328 size=404
    let mut pc: u32 = 0x83008328;
    'dispatch: loop {
        match pc {
            0x83008328 => {
    //   block [0x83008328..0x830084BC)
	// 83008328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300832C: 4819FE35  bl 0x831a8160
	ctx.lr = 0x83008330;
	sub_831A8130(ctx, base);
	// 83008330: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008334: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83008338: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8300833C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83008340: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008344: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83008348: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300834C: 4E800421  bctrl
	ctx.lr = 0x83008350;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008350: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83008354: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83008358: 419A0124  beq cr6, 0x8300847c
	if ctx.cr[6].eq {
	pc = 0x8300847C; continue 'dispatch;
	}
	// 8300835C: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008360: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83008364: 41820118  beq 0x8300847c
	if ctx.cr[0].eq {
	pc = 0x8300847C; continue 'dispatch;
	}
	// 83008368: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300836C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83008370: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83008374: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83008378: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300837C: 4E800421  bctrl
	ctx.lr = 0x83008380;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008380: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83008384: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 83008388: 3B7D0004  addi r27, r29, 4
	ctx.r[27].s64 = ctx.r[29].s64 + 4;
	// 8300838C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83008390: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83008394: 4BFDE145  bl 0x82fe64d8
	ctx.lr = 0x83008398;
	sub_82FE64D8(ctx, base);
	// 83008398: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300839C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830083A0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830083A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830083A8: 4E800421  bctrl
	ctx.lr = 0x830083AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830083AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830083B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830083B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830083B8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 830083BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830083C0: 4E800421  bctrl
	ctx.lr = 0x830083C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830083C4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830083C8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830083CC: 419A0048  beq cr6, 0x83008414
	if ctx.cr[6].eq {
	pc = 0x83008414; continue 'dispatch;
	}
	// 830083D0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830083D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830083D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830083DC: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830083E0: 4800002C  b 0x8300840c
	pc = 0x8300840C; continue 'dispatch;
	// 830083E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830083E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830083EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830083F0: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830083F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830083F8: 4E800421  bctrl
	ctx.lr = 0x830083FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830083FC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008400: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83008404: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83008408: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8300840C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83008410: 4E800421  bctrl
	ctx.lr = 0x83008414;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008414: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300841C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83008420: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83008424: 4E800421  bctrl
	ctx.lr = 0x83008428;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008428: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8300842C: 4082FFB8  bne 0x830083e4
	if !ctx.cr[0].eq {
	pc = 0x830083E4; continue 'dispatch;
	}
	// 83008430: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83008434: 419A0020  beq cr6, 0x83008454
	if ctx.cr[6].eq {
	pc = 0x83008454; continue 'dispatch;
	}
	// 83008438: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300843C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83008440: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83008444: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83008448: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8300844C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83008450: 4E800421  bctrl
	ctx.lr = 0x83008454;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008454: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83008458: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300845C: 4803290D  bl 0x8303ad68
	ctx.lr = 0x83008460;
	sub_8303AD68(ctx, base);
	// 83008460: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83008464: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83008468: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8300846C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83008470: 4BFD7309  bl 0x82fdf778
	ctx.lr = 0x83008474;
	sub_82FDF778(ctx, base);
	// 83008474: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83008478: 4800003C  b 0x830084b4
	pc = 0x830084B4; continue 'dispatch;
	// 8300847C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83008480: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83008484: 4BFD9555  bl 0x82fe19d8
	ctx.lr = 0x83008488;
	sub_82FE19D8(ctx, base);
	// 83008488: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300848C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83008490: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83008494: 816B00F8  lwz r11, 0xf8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(248 as u32) ) } as u64;
	// 83008498: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 8300849C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830084A0: 4E800421  bctrl
	ctx.lr = 0x830084A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830084A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830084A8: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830084AC: 4803274D  bl 0x8303abf8
	ctx.lr = 0x830084B0;
	sub_8303ABF8(ctx, base);
	// 830084B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830084B4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830084B8: 4819FCF8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830084C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830084C0 size=12
    let mut pc: u32 = 0x830084C0;
    'dispatch: loop {
        match pc {
            0x830084C0 => {
    //   block [0x830084C0..0x830084CC)
	// 830084C0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830084C4: 386B2F60  addi r3, r11, 0x2f60
	ctx.r[3].s64 = ctx.r[11].s64 + 12128;
	// 830084C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830084D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830084D0 size=8
    let mut pc: u32 = 0x830084D0;
    'dispatch: loop {
        match pc {
            0x830084D0 => {
    //   block [0x830084D0..0x830084D8)
	// 830084D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830084D4: 82141320  lwz r16, 0x1320(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(4896 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830084D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830084D8 size=312
    let mut pc: u32 = 0x830084D8;
    'dispatch: loop {
        match pc {
            0x830084D8 => {
    //   block [0x830084D8..0x83008610)
	// 830084D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830084DC: 4819FC85  bl 0x831a8160
	ctx.lr = 0x830084E0;
	sub_831A8130(ctx, base);
	// 830084E0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830084E4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830084E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830084EC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830084F0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830084F4: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 830084F8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830084FC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83008500: 396B1040  addi r11, r11, 0x1040
	ctx.r[11].s64 = ctx.r[11].s64 + 4160;
	// 83008504: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83008508: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 8300850C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83008510: 409A0008  bne cr6, 0x83008518
	if !ctx.cr[6].eq {
	pc = 0x83008518; continue 'dispatch;
	}
	// 83008514: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83008518: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8300851C: 4BFD6CFD  bl 0x82fdf218
	ctx.lr = 0x83008520;
	sub_82FDF218(ctx, base);
	// 83008520: 3B9E000C  addi r28, r30, 0xc
	ctx.r[28].s64 = ctx.r[30].s64 + 12;
	// 83008524: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83008528: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300852C: 4BFF3F6D  bl 0x82ffc498
	ctx.lr = 0x83008530;
	sub_82FFC498(ctx, base);
	// 83008530: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 83008534: 4818567D  bl 0x8318dbb0
	ctx.lr = 0x83008538;
	sub_8318DBB0(ctx, base);
	// 83008538: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300853C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83008540: 935E0024  stw r26, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[26].u32 ) };
	// 83008544: 935E0028  stw r26, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[26].u32 ) };
	// 83008548: 4BFD9491  bl 0x82fe19d8
	ctx.lr = 0x8300854C;
	sub_82FE19D8(ctx, base);
	// 8300854C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83008550: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83008554: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83008558: 4BFFF3D9  bl 0x83007930
	ctx.lr = 0x8300855C;
	sub_83007930(ctx, base);
	// 8300855C: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83008560: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83008564: 409A006C  bne cr6, 0x830085d0
	if !ctx.cr[6].eq {
	pc = 0x830085D0; continue 'dispatch;
	}
	// 83008568: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300856C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 83008570: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83008574: 4BFD947D  bl 0x82fe19f0
	ctx.lr = 0x83008578;
	sub_82FE19F0(ctx, base);
	// 83008578: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8300857C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83008580: 41820014  beq 0x83008594
	if ctx.cr[0].eq {
	pc = 0x83008594; continue 'dispatch;
	}
	// 83008584: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83008588: 48031339  bl 0x830398c0
	ctx.lr = 0x8300858C;
	sub_830398C0(ctx, base);
	// 8300858C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83008590: 48000008  b 0x83008598
	pc = 0x83008598; continue 'dispatch;
	// 83008594: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 83008598: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300859C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 830085A0: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 830085A4: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830085A8: 4BFD9449  bl 0x82fe19f0
	ctx.lr = 0x830085AC;
	sub_82FE19F0(ctx, base);
	// 830085AC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830085B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830085B4: 41820010  beq 0x830085c4
	if ctx.cr[0].eq {
	pc = 0x830085C4; continue 'dispatch;
	}
	// 830085B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830085BC: 48031305  bl 0x830398c0
	ctx.lr = 0x830085C0;
	sub_830398C0(ctx, base);
	// 830085C0: 48000008  b 0x830085c8
	pc = 0x830085C8; continue 'dispatch;
	// 830085C4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830085C8: 907E0024  stw r3, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 830085CC: 48000038  b 0x83008604
	pc = 0x83008604; continue 'dispatch;
	// 830085D0: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830085D4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 830085D8: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830085DC: 4BFD9415  bl 0x82fe19f0
	ctx.lr = 0x830085E0;
	sub_82FE19F0(ctx, base);
	// 830085E0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830085E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830085E8: 41820014  beq 0x830085fc
	if ctx.cr[0].eq {
	pc = 0x830085FC; continue 'dispatch;
	}
	// 830085EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830085F0: 80BE0028  lwz r5, 0x28(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830085F4: 480328BD  bl 0x8303aeb0
	ctx.lr = 0x830085F8;
	sub_8303AEB0(ctx, base);
	// 830085F8: 48000008  b 0x83008600
	pc = 0x83008600; continue 'dispatch;
	// 830085FC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83008600: 907E0024  stw r3, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 83008604: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83008608: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8300860C: 4819FBA4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008610 size=40
    let mut pc: u32 = 0x83008610;
    'dispatch: loop {
        match pc {
            0x83008610 => {
    //   block [0x83008610..0x83008638)
	// 83008610: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83008614: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008618: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300861C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008620: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83008624: 4BFFD55D  bl 0x83005b80
	ctx.lr = 0x83008628;
	sub_83005B80(ctx, base);
	// 83008628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300862C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008638 size=44
    let mut pc: u32 = 0x83008638;
    'dispatch: loop {
        match pc {
            0x83008638 => {
    //   block [0x83008638..0x83008664)
	// 83008638: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8300863C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008640: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008644: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008648: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300864C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83008650: 480BF491  bl 0x830c7ae0
	ctx.lr = 0x83008654;
	sub_830C7AE0(ctx, base);
	// 83008654: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83008658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300865C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008664(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008664 size=44
    let mut pc: u32 = 0x83008664;
    'dispatch: loop {
        match pc {
            0x83008664 => {
    //   block [0x83008664..0x83008690)
	// 83008664: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83008668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300866C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008674: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83008678: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 8300867C: 4BFD8BAD  bl 0x82fe1228
	ctx.lr = 0x83008680;
	sub_82FE1228(ctx, base);
	// 83008680: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83008684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300868C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008690 size=44
    let mut pc: u32 = 0x83008690;
    'dispatch: loop {
        match pc {
            0x83008690 => {
    //   block [0x83008690..0x830086BC)
	// 83008690: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83008694: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008698: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300869C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830086A0: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830086A4: 386B001C  addi r3, r11, 0x1c
	ctx.r[3].s64 = ctx.r[11].s64 + 28;
	// 830086A8: 480BF439  bl 0x830c7ae0
	ctx.lr = 0x830086AC;
	sub_830C7AE0(ctx, base);
	// 830086AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830086B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830086B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830086B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830086BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830086BC size=44
    let mut pc: u32 = 0x830086BC;
    'dispatch: loop {
        match pc {
            0x830086BC => {
    //   block [0x830086BC..0x830086E8)
	// 830086BC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830086C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830086C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830086C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830086CC: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830086D0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830086D4: 480BF40D  bl 0x830c7ae0
	ctx.lr = 0x830086D8;
	sub_830C7AE0(ctx, base);
	// 830086D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830086DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830086E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830086E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830086E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830086E8 size=44
    let mut pc: u32 = 0x830086E8;
    'dispatch: loop {
        match pc {
            0x830086E8 => {
    //   block [0x830086E8..0x83008714)
	// 830086E8: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830086EC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830086F0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830086F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830086F8: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830086FC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83008700: 480BF3E1  bl 0x830c7ae0
	ctx.lr = 0x83008704;
	sub_830C7AE0(ctx, base);
	// 83008704: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83008708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300870C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008714(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008714 size=44
    let mut pc: u32 = 0x83008714;
    'dispatch: loop {
        match pc {
            0x83008714 => {
    //   block [0x83008714..0x83008740)
	// 83008714: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83008718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300871C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008724: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83008728: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300872C: 480BF3B5  bl 0x830c7ae0
	ctx.lr = 0x83008730;
	sub_830C7AE0(ctx, base);
	// 83008730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83008734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300873C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83008740 size=8
    let mut pc: u32 = 0x83008740;
    'dispatch: loop {
        match pc {
            0x83008740 => {
    //   block [0x83008740..0x83008748)
	// 83008740: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83008744: 821413D0  lwz r16, 0x13d0(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(5072 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


